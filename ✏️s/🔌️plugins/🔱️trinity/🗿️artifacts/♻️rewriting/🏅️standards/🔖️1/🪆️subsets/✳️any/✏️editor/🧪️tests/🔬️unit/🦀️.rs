use super::*;
use crate::standards::v1::subsets::any::schema::Rhs;
use protocol::{OpBinary, OpText};
use semio_framework_plugin::{artifact_app_laws, App, EditorApp, Locale, PluginApp, Terminology, VcsArtifactApp, ViewModel};

/// 🎫️ See `jack`'s `trinity_jack_manifest_for_tests` doc comment for why this wrapper exists
/// (SDK gap, `artifact_app_laws::new_app_with_registry`'s signature is still `fn(manifest: fn() -> App)`).
fn trinity_rewriting_manifest_for_tests() -> App {
    App { definition: create_rewriting_app(), examples: Vec::new() }
}

fn meta(actor: &str) -> semio_framework_plugin::ActionMeta {
    artifact_app_laws::meta(actor)
}

/// 🎫️ Permanent wire guard (TEMPLATE.md §7): every `TrinityRewritingCommand` variant round-trips
/// through both its binary (`OpBinary`) and text (`OpText`) codecs.
#[semio_framework_async_macros::async_test]
async fn trinity_rewriting_command_text_and_binary_round_trip() {
    let commands = vec![
        TrinityRewritingCommand::NodeGraphEdit { surface_id: "trinity.rewriting.before".into(), operations_json: "[]".into() },
        TrinityRewritingCommand::SetLhsJson { value: "{}".into() },
        TrinityRewritingCommand::SetRhsJson { value: "{}".into() },
        TrinityRewritingCommand::SetParameter { name: "label".into(), value: "hi".into() },
        TrinityRewritingCommand::AddRuleClause { kind: "where".into() },
        TrinityRewritingCommand::ResetRule,
        TrinityRewritingCommand::PatchNodes { node_ids: vec!["a".into()], field: "name".into(), value: "Renamed".into() },
        TrinityRewritingCommand::SetViewport { surface_id: Some("trinity.rewriting.before".into()), viewport: semio_framework_os_kernel::Viewport2d { x: 1.0, y: 2.0, zoom: 1.0 } },
        TrinityRewritingCommand::Reorganize,
        TrinityRewritingCommand::SetLodMode { value: "compact".into() },
    ];
    for command in commands {
        let bytes = command.encode_op().expect("encode");
        assert_eq!(TrinityRewritingCommand::decode_op(&bytes).expect("decode"), command);
        let text = command.print_op();
        assert_eq!(TrinityRewritingCommand::parse_op(&text).expect("parse"), command);
    }
}

/// 🕹️ Registry-backed (not the bare `artifact_app_laws::new_app`): `interactionSelect`/`interactionHover`
/// resolve the dispatching app's declared `AppActionRegistry.interactions`, so any test exercising
/// domain "graph" selection needs the real manifest's `.interaction(...)` declaration present.
/// 🔌️ Mounted (`bind_instance_id`): `dispatch_typed` refuses `interactive-job.live-instance` for an
/// unbound app, and the mounted app answers BEFORE its retained typed operation publishes, so a
/// dispatching test settles through `settle(&mut app)` before reading the projection.
/// 🔚 Self-closing: the store's `Drop` demands the terminal-empty witness, so the guard retires the
/// app through the framework's exact close loop (skipped while unwinding).
async fn new_app() -> RewritingTestApp {
    let mut app = artifact_app_laws::new_app_with_registry::<EditorApp<TrinityRewritingPlayApp>>(trinity_rewriting_manifest_for_tests).await;
    app.bind_instance_id(REWRITING_TEST_INSTANCE).await;
    RewritingTestApp(app)
}

const REWRITING_TEST_INSTANCE: u32 = 1;

pub(crate) struct RewritingTestApp(VcsArtifactApp<EditorApp<TrinityRewritingPlayApp>>);

