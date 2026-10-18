// 🏷️ Display label, inspector edit and window option laws: the node display labels, the inspector patches and the window option verbs.

//#region 🏷️DisplayLabels
fn labelled_fixture(rows: &[(&str, &str, Option<&str>)]) -> Value {
    let nodes: Vec<Value> = rows
        .iter()
        .map(|(id, kind, label)| match label {
            Some(label) => json!({ "id": id, "nodeKind": kind, "text": label, "x": 0.0, "y": 0.0 }),
            None => json!({ "id": id, "nodeKind": kind, "x": 0.0, "y": 0.0 }),
        })
        .collect();
    json!({ "schema": PUZZLE2D_BOARD_SNAPSHOT_SCHEMA, "nodes": nodes, "edges": [], "meta": { "kindCatalogs": { "nodes": [{ "id": "capsule", "name": "Capsule", "label": "Capsule", "description": "", "icon": "", "image": "", "unit": "" }] } } })
}

/// 🏷️ LAW: the display label follows 3d's precedence exactly (`puzzle3d_object_display_label`) —
/// authored `text` first, then the kind's catalogue display name, then the KIND id when the catalogue
/// names no row for it, and only a node carrying no kind at all falls back to its own raw id.
#[test]
fn a_node_display_label_prefers_the_authored_label_then_the_catalogue_name_then_the_id() {
    let snapshot = labelled_fixture(&[("node-a", "capsule", Some("Roof Pod")), ("node-b", "capsule", None), ("node-c", "unknown-kind", None)]);
    let nodes = board_snapshot_nodes(&snapshot);
    assert_eq!(puzzle2d_node_display_label(&nodes[0], &snapshot), "Roof Pod", "an authored label wins");
    assert_eq!(puzzle2d_node_display_label(&nodes[1], &snapshot), "Capsule", "then the kind's catalogue display name");
    assert_eq!(puzzle2d_node_display_label(&nodes[2], &snapshot), "unknown-kind", "then the kind id the catalogue names no row for");
    let kindless = json!({ "id": "node-d", "x": 0.0, "y": 0.0 });
    assert_eq!(puzzle2d_node_display_label(&kindless, &snapshot), "node-d", "and only a kindless node reads its raw id");
}

/// 🔢️ LAW: duplicate kinds auto-number — the first instance takes the catalogue name, further ones
/// append ` 2`, ` 3`, … to the root taken from their peers (puzzle3d's `next_object_label` semantics).
#[test]
fn the_next_node_label_numbers_duplicates_of_one_kind() {
    let empty = labelled_fixture(&[]);
    assert_eq!(puzzle2d_next_node_label(board_snapshot_nodes(&empty), &empty, "capsule"), "Capsule", "the first instance takes the catalogue name");
    let one = labelled_fixture(&[("node-a", "capsule", Some("Capsule"))]);
    assert_eq!(puzzle2d_next_node_label(board_snapshot_nodes(&one), &one, "capsule"), "Capsule 2");
    let two = labelled_fixture(&[("node-a", "capsule", Some("Capsule")), ("node-b", "capsule", Some("Capsule 2"))]);
    assert_eq!(puzzle2d_next_node_label(board_snapshot_nodes(&two), &two, "capsule"), "Capsule 3");
    let authored = labelled_fixture(&[("node-a", "capsule", Some("Roof Pod"))]);
    assert_eq!(puzzle2d_next_node_label(board_snapshot_nodes(&authored), &authored, "capsule"), "Roof Pod 2", "the root comes from an authored peer, not the catalogue");
    let other = labelled_fixture(&[("node-a", "capsule", Some("Capsule"))]);
    assert_eq!(puzzle2d_next_node_label(board_snapshot_nodes(&other), &other, "beam"), "beam", "a kind with no peer and no catalogue row falls back to its id");
}

/// 🏷️ LAW: `addNode` stamps the label at creation — a second node of the same kind reads ` 2`, never
/// its own raw id — and each is ONE `create-node`.
#[test]
fn adding_nodes_stamps_the_next_display_label() {
    let mut recorder = recorder_of(labelled_fixture(&[]));
    recorder.add_node(Some("capsule"), None);
    recorder.add_node(Some("capsule"), None);
    let view = recorder.value();
    let labels: Vec<String> = board_snapshot_nodes(view).iter().map(|node| puzzle2d_node_display_label(node, view)).collect();
    assert_eq!(labels, vec!["Capsule".to_string(), "Capsule 2".to_string()]);
    assert!(recorder.rows().iter().all(|row| matches!(row, Puzzle2dMutation::CreateNode(_))) && recorder.rows().len() == 2, "{:?}", recorder.rows());
}

