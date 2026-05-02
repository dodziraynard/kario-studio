// simple template — VO-driven motion graphics, dark palette.
//
// HOW CLAUDE SHOULD EDIT THIS FILE:
// 1. Update VO_DUR and SCENE_DUR to match the actual whisperx audio length.
// 2. Replace every build_beat_*() function body with real content derived from
//    the phrase timings and script. Add/remove build_beat_*() calls in
//    build_scene() to match the number of VO phrases. Keep each beat to a
//    single focused visual idea — no paragraph of text on screen.
// 3. Do NOT invent new API methods. Only use the builder methods shown below.
//
// ── Verified API reference (do not deviate) ──────────────────────────────────
// Composition builder (comp.build_layer()):
//   .rect(w, h)               – filled rectangle
//   .circle(diameter)         – filled/stroked circle
//   .text("str", font_size)   – text layer
//   .svg("…svg string…")      – inline SVG image
//   .line_path(x1,y1,x2,y2)  – a straight line (use .stroke, no fill)
//   Chainable modifiers (select relevant ones):
//     .fill(Color)  .stroke(Color, width)  .no_fill()  .no_stroke()
//     .corner_radius(r)  .bold()  .width(w)  .height(h)
//     .text_align_center()  .text_align_left()  .vertical_align_middle()
//     .letter_spacing(px)
//   Placement:  .at(x, y)  – top-left corner of the layer
//               .depth(f)  – z-order (0.0 = back, 1.0 = front)
//   Terminate:  .add()     – returns an Id
//
// Animate (comp.animate(id)):
//   .fade_in(t, dur)           – opacity 0→1
//   .ease_out() / .ease_in()   – easing modifier for preceding animation
//   .slide_from(dx, dy, t, dur)
//   .scale_from(from_scale, t, dur)
//   .spring(stiffness, damping)  – replaces the preceding tween with a spring
//   .kf(t, AnimatedProperty::…)         – raw keyframe
//   .kf_eased(t, AnimatedProperty::…, Easing::EASE_IN_OUT)
//   .clip_start(t)             – hide layer before time t
//   .apply()                   – commit animations
//
// AnimatedProperty variants:
//   ::opacity(f32)  ::position(x,y)  ::scale(sx,sy)  ::rotation_z(deg)
//   ::letter_spacing(px)  ::trim_path_end(0.0..=1.0)
//
// Easing: Easing::EASE_OUT  Easing::EASE_IN  Easing::EASE_IN_OUT  Easing::LINEAR
//
// Color: Color::hex("#RRGGBB")  .with_alpha(0.0..=1.0)
//
// Audio:
//   let vo_asset = AudioAsset::new(VO_SRC);
//   let vo_id = scene.add_audio_asset(vo_asset);
//   scene.main_composition.add_audio(
//       Audio::new(vo_id).with_start_time(0.0).with_duration(VO_DUR).with_volume(1.0),
//   );
//
// Inserting a sub-composition:
//   scene.assets.insert(comp_id.clone(), Asset::Composition(comp));
//   scene.main_composition.add_composition_clip(comp_id, start_t, duration);
// ─────────────────────────────────────────────────────────────────────────────

use kario_base::{
    animations::{AnimatedProperty, Easing},
    Asset, Audio, AudioAsset, Clip, Composition, Duration, Id, Scene,
};

const W: f32 = 1280.0;
const H: f32 = 720.0;
const VO_SRC: &str = "assets/vo.mp3";

// ── REPLACE these with real values from phrase timings ──────────────────────
const VO_DUR: f32 = 30.0;
const SCENE_DUR: f32 = 31.5;

// ── Palette (dark) ───────────────────────────────────────────────────────────
const HEX_BG: &str = "#0D1117";
const HEX_WHITE: &str = "#F0F6FC";
const HEX_MUTED: &str = "#8B949E";
const HEX_ACCENT: &str = "#4F46E5";
const HEX_ACCENT2: &str = "#7C3AED";

