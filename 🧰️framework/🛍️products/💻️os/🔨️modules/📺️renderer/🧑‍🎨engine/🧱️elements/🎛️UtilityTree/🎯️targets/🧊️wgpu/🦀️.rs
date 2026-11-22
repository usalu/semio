//! 🌳️ wgpu twin of React's `🎛️UtilityTree` model (`🟦️.tsx`): category grouping, ribbon segments, active-path reconciliation and the row buckets (depth rows, `row-utility-options`, `row-selection-options`), pinned by `🧪️tests/🧫️fixtures/🌳️utility-tree/🔣️.json`.

use semio_framework::{IconName, SET_ACTIVE_UTILITY_ACTION_ID};
use semio_renderer_wgpu_shell_prefs::dsl_value_as_json;
use semio_ui_declaration::layout::ActionDescriptor;
use semio_ui_declaration::utilities::{UtilityCategory, UtilityNode};

pub const UTILITY_CATEGORY_ORDER: [UtilityCategory; 4] = [UtilityCategory::Selection, UtilityCategory::Utilities, UtilityCategory::History, UtilityCategory::Sync];

/// 🪟️ Categories scoped to whatever window the user interacts with; they live in each window's own bottom-left panel.
pub const UTILITY_CATEGORIES: [UtilityCategory; 2] = [UtilityCategory::Selection, UtilityCategory::Utilities];

pub fn utility_category_tag(category: UtilityCategory) -> &'static str {
    match category {
        UtilityCategory::Selection => "selection",
        UtilityCategory::Utilities => "utilities",
        UtilityCategory::History => "history",
        UtilityCategory::Sync => "sync",
    }
}

pub fn utility_category_icon(category: UtilityCategory) -> IconName {
    match category {
        UtilityCategory::Selection => IconName::MousePointer,
        UtilityCategory::Utilities => IconName::Wrench,
        UtilityCategory::History => IconName::Undo,
        UtilityCategory::Sync => IconName::Cloud,
    }
}

fn node_id(node: &UtilityNode) -> &str {
    match node {
        UtilityNode::Separator { id, .. } | UtilityNode::Button { id, .. } | UtilityNode::Toggle { id, .. } | UtilityNode::Collection { id, .. } => id,
    }
}

fn node_order(node: &UtilityNode) -> u32 {
    match node {
        UtilityNode::Separator { order, .. } | UtilityNode::Button { order, .. } | UtilityNode::Toggle { order, .. } | UtilityNode::Collection { order, .. } => order.unwrap_or(0),
    }
}

fn node_disabled(node: &UtilityNode) -> bool {
    match node {
        UtilityNode::Separator { disabled, .. } | UtilityNode::Button { disabled, .. } | UtilityNode::Toggle { disabled, .. } | UtilityNode::Collection { disabled, .. } => disabled.unwrap_or(false),
    }
}

/// 🔢️ React's `sortUtilityNodes`: stable by `order`, a missing order reading as zero.
pub fn sort_utility_nodes(nodes: &[UtilityNode]) -> Vec<UtilityNode> {
    let mut sorted = nodes.to_vec();
    sorted.sort_by_key(node_order);
    sorted
}

/// 🎀️ A node a person can press: never a separator, and a collection only while it holds one.
pub fn has_interactive_utility_nodes(nodes: &[UtilityNode]) -> bool {
    nodes.iter().any(|node| match node {
        UtilityNode::Separator { .. } => false,
        UtilityNode::Collection { children, .. } => has_interactive_utility_nodes(children),
        _ => true,
    })
}

/// 🗂️ React's `groupUtilityNodesByCategory`: buckets top-level nodes into `categories` (an uncategorized node is a utility, a separator one too), drops buckets with nothing interactive, and wraps each in a collection numbered by its position — a bucket of one collection is used as-is.
pub fn group_utility_nodes_by_category(nodes: &[UtilityNode], categories: &[UtilityCategory]) -> Vec<UtilityNode> {
    categories
        .iter()
        .map(|category| nodes.iter().filter(|node| node.category() == *category).cloned().collect::<Vec<_>>())
        .zip(categories)
        .filter(|(bucket, _)| has_interactive_utility_nodes(bucket))
        .enumerate()
        .map(|(order, (bucket, category))| match bucket.as_slice() {
            [UtilityNode::Collection { id, icon_id, label, text, title, disabled, category: inner, children, .. }] => UtilityNode::Collection { id: id.clone(), icon_id: *icon_id, label: label.clone(), text: text.clone(), title: title.clone(), order: Some(order as u32), disabled: *disabled, category: *inner, children: children.clone() },
            _ => UtilityNode::Collection { id: utility_category_tag(*category).into(), icon_id: utility_category_icon(*category), label: None, text: Some(utility_category_tag(*category).into()), title: None, order: Some(order as u32), disabled: None, category: Some(*category), children: bucket },
        })
        .collect()
}

