// agentic_hackathon.rs — Promo video for The Agentic Evolution Hackathon
// MongoDB × Cerebral Valley · London · May 2 2026
// 1280×720 · ~51 s · Dark palette · VO-driven motion graphics.
//
// Design principle: VISUAL FIRST. The voice-over carries the words; the canvas
// carries the meaning through metaphors, paths, and motion. Text on screen is
// minimal — usually one or two words at a time when it really lands.

use kario_base::{
    Asset, Audio, AudioAsset, Clip, Composition, Duration, Id, Iris, MaskMode, Ripple, Scene,
    Swipe, ZoomThrough,
    animations::{AnimatedProperty, Easing},
    layers::Instance,
    styles::{Color, Paint},
};

// ── Canvas / timing ──────────────────────────────────────────────────────────
const W: f32 = 1280.0;
const H: f32 = 720.0;
const VO_DUR: f32 = 49.143;
const SCENE_DUR: f32 = 51.0;
const VO_SRC: &str = "crates/base/src/assets/agentic_hackathon_vo.mp3";
const SFX_WHOOSH: &str = "crates/base/src/assets/whoosh.mp3";
const SFX_CLICK: &str = "crates/base/src/assets/universal-click.mp3";
const BG_MUSIC: &str =
    "crates/base/src/stock-assets/nastelbom-upbeat-upbeat-background.mp3";

const SCRIPT: &str = "Something is shifting in AI. \
Agents aren't waiting to be told what to do anymore. They're remembering. Deciding. \
Getting better on their own. And somebody has to build the infrastructure for that world. \
Why not you? The Agentic Evolution Hackathon. One day in London to architect the memory \
systems, integrations, and self-evolution engines that turn agents into genuine partners. \
Hosted by MongoDB and Cerebral Valley. May second. Fifteen thousand pounds in prizes. \
A month of residency at London Founder House. And the chance to demo live at MongoDB.local. \
Teams of up to four. Space is limited. Apply now. \
One day. One idea. Let's see what you build.";

const FINAL_OUT_T: f32 = 50.10;
const FINAL_OUT_DUR: f32 = 0.90;

// ── Dark palette ──────────────────────────────────────────────────────────────
const HEX_BG: &str = "#070C18";
const HEX_BORDER: &str = "#1E293B";
const HEX_INK: &str = "#F8FAFC";
const HEX_INK2: &str = "#CBD5E1";
const HEX_MUTED: &str = "#64748B";
const HEX_PRIMARY: &str = "#6366F1";
const HEX_VIOLET: &str = "#8B5CF6";
const HEX_EMERALD: &str = "#10B981";
const HEX_AMBER: &str = "#F59E0B";

