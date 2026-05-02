/**
 * setup_templates.mjs
 *
 * Creates the `templates` collection in kario_studio, embeds each template
 * description using OpenAI text-embedding-3-small, inserts the documents,
 * and creates the Atlas Vector Search index.
 *
 * Usage:
 *   node scripts/setup_templates.mjs
 *
 * Requires: MONGODB_URI and OPENAI_API_KEY in .env (or environment).
 */

import { MongoClient } from "mongodb";
import { readFileSync } from "fs";
import { resolve, dirname } from "path";
import { fileURLToPath } from "url";

// ── Load .env manually (no external deps needed) ─────────────────────────────
const __dir = dirname(fileURLToPath(import.meta.url));
const envPath = resolve(__dir, "../.env");
for (const line of readFileSync(envPath, "utf8").split("\n")) {
  const m = line.match(/^([^#=\s]+)\s*=\s*(.*)$/);
  if (m) process.env[m[1]] = m[2].trim();
}

const MONGODB_URI = process.env.MONGODB_URI;
const OPENAI_API_KEY = process.env.OPENAI_API_KEY;
if (!MONGODB_URI) throw new Error("MONGODB_URI not set");
if (!OPENAI_API_KEY) throw new Error("OPENAI_API_KEY not set");

// ── Template definitions ──────────────────────────────────────────────────────
const TEMPLATES = [
  {
    name: "product_launch",
    description:
      "Product launch video showcasing a new product's features, benefits, and call to action. Great for SaaS tools, physical products, and app launches.",
  },
];

// ── Embed via OpenAI ──────────────────────────────────────────────────────────
async function embed(text) {
  const res = await fetch("https://api.openai.com/v1/embeddings", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${OPENAI_API_KEY}`,
    },
    body: JSON.stringify({ input: text, model: "text-embedding-3-small" }),
  });
  if (!res.ok) throw new Error(`OpenAI error ${res.status}: ${await res.text()}`);
  const json = await res.json();
  return json.data[0].embedding; // float[]
}

// ── Main ──────────────────────────────────────────────────────────────────────
const client = new MongoClient(MONGODB_URI);
await client.connect();
const db = client.db("kario_studio");
const col = db.collection("templates");

console.log("Embedding templates…");
for (const tpl of TEMPLATES) {
  const embedding = await embed(tpl.description);
  await col.updateOne(
    { name: tpl.name },
    { $set: { name: tpl.name, description: tpl.description, embedding } },
    { upsert: true }
  );
  console.log(`  ✓ ${tpl.name} (${embedding.length} dims)`);
}

// ── Create Atlas Vector Search index ─────────────────────────────────────────
console.log("\nCreating vector search index…");
try {
  await db.command({
    createSearchIndexes: "templates",
    indexes: [
      {
        name: "template_embedding_index",
        type: "vectorSearch",
        definition: {
          fields: [
            {
              type: "vector",
              path: "embedding",
              numDimensions: 1536,
              similarity: "cosine",
            },
          ],
        },
      },
    ],
  });
  console.log("  ✓ Index creation initiated (may take ~1 min to become READY)");
} catch (e) {
  if (e.message?.includes("already exists") || e.code === 68) {
    console.log("  ℹ  Index already exists, skipping");
  } else {
    throw e;
  }
}

await client.close();
console.log("\nDone ✓");
