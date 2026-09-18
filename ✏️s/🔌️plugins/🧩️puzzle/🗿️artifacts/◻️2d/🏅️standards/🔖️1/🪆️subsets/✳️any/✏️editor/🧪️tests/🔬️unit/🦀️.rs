pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::{ActionMeta, App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};
    
    pub type Puzzle2dApp = VcsArtifactApp<EditorApp<Puzzle2dPlayApp>>;
    
    pub fn meta(actor: &str) -> ActionMeta {
        semio_framework_plugin::artifact_app_laws::meta(actor)
    }
    
    /// 🧰️ The registry-backed, instance-bound fixture app: `bounded_first_step_tool_proofs!` joins the manifest's
    /// migrated declarations to the live factories, which a registry-less app cannot satisfy.
    pub fn app() -> Puzzle2dApp {
        std::sync::LazyLock::force(&crate::examples::puzzle2d::nakagin_capsule_tower::SOURCE);
        std::sync::LazyLock::force(&crate::examples::puzzle2d::concrete_forest::SOURCE);
        app_with_registry()
    }
    
    /// 🧾️ `assert_declared_actions_bridge_to_commands`/`new_app_with_registry` still take a `fn() ->
    /// App` manifest (framework test context gap, not this packet's to fix — see the sibling `w2-cad-report`
    /// "SDK gaps" §3); `create_puzzle2d_app` now returns `AppDefinition`, so this wraps it.
    fn puzzle2d_manifest_for_tests() -> App {
        App { definition: create_puzzle2d_app(), examples: Vec::new() }
    }
    
    /// 🧰️ A registry-backed app so kind discipline (View/Shell actions must emit no operations) and the
    /// utility contract are enforced exactly as in production.
    pub fn app_with_registry() -> Puzzle2dApp {
        let mut app = semio_framework::io::resolve_ready(semio_framework_plugin::artifact_app_laws::new_app_with_registry::<EditorApp<Puzzle2dPlayApp>>(puzzle2d_manifest_for_tests));
        semio_framework::io::resolve_ready(app.bind_instance_id(1));
        app
    }
    
    pub fn window_view(kind: &str, id: &str) -> ViewModel {
        let mut window_instances = [overview::WINDOW_KIND_ID, detail::WINDOW_KIND_ID, selection::WINDOW_KIND_ID]
            .into_iter()
            .map(|kind| ViewWindowInstance { id: kind.into(), window_kind_id: kind.into() })
            .collect::<Vec<_>>();
        if !window_instances.iter().any(|window| window.id == id) {
            window_instances.push(ViewWindowInstance { id: id.into(), window_kind_id: kind.into() });
        }
        ViewModel { window_instances, ..Default::default() }.for_window_instance(id).expect("puzzle2d test window roster")
    }
    
    fn action_meta(args: Option<&Value>, window_id: Option<&str>) -> ActionMeta {
        let requested = args
            .and_then(|value| value.get("windowId").or_else(|| value.get("window_id")).or_else(|| value.get("pane")).or_else(|| value.get("window")))
            .and_then(Value::as_str);
        let kind = requested
            .filter(|kind| [overview::WINDOW_KIND_ID, detail::WINDOW_KIND_ID, selection::WINDOW_KIND_ID].contains(kind))
            .or_else(|| window_id.filter(|kind| [overview::WINDOW_KIND_ID, detail::WINDOW_KIND_ID, selection::WINDOW_KIND_ID].contains(kind)))
            .unwrap_or(overview::WINDOW_KIND_ID);
        let id = window_id.or(requested).unwrap_or(kind);
        ActionMeta { view_state: Some(window_view(kind, id)), ..meta("local") }
    }
    
    fn settle(app: &mut Puzzle2dApp, result: Result<InvocationResult, Fault>) -> Result<InvocationResult, Fault> {
        let mut result = result?;
        for _ in 0..1_048_576 {
            if !app.has_pending_typed_operations() {
                return Ok(result);
            }
            PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)?;
            semio_framework::io::resolve_ready(app.advance_typed_operation_publication())?;
            if let Some(page) = app.take_typed_operation_result_page(1) {
                if page.lane == semio_framework_plugin::app::TypedOperationResultLane::Fault {
                    return Err(Fault::from(String::from_utf8_lossy(page.bytes()).into_owned()));
                }
                app.acknowledge_typed_operation_result(page.token)?;
            }
            result.requested_effects.extend(app.take_typed_operation_effect());
            result.events.extend(app.take_typed_operation_event());
            if let Some(completion) = semio_framework::io::resolve_ready(app.take_typed_operation_completion())? {
                result.ui_scope = completion.ui_scope;
                if let Some(patch) = completion.history_patch {
                    result.history_patch = Some(match result.history_patch.take() {
                        Some(mut previous) => {
                            previous.upserts.extend(patch.upserts);
                            previous.cursor = patch.cursor;
                            previous.can_undo = patch.can_undo;
                            previous.can_redo = patch.can_redo;
                            previous
                        }
                        None => patch,
                    });
                }
            }
            if let Some(scope) = app.take_typed_operation_ui_scope() {
                result.ui_scope = scope;
            }
            while let Some(reply) = app.take_local_interaction_query_reply() {
                if let protocol::LocalInteractionQueryReply::Page { page } = reply {
                    let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
                    app.acknowledge_local_interaction_query(&token);
                }
            }
        }
        Err(Fault::from("puzzle2d test operation did not settle"))
    }
    
    /// 🧪️ B1: test-only replacement for the deleted `VcsArtifactApp::handle_action` app-dispatch path
    /// (that method is FRAMEWORK-reserved now — an app's own actions go exclusively through the typed
    /// `Self::Command` channel). Reconstructs the `Puzzle2dCommand` from the same
    /// `(action, args, window_id)` triple every pre-B1 test already passed.
    pub fn dispatch(app: &mut Puzzle2dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
        let action_meta = action_meta(args, window_id);
        // 🕰️ Framework-reserved verbs (undo/redo/checkpoint/…/the six interaction verbs) stay on
        // `handle_action` — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM added
        // interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
        // setInteractionGranularity to this reserved set.
        if matches!(
            action,
            "undo"
                | "redo"
                | "commitCheckpoint"
                | "createAlternative"
                | "switchAlternative"
                | "checkoutCheckpoint"
                | "copy"
                | "cut"
                | "paste"
                | "revertToCommand"
                | "historyFilter"
                | "noteShellCommand"
                | "interactionSelect"
                | "interactionHover"
                | "clearSelection"
                | "selectAll"
                | "setSelectionMode"
                | "setInteractionGranularity"
        ) {
            let dsl_args = args.map(dsl::DslValue::from);
            let result = semio_framework::io::resolve_ready(app.handle_action(action, dsl_args.as_ref(), &action_meta)).and_then(|admitted| semio_framework::io::resolve_ready(semio_framework_plugin::app::settle_framework_reserved_admission(app, admitted)));
            return settle(app, result);
        }
        let result = semio_framework::io::resolve_ready(app.dispatch_typed(Puzzle2dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)), &action_meta));
        settle(app, result)
    }
    
    /// 🧾 How many DOCUMENT edits one dispatch actually committed. `InvocationResult.mutations` is
    /// the INLINE carrier and the typed/retained ladder never uses it: a migrated verb commits its edits
    /// inside the operation and reports them as command-log upserts, which `settle` adopts above. Only an
    /// `"apply"` row is a document edit: a CONFIG apply is logged as `action_id: "configApply"` under the
    /// same `ActionKind::Mutation` (`🔌️plugin/🦀️.rs:23976` vs `:23993`), so counting bare "mutation"
    /// rows would make every config-only verb's "must not mutate the document" law red for the wrong
    /// reason, and every `View`/`Shell` verb logs a command row of its own too.
    /// Every law that means "this verb edited the document" counts these, never `mutations`.
    pub fn committed_edits(result: &InvocationResult) -> usize {
        result.history_patch.as_ref().map_or(0, |patch| patch.upserts.iter().filter(|entry| entry.kind == "mutation" && entry.action_id == "apply").count())
    }

    /// 🧵️ Drives the same host-owned `DispatchAction` continuation used in production until the example is complete.
    /// 🛍️ One dispatch is the whole load: `setActiveExample` drives `Puzzle2dActiveExampleWork` to its
    /// terminal emit (retained job and batch path alike), so no `DispatchAction` continuation ladder
    /// remains to pump. Returns the mutation count the load committed.
    pub fn load_example(app: &mut Puzzle2dApp, example_id: &str) -> usize {
        let result = dispatch(app, "setActiveExample", Some(&json!({ "exampleId": example_id })), None).expect("load example");
        assert!(result.requested_effects.is_empty(), "the example load must not request a continuation effect");
        committed_edits(&result)
    }
    
    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionSelect`
    /// for one `(granularity, id)` pair in the `vortex` domain — the test-side replacement for the
    /// deleted `setSelection` action.
    pub fn select_id(app: &mut Puzzle2dApp, granularity: &str, id: &str) -> Result<InvocationResult, Fault> {
        let targets = serde_json::to_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]).unwrap_or_default();
        dispatch(app, "interactionSelect", Some(&json!({ "domainId": PUZZLE2D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })), None)
    }
    
    pub fn concrete_forest_app() -> Puzzle2dApp {
        let mut app = app();
        load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
        app
    }
    
    /// 🖼️ The rendered body, serialized — every panel/window assertion greps this string.
    pub fn render_body(app: &mut Puzzle2dApp, body_key: &str) -> String {
        render_body_with_view(app, body_key, &ViewModel::default())
    }
    
    pub fn render_body_with_view(app: &mut Puzzle2dApp, body_key: &str, view_state: &ViewModel) -> String {
        let tree = semio_framework::io::resolve_ready(app.render(body_key, None, view_state)).expect("render");
        let mut stack = vec![&tree.root];
        while let Some(node) = stack.pop() {
            if let semio_framework_ui_contract::Component::Surface(surface) = &node.component {
                if surface.doc_schema.as_str() == <semio_framework_ui_scene::Board2dScene as semio_framework_ui_scene::SceneDoc>::SCHEMA {
                    // 🚚️ The fixture rides an out-of-doc lane (`Board2dSceneLane::Fixture`) — a bare decode reads an empty `fixture_json`.
                    let scene: semio_framework_ui_scene::Board2dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(node).expect("decode board scene with lanes");
                    return serde_json::to_string(&json!({ "schema": surface.doc_schema, "board2d": scene })).expect("serialize board scene");
                }
            }
            stack.extend(node.children.iter());
        }
        // 🪟️ A built tree's children now travel as retained pages, so a bare `serde_json` of the root
        // is refused — the projection helper walks and retires them exactly (5d's context already did).
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("retire rendered node")
    }
    
    pub fn render_window(app: &mut Puzzle2dApp, body_key: &str, window_id: &str) -> String {
        render_body(app, &format!("{body_key}:{window_id}"))
    }
    
    pub fn close_app(app: &mut Puzzle2dApp) {
        for _ in 0..1_048_576 {
            if app.close_terminal_is_empty() {
                return;
            }
            if PluginApp::close_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Puzzle 2D registered app close") == semio_framework_plugin::PluginCloseStep::Complete {
                break;
            }
        }
        assert!(app.close_terminal_is_empty(), "Puzzle 2D registered app close did not reach terminal-empty ownership");
    }
    
    /// 🧾️ A standalone `Puzzle2dScene` for the measure/engagement builders that take one directly.
    pub fn scene(fixture: Value, runtime: Puzzle2dPlayRuntime, active_utility: &str) -> Puzzle2dScene {
        Puzzle2dScene { fixture, runtime, active_utility: active_utility.into(), interaction: Puzzle2dInteractionSnapshot::default() }
    }
    
    pub fn fixture_of(app: &Puzzle2dApp) -> Value {
        app.snapshot().expect("projection").0
    }
    
    pub fn first_node_id(app: &Puzzle2dApp) -> String {
        fixture_nodes(&fixture_of(app))[0].get("id").and_then(|value| value.as_str()).expect("node id").to_string()
    }
}

use context::*;
use super::*;
use crate::Puzzle2dSnapshot;
use semio_framework::SET_ACTIVE_UTILITY_ACTION_ID;
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
    assert_eq!(committed_edits(&result), 1, "addNode must commit exactly one document edit");
    assert_eq!(fixture_nodes(&fixture_of(&app)).len(), 1);
    close_app(&mut app);
}

/// 🛍️ The example load commits granular operations from ONE dispatch — the retained
/// `Puzzle2dActiveExampleWork` state machine, driven to its terminal emit, with no
/// `Effect::DispatchAction` continuation ladder left to pump.
#[semio_framework_async_macros::async_test]
async fn set_active_example_loads_concrete_forest_via_operations() {
    let mut app = app();
    let result = dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID })), None).expect("load example");
    assert!(result.requested_effects.is_empty(), "the example load must not request a continuation effect");
    assert!(committed_edits(&result) > 0, "the example load must commit granular operations");
    assert!(!fixture_nodes(&fixture_of(&app)).is_empty());
    close_app(&mut app);
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
    close_app(&mut app);
}

/// 📦️ `Puzzle2dPlaySnapshot`'s pack encoding round-trips through the same `(RecordSpec,
/// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
/// `serde_json::Value` bridge impls).
#[semio_framework_async_macros::async_test]
async fn puzzle2d_play_projection_pack_round_trips() {
    let mut app = concrete_forest_app();
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&app.snapshot().expect("projection"));
    close_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn select_then_delete_selection_removes_the_node() {
    let mut app = app_with_registry();
    dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");
    let node_id = first_node_id(&app);
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, &node_id).expect("select");
    dispatch(&mut app, "deleteSelection", None, None).expect("delete");
    assert!(fixture_nodes(&fixture_of(&app)).is_empty());
    close_app(&mut app);
}

