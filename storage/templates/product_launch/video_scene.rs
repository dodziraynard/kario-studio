// kario_launch_video.rs — Founder-targeted product launch video for Kario AI
// (karioai.com). 1280×720, ~44 s. Voice-over-driven motion graphics built with
// the kario_base builder pattern API. All visual timings are locked to the
// whisperx alignment of the rendered VO (kario_launch_video_vo.json).
//
// Direction: light/professional. Off-white surface (#FAFAFC), white cards
// with hairline slate borders + soft drop shadows, indigo brand accent.
// Iconography is rendered via SVG (Lucide-style line icons). Each beat owns
// the canvas alone (no dashboard pile-up) and uses a distinct entrance
// pattern — kinetic typography, slide-push, iris-reveal, multi-direction
// slams, letterform stagger, trim-path draws, shockwave rings — so the cut
// rhythm reads as motion-design, not static slideshow.
//
// Story arc:
//   Act 1 (0–6 s)   Problem        — kinetic headline + "the launch video".
//   Act 2 (6–16 s)  Exaggerate     — 5 single-focus pain shots: calendar,
//                                    $2,000 slam, six revisions fly-in,
//                                    AE iris, Fri→Mon swap.
//   Act 3 (16–27 s) Solution       — Kario letterform hero, studio (prompt
//                                    + brand kit), three-stage pipeline.
//   Act 4 (27–38 s) Features       — video preview builds with brand
//                                    motion, synced waveform, four feature
//                                    cards from four directions, render.
//   Act 5 (38–44 s) CTA            — founders + creators, karioai.com slam.

use kario_base::{
    animations::{AnimatedProperty, Easing},
    layers::Instance,
    styles::{Color, Paint},
    Asset, Audio, AudioAsset, Clip, Composition, Duration, Id, Iris, Ripple, Scene, Swipe,
    ZoomThrough,
};

// ── Canvas / timing ─────────────────────────────────────────────────────────
const W: f32 = 1280.0;
const H: f32 = 720.0;
const VO_DUR: f32 = 42.912;
const SCENE_DUR: f32 = 44.0;
const VO_SRC: &str = "crates/base/src/assets/kario_launch_video_vo.mp3";

const SCRIPT: &str = "The hardest part of shipping a feature isn't the feature. \
It's the launch video that goes with it. \
A polished one eats your whole week. A freelancer wants two thousand dollars. \
And six rounds of notes. Open After Effects yourself? Your launch slips past Friday. \
Kario does it differently. Type a prompt. Drop in your logo and brand colors. \
Kario writes the script, generates the voice-over, and animates the whole thing, end to end. \
Custom motion in your brand. Audio perfectly synced with visuals. \
Captions, charts, transitions, music, composed for you. Rendered in minutes. \
Founders and creators ship launch videos at karioai.com.";

// ── Final-fade timing ──────────────────────────────────────────────────────
const FINAL_OUT_T: f32 = 43.45;
const FINAL_OUT_DUR: f32 = 0.55;

// ── Palette (light, Linear/Stripe-inspired) ────────────────────────────────
const HEX_BG: &str = "#FAFAFC";
const HEX_SURFACE: &str = "#FFFFFF";
const HEX_BORDER: &str = "#E5E7EB";
const HEX_INK: &str = "#0F172A"; // slate-900
const HEX_INK2: &str = "#334155"; // slate-700
const HEX_MUTED: &str = "#64748B"; // slate-500
const HEX_SOFT: &str = "#94A3B8"; // slate-400
const HEX_GRAY100: &str = "#F1F5F9"; // slate-100
const HEX_GRAY50: &str = "#F8FAFC"; // slate-50
const HEX_PRIMARY: &str = "#4F46E5"; // indigo-600
const HEX_VIOLET: &str = "#7C3AED"; // violet-600
const HEX_EMERALD: &str = "#10B981";
const HEX_RED: &str = "#EF4444";
const _HEX_RED_BG: &str = "#FEF2F2";
const HEX_RED_INK: &str = "#B91C1C";
const HEX_AMBER: &str = "#F59E0B";
const HEX_AMBER_BG: &str = "#FFFBEB";

// ── SVG icon factory (Lucide-style 24×24 line icons) ───────────────────────
fn svg_check_circle(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><circle cx="12" cy="12" r="9" fill="{c}"/><path d="M8 12l3 3 5-6" stroke="#FFFFFF" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round"/></svg>"##
    )
}
fn svg_x(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M6 6l12 12M18 6L6 18" stroke="{c}" stroke-width="2.5" stroke-linecap="round"/></svg>"##
    )
}
fn svg_video(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><rect x="2" y="6" width="14" height="12" rx="2" stroke="{c}" stroke-width="2"/><path d="M22 8l-6 4 6 4V8z" stroke="{c}" stroke-width="2" stroke-linejoin="round"/></svg>"##
    )
}
fn svg_calendar(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><rect x="3" y="4" width="18" height="18" rx="2" stroke="{c}" stroke-width="2"/><path d="M3 10h18M8 2v4M16 2v4" stroke="{c}" stroke-width="2" stroke-linecap="round"/></svg>"##
    )
}
fn svg_dollar(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M12 2v20M17 6H10a3 3 0 000 6h4a3 3 0 010 6H7" stroke="{c}" stroke-width="2" stroke-linecap="round"/></svg>"##
    )
}
fn svg_comment(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M21 15a2 2 0 01-2 2H7l-4 4V5a2 2 0 012-2h14a2 2 0 012 2v10z" stroke="{c}" stroke-width="2" stroke-linejoin="round"/></svg>"##
    )
}
fn svg_clock(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><circle cx="12" cy="12" r="9" stroke="{c}" stroke-width="2"/><path d="M12 7v5l3 2" stroke="{c}" stroke-width="2" stroke-linecap="round"/></svg>"##
    )
}
#[allow(dead_code)]
fn svg_layers(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M12 2l10 6-10 6L2 8l10-6z M2 14l10 6 10-6 M2 17l10 6 10-6" stroke="{c}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>"##
    )
}
fn svg_sparkle(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="{c}"><path d="M12 3l1.5 5.5L19 10l-5.5 1.5L12 17l-1.5-5.5L5 10l5.5-1.5L12 3z"/><path d="M19 4l.7 1.8L21.5 6.5l-1.8.7L19 9l-.7-1.8L16.5 6.5l1.8-.7L19 4z"/></svg>"##
    )
}
fn svg_palette(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M12 2C6.5 2 2 6.5 2 12c0 5.5 4.5 10 10 10 1.1 0 2-.9 2-2 0-.6-.2-1-.5-1.4-.3-.4-.5-.9-.5-1.4 0-1.1.9-2 2-2h2c2.8 0 5-2.2 5-5C22 5.7 17.5 2 12 2z" stroke="{c}" stroke-width="2"/><circle cx="6.5" cy="11.5" r="1.5" fill="{c}"/><circle cx="9.5" cy="7" r="1.5" fill="{c}"/><circle cx="14.5" cy="7" r="1.5" fill="{c}"/><circle cx="17.5" cy="11.5" r="1.5" fill="{c}"/></svg>"##
    )
}
fn svg_doc(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z" stroke="{c}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/><path d="M14 2v6h6M9 13h6M9 17h6" stroke="{c}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>"##
    )
}
fn svg_mic(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><rect x="9" y="3" width="6" height="11" rx="3" stroke="{c}" stroke-width="2"/><path d="M5 11a7 7 0 0014 0M12 18v3M8 21h8" stroke="{c}" stroke-width="2" stroke-linecap="round"/></svg>"##
    )
}
fn svg_play(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><circle cx="12" cy="12" r="9" stroke="{c}" stroke-width="2"/><path d="M10 8l6 4-6 4V8z" fill="{c}"/></svg>"##
    )
}
fn svg_captions(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><rect x="2" y="5" width="20" height="14" rx="2" stroke="{c}" stroke-width="2"/><path d="M7 12a2 2 0 012-2h1M14 12a2 2 0 012-2h1M7 14a2 2 0 002 2h1M14 14a2 2 0 002 2h1" stroke="{c}" stroke-width="2" stroke-linecap="round"/></svg>"##
    )
}
fn svg_chart(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><rect x="3" y="14" width="4" height="7" rx="1" stroke="{c}" stroke-width="2"/><rect x="10" y="9" width="4" height="12" rx="1" stroke="{c}" stroke-width="2"/><rect x="17" y="4" width="4" height="17" rx="1" stroke="{c}" stroke-width="2"/></svg>"##
    )
}
fn svg_transitions(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M17 3l4 4-4 4M21 7H8M7 13l-4 4 4 4M3 17h13" stroke="{c}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>"##
    )
}
fn svg_music(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M9 18V5l12-2v13" stroke="{c}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/><circle cx="6" cy="18" r="3" stroke="{c}" stroke-width="2"/><circle cx="18" cy="16" r="3" stroke="{c}" stroke-width="2"/></svg>"##
    )
}
fn svg_user(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><circle cx="12" cy="8" r="4" stroke="{c}" stroke-width="2"/><path d="M4 21a8 8 0 0116 0" stroke="{c}" stroke-width="2" stroke-linecap="round"/></svg>"##
    )
}
fn svg_arrow_right(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M5 12h14M12 5l7 7-7 7" stroke="{c}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>"##
    )
}
fn svg_check_big(c: &str) -> String {
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M5 13l4 4L19 7" stroke="{c}" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/></svg>"##
    )
}
fn svg_cursor(c: &str) -> String {
    // Classic arrow pointer — tip at (5,3). Filled body with white outline so it
    // reads on light or dark surfaces.
    format!(
        r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="M5 3l14 8-7 2-3 7-4-17z" fill="{c}" stroke="#FFFFFF" stroke-width="1.4" stroke-linejoin="round"/></svg>"##
    )
}

fn make_comp_colors() -> (
    Color,
    Color,
    Color,
    Color,
    Color,
    Color,
    Color,
    Color,
    Color,
    Color,
    Color,
    Color,
    Color,
    Color,
    Color,
    Color,
    Color,
) {
    let bg = Color::hex(HEX_BG);
    let surface = Color::hex(HEX_SURFACE);
    let border = Color::hex(HEX_BORDER);
    let ink = Color::hex(HEX_INK);
    let ink2 = Color::hex(HEX_INK2);
    let muted = Color::hex(HEX_MUTED);
    let soft = Color::hex(HEX_SOFT);
    let gray100 = Color::hex(HEX_GRAY100);
    let gray50 = Color::hex(HEX_GRAY50);
    let primary = Color::hex(HEX_PRIMARY);
    let violet = Color::hex(HEX_VIOLET);
    let emerald = Color::hex(HEX_EMERALD);
    let red = Color::hex(HEX_RED);
    let red_ink = Color::hex(HEX_RED_INK);
    let amber = Color::hex(HEX_AMBER);
    let amber_bg = Color::hex(HEX_AMBER_BG);
    let shadow = Color::hex("#0F172A");
    (
        bg, surface, border, ink, ink2, muted, soft, gray100, gray50, primary, violet, emerald,
        red, red_ink, amber, amber_bg, shadow,
    )
}

