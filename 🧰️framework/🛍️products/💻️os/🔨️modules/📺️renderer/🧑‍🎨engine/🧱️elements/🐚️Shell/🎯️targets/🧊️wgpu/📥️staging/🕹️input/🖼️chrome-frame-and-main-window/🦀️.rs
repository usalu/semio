//! 🕹️ `shell-input` staging of `chrome-frame-and-main-window`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl ShellInputState {
    pub(crate) fn reveal_panel_tab_row(&mut self, anchor: PanelAnchor, tab_id: &str, dock: &ShellDockState) {
        if let Some(row_index) = dock.anchor_panel_tab_rows(anchor).iter().position(|row| row.iter().any(|node| node.id == tab_id)) {
            self.scroll_offsets.insert(panel_tab_scroll_id(anchor, row_index), f32::MAX);
        }
    }
}

//#endregion ShellChrome

pub(super) fn panel_tab_scroll_id(anchor: PanelAnchor, row_index: usize) -> String {
    format!("shell.panel.tabs.{}.{}.scroll", anchor.as_str(), row_index)
}
