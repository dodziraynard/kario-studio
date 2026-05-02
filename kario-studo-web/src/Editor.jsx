import React, { useCallback, useEffect, useLayoutEffect, useRef, useState } from 'react';
import StartNode from './components/StartNode.jsx';
import NodeCard from './components/NodeCard.jsx';
import InputNode from './components/InputNode.jsx';
import Lightbox from './components/Lightbox.jsx';
import Edges from './components/Edges.jsx';
import { COLORS, VS, NW } from './lib/constants.js';
import { readFilesAsAssets } from './lib/utils.js';
import { layout, activeChain, branchNodeIds, nearestVS, allBranches, getChildrenOf } from './lib/layout.js';

let _uidCounter = 0;
const uid = () => 'n' + ++_uidCounter;

import { API_BASE, renderUrl } from './lib/api.js';

// Map real job step status → activity status used by NodeCard
// Map server step status → activity status used by NodeCard
const STEP_STATUS_MAP = {
  done: 'completed',
  completed: 'completed',
  success: 'completed',
  running: 'in_progress',
  pending: 'pending',
  queued: 'pending',
  failed: 'failed',
  error: 'failed',
};

// Convert /api/projects/:id/jobs payload into a linear chain of user/ai nodes
function jobsToNodes(jobs, project) {
  const sorted = [...(jobs || [])].sort(
    (a, b) => new Date(a.created_at).getTime() - new Date(b.created_at).getTime()
  );
  const nodes = {};
  let prevAiId = null;
  let lastAiId = null;
  const branch = 'main';
  const color = COLORS[0];

  sorted.forEach((job) => {
    const userId = 'u-' + job.id;
    const aiId = job.id;

    nodes[userId] = {
      id: userId,
      parent: prevAiId,
      role: 'user',
      content: job.prompt || job.template || '(no prompt)',
      ts: new Date(job.created_at).getTime(),
      vs: null,
      branch,
      color,
      assets: null,
      activities: null,
    };

    const activities = (job.steps || []).map((s, i) => {
      const startedAt = s.started_at ? new Date(s.started_at).getTime() : null;
      const completedAt = s.finished_at ? new Date(s.finished_at).getTime() : null;
      return {
        step: i,
        label: s.name,
        status: STEP_STATUS_MAP[s.status] || 'pending',
        detail: s.message || null,
        startedAt,
        completedAt,
        dur: completedAt && startedAt ? completedAt - startedAt : 0,
      };
    });

    const isLatest = project?.latestJobId === job.id;
    const allDone = activities.length > 0 && activities.every((a) => a.status === 'completed');

    nodes[aiId] = {
      id: aiId,
      parent: userId,
      role: 'ai',
      content: job.status === 'failed' ? 'Job failed' : 'Generated video',
      ts: new Date(job.updated_at || job.created_at).getTime(),
      vs: allDone ? VS[0] : null,
      branch,
      color,
      assets: null,
      activities,
      videoUrl: isLatest && project?.latestRenderUrl ? project.latestRenderUrl : (job.render_url ? renderUrl(job.render_url) : null),
      _jobStatus: job.status,
    };

    prevAiId = aiId;
    lastAiId = aiId;
  });

  return { nodes, lastId: lastAiId };
}

