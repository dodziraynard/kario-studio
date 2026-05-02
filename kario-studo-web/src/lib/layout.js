import { NW, GX, GY, INPUT_H } from './constants.js';

export function nodeHeight(n, selectedId) {
  const assetH = n.assets && n.assets.length ? 44 : 0;
  const actH = n.activities && n.activities.length ? n.activities.length * 26 + 16 : 0;
  const isLoading = n.activities && n.activities.some((a) => a.status !== 'completed');
  if (n.role === 'ai' && isLoading) return actH + 80;
  if (n.role === 'ai' && n.vs) return n.id === selectedId ? 460 : 390;
  if (n.role === 'ai' && !n.vs) {
    if (n._collapsed && n.content.length > 120) return 130 + actH;
    const rawLines = n.content.split('\n');
    let totalLines = 0;
    rawLines.forEach((line) => {
      totalLines += Math.max(1, Math.ceil(line.length / 65));
    });
    return totalLines * 21 + 60 + actH;
  }
  return 110 + assetH;
}

function childrenOf(nodes, id) {
  return Object.values(nodes).filter((n) => n.parent === id);
}

export function layout(nodes, selectedId, collapsedBranches, viewport, zoom) {
  const roots = Object.values(nodes).filter((n) => !n.parent);
  if (!roots.length) return { pos: {}, w: 0, h: 0, inputPos: null };

  const pos = {};
  let nextSlot = 0;

  function isCollapsedRoot(n) {
    if (!n.parent) return false;
    const parent = nodes[n.parent];
    return parent && parent.branch !== n.branch && collapsedBranches.has(n.branch);
  }

  function assignX(nid) {
    const n = nodes[nid];
    if (isCollapsedRoot(n)) {
      pos[nid] = { x: nextSlot * (NW + GX), y: 0, w: NW, h: 44, collapsed: true, branch: n.branch };
      nextSlot++;
      return;
    }
    const visibleKids = childrenOf(nodes, nid);
    if (visibleKids.length === 0) {
      pos[nid] = { x: nextSlot * (NW + GX), y: 0, w: NW, h: nodeHeight(n, selectedId) };
      nextSlot++;
      return;
    }
    visibleKids.forEach((c) => assignX(c.id));
    const childXs = visibleKids.map((c) => pos[c.id].x);
    const minCX = Math.min(...childXs);
    const maxCX = Math.max(...childXs);
    pos[nid] = { x: minCX + (maxCX - minCX) / 2, y: 0, w: NW, h: nodeHeight(n, selectedId) };
  }
  roots.forEach((r) => assignX(r.id));

  function assignY(nid, y) {
    if (!pos[nid]) return;
    pos[nid].y = y;
    if (pos[nid].collapsed) return;
    const kids = childrenOf(nodes, nid);
    const childY = y + pos[nid].h + GY;
    kids.forEach((c) => assignY(c.id, childY));
  }
  roots.forEach((r) => assignY(r.id, 0));

  // input node position
  let inputPos = null;
  if (selectedId && pos[selectedId] && nodes[selectedId] && nodes[selectedId].role === 'ai') {
    const sp = pos[selectedId];
    const hasKids = childrenOf(nodes, selectedId).length > 0;
    if (hasKids) {
      inputPos = { x: sp.x + NW + GX, y: sp.y, w: NW, h: INPUT_H };
    } else {
      inputPos = { x: sp.x, y: sp.y + sp.h, w: NW, h: INPUT_H };
    }
  }

  const allPos = [...Object.values(pos)];
  if (inputPos) allPos.push(inputPos);
  if (!allPos.length) return { pos, w: 0, h: 0, inputPos };

  const minX = Math.min(...allPos.map((p) => p.x));
  const maxX = Math.max(...allPos.map((p) => p.x + p.w));
  const minY = Math.min(...allPos.map((p) => p.y));
  const maxY = Math.max(...allPos.map((p) => p.y + p.h));
  const treeW = maxX - minX;
  const treeH = maxY - minY;

  const vpW = (viewport?.w || 1200) / zoom;
  const vpH = (viewport?.h || 800) / zoom;
  const margin = Math.max(vpW, vpH);

  const offsetX = margin - minX;
  const offsetY = margin - minY;

  Object.keys(pos).forEach((id) => {
    pos[id].x += offsetX;
    pos[id].y += offsetY;
  });
  if (inputPos) {
    inputPos.x += offsetX;
    inputPos.y += offsetY;
  }

  const fw = treeW + margin * 2;
  const fh = treeH + margin * 2;
  return { pos, w: fw, h: fh, inputPos };
}

export function activeChain(nodes, selectedId) {
  const chain = new Set();
  let cur = selectedId ? nodes[selectedId] : null;
  while (cur) {
    chain.add(cur.id);
    cur = cur.parent ? nodes[cur.parent] : null;
  }
  return chain;
}

export function branchNodeIds(nodes, branch) {
  return Object.values(nodes)
    .filter((n) => n.branch === branch)
    .map((n) => n.id);
}

export function nearestVS(nodes, nid) {
  let c = nodes[nid];
  while (c) {
    if (c.vs) return c.vs;
    c = c.parent ? nodes[c.parent] : null;
  }
  return null;
}

export function allBranches(nodes) {
  return [...new Set(Object.values(nodes).map((n) => n.branch))];
}

export function getChildrenOf(nodes, id) {
  return childrenOf(nodes, id);
}
