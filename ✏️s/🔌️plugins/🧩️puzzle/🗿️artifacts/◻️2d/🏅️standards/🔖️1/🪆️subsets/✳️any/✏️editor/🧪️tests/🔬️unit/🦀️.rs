
use super::testkit::*;
use super::*;
use crate::Puzzle2dSnapshot;
use semio_framework_plugin::PluginApp;
use store::{Backbone, BackboneMessage, MemoryBackbone};

fn cohort_routes_are_cursorized(source: &str) -> bool {
    [
        r#""forceLayout" => Box::new(Puzzle2dForceLayoutWork::default())"#,
        r#""setActiveExample" => Box::new(Puzzle2dActiveExampleWork::default())"#,
        "Puzzle2dForceStage::Nodes",
        "Puzzle2dForceStage::Handles",
        "Puzzle2dForceStage::Edges",
        "Puzzle2dForceStage::Repel",
        "Puzzle2dForceStage::Springs",
        "Puzzle2dForceStage::Integrate",
        "Puzzle2dForceStage::Emit",
        "Puzzle2dExampleStage::ClearEdges",
        "Puzzle2dExampleStage::ClearNodes",
        "Puzzle2dExampleStage::AddCompatibility",
        "Puzzle2dExampleStage::Nodes",
        "Puzzle2dExampleStage::Edges",
        r#"matches!(command.action_id(), "addNode").then_some(1)"#,
    ]
    .into_iter()
    .all(|marker| source.contains(marker))
        && !source.contains(r#""forceLayout" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""setActiveExample" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn cohort_hostile_static_law_rejects_one_grant_complex_routes_and_missing_cursors() {
    let source = include_str!("../../🦀️.rs");
    assert!(cohort_routes_are_cursorized(source));
    for (retained, direct) in [
        (r#""forceLayout" => Box::new(Puzzle2dForceLayoutWork::default())"#, r#""forceLayout" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle2d_retained_reduce, puzzle2d_retained_extent))"#),
        (r#""setActiveExample" => Box::new(Puzzle2dActiveExampleWork::default())"#, r#""setActiveExample" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle2d_retained_reduce, puzzle2d_retained_extent))"#),
    ] {
        assert!(!cohort_routes_are_cursorized(&source.replace(retained, direct)));
    }
    for marker in [
        "Puzzle2dForceStage::Nodes",
        "Puzzle2dForceStage::Handles",
        "Puzzle2dForceStage::Edges",
        "Puzzle2dForceStage::Repel",
        "Puzzle2dForceStage::Springs",
        "Puzzle2dForceStage::Integrate",
        "Puzzle2dForceStage::Emit",
        "Puzzle2dExampleStage::ClearEdges",
        "Puzzle2dExampleStage::ClearNodes",
        "Puzzle2dExampleStage::AddCompatibility",
        "Puzzle2dExampleStage::Nodes",
        "Puzzle2dExampleStage::Edges",
    ] {
        assert!(!cohort_routes_are_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing retained cursor was falsely accepted: {marker}");
    }
}

/// 🪣️ The fill family is retained-only: `handle` must refuse it outright, the mounted store-lease
/// hooks the removed session registry needed must be gone, and every verb must resolve to the
/// bespoke session work.
fn fill_session_retained_only_contract(source: &str) -> bool {
    let production = source.split("//#region 🧪️Tests").next().unwrap_or(source);
    let Some(handle) = production.find("    fn handle(") else { return false };
    let Some(fill_relative) = production[handle..].find("if set_fill_count::is_fill_session_action(action) {") else { return false };
    let fill = handle + fill_relative;
    let Some(normal_relative) = production[fill..].find("let before = doc.snapshot.0.clone();") else { return false };
    let branch = &production[fill..fill + normal_relative];
    branch.contains("puzzle2d-fill-requires-retained-job")
        && !branch.contains("Puzzle2dFillActionCtx")
        && !branch.contains("Puzzle2dConfigMutation")
        && !branch.contains("artifact_mutations")
        && !production.contains("fn mounted_job_prepare_snapshot_read")
        && !production.contains("fn pending_effects")
        && !production.contains("dispatch_fill_session_action")
        && production.contains("fill if set_fill_count::is_fill_session_action(fill) => Box::new(set_fill_count::Puzzle2dFillSessionWork::new(fill))")
        && set_fill_count::PUZZLE2D_FILL_SESSION_ACTIONS.iter().all(|action| PUZZLE2D_RETAINED_TOOL_IDS.contains(action))
}

/// 🧱️ Reviving the mounted fill dispatch path — the action context, a config mutation published
/// from `handle`, or the store-lease hooks the process-global session registry needed — fails the
/// retained-only law.
#[test]
fn mounted_fill_dispatch_revivals_are_rejected() {
    let source = include_str!("../../🦀️.rs");
    assert!(fill_session_retained_only_contract(source));
    let ctx = source.replacen(
        "            return Err(Fault::from(\"puzzle2d-fill-requires-retained-job\"));",
        "            let ctx = set_fill_count::Puzzle2dFillActionCtx {};\n            return Err(Fault::from(\"puzzle2d-fill-requires-retained-job\"));",
        1,
    );
    assert!(!fill_session_retained_only_contract(&ctx));
    let published = source.replacen("            return Err(Fault::from(\"puzzle2d-fill-requires-retained-job\"));", "            return Ok(Emit { config_mutations: vec![Puzzle2dConfigMutation::Fill { runtime }], ..Default::default() });", 1);
    assert!(!fill_session_retained_only_contract(&published));
    let lease = source.replacen("    fn handle(", "    fn pending_effects(doc: &ArtifactView<'_, Puzzle2dPlaySnapshot>) -> Vec<Effect> { Vec::new() }\n\n    fn handle(", 1);
    assert!(!fill_session_retained_only_contract(&lease));
    let unmapped = source.replacen("fill if set_fill_count::is_fill_session_action(fill) => Box::new(set_fill_count::Puzzle2dFillSessionWork::new(fill))", "fill if false => Box::new(crate::retained_command::NoopPuzzleCommandWork::new(fill))", 1);
    assert!(!fill_session_retained_only_contract(&unmapped));
}

/// 🎥️ Recovers the rendered pane camera `(x, y, zoom)` from a rendered `UiNode`'s embedded
/// `Board2dScene.cameraJson` — the only externally observable surface for the runtime camera
/// (the camera is never a document field, so it cannot be read back off `app.snapshot()`).
fn rendered_camera(rendered: &str) -> (f64, f64, f64) {
    fn find_camera_json(value: &Value) -> Option<String> {
        if let Some(json) = value.get("cameraJson").and_then(Value::as_str) {
            return Some(json.to_string());
        }
        match value {
            Value::Object(map) => map.values().find_map(find_camera_json),
            Value::Array(items) => items.iter().find_map(find_camera_json),
            _ => None,
        }
    }
    let value: Value = serde_json::from_str(rendered).expect("rendered node parses");
    let camera_json = find_camera_json(&value).expect("rendered scene must carry cameraJson");
    let camera: Value = serde_json::from_str(&camera_json).expect("cameraJson parses");
    (camera.get("x").and_then(Value::as_f64).unwrap_or(f64::NAN), camera.get("y").and_then(Value::as_f64).unwrap_or(f64::NAN), camera.get("zoom").and_then(Value::as_f64).unwrap_or(f64::NAN))
}

//#region 🔖️Operations
#[semio_framework_async_macros::async_test]
async fn add_node_action_emits_upsert_op_and_appends_node() {
    let mut app = app();
    let result = dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");
    assert_eq!(result.mutations.len(), 1, "addNode must emit exactly one granular operation");
    assert_eq!(fixture_nodes(&fixture_of(&app)).len(), 1);
}

/// 🛍️ The example load commits granular operations from ONE dispatch — the retained
/// `Puzzle2dActiveExampleWork` state machine, driven to its terminal emit, with no
/// `Effect::DispatchAction` continuation ladder left to pump.
#[semio_framework_async_macros::async_test]
async fn set_active_example_loads_concrete_forest_via_operations() {
    let mut app = app();
    let result = dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID })), None).expect("load example");
    assert!(result.requested_effects.is_empty(), "the example load must not request a continuation effect");
    assert!(!result.mutations.is_empty(), "the example load must commit granular operations");
    assert!(!fixture_nodes(&fixture_of(&app)).is_empty());
}

