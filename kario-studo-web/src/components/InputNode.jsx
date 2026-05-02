import React, { useEffect, useRef, useState } from 'react';

export default function InputNode({ x, y, branch, color, assets, onRemoveAsset, onAttach, onSend, disabled }) {
  const [text, setText] = useState('');
  const taRef = useRef(null);
  const cardRef = useRef(null);
  const [drag, setDrag] = useState(false);

  useEffect(() => {
    requestAnimationFrame(() => taRef.current?.focus());
  }, [x, y]);

  const handleKey = (e) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      if (!disabled) { onSend(text); setText(''); }
    }
  };

  const onInput = (e) => {
    setText(e.target.value);
    e.target.style.height = 'auto';
    e.target.style.height = Math.min(e.target.scrollHeight, 80) + 'px';
  };

  return (
    <div className="input-node" style={{ left: x, top: y, width: 580, opacity: disabled ? 0.45 : 1, pointerEvents: disabled ? 'none' : 'auto' }}>
      <div
        ref={cardRef}
        className={'input-card' + (drag ? ' dragover' : '')}
        onDragOver={(e) => {
          e.preventDefault();
          setDrag(true);
        }}
        onDragLeave={() => setDrag(false)}
        onDrop={(e) => {
          e.preventDefault();
          setDrag(false);
          onAttach(e.dataTransfer.files);
        }}
      >
        <div className="input-head">
          <span className="nrdot" style={{ background: color }} />
          {branch}
        </div>
        {assets.length > 0 && (
          <div className="input-assets">
            {assets.map((a, i) => (
              <div className="asset-chip" key={i}>
                {a.type === 'image' ? (
                  <img src={a.dataUrl} alt="" />
                ) : (
                  <div className="aicon">{a.type === 'audio' ? '♪' : 'Aa'}</div>
                )}
                <span className="aname">{a.name}</span>
                <button className="ax" onClick={() => onRemoveAsset(i)}>
                  ×
                </button>
              </div>
            ))}
          </div>
        )}
        <div className="input-body">
          <textarea
            ref={taRef}
            rows={2}
            placeholder="Continue from here..."
            value={text}
            onChange={onInput}
            onKeyDown={handleKey}
          />
        </div>
        <div className="input-foot">
          <span className="input-hint">↵ send</span>
          <label className="attach-btn">
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
          <button
            className="send-btn"
            disabled={disabled}
            onClick={() => {
              if (!disabled) { onSend(text); setText(''); }
            }}
          >
            {disabled ? 'Generating…' : 'Send'}
          </button>
        </div>
      </div>
    </div>
  );
}
