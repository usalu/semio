//! 🖍 `shell-view-paint` staging of `dock-model-and-measures`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region ShellLifecycle
//#region 🧭️PanelAnchorModel
/// 📑️ Places root-to-leaf tab rows from the anchor's logical block start.
pub(crate) fn flowed_anchor_tab_row_y(block: ui_contract::FlowBlock, panel: Rect, row_index: usize, row_count: usize, row_height: f32) -> f32 {
    if block.is_reversed() {
        panel.y + panel.h - (row_index.min(row_count) + 1) as f32 * row_height
    } else {
        panel.y + row_index as f32 * row_height
    }
}

/// 📐️ The boundary between retained content and an anchor's complete tab bar.
pub(crate) fn flowed_anchor_tab_divider_y(block: ui_contract::FlowBlock, panel: Rect, bar_height: f32) -> f32 {
    if block.is_reversed() {
        panel.y + panel.h - bar_height
    } else {
        panel.y + bar_height
    }
}
//#endregion 🧭️PanelAnchorModel
//#endregion ShellLifecycle