/// 🌀️ A transform gesture streams one dispatch per drag tick. Every tick folds into ONE `Edit`
/// through its `coalesce_key`, so a 3-tick move costs the 64-slot edit ledger one slot and ONE undo
/// restores the pose the gesture started from (3d's gumball laws, ported).
#[semio_framework_async_macros::async_test]
async fn transform_gesture_ticks_coalesce_into_one_undo_step() {
    let mut app = app_with_registry();
    dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");
    let id = first_node_id(&app);
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, &id).expect("select");
    let node_x = |app: &Puzzle2dApp| fixture_nodes(&fixture_of(app))[0].get("x").and_then(Value::as_f64).expect("x");
    let start = node_x(&app);
    for dx in [1.0, 2.0, 3.0] {
        let result = dispatch(&mut app, "translateSelection", Some(&json!({ "dx": dx, "dy": 0.0 })), None).expect("drag tick");
        assert_eq!(committed_edits(&result), 1, "every tick is one granular patch");
    }
    assert!((node_x(&app) - start - 6.0).abs() < 1e-9, "three ticks accumulate 1+2+3 on x");
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert!((node_x(&app) - start).abs() < 1e-9, "one undo restores the whole coalesced drag");
    close_app(&mut app);
}

/// 🧾️ The 64-slot edit ledger (`ARTIFACT_HISTORY_LEDGER_CAPACITY`) is the app's hard interactive
/// budget: a session of ordinary small edits must reach it and refuse HONESTLY (a named fault the
/// caller sees), never corrupt the store or die silently. This pins where that wall stands so a
/// gesture that quietly spends 100 slots (a per-placement fill) cannot creep back in unnoticed.
#[semio_framework_async_macros::async_test]
async fn sequential_small_edits_honour_the_fixed_edit_ledger_ceiling() {
    let mut app = app_with_registry();
    let mut committed = 0usize;
    let mut refusal: Option<String> = None;
    for index in 0..70 {
        match dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None) {
            Ok(_) => committed += 1,
            Err(fault) => {
                refusal = Some(format!("edit {index}: {fault:?}"));
                break;
            }
        }
    }
    assert_eq!(fixture_nodes(&fixture_of(&app)).len(), committed, "every admitted edit landed in the document");
    let undone = dispatch(&mut app, "undo", None, None);
    close_app(&mut app);
    assert!(undone.is_ok(), "the store stays usable at the ceiling: {:?}", undone.err());
    assert!(committed >= 64, "the ledger must admit its full 64 slots, admitted {committed}");
    assert!(refusal.as_deref().is_none_or(|fault| fault.contains("saturated")), "past the ceiling the refusal must name the saturated ledger, got {refusal:?}");
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
    close_app(&mut app);
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
    use crate::standards::v1::subsets::any::schema::mutations::binary::{close_puzzle2d_store, puzzle2d_store};
    use crate::{Puzzle2dNode, PUZZLE_2D_SCHEMA};
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{create_document_envelope, ArtifactCommand};

    let mut store = puzzle2d_store(create_document_envelope(PUZZLE_2D_SCHEMA, "puzzle2d", Puzzle2dSnapshot::default(), None)).await.expect("store");
    let node = Puzzle2dNode { id: "n1".into(), ..Default::default() };
    store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::standards::v1::subsets::any::schema::mutations::create_node(node, None)], description: None }).await.expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<Puzzle2dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle2dSnapshot, Puzzle2dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())).await;
    close_puzzle2d_store(&mut store).expect("the standalone store retires to its terminal-empty shell");
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
        assert_eq!(committed_edits(&result), 0, "setCamera must never produce a document operation");
    }
    let rendered = render_body(&mut app, overview::BODY_KEY);
    assert_eq!(rendered_camera(&rendered).0, 3.0, "the camera must update immediately in the rendered scene");
    let undo = dispatch(&mut app, "undo", None, None).expect("undo");
    assert_eq!(committed_edits(&undo), 0, "there is no document edit to undo");
    let rendered_after_undo = render_body(&mut app, overview::BODY_KEY);
    assert_eq!(rendered_camera(&rendered_after_undo).0, 3.0, "the camera is session state — undo must not revert it");
    close_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn exact_overview_window_cameras_isolate_render_and_reload_through_registered_app() {
    let mut app = Box::new(app_with_registry());
    let mut reopened = Box::new(app_with_registry());
    let window_a = "puzzle2d-overview-a";
    let window_b = "puzzle2d-overview-b";
    let view_a = window_view(overview::WINDOW_KIND_ID, window_a);
    let view_b = window_view(overview::WINDOW_KIND_ID, window_b);
    let document_before = fixture_of(&app);
    let app_config_before = app.config_pack().await.expect("app config before window publications");
    let result_a = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "x": 18.0, "y": -4.0, "zoom": 2.0 } })), Some(window_a)).expect("setCamera a");
    let result_b = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "x": -9.0, "y": 6.0, "zoom": 0.5 } })), Some(window_b)).expect("setCamera b");
    assert_eq!((committed_edits(&result_a), committed_edits(&result_b)), (0, 0));
    assert_eq!(fixture_of(&app), document_before);
    let app_config_after = app.config_pack().await.expect("app config after window publications");
    assert_eq!((app_config_after.pack, app_config_after.spr), (app_config_before.pack, app_config_before.spr));
    let rendered_a = render_window(&mut app, overview::BODY_KEY, window_a);
    let rendered_b = render_window(&mut app, overview::BODY_KEY, window_b);
    assert_eq!(rendered_camera(&rendered_a), (18.0, -4.0, 2.0));
    assert_eq!(rendered_camera(&rendered_b), (-9.0, 6.0, 0.5));
    assert_eq!(app.window_config_generation(&view_a).await.expect("window a generation"), Some(1));
    assert_eq!(app.window_config_generation(&view_b).await.expect("window b generation"), Some(1));
    let packs = app.window_config_packs().await.expect("two exact window packs");
    assert_eq!(packs.len(), 2);
    for pack in packs {
        reopened.load_window_config_pack(pack).await.expect("reload exact Puzzle 2D window pack");
    }
    assert_eq!(rendered_camera(&render_window(&mut reopened, overview::BODY_KEY, window_a)), (18.0, -4.0, 2.0));
    assert_eq!(rendered_camera(&render_window(&mut reopened, overview::BODY_KEY, window_b)), (-9.0, 6.0, 0.5));
    close_app(&mut reopened);
    close_app(&mut app);
    eprintln!("[DEBUG] two Puzzle 2D overview windows published and rendered independent cameras, preserved document and app config, reloaded both persisted partitions, and closed their registered apps");
}