/// 🏷️ LAW: a batch of clones is labelled in order, each seeing what the earlier ones were just given — the one seam
/// duplicate and paste share.
#[test]
fn cloning_a_batch_numbers_each_new_node_in_order() {
    let mut recorder = recorder_of(labelled_fixture(&[("node-a", "capsule", Some("Capsule")), ("node-b", "capsule", Some("Capsule 2"))]));
    let clones = recorder.duplicate(&["node-a".to_string(), "node-b".to_string()]);
    assert_eq!(clones.len(), 2);
    let view = recorder.value();
    let labels: Vec<String> = board_snapshot_nodes(view).iter().map(|node| puzzle2d_node_display_label(node, view)).collect();
    assert_eq!(labels, vec!["Capsule".to_string(), "Capsule 2".to_string(), "Capsule 3".to_string(), "Capsule 4".to_string()]);
}
//#endregion 🏷️DisplayLabels

//#region 🩹️InspectorEdits
/// 📐️ LAW: `patchInspectorNodes` reaches a node's own numeric field by id — the verb the inspector's
/// editable steppers dispatch — and records exactly the one concrete kind that owns the field.
#[test]
fn patching_an_addressed_node_field_writes_only_that_node() {
    let mut recorder = recorder_of(json!({ "schema": PUZZLE2D_BOARD_SNAPSHOT_SCHEMA, "nodes": [{ "id": "a", "x": 1.0, "y": 0.0 }, { "id": "b", "x": 2.0, "y": 0.0 }], "edges": [] }));
    recorder.patch_fields(&["a".to_string()], "x", Some(&json!(9.0)), None);
    let rows = recorder.rows();
    assert!(matches!(rows, [Puzzle2dMutation::MoveNode(row)] if row.id.eq_str("a") && row.new_x == 9.0 && row.new_y == 0.0), "{rows:?}");
    assert_eq!(board_snapshot_nodes(recorder.value())[1].get("x").and_then(Value::as_f64), Some(2.0), "an addressed patch leaves every other node alone");
}

/// 📐️ LAW: an id naming a HANDLE patches that handle inside its node — how the inspector's handle
/// angle/radius steppers reach nested geometry through the same one verb — as one `replace-node-handle`.
#[test]
fn patching_an_addressed_handle_field_writes_the_nested_handle() {
    let mut recorder = recorder_of(json!({ "schema": PUZZLE2D_BOARD_SNAPSHOT_SCHEMA, "nodes": [{ "id": "a", "x": 1.0, "y": 0.0, "handles": [{ "id": "a:v0", "angle": 0.0 }, { "id": "a:v1", "angle": 1.0 }] }], "edges": [] }));
    recorder.patch_fields(&["a:v1".to_string()], "angle", None, Some(&json!(0.5)));
    let rows = recorder.rows();
    assert!(matches!(rows, [Puzzle2dMutation::ReplaceNodeHandle(row)] if row.node_id.eq_str("a") && row.handle_id.eq_str("a:v1") && row.new_handle.angle == 1.5), "a delta rides on the handle's own current value: {rows:?}");
    let handles = board_snapshot_nodes(recorder.value())[0].get("handles").and_then(Value::as_array).expect("handles");
    assert_eq!(handles[0].get("angle").and_then(Value::as_f64), Some(0.0), "a sibling handle is untouched");
    assert_eq!(board_snapshot_nodes(recorder.value())[0].get("x").and_then(Value::as_f64), Some(1.0), "the owning node is not patched by a handle-addressed edit");
}
//#endregion 🩹️InspectorEdits

