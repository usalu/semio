//! ⚓️ `shell-dock` staging of `dock-model-and-measures`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
//#region 🧭️PanelAnchorModel

/// 🧭️ Reserves an anchor's tab bar on the logical block end, then applies the retained-body inset.
pub fn flowed_anchor_content_rect(block: ui_contract::FlowBlock, panel: Rect, bar_height: f32, inset: f32) -> Rect {
    let content_height = (panel.h - bar_height).max(0.0);
    let content_y = if block.is_reversed() { panel.y } else { panel.y + bar_height };
    Rect::new(panel.x, content_y, panel.w, content_height).inset(inset)
}

/// 🧭️ The whole dock: one tab tree per anchor, slotted by `PanelAnchor::index` — React's `PanelDock`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShellDock {
    pub anchors: [Vec<DockTabNode>; 8],
}

impl ShellDock {
    pub fn tabs(&self, anchor: PanelAnchor) -> &[DockTabNode] {
        &self.anchors[anchor.index()]
    }

    pub fn tabs_mut(&mut self, anchor: PanelAnchor) -> &mut Vec<DockTabNode> {
        &mut self.anchors[anchor.index()]
    }

    pub fn is_empty(&self) -> bool {
        self.anchors.iter().all(|tabs| tabs.is_empty())
    }

    /// 🧭️ The anchor and root-to-node path a tab id lives at — React's `findPanelTabInDock` plus `findPanelTabPath`.
    pub fn locate(&self, tab_id: &str) -> Option<(PanelAnchor, Vec<String>)> {
        for anchor in PanelAnchor::ALL {
            if let Some(path) = dock_tab_path(self.tabs(anchor), tab_id) {
                return Some((anchor, path));
            }
        }
        None
    }

    pub fn node_at(&self, anchor: PanelAnchor, path: &[String]) -> Option<&DockTabNode> {
        let mut nodes = self.tabs(anchor);
        let mut found: Option<&DockTabNode> = None;
        for id in path {
            let node = nodes.iter().find(|node| &node.id == id)?;
            nodes = &node.children;
            found = Some(node);
        }
        found
    }

    /// 🧭️ The path this anchor opens at when nothing is remembered: the first root tab, drilled to its first leaf.
    pub fn default_path(&self, anchor: PanelAnchor) -> Vec<String> {
        let mut path = Vec::new();
        let mut nodes = self.tabs(anchor);
        while let Some(node) = nodes.first() {
            path.push(node.id.clone());
            nodes = &node.children;
        }
        path
    }

    /// 🧭️ Trims a remembered path back onto the tabs that still exist — React's `reconcileActivePath`.
    pub fn reconcile_path(&self, anchor: PanelAnchor, path: &[String]) -> Vec<String> {
        let mut kept = Vec::new();
        let mut nodes = self.tabs(anchor);
        for id in path {
            let Some(node) = nodes.iter().find(|node| &node.id == id) else { break };
            kept.push(node.id.clone());
            nodes = &node.children;
        }
        if kept.is_empty() {
            return self.default_path(anchor);
        }
        while let Some(node) = nodes.first() {
            kept.push(node.id.clone());
            nodes = &node.children;
        }
        kept
    }
}

/// 🧭️ One anchor's chrome state — React's `PanelState` (`🐚️Shell/🟦️.tsx`'s `SET_PANEL_VISIBLE`/
/// `SET_PANEL_SIZE`/`SET_PANEL_PATH` reducer trio) plus the folded/collapsed branch set the tab bar
/// paints from.
#[derive(Clone, Debug, PartialEq)]
pub struct PanelAnchorState {
    pub visible: bool,
    pub size: f32,
    pub path: Vec<String>,
}

impl Default for PanelAnchorState {
    fn default() -> Self {
        Self { visible: false, size: DEFAULT_PANEL_WIDTH_PX, path: Vec::new() }
    }
}

