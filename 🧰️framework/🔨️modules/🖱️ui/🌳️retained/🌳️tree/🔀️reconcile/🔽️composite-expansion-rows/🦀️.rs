//! 🔽️ Composite expansion rows: the select item rows, the item value arguments, the tree section and item rows and the own field and layout change checks. See [`super`].

use super::*;

// #region reconcile
//#region 🔖️CompositeExpansion
/// 🔽️ Synthesizes one retained `Button` row per `Select` item, keyed by the item's own `value` (via
/// `explicit_id`'s `UiNode::Button` arm) — `UiSelectItem.value` is already Select's stable per-option
/// identity (it's what `UiSelectNode.value` itself holds to name the current choice), so reusing it as
/// the row's key needs no extra bookkeeping.
///
/// 🔓️ Ungated in ticket 26/09/17 packet W15a. It used to be `cfg(test, testkit)` alongside the
/// `apply_tree` reconciler that was its only caller, which meant PRODUCTION never materialized an
/// open popup's rows at all: `paint_select` (which draws the rows from the inline `items`) is itself
/// `cfg(test)`, the retained `UiNode::Select` paint arm draws only the trigger, and
/// `retained_hit_registration`'s `Select` arm mints one target for the trigger — so an open popup on
/// the live renderer painted nothing, hit nothing and projected nothing. The production minter is
/// `paint::sync_interactive_state_node_step`'s `SelectMint` phase, through
/// `UiTree::mint_composite_row`.
pub fn select_item_row(select: &UiSelectNode, item: &UiSelectItem) -> UiNode {
    UiNode::Button(UiButtonNode { id: Some(item.value.clone()), icon_id: IconName::CircleDot, label: item.label.clone(), action: with_item_value_arg(&select.on_change, &item.value), style: None, presence: UiPresence::default(), menu: None })
}

/// 🏷️ Clones `action`, merging a `"value"` key into its JSON `args` object (creating one if absent)
/// so a click on one synthesized `Select` row is distinguishable from any other row once a later
/// events milestone dispatches it — `on_change.clone()` alone would fire an identical, valueless
/// action for every row. Ungated with [`select_item_row`] (packet W15a).
fn with_item_value_arg(action: &ActionDescriptor, value: &str) -> ActionDescriptor {
    let mut merged = action.clone();
    let mut entries = match merged.args.take() {
        Some(DslValue::Object(map)) => map,
        _ => Vec::new(),
    };
    entries.push(("value".to_string(), DslValue::String(value.to_string())));
    merged.args = Some(DslValue::Object(entries));
    merged
}

/// 🌳️ Synthesizes one retained `Stack` row per `Tree` section, keyed by `section.id`, wrapping its
/// `items` (recursively expanded by `tree_item_row`) as retained children.
#[cfg(any(test, feature = "testkit"))]
pub(super) fn tree_section_row(tree_node: &UiTreeNode, section: &UiTreeSectionNode) -> UiNode {
    let mut children = Vec::new();
    if let Some(toolbar) = &section.header_toolbar {
        children.push(UiNode::Stack(toolbar.clone()));
    }
    children.extend(section.items.iter().map(tree_item_row));
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: None,
        padding: None,
        id: Some(section.id.clone()),
        presence: section.presence.clone(),
        activate: None,
        drop_action: tree_node.drop_action.clone(),
        drop_overlay: None,
        children,
        menu: None,
    })
}

/// 🌳️ Synthesizes one retained `Stack` row per `Tree` item, keyed by `item.id`. Carries the item's own
/// `presence` (already the single source of truth for selected/previewed/status — no more union with
/// a tree-level id list) and `activate` (the row's click `action`) as a `UiStackNode`'s own fields,
/// plus its embedded `control` (via `ui_control_to_node`), trailing `actions` (via
/// `tree_item_action_row`), and nested `items` (recursively) as retained children.
/// `draggable`/`drag_data` have no matching `UiStackNode` field to carry them structurally —
/// `events::is_plain_stack_container`/`find_tree_item_spec` re-derive those straight from this row's
/// key (`item.id`) against the parent `Tree` node's still-fully-intact `spec.0` (reconcile never drops
/// fields, only clones them into `WidgetSpec`). ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-
/// MECHANISM W3a: `hover_action`/`unhover_action` are deleted — hover is now framework-owned per
/// `UiTreeNode.interaction_domain`, never a per-item action.
#[cfg(any(test, feature = "testkit"))]
fn tree_item_row(item: &UiTreeItemNode) -> UiNode {
    let mut children: Vec<UiNode> = Vec::new();
    if let Some(control) = &item.control {
        children.push(ui_control_to_node(control.clone()));
    }
    if let Some(toolbar) = &item.inline_toolbar {
        children.push(UiNode::Stack(toolbar.clone()));
    }
    if let Some(detail) = &item.detail {
        children.push(UiNode::ComponentScene(detail.clone()));
    }
    for action in item.actions.iter().flatten() {
        if action.placement() == UiTreeActionPlacement::Menu {
            continue;
        }
        children.push(tree_item_action_row(action));
    }
    for nested in item.items.iter().flatten() {
        children.push(tree_item_row(nested));
    }
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: None,
        padding: None,
        id: Some(item.id.clone()),
        presence: item.presence.clone(),
        activate: item.action.clone(),
        drop_action: None,
        drop_overlay: None,
        children,
        menu: item.menu.clone(),
    })
}

