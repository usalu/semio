// #region reconcile
//! 🔁️ Keyed single-pass reconciliation: applies an incoming declarative `UiNode` tree to a retained
//! `UiTree`, matching children by key, diffing matched nodes, and marking the minimal dirty flags.
//! `Stack`/`Section`/`Field` recurse into their own literal `UiNode` children; `Select`/`Tree` recurse
//! into *synthesized* retained children (see `🔖️CompositeExpansion` below) built from their
//! non-`UiNode` payload (`items`/`sections`) since there is no dedicated `UiNode` variant for "one
//! Select option row" or "one Tree row" to reuse verbatim — the remaining 14 variants have no nested
//! `UiNode`/composite payload at all, so a diffed leaf is already their complete, correct treatment.
//! KNOWN GAP (wiring request, not fixable from this region alone — see `tree::WidgetState`'s own doc
//! comment): `Select`'s synthesized option rows are always built, unconditionally, regardless of
//! open/closed — `tree::WidgetState` is currently a zero-field marker with nowhere to record "is this
//! Select open", so this region can't gate the *rows' existence* on it. `NodeFlags::HAS_POPUP` is set
//! on the `Select` node itself (whenever it has ≥1 item) so a later events/paint milestone can find
//! the always-ready rows once `WidgetState` grows an `open`-like field to gate *showing*/hit-testing
//! them — no further reconcile-side change should be needed at that point.

#[cfg(any(test, feature = "testkit"))]
use crate::wgpu::Label;
#[cfg(any(test, feature = "testkit"))]
use crate::wgpu::UiTreeActionPlacement;
#[cfg(any(test, feature = "testkit"))]
use dsl::DslValue;
#[cfg(any(test, feature = "testkit"))]
use std::borrow::Cow;
#[cfg(any(test, feature = "testkit"))]
use std::collections::{HashMap, HashSet};

