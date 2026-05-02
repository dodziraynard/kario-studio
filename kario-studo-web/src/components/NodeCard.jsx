import React, { useEffect, useRef, useState } from 'react';
import { ft, fmtTime } from '../lib/utils.js';
import { getChildrenOf } from '../lib/layout.js';

export default function NodeCard({
  node,
  pos,
  isSelected,
  isDimmed,
  nodes,
  collapsedBranches,
  onSelect,
  onToggleCollapse,
  onToggleExpand,
  onToggleAct,
  onApprovePlan,
  onApproveScript,
  onOpenLB,
  onSnap,
  onMeasure,
  wizPhase,
  cardRef,
}) {
  const videoRef = useRef(null);
  const [vidTime, setVidTime] = useState(0);
  const [vidDur, setVidDur] = useState(0);
  const [playing, setPlaying] = useState(false);
  const elRef = useRef(null);

  const kids = getChildrenOf(nodes, node.id);
  const isLeaf = kids.length === 0;
  const isBP = kids.length > 0;
  const isTextOnly = node.role === 'ai' && !node.vs;
  const isCollapsedContent = !!node._collapsed;

  const branchKids = kids.filter((c) => c.branch !== node.branch);
  const hasBranchKids = branchKids.length > 0;

  const isLoading = node.activities && node.activities.some((a) => a.status !== 'completed');

  useEffect(() => {
    const v = videoRef.current;
    if (!v) return;
    const onMeta = () => setVidDur(v.duration);
    const onTime = () => setVidTime(v.currentTime);
    v.addEventListener('loadedmetadata', onMeta);
    v.addEventListener('timeupdate', onTime);
    return () => {
      v.removeEventListener('loadedmetadata', onMeta);
      v.removeEventListener('timeupdate', onTime);
    };
  }, [node.vs]);

  // notify parent of measured height
  useEffect(() => {
    if (!elRef.current || !onMeasure) return;
    const ro = new ResizeObserver(() => {
      onMeasure(node.id, elRef.current.getBoundingClientRect().height);
    });
    ro.observe(elRef.current);
    onMeasure(node.id, elRef.current.getBoundingClientRect().height);
    return () => ro.disconnect();
  });

  const togglePlay = (e) => {
    e.stopPropagation();
    const v = videoRef.current;
    if (!v) return;
    if (v.paused) {
      v.play();
      setPlaying(true);
    } else {
      v.pause();
      setPlaying(false);
    }
  };

  const seek = (e) => {
    e.stopPropagation();
    const v = videoRef.current;
    if (!v || !v.duration) return;
    const r = e.currentTarget.getBoundingClientRect();
    v.currentTime = ((e.clientX - r.left) / r.width) * v.duration;
  };

  const cls =
    'node' +
    (node.role === 'user' ? ' user' : '') +
    (isSelected ? ' sel' : '') +
    (isBP ? ' branch-point' : '') +
    (isLeaf ? ' leaf' : '') +
    (isTextOnly ? ' textonly' : '') +
    (isDimmed ? ' dimmed' : '') +
    (isCollapsedContent ? ' collapsed-content' : '');

  const handleClick = (e) => {
    if (
      e.target.closest('.nbtn') ||
      e.target.closest('.ncollapse') ||
      e.target.closest('.nplay') ||
      e.target.closest('.nexpand') ||
      e.target.closest('.napprove') ||
      e.target.closest('.nmore') ||
      e.target.closest('.act-summary')
    )
      return;
    onSelect(node.id);
  };

  return (
    <div
      ref={(el) => {
        elRef.current = el;
        if (cardRef) cardRef.current = el;
      }}
      className={cls}
      style={{ left: pos.x, top: pos.y, width: pos.w }}
      data-nid={node.id}
      onClick={handleClick}
    >
      <div className="ncard">
        {!isLoading && node.role === 'ai' && node.vs && (
          <div className="nthumb">
            <video
              ref={videoRef}
              src={node.videoUrl || '/video.mp4'}
              loop
              playsInline
              preload="metadata"
              data-nid={node.id}
            />
            <button
              className="nexpand"
              onClick={(e) => {
                e.stopPropagation();
                onOpenLB(node.id);
              }}
              title="Enlarge"
            >
              ⛶
            </button>
            <div className="nplay">
              <button className="npbtn" onClick={togglePlay}>
                {playing ? '⏸' : '▶'}
              </button>
              <button
                className="nsnap"
                onClick={(e) => {
                  e.stopPropagation();
                  onSnap(node.id, videoRef.current);
                }}
              >
                Snap
              </button>
              <div className="nptl" onClick={seek}>
                <div
                  className="npprog"
                  style={{ width: vidDur ? (vidTime / vidDur) * 100 + '%' : '0%' }}
                />
              </div>
              <span className="nptime">
                {fmtTime(vidTime)}
                {vidDur ? ' / ' + fmtTime(vidDur) : ''}
              </span>
            </div>
          </div>
        )}

        <div className="nbody">
          <div className="nrole">
            <span className="nrdot" style={{ background: node.color }} />
            {node.role === 'user' ? 'You' : 'MotionProp'}
          </div>
        </div>

        {node.activities && node.activities.length > 0 && <ActivityBlock node={node} onToggleAct={onToggleAct} />}

        {!isLoading && (
          <div className="nbody" style={{ paddingTop: 0 }}>
            <div className="ntext">{node.content}</div>
          </div>
        )}

        {!isLoading && isTextOnly && node.content.length > 120 && (
          <button className="nmore" onClick={() => onToggleExpand(node.id)}>
            {isCollapsedContent ? 'Show more' : 'Show less'}
          </button>
        )}

        {!isLoading && node.assets && node.assets.length > 0 && (
          <div className="msg-assets">
            {node.assets.map((a, i) => (
              <div className="msg-asset" key={i}>
                {a.type === 'image' ? (
                  <img src={a.dataUrl} alt="" />
                ) : (
                  <div className="aicon">{a.type === 'audio' ? '♪' : 'Aa'}</div>
                )}
                <span>{a.name}</span>
              </div>
            ))}
          </div>
        )}

        <div className="nfoot">
          <span className="ntime">{ft(node.ts)}</span>
          <span className="nspacer" />
          {isBP && (
            <div className="nbadge">
              <span className="nbadge-dot" />
              {kids.length} branch{kids.length > 1 ? 'es' : ''}
            </div>
          )}
          {hasBranchKids &&
            branchKids.map((c) => (
              <button
                key={c.branch}
                className="ncollapse"
                onClick={(e) => {
                  e.stopPropagation();
                  onToggleCollapse(c.branch);
                }}
              >
                {collapsedBranches.has(c.branch) ? 'Show' : 'Hide'} {c.branch}
              </button>
            ))}
          {!isLoading && isTextOnly && wizPhase === 'plan' && (
            <button
              className="napprove"
              onClick={(e) => {
                e.stopPropagation();
                onApprovePlan();
              }}
            >
              Approve plan
            </button>
          )}
          {!isLoading && isTextOnly && wizPhase === 'script' && (
            <button
              className="napprove"
              onClick={(e) => {
                e.stopPropagation();
                onApproveScript();
              }}
            >
              Approve script
            </button>
          )}
        </div>
      </div>
    </div>
  );
}