// ── SVG metaphor library ──────────────────────────────────────────────────────
fn svg_zap(c: &str) -> String {
    format!(r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" stroke="{c}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>"##)
}
fn svg_arrow_right(c: &str) -> String {
    format!(r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M5 12h14M12 5l7 7-7 7" stroke="{c}" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/></svg>"##)
}
// Memory: concentric expanding rings ("memory layers")
fn svg_memory(c: &str) -> String {
    format!(r##"<svg viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg" fill="none"><circle cx="32" cy="32" r="6" fill="{c}"/><circle cx="32" cy="32" r="14" stroke="{c}" stroke-width="2.5"/><circle cx="32" cy="32" r="22" stroke="{c}" stroke-width="2" opacity="0.7"/><circle cx="32" cy="32" r="30" stroke="{c}" stroke-width="1.5" stroke-dasharray="3 4" opacity="0.45"/></svg>"##)
}
// Decision: branching fork (one path → two paths)
fn svg_branch(c: &str) -> String {
    format!(r##"<svg viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M32 56V36" stroke="{c}" stroke-width="3.5" stroke-linecap="round"/><path d="M32 36C32 24, 18 24, 12 14" stroke="{c}" stroke-width="3.5" stroke-linecap="round"/><path d="M32 36C32 24, 46 24, 52 14" stroke="{c}" stroke-width="3.5" stroke-linecap="round"/><circle cx="12" cy="14" r="5" fill="{c}"/><circle cx="52" cy="14" r="5" fill="{c}"/><circle cx="32" cy="56" r="5" fill="{c}"/></svg>"##)
}
// Growth: rising curve with arrow + ascending dots
fn svg_growth(c: &str) -> String {
    format!(r##"<svg viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M6 52C18 48 26 40 34 28C40 19 46 12 56 8" stroke="{c}" stroke-width="3.5" stroke-linecap="round"/><path d="M44 8L56 8L56 20" stroke="{c}" stroke-width="3.5" stroke-linecap="round" stroke-linejoin="round"/><circle cx="12" cy="50" r="3" fill="{c}" opacity="0.5"/><circle cx="22" cy="44" r="3" fill="{c}" opacity="0.65"/><circle cx="32" cy="32" r="3" fill="{c}" opacity="0.85"/><circle cx="44" cy="18" r="3" fill="{c}"/></svg>"##)
}
// Network/integrations: central hub + 5 satellites with ring connections
fn svg_network(c: &str) -> String {
    format!(r##"<svg viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M40 40 L40 12 M40 40 L66 28 M40 40 L60 66 M40 40 L20 66 M40 40 L14 28" stroke="{c}" stroke-width="2.4" stroke-linecap="round"/><path d="M40 12 L66 28 L60 66 L20 66 L14 28 Z" stroke="{c}" stroke-width="1.6" stroke-linecap="round" opacity="0.55"/><circle cx="40" cy="12" r="5.5" fill="{c}"/><circle cx="66" cy="28" r="5.5" fill="{c}"/><circle cx="60" cy="66" r="5.5" fill="{c}"/><circle cx="20" cy="66" r="5.5" fill="{c}"/><circle cx="14" cy="28" r="5.5" fill="{c}"/><circle cx="40" cy="40" r="8" fill="{c}"/><circle cx="40" cy="40" r="3.5" fill="#070C18"/></svg>"##)
}
// Sparkle/evolution: 4-point star
fn svg_sparkle(c: &str) -> String {
    format!(r##"<svg viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M32 4 L36 28 L60 32 L36 36 L32 60 L28 36 L4 32 L28 28 Z" fill="{c}"/><path d="M52 8 L54 16 L62 18 L54 20 L52 28 L50 20 L42 18 L50 16 Z" fill="{c}" opacity="0.7"/></svg>"##)
}
// Trophy
fn svg_trophy(c: &str) -> String {
    format!(r##"<svg viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M16 6h32v18a16 16 0 01-32 0V6z" stroke="{c}" stroke-width="3"/><path d="M16 12H10a4 4 0 000 8h6M48 12h6a4 4 0 010 8h-6M32 40v10M22 54h20" stroke="{c}" stroke-width="3" stroke-linecap="round"/><path d="M16 6h32" stroke="{c}" stroke-width="3" stroke-linecap="round"/></svg>"##)
}
// London skyline silhouette (Big Ben + tower) — simple 2-tower
fn svg_skyline(c: &str) -> String {
    format!(r##"<svg viewBox="0 0 200 80" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M0 80 L0 60 L20 60 L20 40 L34 40 L34 56 L48 56 L48 30 L56 22 L64 30 L64 56 L82 56 L82 36 L96 36 L96 50 L120 50 L120 24 L132 12 L144 24 L144 50 L168 50 L168 44 L184 44 L184 80 Z" fill="{c}"/><circle cx="132" cy="32" r="3" fill="{c}"/></svg>"##)
}
// Hourglass — "space is limited"
fn svg_hourglass(c: &str) -> String {
    format!(r##"<svg viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M14 6h36M14 58h36" stroke="{c}" stroke-width="3" stroke-linecap="round"/><path d="M16 6c0 12 16 18 16 26s-16 14-16 26M48 6c0 12 -16 18 -16 26s16 14 16 26" stroke="{c}" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/><path d="M22 14h20M22 50h20" stroke="{c}" stroke-width="2" opacity="0.6"/></svg>"##)
}
// Person silhouette (single user, head + shoulders)
fn svg_person(c: &str) -> String {
    format!(r##"<svg viewBox="0 0 24 32" xmlns="http://www.w3.org/2000/svg" fill="none"><circle cx="12" cy="8" r="5.5" fill="{c}"/><path d="M2 32C2 22 6 18 12 18C18 18 22 22 22 32" fill="{c}"/></svg>"##)
}

// ── Palette helper ────────────────────────────────────────────────────────────
// Returns: (bg, border, ink, ink2, muted, primary, violet, emerald, amber)
fn make_colors() -> (Color, Color, Color, Color, Color, Color, Color, Color, Color) {
    (
        Color::hex(HEX_BG),
        Color::hex(HEX_BORDER),
        Color::hex(HEX_INK),
        Color::hex(HEX_INK2),
        Color::hex(HEX_MUTED),
        Color::hex(HEX_PRIMARY),
        Color::hex(HEX_VIOLET),
        Color::hex(HEX_EMERALD),
        Color::hex(HEX_AMBER),
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// ACT 1  (0.0 – 3.2 s)  "Something is shifting in AI."
//   100% visual: morphing geometric shapes settle into a single glowing core,
//   ringed by a rotating circle. Only "AI." appears, in the final beat.
// ─────────────────────────────────────────────────────────────────────────────
fn build_act1(scene: &mut Scene) -> Id {
    let comp_id = Id::new();
    let mut comp = Composition::new(W, H);
    comp.id = comp_id.clone();
    comp.duration = Duration::Seconds(SCENE_DUR);
    let cx = W * 0.5;
    let cy = H * 0.5;
    let (bg, _, ink, _, _, primary, violet, _, _) = make_colors();

    comp.build_layer().rect(W, H).fill(bg).at(0.0, 0.0).depth(0.0).add();

    let act1_out = 2.30;

    // ── Beat A (0.10 – 1.30): three offset squares "shift" inward ────────────
    // Three small primary squares slide in from three directions, rotating,
    // converging toward center — the "shift."
    let shift_specs: [(f32, f32, Color); 3] = [
        (cx - 360.0, cy - 80.0, primary),
        (cx + 280.0, cy - 60.0, violet),
        (cx + 100.0, cy + 200.0, primary),
    ];
    for (i, (sx, sy, scol)) in shift_specs.iter().enumerate() {
        let sq = comp
            .build_layer()
            .rect(56.0, 56.0)
            .corner_radius(8.0)
            .no_fill()
            .stroke(scol.with_alpha(0.85), 3.0)
            .at(*sx, *sy)
            .depth(0.10)
            .add();
        let t0 = 0.10 + i as f32 * 0.08;
        comp.animate(sq)
            .fade_in(t0, 0.18)
            .ease_out()
            .scale_from(0.0, t0, 0.34)
            .spring(420.0, 14.0)
            .kf(t0, AnimatedProperty::rotation_z(0.0))
            .kf_eased(1.20, AnimatedProperty::rotation_z(180.0), Easing::EASE_IN_OUT)
            .kf(1.10, AnimatedProperty::position(*sx, *sy))
            .kf_eased(
                1.40,
                AnimatedProperty::position(cx - 28.0, cy - 28.0),
                Easing::EASE_IN_OUT,
            )
            .kf(1.40, AnimatedProperty::opacity(1.0))
            .kf_eased(1.55, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();
    }

    // ── Beat B (1.42 – 2.55): rings draw + "AI." massive ─────────────────────
    // Outer ring draws around AI and rotates
    let ring = comp
        .build_layer()
        .circle(380.0)
        .no_fill()
        .stroke(primary.with_alpha(0.70), 3.0)
        .at(cx - 190.0, cy - 190.0)
        .depth(0.110)
        .add();
    comp.animate(ring)
        .clip_start(1.42)
        .kf(1.42, AnimatedProperty::opacity(0.0))
        .kf_eased(1.55, AnimatedProperty::opacity(1.0), Easing::EASE_OUT)
        .kf(1.42, AnimatedProperty::trim_path_end(0.0))
        .kf_eased(2.10, AnimatedProperty::trim_path_end(1.0), Easing::EASE_OUT)
        .kf(2.10, AnimatedProperty::rotation_z(0.0))
        .kf_eased(act1_out + 0.30, AnimatedProperty::rotation_z(34.0), Easing::EASE_IN_OUT)
        .kf(act1_out, AnimatedProperty::opacity(1.0))
        .kf_eased(act1_out + 0.30, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Inner halo ring (faint, counter-rotating)
    let halo = comp
        .build_layer()
        .circle(460.0)
        .no_fill()
        .stroke(violet.with_alpha(0.30), 1.5)
        .at(cx - 230.0, cy - 230.0)
        .depth(0.108)
        .add();
    comp.animate(halo)
        .clip_start(1.55)
        .kf(1.55, AnimatedProperty::opacity(0.0))
        .kf_eased(1.85, AnimatedProperty::opacity(1.0), Easing::EASE_OUT)
        .kf(1.55, AnimatedProperty::rotation_z(0.0))
        .kf_eased(act1_out + 0.30, AnimatedProperty::rotation_z(-26.0), Easing::EASE_IN_OUT)
        .kf(act1_out, AnimatedProperty::opacity(1.0))
        .kf_eased(act1_out + 0.30, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // "AI." massive
    let ai_text = comp
        .build_layer()
        .text("AI.", 168.0)
        .width(680.0)
        .height(190.0)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(ink)
        .glow(primary, 14.0)
        .at(cx - 340.0, cy - 95.0)
        .depth(0.115)
        .add();
    comp.animate(ai_text)
        .fade_in(1.42, 0.16)
        .ease_out()
        .scale_from(0.4, 1.42, 0.50)
        .spring(420.0, 13.0)
        .kf(act1_out, AnimatedProperty::opacity(1.0))
        .kf_eased(act1_out + 0.30, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    scene.assets.insert(comp_id.clone(), Asset::Composition(comp));
    comp_id
}

// ─────────────────────────────────────────────────────────────────────────────
// ACT 2  (2.4 – 10.0 s)  Three pure metaphor beats — no opening text
//   Beat A (2.50 – 5.55): horizontal "wait" bars dissolving (no text)
//   Beat B (5.80 – 6.92): MEMORY — concentric rings expanding
//   Beat C (6.95 – 8.00): DECISION — branching fork
//   Beat D (8.05 – 9.50): GROWTH — rising curve
//   Each metaphor is huge and centered. Tiny single-word label appears below.
// ─────────────────────────────────────────────────────────────────────────────
fn build_act2(scene: &mut Scene) -> Id {
    let comp_id = Id::new();
    let mut comp = Composition::new(W, H);
    comp.id = comp_id.clone();
    comp.duration = Duration::Seconds(SCENE_DUR);
    let cx = W * 0.5;
    let cy = H * 0.5;
    let (bg, _, _, _, muted, primary, violet, emerald, amber) = make_colors();

    comp.build_layer().rect(W, H).fill(bg).at(0.0, 0.0).depth(0.0).add();

    // ── Beat A (2.50 – 5.55): "agents aren't waiting to be told what to do"
    //
    // Visual metaphor: TETHER SNAP.
    //   1) An "OPERATOR" anchor (small primary square) sits at the left.
    //   2) A taut tether line stretches from the anchor across to an emerald
    //      AGENT dot at center — the agent is leashed, waiting for orders.
    //   3) The tether SNAPS mid-line: the two halves recoil back toward the
    //      anchor and the agent.
    //   4) The agent accelerates off-canvas right, leaving a trail of fading
    //      afterimages behind it.
    let beat_a_out = 5.55;

    // Eyebrow "TETHERED"
    let eyebrow = comp
        .build_layer()
        .text("TETHERED", 22.0)
        .width(700.0)
        .height(34.0)
        .bold()
        .letter_spacing(8.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(muted)
        .at(cx - 350.0, cy - 196.0)
        .depth(0.105)
        .add();
    comp.animate(eyebrow)
        .fade_in(2.50, 0.28)
        .ease_out()
        .slide_from(0.0, 12.0, 2.50, 0.32)
        .ease_out()
        .kf(3.95, AnimatedProperty::opacity(1.0))
        .kf_eased(4.20, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Bracket lines either side of eyebrow
    for (sx, slen) in [(cx - 480.0, 100.0_f32), (cx + 380.0, 100.0)] {
        let br = comp
            .build_layer()
            .line_path(0.0, 0.0, slen, 0.0)
            .stroke(muted.with_alpha(0.55), 1.5)
            .at(sx, cy - 180.0)
            .depth(0.105)
            .add();
        comp.animate(br)
            .clip_start(2.62)
            .kf(2.62, AnimatedProperty::trim_path_end(0.0))
            .kf_eased(2.95, AnimatedProperty::trim_path_end(1.0), Easing::EASE_OUT)
            .kf(2.62, AnimatedProperty::opacity(0.0))
            .kf_eased(2.72, AnimatedProperty::opacity(1.0), Easing::EASE_OUT)
            .kf(3.95, AnimatedProperty::opacity(1.0))
            .kf_eased(4.20, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();
    }

    // Geometry: anchor on the left, agent at center
    let anchor_x = cx - 380.0;
    let anchor_y = cy;
    let anchor_size = 28.0;
    let agent_x = cx + 100.0;
    let agent_y = cy;
    let agent_r = 22.0;
    // Tether endpoints (just outside each shape)
    let tether_left_x = anchor_x + anchor_size * 0.5;
    let tether_right_x = agent_x - agent_r;
    let tether_y = cy;
    let tether_len = tether_right_x - tether_left_x;
    let snap_x = tether_left_x + tether_len * 0.55;

    // Caption beneath the anchor — "OPERATOR"
    let cap_w = 200.0;
    let operator_label = comp
        .build_layer()
        .text("OPERATOR", 16.0)
        .width(cap_w)
        .height(24.0)
        .letter_spacing(4.0)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(muted)
        .at(anchor_x - cap_w * 0.5, anchor_y + 36.0)
        .depth(0.106)
        .add();
    comp.animate(operator_label)
        .fade_in(3.00, 0.24)
        .ease_out()
        .kf(4.05, AnimatedProperty::opacity(1.0))
        .kf_eased(4.40, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Caption beneath the agent — "AGENT"
    let agent_label = comp
        .build_layer()
        .text("AGENT", 16.0)
        .width(cap_w)
        .height(24.0)
        .letter_spacing(4.0)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(emerald)
        .at(agent_x - cap_w * 0.5, agent_y + 38.0)
        .depth(0.113)
        .add();
    comp.animate(agent_label)
        .fade_in(3.40, 0.24)
        .ease_out()
        // ride along when the agent breaks free
        .kf(4.32, AnimatedProperty::position(agent_x - cap_w * 0.5, agent_y + 38.0))
        .kf_eased(
            5.10,
            AnimatedProperty::position(W + 80.0 - cap_w * 0.5, agent_y + 38.0),
            Easing::EASE_IN,
        )
        .kf(4.95, AnimatedProperty::opacity(1.0))
        .kf_eased(5.20, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Anchor square — operator/instruction source
    let anchor = comp
        .build_layer()
        .rect(anchor_size, anchor_size)
        .corner_radius(4.0)
        .fill(muted.with_alpha(0.85))
        .at(anchor_x - anchor_size * 0.5, anchor_y - anchor_size * 0.5)
        .depth(0.108)
        .add();
    comp.animate(anchor)
        .fade_in(2.85, 0.20)
        .ease_out()
        .scale_from(0.0, 2.85, 0.34)
        .spring(420.0, 14.0)
        // jitter on snap, then settle
        .kf(4.05, AnimatedProperty::position(
            anchor_x - anchor_size * 0.5,
            anchor_y - anchor_size * 0.5,
        ))
        .kf_eased(
            4.12,
            AnimatedProperty::position(
                anchor_x - anchor_size * 0.5 - 6.0,
                anchor_y - anchor_size * 0.5,
            ),
            Easing::EASE_OUT,
        )
        .kf_eased(
            4.22,
            AnimatedProperty::position(
                anchor_x - anchor_size * 0.5,
                anchor_y - anchor_size * 0.5,
            ),
            Easing::EASE_OUT,
        )
        .kf(4.95, AnimatedProperty::opacity(1.0))
        .kf_eased(5.20, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Tether line — single taut line from anchor to agent. Draws on
    // (trim 0→1), holds taut, then snaps (opacity drop at 4.05s).
    let tether = comp
        .build_layer()
        .line_path(0.0, 0.0, tether_len, 0.0)
        .stroke(muted.with_alpha(0.85), 3.0)
        .at(tether_left_x, tether_y)
        .depth(0.107)
        .add();
    comp.animate(tether)
        .clip_start(3.05)
        .kf(3.05, AnimatedProperty::trim_path_end(0.0))
        .kf_eased(3.45, AnimatedProperty::trim_path_end(1.0), Easing::EASE_OUT)
        .kf(3.05, AnimatedProperty::opacity(0.0))
        .kf_eased(3.20, AnimatedProperty::opacity(0.85), Easing::EASE_OUT)
        // SNAP: line vanishes in one frame
        .kf(4.04, AnimatedProperty::opacity(0.85))
        .kf(4.05, AnimatedProperty::opacity(0.0))
        .apply();

    // Tether recoil halves — two short line segments that briefly appear at
    // the snap moment, recoiling toward each end. Implemented as two short
    // lines that fade in at snap_x and slide back.
    // Left half (recoils toward anchor)
    let recoil_left_len = tether_len * 0.45;
    let recoil_left = comp
        .build_layer()
        .line_path(0.0, 0.0, recoil_left_len, 0.0)
        .stroke(muted.with_alpha(0.85), 3.0)
        .at(tether_left_x, tether_y)
        .depth(0.108)
        .add();
    comp.animate(recoil_left)
        .clip_start(4.05)
        .kf(4.05, AnimatedProperty::opacity(0.85))
        // recoil: contract toward the anchor (scale x 1→0.2, anchor at left)
        .kf(4.05, AnimatedProperty::scale(1.0, 1.0))
        .kf_eased(4.32, AnimatedProperty::scale(0.20, 1.0), Easing::EASE_OUT)
        .kf_eased(4.45, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Right half (recoils toward agent — but agent is about to leave; just fades)
    let recoil_right_len = tether_len * 0.45;
    let recoil_right = comp
        .build_layer()
        .line_path(0.0, 0.0, recoil_right_len, 0.0)
        .stroke(muted.with_alpha(0.85), 3.0)
        .at(snap_x, tether_y)
        .depth(0.108)
        .add();
    comp.animate(recoil_right)
        .clip_start(4.05)
        .kf(4.05, AnimatedProperty::opacity(0.85))
        // contract & slide right toward where the agent was
        .kf(4.05, AnimatedProperty::position(snap_x, tether_y))
        .kf_eased(
            4.32,
            AnimatedProperty::position(snap_x + recoil_right_len * 0.6, tether_y),
            Easing::EASE_OUT,
        )
        .kf(4.05, AnimatedProperty::scale(1.0, 1.0))
        .kf_eased(4.32, AnimatedProperty::scale(0.20, 1.0), Easing::EASE_OUT)
        .kf_eased(4.45, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Snap flash — a small bright primary burst at snap_x on the snap moment
    let flash = comp
        .build_layer()
        .circle(40.0)
        .fill(primary)
        .glow(primary, 14.0)
        .at(snap_x - 20.0, tether_y - 20.0)
        .depth(0.111)
        .add();
    comp.animate(flash)
        .clip_start(4.05)
        .kf(4.05, AnimatedProperty::opacity(0.0))
        .kf_eased(4.08, AnimatedProperty::opacity(1.0), Easing::EASE_OUT)
        .kf(4.05, AnimatedProperty::scale(0.0, 0.0))
        .kf_eased(4.18, AnimatedProperty::scale(1.0, 1.0), Easing::EASE_OUT)
        .kf_eased(4.30, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Agent dot — emerald, sits at center waiting, then accelerates off-canvas
    let agent = comp
        .build_layer()
        .circle(agent_r * 2.0)
        .fill(emerald)
        .glow(emerald, 14.0)
        .at(agent_x - agent_r, agent_y - agent_r)
        .depth(0.112)
        .add();
    comp.animate(agent)
        .fade_in(3.20, 0.20)
        .ease_out()
        .scale_from(0.0, 3.20, 0.34)
        .spring(420.0, 14.0)
        // small recoil at snap, then accelerate off-canvas right
        .kf(4.05, AnimatedProperty::position(agent_x - agent_r, agent_y - agent_r))
        .kf_eased(
            4.15,
            AnimatedProperty::position(agent_x - agent_r + 14.0, agent_y - agent_r),
            Easing::EASE_OUT,
        )
        .kf(4.32, AnimatedProperty::position(agent_x - agent_r + 14.0, agent_y - agent_r))
        .kf_eased(
            5.10,
            AnimatedProperty::position(W + 80.0, agent_y - agent_r),
            Easing::EASE_IN,
        )
        .kf(4.95, AnimatedProperty::opacity(1.0))
        .kf_eased(5.20, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Motion-trail afterimages behind the agent — three smaller fading dots
    for i in 0..3 {
        let trail_scale = 0.85 - i as f32 * 0.18;
        let trail_alpha = 0.55 - i as f32 * 0.15;
        let lag = 0.05 + i as f32 * 0.04;
        let trail = comp
            .build_layer()
            .circle(agent_r * 2.0)
            .fill(emerald.with_alpha(trail_alpha))
            .at(agent_x - agent_r, agent_y - agent_r)
            .depth(0.111)
            .add();
        comp.animate(trail)
            .clip_start(4.32)
            .kf(4.32, AnimatedProperty::opacity(0.0))
            .kf(4.32 + lag, AnimatedProperty::opacity(trail_alpha))
            .kf(4.32, AnimatedProperty::scale(trail_scale, trail_scale))
            .kf(4.32, AnimatedProperty::position(agent_x - agent_r + 14.0, agent_y - agent_r))
            .kf_eased(
                5.10 + lag,
                AnimatedProperty::position(W + 80.0, agent_y - agent_r),
                Easing::EASE_IN,
            )
            .kf_eased(5.15 + lag, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();
    }

    let _ = primary;
    let _ = violet;
    let _ = amber;

    // Helper: a single-metaphor "word slam" beat with HUGE icon and tiny label
    fn build_metaphor_beat(
        comp: &mut Composition,
        cx: f32,
        cy: f32,
        word: &str,
        word_color: Color,
        svg_str: String,
        icon_size: f32,
        t_in: f32,
        t_out: f32,
    ) {
        // HUGE metaphor icon centered
        let icon = comp
            .build_layer()
            .svg(svg_str)
            .width(icon_size)
            .height(icon_size)
            .at(cx - icon_size * 0.5, cy - icon_size * 0.5 - 30.0)
            .depth(0.110)
            .add();
        comp.animate(icon)
            .fade_in(t_in, 0.18)
            .ease_out()
            .scale_from(0.4, t_in, 0.42)
            .spring(420.0, 14.0)
            .kf(t_in + 0.34, AnimatedProperty::rotation_z(-4.0))
            .kf_eased(t_in + 0.78, AnimatedProperty::rotation_z(4.0), Easing::EASE_IN_OUT)
            .kf_eased(t_out - 0.20, AnimatedProperty::rotation_z(0.0), Easing::EASE_IN_OUT)
            .kf(t_out - 0.10, AnimatedProperty::opacity(1.0))
            .kf_eased(t_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();

        // Small word label below — light, elegant, just the keyword
        let label = comp
            .build_layer()
            .text(word, 38.0)
            .width(900.0)
            .height(54.0)
            .bold()
            .letter_spacing(2.0)
            .text_align_center()
            .vertical_align_middle()
            .fill(word_color)
            .at(cx - 450.0, cy + icon_size * 0.5 - 12.0)
            .depth(0.115)
            .add();
        comp.animate(label)
            .fade_in(t_in + 0.18, 0.22)
            .ease_out()
            .slide_from(0.0, 14.0, t_in + 0.18, 0.30)
            .ease_out()
            .kf(t_out - 0.10, AnimatedProperty::opacity(1.0))
            .kf_eased(t_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();
    }

    build_metaphor_beat(
        &mut comp,
        cx,
        cy,
        "remembering",
        emerald,
        svg_memory(HEX_EMERALD),
        320.0,
        5.80,
        6.92,
    );
    build_metaphor_beat(
        &mut comp,
        cx,
        cy,
        "deciding",
        violet,
        svg_branch(HEX_VIOLET),
        320.0,
        6.95,
        8.00,
    );
    build_metaphor_beat(
        &mut comp,
        cx,
        cy,
        "evolving",
        amber,
        svg_growth(HEX_AMBER),
        320.0,
        8.05,
        9.50,
    );

    let _ = primary;
    scene.assets.insert(comp_id.clone(), Asset::Composition(comp));
    comp_id
}

// ─────────────────────────────────────────────────────────────────────────────
// ACT 3  (9.5 – 14.9 s)  Challenge → "you?"
//   Beat A: blueprint grid draws on; single building-block stack rises in
//           the center. Pure visual — no text. ("infrastructure for that world")
//   Beat B: arrow draws from above, points down at "you?" (only text in act).
// ─────────────────────────────────────────────────────────────────────────────
fn build_act3(scene: &mut Scene) -> Id {
    let comp_id = Id::new();
    let mut comp = Composition::new(W, H);
    comp.id = comp_id.clone();
    comp.duration = Duration::Seconds(SCENE_DUR);
    let cx = W * 0.5;
    let cy = H * 0.5;
    let (bg, _, ink, _, _, primary, _, _, _) = make_colors();

    comp.build_layer().rect(W, H).fill(bg).at(0.0, 0.0).depth(0.0).add();

    let beat_a_out = 13.20;
    let act3_out = 14.40;

    // Blueprint grid: 4 horizontal + 5 vertical guide lines
    let h_ys = [cy - 200.0, cy - 80.0, cy + 80.0, cy + 200.0];
    for (i, &y) in h_ys.iter().enumerate() {
        let l = comp
            .build_layer()
            .line_path(0.0, 0.0, 1100.0, 0.0)
            .stroke(primary.with_alpha(0.22), 1.0)
            .at(cx - 550.0, y)
            .depth(0.10)
            .add();
        let t0 = 9.55 + i as f32 * 0.08;
        comp.animate(l)
            .clip_start(t0)
            .kf(t0, AnimatedProperty::trim_path_end(0.0))
            .kf_eased(t0 + 0.85, AnimatedProperty::trim_path_end(1.0), Easing::EASE_OUT)
            .kf(t0, AnimatedProperty::opacity(0.0))
            .kf_eased(t0 + 0.18, AnimatedProperty::opacity(1.0), Easing::EASE_OUT)
            .kf(beat_a_out - 0.10, AnimatedProperty::opacity(1.0))
            .kf_eased(beat_a_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();
    }
    let v_xs = [cx - 440.0, cx - 220.0, cx, cx + 220.0, cx + 440.0];
    for (i, &x) in v_xs.iter().enumerate() {
        let l = comp
            .build_layer()
            .line_path(0.0, 0.0, 0.0, 480.0)
            .stroke(primary.with_alpha(0.22), 1.0)
            .at(x, cy - 240.0)
            .depth(0.10)
            .add();
        let t0 = 9.65 + i as f32 * 0.08;
        comp.animate(l)
            .clip_start(t0)
            .kf(t0, AnimatedProperty::trim_path_end(0.0))
            .kf_eased(t0 + 0.85, AnimatedProperty::trim_path_end(1.0), Easing::EASE_OUT)
            .kf(t0, AnimatedProperty::opacity(0.0))
            .kf_eased(t0 + 0.18, AnimatedProperty::opacity(1.0), Easing::EASE_OUT)
            .kf(beat_a_out - 0.10, AnimatedProperty::opacity(1.0))
            .kf_eased(beat_a_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();
    }

    // Building blocks "constructing" — 5 cubes drop in from above and stack
    let block_w = 110.0;
    let block_h = 60.0;
    let block_specs: [(f32, f32, Color); 5] = [
        (cx - block_w * 1.5, cy + 90.0, primary),
        (cx - block_w * 0.5, cy + 90.0, primary),
        (cx + block_w * 0.5, cy + 90.0, primary),
        (cx - block_w * 1.0, cy + 30.0, primary),
        (cx, cy + 30.0, primary),
    ];
    for (i, (bx, by, bcol)) in block_specs.iter().enumerate() {
        let blk = comp
            .build_layer()
            .rect(block_w - 10.0, block_h - 10.0)
            .corner_radius(6.0)
            .no_fill()
            .stroke(bcol.with_alpha(0.85), 2.5)
            .at(*bx + 5.0, *by + 5.0)
            .depth(0.105)
            .add();
        let t0 = 10.25 + i as f32 * 0.22;
        comp.animate(blk)
            .fade_in(t0, 0.20)
            .ease_out()
            .slide_from(0.0, -260.0, t0, 0.40)
            .with_easing(Easing::Spring {
                stiffness: 360.0,
                damping: 15.0,
                mass: 1.0,
            })
            .kf(beat_a_out - 0.10, AnimatedProperty::opacity(1.0))
            .kf_eased(beat_a_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();
    }

    // ── BEAT B: "YOU?" framed by angle brackets that slam in from the sides
    //
    // Two thick primary angle-brackets [ ] slide in from off-canvas L/R and
    // lock around "YOU?". A small primary tick mark snaps in above the word
    // (a punctuation accent, not a pointer) and an underline scales out from
    // center beneath. Far more confident than a generic down-arrow.

    let bracket_h = 200.0;
    let bracket_arm = 32.0;
    let bracket_thick = 8.0;
    // Vertically center the whole [you?] assembly on cy.
    let bracket_y_top = cy - bracket_h * 0.5; // -> cy - 100
    let bracket_inset = 320.0; // distance from cx to inner edge of bracket

    // Left bracket: [
    //   vertical bar
    let lb_v = comp
        .build_layer()
        .rect(bracket_thick, bracket_h)
        .corner_radius(2.0)
        .fill(primary)
        .at(cx - bracket_inset - bracket_thick, bracket_y_top)
        .depth(0.116)
        .add();
    comp.animate(lb_v)
        .fade_in(13.50, 0.16)
        .ease_out()
        .slide_from(-220.0, 0.0, 13.50, 0.40)
        .with_easing(Easing::Spring {
            stiffness: 380.0,
            damping: 16.0,
            mass: 1.0,
        })
        .kf(act3_out - 0.10, AnimatedProperty::opacity(1.0))
        .kf_eased(act3_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();
    //   top arm
    let lb_t = comp
        .build_layer()
        .rect(bracket_arm, bracket_thick)
        .corner_radius(2.0)
        .fill(primary)
        .at(cx - bracket_inset - bracket_thick, bracket_y_top)
        .depth(0.116)
        .add();
    comp.animate(lb_t)
        .fade_in(13.55, 0.16)
        .ease_out()
        .slide_from(-220.0, 0.0, 13.55, 0.40)
        .with_easing(Easing::Spring {
            stiffness: 380.0,
            damping: 16.0,
            mass: 1.0,
        })
        .kf(act3_out - 0.10, AnimatedProperty::opacity(1.0))
        .kf_eased(act3_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();
    //   bottom arm
    let lb_b = comp
        .build_layer()
        .rect(bracket_arm, bracket_thick)
        .corner_radius(2.0)
        .fill(primary)
        .at(
            cx - bracket_inset - bracket_thick,
            bracket_y_top + bracket_h - bracket_thick,
        )
        .depth(0.116)
        .add();
    comp.animate(lb_b)
        .fade_in(13.55, 0.16)
        .ease_out()
        .slide_from(-220.0, 0.0, 13.55, 0.40)
        .with_easing(Easing::Spring {
            stiffness: 380.0,
            damping: 16.0,
            mass: 1.0,
        })
        .kf(act3_out - 0.10, AnimatedProperty::opacity(1.0))
        .kf_eased(act3_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Right bracket: ]
    //   vertical bar
    let rb_v = comp
        .build_layer()
        .rect(bracket_thick, bracket_h)
        .corner_radius(2.0)
        .fill(primary)
        .at(cx + bracket_inset, bracket_y_top)
        .depth(0.116)
        .add();
    comp.animate(rb_v)
        .fade_in(13.50, 0.16)
        .ease_out()
        .slide_from(220.0, 0.0, 13.50, 0.40)
        .with_easing(Easing::Spring {
            stiffness: 380.0,
            damping: 16.0,
            mass: 1.0,
        })
        .kf(act3_out - 0.10, AnimatedProperty::opacity(1.0))
        .kf_eased(act3_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();
    //   top arm
    let rb_t = comp
        .build_layer()
        .rect(bracket_arm, bracket_thick)
        .corner_radius(2.0)
        .fill(primary)
        .at(cx + bracket_inset - bracket_arm + bracket_thick, bracket_y_top)
        .depth(0.116)
        .add();
    comp.animate(rb_t)
        .fade_in(13.55, 0.16)
        .ease_out()
        .slide_from(220.0, 0.0, 13.55, 0.40)
        .with_easing(Easing::Spring {
            stiffness: 380.0,
            damping: 16.0,
            mass: 1.0,
        })
        .kf(act3_out - 0.10, AnimatedProperty::opacity(1.0))
        .kf_eased(act3_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();
    //   bottom arm
    let rb_b = comp
        .build_layer()
        .rect(bracket_arm, bracket_thick)
        .corner_radius(2.0)
        .fill(primary)
        .at(
            cx + bracket_inset - bracket_arm + bracket_thick,
            bracket_y_top + bracket_h - bracket_thick,
        )
        .depth(0.116)
        .add();
    comp.animate(rb_b)
        .fade_in(13.55, 0.16)
        .ease_out()
        .slide_from(220.0, 0.0, 13.55, 0.40)
        .with_easing(Easing::Spring {
            stiffness: 380.0,
            damping: 16.0,
            mass: 1.0,
        })
        .kf(act3_out - 0.10, AnimatedProperty::opacity(1.0))
        .kf_eased(act3_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Tick mark above — small primary horizontal bar (punctuation accent)
    let tick = comp
        .build_layer()
        .rect(80.0, 4.0)
        .corner_radius(2.0)
        .fill(primary)
        .anchor_left()
        .at(cx - 40.0, bracket_y_top - 24.0)
        .depth(0.117)
        .add();
    comp.animate(tick)
        .clip_start(13.95)
        .kf(13.95, AnimatedProperty::opacity(0.0))
        .kf_eased(14.05, AnimatedProperty::opacity(1.0), Easing::EASE_OUT)
        .kf(13.95, AnimatedProperty::scale(0.0, 1.0))
        .kf_eased(14.20, AnimatedProperty::scale(1.0, 1.0), Easing::EASE_OUT)
        .kf(act3_out - 0.10, AnimatedProperty::opacity(1.0))
        .kf_eased(act3_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Underline beneath "you?" — scales out from center, anchored visually
    let underline = comp
        .build_layer()
        .rect(220.0, 4.0)
        .corner_radius(2.0)
        .fill(primary)
        .anchor_left()
        .at(cx - 110.0, bracket_y_top + bracket_h + 24.0)
        .depth(0.118)
        .add();
    comp.animate(underline)
        .clip_start(14.05)
        .kf(14.05, AnimatedProperty::opacity(0.0))
        .kf_eased(14.18, AnimatedProperty::opacity(1.0), Easing::EASE_OUT)
        .kf(14.05, AnimatedProperty::scale(0.0, 1.0))
        .kf_eased(14.42, AnimatedProperty::scale(1.0, 1.0), Easing::EASE_OUT)
        .kf(act3_out - 0.10, AnimatedProperty::opacity(1.0))
        .kf_eased(act3_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // "you?" massive — only text in the act
    // Note: text_align_center is unreliable; the renderer left-aligns glyphs
    // from the box's `at` x. So we size the box to the actual glyph width
    // (~340 px for "you?" at 156 px bold) and place at(cx - width/2).
    let you_w = 340.0;
    let you_h = 180.0;
    let you = comp
        .build_layer()
        .text("you?", 156.0)
        .width(you_w)
        .height(you_h)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(ink)
        .glow(primary, 16.0)
        .at(cx - you_w * 0.5, cy - you_h * 0.5)
        .depth(0.12)
        .add();
    comp.animate(you)
        .fade_in(13.85, 0.18)
        .ease_out()
        .scale_from(0.0, 13.85, 0.46)
        .spring(420.0, 14.0)
        .kf(14.20, AnimatedProperty::scale(1.0, 1.0))
        .kf_eased(14.32, AnimatedProperty::scale(1.07, 1.07), Easing::EASE_OUT)
        .kf_eased(14.48, AnimatedProperty::scale(1.0, 1.0), Easing::EASE_IN_OUT)
        .kf(act3_out - 0.10, AnimatedProperty::opacity(1.0))
        .kf_eased(act3_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    scene.assets.insert(comp_id.clone(), Asset::Composition(comp));
    comp_id
}

// ─────────────────────────────────────────────────────────────────────────────
// ACT 4  (14.4 – 25.3 s)  HACKATHON hero + 3 visual concept beats
//   Beat A (14.55 – 17.30): "HACKATHON" hero with bracket lines + accent bar.
//   Beat B (17.50 – 18.78): Skyline silhouette of London draws in (no text).
//   Beat C (18.84 – 19.92): Memory rings huge (only)  — primary
//   Beat D (19.92 – 21.04): Network nodes huge (only) — violet
//   Beat E (21.04 – 23.30): Sparkle huge (only)       — amber
//   No tagline. The VO carries the meaning; visuals carry the rhythm.
// ─────────────────────────────────────────────────────────────────────────────
fn build_act4(scene: &mut Scene) -> Id {
    let comp_id = Id::new();
    let mut comp = Composition::new(W, H);
    comp.id = comp_id.clone();
    comp.duration = Duration::Seconds(SCENE_DUR);
    let cx = W * 0.5;
    let cy = H * 0.5;
    let (bg, _, ink, _, muted, primary, violet, _, amber) = make_colors();

    comp.build_layer().rect(W, H).fill(bg).at(0.0, 0.0).depth(0.0).add();

    let act4_out = 24.85;

    // ── Beat A: "HACKATHON" hero ─────────────────────────────────────────────
    let beat_a_out = 17.30;

    // Tiny eyebrow "AGENTIC EVOLUTION"
    let eyebrow = comp
        .build_layer()
        .text("AGENTIC EVOLUTION", 18.0)
        .width(700.0)
        .height(28.0)
        .bold()
        .letter_spacing(8.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(muted)
        .at(cx - 350.0, cy - 130.0)
        .depth(0.10)
        .add();
    comp.animate(eyebrow)
        .fade_in(14.58, 0.28)
        .ease_out()
        .slide_from(0.0, 14.0, 14.58, 0.36)
        .ease_out()
        .kf(beat_a_out - 0.20, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_a_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Bracket lines either side of eyebrow
    for (sx, slen) in [(cx - 470.0, 220.0_f32), (cx + 250.0, 220.0)] {
        let br = comp
            .build_layer()
            .line_path(0.0, 0.0, slen, 0.0)
            .stroke(primary.with_alpha(0.55), 1.5)
            .at(sx, cy - 116.0)
            .depth(0.105)
            .add();
        comp.animate(br)
            .clip_start(14.78)
            .kf(14.78, AnimatedProperty::trim_path_end(0.0))
            .kf_eased(15.10, AnimatedProperty::trim_path_end(1.0), Easing::EASE_OUT)
            .kf(14.78, AnimatedProperty::opacity(0.0))
            .kf_eased(14.86, AnimatedProperty::opacity(1.0), Easing::EASE_OUT)
            .kf(beat_a_out - 0.20, AnimatedProperty::opacity(1.0))
            .kf_eased(beat_a_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();
    }

    // HACKATHON
    let hackathon = comp
        .build_layer()
        .text("HACKATHON", 124.0)
        .width(1240.0)
        .height(150.0)
        .bold()
        .letter_spacing(2.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(ink)
        .glow(primary, 14.0)
        .at(cx - 620.0, cy - 60.0)
        .depth(0.11)
        .add();
    comp.animate(hackathon)
        .fade_in(15.82, 0.20)
        .ease_out()
        .scale_from(0.5, 15.82, 0.54)
        .spring(380.0, 14.0)
        .kf(beat_a_out - 0.20, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_a_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Accent bar
    let bar = comp
        .build_layer()
        .rect(360.0, 6.0)
        .corner_radius(3.0)
        .fill(primary)
        .anchor_left()
        .at(cx - 180.0, cy + 90.0)
        .depth(0.115)
        .add();
    comp.animate(bar)
        .clip_start(15.95)
        .kf(15.95, AnimatedProperty::scale(0.0, 1.0))
        .kf_eased(16.42, AnimatedProperty::scale(1.0, 1.0), Easing::EASE_OUT)
        .kf(15.95, AnimatedProperty::opacity(1.0))
        .kf(beat_a_out - 0.20, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_a_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // ── Beat B: London skyline (visual = "in London") ────────────────────────
    let beat_b_out = 18.78;
    let skyline_w = 720.0;
    let skyline_h = 288.0;
    let skyline = comp
        .build_layer()
        .svg(svg_skyline(HEX_INK))
        .width(skyline_w)
        .height(skyline_h)
        .at(cx - skyline_w * 0.5, cy - 60.0)
        .depth(0.11)
        .add();
    comp.animate(skyline)
        .fade_in(17.30, 0.30)
        .ease_out()
        .slide_from(0.0, 60.0, 17.30, 0.46)
        .ease_out()
        .kf(beat_b_out - 0.18, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_b_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Horizon line under skyline
    let horizon = comp
        .build_layer()
        .line_path(0.0, 0.0, 880.0, 0.0)
        .stroke(primary.with_alpha(0.65), 2.0)
        .at(cx - 440.0, cy + skyline_h - 60.0)
        .depth(0.115)
        .add();
    comp.animate(horizon)
        .clip_start(17.40)
        .kf(17.40, AnimatedProperty::trim_path_end(0.0))
        .kf_eased(17.95, AnimatedProperty::trim_path_end(1.0), Easing::EASE_OUT)
        .kf(17.40, AnimatedProperty::opacity(0.0))
        .kf_eased(17.50, AnimatedProperty::opacity(1.0), Easing::EASE_OUT)
        .kf(beat_b_out - 0.18, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_b_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Tiny date stamp top-right of skyline
    let date = comp
        .build_layer()
        .text("MAY 2 · 2026", 16.0)
        .width(280.0)
        .height(24.0)
        .bold()
        .letter_spacing(5.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(amber)
        .at(cx - 140.0, cy + skyline_h - 12.0)
        .depth(0.115)
        .add();
    comp.animate(date)
        .fade_in(17.85, 0.26)
        .ease_out()
        .kf(beat_b_out - 0.18, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_b_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // ── Beats C/D/E: metaphor beats with a small clarifying caption ──────────
    fn build_pure_metaphor(
        comp: &mut Composition,
        cx: f32,
        cy: f32,
        accent: Color,
        accent_hex: &str,
        icon_fn: fn(&str) -> String,
        caption: &str,
        t_in: f32,
        t_out: f32,
    ) {
        let icon_size = 360.0;
        let icon = comp
            .build_layer()
            .svg(icon_fn(accent_hex))
            .width(icon_size)
            .height(icon_size)
            .at(cx - icon_size * 0.5, cy - icon_size * 0.5 - 30.0)
            .depth(0.115)
            .add();
        comp.animate(icon)
            .fade_in(t_in, 0.16)
            .ease_out()
            .scale_from(0.3, t_in, 0.46)
            .spring(380.0, 13.0)
            .kf(t_in + 0.34, AnimatedProperty::rotation_z(-3.0))
            .kf_eased(t_in + 0.78, AnimatedProperty::rotation_z(3.0), Easing::EASE_IN_OUT)
            .kf_eased(t_out - 0.20, AnimatedProperty::rotation_z(0.0), Easing::EASE_IN_OUT)
            .kf(t_out - 0.10, AnimatedProperty::opacity(1.0))
            .kf_eased(t_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();

        // Faint glow ring expanding outward as accent
        let ring = comp
            .build_layer()
            .circle(icon_size + 40.0)
            .no_fill()
            .stroke(accent.with_alpha(0.40), 2.0)
            .at(cx - (icon_size + 40.0) * 0.5, cy - (icon_size + 40.0) * 0.5 - 30.0)
            .depth(0.112)
            .add();
        comp.animate(ring)
            .clip_start(t_in + 0.10)
            .kf(t_in + 0.10, AnimatedProperty::trim_path_end(0.0))
            .kf_eased(t_in + 0.70, AnimatedProperty::trim_path_end(1.0), Easing::EASE_OUT)
            .kf(t_in + 0.10, AnimatedProperty::opacity(0.0))
            .kf_eased(t_in + 0.20, AnimatedProperty::opacity(1.0), Easing::EASE_OUT)
            .kf(t_in + 0.20, AnimatedProperty::rotation_z(0.0))
            .kf_eased(t_out, AnimatedProperty::rotation_z(20.0), Easing::EASE_IN_OUT)
            .kf(t_out - 0.10, AnimatedProperty::opacity(1.0))
            .kf_eased(t_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();

        // Caption below — small, letter-spaced, lowercase
        let cap = comp
            .build_layer()
            .text(caption, 34.0)
            .width(1100.0)
            .height(48.0)
            .bold()
            .letter_spacing(3.0)
            .text_align_center()
            .vertical_align_middle()
            .fill(accent)
            .at(cx - 550.0, cy + icon_size * 0.5 + 30.0)
            .depth(0.118)
            .add();
        comp.animate(cap)
            .fade_in(t_in + 0.18, 0.24)
            .ease_out()
            .slide_from(0.0, 14.0, t_in + 0.18, 0.32)
            .ease_out()
            .kf(t_out - 0.10, AnimatedProperty::opacity(1.0))
            .kf_eased(t_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();
    }

    build_pure_metaphor(
        &mut comp,
        cx,
        cy,
        primary,
        HEX_PRIMARY,
        svg_memory,
        "MEMORY SYSTEMS",
        18.84,
        19.92,
    );
    build_pure_metaphor(
        &mut comp,
        cx,
        cy,
        violet,
        HEX_VIOLET,
        svg_network,
        "INTEGRATIONS",
        19.92,
        21.04,
    );
    build_pure_metaphor(
        &mut comp,
        cx,
        cy,
        amber,
        HEX_AMBER,
        svg_sparkle,
        "SELF-EVOLUTION",
        21.04,
        23.30,
    );

    // ── Final small beat (23.50 – 24.85): single word "partners." ────────────
    // VO ends "…genuine partners." — we land on that word alone, big.
    let partners = comp
        .build_layer()
        .text("partners.", 96.0)
        .width(1200.0)
        .height(120.0)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(ink)
        .glow(primary, 12.0)
        .at(cx - 600.0, cy - 60.0)
        .depth(0.12)
        .add();
    comp.animate(partners)
        .fade_in(23.50, 0.20)
        .ease_out()
        .scale_from(0.55, 23.50, 0.44)
        .spring(380.0, 14.0)
        .kf(act4_out - 0.20, AnimatedProperty::opacity(1.0))
        .kf_eased(act4_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    scene.assets.insert(comp_id.clone(), Asset::Composition(comp));
    comp_id
}

// ─────────────────────────────────────────────────────────────────────────────
// ACT 5  (24.8 – 30.2 s)  Hosts + date — very minimal text, two brand names + date
//   Beat A: MongoDB    (slides from left)
//   Beat B: × splits   (rotates)
//   Beat C: Cerebral Valley (slides from right) — both visible, lockup
//   Beat D: full clear → "MAY 2" giant amber, "2026" smaller below
// ─────────────────────────────────────────────────────────────────────────────
fn build_act5(scene: &mut Scene) -> Id {
    let comp_id = Id::new();
    let mut comp = Composition::new(W, H);
    comp.id = comp_id.clone();
    comp.duration = Duration::Seconds(SCENE_DUR);
    let cx = W * 0.5;
    let cy = H * 0.5;
    let (bg, _, _, ink2, muted, primary, _, emerald, amber) = make_colors();
    let _ = ink2;

    comp.build_layer().rect(W, H).fill(bg).at(0.0, 0.0).depth(0.0).add();

    let lockup_out = 27.85;
    let act5_out = 29.78;

    // Eyebrow: "HOSTED BY"
    let host_eb = comp
        .build_layer()
        .text("HOSTED BY", 18.0)
        .width(500.0)
        .height(28.0)
        .bold()
        .letter_spacing(10.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(muted)
        .at(cx - 250.0, cy - 150.0)
        .depth(0.10)
        .add();
    comp.animate(host_eb)
        .fade_in(24.95, 0.28)
        .ease_out()
        .slide_from(0.0, 12.0, 24.95, 0.34)
        .ease_out()
        .kf(lockup_out - 0.18, AnimatedProperty::opacity(1.0))
        .kf_eased(lockup_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // ── Hosted-by lockup with TRACK MATTE reveal ────────────────────────────
    // Concept: the × cross lands first as a focal anchor; then two alpha
    // mattes wipe outward from its center — left for MongoDB, right for
    // Cerebral Valley — making each name appear to pour OUT of the cross.

    // "MongoDB" and "Cerebral Valley" have very different visual widths,
    // so we shift the whole lockup left by half the diff to optically
    // center the assembly around cx.
    let mongo_visual: f32 = 370.0;
    let cv_visual: f32 = 580.0;
    let lockup_cx: f32 = cx - (cv_visual - mongo_visual) * 0.5;

    // × — center of lockup, springs in FIRST. This is the matte anchor.
    let cross = comp
        .build_layer()
        .text("×", 72.0)
        .width(120.0)
        .height(120.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(muted)
        .at(lockup_cx - 60.0, cy - 60.0)
        .depth(0.115)
        .add();
    comp.animate(cross)
        .fade_in(25.18, 0.16)
        .ease_out()
        .scale_from(0.0, 25.18, 0.32)
        .spring(440.0, 14.0)
        .kf(27.00, AnimatedProperty::rotation_z(0.0))
        .kf_eased(27.40, AnimatedProperty::rotation_z(90.0), Easing::EASE_IN_OUT)
        .kf(lockup_out - 0.18, AnimatedProperty::opacity(1.0))
        .kf_eased(lockup_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // ── Left wipe matte ─────────────────────────────────────────────────────
    // Anchored at right edge = lockup_cx; scale-x from 0→1 makes it expand
    // leftward FROM the cross. The MongoDB layer slides left by its full
    // width in sync, so the text literally extrudes out of the × leftward.
    let mongo_w: f32 = 720.0;
    let gap: f32 = 60.0; // breathing room between text and × cross
    let mongo_final_x: f32 = lockup_cx - mongo_w - gap;
    let pour_t0: f32 = 25.55;
    let pour_t1: f32 = 26.20;
    let left_mask = comp
        .build_layer()
        .rect(mongo_w, 180.0)
        .fill(Color::hex(HEX_INK))
        .anchor_right()
        .at(mongo_final_x, cy - 90.0)
        .depth(0.105)
        .add();
    comp.animate(left_mask.clone())
        .kf(pour_t0, AnimatedProperty::scale(0.0, 1.0))
        .kf_eased(pour_t1, AnimatedProperty::scale(1.0, 1.0), Easing::EASE_OUT)
        .kf(lockup_out - 0.18, AnimatedProperty::opacity(1.0))
        .kf_eased(lockup_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    let mongodb = comp
        .build_layer()
        .text("MongoDB", 76.0)
        .width(mongo_w)
        .height(120.0)
        .bold()
        .text_align_right()
        .vertical_align_middle()
        .fill(emerald)
        .glow(emerald, 12.0)
        .at(lockup_cx - gap, cy - 60.0)
        .depth(0.110)
        .add();
    comp.animate(mongodb.clone())
        .clip_start(pour_t0)
        .kf(pour_t0, AnimatedProperty::position(lockup_cx - gap, cy - 60.0))
        .kf_eased(
            pour_t1,
            AnimatedProperty::position(mongo_final_x, cy - 60.0),
            Easing::EASE_OUT,
        )
        .kf(lockup_out - 0.18, AnimatedProperty::opacity(1.0))
        .kf_eased(lockup_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();
    if let Some(layer) = comp.get_layer_mut(&mongodb) {
        layer.matte_id = Some(left_mask);
        layer.matte_mode = MaskMode::Alpha;
    }

    // ── Right wipe matte ────────────────────────────────────────────────────
    let cv_w: f32 = 900.0;
    let cv_final_x: f32 = lockup_cx + gap;
    let right_mask = comp
        .build_layer()
        .rect(cv_w, 180.0)
        .fill(Color::hex(HEX_INK))
        .anchor_left()
        .at(cv_final_x, cy - 90.0)
        .depth(0.105)
        .add();
    comp.animate(right_mask.clone())
        .kf(pour_t0, AnimatedProperty::scale(0.0, 1.0))
        .kf_eased(pour_t1, AnimatedProperty::scale(1.0, 1.0), Easing::EASE_OUT)
        .kf(lockup_out - 0.18, AnimatedProperty::opacity(1.0))
        .kf_eased(lockup_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    let cv = comp
        .build_layer()
        .text("Cerebral Valley", 76.0)
        .width(cv_w)
        .height(120.0)
        .bold()
        .text_align(kario_base::TextAlign::Left)
        .vertical_align_middle()
        .fill(primary)
        .glow(primary, 12.0)
        .at(cv_final_x - cv_w, cy - 60.0)
        .depth(0.110)
        .add();
    comp.animate(cv.clone())
        .clip_start(pour_t0)
        .kf(pour_t0, AnimatedProperty::position(cv_final_x - cv_w, cy - 60.0))
        .kf_eased(
            pour_t1,
            AnimatedProperty::position(cv_final_x, cy - 60.0),
            Easing::EASE_OUT,
        )
        .kf(lockup_out - 0.18, AnimatedProperty::opacity(1.0))
        .kf_eased(lockup_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();
    if let Some(layer) = comp.get_layer_mut(&cv) {
        layer.matte_id = Some(right_mask);
        layer.matte_mode = MaskMode::Alpha;
    }

    // ── Date beat ────────────────────────────────────────────────────────────
    // "MAY 2" massive amber centered
    let may2 = comp
        .build_layer()
        .text("MAY 2", 220.0)
        .width(1200.0)
        .height(260.0)
        .bold()
        .letter_spacing(4.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(amber)
        .glow(amber, 18.0)
        .at(cx - 600.0, cy - 130.0)
        .depth(0.12)
        .add();
    comp.animate(may2)
        .fade_in(28.08, 0.18)
        .ease_out()
        .scale_from(0.5, 28.08, 0.50)
        .spring(380.0, 14.0)
        .kf(act5_out - 0.16, AnimatedProperty::opacity(1.0))
        .kf_eased(act5_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // "2026" below, small
    let year = comp
        .build_layer()
        .text("2026", 32.0)
        .width(400.0)
        .height(48.0)
        .bold()
        .letter_spacing(12.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(amber)
        .at(cx - 200.0, cy + 130.0)
        .depth(0.12)
        .add();
    comp.animate(year)
        .fade_in(28.42, 0.26)
        .ease_out()
        .slide_from(0.0, 14.0, 28.42, 0.32)
        .ease_out()
        .kf(act5_out - 0.16, AnimatedProperty::opacity(1.0))
        .kf_eased(act5_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Underline beneath MAY 2
    let und = comp
        .build_layer()
        .line_path(0.0, 0.0, 540.0, 0.0)
        .stroke(amber, 4.0)
        .at(cx - 270.0, cy + 100.0)
        .depth(0.12)
        .add();
    comp.animate(und)
        .clip_start(28.30)
        .kf(28.30, AnimatedProperty::trim_path_end(0.0))
        .kf_eased(28.85, AnimatedProperty::trim_path_end(1.0), Easing::EASE_OUT)
        .kf(28.30, AnimatedProperty::opacity(0.0))
        .kf_eased(28.40, AnimatedProperty::opacity(1.0), Easing::EASE_OUT)
        .kf(act5_out - 0.16, AnimatedProperty::opacity(1.0))
        .kf_eased(act5_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    scene.assets.insert(comp_id.clone(), Asset::Composition(comp));
    comp_id
}

// ─────────────────────────────────────────────────────────────────────────────
// ACT 6  (29.7 – 41.5 s)  Prizes — three sequential metaphor beats
//   Each: huge icon centered, single number/phrase below it. No kicker, no sub.
//   Beat A: trophy + "£15,000"
//   Beat B: house silhouette + "1 month"
//   Beat C: zap + "Demo live"
//   Beat D: 5 audience silhouettes (visual = "the people shaping this industry")
// ─────────────────────────────────────────────────────────────────────────────
fn build_act6(scene: &mut Scene) -> Id {
    let comp_id = Id::new();
    let mut comp = Composition::new(W, H);
    comp.id = comp_id.clone();
    comp.duration = Duration::Seconds(SCENE_DUR);
    let cx = W * 0.5;
    let cy = H * 0.5;
    let (bg, _, ink, _, _, primary, _, emerald, amber) = make_colors();

    comp.build_layer().rect(W, H).fill(bg).at(0.0, 0.0).depth(0.0).add();

    fn build_prize_beat(
        comp: &mut Composition,
        cx: f32,
        cy: f32,
        big: &str,
        big_size: f32,
        accent: Color,
        accent_hex: &str,
        icon_fn: fn(&str) -> String,
        t_in: f32,
        t_out: f32,
    ) {
        // Huge metaphor icon above
        let icon_size = 200.0;
        let icon = comp
            .build_layer()
            .svg(icon_fn(accent_hex))
            .width(icon_size)
            .height(icon_size)
            .at(cx - icon_size * 0.5, cy - icon_size - 30.0)
            .depth(0.11)
            .add();
        comp.animate(icon)
            .fade_in(t_in, 0.18)
            .ease_out()
            .scale_from(0.4, t_in, 0.44)
            .spring(420.0, 14.0)
            .kf(t_in + 0.42, AnimatedProperty::rotation_z(-6.0))
            .kf_eased(t_in + 0.85, AnimatedProperty::rotation_z(6.0), Easing::EASE_IN_OUT)
            .kf_eased(t_in + 1.20, AnimatedProperty::rotation_z(0.0), Easing::EASE_IN_OUT)
            .kf(t_out - 0.12, AnimatedProperty::opacity(1.0))
            .kf_eased(t_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();

        // Glow ring around the icon
        let ring = comp
            .build_layer()
            .circle(icon_size + 80.0)
            .no_fill()
            .stroke(accent.with_alpha(0.40), 2.0)
            .at(cx - (icon_size + 80.0) * 0.5, cy - icon_size - 30.0 - 40.0)
            .depth(0.105)
            .add();
        comp.animate(ring)
            .clip_start(t_in + 0.08)
            .kf(t_in + 0.08, AnimatedProperty::trim_path_end(0.0))
            .kf_eased(t_in + 0.70, AnimatedProperty::trim_path_end(1.0), Easing::EASE_OUT)
            .kf(t_in + 0.08, AnimatedProperty::opacity(0.0))
            .kf_eased(t_in + 0.18, AnimatedProperty::opacity(1.0), Easing::EASE_OUT)
            .kf(t_in + 0.20, AnimatedProperty::rotation_z(0.0))
            .kf_eased(t_out, AnimatedProperty::rotation_z(15.0), Easing::EASE_IN_OUT)
            .kf(t_out - 0.12, AnimatedProperty::opacity(1.0))
            .kf_eased(t_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();

        // Big number / phrase below — single line, no extras
        let big_l = comp
            .build_layer()
            .text(big, big_size)
            .width(1200.0)
            .height(big_size * 1.3)
            .bold()
            .text_align_center()
            .vertical_align_middle()
            .fill(Color::hex(HEX_INK))
            .glow(accent, 12.0)
            .at(cx - 600.0, cy + 30.0)
            .depth(0.115)
            .add();
        comp.animate(big_l)
            .fade_in(t_in + 0.18, 0.20)
            .ease_out()
            .scale_from(0.5, t_in + 0.18, 0.46)
            .spring(380.0, 14.0)
            .kf(t_out - 0.12, AnimatedProperty::opacity(1.0))
            .kf_eased(t_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();
    }

    build_prize_beat(
        &mut comp,
        cx,
        cy,
        "£15,000",
        148.0,
        amber,
        HEX_AMBER,
        svg_trophy,
        29.83,
        31.85,
    );
    build_prize_beat(
        &mut comp,
        cx,
        cy,
        "1 Month  •  London",
        88.0,
        primary,
        HEX_PRIMARY,
        svg_skyline,
        31.91,
        36.42,
    );
    build_prize_beat(
        &mut comp,
        cx,
        cy,
        "Demo live",
        118.0,
        emerald,
        HEX_EMERALD,
        svg_zap,
        36.54,
        38.20,
    );

    // ── Beat D (38.33 – 41.12): audience silhouettes ─────────────────────────
    // 7 person silhouettes pop up in a row — "the people shaping this industry"
    let beat_d_out = 41.12;
    let person_w = 90.0;
    let person_h = 120.0;
    let count = 7;
    let total = count as f32 * person_w + (count - 1) as f32 * 16.0;
    let row_x0 = cx - total * 0.5;
    let row_y = cy - person_h * 0.5 + 30.0;
    for i in 0..count {
        let px = row_x0 + i as f32 * (person_w + 16.0);
        let scale_var = 0.85 + ((i as f32) * 0.51).sin() * 0.18;
        let p = comp
            .build_layer()
            .svg(svg_person(HEX_INK2))
            .width(person_w * scale_var)
            .height(person_h * scale_var)
            .at(px + (person_w * (1.0 - scale_var)) * 0.5, row_y)
            .depth(0.11)
            .add();
        let t0 = 38.33 + i as f32 * 0.10;
        comp.animate(p)
            .fade_in(t0, 0.20)
            .ease_out()
            .slide_from(0.0, 60.0, t0, 0.40)
            .with_easing(Easing::Spring {
                stiffness: 380.0,
                damping: 14.0,
                mass: 1.0,
            })
            .kf(beat_d_out - 0.20, AnimatedProperty::opacity(1.0))
            .kf_eased(beat_d_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();
    }

    // Floor line under the people
    let floor = comp
        .build_layer()
        .line_path(0.0, 0.0, 1100.0, 0.0)
        .stroke(primary.with_alpha(0.55), 2.0)
        .at(cx - 550.0, row_y + person_h + 12.0)
        .depth(0.105)
        .add();
    comp.animate(floor)
        .clip_start(38.55)
        .kf(38.55, AnimatedProperty::trim_path_end(0.0))
        .kf_eased(39.30, AnimatedProperty::trim_path_end(1.0), Easing::EASE_OUT)
        .kf(38.55, AnimatedProperty::opacity(0.0))
        .kf_eased(38.65, AnimatedProperty::opacity(1.0), Easing::EASE_OUT)
        .kf(beat_d_out - 0.20, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_d_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Caption: "THE INDUSTRY" beneath the row
    let aud_cap = comp
        .build_layer()
        .text("THE INDUSTRY", 28.0)
        .width(900.0)
        .height(40.0)
        .bold()
        .letter_spacing(8.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(primary)
        .at(cx - 450.0, row_y + person_h + 30.0)
        .depth(0.115)
        .add();
    comp.animate(aud_cap)
        .fade_in(39.20, 0.30)
        .ease_out()
        .slide_from(0.0, 12.0, 39.20, 0.34)
        .ease_out()
        .kf(beat_d_out - 0.20, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_d_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    let _ = ink;
    scene.assets.insert(comp_id.clone(), Asset::Composition(comp));
    comp_id
}

// ─────────────────────────────────────────────────────────────────────────────
// ACT 7  (41.0 – 46.2 s)  Requirements + CTA — all visual
//   Beat A (41.22 – 42.95): 4 person silhouettes pop up in a row (= teams of 4)
//   Beat B (43.00 – 44.55): hourglass big — visual for "limited"
//   Beat C (44.62 – 45.85): "Apply now →" gradient pill
// ─────────────────────────────────────────────────────────────────────────────
fn build_act7(scene: &mut Scene) -> Id {
    let comp_id = Id::new();
    let mut comp = Composition::new(W, H);
    comp.id = comp_id.clone();
    comp.duration = Duration::Seconds(SCENE_DUR);
    let cx = W * 0.5;
    let cy = H * 0.5;
    let (bg, _, ink, _, _, primary, violet, _, amber) = make_colors();
    let _ = ink;

    comp.build_layer().rect(W, H).fill(bg).at(0.0, 0.0).depth(0.0).add();

    let act7_out = 45.85;

    // ── Beat A: 4 people pop up ──────────────────────────────────────────────
    let beat_a_out = 42.95;
    let person_w = 130.0;
    let person_h = 174.0;
    let gap = 36.0;
    let total = 4.0 * person_w + 3.0 * gap;
    let row_x0 = cx - total * 0.5;
    let row_y = cy - person_h * 0.5;
    let person_colors = [HEX_PRIMARY, HEX_VIOLET, HEX_EMERALD, HEX_AMBER];
    for i in 0..4 {
        let px = row_x0 + i as f32 * (person_w + gap);
        let p = comp
            .build_layer()
            .svg(svg_person(person_colors[i]))
            .width(person_w)
            .height(person_h)
            .at(px, row_y)
            .depth(0.11)
            .add();
        let t0 = 41.22 + i as f32 * 0.10;
        comp.animate(p)
            .fade_in(t0, 0.18)
            .ease_out()
            .slide_from(0.0, 80.0, t0, 0.40)
            .with_easing(Easing::Spring {
                stiffness: 380.0,
                damping: 14.0,
                mass: 1.0,
            })
            .kf(beat_a_out - 0.18, AnimatedProperty::opacity(1.0))
            .kf_eased(beat_a_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();
    }
    // Floor line
    let floor = comp
        .build_layer()
        .line_path(0.0, 0.0, total + 80.0, 0.0)
        .stroke(primary.with_alpha(0.55), 2.0)
        .at(row_x0 - 40.0, row_y + person_h + 18.0)
        .depth(0.105)
        .add();
    comp.animate(floor)
        .clip_start(41.30)
        .kf(41.30, AnimatedProperty::trim_path_end(0.0))
        .kf_eased(41.95, AnimatedProperty::trim_path_end(1.0), Easing::EASE_OUT)
        .kf(41.30, AnimatedProperty::opacity(0.0))
        .kf_eased(41.40, AnimatedProperty::opacity(1.0), Easing::EASE_OUT)
        .kf(beat_a_out - 0.18, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_a_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Caption: "TEAMS OF 4"
    let teams_cap = comp
        .build_layer()
        .text("TEAMS OF 4", 32.0)
        .width(700.0)
        .height(48.0)
        .bold()
        .letter_spacing(8.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(primary)
        .at(cx - 350.0, row_y + person_h + 36.0)
        .depth(0.115)
        .add();
    comp.animate(teams_cap)
        .fade_in(41.85, 0.30)
        .ease_out()
        .slide_from(0.0, 14.0, 41.85, 0.34)
        .ease_out()
        .kf(beat_a_out - 0.18, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_a_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // ── Beat B: hourglass huge ───────────────────────────────────────────────
    let beat_b_out = 44.55;
    let hg_size = 280.0;
    let hg_cy = cy - 50.0; // shift up so caption clears the warning ring
    let hourglass = comp
        .build_layer()
        .svg(svg_hourglass(HEX_AMBER))
        .width(hg_size)
        .height(hg_size)
        .at(cx - hg_size * 0.5, hg_cy - hg_size * 0.5)
        .depth(0.115)
        .add();
    comp.animate(hourglass)
        .fade_in(43.00, 0.20)
        .ease_out()
        .scale_from(0.4, 43.00, 0.46)
        .spring(380.0, 14.0)
        // tilt right then left, like grains falling
        .kf(43.50, AnimatedProperty::rotation_z(0.0))
        .kf_eased(43.85, AnimatedProperty::rotation_z(180.0), Easing::EASE_IN_OUT)
        .kf(beat_b_out - 0.18, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_b_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Faint warning ring around hourglass
    let ring_d = hg_size + 80.0;
    let ring = comp
        .build_layer()
        .circle(ring_d)
        .no_fill()
        .stroke(amber.with_alpha(0.35), 1.5)
        .at(cx - ring_d * 0.5, hg_cy - ring_d * 0.5)
        .depth(0.110)
        .add();
    comp.animate(ring)
        .clip_start(43.10)
        .kf(43.10, AnimatedProperty::trim_path_end(0.0))
        .kf_eased(43.75, AnimatedProperty::trim_path_end(1.0), Easing::EASE_OUT)
        .kf(43.10, AnimatedProperty::opacity(0.0))
        .kf_eased(43.20, AnimatedProperty::opacity(1.0), Easing::EASE_OUT)
        .kf(beat_b_out - 0.18, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_b_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Caption: "SPACE IS LIMITED" — placed BELOW the warning ring with
    // breathing room so it never collides with the rotating hourglass.
    let cap_y = hg_cy + ring_d * 0.5 + 36.0;
    let lim_cap = comp
        .build_layer()
        .text("SPACE IS LIMITED", 28.0)
        .width(900.0)
        .height(40.0)
        .bold()
        .letter_spacing(8.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(amber)
        .at(cx - 450.0, cap_y)
        .depth(0.116)
        .add();
    comp.animate(lim_cap)
        .fade_in(43.30, 0.28)
        .ease_out()
        .slide_from(0.0, 14.0, 43.30, 0.34)
        .ease_out()
        .kf(beat_b_out - 0.18, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_b_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // ── Beat C: Apply now button ─────────────────────────────────────────────
    let btn_w = 460.0;
    let btn_h = 100.0;
    let btn_x = cx - btn_w * 0.5;
    let btn_y = cy - btn_h * 0.5;

    let btn_bg = comp
        .build_layer()
        .rect(btn_w, btn_h)
        .corner_radius(btn_h * 0.5)
        .fill(Paint::linear(
            [0.0, 0.5],
            [1.0, 0.5],
            [(0.0, primary), (1.0, violet)],
        ))
        .drop_shadow([0.0, 0.0], primary.with_alpha(0.30), 28.0, 0.0)
        .at(btn_x, btn_y)
        .depth(0.12)
        .add();
    comp.animate(btn_bg)
        .fade_in(44.62, 0.18)
        .ease_out()
        .scale_from(0.0, 44.62, 0.50)
        .spring(420.0, 13.0)
        .kf(act7_out - 0.16, AnimatedProperty::opacity(1.0))
        .kf_eased(act7_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    let btn_label = comp
        .build_layer()
        .text("Apply now", 38.0)
        .width(btn_w - 100.0)
        .height(btn_h)
        .bold()
        .vertical_align_middle()
        .fill(Color::hex(HEX_INK))
        .at(btn_x + 56.0, btn_y)
        .depth(0.13)
        .add();
    comp.animate(btn_label)
        .fade_in(44.74, 0.20)
        .ease_out()
        .kf(act7_out - 0.16, AnimatedProperty::opacity(1.0))
        .kf_eased(act7_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    let btn_arrow = comp
        .build_layer()
        .svg(svg_arrow_right(HEX_INK))
        .width(36.0)
        .height(36.0)
        .at(btn_x + btn_w - 76.0, btn_y + (btn_h - 36.0) * 0.5)
        .depth(0.13)
        .add();
    comp.animate(btn_arrow)
        .fade_in(44.78, 0.20)
        .ease_out()
        .kf(45.10, AnimatedProperty::position_x(btn_x + btn_w - 76.0))
        .kf_eased(
            45.30,
            AnimatedProperty::position_x(btn_x + btn_w - 60.0),
            Easing::EASE_OUT,
        )
        .kf_eased(
            45.55,
            AnimatedProperty::position_x(btn_x + btn_w - 76.0),
            Easing::EASE_IN_OUT,
        )
        .kf(act7_out - 0.16, AnimatedProperty::opacity(1.0))
        .kf_eased(act7_out, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    scene.assets.insert(comp_id.clone(), Asset::Composition(comp));
    comp_id
}

// ─────────────────────────────────────────────────────────────────────────────
// ACT 8  (45.7 – 51.0 s)  Closer — Horizontal Lockup
//   Concept: A single typographic lockup sits centered above the headline:
//      ONE DAY  ◼  ONE IDEA
//   "ONE DAY" slides in from the left, a small primary square divider snaps
//   in at center, "ONE IDEA" slides in from the right. Beat, then the
//   headline "Let's see what you build." springs in below. Clean, designed,
//   no superfluous geometry.
// ─────────────────────────────────────────────────────────────────────────────
fn build_act8(scene: &mut Scene) -> Id {
    let comp_id = Id::new();
    let mut comp = Composition::new(W, H);
    comp.id = comp_id.clone();
    comp.duration = Duration::Seconds(SCENE_DUR);
    let cx = W * 0.5;
    let (bg, _, ink, ink2, muted, primary, _, _, _) = make_colors();

    comp.build_layer().rect(W, H).fill(bg).at(0.0, 0.0).depth(0.0).add();

    // Layout (canvas H = 720)
    //   y=232  ONE DAY  ◼  ONE IDEA  (eyebrow lockup)
    //   y=256  thin underline accent (centered, narrow)
    //   y=380..560  "Let's see what you build." (two lines, 80px)
    //   y=640  agentic.london
    let lockup_y = 232.0;

    // Center divider — small primary square. Sits exactly at cx.
    let divider_size = 10.0;
    let divider = comp
        .build_layer()
        .rect(divider_size, divider_size)
        .corner_radius(2.0)
        .fill(primary)
        .at(cx - divider_size * 0.5, lockup_y - divider_size * 0.5)
        .depth(0.108)
        .add();
    comp.animate(divider)
        .fade_in(46.20, 0.18)
        .ease_out()
        .scale_from(0.0, 46.20, 0.34)
        .spring(420.0, 14.0)
        // gentle pulse on the headline beat
        .kf(47.85, AnimatedProperty::scale(1.0, 1.0))
        .kf_eased(48.05, AnimatedProperty::scale(1.6, 1.6), Easing::EASE_OUT)
        .kf_eased(48.30, AnimatedProperty::scale(1.0, 1.0), Easing::EASE_IN_OUT)
        .kf(FINAL_OUT_T - 0.20, AnimatedProperty::opacity(1.0))
        .kf_eased(
            FINAL_OUT_T + FINAL_OUT_DUR,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // "ONE DAY" — sits to the LEFT of the divider, right-aligned to its
    // inner edge with breathing room. Tight text box so center-of-glyphs
    // aligns with center-of-box.
    let eyebrow_w = 220.0;
    let eyebrow_gap = 28.0;
    // Right edge of the "ONE DAY" box should sit at (cx - divider_size/2 - gap)
    let one_day_x = cx - divider_size * 0.5 - eyebrow_gap - eyebrow_w;
    let one_day = comp
        .build_layer()
        .text("ONE DAY", 24.0)
        .width(eyebrow_w)
        .height(36.0)
        .letter_spacing(8.0)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(ink2)
        .at(one_day_x, lockup_y - 18.0)
        .depth(0.108)
        .add();
    comp.animate(one_day)
        .fade_in(45.95, 0.30)
        .ease_out()
        .slide_from(-40.0, 0.0, 45.95, 0.42)
        .ease_out()
        .kf(FINAL_OUT_T - 0.20, AnimatedProperty::opacity(1.0))
        .kf_eased(
            FINAL_OUT_T + FINAL_OUT_DUR,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // "ONE IDEA" — symmetric, sits to the RIGHT of the divider
    let one_idea_x = cx + divider_size * 0.5 + eyebrow_gap;
    let one_idea = comp
        .build_layer()
        .text("ONE IDEA", 24.0)
        .width(eyebrow_w)
        .height(36.0)
        .letter_spacing(8.0)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(ink2)
        .at(one_idea_x, lockup_y - 18.0)
        .depth(0.108)
        .add();
    comp.animate(one_idea)
        .fade_in(46.45, 0.30)
        .ease_out()
        .slide_from(40.0, 0.0, 46.45, 0.42)
        .ease_out()
        .kf(FINAL_OUT_T - 0.20, AnimatedProperty::opacity(1.0))
        .kf_eased(
            FINAL_OUT_T + FINAL_OUT_DUR,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Thin accent underline beneath the lockup — centered, scales out from
    // center as the divider lands. Provides the typographic baseline.
    let underline_w = 80.0;
    let underline = comp
        .build_layer()
        .rect(underline_w, 2.0)
        .corner_radius(1.0)
        .fill(primary.with_alpha(0.55))
        .anchor_left()
        .at(cx - underline_w * 0.5, lockup_y + 22.0)
        .depth(0.107)
        .add();
    comp.animate(underline)
        .clip_start(46.40)
        .kf(46.40, AnimatedProperty::opacity(0.0))
        .kf_eased(46.55, AnimatedProperty::opacity(1.0), Easing::EASE_OUT)
        .kf(46.40, AnimatedProperty::scale(0.0, 1.0))
        .kf_eased(46.85, AnimatedProperty::scale(1.0, 1.0), Easing::EASE_OUT)
        .kf(FINAL_OUT_T - 0.20, AnimatedProperty::opacity(1.0))
        .kf_eased(
            FINAL_OUT_T + FINAL_OUT_DUR,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Headline — "Let's see what you build." centered, fits the canvas
    // Two lines at 80px → ~220px box. Centered around y=460.
    let lets_build = comp
        .build_layer()
        .text("Let's see\nwhat you build.", 80.0)
        .width(1200.0)
        .height(220.0)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(ink)
        .glow(primary, 14.0)
        .at(cx - 600.0, 350.0)
        .depth(0.12)
        .add();
    comp.animate(lets_build)
        .fade_in(47.85, 0.20)
        .ease_out()
        .scale_from(0.70, 47.85, 0.50)
        .spring(380.0, 14.0)
        .kf(FINAL_OUT_T - 0.20, AnimatedProperty::opacity(1.0))
        .kf_eased(
            FINAL_OUT_T + FINAL_OUT_DUR,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // URL footer — tucked safely below the headline
    let url = comp
        .build_layer()
        .text("agentic.london", 18.0)
        .width(600.0)
        .height(28.0)
        .letter_spacing(3.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(muted)
        .at(cx - 300.0, 640.0)
        .depth(0.115)
        .add();
    comp.animate(url)
        .fade_in(48.60, 0.32)
        .ease_out()
        .kf(FINAL_OUT_T - 0.20, AnimatedProperty::opacity(1.0))
        .kf_eased(
            FINAL_OUT_T + FINAL_OUT_DUR,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    scene.assets.insert(comp_id.clone(), Asset::Composition(comp));
    comp_id
}

// ─────────────────────────────────────────────────────────────────────────────
// Scene assembly with VARIED transitions
// ─────────────────────────────────────────────────────────────────────────────
pub fn build_scene() -> Scene {
    let mut scene = Scene::new(W, H);
    scene.main_composition.duration = Duration::Seconds(SCENE_DUR);

    let vo_asset = AudioAsset::new(VO_SRC)
        .with_script(SCRIPT)
        .with_word_timing("Something is shifting in AI.", 0.291, 1.601)
        .with_word_timing(
            "Agents aren't waiting to be told what to do anymore.",
            2.472,
            2.680,
        )
        .with_word_timing(
            "They're remembering. Deciding. Getting better on their own.",
            5.633,
            3.601,
        )
        .with_word_timing(
            "And somebody has to build the infrastructure for that world.",
            9.774,
            2.921,
        )
        .with_word_timing("Why not you?", 13.335, 0.700)
        .with_word_timing("The Agentic Evolution Hackathon.", 14.555, 1.961)
        .with_word_timing(
            "One day in London to architect the memory systems, integrations, \
             and self-evolution engines that turn agents into genuine partners.",
            17.036,
            7.462,
        )
        .with_word_timing("Hosted by MongoDB and Cerebral Valley.", 25.018, 2.705)
        .with_word_timing("May second.", 28.144, 1.483)
        .with_word_timing("Fifteen thousand pounds in prizes.", 29.827, 1.523)
        .with_word_timing(
            "A month of residency at London Founder House.",
            31.811,
            2.424,
        )
        .with_word_timing(
            "And the chance to demo live at MongoDB.local.",
            34.736,
            3.446,
        )
        .with_word_timing(
            "In front of the people who are shaping this industry.",
            38.323,
            2.444,
        )
        .with_word_timing("Teams of up to four.", 41.248, 1.202)
        .with_word_timing("Space is limited.", 43.011, 1.142)
        .with_word_timing("Apply now.", 44.674, 0.722)
        .with_word_timing("One day. One idea.", 45.957, 1.603)
        .with_word_timing("Let's see what you build.", 48.041, 1.102);
    let vo_id = scene.add_audio_asset(vo_asset);
    scene.main_composition.add_audio(
        Audio::new(vo_id)
            .with_start_time(0.0)
            .with_duration(VO_DUR)
            .with_volume(2.0),
    );

    // ── Background music ────────────────────────────────────────────────
    // Upbeat bed kept well under the VO so dialogue stays crisp.
    // Source has ~1s of silence at the head, so skip past it via clip_start.
    let bg_asset = AudioAsset::new(BG_MUSIC);
    let bg_id = scene.add_audio_asset(bg_asset);
    scene.main_composition.add_audio(
        Audio::new(bg_id)
            .with_start_time(0.0)
            .with_duration(SCENE_DUR)
            .with_offset(1.2)
            .with_volume(0.18),
    );

    // ── SFX ──────────────────────────────────────────────────────────────
    // Two SFX assets are reused across the timeline:
    //   whoosh.mp3       — for big motion / transition moments
    //   universal-click  — for punctuation accents on key visual hits
    //
    // SFX volumes sit well below VO so the voice stays foregrounded.
    let whoosh_asset = AudioAsset::new(SFX_WHOOSH);
    let whoosh_id = scene.add_audio_asset(whoosh_asset);
    let click_asset = AudioAsset::new(SFX_CLICK);
    let click_id = scene.add_audio_asset(click_asset);

    // Whoosh cues — motion / transitions
    //   4.10s  agent breaking free after tether snap
    //   13.45s brackets slamming in for "you?"
    //   14.55s HACKATHON hero
    //   47.80s final headline "Let's see what you build."
    let whoosh_cues: &[(f32, f32, f32)] = &[
        (4.10, 1.20, 0.38),
        (13.45, 1.00, 0.34),
        (14.55, 1.20, 0.42),
        (47.80, 1.40, 0.38),
    ];
    for &(t, dur, vol) in whoosh_cues {
        scene.main_composition.add_audio(
            Audio::new(whoosh_id.clone())
                .with_start_time(t)
                .with_duration(dur)
                .with_volume(vol),
        );
    }

    // Click cues — punctuation accents
    //   4.05s  tether snap moment
    //   13.78s "you?" punch lands
    //   18.84s MEMORY metaphor reveal
    //   19.92s INTEGRATIONS metaphor reveal
    //   21.04s SELF-EVOLUTION metaphor reveal
    //   44.67s "Apply now."
    //   46.20s closer divider square lands
    let click_cues: &[(f32, f32)] = &[
        (4.05, 0.30),
        (13.78, 0.38),
        (18.84, 0.26),
        (19.92, 0.26),
        (21.04, 0.26),
        (44.67, 0.30),
        (46.20, 0.26),
    ];
    for &(t, vol) in click_cues {
        scene.main_composition.add_audio(
            Audio::new(click_id.clone())
                .with_start_time(t)
                .with_duration(0.6)
                .with_volume(vol),
        );
    }

    let act1_id = build_act1(&mut scene);
    let act2_id = build_act2(&mut scene);
    let act3_id = build_act3(&mut scene);
    let act4_id = build_act4(&mut scene);
    let act5_id = build_act5(&mut scene);
    let act6_id = build_act6(&mut scene);
    let act7_id = build_act7(&mut scene);
    let act8_id = build_act8(&mut scene);

    let (inst1, inst2, inst3, inst4, inst5, inst6, inst7, inst8) = {
        let comp = &mut scene.main_composition;
        let (bg, _, _, _, _, primary, _, _, _) = make_colors();

        comp.build_layer().rect(W, H).fill(bg).at(0.0, 0.0).depth(0.0).add();

        comp.build_layer()
            .rect(W, H)
            .fill(Paint::linear(
                [0.5, 0.0],
                [0.5, 0.6],
                [
                    (0.0, primary.with_alpha(0.035)),
                    (1.0, primary.with_alpha(0.0)),
                ],
            ))
            .at(0.0, 0.0)
            .depth(0.002)
            .add();

        let mut i1 = Instance::new(act1_id);
        i1.time_offset = 0.0;
        let mut i1l = i1.as_layer();
        i1l.depth = 0.010;
        let inst1 = comp.add_layer(i1l);
        comp.add_clip(Clip::new(inst1.clone(), 0.0, Duration::Seconds(2.80)));

        let mut i2 = Instance::new(act2_id);
        i2.time_offset = -2.35;
        let mut i2l = i2.as_layer();
        i2l.depth = 0.011;
        let inst2 = comp.add_layer(i2l);
        comp.add_clip(Clip::new(inst2.clone(), 2.35, Duration::Seconds(7.65)));

        let mut i3 = Instance::new(act3_id);
        i3.time_offset = -9.55;
        let mut i3l = i3.as_layer();
        i3l.depth = 0.012;
        let inst3 = comp.add_layer(i3l);
        comp.add_clip(Clip::new(inst3.clone(), 9.55, Duration::Seconds(5.35)));

        let mut i4 = Instance::new(act4_id);
        i4.time_offset = -14.45;
        let mut i4l = i4.as_layer();
        i4l.depth = 0.013;
        let inst4 = comp.add_layer(i4l);
        comp.add_clip(Clip::new(inst4.clone(), 14.45, Duration::Seconds(10.87)));

        let mut i5 = Instance::new(act5_id);
        i5.time_offset = -24.85;
        let mut i5l = i5.as_layer();
        i5l.depth = 0.014;
        let inst5 = comp.add_layer(i5l);
        comp.add_clip(Clip::new(inst5.clone(), 24.85, Duration::Seconds(5.32)));

        let mut i6 = Instance::new(act6_id);
        i6.time_offset = -29.72;
        let mut i6l = i6.as_layer();
        i6l.depth = 0.015;
        let inst6 = comp.add_layer(i6l);
        comp.add_clip(Clip::new(inst6.clone(), 29.72, Duration::Seconds(11.73)));

        let mut i7 = Instance::new(act7_id);
        i7.time_offset = -41.00;
        let mut i7l = i7.as_layer();
        i7l.depth = 0.016;
        let inst7 = comp.add_layer(i7l);
        comp.add_clip(Clip::new(inst7.clone(), 41.00, Duration::Seconds(5.20)));

        let mut i8 = Instance::new(act8_id);
        i8.time_offset = -45.75;
        let mut i8l = i8.as_layer();
        i8l.depth = 0.017;
        let inst8 = comp.add_layer(i8l);
        comp.add_clip(Clip::new(
            inst8.clone(),
            45.75,
            Duration::Seconds(SCENE_DUR - 45.75),
        ));

        // ── Pro-AE polish overlay ────────────────────────────────────
        // Cinematic vignette bands (top + bottom) and side wisps unify
        // the frame and pull focus toward center — a staple of
        // professional motion design.
        let (_, _, _, ink2, _, _polish_primary, _, _, _) = make_colors();
        let band_h: f32 = 110.0;
        comp.build_layer()
            .rect(W, band_h)
            .fill(Paint::linear(
                [0.5, 0.0],
                [0.5, 1.0],
                [
                    (0.0, Color::hex(HEX_BG).with_alpha(0.55)),
                    (1.0, Color::hex(HEX_BG).with_alpha(0.0)),
                ],
            ))
            .at(0.0, 0.0)
            .depth(0.020)
            .add();
        comp.build_layer()
            .rect(W, band_h)
            .fill(Paint::linear(
                [0.5, 0.0],
                [0.5, 1.0],
                [
                    (0.0, Color::hex(HEX_BG).with_alpha(0.0)),
                    (1.0, Color::hex(HEX_BG).with_alpha(0.55)),
                ],
            ))
            .at(0.0, H - band_h)
            .depth(0.020)
            .add();

        let side_w: f32 = 80.0;
        comp.build_layer()
            .rect(side_w, H)
            .fill(Paint::linear(
                [0.0, 0.5],
                [1.0, 0.5],
                [
                    (0.0, Color::hex(HEX_BG).with_alpha(0.45)),
                    (1.0, Color::hex(HEX_BG).with_alpha(0.0)),
                ],
            ))
            .at(0.0, 0.0)
            .depth(0.020)
            .add();
        comp.build_layer()
            .rect(side_w, H)
            .fill(Paint::linear(
                [0.0, 0.5],
                [1.0, 0.5],
                [
                    (0.0, Color::hex(HEX_BG).with_alpha(0.0)),
                    (1.0, Color::hex(HEX_BG).with_alpha(0.45)),
                ],
            ))
            .at(W - side_w, 0.0)
            .depth(0.020)
            .add();

        // Transition flash sweeps — quick white pulses at every act
        // boundary, timed to the SFX. Pros use these to mask the cut
        // and inject kinetic energy.
        let flash_times: &[f32] =
            &[2.35, 9.55, 14.45, 24.85, 29.72, 41.00, 45.75];
        for &t in flash_times {
            let flash = comp
                .build_layer()
                .rect(W, H)
                .fill(Color::hex(HEX_INK))
                .at(0.0, 0.0)
                .depth(0.024)
                .add();
            comp.animate(flash)
                .clip_start(t - 0.05)
                .kf(t - 0.05, AnimatedProperty::opacity(0.0))
                .kf_eased(t + 0.04, AnimatedProperty::opacity(0.18), Easing::EASE_OUT)
                .kf_eased(t + 0.24, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
                .apply();
        }

        // Persistent micro-eyebrow — top-center campaign tag.
        let eyebrow = comp
            .build_layer()
            .text("AGENTIC EVOLUTION  ·  LONDON  ·  MAY 2", 11.0)
            .width(W)
            .height(28.0)
            .text_align_center()
            .letter_spacing(4.5)
            .fill(ink2.with_alpha(0.55))
            .at(0.0, 26.0)
            .depth(0.026)
            .add();
        comp.animate(eyebrow)
            .fade_in(0.80, 0.60)
            .ease_out()
            .apply();

        // Bottom progress line — left→right fill across the scene.
        let progress = comp
            .build_layer()
            .rect(W, 2.0)
            .fill(Color::hex(HEX_PRIMARY).with_alpha(0.55))
            .anchor_left()
            .at(0.0, H - 2.0)
            .depth(0.026)
            .add();
        comp.animate(progress)
            .kf(0.0, AnimatedProperty::scale(0.0, 1.0))
            .kf_eased(
                SCENE_DUR - 0.4,
                AnimatedProperty::scale(1.0, 1.0),
                Easing::EASE_IN_OUT,
            )
            .apply();

        (inst1, inst2, inst3, inst4, inst5, inst6, inst7, inst8)
    };

    // Varied transitions
    scene.add_transition(Swipe::up(&inst1, &inst2).start(2.35).duration(0.45));
    scene.add_transition(
        Iris::new(&inst2, &inst3)
            .center(W * 0.5, H * 0.5)
            .color(Color::hex(HEX_PRIMARY))
            .start(9.55)
            .duration(0.50),
    );
    scene.add_transition(
        ZoomThrough::new(&inst3, &inst4)
            .center(W * 0.5, H * 0.5)
            .portal_size(380.0, 220.0)
            .start(14.45)
            .duration(0.52),
    );
    scene.add_transition(Swipe::right(&inst4, &inst5).start(24.85).duration(0.45));
    scene.add_transition(
        Ripple::new(&inst5, &inst6)
            .center(W * 0.5, H * 0.5)
            .fill_color(Color::hex(HEX_BG))
            .ring_colors([
                Color::hex(HEX_AMBER),
                Color::hex(HEX_PRIMARY),
                Color::hex(HEX_EMERALD),
            ])
            .start(29.72)
            .duration(0.55),
    );
    scene.add_transition(Swipe::down(&inst6, &inst7).start(41.00).duration(0.45));
    scene.add_transition(
        Iris::new(&inst7, &inst8)
            .center(W * 0.5, H * 0.5)
            .color(Color::hex(HEX_VIOLET))
            .start(45.75)
            .duration(0.50),
    );

    scene
}

pub fn build() -> Scene {
    build_scene()
}
