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
    assert_ne!(state.before_fixture_json, after_fixture_json(&state));
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
    let rhs: Rhs = pack::from_json_str(&app.snapshot().unwrap().rhs_json).unwrap();
    assert_eq!(rhs.set.len(), 2);
    let result = app
        .dispatch_typed(TrinityRewritingCommand::NodeGraphEdit { surface_id: TRINITY_REWRITING_PLAY_SURFACE_RHS.into(), operations_json: pack::json!([{ "operation": "delete", "nodeIds": ["rhs-set-1"], "synapseIds": [] }]).to_string() }, &meta("local"))
        .await
        .expect("delete the clause node");
    let receipt = settle(&mut app).await;
    assert!(!result.mutations.is_empty() || receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact), "a delete row must publish an artifact mutation: {:?}", receipt.lanes);
    let rhs: Rhs = pack::from_json_str(&app.snapshot().unwrap().rhs_json).unwrap();
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
}

//#region 🩹️RailVerbLaws
fn working_graph_node_ids(state: &RewritingSnapshot) -> Vec<String> {
    semio_s_artifact_trinity_jack::JackSnapshot::from_json(&state.before_fixture_json).expect("the working graph decodes").nodes().into_iter().map(|node| node.id).collect()
}

fn working_graph_node_name(state: &RewritingSnapshot, id: &str) -> String {
    semio_s_artifact_trinity_jack::JackSnapshot::from_json(&state.before_fixture_json).expect("the working graph decodes").nodes().into_iter().find(|node| node.id == id).map(|node| node.name).expect("node")
}

/// ⚖️ LAW: `patchNodes` pressed from the rail with an EMPTY `nodeIds` patches the selected nodes of the
/// working graph, and a comma list in the text field names several nodes (S15: the verb "moved nothing").
#[semio_framework_async_macros::async_test]
async fn patch_nodes_from_the_rail_patches_the_selection_or_the_listed_nodes() {
    let mut app = new_app().await;
    let ids = working_graph_node_ids(&app.snapshot().expect("projection"));
    select_graph(&mut app, &[&ids[0]]).await;
    let rail = |pairs: &[(&str, &str)]| pack::json_to_dsl_value(&pack::JsonValue::Object(pairs.iter().map(|(key, value)| ((*key).to_string(), pack::JsonValue::String((*value).to_string()))).collect()));
    app.handle_action("patchNodes", Some(&rail(&[("field", "name"), ("value", "S15 Selected")])), &meta("local")).await.expect("an empty nodeIds patches the selection");
    settle(&mut app).await;
    assert_eq!(working_graph_node_name(&app.snapshot().expect("projection"), &ids[0]), "S15 Selected");
    app.handle_action("patchNodes", Some(&rail(&[("nodeIds", &format!("{} {}", ids[0], ids[1])), ("field", "name"), ("value", "S15 Listed")])), &meta("local")).await.expect("a listed nodeIds patches both nodes");
    settle(&mut app).await;
    let state = app.snapshot().expect("projection");
    assert_eq!((working_graph_node_name(&state, &ids[0]), working_graph_node_name(&state, &ids[1])), ("S15 Listed".to_string(), "S15 Listed".to_string()));
}

