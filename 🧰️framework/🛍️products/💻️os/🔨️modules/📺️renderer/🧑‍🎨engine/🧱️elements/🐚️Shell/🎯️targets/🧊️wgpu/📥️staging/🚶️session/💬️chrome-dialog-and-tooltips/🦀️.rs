//! 🚶️ `shell-session` staging of `chrome-dialog-and-tooltips`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl<'ctx> SessionCtx<'ctx> {
    /// 🎓️ Force-reveals whatever the active step's `introduce`/`show` ids target — folded action rails,
    /// nested utility collections, closed panel tabs — before any of that chrome paints this frame; must
    /// run ahead of `render_main_window`/`render_left_panel`/`render_right_panel`/`render_footer`, all of
    /// which read the fold/open state this writes. Latched per step id in owned build state so a
    /// user who re-folds/closes what the tour revealed doesn't get it snapped back open next frame —
    /// mirrors the React shell's own reveal effects, which likewise fire once per step.
    pub fn chrome_tour_frame_begin(&mut self) {
        let Some(step) = self.docs.chrome_tour_active_step(&self.chrome) else {
            return;
        };
        let already_latched = self.chrome.chrome_build.tour_reveal_latch.as_deref() == Some(step.id.as_str());
        if already_latched {
            return;
        }
        self.chrome.chrome_build.tour_reveal_latch = Some(step.id.clone());
        let Some(session) = self.docs.session.clone() else {
            return;
        };
        let ids: Vec<String> = step.introduce.iter().cloned().chain(step.show.iter().cloned()).collect();
        for id in ids {
            if id == semio_framework::UI_NAVBAR_ELEMENT_ID || id == semio_framework::UI_FOOTER_ELEMENT_ID {
                continue;
            }
            if let Some(rest) = id.strip_prefix("framework.window.") {
                if let Some((segment, _action_id)) = rest.split_once(".action.") {
                    let window_id = session.app.window_kinds.iter().find(|kind| semio_framework::element_id_segment(&kind.id) == segment).map(|kind| kind.id.clone());
                    if let Some(window_id) = window_id {
                        self.windows.action_panel_folded.insert(window_id, false);
                    }
                }
                continue;
            }
            if let Some(rest) = id.strip_prefix("framework.panelTab.") {
                let tab_id = rest.strip_suffix(".firstDraggable").unwrap_or(rest);
                self.dock.chrome_tour_reveal_panel_tab(&session, tab_id, &mut self.settle, &self.plugins);
                continue;
            }
            for collection_id in utility_collection_path_to_id(&self.windows.active_utilities, &id) {
                self.windows.utility_collection_expanded.insert(collection_id, true);
            }
        }
    }
}

//#endregion ShellChrome

