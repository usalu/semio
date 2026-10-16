//! ⚓️ `shell-dock` staging of `tree-windows`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use super::UiDirtyScope;
use std::collections::{BTreeMap, BTreeSet};
use ui_wgpu::wgpu::tree_window::{served_requests, TreeWindowRequest, TreeWindowServedMemory};

/// 🪟️ One body's host-owned tree state, keyed by window path (`TreeWindowBodyState`).
#[derive(Clone, Debug, Default)]
pub struct TreeWindowBodyState {
    pub open: BTreeMap<String, bool>,
    pub windows: BTreeMap<String, (u32, u32)>,
    pub measured: BTreeSet<String>,
}

impl ShellDockState {
    /// 🪟️ Measures one retained body that just finished a paint and reports its windows (React's `useTreeWindowObserver` measure):
    /// the served rule places each window, the scheduler decides whether that changes anything.
    pub fn observe_tree_windows(&mut self, surface_id: &str, docs: &DocumentState) {
        let Some((viewport_height, containers)) = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::tree_window_measures(surface_id) else { return };
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
    pub fn tree_window_body_key(&self, surface_id: &str, docs: &DocumentState) -> String {
        self.document_body_key(surface_id, docs).unwrap_or_else(|| surface_id.to_owned())
    }
}

/// 🚧️ Whether one path may cross in a view context (`treeWindowSendableIdentifierV1`): non-empty, at most 256 code points, no C0
/// control character and no DEL.
pub fn tree_window_sendable_identifier(identifier: &str) -> bool {
    !identifier.is_empty() && identifier.chars().count() <= 256 && !identifier.chars().any(|character| character.is_ascii_control())
}

impl TreeWindowScheduler {
    /// 🪟️ Records the user opening or closing one container (`setOpen`) — due immediately, and opening forgets what the observer
    /// last measured for it so the first-paint allowance speaks for it again.
    pub fn set_open(&mut self, body: &str, node_key: &str, open: bool) {
        let state = self.bodies.entry(body.to_owned()).or_default();
        if state.open.get(node_key) == Some(&open) {
            return;
        }
        state.open.insert(node_key.to_owned(), open);
        if open {
            state.measured.remove(node_key);
            state.windows.remove(node_key);
        }
        self.pending.insert(body.to_owned());
        self.open_now = true;
    }

    /// 🪟️ One body's WHOLE window report (`reportWindows`): replaces its windows (pruning containers that left the body), opens
    /// every measured container the host was not told otherwise about, and arms the debounce — unless nothing changed.
    pub fn report(&mut self, body: &str, requests: &[TreeWindowRequest], rows: u32, now_ms: u64) {
        let state = self.bodies.entry(body.to_owned()).or_default();
        let next: BTreeMap<String, (u32, u32)> = requests.iter().map(|request| (request.key.clone(), (request.offset, request.rows))).collect();
        let same_windows = next == state.windows;
        let mut opened_any = false;
        for key in next.keys() {
            state.measured.insert(key.clone());
            if !state.open.contains_key(key) {
                state.open.insert(key.clone(), true);
                opened_any = true;
            }
        }
        let widest = self.viewport_rows.unwrap_or(0).max(rows);
        let same_rows = self.viewport_rows == Some(widest) || (self.viewport_rows.is_none() && widest == 0);
        if same_windows && same_rows && !opened_any {
            return;
        }
        state.windows = next;
        self.viewport_rows = (widest > 0).then_some(widest);
        self.pending.insert(body.to_owned());
        self.armed_at.get_or_insert(now_ms);
    }

    /// 🪟️ The bodies that should refresh NOW: every pending one not already in flight, once an open toggle asked or the debounce
    /// elapsed (`flush`). Each returned body is in flight until [`Self::settled`].
    pub fn due(&mut self, now_ms: u64) -> Vec<String> {
        let elapsed = self.armed_at.is_some_and(|armed| now_ms.saturating_sub(armed) >= TREE_WINDOW_REPORT_DEBOUNCE_MS);
        if !(self.open_now || elapsed) {
            return Vec::new();
        }
        self.open_now = false;
        self.armed_at = None;
        let due: Vec<String> = self.pending.iter().filter(|body| !self.in_flight.contains(*body)).cloned().collect();
        for body in &due {
            self.pending.remove(body);
            self.in_flight.insert(body.clone());
        }
        due
    }