/// 🌳️ Synthesizes one retained `Button` row per `UiTreeItemAction` (a `Tree` item's trailing/
/// row-placement action buttons). No stable id exists on `UiTreeItemAction` itself (unlike items/
/// sections), so this leaves `UiButtonNode.id` unset — `node_key`'s positional fallback (keyed by the
/// action's ordinal within its parent row's `actions` list) is already stable across re-renders for a
/// fixed action set, matching every other id-less synthesized/leaf child in this module.
#[cfg(any(test, feature = "testkit"))]
fn tree_item_action_row(action: &UiTreeItemAction) -> UiNode {
    let presence = UiPresence { state: if action.disabled { UiState::Disabled } else { UiState::Normal }, ..UiPresence::default() };
    UiNode::Button(UiButtonNode { id: None, icon_id: action.icon_id, label: action.label.clone().unwrap_or_else(|| Label::data("")), action: action.action.clone(), style: None, presence, menu: None })
}
//#endregion 🔖️CompositeExpansion

/// ⚖️ Whether the two nodes' *own* scalar fields (excluding nested `UiNode` children, which are
/// reconciled and dirtied independently) are equal.
#[cfg(any(test, feature = "testkit"))]
pub(super) fn own_fields_equal(previous: &UiNode, next: &UiNode) -> bool {
    match (previous, next) {
        (UiNode::Stack(p), UiNode::Stack(n)) => {
            p.direction == n.direction && p.gap == n.gap && p.padding == n.padding && p.id == n.id && p.presence == n.presence && p.activate == n.activate && p.drop_action == n.drop_action && p.children.len() == n.children.len()
        }
        (UiNode::Section(p), UiNode::Section(n)) => p.id == n.id && p.label == n.label && p.default_open == n.default_open && p.presence == n.presence && p.children.len() == n.children.len(),
        (UiNode::Field(p), UiNode::Field(n)) => p.id == n.id && p.label == n.label && p.description == n.description && p.required == n.required && p.error == n.error && p.presence == n.presence,
        _ => previous == next,
    }
}

/// 📐️ Whether the field(s) that differ between `previous` and `next` affect measurement/layout (as
/// opposed to paint-only state like `selected`/`status`/`disabled`). `presence.visible()` flipping
/// (i.e. `state` crossing into/out of `Hidden`) always counts — a hidden element occupies no layout
/// space at all, so becoming hidden/unhidden must re-run layout for its parent, unlike every other
/// `presence` change (selected/status/hover/previewed/disabled), which is paint-only.
#[cfg(any(test, feature = "testkit"))]
pub(super) fn layout_affecting_change(previous: &UiNode, next: &UiNode) -> bool {
    if previous.presence().visible() != next.presence().visible() {
        return true;
    }
    match (previous, next) {
        (UiNode::Stack(p), UiNode::Stack(n)) => p.direction != n.direction || p.gap != n.gap || p.padding != n.padding || p.children.len() != n.children.len(),
        (UiNode::Text(p), UiNode::Text(n)) => p.value != n.value,
        (UiNode::Slider(p), UiNode::Slider(n)) => p.unit_label() != n.unit_label(),
        (UiNode::Field(p), UiNode::Field(n)) => p.label != n.label || p.description != n.description || p.error != n.error,
        (UiNode::Section(p), UiNode::Section(n)) => p.label != n.label || p.children.len() != n.children.len(),
        _ => false,
    }
}
// #endregion reconcile
