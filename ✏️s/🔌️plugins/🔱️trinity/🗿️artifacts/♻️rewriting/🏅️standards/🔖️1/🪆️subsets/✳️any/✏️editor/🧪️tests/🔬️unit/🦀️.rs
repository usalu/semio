use super::*;
use crate::standards::v1::subsets::any::schema::Rhs;
use protocol::{OpBinary, OpText};
use semio_framework_plugin::artifact_app_laws;
use semio_framework_plugin::App;
use semio_framework_plugin::EditorApp;
use semio_framework_ui_locale::Locale;
use semio_framework_plugin::PluginApp;
use semio_framework_ui_locale::Terminology;
use semio_framework_plugin::VcsArtifactApp;
use semio_framework_plugin::ViewModel;

/// 🎫️ See `jack`'s `trinity_jack_manifest_for_tests` doc comment for why this wrapper exists
/// (SDK gap, `artifact_app_laws::new_app_with_registry`'s signature is still `fn(manifest: fn() -> App)`).
fn trinity_rewriting_manifest_for_tests() -> App {
    App { definition: create_rewriting_app(), examples: Vec::new() }
}

semio_framework_plugin::history_edit_acceptance_law!("trinity", TrinityRewritingPlayApp, trinity_rewriting_manifest_for_tests, "../..");
semio_framework_plugin::composed_reload_law!("trinity", TrinityRewritingPlayApp, trinity_rewriting_manifest_for_tests, "../..");
semio_framework_plugin::composed_child_history_law!("trinity", TrinityRewritingPlayApp, trinity_rewriting_manifest_for_tests, [("patchNodes", r#"{"nodeIds":["7dc5b737-3b6b-4068-b315-b7bacc91c2e1"],"field":"name","value":"Renamed core"}"#)]);

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
    let mut app = artifact_app_laws::new_app_with_registry_and_members::<EditorApp<TrinityRewritingPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(trinity_rewriting_manifest_for_tests).await;
    app.bind_instance_id(REWRITING_TEST_INSTANCE).await;
    RewritingTestApp(app)
}

const REWRITING_TEST_INSTANCE: u32 = 1;

pub(crate) struct RewritingTestApp(VcsArtifactApp<EditorApp<TrinityRewritingPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>);

impl std::ops::Deref for RewritingTestApp {
    type Target = VcsArtifactApp<EditorApp<TrinityRewritingPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>;
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
    let targets: Vec<semio_framework_pack_json::Value> = ids.iter().map(|id| semio_framework_pack_json::json!({ "granularity": "node", "id": id })).collect();
    let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({ "domainId": "graph", "targets": semio_framework_pack_json::to_json_string(&targets) }));
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
    let menu = app.context_menu(&request, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
    let last = menu.last().expect("grouped disclosure menu should not be empty");
    let last_is_destructive_leaf = last.id == "delete-selection" && last.destructive == Some(true) && last.action.as_deref() == Some("nodeGraphEdit");
    let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.destructive == Some(true));
    assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive delete-selection must be last: {menu:?}");
}

#[semio_framework_async_macros::async_test]
async fn renders_before_and_after_graphs() {
    let mut app = new_app().await;
    let before = render(&mut app, TRINITY_REWRITING_PLAY_BODY_BEFORE, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    let after = render(&mut app, TRINITY_REWRITING_PLAY_BODY_AFTER, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
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
    assert_ne!(state.working_graph, after_fixture(&state).expect("valid typed rule"));
}

#[semio_framework_async_macros::async_test]
async fn renders_lhs_rhs_graphs() {
    let mut app = new_app().await;
    let lhs_json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_LHS, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    let rhs_json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_RHS, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
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
    let rhs: Rhs = app.snapshot().unwrap().rhs;
    assert_eq!(rhs.set.len(), 2);
    let result = app
        .dispatch_typed(TrinityRewritingCommand::NodeGraphEdit { surface_id: TRINITY_REWRITING_PLAY_SURFACE_RHS.into(), operations_json: semio_framework_pack_json::json!([{ "operation": "delete", "nodeIds": ["rhs-set-1"], "synapseIds": [] }]).to_string() }, &meta("local"))
        .await
        .expect("delete the clause node");
    let receipt = settle(&mut app).await;
    assert!(!result.mutations.is_empty() || receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact), "a delete row must publish an artifact mutation: {:?}", receipt.lanes);
    let rhs: Rhs = app.snapshot().unwrap().rhs;
    assert_eq!(rhs.set.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn jack_view_renders_compiled_query_tokens() {
    let mut app = new_app().await;
    let json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_JACK, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::TextEditorScene>(&json).expect("text-editor scene");
    assert!(scene.tokens_json.is_some_and(|tokens| !tokens.is_empty()));
}

#[semio_framework_async_macros::async_test]
async fn graph_scenes_have_lod_json() {
    let mut app = new_app().await;
    let json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_BEFORE, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&json).expect("node-graph scene");
    assert!(scene.lod_json.is_some(), "lodJson lane missing: {json}");
}

#[semio_framework_async_macros::async_test]
async fn app_definition_declares_reorganize_and_history_actions() {
    let definition = create_rewriting_app();
    let action_ids: Vec<&str> = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).map(|action| action.id.as_str()).collect();
    assert!(action_ids.contains(&"undo"));
    assert!(action_ids.contains(&"reorganize"));
}

/// ⚖️ LAW: the navbar example picker's verb reaches EVERY window kind. `setActiveExample` is
/// app-scoped — no `window_kind_action_refs` owns it — so `try_build_definition` leaves it on the
/// app roster (`AppDefinition.actions`), which is what makes the shell's boot dispatch dispatchable
/// from whichever pane happens to be focused: `undeclaredActionDiagnostic` accepts an action found
/// on a window kind OR on the app itself (`🧱️elements/🛠️ShellHelpers/🟦️.tsx`). It was declared
/// nowhere at all, so the very first dispatch of every boot was dropped `undeclared-action` from
/// `trinity-rewriting-lhs` (ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END,
/// `📓️b3b-trinity-wfc-puzzle.md` §3.1).
#[semio_framework_async_macros::async_test]
async fn app_definition_declares_set_active_example_unscoped_for_every_window_kind() {
    let definition = create_rewriting_app();
    let scoped = definition.window_kinds.iter().filter(|window| window.actions.iter().any(|action| action.id == "setActiveExample")).count();
    assert!(definition.actions.iter().any(|action| action.id == "setActiveExample"), "setActiveExample must sit on the app roster so every window kind can dispatch it");
    assert_eq!(scoped, 0, "scoping setActiveExample to one window kind would take it off the app roster and strand the panes that do not own it");
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
    let json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_ARTIFACT, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    assert!(json.contains("\"Pieces\""));
    assert!(!json.contains("Stücke"));
}

#[semio_framework_async_macros::async_test]
async fn trinity_rewriting_labels_translate_panels_in_german() {
    let mut app = new_app().await;
    let view = ViewModel { locale: Locale::De, terminology: Terminology::Native, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
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
    let reset_rule = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == "resetRule").expect("resetRule action");
    assert_eq!(reset_rule.label.resolve(Terminology::Native, Locale::De), "Regel zurücksetzen");
}

#[semio_framework_async_macros::async_test]
async fn set_lhs_json_undo_redo_round_trip() {
    let mut app = new_app().await;
    let original = app.snapshot().unwrap().lhs;
    let next_lhs = r#"{"pattern":{"leftVar":"x","leftKind":"Piece","edgeVar":"r","edgeKind":"Connection","rightVar":"y","rightKind":"Piece"}}"#;
    app.dispatch_typed(TrinityRewritingCommand::SetLhsJson { value: next_lhs.into() }, &meta("local")).await.expect("set lhs");
    settle(&mut app).await;
    assert_eq!(app.snapshot().unwrap().lhs, semio_framework_pack_json::from_json_str::<crate::standards::v1::subsets::any::schema::Lhs>(next_lhs, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("typed match input"));
    history(&mut app, "undo").await;
    assert_eq!(app.snapshot().unwrap().lhs, original);
    history(&mut app, "redo").await;
    assert_eq!(app.snapshot().unwrap().lhs, semio_framework_pack_json::from_json_str::<crate::standards::v1::subsets::any::schema::Lhs>(next_lhs, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("typed match input"));
}

#[semio_framework_async_macros::async_test]
async fn export_media_graph_out_reflects_rule_applied_fixture() {
    let mut app = new_app().await;
    let graph_out = app.export_media("graph:out").await.expect("graph:out export");
    let MediaPayload::Structured { json, .. } = graph_out.payload else { panic!("structured payload") };
    let bytes = store::pack_rt::pack_value_from_base64(&json).expect("decode base64");
    let fixture = <JackSnapshot as ArtifactPack>::decode_pack(&bytes).expect("decode pack");
    let expected = after_fixture(&composed_state(&app).await).expect("valid typed rule");
    assert_eq!(fixture.nodes().expect("valid retained Jack child").len(), expected.nodes().expect("valid retained Jack child").len());
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
    let source = default_rule_state();
    let before = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&source)).unwrap();
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = reset_document_effect(&source) else { panic!("reset must load a document"); };
    let decoded = <RewritingSnapshot as ArtifactPack>::decode_pack(&pack).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).unwrap(), before);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&source)).unwrap(), before);
    let history = store::os_spr::decode_history(&spr, &store::os_spr::DecodeOptions::default()).await.unwrap();
    let actual = serde_json::json!({ "documentId": history.doc_id, "schema": history.schema, "edits": history.edits.len(), "transitions": history.transitions.len(), "conflicts": history.conflicts.len() });
    assert_eq!(actual, expected);
}

