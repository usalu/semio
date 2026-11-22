//! 🖍 `shell-view-paint` staging of `tutorial-runtime`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region ShellChrome
//#region 🎬️Tutorial
//#region 🎬️PureHelpers
/// 🎚️ Pure scrub-bar progress (0–1) for a given playhead/duration — `0.0` for a zero-length timeline
/// (the in-progress recording case) rather than dividing by zero.
pub fn tutorial_scrub_progress(playhead_ms: f64, duration_ms: u64) -> f32 {
    if duration_ms == 0 {
        0.0
    } else {
        (playhead_ms / duration_ms as f64).clamp(0.0, 1.0) as f32
    }
}

/// 🎚️ Pure scrub-bar hit math: pointer x within `track` → target playhead ms.
pub fn tutorial_scrub_target_ms(pointer_x: f32, track: Rect, duration_ms: u64) -> f64 {
    let t = ((pointer_x - track.x) / track.w.max(1.0)).clamp(0.0, 1.0);
    t as f64 * duration_ms as f64
}
//#endregion 🎬️PureHelpers
//#region 🎥️TutorialPresentation
/// 💬️ React's `max-w-2xl` caption box width, in pixels.
pub const TUTORIAL_CAPTION_MAX_WIDTH_PX: f32 = 672.0;

/// 👻️ React's `opacity-90` ghost pointer.
pub const TUTORIAL_GHOST_OPACITY: f32 = 0.9;

/// 👻️ React's `size-double`: the ghost pointer's side in UI spacing units.
pub const TUTORIAL_GHOST_SIZE_UI_SPACING: f32 = 2.0;

/// 📹️ Video drift past which the overlay reseeks to the playhead, in milliseconds.
pub const TUTORIAL_VIDEO_RESYNC_TOLERANCE_MS: f64 = 300.0;

/// ✂️ Index of the first cue whose half-open `[at, at + duration)` window covers `playhead_ms` — React's `tutorialCuesBetween(...)[0]`.
pub fn tutorial_cue_index(spans: impl IntoIterator<Item = (u64, u64)>, playhead_ms: f64) -> Option<usize> {
    spans.into_iter().position(|(at, duration)| playhead_ms >= at as f64 && playhead_ms < (at + duration) as f64)
}

/// 📖️ Scrub-bar position `0..=1` of a chapter tick: `atMs / durationMs`, `0` for a zero-length timeline.
pub fn tutorial_chapter_fraction(at_ms: u64, duration_ms: u64) -> f32 {
    if duration_ms == 0 {
        0.0
    } else {
        (at_ms as f64 / duration_ms as f64).clamp(0.0, 1.0) as f32
    }
}

/// 📖️ The chapter tick on the scrub track: a `size-1` (one spacing unit) dot centred at `atMs / durationMs`.
pub fn tutorial_chapter_tick_rect(track: Rect, at_ms: u64, duration_ms: u64, theme: &Theme) -> Rect {
    let size = theme.gap_standard;
    Rect::new(track.x + track.w * tutorial_chapter_fraction(at_ms, duration_ms) - size * 0.5, track.y + (track.h - size) * 0.5, size, size)
}

/// 🎯️ The pointer target of a chapter tick: at least 12 px square around the dot.
pub fn tutorial_chapter_tick_hit_rect(tick: Rect) -> Rect {
    let side = tick.w.max(12.0);
    Rect::new(tick.x + (tick.w - side) * 0.5, tick.y + (tick.h - side) * 0.5, side, side)
}

/// 📹️ A cue's normalized `[x, y, width, height]` rect in viewport pixels.
pub fn tutorial_video_rect(normalized: [f64; 4], viewport_w: f32, viewport_h: f32) -> Rect {
    Rect::new(normalized[0] as f32 * viewport_w, normalized[1] as f32 * viewport_h, normalized[2] as f32 * viewport_w, normalized[3] as f32 * viewport_h)
}

/// ⏱️ Offset into a video cue's own source at `playhead_ms`.
pub fn tutorial_video_local_time_ms(playhead_ms: f64, cue_at_ms: u64, source_offset_ms: u64) -> f64 {
    playhead_ms - cue_at_ms as f64 + source_offset_ms as f64
}

/// 🎯️ Whether a video that sits at `current_ms` must be hard-reseeked to `target_ms`.
pub fn tutorial_video_needs_reseek(current_ms: f64, target_ms: f64) -> bool {
    (current_ms - target_ms).abs() > TUTORIAL_VIDEO_RESYNC_TOLERANCE_MS
}

/// 📈️ `0..=1` progress through a gesture cue, driven by the playhead.
pub fn tutorial_ghost_progress(playhead_ms: f64, cue_at_ms: u64, duration_ms: u64) -> f32 {
    ((playhead_ms - cue_at_ms as f64) / duration_ms.max(1) as f64).clamp(0.0, 1.0) as f32
}