impl std::ops::Deref for RewritingTestApp {
    type Target = VcsArtifactApp<EditorApp<TrinityRewritingPlayApp>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for RewritingTestApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for RewritingTestApp {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            artifact_app_laws::close_registered_fixture_app(&mut self.0);
        }
    }
}

/// 📬️ Drives the mounted app's retained typed operation to quiescence — the receipt carries the
/// lanes, effects and events the host would have seen.
async fn settle(app: &mut RewritingTestApp) -> artifact_app_laws::TypedOperationFixtureReceipt {
    artifact_app_laws::settle_registered_typed_operation(&mut app.0, REWRITING_TEST_INSTANCE).await.expect("settle the typed operation")
}

/// ↩️ Dispatches a framework-reserved history verb (`undo`/`redo`) and settles its reserved job.
async fn history(app: &mut RewritingTestApp, verb: &str) {
    artifact_app_laws::settle_history_verb(&mut app.0, verb, REWRITING_TEST_INSTANCE).await;
}

/// 🖼️ Projects a rendered body through the fixture transport — a bare `serde_json::to_string(&tree.root)`
/// fails `BuiltChildren requires retained page transport`.
async fn render(app: &mut RewritingTestApp, body_key: &str, view: &ViewModel) -> String {
    let tree = app.render(body_key, None, view).await.expect("render");
    artifact_app_laws::project_and_retire_fixture_tree(tree).expect("project semantic UI test tree")
}

/// 🕹️ Dispatches the framework-injected `interactionSelect` verb against domain "graph" — the
/// replacement for the deleted `TrinityRewritingCommand::SetSelection`. On a mounted app the verb is
/// admitted as a framework-reserved job and publishes later, so both are settled here.
async fn select_graph(app: &mut RewritingTestApp, ids: &[&str]) {
    let targets: Vec<pack::JsonValue> = ids.iter().map(|id| pack::json!({ "granularity": "node", "id": id })).collect();
    let args = pack::json_to_dsl_value(&pack::json!({ "domainId": "graph", "targets": pack::to_json_string(&targets) }));
    let admitted = app.handle_action("interactionSelect", Some(&args), &meta("local")).await.expect("interactionSelect");
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted).await.expect("interactionSelect reserved-job commit");
    settle(app).await;
}

#[semio_framework_async_macros::async_test]
async fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
    let mut app = new_app().await;
    let request = ContextMenuRequest {
        menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None },
        surface: Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: TRINITY_REWRITING_PLAY_SURFACE_BEFORE.into(),
            kind: "nodeGraph".into(),
            hits: vec![semio_framework_plugin::ContextMenuHit { domain: "node".into(), id: "n1".into(), label: None }],
            selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: "node".into(), ids: vec!["n1".into(), "n2".into()] }],
            text: None,
        }),
        window_instance_id: None,
        point: None,
    };
    let menu = app.context_menu(&request, &ViewModel::default()).await;
    assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
    let last = menu.last().expect("grouped disclosure menu should not be empty");
    let last_is_destructive_leaf = last.id == "delete-selection" && last.destructive == Some(true) && last.action.as_deref() == Some("nodeGraphEdit");
    let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.destructive == Some(true));
    assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive delete-selection must be last: {menu:?}");
}

#[semio_framework_async_macros::async_test]
async fn renders_before_and_after_graphs() {
    let mut app = new_app().await;
    let before = render(&mut app, TRINITY_REWRITING_PLAY_BODY_BEFORE, &ViewModel::default()).await;
    let after = render(&mut app, TRINITY_REWRITING_PLAY_BODY_AFTER, &ViewModel::default()).await;
    assert!(before.contains("node-graph"));
    assert!(after.contains("node-graph"));
}

#[semio_framework_async_macros::async_test]
async fn compiles_jack_query_from_rule() {
    let query = compiled_jack_query(&default_rule_state());
    assert!(query.contains("MATCH"));
    assert!(query.contains("SET"));
}

#[semio_framework_async_macros::async_test]
async fn apply_rewriting_changes_after_fixture() {
    let state = default_rule_state();
    assert_ne!(state.before_fixture_json, after_fixture_json(&state));
}

