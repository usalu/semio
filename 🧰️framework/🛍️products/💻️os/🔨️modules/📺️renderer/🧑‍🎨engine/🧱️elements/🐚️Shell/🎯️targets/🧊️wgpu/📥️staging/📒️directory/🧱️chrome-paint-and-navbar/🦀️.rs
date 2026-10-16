//! 📒️ `shell-directory` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
//#region 🔀️ChromeParity
/// 👥️🖼️ One painted piece of board peer presence: a peer's cursor (with its viewport and name chip) or a
/// peer mark chip, each with the board it is clipped to.
#[derive(Clone, Debug)]
pub(crate) enum ShellBoardPresencePaint {
    Cursor { bounds: Rect, peer: crate::canvas_presence::BoardPeerCursor },
    Mark { bounds: Rect, rect: Rect, mark: crate::canvas_presence::BoardPeerMark },
}

/// 👥️🖼️ A presence chip's width for its text — the same estimate the status pills use.
pub(crate) fn board_presence_chip_width(theme: &Theme, text: &str) -> f32 {
    8.0 + text.chars().count() as f32 * theme.font_size_small * 0.6
}

//#endregion 🔀️ChromeParity
//#endregion ActionPanelAndUtilities

