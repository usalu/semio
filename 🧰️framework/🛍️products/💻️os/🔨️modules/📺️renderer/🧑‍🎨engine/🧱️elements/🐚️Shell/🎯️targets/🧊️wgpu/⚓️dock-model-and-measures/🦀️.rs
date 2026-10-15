//! ⚓️ Dock model, panel anchors and window measures.
//!
//! Panel anchors, the dock tree and skeleton, dock UI persistence helpers and the window-measures projection
//! with its `impl ShellState` publication half.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellLifecycle
//#region 🧭️PanelAnchorModel
/// 🧭️ The framework's generic 8-anchor panel positioning model — mirrors `PanelGroup::anchor()`
/// (`🔨️modules/🛂️manifest/🦀️.rs`) and React's `Anchor`/`ANCHORS`
/// (`🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`). Every anchor is an independent floating panel slot
/// carrying its own tab tree ({@link ShellDock}) and its own open/size/active-path chrome state
/// ({@link PanelAnchorState}) — the same `PanelDock`/`PanelState` pair React's `Layout` maps over.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PanelAnchor {
    TopLeft,
    TopMiddle,
    TopRight,
    RightMiddle,
    BottomRight,
    BottomMiddle,
    BottomLeft,
    LeftMiddle,
}

impl PanelAnchor {
    pub const ALL: [PanelAnchor; 8] = [PanelAnchor::TopLeft, PanelAnchor::TopMiddle, PanelAnchor::TopRight, PanelAnchor::RightMiddle, PanelAnchor::BottomRight, PanelAnchor::BottomMiddle, PanelAnchor::BottomLeft, PanelAnchor::LeftMiddle];

    pub fn as_str(&self) -> &'static str {
        match self {
            PanelAnchor::TopLeft => "top-left",
            PanelAnchor::TopMiddle => "top-middle",
            PanelAnchor::TopRight => "top-right",
            PanelAnchor::RightMiddle => "right-middle",
            PanelAnchor::BottomRight => "bottom-right",
            PanelAnchor::BottomMiddle => "bottom-middle",
            PanelAnchor::BottomLeft => "bottom-left",
            PanelAnchor::LeftMiddle => "left-middle",
        }
    }

    /// 🧭️ Parses one of the eight `ANCHORS` ids — the read side of every persisted dock document.
    pub fn from_str(value: &str) -> Option<PanelAnchor> {
        PanelAnchor::ALL.into_iter().find(|anchor| anchor.as_str() == value)
    }

    /// 🧭️ This anchor's slot in every `[_; 8]` the dock keys by anchor, in `ALL` order.
    pub fn index(&self) -> usize {
        match self {
            PanelAnchor::TopLeft => 0,
            PanelAnchor::TopMiddle => 1,
            PanelAnchor::TopRight => 2,
            PanelAnchor::RightMiddle => 3,
            PanelAnchor::BottomRight => 4,
            PanelAnchor::BottomMiddle => 5,
            PanelAnchor::BottomLeft => 6,
            PanelAnchor::LeftMiddle => 7,
        }
    }

    /// 🧭️ `"top"`/`"middle"`/`"bottom"` row of this anchor — the Rust twin of React's `anchorVertical`.
    pub fn vertical(&self) -> &'static str {
        match self {
            PanelAnchor::TopLeft | PanelAnchor::TopMiddle | PanelAnchor::TopRight => "top",
            PanelAnchor::BottomLeft | PanelAnchor::BottomMiddle | PanelAnchor::BottomRight => "bottom",
            PanelAnchor::LeftMiddle | PanelAnchor::RightMiddle => "middle",
        }
    }

    /// 🧭️ `"left"`/`"middle"`/`"right"` column of this anchor — the Rust twin of React's `anchorHorizontal`.
    pub fn horizontal(&self) -> &'static str {
        match self {
            PanelAnchor::TopLeft | PanelAnchor::BottomLeft | PanelAnchor::LeftMiddle => "left",
            PanelAnchor::TopRight | PanelAnchor::BottomRight | PanelAnchor::RightMiddle => "right",
            PanelAnchor::TopMiddle | PanelAnchor::BottomMiddle => "middle",
        }
    }

    /// 🧭️ Mirrors `PanelGroup::anchor()`'s corner mapping exactly; a group never maps to a middle anchor.
    /// 🧭️ This anchor as the shared `ui_contract::Anchor` vocabulary React's `flowFromAnchor` reads.
    /// `Left`/`Right` are the physical edges this target names its anchors by; `Start`/`End` are the
    /// logical ones the contract names them by, and in an LTR shell they coincide — which is exactly
    /// the mapping `🖼️Panel/🟦️.tsx`'s own `anchorHorizontal` makes.
    pub fn contract_anchor(&self) -> ui_contract::Anchor {
        match self {
            PanelAnchor::TopLeft => ui_contract::Anchor::TopStart,
            PanelAnchor::TopMiddle => ui_contract::Anchor::Top,
            PanelAnchor::TopRight => ui_contract::Anchor::TopEnd,
            PanelAnchor::RightMiddle => ui_contract::Anchor::End,
            PanelAnchor::BottomRight => ui_contract::Anchor::BottomEnd,
            PanelAnchor::BottomMiddle => ui_contract::Anchor::Bottom,
            PanelAnchor::BottomLeft => ui_contract::Anchor::BottomStart,
            PanelAnchor::LeftMiddle => ui_contract::Anchor::Start,
        }
    }

    /// 🧭️ The logical flow a panel docked here presents its content in — React's
    /// `flowFromAnchor(anchor)`, fed to the `FlowProvider` that wraps every `Panel`.
    pub fn flow(&self) -> ui_contract::UiFlow {
        ui_contract::UiFlow::for_anchor(self.contract_anchor())
    }

    pub fn from_group(group: PanelGroup) -> PanelAnchor {
        PanelAnchor::from_str(group.anchor()).unwrap_or(PanelAnchor::TopLeft)
    }
}

/// 🧭️ Reserves an anchor's tab bar on the logical block end, then applies the retained-body inset.
pub(crate) fn flowed_anchor_content_rect(block: ui_contract::FlowBlock, panel: Rect, bar_height: f32, inset: f32) -> Rect {
    let content_height = (panel.h - bar_height).max(0.0);
    let content_y = if block.is_reversed() { panel.y } else { panel.y + bar_height };
    Rect::new(panel.x, content_y, panel.w, content_height).inset(inset)
}

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

