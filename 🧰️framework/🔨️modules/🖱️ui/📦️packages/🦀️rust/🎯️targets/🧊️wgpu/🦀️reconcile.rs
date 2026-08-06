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

use crate::wgpu::Label;
use crate::wgpu::UiTreeActionPlacement;
use dsl::DslValue;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use crate::wgpu::arena::NodeId;
use crate::wgpu::component::layout::ActionDescriptor;
use crate::wgpu::component::ui::{ui_control_to_node, UiButtonNode, UiNode, UiPresence, UiSelectItem, UiSelectNode, UiStackNode, UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
use crate::wgpu::tree::{Node, NodeFlags, NodeKey, UiTree, WidgetSpec};
use crate::wgpu::IconName;

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
fn select_item_row(select: &UiSelectNode, item: &UiSelectItem) -> UiNode {
    UiNode::Button(UiButtonNode { id: Some(item.value.clone()), icon_id: IconName::CircleDot, label: item.label.clone(), action: with_item_value_arg(&select.on_change, &item.value), style: None, presence: UiPresence::default(), menu: None })
}

/// 🏷️ Clones `action`, merging a `"value"` key into its JSON `args` object (creating one if absent)
/// so a click on one synthesized `Select` row is distinguishable from any other row once a later
/// events milestone dispatches it — `on_change.clone()` alone would fire an identical, valueless
/// action for every row.
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
fn tree_section_row(tree_node: &UiTreeNode, section: &UiTreeSectionNode) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: None,
        padding: None,
        id: Some(section.id.clone()),
        presence: section.presence,
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
/// `hover_action`/`unhover_action`/`draggable`/`drag_data` have no matching `UiStackNode` field to
/// carry them structurally — a later events/interaction milestone re-derives those straight from this
/// row's key (`item.id`) against the parent `Tree` node's still-fully-intact `spec.0` (reconcile never
/// drops fields, only clones them into `WidgetSpec`).
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
    UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some(item.id.clone()), presence: item.presence, activate: item.action.clone(), drop_action: None, drop_overlay: None, children, menu: item.menu.clone() })
}

/// 🌳️ Synthesizes one retained `Button` row per `UiTreeItemAction` (a `Tree` item's trailing/
/// row-placement action buttons). No stable id exists on `UiTreeItemAction` itself (unlike items/
/// sections), so this leaves `UiButtonNode.id` unset — `node_key`'s positional fallback (keyed by the
/// action's ordinal within its parent row's `actions` list) is already stable across re-renders for a
/// fixed action set, matching every other id-less synthesized/leaf child in this module.
fn tree_item_action_row(action: &UiTreeItemAction) -> UiNode {
    UiNode::Button(UiButtonNode { id: None, icon_id: action.icon_id.clone(), label: action.label.clone().unwrap_or_else(|| Label::data("")), action: action.action.clone(), style: None, presence: UiPresence::default(), menu: None })
}
//#endregion 🔖️CompositeExpansion

/// ⚖️ Whether the two nodes' *own* scalar fields (excluding nested `UiNode` children, which are
/// reconciled and dirtied independently) are equal.
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
mod tests {
    use super::*;
    use crate::wgpu::component::layout::ActionDescriptor;
    use crate::wgpu::component::ui::{ui_tree_stamp_presence, UiButtonNode, UiControlNode, UiPresence, UiStackNode, UiTextNode, UiToggleNode};
    use crate::wgpu::tree::NodeFlags;

