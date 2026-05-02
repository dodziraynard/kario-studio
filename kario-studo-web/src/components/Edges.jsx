import React from 'react';

export default function Edges({ nodes, pos, inputPos, selectedId, w, h }) {
  const lines = [];
  const labels = [];

  function getBottom(id) {
    const p = pos[id];
    if (!p) return 0;
    return p.y + p.h;
  }

  Object.values(nodes).forEach((n) => {
    if (!n.parent || !pos[n.parent] || !pos[n.id]) return;
    const pp = pos[n.parent];
    const cp = pos[n.id];
    const isBranch = nodes[n.parent].branch !== n.branch;
    const x1 = pp.x + pp.w / 2;
    const y1 = getBottom(n.parent);
    const x2 = cp.x + cp.w / 2;
    const y2 = cp.y;
    const col = isBranch ? n.color : 'rgba(255,255,255,.2)';
    if (Math.abs(x1 - x2) < 2) {
      lines.push(<line key={n.id} x1={x1} y1={y1} x2={x2} y2={y2} stroke={col} strokeWidth={1.5} />);
    } else {
      const my = y1 + (y2 - y1) * 0.45;
      const r = 14;
      const dx = x2 > x1 ? 1 : -1;
      const d = `M${x1},${y1} L${x1},${my - r} Q${x1},${my} ${x1 + r * dx},${my} L${x2 - r * dx},${my} Q${x2},${my} ${x2},${my + r} L${x2},${y2}`;
      lines.push(<path key={n.id} d={d} fill="none" stroke={col} strokeWidth={1.5} />);
      if (isBranch) {
        labels.push(
          <div
            key={'l' + n.id}
            className="elabel"
            style={{
              left: (x1 + x2) / 2 - 20,
              top: my - 9,
              color: n.color,
              borderColor: n.color + '30',
            }}
          >
            {n.branch}
          </div>
        );
      }
    }
  });

  if (selectedId && pos[selectedId] && inputPos) {
    const sp = pos[selectedId];
    const x1 = sp.x + sp.w / 2;
    const y1 = getBottom(selectedId);
    const x2 = inputPos.x + inputPos.w / 2;
    const y2 = inputPos.y;
    lines.push(
      <line
        key="input-edge"
        x1={x1}
        y1={y1}
        x2={x2}
        y2={y2}
        stroke="rgba(167,139,250,.4)"
        strokeWidth={1.5}
      />
    );
  }

  return (
    <>
      <svg width={w} height={h}>
        {lines}
      </svg>
      {labels}
    </>
  );
}