/// 🧭️ One node of an anchor's tab tree — a leaf when `children` is empty, otherwise a collapsible
/// branch, the Rust twin of React's `PanelTabNode` (`🧭️PanelTabBar/🟦️.tsx`). A leaf's `id` is the
/// panel surface id whose retained document the open panel paints.
#[derive(Clone, Debug, PartialEq)]
pub struct DockTabNode {
    pub id: String,
    pub label: String,
    pub icon_id: String,
    pub order: i32,
    pub children: Vec<DockTabNode>,
}

impl DockTabNode {
    pub fn leaf(id: impl Into<String>, label: impl Into<String>, icon_id: impl Into<String>, order: i32) -> Self {
        Self { id: id.into(), label: label.into(), icon_id: icon_id.into(), order, children: Vec::new() }
    }

    pub fn branch(id: impl Into<String>, label: impl Into<String>, icon_id: impl Into<String>, order: i32, children: Vec<DockTabNode>) -> Self {
        Self { id: id.into(), label: label.into(), icon_id: icon_id.into(), order, children }
    }

    pub fn is_branch(&self) -> bool {
        !self.children.is_empty()
    }

    pub(crate) fn contains(&self, id: &str) -> bool {
        self.id == id || self.children.iter().any(|child| child.contains(id))
    }
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

fn dock_tab_path(nodes: &[DockTabNode], tab_id: &str) -> Option<Vec<String>> {
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

fn remove_dock_tab(nodes: &mut Vec<DockTabNode>, tab_id: &str) -> Option<DockTabNode> {
    if let Some(index) = nodes.iter().position(|node| node.id == tab_id) {
        return Some(nodes.remove(index));
    }
    for node in nodes.iter_mut() {
        if let Some(removed) = remove_dock_tab(&mut node.children, tab_id) {
            return Some(removed);
        }
    }
    None
}

fn insert_dock_tab_at(nodes: &mut Vec<DockTabNode>, parent_path: &[String], node: DockTabNode, index: usize) -> bool {
    let Some((head, rest)) = parent_path.split_first() else {
        let index = index.min(nodes.len());
        nodes.insert(index, node);
        return true;
    };
    let Some(parent) = nodes.iter_mut().find(|candidate| &candidate.id == head) else { return false };
    insert_dock_tab_at(&mut parent.children, rest, node, index)
}

/// 🎯️ Where a dragged tab lands — React's `PanelTabDockTarget`: at `index` under `parent_path`'s
/// branch (an empty path being the anchor's own root row).
#[derive(Clone, Debug, PartialEq)]
pub struct DockTabMoveTarget {
    pub anchor: PanelAnchor,
    pub parent_path: Vec<String>,
    pub index: usize,
}

/// 🎯️ Pure move transform — removes `tab_id`'s subtree from wherever it lives and reinserts it at
/// `target`, the Rust twin of React's `moveTabInDock`. Dropping a subtree into itself is refused
/// (`false`, dock untouched); visibility is never touched here.
pub fn move_tab_in_dock(dock: &mut ShellDock, tab_id: &str, target: &DockTabMoveTarget) -> bool {
    let Some((from_anchor, _)) = dock.locate(tab_id) else { return false };
    let Some(node) = dock.node_at(from_anchor, &dock_tab_path(dock.tabs(from_anchor), tab_id).unwrap_or_default()) else { return false };
    if target.parent_path.iter().any(|id| node.contains(id)) {
        return false;
    }
    let Some(removed) = remove_dock_tab(dock.tabs_mut(from_anchor), tab_id) else { return false };
    if insert_dock_tab_at(dock.tabs_mut(target.anchor), &target.parent_path, removed.clone(), target.index) {
        return true;
    }
    dock.tabs_mut(from_anchor).push(removed);
    false
}

/// 🗄️ One persisted tab id plus, for a branch, its persisted child order, or, for a leaf, its tree-unit
/// order — byte-shape identical to React's `DockTabSkeleton` (`🔨️modules/🖥️platform/🟦️.ts`).
///
/// 🌳️ `trees` is React's `PanelTreeUnit` id list (`🧭️PanelTabBar/🟦️.tsx`'s `panelTabNodeToSkeleton`). A
/// wgpu leaf hosts exactly ONE retained document, so its unit list is always the single
/// `"<leafId>.tree"` id React's `singleTreeLeaf` mints — written so a skeleton this renderer saves
/// round-trips through React's `applyDockSkeleton` unit resolution instead of losing every leaf's
/// units, and read back only to be validated, since there is no second unit to reorder.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct DockTabSkeleton {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DockTabSkeleton>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trees: Option<Vec<String>>,
}

/// 📱️ Trims a remembered mobile-panel path onto the flattened tab list and drills a branch to its
/// first leaf — the mobile twin of [`ShellDock::reconcile_path`], over one flat list instead of eight.
pub fn mobile_panel_reconcile_path(tabs: &[DockTabNode], path: &[String]) -> Vec<String> {
    let mut kept = Vec::new();
    let mut nodes = tabs;
    for id in path {
        let Some(node) = nodes.iter().find(|node| &node.id == id) else { break };
        kept.push(node.id.clone());
        nodes = &node.children;
    }
    if kept.is_empty() {
        nodes = tabs;
    }
    while let Some(node) = nodes.first() {
        kept.push(node.id.clone());
        nodes = &node.children;
    }
    kept
}

/// 🌳️ The single tree-unit id a wgpu leaf carries — React's `singleTreeLeaf` id rule.
pub fn dock_leaf_tree_unit_id(leaf_id: &str) -> String {
    format!("{leaf_id}.tree")
}

/// 🗄️ The full persisted dock arrangement, one tab tree per anchor — React's `DockSkeleton`, stored
/// under the same `semio.os.config` document's `dockLayouts` so a layout survives a renderer switch.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DockSkeleton {
    pub version: u8,
    pub anchors: BTreeMap<String, Vec<DockTabSkeleton>>,
}

impl Default for DockSkeleton {
    fn default() -> Self {
        Self { version: DOCK_SKELETON_VERSION, anchors: BTreeMap::new() }
    }
}

pub(crate) const DOCK_SKELETON_VERSION: u8 = 3;

fn dock_tab_skeleton_of(node: &DockTabNode) -> DockTabSkeleton {
    DockTabSkeleton { id: node.id.clone(), children: node.is_branch().then(|| node.children.iter().map(dock_tab_skeleton_of).collect()), trees: (!node.is_branch()).then(|| vec![dock_leaf_tree_unit_id(&node.id)]) }
}

