//! 🪟️ The wgpu Shell's tree window observer and scheduler — the host half of ticket 26/09/16 ARTIFACT-TREE-VIRTUALISED-STREAMING
//! packet P5 on this renderer. After every retained body finishes a paint the Shell measures its windowed containers
//! (`Ui::tree_window_measures`), asks the ONE request rule (`ui_wgpu::wgpu::tree_window::served_requests`) which rows the viewport
//! needs, and hands the answer to [`TreeWindowScheduler`] — the Rust twin of React's `createTreeWindowSchedulerV1`
//! (`🛠️ShellHelpers/🟦️.tsx`): windows coalesce on a trailing debounce, at most one refresh per body is in flight, and a report
//! that changes nothing schedules nothing. The scheduled state crosses to guests as `ViewModel.tree_windows`/`tree_viewport_rows`
//! (the fields React stamps), and a shell-owned windowed panel reads it back through [`TreeWindowScheduler::window_of`].
use super::*;
use std::collections::{BTreeMap, BTreeSet};
use ui_wgpu::wgpu::tree_window::{served_requests, TreeWindowRequest, TreeWindowServedMemory};

/// 🪟️ Trailing-edge coalescing of window reports (`TREE_WINDOW_REPORT_DEBOUNCE_MS`, `🛠️ShellHelpers/🟦️.tsx`).
pub(crate) const TREE_WINDOW_REPORT_DEBOUNCE_MS: u64 = 40;

/// 🪟️ Rows a container the host has just opened asks for before any viewport was measured (`TREE_WINDOW_DEFAULT_ROWS`).
pub(crate) const TREE_WINDOW_DEFAULT_ROWS: u32 = 48;

/// 🪟️ The ONE place host-owned tree expansion and scroll windows are held and turned into refreshes (`createTreeWindowSchedulerV1`):
/// an open toggle is due at once, window reports coalesce on a [`TREE_WINDOW_REPORT_DEBOUNCE_MS`] trailing edge, at most ONE
/// refresh per body is in flight, and a body whose state moved while its refresh crossed is re-sent with the LATEST state the
/// moment it settles. Time is injected (milliseconds), so every rule is a pure law.
#[derive(Clone, Debug, Default)]
pub(crate) struct TreeWindowScheduler {
    pub(super) bodies: BTreeMap<String, TreeWindowBodyState>,
    pub(super) pending: BTreeSet<String>,
    pub(super) in_flight: BTreeSet<String>,
    pub(super) viewport_rows: Option<u32>,
    armed_at: Option<u64>,
    open_now: bool,
}

impl TreeWindowScheduler {
    /// 🪟️ Records the user opening or closing one container (`setOpen`) — due immediately, and opening forgets what the observer
    /// last measured for it so the first-paint allowance speaks for it again.
    pub(crate) fn set_open(&mut self, body: &str, node_key: &str, open: bool) {
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
    pub(crate) fn report(&mut self, body: &str, requests: &[TreeWindowRequest], rows: u32, now_ms: u64) {
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
    pub(crate) fn due(&mut self, now_ms: u64) -> Vec<String> {
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
    pub(crate) fn settled(&mut self, body: &str) {
        self.in_flight.remove(body);
        if self.pending.contains(body) {
            self.open_now = true;
        }
    }

    /// 🪟️ Every body whose refresh is still crossing.
    pub(crate) fn in_flight(&self) -> Vec<String> {
        self.in_flight.iter().cloned().collect()
    }

    /// 🪟️ The window one container of `body` asks for — what a shell-owned windowed panel materialises. `None` for a container
    /// the host never measured (the builder's first-paint allowance speaks for it).
    pub(crate) fn window_of(&self, body: &str, node_key: &str) -> Option<(u32, u32)> {
        let state = self.bodies.get(body)?;
        match (state.open.get(node_key), state.windows.get(node_key)) {
            (Some(false), _) => Some((0, 0)),
            (_, Some(window)) => Some(*window),
            _ => None,
        }
    }

    /// 🪟️ The measured viewport rows, or the first-paint default when nothing was measured yet.
    pub(crate) fn viewport_rows_or_default(&self) -> u32 {
        self.viewport_rows.unwrap_or(TREE_WINDOW_DEFAULT_ROWS)
    }

    /// 🪟️ What crosses to every guest (`viewStateFields`): every known container of every body, bodies and keys in sorted order;
    /// a never-measured open container asks one viewport, a measured one exactly its window. A path that is no view-context
    /// identifier (empty, over 256 code points, a control character) never crosses — it would take the whole crossing down.
    pub(crate) fn view_state_fields(&self) -> (Vec<semio_framework::TreeWindowRequest>, Option<u32>) {
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

/// 🚧️ Whether one path may cross in a view context (`treeWindowSendableIdentifierV1`): non-empty, at most 256 code points, no C0
/// control character and no DEL.
fn tree_window_sendable_identifier(identifier: &str) -> bool {
    !identifier.is_empty() && identifier.chars().count() <= 256 && !identifier.chars().any(|character| character.is_ascii_control())
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🪟️tree-windows/🦀️.rs"]
mod tests;
