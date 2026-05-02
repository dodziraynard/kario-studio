import React, { useEffect, useRef, useState } from 'react';
import { fmtTime } from '../lib/utils.js';

export default function Lightbox({ open, node, onClose, onSnap }) {
  const vidRef = useRef(null);
  const [t, setT] = useState(0);
  const [d, setD] = useState(0);
  const [playing, setPlaying] = useState(false);

  useEffect(() => {
    if (!open) {
      const v = vidRef.current;
      if (v) v.pause();
      setPlaying(false);
      return;
    }
  }, [open]);

  const togglePlay = (e) => {
    e.stopPropagation();
    const v = vidRef.current;
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
    const v = vidRef.current;
    if (!v || !v.duration) return;
    const r = e.currentTarget.getBoundingClientRect();
    v.currentTime = ((e.clientX - r.left) / r.width) * v.duration;
  };

  return (
    <div
      className={'lightbox' + (open ? ' show' : '')}
      onClick={(e) => {
        if (e.target.classList.contains('lightbox')) onClose();
      }}
    >
      <button className="lb-close" onClick={onClose}>
        ✕
      </button>
      <div className="lb-video" onClick={(e) => e.stopPropagation()}>
        {open && (
          <video
            ref={vidRef}
            src="/video.mp4"
            loop
            playsInline
            preload="metadata"
            onLoadedMetadata={(e) => setD(e.currentTarget.duration)}
            onTimeUpdate={(e) => setT(e.currentTarget.currentTime)}
          />
        )}
      </div>
      <div className="lb-ctrl" onClick={(e) => e.stopPropagation()}>
        <button className="vcb" onClick={togglePlay}>
          {playing ? '⏸' : '▶'}
        </button>
        <button className="lb-snap" onClick={() => onSnap(vidRef.current)}>
          Snap
        </button>
        <div className="vtl" onClick={seek}>
          <div className="vprog" style={{ width: d ? (t / d) * 100 + '%' : '0%' }} />
        </div>
        <span className="vctime">
          {fmtTime(t)} / {fmtTime(d)}
        </span>
      </div>
      <div className="lb-info">{node?.content || ''}</div>
    </div>
  );
}