/// 🗄️ Reduces a full {@link ShellDock} to the id-only {@link DockSkeleton} persisted across sessions.
pub fn dock_skeleton_of(dock: &ShellDock) -> DockSkeleton {
    let mut anchors = BTreeMap::new();
    for anchor in PanelAnchor::ALL {
        anchors.insert(anchor.as_str().to_string(), dock.tabs(anchor).iter().map(dock_tab_skeleton_of).collect());
    }
    DockSkeleton { version: DOCK_SKELETON_VERSION, anchors }
}

/// 🗄️ Structural equality across all eight anchors, absent and empty being the same thing — decides
/// whether an arrangement equals its computed default and therefore needs no persistence.
pub fn dock_skeletons_equal(a: Option<&DockSkeleton>, b: Option<&DockSkeleton>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => PanelAnchor::ALL.into_iter().all(|anchor| {
            let empty: Vec<DockTabSkeleton> = Vec::new();
            a.anchors.get(anchor.as_str()).unwrap_or(&empty) == b.anchors.get(anchor.as_str()).unwrap_or(&empty)
        }),
        _ => false,
    }
}

fn index_dock_nodes(dock: &ShellDock, into: &mut HashMap<String, DockTabNode>) {
    fn visit(node: &DockTabNode, into: &mut HashMap<String, DockTabNode>) {
        into.insert(node.id.clone(), node.clone());
        for child in &node.children {
            visit(child, into);
        }
    }
    for anchor in PanelAnchor::ALL {
        for node in dock.tabs(anchor) {
            visit(node, into);
        }
    }
}

fn collect_skeleton_mentions(entries: &[DockTabSkeleton], nodes: &HashMap<String, DockTabNode>, mentioned: &mut std::collections::HashSet<String>) {
    for entry in entries {
        mentioned.insert(entry.id.clone());
        if let (Some(default_node), Some(children)) = (nodes.get(&entry.id), entry.children.as_ref()) {
            if default_node.is_branch() {
                collect_skeleton_mentions(children, nodes, mentioned);
            }
        }
    }
}

fn default_subtree_mentioned(node: &DockTabNode, mentioned: &std::collections::HashSet<String>) -> bool {
    mentioned.contains(&node.id) || node.children.iter().any(|child| default_subtree_mentioned(child, mentioned))
}

fn resolve_skeleton_node(entry: &DockTabSkeleton, nodes: &HashMap<String, DockTabNode>, mentioned: &std::collections::HashSet<String>, resolved: &mut std::collections::HashSet<String>) -> Option<DockTabNode> {
    let default_node = nodes.get(&entry.id)?;
    if !resolved.insert(entry.id.clone()) {
        return None;
    }
    if !default_node.is_branch() {
        return Some(DockTabNode { children: Vec::new(), ..default_node.clone() });
    }
    let Some(children) = entry.children.as_ref() else { return Some(default_node.clone()) };
    let mut merged: Vec<DockTabNode> = children.iter().filter_map(|child| resolve_skeleton_node(child, nodes, mentioned, resolved)).collect();
    for child in &default_node.children {
        if !default_subtree_mentioned(child, mentioned) {
            resolved.insert(child.id.clone());
            merged.push(child.clone());
        }
    }
    Some(DockTabNode { children: merged, ..default_node.clone() })
}

/// 🗄️ Rearranges (never reconstructs) `default_dock`'s nodes to match a persisted skeleton: ids the
/// default no longer declares are dropped, and any default tab the skeleton never mentions anywhere is
/// appended at its default location. The Rust twin of React's `applyDockSkeleton`, mention-collection
/// pass included, so an anchor processed first can never reclaim a tab moved to a later one.
pub fn apply_dock_skeleton(default_dock: &ShellDock, skeleton: Option<&DockSkeleton>) -> ShellDock {
    let Some(skeleton) = skeleton else { return default_dock.clone() };
    let mut nodes = HashMap::new();
    index_dock_nodes(default_dock, &mut nodes);
    let mut mentioned = std::collections::HashSet::new();
    let empty: Vec<DockTabSkeleton> = Vec::new();
    for anchor in PanelAnchor::ALL {
        collect_skeleton_mentions(skeleton.anchors.get(anchor.as_str()).unwrap_or(&empty), &nodes, &mut mentioned);
    }
    let mut resolved = std::collections::HashSet::new();
    let mut result = ShellDock::default();
    for anchor in PanelAnchor::ALL {
        let entries = skeleton.anchors.get(anchor.as_str()).unwrap_or(&empty);
        let mut tabs: Vec<DockTabNode> = entries.iter().filter_map(|entry| resolve_skeleton_node(entry, &nodes, &mentioned, &mut resolved)).collect();
        for tab in default_dock.tabs(anchor) {
            if !default_subtree_mentioned(tab, &mentioned) {
                resolved.insert(tab.id.clone());
                tabs.push(tab.clone());
            }
        }
        *result.tabs_mut(anchor) = tabs;
    }
    result
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

/// 🗄️ One anchor's persisted chrome state — only the fields that differ from the shell's computed
/// defaults are ever stored, byte-shape identical to React's `DockUiPanelState`.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct DockUiPanelState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<String>>,
}

/// 🗄️ The persisted per-anchor panel chrome — React's `DockUiState`, same `semio.os.config` document,
/// same `dockUi` layer split (a per-app entry wins over the shared `os` one).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DockUiState {
    pub version: u8,
    #[serde(default)]
    pub anchors: BTreeMap<String, DockUiPanelState>,
}

impl Default for DockUiState {
    fn default() -> Self {
        Self { version: DOCK_SKELETON_VERSION, anchors: BTreeMap::new() }
    }
}

/// ↔ Shared starting width for every panel anchor, one compact step wider than the former 280px Document panel.
pub(crate) const DEFAULT_PANEL_WIDTH_PX: f32 = 300.0;