/// 🔁️ Loading a second example clears the first one out of the document rather than merging into
/// it — the work's `ClearEdges`/`ClearNodes` stages run against the live snapshot every time.
#[semio_framework_async_macros::async_test]
async fn a_newer_example_load_replaces_the_previous_document() {
    let mut app = app();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let forest_nodes = fixture_nodes(&fixture_of(&app)).len();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    assert!(!fixture_edges(&fixture_of(&app)).is_empty());
    assert_ne!(fixture_nodes(&fixture_of(&app)).len(), forest_nodes, "the second load must replace, not append to, the first example");
}

/// 📦️ `Puzzle2dPlaySnapshot`'s pack encoding round-trips through the same `(RecordSpec,
/// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
/// `serde_json::Value` bridge impls).
#[semio_framework_async_macros::async_test]
async fn puzzle2d_play_projection_pack_round_trips() {
    let app = concrete_forest_app();
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&app.snapshot().expect("projection"));
}

#[semio_framework_async_macros::async_test]
async fn select_then_delete_selection_removes_the_node() {
    let mut app = app_with_registry();
    dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");
    let node_id = first_node_id(&app);
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, &node_id).expect("select");
    dispatch(&mut app, "deleteSelection", None, None).expect("delete");
    assert!(fixture_nodes(&fixture_of(&app)).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trip_through_the_wrapper() {
    let mut app = app();
    dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add");
    assert_eq!(fixture_nodes(&fixture_of(&app)).len(), 1);
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert_eq!(fixture_nodes(&fixture_of(&app)).len(), 0);
    dispatch(&mut app, "redo", None, None).expect("redo");
    assert_eq!(fixture_nodes(&fixture_of(&app)).len(), 1);
}
//#endregion 🔖️Operations

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`). Deliberately
/// dispatches through a standalone typed `Puzzle2dStore` — NOT through `Puzzle2dPlayApp`/
/// `Puzzle2dPlaySnapshot` (the `🔖️ValueBridge` `serde_json::Value` wrapper this app still uses)
/// — since `Puzzle2dMutation`'s canonical `Mutation<Puzzle2dSnapshot>` impl (not its
/// `Mutation<Value>` bridge impl) is what the CW7 law is about.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::standards::v1::subsets::any::schema::mutations::binary::Puzzle2dStore;
    use crate::{PUZZLE_2D_SCHEMA, Puzzle2dNode};
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{ArtifactCommand, create_document_envelope};

    let mut store = Puzzle2dStore::new(create_document_envelope(PUZZLE_2D_SCHEMA, "puzzle2d", Puzzle2dSnapshot::default(), None)).await.expect("store");
    let node = Puzzle2dNode { id: "n1".into(), ..Default::default() };
    store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::standards::v1::subsets::any::schema::mutations::create_node(node, None)], description: None }).await.expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<Puzzle2dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle2dSnapshot, Puzzle2dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())).await;
}
//#endregion 🔖️CommandEnvelopeTests

//#region 🔖️BoardEvents
/// 🎥️ `setCamera` is session-only view state: a camera drag never creates a VCS edit, so there is
/// nothing to coalesce and nothing for `undo` to revert.
#[semio_framework_async_macros::async_test]
async fn set_camera_is_session_only_and_never_undoable() {
    let mut app = app();
    for x in [1.0, 2.0, 3.0] {
        let result = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "x": x, "y": 0.0, "zoom": 1.0 } })), None).expect("camera");
        assert!(result.mutations.is_empty(), "setCamera must never produce a document operation");
    }
    let rendered = render_body(&mut app, overview::BODY_KEY);
    assert_eq!(rendered_camera(&rendered).0, 3.0, "the camera must update immediately in the rendered scene");
    let undo = dispatch(&mut app, "undo", None, None).expect("undo");
    assert!(undo.mutations.is_empty(), "there is no document edit to undo");
    let rendered_after_undo = render_body(&mut app, overview::BODY_KEY);
    assert_eq!(rendered_camera(&rendered_after_undo).0, 3.0, "the camera is session state — undo must not revert it");
}

/// 🐢️ Regression test for a perf-round-2 bug: `parse_fixture_v1` always `clear_scene()`s then
/// rebuilds, so every edge looked "new" and got re-`push_event`'d as `edgeCreate` — which
/// `apply_host_events` then replayed into the fixture on the *next* action, duplicating every edge
/// once per action forever.
#[semio_framework_async_macros::async_test]
async fn repeated_actions_do_not_duplicate_edges() {
    let mut app = app();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    let edge_count = |app: &Puzzle2dApp| fixture_edges(&fixture_of(app)).len();
    let before = edge_count(&app);
    assert!(before > 0, "fixture must have edges for this regression test to be meaningful");
    let node_id = first_node_id(&app);
    for _ in 0..5 {
        dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), None).expect("select");
    }
    assert_eq!(edge_count(&app), before, "selecting repeatedly must not grow the edges array");
}

/// 🪞️ Regression test: `applyBoardEvents`'s `select` case only mutated the runtime, never the
/// host, so `apply_host_events`'s `host.selection`-is-truth re-sync silently reverted the
/// selection to whatever the host held before the action (empty, on a fresh sync).
#[semio_framework_async_macros::async_test]
async fn apply_board_events_select_persists_across_the_next_action() {
    let mut app = concrete_forest_app();
    let node_id = first_node_id(&app);
    dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), None).expect("select");
    assert!(render_body(&mut app, overview::BODY_KEY).contains(&node_id), "selection must be visible immediately after the select action");
    // A second, unrelated action used to silently clear the selection via the stale `host.selection` re-sync.
    dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": "[]" })), None).expect("no-operation");
    assert!(render_body(&mut app, overview::BODY_KEY).contains(&node_id), "selection must survive a subsequent unrelated action");
}

/// 🪞️ Regression test: `apply_host_events` used to epsilon-compare `host.camera` (still the
/// *pre-action* value) against the runtime and blindly overwrite it, reverting a plain `camera`
/// board event (used for the live wheel-zoom echo) before it ever committed.
#[semio_framework_async_macros::async_test]
async fn apply_board_events_camera_event_commits() {
    let mut app = app();
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "camera", "payload": { "x": 5.0, "y": 6.0, "zoom": 1.2 } }]).to_string() })), None).expect("camera event");
    assert!(result.mutations.is_empty(), "a camera board event must never produce a document operation");
    let (x, y, zoom) = rendered_camera(&render_body(&mut app, overview::BODY_KEY));
    assert_eq!(x, 5.0);
    assert_eq!(y, 6.0);
    assert_eq!(zoom, 1.2);
}

/// 🐢️ A pure selection change is runtime state, not document state — it must not produce any
/// operations (previously it fell back to a whole-document replace once the edge-duplication bug
/// made `before` and `after` genuinely diverge).
#[semio_framework_async_macros::async_test]
async fn select_action_emits_no_operations() {
    let mut app = concrete_forest_app();
    let node_id = first_node_id(&app);
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), None).expect("select");
    assert!(result.mutations.is_empty(), "selection must not produce document operations");
}
//#endregion 🔖️BoardEvents

//#region 🔖️UiScope
/// 🐢️ Perf round 3: a select event must declare a narrow `Partial` ui_scope (the 3 canvas panes +
/// layers/properties panels + engagements) — never `Full`, or the shell's batched `refresh-ui`
/// call degrades back to fetching everything on every select.
#[semio_framework_async_macros::async_test]
async fn select_action_declares_partial_ui_scope() {
    let mut app = concrete_forest_app();
    let node_id = first_node_id(&app);
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), None).expect("select");
    match result.ui_scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
            // 🐢️ Regression: `window_bodies` must list the window *body keys* (matched against
            // `AppDefinition.windowKinds[].bodyKey` by the shell's `buildUiRefreshRequest`), not
            // the pane/kind-id constants (`PUZZLE2D_PANES`) — those are a different id space.
            assert_eq!(window_bodies, vec![overview::BODY_KEY, detail::BODY_KEY, selection::BODY_KEY], "window_bodies must be body keys, not pane ids");
            assert!(panel_bodies.contains(&document::PUZZLE2D_PLAY_BODY_LAYERS.to_string()));
            assert!(panel_bodies.contains(&inspection::PUZZLE2D_PLAY_BODY_PROPERTIES.to_string()));
            assert!(engagements, "select must refresh the engagement bar");
            assert!(!measures, "select must not force a measures refresh");
            assert!(!utilities);
            assert!(!tools);
            assert!(!labels);
        }
        other => panic!("expected a Partial ui_scope for select, got {other:?}"),
    }
}

/// 🐢️ Perf round 3: a camera-only board event touches only the 3 canvas panes — no panels,
/// engagements, measures, or utilities.
#[semio_framework_async_macros::async_test]
async fn camera_event_declares_window_only_ui_scope() {
    let mut app = app();
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "camera", "payload": { "x": 1.0, "y": 2.0, "zoom": 1.0 } }]).to_string() })), None).expect("camera event");
    match result.ui_scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
            assert_eq!(window_bodies.len(), 3);
            assert!(panel_bodies.is_empty(), "a config-only camera event does not dirty command history");
            assert!(!engagements && !measures && !utilities && !tools && !labels);
        }
        other => panic!("expected a Partial ui_scope for a camera event, got {other:?}"),
    }
}

/// 🐢️ Perf round 3: an empty `applyBoardEvents` batch (no-operation) must declare nothing beyond the
/// history panel body — the empty View action neither logs an edit nor dirties a surface.
#[semio_framework_async_macros::async_test]
async fn empty_board_events_declare_none_ui_scope() {
    let mut app = app();
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": "[]" })), None).expect("no-operation");
    assert_eq!(result.ui_scope, UiDirtyScope::None);
}

/// 🐢️ Perf round 3: cold-tier structural actions (document operations) must keep the safe `Full`
/// default — no puzzle2d scope helper narrows them.
#[semio_framework_async_macros::async_test]
async fn add_node_action_declares_full_ui_scope() {
    let mut app = app();
    let result = dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");
    assert!(matches!(result.ui_scope, UiDirtyScope::Full), "addNode must stay Full, got {:?}", result.ui_scope);
}
//#endregion 🔖️UiScope

//#region 🔖️Manifest
#[test]
fn app_definition_has_three_lod_pane_window_kinds() {
    let definition = create_puzzle2d_app();
    let ids: Vec<&str> = definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
    assert_eq!(ids, vec![overview::WINDOW_KIND_ID, detail::WINDOW_KIND_ID, selection::WINDOW_KIND_ID]);
    for window in &definition.window_kinds {
        assert!(window.options.engagement.as_option().is_some(), "pane {} must have engagement", window.id);
        assert!(!window.options.measures.is_empty(), "pane {} must have LOD/suggestion measures", window.id);
    }
}

/// 🧰️ The app declares exactly the select/brush canvas utilities and binds them to the interactive
/// overview pane; fill is declared as a mode-level tool instead.
#[test]
fn utility_registry_declares_utilities() {
    let definition = create_puzzle2d_app();
    let ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert_eq!(ids, vec![select_utility::UTILITY_ID, brush_utility::UTILITY_ID]);
    let overview_window = definition.window_kinds.iter().find(|window| window.id == overview::WINDOW_KIND_ID).expect("overview pane");
    let overview_utilities: Vec<&str> = overview_window.utilities.iter().map(|utility| utility.as_str()).collect();
    assert_eq!(overview_utilities, vec![select_utility::UTILITY_ID, brush_utility::UTILITY_ID]);
    assert!(overview_window.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID), "declaring utilities must inject the setActiveUtility action");
    // 🧰️ D-1: select/brush are this window's whole exclusive utility set, NOT a sub-collection, so
    // each carries `group: None` and renders as a flat utility bar icon (never one collapsed dropdown).
    for utility in &definition.utilities {
        assert_eq!(utility.group, None, "utility {} must render flat (no shared group)", utility.id);
    }
}

/// 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility.
#[test]
fn tool_registry_declares_fill_tool() {
    use semio_framework_plugin::{SET_ACTIVE_TOOL_ACTION_ID, ToolRef};
    let definition = create_puzzle2d_app();
    let tool_ids: Vec<&str> = definition.tools.iter().map(|tool| tool.id.as_str()).collect();
    assert_eq!(tool_ids, vec![fill::TOOL_ID]);
    assert_eq!(definition.modes[0].tools, vec![semio_framework::io::resolve_ready(ToolRef::new(fill::TOOL_ID))]);
    assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID), "declaring tools must inject the setActiveTool action");
}

/// 🎥️ The camera is session-only runtime state, never a document field — a DWG import (which has
/// no live app instance to receive a runtime write) must produce a bare empty board with no
/// `"camera"` key at all, regardless of the drawing's extents.
//#endregion 🔖️Manifest

//#region 🔖️Convergence
/// 🧪️ Definitional convergence proof: two instances on one backbone make DISJOINT node edits
/// (each adds its own node) and, after exchanging operations, both converge to contain BOTH nodes —
/// impossible under whole-document `setSnapshot` snapshots, which would clobber one side.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_node_edits_via_backbone() {
    let mut instance_a = app();
    let mut instance_b = app();
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://puzzle2d-convergence", "mem://puzzle2d-convergence").await;
    instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");

    dispatch(&mut instance_a, "addNode", Some(&json!({ "kind": "seed" })), None).expect("a adds node");
    dispatch(&mut instance_b, "addNode", Some(&json!({ "kind": "other" })), None).expect("b adds node");

    // A neutral history action always calls store.dispatch(), which pumps inbound operations first.
    dispatch(&mut instance_a, "commitCheckpoint", None, None).expect("pump a");
    dispatch(&mut instance_b, "commitCheckpoint", None, None).expect("pump b");

    assert_eq!(fixture_nodes(&fixture_of(&instance_a)).len(), 2, "instance A must contain both nodes");
    assert_eq!(fixture_nodes(&fixture_of(&instance_b)).len(), 2, "instance B must contain both nodes");
}

#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent() {
    let mut sender = app();
    let (near, mut far) = MemoryBackbone::pair("mem://puzzle2d-doc", "mem://puzzle2d-doc").await;
    sender.attach_backbone(store::Backbones::Memory(near)).await.expect("attach");
    dispatch(&mut sender, "addNode", Some(&json!({ "kind": "seed" })), None).expect("add");

    let mut envelopes = Vec::new();
    for message in far.receive().await.expect("receive") {
        if let BackboneMessage::Mutations { envelopes: operations } = message {
            envelopes.extend(operations);
        }
    }
    assert!(!envelopes.is_empty(), "the applied operation must flow onto the channel");
    let operations = envelopes;

    let mut receiver = app();
    receiver.ingest_operations(&operations).await.expect("ingest once");
    receiver.ingest_operations(&operations).await.expect("ingest twice");
    assert_eq!(fixture_nodes(&fixture_of(&receiver)).len(), 1, "feeding the same operation twice must not double-apply");
}
//#endregion 🔖️Convergence

//#region 🔖️Registry
/// 🧰️ B1: `setActiveUtility` is a real typed `Puzzle2dCommand` now (was a host-applied `ViewModel`
/// notification): switching utilities must still emit no DOCUMENT operations — the new value lands
/// in `Puzzle2dConfig::active_utility_by_window_id` as a config operation instead.
#[semio_framework_async_macros::async_test]
async fn utility_switch_emits_no_ops_and_no_history() {
    let mut app = app_with_registry();
    let result = dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": brush_utility::UTILITY_ID })), Some(overview::WINDOW_KIND_ID)).expect("switch utility");
    assert!(result.mutations.is_empty(), "a utility switch must not produce document operations");
    let can_undo = dispatch(&mut app, "undo", None, None);
    assert!(can_undo.map_or(true, |r| r.mutations.is_empty()), "a utility switch must not have created a document undo step");
}

/// 🧭️ Kind discipline: every View-declared runtime/host action must run through the registry
/// without tripping the "must not emit operations" guard (proving each is correctly classified).
#[semio_framework_async_macros::async_test]
async fn view_actions_emit_no_ops_through_the_registry() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let node_id = first_node_id(&app);
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, &node_id).expect("select");
    let view_dispatches: Vec<(&str, Value)> = vec![
        ("setCamera", json!({ "camera": { "x": 7.0, "y": 8.0, "zoom": 1.5 } })),
        ("selectSameKind", Value::Null),
        ("setGridSnapEnabled", json!({ "enabled": true })),
        ("setGridFactor", json!({ "value": 2.0 })),
        ("setLodModeForPane", json!({ "pane": overview::WINDOW_KIND_ID, "value": "detail" })),
        ("setBrushKindWeights", json!({ "kindId": "node", "value": 0.5 })),
        ("setBrushNodeSize", json!({ "size": 12.0 })),
        ("setSuggestionOffset", json!({ "value": 40.0 })),
        ("engagementInput", json!({ "pane": overview::WINDOW_KIND_ID, "value": "brush" })),
        ("engagementSubmit", json!({ "pane": overview::WINDOW_KIND_ID, "value": "brush" })),
        ("engagementAbort", json!({ "pane": overview::WINDOW_KIND_ID })),
        ("brushCycleCandidate", json!({ "forward": true })),
        ("brushSetCandidateIndex", json!({ "index": 0 })),
        ("lodScaleJson", Value::Null),
    ];
    for (action, args) in view_dispatches {
        let args_ref = (!args.is_null()).then_some(&args);
        let result = dispatch(&mut app, action, args_ref, None).unwrap_or_else(|error| panic!("view action '{action}' must not error: {error:?}"));
        assert!(result.mutations.is_empty(), "view action '{action}' must not emit document operations");
    }
}

/// 🗂️ Grouped-context-menu disclosure: the top-level row budget stays small (leaves+groups
/// combined) and the known `deleteSelection` destructive row stays last.
#[semio_framework_async_macros::async_test]
async fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
    use semio_framework_plugin::{ContextMenuRequest, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, UiMenuRef};

    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let node_id = first_node_id(&app);
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `context_menu` reads the
    // CLIENT-supplied `request.surface.selection` now (selection is framework-owned, no live
    // config field to derive it from) — see `context_menu`'s own doc comment.
    let request = ContextMenuRequest {
        menu: UiMenuRef { id: "puzzle2d".into(), args: None },
        surface: Some(ContextMenuSurfaceTarget { surface_id: "puzzle2d".into(), kind: "board".into(), hits: Vec::new(), selection: vec![ContextMenuSelectionGroup { domain: PUZZLE2D_GRANULARITY_NODE.into(), ids: vec![node_id] }], text: None }),
        window_instance_id: None,
        point: None,
    };
    let menu = semio_framework::io::resolve_ready(app.context_menu(&request));
    assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
    let last = menu.last().expect("grouped disclosure menu should not be empty");
    assert_eq!(last.id, "deleteSelection", "the destructive row must stay last as a top-level leaf");
    assert_eq!(last.destructive, Some(true), "the destructive row must carry destructive: true");
}
//#endregion 🔖️Registry
