use super::*;
use crate::wgpu::component::layout::ActionDescriptor;
use crate::wgpu::component::ui::{ui_tree_stamp_presence, UiButtonNode, UiControlNode, UiPresence, UiStackNode, UiTextNode, UiToggleNode};
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

#[test]
fn a_declarative_table_row_does_not_turn_its_remove_action_into_row_activation() {
    let row: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 0,
        "key": "row-7",
        "children": [1, 2],
        "component": {
            "type": "tableRow",
            "cells": ["Ada"],
            "rowActions": [{
                "icon": "trash-2",
                "label": "Remove row",
                "action": {
                    "trigger": "activate",
                    "action": { "scope": "s.stdio.csv@rfc4180/*#editor", "name": "remove-row", "version": 1 },
                    "args": { "row": 7, "revision": "0123456789abcdef" }
                }
            }]
        },
        "layout": { "kind": "stack", "axis": "horizontal", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "wrap": false, "grow": false },
        "style": {},
        "activity": "idle",
        "accessibility": {}
    }))
    .expect("table row fixture");
    let child: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 1,
        "key": "cell-0",
        "component": { "type": "input", "kind": "text", "value": "Ada", "commit": "blur" },
        "layout": { "kind": "leaf", "width": "hug", "height": "hug" },
        "style": {},
        "activity": "idle",
        "accessibility": { "label": "Name" }
    }))
    .expect("cell fixture");
    let action: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 2,
        "key": "row-action-0",
        "component": { "type": "button", "icon": "trash-2", "label": "Remove row" },
        "bindings": [{
            "trigger": "activate",
            "action": { "scope": "s.stdio.csv@rfc4180/*#editor", "name": "remove-row", "version": 1 },
            "args": { "row": 7, "revision": "0123456789abcdef" }
        }],
        "layout": { "kind": "leaf", "width": "hug", "height": "hug" },
        "style": {},
        "activity": "idle",
        "accessibility": { "label": "Remove row" }
    }))
    .expect("row action fixture");
    let header = ui_contract::UiDocumentLeaseHeader { generation: 1, surface: ui_contract::SurfaceId::try_from("table.action").expect("surface"), revision: ui_contract::UiRevision(0), root: row.id, layout_epoch: 0, node_count: 3 };
    let mut document = UiDocumentTree::new(header).expect("document");
    document.try_upsert_record(row).expect("row record");
    document.try_upsert_record(child).expect("cell record");
    document.try_upsert_record(action).expect("action record");
    {
        let row = document.record(ui_contract::UiNodeId(0)).expect("row");
        let UiNode::Stack(projected) = ui_node_from_record(&document, row, "table.action", "s.stdio.csv@rfc4180/*#editor") else { panic!("a table row with declarative cells projects as a stack") };
        assert!(projected.activate.is_none(), "focusing or activating an editable row must not run its destructive trailing action");
    }
    let mut tree = UiTree::new();
    tree.publish_document(document);
    let mut cursor = UiDocumentReconcileCursor::default();
    cursor.rearm(1);
    for _ in 0..64 {
        if matches!(tree.step_document_reconcile(&mut cursor, "table.action", "s.stdio.csv@rfc4180/*#editor"), UiDocumentReconcileStep::Complete) {
            break;
        }
    }
    let row = tree.document_node(ui_contract::UiNodeId(0)).expect("row mounted");
    let children = tree.children(row).collect::<Vec<_>>();
    assert_eq!(children.len(), 2);
    let UiNode::Button(action) = &tree.node(children[1]).expect("action mounted").spec.0 else { panic!("the row action mounts as its own button") };
    assert_eq!(action.label.as_str(), "Remove row");
    assert_eq!(action.action.action, "remove-row");
    let Some(DslValue::Object(arguments)) = &action.action.args else { panic!("remove address is retained") };
    assert!(arguments.contains(&("revision".into(), DslValue::String("0123456789abcdef".into()))));
    assert!(arguments.contains(&("row".into(), DslValue::uint(7))));
}

#[test]
fn a_childless_table_row_never_implicitly_activates_its_first_row_action() {
    let row: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 0,
        "key": "legacy-row",
        "component": {
            "type": "tableRow",
            "cells": ["Ada"],
            "rowActions": [{
                "icon": "trash-2",
                "label": "Remove row",
                "action": {
                    "trigger": "activate",
                    "action": { "scope": "s.stdio.csv@rfc4180/*#editor", "name": "remove-row", "version": 1 },
                    "args": { "row": 7, "revision": "0123456789abcdef" }
                }
            }]
        },
        "layout": { "kind": "stack", "axis": "horizontal", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "wrap": false, "grow": false },
        "style": {},
        "activity": "idle",
        "accessibility": {}
    }))
    .expect("childless row fixture");
    let header = ui_contract::UiDocumentLeaseHeader { generation: 1, surface: ui_contract::SurfaceId::try_from("table.childless").expect("surface"), revision: ui_contract::UiRevision(0), root: row.id, layout_epoch: 0, node_count: 1 };
    let mut document = UiDocumentTree::new(header).expect("document");
    document.try_upsert_record(row).expect("row record");
    let row = document.record(ui_contract::UiNodeId(0)).expect("row");
    let UiNode::Button(projected) = ui_node_from_record(&document, row, "table.childless", "s.stdio.csv@rfc4180/*#editor") else { panic!("childless row projects as its legacy button") };
    assert!(projected.action.action.is_empty(), "the row action is not borrowed as an implicit destructive row activation");
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
        window: None,
        granularity: None,
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
        inline_toolbar: None,
        detail: None,
        dimmed: None,
        menu: None,
    }
}