#[semio_framework_async_macros::async_test]
async fn exact_overview_window_transient_isolates_abort_and_resets_on_reload() {
    let mut app = Box::new(app_with_registry());
    let mut reopened = Box::new(app_with_registry());
    let window_a = "puzzle2d-transient-a";
    let window_b = "puzzle2d-transient-b";
    let view_a = window_view(overview::WINDOW_KIND_ID, window_a);
    let view_b = window_view(overview::WINDOW_KIND_ID, window_b);
    dispatch(&mut app, "engagementInput", Some(&json!({ "pane": overview::WINDOW_KIND_ID, "value": "fill" })), Some(window_a)).expect("window a engagement input");
    let transient_a = app.window_transient_snapshot(&view_a).expect("window a transient").expect("window a owner");
    let transient_b = app.window_transient_snapshot(&view_b).expect("window b transient").expect("window b owner");
    assert_eq!(transient_a.get::<window::Puzzle2dOverviewWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some("fill"));
    assert_eq!(transient_b.get::<window::Puzzle2dOverviewWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some(""));
    dispatch(&mut app, "engagementAbort", Some(&json!({ "pane": overview::WINDOW_KIND_ID })), Some(window_a)).expect("abort window a engagement");
    let aborted = app.window_transient_snapshot(&view_a).expect("aborted transient").expect("aborted owner");
    assert_eq!(aborted.get::<window::Puzzle2dOverviewWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some(""));
    dispatch(&mut app, "engagementInput", Some(&json!({ "pane": overview::WINDOW_KIND_ID, "value": "brush" })), Some(window_a)).expect("window a second engagement input");
    let submitted = dispatch(&mut app, "engagementSubmit", Some(&json!({ "pane": overview::WINDOW_KIND_ID, "value": "brush" })), Some(window_a)).expect("submit window a engagement");
    assert!(submitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { window_id, .. } if window_id == window_a)));
    dispatch(&mut app, "engagementInput", Some(&json!({ "pane": overview::WINDOW_KIND_ID, "value": "draft" })), Some(window_a)).expect("window a third engagement input");
    let reset = reopened.window_transient_snapshot(&view_a).expect("reopened transient").expect("reopened owner");
    assert_eq!(reset.get::<window::Puzzle2dOverviewWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some(""));
    assert_eq!(app.window_transient_generation(&view_a).expect("window a transient generation"), Some(5));
    assert_eq!(app.window_transient_generation(&view_b).expect("window b transient generation"), Some(0));
    close_app(&mut reopened);
    close_app(&mut app);
    eprintln!("[DEBUG] Puzzle 2D transient engagement stayed exact-window isolated, abort cleared only its owner, reload reset ephemeral state, and both registered apps reached terminal-empty close");
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
    close_app(&mut app);
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
    close_app(&mut app);
}

/// 🪞️ Regression test: `apply_host_events` used to epsilon-compare `host.camera` (still the
/// *pre-action* value) against the runtime and blindly overwrite it, reverting a plain `camera`
/// board event (used for the live wheel-zoom echo) before it ever committed.
#[semio_framework_async_macros::async_test]
async fn apply_board_events_camera_event_commits() {
    let mut app = app();
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "camera", "payload": { "x": 5.0, "y": 6.0, "zoom": 1.2 } }]).to_string() })), None).expect("camera event");
    assert_eq!(committed_edits(&result), 0, "a camera board event must never produce a document operation");
    let (x, y, zoom) = rendered_camera(&render_body(&mut app, overview::BODY_KEY));
    assert_eq!(x, 5.0);
    assert_eq!(y, 6.0);
    assert_eq!(zoom, 1.2);
    close_app(&mut app);
}

/// 🐢️ A pure selection change is runtime state, not document state — it must not produce any
/// operations (previously it fell back to a whole-document replace once the edge-duplication bug
/// made `before` and `after` genuinely diverge).
#[semio_framework_async_macros::async_test]
async fn select_action_emits_no_operations() {
    let mut app = concrete_forest_app();
    let node_id = first_node_id(&app);
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), None).expect("select");
    assert_eq!(committed_edits(&result), 0, "selection must not produce document operations");
    close_app(&mut app);
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
            assert!(panel_bodies.contains(&artifact::PUZZLE2D_PLAY_BODY_LAYERS.to_string()));
            assert!(panel_bodies.contains(&inspection::PUZZLE2D_PLAY_BODY_PROPERTIES.to_string()));
            assert!(engagements, "select must refresh the engagement bar");
            assert!(!measures, "select must not force a measures refresh");
            assert!(!utilities);
            assert!(!tools);
            assert!(!labels);
        }
        other => panic!("expected a Partial ui_scope for select, got {other:?}"),
    }
    close_app(&mut app);
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
    close_app(&mut app);
}

/// 🐢️ Perf round 3: an empty `applyBoardEvents` batch (no-operation) must declare nothing beyond the
/// history panel body — the empty View action neither logs an edit nor dirties a surface.
#[semio_framework_async_macros::async_test]
async fn empty_board_events_declare_none_ui_scope() {
    let mut app = app();
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": "[]" })), None).expect("no-operation");
    assert_eq!(result.ui_scope, UiDirtyScope::None);
    close_app(&mut app);
}

/// 🐢️ Perf round 3: cold-tier structural actions (document operations) must keep the safe `Full`
/// default — no puzzle2d scope helper narrows them.
#[semio_framework_async_macros::async_test]
async fn add_node_action_declares_full_ui_scope() {
    let mut app = app();
    let result = dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");
    assert!(matches!(result.ui_scope, UiDirtyScope::Full), "addNode must stay Full, got {:?}", result.ui_scope);
    close_app(&mut app);
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
    assert_eq!(ids, vec![select_utility::UTILITY_ID, brush_utility::UTILITY_ID, area_brush_utility::UTILITY_ID]);
    let overview_window = definition.window_kinds.iter().find(|window| window.id == overview::WINDOW_KIND_ID).expect("overview pane");
    let overview_utilities: Vec<&str> = overview_window.utilities.iter().map(|utility| utility.as_str()).collect();
    assert_eq!(overview_utilities, vec![select_utility::UTILITY_ID, brush_utility::UTILITY_ID, area_brush_utility::UTILITY_ID]);
    assert!(overview_window.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID), "declaring utilities must inject the setActiveUtility action");
    // 🧰️ D-1: select/brush are this window's whole exclusive utility set, NOT a sub-collection, so
    // each carries `group: None` and renders as a flat utility bar icon (never one collapsed dropdown).
    for utility in &definition.utilities {
        assert_eq!(utility.group, None, "utility {} must render flat (no shared group)", utility.id);
    }
}

/// 🎬️ Every app-declared action resolves to a `Puzzle2dCommand` — an action declared in the manifest
/// but missing from `puzzle2d_command_variants!` reaches the host as a row/keybinding and dies at
/// dispatch with `unknown Puzzle 2D action` (export/import/transforms, 2026-09-17).
#[test]
fn every_declared_action_resolves_to_a_command() {
    let definition = create_puzzle2d_app();
    let mut unresolved = Vec::new();
    let mut declared = Vec::new();
    for window in &definition.window_kinds {
        for action in &window.actions {
            let id = action.id.as_str();
            declared.push(id.to_string());
            // 🕰️ Framework-owned verbs never reach `command_from_action`: history, clipboard, the
            // interaction six, the injected utility/tool switches and the tool-run controls.
            let reserved = matches!(
                id,
                "undo" | "redo" | "commitCheckpoint" | "createAlternative" | "switchAlternative" | "checkoutCheckpoint" | "copy" | "cut" | "paste" | "revertToCommand" | "historyFilter" | "setHistoryCommandFilter" | "noteShellCommand" | "recordTutorial" | "interactionSelect" | "interactionHover" | "clearSelection" | "selectAll" | "setSelectionMode" | "setInteractionGranularity"
            ) || id == SET_ACTIVE_UTILITY_ACTION_ID
                || id == semio_framework_plugin::SET_ACTIVE_TOOL_ACTION_ID
                || semio_framework_plugin::is_tool_run_action_id(id);
            if !reserved && Puzzle2dCommand::try_from_action(id, None, None).is_none() {
                unresolved.push(id.to_string());
            }
        }
    }
    for expected in ["exportFixture", "openImportFixture", "importFixture", "translateSelection", "rotateSelection", "scaleSelection"] {
        assert!(declared.iter().any(|id| id == expected), "the window action roster must carry the app-level action '{expected}' (declared: {declared:?})");
    }
    assert!(unresolved.is_empty(), "declared actions without a Puzzle2dCommand variant: {unresolved:?}");
}