//#region 🩹️RailVerbLaws
/// 🧸️ The live `workingGraph` member — the working graph's single truth (design §20.15), never the parent's genesis owner.
async fn live_working(app: &RewritingTestApp) -> semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
    use store::{ArtifactPack, SpaceMember};
    let snapshot = app.snapshot().expect("rewriting parent projection");
    let bytes = app.child_store(crate::content::WORKING_CHILD_SLOT, &snapshot.working_graph.content.child_id).await.expect("working child").document_pack_bytes().await.expect("working child pack");
    SemioGraphSnapshot::decode_pack(&bytes).expect("working child snapshot")
}

/// 🧩️ The parent projection composed with its live working child, as every reader sees the rule.
async fn composed_state(app: &RewritingTestApp) -> RewritingSnapshot {
    let mut state = app.snapshot().expect("rewriting parent projection");
    semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut state.working_graph.content, live_working(app).await);
    state
}

async fn working_node_ids(app: &RewritingTestApp) -> Vec<String> {
    live_working(app).await.nodes.iter().map(|node| node.id.value.clone()).collect()
}

async fn working_node_label(app: &RewritingTestApp, id: &str) -> String {
    live_working(app).await.nodes.iter().find(|node| node.id.value == id).map(|node| node.label.clone()).expect("node")
}