impl PanelAnchorState {
    /// 🧭️ The leaf this anchor currently paints — the last path segment, which is always a leaf id
    /// because every path write runs through `ShellDock::reconcile_path`.
    pub fn active_tab(&self) -> Option<&str> {
        self.path.last().map(String::as_str)
    }
}

/// 🧭️ A single anchor's current visible/size/active-tab projection — the read side of the anchor model.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PanelAnchorSnapshot {
    pub visible: bool,
    pub size: f32,
    pub active_tab: Option<String>,
}

/// ↔ Shared starting width for every panel anchor, one compact step wider than the former 280px Document panel.
pub const DEFAULT_PANEL_WIDTH_PX: f32 = 300.0;

/// 🗄️ **Dedup note**: panel layout used to be its own `semio.panelLayout.v1` preference carrying a
/// tab-separated left/right pair, which no React shell could ever read. Both dock documents now live
/// in the one `semio.os.config` document React's `OsShellConfig` owns (`🔨️modules/🖥️platform/🟦️.ts`),
/// under `dockLayouts` (the arrangement) and `dockUi` (the chrome) — one authority, one storage key,
/// and a dock rearranged in one renderer opens rearranged in the other.
pub fn os_shell_config_document() -> Value {
    raw_prefs_get(OS_SHELL_CONFIG_STORAGE_KEY)
        .filter(|raw| raw.len() <= OS_SHELL_CONFIG_MAX_BYTES)
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok())
        .filter(|config| config.get("version").and_then(Value::as_u64) == Some(1))
        .unwrap_or_else(empty_os_shell_config)
}

fn write_os_shell_config_document(config: &Value) {
    if let Ok(raw) = serde_json::to_string(config) {
        if raw.len() <= OS_SHELL_CONFIG_MAX_BYTES {
            raw_prefs_set(OS_SHELL_CONFIG_STORAGE_KEY, &raw);
        }
    }
}

/// 🗄️ Writes (or, with `None`, removes) one dock layer in an `semio.os.config` document — React's
/// `DockLayoutStore::save`/`saveOs`, pure over the document for the same reason as its reader.
pub fn write_os_shell_config_layer_in<T: Serialize>(config: &mut Value, section: &str, app_id: Option<&str>, value: Option<&T>) {
    if !config.get(section).is_some_and(Value::is_object) {
        config[section] = serde_json::json!({ "apps": {} });
    }
    if !config[section].get("apps").is_some_and(Value::is_object) {
        config[section]["apps"] = serde_json::json!({});
    }
    let encoded = value.and_then(|value| serde_json::to_value(value).ok());
    match (app_id, encoded) {
        (Some(app_id), Some(encoded)) => config[section]["apps"][app_id] = encoded,
        (Some(app_id), None) => {
            if let Some(apps) = config[section]["apps"].as_object_mut() {
                apps.remove(app_id);
            }
        }
        (None, Some(encoded)) => config[section]["os"] = encoded,
        (None, None) => {
            if let Some(section) = config[section].as_object_mut() {
                section.remove("os");
            }
        }
    }
}

fn write_os_shell_config_layer<T: Serialize>(section: &str, app_id: Option<&str>, value: Option<&T>) {
    let mut config = os_shell_config_document();
    write_os_shell_config_layer_in(&mut config, section, app_id, value);
    write_os_shell_config_document(&config);
}

/// 🗄️ Persists (or, with `None`, removes) the dock arrangement at `app_id`'s layer — React's `DockLayoutStore::save`.
pub fn save_dock_skeleton_to_store(app_id: Option<&str>, skeleton: Option<&DockSkeleton>) {
    write_os_shell_config_layer("dockLayouts", app_id, skeleton);
}

/// 🖥️ The app a saved layout is filed under — React's Display host `focusedApp?.id ?? "framework-os"`
/// (`🏛️ShellHost/🟦️.tsx`, `displayLayoutAppId`).
pub const NAMED_LAYOUT_STORE_FALLBACK_APP_ID: &str = "framework-os";

