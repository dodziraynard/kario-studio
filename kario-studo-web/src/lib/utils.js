import { VS } from './constants.js';

export const rnd = (a) => a[Math.floor(Math.random() * a.length)];

export const ft = (ts) =>
  new Date(ts).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });

export const fmtTime = (s) => {
  if (!s || isNaN(s)) return '0:00';
  const m = Math.floor(s / 60);
  return m + ':' + String(Math.floor(s % 60)).padStart(2, '0');
};

export function isTextOnlyPrompt(text) {
  const t = text.toLowerCase();
  if (t.match(/^(what|how|why|when|where|who|is |are |can |does |do |should |could |would |tell me|explain|describe|show me|list)/))
    return 'question';
  if (t.match(/^(wait|hold|pause|stop|thinking|hmm|ok |okay|got it|understood|i see|interesting|note|remember|keep in mind|fyi|btw)/))
    return 'feedback';
  if (t.match(/^(analyze|check|review|evaluate|compare|assess|look at|inspect)/))
    return 'status';
  if (t.length < 15 && !t.match(/red|blue|green|fast|slow|dark|bright|glow|minimal|reverse|flip|add|remove|change|make|set/))
    return 'feedback';
  return null;
}

export function inferVS(prompt, base) {
  const p = prompt.toLowerCase();
  if (p.match(/red|fire|warm|glow/)) return VS[1];
  if (p.match(/fast|energy|speed/)) return VS[2];
  if (p.match(/minimal|clean|simple|white/)) return VS[3];
  if (p.match(/reverse|flip/)) return VS[4];
  if (p.match(/blue|cyber|neon|electric/)) return VS[0];
  if (p.match(/dark|noir|black/)) return { ...VS[3], text: base?.text || 'MOTION' };
  if (p.match(/purple|violet/))
    return { ...VS[0], pc: 'purple', bg: ['#0a0514', '#14051a', '#0a0514'] };
  const q = prompt.match(/["'](.+?)["']/);
  if (q) return { ...(base || VS[0]), text: q[1].toUpperCase().slice(0, 20) };
  return VS[Math.floor(Math.random() * VS.length)];
}

export function generatePlan(input) {
  const w = (input || '').toLowerCase();
  const plan = [
    { title: 'Opening', dur: '0:00–0:03', desc: 'Dark background fade-in. Set the tone with ambient particles and subtle motion.' },
    { title: 'Title reveal', dur: '0:03–0:06', desc: 'Main title emerges from particles/elements. Primary text animation with easing.' },
    { title: 'Body sequence', dur: '0:06–0:09', desc: 'Supporting content, secondary text, or visual details. Build on the established style.' },
    { title: 'Closing', dur: '0:09–0:12', desc: 'Final composition hold. Tagline or call-to-action with fade-out.' },
  ];
  if (w.includes('logo'))
    plan[1].desc = 'Logo reveal with particle convergence effect. Elements scatter then form the logo shape.';
  if (w.includes('product'))
    plan.splice(2, 0, {
      title: 'Product showcase',
      dur: '0:06–0:08',
      desc: 'Product appears with highlight effects. Key features called out with text overlays.',
    });
  if (w.includes('cinematic'))
    plan[0].desc = 'Cinematic dark opening with lens flare and depth-of-field particles.';
  if (w.includes('gold')) plan.forEach((s) => (s.desc = s.desc.replace('particles', 'gold particles')));
  return plan;
}

export function planToText(plan) {
  return plan.map((s, i) => `${i + 1}. ${s.title} [${s.dur}]\n   ${s.desc}`).join('\n\n');
}

export function planToScript(plan) {
  return (
    plan.map((s) => `[${s.dur}] ${s.title.toUpperCase()}\n${s.desc}`).join('\n\n') +
    '\n\nMotion: ease-in-out, 24fps\nColor: inherit from global palette'
  );
}

export function assetType(file) {
  if (file.type.startsWith('image/')) return 'image';
  if (file.type.startsWith('audio/')) return 'audio';
  const ext = file.name.split('.').pop().toLowerCase();
  if (['ttf', 'otf', 'woff', 'woff2'].includes(ext)) return 'font';
  return 'other';
}

export function readFileAsAsset(file) {
  return new Promise((resolve) => {
    const type = assetType(file);
    if (type === 'other') {
      resolve(null);
      return;
    }
    const reader = new FileReader();
    reader.onload = () => resolve({ name: file.name, type, dataUrl: reader.result });
    reader.readAsDataURL(file);
  });
}

export async function readFilesAsAssets(fileList) {
  const arr = await Promise.all(Array.from(fileList).map(readFileAsAsset));
  return arr.filter(Boolean);
}

export function createActivities(steps) {
  return steps.map((s, i) => ({
    step: i,
    label: s.label,
    status: 'pending',
    detail: null,
    startedAt: null,
    completedAt: null,
    dur: s.dur,
  }));
}
