//! 🖍 `shell-view-paint` staging of `space-admin-and-settle`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region ShellTypes
/// 📏️ Opportunities one window body's retained paint may spend inside a single chrome walk before
/// the walk gives up on it and moves to the next window. Generous enough for the largest document
/// this shell renders and finite so one stalled body cannot hold the frame — and with it the navbar,
/// the panels and the GPU present — forever.
pub const SHELL_WINDOW_PAINT_OPPORTUNITIES: usize = 1 << 20;
//#endregion ShellTypes