fn tree_ui(mut sections: Vec<UiTreeSectionNode>, selected_ids: Option<Vec<String>>) -> UiNode {
    if let Some(ids) = selected_ids {
        let selected: HashSet<String> = ids.into_iter().collect();
        ui_tree_stamp_presence(&mut sections, &selected, &HashSet::new(), None, &|_id: &str| Vec::new());
    }
    UiNode::Tree(UiTreeNode { presentation: Default::default(), sections, presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None })
}

/// 🔽️ Opens `id`'s popup the way `events::EventRouter::toggle_select_popup` does, then re-applies —
/// a CLOSED `Select` synthesizes no option rows at all, so every law about those rows is a law about
/// an OPEN one.
fn open_select(tree: &mut UiTree, ui: &UiNode) -> NodeId {
    tree.apply_tree(ui);
    let root = tree.root.unwrap();
    tree.node_mut(root).unwrap().state.open = true;
    tree.apply_tree(ui);
    root
}

#[test]
fn a_closed_select_materializes_no_option_rows_at_all() {
    let mut tree = UiTree::new();
    let ui = select("sel", "a", vec![("a", "Alpha"), ("b", "Beta")]);
    tree.apply_tree(&ui);
    let root = tree.root.unwrap();

    assert_eq!(tree.children(root).count(), 0, "React's `SelectContent` mounts only while the dropdown is open; the rows used to be built unconditionally because `WidgetState` had nowhere to record the bit");
    assert!(tree.node(root).unwrap().flags.contains(NodeFlags::HAS_POPUP), "the popup is still ANNOUNCED — only its rows wait for the open gesture");

    tree.node_mut(root).unwrap().state.open = true;
    tree.apply_tree(&ui);
    assert_eq!(tree.children(root).count(), 2, "opening it materializes exactly its items");

    tree.node_mut(root).unwrap().state.open = false;
    tree.apply_tree(&ui);
    assert_eq!(tree.children(root).count(), 0, "closing it retires them again");
}

#[test]
fn select_expands_items_into_keyed_button_rows_carrying_the_chosen_value_and_flags_has_popup() {
    let mut tree = UiTree::new();
    let root = open_select(&mut tree, &select("sel", "a", vec![("a", "Alpha"), ("b", "Beta")]));

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
    let root = open_select(&mut tree, &select("sel", "a", vec![("a", "Alpha"), ("b", "Beta")]));
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
    let nested = UiTreeItemNode { window: None, granularity: None, items: Some(vec![tree_item("child", "Child")]), menu: None, ..tree_item("parent", "Parent") };
    let ui = tree_ui(vec![UiTreeSectionNode { header_toolbar: None, window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![nested] }], Some(vec!["parent".into()]));
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
        window: None,
        granularity: None,
        control: Some(UiControlNode::Toggle(UiToggleNode { appearance: ui_contract::ToggleAppearance::Button, id: "tog".into(), icon_id: IconName::CircleDot, text: None, on_change: action(), presence: UiPresence::selected(true), menu: None })),
        inline_toolbar: None,
        detail: None,
        actions: Some(vec![UiTreeItemAction { icon_id: IconName::Trash2, label: Some(Label::data("Delete")), action: action(), placement: Some(UiTreeActionPlacement::Menu) }]),
        ..tree_item("leaf", "Leaf")
    };
    let ui = tree_ui(vec![UiTreeSectionNode { header_toolbar: None, window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] }], None);
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
    let root = open_select(&mut tree, &select_ui);
    clear_dirty(&mut tree, root);
    tree.apply_tree(&select_ui);
    assert!(!any_dirty(&tree, root), "re-applying an identical Select must not dirty its synthesized rows");

    let mut tree = UiTree::new();
    let tree_ui_value = tree_ui(vec![UiTreeSectionNode { header_toolbar: None, window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![tree_item("a", "A")] }], None);
    tree.apply_tree(&tree_ui_value);
    let root = tree.root.unwrap();
    clear_dirty(&mut tree, root);
    tree.apply_tree(&tree_ui_value);
    assert!(!any_dirty(&tree, root), "re-applying an identical Tree must not dirty its synthesized rows");
}
//#endregion 🔖️CompositeExpansionTests

#[test]
fn changing_only_a_field_error_dirties_layout_for_the_new_error_band() {
    let field = |error: Option<&str>| {
        UiNode::Field(crate::wgpu::component::ui::UiFieldNode {
            id: "field".into(),
            label: Label::data("Field"),
            description: Some("Description".into()),
            required: None,
            error: error.map(String::from),
            child: Box::new(button("field.control", "Control")),
            presence: UiPresence::default(),
            menu: None,
        })
    };
    let mut tree = UiTree::new();
    tree.apply_tree(&field(None));
    let root = tree.root.expect("field root");
    clear_dirty(&mut tree, root);

    tree.apply_tree(&field(Some("Validation error")));

    let node = tree.node(root).expect("retained field");
    assert!(node.flags.contains(NodeFlags::DIRTY_LAYOUT));
    assert!(node.flags.contains(NodeFlags::DIRTY_PAINT));
}
