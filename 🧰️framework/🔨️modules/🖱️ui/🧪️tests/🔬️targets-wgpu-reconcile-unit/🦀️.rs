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
fn an_editable_table_row_keeps_one_child_per_cell_and_its_remove_action_as_a_row_action() {
    let table: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 0,
        "key": "table",
        "children": [1],
        "component": { "type": "table", "label": "People", "columns": ["Name"], "actionsLabel": "Actions" },
        "layout": { "kind": "stack", "axis": "vertical", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "wrap": false, "grow": false },
        "style": {},
        "activity": "idle",
        "accessibility": {}
    }))
    .expect("table fixture");
    let row: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 1,
        "key": "row-7",
        "children": [2],
        "component": {
            "type": "tableRow",
            "cells": ["Ada"],
            "rowActions": [{ "icon": "trash-2", "label": "Remove row", "verb": "remove-row" }],
            "target": { "scope": "s.stdio.csv@rfc4180/*#editor", "version": 1, "args": { "revision": "0123456789abcdef", "row": 7 } }
        },
        "layout": { "kind": "stack", "axis": "horizontal", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "wrap": false, "grow": false },
        "style": {},
        "activity": "idle",
        "accessibility": {}
    }))
    .expect("table row fixture");
    let cell: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 2,
        "key": "cell-0",
        "component": { "type": "input", "kind": "text", "value": "Ada", "commit": "blur" },
        "layout": { "kind": "leaf", "width": "hug", "height": "hug" },
        "style": {},
        "activity": "idle",
        "accessibility": { "label": "Name" }
    }))
    .expect("cell fixture");
    let header = ui_contract::UiDocumentLeaseHeader { generation: 1, surface: ui_contract::SurfaceId::try_from("table.action").expect("surface"), revision: ui_contract::UiRevision(0), root: table.id, layout_epoch: 0, node_count: 3 };
    let mut document = UiDocumentTree::new(header).expect("document");
    document.try_upsert_record(table).expect("table record");
    document.try_upsert_record(row).expect("row record");
    document.try_upsert_record(cell).expect("cell record");
    {
        let row = document.record(ui_contract::UiNodeId(1)).expect("row");
        let UiNode::Stack(projected) = ui_node_from_record(&document, row, "table.action", "s.stdio.csv@rfc4180/*#editor") else { panic!("a table row mounts as its keyed identity row") };
        assert!(projected.activate.is_none(), "focusing or activating an editable row must not run its destructive trailing action");
        let table = document.record(ui_contract::UiNodeId(0)).expect("table");
        let UiNode::Tree(projected) = ui_node_from_record(&document, table, "table.action", "s.stdio.csv@rfc4180/*#editor") else { panic!("a table paints through the retained tree") };
        let [section] = projected.sections.as_slice() else { panic!("a table is ONE section") };
        assert_eq!(section.id, "table", "the section is keyed by the table's own record key");
        let [item] = section.items.as_slice() else { panic!("one row item") };
        assert_eq!((item.id.as_str(), item.label.as_str()), ("row-7", "Ada"), "a row is named by its first cell");
        assert!(item.action.is_none(), "the row action is not the row's activation");
        let Some([remove]) = item.actions.as_deref() else { panic!("the RowAction prop is the row's one trailing action") };
        assert_eq!(remove.label.as_ref().map(|label| label.as_str()), Some("Remove row"));
        assert_eq!(remove.action.action, "remove-row");
        let Some(DslValue::Object(arguments)) = &remove.action.args else { panic!("remove address is retained") };
        assert!(arguments.contains(&("revision".into(), DslValue::String("0123456789abcdef".into()))));
        assert!(arguments.contains(&("row".into(), DslValue::uint(7))));
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
    let row = tree.document_node(ui_contract::UiNodeId(1)).expect("row mounted");
    let children = tree.children(row).collect::<Vec<_>>();
    let [cell] = children.as_slice() else { panic!("an editable row keeps exactly one child per materialised cell") };
    let UiNode::Input(input) = &tree.node(*cell).expect("cell mounted").spec.0 else { panic!("the cell mounts as its own input") };
    assert_eq!(input.id, "cell-0");
}