/// 🕹️ Dispatches one rail press and settles its retained job; a refusal surfaces as the fault.
async fn dispatch_rail(app: &mut RewritingTestApp, action: &str, args: &[(&str, &str)]) -> Result<artifact_app_laws::TypedOperationFixtureReceipt, Fault> {
    let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::Value::Object(args.iter().map(|(key, value)| ((*key).to_string(), semio_framework_pack_json::Value::String((*value).to_string()))).collect()));
    app.handle_action(action, Some(&args), &meta("local")).await?;
    artifact_app_laws::settle_registered_typed_operation(&mut app.0, REWRITING_TEST_INSTANCE).await
}

/// 🕸️ Dispatches one `nodeGraphEdit` batch on `surface` and settles it.
async fn graph_edit(app: &mut RewritingTestApp, surface: &str, rows: semio_framework_pack_json::Value) -> Result<artifact_app_laws::TypedOperationFixtureReceipt, Fault> {
    let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({ "surfaceId": surface, "operations": rows.to_string() }));
    app.handle_action("nodeGraphEdit", Some(&args), &meta("local")).await?;
    artifact_app_laws::settle_registered_typed_operation(&mut app.0, REWRITING_TEST_INSTANCE).await
}

/// 📜️ The edited history rows of the document.
async fn edit_rows(app: &mut RewritingTestApp) -> usize {
    use semio_framework_plugin::PluginApp;
    app.history_snapshot().await.expect("history").upserts.into_iter().filter(|row| row.edit_id.is_some()).count()
}

fn child_lane(receipt: &artifact_app_laws::TypedOperationFixtureReceipt) -> bool {
    receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Child)
}

/// ⚖️ LAW: `patchNodes` pressed from the rail with an EMPTY `nodeIds` patches the selected nodes of the
/// working graph, and a comma list in the text field names several nodes (S15: the verb "moved nothing") — each press ONE edit of
/// the composed `workingGraph` child (design §20.15).
#[semio_framework_async_macros::async_test]
async fn patch_nodes_from_the_rail_patches_the_selection_or_the_listed_nodes() {
    let mut app = new_app().await;
    let ids = working_node_ids(&app).await;
    select_graph(&mut app, &[&ids[0]]).await;
    let receipt = dispatch_rail(&mut app, "patchNodes", &[("field", "name"), ("value", "S15 Selected")]).await.expect("an empty nodeIds patches the selection");
    assert!(child_lane(&receipt), "patchNodes edits the working child: lanes {:?}", receipt.lanes);
    assert_eq!(working_node_label(&app, &ids[0]).await, "S15 Selected");
    dispatch_rail(&mut app, "patchNodes", &[("nodeIds", &format!("{} {}", ids[0], ids[1])), ("field", "name"), ("value", "S15 Listed")]).await.expect("a listed nodeIds patches both nodes");
    assert_eq!((working_node_label(&app, &ids[0]).await, working_node_label(&app, &ids[1]).await), ("S15 Listed".to_string(), "S15 Listed".to_string()));
}