/// ⚖️ LAW: a `patchNodes` that cannot move the document is refused by name — an unknown id is
/// `mutation.target-missing`, no id and no selection is `app.command.targets-required` (the precondition an agent,
/// which has no selection, meets by naming `nodeIds`), an unsupported field or an empty value is
/// `app.command.invalid-args`. It used to answer an empty emit that read as an accepted edit.
#[semio_framework_async_macros::async_test]
async fn patch_nodes_refuses_what_it_cannot_apply() {
    let state = default_rule_state();
    let first = working_graph_node_ids(&state)[0].clone();
    let code = |result: Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault>| result.err().expect("refused").code.0;
    assert_eq!(code(crate::editor::rewriting::commands::patch_nodes(&state, &["no-such-node".into()], &[], "name", "x")), "mutation.target-missing");
    assert_eq!(code(crate::editor::rewriting::commands::patch_nodes(&state, &[], &[], "name", "x")), "app.command.targets-required");
    assert_eq!(code(crate::editor::rewriting::commands::patch_nodes(&state, &[first.clone()], &[], "colour", "x")), "app.command.invalid-args");
    assert_eq!(code(crate::editor::rewriting::commands::patch_nodes(&state, &[first.clone()], &[], "kind", " ")), "app.command.invalid-args");
    assert_eq!(code(crate::editor::rewriting::commands::patch_nodes(&state, &[first.clone()], &[], "kind", "NoSuchKind")), "app.command.invalid-args", "a kind the manifest does not declare is refused, not written");
    assert!(!crate::editor::rewriting::commands::patch_nodes(&state, &[], &[first], "name", "Beam").expect("the selection is the target").artifact_mutations.is_empty());
}

/// 🚫️ LAW: `addRuleClause` refuses what it cannot add — a second WHERE clause, an unknown clause kind, a rule whose
/// own JSON no longer decodes — instead of an empty emit (S15: `kind=where` on the default rule journalled a row and
/// changed nothing).
#[semio_framework_async_macros::async_test]
async fn add_rule_clause_refuses_what_it_cannot_add() {
    let state = default_rule_state();
    let add = crate::editor::rewriting::commands::add_rule_clause_command;
    let code = |result: Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault>| result.err().expect("refused").code.0;
    assert_eq!(code(add(&state, "where")), "app.command.invalid-args", "the default rule already has a WHERE clause");
    assert_eq!(code(add(&state, "optional")), "app.command.invalid-args");
    let broken = RewritingSnapshot { lhs_json: "{".into(), ..state.clone() };
    assert_eq!(code(add(&broken, "create")), "trinity.rewriting.rule-undecodable");
}