/// 🦶️ React's `dedupeUtilityNodesById`: the first node of each id across every window's utility set.
pub fn dedupe_utility_nodes_by_id(lists: &[Vec<UtilityNode>]) -> Vec<UtilityNode> {
    let mut seen: Vec<UtilityNode> = Vec::new();
    for node in lists.iter().flatten() {
        if !seen.iter().any(|kept| node_id(kept) == node_id(node)) {
            seen.push(node.clone());
        }
    }
    seen
}

/// 🎀️ One level of the drill-down ribbon: the picker of its collections, or the loose leaves; `depth` counts the collections drilled into.
#[derive(Clone, Debug, PartialEq)]
pub enum UtilityRibbonSegment {
    Picker { collections: Vec<UtilityNode>, depth: usize },
    Utilities { items: Vec<UtilityNode>, depth: usize },
}

impl UtilityRibbonSegment {
    pub fn depth(&self) -> usize {
        match self {
            Self::Picker { depth, .. } | Self::Utilities { depth, .. } => *depth,
        }
    }

    /// 🔑️ React's `utilityRibbonSegmentKey`, `index` being the segment's position among its siblings.
    pub fn key(&self, index: usize) -> String {
        let ids = |nodes: &[UtilityNode]| nodes.iter().map(node_id).collect::<Vec<_>>().join("-");
        match self {
            Self::Picker { collections, depth } => format!("picker-{depth}-{}", ids(collections)),
            Self::Utilities { items, .. } => format!("utilities-{index}-{}", ids(items)),
        }
    }
}

fn enabled_collections(nodes: &[UtilityNode]) -> Vec<UtilityNode> {
    sort_utility_nodes(nodes).into_iter().filter(|node| matches!(node, UtilityNode::Collection { .. }) && !node_disabled(node)).collect()
}

/// 🎀️ React's `buildUtilityRibbonSegments`: a level recurses only when `path[depth]` names one of its enabled collections, so at most one group is open per level.
pub fn build_utility_ribbon_segments(nodes: &[UtilityNode], path: &[String], depth: usize) -> Vec<UtilityRibbonSegment> {
    let sorted = sort_utility_nodes(nodes);
    let collections = enabled_collections(nodes);
    let loose: Vec<UtilityNode> = sorted.into_iter().filter(|node| !matches!(node, UtilityNode::Collection { .. })).collect();
    let mut segments = Vec::new();
    if !collections.is_empty() {
        segments.push(UtilityRibbonSegment::Picker { collections: collections.clone(), depth });
    }
    if loose.iter().any(|node| !matches!(node, UtilityNode::Separator { .. })) {
        segments.push(UtilityRibbonSegment::Utilities { items: loose, depth });
    }
    let Some(active) = path.get(depth).and_then(|id| collections.iter().find(|node| node_id(node) == id)) else { return segments };
    let UtilityNode::Collection { children, .. } = active else { return segments };
    segments.extend(build_utility_ribbon_segments(children, path, depth + 1));
    segments
}

/// 🎀️ React's `reconcileUtilityPath`: keeps each entry only while it names an enabled collection at its level, truncating at the first miss.
pub fn reconcile_utility_path(nodes: &[UtilityNode], path: &[String]) -> Vec<String> {
    let mut current = nodes.to_vec();
    let mut reconciled = Vec::new();
    for id in path {
        let Some(UtilityNode::Collection { children, .. }) = enabled_collections(&current).into_iter().find(|node| node_id(node) == id) else { break };
        reconciled.push(id.clone());
        current = children;
    }
    reconciled
}

/// 🎛️ React's `findPressedUtilityLeafId`: the first pressed toggle, depth first.
pub fn find_pressed_utility_leaf_id(nodes: &[UtilityNode]) -> Option<String> {
    nodes.iter().find_map(|node| match node {
        UtilityNode::Collection { children, .. } => find_pressed_utility_leaf_id(children),
        UtilityNode::Toggle { id, pressed: Some(true), .. } => Some(id.clone()),
        _ => None,
    })
}

/// 🎓️ React's `findUtilityGroupPath`: the group ids leading to `target_id` (`[]` for a top-level node), `None` when no node has that id.
pub fn find_utility_group_path(nodes: &[UtilityNode], target_id: &str, prefix: &[String]) -> Option<Vec<String>> {
    nodes.iter().find_map(|node| {
        if node_id(node) == target_id {
            return Some(prefix.to_vec());
        }
        let UtilityNode::Collection { id, children, .. } = node else { return None };
        find_utility_group_path(children, target_id, &[prefix, &[id.clone()]].concat())
    })
}