/// 🌳️ A retained tree paints its sections in document order, including sections a later revision inserts ahead of the
/// ones already shown — the parity half of the React `mergeTreeSectionOrder` law (a program that leads with a new section,
/// like the history body's edit band, is never sunk below the sections a host already painted).
#[test]
fn a_retained_tree_paints_inserted_sections_where_the_document_puts_them() {
    fn record(id: u64, key: &str, component: serde_json::Value, children: &[u64]) -> ui_contract::UiNodeRecord {
        serde_json::from_value(serde_json::json!({
            "id": id,
            "key": key,
            "children": children,
            "component": component,
            "layout": { "kind": "stack", "axis": "vertical", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "wrap": false, "grow": false },
            "style": {},
            "activity": "idle",
            "accessibility": {}
        }))
        .expect("tree record fixture")
    }
    let section = |id: u64, key: &str| record(id, key, serde_json::json!({ "type": "treeSection", "label": key }), &[]);
    let painted = |document: &UiDocumentTree| {
        let root = document.record(ui_contract::UiNodeId(0)).expect("the tree");
        let UiNode::Tree(tree) = ui_node_from_record(document, root, "history.sections", "s.test@1/*#editor") else { panic!("a tree paints as a tree") };
        tree.sections.iter().map(|section| section.id.clone()).collect::<Vec<_>>()
    };
    let header = ui_contract::UiDocumentLeaseHeader { generation: 1, surface: ui_contract::SurfaceId::try_from("history.sections").expect("surface"), revision: ui_contract::UiRevision(0), root: ui_contract::UiNodeId(0), layout_epoch: 0, node_count: 5 };
    let mut document = UiDocumentTree::new(header).expect("document");
    document.try_upsert_record(record(0, "framework.history", serde_json::json!({ "type": "tree" }), &[1, 2])).expect("tree");
    document.try_upsert_record(section(1, "framework.history.actions")).expect("actions");
    document.try_upsert_record(section(2, "framework.history.commands")).expect("commands");
    assert_eq!(painted(&document), ["framework.history.actions", "framework.history.commands"]);
    document.try_upsert_record(section(3, "framework.history.timeTravel")).expect("band");
    document.try_upsert_record(section(4, "framework.history.editor")).expect("editor");
    document.try_upsert_record(record(0, "framework.history", serde_json::json!({ "type": "tree" }), &[3, 4, 1, 2])).expect("the reordered tree");
    assert_eq!(painted(&document), ["framework.history.timeTravel", "framework.history.editor", "framework.history.actions", "framework.history.commands"]);
}

#[test]
fn a_childless_table_row_never_implicitly_activates_its_first_row_action() {
    let row: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 0,
        "key": "legacy-row",
        "component": {
            "type": "tableRow",
            "cells": ["Ada"],
            "rowActions": [{ "icon": "trash-2", "label": "Remove row", "verb": "remove-row" }],
            "target": { "scope": "s.stdio.csv@rfc4180/*#editor", "version": 1, "args": { "row": 7, "revision": "0123456789abcdef" } }
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
    let UiNode::Stack(projected) = ui_node_from_record(&document, row, "table.childless", "s.stdio.csv@rfc4180/*#editor") else { panic!("a childless table row mounts as its keyed identity row") };
    assert!(projected.activate.is_none(), "the row action is not borrowed as an implicit destructive row activation");
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
        content_lines: None,
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
        content_lines: None,
        inline_toolbar: None,
        detail: None,
        actions: Some(vec![UiTreeItemAction { icon_id: IconName::Trash2, label: Some(Label::data("Delete")), action: action(), placement: Some(UiTreeActionPlacement::Menu), disabled: false }]),
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