/// 🗄️ **Dedup note**: panel layout used to be its own `semio.panelLayout.v1` preference carrying a
/// tab-separated left/right pair, which no React shell could ever read. Both dock documents now live
/// in the one `semio.os.config` document React's `OsShellConfig` owns (`🔨️modules/🖥️platform/🟦️.ts`),
/// under `dockLayouts` (the arrangement) and `dockUi` (the chrome) — one authority, one storage key,
/// and a dock rearranged in one renderer opens rearranged in the other.
fn os_shell_config_document() -> Value {
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

/// 🗄️ Reads one dock layer out of an `semio.os.config` DOCUMENT — the per-app entry when it parses,
/// else the shared `os` one, exactly React's `DockLayoutStore::getSnapshot` precedence. Pure over the
/// document so the layer rules are provable without the process-global store underneath.
pub(crate) fn read_os_shell_config_layer<T: serde::de::DeserializeOwned>(config: &Value, section: &str, app_id: Option<&str>) -> Option<T> {
    let layer = config.get(section)?;
    if let Some(app_id) = app_id {
        if let Some(entry) = layer.get("apps").and_then(|apps| apps.get(app_id)) {
            if let Ok(parsed) = serde_json::from_value::<T>(entry.clone()) {
                return Some(parsed);
            }
        }
    }
    serde_json::from_value::<T>(layer.get("os")?.clone()).ok()
}

/// 🗄️ Writes (or, with `None`, removes) one dock layer in an `semio.os.config` document — React's
/// `DockLayoutStore::save`/`saveOs`, pure over the document for the same reason as its reader.
pub(crate) fn write_os_shell_config_layer_in<T: Serialize>(config: &mut Value, section: &str, app_id: Option<&str>, value: Option<&T>) {
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

fn os_shell_config_layer<T: serde::de::DeserializeOwned>(section: &str, app_id: Option<&str>) -> Option<T> {
    read_os_shell_config_layer(&os_shell_config_document(), section, app_id)
}

fn write_os_shell_config_layer<T: Serialize>(section: &str, app_id: Option<&str>, value: Option<&T>) {
    let mut config = os_shell_config_document();
    write_os_shell_config_layer_in(&mut config, section, app_id, value);
    write_os_shell_config_document(&config);
}

/// 🗄️ Reads the persisted dock arrangement — the per-app layer when present, else the shared `os` one,
/// exactly the precedence React's `DockLayoutStore::getSnapshot` applies.
pub(crate) fn load_dock_skeleton_from_store(app_id: Option<&str>) -> Option<DockSkeleton> {
    os_shell_config_layer::<DockSkeleton>("dockLayouts", app_id).filter(|skeleton| skeleton.version == DOCK_SKELETON_VERSION)
}

/// 🗄️ Persists (or, with `None`, removes) the dock arrangement at `app_id`'s layer — React's `DockLayoutStore::save`.
pub(crate) fn save_dock_skeleton_to_store(app_id: Option<&str>, skeleton: Option<&DockSkeleton>) {
    write_os_shell_config_layer("dockLayouts", app_id, skeleton);
}

pub(crate) fn load_dock_ui_from_store(app_id: Option<&str>) -> Option<DockUiState> {
    os_shell_config_layer::<DockUiState>("dockUi", app_id).filter(|state| state.version == DOCK_SKELETON_VERSION)
}

/// 🖥️ The app a saved layout is filed under — React's Display host `focusedApp?.id ?? "framework-os"`
/// (`🏛️ShellHost/🟦️.tsx`, `displayLayoutAppId`).
pub(crate) const NAMED_LAYOUT_STORE_FALLBACK_APP_ID: &str = "framework-os";

fn named_layout_store_app_id(app_id: Option<&str>) -> String {
    app_id.filter(|id| !id.is_empty()).unwrap_or(NAMED_LAYOUT_STORE_FALLBACK_APP_ID).to_string()
}

/// 🖥️ This app's USER layouts, projected from the replayed `os.config.ui-preferences` preferences — React's
/// `displayUserLayouts`: `namedLayouts[appId]` sorted by layout id, each `origin: "user"`. A saved arrangement this
/// renderer cannot type is left out rather than failing the roster.
pub(crate) fn user_named_layouts(preferences: &UiPreferences, app_id: Option<&str>) -> Vec<ui_wgpu::wgpu::NamedLayout> {
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
pub(crate) fn named_layout_events(preferences: &UiPreferences, app_id: Option<&str>, layouts: &[ui_wgpu::wgpu::NamedLayout]) -> Vec<UiPreferencesConfigMutation> {
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
pub(crate) fn load_named_layouts_from_store(app_id: Option<&str>) -> Vec<ui_wgpu::wgpu::NamedLayout> {
    user_named_layouts(&read_ui_preferences(), app_id)
}

/// 🖥️ Appends the events that make `layouts` this app's saved roster to the preference log.
pub(crate) fn save_named_layouts_to_store(app_id: Option<&str>, layouts: &[ui_wgpu::wgpu::NamedLayout]) {
    commit_ui_preferences_write(|log| {
        let events = named_layout_events(&replay_ui_preferences(log), app_id, layouts);
        log.events.extend(events);
    });
}

pub(crate) fn save_dock_ui_to_store(app_id: Option<&str>, state: Option<&DockUiState>) {
    write_os_shell_config_layer("dockUi", app_id, state);
}

//#endregion 🧭️PanelAnchorModel

//#region 📏️WindowMeasures
/// 📏️ The retained engine surface one window instance's Measures overlay paints into — the window
/// instance, then the reserved section body key, the same `{windowId}/{id}` rule React's
/// `windowMeasureDomId` qualifies measure ids with. Window instance ids are plain identifiers, so the
/// suffix can never collide with an app-authored surface.
pub(crate) fn window_measures_surface_id(window_id: &str) -> String {
    format!("{window_id}/{}", semio_framework::UiRefreshSection::Measures.body_key())
}

/// 🏁️ The refresh one guest publication owes — the wgpu half of the shared law
/// `🛠️ShellHelpers/🧫️fixtures/🧫️operation-publication/🔣️.json` (React: `typedOperationCompletionRefreshV1` and the
/// progress subscriber). Its own scope when it has one; a COMPLETION that patched history and dirtied nothing else still
/// owes the reserved History body the row it minted; progress with a patch alone moves the band and re-renders nothing;
/// a lossy lane (`resync`) owes everything.
pub(crate) fn operation_publication_refresh(publication: &crate::program_bridge::OperationPublication) -> UiDirtyScope {
    if publication.resync {
        return UiDirtyScope::Full;
    }
    if !publication.ui_scope.asks_for_nothing() {
        return publication.ui_scope.clone();
    }
    if publication.completed && publication.history_patch.is_some() {
        return UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: vec![ui_wgpu::wgpu::FRAMEWORK_HISTORY_BODY_KEY.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false };
    }
    UiDirtyScope::None
}

/// 🩺️ One refresh pass's scope, short enough to read in a console line — which surfaces a settle
/// asked for is the only thing that separates "the guest re-rendered the previous document" from
/// "this pass never looked at that surface".
pub(crate) fn refresh_scope_label(scope: &UiDirtyScope) -> String {
    match scope {
        UiDirtyScope::Full => "full".to_string(),
        UiDirtyScope::None => "none".to_string(),
        UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels } => {
            format!("partial windows=[{}] panels=[{}] utilities={utilities} tools={tools} engagements={engagements} measures={measures} labels={labels}", window_bodies.join(","), panel_bodies.join(","))
        }
    }
}

/// 🪟️ The window instance a Measures overlay surface belongs to, `None` for every other surface.
pub(crate) fn window_measures_owner(surface_id: &str) -> Option<&str> {
    surface_id.strip_suffix(semio_framework::UiRefreshSection::Measures.body_key()).and_then(|owner| owner.strip_suffix('/')).filter(|owner| !owner.is_empty())
}

/// 🪟️ Resolves only the reserved pane identities to their concrete application window.
pub(crate) fn window_pane_owner(surface_id: &str) -> Option<&str> {
    [semio_framework::UiRefreshSection::Measures.body_key(), semio_framework::UiRefreshSection::Engagements.body_key(), WINDOW_SEARCH_BODY_KEY, WINDOW_PROJECTION_BODY_KEY]
        .into_iter()
        .find_map(|suffix| surface_id.strip_suffix(suffix).and_then(|owner| owner.strip_suffix('/')).filter(|owner| !owner.is_empty()))
}

/// 📏️ Projects general window measures into a compact retained Tree with one labelled row per
/// measure. Controls preserve their original keys and action bindings; groups retain disclosure
/// and their optional inline header slider. Utility-scoped groups belong to the utility rail.
/// See `🧑‍🎨engine/🧫️fixtures/📏️window-measures/🔣️.json`.
pub(crate) fn window_measures_overlay_records(window_id: &str, measures: &[WindowMeasure], active_utility_id: Option<&str>) -> Result<Option<Vec<ui_contract::UiNodeRecord>>, String> {
    let partition = ui_wgpu::wgpu::partition_window_measures(measures, active_utility_id)
        .map_err(|measure| format!("window '{window_id}' authored more than {} measures, refused at {measure:?}", ui_wgpu::wgpu::component::layout::WINDOW_MEASURE_PARTITION_CAPACITY))?;
    if partition.general.is_empty() {
        return Ok(None);
    }
    let mut projection = WindowMeasuresProjection { window_id, records: Vec::new() };
    let root = projection.reserve();
    let section = projection.reserve();
    let mut children = ui_contract::UiNodeChildren::default();
    for measure in partition.general.iter() {
        if let Some(child) = projection.measure(measure)? {
            children.try_push(child).map_err(|_| format!("window '{window_id}' measures exceed one overlay document"))?;
        }
    }
    projection.place(
        section,
        format!("{window_id}/measures.rows"),
        ui_contract::Component::TreeSection(ui_contract::TreeSectionProps { label: None, default_open: Some(true), header_toolbar: None, window: None }),
        MeasureRecord { children, ..MeasureRecord::default() },
    )?;
    let mut children = ui_contract::UiNodeChildren::default();
    children.try_push(section).map_err(|_| "measure tree admits its section".to_string())?;
    projection.place(
        root,
        semio_framework::UiRefreshSection::Measures.body_key().to_string(),
        ui_contract::Component::Tree(ui_contract::TreeProps { presentation: ui_contract::TreePresentation::Compact, interaction_domain: None }),
        MeasureRecord { children, ..MeasureRecord::default() },
    )?;
    projection.records.into_iter().collect::<Option<Vec<_>>>().map(Some).ok_or_else(|| format!("window '{window_id}' measures projection left a reserved record unplaced"))
}

/// 🧩️ Everything a measure record carries beside its key and component.
#[derive(Default)]
struct MeasureRecord {
    pub(crate) children: ui_contract::UiNodeChildren,
    pub(crate) bindings: ui_contract::UiNodeBindings,
    pub(crate) activity: ui_contract::Activity,
    pub(crate) disabled: bool,
    pub(crate) label: Option<ui_contract::Label>,
}

/// 🌳️ One overlay document under construction: ids are reserved pre-order and filled once a node's
/// children exist, so the published record order is parent-first.
struct WindowMeasuresProjection<'a> {
    pub(crate) window_id: &'a str,
    pub(crate) records: Vec<Option<ui_contract::UiNodeRecord>>,
}

impl WindowMeasuresProjection<'_> {
    pub(crate) fn reserve(&mut self) -> ui_contract::UiNodeId {
        self.records.push(None);
        ui_contract::UiNodeId(self.records.len() as u64)
    }

    pub(crate) fn place(&mut self, id: ui_contract::UiNodeId, key: String, component: ui_contract::Component, record: MeasureRecord) -> Result<ui_contract::UiNodeId, String> {
        if self.records.len() > ui_contract::UI_DOCUMENT_NODES {
            return Err(format!("window '{}' measures exceed {} overlay nodes", self.window_id, ui_contract::UI_DOCUMENT_NODES));
        }
        let key = UiText::try_from_string(key).map_err(|key| format!("measure key '{key}' exceeds the retained contract"))?;
        let MeasureRecord { children, bindings, activity, disabled, label } = record;
        self.records[(id.0 - 1) as usize] = Some(ui_contract::UiNodeRecord {
            id,
            key,
            component,
            layout: ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { axis: ui_contract::Axis::Vertical, gap: ui_contract::SpaceToken::None, ..Default::default() }),
            style: Default::default(),
            activity,
            disabled,
            transition: None,
            accessibility: ui_contract::AccessibilitySpec { label, ..Default::default() },
            bindings,
            menu: None,
            children,
        });
        Ok(id)
    }

    pub(crate) fn control(&mut self, id: &str, label: Option<&str>, component: ui_contract::Component, record: MeasureRecord) -> Result<ui_contract::UiNodeId, String> {
        let key = format!("{}/{id}", self.window_id);
        let row = self.reserve();
        let control = self.reserve();
        let row_label = label.map(measure_label).or_else(|| record.label.clone()).unwrap_or_else(|| measure_label(""));
        self.place(control, key.clone(), component, record)?;
        let mut children = ui_contract::UiNodeChildren::default();
        children.try_push(control).map_err(|_| "measure row admits its control".to_string())?;
        self.place(row, format!("{key}.row"), measure_tree_item(row_label.clone(), None), MeasureRecord { children, label: Some(row_label), ..MeasureRecord::default() })
    }

    pub(crate) fn measure(&mut self, measure: &WindowMeasure) -> Result<Option<ui_contract::UiNodeId>, String> {
        match measure {
            WindowMeasure::Number { id, label, value, min, max, step, loading, waiting, disabled, on_change, .. } => {
                let input = ui_contract::InputProps { kind: ui_contract::InputKind::Number, value: UiText::clipped(&value.to_string()), placeholder: None, commit: Some(UiText::clipped("blur")), min: *min, max: *max, step: *step, accept: None, precision: None, ..Default::default() };
                let record = MeasureRecord {
                    bindings: measure_bindings(ui_contract::Trigger::Commit, on_change, None)?,
                    activity: measure_activity(*loading, *waiting),
                    disabled: disabled.unwrap_or(false),
                    label: label.as_deref().map(measure_label),
                    ..MeasureRecord::default()
                };
                self.control(id, label.as_deref(), ui_contract::Component::Input(input), record).map(Some)
            }
            WindowMeasure::Slider { id, label, value, min, max, step, loading, waiting, disabled, on_change, .. } => {
                let record = MeasureRecord {
                    bindings: measure_bindings(ui_contract::Trigger::Change, on_change, None)?,
                    activity: measure_activity(*loading, *waiting),
                    disabled: disabled.unwrap_or(false),
                    label: label.as_deref().map(measure_label),
                    ..MeasureRecord::default()
                };
                self.control(id, label.as_deref(), measure_slider(*value, *min, *max, *step, None), record).map(Some)
            }
            WindowMeasure::Select { id, label, value, items, on_change } => {
                let mut options = UiFixedList::default();
                for item in items {
                    options.try_push(ui_contract::SelectItem { value: UiText::clipped(&item.value), label: measure_label(&item.label) }).map_err(|_| format!("measure '{id}' has more options than one select admits"))?;
                }
                let record = MeasureRecord { bindings: measure_bindings(ui_contract::Trigger::Change, on_change, None)?, label: label.as_deref().map(measure_label), ..MeasureRecord::default() };
                self.control(id, label.as_deref(), ui_contract::Component::Select(ui_contract::SelectProps { value: UiText::clipped(value), items: options, placeholder: None, appearance: ui_contract::SelectAppearance::Menu }), record).map(Some)
            }
            WindowMeasure::Toggle { id, icon_id, label, pressed, text, on_change } => {
                let toggle = ui_contract::ToggleProps { appearance: ui_contract::ToggleAppearance::Checkbox, on: *pressed, icon: UiText::clipped(icon_id.as_str()), text: text.as_deref().map(measure_label) };
                let record =
                    MeasureRecord { bindings: measure_bindings(ui_contract::Trigger::Change, on_change, Some(("pressed", semio_framework_value::DslValue::Bool(!pressed))))?, label: label.as_deref().or(text.as_deref()).map(measure_label), ..MeasureRecord::default() };
                self.control(id, label.as_deref(), ui_contract::Component::Toggle(toggle), record).map(Some)
            }
            WindowMeasure::Group { id, label, default_open, value, min, max, step, loading, waiting, on_change, children, .. } => {
                let key = format!("{}/{id}", self.window_id);
                let section = self.reserve();
                let mut ids = ui_contract::UiNodeChildren::default();
                if let (Some(value), Some(on_change)) = (value, on_change) {
                    let header = self.reserve();
                    let record = MeasureRecord { bindings: measure_bindings(ui_contract::Trigger::Change, on_change, None)?, activity: measure_activity(*loading, *waiting), label: Some(measure_label(label)), ..MeasureRecord::default() };
                    ids.try_push(self.place(header, format!("{key}.header-slider"), measure_slider(*value, min.unwrap_or(0.0), max.unwrap_or(1.0), *step, None), record)?).map_err(|_| format!("measure group '{id}' exceeds one overlay document"))?;
                }
                for child in children {
                    if let Some(child) = self.measure(child)? {
                        ids.try_push(child).map_err(|_| format!("measure group '{id}' exceeds one overlay document"))?;
                    }
                }
                let record = MeasureRecord { children: ids, activity: measure_activity(*loading, *waiting), label: Some(measure_label(label)), ..MeasureRecord::default() };
                self.place(section, format!("{key}.row"), measure_tree_item(measure_label(label), Some(default_open.unwrap_or(false))), record).map(Some)
            }
        }
    }
}