/// ⚖️ LAW: a `patchNodes` that cannot move the document is refused by name — an unknown id is
/// `mutation.target-missing`, no id and no selection is `app.command.targets-required` (the precondition an agent,
/// which has no selection, meets by naming `nodeIds`), an unsupported field, an empty value or a kind the manifest does not
/// declare is `app.command.invalid-args` — and every refusal leaves the working child untouched.
#[semio_framework_async_macros::async_test]
async fn patch_nodes_refuses_what_it_cannot_apply() {
    let mut app = new_app().await;
    let first = working_node_ids(&app).await[0].clone();
    let before = live_working(&app).await;
    let refusal = |result: Result<artifact_app_laws::TypedOperationFixtureReceipt, Fault>| result.err().expect("refused").code.0;
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("nodeIds", "no-such-node"), ("field", "name"), ("value", "x")]).await), "mutation.target-missing");
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("field", "name"), ("value", "x")]).await), "app.command.targets-required");
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("nodeIds", first.as_str()), ("field", "colour"), ("value", "x")]).await), "app.command.invalid-args");
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("nodeIds", first.as_str()), ("field", "kind"), ("value", " ")]).await), "app.command.invalid-args");
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("nodeIds", first.as_str()), ("field", "kind"), ("value", "NoSuchKind")]).await), "app.command.invalid-args", "a kind the manifest does not declare is refused, not written");
    assert_eq!(live_working(&app).await, before, "every refusal leaves the working child untouched");
}

/// 🚫️ LAW: `addRuleClause` refuses what it cannot add — a second WHERE clause, an unknown clause kind, an input whose
/// typed match variable is not text — instead of an empty emit (S15: `kind=where` on the default rule journalled a row and
/// changed nothing).
#[semio_framework_async_macros::async_test]
async fn add_rule_clause_refuses_what_it_cannot_add() {
    let state = default_rule_state();
    let add = crate::editor::rewriting::commands::add_rule_clause_command;
    let code = |result: Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault>| result.err().expect("refused").code.0;
    assert_eq!(code(add(&state, "where")), "app.command.invalid-args", "the default rule already has a WHERE clause");
    assert_eq!(code(add(&state, "optional")), "app.command.invalid-args");
    assert_eq!(code(crate::editor::rewriting::commands::set_lhs(&state, r#"{"pattern":{"leftVar":42,"leftKind":"Piece"}}"#)), "app.command.invalid-args", "the declared input rejects a nontext match variable");
}

/// ⏪️ LAW: a rail `addRuleClause kind=create` is ONE undoable edit that rewrites only the RHS — undo restores the
/// rule and redo adds the clause again (S15, session 12: its row carried a re-printed LHS and undo did nothing).
#[semio_framework_async_macros::async_test]
async fn a_rail_add_rule_clause_rewrites_only_the_rhs_and_round_trips() {
    let mut app = new_app().await;
    let before = app.snapshot().expect("projection");
    let emit = crate::editor::rewriting::commands::add_rule_clause_command(&before, "create").expect("create clause");
    assert!(!emit.artifact_mutations.is_empty() && emit.artifact_mutations.iter().all(|mutation| matches!(mutation, RewriteRuleMutation::EditRhs(_))), "only the RHS changes: {:?}", emit.artifact_mutations);
    let rail = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({ "kind": "create" }));
    app.handle_action("addRuleClause", Some(&rail), &meta("local")).await.expect("addRuleClause");
    settle(&mut app).await;
    let after = app.snapshot().expect("projection");
    assert_ne!(after.rhs, before.rhs, "the clause is added");
    assert_eq!(after.lhs, before.lhs, "the LHS is untouched");
    history(&mut app, "undo").await;
    assert_eq!(app.snapshot().expect("projection").rhs, before.rhs, "undo restores the RHS");
    history(&mut app, "redo").await;
    assert_eq!(app.snapshot().expect("projection").rhs, after.rhs, "redo adds the clause again");
}

/// ⚖️ LAW: `nodeGraphEdit` is the node-graph host's gesture verb (a `surfaceId` plus an `operations`
/// list), so it stays out of the palette, the Actions rail and the context menu's `transform` group —
/// pressed there it was refused `missing operationsJson` (S15). The menu's own delete row still
/// dispatches it with its full payload.
#[semio_framework_async_macros::async_test]
async fn the_node_graph_gesture_verb_is_kept_off_the_rail_and_the_transform_group() {
    let definition = create_rewriting_app();
    let edit = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == "nodeGraphEdit").expect("declared");
    assert!(!edit.in_palette, "nodeGraphEdit is a canvas gesture, never a rail row");
    let patch = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == "patchNodes").expect("declared");
    assert!(patch.args.iter().any(|arg| arg.id == "nodeIds" && !arg.required), "nodeIds is optional: empty means the selection");
    let mut app = new_app().await;
    let request = ContextMenuRequest {
        menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None },
        surface: Some(semio_framework_plugin::ContextMenuSurfaceTarget { surface_id: TRINITY_REWRITING_PLAY_SURFACE_BEFORE.into(), kind: "nodeGraph".into(), hits: Vec::new(), selection: Vec::new(), text: None }),
        window_instance_id: None,
        point: None,
    };
    let menu = app.context_menu(&request, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    let bare_edit_rows = menu.iter().flat_map(|row| std::iter::once(row).chain(row.children.iter().flatten())).filter(|row| row.action.as_deref() == Some("nodeGraphEdit") && row.id != "delete-selection").count();
    assert_eq!(bare_edit_rows, 0, "no argument-less nodeGraphEdit row: {menu:?}");
}
//#endregion 🩹️RailVerbLaws