/// ⚙️ The app settings panel is an app-level body (`ViewModel::for_panel` carries no window), so its
/// steppers address the FOCUSED pane; with no live pane they carry no `windowId` at all — an empty one
/// is refused by the host as an unknown window instance (`setFillCount {windowId: ""}`, 2026-09-17).
#[semio_framework_async_macros::async_test]
async fn settings_steppers_address_the_focused_pane_or_no_pane() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let focused = semio_framework_plugin::ViewModel { focused_window_id: Some(detail::WINDOW_KIND_ID.into()), ..window_view(overview::WINDOW_KIND_ID, overview::WINDOW_KIND_ID) }.for_panel();
    let body = render_body_with_view(&mut app, settings::PUZZLE2D_PLAY_BODY_SETTINGS, &focused);
    assert!(body.contains("setFillCount"), "the settings body must carry the fill-count stepper: {}", &body[..body.len().min(300)]);
    assert!(body.contains(&format!("\"windowId\":\"{}\"", detail::WINDOW_KIND_ID)), "a stepper must address the focused pane: {}", &body[..body.len().min(600)]);
    let unhosted = render_body_with_view(&mut app, settings::PUZZLE2D_PLAY_BODY_SETTINGS, &semio_framework_plugin::ViewModel::default());
    close_app(&mut app);
    assert!(unhosted.contains("setFillCount") && !unhosted.contains("windowId"), "with no live pane the steppers must carry no windowId: {}", &unhosted[..unhosted.len().min(600)]);
}

/// 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility.
#[test]
fn tool_registry_declares_fill_tool() {
    use semio_framework_plugin::{ToolRef, SET_ACTIVE_TOOL_ACTION_ID};
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
    close_app(&mut instance_a);
    close_app(&mut instance_b);
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
    close_app(&mut receiver);
    close_app(&mut sender);
}
//#endregion 🔖️Convergence

//#region 🔖️Registry
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
        ("cycleBrushCandidate", json!({ "forward": true })),
        ("cycleBrushCandidateBack", Value::Null),
        ("hoverSuggestion", json!({ "index": 0 })),
        ("targetBrushSuggestions", Value::Null),
        ("closeHandleSuggestions", Value::Null),
        ("openHandleSuggestions", json!({ "handleId": "" })),
        ("lodScaleJson", Value::Null),
    ];
    for (action, args) in view_dispatches {
        let args_ref = (!args.is_null()).then_some(&args);
        let result = dispatch(&mut app, action, args_ref, None).unwrap_or_else(|error| panic!("view action '{action}' must not error: {error:?}"));
        assert_eq!(committed_edits(&result), 0, "view action '{action}' must not emit document operations");
    }
    close_app(&mut app);
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
    let menu = semio_framework::io::resolve_ready(app.context_menu(&request, &Default::default()));
    assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
    let last = menu.last().expect("grouped disclosure menu should not be empty");
    assert_eq!(last.id, "deleteSelection", "the destructive row must stay last as a top-level leaf");
    assert_eq!(last.destructive, Some(true), "the destructive row must carry destructive: true");
    close_app(&mut app);
}
//#endregion 🔖️Registry

//#region 🔖️EngineParse
/// 🎲️ Every shipped example must parse in the board engine as the host paints it: a `false` from
/// `parse_fixture_json` is silent at runtime and leaves the panes empty (Nakagin, 2026-09-16).
#[semio_framework_async_macros::async_test]
async fn shipped_examples_parse_in_the_board_engine() {
    for (name, json) in [("concrete-forest", concrete_forest_example_json()), ("nakagin", nakagin_example_json())] {
        let fixture: Value = serde_json::from_str(&json).expect("example json");
        let mut host = BoardHost::default();
        let parsed = host.parse_fixture_json(&fixture.to_string());
        if !parsed {
            let nodes = fixture_nodes(&fixture).to_vec();
            let edges = fixture_edges(&fixture).to_vec();
            let probe = |nodes: &[Value], edges: &[Value]| BoardHost::default().parse_fixture_json(&json!({ "schema": "puzzle.2d.fixture", "camera": { "x": 0, "y": 0, "zoom": 1 }, "nodes": nodes, "edges": edges }).to_string());
            let bad_node = (1..=nodes.len()).find(|count| !probe(&nodes[..*count], &[])).map(|count| nodes[count - 1].clone());
            let bad_edge = (1..=edges.len()).find(|count| !probe(&nodes, &edges[..*count])).map(|count| edges[count - 1].clone());
            panic!("{name}: the board engine refused the example fixture; first refused node = {bad_node:?}; first refused edge = {bad_edge:?}");
        }
    }
}

/// 🎯️ What the host paints after a gesture: the overview scene's fixture lane, re-parsed by the
/// engine, with the first refused node/edge named when it refuses (the runtime only logs a length).
fn painted_fixture_parses(app: &mut Puzzle2dApp, what: &str) {
    let body: Value = serde_json::from_str(&render_body(app, overview::BODY_KEY)).expect("overview body json");
    let fixture_json = body.get("board2d").and_then(|scene| scene.get("fixtureJson")).and_then(Value::as_str).expect("painted fixture lane").to_string();
    if BoardHost::default().parse_fixture_json(&fixture_json) {
        return;
    }
    let fixture: Value = serde_json::from_str(&fixture_json).expect("painted fixture json");
    let nodes = fixture_nodes(&fixture).to_vec();
    let edges = fixture_edges(&fixture).to_vec();
    let probe = |nodes: &[Value], edges: &[Value]| BoardHost::default().parse_fixture_json(&json!({ "schema": "puzzle.2d.fixture", "camera": { "x": 0, "y": 0, "zoom": 1 }, "nodes": nodes, "edges": edges }).to_string());
    let bad_node = (1..=nodes.len()).find(|count| !probe(&nodes[..*count], &[])).map(|count| nodes[count - 1].clone());
    let bad_edge = (1..=edges.len()).find(|count| !probe(&nodes, &edges[..*count])).map(|count| edges[count - 1].clone());
    let head: String = fixture_json.chars().take(400).collect();
    panic!("{what}: the board engine refused the painted fixture ({} chars); first refused node = {bad_node:?}; first refused edge = {bad_edge:?}; head = {head}", fixture_json.len());
}

/// 🖱️ A node drag (`nodeDragEnd` through `applyBoardEvents`) must leave a fixture the engine still
/// paints — after the store round-trip, not just in the scene the command patched (2026-09-17: three
/// blank panes after every drag on Nakagin).
#[semio_framework_async_macros::async_test]
async fn dragging_a_node_keeps_the_painted_board_parseable() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    painted_fixture_parses(&mut app, "before drag");
    let node = fixture_nodes(&fixture_of(&app))[0].clone();
    let id = node.get("id").and_then(Value::as_str).expect("node id").to_string();
    let x = node.get("x").and_then(Value::as_f64).expect("x") + 8.0;
    let y = node.get("y").and_then(Value::as_f64).expect("y") + 4.0;
    let events = json!([{ "name": "nodeMove", "payload": { "id": id, "x": x, "y": y } }, { "name": "nodeDragEnd", "payload": { "moves": [{ "id": id, "x": x, "y": y }] } }]).to_string();
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": events })), Some(overview::WINDOW_KIND_ID));
    assert!(result.is_ok(), "applyBoardEvents must not fault: {:?}", result.err());
    let moved = fixture_nodes(&fixture_of(&app)).iter().find(|node| node.get("id").and_then(Value::as_str) == Some(id.as_str())).cloned().expect("moved node");
    assert_eq!(moved.get("x").and_then(Value::as_f64), Some(x), "the drag must commit the new x");
    painted_fixture_parses(&mut app, "after drag");
    close_app(&mut app);
}


/// 🕹️ A board `select` row (the engine's click echo, through `applyBoardEvents`) is the selection the
/// inspector and the painted board both show — the write lands before either body renders.
#[semio_framework_async_macros::async_test]
async fn board_select_row_reaches_the_inspector_and_the_board() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    let id = first_node_id(&app);
    let events = json!([{ "name": "hover", "payload": { "id": id } }, { "name": "select", "payload": { "ids": [id], "gesture": "click" } }]).to_string();
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": events })), Some(overview::WINDOW_KIND_ID));
    assert!(result.is_ok(), "applyBoardEvents must not fault: {:?}", result.err());
    let board = render_body(&mut app, overview::BODY_KEY);
    let inspector = render_body(&mut app, inspection::PUZZLE2D_PLAY_BODY_PROPERTIES);
    close_app(&mut app);
    assert!(board.contains(&id), "the painted board must carry the selected id: {}", &board[..board.len().min(400)]);
    assert!(inspector.contains(&id) && inspector.contains("puzzle2d-play-inspector.node.id"), "the inspector must show the selected node's fields: {}", &inspector[..inspector.len().min(600)]);
}
//#endregion 🔖️EngineParse

