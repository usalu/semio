//! ⚓️ `shell-dock` staging of `tree-windows`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use super::tree_windows::*;
use super::{ShellState, UiDirtyScope};
use std::collections::{BTreeMap, BTreeSet};
use ui_wgpu::wgpu::tree_window::{served_requests, TreeWindowRequest, TreeWindowServedMemory};

/// 🪟️ One body's host-owned tree state, keyed by window path (`TreeWindowBodyState`).
#[derive(Clone, Debug, Default)]
pub(super) struct TreeWindowBodyState {
    pub(super) open: BTreeMap<String, bool>,
    pub(super) windows: BTreeMap<String, (u32, u32)>,
    pub(super) measured: BTreeSet<String>,
}

impl ShellDockState {
    /// 🪟️ Measures one retained body that just finished a paint and reports its windows (React's `useTreeWindowObserver` measure):
    /// the served rule places each window, the scheduler decides whether that changes anything.
    pub(crate) fn observe_tree_windows(&mut self, surface_id: &str, docs: &DocumentState) {
        let Some((viewport_height, containers)) = crate::interpreter::tree_window_measures(surface_id) else { return };
        let body = self.tree_window_body_key(surface_id, docs);
        let memory = self.tree_window_served.remove(&body).unwrap_or_default();
        let (requests, memory) = served_requests(&containers, viewport_height, &memory, ui_contract::TREE_WINDOW_BODY_NODE_BUDGET);
        self.tree_window_served.insert(body.clone(), memory);
        let row_px = f64::from(ui_wgpu::wgpu::layout::tree_window_row_extent_px(ui_wgpu::wgpu::UiTreeWindowRowExtent::Standard));
        let viewport_rows = ((viewport_height / row_px).ceil() as u32).max(1);
        let now = semio_framework_job::default_now_us().map_or(0, |now| now / 1_000);
        self.tree_windows.report(&body, &requests, viewport_rows, now);
    }

    /// 🪟️ The body a retained surface's windows belong to: the guest body it renders, else (a shell-owned panel) the surface itself.
    pub(super) fn tree_window_body_key(&self, surface_id: &str, docs: &DocumentState) -> String {
        self.document_body_key(surface_id, docs).unwrap_or_else(|| surface_id.to_owned())
    }
}

