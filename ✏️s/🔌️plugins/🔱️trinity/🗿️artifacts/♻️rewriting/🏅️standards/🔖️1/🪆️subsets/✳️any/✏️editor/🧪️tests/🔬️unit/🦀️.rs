use super::*;
use crate::standards::v1::subsets::any::schema::Rhs;
use protocol::{OpBinary, OpText};
use semio_framework_plugin::{testkit, App, EditorApp, Locale, PluginApp, Terminology, VcsArtifactApp, ViewModel};

/// 🎫️ See `jack`'s `trinity_jack_manifest_for_testkit` doc comment for why this wrapper exists
/// (SDK gap, `testkit::new_app_with_registry`'s signature is still `fn(manifest: fn() -> App)`).
fn trinity_rewriting_manifest_for_testkit() -> App {
    App { definition: create_rewriting_app(), examples: Vec::new() }
}

fn meta(actor: &str) -> semio_framework_plugin::ActionMeta {
    testkit::meta(actor)
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
        TrinityRewritingCommand::SetViewport { surface_id: Some("trinity.rewriting.before".into()), viewport_json: "{\"x\":1.0,\"y\":2.0,\"zoom\":1.0}".into() },
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

/// 🕹️ Registry-backed (not the bare `testkit::new_app`): `interactionSelect`/`interactionHover`
/// resolve the dispatching app's declared `AppActionRegistry.interactions`, so any test exercising
/// domain "graph" selection needs the real manifest's `.interaction(...)` declaration present.
async fn new_app() -> VcsArtifactApp<EditorApp<TrinityRewritingPlayApp>> {
    testkit::new_app_with_registry::<EditorApp<TrinityRewritingPlayApp>>(trinity_rewriting_manifest_for_testkit).await
}

/// 🕹️ Dispatches the framework-injected `interactionSelect` verb against domain "graph" — the
/// replacement for the deleted `TrinityRewritingCommand::SetSelection`.
async fn select_graph(app: &mut VcsArtifactApp<EditorApp<TrinityRewritingPlayApp>>, ids: &[&str]) {
    let targets: Vec<pack::JsonValue> = ids.iter().map(|id| pack::json!({ "granularity": "node", "id": id })).collect();
    let args = pack::json_to_dsl_value(&pack::json!({ "domainId": "graph", "targets": pack::to_json_string(&targets) }));
    app.handle_action("interactionSelect", Some(&args), &meta("local")).await.expect("interactionSelect");
}

#[semio_framework_async_macros::async_test]
async fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
    let mut app = testkit::new_app_with_registry::<EditorApp<TrinityRewritingPlayApp>>(trinity_rewriting_manifest_for_testkit).await;
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
    let before = app.render(TRINITY_REWRITING_PLAY_BODY_BEFORE, None, &ViewModel::default()).await.expect("render");
    let after = app.render(TRINITY_REWRITING_PLAY_BODY_AFTER, None, &ViewModel::default()).await.expect("render");
    assert!(serde_json::to_string(&before.root).expect("serialize semantic UI test tree").contains("node-graph"));
    assert!(serde_json::to_string(&after.root).expect("serialize semantic UI test tree").contains("node-graph"));
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
    let lhs_json = serde_json::to_string(&app.render(TRINITY_REWRITING_PLAY_BODY_LHS, None, &ViewModel::default()).await.expect("render").root).expect("serialize semantic UI test tree");
    let rhs_json = serde_json::to_string(&app.render(TRINITY_REWRITING_PLAY_BODY_RHS, None, &ViewModel::default()).await.expect("render").root).expect("serialize semantic UI test tree");
    assert!(lhs_json.contains("node-graph"));
    assert!(rhs_json.contains("node-graph"));
    assert!(lhs_json.contains("\"editable\":true"));
    assert!(rhs_json.contains("\"editable\":true"));
}

#[semio_framework_async_macros::async_test]
async fn set_parameter_emits_one_op_and_is_undoable() {
    let mut app = new_app().await;
    let result = app.dispatch_typed(TrinityRewritingCommand::SetParameter { name: "label".into(), value: "changed".into() }, &meta("local")).await.expect("set parameter");
    assert_eq!(result.mutations.len(), 1, "a single-key parameter edit is one ChangeParameterBinding operation");
    assert_eq!(app.snapshot().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("changed".into())));
    app.handle_action("undo", None, &meta("local")).await.expect("undo");
    assert_eq!(app.snapshot().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("nakagin-core".into())));
}