/// ⏪️ LAW: a rail `addRuleClause kind=create` is ONE undoable edit that rewrites only the RHS — undo restores the
/// rule and redo adds the clause again (S15, session 12: its row carried a re-printed LHS and undo did nothing).
#[semio_framework_async_macros::async_test]
async fn a_rail_add_rule_clause_rewrites_only_the_rhs_and_round_trips() {
    let mut app = new_app().await;
    let before = app.snapshot().expect("projection");
    let emit = crate::editor::rewriting::commands::add_rule_clause_command(&before, "create").expect("create clause");
    assert!(!emit.artifact_mutations.is_empty() && emit.artifact_mutations.iter().all(|mutation| matches!(mutation, RewriteRuleMutation::EditRhs(_))), "only the RHS changes: {:?}", emit.artifact_mutations);
    let rail = pack::json_to_dsl_value(&pack::json!({ "kind": "create" }));
    app.handle_action("addRuleClause", Some(&rail), &meta("local")).await.expect("addRuleClause");
    settle(&mut app).await;
    let after = app.snapshot().expect("projection");
    assert_ne!(after.rhs_json, before.rhs_json, "the clause is added");
    assert_eq!(after.lhs_json, before.lhs_json, "the LHS is untouched");
    history(&mut app, "undo").await;
    assert_eq!(app.snapshot().expect("projection").rhs_json, before.rhs_json, "undo restores the RHS");
    history(&mut app, "redo").await;
    assert_eq!(app.snapshot().expect("projection").rhs_json, after.rhs_json, "redo adds the clause again");
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
fn working_graph_positions(state: &RewritingSnapshot) -> Vec<(String, f64, f64)> {
    semio_s_artifact_trinity_jack::JackSnapshot::from_json(&state.before_fixture_json).expect("the working graph decodes").nodes().into_iter().map(|node| (node.id, node.x, node.y)).collect()
}

fn drag_row(gesture: &str, node_ids: &[&str], dx: f64, dy: f64) -> pack::JsonValue {
    pack::json!({ "operation": "move", "gestureId": gesture, "nodeIds": node_ids, "dx": dx, "dy": dy })
}

fn applied(state: &RewritingSnapshot, leaves: &[RewriteRuleMutation]) -> RewritingSnapshot {
    let mut next = state.clone();
    for leaf in leaves {
        crate::apply_rewrite_rule_mutation(&mut next, leaf).expect("the leaf applies");
    }
    next
}

/// ⚖️ LAW: a released working-graph drag (the hosts' `move` gesture record) is ONE tool transaction of `<appId>#nodeGraphEdit`
/// holding ONE relative `drag-working-nodes {targets, dx, dy}` — every target moves by the offset, every other node stays — and
/// ONE inverse row restores the graph; two gestures are two transactions, a seedless view publishes the leaf plainly, and a
/// release that moves nothing leaves zero trace.
#[test]
fn a_released_working_graph_drag_is_one_tool_transaction_of_one_relative_leaf() {
    let state = default_rule_state();
    let ids = working_graph_node_ids(&state);
    let (first, second) = (ids[0].as_str(), ids[1].as_str());
    let operations = pack::JsonValue::Array(vec![drag_row("g-1", &[first, second], 24.0, -8.0)]).to_string();
    let emit = crate::editor::rewriting::commands::node_graph_edit(&state, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, &operations, "seed").expect("a drag row is admitted");
    let transaction = emit.transaction.clone().expect("the release is a tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.trinity.rewriting@1/*#editor#nodeGraphEdit", "{transaction:?}");
    assert!(emit.coalesce_key.is_none(), "a committed transaction is a plain edit");
    assert!(matches!(emit.artifact_mutations.as_slice(), [RewriteRuleMutation::DragWorkingNodes(leaf)] if leaf.targets == vec![first.to_string(), second.to_string()] && (leaf.dx, leaf.dy) == (24.0, -8.0)), "{:?}", emit.artifact_mutations);
    let moved = applied(&state, &emit.artifact_mutations);
    for ((id, x, y), (_, before_x, before_y)) in working_graph_positions(&moved).into_iter().zip(working_graph_positions(&state)) {
        let expected = if id == first || id == second { (before_x + 24.0, before_y - 8.0) } else { (before_x, before_y) };
        assert_eq!((x, y), expected, "node {id}");
    }
    let inverse = crate::inverse_rewrite_rule_mutation(&state, &emit.artifact_mutations[0]);
    assert!(matches!(inverse.as_slice(), [RewriteRuleMutation::EditBeforeFixture(_)]), "one inverse row: {inverse:?}");
    assert_eq!(applied(&moved, &inverse), state, "the inverse row restores the working graph");
    let again = crate::editor::rewriting::commands::node_graph_edit(&state, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, &pack::JsonValue::Array(vec![drag_row("g-2", &[first], 1.0, 0.0)]).to_string(), "seed").expect("a second drag");
    assert_ne!(again.transaction.expect("second ref").id, transaction.id, "two gestures are two transactions");
    let plain = crate::editor::rewriting::commands::node_graph_edit(&state, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, &operations, "").expect("a seedless drag");
    assert!(plain.transaction.is_none() && plain.artifact_mutations.len() == 1, "a view without command authority publishes the leaf plainly");
    let idle = crate::editor::rewriting::commands::node_graph_edit(&state, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, &pack::JsonValue::Array(vec![drag_row("g-3", &[first], 0.0, 0.0)]).to_string(), "seed").expect("a zero drag");
    assert!(idle.artifact_mutations.is_empty() && idle.transaction.is_none(), "a release that moved nothing leaves zero trace");
    let malformed = crate::editor::rewriting::commands::node_graph_edit(&state, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, &pack::json!([{ "operation": "move", "gestureId": "g", "nodeIds": [], "dx": 1.0, "dy": 0.0 }]).to_string(), "seed");
    assert!(malformed.is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "a malformed gesture record is refused by name");
}

/// ⚖️ LAW: a rule-node drag on the LHS/RHS canvas is ONE relative `drag-rule-nodes` that moves each clause from where it sits —
/// its layout point, else its default slot — and ONE `set-rule-layout-points` row undoes it exactly (a node that had no point
/// is cleared back to its slot).
#[test]
fn a_rule_node_drag_moves_each_clause_from_where_it_sits_and_undoes_in_one_row() {
    let mut state = default_rule_state();
    state.rule_layout.insert("lhs-where".into(), LayoutPoint { x: 300.0, y: 90.0 });
    let operations = pack::JsonValue::Array(vec![drag_row("g-lhs", &["lhs-match", "lhs-where"], 30.0, 10.0)]).to_string();
    let emit = crate::editor::rewriting::commands::node_graph_edit(&state, TRINITY_REWRITING_PLAY_SURFACE_LHS, &operations, "seed").expect("a rule-node drag");
    assert!(matches!(emit.artifact_mutations.as_slice(), [RewriteRuleMutation::DragRuleNodes(_)]), "{:?}", emit.artifact_mutations);
    let moved = applied(&state, &emit.artifact_mutations);
    assert_eq!(moved.rule_layout.get("lhs-match"), Some(&LayoutPoint { x: 30.0, y: 10.0 }), "the match moves from its default slot");
    assert_eq!(moved.rule_layout.get("lhs-where"), Some(&LayoutPoint { x: 330.0, y: 100.0 }), "the WHERE clause moves from its layout point");
    let inverse = crate::inverse_rewrite_rule_mutation(&state, &emit.artifact_mutations[0]);
    assert!(matches!(inverse.as_slice(), [RewriteRuleMutation::SetRuleLayoutPoints(undo)] if undo.cleared == vec!["lhs-match".to_string()] && undo.points.len() == 1), "one exact inverse row: {inverse:?}");
    assert_eq!(applied(&moved, &inverse), state);
}

/// ⚖️ LAW (fixture `🧫️fixtures/🧫️node-graph-edit-rows`): every accepted row of the shared node-graph record vocabulary maps to an
/// intent leaf on the working graph — `connect` draws ONE `connect-working-ports` carrying the graph's edge kind, `disconnect` cuts
/// ONE `disconnect-working-edges` — every refused row refuses the whole batch by name, `setSlider`/`insertPort` are refused (the
/// graph has neither), and a rule side draws or cuts no wire alone.
#[test]
fn the_shared_node_graph_rows_map_to_intent_leaves() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json")).expect("node-graph-edit-rows fixture");
    let state = default_rule_state();
    let edit = |surface: &str, rows: serde_json::Value| crate::editor::rewriting::commands::node_graph_edit(&state, surface, &rows.to_string(), "seed");
    for refused in corpus["refused"].as_array().expect("refused rows") {
        let emit = edit(TRINITY_REWRITING_PLAY_SURFACE_BEFORE, serde_json::json!([refused["row"]]));
        assert!(emit.is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "refused row {} is refused by name", refused["id"]);
    }
    let graph = semio_s_artifact_trinity_jack::JackSnapshot::from_json(&state.before_fixture_json).expect("the working graph decodes");
    let (nodes, edges) = (graph.nodes(), graph.edges());
    let connect = serde_json::json!([{ "operation": "connect", "sourceNodeId": nodes[0].id, "sourcePortId": "out", "targetNodeId": nodes[1].id, "targetPortId": "in" }]);
    let drawn = edit(TRINITY_REWRITING_PLAY_SURFACE_BEFORE, connect.clone()).expect("a connect row");
    let (source, target) = (format!("{}@out", nodes[0].id), format!("{}@in", nodes[1].id));
    assert!(drawn.transaction.is_none() && matches!(drawn.artifact_mutations.as_slice(), [RewriteRuleMutation::ConnectWorkingPorts(leaf)] if leaf.source == source && leaf.target == target && leaf.kind == edges[0].kind), "{:?}", drawn.artifact_mutations);
    let wired = applied(&state, &drawn.artifact_mutations);
    let after = semio_s_artifact_trinity_jack::JackSnapshot::from_json(&wired.before_fixture_json).expect("the wired graph decodes");
    assert_eq!(after.edges().len(), edges.len() + 1, "one wire is drawn");
    assert!(after.edges().iter().any(|edge| edge.source == source && edge.target == target && edge.id == format!("{source}->{target}")));
    let inverse = crate::inverse_rewrite_rule_mutation(&state, &drawn.artifact_mutations[0]);
    assert!(matches!(inverse.as_slice(), [RewriteRuleMutation::EditBeforeFixture(_)]), "one inverse row: {inverse:?}");
    assert_eq!(applied(&wired, &inverse), state, "the inverse row restores the working graph");
    let cut = edit(TRINITY_REWRITING_PLAY_SURFACE_BEFORE, serde_json::json!([{ "operation": "disconnect", "synapseId": edges[0].id }])).expect("a disconnect row");
    assert!(matches!(cut.artifact_mutations.as_slice(), [RewriteRuleMutation::DisconnectWorkingEdges(leaf)] if leaf.targets == vec![edges[0].id.clone()]), "{:?}", cut.artifact_mutations);
    let severed = semio_s_artifact_trinity_jack::JackSnapshot::from_json(&applied(&state, &cut.artifact_mutations).before_fixture_json).expect("the cut graph decodes");
    assert_eq!((severed.nodes().len(), severed.edges().len()), (nodes.len(), edges.len() - 1), "one wire is cut, every node stays");
    for row in [serde_json::json!({ "operation": "setSlider", "widgetId": "w", "value": 1.0 }), serde_json::json!({ "operation": "insertPort", "nodeId": nodes[0].id, "side": "input", "index": 0 })] {
        assert!(edit(TRINITY_REWRITING_PLAY_SURFACE_BEFORE, serde_json::json!([row])).is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "{row} is refused by name");
    }
    assert!(edit(TRINITY_REWRITING_PLAY_SURFACE_LHS, connect).is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "a rule side draws no wire alone");
}

