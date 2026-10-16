//! ⚓️ `shell-dock` staging of `keyboard-and-palette`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
impl ShellDockState {
    /// 🪟️ The body key one live window instance renders — the address a scoped refresh names a window
    /// by, resolved through the dock plan and the app's own window kinds.
    pub fn window_body_key(&self, window_id: &str, docs: &DocumentState) -> Option<String> {
        let session = docs.session.as_ref()?;
        let kind_id = self.dock.window_instances().into_iter().find(|(id, _)| id == window_id).map(|(_, kind_id)| kind_id)?;
        session.app.window_kinds.iter().find(|kind| kind.id == kind_id).map(|kind| kind.body_key.clone())
    }

    /// 📑️ One tab-bar press: a branch folds/unfolds in place, a leaf opens its anchor at that leaf —
    /// routed through the host's own `setActivePanelTab` action first whenever this session claims that
    /// leaf, so the host panel document stays the single authority on which leaf is active.
    /// 📑️ Which anchor owns `tab_id` — the wgpu twin of React reading a tab's own `group`
    /// (`panelAnchorForGroup`), and the reason a panel-tab control id no longer has to spell its
    /// anchor out. `None` for anything that is not a dock tab, which is what makes it safe as the
    /// guard of `handle_shell_hit`'s panel-tab arm.
    pub fn panel_tab_anchor(&self, tab_id: &str) -> Option<PanelAnchor> {
        PanelAnchor::ALL.into_iter().find(|anchor| self.dock_tabs.tabs(*anchor).iter().any(|node| dock_tab_node_carries(node, tab_id)))
    }

    /// ⌨️ The native shell's keyboard ring — React's DOM order has no twin without a DOM, so it is the chrome's
    /// accessibility publication (every focusable, enabled, visible control, in reading order) followed by the
    /// dock's windows. The keyboard, a platform assistive technology and the browser mirror therefore reach
    /// exactly the same controls (ticket 26/09/23 slice WG10).
    pub fn keyboard_ring(&self, chrome: &ChromeState) -> Vec<ShellKeyboardStop> {
        let mut ring: Vec<ShellKeyboardStop> = chrome.presented_chrome_accessibility.iter().filter(|node| node.focusable && !node.disabled && !node.hidden).map(|node| ShellKeyboardStop::Chrome(node.key.clone())).collect();
        let mut order = Vec::new();
        dock_window_order(&self.dock.root, &mut Vec::new(), &mut order);
        ring.extend(order.into_iter().map(|(path, window_id)| ShellKeyboardStop::Window(path, window_id)));
        ring
    }

    //#region ⌨️ShellShortcutDispatch
    /// 🪟️ The dock stack path holding `window_id`, read off the same order walk Tab cycling uses.
    pub fn dock_stack_path_for(&self, window_id: &str) -> Option<semio_framework_os_renderer_wgpu_dock::DockPath> {
        let mut order = Vec::new();
        dock_window_order(&self.dock.root, &mut Vec::new(), &mut order);
        order.into_iter().find(|(_, id)| id == window_id).map(|(path, _)| path)
    }

    /// 🧭️ Toggles one named panel anchor — the wgpu twin of React's per-anchor `SET_PANEL_VISIBLE`
    /// toggle that `ui.shell.panelAnchor.*` binds. An anchor carrying no tab is a witnessed no-op
    /// (`false`) rather than an empty box appearing over the canvas.
    pub fn toggle_panel_anchor(&mut self, anchor: PanelAnchor) -> bool {
        if self.dock_tabs.tabs(anchor).is_empty() {
            return false;
        }
        self.toggle_anchor(anchor);
        true
    }

    /// 🧭️ Opens this anchor at its remembered (or default) path, or folds it shut when it is already open.
    pub fn toggle_anchor(&mut self, anchor: PanelAnchor) {
        if self.anchor_open(anchor) {
            self.anchor_state_mut(anchor).visible = false;
            return;
        }
        let path = self.dock_tabs.reconcile_path(anchor, &self.anchor_state(anchor).path.clone());
        let state = self.anchor_state_mut(anchor);
        state.path = path;
        state.visible = true;
    }