/// 🎯️ LAW: the editor declares the artifact kind it edits (the artifact's own `artifact_kind()`), which is
/// what the hub's one open-target rule (`app_opens_kind`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`)
/// pairs with this editor and the viewer of its dialect — so a Trinity rewrite rule can be created and opened as a hub document.
#[semio_framework_async_macros::async_test]
async fn the_editor_declares_the_artifact_kind_it_edits() {
    assert_eq!(create_rewriting_app().artifact_kinds, vec![crate::artifact_kind()]);
}

//#region 🕹️NodeDragLaws
async fn working_positions(app: &RewritingTestApp) -> Vec<(String, f64, f64)> {
    live_working(app).await.nodes.iter().map(|node| (node.id.value.clone(), node.position.x, node.position.y)).collect()
}

fn drag_row(gesture: &str, node_ids: &[&str], dx: f64, dy: f64) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::json!({ "operation": "move", "gestureId": gesture, "nodeIds": node_ids, "dx": dx, "dy": dy })
}

fn applied(state: &RewritingSnapshot, leaves: &[RewriteRuleMutation]) -> RewritingSnapshot {
    let mut next = state.clone();
    for leaf in leaves {
        crate::apply_rewrite_rule_mutation(&mut next, leaf).expect("the leaf applies");
    }
    next
}

/// ⚖️ LAW: a released working-graph drag (the hosts' `move` gesture record) is ONE edit of the composed `workingGraph` child holding
/// ONE relative `drag-nodes {targets, dx, dy}` (design §20.15) — every target moves by the offset, every other node stays — and ONE
/// undo restores the graph; two gestures are two rows, a release that moves nothing leaves zero trace, and a malformed record is
/// refused by name.
#[semio_framework_async_macros::async_test]
async fn a_released_working_graph_drag_is_one_tool_transaction_of_one_relative_leaf() {
    let mut app = new_app().await;
    let before = working_positions(&app).await;
    let ids = working_node_ids(&app).await;
    let (first, second) = (ids[0].as_str(), ids[1].as_str());
    let rows = edit_rows(&mut app).await;
    let receipt = graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::Value::Array(vec![drag_row("g-1", &[first, second], 24.0, -8.0)])).await.expect("a drag row is admitted");
    assert!(child_lane(&receipt), "the drag edits the working child: lanes {:?}", receipt.lanes);
    for ((id, x, y), (_, before_x, before_y)) in working_positions(&app).await.into_iter().zip(before.iter().cloned()) {
        let expected = if id == first || id == second { (before_x + 24.0, before_y - 8.0) } else { (before_x, before_y) };
        assert_eq!((x, y), expected, "node {id}");
    }
    assert_eq!(edit_rows(&mut app).await, rows + 1, "one drag, one row");
    graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::Value::Array(vec![drag_row("g-2", &[first], 1.0, 0.0)])).await.expect("a second drag");
    assert_eq!(edit_rows(&mut app).await, rows + 2, "two gestures are two rows");
    graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::Value::Array(vec![drag_row("g-3", &[first], 0.0, 0.0)])).await.expect("a zero drag");
    assert_eq!(edit_rows(&mut app).await, rows + 2, "a release that moved nothing leaves zero trace");
    history(&mut app, "undo").await;
    history(&mut app, "undo").await;
    assert_eq!(working_positions(&app).await, before, "the undo rows restore the working graph");
    let malformed = graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::json!([{ "operation": "move", "gestureId": "g", "nodeIds": [], "dx": 1.0, "dy": 0.0 }])).await;
    assert!(malformed.is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "a malformed gesture record is refused by name");
}