export default function Editor({ project, onBack, onCreated }) {
  const [nodes, setNodes] = useState(() => project?.nodes || {});
  const [selected, setSelected] = useState(() => project?.selected || null);
  const [collapsedBranches, setCollapsedBranches] = useState(() => new Set(project?.collapsedBranches || []));
  const [pendingAssets, setPendingAssets] = useState([]);

  // wizard (handoff only)
  const [wizPhase, setWizPhase] = useState(() =>
    project?.server || (project?.nodes && Object.keys(project.nodes).length) ? 'done' : 'start'
  );
  const [wizAssets, setWizAssets] = useState([]);

  const [zoom, setZoom] = useState(1);
  const [toast, setToast] = useState('');

  const [lb, setLB] = useState({ open: false, nid: null });
  const [snap, setSnap] = useState(null); // {video, container}

  const wrapRef = useRef(null);
  const [vp, setVp] = useState({ w: 1200, h: 800 });
  const measuredHeights = useRef({});
  const didFitRef = useRef(false);

  // sync uid counter with loaded project
  useEffect(() => {
    let max = 0;
    Object.keys(nodes).forEach((id) => {
      const m = id.match(/^n(\d+)$/);
      if (m) max = Math.max(max, parseInt(m[1], 10));
    });
    if (max > _uidCounter) _uidCounter = max;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // hydrate from server jobs (jobs ARE the nodes)
  useEffect(() => {
    if (!project?.server || !project?.id) return;
    let cancelled = false;
    let timer = null;

    const fetchJobs = async () => {
      try {
        const res = await fetch(`${API_BASE}/api/projects/${project.id}/jobs`);
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        const jobs = await res.json();
        if (cancelled) return;
        const built = jobsToNodes(jobs, project);
        setNodes(built.nodes);
        setSelected((cur) => cur || built.lastId || null);
        const stillRunning = jobs.some(
          (j) => j.status === 'running' || j.status === 'pending' || j.status === 'queued'
        );
        if (stillRunning) {
          timer = setTimeout(fetchJobs, 2500);
        }
      } catch (e) {
        if (!cancelled) showToast(`Couldn't load jobs: ${e.message}`);
      }
    };
    fetchJobs();
    return () => {
      cancelled = true;
      if (timer) clearTimeout(timer);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [project?.id, project?.server]);

  // persist project state back to parent
  // (server projects: source of truth is the API; no local save needed)

  useEffect(() => {
    const onResize = () => {
      if (wrapRef.current) setVp({ w: wrapRef.current.clientWidth, h: wrapRef.current.clientHeight });
    };
    onResize();
    window.addEventListener('resize', onResize);
    return () => window.removeEventListener('resize', onResize);
  }, []);

  const showToast = useCallback((m) => {
    setToast(m);
    setTimeout(() => setToast(''), 2000);
  }, []);

  // ── add node
  const addNode = useCallback((id, o) => {
    setNodes((prev) => ({
      ...prev,
      [id]: {
        id,
        parent: o.parent || null,
        role: o.role,
        content: o.content,
        ts: o.ts || Date.now(),
        vs: o.vs || null,
        branch: o.branch || 'main',
        color: o.color || COLORS[0],
        assets: o.assets || null,
        activities: o.activities || null,
        videoUrl: o.videoUrl || null,
      },
    }));
    return id;
  }, []);

  const updateNode = useCallback((id, patch) => {
    setNodes((prev) => {
      const n = prev[id];
      if (!n) return prev;
      return { ...prev, [id]: { ...n, ...(typeof patch === 'function' ? patch(n) : patch) } };
    });
  }, []);

  // ── poll a real job and update the AI node's activities + video
  const pollJob = useCallback((jobId, aiNodeId) => {
    let cancelled = false;
    let timer = null;
    const poll = async () => {
      try {
        const res = await fetch(`${API_BASE}/api/jobs/${jobId}`);
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        const job = await res.json();
        if (cancelled) return;
        const activities = (job.steps || []).map((s, i) => ({
          step: i,
          label: s.name,
          status: STEP_STATUS_MAP[s.status] || 'pending',
          detail: s.message || null,
          startedAt: s.started_at ? new Date(s.started_at).getTime() : null,
          completedAt: s.finished_at ? new Date(s.finished_at).getTime() : null,
          dur: 0,
        }));
        const done = job.status === 'done';
        const failed = job.status === 'failed';
        setNodes((prev) => {
          const n = prev[aiNodeId];
          if (!n) return prev;
          return {
            ...prev,
            [aiNodeId]: {
              ...n,
              activities,
              content: done ? 'Generated video' : failed ? 'Job failed' : '',
              vs: done ? VS[0] : null,
              videoUrl: done && job.render_url ? renderUrl(job.render_url) : null,
            },
          };
        });
        if (!done && !failed) {
          timer = setTimeout(poll, 2500);
        }
      } catch (e) {
        if (!cancelled) showToast(`Job error: ${e.message}`);
      }
    };
    poll();
    return () => { cancelled = true; if (timer) clearTimeout(timer); };
  }, [showToast]);

  // ── wizard handlers
  const wizAttach = async (files) => {
    const arr = await readFilesAsAssets(files);
    setWizAssets((a) => [...a, ...arr]);
  };
  const wizRemove = (i) => setWizAssets((a) => a.filter((_, idx) => idx !== i));

  const wizSubmit = async (text) => {
    text = text.trim();
    if (!text && !wizAssets.length) return;
    const c0 = COLORS[0];
    const assets = wizAssets.length ? [...wizAssets] : null;
    setWizAssets([]);
    const n1 = uid();
    addNode(n1, { role: 'user', content: text || '(attached assets)', branch: 'main', color: c0, assets });
    setWizPhase('done');
    const aId = uid();
    addNode(aId, { parent: n1, role: 'ai', content: '', branch: 'main', color: c0, activities: [] });
    setSelected(aId);
    try {
      const res = await fetch(`${API_BASE}/api/generate`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt: text }),
      });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const { job_id, project_id } = await res.json();
      pollJob(job_id, aId);
      if (project_id && onCreated) onCreated(project_id);
    } catch (e) {
      showToast(`Generate failed: ${e.message}`);
      updateNode(aId, { content: 'Error: ' + e.message });
    }
  };

  // helper using closure over nodes via getter
  const nodesRef = useRef(nodes);
  useEffect(() => {
    nodesRef.current = nodes;
  }, [nodes]);

  const branchTipNode = (branch) => {
    const all = nodesRef.current;
    const bn = Object.values(all).filter((n) => n.branch === branch);
    return bn.filter((n) => !Object.values(all).some((c) => c.parent === n.id && c.branch === branch)).pop() ||
      bn.pop() ||
      null;
  };

  // ── send (continues from selected leaf)
  const send = (text) => {
    text = (text || '').trim();
    if (!text && !pendingAssets.length) return;
    if (!selected) {
      showToast('Select a node first');
      return;
    }
    const all = nodesRef.current;
    const parent = all[selected];
    if (!parent) return;

    // block while any node in the tree is still processing
    const anyRunning = Object.values(all).some(
      (n) => n.activities && n.activities.some((a) => a.status === 'in_progress' || a.status === 'pending')
    );
    if (anyRunning) {
      showToast('Please wait — generation in progress');
      return;
    }

    const sameBranchKids = getChildrenOf(all, selected).filter((c) => c.branch === parent.branch);
    let branch = parent.branch;
    let color = parent.color;
    if (sameBranchKids.length > 0) {
      const branches = allBranches(all);
      branch = 'fork-' + branches.length;
      color = COLORS[branches.length % COLORS.length];
      showToast(`Forked → "${branch}"`);
    }

    const assets = pendingAssets.length ? [...pendingAssets] : null;
    setPendingAssets([]);
    const uId = uid();
    addNode(uId, { parent: selected, role: 'user', content: text || '(attached assets)', branch, color, assets });

    const aId = uid();
    addNode(aId, { parent: uId, role: 'ai', content: '', branch, color, activities: [] });
    setSelected(aId);

    // find the server project_id (walk up to find a node whose id is a real UUID)
    const projectId = project?.server ? project.id : null;
    const endpoint = projectId
      ? `${API_BASE}/api/projects/${projectId}/edit`
      : `${API_BASE}/api/generate`;

    fetch(endpoint, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ prompt: text }),
    })
      .then((r) => { if (!r.ok) throw new Error(`HTTP ${r.status}`); return r.json(); })
      .then(({ job_id }) => pollJob(job_id, aId))
      .catch((e) => {
        showToast(`Request failed: ${e.message}`);
        updateNode(aId, { content: 'Error: ' + e.message });
      });
  };

  // ── pending asset attach
  const attachPending = async (files) => {
    const arr = await readFilesAsAssets(files);
    setPendingAssets((p) => [...p, ...arr]);
  };
  const removePending = (i) => setPendingAssets((p) => p.filter((_, idx) => idx !== i));

  // ── select / collapse / expand
  const toggleCollapse = (b) => {
    setCollapsedBranches((s) => {
      const n = new Set(s);
      if (n.has(b)) n.delete(b);
      else n.add(b);
      return n;
    });
  };
  const expandBranch = (b) => {
    setCollapsedBranches((s) => {
      const n = new Set(s);
      n.delete(b);
      return n;
    });
  };
  const toggleExpand = (id) => updateNode(id, (n) => ({ _collapsed: !n._collapsed }));
  const toggleAct = (id) => updateNode(id, (n) => ({ _actExpanded: !n._actExpanded }));

  // ── layout (recompute on relevant deps)
  const { pos, w, h, inputPos } = layout(nodes, selected, collapsedBranches, vp, zoom);

  // override pos heights with measured DOM heights for accurate input/edge anchoring
  // (simple approach: replace h with measured value if available)
  Object.keys(measuredHeights.current).forEach((id) => {
    if (pos[id] && !pos[id].collapsed) {
      // do not rewrite — would invalidate positions; but this can affect the input edge
    }
  });

  // adjust input top to be just below selected node's measured height
  let inputPosFixed = inputPos;
  if (inputPos && selected && pos[selected] && measuredHeights.current[selected]) {
    const measured = measuredHeights.current[selected] / zoom;
    const sp = pos[selected];
    const hasKids = getChildrenOf(nodes, selected).length > 0;
    if (!hasKids) {
      inputPosFixed = { ...inputPos, y: sp.y + measured };
    }
  }

  const chain = activeChain(nodes, selected);

  // ── zoom controls
  const setZoomAt = useCallback(
    (newZoom, pivotX, pivotY) => {
      const wrap = wrapRef.current;
      if (!wrap) return;
      const prev = zoom;
      const nz = Math.min(3, Math.max(0.2, newZoom));
      if (nz === prev) return;
      if (pivotX != null && pivotY != null) {
        const canvasX = (wrap.scrollLeft + pivotX) / prev;
        const canvasY = (wrap.scrollTop + pivotY) / prev;
        setZoom(nz);
        requestAnimationFrame(() => {
          wrap.scrollLeft = canvasX * nz - pivotX;
          wrap.scrollTop = canvasY * nz - pivotY;
        });
      } else {
        setZoom(nz);
      }
    },
    [zoom]
  );

  const zoomIn = () => {
    const w = wrapRef.current;
    if (!w) return;
    setZoomAt(zoom * 1.2, w.clientWidth / 2, w.clientHeight / 2);
  };
  const zoomOut = () => {
    const w = wrapRef.current;
    if (!w) return;
    setZoomAt(zoom / 1.2, w.clientWidth / 2, w.clientHeight / 2);
  };
  const resetZoom = () => {
    const positions = Object.values(pos);
    if (!positions.length) return;
    const minX = Math.min(...positions.map((p) => p.x));
    const maxX = Math.max(...positions.map((p) => p.x + p.w));
    const minY = Math.min(...positions.map((p) => p.y));
    const maxY = Math.max(...positions.map((p) => p.y + p.h));
    const treeW = maxX - minX;
    const treeH = maxY - minY;
    const wrap = wrapRef.current;
    const pad = 100;
    const sx = (wrap.clientWidth - pad * 2) / treeW;
    const sy = (wrap.clientHeight - pad * 2) / treeH;
    const fz = Math.min(sx, sy, 1);
    setZoom(fz);
    const cx = minX + treeW / 2;
    const cy = minY + treeH / 2;
    requestAnimationFrame(() => {
      wrap.scrollTo({
        left: cx * fz - wrap.clientWidth / 2,
        top: cy * fz - wrap.clientHeight / 2,
        behavior: 'smooth',
      });
    });
  };

  // auto-fit once when nodes first appear
  useEffect(() => {
    if (didFitRef.current) return;
    const positions = Object.values(pos);
    if (!positions.length || !wrapRef.current) return;
    const wrap = wrapRef.current;
    if (!wrap.clientWidth || !wrap.clientHeight) return;
    didFitRef.current = true;
    const minX = Math.min(...positions.map((p) => p.x));
    const maxX = Math.max(...positions.map((p) => p.x + p.w));
    const minY = Math.min(...positions.map((p) => p.y));
    const maxY = Math.max(...positions.map((p) => p.y + p.h));
    const treeW = maxX - minX || 1;
    const treeH = maxY - minY || 1;
    const pad = 100;
    const fz = Math.min((wrap.clientWidth - pad * 2) / treeW, (wrap.clientHeight - pad * 2) / treeH, 1);
    setZoom(fz);
    const cx = minX + treeW / 2;
    const cy = minY + treeH / 2;
    requestAnimationFrame(() => {
      wrap.scrollTo({
        left: cx * fz - wrap.clientWidth / 2,
        top: cy * fz - wrap.clientHeight / 2,
        behavior: 'smooth',
      });
    });
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [pos]);

  // wheel zoom
  useEffect(() => {
    const wrap = wrapRef.current;
    if (!wrap) return;
    const onWheel = (e) => {
      if (e.ctrlKey || e.metaKey) {
        e.preventDefault();
        const rect = wrap.getBoundingClientRect();
        const px = e.clientX - rect.left;
        const py = e.clientY - rect.top;
        const factor = 1 - e.deltaY * 0.005;
        setZoomAt(zoom * factor, px, py);
      }
    };
    wrap.addEventListener('wheel', onWheel, { passive: false });
    return () => wrap.removeEventListener('wheel', onWheel);
  }, [zoom, setZoomAt]);

  // pan with space + drag
  useEffect(() => {
    const wrap = wrapRef.current;
    if (!wrap) return;
    let space = false;
    let panning = false;
    let panX = 0;
    let panY = 0;
    const onKD = (e) => {
      if (e.code === 'Space' && !e.target.closest('textarea,input')) {
        e.preventDefault();
        space = true;
        wrap.classList.add('pan-ready');
      }
      if (e.key === 'Escape') {
        if (snap) setSnap(null);
        else if (lb.open) setLB({ open: false, nid: null });
      }
    };
    const onKU = (e) => {
      if (e.code === 'Space') {
        space = false;
        wrap.classList.remove('pan-ready');
      }
    };
    const onMD = (e) => {
      if ((space && e.button === 0) || e.button === 1) {
        e.preventDefault();
        panning = true;
        panX = e.clientX;
        panY = e.clientY;
        wrap.classList.add('panning');
      }
    };
    const onMM = (e) => {
      if (!panning) return;
      wrap.scrollLeft -= e.clientX - panX;
      wrap.scrollTop -= e.clientY - panY;
      panX = e.clientX;
      panY = e.clientY;
    };
    const onMU = () => {
      if (panning) {
        panning = false;
        wrap.classList.remove('panning');
      }
    };
    document.addEventListener('keydown', onKD);
    document.addEventListener('keyup', onKU);
    wrap.addEventListener('mousedown', onMD);
    window.addEventListener('mousemove', onMM);
    window.addEventListener('mouseup', onMU);
    return () => {
      document.removeEventListener('keydown', onKD);
      document.removeEventListener('keyup', onKU);
      wrap.removeEventListener('mousedown', onMD);
      window.removeEventListener('mousemove', onMM);
      window.removeEventListener('mouseup', onMU);
    };
  }, [snap, lb]);

  // ── snap region
  const startSnap = (videoEl, container, opts = {}) => {
    if (!videoEl || videoEl.readyState < 2) {
      showToast('Video not ready');
      return;
    }
    setSnap({ videoEl, container, opts });
  };

  // measure callback
  const onMeasure = useCallback((id, h) => {
    measuredHeights.current[id] = h;
  }, []);

  // ── render start node center
  let startPos = null;
  if (wizPhase === 'start' && Object.keys(nodes).length === 0) {
    const wrap = wrapRef.current;
    const vpW = (wrap?.clientWidth || vp.w) / zoom;
    const vpH = (wrap?.clientHeight || vp.h) / zoom;
    const sl = (wrap?.scrollLeft || 0) / zoom;
    const st = (wrap?.scrollTop || 0) / zoom;
    startPos = { x: vpW / 2 - NW / 2 + sl, y: vpH / 2 - 200 + st };
  }

  // ── current selected branch info for input
  const sn = selected ? nodes[selected] : null;

  // ── render
  return (
    <>
      <input
        type="file"
        id="fileInput"
        multiple
        accept="image/*,audio/*,.ttf,.otf,.woff,.woff2"
        hidden
      />
      <div className="topbar">
        <button className="topbtn" onClick={onBack} style={{ marginLeft: 0 }}>
          ← Projects
        </button>
        <div className="logo" style={{ marginLeft: 12 }}>
          {project?.name || 'MotionProp'}<span> editor</span>
        </div>
        <div className="spacer" />
        <button className="topbtn" onClick={zoomOut}>
          −
        </button>
        <span className="topbtn" style={{ cursor: 'default', minWidth: 48, textAlign: 'center' }}>
          {Math.round(zoom * 100)}%
        </span>
        <button className="topbtn" onClick={zoomIn}>
          +
        </button>
        <button className="topbtn" onClick={resetZoom}>
          Fit
        </button>
      </div>

      <div className="wrap" ref={wrapRef}>
        <div
          className="canvas"
          style={{ width: w || '100%', height: h || '100%', transform: `scale(${zoom})` }}
        >
          <Edges
            nodes={nodes}
            pos={pos}
            inputPos={inputPosFixed}
            selectedId={selected}
            w={w}
            h={h}
          />

          {wizPhase === 'start' && Object.keys(nodes).length === 0 && startPos && (
            <StartNode
              x={startPos.x}
              y={startPos.y}
              assets={wizAssets}
              onAttach={wizAttach}
              onRemove={wizRemove}
              onSubmit={wizSubmit}
            />
          )}

          {Object.values(nodes).map((n) => {
            const p = pos[n.id];
            if (!p) return null;
            if (p.collapsed) {
              const count = branchNodeIds(nodes, n.branch).length;
              const dimmed = selected && !chain.has(n.id);
              return (
                <div
                  key={n.id}
                  className={'node-pill' + (dimmed ? ' dimmed' : '')}
                  style={{ left: p.x, top: p.y }}
                  onClick={(e) => {
                    e.stopPropagation();
                    expandBranch(n.branch);
                  }}
                >
                  <div className="pill-card">
                    <div className="pill-dot" style={{ background: n.color }} />
                    <span className="pill-label">{n.branch}</span>
                    <span className="pill-count">{count}</span>
                    <span className="pill-expand">▸</span>
                  </div>
                </div>
              );
            }
            return (
              <NodeCard
                key={n.id}
                node={n}
                pos={p}
                isSelected={n.id === selected}
                isDimmed={selected && !chain.has(n.id)}
                nodes={nodes}
                collapsedBranches={collapsedBranches}
                onSelect={setSelected}
                onToggleCollapse={toggleCollapse}
                onToggleExpand={toggleExpand}
                onToggleAct={toggleAct}
                onApprovePlan={() => {}}
                onApproveScript={() => {}}
                onOpenLB={(nid) => setLB({ open: true, nid })}
                onSnap={(nid, vidEl) => startSnap(vidEl, vidEl?.closest('.nthumb'))}
                onMeasure={onMeasure}
                wizPhase={wizPhase}
              />
            );
          })}

          {selected && inputPosFixed && sn && sn.role === 'ai' && (
            <InputNode
              x={inputPosFixed.x}
              y={inputPosFixed.y}
              branch={sn.branch}
              color={sn.color}
              assets={pendingAssets}
              onAttach={attachPending}
              onRemoveAsset={removePending}
              onSend={send}
              disabled={Object.values(nodes).some(
                (n) => n.activities && n.activities.some((a) => a.status === 'in_progress' || a.status === 'pending')
              )}
            />
          )}
        </div>
      </div>

      <Lightbox
        open={lb.open}
        node={lb.nid ? nodes[lb.nid] : null}
        onClose={() => setLB({ open: false, nid: null })}
        onSnap={(vidEl) => {
          if (!vidEl) return;
          startSnap(vidEl, vidEl.closest('.lb-video'), { closeLB: true });
        }}
      />

      {snap && (
        <SnapOverlay
          videoEl={snap.videoEl}
          container={snap.container}
          zoom={zoom}
          onCancel={() => setSnap(null)}
          onCapture={(dataUrl) => {
            setPendingAssets((p) => [...p, { name: 'snap-' + Date.now() + '.png', type: 'image', dataUrl }]);
            if (snap.opts?.closeLB) setLB({ open: false, nid: null });
            setSnap(null);
            showToast('Region captured');
          }}
          onFail={() => {
            setSnap(null);
            showToast('Capture failed');
          }}
        />
      )}

      <div className={'toast' + (toast ? ' show' : '')}>{toast}</div>
    </>
  );
}