#[semio_framework_async_macros::async_test]
async fn add_and_delete_rhs_set_clause() {
    let mut app = new_app().await;
    app.dispatch_typed(TrinityRewritingCommand::AddRuleClause { kind: "set".into() }, &meta("local")).await.expect("add clause");
    let rhs: Rhs = pack::from_json_str(&app.snapshot().unwrap().rhs_json).unwrap();
    assert_eq!(rhs.set.len(), 2);
    select_graph(&mut app, &["rhs-set-1"]).await;
    let result = app
        .dispatch_typed(TrinityRewritingCommand::NodeGraphEdit { surface_id: TRINITY_REWRITING_PLAY_SURFACE_RHS.into(), operations_json: pack::json!([{ "operation": "deleteSelection" }]).to_string() }, &meta("local"))
        .await
        .expect("delete selection");
    assert!(!result.mutations.is_empty());
    let rhs: Rhs = pack::from_json_str(&app.snapshot().unwrap().rhs_json).unwrap();
    assert_eq!(rhs.set.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn jack_view_renders_compiled_query_tokens() {
    let mut app = new_app().await;
    let node = app.render(TRINITY_REWRITING_PLAY_BODY_JACK, None, &ViewModel::default()).await.expect("render");
    assert!(serde_json::to_string(&node.root).expect("serialize semantic UI test tree").contains("tokensJson"));
}

#[semio_framework_async_macros::async_test]
async fn graph_scenes_have_lod_json() {
    let mut app = new_app().await;
    let before = app.render(TRINITY_REWRITING_PLAY_BODY_BEFORE, None, &ViewModel::default()).await.expect("render");
    assert!(serde_json::to_string(&before.root).expect("serialize semantic UI test tree").contains("lodJson"));
}

#[semio_framework_async_macros::async_test]
async fn app_definition_declares_reorganize_and_history_actions() {
    let definition = create_rewriting_app();
    let action_ids: Vec<&str> = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).map(|action| action.id.as_str()).collect();
    assert!(action_ids.contains(&"undo"));
    assert!(action_ids.contains(&"reorganize"));
}

#[semio_framework_async_macros::async_test]
async fn trinity_rewriting_labels_resolve_native_by_default() {
    let mut app = new_app().await;
    let json = serde_json::to_string(&app.render(TRINITY_REWRITING_PLAY_BODY_DOCUMENT, None, &ViewModel::default()).await.expect("render").root).expect("serialize semantic UI test tree");
    assert!(json.contains("\"Pieces\""));
    assert!(!json.contains("Stücke"));
}

#[semio_framework_async_macros::async_test]
async fn trinity_rewriting_labels_translate_panels_in_german() {
    let mut app = new_app().await;
    let view = ViewModel { locale: Locale::De, terminology: Terminology::Native, ..ViewModel::default() };
    let document_json = serde_json::to_string(&app.render(TRINITY_REWRITING_PLAY_BODY_DOCUMENT, None, &view).await.expect("render").root).expect("serialize semantic UI test tree");
    assert!(document_json.contains("Stücke"));
    assert!(!document_json.contains("\"Pieces\""));
    let catalogue_json = serde_json::to_string(&app.render(TRINITY_REWRITING_PLAY_BODY_CATALOGUE, None, &view).await.expect("render").root).expect("serialize semantic UI test tree");
    assert!(catalogue_json.contains("Katalog"));
    assert!(catalogue_json.contains("Zu LHS hinzufügen"));
    assert!(catalogue_json.contains("Zu RHS hinzufügen"));
    let parameters_json = serde_json::to_string(&app.render(TRINITY_REWRITING_PLAY_BODY_PARAMETERS, None, &view).await.expect("render").root).expect("serialize semantic UI test tree");
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
    assert_eq!(app.snapshot().unwrap().lhs_json, next_lhs);
    app.handle_action("undo", None, &meta("local")).await.expect("undo");
    assert_eq!(app.snapshot().unwrap().lhs_json, original);
    app.handle_action("redo", None, &meta("local")).await.expect("redo");
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
    assert_eq!(io.document_schema, REWRITE_RULE_SCHEMA);
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
    let actual = serde_json::json!({ "documentId": history.doc_id, "schema": history.schema, "edits": history.edits.len(), "changes": history.changes.len(), "checkpoints": history.checkpoints.len(), "alternatives": history.alternatives.len(), "conflicts": history.conflicts.len() });
    assert_eq!(actual, expected);
    println!("[DEBUG] rewriting reset preserves its source and pack and emits neutral edit-free history without an envelope owner");
}