/// 🌈️ React's `introductionDemoArcPoint`: the quadratic arc an orbit gesture bows along.
pub fn tutorial_ghost_arc_point(from: (f32, f32), to: (f32, f32), t: f32) -> (f32, f32) {
    let (dx, dy) = (to.0 - from.0, to.1 - from.1);
    let distance = dx.hypot(dy);
    let distance = if distance == 0.0 { 1.0 } else { distance };
    let bulge = (distance * 0.35).min(64.0);
    let control = ((from.0 + to.0) / 2.0 + (-dy / distance) * bulge, (from.1 + to.1) / 2.0 + (dx / distance) * bulge);
    let remaining = 1.0 - t;
    (remaining * remaining * from.0 + 2.0 * remaining * t * control.0 + t * t * to.0, remaining * remaining * from.1 + 2.0 * remaining * t * control.1 + t * t * to.1)
}

/// 🖱️ React's `INTRODUCTION_DEMO_GESTURE_DEFAULT_CURSOR`.
pub fn tutorial_ghost_default_cursor(gesture: &semio_framework::onboarding::IntroductionGesture) -> semio_framework::onboarding::IntroductionCursor {
    use semio_framework::onboarding::{IntroductionCursor as C, IntroductionGesture as G};
    match gesture {
        G::LeftClick { .. } | G::RightClick { .. } | G::DoubleClick { .. } => C::Pointer,
        G::Drag { .. } => C::Grab,
        G::Scroll { .. } => C::Default,
        G::Orbit { .. } => C::Move,
    }
}

/// 🌗️ Whether the theme paints on a dark ground, choosing the dark cursor assets.
pub fn tutorial_theme_is_dark(theme: &Theme) -> bool {
    0.2126 * theme.background.r + 0.7152 * theme.background.g + 0.0722 * theme.background.b < 0.214
}

/// 💬️ The covering narration cue's text, `None` while captions are off or no cue covers the playhead.
pub fn tutorial_caption_text(runtime: &semio_renderer_wgpu_shell_surfaces::TutorialRuntime, terminology: semio_ui_locale::Terminology, locale: semio_ui_locale::Locale) -> Option<String> {
    if !runtime.captions_on {
        return None;
    }
    let narration = &runtime.definition.tracks.narration;
    let cue = narration.get(tutorial_cue_index(narration.iter().map(|cue| (cue.at, cue.duration_ms)), runtime.playhead_ms)?)?;
    let text = cue.text.resolve(terminology, locale);
    (!text.is_empty()).then(|| text.to_string())
}

/// 💬️ The caption glass box and its text rect: bottom-centre, `bottom-double`, at most [`TUTORIAL_CAPTION_MAX_WIDTH_PX`] and half the viewport wide (a `left-1/2` shrink-to-fit box), `p-double` inside.
pub fn tutorial_caption_layout(runtime: &semio_renderer_wgpu_shell_surfaces::TutorialRuntime, atlas: &mut FontAtlas, theme: &Theme, terminology: semio_ui_locale::Terminology, locale: semio_ui_locale::Locale, width: f32, height: f32) -> Option<(String, Rect, Rect)> {
    if !matches!(runtime.mode, semio_renderer_wgpu_shell_surfaces::TutorialMode::Playing | semio_renderer_wgpu_shell_surfaces::TutorialMode::Paused | semio_renderer_wgpu_shell_surfaces::TutorialMode::Deviated) {
        return None;
    }
    let text = tutorial_caption_text(runtime, terminology, locale)?;
    let pad = theme.gap_standard * 2.0;
    let inner_max = (TUTORIAL_CAPTION_MAX_WIDTH_PX.min(width * 0.5) - pad * 2.0).max(1.0);
    let (measured_w, measured_h) = atlas.measure_text_wrapped(&text, inner_max, theme.font_size_body);
    let inner_w = measured_w.min(inner_max).max(1.0);
    let rect = Rect::new((width - inner_w - pad * 2.0) * 0.5, height - pad - measured_h - pad * 2.0, inner_w + pad * 2.0, measured_h + pad * 2.0);
    Some((text, rect, Rect::new(rect.x + pad, rect.y + pad, inner_w, measured_h)))
}

/// 📹️ One resolved video cue: where it sits, what it plays and how it is kept on the playhead.
#[derive(Clone, Debug, PartialEq)]
pub struct TutorialVideoOverlay {
    pub rect: Rect,
    pub src: String,
    pub muted: bool,
    pub playing: bool,
    pub rate: f32,
    pub local_time_ms: f64,
}