#[cfg(any(test, feature = "testkit"))]
use crate::wgpu::arena::NodeId;
#[cfg(any(test, feature = "testkit"))]
use crate::wgpu::component::layout::ActionDescriptor;
#[cfg(any(test, feature = "testkit"))]
use crate::wgpu::component::ui::{ui_control_to_node, UiButtonNode, UiNode, UiPresence, UiSelectItem, UiSelectNode, UiStackNode, UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
use crate::wgpu::tree::{UiDocumentPageRejection, UiDocumentTree, UiDocumentTreeFault};
#[cfg(any(test, feature = "testkit"))]
use crate::wgpu::tree::{Node, NodeFlags, NodeKey, UiTree, WidgetSpec};
#[cfg(any(test, feature = "testkit"))]
use crate::wgpu::IconName;
use ui_contract::UiDocumentNodePage;

//#region 📄️DocumentPageReconcile
impl UiDocumentTree {
    pub fn try_push_page(&mut self, page: UiDocumentNodePage, expected_index: usize) -> Result<(), UiDocumentPageRejection> {
        let generation = page.generation();
        let revision = page.revision();
        let index = page.index();
        let fault = if generation != self.generation {
            Some(UiDocumentTreeFault::Generation)
        } else if revision != self.revision {
            Some(UiDocumentTreeFault::Revision)
        } else if index != expected_index {
            Some(UiDocumentTreeFault::PageOrder)
        } else if self.nodes.get(&page.record().id).is_some() {
            Some(UiDocumentTreeFault::DuplicateNode)
        } else {
            None
        };
        let record = page.into_record();
        if let Some(fault) = fault {
            return Err(UiDocumentPageRejection { fault, generation, revision, index, record });
        }
        self.nodes.try_insert(record).map(|_| ()).map_err(|record| UiDocumentPageRejection { fault: UiDocumentTreeFault::NodeCapacity, generation, revision, index, record })
    }
}
//#endregion 📄️DocumentPageReconcile

#[cfg(any(test, feature = "testkit"))]
fn variant_discriminant(node: &UiNode) -> u32 {
    match node {
        UiNode::Stack(_) => 0,
        UiNode::Text(_) => 1,
        UiNode::Button(_) => 2,
        UiNode::Separator(_) => 3,
        UiNode::Input(_) => 4,
        UiNode::Select(_) => 5,
        UiNode::Toggle(_) => 6,
        UiNode::KeyValue(_) => 8,
        UiNode::Slider(_) => 9,
        UiNode::NumberStepper(_) => 10,
        UiNode::Ring(_) => 11,
        UiNode::IconSelect(_) => 12,
        UiNode::Field(_) => 13,
        UiNode::Section(_) => 14,
        UiNode::Tree(_) => 15,
        UiNode::Image(_) => 16,
        UiNode::ComponentScene(_) => 17,
        UiNode::ExternalSlot(_) => 18,
        UiNode::Group(_) => 19,
    }
}

#[cfg(any(test, feature = "testkit"))]
fn explicit_id(node: &UiNode) -> Option<&str> {
    match node {
        UiNode::Stack(n) => n.id.as_deref(),
        UiNode::Button(n) => n.id.as_deref(),
        UiNode::Input(n) => Some(n.id.as_str()),
        UiNode::Select(n) => Some(n.id.as_str()),
        UiNode::Toggle(n) => Some(n.id.as_str()),
        UiNode::Slider(n) => Some(n.id.as_str()),
        UiNode::NumberStepper(n) => Some(n.id.as_str()),
        UiNode::Ring(n) => Some(n.id.as_str()),
        UiNode::IconSelect(n) => Some(n.id.as_str()),
        UiNode::Field(n) => Some(n.id.as_str()),
        UiNode::Section(n) => Some(n.id.as_str()),
        UiNode::Group(n) => Some(n.id.as_str()),
        UiNode::Image(n) => Some(n.id.as_str()),
        UiNode::ComponentScene(n) => Some(n.surface_id.as_str()),
        UiNode::ExternalSlot(n) => Some(n.body_key.as_str()),
        UiNode::Text(_) | UiNode::Separator(_) | UiNode::KeyValue(_) | UiNode::Tree(_) => None,
    }
}

#[cfg(any(test, feature = "testkit"))]
fn node_key(node: &UiNode, ordinal: u32) -> NodeKey {
    match explicit_id(node) {
        Some(id) if !id.is_empty() => NodeKey::Explicit(id.to_string()),
        _ => NodeKey::Positional(variant_discriminant(node), ordinal),
    }
}

/// 🌿️ The keyed-diffable children of `node`: `Stack`/`Section`'s own `children`, `Field`'s single
/// `child`, borrowed straight from `node` (no allocation); `Select`/`Tree`'s *synthesized* rows (see
/// `🔖️CompositeExpansion`), freshly built each call since they're derived from non-`UiNode` payload.
/// Everything else has no nested `UiNode` payload to recurse into. `presence.state == Hidden`
/// children are dropped here — hidden means not rendered at all, so they get no retained node, no
/// layout, no paint, no hit-test; this is the one choke point every caller goes through.
#[cfg(any(test, feature = "testkit"))]
fn children_of(node: &UiNode) -> Vec<Cow<'_, UiNode>> {
    let children = match node {
        UiNode::Stack(n) => n.children.iter().map(Cow::Borrowed).collect(),
        UiNode::Section(n) => n.children.iter().map(Cow::Borrowed).collect(),
        UiNode::Group(n) => n.children.iter().map(Cow::Borrowed).collect(),
        UiNode::Field(n) => vec![Cow::Borrowed(n.child.as_ref())],
        UiNode::Select(select) => select.items.iter().map(|item| Cow::Owned(select_item_row(select, item))).collect(),
        UiNode::Tree(tree_node) => tree_node.sections.iter().map(|section| Cow::Owned(tree_section_row(tree_node, section))).collect(),
        _ => Vec::new(),
    };
    children.into_iter().filter(|child: &Cow<'_, UiNode>| child.presence().visible()).collect()
}

//#region 🔖️CompositeExpansion
/// 🔽️ Synthesizes one retained `Button` row per `Select` item, keyed by the item's own `value` (via
/// `explicit_id`'s `UiNode::Button` arm) — `UiSelectItem.value` is already Select's stable per-option
/// identity (it's what `UiSelectNode.value` itself holds to name the current choice), so reusing it as
/// the row's key needs no extra bookkeeping. See this module's doc comment for the open/closed
/// `WidgetState` wiring request this groundwork is waiting on.
#[cfg(any(test, feature = "testkit"))]
fn select_item_row(select: &UiSelectNode, item: &UiSelectItem) -> UiNode {
    UiNode::Button(UiButtonNode { id: Some(item.value.clone()), icon_id: IconName::CircleDot, label: item.label.clone(), action: with_item_value_arg(&select.on_change, &item.value), style: None, presence: UiPresence::default(), menu: None })
}

/// 🏷️ Clones `action`, merging a `"value"` key into its JSON `args` object (creating one if absent)
/// so a click on one synthesized `Select` row is distinguishable from any other row once a later
/// events milestone dispatches it — `on_change.clone()` alone would fire an identical, valueless
/// action for every row.
#[cfg(any(test, feature = "testkit"))]
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
fn tree_section_row(tree_node: &UiTreeNode, section: &UiTreeSectionNode) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: None,
        padding: None,
        id: Some(section.id.clone()),
        presence: section.presence.clone(),
        activate: None,
        drop_action: tree_node.drop_action.clone(),
        drop_overlay: None,
        children: section.items.iter().map(|item| tree_item_row(tree_node, item)).collect(),
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
fn tree_item_row(tree_node: &UiTreeNode, item: &UiTreeItemNode) -> UiNode {
    let mut children: Vec<UiNode> = Vec::new();
    if let Some(control) = &item.control {
        children.push(ui_control_to_node(control.clone()));
    }
    for action in item.actions.iter().flatten() {
        if action.placement() == UiTreeActionPlacement::Menu {
            continue;
        }
        children.push(tree_item_action_row(action));
    }
    for nested in item.items.iter().flatten() {
        children.push(tree_item_row(tree_node, nested));
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
    UiNode::Button(UiButtonNode { id: None, icon_id: action.icon_id.clone(), label: action.label.clone().unwrap_or_else(|| Label::data("")), action: action.action.clone(), style: None, presence: UiPresence::default(), menu: None })
}
//#endregion 🔖️CompositeExpansion

/// ⚖️ Whether the two nodes' *own* scalar fields (excluding nested `UiNode` children, which are
/// reconciled and dirtied independently) are equal.
#[cfg(any(test, feature = "testkit"))]
fn own_fields_equal(previous: &UiNode, next: &UiNode) -> bool {
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
fn layout_affecting_change(previous: &UiNode, next: &UiNode) -> bool {
    if previous.presence().visible() != next.presence().visible() {
        return true;
    }
    match (previous, next) {
        (UiNode::Stack(p), UiNode::Stack(n)) => p.direction != n.direction || p.gap != n.gap || p.padding != n.padding || p.children.len() != n.children.len(),
        (UiNode::Text(p), UiNode::Text(n)) => p.value != n.value,
        (UiNode::Field(p), UiNode::Field(n)) => p.label != n.label || p.description != n.description,
        (UiNode::Section(p), UiNode::Section(n)) => p.label != n.label || p.children.len() != n.children.len(),
        _ => false,
    }
}

#[cfg(any(test, feature = "testkit"))]
impl UiTree {
    /// 🔁️ Applies an incoming declarative `UiNode` tree to this retained tree: keyed single-pass
    /// child matching, minimal-dirty-flag diffing of matched nodes, insertion of unmatched incoming
    /// children, removal of unmatched existing children. Re-applying an identical tree sets zero
    /// dirty flags anywhere in the tree.
    pub fn apply_tree(&mut self, incoming: &UiNode) {
        let key = node_key(incoming, 0);
        match self.root {
            Some(root_id) if self.node(root_id).map(|n| &n.key) == Some(&key) => {
                self.diff_and_update(root_id, incoming);
                self.reconcile_children(root_id, incoming);
            }
            Some(root_id) => {
                self.remove(root_id);
                self.root = None;
                self.insert_new_root(key, incoming);
            }
            None => self.insert_new_root(key, incoming),
        }
    }

    fn insert_new_root(&mut self, key: NodeKey, incoming: &UiNode) {
        let id = self.insert_child(None, Node::new(key, WidgetSpec(incoming.clone())));
        self.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
        self.root = Some(id);
        self.reconcile_children(id, incoming);
    }

    fn diff_and_update(&mut self, id: NodeId, incoming: &UiNode) {
        let (needs_layout, needs_paint) = match self.node(id) {
            Some(node) if own_fields_equal(&node.spec.0, incoming) => (false, false),
            Some(node) if layout_affecting_change(&node.spec.0, incoming) => (true, true),
            Some(_) => (false, true),
            None => return,
        };
        if let Some(node) = self.node_mut(id) {
            node.spec = WidgetSpec(incoming.clone());
        }
        if needs_layout {
            self.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
        } else if needs_paint {
            self.mark_dirty(id, NodeFlags::DIRTY_PAINT);
        }
    }

    /// 🚩️ Keeps structural `NodeFlags` that reflect `incoming`'s own shape (not its diff status) in
    /// sync — currently just `HAS_POPUP` on a `Select` with ≥1 item, so a later events/paint milestone
    /// can find "this Select has synthesized option rows ready under it" (see this module's own doc
    /// comment for the `WidgetState` open/closed wiring request that gates actually showing them)
    /// without re-deriving it from `spec.0` itself. Deliberately bypasses `mark_dirty` — direct flag
    /// mutation, no `SUBTREE_DIRTY` bubbling — since this is bookkeeping metadata, not a repaint signal.
    fn sync_composite_flags(&mut self, id: NodeId, incoming: &UiNode) {
        if let UiNode::Select(select) = incoming {
            if let Some(node) = self.node_mut(id) {
                node.flags.set(NodeFlags::HAS_POPUP, !select.items.is_empty());
            }
        }
    }

    fn reconcile_children(&mut self, parent: NodeId, incoming: &UiNode) {
        self.sync_composite_flags(parent, incoming);
        let incoming_children = children_of(incoming);
        let existing_children: Vec<NodeId> = self.children(parent).collect();

        let mut existing_by_key: HashMap<NodeKey, NodeId> = HashMap::with_capacity(existing_children.len());
        for child_id in &existing_children {
            if let Some(node) = self.node(*child_id) {
                existing_by_key.insert(node.key.clone(), *child_id);
            }
        }

        let mut used_keys: HashSet<NodeKey> = HashSet::with_capacity(incoming_children.len());
        let mut matched_ids: HashSet<NodeId> = HashSet::with_capacity(incoming_children.len());
        for (ordinal, child) in incoming_children.iter().enumerate() {
            let key = node_key(child, ordinal as u32);
            let matched_id = match existing_by_key.get(&key) {
                Some(existing_id) if !used_keys.contains(&key) => {
                    used_keys.insert(key);
                    self.diff_and_update(*existing_id, child);
                    self.reconcile_children(*existing_id, child);
                    *existing_id
                }
                _ => {
                    let id = self.insert_child(Some(parent), Node::new(key, WidgetSpec(child.clone().into_owned())));
                    self.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
                    self.reconcile_children(id, child);
                    id
                }
            };
            matched_ids.insert(matched_id);
        }

        for existing_id in existing_children {
            if !matched_ids.contains(&existing_id) {
                self.remove(existing_id);
            }
        }
    }
}

#[cfg(test)]
#[path = "../../../../🧪️tests/🔬️targets-wgpu-reconcile-unit/🦀️.rs"]
mod tests;
// #endregion reconcile
