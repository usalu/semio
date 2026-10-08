use super::*;
use crate::schema::snapshot::JsonValue;

/// 🧬️ Registers the document schema json's declaration contributes (`.schema(json_artifact_schema_descriptor())`) — the
/// registered contract every snapshot edit validates against; a fixture editor runs without the plugin assembly that publishes it.
fn register_document_schema() {
    semio_framework_schema_registry::register_artifact_schema_descriptors(vec![crate::schema::json_artifact_schema_descriptor()]).expect("the json document schema registers");
}

#[test]
fn set_node_requires_a_complete_address_and_value_without_forbidding_empty_text() {
    assert!(json_any_command_from_action(JSON_ANY_KIT_ACTION_ID, None).is_err());
    let missing_value = semio_framework_value::DslValue::object([("nodeId".into(), semio_framework_value::DslValue::String(main::JSON_ROOT_NODE_ID.into()))]);
    assert!(json_any_command_from_action(JSON_ANY_KIT_ACTION_ID, Some(&missing_value)).is_err());
    let args = semio_framework_value::DslValue::object([("nodeId".into(), semio_framework_value::DslValue::String(main::JSON_ROOT_NODE_ID.into())), ("revision".into(), semio_framework_value::DslValue::String("revision".into())), ("value".into(), semio_framework_value::DslValue::String(String::new()))]);
    assert!(matches!(json_any_command_from_action(JSON_ANY_KIT_ACTION_ID, Some(&args)), Ok(JsonAnyEditorCommand::SetNode { value, .. }) if value.is_empty()));
}

#[semio_framework_async_macros::async_test]
async fn create_json_editor_builds_a_definition_for_the_editor_role() {
    let def = create_json_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, JSON_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<JsonAnyEditor as ArtifactEditor>::DIALECT, JSON_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_tree_window() {
    let def = create_json_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[test]
fn natural_file_route_exports_json_and_reopens_the_same_document() {
    let source = r#"{"title":"Natural Open Save","edited":true,"count":3}"#;
    let edited = JsonSnapshot::from_value(crate::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text(source).expect("JSON fixture"));
    let bytes = <JsonAnyEditor as ArtifactEditor>::encode_natural_file(&edited).expect("JSON natural bytes");
    let independent: serde_json::Value = serde_json::from_slice(&bytes).expect("serde_json reads exported JSON");
    assert_eq!(independent["title"], "Natural Open Save");
    let reopened = <JsonAnyEditor as ArtifactEditor>::decode_natural_file(&bytes).expect("JSON natural bytes reopen");
    assert_eq!(reopened, edited);
}

#[semio_framework_async_macros::async_test]
async fn decode_path_id_roundtrips_root_and_nested() {
    assert!(decode_path_id("").is_err());
    assert_eq!(decode_path_id(main::JSON_ROOT_NODE_ID).unwrap(), "/value");
    let nested = main::encode_path_id(&[main::member_segment(2), "i=0".into()]);
    assert_eq!(decode_path_id(&nested).unwrap(), "/value/members/2/value/items/0");
    assert!(decode_path_id(&main::encode_path_id(&["m=02".into()])).is_err());
    assert!(decode_path_id("bad").is_err());
}

#[test]
fn set_node_rejects_every_noncanonical_address_before_emitting() {
    let snapshot = JsonSnapshot::default();
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
    let separator = semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR;
    for node_id in [String::new(), "m=0".into(), format!("${separator}{separator}m=0"), format!("${separator}${separator}m=0")] {
        let command = JsonAnyEditorCommand::SetNode { node_id, revision: revision.clone(), value: "null".into() };
        assert!(json_any_emit(&command, &snapshot, None).is_err());
    }
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = JsonAnyEditorCommand::SetNode { node_id: main::encode_path_id(&[main::member_segment(2), "i=0".into()]), revision: "revision %20 Grüße 🌍".into(), value: "\"literal %20\\nGrüße 🌍\"".into() };
    let printed = <JsonAnyEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <JsonAnyEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);

    let source = JsonAnyEditorCommand::EditSnapshot { event: SnapshotEditEvent::ReplaceSource { source: "{\"literal\":\"%20\\nGrüße 🌍\"}".into() } };
    let printed = <JsonAnyEditorCommand as protocol::OpText>::print_op(&source);
    let parsed = <JsonAnyEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse snapshot edit");
    assert_eq!(parsed, source);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_all_typed_snapshot_edit_actions() {
    let definition = create_json_editor();
    for action_id in semio_s_artifact_stdio_contract::editing::SNAPSHOT_EDIT_ACTION_IDS {
        let action = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == *action_id).expect("typed snapshot edit action (window-owned actions live on their window kind)");
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    }
}

#[semio_framework_async_macros::async_test]
async fn set_node_preserves_every_json_value_kind_and_rejects_invalid_source() {
    register_document_schema();
    for source in ["null", "true", "-123.4500e+9", "\"text %20\\n日本語\"", "[1,false,null]", "{\"answer\":42}"] {
        let snapshot = JsonSnapshot { schema: JsonSnapshot::default().schema, value: JsonValue::String { value: "before".into() } };
        let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
        let command = JsonAnyEditorCommand::SetNode { node_id: main::JSON_ROOT_NODE_ID.into(), revision, value: source.into() };
        let emit = json_any_emit(&command, &snapshot, None).expect("valid JSON value");
        let next = protocol::apply_diff(<JsonMutation as protocol::Mutation<JsonSnapshot>>::diff(&emit.artifact_mutations[0], &snapshot).diff(), &snapshot).expect("compact node patch applies");
        let native = crate::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text(source).expect("JSON file parser");
        let expected = serde_json::from_str::<serde_json::Value>(source).expect("serde_json oracle");
        assert_eq!(next.value, native);
        assert_eq!(serde_json::Value::from(&next.value), expected);
    }
    let snapshot = JsonSnapshot::default();
    let invalid = JsonAnyEditorCommand::SetNode { node_id: main::JSON_ROOT_NODE_ID.into(), revision: semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot), value: "{invalid".into() };
    assert!(json_any_emit(&invalid, &snapshot, None).is_err());
}

