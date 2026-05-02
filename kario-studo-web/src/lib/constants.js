export const COLORS = ['#a78bfa', '#34d399', '#f472b6', '#fbbf24', '#38bdf8', '#fb923c'];

export const VS = [
  { text: 'MOTION PROP', sub: 'TITLE SEQUENCE', bg: ['#0a0a1a', '#1a0a2e', '#0a1a2e'], pc: 'blue' },
  { text: 'MOTION PROP', sub: 'NEON GLOW', bg: ['#1a0505', '#2e0808', '#1a0518'], pc: 'red' },
  { text: 'FAST CUT', sub: 'HIGH ENERGY', bg: ['#051a05', '#082e08', '#051a0e'], pc: 'green' },
  { text: 'MOTION', sub: 'MINIMAL', bg: ['#060606', '#0d0d0d', '#060610'], pc: 'white' },
  { text: 'PROP MOTION', sub: 'REVERSED', bg: ['#051a2e', '#1a051a', '#2e1a05'], pc: 'purple' },
];

export const PC = {
  blue: ['#a78bfa', '#7c3aed', '#6366f1'],
  red: ['#f472b6', '#fb923c', '#e11d48'],
  green: ['#34d399', '#22c55e', '#10b981'],
  gold: ['#fbbf24', '#f59e0b'],
  white: ['rgba(255,255,255,.35)', 'rgba(255,255,255,.18)'],
  purple: ['#c084fc', '#a855f7'],
};

export const REPLIES = [
  (t) => `Applied "${t}" with updated keyframes and motion.`,
  (t) => `Style adjusted for "${t}".`,
  () => `Rendered with updated timing, color, and dynamics.`,
];

export const TEXT_REPLIES = {
  question: [
    (t) => `The current composition uses a 12-second timeline at 24fps. The particle system has ~200 elements with ease-in-out curves. "${t}" — happy to adjust any of these parameters.`,
    (t) => `Good question. Right now the color palette is derived from the base gradient. If you want to change "${t}", I'd suggest trying a branch to compare.`,
    () => `The video is set to 1920×1080 at 24fps, 12 seconds. The text uses Inter Bold for titles and Inter Regular for subtitles. All animations use cubic-bezier easing.`,
  ],
  status: [
    (t) => `Analyzing "${t}"...\n\nBreaking this into steps:\n1. Adjusting color channels\n2. Recalculating particle trajectories\n3. Re-rendering affected keyframes\n\nThis will be reflected in the next video node.`,
    (t) => `Working on "${t}".\n\nChanges queued:\n→ Updated motion curves\n→ Adjusted timing offsets\n→ Recalculated blend modes\n\nSend another prompt to generate the updated render.`,
    (t) => `Processing "${t}".\n\nI'll need a few parameters to get this right. Try being more specific about:\n- Timing (which part of the video)\n- Intensity (subtle vs dramatic)\n- Style reference (if any)`,
  ],
  feedback: [
    (t) => `Noted: "${t}". I've logged this as a constraint for future renders on this branch. It won't change the current output but will influence all subsequent generations.`,
    (t) => `Got it. "${t}" — this kind of creative direction is best applied by sending a follow-up prompt with the specific edit. I'll factor it into the next render.`,
  ],
};

export const EDIT_ACTIVITIES = [
  { label: 'Analyzing prompt', dur: 400 },
  { label: 'Generating keyframes', dur: 600 },
  { label: 'Adjusting color palette', dur: 350 },
  { label: 'Computing particle dynamics', dur: 500 },
  { label: 'Rendering composition', dur: 700 },
];

export const PLAN_ACTIVITIES = [
  { label: 'Parsing request', dur: 300 },
  { label: 'Breaking down structure', dur: 500 },
  { label: 'Generating timeline', dur: 400 },
  { label: 'Writing plan', dur: 600 },
];

export const SCRIPT_ACTIVITIES = [
  { label: 'Analyzing plan', dur: 300 },
  { label: 'Generating motion scripts', dur: 600 },
  { label: 'Setting keyframe params', dur: 400 },
];

export const NW = 580;
export const GX = 72;
export const GY = 48;
export const INPUT_H = 140;
