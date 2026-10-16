//! 🚶️ `shell-session` staging of `tree-windows`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use super::tree_windows::*;
use super::{ShellState, UiDirtyScope};

impl<'ctx> SessionCtx<'ctx> {
    /// 🪟️ One settle step of the scheduler: bodies whose refresh crossed settle (the owed scope drained and nothing renders), and
    /// every body now due refreshes — a guest body through the owed partial refresh, a shell-owned panel by republishing it.
    pub(crate) fn pump_tree_windows(&mut self) {
        if self.settle.owed_refresh_scope.asks_for_nothing() && self.settle.settle_pump.rendering.is_none() {
            for body in self.dock.tree_windows.in_flight() {
                self.dock.tree_windows.settled(&body);
            }
        }
        let now = semio_framework_job::default_now_us().map_or(0, |now| now / 1_000);
        for body in self.dock.tree_windows.due(now) {
            let window_body = self.docs.session.as_ref().is_some_and(|session| session.app.window_kinds.iter().any(|kind| kind.body_key == body));
            let guest_panel = !window_body && self.docs.session.as_ref().is_some_and(|session| flatten_panel_tab_leaves(&session.app.panel_tabs).into_iter().any(|tab| tab.body_key.as_deref() == Some(body.as_str())));
            if window_body || guest_panel {
                let (window_bodies, panel_bodies) = if window_body { (vec![body.clone()], Vec::new()) } else { (Vec::new(), vec![body.clone()]) };
                self.settle.owe_refresh(UiDirtyScope::Partial { window_bodies, panel_bodies, utilities: false, tools: false, engagements: false, measures: false, labels: false });
                continue;
            }
            if let Err(error) = self.republish_shell_panel_document(&body) {
                self.chrome.error = Some(error);
            }
            self.dock.tree_windows.settled(&body);
        }
    }
}