/// ⚖️ LAW: a rule-node drag on the LHS/RHS canvas is ONE relative `drag-rule-nodes` that moves each clause from where it sits —
/// its layout point, else its default slot — and ONE `set-rule-layout-points` row undoes it exactly (a node that had no point
/// is cleared back to its slot).
#[test]
fn a_rule_node_drag_moves_each_clause_from_where_it_sits_and_undoes_in_one_row() {
    let mut state = default_rule_state();
    state.rule_layout.insert("lhs-where".into(), LayoutPoint { x: 300.0, y: 90.0 });
    let operations = semio_framework_pack_json::Value::Array(vec![drag_row("g-lhs", &["lhs-match", "lhs-where"], 30.0, 10.0)]).to_string();
    let emit = crate::editor::rewriting::commands::node_graph_edit(&state, &semio_framework_plugin::app::ChildContentView::EMPTY, TRINITY_REWRITING_PLAY_SURFACE_LHS, &operations, "seed").expect("a rule-node drag");
    assert!(matches!(emit.artifact_mutations.as_slice(), [RewriteRuleMutation::DragRuleNodes(_)]), "{:?}", emit.artifact_mutations);
    let moved = applied(&state, &emit.artifact_mutations);
    assert_eq!(moved.rule_layout.get("lhs-match"), Some(&LayoutPoint { x: 30.0, y: 10.0 }), "the match moves from its default slot");
    assert_eq!(moved.rule_layout.get("lhs-where"), Some(&LayoutPoint { x: 330.0, y: 100.0 }), "the WHERE clause moves from its layout point");
    let inverse = crate::inverse_rewrite_rule_mutation(&state, &emit.artifact_mutations[0]).expect("valid retained mutation inverse fixture");
    assert!(matches!(inverse.as_slice(), [RewriteRuleMutation::SetRuleLayoutPoints(undo)] if undo.cleared == vec!["lhs-match".to_string()] && undo.points.len() == 1), "one exact inverse row: {inverse:?}");
    assert_eq!(applied(&moved, &inverse), state);
}

/// ⚖️ LAW (fixture `🧫️fixtures/🧫️node-graph-edit-rows`): every accepted row of the shared node-graph record vocabulary maps to a
/// child-lane leaf on the working graph — `connect` draws ONE `create-edge` carrying the graph's edge kind and the `source->target`
/// id, `disconnect` cuts ONE `delete-edge`, each undone by ONE row — every refused row refuses the whole batch by name,
/// `setSlider`/`insertPort` are refused (the graph has neither), and a rule side draws or cuts no wire alone.
#[semio_framework_async_macros::async_test]
async fn the_shared_node_graph_rows_map_to_intent_leaves() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json")).expect("node-graph-edit-rows fixture");
    let mut app = new_app().await;
    let row = |value: &serde_json::Value| semio_framework_pack_json::parse(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("corpus row");
    for refused in corpus["refused"].as_array().expect("refused rows") {
        let emit = graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::Value::Array(vec![row(&refused["row"])])).await;
        assert!(emit.is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "refused row {} is refused by name", refused["id"]);
    }
    let before = live_working(&app).await;
    let (source_node, target_node) = (before.nodes[0].id.value.clone(), before.nodes[1].id.value.clone());
    let connect = semio_framework_pack_json::json!([{ "operation": "connect", "sourceNodeId": source_node, "sourcePortId": "out", "targetNodeId": target_node, "targetPortId": "in" }]);
    let drawn = graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, connect.clone()).await.expect("a connect row");
    assert!(child_lane(&drawn), "the wire edits the working child: lanes {:?}", drawn.lanes);
    let wired = live_working(&app).await;
    let id = format!("{source_node}@out->{target_node}@in");
    assert_eq!(wired.edges.len(), before.edges.len() + 1, "one wire is drawn");
    assert!(wired.edges.iter().any(|edge| edge.id.value == id && edge.source.value == source_node && edge.target.value == target_node && edge.source_port.as_deref() == Some("out") && edge.target_port.as_deref() == Some("in") && edge.kind == before.edges[0].kind), "{:?}", wired.edges);
    history(&mut app, "undo").await;
    assert_eq!(live_working(&app).await, before, "one undo row restores the working graph");
    graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::json!([{ "operation": "disconnect", "synapseId": before.edges[0].id.value }])).await.expect("a disconnect row");
    let severed = live_working(&app).await;
    assert_eq!((severed.nodes.len(), severed.edges.len()), (before.nodes.len(), before.edges.len() - 1), "one wire is cut, every node stays");
    for rejected in [semio_framework_pack_json::json!({ "operation": "setSlider", "widgetId": "w", "value": 1.0 }), semio_framework_pack_json::json!({ "operation": "insertPort", "nodeId": source_node, "side": "input", "index": 0 })] {
        assert!(graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::Value::Array(vec![rejected.clone()])).await.is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "{rejected} is refused by name");
    }
    assert!(graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_LHS, connect).await.is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "a rule side draws no wire alone");
}