/// ⚖️ LAW: one canvas drag through the shell is ONE edit and ONE history row labelled from its leaf in every language, and ONE
/// undo moves the nodes back.
#[semio_framework_async_macros::async_test]
async fn one_canvas_drag_is_one_history_row_labelled_from_its_leaf() {
    use semio_framework_plugin::PluginApp;
    let mut app = new_app().await;
    let before = app.snapshot().expect("projection");
    let ids = working_graph_node_ids(&before);
    let rows_before = app.history_snapshot().await.expect("history").upserts.into_iter().filter(|row| row.edit_id.is_some()).count();
    let operations = pack::JsonValue::Array(vec![drag_row("g-shell", &[ids[0].as_str(), ids[1].as_str()], 24.0, -8.0)]).to_string();
    let args = pack::json_to_dsl_value(&pack::json!({ "surfaceId": TRINITY_REWRITING_PLAY_SURFACE_BEFORE, "operations": operations }));
    app.handle_action("nodeGraphEdit", Some(&args), &meta("local")).await.expect("the drag is admitted");
    settle(&mut app).await;
    let rows: Vec<_> = app.history_snapshot().await.expect("history").upserts.into_iter().filter(|row| row.edit_id.is_some()).collect();
    assert_eq!(rows.len(), rows_before + 1, "one drag, one edit, one row");
    let row = rows.iter().max_by_key(|row| row.seq).expect("the drag's row");
    assert_eq!(row.mutations.len(), 1, "one relative leaf");
    assert_eq!(row.mutations[0].label.resolve(Terminology::Native, Locale::En), "Drag 2 nodes by (24, -8)");
    assert_eq!(row.mutations[0].label.resolve(Terminology::Native, Locale::De), "2 Knoten um (24; -8) ziehen");
    history(&mut app, "undo").await;
    assert_eq!(working_graph_positions(&app.snapshot().expect("projection")), working_graph_positions(&before), "one undo moves the nodes back");
}

