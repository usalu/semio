//! 🚶️ `shell-session` staging of `dock-model-and-measures`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
//#region 🧭️PanelAnchorModel
pub(super) fn dock_tab_path(nodes: &[DockTabNode], tab_id: &str) -> Option<Vec<String>> {
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

fn os_shell_config_layer<T: serde::de::DeserializeOwned>(section: &str, app_id: Option<&str>) -> Option<T> {
    read_os_shell_config_layer(&os_shell_config_document(), section, app_id)
}

/// 🗄️ Reads the persisted dock arrangement — the per-app layer when present, else the shared `os` one,
/// exactly the precedence React's `DockLayoutStore::getSnapshot` applies.
pub(crate) fn load_dock_skeleton_from_store(app_id: Option<&str>) -> Option<DockSkeleton> {
    os_shell_config_layer::<DockSkeleton>("dockLayouts", app_id).filter(|skeleton| skeleton.version == DOCK_SKELETON_VERSION)
}

pub(crate) fn load_dock_ui_from_store(app_id: Option<&str>) -> Option<DockUiState> {
    os_shell_config_layer::<DockUiState>("dockUi", app_id).filter(|state| state.version == DOCK_SKELETON_VERSION)
}

//#endregion 🧭️PanelAnchorModel
//#region 📏️WindowMeasures
/// 🪟️ Resolves only the reserved pane identities to their concrete application window.
pub(crate) fn window_pane_owner(surface_id: &str) -> Option<&str> {
    [semio_framework::UiRefreshSection::Measures.body_key(), semio_framework::UiRefreshSection::Engagements.body_key(), WINDOW_SEARCH_BODY_KEY, WINDOW_PROJECTION_BODY_KEY]
        .into_iter()
        .find_map(|suffix| surface_id.strip_suffix(suffix).and_then(|owner| owner.strip_suffix('/')).filter(|owner| !owner.is_empty()))
}

//#endregion 📏️WindowMeasures
//#endregion ShellLifecycle

