//! ⚓️ `shell-dock` staging of `keyboard-and-palette`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
/// 🎯️ What [`ShellDockState::focus_window_of_kind`] did: raised a window that was open, or opened the window first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WindowKindFocus {
    Raised { window_id: String },
    Opened { window_id: String },
}

/// 📱️ The device class a window focus resolves against: the layout family the shell presents (a narrow viewport is mobile, otherwise the user's desktop or tablet preference).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FocusDevice {
    Desktop,
    Mobile,
    Tablet,
}

impl FocusDevice {
    /// 🆔️ The device id a named layout is matched by, and the suffix of the vocabulary key `window.focus.device.<id>`.
    pub fn id(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Mobile => "mobile",
            Self::Tablet => "tablet",
        }
    }

    /// 🌐️ The chrome vocabulary key naming the device in the interface language.
    pub fn vocab_key(self) -> &'static str {
        match self {
            Self::Desktop => "window.focus.device.desktop",
            Self::Mobile => "window.focus.device.mobile",
            Self::Tablet => "window.focus.device.tablet",
        }
    }

    /// 🔎️ The device of an id (`desktop`, `mobile`, `tablet`).
    pub fn parse(id: &str) -> Option<Self> {
        [Self::Desktop, Self::Mobile, Self::Tablet].into_iter().find(|device| device.id() == id)
    }
}

/// 🪟️ The window kinds the layout `device` presents holds: the app's named layout whose id is the device id or ends in `-<device id>` (`norm-visual-mobile`); `None` when the app declares no layout for the device, which leaves every declared kind available.
pub fn device_layout_window_kinds(layouts: &[semio_ui_declaration::layout::NamedLayout], device: FocusDevice) -> Option<Vec<String>> {
    let suffix = format!("-{}", device.id());
    layouts.iter().find(|layout| layout.id == device.id() || layout.id.ends_with(&suffix)).map(|layout| semio_ui_declaration::layout::collect_window_kind_ids_from_layout(&layout.layout))
}

/// 🎯️ The window kind a focus lands on for a device: `requested` when the device layout holds it (or declares no layout), else the `fallback` stand-in when the layout holds that, else `None` (the focus is refused as unavailable).
pub fn resolve_device_window_kind(available: Option<&[String]>, requested: &str, fallback: Option<&str>) -> Option<String> {
    let Some(available) = available else {
        return Some(requested.to_string());
    };
    let holds = |kind: &str| available.iter().any(|candidate| candidate == kind);
    if holds(requested) {
        Some(requested.to_string())
    } else {
        fallback.filter(|kind| holds(kind)).map(str::to_string)
    }
}

impl ShellDockState {
    /// 📱️ The window kind a focus of `requested` lands on on `device` (see [`resolve_device_window_kind`]); a kind the app never declared passes through so [`Self::focus_window_of_kind`] refuses it with its own reason.
    pub fn resolve_window_kind_for_device(&self, docs: &DocumentState, requested: &str, fallback: Option<&str>, device: FocusDevice) -> Option<String> {
        let Some(session) = docs.session.as_ref() else {
            return Some(requested.to_string());
        };
        if !session.app.window_kinds.iter().any(|kind| kind.id == requested) {
            return Some(requested.to_string());
        }
        resolve_device_window_kind(device_layout_window_kinds(&session.app.named_layouts, device).as_deref(), requested, fallback)
    }

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
    pub fn dock_stack_path_for(&self, window_id: &str) -> Option<semio_renderer_wgpu_dock::DockPath> {
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
        if !self.dock.split_stack_with_instance(&path, &instance_id, &window_kind_id, semio_renderer_wgpu_dock::DockSide::Right) {
            return false;
        }
        windows.active_window_id = Some(instance_id.clone());
        self.persist_dock_layout();
        windows.commit_window_topology_publication(&instance_id, &body_key, None, settle);
        true
    }

    /// 🎯️ Focuses the window of kind `window_kind_id`: the focused instance of that kind if it is mounted, else the first one, else it opens one as a split at the right of the mode root (the first instance carries the kind id as its id). The window becomes the active one of its stack and of the dock, the keyboard ring starts at it (no chrome stop keeps the focus) and the layout is remembered when a window was opened. Refuses with the reason when no app is mounted, the app never declared the kind or the topology queue is full; the caller announces the outcome.
    ///
    /// 🔗️ The ONE verb behind the `focusWindow` action ([`semio_framework::FOCUS_WINDOW_ACTION_ID`]) a chord, a menu row and the palette all raise.
    pub fn focus_window_of_kind(&mut self, docs: &DocumentState, window_kind_id: &str, windows: &mut WindowState, settle: &mut SettleState) -> Result<WindowKindFocus, String> {
        let Some(session) = docs.session.as_ref() else {
            return Err("no mounted app owns a window of that kind".to_string());
        };
        let Some(body_key) = session.app.window_kinds.iter().find(|kind| kind.id == window_kind_id).map(|kind| kind.body_key.clone()) else {
            return Err(format!("window kind {window_kind_id} is not declared by the app"));
        };
        let mounted: Vec<String> = self.dock.window_instances().into_iter().filter(|(_, kind_id)| kind_id == window_kind_id).map(|(window_id, _)| window_id).collect();
        let existing = windows.active_window_id.clone().filter(|active| mounted.contains(active)).or_else(|| mounted.first().cloned());
        let (window_id, opened) = match existing {
            Some(window_id) => (window_id, false),
            None => {
                if !windows.window_topology_publication_has_credit() {
                    return Err("the window topology queue is full".to_string());
                }
                if !self.dock.split_root_with_window(window_kind_id, semio_renderer_wgpu_dock::DockSide::Right) {
                    return Err(format!("window kind {window_kind_id} could not be opened"));
                }
                (window_kind_id.to_string(), true)
            }
        };
        if let Some(path) = self.dock_stack_path_for(&window_id) {
            self.dock.set_stack_active(&path, &window_id);
        }
        self.dock.sync_active_window(&window_id);
        windows.active_window_id = Some(window_id.clone());
        if opened {
            self.persist_dock_layout();
            windows.commit_window_topology_publication(&window_id, &body_key, None, settle);
            return Ok(WindowKindFocus::Opened { window_id });
        }
        settle.owe_refresh(UiDirtyScope::Full);
        settle.owe_settle();
        Ok(WindowKindFocus::Raised { window_id })
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
pub fn dock_window_order(node: &semio_renderer_wgpu_dock::DockNode, path: &mut Vec<usize>, out: &mut Vec<(Vec<usize>, String)>) {
    match node {
        semio_renderer_wgpu_dock::DockNode::Stack { windows, .. } => {
            for tab in windows {
                out.push((path.clone(), tab.window_id.clone()));
            }
        }
        semio_renderer_wgpu_dock::DockNode::Row(children) | semio_renderer_wgpu_dock::DockNode::Column(children) => {
            for (index, (child, _)) in children.iter().enumerate() {
                path.push(index);
                dock_window_order(child, path, out);
                path.pop();
            }
        }
    }
}