/// 🌿️ One labelled measure row with a distinct control child and retained disclosure state.
fn measure_tree_item(label: ui_contract::Label, default_open: Option<bool>) -> ui_contract::Component {
    ui_contract::Component::TreeItem(ui_contract::TreeItemProps {
        label,
        description: None,
        icon: None,
        default_open,
        draggable: None,
        drag_data: None,
        dimmed: None,
        selected: None,
        window: None,
        granularity: None,
        inline_toolbar: None,
        detail: None,
        row_actions: Default::default(),
        target: None,
    })
}

pub(crate) fn measure_label(value: &str) -> ui_contract::Label {
    ui_contract::Label(UiText::clipped(value))
}

pub(crate) fn measure_container(role: ui_contract::ContainerRole, label: Option<&str>, default_open: Option<bool>) -> ui_contract::ContainerProps {
    ui_contract::ContainerProps { role, label: label.map(measure_label), description: None, required: None, error: None, default_open, drop_overlay: None }
}

pub(crate) fn measure_slider(value: f64, min: f64, max: f64, step: Option<f64>, unit: Option<&str>) -> ui_contract::Component {
    ui_contract::Component::Slider(ui_contract::SliderProps { value, min, max, step: step.unwrap_or(0.0), unit: unit.map(UiText::clipped), ..Default::default() })
}