#[semio_framework_async_macros::async_test]
async fn renders_lhs_rhs_graphs() {
    let mut app = new_app().await;
    let lhs_json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_LHS, &ViewModel::default()).await;
    let rhs_json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_RHS, &ViewModel::default()).await;
    assert!(lhs_json.contains("node-graph"));
    assert!(rhs_json.contains("node-graph"));
    let lhs = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&lhs_json).expect("lhs node-graph scene");
    let rhs = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&rhs_json).expect("rhs node-graph scene");
    assert_eq!(lhs.editable, Some(true));
    assert_eq!(rhs.editable, Some(true));
}

#[semio_framework_async_macros::async_test]
async fn set_parameter_emits_one_op_and_is_undoable() {
    let mut app = new_app().await;
    let history_before = app.history_snapshot().await.expect("history").upserts.len();
    app.dispatch_typed(TrinityRewritingCommand::SetParameter { name: "label".into(), value: "changed".into() }, &meta("local")).await.expect("set parameter");
    let receipt = settle(&mut app).await;
    assert!(receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact), "a parameter edit publishes on the artifact lane: {:?}", receipt.lanes);
    assert_eq!(app.history_snapshot().await.expect("history").upserts.len(), history_before + 1, "a single-key parameter edit is one ChangeParameterBinding operation (one history entry)");
    assert_eq!(app.snapshot().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("changed".into())));
    history(&mut app, "undo").await;
    assert_eq!(app.snapshot().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("nakagin-core".into())));
}

#[semio_framework_async_macros::async_test]
async fn add_and_delete_rhs_set_clause() {
    let mut app = new_app().await;
    app.dispatch_typed(TrinityRewritingCommand::AddRuleClause { kind: "set".into() }, &meta("local")).await.expect("add clause");
    settle(&mut app).await;
    let rhs: Rhs = pack::from_json_str(&app.snapshot().unwrap().rhs_json).unwrap();
    assert_eq!(rhs.set.len(), 2);
    select_graph(&mut app, &["rhs-set-1"]).await;
    let result = app
        .dispatch_typed(TrinityRewritingCommand::NodeGraphEdit { surface_id: TRINITY_REWRITING_PLAY_SURFACE_RHS.into(), operations_json: pack::json!([{ "operation": "deleteSelection" }]).to_string() }, &meta("local"))
        .await
        .expect("delete selection");
    let receipt = settle(&mut app).await;
    assert!(!result.mutations.is_empty() || receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact), "deleteSelection must publish an artifact mutation: {:?}", receipt.lanes);
    let rhs: Rhs = pack::from_json_str(&app.snapshot().unwrap().rhs_json).unwrap();
    assert_eq!(rhs.set.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn jack_view_renders_compiled_query_tokens() {
    let mut app = new_app().await;
    let json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_JACK, &ViewModel::default()).await;
    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::TextEditorScene>(&json).expect("text-editor scene");
    assert!(scene.tokens_json.is_some_and(|tokens| !tokens.is_empty()));
}

#[semio_framework_async_macros::async_test]
async fn graph_scenes_have_lod_json() {
    let mut app = new_app().await;
    let json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_BEFORE, &ViewModel::default()).await;
    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&json).expect("node-graph scene");
    assert!(scene.lod_json.is_some(), "lodJson lane missing: {json}");
}

#[semio_framework_async_macros::async_test]
async fn app_definition_declares_reorganize_and_history_actions() {
    let definition = create_rewriting_app();
    let action_ids: Vec<&str> = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).map(|action| action.id.as_str()).collect();
    assert!(action_ids.contains(&"undo"));
    assert!(action_ids.contains(&"reorganize"));
}