/// 📹️ The video cue covering the playhead, `None` for a blob source (no blob store is reachable from the shell chrome) or while no cue covers it.
pub fn tutorial_video_overlay(runtime: &semio_renderer_wgpu_shell_surfaces::TutorialRuntime, viewport_w: f32, viewport_h: f32) -> Option<TutorialVideoOverlay> {
    use semio_framework::onboarding::TutorialAssetSrc as Src;
    let video = &runtime.definition.tracks.video;
    let cue = video.get(tutorial_cue_index(video.iter().map(|cue| (cue.at, cue.duration_ms)), runtime.playhead_ms)?)?;
    let src = match &cue.src {
        Src::Url { url } => url.clone(),
        Src::DataUrl { data } => data.clone(),
        Src::Blob { .. } => return None,
    };
    Some(TutorialVideoOverlay {
        rect: tutorial_video_rect([cue.rect.x, cue.rect.y, cue.rect.width, cue.rect.height], viewport_w, viewport_h),
        src,
        muted: runtime.muted || cue.muted,
        playing: runtime.mode == semio_renderer_wgpu_shell_surfaces::TutorialMode::Playing,
        rate: runtime.rate,
        local_time_ms: tutorial_video_local_time_ms(runtime.playhead_ms, cue.at, cue.source_offset_ms),
    })
}

/// 👻️ Paints one ghost glyph contain-fitted into the `size` square centred on `(x, y)`, at [`TUTORIAL_GHOST_OPACITY`] — React's `bg-contain bg-no-repeat` cursor image.
pub fn paint_tutorial_ghost_glyph(overlay: &mut DrawList, glyph: &GhostGlyph, x: f32, y: f32, size: f32) {
    let scale = (size / glyph.width).min(size / glyph.height);
    let (origin_x, origin_y) = (x - glyph.width * scale * 0.5, y - glyph.height * scale * 0.5);
    for polygon in glyph.polygons {
        let points: Vec<[f32; 2]> = polygon.points.iter().map(|point| [origin_x + point[0] * scale, origin_y + point[1] * scale]).collect();
        overlay.push_triangle_fan_overlay(&points, Rgba::from_srgb8(polygon.fill[0], polygon.fill[1], polygon.fill[2], 255).with_alpha(TUTORIAL_GHOST_OPACITY));
        let stroke = Rgba::from_srgb8(polygon.stroke[0], polygon.stroke[1], polygon.stroke[2], 255).with_alpha(TUTORIAL_GHOST_OPACITY);
        for (index, from) in points.iter().enumerate() {
            let to = points[(index + 1) % points.len()];
            overlay.push_line_overlay(from[0], from[1], to[0], to[1], stroke, polygon.stroke_width * scale);
        }
    }
}

/// 👻️ Paints the ghost pointer of the gesture cue covering the playhead: click-family and scroll gestures sit at their point, a drag lerps from to, an orbit bows along the arc.
pub fn render_tutorial_gesture_overlay_node(chrome: &semio_renderer_wgpu_shell_chrome_state::ChromeState, surfaces: &semio_renderer_wgpu_shell_surfaces::SurfaceState, dock: &semio_renderer_wgpu_shell_dock::ShellDockState, tutorial: &semio_renderer_wgpu_shell_surfaces::TutorialState, overlay: &mut DrawList, theme: &Theme) {
    use semio_framework::onboarding::IntroductionGesture as G;
    use semio_renderer_wgpu_shell_surfaces::TutorialMode;
    let Some(runtime) = tutorial.tutorial.as_ref() else { return };
    if !matches!(runtime.mode, TutorialMode::Playing | TutorialMode::Paused | TutorialMode::Deviated) {
        return;
    }
    let gestures = &runtime.definition.tracks.gestures;
    let Some(cue) = tutorial_cue_index(gestures.iter().map(|cue| (cue.at, cue.duration_ms)), runtime.playhead_ms).and_then(|index| gestures.get(index)) else { return };
    let progress = tutorial_ghost_progress(runtime.playhead_ms, cue.at, cue.duration_ms);
    let resolve = |point| semio_renderer_wgpu_shell_dock::tutorial_resolve_gesture_point(chrome, surfaces, dock, point);
    let position = match &cue.gesture {
        G::LeftClick { at } | G::RightClick { at } | G::DoubleClick { at } | G::Scroll { at, .. } => resolve(at),
        G::Drag { from, to, .. } => match (resolve(from), resolve(to)) {
            (Some(from), Some(to)) => Some((from.0 + (to.0 - from.0) * progress, from.1 + (to.1 - from.1) * progress)),
            (from, to) => from.or(to),
        },
        G::Orbit { from, to, .. } => match (resolve(from), resolve(to)) {
            (Some(from), Some(to)) => Some(tutorial_ghost_arc_point(from, to, progress)),
            (from, to) => from.or(to),
        },
    };
    let Some((x, y)) = position else { return };
    let glyph = tutorial_ghost_glyph(cue.cursor.unwrap_or_else(|| tutorial_ghost_default_cursor(&cue.gesture)), tutorial_theme_is_dark(theme));
    paint_tutorial_ghost_glyph(overlay, glyph, x, y, theme.gap_standard * TUTORIAL_GHOST_SIZE_UI_SPACING);
}
//#endregion 🎥️TutorialPresentation
//#endregion 🎬️Tutorial
//#endregion ShellChrome
