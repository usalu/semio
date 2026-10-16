//! 🎓️ Tutorial runtime.
//!
//! Tutorial mode and runtime types, camera conversion, UI snapshots, the gesture overlay, recording and the
//! playhead/scrub helpers with their `impl ShellState`.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellChrome
//#region 🎬️Tutorial
// 🎬️ The wgpu-shell half of the Tutorial mechanism (sibling of `//#region 🔖️ChromeOverlaysAndTour`'s
// introduction tour, above) — see `framework/core/rs/lib.rs`'s `//#region 🔖️Tutorial` for the shared
// data model and pure engine fns (`tutorial_slice`/`compose_tutorial_ui`/`interpolate_tutorial_camera`/
// `tutorial_camera_at`/`apply_tutorial_ui_change`) this region calls directly rather than reimplementing.
// The React shell's own half lives in `framework/renderer/react/index.tsx`; both are built against the
// same core model but neither coordinates with the other beyond that shared semantics.







//#endregion 🎬️Tutorial
//#endregion ShellChrome