// ── Snap region overlay (portal-style: appended to container)
function SnapOverlay({ videoEl, container, zoom, onCancel, onCapture, onFail }) {
  const overlayRef = useRef(null);

  useEffect(() => {
    if (!container) {
      onCancel();
      return;
    }
    videoEl.pause();
    const overlay = document.createElement('div');
    overlay.className = 'snap-overlay';
    overlay.innerHTML = '<div class="snap-hint">Drag to select region · Esc to cancel</div>';
    const prevOverflow = container.style.overflow;
    container.style.overflow = 'visible';
    container.appendChild(overlay);
    overlayRef.current = overlay;

    let rect = null;
    let startX = 0;
    let startY = 0;

    const finish = (sel) => {
      overlay.remove();
      container.style.overflow = prevOverflow;
      if (!sel) {
        onCancel();
        return;
      }
      const or = container.getBoundingClientRect();
      const vw = videoEl.videoWidth;
      const vh = videoEl.videoHeight;
      const scaleX = vw / or.width;
      const scaleY = vh / or.height;
      const c = document.createElement('canvas');
      c.width = Math.round(sel.w * scaleX);
      c.height = Math.round(sel.h * scaleY);
      if (c.width < 1 || c.height < 1) {
        onCancel();
        return;
      }
      try {
        c.getContext('2d').drawImage(
          videoEl,
          sel.x * scaleX,
          sel.y * scaleY,
          sel.w * scaleX,
          sel.h * scaleY,
          0,
          0,
          c.width,
          c.height
        );
        const dataUrl = c.toDataURL('image/png');
        onCapture(dataUrl);
      } catch (e) {
        onFail();
      }
    };

    const onDown = (e) => {
      e.stopPropagation();
      e.preventDefault();
      const r = container.getBoundingClientRect();
      startX = (e.clientX - r.left) / zoom;
      startY = (e.clientY - r.top) / zoom;
      if (rect) rect.remove();
      rect = document.createElement('div');
      rect.className = 'snap-rect';
      overlay.appendChild(rect);
      overlay.querySelector('.snap-hint')?.remove();

      const onMove = (e) => {
        const r = container.getBoundingClientRect();
        const cx = (e.clientX - r.left) / zoom;
        const cy = (e.clientY - r.top) / zoom;
        const x = Math.min(startX, cx);
        const y = Math.min(startY, cy);
        const w = Math.abs(cx - startX);
        const h = Math.abs(cy - startY);
        if (rect) rect.style.cssText = `left:${x}px;top:${y}px;width:${w}px;height:${h}px`;
      };
      const onUp = (e) => {
        window.removeEventListener('mousemove', onMove);
        window.removeEventListener('mouseup', onUp);
        const r = container.getBoundingClientRect();
        const cx = (e.clientX - r.left) / zoom;
        const cy = (e.clientY - r.top) / zoom;
        const x = Math.min(startX, cx);
        const y = Math.min(startY, cy);
        const w = Math.abs(cx - startX);
        const h = Math.abs(cy - startY);
        if (w < 5 || h < 5) {
          finish(null);
          return;
        }
        finish({ x, y, w, h });
      };
      window.addEventListener('mousemove', onMove);
      window.addEventListener('mouseup', onUp);
    };

    overlay.addEventListener('mousedown', onDown);
    return () => {
      overlay.removeEventListener('mousedown', onDown);
      if (overlay.parentElement) overlay.remove();
      if (container) container.style.overflow = prevOverflow;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return null;
}