function ActivityBlock({ node, onToggleAct }) {
  const allDone = node.activities.every((a) => a.status === 'completed');
  if (allDone) {
    const totalMs = node.activities.reduce(
      (s, a) => s + (a.completedAt && a.startedAt ? a.completedAt - a.startedAt : 0),
      0
    );
    const totalSec = (totalMs / 1000).toFixed(1);
    const isExp = !!node._actExpanded;
    return (
      <div className={'nactivity act-collapsed' + (isExp ? ' act-expanded' : '')}>
        <div
          className="act-summary"
          onClick={(e) => {
            e.stopPropagation();
            onToggleAct(node.id);
          }}
        >
          <span className="act-summary-icon">✓</span> Completed in {totalSec}s
          <span className="act-expand-hint">›</span>
        </div>
        <div className="act-detail-list">
          {node.activities.map((a, i) => (
            <div className="act-step completed" key={i}>
              <div className="act-icon">✓</div>
              <div className="act-label">{a.label}</div>
              <span className="act-elapsed">{a.completedAt - a.startedAt}ms</span>
            </div>
          ))}
        </div>
      </div>
    );
  }
  return (
    <div className="nactivity">
      {node.activities.map((a, i) => {
        const icon =
          a.status === 'completed' ? '✓' : a.status === 'in_progress' ? '●' : a.status === 'failed' ? '✕' : '';
        return (
          <div className={'act-step ' + a.status} key={i}>
            <div className="act-icon">{icon}</div>
            <div className="act-label">{a.label}</div>
          </div>
        );
      })}
    </div>
  );
}
