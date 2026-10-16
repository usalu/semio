//! 🗨️ Chrome overlays: notices, bands, agent approvals, dialog requests and the tour.
//!
//! Tooltips, transient notices, plugin-install and document-open bands, dispatch fault notices, agent approval
//! paint ops, chrome dialog requests and helpers, the chrome tour state and element rects.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
// 🍿️ w3-overlays-chrome-polish (WP15+WP16): tooltips, a generic modal dialog, and the app introduction
// tour — layered on the existing immediate-mode chrome through `ShellChromeBuildState`, while
// `OverlayState` remains dedicated to the pre-existing menu/search overlays.
// Placement math reuses `ui_wgpu`'s w1d-events-overlay manager types (`OverlayKind`, `OverlayPlacement`,
// `resolve_overlay_placement`) even though the manager's own `EventRouter`/`open_overlay` stay
// `pub(crate)` to `ui_wgpu` (an `engine::Ui`/retained-`UiTree` implementation detail) and out of reach
// for this non-tree-based immediate-mode chrome renderer — confirmed unreachable the same way
// `report-w2-text-editor.md` found for its own local-fallback popup.

//#region 🧯️TransientNoticeAndAgentOverlays
// 🧯️ WGPU-RENDERER-REACT-PARITY packet W1j. React's `showTransientNotice` (`🏛️ShellHost/🟦️.tsx`) is
// the ONE channel every non-fatal shell fault reaches the user through — viewer-read-only,
// mutation-rejected, render-error, keybinding-unowned, a guest `Effect::Notify`, a degraded remote
// merge. This shell had no equivalent at all: such faults either set the persistent bottom-left
// `self.error` line (which never clears) or vanished. The three rules below are React's, exactly:
// ONE notice at a time (a new one replaces rather than queues), auto-dismiss after 4 s, and an
// explicit close control.



//#endregion 🧯️TransientNoticeAndAgentOverlays

//#region 🗨️ChromeDialog

//#endregion 🗨️ChromeDialog

//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome
