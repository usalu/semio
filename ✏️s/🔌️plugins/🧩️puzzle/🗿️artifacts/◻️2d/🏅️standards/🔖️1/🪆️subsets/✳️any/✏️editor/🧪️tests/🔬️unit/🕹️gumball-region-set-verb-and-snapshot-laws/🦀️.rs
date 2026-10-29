// 🕹️ Gumball, region, set verb and snapshot laws: the transform gumball, the board region events, the idempotent set verbs, the shipped kind catalog and the initial snapshot cost and import.

//#region 🕹️TransformGumball
/// 🔍️ One snapshot node by id, for the rotate reducer's laws.
fn transform_law_node<'a>(snapshot: &'a Value, id: &str) -> &'a Value {
    board_snapshot_nodes(snapshot).into_iter().find(|node| node.get("id").and_then(Value::as_str) == Some(id)).expect("snapshot node")
}

/// 🕹️ Reads the `(move, rotate)` pair out of a rendered board surface's `transformFlags` carrier.
fn rendered_transform_flags(scene_json: &str) -> (bool, bool) {
    let rendered: Value = semio_framework_pack_json::from_json_str(scene_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("rendered board surface parses");
    let encoded = rendered.get("board2d").and_then(|board| board.get("transformFlags")).and_then(Value::as_str).expect("the board scene declares transformFlags");
    let flags: Value = semio_framework_pack_json::from_json_str(encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("transformFlags is a JSON object");
    (flags.get("move").and_then(Value::as_bool).expect("move flag"), flags.get("rotate").and_then(Value::as_bool).expect("rotate flag"))
}

/// 🕹️ LAW: `setTransformGumballFlag` composes the select utility's gumball handles per pane and emits
/// no document operation — it is WindowConfig lane state, exactly like `setGridSnapEnabled`.
#[semio_framework_async_macros::async_test]
async fn set_transform_gumball_flag_composes_the_handles_without_touching_the_document() {
    let mut app = app();
    let result = dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "rotate", "pressed": false })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("rotate off");
    assert_eq!(committed_edits(&result), 0, "a gumball flag is window config, never a document operation");
    assert_eq!(rendered_transform_flags(&render_body(&mut app, overview::BODY_KEY)), (true, false), "the board scene must carry the composed flags");
    let result = dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "rotate", "pressed": true })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("rotate on");
    assert_eq!(committed_edits(&result), 0);
    assert_eq!(rendered_transform_flags(&render_body(&mut app, overview::BODY_KEY)), (true, true), "and turn it back on");
    let unknown = dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "scale", "pressed": true })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("unknown flag");
    assert_eq!(committed_edits(&unknown), 0, "scale is deliberately absent — an unknown flag is a no-op, not a new handle");
    close_app(&mut app);
}

/// 🔄️ LAW: one ring `rotate` gesture record is ONE history edit that turns its targets about the pivot the ring
/// drew — the `rotate-selection` leaf the select tool yields, so the in-canvas preview and the committed document
/// agree. A zero-angle or target-less record commits nothing.
#[semio_framework_async_macros::async_test]
async fn a_rotate_gesture_record_commits_one_rotate_selection_edit() {
    let mut app = concrete_forest_app();
    let node_id = first_node_id(&app);
    let before = fixture_of(&app);
    let node_before = board_snapshot_nodes(&before).iter().find(|node| node.get("id").and_then(Value::as_str) == Some(node_id.as_str())).cloned().expect("node before");
    let (x0, y0) = (node_before.get("x").and_then(Value::as_f64).expect("x"), node_before.get("y").and_then(Value::as_f64).expect("y"));
    let record = |angle: f64, targets: Value| json!([{ "name": "gesture", "payload": { "gestureId": "gesture-1", "kind": "rotate", "targets": targets, "pivotX": x0, "pivotY": y0 + 10.0, "angle": angle, "proximity": [] } }]).to_string();
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": record(std::f64::consts::PI, json!([node_id.clone()])) })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("rotate record");
    assert_eq!(committed_edits(&result), 1, "a rotate record is one document edit");
    let after = fixture_of(&app);
    let node_after = board_snapshot_nodes(&after).iter().find(|node| node.get("id").and_then(Value::as_str) == Some(node_id.as_str())).cloned().expect("node after");
    let (x1, y1) = (node_after.get("x").and_then(Value::as_f64).expect("x"), node_after.get("y").and_then(Value::as_f64).expect("y"));
    assert!((x1 - x0).abs() < 1e-6 && (y1 - (y0 + 20.0)).abs() < 1e-6, "a half turn about a pivot 10 below mirrors the node 20 down: ({x0},{y0}) -> ({x1},{y1})");
    let noop = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": record(0.0, json!([node_id.clone()])) })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("zero rotate");
    assert_eq!(committed_edits(&noop), 0, "a zero-angle record commits nothing");
    let targetless = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": record(1.0, json!([])) })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("target-less rotate");
    assert_eq!(committed_edits(&targetless), 0, "a target-less record commits nothing");
    close_app(&mut app);
}

/// 🔄️ LAW: a rotate record turns EVERY unlocked target about the recorded pivot and carries its handle angles
/// with it, so edges keep their geometry; a locked target stays put — the guest half of the ring's preview.
#[test]
fn a_rotate_record_orbits_a_two_node_selection_about_its_recorded_pivot() {
    let snapshot = json!({
        "schema": "board.ports.directed.v1",
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "nodes": [
            { "id": "node-a", "x": -40.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-a:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] },
            { "id": "node-b", "x": 40.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-b:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] },
            { "id": "node-locked", "x": 0.0, "y": 0.0, "locked": true, "shape": "circle", "radius": 10.0, "handles": [] }
        ],
        "edges": []
    });
    let base = Puzzle2dPlaySnapshot::new(semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(&(snapshot))).expect("typed fixture admits")).typed().clone();
    let targets = vec!["node-a".to_string(), "node-b".to_string(), "node-locked".to_string()];
    let (pivot_x, pivot_y) = puzzle2d_selection_pivot(&base, &targets, false).expect("pivot");
    let record = Puzzle2dSelectionRecord { targets, motion: Puzzle2dSelectionMotion::Rotate { pivot_x, pivot_y, angle: std::f64::consts::FRAC_PI_2 }, proximity: Vec::new(), connect: false };
    let yields = select_utility::puzzle2d_selection_yields(&base, std::slice::from_ref(&record), 0.0);
    assert_eq!(yields.len(), 1, "a rotate yields its one leaf and no connection");
    let mut after = base.clone();
    semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::apply_puzzle2d_mutation(&mut after, &yields[0].1).expect("the leaf applies");
    let snapshot = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&(semio_framework_value::ToValue::to_value(&after))));
    let a = transform_law_node(&snapshot, "node-a");
    assert!(a.get("x").and_then(Value::as_f64).expect("x").abs() < 1e-6 && (a.get("y").and_then(Value::as_f64).expect("y") + 40.0).abs() < 1e-6, "node-a orbits to (0,-40): {a}");
    let b = transform_law_node(&snapshot, "node-b");
    assert!(b.get("x").and_then(Value::as_f64).expect("x").abs() < 1e-6 && (b.get("y").and_then(Value::as_f64).expect("y") - 40.0).abs() < 1e-6, "node-b orbits to (0,40): {b}");
    let angle = a.get("handles").and_then(Value::as_array).expect("handles")[0].get("angle").and_then(Value::as_f64).expect("angle");
    assert!((angle - std::f64::consts::FRAC_PI_2).abs() < 1e-9, "the handle angle turns with its node, got {angle}");
    let locked = transform_law_node(&snapshot, "node-locked");
    assert!(locked.get("x").and_then(Value::as_f64).expect("x").abs() < 1e-9 && locked.get("y").and_then(Value::as_f64).expect("y").abs() < 1e-9, "a locked node stays put: {locked}");
}
//#endregion 🕹️TransformGumball