fn measure_activity(loading: Option<bool>, waiting: Option<bool>) -> ui_contract::Activity {
    match (loading, waiting) {
        (Some(true), _) => ui_contract::Activity::Loading,
        (_, Some(true)) => ui_contract::Activity::Waiting,
        _ => ui_contract::Activity::Idle,
    }
}

/// 🔗️ A measure's `on_change` as the record's one binding: the authored args, plus `extra` merged over
/// them. The gesture's own scalar is merged over these again by the retained router.
pub(crate) fn measure_binding(trigger: ui_contract::Trigger, action: &ActionDescriptor, extra: Option<(&str, DslValue)>) -> Result<ui_contract::ActionBinding, String> {
    let mut entries = match action.args.as_ref() {
        Some(semio_framework_value::DslValue::Object(entries)) => entries.clone(),
        _ => Vec::new(),
    };
    if let Some((key, value)) = extra {
        entries.retain(|(existing, _)| existing != key);
        entries.push((key.to_string(), value));
    }
    let args = if entries.is_empty() {
        None
    } else {
        Some(serde_json::from_value::<ui_contract::UiValue>(dsl_value_as_json(&semio_framework_value::DslValue::Object(entries))).map_err(|error| format!("measure action '{}' args exceed the retained contract: {error}", action.action))?)
    };
    let action_id = ui_contract::ActionId::try_v1(&action.controller_id, &action.action).ok_or_else(|| format!("measure action '{}' exceeds the retained contract", action.action))?;
    Ok(ui_contract::ActionBinding { trigger, action: action_id, args, capability: None })
}