    /// 🪟️ A body's refresh crossed (`settled`): a body that moved meanwhile is due again at once.
    pub fn settled(&mut self, body: &str) {
        self.in_flight.remove(body);
        if self.pending.contains(body) {
            self.open_now = true;
        }
    }

    /// 🪟️ Every body whose refresh is still crossing.
    pub fn in_flight(&self) -> Vec<String> {
        self.in_flight.iter().cloned().collect()
    }

    /// 🪟️ The window one container of `body` asks for — what a shell-owned windowed panel materialises. `None` for a container
    /// the host never measured (the builder's first-paint allowance speaks for it).
    pub fn window_of(&self, body: &str, node_key: &str) -> Option<(u32, u32)> {
        let state = self.bodies.get(body)?;
        match (state.open.get(node_key), state.windows.get(node_key)) {
            (Some(false), _) => Some((0, 0)),
            (_, Some(window)) => Some(*window),
            _ => None,
        }
    }

    /// 🪟️ The measured viewport rows, or the first-paint default when nothing was measured yet.
    pub fn viewport_rows_or_default(&self) -> u32 {
        self.viewport_rows.unwrap_or(TREE_WINDOW_DEFAULT_ROWS)
    }

    /// 🪟️ What crosses to every guest (`viewStateFields`): every known container of every body, bodies and keys in sorted order;
    /// a never-measured open container asks one viewport, a measured one exactly its window. A path that is no view-context
    /// identifier (empty, over 256 code points, a control character) never crosses — it would take the whole crossing down.
    pub fn view_state_fields(&self) -> (Vec<semio_framework::TreeWindowRequest>, Option<u32>) {
        let mut flattened = Vec::new();
        for (body, state) in &self.bodies {
            let keys: BTreeSet<&String> = state.open.keys().chain(state.windows.keys()).collect();
            for key in keys {
                if !tree_window_sendable_identifier(key) || !tree_window_sendable_identifier(body) {
                    continue;
                }
                let open = state.open.get(key).copied();
                let window = state.windows.get(key).copied();
                let rows = window.map_or_else(|| if open == Some(false) || state.measured.contains(key) { 0 } else { self.viewport_rows_or_default() }, |(_, rows)| rows);
                flattened.push(semio_framework::TreeWindowRequest { body_key: body.clone(), node_key: key.clone(), open, offset: window.map_or(0, |(offset, _)| offset), rows });
            }
        }
        (flattened, self.viewport_rows)
    }
}

/// 🪟️ The ONE place host-owned tree expansion and scroll windows are held and turned into refreshes (`createTreeWindowSchedulerV1`):
/// an open toggle is due at once, window reports coalesce on a [`TREE_WINDOW_REPORT_DEBOUNCE_MS`] trailing edge, at most ONE
/// refresh per body is in flight, and a body whose state moved while its refresh crossed is re-sent with the LATEST state the
/// moment it settles. Time is injected (milliseconds), so every rule is a pure law.
#[derive(Clone, Debug, Default)]
pub struct TreeWindowScheduler {
    pub bodies: BTreeMap<String, TreeWindowBodyState>,
    pub pending: BTreeSet<String>,
    pub in_flight: BTreeSet<String>,
    pub viewport_rows: Option<u32>,
    armed_at: Option<u64>,
    open_now: bool,
}

/// 🪟️ Rows a container the host has just opened asks for before any viewport was measured (`TREE_WINDOW_DEFAULT_ROWS`).
pub const TREE_WINDOW_DEFAULT_ROWS: u32 = 48;

/// 🪟️ Trailing-edge coalescing of window reports (`TREE_WINDOW_REPORT_DEBOUNCE_MS`, `🛠️ShellHelpers/🟦️.tsx`).
pub const TREE_WINDOW_REPORT_DEBOUNCE_MS: u64 = 40;