//#region 🎯️BoardRegionEvents
/// 🎯️ The document's target-region rows, for the board-event laws.
fn law_target_regions(snapshot: &Value) -> Vec<Value> {
    snapshot.get("targetRegions").and_then(Value::as_array).cloned().unwrap_or_default()
}

/// 🎯️ LAW: the board engine's region gestures land one history edit each — `regionCreate` mints one row through
/// `addTargetRegion`'s own push, a body drag is a `drag` gesture record yielding the `drag-selection` leaf, and
/// `regionResize` pushes an absolute pose through `relocateTargetRegion`.
#[semio_framework_async_macros::async_test]
async fn board_region_events_commit_one_edit_each_through_the_target_region_verbs() {
    let mut app = concrete_forest_app();
    let created = json!([{ "name": "regionCreate", "payload": { "x": -20.5, "y": -10.5, "width": 60.5, "height": 40.5 } }]).to_string();
    dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": created })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("region create");
    let regions = law_target_regions(&fixture_of(&app));
    assert_eq!(regions.len(), 1, "exactly one row is minted: {regions:?}");
    let id = regions[0].get("id").and_then(Value::as_str).expect("minted id").to_string();
    assert_eq!(regions[0].get("width").and_then(Value::as_f64), Some(60.5), "the engine's rectangle is committed verbatim — it already snapped it");

    let moved = json!([{ "name": "gesture", "payload": { "gestureId": "gesture-1", "kind": "drag", "targets": [id.clone()], "dx": 121.0, "dy": 211.0, "proximity": [] } }]).to_string();
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": moved })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("region move");
    assert_eq!(committed_edits(&result), 1, "a region body drag is one edit");
    let regions = law_target_regions(&fixture_of(&app));
    assert_eq!(regions[0].get("x").and_then(Value::as_f64), Some(100.5), "a move translates the minimum corner by the offset");
    assert_eq!(regions[0].get("width").and_then(Value::as_f64), Some(60.5), "and never restates the extent it did not touch");

    let resized = json!([{ "name": "regionResize", "payload": { "id": id.clone(), "x": 100.5, "y": 200.5, "width": 12.5, "height": 8.5 } }]).to_string();
    dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": resized })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("region resize");
    let regions = law_target_regions(&fixture_of(&app));
    assert_eq!(regions[0].get("width").and_then(Value::as_f64), Some(12.5), "a resize pushes corner AND extent");
    assert_eq!(regions[0].get("height").and_then(Value::as_f64), Some(8.5));

    let unknown = json!([{ "name": "gesture", "payload": { "gestureId": "gesture-2", "kind": "drag", "targets": ["never-painted"], "dx": 1.5, "dy": 1.5, "proximity": [] } }]).to_string();
    dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": unknown })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("unknown region");
    let regions = law_target_regions(&fixture_of(&app));
    assert_eq!(regions[0].get("x").and_then(Value::as_f64), Some(100.5), "a row naming a region the board never held moves nothing");
    close_app(&mut app);
}

