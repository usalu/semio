//! 🖍 `shell-view-paint` staging of `tutorial-runtime`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region ShellChrome
//#region 🎬️Tutorial
//#region 🎬️PureHelpers
/// 🎚️ Pure scrub-bar progress (0–1) for a given playhead/duration — `0.0` for a zero-length timeline
/// (the in-progress recording case) rather than dividing by zero.
pub(crate) fn tutorial_scrub_progress(playhead_ms: f64, duration_ms: u64) -> f32 {
    if duration_ms == 0 {
        0.0
    } else {
        (playhead_ms / duration_ms as f64).clamp(0.0, 1.0) as f32
    }
}

/// 🎚️ Pure scrub-bar hit math: pointer x within `track` → target playhead ms.
pub(crate) fn tutorial_scrub_target_ms(pointer_x: f32, track: Rect, duration_ms: u64) -> f64 {
    let t = ((pointer_x - track.x) / track.w.max(1.0)).clamp(0.0, 1.0);
    t as f64 * duration_ms as f64
}
//#endregion 🎬️PureHelpers
//#endregion 🎬️Tutorial
//#endregion ShellChrome
