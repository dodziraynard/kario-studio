import React, { useState, useRef } from 'react';

export default function StartNode({ x, y, assets, onAttach, onRemove, onSubmit }) {
  const [text, setText] = useState('');
  const taRef = useRef(null);
  const cardRef = useRef(null);

  React.useEffect(() => {
    requestAnimationFrame(() => taRef.current?.focus());
  }, []);

  const handleKey = (e) => {
    if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      onSubmit(text);
    }
  };

  const handleDragOver = (e) => {
    e.preventDefault();
    cardRef.current.style.borderColor = 'rgba(167,139,250,.5)';
  };
  const handleDragLeave = () => {
    cardRef.current.style.borderColor = '';
  };
  const handleDrop = (e) => {
    e.preventDefault();
    cardRef.current.style.borderColor = '';
    onAttach(e.dataTransfer.files);
  };

  return (
    <div className="start-node" style={{ left: x, top: y, width: 580 }}>
      <div
        className="start-card"
        ref={cardRef}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
      >
        <div className="start-header">
          <div className="start-logo">
            MotionProp<span> studio</span>
          </div>
          <div className="start-sub">Describe your video. Attach assets. Choose your mode.</div>
        </div>
        <div className="start-body">
          <textarea
            ref={taRef}
            rows={4}
            placeholder="e.g. A 30-second product launch intro with cinematic particle effects..."
            value={text}
            onChange={(e) => setText(e.target.value)}
            onKeyDown={handleKey}
          />
          {assets.length > 0 && (
            <div className="start-assets">
              {assets.map((a, i) => (
                <div className="asset-chip" key={i}>
                  {a.type === 'image' ? (
                    <img src={a.dataUrl} alt="" />
                  ) : (
                    <div className="aicon">{a.type === 'audio' ? '♪' : 'Aa'}</div>
                  )}
                  <span className="aname">{a.name}</span>
                  <button className="ax" onClick={() => onRemove(i)}>
                    ×
                  </button>
                </div>
              ))}
            </div>
          )}
        </div>
        <div className="start-foot">
          <label className="start-attach">
            + Attach
            <input
              type="file"
              multiple
              accept="image/*,audio/*,.ttf,.otf,.woff,.woff2"
              hidden
              onChange={(e) => {
                onAttach(e.target.files);
                e.target.value = '';
              }}
            />
          </label>
          <div className="start-spacer" />
          <button className="start-go" onClick={() => onSubmit(text)}>
            Go
          </button>
        </div>
      </div>
    </div>
  );
}