/// 🎯️ A measured row's click as its contract [`ui_contract::RowTarget`]: the descriptor's controller and version, its argument
/// map, and its action as the target's activation verb.
pub(crate) fn measure_row_target(action: &ActionDescriptor) -> Result<ui_contract::RowTarget, String> {
    let binding = measure_binding(ui_contract::Trigger::Activate, action, None)?;
    let args = match binding.args {
        Some(ui_contract::UiValue::Map(args)) => Some(args),
        None => None,
        Some(_) => return Err(format!("measure row action '{}' args are not an argument map", action.action)),
    };
    Ok(ui_contract::RowTarget { scope: binding.action.scope, version: binding.action.version, args, activation: Some(binding.action.name) })
}

pub(crate) fn measure_bindings(trigger: ui_contract::Trigger, action: &ActionDescriptor, extra: Option<(&str, DslValue)>) -> Result<ui_contract::UiNodeBindings, String> {
    let mut bindings = ui_contract::UiNodeBindings::default();
    bindings.try_push(measure_binding(trigger, action, extra)?).map_err(|_| "measure binding admits one action".to_string())?;
    Ok(bindings)
}

impl ShellState {
    /// 🎬️ Installs one canonical engagements section and retires its transport lease before the
    /// shell republishes Actions/Search bodies. A malformed payload reports a fault and preserves
    /// the last valid snapshot.
    pub(crate) fn install_window_engagements_section(&mut self, document: UiDocumentLease, faults: &mut Vec<(String, String, crate::program_bridge::ProgramFault)>) -> Result<(), String> {
        let body_key = semio_framework::UiRefreshSection::Engagements.body_key();
        let engagements = crate::program_bridge::window_engagements_from_section(&document);
        self.retire_one_surface_document(Some(document))?;
        match engagements {
            Ok(engagements) => self.window_engagements = engagements,
            Err(error) => faults.push((body_key.to_string(), body_key.to_string(), error.into())),
        }
        Ok(())
    }

    /// 🎬️ Reads the canonical reserved engagements surface on either ProgramBridge backend.
    pub(crate) async fn refresh_window_engagements(&mut self, program: &ProgramBridgeEntry, instance_id: u32, view_state: &ViewModel, read_ahead: Option<Result<UiDocumentLease, crate::program_bridge::ProgramFault>>, faults: &mut Vec<(String, String, crate::program_bridge::ProgramFault)>) -> Result<(), String> {
        let body_key = semio_framework::UiRefreshSection::Engagements.body_key();
        let section = match read_ahead {
            Some(section) => section,
            None => program.window_engagements_section(instance_id, view_state).await,
        };
        match section {
            Ok(document) => self.install_window_engagements_section(document, faults)?,
            Err(error) => faults.push((body_key.to_string(), body_key.to_string(), error.into())),
        }
        Ok(())
    }

    /// 📏️ Reads the instance's reserved measures surface once per refresh and republishes every live
    /// window instance's Measures overlay document. A window whose measures fail to project keeps no
    /// overlay and reports a surface fault instead of blanking the refresh.
    pub(crate) async fn refresh_window_measures(
        &mut self,
        program: &ProgramBridgeEntry,
        instance_id: u32,
        view_state: &ViewModel,
        read_ahead: Option<Result<UiDocumentLease, crate::program_bridge::ProgramFault>>,
        windows: &[String],
        faults: &mut Vec<(String, String, crate::program_bridge::ProgramFault)>,
    ) -> Result<(), String> {
        let body_key = semio_framework::UiRefreshSection::Measures.body_key();
        let section = match read_ahead {
            Some(section) => section,
            None => program.window_measures_section(instance_id, view_state).await,
        };
        let measures = match section {
            Ok(document) => {
                let measures = crate::program_bridge::window_measures_from_section(&document).map_err(crate::program_bridge::ProgramFault::from);
                self.retire_one_surface_document(Some(document))?;
                measures
            }
            Err(error) => Err(error),
        };
        let measures = measures.unwrap_or_else(|error| {
            faults.push((body_key.to_string(), body_key.to_string(), error.into()));
            HashMap::new()
        });
        let stale: Vec<String> = self.window_measures_documents.keys().filter(|window_id| !windows.contains(window_id)).cloned().collect();
        for window_id in stale.into_iter().chain(windows.iter().cloned()) {
            let previous = self.window_measures_documents.remove(&window_id);
            self.retire_one_surface_document(previous)?;
            if !windows.contains(&window_id) {
                continue;
            }
            let surface = window_measures_surface_id(&window_id);
            match self.publish_window_measures(&window_id, &surface, measures.get(&window_id).map_or(&[], Vec::as_slice)) {
                Ok(Some(document)) => {
                    self.window_measures_documents.insert(window_id, document);
                }
                Ok(None) => {}
                Err(error) => faults.push((surface, body_key.to_string(), error.into())),
            }
        }
        Ok(())
    }

    /// 🛠️ Reads the instance's reserved TOOLS section once per refresh into
    /// [`ShellState::tool_measures`] — the wgpu twin of React's `toolMeasuresByToolId` ref, which its
    /// `buildToolTabs` `resolveTree` reads fresh at render time. A read failure reports a surface
    /// fault and leaves the previous roster rather than blanking every armed tool's options.
    pub(crate) async fn refresh_tool_measures(&mut self, program: &ProgramBridgeEntry, instance_id: u32, view_state: &ViewModel, read_ahead: Option<Result<UiDocumentLease, crate::program_bridge::ProgramFault>>, faults: &mut Vec<(String, String, crate::program_bridge::ProgramFault)>) -> Result<(), String> {
        let body_key = semio_framework::UiRefreshSection::Tools.body_key();
        let section = match read_ahead {
            Some(section) => section,
            None => program.tool_measures_section(instance_id, view_state).await,
        };
        let measures = match section {
            Ok(document) => {
                let measures = crate::program_bridge::tool_measures_from_section(&document).map_err(crate::program_bridge::ProgramFault::from);
                self.retire_one_surface_document(Some(document))?;
                measures
            }
            Err(error) => Err(error),
        };
        match measures {
            Ok(measures) => self.tool_measures = measures,
            Err(error) => faults.push((body_key.to_string(), body_key.to_string(), error.into())),
        }
        Ok(())
    }