/// ⚖️ LAW: the navbar example picker's verb reaches EVERY window kind. `setActiveExample` is
/// app-scoped — no `window_kind_action_refs` owns it — so `build_definition` copies it onto every
/// window, which is what makes the shell's boot dispatch dispatchable from whichever pane happens to
/// be focused. It was declared nowhere at all, so the very first dispatch of every boot was dropped
/// `undeclared-action` from `trinity-rewriting-lhs`
/// (ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END, `📓️b3b-trinity-wfc-puzzle.md` §3.1).
#[semio_framework_async_macros::async_test]
async fn app_definition_declares_set_active_example_on_every_window_kind() {
    let definition = create_rewriting_app();
    let owners = definition.window_kinds.iter().filter(|window| window.actions.iter().any(|action| action.id == "setActiveExample")).count();
    assert_eq!(owners, definition.window_kinds.len(), "setActiveExample must stay unscoped so every window kind declares it");
}

/// ⚖️ LAW: `setActiveExample` answers the id the SHELL sends with a whole-document `LoadDocument`
/// effect. The navbar picker dispatches a REGISTERED example id, and `demo` is the only example this
/// subset registers, so that id must resolve. The effect is HOST-applied — the guest's own snapshot
/// is deliberately left alone here (asserting it changed is what a first draft of this test got
/// wrong), so the guest-side contract is exactly "one `LoadDocument` carrying the example's document".
#[semio_framework_async_macros::async_test]
async fn set_active_example_loads_the_registered_demo_document() {
    let mut app = new_app().await;
    app.dispatch_typed(TrinityRewritingCommand::SetActiveExample { example_id: crate::examples::demo::ID.into() }, &meta("local")).await.expect("set active example");
    let receipt = settle(&mut app).await;
    let loads = receipt.effects.iter().filter(|effect| matches!(effect, semio_framework_plugin::Effect::LoadDocument { .. })).count();
    assert_eq!(loads, 1, "the registered example id produces exactly one LoadDocument effect");
    let loaded = crate::editor::rewriting::commands::set_active_example_document(crate::examples::demo::ID).expect("the demo example resolves to a document");
    assert_eq!(loaded, store::ArtifactDsl::parse_dsl(crate::examples::demo::PRIMARY_TEXT).expect("the demo example's own dsl parses"), "the document the effect carries is the example the subset registers");
    assert_ne!(loaded, app.snapshot().unwrap(), "the effect is HOST-applied: a mounted app with no host keeps its own snapshot, which is what makes the effect the only guest-side witness");
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_ignores_an_unregistered_id() {
    let mut app = new_app().await;
    app.dispatch_typed(TrinityRewritingCommand::SetActiveExample { example_id: "not-an-example".into() }, &meta("local")).await.expect("set active example");
    let receipt = settle(&mut app).await;
    assert!(receipt.effects.is_empty(), "an unknown example id loads nothing");
}

/// ⚖️ LAW: the verb crosses the SAME two gates every other rewriting document verb crosses — the
/// binary tool-job roster (`TOOL_JOB_IDS`, the wire contract) and the retained document-tool roster
/// (`REWRITING_DOCUMENT_TOOL_IDS`, which the factory keys on). Declaring the action without both
/// leaves it dispatchable but unroutable.
#[semio_framework_async_macros::async_test]
async fn set_active_example_is_registered_on_both_tool_rosters() {
    assert!(<TrinityRewritingCommand as OpBinary>::TOOL_JOB_IDS.contains(&"setActiveExample"));
    assert!(REWRITING_DOCUMENT_TOOL_IDS.contains(&"setActiveExample"));
}

#[semio_framework_async_macros::async_test]
async fn trinity_rewriting_labels_resolve_native_by_default() {
    let mut app = new_app().await;
    let json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_ARTIFACT, &ViewModel::default()).await;
    assert!(json.contains("\"Pieces\""));
    assert!(!json.contains("Stücke"));
}

#[semio_framework_async_macros::async_test]
async fn trinity_rewriting_labels_translate_panels_in_german() {
    let mut app = new_app().await;
    let view = ViewModel { locale: Locale::De, terminology: Terminology::Native, ..ViewModel::default() };
    let document_json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_ARTIFACT, &view).await;
    assert!(document_json.contains("Stücke"));
    assert!(!document_json.contains("\"Pieces\""));
    let catalogue_json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_CATALOGUE, &view).await;
    assert!(catalogue_json.contains("Katalog"));
    assert!(catalogue_json.contains("Zu LHS hinzufügen"));
    assert!(catalogue_json.contains("Zu RHS hinzufügen"));
    let parameters_json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_PARAMETERS, &view).await;
    assert!(parameters_json.contains("\"Parameter\""));
    let definition = create_rewriting_app();
    let reset_rule = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "resetRule").expect("resetRule action");
    assert_eq!(reset_rule.label.resolve(Terminology::Native, Locale::De), "Regel zurücksetzen");
}