// ── SVG helpers (Lucide-style, inline, 64×64 viewBox) ───────────────────────
fn svg_check(c: &str) -> String {
    format!(r##"<svg viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg" fill="none"><circle cx="32" cy="32" r="28" fill="{c}"/><path d="M20 32l8 8 16-16" stroke="#FFFFFF" stroke-width="5" stroke-linecap="round" stroke-linejoin="round"/></svg>"##)
}
fn svg_arrow(c: &str) -> String {
    format!(r##"<svg viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg" fill="none"><path d="M10 32h44M38 18l16 14-16 14" stroke="{c}" stroke-width="5" stroke-linecap="round" stroke-linejoin="round"/></svg>"##)
}
fn svg_sparkle(c: &str) -> String {
    format!(r##"<svg viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg" fill="{c}"><path d="M32 4l4 24 24 4-24 4-4 24-4-24-24-4 24-4 4-24z"/></svg>"##)
}
fn svg_play(c: &str) -> String {
    format!(r##"<svg viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg" fill="none"><circle cx="32" cy="32" r="28" stroke="{c}" stroke-width="4"/><path d="M26 20l20 12-20 12V20z" fill="{c}"/></svg>"##)
}

// ── Accent underline bar drawn under a headline ──────────────────────────────
// Call after adding a headline; bar appears at (bar_x, bar_y).
fn add_underline(comp: &mut Composition, bar_x: f32, bar_y: f32, appear_t: f32, hide_t: f32) {
    let bar = comp
        .build_layer()
        .line_path(0.0, 0.0, 120.0, 0.0)
        .stroke(kario_base::styles::Color::hex(HEX_ACCENT), 4.0)
        .at(bar_x, bar_y)
        .depth(0.50)
        .add();
    comp.animate(bar)
        .clip_start(appear_t)
        .kf(appear_t, AnimatedProperty::trim_path_end(0.0))
        .kf_eased(appear_t + 0.35, AnimatedProperty::trim_path_end(1.0), Easing::EASE_OUT)
        .kf(hide_t, AnimatedProperty::opacity(1.0))
        .kf_eased(hide_t + 0.25, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();
}

// ═══════════════════════════════════════════════════════════════════════════════
// BEAT BUILDERS — replace each body with content for the real VO phrase.
// Each function builds one "beat" composition that lives for ~4–6 s on screen.
// Return the Id and call scene.assets.insert + scene.main_composition.add_composition_clip.
// ═══════════════════════════════════════════════════════════════════════════════

/// Beat 1: Opening hook — single large keyword slams in.
/// Replace "Your Headline" with the hook word/phrase from the script.
/// beat_in / beat_out → timing from phrase_timings.
fn build_beat_1(scene: &mut Scene, beat_in: f32, beat_out: f32) -> Id {
    let comp_id = Id::new();
    let mut comp = Composition::new(W, H);
    comp.id = comp_id.clone();
    comp.duration = Duration::Seconds(SCENE_DUR);
    let cx = W * 0.5;
    let cy = H * 0.5;

    // Full-canvas dark background
    let _bg = comp
        .build_layer()
        .rect(W, H)
        .fill(kario_base::styles::Color::hex(HEX_BG))
        .at(0.0, 0.0)
        .depth(0.0)
        .add();

    // Large icon metaphor (replace svg_sparkle with a thematic icon)
    let icon = comp
        .build_layer()
        .svg(svg_sparkle(HEX_ACCENT))
        .width(120.0)
        .height(120.0)
        .at(cx - 60.0, cy - 160.0)
        .depth(0.10)
        .add();
    comp.animate(icon)
        .fade_in(beat_in, 0.30)
        .ease_out()
        .scale_from(0.4, beat_in, 0.45)
        .spring(380.0, 16.0)
        .kf(beat_out, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_out + 0.25, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Eyebrow label (small caps, accent colour)
    let eyebrow = comp
        .build_layer()
        .text("THE PROBLEM", 13.0)   // ← replace text
        .width(400.0)
        .height(22.0)
        .bold()
        .letter_spacing(4.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(kario_base::styles::Color::hex(HEX_ACCENT))
        .at(cx - 200.0, cy - 24.0)
        .depth(0.11)
        .add();
    comp.animate(eyebrow)
        .fade_in(beat_in + 0.10, 0.35)
        .ease_out()
        .slide_from(0.0, 14.0, beat_in + 0.10, 0.45)
        .ease_out()
        .kf(beat_out, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_out + 0.25, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Hero headline (bold, white, large)
    let headline = comp
        .build_layer()
        .text("Your Headline", 72.0)   // ← replace text
        .width(1100.0)
        .height(90.0)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(kario_base::styles::Color::hex(HEX_WHITE))
        .at(cx - 550.0, cy + 12.0)
        .depth(0.12)
        .add();
    comp.animate(headline)
        .fade_in(beat_in + 0.20, 0.30)
        .ease_out()
        .scale_from(1.35, beat_in + 0.20, 0.40)
        .spring(420.0, 18.0)
        .kf(beat_out, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_out + 0.25, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    add_underline(&mut comp, cx - 60.0, cy + 108.0, beat_in + 0.50, beat_out);

    scene.assets.insert(comp_id.clone(), Asset::Composition(comp));
    comp_id
}

/// Beat 2: Pain / tension — keyword + supporting stat/phrase.
fn build_beat_2(scene: &mut Scene, beat_in: f32, beat_out: f32) -> Id {
    let comp_id = Id::new();
    let mut comp = Composition::new(W, H);
    comp.id = comp_id.clone();
    comp.duration = Duration::Seconds(SCENE_DUR);
    let cx = W * 0.5;
    let cy = H * 0.5;

    let _bg = comp
        .build_layer()
        .rect(W, H)
        .fill(kario_base::styles::Color::hex(HEX_BG))
        .at(0.0, 0.0)
        .depth(0.0)
        .add();

    // Visual metaphor icon on the left
    let icon = comp
        .build_layer()
        .svg(svg_arrow(HEX_ACCENT2))
        .width(100.0)
        .height(100.0)
        .at(cx - 300.0, cy - 50.0)
        .depth(0.10)
        .add();
    comp.animate(icon)
        .fade_in(beat_in, 0.25)
        .ease_out()
        .slide_from(-60.0, 0.0, beat_in, 0.45)
        .ease_out()
        .kf(beat_out, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_out + 0.25, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Headline to the right of the icon
    let headline = comp
        .build_layer()
        .text("Key Point", 64.0)   // ← replace text
        .width(700.0)
        .height(80.0)
        .bold()
        .vertical_align_middle()
        .fill(kario_base::styles::Color::hex(HEX_WHITE))
        .at(cx - 180.0, cy - 42.0)
        .depth(0.11)
        .add();
    comp.animate(headline)
        .fade_in(beat_in + 0.10, 0.30)
        .ease_out()
        .slide_from(40.0, 0.0, beat_in + 0.10, 0.45)
        .ease_out()
        .kf(beat_out, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_out + 0.25, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Supporting sub-label (muted, smaller)
    let sub = comp
        .build_layer()
        .text("supporting detail", 26.0)   // ← replace text
        .width(700.0)
        .height(36.0)
        .vertical_align_middle()
        .fill(kario_base::styles::Color::hex(HEX_MUTED))
        .at(cx - 180.0, cy + 50.0)
        .depth(0.11)
        .add();
    comp.animate(sub)
        .fade_in(beat_in + 0.45, 0.35)
        .ease_out()
        .slide_from(0.0, 10.0, beat_in + 0.45, 0.40)
        .ease_out()
        .kf(beat_out, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_out + 0.25, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    scene.assets.insert(comp_id.clone(), Asset::Composition(comp));
    comp_id
}

/// Beat 3: Solution reveal — accent headline + check icon.
fn build_beat_3(scene: &mut Scene, beat_in: f32, beat_out: f32) -> Id {
    let comp_id = Id::new();
    let mut comp = Composition::new(W, H);
    comp.id = comp_id.clone();
    comp.duration = Duration::Seconds(SCENE_DUR);
    let cx = W * 0.5;
    let cy = H * 0.5;

    let _bg = comp
        .build_layer()
        .rect(W, H)
        .fill(kario_base::styles::Color::hex(HEX_BG))
        .at(0.0, 0.0)
        .depth(0.0)
        .add();

    // Faint accent circle glow behind icon
    let glow = comp
        .build_layer()
        .circle(260.0)
        .fill(kario_base::styles::Color::hex(HEX_ACCENT).with_alpha(0.08))
        .at(cx - 130.0, cy - 200.0)
        .depth(0.08)
        .add();
    comp.animate(glow)
        .fade_in(beat_in, 0.50)
        .ease_out()
        .kf(beat_out, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_out + 0.25, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    let icon = comp
        .build_layer()
        .svg(svg_check(HEX_ACCENT))
        .width(120.0)
        .height(120.0)
        .at(cx - 60.0, cy - 170.0)
        .depth(0.10)
        .add();
    comp.animate(icon)
        .fade_in(beat_in, 0.25)
        .ease_out()
        .scale_from(0.3, beat_in, 0.45)
        .spring(400.0, 16.0)
        .kf(beat_out, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_out + 0.25, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    let headline = comp
        .build_layer()
        .text("The Solution", 72.0)   // ← replace text
        .width(1100.0)
        .height(90.0)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(kario_base::styles::Color::hex(HEX_ACCENT))
        .at(cx - 550.0, cy - 10.0)
        .depth(0.11)
        .add();
    comp.animate(headline)
        .fade_in(beat_in + 0.15, 0.30)
        .ease_out()
        .slide_from(0.0, 20.0, beat_in + 0.15, 0.45)
        .ease_out()
        .kf(beat_out, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_out + 0.25, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    let sub = comp
        .build_layer()
        .text("brief supporting phrase", 28.0)   // ← replace text
        .width(900.0)
        .height(40.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(kario_base::styles::Color::hex(HEX_MUTED))
        .at(cx - 450.0, cy + 88.0)
        .depth(0.11)
        .add();
    comp.animate(sub)
        .fade_in(beat_in + 0.55, 0.35)
        .ease_out()
        .slide_from(0.0, 12.0, beat_in + 0.55, 0.40)
        .ease_out()
        .kf(beat_out, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_out + 0.25, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    scene.assets.insert(comp_id.clone(), Asset::Composition(comp));
    comp_id
}

/// Beat 4: CTA / closing — product name + URL.
fn build_beat_4(scene: &mut Scene, beat_in: f32, beat_out: f32) -> Id {
    let comp_id = Id::new();
    let mut comp = Composition::new(W, H);
    comp.id = comp_id.clone();
    comp.duration = Duration::Seconds(SCENE_DUR);
    let cx = W * 0.5;
    let cy = H * 0.5;

    let _bg = comp
        .build_layer()
        .rect(W, H)
        .fill(kario_base::styles::Color::hex(HEX_BG))
        .at(0.0, 0.0)
        .depth(0.0)
        .add();

    // Play icon = "watch / try"
    let icon = comp
        .build_layer()
        .svg(svg_play(HEX_ACCENT))
        .width(100.0)
        .height(100.0)
        .at(cx - 50.0, cy - 160.0)
        .depth(0.10)
        .add();
    comp.animate(icon)
        .fade_in(beat_in, 0.30)
        .ease_out()
        .scale_from(0.5, beat_in, 0.40)
        .spring(380.0, 15.0)
        .kf(beat_out, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_out + 0.30, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // Product / brand name
    let brand = comp
        .build_layer()
        .text("Product Name", 80.0)   // ← replace text
        .width(1100.0)
        .height(100.0)
        .bold()
        .text_align_center()
        .vertical_align_middle()
        .fill(kario_base::styles::Color::hex(HEX_WHITE))
        .at(cx - 550.0, cy - 24.0)
        .depth(0.11)
        .add();
    comp.animate(brand)
        .fade_in(beat_in + 0.10, 0.25)
        .ease_out()
        .scale_from(1.3, beat_in + 0.10, 0.40)
        .spring(440.0, 20.0)
        .kf(beat_out, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_out + 0.30, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    // URL / CTA
    let url = comp
        .build_layer()
        .text("yourproduct.com", 30.0)   // ← replace text
        .width(900.0)
        .height(42.0)
        .text_align_center()
        .vertical_align_middle()
        .fill(kario_base::styles::Color::hex(HEX_ACCENT))
        .at(cx - 450.0, cy + 88.0)
        .depth(0.11)
        .add();
    comp.animate(url)
        .fade_in(beat_in + 0.55, 0.40)
        .ease_out()
        .slide_from(0.0, 14.0, beat_in + 0.55, 0.45)
        .ease_out()
        .kf(beat_out, AnimatedProperty::opacity(1.0))
        .kf_eased(beat_out + 0.30, AnimatedProperty::opacity(0.0), Easing::EASE_IN)
        .apply();

    add_underline(&mut comp, cx - 60.0, cy + 140.0, beat_in + 0.70, beat_out);

    scene.assets.insert(comp_id.clone(), Asset::Composition(comp));
    comp_id
}

// ═══════════════════════════════════════════════════════════════════════════════
// MAIN BUILD — wire VO, background, and beats together.
// Adjust beat_in/beat_out values to match phrase_timings.
// Add/duplicate build_beat_* calls for more phrases.
// ═══════════════════════════════════════════════════════════════════════════════
pub fn build() -> Scene {
    let mut scene = Scene::new(W, H);
    scene.main_composition.duration = Duration::Seconds(SCENE_DUR);

    // ── Persistent dark background ────────────────────────────────────────────
    let bg_id = {
        let comp_id = Id::new();
        let mut comp = Composition::new(W, H);
        comp.id = comp_id.clone();
        comp.duration = Duration::Seconds(SCENE_DUR);
        let _bg = comp
            .build_layer()
            .rect(W, H)
            .fill(kario_base::styles::Color::hex(HEX_BG))
            .at(0.0, 0.0)
            .depth(0.0)
            .add();
        scene.assets.insert(comp_id.clone(), Asset::Composition(comp));
        comp_id
    };
    scene.main_composition.add_composition_clip(bg_id, 0.0, SCENE_DUR);

    // ── Voice-over audio ──────────────────────────────────────────────────────
    let vo_asset = AudioAsset::new(VO_SRC);
    let vo_id = scene.add_audio_asset(vo_asset);
    scene.main_composition.add_audio(
        Audio::new(vo_id)
            .with_start_time(0.0)
            .with_duration(VO_DUR)
            .with_volume(1.0),
    );

    // ── Beats (replace timings with real phrase_timings values) ──────────────
    // beat_in  = phrase start_seconds
    // beat_out = phrase start_seconds + phrase duration_seconds
    let b1 = build_beat_1(&mut scene, 0.0,  7.0);
    scene.main_composition.add_composition_clip(b1, 0.0, SCENE_DUR);

    let b2 = build_beat_2(&mut scene, 7.0,  15.0);
    scene.main_composition.add_composition_clip(b2, 0.0, SCENE_DUR);

    let b3 = build_beat_3(&mut scene, 15.0, 23.0);
    scene.main_composition.add_composition_clip(b3, 0.0, SCENE_DUR);

    let b4 = build_beat_4(&mut scene, 23.0, 30.0);
    scene.main_composition.add_composition_clip(b4, 0.0, SCENE_DUR);

    scene
}