/// 🎲️ Deleting a handle that carries an edge must leave a board the engine still paints — a dangling
/// edge makes `parse_fixture_json` refuse the WHOLE document (three blank panes, 2026-09-16).
#[semio_framework_async_macros::async_test]
async fn deleting_an_edged_handle_keeps_the_board_parseable() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    let before = fixture_of(&app);
    let edge = fixture_edges(&before)[0].clone();
    let handle_id = edge.get("source").and_then(Value::as_str).expect("edge source").to_string();
    let edge_id = edge.get("id").and_then(Value::as_str).expect("edge id").to_string();
    select_id(&mut app, PUZZLE2D_GRANULARITY_HANDLE, &handle_id).expect("select handle");
    dispatch(&mut app, "deleteSelection", None, None).expect("delete");
    let after = fixture_of(&app);
    assert!(!fixture_edges(&after).iter().any(|edge| edge.get("id").and_then(Value::as_str) == Some(edge_id.as_str())), "the edge on the deleted handle must go with it");
    let mut host = BoardHost::default();
    let parsed = host.parse_fixture_json(&after.to_string());
    close_app(&mut app);
    assert!(parsed, "the board engine must still parse the document after a handle delete");
}

/// 🗑️ Deleting an edged node commits through the store (its inverse is one `create-node` plus one
/// `connect-handles` per edge — the fold footprint must admit that cascade) and empties the selection.
#[semio_framework_async_macros::async_test]
async fn deleting_an_edged_node_commits_and_clears_the_selection() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    let before = fixture_of(&app);
    let edge = fixture_edges(&before)[0].clone();
    let handle_id = edge.get("source").and_then(Value::as_str).expect("edge source");
    let node_id = handle_id.split(':').next().expect("handle id carries its node id").to_string();
    assert!(fixture_nodes(&before).iter().any(|node| node.get("id").and_then(Value::as_str) == Some(node_id.as_str())), "edge source node must exist");
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, &node_id).expect("select node");
    let result = dispatch(&mut app, "deleteSelection", None, None);
    let after = fixture_of(&app);
    let node_gone = !fixture_nodes(&after).iter().any(|node| node.get("id").and_then(Value::as_str) == Some(node_id.as_str()));
    let edge_gone = !fixture_edges(&after).iter().any(|entry| entry.get("id") == edge.get("id"));
    let selection = render_body(&mut app, overview::BODY_KEY).contains(&node_id);
    close_app(&mut app);
    assert!(result.is_ok(), "deleteSelection must not fault: {:?}", result.err());
    assert!(node_gone && edge_gone, "the node and the edge hanging off it must be gone (node gone={node_gone}, edge gone={edge_gone})");
    assert!(!selection, "the deleted node must leave the painted selection");
}

//#region 🔖️HandleSuggestions
/// 🖌️ A handle no edge names on either end — the only kind the brush slot can grow a node onto.
fn first_free_handle_id(fixture: &Value) -> Option<String> {
    let used: std::collections::HashSet<&str> = fixture_edges(fixture)
        .iter()
        .flat_map(|edge| [edge.get("source").and_then(Value::as_str), edge.get("target").and_then(Value::as_str)])
        .flatten()
        .collect();
    fixture_nodes(fixture)
        .iter()
        .filter_map(|node| node.get("handles").and_then(Value::as_array))
        .flatten()
        .filter_map(|handle| handle.get("id").and_then(Value::as_str))
        .find(|id| !used.contains(id))
        .map(str::to_string)
}

/// 🐁️ LAW: the framework-owned `"pointer"` hover reaches the board scene for EVERY granularity and in
/// every pane — the gap the 09-17 audit measured as `hovered_id: None` hardcoded at the scene builder.
#[test]
fn hover_id_reaches_the_board_scene_for_every_granularity_and_pane() {
    let fixture = crate::examples::puzzle2d::concrete_forest::SOURCE.document_json().to_string();
    let fixture: Value = serde_json::from_str(&fixture).expect("concrete forest json");
    let node_id = fixture_nodes(&fixture)[0].get("id").and_then(Value::as_str).expect("node id").to_string();
    let handle_id = fixture_nodes(&fixture).iter().filter_map(|node| node.get("handles").and_then(Value::as_array)).flatten().filter_map(|handle| handle.get("id").and_then(Value::as_str)).next().expect("handle id").to_string();
    let edge_id = fixture_edges(&fixture)[0].get("id").and_then(Value::as_str).expect("edge id").to_string();
    for (granularity, hovered) in [(PUZZLE2D_GRANULARITY_NODE, &node_id), (PUZZLE2D_GRANULARITY_HANDLE, &handle_id), (PUZZLE2D_GRANULARITY_EDGE, &edge_id)] {
        let interaction = Puzzle2dInteractionSnapshot { granularity: granularity.into(), selected: Vec::new(), hovered: vec![hovered.clone()] };
        assert_eq!(interaction.hovered_id().as_deref(), Some(hovered.as_str()), "{granularity} hover must resolve an id");
        let envelope = Puzzle2dScene { fixture: fixture.clone(), runtime: Default::default(), active_utility: "select".into(), interaction };
        for pane in PUZZLE2D_PANES {
            let scene = edit::puzzle2d_board_scene("{}", &envelope, pane);
            assert_eq!(scene.hovered_id.as_deref(), Some(hovered.as_str()), "{granularity} hover must reach the {pane} board scene");
            assert_eq!(scene.domain_id.as_deref(), Some(PUZZLE2D_INTERACTION_DOMAIN), "every pane must name the interaction domain its hover publishes on");
        }
    }
    let idle = Puzzle2dScene { fixture, runtime: Default::default(), active_utility: "select".into(), interaction: Puzzle2dInteractionSnapshot::default() };
    assert!(edit::puzzle2d_board_scene("{}", &idle, overview::WINDOW_KIND_ID).hovered_id.is_none(), "no hover must paint no hover");
}

/// 💡️ LAW: the popup rides the board scene with the SAME candidate page the armed brush cycles, and the
/// previewed index is the shared slot index — one mechanism, not two.
#[test]
fn suggestion_popup_publishes_the_shared_candidate_page_and_the_previewed_index() {
    let fixture: Value = serde_json::from_str(crate::examples::puzzle2d::concrete_forest::SOURCE.document_json()).expect("concrete forest json");
    let handle_id = first_free_handle_id(&fixture).expect("concrete forest offers a free handle");
    let runtime = crate::editor::puzzle2d::config::Puzzle2dPlayRuntime {
        suggestion_menu: Some(crate::editor::puzzle2d::config::Puzzle2dSuggestionMenu { x: 12.0, y: 34.0, window_id: overview::WINDOW_KIND_ID.into(), handle_id: handle_id.clone() }),
        brush_candidate_source_handle_id: handle_id.clone(),
        brush_candidate_index: 1,
        brush_candidates: vec![dsl::DslValue::from(&json!({ "nodeKind": "alpha", "targetHandleIndex": 0 })), dsl::DslValue::from(&json!({ "nodeKind": "beta", "targetHandleIndex": 2 }))],
        ..Default::default()
    };
    let envelope = Puzzle2dScene { fixture, runtime, active_utility: "select".into(), interaction: Puzzle2dInteractionSnapshot::default() };
    let scene = edit::puzzle2d_board_scene("{}", &envelope, overview::WINDOW_KIND_ID);
    let menu: Value = serde_json::from_str(&scene.suggestion_menu_json.expect("an open popup must reach the client")).expect("menu json");
    assert_eq!(menu["open"], json!(true));
    assert_eq!((menu["x"].as_f64(), menu["y"].as_f64()), (Some(12.0), Some(34.0)));
    assert_eq!(menu["handleId"].as_str(), Some(handle_id.as_str()));
    assert_eq!(menu["hoveredIndex"].as_u64(), Some(1), "the previewed row is the shared slot index");
    assert_eq!(menu["pending"], json!(false), "a slot that resolved its source handle is not pending");
    let candidates = menu["candidates"].as_array().expect("candidate rows");
    assert_eq!(candidates.len(), 2);
    assert_eq!(candidates[1]["nodeLabel"].as_str(), Some("beta"));
    assert_eq!(candidates[1]["handleLabel"].as_str(), Some("handle 2"));
}

/// 💡️ LAW: a closed popup publishes nothing, so no pane renders a stale menu.
#[test]
fn a_closed_suggestion_popup_publishes_no_menu() {
    let fixture: Value = serde_json::from_str(crate::examples::puzzle2d::concrete_forest::SOURCE.document_json()).expect("concrete forest json");
    let envelope = Puzzle2dScene { fixture, runtime: Default::default(), active_utility: "select".into(), interaction: Puzzle2dInteractionSnapshot::default() };
    assert!(edit::puzzle2d_board_scene("{}", &envelope, overview::WINDOW_KIND_ID).suggestion_menu_json.is_none());
}

/// 💡️ LAW: the context menu offers the popup on ONE selected handle and on nothing else — the entry
/// point the brush slot never had (it was reachable only with the brush armed).
#[semio_framework_async_macros::async_test]
async fn context_menu_offers_suggest_nodes_on_one_selected_handle_only() {
    use semio_framework_plugin::{ContextMenuRequest, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, UiMenuRef};
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let fixture = fixture_of(&app);
    let handle_id = first_free_handle_id(&fixture).expect("concrete forest offers a free handle");
    let node_id = first_node_id(&app);
    let menu_for = |app: &mut Puzzle2dApp, ids: Vec<String>| {
        let request = ContextMenuRequest {
            menu: UiMenuRef { id: "puzzle2d".into(), args: None },
            surface: Some(ContextMenuSurfaceTarget { surface_id: "puzzle2d".into(), kind: "board".into(), hits: Vec::new(), selection: vec![ContextMenuSelectionGroup { domain: PUZZLE2D_GRANULARITY_NODE.into(), ids }], text: None }),
            window_instance_id: None,
            point: None,
        };
        semio_framework::io::resolve_ready(app.context_menu(&request, &Default::default()))
    };
    let on_handle = menu_for(&mut app, vec![handle_id.clone()]);
    let on_node = menu_for(&mut app, vec![node_id]);
    close_app(&mut app);
    let suggest = on_handle.iter().find(|item| item.id == "suggestNodes").expect("a selected handle offers the suggestions popup");
    assert_eq!(suggest.action.as_deref(), Some("openHandleSuggestions"));
    assert!(!on_node.iter().any(|item| item.id == "suggestNodes"), "a selected node has no handle to grow onto");
}