/// 🧰️ React's `findSetActiveUtilityDescriptor`: the first `setActiveUtility` change descriptor of the tree.
pub fn find_set_active_utility_descriptor(nodes: &[UtilityNode]) -> Option<&ActionDescriptor> {
    nodes.iter().find_map(|node| match node {
        UtilityNode::Collection { children, .. } => find_set_active_utility_descriptor(children),
        UtilityNode::Toggle { on_change, .. } if on_change.action == SET_ACTIVE_UTILITY_ACTION_ID => Some(on_change),
        _ => None,
    })
}

/// 🔢️ React's `expectedGenerationOf`: the register generation a `setActiveUtility` descriptor was stamped with, when a non-negative integer.
pub fn expected_generation_of(template: Option<&ActionDescriptor>) -> Option<u64> {
    template?.args.as_ref().map(dsl_value_as_json)?.get("expectedGeneration")?.as_u64()
}

/// 🧭️ The first pressed toggle whose id starts with `select`: the selection utility a host row renders its options for.
pub fn find_pressed_selection_utility(nodes: &[UtilityNode]) -> Option<&UtilityNode> {
    nodes.iter().find_map(|node| match node {
        UtilityNode::Collection { children, .. } => find_pressed_selection_utility(children),
        UtilityNode::Toggle { id, pressed: Some(true), .. } if id.starts_with("select") => Some(node),
        _ => None,
    })
}

/// 🎀️ Where a ribbon stacks its collections: `Inline` keeps the horizontal drill-down (footer), `Up` and `Down` stack one line per depth.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UtilityTreeDirection {
    Inline,
    Up,
    Down,
}

impl UtilityTreeDirection {
    pub fn from_tag(tag: &str) -> Option<Self> {
        match tag {
            "inline" => Some(Self::Inline),
            "up" => Some(Self::Up),
            "down" => Some(Self::Down),
            _ => None,
        }
    }
}

/// 🔢️ The pressed selection utility a host row renders its options for: its id, the owning window and the register generation its press is stamped with.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UtilityTreeSelection {
    pub active_utility_id: String,
    pub window_id: String,
    pub generation: Option<u64>,
}

/// 🎀️ The ribbon rows' keys, top to bottom: one per segment inline, one per depth when stacked; stacked, the host's utility options take the last row, else the selection options of a pressed selection utility.
pub fn utility_ribbon_row_keys(utilities: &[UtilityNode], path: &[String], direction: UtilityTreeDirection, has_utility_options: bool, has_selection_renderer: bool) -> Vec<String> {
    let segments = build_utility_ribbon_segments(utilities, path, 0);
    let mut keys: Vec<String> = if direction == UtilityTreeDirection::Inline {
        segments.iter().enumerate().map(|(index, segment)| segment.key(index)).collect()
    } else {
        let mut depths: Vec<usize> = segments.iter().map(UtilityRibbonSegment::depth).collect();
        depths.sort_unstable();
        depths.dedup();
        depths.into_iter().map(|depth| format!("row-{depth}")).collect()
    };
    if direction != UtilityTreeDirection::Inline {
        if has_utility_options {
            keys.push("row-utility-options".into());
        } else if has_selection_renderer && find_pressed_selection_utility(utilities).is_some() {
            keys.push("row-selection-options".into());
        }
    }
    keys
}

/// 🧭️ The selection a host row is rendered for, `window_id` read off a `ui.utilities.{window}` tree id.
pub fn utility_tree_selection(utilities: &[UtilityNode], tree_id: &str) -> Option<UtilityTreeSelection> {
    let active = find_pressed_selection_utility(utilities)?;
    Some(UtilityTreeSelection { active_utility_id: node_id(active).to_string(), window_id: tree_id.strip_prefix("ui.utilities.").unwrap_or("").to_string(), generation: expected_generation_of(find_set_active_utility_descriptor(utilities)) })
}

fn is_select_group(node: &UtilityNode) -> bool {
    let UtilityNode::Collection { id, label, text, .. } = node else { return false };
    id == "group:Select" || id == "group:selection" || label.as_deref() == Some("Select") || text.as_deref() == Some("Select")
}

/// 🪟️ React's `utilityBarNode` tree: the window utility bar groups by [`UTILITY_CATEGORIES`] (history and sync live elsewhere) and then unwraps each selection/utilities bucket into its children, a `Select` group included, so the bar's top level is the utilities themselves.
pub fn window_utility_bar_nodes(nodes: &[UtilityNode]) -> Vec<UtilityNode> {
    let mut bar = Vec::new();
    for node in group_utility_nodes_by_category(nodes, &UTILITY_CATEGORIES) {
        let UtilityNode::Collection { children, category: Some(UtilityCategory::Selection | UtilityCategory::Utilities), .. } = &node else {
            bar.push(node);
            continue;
        };
        if is_select_group(&node) {
            bar.extend(children.iter().cloned());
            continue;
        }
        for child in children {
            match child {
                UtilityNode::Collection { children: inner, .. } if is_select_group(child) => bar.extend(inner.iter().cloned()),
                other => bar.push(other.clone()),
            }
        }
    }
    bar
}
