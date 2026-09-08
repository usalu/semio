
use super::*;
use crate::wgpu::component::layout::ActionDescriptor;
use crate::wgpu::component::ui::{UiButtonNode, UiControlNode, UiPresence, UiStackNode, UiTextNode, UiToggleNode, ui_tree_stamp_presence};
use crate::wgpu::tree::NodeFlags;

fn action() -> ActionDescriptor {
    ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None }
}

fn text(value: &str) -> UiNode {
    UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
}

fn button(id: &str, label: &str) -> UiNode {
    UiNode::Button(UiButtonNode { id: Some(id.into()), icon_id: IconName::CircleDot, label: Label::data(label), action: action(), style: None, presence: UiPresence::default(), menu: None })
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
        items: items.into_iter().map(|(value, label)| UiSelectItem { value: value.into(), label: Label::data(label) }).collect(),
        placeholder: None,
        on_change: action(),
        presence: UiPresence::default(),
        menu: None,
    })
}

fn tree_item(id: &str, label: &str) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: Label::data(label),
        description: None,
        icon_id: None,
        presence: UiPresence::default(),
        default_open: None,
        action: None,
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
        ui_tree_stamp_presence(&mut sections, &selected, &HashSet::new(), None, &|_id: &str| Vec::new());
    }
    UiNode::Tree(UiTreeNode { sections, presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None })
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
            assert_eq!(button.label.as_str(), "Alpha");
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
        actions: Some(vec![UiTreeItemAction { icon_id: IconName::Trash2, label: Some(Label::data("Delete")), action: action(), placement: Some(UiTreeActionPlacement::Menu) }]),
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