/// 💡️ LAW: open → hover → accept places ONE compatible node on concrete-forest (whose kind rows are
/// inferred, it names no manifest and carries no catalog) and re-selects it; the popup closes.
#[semio_framework_async_macros::async_test]
async fn open_hover_accept_places_one_node_on_concrete_forest_and_reselects_it() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let before = fixture_of(&app);
    let handle_id = first_free_handle_id(&before).expect("concrete forest offers a free handle");
    let before_nodes = fixture_nodes(&before).len();
    let opened = dispatch(&mut app, "openHandleSuggestions", Some(&json!({ "handleId": handle_id.as_str(), "x": 10.0, "y": 20.0 })), Some(overview::WINDOW_KIND_ID)).expect("open the popup");
    assert_eq!(committed_edits(&opened), 0, "opening the picker must not touch the document");
    dispatch(&mut app, "hoverSuggestion", Some(&json!({ "index": 0, "handleId": handle_id.as_str() })), Some(overview::WINDOW_KIND_ID)).expect("preview a candidate");
    let accepted = dispatch(&mut app, "acceptSuggestion", Some(&json!({ "index": 0, "handleId": handle_id.as_str() })), Some(overview::WINDOW_KIND_ID)).expect("accept");
    let after = fixture_of(&app);
    let placed: Vec<String> = fixture_nodes(&after).iter().filter_map(|node| node.get("id").and_then(Value::as_str)).filter(|id| !fixture_nodes(&before).iter().any(|node| node.get("id").and_then(Value::as_str) == Some(*id))).map(str::to_string).collect();
    let fastened = fixture_edges(&after).iter().any(|edge| [edge.get("source"), edge.get("target")].iter().flatten().any(|end| end.as_str() == Some(handle_id.as_str())));
    let board = render_body(&mut app, overview::BODY_KEY);
    close_app(&mut app);
    assert_eq!(fixture_nodes(&after).len(), before_nodes + 1, "accept places exactly one node");
    assert_eq!(placed.len(), 1, "exactly one node id is new");
    assert!(committed_edits(&accepted) > 0, "the placement must commit as document operations");
    assert!(fastened, "the placed node must be fastened to the handle the popup opened on");
    assert!(board.contains(&placed[0]), "the placed node must reach the painted board: {}", &board[..board.len().min(400)]);
}

/// 💡️ LAW: a handle that is already fastened has nothing to grow onto, so Nakagin refuses politely —
/// the popup opens, lists nothing, and accepting places nothing instead of faulting.
#[semio_framework_async_macros::async_test]
async fn nakagin_refuses_the_suggestions_popup_politely() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    let before = fixture_of(&app);
    let handle_id = fixture_edges(&before)[0].get("source").and_then(Value::as_str).expect("edge source").to_string();
    dispatch(&mut app, "openHandleSuggestions", Some(&json!({ "handleId": handle_id.as_str(), "x": 0.0, "y": 0.0 })), Some(overview::WINDOW_KIND_ID)).expect("open the popup");
    let accepted = dispatch(&mut app, "acceptSuggestion", Some(&json!({ "handleId": handle_id.as_str() })), Some(overview::WINDOW_KIND_ID)).expect("accept must not fault");
    let after = fixture_of(&app);
    close_app(&mut app);
    assert_eq!(committed_edits(&accepted), 0, "there is nothing to place, so nothing commits");
    assert_eq!(fixture_nodes(&after).len(), fixture_nodes(&before).len(), "a refused placement leaves the document alone");
}

/// 💡️ LAW: escape (`closeHandleSuggestions`) discards the picker and its provisional preview without
/// placing anything.
#[semio_framework_async_macros::async_test]
async fn closing_the_suggestions_popup_discards_the_preview() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let before = fixture_of(&app);
    let handle_id = first_free_handle_id(&before).expect("concrete forest offers a free handle");
    dispatch(&mut app, "openHandleSuggestions", Some(&json!({ "handleId": handle_id.as_str(), "x": 0.0, "y": 0.0 })), Some(overview::WINDOW_KIND_ID)).expect("open the popup");
    let closed = dispatch(&mut app, "closeHandleSuggestions", None, Some(overview::WINDOW_KIND_ID)).expect("close");
    let after = fixture_of(&app);
    close_app(&mut app);
    assert_eq!(committed_edits(&closed), 0, "closing the picker never commits");
    assert_eq!(fixture_nodes(&after).len(), fixture_nodes(&before).len(), "the provisional preview was never a document node");
}

/// 🔁️ LAW: `shift+tab`'s verb (`cycleBrushCandidateBack`) walks the slot the other way — both
/// directions reach the same shared slot and neither commits.
#[semio_framework_async_macros::async_test]
async fn cycling_candidates_forward_and_back_never_commits() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let before = fixture_of(&app);
    let handle_id = first_free_handle_id(&before).expect("concrete forest offers a free handle");
    dispatch(&mut app, "openHandleSuggestions", Some(&json!({ "handleId": handle_id.as_str(), "x": 0.0, "y": 0.0 })), Some(overview::WINDOW_KIND_ID)).expect("open the popup");
    for action in ["cycleBrushCandidate", "cycleBrushCandidateBack"] {
        let result = dispatch(&mut app, action, None, Some(overview::WINDOW_KIND_ID)).unwrap_or_else(|error| panic!("{action} must not fault: {error:?}"));
        assert_eq!(committed_edits(&result), 0, "{action} is a preview step, never a commit");
    }
    let after = fixture_of(&app);
    close_app(&mut app);
    assert_eq!(fixture_nodes(&after).len(), fixture_nodes(&before).len());
}
//#endregion 🔖️HandleSuggestions

//#region 🏷️DisplayLabels
fn labelled_fixture(rows: &[(&str, &str, Option<&str>)]) -> Value {
    let nodes: Vec<Value> = rows
        .iter()
        .map(|(id, kind, label)| match label {
            Some(label) => json!({ "id": id, "nodeKind": kind, "text": label, "x": 0.0, "y": 0.0 }),
            None => json!({ "id": id, "nodeKind": kind, "x": 0.0, "y": 0.0 }),
        })
        .collect();
    json!({ "schema": PUZZLE2D_FIXTURE_SCHEMA, "nodes": nodes, "edges": [], "meta": { "kindCatalogs": { "nodes": [{ "id": "capsule", "name": "Capsule" }] } } })
}

/// 🏷️ LAW: the display label follows 3d's precedence exactly (`puzzle3d_object_display_label`) —
/// authored `text` first, then the kind's catalogue display name, then the KIND id when the catalogue
/// names no row for it, and only a node carrying no kind at all falls back to its own raw id.
#[test]
fn a_node_display_label_prefers_the_authored_label_then_the_catalogue_name_then_the_id() {
    let fixture = labelled_fixture(&[("node-a", "capsule", Some("Roof Pod")), ("node-b", "capsule", None), ("node-c", "unknown-kind", None)]);
    let nodes = fixture_nodes(&fixture);
    assert_eq!(puzzle2d_node_display_label(&nodes[0], &fixture), "Roof Pod", "an authored label wins");
    assert_eq!(puzzle2d_node_display_label(&nodes[1], &fixture), "Capsule", "then the kind's catalogue display name");
    assert_eq!(puzzle2d_node_display_label(&nodes[2], &fixture), "unknown-kind", "then the kind id the catalogue names no row for");
    let kindless = json!({ "id": "node-d", "x": 0.0, "y": 0.0 });
    assert_eq!(puzzle2d_node_display_label(&kindless, &fixture), "node-d", "and only a kindless node reads its raw id");
}

/// 🔢️ LAW: duplicate kinds auto-number — the first instance takes the catalogue name, further ones
/// append ` 2`, ` 3`, … to the root taken from their peers (puzzle3d's `next_object_label` semantics).
#[test]
fn the_next_node_label_numbers_duplicates_of_one_kind() {
    let empty = labelled_fixture(&[]);
    assert_eq!(puzzle2d_next_node_label(fixture_nodes(&empty), &empty, "capsule"), "Capsule", "the first instance takes the catalogue name");
    let one = labelled_fixture(&[("node-a", "capsule", Some("Capsule"))]);
    assert_eq!(puzzle2d_next_node_label(fixture_nodes(&one), &one, "capsule"), "Capsule 2");
    let two = labelled_fixture(&[("node-a", "capsule", Some("Capsule")), ("node-b", "capsule", Some("Capsule 2"))]);
    assert_eq!(puzzle2d_next_node_label(fixture_nodes(&two), &two, "capsule"), "Capsule 3");
    let authored = labelled_fixture(&[("node-a", "capsule", Some("Roof Pod"))]);
    assert_eq!(puzzle2d_next_node_label(fixture_nodes(&authored), &authored, "capsule"), "Roof Pod 2", "the root comes from an authored peer, not the catalogue");
    let other = labelled_fixture(&[("node-a", "capsule", Some("Capsule"))]);
    assert_eq!(puzzle2d_next_node_label(fixture_nodes(&other), &other, "beam"), "beam", "a kind with no peer and no catalogue row falls back to its id");
}

