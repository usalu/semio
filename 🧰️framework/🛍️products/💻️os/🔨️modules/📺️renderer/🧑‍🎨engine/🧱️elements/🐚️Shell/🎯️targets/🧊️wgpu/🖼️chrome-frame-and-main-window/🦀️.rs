//! 🖼️ Chrome frame cursors and the main window render steps.
//!
//! `ShellChromeFrameCursor` and the child cursors, plus the first render half of `impl ShellState`: the chrome
//! step, preferences/introduction steps, maintenance, board presence, navbar layout, dock windows, the main
//! window, anchors, panels and the mobile panel.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellChrome

/// 🩺️ Opportunities one chrome phase may spend before the walk reports itself parked. A complete
/// walk of a busy shell spends far fewer than this in ANY single phase — every phase is bounded by
/// its own content (a glyph run, a dock tab, one document's `SHELL_WINDOW_PAINT_OPPORTUNITIES`) —
/// so crossing it means the phase's own cursor is not advancing at all.
pub(super) const SHELL_CHROME_PHASE_OPPORTUNITIES: u64 = 1 << 18;
/// 🩺️ How many times one parked phase reports itself before it goes quiet: enough to prove the walk
/// is still parked rather than merely slow, bounded so a wedged shell cannot flood the console.
pub(super) const SHELL_CHROME_PARKED_REPORTS: u8 = 4;

//#endregion ShellChrome