    /// 🧭️ `⌘️B`/`⌘️⇧️B` — folds a whole column shut when any of its anchors is open, else reopens its
    /// top corner, the column-level chord the two former `left_panel_open`/`right_panel_open` flags carried.
    pub fn toggle_column(&mut self, left: bool) {
        let anchors = column_anchors(left);
        if anchors.into_iter().any(|anchor| self.anchor_open(anchor)) {
            for anchor in anchors {
                self.anchor_state_mut(anchor).visible = false;
            }
            return;
        }
        self.toggle_anchor(anchors[0]);
    }

    /// 🪟️ `mod+shift+enter` — the same maximize toggle the window's own focus control performs. React
    /// gates it on `canMaximize` (more than one window in the mode); a single-window mode has nothing to
    /// maximize away from, so this refuses there too.
    pub fn focus_active_window(&mut self, windows: &WindowState) -> bool {
        let Some(window_id) = windows.active_window_id.clone() else {
            return false;
        };
        let mut order = Vec::new();
        dock_window_order(&self.dock.root, &mut Vec::new(), &mut order);
        if order.len() <= 1 {
            return false;
        }
        let Some(path) = order.into_iter().find(|(_, id)| *id == window_id).map(|(path, _)| path) else {
            return false;
        };
        self.dock.set_stack_active(&path, &window_id);
        self.dock.toggle_maximize(&path);
        true
    }

    /// 🪟️ `mod+shift+n` — "Open in New Window" is a second INSTANCE of the active window's kind split to
    /// the right of its stack, never an OS window: the exact shape of React's `onWindowOpenInNewWindow`
    /// (`🏛️ShellHost/🟦️.tsx`), down to the `{kindId}-{n}` instance id and the right-hand split.
    pub fn open_active_window_in_new_window(&mut self, settle: &mut SettleState, docs: &DocumentState, windows: &mut WindowState) -> bool {
        let Some(window_id) = windows.active_window_id.clone() else {
            return false;
        };
        let Some(session) = docs.session.as_ref() else {
            return false;
        };
        let Some(window_kind_id) = self.live_window_kind_id(session, &window_id).map(str::to_string) else {
            return false;
        };
        let Some(body_key) = session.app.window_kinds.iter().find(|kind| kind.id == window_kind_id).map(|kind| kind.body_key.clone()) else {
            return false;
        };
        if !windows.window_topology_publication_has_credit() {
            return false;
        }
        let taken: Vec<String> = self.dock.window_instances().into_iter().map(|(id, _)| id).collect();
        let instance_id = (2..).map(|index| format!("{window_kind_id}-{index}")).find(|candidate| !taken.contains(candidate)).expect("an unbounded counter always clears a finite dock roster");
        let Some(path) = self.dock_stack_path_for(&window_id) else {
            return false;
        };
        if !self.dock.split_stack_with_instance(&path, &instance_id, &window_kind_id, semio_framework_os_renderer_wgpu_dock::DockSide::Right) {
            return false;
        }
        windows.active_window_id = Some(instance_id.clone());
        self.persist_dock_layout();
        windows.commit_window_topology_publication(&instance_id, &body_key, None, settle);
        true
    }
    //#endregion ⌨️ShellShortcutDispatch
}

//#endregion ShellInput

/// 🪟️ Read-only walk of the dock tree collecting `(stack path, window id)` in tab/visual order
/// (depth-first Row/Column child order, each Stack's own tab order). Duplicated locally rather than
/// reusing `dock`'s own module-private `find_stack_path`-style traversal, because that helper is
/// private to the `dock` module and this region (`shell::ShellInput`) must not edit `dock` to expose
/// a public equivalent (region-claims.json `must_not_touch`). `DockNode`'s variants/fields are
/// public, so this is a plain read of already-public state, not a layering violation.
pub fn dock_window_order(node: &semio_framework_os_renderer_wgpu_dock::DockNode, path: &mut Vec<usize>, out: &mut Vec<(Vec<usize>, String)>) {
    match node {
        semio_framework_os_renderer_wgpu_dock::DockNode::Stack { windows, .. } => {
            for tab in windows {
                out.push((path.clone(), tab.window_id.clone()));
            }
        }
        semio_framework_os_renderer_wgpu_dock::DockNode::Row(children) | semio_framework_os_renderer_wgpu_dock::DockNode::Column(children) => {
            for (index, (child, _)) in children.iter().enumerate() {
                path.push(index);
                dock_window_order(child, path, out);
                path.pop();
            }
        }
    }
}