/// 🖍️ LAW: the board scene carries the Area Brush's own steppers as WORLD extent, so one canvas click
/// and one `addTargetRegion` dispatch paint the identical rectangle. The regions themselves ride the
/// snapshot lane — the document's `targetRegions` — never a second carrier.
#[semio_framework_async_macros::async_test]
async fn the_board_scene_carries_the_area_brush_extent_and_the_regions_ride_the_fixture() {
    let mut app = concrete_forest_app();
    dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "regionCreate", "payload": { "x": 0.5, "y": 0.5, "width": 10.5, "height": 10.5 } }]).to_string() })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("region create");
    let rendered: Value = semio_framework_pack_json::from_json_str(&render_body(&mut app, overview::BODY_KEY), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("rendered board surface parses");
    let board = rendered.get("board2d").expect("board scene");
    let encoded = board.get("areaBrushSize").and_then(Value::as_str).expect("the board scene declares areaBrushSize");
    let size: Value = semio_framework_pack_json::from_json_str(encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("areaBrushSize is a JSON object");
    assert!(size.get("width").and_then(Value::as_f64).is_some_and(|width| width > 0.0), "the brush extent is a positive world width: {encoded}");
    assert!(size.get("height").and_then(Value::as_f64).is_some_and(|height| height > 0.0), "and a positive world height: {encoded}");
    let snapshot_json = board.get("snapshotJson").and_then(Value::as_str).expect("the board scene carries the snapshot");
    assert!(snapshot_json.contains("targetRegions"), "the painted region reaches the engine through the snapshot lane: {}", &snapshot_json[..snapshot_json.len().min(200)]);
    close_app(&mut app);
}
//#endregion 🎯️BoardRegionEvents

//#region 🙈️SetVerbIdempotence
/// 🙈️ LAW: an outliner row's set-verbs are idempotent by value — replaying `{verb}{ids|id, flag: true}` (a stale view's
/// second click) leaves exactly the first dispatch's ONE document edit and a byte-identical document, for the node verbs
/// and the target-region verbs alike; and a set-verb without its value is refused at the command boundary, never defaulted.
#[test]
fn outliner_set_verbs_are_idempotent_by_value_and_refuse_a_missing_value() {
    let mut app = concrete_forest_app();
    dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "regionCreate", "payload": { "x": 0.5, "y": 0.5, "width": 10.5, "height": 10.5 } }]).to_string() })), Some(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID)).expect("region create");
    let node = first_node_id(&app);
    let region = law_target_regions(&fixture_of(&app))[0].get("id").and_then(Value::as_str).expect("the painted region has an id").to_string();
    for (verb, flag, mut args) in [
        ("setSelectionHidden", "hidden", json!({ "ids": [node.clone()] })),
        ("setSelectionLocked", "locked", json!({ "ids": [node.clone()] })),
        ("setTargetRegionHidden", "hidden", json!({ "id": region.clone() })),
        ("setTargetRegionLocked", "locked", json!({ "id": region.clone() })),
    ] {
        assert!(<Puzzle2dPlayApp as ArtifactEditor>::command_from_action(verb, Some(&semio_framework_value::ToValue::to_value(&args))).is_err(), "{verb} without its {flag} value must be refused");
        set_at(&mut args, flag, json!(true));
        let first = dispatch(&mut app, verb, Some(&args), None).unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
        assert_eq!(committed_edits(&first), 1, "{verb} commits exactly one document edit");
        let set = fixture_of(&app);
        let replay = dispatch(&mut app, verb, Some(&args), None).unwrap_or_else(|fault| panic!("{verb} replay: {fault:?}"));
        assert_eq!(committed_edits(&replay), 0, "replaying {verb} with the same value commits no second edit");
        assert_eq!(fixture_of(&app), set, "replaying {verb} with the same value leaves the document byte-identical");
    }
    let snapshot = fixture_of(&app);
    let flagged = board_snapshot_nodes(&snapshot).iter().find(|entry| entry.get("id").and_then(Value::as_str) == Some(node.as_str())).cloned().expect("the flagged node");
    assert!(puzzle2d_entity_hidden(&flagged) && flagged.get("locked").and_then(Value::as_bool) == Some(true), "the node row's set-verbs land in the document's own fields: {flagged}");
    close_app(&mut app);
}
//#endregion 🙈️SetVerbIdempotence