/// ⚖️ LAW: a `delete` row on the working graph is ONE relative `delete-working-nodes` of its nodes — every edge touching them goes
/// with them — followed by ONE `disconnect-working-edges` of the wires it names apart from those; each undoes with ONE row, and a
/// node the graph does not hold is skipped (`mutation.partial`), never a whole-graph write.
#[test]
fn a_deleted_working_graph_selection_is_relative_leaves() {
    use semio_s_artifact_trinity_jack::port_node_id;
    let state = default_rule_state();
    let graph = semio_s_artifact_trinity_jack::JackSnapshot::from_json(&state.before_fixture_json).expect("the working graph decodes");
    let touches = |edge: &semio_s_artifact_trinity_jack::Edge, id: &str| port_node_id(&edge.source).unwrap_or(&edge.source) == id || port_node_id(&edge.target).unwrap_or(&edge.target) == id;
    let doomed = graph.nodes().into_iter().map(|node| node.id).find(|id| graph.edges().iter().any(|edge| touches(edge, id))).expect("a node with an edge");
    let touching: Vec<String> = graph.edges().iter().filter(|edge| touches(edge, &doomed)).map(|edge| edge.id.clone()).collect();
    let apart = graph.edges().into_iter().find(|edge| !touches(edge, &doomed)).map(|edge| edge.id);
    let synapses: Vec<String> = touching.iter().take(1).cloned().chain(apart.clone()).collect();
    let delete = serde_json::json!([{ "operation": "delete", "nodeIds": [doomed, "absent"], "synapseIds": synapses }]).to_string();
    let emit = crate::editor::rewriting::commands::node_graph_edit(&state, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, &delete, "seed").expect("a delete");
    match apart.clone() {
        Some(apart) => assert!(matches!(emit.artifact_mutations.as_slice(), [RewriteRuleMutation::DeleteWorkingNodes(nodes), RewriteRuleMutation::DisconnectWorkingEdges(wires)] if nodes.targets == vec![doomed.clone(), "absent".to_string()] && wires.targets == vec![apart.clone()]), "{:?}", emit.artifact_mutations),
        None => assert!(matches!(emit.artifact_mutations.as_slice(), [RewriteRuleMutation::DeleteWorkingNodes(_)]), "{:?}", emit.artifact_mutations),
    }
    let deleted = applied(&state, &emit.artifact_mutations);
    let after = semio_s_artifact_trinity_jack::JackSnapshot::from_json(&deleted.before_fixture_json).expect("the deleted graph decodes");
    assert!(after.nodes().iter().all(|node| node.id != doomed) && after.nodes().len() + 1 == graph.nodes().len(), "only the named node the graph holds is gone");
    assert_eq!(after.edges().len() + touching.len() + usize::from(apart.is_some()), graph.edges().len(), "every edge touching it and the named wire apart are gone, every other edge stays");
    let outcome = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&emit.artifact_mutations[0], &state);
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.partial"), "the node the graph lacks is skipped: {:?}", outcome.messages());
    let inverse = crate::inverse_rewrite_rule_mutation(&state, &emit.artifact_mutations[0]);
    assert!(matches!(inverse.as_slice(), [RewriteRuleMutation::EditBeforeFixture(_)]), "one inverse row: {inverse:?}");
    let rule = crate::editor::rewriting::commands::node_graph_edit(&state, TRINITY_REWRITING_PLAY_SURFACE_LHS, &serde_json::json!([{ "operation": "delete", "nodeIds": [], "synapseIds": ["lhs-wire"] }]).to_string(), "seed");
    assert!(rule.is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "a rule wire is cut with its clause, never alone");
}
//#endregion 🕹️NodeDragLaws