fn named_layout_store_app_id(app_id: Option<&str>) -> String {
    app_id.filter(|id| !id.is_empty()).unwrap_or(NAMED_LAYOUT_STORE_FALLBACK_APP_ID).to_string()
}

/// 🖥️ This app's USER layouts, projected from the replayed `os.config.ui-preferences` preferences — React's
/// `displayUserLayouts`: `namedLayouts[appId]` sorted by layout id, each `origin: "user"`. A saved arrangement this
/// renderer cannot type is left out rather than failing the roster.
pub fn user_named_layouts(preferences: &UiPreferences, app_id: Option<&str>) -> Vec<ui_wgpu::wgpu::NamedLayout> {
    let mut layouts: Vec<ui_wgpu::wgpu::NamedLayout> = preferences
        .named_layouts
        .get(&named_layout_store_app_id(app_id))
        .map(|saved| {
            saved
                .iter()
                .filter_map(|(id, entry)| Some(ui_wgpu::wgpu::NamedLayout { id: id.clone(), label: entry.label.clone(), icon_id: None, layout: ui_wgpu::wgpu::WindowLayout::from_value(entry.layout.clone()).ok()?, origin: "user".to_string(), group_path: None }))
                .collect()
        })
        .unwrap_or_default();
    layouts.sort_by(|left, right| left.id.cmp(&right.id));
    layouts
}

/// 🖥️ The `setNamedLayout` events that turn `preferences`' saved layouts of this app into `layouts` — one per
/// added, changed or removed user layout, none for an unchanged roster and never one for a builtin (React's
/// `onSaveUserLayout`/`onDeleteUserLayout`, one event per gesture).
pub fn named_layout_events(preferences: &UiPreferences, app_id: Option<&str>, layouts: &[ui_wgpu::wgpu::NamedLayout]) -> Vec<UiPreferencesConfigMutation> {
    let key = named_layout_store_app_id(app_id);
    let before = preferences.named_layouts.get(&key).cloned().unwrap_or_default();
    let after: HashMap<String, UserNamedLayout> = layouts
        .iter()
        .filter(|layout| layout.origin == "user")
        .filter_map(|layout| Some((layout.id.clone(), UserNamedLayout { label: layout.label.clone(), layout: layout.layout.to_value() })))
        .collect();
    let ids: std::collections::BTreeSet<&String> = before.keys().chain(after.keys()).collect();
    ids.into_iter().filter(|id| before.get(*id) != after.get(*id)).map(|id| set_named_layout(key.clone(), id.clone(), after.get(id).cloned())).collect()
}

/// 🖥️ The persisted user layouts for `app_id`, off the replayed preference log.
pub fn load_named_layouts_from_store(app_id: Option<&str>) -> Vec<ui_wgpu::wgpu::NamedLayout> {
    user_named_layouts(&read_ui_preferences(), app_id)
}

/// 🖥️ Appends the events that make `layouts` this app's saved roster to the preference log.
pub fn save_named_layouts_to_store(app_id: Option<&str>, layouts: &[ui_wgpu::wgpu::NamedLayout]) {
    commit_ui_preferences_write(|log| {
        let events = named_layout_events(&replay_ui_preferences(log), app_id, layouts);
        log.events.extend(events);
    });
}

pub fn save_dock_ui_to_store(app_id: Option<&str>, state: Option<&DockUiState>) {
    write_os_shell_config_layer("dockUi", app_id, state);
}

//#endregion 🧭️PanelAnchorModel
//#endregion ShellLifecycle

//#region ShellLifecycle
//#region 🧭️PanelAnchorModel
pub fn dock_tab_path(nodes: &[DockTabNode], tab_id: &str) -> Option<Vec<String>> {
    for node in nodes {
        if node.id == tab_id {
            return Some(vec![node.id.clone()]);
        }
        if let Some(mut rest) = dock_tab_path(&node.children, tab_id) {
            rest.insert(0, node.id.clone());
            return Some(rest);
        }
    }
    None
}
//#endregion 🧭️PanelAnchorModel
//#endregion ShellLifecycle