//#region 🔖️Pz2ShippedKindCatalog
/// 🗂️ The authored [`PUZZLE2D_SHIPPED_NODE_KINDS`] IS the two shipped examples' own node-kind rows,
/// in their own order — the law that lets the `kind` select be a `const` instead of a parse of
/// 96 005 B of DSL on the `describe()` path (slice PZ2, 2026-09-22). It pays that parse HERE, once,
/// and fails the moment an example's kinds change without the authored list following.
#[semio_framework_async_macros::async_test]
async fn shipped_node_kinds_are_the_two_examples_own_catalog_rows() {
    let mut derived: Vec<(String, String)> = Vec::new();
    for json in [concrete_forest_example_json(), nakagin_example_json()] {
        let Ok(snapshot) = semio_framework_pack_json::from_json_str::<Value>(&json, semio_framework_pack_json::JsonMemberPolicy::Reject) else { continue };
        for row in puzzle2d_node_kind_rows(&snapshot) {
            if derived.len() >= PUZZLE2D_NODE_KIND_OPTIONS_MAX {
                break;
            }
            let Some(id) = row.get("id").and_then(Value::as_str).filter(|id| !id.is_empty()) else { continue };
            if derived.iter().any(|(existing, _)| existing == id) {
                continue;
            }
            let label = row.get("name").and_then(Value::as_str).filter(|name| !name.is_empty()).unwrap_or(id);
            derived.push((id.to_string(), label.to_string()));
        }
    }
    let authored: Vec<(String, String)> = PUZZLE2D_SHIPPED_NODE_KINDS.iter().map(|(id, label)| ((*id).to_string(), (*label).to_string())).collect();
    assert_eq!(authored, derived, "PUZZLE2D_SHIPPED_NODE_KINDS drifted from the shipped documents — re-author it from `concrete-forest` then `nakagin-capsule-tower`");
}
//#endregion 🔖️Pz2ShippedKindCatalog

//#region 🔖️InitialSnapshotCost
/// 🚀️ LAW (ticket 26/09/23, H12): the initial snapshot costs its document — the empty snapshot — and nothing else.
/// Every construction of this app pays for it — each mount, and every `codec` call a hub makes to create or
/// validate a document, where the interpreter multiplies it — so it must never parse an example (the example
/// load forces its own `LazyLock` when it is switched to). Weighed by this thread's heap peak (`HeapWitness`),
/// independent of other laws' allocations: within 4 KiB of converting the empty snapshot itself. The examples are
/// process-wide `LazyLock`s, so the law detects an example parse where it runs as its own process (nextest).
#[test]
fn the_initial_snapshot_costs_only_its_document() {
    let peak_of = |work: &dyn Fn()| {
        let before = semio_framework_trace::retained_heap_bytes_on_this_thread();
        semio_framework_trace::reset_heap_peak_on_this_thread();
        work();
        semio_framework_trace::peak_heap_bytes_on_this_thread() - before
    };
    let document = peak_of(&|| drop(Puzzle2dPlaySnapshot::new(semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(&(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&(empty_board_snapshot())))))).expect("typed fixture admits"))));
    let snapshot = peak_of(&|| drop(<Puzzle2dPlayApp as ArtifactEditor>::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births")));
    assert!(document > 0, "the witness weighs the document conversion: {document}");
    assert!(snapshot <= document + 4096, "the initial snapshot peaked at {snapshot} B, its document at {document} B: it does work the document never reads");
}
//#endregion 🔖️InitialSnapshotCost

#[test]
fn snapshot_import_requires_the_explicit_payload_contract() {
    use crate::editor::puzzle2d::commands::import_snapshot::puzzle2d_import_value;
    let vectors: semio_framework_pack_json::Value = semio_framework_pack_json::from_json_str(fixture_text!("../../../../🎮️commands/🧊️document/📥️import-snapshot/🧪️tests/🧫️fixtures/📄️payload-input/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("plain input examples");
    for case in vectors["cases"].as_array().expect("input cases") {
        let result = puzzle2d_import_value(&case["args"]);
        assert_eq!(result.is_ok(), case["accepted"].as_bool().expect("acceptance"));
        if let Ok(snapshot) = result {
            let input = &case["args"]["payload"];
            let source = input.as_str().map(str::to_string).unwrap_or_else(|| semio_framework_pack_json::to_string(input));
            let oracle: serde_json::Value = serde_json::from_str(&source).expect("third-party JSON oracle");
            let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_string(&snapshot)).expect("native output oracle");
            assert_eq!(actual, oracle);
        }
    }
}