fn build_act1(scene: &mut Scene) -> Id {
    let comp_id = Id::new();
    let mut comp = Composition::new(W, H);
    comp.id = comp_id.clone();
    comp.duration = Duration::Seconds(SCENE_DUR);
    let cx = W * 0.5;
    let cy = H * 0.5;
    let (_, _surface, _, ink, _, muted, _, _, _, primary, _, _, red, red_ink, _, _, _) =
        make_comp_colors();

    // =========================================================================
    // ACT 1 — PROBLEM
    // =========================================================================

    // ── Beat 1A (0.1 – 3.0): Kinetic 2-line headline ────────────────────────
    // Line 1 fades up + letter-spacing collapses from loose to tight.
    // Line 2 ("isn't the feature.") slams in with red emphasis at 2.1s.
    let beat1a_out = 3.20_f32;

    let line1 = comp
        .build_layer()
        .text("The hardest part of shipping a feature", 36.0)
        .width(1100.0)
        .height(60.0)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(ink)
        .at(cx - 550.0, cy - 76.0)
        .depth(0.10)
        .add();
    comp.animate(line1)
        .fade_in(0.10, 0.50)
        .ease_out()
        .slide_from(0.0, 18.0, 0.10, 0.55)
        .ease_out()
        // Letter-spacing collapse
        .kf(0.10, AnimatedProperty::letter_spacing(8.0))
        .kf_eased(
            1.20,
            AnimatedProperty::letter_spacing(0.0),
            Easing::EASE_OUT,
        )
        .kf(beat1a_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat1a_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Eyebrow above line 1, sets up the punchline
    let eyebrow_a = comp
        .build_layer()
        .text("FOR FOUNDERS", 12.0)
        .width(280.0)
        .height(20.0)
        .bold()
        .letter_spacing(4.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(primary)
        .at(cx - 140.0, cy - 144.0)
        .depth(0.10)
        .add();
    comp.animate(eyebrow_a)
        .fade_in(0.0, 0.40)
        .ease_out()
        .kf(beat1a_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat1a_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Line 2 — slam-in with emphasis, scaling down from 1.5 to 1.0
    let line2 = comp
        .build_layer()
        .text("isn't the feature.", 56.0)
        .width(1100.0)
        .height(80.0)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(red_ink)
        .at(cx - 550.0, cy + 4.0)
        .depth(0.11)
        .add();
    comp.animate(line2)
        .fade_in(2.05, 0.18)
        .ease_out()
        .scale_from(1.4, 2.05, 0.35)
        .spring(420.0, 18.0)
        .kf(beat1a_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat1a_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Underline under line 2 — draws in via trim_path
    let underline = comp
        .build_layer()
        .line_path(0.0, 0.0, 380.0, 0.0)
        .stroke(red, 4.0)
        .at(cx - 190.0, cy + 70.0)
        .depth(0.12)
        .add();
    comp.animate(underline)
        .clip_start(2.30)
        .kf(2.30, AnimatedProperty::trim_path_end(0.0))
        .kf_eased(2.85, AnimatedProperty::trim_path_end(1.0), Easing::EASE_OUT)
        .kf(beat1a_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat1a_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // ── Beat 1B (3.4 – 5.85): "It's the launch video" — slide-push from right ─
    let beat1b_in = 3.40_f32;
    let beat1b_out = 5.65_f32;

    // Eyebrow
    let eb_b = comp
        .build_layer()
        .text("THE REAL BLOCKER", 12.0)
        .width(320.0)
        .height(20.0)
        .bold()
        .letter_spacing(4.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(red)
        .at(cx - 160.0, cy - 132.0)
        .depth(0.10)
        .add();
    comp.animate(eb_b)
        .fade_in(beat1b_in, 0.30)
        .ease_out()
        .slide_from(60.0, 0.0, beat1b_in, 0.45)
        .ease_out()
        .kf(beat1b_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat1b_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Big video icon (96px) on the left of phrase
    let video_icon = comp
        .build_layer()
        .svg(svg_video(HEX_RED))
        .width(96.0)
        .height(96.0)
        .at(cx - 360.0, cy - 48.0)
        .depth(0.11)
        .add();
    comp.animate(video_icon)
        .fade_in(beat1b_in + 0.05, 0.35)
        .ease_out()
        .scale_from(0.3, beat1b_in + 0.05, 0.45)
        .spring(360.0, 18.0)
        .kf(beat1b_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat1b_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Big phrase "the launch video"
    let phrase_b = comp
        .build_layer()
        .text("the launch video", 60.0)
        .width(720.0)
        .height(80.0)
        .bold()
        .vertical_align_middle()
        .fill(ink)
        .at(cx - 240.0, cy - 40.0)
        .depth(0.11)
        .add();
    comp.animate(phrase_b)
        .fade_in(beat1b_in + 0.10, 0.40)
        .ease_out()
        .slide_from(120.0, 0.0, beat1b_in + 0.10, 0.55)
        .ease_out()
        .kf(beat1b_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat1b_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Sub-line with TODO badge style
    let sub_b = comp
        .build_layer()
        .text("…still on your TODO list", 22.0)
        .width(720.0)
        .height(34.0)
        .vertical_align_middle()
        .fill(muted)
        .at(cx - 240.0, cy + 50.0)
        .depth(0.11)
        .add();
    comp.animate(sub_b)
        .fade_in(beat1b_in + 0.55, 0.40)
        .ease_out()
        .slide_from(0.0, 12.0, beat1b_in + 0.55, 0.50)
        .ease_out()
        .kf(beat1b_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat1b_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    scene
        .assets
        .insert(comp_id.clone(), Asset::Composition(comp));
    comp_id
}

fn build_act2(scene: &mut Scene) -> Id {
    let comp_id = Id::new();
    let mut comp = Composition::new(W, H);
    comp.id = comp_id.clone();
    comp.duration = Duration::Seconds(SCENE_DUR);
    let cx = W * 0.5;
    let cy = H * 0.5;
    let (
        _,
        surface,
        border,
        ink,
        _,
        muted,
        soft,
        gray100,
        _,
        primary,
        violet,
        emerald,
        red,
        red_ink,
        amber,
        amber_bg,
        shadow,
    ) = make_comp_colors();

    // =========================================================================
    // ACT 2 — EXAGGERATE (one focal beat at a time)
    // =========================================================================

    // ── Beat 2A (5.85 – 7.95): Calendar — drop from top, cells stamp ────────
    let beat2a_in = 5.85_f32;
    let beat2a_out = 7.95_f32;
    let cal_w = 800.0;
    let cal_h = 280.0;
    let cal_x = cx - cal_w * 0.5;
    let cal_y = cy - cal_h * 0.5 + 8.0;

    let cal_card = comp
        .build_layer()
        .rect(cal_w, cal_h)
        .corner_radius(20.0)
        .fill(surface)
        .stroke(border, 1.0)
        .drop_shadow([0.0, 14.0], shadow.with_alpha(0.10), 32.0, 0.0)
        .at(cal_x, cal_y)
        .depth(0.10)
        .add();
    comp.animate(cal_card)
        .fade_in(beat2a_in, 0.35)
        .ease_out()
        .slide_from(0.0, -80.0, beat2a_in, 0.55)
        .with_easing(Easing::Spring {
            stiffness: 320.0,
            damping: 22.0,
            mass: 1.0,
        })
        .kf(beat2a_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat2a_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Eyebrow with calendar icon — top-left of card
    let cal_icon = comp
        .build_layer()
        .svg(svg_calendar(HEX_RED))
        .width(20.0)
        .height(20.0)
        .at(cal_x + 30.0, cal_y + 24.0)
        .depth(0.105)
        .add();
    comp.animate(cal_icon)
        .fade_in(beat2a_in + 0.20, 0.30)
        .ease_out()
        .kf(beat2a_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat2a_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    let cal_eyebrow = comp
        .build_layer()
        .text("ONE WEEK GONE", 12.0)
        .width(300.0)
        .height(20.0)
        .bold()
        .letter_spacing(3.0)
        .vertical_align_middle()
        .fill(ink)
        .at(cal_x + 60.0, cal_y + 24.0)
        .depth(0.105)
        .add();
    comp.animate(cal_eyebrow)
        .fade_in(beat2a_in + 0.20, 0.30)
        .ease_out()
        .kf(beat2a_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat2a_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // 5 day cells — stamping in to whisperx syllables
    let day_letters = ["MON", "TUE", "WED", "THU", "FRI"];
    let cell_times = [5.95_f32, 6.20, 6.50, 6.80, 7.10];
    let cell_w = 130.0;
    let cell_h = 130.0;
    let cell_gap = 14.0;
    let cells_total_w = 5.0 * cell_w + 4.0 * cell_gap;
    let cell_x0 = cal_x + (cal_w - cells_total_w) * 0.5;
    let cell_y = cal_y + 80.0;

    for i in 0..5 {
        let cxi = cell_x0 + i as f32 * (cell_w + cell_gap);

        // Gray base cell (always visible during beat)
        let base = comp
            .build_layer()
            .rect(cell_w, cell_h)
            .corner_radius(14.0)
            .fill(gray100)
            .stroke(border, 1.0)
            .at(cxi, cell_y)
            .depth(0.105)
            .add();
        comp.animate(base)
            .fade_in(beat2a_in + 0.25, 0.30)
            .ease_out()
            .kf(beat2a_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat2a_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        // Slate label (always visible)
        let slate_letter = comp
            .build_layer()
            .text(day_letters[i], 22.0)
            .width(cell_w)
            .height(cell_h)
            .bold()
            .letter_spacing(1.5)
            .text_align_center()
            .vertical_align_middle()
            .fill(soft)
            .at(cxi, cell_y)
            .depth(0.108)
            .add();
        comp.animate(slate_letter)
            .fade_in(beat2a_in + 0.30, 0.30)
            .ease_out()
            .kf(beat2a_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat2a_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        // Red overlay cell — stamps in at hit time
        let red_cell = comp
            .build_layer()
            .rect(cell_w, cell_h)
            .corner_radius(14.0)
            .fill(red)
            .at(cxi, cell_y)
            .depth(0.11)
            .add();
        comp.animate(red_cell)
            .fade_in(cell_times[i], 0.18)
            .ease_out()
            .scale_from(0.5, cell_times[i], 0.30)
            .spring(420.0, 16.0)
            .kf(beat2a_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat2a_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        // White day label on red
        let white_letter = comp
            .build_layer()
            .text(day_letters[i], 22.0)
            .width(cell_w)
            .height(cell_h)
            .bold()
            .letter_spacing(1.5)
            .text_align_center()
            .vertical_align_middle()
            .fill(surface)
            .at(cxi, cell_y)
            .depth(0.115)
            .add();
        comp.animate(white_letter)
            .fade_in(cell_times[i] + 0.05, 0.18)
            .ease_out()
            .kf(beat2a_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat2a_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        // X stamp
        let x_size = 24.0;
        let x_icon = comp
            .build_layer()
            .svg(svg_x("#FFFFFF"))
            .width(x_size)
            .height(x_size)
            .at(
                cxi + cell_w - x_size - 12.0,
                cell_y + cell_h - x_size - 12.0,
            )
            .depth(0.12)
            .add();
        comp.animate(x_icon)
            .fade_in(cell_times[i] + 0.10, 0.18)
            .ease_out()
            .scale_from(0.0, cell_times[i] + 0.10, 0.25)
            .spring(450.0, 14.0)
            .kf(beat2a_out, AnimatedProperty::opacity(0.85))
            .kf_eased(
                beat2a_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();
    }

    // Foot label inside card
    let cal_foot = comp
        .build_layer()
        .text("5 days · gone", 16.0)
        .width(cal_w - 60.0)
        .height(22.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(muted)
        .at(cal_x + 30.0, cal_y + cal_h - 36.0)
        .depth(0.105)
        .add();
    comp.animate(cal_foot)
        .fade_in(7.20, 0.30)
        .ease_out()
        .kf(beat2a_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat2a_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // ── Beat 2B (8.20 – 9.85): "$2,000" slam + shockwave ring ───────────────
    let beat2b_in = 8.20_f32;
    let beat2b_out = 9.75_f32;

    // Eyebrow
    let price_eyebrow = comp
        .build_layer()
        .text("FREELANCER QUOTE", 12.0)
        .width(360.0)
        .height(20.0)
        .bold()
        .letter_spacing(4.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(red)
        .at(cx - 180.0, cy - 138.0)
        .depth(0.10)
        .add();
    comp.animate(price_eyebrow)
        .fade_in(beat2b_in, 0.30)
        .ease_out()
        .kf(beat2b_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat2b_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Shockwave ring (expands + fades) — coincides with the "$2,000" word slam
    let shock1 = comp
        .build_layer()
        .circle(20.0)
        .no_fill()
        .stroke(red.with_alpha(0.6), 3.0)
        .at(cx - 10.0, cy - 10.0)
        .depth(0.105)
        .add();
    comp.animate(shock1)
        .fade_in(8.42, 0.10)
        .ease_out()
        .kf(8.42, AnimatedProperty::scale(0.5, 0.5))
        .kf_eased(9.10, AnimatedProperty::scale(20.0, 20.0), Easing::EASE_OUT)
        .kf(8.42, AnimatedProperty::opacity(0.7))
        .kf_eased(9.10, AnimatedProperty::opacity(0.0), Easing::EASE_OUT)
        .apply();

    // Big "$2,000" slam-in
    let price_text = comp
        .build_layer()
        .text("$2,000", 196.0)
        .width(900.0)
        .height(220.0)
        .bold()
        .letter_spacing(2.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(red_ink)
        .at(cx - 450.0, cy - 110.0)
        .depth(0.11)
        .add();
    comp.animate(price_text)
        .fade_in(beat2b_in + 0.15, 0.20)
        .ease_out()
        .scale_from(1.45, beat2b_in + 0.15, 0.35)
        .spring(420.0, 18.0)
        .kf(beat2b_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat2b_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Side dollar icon for visual anchor
    let dollar_icon = comp
        .build_layer()
        .svg(svg_dollar(HEX_RED))
        .width(48.0)
        .height(48.0)
        .at(cx - 360.0, cy - 24.0)
        .depth(0.115)
        .add();
    comp.animate(dollar_icon)
        .fade_in(beat2b_in + 0.30, 0.25)
        .ease_out()
        .scale_from(0.3, beat2b_in + 0.30, 0.40)
        .spring(360.0, 18.0)
        .kf(beat2b_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat2b_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Subtitle
    let price_sub = comp
        .build_layer()
        .text("for one launch video", 22.0)
        .width(700.0)
        .height(30.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(muted)
        .at(cx - 350.0, cy + 130.0)
        .depth(0.11)
        .add();
    comp.animate(price_sub)
        .fade_in(beat2b_in + 0.55, 0.30)
        .ease_out()
        .slide_from(0.0, 16.0, beat2b_in + 0.55, 0.40)
        .ease_out()
        .kf(beat2b_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat2b_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // ── Beat 2C (9.85 – 12.20): six revisions fly in from random angles ──────
    let beat2c_in = 9.85_f32;
    let beat2c_out = 12.10_f32;

    let rev_eyebrow = comp
        .build_layer()
        .text("SIX ROUNDS OF NOTES", 12.0)
        .width(420.0)
        .height(20.0)
        .bold()
        .letter_spacing(4.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(amber)
        .at(cx - 210.0, cy - 174.0)
        .depth(0.10)
        .add();
    comp.animate(rev_eyebrow)
        .fade_in(beat2c_in, 0.30)
        .ease_out()
        .kf(beat2c_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat2c_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Six revision badges fly in from six different angles, settle in arc
    let rev_w = 130.0;
    let rev_h = 64.0;
    let rev_labels = ["v1", "v2", "v3", "v4", "v5", "v6"];
    // Final positions: arc of 6 around center
    let rev_finals: [(f32, f32, f32); 6] = [
        (cx - 380.0, cy - 30.0, -10.0),
        (cx - 220.0, cy + 40.0, 7.0),
        (cx - 60.0, cy - 60.0, -5.0),
        (cx + 100.0, cy + 30.0, 12.0),
        (cx + 260.0, cy - 50.0, -8.0),
        (cx + 380.0, cy + 50.0, 6.0),
    ];
    // Off-screen origins (fly-from positions): different direction per badge
    let rev_origins: [(f32, f32); 6] = [
        (-300.0, -250.0), //  badge 1: from top-left
        (-380.0, 220.0),  //  badge 2: from bottom-left
        (0.0, -380.0),    //  badge 3: from above
        (380.0, 220.0),   //  badge 4: from bottom-right
        (380.0, -250.0),  //  badge 5: from top-right
        (0.0, 340.0),     //  badge 6: from below
    ];
    let rev_times = [9.95_f32, 10.30, 10.65, 11.00, 11.35, 11.65];

    for i in 0..6 {
        let (fx, fy, rot) = rev_finals[i];
        let (ox, oy) = rev_origins[i];
        let t_in = rev_times[i];

        let badge = comp
            .build_layer()
            .rect(rev_w, rev_h)
            .corner_radius(32.0)
            .fill(amber_bg)
            .stroke(amber, 1.5)
            .drop_shadow([0.0, 6.0], shadow.with_alpha(0.10), 14.0, 0.0)
            .at(fx, fy)
            .depth(0.10)
            .add();
        comp.animate(badge)
            .fade_in(t_in, 0.20)
            .ease_out()
            .kf(t_in, AnimatedProperty::position(fx + ox, fy + oy))
            .kf_eased(
                t_in + 0.55,
                AnimatedProperty::position(fx, fy),
                Easing::Spring {
                    stiffness: 280.0,
                    damping: 18.0,
                    mass: 1.0,
                },
            )
            .kf(t_in, AnimatedProperty::rotation_z(rot * 3.0))
            .kf_eased(
                t_in + 0.55,
                AnimatedProperty::rotation_z(rot),
                Easing::EASE_OUT,
            )
            .kf(beat2c_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat2c_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        let lbl = comp
            .build_layer()
            .text(rev_labels[i], 26.0)
            .width(rev_w)
            .height(rev_h)
            .bold()
            .letter_spacing(1.0)
            .text_align_center()
            .vertical_align_middle()
            .fill(Color::hex("#92400E"))
            .at(fx, fy)
            .depth(0.11)
            .add();
        comp.animate(lbl)
            .fade_in(t_in + 0.10, 0.20)
            .ease_out()
            .kf(t_in, AnimatedProperty::position(fx + ox, fy + oy))
            .kf_eased(
                t_in + 0.55,
                AnimatedProperty::position(fx, fy),
                Easing::Spring {
                    stiffness: 280.0,
                    damping: 18.0,
                    mass: 1.0,
                },
            )
            .kf(t_in, AnimatedProperty::rotation_z(rot * 3.0))
            .kf_eased(
                t_in + 0.55,
                AnimatedProperty::rotation_z(rot),
                Easing::EASE_OUT,
            )
            .kf(beat2c_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat2c_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();
    }

    // Center label "REVISIONS" with a comment icon — pops in after badges settle
    let rev_center_icon = comp
        .build_layer()
        .svg(svg_comment(HEX_AMBER))
        .width(32.0)
        .height(32.0)
        .at(cx - 16.0, cy + 130.0)
        .depth(0.10)
        .add();
    comp.animate(rev_center_icon)
        .fade_in(11.55, 0.30)
        .ease_out()
        .scale_from(0.3, 11.55, 0.35)
        .spring(360.0, 18.0)
        .kf(beat2c_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat2c_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // ── Beat 2D (12.30 – 14.00): "Open After Effects yourself?" — DIY pain ──
    // Faded AE chrome behind a stack of red error modals that slam in
    // overlapping and tilted. Communicates overwhelm at the literal AE moment
    // of the VO, rather than a polished iris reveal.
    let beat2d_in = 12.30_f32;
    let beat2d_out = 14.00_f32;
    let ae_bar_w = 880.0_f32;
    let ae_bar_x = cx - ae_bar_w * 0.5;
    let ae_bar_y = cy - 220.0;

    // Faded AE chrome bar (dim title bar)
    let ae_bar = comp
        .build_layer()
        .rect(ae_bar_w, 44.0)
        .corner_radius(10.0)
        .fill(Color::hex("#1E293B").with_alpha(0.92))
        .at(ae_bar_x, ae_bar_y)
        .depth(0.10)
        .add();
    comp.animate(ae_bar)
        .fade_in(beat2d_in, 0.20)
        .ease_out()
        .kf(beat2d_out, AnimatedProperty::opacity(0.85))
        .kf_eased(
            beat2d_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    for i in 0..3 {
        let dc = match i {
            0 => Color::hex("#EF4444"),
            1 => Color::hex("#F59E0B"),
            _ => Color::hex("#10B981"),
        };
        let dot = comp
            .build_layer()
            .circle(11.0)
            .fill(dc)
            .at(ae_bar_x + 22.0 + i as f32 * 18.0, ae_bar_y + 16.0)
            .depth(0.105)
            .add();
        comp.animate(dot)
            .fade_in(beat2d_in + 0.04, 0.18)
            .ease_out()
            .kf(beat2d_out, AnimatedProperty::opacity(0.95))
            .kf_eased(
                beat2d_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();
    }

    let ae_label = comp
        .build_layer()
        .text("After Effects — launch_video.aep   (Not Responding)", 13.0)
        .width(560.0)
        .height(20.0)
        .vertical_align_middle()
        .fill(Color::hex("#94A3B8"))
        .at(ae_bar_x + 90.0, ae_bar_y + 12.0)
        .depth(0.105)
        .add();
    comp.animate(ae_label)
        .fade_in(beat2d_in + 0.10, 0.22)
        .ease_out()
        .kf(beat2d_out, AnimatedProperty::opacity(0.9))
        .kf_eased(
            beat2d_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Dim composition area behind the modals — many faded tracks
    let comp_x = ae_bar_x + 20.0;
    let comp_y = ae_bar_y + 60.0;
    let comp_w = ae_bar_w - 40.0;
    let comp_bg = comp
        .build_layer()
        .rect(comp_w, 320.0)
        .corner_radius(10.0)
        .fill(Color::hex("#0F172A").with_alpha(0.78))
        .at(comp_x, comp_y)
        .depth(0.102)
        .add();
    comp.animate(comp_bg)
        .fade_in(beat2d_in + 0.05, 0.20)
        .ease_out()
        .kf(beat2d_out, AnimatedProperty::opacity(0.78))
        .kf_eased(
            beat2d_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    let track_widths = [
        0.85_f32, 0.62, 0.70, 0.40, 0.78, 0.55, 0.92, 0.48, 0.66, 0.74, 0.36, 0.82,
    ];
    let track_colors = [
        primary, violet, amber, emerald, red, primary, violet, amber, emerald, primary, violet,
        amber,
    ];
    for i in 0..12 {
        let row_y = comp_y + 18.0 + i as f32 * 22.0;
        let bar = comp
            .build_layer()
            .rect(track_widths[i] * (comp_w - 40.0), 10.0)
            .corner_radius(3.0)
            .fill(track_colors[i].with_alpha(0.45))
            .at(comp_x + 20.0, row_y)
            .depth(0.105)
            .add();
        comp.animate(bar)
            .fade_in(beat2d_in + 0.10 + i as f32 * 0.025, 0.16)
            .ease_out()
            .kf(beat2d_out, AnimatedProperty::opacity(0.42))
            .kf_eased(
                beat2d_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();
    }

    // Three red error modals slam in stacked + tilted, each with a small
    // shake on land.
    let modal_data: [(&str, &str, f32, f32, f32, f32); 3] = [
        (
            "Composition not rendering",
            "Effect needs licensed plugin",
            12.50,
            cx - 280.0,
            cy - 70.0,
            -3.5,
        ),
        (
            "Missing 3 fonts",
            "Inter, Display, Mono",
            12.78,
            cx - 180.0,
            cy + 8.0,
            4.2,
        ),
        (
            "Render eta: 2h 40m",
            "12% of frames cached",
            13.06,
            cx - 120.0,
            cy + 90.0,
            -2.6,
        ),
    ];
    let mw = 460.0_f32;
    let mh = 92.0_f32;
    for &(title, sub, t_in, x, y, rot) in modal_data.iter() {
        let modal = comp
            .build_layer()
            .rect(mw, mh)
            .corner_radius(12.0)
            .fill(Color::hex("#FEF2F2"))
            .stroke(red, 1.5)
            .drop_shadow([0.0, 14.0], shadow.with_alpha(0.20), 28.0, 0.0)
            .at(x, y)
            .depth(0.12)
            .add();
        comp.animate(modal)
            .fade_in(t_in, 0.10)
            .ease_out()
            .kf(t_in, AnimatedProperty::scale(0.55, 0.55))
            .kf_eased(
                t_in + 0.18,
                AnimatedProperty::scale(1.06, 1.06),
                Easing::EASE_OUT,
            )
            .kf_eased(
                t_in + 0.30,
                AnimatedProperty::scale(1.0, 1.0),
                Easing::EASE_OUT,
            )
            .kf(t_in, AnimatedProperty::rotation_z(0.0))
            .kf_eased(
                t_in + 0.22,
                AnimatedProperty::rotation_z(rot),
                Easing::EASE_OUT,
            )
            .kf(beat2d_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat2d_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        let xicon = comp
            .build_layer()
            .svg(svg_x(HEX_RED))
            .width(22.0)
            .height(22.0)
            .at(x + 18.0, y + 18.0)
            .depth(0.13)
            .add();
        comp.animate(xicon)
            .fade_in(t_in + 0.08, 0.10)
            .ease_out()
            .kf(t_in, AnimatedProperty::rotation_z(0.0))
            .kf_eased(
                t_in + 0.22,
                AnimatedProperty::rotation_z(rot),
                Easing::EASE_OUT,
            )
            .kf(beat2d_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat2d_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        let ttl = comp
            .build_layer()
            .text(title, 18.0)
            .width(mw - 80.0)
            .height(24.0)
            .bold()
            .vertical_align_middle()
            .fill(red_ink)
            .at(x + 50.0, y + 14.0)
            .depth(0.13)
            .add();
        comp.animate(ttl)
            .fade_in(t_in + 0.10, 0.14)
            .ease_out()
            .kf(t_in, AnimatedProperty::rotation_z(0.0))
            .kf_eased(
                t_in + 0.22,
                AnimatedProperty::rotation_z(rot),
                Easing::EASE_OUT,
            )
            .kf(beat2d_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat2d_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        let sub_t = comp
            .build_layer()
            .text(sub, 13.0)
            .width(mw - 80.0)
            .height(20.0)
            .vertical_align_middle()
            .fill(Color::hex("#7F1D1D"))
            .at(x + 50.0, y + 50.0)
            .depth(0.13)
            .add();
        comp.animate(sub_t)
            .fade_in(t_in + 0.12, 0.14)
            .ease_out()
            .kf(t_in, AnimatedProperty::rotation_z(0.0))
            .kf_eased(
                t_in + 0.22,
                AnimatedProperty::rotation_z(rot),
                Easing::EASE_OUT,
            )
            .kf(beat2d_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat2d_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();
    }

    // ── Beat 2E (14.20 – 16.05): Fri → Mon swap (slide-push horizontal) ─────
    let beat2e_in = 14.20_f32;
    let beat2e_out = 15.95_f32;

    // Eyebrow
    let dead_eb = comp
        .build_layer()
        .text("DEADLINE SLIPS", 12.0)
        .width(380.0)
        .height(20.0)
        .bold()
        .letter_spacing(4.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(red)
        .at(cx - 190.0, cy - 186.0)
        .depth(0.10)
        .add();
    comp.animate(dead_eb)
        .fade_in(beat2e_in, 0.30)
        .ease_out()
        .kf(beat2e_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat2e_out + 0.20,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Clock icon top center
    let clock_icon = comp
        .build_layer()
        .svg(svg_clock(HEX_RED))
        .width(56.0)
        .height(56.0)
        .at(cx - 28.0, cy - 158.0)
        .depth(0.105)
        .add();
    comp.animate(clock_icon)
        .fade_in(beat2e_in + 0.10, 0.30)
        .ease_out()
        .scale_from(0.4, beat2e_in + 0.10, 0.40)
        .spring(360.0, 18.0)
        .kf(beat2e_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat2e_out + 0.20,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // FRI big text — slides in from left, then slides off
    let fri = comp
        .build_layer()
        .text("FRI", 144.0)
        .width(360.0)
        .height(180.0)
        .bold()
        .letter_spacing(4.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(soft)
        .at(cx - 180.0, cy - 90.0)
        .depth(0.11)
        .add();
    comp.animate(fri)
        .fade_in(beat2e_in + 0.10, 0.30)
        .ease_out()
        .slide_from(-100.0, 0.0, beat2e_in + 0.10, 0.40)
        .ease_out()
        // Slide to the left and fade as MON enters
        .kf(15.10, AnimatedProperty::position(cx - 180.0, cy - 90.0))
        .kf_eased(
            15.55,
            AnimatedProperty::position(cx - 380.0, cy - 90.0),
            Easing::EASE_IN,
        )
        .kf(15.10, AnimatedProperty::opacity(1.0))
        .kf_eased(15.55, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Strikethrough on FRI
    let strike_x = cx - 118.0;
    let strike_y = cy - 8.0;
    let strike_fri = comp
        .build_layer()
        .line_path(0.0, 0.0, 236.0, 0.0)
        .stroke(red, 6.0)
        .at(strike_x, strike_y)
        .depth(0.115)
        .add();
    comp.animate(strike_fri)
        .clip_start(14.55)
        .kf(14.55, AnimatedProperty::trim_path_end(0.0))
        .kf_eased(
            15.00,
            AnimatedProperty::trim_path_end(1.0),
            Easing::EASE_OUT,
        )
        .kf(15.10, AnimatedProperty::position(strike_x, strike_y))
        .kf_eased(
            15.55,
            AnimatedProperty::position(strike_x - 200.0, strike_y),
            Easing::EASE_IN,
        )
        .kf(15.10, AnimatedProperty::opacity(1.0))
        .kf_eased(15.55, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // MON slides in from right
    let mon = comp
        .build_layer()
        .text("MON", 144.0)
        .width(420.0)
        .height(180.0)
        .bold()
        .letter_spacing(4.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(red_ink)
        .at(cx - 210.0, cy - 90.0)
        .depth(0.115)
        .add();
    comp.animate(mon)
        .fade_in(15.20, 0.20)
        .ease_out()
        .slide_from(180.0, 0.0, 15.20, 0.55)
        .with_easing(Easing::Spring {
            stiffness: 300.0,
            damping: 22.0,
            mass: 1.0,
        })
        .kf(beat2e_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat2e_out + 0.20,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Subtitle — 20 px below the FRI/MON box bottom (cy + 90)
    let dead_sub = comp
        .build_layer()
        .text("ship date keeps moving", 22.0)
        .width(700.0)
        .height(30.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(muted)
        .at(cx - 350.0, cy + 110.0)
        .depth(0.11)
        .add();
    comp.animate(dead_sub)
        .fade_in(beat2e_in + 0.40, 0.30)
        .ease_out()
        .kf(beat2e_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat2e_out + 0.20,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    scene
        .assets
        .insert(comp_id.clone(), Asset::Composition(comp));
    comp_id
}

fn build_act3(scene: &mut Scene) -> Id {
    let comp_id = Id::new();
    let mut comp = Composition::new(W, H);
    comp.id = comp_id.clone();
    comp.duration = Duration::Seconds(SCENE_DUR);
    let cx = W * 0.5;
    let cy = H * 0.5;
    let (
        _,
        surface,
        border,
        ink,
        _,
        muted,
        soft,
        gray50,
        _,
        primary,
        violet,
        emerald,
        _,
        _,
        _,
        _,
        shadow,
    ) = make_comp_colors();

    // =========================================================================
    // ACT 3 — SOLUTION
    // =========================================================================

    // ── Beat 3A (16.18 – 17.85): Kario hero — letterform reveal ──────────────
    let beat3a_in = 16.18_f32;
    let beat3a_out = 17.70_f32;
    let logo_size = 96.0;
    let logo_x = cx - 196.0;
    let logo_y = cy - 48.0;

    // Logo gradient mark — iris in
    let logo = comp
        .build_layer()
        .rect(logo_size, logo_size)
        .corner_radius(22.0)
        .fill(Paint::linear(
            [0.0, 0.0],
            [1.0, 1.0],
            [(0.0, primary), (1.0, violet)],
        ))
        .drop_shadow([0.0, 12.0], shadow.with_alpha(0.20), 22.0, 0.0)
        .at(logo_x, logo_y)
        .depth(0.10)
        .add();
    comp.animate(logo)
        .fade_in(beat3a_in, 0.25)
        .ease_out()
        .scale_from(0.0, beat3a_in, 0.55)
        .with_easing(Easing::Spring {
            stiffness: 280.0,
            damping: 18.0,
            mass: 1.0,
        })
        .kf(beat3a_in, AnimatedProperty::rotation_z(-25.0))
        .kf_eased(
            beat3a_in + 0.55,
            AnimatedProperty::rotation_z(0.0),
            Easing::EASE_OUT,
        )
        .kf(beat3a_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat3a_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    let logo_letter = comp
        .build_layer()
        .text("K", 56.0)
        .width(logo_size)
        .height(logo_size)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(surface)
        .at(logo_x, logo_y)
        .depth(0.11)
        .add();
    comp.animate(logo_letter)
        .fade_in(beat3a_in + 0.20, 0.30)
        .ease_out()
        .kf(beat3a_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat3a_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Wordmark — letter-by-letter reveal (5 letters: K A R I O)
    let kario_letters = ["K", "A", "R", "I", "O"];
    let letter_x_base = logo_x + logo_size + 28.0;
    let letter_w = 50.0;
    let letter_gap = 8.0;
    for (i, ch) in kario_letters.iter().enumerate() {
        let lx = letter_x_base + i as f32 * (letter_w + letter_gap);
        let t = beat3a_in + 0.30 + i as f32 * 0.07;
        let letter = comp
            .build_layer()
            .text(*ch, 64.0)
            .width(letter_w)
            .height(logo_size)
            .bold()
            .letter_spacing(0.0)
            .text_align_center()
            .vertical_align_middle()
            .fill(ink)
            .at(lx, logo_y)
            .depth(0.10)
            .add();
        comp.animate(letter)
            .fade_in(t, 0.22)
            .ease_out()
            .slide_from(0.0, -22.0, t, 0.40)
            .with_easing(Easing::spring(320.0, 18.0, 1.0))
            .scale_from(0.5, t, 0.30)
            .spring(360.0, 16.0)
            .kf(beat3a_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat3a_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();
    }

    // Tagline below
    let tagline = comp
        .build_layer()
        .text("does it differently", 24.0)
        .width(500.0)
        .height(34.0)
        .italic()
        .text_align_center()
        .vertical_align_middle()
        .fill(muted)
        .at(cx - 250.0, cy + 80.0)
        .depth(0.10)
        .add();
    comp.animate(tagline)
        .fade_in(beat3a_in + 0.75, 0.35)
        .ease_out()
        .slide_from(0.0, 16.0, beat3a_in + 0.75, 0.45)
        .ease_out()
        .kf(beat3a_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat3a_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // ── Beat 3B+3C (17.85 – 21.30): Studio panel — prompt rises, brand kit drops ─
    let beat3bc_in = 17.85_f32;
    let beat3bc_out = 21.20_f32;

    // ─── Prompt input box (rises from below) ───
    let prompt_w = 760.0;
    let prompt_h = 92.0;
    let prompt_x = cx - prompt_w * 0.5;
    let prompt_y = cy - 130.0;

    // Eyebrow
    let prompt_spark = comp
        .build_layer()
        .svg(svg_sparkle(HEX_PRIMARY))
        .width(20.0)
        .height(20.0)
        .at(prompt_x + 6.0, prompt_y - 32.0)
        .depth(0.10)
        .add();
    comp.animate(prompt_spark)
        .fade_in(beat3bc_in, 0.30)
        .ease_out()
        .kf(beat3bc_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat3bc_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    let prompt_eyebrow = comp
        .build_layer()
        .text("PROMPT", 12.0)
        .width(120.0)
        .height(20.0)
        .bold()
        .letter_spacing(3.5)
        .vertical_align_middle()
        .fill(primary)
        .at(prompt_x + 32.0, prompt_y - 32.0)
        .depth(0.10)
        .add();
    comp.animate(prompt_eyebrow)
        .fade_in(beat3bc_in, 0.30)
        .ease_out()
        .kf(beat3bc_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat3bc_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Input box rises from below
    let prompt_box = comp
        .build_layer()
        .rect(prompt_w, prompt_h)
        .corner_radius(16.0)
        .fill(surface)
        .stroke(primary.with_alpha(0.4), 1.5)
        .drop_shadow([0.0, 10.0], primary.with_alpha(0.18), 24.0, 0.0)
        .at(prompt_x, prompt_y)
        .depth(0.10)
        .add();
    comp.animate(prompt_box)
        .fade_in(beat3bc_in + 0.05, 0.30)
        .ease_out()
        .slide_from(0.0, 50.0, beat3bc_in + 0.05, 0.55)
        .with_easing(Easing::Spring {
            stiffness: 290.0,
            damping: 22.0,
            mass: 1.0,
        })
        .kf(beat3bc_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat3bc_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // ── Mouse cursor: deliberate, professional click sequence ───────────────
    // 1. Cursor enters from upper-right with a slow ease-out (decelerating
    //    approach reads as intentional, not flashing).
    // 2. Brief hover pause (~120 ms) at the target before clicking — sells
    //    the "user is about to click" beat.
    // 3. Subtle press pulse (1.0 → 0.92 → 1.0) — small enough to register as
    //    a press, not a flash.
    // 4. Single soft ripple expands ~1.7× and fades — quiet, not theatrical.
    // 5. Cursor lingers on target while text starts typing, then fades out.
    let click_target_x = prompt_x + 220.0;
    let click_target_y = prompt_y + prompt_h * 0.5;
    let cursor_start_x = cx + 320.0;
    let cursor_start_y = prompt_y - 110.0;
    let t_enter = 17.85_f32;
    let t_arrive = 18.18_f32;
    let t_click = 18.30_f32;

    let cursor_arrow = comp
        .build_layer()
        .svg(svg_cursor(HEX_INK))
        .width(28.0)
        .height(28.0)
        .at(cursor_start_x, cursor_start_y)
        .depth(0.25)
        .add();
    comp.animate(cursor_arrow)
        .fade_in(t_enter, 0.22)
        .ease_out()
        // Decelerating travel from upper-right to the click target
        .kf(
            t_enter,
            AnimatedProperty::position(cursor_start_x, cursor_start_y),
        )
        .kf_eased(
            t_arrive,
            AnimatedProperty::position(click_target_x, click_target_y),
            Easing::EASE_OUT,
        )
        // Subtle press pulse — 8% scale dip, then snap back
        .kf(t_click, AnimatedProperty::scale(1.0, 1.0))
        .kf_eased(
            t_click + 0.07,
            AnimatedProperty::scale(0.92, 0.92),
            Easing::EASE_OUT,
        )
        .kf_eased(
            t_click + 0.20,
            AnimatedProperty::scale(1.0, 1.0),
            Easing::EASE_OUT,
        )
        // Linger briefly after click, then fade out so the typewriter takes focus
        .kf(t_click + 0.50, AnimatedProperty::opacity(1.0))
        .kf_eased(
            t_click + 0.95,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Single soft click ripple — quiet, low-contrast, expands ~1.7× and fades
    let ripple = comp
        .build_layer()
        .circle(36.0)
        .no_fill()
        .stroke(primary.with_alpha(0.45), 1.5)
        .at(click_target_x - 18.0, click_target_y - 18.0)
        .depth(0.24)
        .add();
    comp.animate(ripple)
        .fade_in(t_click, 0.04)
        .ease_out()
        .kf(t_click, AnimatedProperty::scale(0.6, 0.6))
        .kf_eased(
            t_click + 0.50,
            AnimatedProperty::scale(1.7, 1.7),
            Easing::EASE_OUT,
        )
        .kf(t_click, AnimatedProperty::opacity(0.55))
        .kf_eased(
            t_click + 0.50,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_OUT,
        )
        .apply();

    // Typed prompt text — typewriter glyph-by-glyph with built-in blinking
    // cursor. Starts AFTER the mouse click so the order reads as "click → type".
    let prompt_text = comp
        .build_layer()
        .text("Launch video for our new pricing tier...", 26.0)
        .width(prompt_w - 80.0)
        .height(prompt_h)
        .vertical_align_middle()
        .fill(ink)
        .cursor_blink(1.0)
        .at(prompt_x + 32.0, prompt_y)
        .depth(0.11)
        .add();
    comp.animate(prompt_text)
        .typewriter(18.32, 0.95)
        .kf(beat3bc_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat3bc_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // ─── Brand kit (literal drag-drop + eyedropper extraction) ──────────────
    // Visual matches the VO "Drop in your logo and brand colors." 19.158-20.86.
    // Cursor drags a "logo.svg" file card into a dashed drop-zone; the zone
    // flashes green; the brand mark materialises inside; three brand colour
    // swatches eyedrop out to the right with their hex values.
    let kit_y = prompt_y + prompt_h + 56.0;

    // Drop zone (left)
    let zone_w = 240.0_f32;
    let zone_h = 130.0_f32;
    let zone_x = prompt_x;
    let zone_y = kit_y;

    let zone_outline = comp
        .build_layer()
        .rect(zone_w, zone_h)
        .corner_radius(16.0)
        .fill(gray50)
        .stroke(primary.with_alpha(0.45), 2.0)
        .at(zone_x, zone_y)
        .depth(0.10)
        .add();
    comp.animate(zone_outline)
        .fade_in(18.95, 0.30)
        .ease_out()
        .kf(beat3bc_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat3bc_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Inner dashed look — a second, smaller stroked rectangle gives a
    // double-border effect that reads as "drop target".
    let zone_inner = comp
        .build_layer()
        .rect(zone_w - 16.0, zone_h - 16.0)
        .corner_radius(12.0)
        .no_fill()
        .stroke(primary.with_alpha(0.28), 1.0)
        .at(zone_x + 8.0, zone_y + 8.0)
        .depth(0.105)
        .add();
    comp.animate(zone_inner)
        .fade_in(18.97, 0.28)
        .ease_out()
        .kf(beat3bc_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat3bc_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    let zone_palette = comp
        .build_layer()
        .svg(svg_palette(HEX_PRIMARY))
        .width(20.0)
        .height(20.0)
        .at(zone_x + 16.0, zone_y - 30.0)
        .depth(0.11)
        .add();
    comp.animate(zone_palette)
        .fade_in(18.98, 0.24)
        .ease_out()
        .kf(beat3bc_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat3bc_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    let zone_label = comp
        .build_layer()
        .text("DROP YOUR LOGO", 12.0)
        .width(zone_w)
        .height(20.0)
        .bold()
        .letter_spacing(3.0)
        .vertical_align_middle()
        .fill(primary)
        .at(zone_x + 42.0, zone_y - 30.0)
        .depth(0.11)
        .add();
    comp.animate(zone_label)
        .fade_in(18.98, 0.26)
        .ease_out()
        .kf(beat3bc_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat3bc_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // File card "logo.svg" — flies in from upper-right with the cursor and
    // releases inside the drop zone at t_drop.
    let file_w = 130.0_f32;
    let file_h = 86.0_f32;
    let file_start_x = cx + 220.0;
    let file_start_y = zone_y - 90.0;
    let file_target_x = zone_x + (zone_w - file_w) * 0.5;
    let file_target_y = zone_y + (zone_h - file_h) * 0.5;
    let t_pickup = 19.05_f32;
    let t_drop = 19.45_f32;
    let t_settle = 19.65_f32;

    let file_card = comp
        .build_layer()
        .rect(file_w, file_h)
        .corner_radius(12.0)
        .fill(surface)
        .stroke(border, 1.0)
        .drop_shadow([0.0, 12.0], shadow.with_alpha(0.18), 22.0, 0.0)
        .at(file_start_x, file_start_y)
        .depth(0.13)
        .add();
    comp.animate(file_card)
        .fade_in(t_pickup, 0.16)
        .ease_out()
        .kf(
            t_pickup,
            AnimatedProperty::position(file_start_x, file_start_y),
        )
        .kf_eased(
            t_drop,
            AnimatedProperty::position(file_target_x, file_target_y),
            Easing::EASE_IN_OUT,
        )
        // Drop "thunk" — brief bounce on landing
        .kf(t_drop, AnimatedProperty::scale(1.0, 1.0))
        .kf_eased(
            t_drop + 0.10,
            AnimatedProperty::scale(1.06, 0.94),
            Easing::EASE_OUT,
        )
        .kf_eased(
            t_drop + 0.22,
            AnimatedProperty::scale(1.0, 1.0),
            Easing::EASE_OUT,
        )
        // After the brand mark settles, fade the card so the mark stands alone
        .kf(t_settle + 0.10, AnimatedProperty::opacity(1.0))
        .kf_eased(
            t_settle + 0.40,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // File card label "logo.svg"
    let file_label = comp
        .build_layer()
        .text("logo.svg", 13.0)
        .width(file_w)
        .height(20.0)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(ink)
        .at(file_start_x, file_start_y + file_h - 26.0)
        .depth(0.135)
        .add();
    comp.animate(file_label)
        .fade_in(t_pickup + 0.03, 0.16)
        .ease_out()
        .kf(
            t_pickup,
            AnimatedProperty::position(file_start_x, file_start_y + file_h - 26.0),
        )
        .kf_eased(
            t_drop,
            AnimatedProperty::position(file_target_x, file_target_y + file_h - 26.0),
            Easing::EASE_IN_OUT,
        )
        .kf(t_settle + 0.10, AnimatedProperty::opacity(1.0))
        .kf_eased(
            t_settle + 0.40,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Mini logo preview inside the file card (small gradient circle)
    let file_mark = comp
        .build_layer()
        .circle(28.0)
        .fill(Paint::linear(
            [0.0, 0.0],
            [1.0, 1.0],
            [(0.0, primary), (1.0, violet)],
        ))
        .at(file_start_x + (file_w - 28.0) * 0.5, file_start_y + 18.0)
        .depth(0.135)
        .add();
    comp.animate(file_mark)
        .fade_in(t_pickup + 0.04, 0.16)
        .ease_out()
        .kf(
            t_pickup,
            AnimatedProperty::position(file_start_x + (file_w - 28.0) * 0.5, file_start_y + 18.0),
        )
        .kf_eased(
            t_drop,
            AnimatedProperty::position(file_target_x + (file_w - 28.0) * 0.5, file_target_y + 18.0),
            Easing::EASE_IN_OUT,
        )
        .kf(t_settle + 0.10, AnimatedProperty::opacity(1.0))
        .kf_eased(
            t_settle + 0.40,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Cursor drags the file card in, releases at t_drop.
    let drag_cursor = comp
        .build_layer()
        .svg(svg_cursor(HEX_INK))
        .width(28.0)
        .height(28.0)
        .at(file_start_x + 64.0, file_start_y + 22.0)
        .depth(0.30)
        .add();
    comp.animate(drag_cursor)
        .fade_in(t_pickup, 0.16)
        .ease_out()
        .kf(
            t_pickup,
            AnimatedProperty::position(file_start_x + 64.0, file_start_y + 22.0),
        )
        .kf_eased(
            t_drop,
            AnimatedProperty::position(file_target_x + 64.0, file_target_y + 22.0),
            Easing::EASE_IN_OUT,
        )
        // Lift off after release
        .kf_eased(
            t_drop + 0.30,
            AnimatedProperty::position(file_target_x + 110.0, file_target_y - 30.0),
            Easing::EASE_OUT,
        )
        .kf(t_drop + 0.20, AnimatedProperty::opacity(1.0))
        .kf_eased(
            t_drop + 0.50,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Drop-zone success flash — green ring pulse on impact.
    let drop_flash = comp
        .build_layer()
        .rect(zone_w + 16.0, zone_h + 16.0)
        .corner_radius(20.0)
        .no_fill()
        .stroke(emerald, 3.0)
        .at(zone_x - 8.0, zone_y - 8.0)
        .depth(0.115)
        .add();
    comp.animate(drop_flash)
        .fade_in(t_drop, 0.06)
        .ease_out()
        .kf(t_drop, AnimatedProperty::scale(0.94, 0.94))
        .kf_eased(
            t_drop + 0.40,
            AnimatedProperty::scale(1.05, 1.05),
            Easing::EASE_OUT,
        )
        .kf(t_drop, AnimatedProperty::opacity(0.85))
        .kf_eased(
            t_drop + 0.50,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_OUT,
        )
        .apply();

    // Settled brand mark inside the zone — gradient circle, springs in once
    // the file card fades.
    let mark_size = 64.0_f32;
    let mark_x = zone_x + (zone_w - mark_size) * 0.5;
    let mark_y = zone_y + (zone_h - mark_size) * 0.5;
    let brand_mark = comp
        .build_layer()
        .circle(mark_size)
        .fill(Paint::linear(
            [0.0, 0.0],
            [1.0, 1.0],
            [(0.0, primary), (1.0, violet)],
        ))
        .at(mark_x, mark_y)
        .depth(0.12)
        .add();
    comp.animate(brand_mark)
        .fade_in(t_settle, 0.18)
        .ease_out()
        .scale_from(0.0, t_settle, 0.40)
        .spring(380.0, 16.0)
        .kf(beat3bc_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            beat3bc_out + 0.30,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Eyedropper extraction — three colour swatches fan out to the right of
    // the brand mark with their hex labels, each "extracted" at a staggered
    // beat across the VO line.
    let chip_w = 130.0_f32;
    let chip_h = 90.0_f32;
    let chip_gap = 18.0_f32;
    let chip_x0 = zone_x + zone_w + 60.0;
    let chip_y = zone_y;
    let extract_colors = [
        (primary, "#4F46E5"),
        (emerald, "#10B981"),
        (Color::hex(HEX_AMBER), "#F59E0B"),
    ];
    let extract_times = [19.85_f32, 20.18, 20.50];
    let mark_cx = mark_x + mark_size * 0.5;
    let mark_cy = mark_y + mark_size * 0.5;

    for i in 0..3 {
        let chip_xi = chip_x0 + i as f32 * (chip_w + chip_gap);
        let (chip_color, chip_hex) = extract_colors[i];
        let t_in = extract_times[i];

        // Beam from logo centre to swatch — short ray that draws on, fades
        let beam = comp
            .build_layer()
            .line_path(0.0, 0.0, chip_xi - mark_cx - 10.0, 0.0)
            .stroke(chip_color, 2.0)
            .at(mark_cx + 5.0, mark_cy + (i as f32 - 1.0) * 18.0)
            .depth(0.13)
            .add();
        comp.animate(beam)
            .clip_start(t_in - 0.08)
            .kf(t_in - 0.08, AnimatedProperty::trim_path_end(0.0))
            .kf_eased(
                t_in + 0.18,
                AnimatedProperty::trim_path_end(1.0),
                Easing::EASE_OUT,
            )
            .kf(t_in + 0.20, AnimatedProperty::opacity(0.85))
            .kf_eased(t_in + 0.40, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();

        // Swatch
        let chip = comp
            .build_layer()
            .rect(chip_w, chip_h)
            .corner_radius(14.0)
            .fill(chip_color)
            .drop_shadow([0.0, 8.0], shadow.with_alpha(0.16), 16.0, 0.0)
            .at(chip_xi, chip_y)
            .depth(0.12)
            .add();
        comp.animate(chip)
            .fade_in(t_in, 0.16)
            .ease_out()
            .scale_from(0.0, t_in, 0.40)
            .spring(360.0, 16.0)
            .kf(beat3bc_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat3bc_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        // Hex label below
        let hex_lbl = comp
            .build_layer()
            .text(chip_hex, 12.0)
            .width(chip_w)
            .height(18.0)
            .bold()
            .letter_spacing(0.5)
            .text_align_center()
            .vertical_align_middle()
            .fill(muted)
            .at(chip_xi, chip_y + chip_h + 8.0)
            .depth(0.12)
            .add();
        comp.animate(hex_lbl)
            .fade_in(t_in + 0.12, 0.20)
            .ease_out()
            .kf(beat3bc_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat3bc_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();
    }

    // ── Beat 3D (21.40 – 26.95): three-stage pipeline, drawn left-to-right ──
    let _beat3d_in = 21.40_f32;
    let beat3d_out = 26.85_f32;
    let pipe_y = cy - 90.0;
    let stage3_w = 280.0;
    let stage3_h = 180.0;
    let stage3_gap = 76.0;
    let stages3_total_w = 3.0 * stage3_w + 2.0 * stage3_gap;
    let stage3_x0 = cx - stages3_total_w * 0.5;

    let stage3_labels = ["SCRIPT", "VOICE-OVER", "MOTION"];
    let stage3_subs = [
        "AI-written copy",
        "Studio-quality TTS",
        "Brand-tinted scenes",
    ];
    let stage3_icons = [
        svg_doc(HEX_PRIMARY),
        svg_mic(HEX_VIOLET),
        svg_play(HEX_EMERALD),
    ];
    let stage3_accents = [primary, violet, emerald];
    let stage3_times = [21.40_f32, 22.92, 24.55];
    let stage3_slide_dx = [-80.0_f32, 0.0, 80.0]; // each stage slides in from a different direction
                                                  // 2.5D perspective: parent groups swing in on the Y axis. Children inherit
                                                  // the rotation since they're parented to the group's null layer.
    let stage3_rot_y = [-32.0_f32, 0.0, 32.0];

    for i in 0..3 {
        let sx = stage3_x0 + i as f32 * (stage3_w + stage3_gap);
        let acc = stage3_accents[i];
        let t_in = stage3_times[i];
        let dx = stage3_slide_dx[i];
        let ry = stage3_rot_y[i];

        // ── Group parent (null) at the card centre — provides the rotation
        // pivot. All card chrome (frame, accent bar, icon, label, sub) is
        // parented here so a single rotation_y applies to the whole stage.
        let group_cx = sx + stage3_w * 0.5;
        let group_cy = pipe_y + stage3_h * 0.5;
        let group_id = comp
            .build_layer()
            .null()
            .at(group_cx, group_cy)
            .depth(0.10)
            .add();
        comp.animate(group_id.clone())
            .fade_in(t_in, 0.30)
            .ease_out()
            .slide_from(dx, if i == 1 { 50.0 } else { 0.0 }, t_in, 0.50)
            .with_easing(Easing::Spring {
                stiffness: 290.0,
                damping: 22.0,
                mass: 1.0,
            })
            .scale_from(0.88, t_in, 0.45)
            .spring(280.0, 22.0)
            // 2.5D Y-rotation on the GROUP — every child inherits this tilt
            .kf(t_in, AnimatedProperty::rotation_y(ry))
            .kf_eased(
                t_in + 0.55,
                AnimatedProperty::rotation_y(0.0),
                Easing::EASE_OUT,
            )
            .kf(beat3d_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat3d_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        // Card frame — parented to group, position is local (relative to group centre)
        let stage_frame = comp
            .build_layer()
            .rect(stage3_w, stage3_h)
            .corner_radius(18.0)
            .fill(surface)
            .stroke(border, 1.0)
            .drop_shadow([0.0, 12.0], shadow.with_alpha(0.10), 28.0, 0.0)
            .parent(&group_id)
            .at(-stage3_w * 0.5, -stage3_h * 0.5)
            .depth(0.10)
            .add();
        comp.animate(stage_frame)
            .fade_in(t_in, 0.30)
            .ease_out()
            .kf(beat3d_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat3d_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        // Top accent bar
        let top_bar = comp
            .build_layer()
            .rect(stage3_w - 36.0, 4.0)
            .corner_radius(2.0)
            .fill(acc)
            .anchor_left()
            .parent(&group_id)
            .at(-stage3_w * 0.5 + 18.0, -stage3_h * 0.5 + 14.0)
            .depth(0.105)
            .add();
        comp.animate(top_bar)
            .fade_in(t_in + 0.15, 0.25)
            .ease_out()
            .kf(t_in + 0.15, AnimatedProperty::scale(0.0, 1.0))
            .kf_eased(
                t_in + 0.50,
                AnimatedProperty::scale(1.0, 1.0),
                Easing::EASE_OUT,
            )
            .kf(beat3d_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat3d_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        // Big icon
        let icon_size = 52.0;
        let icon = comp
            .build_layer()
            .svg(stage3_icons[i].clone())
            .width(icon_size)
            .height(icon_size)
            .parent(&group_id)
            .at(-icon_size * 0.5, -stage3_h * 0.5 + 38.0)
            .depth(0.11)
            .add();
        comp.animate(icon)
            .fade_in(t_in + 0.25, 0.30)
            .ease_out()
            .scale_from(0.3, t_in + 0.25, 0.45)
            .spring(360.0, 18.0)
            .kf(beat3d_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat3d_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        // Stage label
        let lbl = comp
            .build_layer()
            .text(stage3_labels[i], 18.0)
            .width(stage3_w - 30.0)
            .height(28.0)
            .bold()
            .letter_spacing(3.0)
            .text_align_center()
            .vertical_align_middle()
            .fill(ink)
            .parent(&group_id)
            .at(-stage3_w * 0.5 + 15.0, -stage3_h * 0.5 + 102.0)
            .depth(0.11)
            .add();
        comp.animate(lbl)
            .fade_in(t_in + 0.40, 0.30)
            .ease_out()
            .kf(beat3d_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat3d_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        // Sub-label
        let sub = comp
            .build_layer()
            .text(stage3_subs[i], 14.0)
            .width(stage3_w - 30.0)
            .height(22.0)
            .text_align_center()
            .vertical_align_middle()
            .fill(muted)
            .parent(&group_id)
            .at(-stage3_w * 0.5 + 15.0, -stage3_h * 0.5 + 138.0)
            .depth(0.11)
            .add();
        comp.animate(sub)
            .fade_in(t_in + 0.50, 0.30)
            .ease_out()
            .kf(beat3d_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                beat3d_out + 0.30,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        // Connector that draws between this stage and the next
        if i < 2 {
            let conn_x = sx + stage3_w + 8.0;
            let conn_y = pipe_y + stage3_h * 0.5 - 1.0;
            let conn_w = stage3_gap - 16.0;
            let conn_t = stage3_times[i + 1] - 0.20;
            let connector = comp
                .build_layer()
                .line_path(0.0, 0.0, conn_w, 0.0)
                .stroke(soft, 2.0)
                .at(conn_x, conn_y)
                .depth(0.105)
                .add();
            comp.animate(connector)
                .clip_start(conn_t)
                .kf(conn_t, AnimatedProperty::trim_path_end(0.0))
                .kf_eased(
                    conn_t + 0.45,
                    AnimatedProperty::trim_path_end(1.0),
                    Easing::EASE_OUT,
                )
                .kf(beat3d_out, AnimatedProperty::opacity(1.0))
                .kf_eased(
                    beat3d_out + 0.30,
                    AnimatedProperty::opacity(0.0),
                    Easing::EASE_IN,
                )
                .apply();

            // Arrow head at end of connector
            let arrow_head = comp
                .build_layer()
                .svg(svg_arrow_right(HEX_SOFT))
                .width(20.0)
                .height(20.0)
                .at(conn_x + conn_w - 14.0, conn_y - 10.0)
                .depth(0.11)
                .add();
            comp.animate(arrow_head)
                .fade_in(conn_t + 0.30, 0.20)
                .ease_out()
                .scale_from(0.0, conn_t + 0.30, 0.30)
                .spring(420.0, 16.0)
                .kf(beat3d_out, AnimatedProperty::opacity(1.0))
                .kf_eased(
                    beat3d_out + 0.30,
                    AnimatedProperty::opacity(0.0),
                    Easing::EASE_IN,
                )
                .apply();
        }
    }

    scene
        .assets
        .insert(comp_id.clone(), Asset::Composition(comp));
    comp_id
}

fn build_act4(scene: &mut Scene) -> Id {
    let comp_id = Id::new();
    let mut comp = Composition::new(W, H);
    comp.id = comp_id.clone();
    comp.duration = Duration::Seconds(SCENE_DUR);
    let (
        _,
        surface,
        border,
        ink,
        _,
        muted,
        _soft,
        gray100,
        gray50,
        primary,
        violet,
        emerald,
        _,
        _,
        amber,
        _,
        shadow,
    ) = make_comp_colors();

    // =========================================================================
    // ACT 4 - AUDIO SYNC CONSOLE
    // =========================================================================

    let act4_out = 38.05_f32;
    let act4_out_dur = 0.30_f32;

    let board_x = 90.0;
    let board_y = 74.0;
    let board_w = W - 180.0;
    let board_h = 528.0;
    // Locked to VO words: Captions 31.888, Charts 32.830, Transitions 33.671,
    // Music 34.854, Rendered 37.138.
    let sync_times = [31.888_f32, 32.830, 33.671, 34.854, 37.138];
    let sync_fracs = [0.24_f32, 0.36, 0.47, 0.62, 0.92];
    let wave_x = board_x + 292.0;
    let wave_y = board_y + 254.0;
    let wave_w = 650.0;
    let wave_h = 116.0;

    let board = comp
        .build_layer()
        .rect(board_w, board_h)
        .corner_radius(20.0)
        .fill(surface)
        .stroke(border, 1.0)
        .drop_shadow([0.0, 18.0], shadow.with_alpha(0.12), 34.0, 0.0)
        .at(board_x, board_y)
        .depth(0.09)
        .add();
    comp.animate(board)
        .fade_in(27.05, 0.28)
        .ease_out()
        .slide_from(0.0, 34.0, 27.05, 0.50)
        .with_easing(Easing::Spring {
            stiffness: 280.0,
            damping: 22.0,
            mass: 1.0,
        })
        .kf(act4_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            act4_out + act4_out_dur,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    let kicker = comp
        .build_layer()
        .text("AUDIO-LOCKED COMPOSITION", 12.0)
        .width(330.0)
        .height(22.0)
        .bold()
        .letter_spacing(2.8)
        .vertical_align_middle()
        .fill(primary)
        .at(board_x + 42.0, board_y + 34.0)
        .depth(0.11)
        .add();
    comp.animate(kicker)
        .fade_in(27.20, 0.25)
        .ease_out()
        .kf(act4_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            act4_out + act4_out_dur,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    let title = comp
        .build_layer()
        .text("Every visual beat follows the voice track", 32.0)
        .width(710.0)
        .height(44.0)
        .bold()
        .vertical_align_middle()
        .fill(ink)
        .at(board_x + 42.0, board_y + 60.0)
        .depth(0.11)
        .add();
    comp.animate(title)
        .fade_in(27.35, 0.30)
        .ease_out()
        .slide_from(0.0, 12.0, 27.35, 0.42)
        .ease_out()
        .kf(31.10, AnimatedProperty::opacity(1.0))
        .kf_eased(31.55, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    let timecode = comp
        .build_layer()
        .text("29.4s  -  37.9s", 14.0)
        .width(170.0)
        .height(28.0)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(muted)
        .at(board_x + board_w - 220.0, board_y + 45.0)
        .depth(0.11)
        .add();
    comp.animate(timecode)
        .fade_in(27.50, 0.25)
        .ease_out()
        .kf(act4_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            act4_out + act4_out_dur,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    let script_panel = comp
        .build_layer()
        .rect(216.0, 286.0)
        .corner_radius(14.0)
        .fill(gray50)
        .stroke(border, 1.0)
        .at(board_x + 42.0, board_y + 132.0)
        .depth(0.105)
        .add();
    comp.animate(script_panel)
        .fade_in(27.45, 0.28)
        .ease_out()
        .scale_from(0.96, 27.45, 0.42)
        .spring(280.0, 20.0)
        .kf(act4_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            act4_out + act4_out_dur,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    let script_label = comp
        .build_layer()
        .text("VOICE SCRIPT", 11.0)
        .width(150.0)
        .height(22.0)
        .bold()
        .letter_spacing(2.0)
        .vertical_align_middle()
        .fill(muted)
        .at(board_x + 66.0, board_y + 156.0)
        .depth(0.11)
        .add();
    comp.animate(script_label)
        .fade_in(27.70, 0.20)
        .ease_out()
        .kf(act4_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            act4_out + act4_out_dur,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    let script_text = comp
        .build_layer()
        .text("Audio perfectly\nsynced with\nvisuals.", 25.0)
        .width(166.0)
        .height(150.0)
        .bold()
        .fill(ink)
        .at(board_x + 66.0, board_y + 205.0)
        .depth(0.12)
        .add();
    comp.animate(script_text)
        .fade_in(28.05, 0.26)
        .ease_out()
        .slide_from(0.0, 14.0, 28.05, 0.40)
        .ease_out()
        .kf(act4_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            act4_out + act4_out_dur,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Locked to VO line "Audio perfectly synced with visuals." 29.403 + 1.783.
    // Highlights sit *behind* the script text like a marker stroke — no
    // re-drawn word on top (which previously read as a strikethrough).
    // Line layout: 25pt bold, starting y = board_y + 205, ~32pt line height.
    //   line 1 ("Audio perfectly")  top ≈ board_y + 207
    //   line 2 ("synced with")      top ≈ board_y + 239
    //   line 3 ("visuals.")         top ≈ board_y + 271
    let word_highlights = [
        (
            "Audio",
            primary,
            29.403_f32,
            board_x + 62.0,
            board_y + 207.0,
            84.0,
        ),
        (
            "synced",
            emerald,
            30.30,
            board_x + 62.0,
            board_y + 239.0,
            96.0,
        ),
        (
            "visuals",
            amber,
            30.95,
            board_x + 62.0,
            board_y + 271.0,
            96.0,
        ),
    ];
    for (_word, color, t_in, x, y, w) in word_highlights {
        let pill = comp
            .build_layer()
            .rect(w, 30.0)
            .corner_radius(8.0)
            .fill(color.with_alpha(0.22))
            .at(x, y)
            .depth(0.115)
            .add();
        comp.animate(pill)
            .clip_start(t_in)
            .kf(t_in, AnimatedProperty::scale(0.0, 1.0))
            .kf_eased(
                t_in + 0.22,
                AnimatedProperty::scale(1.0, 1.0),
                Easing::EASE_OUT,
            )
            .kf(t_in, AnimatedProperty::opacity(0.0))
            .kf_eased(
                t_in + 0.10,
                AnimatedProperty::opacity(1.0),
                Easing::EASE_OUT,
            )
            .kf(act4_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                act4_out + act4_out_dur,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();
    }

    let wave_panel = comp
        .build_layer()
        .rect(wave_w, wave_h)
        .corner_radius(18.0)
        .fill(Color::hex("#111827"))
        .stroke(Color::hex("#334155"), 1.0)
        .at(wave_x, wave_y)
        .depth(0.105)
        .add();
    comp.animate(wave_panel)
        .fade_in(27.70, 0.28)
        .ease_out()
        .scale_from(0.98, 27.70, 0.38)
        .spring(280.0, 20.0)
        .kf(act4_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            act4_out + act4_out_dur,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    let wave_label = comp
        .build_layer()
        .text("VOICE WAVEFORM", 11.0)
        .width(180.0)
        .height(22.0)
        .bold()
        .letter_spacing(2.0)
        .vertical_align_middle()
        .fill(Color::hex("#CBD5E1"))
        .at(wave_x + 24.0, wave_y + 18.0)
        .depth(0.12)
        .add();
    comp.animate(wave_label)
        .fade_in(27.95, 0.18)
        .ease_out()
        .kf(act4_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            act4_out + act4_out_dur,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    let waveform_heights = [
        18.0_f32, 42.0, 30.0, 58.0, 34.0, 72.0, 46.0, 26.0, 64.0, 38.0, 78.0, 52.0, 28.0, 62.0,
        44.0, 74.0, 34.0, 56.0, 24.0, 66.0, 48.0, 30.0, 70.0, 40.0,
    ];
    let wave_bar_w = 9.0;
    let wave_gap = 15.0;
    for i in 0..waveform_heights.len() {
        let h = waveform_heights[i];
        let x = wave_x + 36.0 + i as f32 * (wave_bar_w + wave_gap);
        let y = wave_y + wave_h * 0.5 - h * 0.5 + 13.0;
        let color = if i % 5 == 0 {
            amber
        } else if i % 3 == 0 {
            emerald
        } else {
            primary
        };
        let bar = comp
            .build_layer()
            .rect(wave_bar_w, h)
            .corner_radius(4.5)
            .fill(color.with_alpha(0.92))
            .at(x, y)
            .depth(0.12)
            .add();
        let t_in = 28.10 + i as f32 * 0.035;
        comp.animate(bar)
            .fade_in(t_in, 0.10)
            .ease_out()
            .kf(t_in, AnimatedProperty::scale(1.0, 0.0))
            .kf_eased(
                t_in + 0.22,
                AnimatedProperty::scale(1.0, 1.0),
                Easing::EASE_OUT,
            )
            .kf(act4_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                act4_out + act4_out_dur,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();
    }

    let playhead_x0 = wave_x + 28.0;
    let playhead_x1 = wave_x + wave_w - 28.0;
    let playhead = comp
        .build_layer()
        .rect(3.0, wave_h + 36.0)
        .corner_radius(1.5)
        .fill(surface)
        .at(playhead_x0, wave_y - 18.0)
        .depth(0.16)
        .add();
    comp.animate(playhead)
        .fade_in(30.00, 0.16)
        .ease_out()
        .kf(30.00, AnimatedProperty::position_x(playhead_x0))
        .kf_eased(
            37.80,
            AnimatedProperty::position_x(playhead_x1),
            Easing::EASE_IN_OUT,
        )
        .kf(37.80, AnimatedProperty::opacity(1.0))
        .kf_eased(38.05, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    let event_labels = [
        ("CAPTIONS", svg_captions(HEX_EMERALD), emerald),
        ("CHARTS", svg_chart(HEX_PRIMARY), primary),
        ("TRANSITIONS", svg_transitions(HEX_VIOLET), violet),
        ("MUSIC", svg_music(HEX_AMBER), amber),
        ("RENDERED", svg_check_circle(HEX_EMERALD), emerald),
    ];
    for i in 0..5 {
        let event_x = playhead_x0 + sync_fracs[i] * (playhead_x1 - playhead_x0);
        let t_in = sync_times[i];
        let connector = comp
            .build_layer()
            .line_path(0.0, 0.0, 0.0, -78.0)
            .stroke(event_labels[i].2.with_alpha(0.56), 2.0)
            .at(event_x, wave_y + 4.0)
            .depth(0.13)
            .add();
        comp.animate(connector)
            .fade_in(t_in, 0.08)
            .ease_out()
            .kf(t_in + 0.42, AnimatedProperty::opacity(1.0))
            .kf_eased(
                t_in + 0.76,
                AnimatedProperty::opacity(0.20),
                Easing::EASE_IN,
            )
            .apply();

        let tag_y = wave_y - 110.0 + if i % 2 == 0 { -6.0 } else { 16.0 };
        let tag_w = if i == 2 { 134.0 } else { 126.0 };
        let tag = comp
            .build_layer()
            .rect(tag_w, 44.0)
            .corner_radius(12.0)
            .fill(surface)
            .stroke(event_labels[i].2.with_alpha(0.55), 1.0)
            .drop_shadow([0.0, 8.0], shadow.with_alpha(0.10), 16.0, 0.0)
            .at(event_x - tag_w * 0.5, tag_y)
            .depth(0.14)
            .add();
        comp.animate(tag)
            .fade_in(t_in, 0.12)
            .ease_out()
            .scale_from(0.76, t_in, 0.26)
            .spring(420.0, 16.0)
            .kf(act4_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                act4_out + act4_out_dur,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        let icon = comp
            .build_layer()
            .svg(event_labels[i].1.clone())
            .width(22.0)
            .height(22.0)
            .at(event_x - tag_w * 0.5 + 12.0, tag_y + 11.0)
            .depth(0.15)
            .add();
        comp.animate(icon)
            .fade_in(t_in + 0.06, 0.10)
            .ease_out()
            .scale_from(0.3, t_in + 0.06, 0.20)
            .spring(400.0, 16.0)
            .kf(act4_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                act4_out + act4_out_dur,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        let label = comp
            .build_layer()
            .text(event_labels[i].0, 11.0)
            .width(tag_w - 44.0)
            .height(22.0)
            .bold()
            .letter_spacing(1.0)
            .vertical_align_middle()
            .fill(event_labels[i].2)
            .at(event_x - tag_w * 0.5 + 42.0, tag_y + 11.0)
            .depth(0.15)
            .add();
        comp.animate(label)
            .fade_in(t_in + 0.10, 0.12)
            .ease_out()
            .kf(act4_out, AnimatedProperty::opacity(1.0))
            .kf_eased(
                act4_out + act4_out_dur,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_IN,
            )
            .apply();

        let pulse = comp
            .build_layer()
            .circle(46.0)
            .no_fill()
            .stroke(event_labels[i].2.with_alpha(0.65), 2.0)
            .at(event_x - 23.0, wave_y + wave_h * 0.5 - 10.0)
            .depth(0.17)
            .add();
        comp.animate(pulse)
            .fade_in(t_in, 0.05)
            .ease_out()
            .kf(t_in, AnimatedProperty::scale(0.55, 0.55))
            .kf_eased(
                t_in + 0.42,
                AnimatedProperty::scale(1.45, 1.45),
                Easing::EASE_OUT,
            )
            .kf(t_in, AnimatedProperty::opacity(0.90))
            .kf_eased(
                t_in + 0.42,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_OUT,
            )
            .apply();
    }

    // ── Full-screen "Rendered in minutes" finale ────────────────────────────
    // Covers the entire console with a clean white sheet and lands the headline
    // edge-to-edge. Locked to the VO word "Rendered" at 37.138.
    let cy_full = H * 0.5;
    let sheet_in = 36.85_f32;
    let sheet_full = 37.05_f32;

    let sheet = comp
        .build_layer()
        .rect(W, H)
        .fill(surface)
        .at(0.0, 0.0)
        .depth(0.20)
        .add();
    comp.animate(sheet)
        .fade_in(sheet_in, sheet_full - sheet_in)
        .ease_out()
        .kf(act4_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            act4_out + act4_out_dur,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    let big_eyebrow = comp
        .build_layer()
        .text("RENDERING", 14.0)
        .width(W)
        .height(24.0)
        .bold()
        .letter_spacing(5.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(primary)
        .at(0.0, cy_full - 96.0)
        .depth(0.21)
        .add();
    comp.animate(big_eyebrow)
        .fade_in(sheet_full, 0.20)
        .ease_out()
        .kf(37.10, AnimatedProperty::opacity(1.0))
        .kf_eased(37.30, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Wide progress bar
    let bar_w = 760.0_f32;
    let bar_h = 18.0_f32;
    let bar_x = (W - bar_w) * 0.5;
    let bar_y = cy_full - 40.0;

    let big_track = comp
        .build_layer()
        .rect(bar_w, bar_h)
        .corner_radius(bar_h * 0.5)
        .fill(gray100)
        .at(bar_x, bar_y)
        .depth(0.21)
        .add();
    comp.animate(big_track)
        .fade_in(sheet_full, 0.18)
        .ease_out()
        .kf(37.10, AnimatedProperty::opacity(1.0))
        .kf_eased(37.40, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    let big_fill = comp
        .build_layer()
        .rect(bar_w, bar_h)
        .corner_radius(bar_h * 0.5)
        .fill(Paint::linear(
            [0.0, 0.5],
            [1.0, 0.5],
            [(0.0, primary), (1.0, emerald)],
        ))
        .anchor_left()
        .at(bar_x, bar_y)
        .depth(0.215)
        .add();
    comp.animate(big_fill)
        .fade_in(sheet_full, 0.10)
        .ease_out()
        .kf(sheet_full, AnimatedProperty::scale(0.0, 1.0))
        .kf_eased(37.10, AnimatedProperty::scale(1.0, 1.0), Easing::EASE_OUT)
        .kf(37.10, AnimatedProperty::opacity(1.0))
        .kf_eased(37.40, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Big check icon — appears as the bar finishes / VO says "Rendered"
    let big_check = comp
        .build_layer()
        .svg(svg_check_circle(HEX_EMERALD))
        .width(120.0)
        .height(120.0)
        .at(W * 0.5 - 60.0, cy_full - 200.0)
        .depth(0.22)
        .add();
    comp.animate(big_check)
        .fade_in(37.14, 0.16)
        .ease_out()
        .scale_from(0.0, 37.14, 0.42)
        .spring(380.0, 14.0)
        .kf(act4_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            act4_out + act4_out_dur,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Headline — edge-to-edge
    let big_rendered = comp
        .build_layer()
        .text("Rendered in minutes", 84.0)
        .width(W - 160.0)
        .height(110.0)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(ink)
        .at(80.0, cy_full + 0.0)
        .depth(0.22)
        .add();
    comp.animate(big_rendered)
        .fade_in(37.14, 0.22)
        .ease_out()
        .slide_from(0.0, 22.0, 37.14, 0.45)
        .ease_out()
        .kf(act4_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            act4_out + act4_out_dur,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Render details below
    let big_detail = comp
        .build_layer()
        .text("0:42  •  1280×720  •  mp4", 22.0)
        .width(W)
        .height(34.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(muted)
        .at(0.0, cy_full + 130.0)
        .depth(0.22)
        .add();
    comp.animate(big_detail)
        .fade_in(37.45, 0.22)
        .ease_out()
        .kf(act4_out, AnimatedProperty::opacity(1.0))
        .kf_eased(
            act4_out + act4_out_dur,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    scene
        .assets
        .insert(comp_id.clone(), Asset::Composition(comp));
    comp_id
}

fn build_act5(scene: &mut Scene) -> Id {
    let comp_id = Id::new();
    let mut comp = Composition::new(W, H);
    comp.id = comp_id.clone();
    comp.duration = Duration::Seconds(SCENE_DUR);
    let cx = W * 0.5;
    let cy = H * 0.5;
    let (
        _,
        surface,
        border,
        ink,
        ink2,
        muted,
        soft,
        _,
        _,
        primary,
        violet,
        _emerald,
        _,
        _,
        _,
        _,
        shadow,
    ) = make_comp_colors();

    // =========================================================================
    // ACT 5 — CTA
    // =========================================================================

    // Beat 5A — FOUNDERS / CREATORS personas (rise from below)
    let beat5a_in = 38.20_f32;
    let beat5a_out = 40.30_f32;

    let persona_w = 240.0;
    let persona_h = 150.0;
    let persona_y = cy - 100.0;
    let persona_gap = 60.0;
    let persona_xs = [cx - persona_gap * 0.5 - persona_w, cx + persona_gap * 0.5];
    let persona_labels = ["FOUNDERS", "CREATORS"];
    let persona_subs = ["ship product launches", "publish content drops"];
    let persona_colors = [primary, violet];
    let persona_times = [beat5a_in, beat5a_in + 0.30];

    for i in 0..2 {
        let px = persona_xs[i];
        let acc = persona_colors[i];
        let t_in = persona_times[i];

        // Card rises from below
        let card = comp
            .build_layer()
            .rect(persona_w, persona_h)
            .corner_radius(18.0)
            .fill(surface)
            .stroke(border, 1.0)
            .drop_shadow([0.0, 12.0], shadow.with_alpha(0.14), 28.0, 0.0)
            .at(px, persona_y)
            .depth(0.10)
            .add();
        comp.animate(card)
            .fade_in(t_in, 0.30)
            .ease_out()
            .slide_from(0.0, 80.0, t_in, 0.55)
            .with_easing(Easing::Spring {
                stiffness: 280.0,
                damping: 20.0,
                mass: 1.0,
            })
            .scale_from(0.85, t_in, 0.50)
            .spring(280.0, 20.0)
            .kf(beat5a_out, AnimatedProperty::scale(1.0, 1.0))
            .kf_eased(40.50, AnimatedProperty::scale(0.7, 0.7), Easing::EASE_IN)
            .kf(40.40, AnimatedProperty::opacity(1.0))
            .kf_eased(40.65, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();

        // Avatar
        let av_size = 64.0;
        let av_cx = px + persona_w * 0.5;
        let av_y = persona_y + 22.0;
        let av = comp
            .build_layer()
            .circle(av_size)
            .fill(Paint::linear(
                [0.0, 0.0],
                [1.0, 1.0],
                [(0.0, acc), (1.0, persona_colors[1 - i])],
            ))
            .at(av_cx - av_size * 0.5, av_y)
            .depth(0.11)
            .add();
        comp.animate(av)
            .fade_in(t_in + 0.10, 0.28)
            .ease_out()
            .scale_from(0.4, t_in + 0.10, 0.40)
            .spring(360.0, 18.0)
            .kf(40.40, AnimatedProperty::opacity(1.0))
            .kf_eased(40.65, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();

        let av_icon = comp
            .build_layer()
            .svg(svg_user("#FFFFFF"))
            .width(32.0)
            .height(32.0)
            .at(av_cx - 16.0, av_y + (av_size - 32.0) * 0.5)
            .depth(0.12)
            .add();
        comp.animate(av_icon)
            .fade_in(t_in + 0.20, 0.25)
            .ease_out()
            .kf(40.40, AnimatedProperty::opacity(1.0))
            .kf_eased(40.65, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();

        // Label
        let lbl = comp
            .build_layer()
            .text(persona_labels[i], 16.0)
            .width(persona_w)
            .height(24.0)
            .bold()
            .letter_spacing(3.0)
            .text_align_center()
            .vertical_align_middle()
            .fill(ink)
            .at(px, persona_y + av_size + 26.0)
            .depth(0.11)
            .add();
        comp.animate(lbl)
            .fade_in(t_in + 0.28, 0.25)
            .ease_out()
            .kf(40.40, AnimatedProperty::opacity(1.0))
            .kf_eased(40.65, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();

        let sub = comp
            .build_layer()
            .text(persona_subs[i], 13.0)
            .width(persona_w)
            .height(20.0)
            .text_align_center()
            .vertical_align_middle()
            .fill(muted)
            .at(px, persona_y + av_size + 52.0)
            .depth(0.11)
            .add();
        comp.animate(sub)
            .fade_in(t_in + 0.36, 0.25)
            .ease_out()
            .kf(40.40, AnimatedProperty::opacity(1.0))
            .kf_eased(40.65, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
            .apply();
    }

    // "&" connector
    let amp = comp
        .build_layer()
        .text("&", 40.0)
        .width(60.0)
        .height(persona_h)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(soft)
        .at(cx - 30.0, persona_y)
        .depth(0.10)
        .add();
    comp.animate(amp)
        .fade_in(beat5a_in + 0.40, 0.30)
        .ease_out()
        .kf(40.40, AnimatedProperty::opacity(1.0))
        .kf_eased(40.65, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Beat 5B — CTA pill SLAM with shockwaves
    let cta_t = 40.55_f32;

    // ── Cursor approach + deliberate click on the CTA target ───────────────
    // Decelerating travel arrives at the click point ~120ms before the pill
    // slams; the click pulse and the slam align in one impact frame, so the
    // pill feels caused by the click rather than just spawning.
    let cta_click_x = cx;
    let cta_click_y = cy + 10.0;
    let cta_t_enter = 40.05_f32;
    let cta_t_arrive = 40.45_f32;

    let cta_cursor = comp
        .build_layer()
        .svg(svg_cursor(HEX_INK))
        .width(32.0)
        .height(32.0)
        .at(cx + 380.0, cy - 220.0)
        .depth(0.30)
        .add();
    comp.animate(cta_cursor)
        .fade_in(cta_t_enter, 0.22)
        .ease_out()
        .kf(
            cta_t_enter,
            AnimatedProperty::position(cx + 380.0, cy - 220.0),
        )
        .kf_eased(
            cta_t_arrive,
            AnimatedProperty::position(cta_click_x, cta_click_y),
            Easing::EASE_OUT,
        )
        // Subtle press pulse — 8% scale dip aligned with the slam impact
        .kf(cta_t, AnimatedProperty::scale(1.0, 1.0))
        .kf_eased(
            cta_t + 0.07,
            AnimatedProperty::scale(0.92, 0.92),
            Easing::EASE_OUT,
        )
        .kf_eased(
            cta_t + 0.20,
            AnimatedProperty::scale(1.0, 1.0),
            Easing::EASE_OUT,
        )
        // Linger briefly post-click, then exit so the URL reads cleanly
        .kf(cta_t + 0.55, AnimatedProperty::opacity(1.0))
        .kf_eased(
            cta_t + 0.95,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Single soft click ripple — quiet, expands ~1.7× and fades
    let cta_ripple = comp
        .build_layer()
        .circle(40.0)
        .no_fill()
        .stroke(primary.with_alpha(0.45), 1.5)
        .at(cta_click_x - 20.0, cta_click_y - 20.0)
        .depth(0.295)
        .add();
    comp.animate(cta_ripple)
        .fade_in(cta_t, 0.04)
        .ease_out()
        .kf(cta_t, AnimatedProperty::scale(0.6, 0.6))
        .kf_eased(
            cta_t + 0.55,
            AnimatedProperty::scale(1.7, 1.7),
            Easing::EASE_OUT,
        )
        .kf(cta_t, AnimatedProperty::opacity(0.55))
        .kf_eased(
            cta_t + 0.55,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_OUT,
        )
        .apply();

    // Two concentric shockwave rings — expand and fade to give the slam impact
    for (i, &ring_t) in [cta_t, cta_t + 0.10].iter().enumerate() {
        let ring = comp
            .build_layer()
            .circle(40.0)
            .no_fill()
            .stroke(primary.with_alpha(0.55), 3.0)
            .at(cx - 20.0, cy - 20.0)
            .depth(0.105 + i as f32 * 0.001)
            .add();
        comp.animate(ring)
            .fade_in(ring_t, 0.10)
            .ease_out()
            .kf(ring_t, AnimatedProperty::scale(0.5, 0.5))
            .kf_eased(
                ring_t + 0.85,
                AnimatedProperty::scale(18.0, 18.0),
                Easing::EASE_OUT,
            )
            .kf(ring_t, AnimatedProperty::opacity(0.7))
            .kf_eased(
                ring_t + 0.85,
                AnimatedProperty::opacity(0.0),
                Easing::EASE_OUT,
            )
            .apply();
    }

    // CTA headline above pill
    let cta_headline = comp
        .build_layer()
        .text("ship launch videos at", 26.0)
        .width(800.0)
        .height(38.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(ink2)
        .at(cx - 400.0, cy - 110.0)
        .depth(0.20)
        .add();
    comp.animate(cta_headline)
        .fade_in(cta_t - 0.20, 0.30)
        .ease_out()
        .slide_from(0.0, 14.0, cta_t - 0.20, 0.45)
        .ease_out()
        .kf(FINAL_OUT_T, AnimatedProperty::opacity(1.0))
        .kf_eased(
            FINAL_OUT_T + FINAL_OUT_DUR,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // CTA pill — slam in
    let pill_w = 480.0;
    let pill_h = 100.0;
    let pill_x = cx - pill_w * 0.5;
    let pill_y = cy - 40.0;
    let pill = comp
        .build_layer()
        .rect(pill_w, pill_h)
        .corner_radius(50.0)
        .fill(Paint::linear(
            [0.0, 0.5],
            [1.0, 0.5],
            [(0.0, primary), (1.0, violet)],
        ))
        .drop_shadow([0.0, 22.0], primary.with_alpha(0.35), 38.0, 0.0)
        .at(pill_x, pill_y)
        .depth(0.20)
        .add();
    comp.animate(pill)
        .fade_in(cta_t, 0.20)
        .ease_out()
        .scale_from(1.4, cta_t, 0.50)
        .with_easing(Easing::Spring {
            stiffness: 320.0,
            damping: 18.0,
            mass: 1.0,
        })
        .kf(FINAL_OUT_T, AnimatedProperty::opacity(1.0))
        .kf_eased(
            FINAL_OUT_T + FINAL_OUT_DUR,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // The URL
    let url = comp
        .build_layer()
        .text("karioai.com", 44.0)
        .width(pill_w)
        .height(pill_h)
        .bold()
        .letter_spacing(2.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(surface)
        .at(pill_x, pill_y)
        .depth(0.21)
        .add();
    comp.animate(url)
        .fade_in(cta_t + 0.25, 0.30)
        .ease_out()
        .kf(FINAL_OUT_T, AnimatedProperty::opacity(1.0))
        .kf_eased(
            FINAL_OUT_T + FINAL_OUT_DUR,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Sub-line
    let sub_arrow = comp
        .build_layer()
        .svg(svg_arrow_right(HEX_MUTED))
        .width(20.0)
        .height(20.0)
        .at(cx - 178.0, pill_y + pill_h + 32.0)
        .depth(0.20)
        .add();
    comp.animate(sub_arrow)
        .fade_in(cta_t + 0.85, 0.30)
        .ease_out()
        .kf(FINAL_OUT_T, AnimatedProperty::opacity(1.0))
        .kf_eased(
            FINAL_OUT_T + FINAL_OUT_DUR,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    let sub = comp
        .build_layer()
        .text("Your next launch starts here", 18.0)
        .width(420.0)
        .height(24.0)
        .vertical_align_middle()
        .fill(muted)
        .at(cx - 152.0, pill_y + pill_h + 30.0)
        .depth(0.20)
        .add();
    comp.animate(sub)
        .fade_in(cta_t + 0.90, 0.40)
        .ease_out()
        .slide_from(0.0, 8.0, cta_t + 0.90, 0.50)
        .ease_out()
        .kf(FINAL_OUT_T, AnimatedProperty::opacity(1.0))
        .kf_eased(
            FINAL_OUT_T + FINAL_OUT_DUR,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    // Tiny check icon next to the sub — quiet "ready" cue
    let sub_check = comp
        .build_layer()
        .svg(svg_check_big(HEX_EMERALD))
        .width(18.0)
        .height(18.0)
        .at(cx + 138.0, pill_y + pill_h + 33.0)
        .depth(0.20)
        .add();
    comp.animate(sub_check)
        .fade_in(cta_t + 1.10, 0.25)
        .ease_out()
        .scale_from(0.0, cta_t + 1.10, 0.30)
        .spring(420.0, 16.0)
        .kf(FINAL_OUT_T, AnimatedProperty::opacity(1.0))
        .kf_eased(
            FINAL_OUT_T + FINAL_OUT_DUR,
            AnimatedProperty::opacity(0.0),
            Easing::EASE_IN,
        )
        .apply();

    scene
        .assets
        .insert(comp_id.clone(), Asset::Composition(comp));
    comp_id
}

fn build_scene() -> Scene {
    let mut scene = Scene::new(W, H);
    scene.main_composition.duration = Duration::Seconds(SCENE_DUR);

    // ── Audio: VO with whisperx-aligned phrase timings ───────────────────────
    let vo_asset = AudioAsset::new(VO_SRC)
        .with_script(SCRIPT)
        .with_word_timing(
            "The hardest part of shipping a feature isn't the feature.",
            0.111,
            2.841,
        )
        .with_word_timing("It's the launch video that goes with it.", 3.472, 2.021)
        .with_word_timing("A polished one eats your whole week.", 5.973, 1.461)
        .with_word_timing("A freelancer wants two thousand dollars.", 8.354, 1.281)
        .with_word_timing("And six rounds of notes.", 9.895, 1.960)
        .with_word_timing("Open After Effects yourself?", 12.456, 1.200)
        .with_word_timing("Your launch slips past Friday.", 14.376, 1.601)
        .with_word_timing("Kario does it differently.", 16.277, 1.121)
        .with_word_timing("Type a prompt.", 17.958, 0.680)
        .with_word_timing("Drop in your logo and brand colors.", 19.158, 1.701)
        .with_word_timing("Kario writes the script,", 21.479, 1.141)
        .with_word_timing("generates the voice-over,", 23.000, 1.120)
        .with_word_timing("and animates the whole thing, end to end.", 24.620, 2.321)
        .with_word_timing("Custom motion in your brand.", 27.181, 1.361)
        .with_word_timing("Audio perfectly synced with visuals.", 29.403, 1.783)
        .with_word_timing("Captions.", 31.888, 0.521)
        .with_word_timing("Charts.", 32.830, 0.360)
        .with_word_timing("Transitions.", 33.671, 0.642)
        .with_word_timing("Music.", 34.854, 0.381)
        .with_word_timing("Composed for you.", 35.655, 0.762)
        .with_word_timing("Rendered in minutes.", 37.138, 0.822)
        .with_word_timing("Founders and creators.", 38.301, 1.382)
        .with_word_timing("Ship launch videos at karioai.com.", 40.144, 2.265);
    let vo_id = scene.add_audio_asset(vo_asset);
    scene.main_composition.add_audio(
        Audio::new(vo_id)
            .with_start_time(0.0)
            .with_duration(VO_DUR)
            .with_volume(1.0),
    );

    // Build act precomps
    let act1_id = build_act1(&mut scene);
    let act2_id = build_act2(&mut scene);
    let act3_id = build_act3(&mut scene);
    let act4_id = build_act4(&mut scene);
    let act5_id = build_act5(&mut scene);

    // ── Main composition: background + act instances ────────────────────────
    let (inst1, inst2, inst3, inst4, inst5) = {
        let comp = &mut scene.main_composition;
        let (bg, surface, _, _, _, _, _, gray100, _, _, _, _, _, _, _, _, _) = make_comp_colors();

        // ── Background (full scene) ──────────────────────────────────────────────
        comp.build_layer()
            .rect(W, H)
            .label("bg")
            .fill(bg)
            .at(0.0, 0.0)
            .depth(0.0)
            .add();

        // Subtle gradient wash to keep canvas from feeling flat
        comp.build_layer()
            .rect(W, H)
            .fill(Paint::linear(
                [0.0, 0.0],
                [0.0, 1.0],
                [
                    (0.0, surface.with_alpha(0.0)),
                    (1.0, gray100.with_alpha(0.6)),
                ],
            ))
            .at(0.0, 0.0)
            .depth(0.005)
            .add();

        // ── Act instance layers with clip windows ──────────────────────────────
        // Each Instance has time_offset = -clip_start so that precomp local_time
        // equals global_time (precomp animations are authored with absolute timestamps).
        // Instance receives: local_time = (global - clip_start) - time_offset = global_time.
        // Act 1: visible 0.0 → 6.35, extended through the Act 1 → Act 2 transition.
        let mut inst1_inst = Instance::new(act1_id);
        inst1_inst.time_offset = 0.0;
        let mut inst1_layer = inst1_inst.as_layer();
        inst1_layer.depth = 0.010;
        let inst1 = comp.add_layer(inst1_layer);
        comp.add_clip(Clip::new(inst1.clone(), 0.0, Duration::Seconds(6.35)));

        // Act 2: visible 5.55 → 16.70, extended through the Act 2 → Act 3 transition.
        let mut inst2_inst = Instance::new(act2_id);
        inst2_inst.time_offset = -5.55;
        let mut inst2_layer = inst2_inst.as_layer();
        inst2_layer.depth = 0.011;
        let inst2 = comp.add_layer(inst2_layer);
        comp.add_clip(Clip::new(inst2.clone(), 5.55, Duration::Seconds(11.15)));

        // Act 3: visible 15.90 → 27.55, extended through the Act 3 → Act 4 transition.
        let mut inst3_inst = Instance::new(act3_id);
        inst3_inst.time_offset = -15.90;
        let mut inst3_layer = inst3_inst.as_layer();
        inst3_layer.depth = 0.012;
        let inst3 = comp.add_layer(inst3_layer);
        comp.add_clip(Clip::new(inst3.clone(), 15.90, Duration::Seconds(11.65)));

        // Act 4: visible 26.75 → 38.70, extended through the Act 4 → Act 5 transition.
        let mut inst4_inst = Instance::new(act4_id);
        inst4_inst.time_offset = -26.75;
        let mut inst4_layer = inst4_inst.as_layer();
        inst4_layer.depth = 0.013;
        let inst4 = comp.add_layer(inst4_layer);
        comp.add_clip(Clip::new(inst4.clone(), 26.75, Duration::Seconds(11.95)));

        // Act 5: visible 37.90 → SCENE_DUR
        let mut inst5_inst = Instance::new(act5_id);
        inst5_inst.time_offset = -37.90;
        let mut inst5_layer = inst5_inst.as_layer();
        inst5_layer.depth = 0.014;
        let inst5 = comp.add_layer(inst5_layer);
        comp.add_clip(Clip::new(
            inst5.clone(),
            37.90,
            Duration::Seconds(SCENE_DUR - 37.90),
        ));

        (inst1, inst2, inst3, inst4, inst5)
    };

    // ── Renderer-level act transitions from transition_demo.rs ──────────────
    scene.add_transition(Swipe::left(&inst1, &inst2).start(5.55).duration(0.80));
    scene.add_transition(
        ZoomThrough::new(&inst2, &inst3)
            .center(W / 2.0, H / 2.0)
            .portal_size(360.0, 202.5)
            .start(15.90)
            .duration(0.80),
    );
    scene.add_transition(
        Iris::new(&inst3, &inst4)
            .center(W / 2.0, H / 2.0)
            .color(Color::hex(HEX_PRIMARY))
            .start(26.75)
            .duration(0.80),
    );
    scene.add_transition(
        Ripple::new(&inst4, &inst5)
            .center(W / 2.0, H / 2.0)
            .fill_color(Color::hex(HEX_BG))
            .ring_colors([
                Color::hex(HEX_PRIMARY),
                Color::hex(HEX_VIOLET),
                Color::hex(HEX_EMERALD),
            ])
            .start(37.90)
            .duration(0.80),
    );

    scene
}

pub fn build() -> kario_base::Scene {
    build_scene()
}
