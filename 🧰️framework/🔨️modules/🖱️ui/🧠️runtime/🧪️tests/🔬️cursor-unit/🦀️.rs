use super::*;
use crate::testkit::*;

#[cfg(test)]
#[path = "../../📏️ownership/🧪️tests/📏️ownership/🦀️.rs"]
mod ownership;

#[cfg(test)]
#[path = "../../📃️document/🧪️tests/📃️document/🦀️.rs"]
mod canonical_document_tests;

//#region ⏭️ResumableCursor
#[test]
fn abandoned_large_tree_cursor_leaves_the_retained_shadow_and_revision_unchanged() {
    let _guard = registry_guard();
    let mut current = SurfaceShadow::from_text("s");
    current.reconcile(&tree(container("root", vec![leaf("baseline")]))).expect("baseline");
    let before = current.snapshot();
    let children = (0..30).map(|index| leaf(&format!("item-{index}"))).collect();
    let mut cursor = SurfaceReconcileCursor::new(tree(container("root", children)), &current);

    assert!(matches!(cursor.step(&current), SurfaceReconcileStep::Yield { .. }));
    drop(cursor);

    assert_eq!(current.snapshot(), before, "cancellation or supersession must discard only candidate state");
}
#[test]
fn semantic_aggregate_quota_faults_before_key_or_record_clone() {
    let _guard = registry_guard();
    let mut data_attributes = ui_contract::UiFixedMap::default();
    data_attributes.try_push(ui_text("semantic"), ui_text("payload")).expect("bounded fixture attribute");
    let node = crate::TreeNode::try_new("exact", ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label(ui_text("value")), emphasize: None, data_attributes: Some(data_attributes) })).expect("bounded fixture node");
    let current = SurfaceShadow::from_text("s");
    let limits = SurfaceReconcileLimits { max_bytes: SURFACE_RECONCILE_PAGE_BYTES, ..Default::default() };
    let mut cursor = SurfaceReconcileCursor::new_with_limits(tree(node), &current, limits);
    let mut fault = None;
    for _ in 0..4_096 {
        if let SurfaceReconcileStep::Fault(found) = cursor.step(&current) {
            fault = Some(found);
            break;
        }
    }
    assert!(matches!(fault, Some(SurfaceReconcileFault::Credits { .. })));
    let retained = cursor.held_node.as_ref().expect("exact unmaterialized node remains retained");
    assert_eq!(retained.1.key.as_str(), "exact");
    assert!(cursor.flat.is_empty());
    assert!(cursor.seen.is_empty());
    assert!(cursor.new_ordinals.is_empty());
    assert!(cursor.ops.is_empty());
}
#[test]
fn semantic_census_low_fuel_wide_container_and_deep_value_advance_one_unit_without_recursion() {
    let _guard = registry_guard();
    let wide = (0..128).map(|index| ui_contract::UiValue::Text(ui_contract::UiText::try_from_string(format!("value-{index}")).expect("bounded fixture text"))).collect::<Vec<_>>();
    let node = crate::TreeNode::try_new("wide", ui_contract::Component::Extension(ui_contract::ExtensionProps { extension: ui_text("fixture"), props: ui_contract::UiValue::List(ui_list(wide)) })).expect("bounded fixture node");
    let current = SurfaceShadow::from_text("s");
    let mut cursor = SurfaceReconcileCursor::new(tree(node), &current);
    for _ in 0..32 {
        assert!(matches!(cursor.step(&current), SurfaceReconcileStep::Yield { .. }));
    }
    assert!(cursor.flat.is_empty(), "a wide value cannot complete census in one grant");

    let mut deep = ui_contract::UiValue::Null;
    for _ in 0..=SURFACE_RECONCILE_VALUE_DEPTH {
        deep = ui_contract::UiValue::List(ui_list([deep]));
    }
    let node = crate::TreeNode::try_new("deep", ui_contract::Component::Extension(ui_contract::ExtensionProps { extension: ui_text("fixture"), props: deep })).expect("bounded fixture node");
    let mut cursor = SurfaceReconcileCursor::new(tree(node), &current);
    let mut fault = None;
    for _ in 0..8_192 {
        if let SurfaceReconcileStep::Fault(found) = cursor.step(&current) {
            fault = Some(found);
            break;
        }
    }
    assert!(matches!(fault, Some(SurfaceReconcileFault::ValueDepth { .. })));
    assert!(cursor.flat.is_empty());
}
//#endregion ⏭️ResumableCursor
//#region 🔖️FirstReconcileAndIdempotence
#[test]
fn first_reconcile_emits_set_root_and_one_upsert_per_node_then_is_idempotent() {
    let _guard = registry_guard();
    let mut reconciler = SurfaceShadow::from_text("s");
    let component_tree = tree(container("root", vec![leaf("a"), leaf("b")]));

    let patch = reconciler.reconcile(&component_tree).expect("first reconcile must emit a patch");
    assert_eq!(patch.base_revision, ui_contract::UiRevision(0));
    assert_eq!(patch.revision, ui_contract::UiRevision(1));
    assert_eq!(patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::Upsert(_))).count(), 3);
    assert_eq!(patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::SetRoot { .. })).count(), 1);

    assert!(reconciler.reconcile(&component_tree).is_none(), "an unchanged tree must emit no patch");
}
//#endregion 🔖️FirstReconcileAndIdempotence
//#region 🔖️TargetedOps
#[test]
fn changing_one_leaf_text_emits_exactly_one_op_naming_exactly_that_node() {
    let _guard = registry_guard();
    let mut reconciler = SurfaceShadow::from_text("s");
    reconciler.reconcile(&tree(container("root", vec![text("a", "hello"), leaf("b")]))).unwrap();
    let target_id = id_of(&reconciler.snapshot(), "a");

    let patch = reconciler.reconcile(&tree(container("root", vec![text("a", "world"), leaf("b")]))).expect("a changed leaf must emit a patch");
    assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
    match &patch.ops[0] {
        ui_contract::UiPatchOp::SetComponent { id, component } => {
            assert_eq!(*id, target_id);
            assert_eq!(component, &ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::try_from("world").expect("bounded fixture label"), emphasize: None, data_attributes: None }));
        }
        other => panic!("expected SetComponent (not Upsert), got {other:?}"),
    }
}
#[test]
fn reordering_siblings_preserves_every_id_and_emits_only_set_children() {
    let _guard = registry_guard();
    let mut reconciler = SurfaceShadow::from_text("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a"), leaf("b"), leaf("c")]))).unwrap();
    let before = reconciler.snapshot();
    let (a_id, b_id, c_id) = (id_of(&before, "a"), id_of(&before, "b"), id_of(&before, "c"));

    let patch = reconciler.reconcile(&tree(container("root", vec![leaf("c"), leaf("a"), leaf("b")]))).expect("a reorder must emit a patch");
    assert_eq!(patch.ops.len(), 1);
    match &patch.ops[0] {
        ui_contract::UiPatchOp::SetChildren { children, .. } => assert_eq!(children.iter().copied().collect::<Vec<_>>(), vec![c_id, a_id, b_id]),
        other => panic!("expected SetChildren, got {other:?}"),
    }

    let after = reconciler.snapshot();
    assert_eq!(id_of(&after, "a"), a_id);
    assert_eq!(id_of(&after, "b"), b_id);
    assert_eq!(id_of(&after, "c"), c_id);
}
#[test]
fn inserting_a_middle_sibling_preserves_the_others_ids() {
    let _guard = registry_guard();
    let mut reconciler = SurfaceShadow::from_text("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a"), leaf("c")]))).unwrap();
    let before = reconciler.snapshot();
    let (a_id, c_id) = (id_of(&before, "a"), id_of(&before, "c"));

    let patch = reconciler.reconcile(&tree(container("root", vec![leaf("a"), leaf("b"), leaf("c")]))).expect("an insertion must emit a patch");
    assert!(patch.ops.iter().any(|op| matches!(op, ui_contract::UiPatchOp::Upsert(record) if record.key.as_str() == "b")));

    let after = reconciler.snapshot();
    assert_eq!(id_of(&after, "a"), a_id);
    assert_eq!(id_of(&after, "c"), c_id);
}
#[test]
fn changed_component_with_unchanged_layout_emits_set_component_not_upsert() {
    let _guard = registry_guard();
    let mut reconciler = SurfaceShadow::from_text("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();

    let patch = reconciler.reconcile(&tree(container("root", vec![text("a", "now text")]))).expect("a component change must emit a patch");
    assert_eq!(patch.ops.len(), 1);
    assert!(matches!(patch.ops[0], ui_contract::UiPatchOp::SetComponent { .. }), "expected SetComponent, got {:?}", patch.ops[0]);
}
/// 🎨️ The finding this packet exists to fix: a style-only change on a leaf with everything else
/// unchanged must emit exactly one `SetStyle`, never a whole-node `Upsert`.
#[test]
fn changing_only_style_emits_exactly_one_set_style_not_upsert() {
    let _guard = registry_guard();
    let mut reconciler = SurfaceShadow::from_text("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
    let target_id = id_of(&reconciler.snapshot(), "a");

    let patch = reconciler.reconcile(&tree(container("root", vec![styled(leaf("a"), ui_contract::Tone::Danger)]))).expect("a style change must emit a patch");
    assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
    match &patch.ops[0] {
        ui_contract::UiPatchOp::SetStyle { id, style } => {
            assert_eq!(*id, target_id);
            assert_eq!(style.tone, ui_contract::Tone::Danger);
        }
        other => panic!("expected SetStyle (not Upsert), got {other:?}"),
    }
}
#[test]
fn changing_only_accessibility_emits_exactly_one_set_accessibility_not_upsert() {
    let _guard = registry_guard();
    let mut reconciler = SurfaceShadow::from_text("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
    let target_id = id_of(&reconciler.snapshot(), "a");

    let patch = reconciler.reconcile(&tree(container("root", vec![with_shortcut(leaf("a"), "Ctrl+S")]))).expect("an accessibility change must emit a patch");
    assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
    match &patch.ops[0] {
        ui_contract::UiPatchOp::SetAccessibility { id, accessibility } => {
            assert_eq!(*id, target_id);
            assert_eq!(accessibility.shortcut.as_deref(), Some("Ctrl+S"));
        }
        other => panic!("expected SetAccessibility (not Upsert), got {other:?}"),
    }
}
#[test]
fn changing_only_bindings_emits_exactly_one_set_bindings_not_upsert() {
    let _guard = registry_guard();
    let mut reconciler = SurfaceShadow::from_text("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
    let target_id = id_of(&reconciler.snapshot(), "a");

    let patch = reconciler.reconcile(&tree(container("root", vec![with_binding(leaf("a"), "scope", "name")]))).expect("a bindings change must emit a patch");
    assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
    match &patch.ops[0] {
        ui_contract::UiPatchOp::SetBindings { id, bindings } => {
            assert_eq!(*id, target_id);
            assert_eq!(bindings.len(), 1);
        }
        other => panic!("expected SetBindings (not Upsert), got {other:?}"),
    }
}
#[test]
fn changing_only_menu_emits_exactly_one_set_menu_not_upsert() {
    let _guard = registry_guard();
    let mut reconciler = SurfaceShadow::from_text("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
    let target_id = id_of(&reconciler.snapshot(), "a");

    let patch = reconciler.reconcile(&tree(container("root", vec![with_menu(leaf("a"), "menu")]))).expect("a menu change must emit a patch");
    assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
    match &patch.ops[0] {
        ui_contract::UiPatchOp::SetMenu { id, menu } => {
            assert_eq!(*id, target_id);
            assert_eq!(menu.as_ref().map(|menu| menu.id.as_str()), Some("menu"));
        }
        other => panic!("expected SetMenu (not Upsert), got {other:?}"),
    }
}
/// 💰️ Once several groups change at once, [`SurfaceShadow::estimate_bytes`] weighs a single
/// `Upsert` against the pile of targeted ops it would replace — here five groups change on a leaf
/// whose new component/accessibility still carry no real text, so the targeted ops' fixed per-op
/// overhead alone outweighs one full-record `Upsert`, and `Upsert` wins.
#[test]
fn changing_several_groups_at_once_prefers_a_single_upsert_over_many_targeted_ops() {
    let _guard = registry_guard();
    let mut reconciler = SurfaceShadow::from_text("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
    let target_id = id_of(&reconciler.snapshot(), "a");

    let mut changed =
        crate::TreeNode::try_new("a", ui_contract::Component::Container(ui_contract::ContainerProps { role: ui_contract::ContainerRole::Plain, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }))
            .expect("bounded fixture node");
    changed.style = ui_contract::StyleSpec { tone: ui_contract::Tone::Danger, ..Default::default() };
    changed.activity = ui_contract::Activity::Loading;
    changed.disabled = true;
    changed.accessibility = ui_contract::AccessibilitySpec { hidden: true, ..Default::default() };

    let patch = reconciler.reconcile(&tree(container("root", vec![changed]))).expect("a multi-group change must emit a patch");
    assert_eq!(patch.ops.len(), 1, "expected one Upsert to beat several targeted ops, got {:?}", patch.ops);
    match &patch.ops[0] {
        ui_contract::UiPatchOp::Upsert(record) => assert_eq!(record.id, target_id),
        other => panic!("expected Upsert, got {other:?}"),
    }
}
//#endregion 🔖️TargetedOps
//#region 🔖️Removal
#[test]
fn removing_a_subtree_emits_one_remove_and_leaves_no_orphan_in_retained() {
    let _guard = registry_guard();
    let mut reconciler = SurfaceShadow::from_text("s");
    reconciler.reconcile(&tree(container("root", vec![container("mid", vec![leaf("x"), leaf("y")]), leaf("z")]))).unwrap();
    let mid_id = id_of(&reconciler.snapshot(), "mid");

    let patch = reconciler.reconcile(&tree(container("root", vec![leaf("z")]))).expect("a removal must emit a patch");
    let removes: Vec<_> = patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::Remove { .. })).collect();
    assert_eq!(removes.len(), 1);
    assert!(matches!(removes[0], ui_contract::UiPatchOp::Remove { id } if *id == mid_id));

    let after = reconciler.snapshot();
    assert!(!after.nodes.iter().any(|record| matches!(record.key.as_str(), "mid" | "x" | "y")), "removed subtree must leave no orphan");
    assert_eq!(after.nodes.len(), 2, "only root and z should remain");
}
#[test]
fn ids_are_never_reused_after_removal() {
    let _guard = registry_guard();
    let mut reconciler = SurfaceShadow::from_text("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
    let removed_id = id_of(&reconciler.snapshot(), "a");

    reconciler.reconcile(&tree(container("root", vec![]))).unwrap();
    reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
    let reinserted_id = id_of(&reconciler.snapshot(), "a");

    assert_ne!(reinserted_id, removed_id, "a fresh node at a previously-used key must never reuse a removed id");
}
//#endregion 🔖️Removal
//#region 🔖️Rejection
#[test]
fn mark_rejected_then_reconcile_emits_a_full_resend() {
    let _guard = registry_guard();
    let mut reconciler = SurfaceShadow::from_text("s");
    let component_tree = tree(container("root", vec![leaf("a"), leaf("b")]));
    reconciler.reconcile(&component_tree).unwrap();
    assert!(reconciler.reconcile(&component_tree).is_none());

    reconciler.mark_rejected();
    let patch = reconciler.reconcile(&component_tree).expect("resend after rejection must emit a patch");
    assert_eq!(patch.base_revision, ui_contract::UiRevision(0));
    assert_eq!(patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::Upsert(_))).count(), 3);
    assert!(patch.ops.iter().any(|op| matches!(op, ui_contract::UiPatchOp::SetRoot { .. })));
}
//#endregion 🔖️Rejection
//#region 🔖️DuplicateKeys
#[test]
fn duplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed() {
    let _guard = registry_guard();
    let mut reconciler = SurfaceShadow::from_text("s");
    let mut children = ui_contract::BuiltChildren::default();
    children.try_push(leaf("a")).expect("bounded fixture child");
    children.try_push(leaf("a")).expect("bounded fixture child");
    let root = crate::TreeNode { children, ..leaf("root") };
    let component_tree = crate::ComponentTree::new(root);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reconciler.reconcile(&component_tree)));
    assert!(result.is_err(), "a duplicate sibling key must panic, not silently shadow");
}
//#endregion 🔖️DuplicateKeys
//#region 🪟️TreeWindowReconcileSizeLaw
/// 🪟️ Sections one virtualised panel tree declares (objects / references / target volumes /
/// attractions — the puzzle3d artifact panel whose stale `PANEL_RECONCILE_NODE_BUDGET = 16` this law
/// replaces).
const TREE_WINDOW_LAW_SECTIONS: usize = 4;
/// 📑️ Rows one section's `TreeWindow.total` declares: one full built child list
/// ([`ui_contract::UI_BUILT_CHILDREN_MAX`]).
const TREE_WINDOW_LAW_SECTION_TOTAL: usize = ui_contract::UI_BUILT_CHILDREN_MAX;
/// 📄️ The per-section slice one presented document could carry if the RECORD arena were the only
/// bound: a surface document is [`ui_contract::UI_DOCUMENT_NODES`] records wide IN TOTAL
/// (`UiSnapshotNodes`), and the tree root plus one record per section are spent before a single row
/// is materialised, so `(128 − 1 − 4) / 4 = 30` — never the 128 a `BuiltChildren` list can hold. The
/// law below starts here and walks DOWN until admission actually accepts the page.
const TREE_WINDOW_LAW_PAGE_ROWS: usize = (ui_contract::UI_DOCUMENT_NODES - 1 - TREE_WINDOW_LAW_SECTIONS) / TREE_WINDOW_LAW_SECTIONS;
/// 🕹️ Rows per section that carry their own target — a four-entry argument map and an activation verb —
/// mirroring an app row that still owns an action (catalogue `addWidget`, workshop install …) rather
/// than picking through the tree-level interaction domain. Pick rows cost no arena at all.
const TREE_WINDOW_LAW_BOUND_ROWS: usize = 31;
fn tree_window_law_row(section: usize, index: usize) -> crate::TreeNode {
    let key = format!("s{section}.row-{index}");
    let props = ui_contract::TreeItemProps {
        label: ui_contract::Label::try_from(key.as_str()).expect("bounded fixture label"),
        description: None,
        icon: Some(ui_text("layers")),
        default_open: None,
        draggable: None,
        drag_data: None,
        dimmed: None,
        selected: None,
        window: None,
        granularity: Some(ui_text("object")),
        inline_toolbar: None,
        detail: None,
        row_actions: ui_contract::UiFixedList::default(),
        target: (index < TREE_WINDOW_LAW_BOUND_ROWS)
            .then(|| tree_window_law_args(&key))
            .flatten()
            .map(|args| ui_contract::RowTarget { scope: ui_text("puzzle"), version: 1, args: Some(args), activation: Some(ui_text("addWidget")) }),
    };
    crate::TreeNode::try_new(&key, ui_contract::Component::TreeItem(props)).expect("bounded fixture row")
}
/// 🌱️ A four-entry argument map, or `None` when the shared `UiValue` arena refuses one — a refused
/// row shortens the window, it never faults, so this law measures whatever admission actually allowed.
fn tree_window_law_args(key: &str) -> Option<ui_contract::UiMap> {
    let mut builder = ui_contract::UiMapBuilder::try_new()?;
    for (name, value) in [("id", key), ("kind", "assembly"), ("mode", "insert"), ("origin", "panel")] {
        builder.push(name.to_string(), ui_contract::UiValue::Text(ui_contract::UiText::try_from_str(value)?)).ok()?;
    }
    Some(builder.finish())
}
/// 🌳️ One presented document of the paged tree: every section stamps its FULL `total`, materialises
/// only `[offset, offset + rows)`, and never mints a continuation row.
fn tree_window_law_document(offset: usize, rows_per_section: usize) -> crate::ComponentTree {
    let mut sections = Vec::with_capacity(TREE_WINDOW_LAW_SECTIONS);
    for section in 0..TREE_WINDOW_LAW_SECTIONS {
        let rows: Vec<crate::TreeNode> = (offset..TREE_WINDOW_LAW_SECTION_TOTAL.min(offset + rows_per_section)).map(|index| tree_window_law_row(section, index)).collect();
        let props = ui_contract::TreeSectionProps {
            header_toolbar: None,
            label: Some(ui_contract::Label::try_from(format!("Section {section}").as_str()).expect("bounded fixture label")),
            default_open: Some(true),
            window: Some(ui_contract::TreeWindow { row_extent: Default::default(), total: u32::try_from(TREE_WINDOW_LAW_SECTION_TOTAL).expect("bounded fixture total"), offset: u32::try_from(offset).expect("bounded fixture offset") }),
        };
        let node = crate::TreeNode::try_new(format!("section-{section}"), ui_contract::Component::TreeSection(props)).expect("bounded fixture section");
        sections.push(node.try_with_children(rows).unwrap_or_else(|_| panic!("bounded fixture section children")));
    }
    let root = crate::TreeNode::try_new("artifact-tree", ui_contract::Component::Tree(ui_contract::TreeProps { presentation: Default::default(), interaction_domain: Some(ui_text("object")) })).expect("bounded fixture tree");
    tree(root.try_with_children(sections).unwrap_or_else(|_| panic!("bounded fixture tree children")))
}
/// 📊️ One Home-shaped table row: six cells, five row actions and a row activation, each binding carrying its own
/// one-entry `spaceId` argument map — the heaviest row an app ships today.
fn census_law_table_row(index: usize) -> crate::TreeNode {
    census_law_table_row_shaped(index, true)
}
/// 🕯️ A light table row: two cells and one row action on an argument-free target, no row activation.
fn census_law_light_table_row(index: usize) -> crate::TreeNode {
    census_law_table_row_shaped(index, false)
}
fn census_law_table_row_shaped(index: usize, heavy: bool) -> crate::TreeNode {
    let key = format!("space:{index:04}");
    let args = heavy.then(|| {
        let mut args = ui_contract::UiMapBuilder::try_new().expect("bounded fixture args");
        args.push("spaceId".to_string(), ui_contract::UiValue::Text(ui_text(&key))).expect("bounded fixture arg");
        args.finish()
    });
    let name = format!("Studio {index}");
    let all_cells = [name.as_str(), "Atelier", "Private", "1", "2026-09-25 23:05", "Hub"];
    let mut cells = ui_contract::UiFixedList::default();
    for cell in if heavy { &all_cells[..] } else { &all_cells[..2] } {
        cells.try_push(ui_text(cell)).expect("bounded fixture cell");
    }
    let all_actions = [("folder-open", "openSpace"), ("pencil", "renameSpace"), ("link", "shareSpace"), ("trash-2", "deleteSpace"), ("users", "manageSpace")];
    let mut row_actions = ui_contract::UiFixedList::default();
    for (icon, name) in if heavy { &all_actions[..] } else { &all_actions[..1] } {
        row_actions.try_push(ui_contract::RowAction { icon: ui_text(icon), label: Some(ui_contract::Label::try_from(*name).expect("bounded fixture label")), verb: ui_text(name), placement: Default::default(), disabled: false, reason: None }).expect("bounded fixture row action");
    }
    let target = ui_contract::RowTarget { scope: ui_text("s.space.home"), version: 1, args, activation: heavy.then(|| ui_text("openSpace")) };
    crate::TreeNode::try_new(&key, ui_contract::Component::TableRow(ui_contract::TableRowProps { cells, row_actions, target: Some(target) })).expect("bounded fixture row")
}
fn census_law_table_document(rows: usize, row: fn(usize) -> crate::TreeNode) -> crate::ComponentTree {
    let mut columns = ui_contract::UiFixedList::default();
    for column in ["Name", "Kind", "Visibility", "Members", "Updated", "Origin"] {
        columns.try_push(ui_contract::Label::try_from(column).expect("bounded fixture column")).expect("bounded fixture columns");
    }
    let props = ui_contract::TableProps {
        label: ui_contract::Label::try_from("Studios").expect("bounded fixture label"),
        columns,
        row_label: Some(ui_contract::Label::try_from("Row").expect("bounded fixture row label")),
        column_label: Some(ui_contract::Label::try_from("Column").expect("bounded fixture column label")),
        actions_label: Some(ui_contract::Label::try_from("Actions").expect("bounded fixture label")),
        window: Some(ui_contract::TreeWindow { row_extent: Default::default(), total: 500, offset: 0 }),
        column_window: Some(ui_contract::TreeWindow { row_extent: Default::default(), total: 6, offset: 0 }),
    };
    let table = crate::TreeNode::try_new("framework.window.table", ui_contract::Component::Table(props)).expect("bounded fixture table");
    tree(table.try_with_children((0..rows).map(row).collect::<Vec<_>>()).unwrap_or_else(|_| panic!("bounded fixture table rows")))
}
//#endregion 🪟️TreeWindowReconcileSizeLaw