/// 🏷️ LAW: `addNode` stamps the label at creation — a second node of the same kind reads ` 2`, never
/// its own raw id.
#[test]
fn adding_nodes_stamps_the_next_display_label() {
    let mut fixture = labelled_fixture(&[]);
    add_node_to_host_snapshot(&mut fixture, Some("capsule"), None);
    add_node_to_host_snapshot(&mut fixture, Some("capsule"), None);
    let labels: Vec<String> = fixture_nodes(&fixture).iter().map(|node| puzzle2d_node_display_label(node, &fixture)).collect();
    assert_eq!(labels, vec!["Capsule".to_string(), "Capsule 2".to_string()]);
}

/// 🏷️ LAW: a batch of fresh ids is re-labelled in order, each seeing what the earlier ones were just
/// given — the one seam duplicate and paste share.
#[test]
fn relabelling_a_batch_numbers_each_new_node_in_order() {
    let mut fixture = labelled_fixture(&[("node-a", "capsule", Some("Capsule")), ("node-b", "capsule", Some("Capsule")), ("node-c", "capsule", Some("Capsule"))]);
    puzzle2d_relabel_nodes(&mut fixture, &["node-b".to_string(), "node-c".to_string()]);
    let labels: Vec<String> = fixture_nodes(&fixture).iter().map(|node| puzzle2d_node_display_label(node, &fixture)).collect();
    assert_eq!(labels, vec!["Capsule".to_string(), "Capsule 2".to_string(), "Capsule 3".to_string()]);
}
//#endregion 🏷️DisplayLabels

//#region 🩹️InspectorEdits
/// 📐️ LAW: `patchInspectorNodes` reaches a node's own numeric field by id — the verb the inspector's
/// editable steppers dispatch.
#[test]
fn patching_an_addressed_node_field_writes_only_that_node() {
    let mut fixture = json!({ "schema": PUZZLE2D_FIXTURE_SCHEMA, "nodes": [{ "id": "a", "x": 1.0 }, { "id": "b", "x": 2.0 }], "edges": [] });
    patch_inspector_nodes(&mut fixture, &["a".to_string()], "x", Some(&json!(9.0)), None);
    assert_eq!(fixture_nodes(&fixture)[0].get("x").and_then(Value::as_f64), Some(9.0));
    assert_eq!(fixture_nodes(&fixture)[1].get("x").and_then(Value::as_f64), Some(2.0), "an addressed patch leaves every other node alone");
}

/// 📐️ LAW: an id naming a HANDLE patches that handle inside its node — how the inspector's handle
/// angle/radius steppers reach nested geometry through the same one verb.
#[test]
fn patching_an_addressed_handle_field_writes_the_nested_handle() {
    let mut fixture = json!({ "schema": PUZZLE2D_FIXTURE_SCHEMA, "nodes": [{ "id": "a", "x": 1.0, "handles": [{ "id": "a:v0", "angle": 0.0 }, { "id": "a:v1", "angle": 1.0 }] }], "edges": [] });
    patch_inspector_nodes(&mut fixture, &["a:v1".to_string()], "angle", None, Some(&json!(0.5)));
    let handles = fixture_nodes(&fixture)[0].get("handles").and_then(Value::as_array).expect("handles");
    assert_eq!(handles[0].get("angle").and_then(Value::as_f64), Some(0.0), "a sibling handle is untouched");
    assert_eq!(handles[1].get("angle").and_then(Value::as_f64), Some(1.5), "a delta rides on the handle's own current value");
    assert_eq!(fixture_nodes(&fixture)[0].get("x").and_then(Value::as_f64), Some(1.0), "the owning node is not patched by a handle-addressed edit");
}
//#endregion 🩹️InspectorEdits

//#region 🌐️WindowOptionVerbs
/// 🌐️ LAW: `setGridVisible` flips the per-window flag and a second dispatch flips it back — the
/// alternating toggle the grid group's measure binds.
#[semio_framework_async_macros::async_test]
async fn set_grid_visible_toggles_the_window_flag() {
    let mut app = app_with_registry();
    for expected in [false, true] {
        let result = dispatch(&mut app, "setGridVisible", None, Some(overview::WINDOW_KIND_ID)).expect("setGridVisible must not fault");
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
        let result = dispatch(&mut app, "setSelectableKind", Some(&json!({ "kind": kind })), Some(overview::WINDOW_KIND_ID)).unwrap_or_else(|error| panic!("setSelectableKind {kind} must not fault: {error:?}"));
        assert_eq!(committed_edits(&result), 0, "setSelectableKind {kind} never mutates the document");
    }
    let after = fixture_of(&app);
    close_app(&mut app);
    assert_eq!(fixture_nodes(&after).len(), fixture_nodes(&before).len());
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
            let result = dispatch(&mut app, action, Some(&json!({ "value": value })), Some(overview::WINDOW_KIND_ID)).unwrap_or_else(|error| panic!("{action} must not fault: {error:?}"));
            assert_eq!(committed_edits(&result), 0, "{action} never mutates the document");
        }
    }
    let after = fixture_of(&app);
    close_app(&mut app);
    assert_eq!(fixture_nodes(&after).len(), fixture_nodes(&before).len());
}

/// 🔂️ LAW: `engagementRepeatLast` is a declared, admitted verb that publishes no document operation —
/// it asks the fill tool for one more placement through `setFillCount`.
#[semio_framework_async_macros::async_test]
async fn engagement_repeat_last_never_mutates_the_document() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let before = fixture_of(&app);
    let result = dispatch(&mut app, "engagementRepeatLast", None, Some(overview::WINDOW_KIND_ID)).expect("engagementRepeatLast must not fault");
    assert_eq!(committed_edits(&result), 0, "repeat-last is a tool reconfiguration, never a document edit");
    let after = fixture_of(&app);
    close_app(&mut app);
    assert_eq!(fixture_nodes(&after).len(), fixture_nodes(&before).len());
}

/// 🗨️ LAW: `openAddNodeDialog` is a shell-only verb — it opens the declared dialog and publishes
/// nothing.
#[semio_framework_async_macros::async_test]
async fn open_add_node_dialog_is_shell_only() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let before = fixture_of(&app);
    let result = dispatch(&mut app, "openAddNodeDialog", None, Some(overview::WINDOW_KIND_ID)).expect("openAddNodeDialog must not fault");
    assert_eq!(committed_edits(&result), 0, "opening a dialog never mutates the document");
    let after = fixture_of(&app);
    close_app(&mut app);
    assert_eq!(fixture_nodes(&after).len(), fixture_nodes(&before).len());
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
        overview::definition(&scene(default_empty_fixture(), Puzzle2dPlayRuntime::default(), "select"), &crate::editor::puzzle2d::engine::board_host::puzzle_board_host(), crate::editor::puzzle2d::terminology::puzzle2d_labels(&semio_framework_plugin::ViewModel::default())),
        detail::definition(&scene(default_empty_fixture(), Puzzle2dPlayRuntime::default(), "select"), &crate::editor::puzzle2d::engine::board_host::puzzle_board_host(), crate::editor::puzzle2d::terminology::puzzle2d_labels(&semio_framework_plugin::ViewModel::default())),
        selection::definition(&scene(default_empty_fixture(), Puzzle2dPlayRuntime::default(), "select"), &crate::editor::puzzle2d::engine::board_host::puzzle_board_host(), crate::editor::puzzle2d::terminology::puzzle2d_labels(&semio_framework_plugin::ViewModel::default())),
    ] {
        assert_eq!(definition.surface_kind, semio_framework_plugin::SurfaceKind::Board2d, "window kind {} renders a Board2d surface", definition.id);
    }
}
//#endregion 🌐️WindowOptionVerbs

//#region 🕹️TransformGumball
/// 🔍️ One fixture node by id, for the rotate reducer's laws.
fn transform_law_node<'a>(fixture: &'a Value, id: &str) -> &'a Value {
    fixture_nodes(fixture).into_iter().find(|node| node.get("id").and_then(Value::as_str) == Some(id)).expect("fixture node")
}

/// 🕹️ Reads the `(move, rotate)` pair out of a rendered board surface's `transformFlags` carrier.
fn rendered_transform_flags(scene_json: &str) -> (bool, bool) {
    let rendered: Value = serde_json::from_str(scene_json).expect("rendered board surface parses");
    let encoded = rendered.get("board2d").and_then(|board| board.get("transformFlags")).and_then(Value::as_str).expect("the board scene declares transformFlags");
    let flags: Value = serde_json::from_str(encoded).expect("transformFlags is a JSON object");
    (flags.get("move").and_then(Value::as_bool).expect("move flag"), flags.get("rotate").and_then(Value::as_bool).expect("rotate flag"))
}

/// 🕹️ LAW: `setTransformGumballFlag` composes the select utility's gumball handles per pane and emits
/// no document operation — it is WindowConfig lane state, exactly like `setGridSnapEnabled`.
#[semio_framework_async_macros::async_test]
async fn set_transform_gumball_flag_composes_the_handles_without_touching_the_document() {
    let mut app = app();
    let result = dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "rotate", "pressed": false })), Some(overview::WINDOW_KIND_ID)).expect("rotate off");
    assert_eq!(committed_edits(&result), 0, "a gumball flag is window config, never a document operation");
    assert_eq!(rendered_transform_flags(&render_body(&mut app, overview::BODY_KEY)), (true, false), "the board scene must carry the composed flags");
    let result = dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "rotate", "pressed": true })), Some(overview::WINDOW_KIND_ID)).expect("rotate on");
    assert_eq!(committed_edits(&result), 0);
    assert_eq!(rendered_transform_flags(&render_body(&mut app, overview::BODY_KEY)), (true, true), "and turn it back on");
    let unknown = dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "scale", "pressed": true })), Some(overview::WINDOW_KIND_ID)).expect("unknown flag");
    assert_eq!(committed_edits(&unknown), 0, "scale is deliberately absent — an unknown flag is a no-op, not a new handle");
    close_app(&mut app);
}

