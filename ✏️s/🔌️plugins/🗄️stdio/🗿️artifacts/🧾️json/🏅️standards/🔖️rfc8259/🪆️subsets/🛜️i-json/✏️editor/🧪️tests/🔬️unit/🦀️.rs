use super::*;

#[semio_framework_async_macros::async_test]
async fn create_json_i_json_editor_builds_a_definition_for_the_editor_role() {
    let def = create_json_i_json_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, JSON_I_JSON_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<JsonIJsonEditor as ArtifactEditor>::DIALECT, JSON_I_JSON_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_tree_window() {
    let def = create_json_i_json_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn decode_path_id_roundtrips_root_and_nested() {
    assert_eq!(decode_path_id("").unwrap(), Vec::<JsonPathSegment>::new());
    assert_eq!(decode_path_id(main::JSON_ROOT_NODE_ID).unwrap(), Vec::<JsonPathSegment>::new());
    assert_eq!(decode_path_id("k=a/i=0").unwrap(), vec![JsonPathSegment::Key("a".into()), JsonPathSegment::Index(0)]);
    assert!(decode_path_id("bad").is_err());
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = JsonIJsonIJsonEditorCommand::SetNode { node_id: "k=a/i=0".into(), value: "hello world".into() };
    let printed = <JsonIJsonIJsonEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <JsonIJsonIJsonEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
}

//#region 🎬️ExampleSwitchLaws
/// ⚖️ LAW: the example picker's verb is declared on the app itself, which is what
/// `appSwitchesExamples` (`🛠️ShellHelpers/🟦️.tsx`) reads before it offers the picker or announces a
/// boot example.
#[semio_framework_async_macros::async_test]
async fn the_editor_declares_the_example_pickers_verb() {
    let definition = create_json_i_json_editor();
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
    let registry = semio_framework_plugin::AppActionRegistry::from_definition(&create_json_i_json_editor());
    let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<JsonIJsonEditor>>::with_registry(EditorApp::<JsonIJsonEditor>::default(), registry).await;
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
    let example = json_i_json_example_snapshot(crate::standards::v_rfc8259::subsets::i_json::examples::demo::ID);
    assert_ne!(example, JsonSnapshot::default());
    assert_eq!(json_i_json_example_snapshot(""), JsonSnapshot::default());
    assert_eq!(json_i_json_example_snapshot("no-such-example"), JsonSnapshot::default());
}

/// ⚖️ LAW: the shells' `{action, args}` pair resolves into the typed command, for every spelling of
/// the id the navbar, the palette and a replayed shell command use.
#[semio_framework_async_macros::async_test]
async fn the_shell_action_pair_resolves_into_the_typed_command() {
    for key in ["exampleId", "example_id", "id", "value"] {
        let args = dsl::DslValue::object([(key.to_string(), dsl::DslValue::String("demo".into()))]);
        assert_eq!(
            json_i_json_command_from_action(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, Some(&args)).expect("declared verb"),
            JsonIJsonIJsonEditorCommand::SetActiveExample { example_id: "demo".into() }
        );
    }
    assert!(json_i_json_command_from_action("noSuchVerb", None).is_err());
}
//#endregion 🎬️ExampleSwitchLaws