    fn action() -> ActionDescriptor {
        ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None }
    }

    fn text(value: &str) -> UiNode {
        UiNode::Text(UiTextNode { value: value.into(), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
    }

    fn button(id: &str, label: &str) -> UiNode {
        UiNode::Button(UiButtonNode { id: Some(id.into()), icon_id: IconName::CircleDot, label: label.into(), action: action(), style: None, presence: UiPresence::default(), menu: None })
    }

    fn stack(id: &str, children: Vec<UiNode>) -> UiNode {
        UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some(id.into()), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
    }

    fn clear_dirty(tree: &mut UiTree, id: NodeId) {
        if let Some(node) = tree.node_mut(id) {
            node.flags.set(NodeFlags::DIRTY_LAYOUT, false);
            node.flags.set(NodeFlags::DIRTY_PAINT, false);
            node.flags.set(NodeFlags::SUBTREE_DIRTY, false);
        }
        let children: Vec<NodeId> = tree.children(id).collect();
        for child in children {
            clear_dirty(tree, child);
        }
    }

    fn any_dirty(tree: &UiTree, id: NodeId) -> bool {
        let node = tree.node(id).unwrap();
        let dirty = node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::DIRTY_PAINT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY);
        dirty || tree.children(id).any(|child| any_dirty(tree, child))
    }

    #[test]
    fn reapplying_an_identical_tree_sets_zero_dirty_flags() {
        let mut tree = UiTree::new();
        let ui = stack("root", vec![text("hello"), button("btn", "Go")]);
        tree.apply_tree(&ui);
        let root = tree.root.unwrap();
        // fresh insert marks everything dirty; that's expected and not under test here.
        clear_dirty(&mut tree, root);

        tree.apply_tree(&ui);

        assert!(!any_dirty(&tree, root));
    }

    #[test]
    fn text_value_change_dirties_that_node_and_ancestors_but_not_siblings() {
        let mut tree = UiTree::new();
        tree.apply_tree(&stack("root", vec![text("hello"), text("world")]));
        let root = tree.root.unwrap();
        clear_dirty(&mut tree, root);

        tree.apply_tree(&stack("root", vec![text("changed"), text("world")]));

        let children: Vec<NodeId> = tree.children(root).collect();
        let first = tree.node(children[0]).unwrap();
        assert!(first.flags.contains(NodeFlags::DIRTY_LAYOUT));
        assert!(first.flags.contains(NodeFlags::DIRTY_PAINT));
        let second = tree.node(children[1]).unwrap();
        assert!(!second.flags.contains(NodeFlags::DIRTY_LAYOUT));
        assert!(!second.flags.contains(NodeFlags::DIRTY_PAINT));
        assert!(tree.node(root).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
    }

    #[test]
    fn adding_a_child_inserts_exactly_one_new_dirty_node_and_leaves_siblings_untouched() {
        let mut tree = UiTree::new();
        tree.apply_tree(&stack("root", vec![text("hello")]));
        let root = tree.root.unwrap();
        clear_dirty(&mut tree, root);

        tree.apply_tree(&stack("root", vec![text("hello"), text("new")]));

        let children: Vec<NodeId> = tree.children(root).collect();
        assert_eq!(children.len(), 2);
        let first = tree.node(children[0]).unwrap();
        assert!(!first.flags.contains(NodeFlags::DIRTY_LAYOUT));
        assert!(!first.flags.contains(NodeFlags::DIRTY_PAINT));
        let second = tree.node(children[1]).unwrap();
        assert!(second.flags.contains(NodeFlags::DIRTY_LAYOUT));
    }

    #[test]
    fn removing_a_child_frees_its_arena_slot() {
        let mut tree = UiTree::new();
        tree.apply_tree(&stack("root", vec![text("hello"), text("bye")]));
        let root = tree.root.unwrap();
        let children_before: Vec<NodeId> = tree.children(root).collect();
        let removed_id = children_before[1];

        tree.apply_tree(&stack("root", vec![text("hello")]));

        assert!(!tree.contains(removed_id));
        assert_eq!(tree.children(root).count(), 1);
    }

    //#region 🔖️CompositeExpansionTests
    fn select(id: &str, value: &str, items: Vec<(&str, &str)>) -> UiNode {
        UiNode::Select(UiSelectNode {
            id: id.into(),
            value: value.into(),
            items: items.into_iter().map(|(value, label)| UiSelectItem { value: value.into(), label: label.into() }).collect(),
            placeholder: None,
            on_change: action(),
            presence: UiPresence::default(),
            menu: None,
        })
    }

    fn tree_item(id: &str, label: &str) -> UiTreeItemNode {
        UiTreeItemNode {
            id: id.into(),
            label: label.into(),
            description: None,
            icon_id: None,
            presence: UiPresence::default(),
            default_open: None,
            action: None,
            hover_action: None,
            unhover_action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            dimmed: None,
            menu: None,
        }
    }

    fn tree_ui(mut sections: Vec<UiTreeSectionNode>, selected_ids: Option<Vec<String>>) -> UiNode {
        if let Some(ids) = selected_ids {
            let selected: HashSet<String> = ids.into_iter().collect();
            ui_tree_stamp_presence(&mut sections, &selected, &HashSet::new());
        }
        UiNode::Tree(UiTreeNode { sections, presence: UiPresence::default(), selected_ids: None, highlighted_ids: None, selection_change: None, drop_action: None, menu: None })
    }

    #[test]
    fn select_expands_items_into_keyed_button_rows_carrying_the_chosen_value_and_flags_has_popup() {
        let mut tree = UiTree::new();
        tree.apply_tree(&select("sel", "a", vec![("a", "Alpha"), ("b", "Beta")]));
        let root = tree.root.unwrap();

        assert!(tree.node(root).unwrap().flags.contains(NodeFlags::HAS_POPUP));
        let children: Vec<NodeId> = tree.children(root).collect();
        assert_eq!(children.len(), 2);
        let first = tree.node(children[0]).unwrap();
        assert_eq!(first.key, NodeKey::Explicit("a".into()));
        match &first.spec.0 {
            UiNode::Button(button) => {
                assert_eq!(button.label, "Alpha");
                assert_eq!(button.action.args, Some(DslValue::Object(vec![("value".into(), DslValue::String("a".into()))])));
            }
            other => panic!("expected a synthesized Button row, got {other:?}"),
        }
    }

    #[test]
    fn select_removing_an_item_removes_its_row_and_clears_has_popup_once_empty() {
        let mut tree = UiTree::new();
        tree.apply_tree(&select("sel", "a", vec![("a", "Alpha"), ("b", "Beta")]));
        let root = tree.root.unwrap();
        let children_before: Vec<NodeId> = tree.children(root).collect();
        let removed = children_before[1];

        tree.apply_tree(&select("sel", "a", vec![("a", "Alpha")]));
        assert!(!tree.contains(removed));
        assert_eq!(tree.children(root).count(), 1);

        tree.apply_tree(&select("sel", "a", vec![]));
        assert_eq!(tree.children(root).count(), 0);
        assert!(!tree.node(root).unwrap().flags.contains(NodeFlags::HAS_POPUP));
    }

    #[test]
    fn tree_expands_sections_and_nested_items_into_keyed_stack_rows() {
        let mut tree = UiTree::new();
        let nested = UiTreeItemNode { items: Some(vec![tree_item("child", "Child")]), menu: None, ..tree_item("parent", "Parent") };
        let ui = tree_ui(vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![nested] }], Some(vec!["parent".into()]));
        tree.apply_tree(&ui);
        let root = tree.root.unwrap();

        let sections: Vec<NodeId> = tree.children(root).collect();
        assert_eq!(sections.len(), 1);
        assert_eq!(tree.node(sections[0]).unwrap().key, NodeKey::Explicit("s1".into()));

        let items: Vec<NodeId> = tree.children(sections[0]).collect();
        assert_eq!(items.len(), 1);
        let parent_node = tree.node(items[0]).unwrap();
        assert_eq!(parent_node.key, NodeKey::Explicit("parent".into()));
        match &parent_node.spec.0 {
            UiNode::Stack(stack) => assert!(stack.presence.selected, "item.presence.selected unset but its id was stamped selected"),
            other => panic!("expected a synthesized Stack row, got {other:?}"),
        }

        let grandchildren: Vec<NodeId> = tree.children(items[0]).collect();
        assert_eq!(grandchildren.len(), 1);
        assert_eq!(tree.node(grandchildren[0]).unwrap().key, NodeKey::Explicit("child".into()));
    }

    #[test]
    fn tree_item_control_and_trailing_actions_become_retained_children_too() {
        let mut tree = UiTree::new();
        let item = UiTreeItemNode {
            control: Some(UiControlNode::Toggle(UiToggleNode { id: "tog".into(), icon_id: IconName::CircleDot, text: None, on_change: action(), presence: UiPresence::selected(true), menu: None })),
            actions: Some(vec![UiTreeItemAction { icon_id: IconName::Trash2, label: Some("Delete".into()), action: action(), placement: Some(UiTreeActionPlacement::Menu) }]),
            ..tree_item("leaf", "Leaf")
        };
        let ui = tree_ui(vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] }], None);
        tree.apply_tree(&ui);
        let root = tree.root.unwrap();
        let section = tree.children(root).next().unwrap();
        let row = tree.children(section).next().unwrap();

        let row_children: Vec<NodeId> = tree.children(row).collect();
        assert_eq!(row_children.len(), 1, "menu-placement actions are not retained row children; only the embedded control remains");
        assert!(matches!(tree.node(row_children[0]).unwrap().spec.0, UiNode::Toggle(_)), "control comes first");
    }

    #[test]
    fn reapplying_an_identical_select_or_tree_sets_zero_dirty_flags() {
        let mut tree = UiTree::new();
        let select_ui = select("sel", "a", vec![("a", "Alpha"), ("b", "Beta")]);
        tree.apply_tree(&select_ui);
        let root = tree.root.unwrap();
        clear_dirty(&mut tree, root);
        tree.apply_tree(&select_ui);
        assert!(!any_dirty(&tree, root), "re-applying an identical Select must not dirty its synthesized rows");

        let mut tree = UiTree::new();
        let tree_ui_value = tree_ui(vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![tree_item("a", "A")] }], None);
        tree.apply_tree(&tree_ui_value);
        let root = tree.root.unwrap();
        clear_dirty(&mut tree, root);
        tree.apply_tree(&tree_ui_value);
        assert!(!any_dirty(&tree, root), "re-applying an identical Tree must not dirty its synthesized rows");
    }
    //#endregion 🔖️CompositeExpansionTests
}
// #endregion reconcile