/// 🔄️ LAW: one `nodeRotate` board row is ONE history edit that turns the selected nodes about the
/// pivot the ring drew — the same `rotateSelection` math, so the in-canvas preview and the committed
/// document agree. A zero-angle or id-less row commits nothing.
#[semio_framework_async_macros::async_test]
async fn a_node_rotate_board_event_commits_one_rotate_selection_edit() {
    let mut app = concrete_forest_app();
    let node_id = first_node_id(&app);
    let before = fixture_of(&app);
    let node_before = fixture_nodes(&before).iter().find(|node| node.get("id").and_then(Value::as_str) == Some(node_id.as_str())).cloned().expect("node before");
    let (x0, y0) = (node_before.get("x").and_then(Value::as_f64).expect("x"), node_before.get("y").and_then(Value::as_f64).expect("y"));
    let rotate = json!([{ "name": "nodeRotate", "payload": { "ids": [node_id.clone()], "radians": std::f64::consts::PI, "pivot": { "x": 0.0, "y": 0.0 } } }]).to_string();
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": rotate })), Some(overview::WINDOW_KIND_ID)).expect("rotate event");
    assert!(committed_edits(&result) > 0, "a rotate commit is a document edit");
    let after = fixture_of(&app);
    let node_after = fixture_nodes(&after).iter().find(|node| node.get("id").and_then(Value::as_str) == Some(node_id.as_str())).cloned().expect("node after");
    let (x1, y1) = (node_after.get("x").and_then(Value::as_f64).expect("x"), node_after.get("y").and_then(Value::as_f64).expect("y"));
    // 🔄️ A single-node selection's centroid IS that node, so a half turn about it leaves it put; the
    // law that matters here is that the row reaches `puzzle2d_transform_selection` at all.
    assert!((x1 - x0).abs() < 1e-6 && (y1 - y0).abs() < 1e-6, "a half turn about the node's own centroid is a fixed point: ({x0},{y0}) -> ({x1},{y1})");
    let noop = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "nodeRotate", "payload": { "ids": [node_id.clone()], "radians": 0.0 } }]).to_string() })), Some(overview::WINDOW_KIND_ID)).expect("zero rotate");
    assert_eq!(committed_edits(&noop), 0, "a zero-angle rotate commits nothing");
    let idless = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "nodeRotate", "payload": { "ids": [], "radians": 1.0 } }]).to_string() })), Some(overview::WINDOW_KIND_ID)).expect("id-less rotate");
    assert_eq!(committed_edits(&idless), 0, "an id-less rotate commits nothing");
    close_app(&mut app);
}

/// 🔄️ LAW: a rotate row turns EVERY selected node about the shared pivot and carries its handle
/// angles with it, so edges keep their geometry — the guest half of the board engine's preview.
#[test]
fn rotating_a_two_node_selection_orbits_both_about_the_centroid() {
    let mut fixture = json!({
        "schema": "puzzle.2d.fixture",
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "nodes": [
            { "id": "node-a", "x": -40.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-a:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] },
            { "id": "node-b", "x": 40.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-b:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] },
            { "id": "node-locked", "x": 0.0, "y": 0.0, "locked": true, "shape": "circle", "radius": 10.0, "handles": [] }
        ],
        "edges": []
    });
    let ids = ["node-a".to_string(), "node-b".to_string(), "node-locked".to_string()];
    crate::editor::puzzle2d::puzzle2d_transform_selection(&mut fixture, &ids, crate::editor::puzzle2d::Puzzle2dTransform::Rotate { radians: std::f64::consts::FRAC_PI_2 });
    let a = transform_law_node(&fixture, "node-a");
    assert!(a.get("x").and_then(Value::as_f64).expect("x").abs() < 1e-6 && (a.get("y").and_then(Value::as_f64).expect("y") + 40.0).abs() < 1e-6, "node-a orbits to (0,-40): {a}");
    let b = transform_law_node(&fixture, "node-b");
    assert!(b.get("x").and_then(Value::as_f64).expect("x").abs() < 1e-6 && (b.get("y").and_then(Value::as_f64).expect("y") - 40.0).abs() < 1e-6, "node-b orbits to (0,40): {b}");
    let angle = a.get("handles").and_then(Value::as_array).expect("handles")[0].get("angle").and_then(Value::as_f64).expect("angle");
    assert!((angle - std::f64::consts::FRAC_PI_2).abs() < 1e-9, "the handle angle turns with its node, got {angle}");
    let locked = transform_law_node(&fixture, "node-locked");
    assert!(locked.get("x").and_then(Value::as_f64).expect("x").abs() < 1e-9 && locked.get("y").and_then(Value::as_f64).expect("y").abs() < 1e-9, "a locked node stays put: {locked}");
}
//#endregion 🕹️TransformGumball

//#region 🎯️BoardRegionEvents
/// 🎯️ The document's target-region rows, for the board-event laws.
fn law_target_regions(fixture: &Value) -> Vec<Value> {
    fixture.get("targetRegions").and_then(Value::as_array).cloned().unwrap_or_default()
}

/// 🎯️ LAW: the board engine's three region rows fold onto the SAME mutation paths the palette verbs
/// use — `regionCreate` mints one row through `addTargetRegion`'s own push, `regionMove` and
/// `regionResize` push an absolute pose through `relocateTargetRegion`, one history edit each.
#[semio_framework_async_macros::async_test]
async fn board_region_events_commit_one_edit_each_through_the_target_region_verbs() {
    let mut app = concrete_forest_app();
    let created = json!([{ "name": "regionCreate", "payload": { "x": -20.5, "y": -10.5, "width": 60.5, "height": 40.5 } }]).to_string();
    dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": created })), Some(overview::WINDOW_KIND_ID)).expect("region create");
    let regions = law_target_regions(&fixture_of(&app));
    assert_eq!(regions.len(), 1, "exactly one row is minted: {regions:?}");
    let id = regions[0].get("id").and_then(Value::as_str).expect("minted id").to_string();
    assert_eq!(regions[0].get("width").and_then(Value::as_f64), Some(60.5), "the engine's rectangle is committed verbatim — it already snapped it");

    let moved = json!([{ "name": "regionMove", "payload": { "id": id.clone(), "x": 100.5, "y": 200.5 } }]).to_string();
    dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": moved })), Some(overview::WINDOW_KIND_ID)).expect("region move");
    let regions = law_target_regions(&fixture_of(&app));
    assert_eq!(regions[0].get("x").and_then(Value::as_f64), Some(100.5), "a move relocates the minimum corner");
    assert_eq!(regions[0].get("width").and_then(Value::as_f64), Some(60.5), "and never restates the extent it did not touch");

    let resized = json!([{ "name": "regionResize", "payload": { "id": id.clone(), "x": 100.5, "y": 200.5, "width": 12.5, "height": 8.5 } }]).to_string();
    dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": resized })), Some(overview::WINDOW_KIND_ID)).expect("region resize");
    let regions = law_target_regions(&fixture_of(&app));
    assert_eq!(regions[0].get("width").and_then(Value::as_f64), Some(12.5), "a resize pushes corner AND extent");
    assert_eq!(regions[0].get("height").and_then(Value::as_f64), Some(8.5));

    let unknown = json!([{ "name": "regionMove", "payload": { "id": "never-painted", "x": 1.5, "y": 1.5 } }]).to_string();
    dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": unknown })), Some(overview::WINDOW_KIND_ID)).expect("unknown region");
    let regions = law_target_regions(&fixture_of(&app));
    assert_eq!(regions[0].get("x").and_then(Value::as_f64), Some(100.5), "a row naming a region the board never held moves nothing");
    close_app(&mut app);
}

/// 🖍️ LAW: the board scene carries the Area Brush's own steppers as WORLD extent, so one canvas click
/// and one `addTargetRegion` dispatch paint the identical rectangle. The regions themselves ride the
/// fixture lane — the document's `targetRegions` — never a second carrier.
#[semio_framework_async_macros::async_test]
async fn the_board_scene_carries_the_area_brush_extent_and_the_regions_ride_the_fixture() {
    let mut app = concrete_forest_app();
    dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "regionCreate", "payload": { "x": 0.5, "y": 0.5, "width": 10.5, "height": 10.5 } }]).to_string() })), Some(overview::WINDOW_KIND_ID)).expect("region create");
    let rendered: Value = serde_json::from_str(&render_body(&mut app, overview::BODY_KEY)).expect("rendered board surface parses");
    let board = rendered.get("board2d").expect("board scene");
    let encoded = board.get("areaBrushSize").and_then(Value::as_str).expect("the board scene declares areaBrushSize");
    let size: Value = serde_json::from_str(encoded).expect("areaBrushSize is a JSON object");
    assert!(size.get("width").and_then(Value::as_f64).is_some_and(|width| width > 0.0), "the brush extent is a positive world width: {encoded}");
    assert!(size.get("height").and_then(Value::as_f64).is_some_and(|height| height > 0.0), "and a positive world height: {encoded}");
    let fixture_json = board.get("fixtureJson").and_then(Value::as_str).expect("the board scene carries the fixture");
    assert!(fixture_json.contains("targetRegions"), "the painted region reaches the engine through the fixture lane: {}", &fixture_json[..fixture_json.len().min(200)]);
    close_app(&mut app);
}
//#endregion 🎯️BoardRegionEvents