#[semio_framework_async_macros::async_test]
async fn set_lhs_json_undo_redo_round_trip() {
    let mut app = new_app().await;
    let original = app.snapshot().unwrap().lhs_json;
    let next_lhs = r#"{"pattern":{"leftVar":"x","leftKind":"Piece","edgeVar":"r","edgeKind":"Connection","rightVar":"y","rightKind":"Piece"}}"#;
    app.dispatch_typed(TrinityRewritingCommand::SetLhsJson { value: next_lhs.into() }, &meta("local")).await.expect("set lhs");
    settle(&mut app).await;
    assert_eq!(app.snapshot().unwrap().lhs_json, next_lhs);
    history(&mut app, "undo").await;
    assert_eq!(app.snapshot().unwrap().lhs_json, original);
    history(&mut app, "redo").await;
    assert_eq!(app.snapshot().unwrap().lhs_json, next_lhs);
}

#[semio_framework_async_macros::async_test]
async fn export_media_graph_out_reflects_rule_applied_fixture() {
    let mut app = new_app().await;
    let graph_out = app.export_media("graph:out").await.expect("graph:out export");
    let MediaPayload::Structured { json, .. } = graph_out.payload else { panic!("structured payload") };
    let bytes = store::pack_rt::pack_value_from_base64(&json).expect("decode base64");
    let fixture = <JackSnapshot as ArtifactPack>::decode_pack(&bytes).expect("decode pack");
    let expected = JackSnapshot::from_json(&after_fixture_json(&app.snapshot().unwrap())).unwrap();
    assert_eq!(fixture.nodes().len(), expected.nodes().len());
}

#[semio_framework_async_macros::async_test]
async fn rewriting_io_declares_graph_in_and_graph_out_ports() {
    let io = rewriting_io();
    assert_eq!(io.artifact_schema, REWRITE_RULE_SCHEMA);
    let graph_in = io.ports.iter().find(|port| port.id == "graph:in").expect("graph:in declared");
    assert_eq!(graph_in.kind_id.as_deref(), Some("graph.trinity"));
    assert_eq!(graph_in.multiplicity, semio_framework_plugin::PortMultiplicity::One);
    let graph_out = io.ports.iter().find(|port| port.id == "graph:out").expect("graph:out declared");
    assert_eq!(graph_out.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
}

#[semio_framework_async_macros::async_test]
async fn reset_document_ownership_rewriting_preserves_pack_with_an_edit_free_history() {
    use store::ArtifactPack;
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/♻️reset-document.json")).unwrap();
    let source = RewritingSnapshot { before_fixture_json: "{}".into(), lhs_json: "{}".into(), rhs_json: "{}".into(), parameter_bindings: BTreeMap::new(), rule_layout: BTreeMap::new() };
    let before = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::json::to_json_string(&source)).unwrap();
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = reset_document_effect(&source) else { panic!("reset must load a document"); };
    let decoded = <RewritingSnapshot as ArtifactPack>::decode_pack(&pack).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::os_pack::json::to_json_string(&decoded)).unwrap(), before);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::os_pack::json::to_json_string(&source)).unwrap(), before);
    let history = store::os_spr::decode_history(&spr, &store::os_spr::DecodeOptions::default()).await.unwrap();
    let actual = serde_json::json!({ "documentId": history.doc_id, "schema": history.schema, "edits": history.edits.len(), "transitions": history.transitions.len(), "conflicts": history.conflicts.len() });
    assert_eq!(actual, expected);
    println!("[DEBUG] rewriting reset preserves its source and pack and emits neutral edit-free history without an envelope owner");
}