//#region 🎬️ExampleSwitchLaws
/// ⚖️ LAW: the example picker's verb is declared on the app itself, which is what
/// `appSwitchesExamples` (`🛠️ShellHelpers/🟦️.tsx`) reads before it offers the picker or announces a
/// boot example.
#[semio_framework_async_macros::async_test]
async fn the_editor_declares_the_example_pickers_verb() {
    let definition = create_json_editor();
    let action = definition.actions.iter().find(|action| action.id == semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID).expect("declared example verb");
    assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    assert!(action.semantics.effects.destructive);
    assert!(action.args.iter().any(|arg| arg.id == "exampleId"));
}

/// ⚖️ LAW: the example switch's bounded-first-step proof joins its exact registered factory, and the
/// controller that owns it closes through its whole retained close protocol. A registry-backed
/// controller PANICS at construction (`interactive-job.catalog-authority` /
/// `interactive-job.catalog-incomplete`) whenever the retained roster, the publication contract, the
/// `Migrated` classification and `OpBinary::TOOL_JOB_IDS` disagree — so constructing one is the proof.
#[semio_framework_async_macros::async_test]
async fn the_example_switch_joins_its_exact_retained_factory() {
    use semio_framework_plugin::PluginApp;
    let registry = semio_framework_plugin::AppActionRegistry::from_definition(&create_json_editor());
    let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<JsonAnyEditor>>::with_registry(EditorApp::<JsonAnyEditor>::default(), registry, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    while !app.close_terminal_is_empty() {
        if matches!(app.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("registered fixture close"), semio_framework_plugin::PluginCloseStep::Complete) {
            break;
        }
    }
    assert!(app.close_terminal_is_empty(), "registered fixture reaches its exact terminal-empty witness");
}

/// ⚖️ LAW: the curated example parses into a document that differs from the genesis one, and an
/// unknown or empty id falls back to the genesis document.
#[semio_framework_async_macros::async_test]
async fn the_curated_example_carries_visible_content() {
    let example = json_any_example_snapshot(crate::examples::demo::ID);
    assert_ne!(example, JsonSnapshot::default());
    assert_eq!(json_any_example_snapshot(""), JsonSnapshot::default());
    assert_eq!(json_any_example_snapshot("no-such-example"), JsonSnapshot::default());
}

/// ⚖️ LAW: the shells' `{action, args}` pair resolves into the typed command, for every spelling of
/// the id the navbar, the palette and a replayed shell command use.
#[semio_framework_async_macros::async_test]
async fn the_shell_action_pair_resolves_into_the_typed_command() {
    for key in ["exampleId", "example_id", "id", "value"] {
        let args = semio_framework_value::DslValue::object([(key.to_string(), semio_framework_value::DslValue::String("demo".into()))]);
        assert_eq!(json_any_command_from_action(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, Some(&args)).expect("declared verb"), JsonAnyEditorCommand::SetActiveExample { example_id: "demo".into() });
    }
    assert!(json_any_command_from_action("noSuchVerb", None).is_err());
}
//#endregion 🎬️ExampleSwitchLaws

//#region 🪟️KitVerbLaws
type KitFixtureApp = semio_framework_plugin::VcsArtifactApp<EditorApp<JsonAnyEditor>>;

/// 🧪️ One registered fixture app holding `document`, loaded exactly as a host applies the example
/// switch's `Effect::LoadDocument`.
async fn kit_fixture_holding(document: &JsonSnapshot) -> KitFixtureApp {
    use semio_framework_plugin::PluginApp;
    register_document_schema();
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<JsonAnyEditor>, _>(async { semio_framework_plugin::App { definition: create_json_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(document, STDIO_JSON_DOCUMENT_SCHEMA) else { panic!("the example switch hands the host one whole document") };
    semio_framework_plugin::artifact_app_laws::load_document(&mut app, &store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.expect("the host loads the example document");
    app
}

/// 🕹️ Dispatches `action` with text-staged `args`, exactly as a rail sends it, and settles it through
/// the host's bounded publication loop.
async fn dispatch_settled(app: &mut KitFixtureApp, action: &str, args: &[(&str, &str)]) -> Result<(), Fault> {
    use semio_framework_plugin::PluginApp;
    let meta = semio_framework_plugin::artifact_app_laws::meta("local");
    let args = semio_framework_value::DslValue::object(args.iter().map(|(key, value)| ((*key).to_string(), semio_framework_value::DslValue::String((*value).to_string()))).collect::<Vec<_>>());
    app.handle_action(action, Some(&args), &meta).await?;
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, meta.instance_id).await.map(|_| ())
}

/// ⚖️ LAW: `set-node` — the verb the `TreeWindowKit` mints for `🪟️main` — reaches the document through this
/// editor's exact retained factory. Unregistered, the reactor refused it inside `s` with
/// `interactive-job.missing-factory` (S15, session 11).
#[semio_framework_async_macros::async_test]
async fn the_kit_verb_edits_the_document_through_its_exact_retained_factory() {
    let mut app = kit_fixture_holding(&json_any_example_snapshot(crate::examples::demo::ID)).await;
    let snapshot = app.snapshot().expect("json snapshot");
    let JsonValue::Object { members } = &snapshot.value else { panic!("demo object") };
    let name_index = members.iter().position(|member| member.key == "name").expect("name member");
    let node_id = main::encode_path_id(&[main::member_segment(name_index)]);
    let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(app.test_document_revision());
    dispatch_settled(&mut app, "set-node", &[("nodeId", &node_id), ("revision", &revision), ("value", "\"semio-edited\"")]).await.expect("set-node settles");
    let printed = <JsonSnapshot as store::ArtifactDsl>::print_dsl(&app.snapshot().expect("json snapshot"));
    let restored=<JsonSnapshot as store::ArtifactDsl>::parse_dsl(&printed).expect("owned logical edit payload");assert_eq!(restored,app.snapshot().expect("json snapshot"));let JsonValue::Object{members}=&restored.value else{panic!("edited object")};assert_eq!(members[name_index].value,JsonValue::String{value:"semio-edited".into()});
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn source_edit_reaches_the_document_through_the_retained_event_route() {
    let mut app = kit_fixture_holding(&json_any_example_snapshot(crate::examples::demo::ID)).await;
    let source = "{\"kind\":\"typed\",\"count\":3,\"enabled\":true}";
    let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(app.test_document_revision());
    dispatch_settled(&mut app, "set-node", &[("nodeId", main::JSON_ROOT_NODE_ID), ("revision", &revision), ("value", source)]).await.expect("source edit settles");
    let after = app.snapshot().expect("json snapshot");
    assert_eq!(after, JsonSnapshot { schema:crate::STDIO_JSON_DOCUMENT_SCHEMA.into(), value:crate::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text(source).expect("expected source") });
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🪟️KitVerbLaws

semio_framework_plugin::history_edit_acceptance_law!("stdio", JsonAnyEditor, || semio_framework_plugin::App { definition: create_json_editor(), examples: Vec::new() }, "../..");