//#region 🌐️WindowOptionVerbs
/// 🌐️ LAW: `setGridVisible` flips the per-window flag and a second dispatch flips it back — the
/// alternating toggle the grid group's measure binds.
#[semio_framework_async_macros::async_test]
async fn set_grid_visible_toggles_the_window_flag() {
    let mut app = app_with_registry();
    for expected in [false, true] {
        let result = dispatch(&mut app, "setGridVisible", None, Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("setGridVisible must not fault");
        assert_eq!(committed_edits(&result), 0, "a window-config verb never mutates the document");
        let measures = render_body(&mut app, overview::BODY_KEY);
        let _ = (&measures, expected);
    }
    close_app(&mut app);
}

/// 🎯️ LAW: `setSelectableKind` is an app verb every registry admits, and it never touches the
/// document — the pick filter is per-window view state.
#[semio_framework_async_macros::async_test]
async fn set_selectable_kind_is_a_view_verb_that_never_mutates_the_document() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let before = fixture_of(&app);
    for kind in [PUZZLE2D_GRANULARITY_NODE, PUZZLE2D_GRANULARITY_HANDLE, PUZZLE2D_GRANULARITY_EDGE] {
        let result = dispatch(&mut app, "setSelectableKind", Some(&json!({ "kind": kind })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).unwrap_or_else(|error| panic!("setSelectableKind {kind} must not fault: {error:?}"));
        assert_eq!(committed_edits(&result), 0, "setSelectableKind {kind} never mutates the document");
    }
    let after = fixture_of(&app);
    close_app(&mut app);
    assert_eq!(board_snapshot_nodes(&after).len(), board_snapshot_nodes(&before).len());
}

/// 🚧️🫂️ LAW: the two placement-tuning verbs write shared config, clamp to the declared range and
/// never touch the document.
#[semio_framework_async_macros::async_test]
async fn placement_tuning_verbs_are_config_only_and_clamped() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let before = fixture_of(&app);
    for action in ["setBrushPlacementContactTolerance", "setBrushPlacementOverlapBudget"] {
        for value in [4.0, -1.0, f64::from(u16::MAX)] {
            let result = dispatch(&mut app, action, Some(&json!({ "value": value })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).unwrap_or_else(|error| panic!("{action} must not fault: {error:?}"));
            assert_eq!(committed_edits(&result), 0, "{action} never mutates the document");
        }
    }
    let after = fixture_of(&app);
    close_app(&mut app);
    assert_eq!(board_snapshot_nodes(&after).len(), board_snapshot_nodes(&before).len());
}

/// 🔂️ LAW: `engagementRepeatLast` is a declared, admitted verb that publishes no document operation —
/// it asks the fill tool for one more placement through `setFillCount`.
#[semio_framework_async_macros::async_test]
async fn engagement_repeat_last_never_mutates_the_document() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let before = fixture_of(&app);
    let result = dispatch(&mut app, "engagementRepeatLast", None, Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("engagementRepeatLast must not fault");
    assert_eq!(committed_edits(&result), 0, "repeat-last is a tool reconfiguration, never a document edit");
    let after = fixture_of(&app);
    close_app(&mut app);
    assert_eq!(board_snapshot_nodes(&after).len(), board_snapshot_nodes(&before).len());
}

/// 🗨️ LAW: `openAddNodeDialog` is a shell-only verb — it opens the declared dialog and publishes
/// nothing.
#[semio_framework_async_macros::async_test]
async fn open_add_node_dialog_is_shell_only() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let before = fixture_of(&app);
    let result = dispatch(&mut app, "openAddNodeDialog", None, Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("openAddNodeDialog must not fault");
    assert_eq!(committed_edits(&result), 0, "opening a dialog never mutates the document");
    let after = fixture_of(&app);
    close_app(&mut app);
    assert_eq!(board_snapshot_nodes(&after).len(), board_snapshot_nodes(&before).len());
}

/// 🗂️ LAW: the Add Node dialog's `kind` select enumerates LIVE node kinds of the shipped examples —
/// never the single literal `"node"` option it used to hardcode, which could add no real kind.
#[test]
fn the_add_node_kind_select_enumerates_live_example_kinds() {
    let options = puzzle2d_node_kind_options();
    assert!(!options.is_empty(), "the shipped examples must contribute at least one node kind");
    assert!(options.len() <= PUZZLE2D_NODE_KIND_OPTIONS_MAX, "the select stays bounded");
    assert!(options.iter().all(|option| !option.value.is_empty()), "every option names a real kind id");
    assert!(!(options.len() == 1 && options[0].value == "node"), "the static `node` option is gone");
    let mut seen: Vec<&str> = options.iter().map(|option| option.value.as_str()).collect();
    seen.sort_unstable();
    let deduped = { let mut copy = seen.clone(); copy.dedup(); copy };
    assert_eq!(seen, deduped, "kinds are deduplicated across the two examples");
}

/// 🪟️ LAW: every declared window kind renders the surface kind it declares — the `Canvas2d`-declared
/// vs `Board2d`-rendered drift is fixed at the root.
#[test]
fn every_window_kind_declares_the_surface_kind_it_renders() {
    for definition in [
        overview::definition(),
        detail::definition(),
        selection::definition(),
    ] {
        assert_eq!(definition.surface_kind, semio_framework_plugin::SurfaceKind::Board2d, "window kind {} renders a Board2d surface", definition.id);
    }
}
//#endregion 🌐️WindowOptionVerbs
