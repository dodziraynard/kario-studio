export const API_BASE = '';
export const renderUrl = (path) => {
  if (!path) return null;
  const p = path.replace(/^\//, '').replace(/\/$/, '');
  const full = `/${p}`;
  return full.endsWith('.mp4') ? full : `${full}/output.mp4`;
};