    /// 🧾️ One window's overlay document, minted through the engine's own ingress-generation rule
    /// keyed on the projected content, so an unchanged overlay pays no ingress.
    fn publish_window_measures(&mut self, window_id: &str, surface: &str, measures: &[WindowMeasure]) -> Result<Option<UiDocumentLease>, String> {
        let Some(records) = window_measures_overlay_records(window_id, measures, self.active_utility_for_window(window_id))? else { return Ok(None) };
        self.publish_surface_records(surface, records).map(Some)
    }

    /// 🧾️ Mints one SHELL-OWNED retained document from already-projected records: the revision is the
    /// content's own hash, so republishing an unchanged body answers `Published` and pays no ingress.
    /// Every shell-owned panel (the Measures overlay, the Command dock, Settings' five leaves, the Tool
    /// leaves, the Marketplace leaf) goes through this one ingress.
    pub(crate) fn publish_surface_records(&mut self, surface: &str, records: Vec<ui_contract::UiNodeRecord>) -> Result<UiDocumentLease, String> {
        let revision = {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            serde_json::to_string(&records).map_err(|error| error.to_string())?.hash(&mut hasher);
            hasher.finish()
        };
        let generation = ui_wgpu::wgpu::engine::ui_document_ingress_generation(self.window_measures_minted.get(surface).copied(), revision);
        self.window_measures_minted.insert(surface.to_string(), (revision, generation));
        let identity = ui_contract::UiDocumentAssemblyIdentity { generation, revision: ui_contract::UiRevision(revision), root: Some(ui_contract::UiNodeId(1)), layout_epoch: 0 };
        let surface_id = SurfaceId::try_from(surface).map_err(|_| format!("shell surface '{surface}' exceeds the retained contract"))?;
        UiDocumentLease::try_publish(surface_id, identity, records).map_err(|error| crate::program_bridge::retained_publication_refusal(surface, error))
    }

    /// 🖌️ One paint opportunity of a window's Measures overlay, over the window body it belongs to:
    /// the overlay backdrop once, then its retained document through the same stepped paint every body
    /// uses, then its hit registry above the body's. `true` once the overlay is off the walk's critical
    /// path — painted, absent, folded, or faulted after its opportunity ceiling.
    #[allow(clippy::too_many_arguments, reason = "the chrome walk's own paint context, forwarded unchanged")]
    pub(crate) fn paint_window_measures_step(
        &mut self,
        cursor: &mut ShellChromeChildCursor,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        window_id: &str,
        window_rect: Rect,
        world_resources: &mut infinite_world::world::World3dBuildContext,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> bool {
        if !self.window_measures_documents.contains_key(window_id) {
            return true;
        }
        if self.measures_rail_folded(window_id) {
            return true;
        }
        let Some(document) = self.window_measures_documents.remove(window_id) else { return true };
        let surface = window_measures_surface_id(window_id);
        crate::interpreter::set_ui_document_flow(&surface, ui_contract::UiFlow::for_anchor(ui_contract::Anchor::TopEnd));
        let rect = self.window_measures_rect(window_id, window_rect, theme);
        if !cursor.flag && cursor.document.layout_is_accepted() && cursor.rect.is_some_and(|previous| previous != rect) {
            cursor.document.restart_viewport_after_host_reflow();
        }
        cursor.rect = Some(rect);
        if !cursor.flag && cursor.document.phase_name() == "paint" {
            draw.push_solid([rect.x, rect.y, rect.w, rect.h], theme.panel);
            cursor.flag = true;
        }
        let controller = self.document_controller_id();
        let complete = {
            let scroll_offsets = &mut self.scroll_offsets;
            let collapsed_sections = &mut self.collapsed_sections;
            let open_selects = &mut self.open_selects;
            let widget_maps = &mut self.widget_maps_staging;
            let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: scene_chrome_labels(self.locale_id == "de"), world3d_states: &mut self.world3d_states, world_resources, window_id: surface.as_str() };
            let viewport_height_for_widgets = draw.screen_height();
            let mut ctx = framework_widget_context(draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, Some(widget_maps), viewport_height_for_widgets);
            ctx.pick_clip = Some(rect);
            render_ui_document_step(&mut cursor.document, &document, rect, &mut ctx, surface.as_str(), controller.as_str(), self.chrome_build.driver.drag, &mut hosts, cx)
        };
        self.window_measures_documents.insert(window_id.to_string(), document);
        if complete {
            self.note_retained_body_painted(&surface);
        } else {
            if document_opportunity_remains(cursor, SHELL_WINDOW_PAINT_OPPORTUNITIES) {
                return false;
            }
            self.record_document_paint_fault(&surface);
        }
        push_chrome_group_border(draw, rect, theme);
        self.register_retained_body_hits(&surface, rect, input);
        true
    }

    /// 🎛️ React's Window Options fold state: `useState(true)` — the rail starts FOLDED and is opened
    /// by its own chip (`🖱️ui/🧱️elements/🪟️Window/🟦️.tsx:169-173`).
    pub(crate) fn measures_rail_folded(&self, window_id: &str) -> bool {
        self.measures_folded.get(window_id).copied().unwrap_or(true)
    }

    /// 📐️ The Measures overlay's rect inside a window body: right-aligned like React's measures pane,
    /// at the window's own resized width (or the theme default), inset by the panel inset.
    fn window_measures_rect(&self, window_id: &str, body: Rect, theme: &Theme) -> Rect {
        let width = self.measures_width.get(window_id).copied().unwrap_or(theme.window_measures_default_width).min((body.w - theme.panel_inset * 2.0).max(0.0));
        let top = theme.panel_inset + theme.control_height + theme.gap_standard;
        let available = (body.h - top - theme.panel_inset).max(0.0);
        let height = crate::interpreter::retained_content_height(&window_measures_surface_id(window_id)).unwrap_or(available).min(available).max(0.0);
        Rect::new(body.x + body.w - width - theme.panel_inset, body.y + top, width, height)
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/📏️wgpu-window-measures/🦀️.rs"]
mod window_measures_tests;
//#endregion 📏️WindowMeasures
//#endregion ShellLifecycle