/// ⚖️ LAW: one canvas drag through the shell is ONE edit and ONE history row labelled from its `drag-nodes` leaf in every language,
/// and ONE undo moves the nodes back.
#[semio_framework_async_macros::async_test]
async fn one_canvas_drag_is_one_history_row_labelled_from_its_leaf() {
    use semio_framework_plugin::PluginApp;
    let mut app = new_app().await;
    let before = working_positions(&app).await;
    let ids = working_node_ids(&app).await;
    let rows_before = edit_rows(&mut app).await;
    graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::Value::Array(vec![drag_row("g-shell", &[ids[0].as_str(), ids[1].as_str()], 24.0, -8.0)])).await.expect("the drag is admitted");
    let rows: Vec<_> = app.history_snapshot().await.expect("history").upserts.into_iter().filter(|row| row.edit_id.is_some()).collect();
    assert_eq!(rows.len(), rows_before + 1, "one drag, one edit, one row");
    let row = rows.iter().max_by_key(|row| row.seq).expect("the drag's row");
    assert_eq!(row.mutations.len(), 1, "one relative leaf");
    assert_eq!(row.mutations[0].label.resolve(Terminology::Native, Locale::En), "Drag 2 nodes by (24, -8)");
    assert_eq!(row.mutations[0].label.resolve(Terminology::Native, Locale::De), "2 Knoten um (24; -8) ziehen");
    history(&mut app, "undo").await;
    assert_eq!(working_positions(&app).await, before, "one undo moves the nodes back");
}

