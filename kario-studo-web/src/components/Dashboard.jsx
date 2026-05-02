import React from 'react';

const STATUS_COLORS = {
  done: '#34d399',
  completed: '#34d399',
  success: '#34d399',
  running: '#fbbf24',
  pending: '#fbbf24',
  queued: '#fbbf24',
  failed: '#ef4444',
  error: '#ef4444',
};

export default function Dashboard({
  projects,
  loading,
  error,
  onRefresh,
  onOpen,
  onCreate,
  onDelete,
  onRename,
}) {
  return (
    <div className="dash">
      <div className="dash-topbar">
        <div className="logo">
          MotionProp<span> studio</span>
        </div>
        <div className="spacer" />
        {onRefresh && (
          <button className="topbtn" onClick={onRefresh} style={{ marginRight: 8 }}>
            {loading ? 'Refreshing…' : '↻ Refresh'}
          </button>
        )}
        <button className="dash-new" onClick={onCreate}>
          + New project
        </button>
      </div>

      <div className="dash-body">
        <div className="dash-title">
          Projects
          {error && <span className="dash-error"> · {error}</span>}
        </div>

        {loading && projects.length === 0 ? (
          <div className="dash-empty">
            <div className="dash-empty-sub">Loading projects…</div>
          </div>
        ) : projects.length === 0 ? (
          <div className="dash-empty">
            <div className="dash-empty-title">No projects yet</div>
            <div className="dash-empty-sub">Create your first MotionProp project to get started.</div>
            <button className="dash-new" onClick={onCreate} style={{ marginTop: 16 }}>
              + New project
            </button>
          </div>
        ) : (
          <div className="dash-grid">
            {projects.map((p) => {
              const nodeCount = p.nodes ? Object.keys(p.nodes).length : 0;
              const status = p.latestStatus || null;
              const statusColor = STATUS_COLORS[status] || '#a1a1aa';
              const renderUrl = p.latestRenderUrl;
              return (
                <div className="dash-card" key={p.id} onClick={() => onOpen(p.id)}>
                  <div className="dash-card-thumb">
                    {renderUrl ? (
                      <video
                        src={renderUrl}
                        loop
                        playsInline
                        preload="metadata"
                        className="dash-card-video"
                        onMouseEnter={(e) => e.currentTarget.play().catch(() => {})}
                        onMouseLeave={(e) => {
                          e.currentTarget.pause();
                          e.currentTarget.currentTime = 0;
                        }}
                      />
                    ) : (
                      <>
                        <div className="dash-card-thumb-text">
                          {(p.name || 'Untitled').toUpperCase()}
                        </div>
                        <div className="dash-card-thumb-sub">MOTIONPROP</div>
                      </>
                    )}
                    {!p.server && <div className="dash-card-badge">LOCAL</div>}
                  </div>
                  <div className="dash-card-body">
                    <input
                      className="dash-card-name"
                      defaultValue={p.name || 'Untitled'}
                      onClick={(e) => e.stopPropagation()}
                      onBlur={(e) => onRename(p.id, e.target.value.trim() || 'Untitled')}
                      onKeyDown={(e) => {
                        if (e.key === 'Enter') e.target.blur();
                      }}
                    />
                    <div className="dash-card-meta">
                      {status && (
                        <>
                          <span className="dash-status">
                            <span
                              className="dash-status-dot"
                              style={{ background: statusColor }}
                            />
                            {status}
                          </span>
                          <span>·</span>
                        </>
                      )}
                      {p.jobCount != null ? (
                        <span>
                          {p.jobCount} job{p.jobCount === 1 ? '' : 's'}
                        </span>
                      ) : (
                        <span>
                          {nodeCount} node{nodeCount === 1 ? '' : 's'}
                        </span>
                      )}
                      <span>·</span>
                      <span>{new Date(p.updatedAt || p.createdAt).toLocaleDateString()}</span>
                    </div>
                  </div>
                  <button
                    className="dash-card-del"
                    title="Delete"
                    onClick={(e) => {
                      e.stopPropagation();
                      if (confirm(`Delete "${p.name}" and all its jobs? This cannot be undone.`)) onDelete(p.id);
                    }}
                  >
                    ✕
                  </button>
                </div>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
}