/// ⚖️ LAW: a `delete` row on the working graph is ONE edit of the composed `workingGraph` child: the `delete-edge` of every wire
/// touching each named node, then its `delete-node`, then the `delete-edge` of every wire it names apart from those — a node the graph
/// does not hold is skipped, never a whole-graph write — and ONE undo restores the graph.
#[semio_framework_async_macros::async_test]
async fn a_deleted_working_graph_selection_is_relative_leaves() {
    let mut app = new_app().await;
    let before = live_working(&app).await;
    let touches = |edge: &semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphEdge, id: &str| edge.source.value == id || edge.target.value == id;
    let doomed = before.nodes.iter().map(|node| node.id.value.clone()).find(|id| before.edges.iter().any(|edge| touches(edge, id))).expect("a node with an edge");
    let touching = before.edges.iter().filter(|edge| touches(edge, &doomed)).count();
    let apart = before.edges.iter().find(|edge| !touches(edge, &doomed)).map(|edge| edge.id.value.clone());
    let synapses: Vec<String> = before.edges.iter().filter(|edge| touches(edge, &doomed)).take(1).map(|edge| edge.id.value.clone()).chain(apart.clone()).collect();
    let receipt = graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::json!([{ "operation": "delete", "nodeIds": [doomed, "absent"], "synapseIds": synapses }])).await.expect("a delete");
    assert!(child_lane(&receipt), "the delete edits the working child: lanes {:?}", receipt.lanes);
    let after = live_working(&app).await;
    assert!(after.nodes.iter().all(|node| node.id.value != doomed) && after.nodes.len() + 1 == before.nodes.len(), "only the named node the graph holds is gone");
    assert_eq!(after.edges.len() + touching + usize::from(apart.is_some()), before.edges.len(), "every edge touching it and the named wire apart are gone, every other edge stays");
    history(&mut app, "undo").await;
    assert_eq!(live_working(&app).await, before, "one undo row restores the working graph");
    let rule = graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_LHS, semio_framework_pack_json::json!([{ "operation": "delete", "nodeIds": [], "synapseIds": ["lhs-wire"] }])).await;
    assert!(rule.is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "a rule wire is cut with its clause, never alone");
}
/// ⚖️ LAW (audit T1/T2): the guest's own add-node verb is ONE `create-node` of the composed `workingGraph` child carrying the first
/// free `n<k>` id, the graph's node kind, the id as name and the requested position — it lands exactly that node, a second add
/// takes the next free id and its given name, a non-finite position is refused by name before any child is read — and ONE undo
/// removes the added node.
#[semio_framework_async_macros::async_test]
async fn the_add_node_verb_is_one_relative_leaf_and_canonical_graphs_undo_relatively() {
    let mut app = new_app().await;
    let before = live_working(&app).await;
    let add = |pairs: &[(&str, semio_framework_pack_json::Value)]| semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::Value::Object(pairs.iter().map(|(key, value)| ((*key).to_string(), value.clone())).collect()));
    app.handle_action("addWorkingNode", Some(&add(&[("x", semio_framework_pack_json::json!(40.0)), ("y", semio_framework_pack_json::json!(-20.0))])), &meta("local")).await.expect("an add");
    assert!(child_lane(&settle(&mut app).await), "the add edits the working child");
    let added = live_working(&app).await;
    assert_eq!(added.nodes.len(), before.nodes.len() + 1, "exactly one node lands");
    let first = added.nodes.iter().find(|node| before.nodes.iter().all(|held| held.id != node.id)).expect("the added node").clone();
    assert!(first.id.value.starts_with('n') && first.label == first.id.value && (first.position.x, first.position.y) == (40.0, -20.0), "{first:?}");
    app.handle_action("addWorkingNode", Some(&add(&[("kind", semio_framework_pack_json::json!(first.kind)), ("name", semio_framework_pack_json::json!("Second")), ("x", semio_framework_pack_json::json!(0.0)), ("y", semio_framework_pack_json::json!(0.0))])), &meta("local")).await.expect("a second add");
    settle(&mut app).await;
    let again = live_working(&app).await;
    let second = again.nodes.iter().find(|node| added.nodes.iter().all(|held| held.id != node.id)).expect("the second node");
    assert!(second.id != first.id && second.label == "Second", "{second:?}");
    let nan = crate::editor::rewriting::commands::add_working_node_command(&default_rule_state(), &semio_framework_plugin::app::ChildContentView::EMPTY, None, None, f64::NAN, 0.0);
    assert!(nan.is_err_and(|fault| fault.code.0 == "app.command.invalid-args"), "a non-finite position is refused by name");
    history(&mut app, "undo").await;
    assert_eq!(live_working(&app).await, added, "one undo removes the second node");
}
/// ⚖️ LAW (Binary64 transport): a time-travel edit of a binary64 input — the `/dx` offset of a released rule-node drag's
/// `drag-rule-nodes` — validates against its leaf schema (`Binary64Transport`: the exact word or a plain number, the form
/// `payload_value` and the history editor carry) and replays: the overwrite moves the dragged clause by the edited offset.
#[semio_framework_async_macros::async_test]
async fn a_history_edit_of_a_binary64_offset_validates_and_replays() {
    use semio_framework::kernel::HistoryTimeTravelStage;
    use semio_framework_plugin::PluginApp;
    let mut app = new_app().await;
    graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_LHS, semio_framework_pack_json::Value::Array(vec![drag_row("g-history", &["lhs-match"], 24.0, -8.0)])).await.expect("the drag is admitted");
    assert_eq!(app.snapshot().expect("dragged").rule_layout.get("lhs-match"), Some(&LayoutPoint { x: 24.0, y: -8.0 }), "the match moves from its default slot");
    let rows = app.history_snapshot().await.expect("history").upserts;
    let mutation_id = rows.iter().filter(|row| row.edit_id.is_some()).max_by_key(|row| row.seq).and_then(|row| row.mutations.first()).map(|mutation| mutation.mutation_id.clone()).expect("the drag's leaf");
    async fn stage(app: &mut RewritingTestApp) -> Option<HistoryTimeTravelStage> {
        app.history_snapshot().await.expect("history").time_travel.map(|status| status.stage)
    }
    let verbs = [
        ("historyEditBegin", semio_framework_pack_json::json!({ "mutationId": mutation_id })),
        ("historyEditInput", semio_framework_pack_json::json!({ "path": "/dx", "value": 50.0 })),
        ("historyEditAccept", semio_framework_pack_json::json!({})),
    ];
    for (verb, args) in verbs {
        let result = app.handle_action(verb, Some(&semio_framework_pack_json::to_dsl_value(&args)), &meta("local")).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
        assert!(result.output.get("rejected").is_none(), "{verb} was refused: {:?}", result.output);
    }
    for _ in 0..10_000 {
        if stage(&mut app).await != Some(HistoryTimeTravelStage::Replaying) {
            break;
        }
        app.0.advance_typed_operation_publication().await.expect("a driver turn");
        while app.0.take_typed_operation_ui_progress().is_some() {}
    }
    assert_eq!(stage(&mut app).await, Some(HistoryTimeTravelStage::Reviewing), "the edited offset replays to a clean review");
    for (verb, args) in [("historyEditFinalize", semio_framework_pack_json::json!({})), ("historyEditCommit", semio_framework_pack_json::json!({ "choice": "overwrite" }))] {
        let result = app.handle_action(verb, Some(&semio_framework_pack_json::to_dsl_value(&args)), &meta("local")).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
        assert!(result.output.get("rejected").is_none(), "{verb} was refused: {:?}", result.output);
    }
    for _ in 0..10_000 {
        if stage(&mut app).await.is_none() {
            break;
        }
        app.0.advance_typed_operation_publication().await.expect("a driver turn");
        while app.0.take_typed_operation_ui_progress().is_some() {}
    }
    assert_eq!(app.snapshot().expect("edited head").rule_layout.get("lhs-match"), Some(&LayoutPoint { x: 50.0, y: -8.0 }), "the overwrite replays the edited offset");
}

//#endregion 🕹️NodeDragLaws

#[path = "../🪆️child-frame/🦀️.rs"]
mod full_child_frame_laws;
