use super::*;

#[semio_framework_async_macros::async_test]
async fn create_xml_editor_builds_a_definition_for_the_editor_role() {
    let def = create_xml_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, XML_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<XmlAnyEditor as ArtifactEditor>::DIALECT, XML_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_tree_window() {
    let def = create_xml_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn decode_node_id_roundtrips_root_and_nested() {
    assert_eq!(decode_node_id("").unwrap(), Vec::<usize>::new());
    assert_eq!(decode_node_id("0/2").unwrap(), vec![0, 2]);
    assert!(decode_node_id("bad").is_err());
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = XmlAnyEditorCommand::SetNode { node_id: "0/2".into(), value: "hello world".into() };
    let printed = <XmlAnyEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <XmlAnyEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
}

//#region 🎬️ExampleSwitchLaws
/// ⚖️ LAW: the example picker's verb is declared on the app itself, which is what
/// `appSwitchesExamples` (`🛠️ShellHelpers/🟦️.tsx`) reads before it offers the picker or announces a
/// boot example.
#[semio_framework_async_macros::async_test]
async fn the_editor_declares_the_example_pickers_verb() {
    let definition = create_xml_editor();
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
    let registry = semio_framework_plugin::AppActionRegistry::from_definition(&create_xml_editor());
    let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<XmlAnyEditor>>::with_registry(EditorApp::<XmlAnyEditor>::default(), registry).await;
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
    let example = xml_any_example_snapshot(crate::examples::demo::ID);
    assert_ne!(example, XmlSnapshot::default());
    assert_eq!(xml_any_example_snapshot(""), XmlSnapshot::default());
    assert_eq!(xml_any_example_snapshot("no-such-example"), XmlSnapshot::default());
}

/// ⚖️ LAW: the shells' `{action, args}` pair resolves into the typed command, for every spelling of
/// the id the navbar, the palette and a replayed shell command use.
#[semio_framework_async_macros::async_test]
async fn the_shell_action_pair_resolves_into_the_typed_command() {
    for key in ["exampleId", "example_id", "id", "value"] {
        let args = dsl::DslValue::object([(key.to_string(), dsl::DslValue::String("demo".into()))]);
        assert_eq!(
            xml_any_command_from_action(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, Some(&args)).expect("declared verb"),
            XmlAnyEditorCommand::SetActiveExample { example_id: "demo".into() }
        );
    }
    assert!(xml_any_command_from_action("noSuchVerb", None).is_err());
}
//#endregion 🎬️ExampleSwitchLaws

//#region 🪟️KitVerbLaws
type KitFixtureApp = semio_framework_plugin::VcsArtifactApp<EditorApp<XmlAnyEditor>>;

/// 🧪️ One registered fixture app holding `document`, loaded exactly as a host applies the example
/// switch's `Effect::LoadDocument`.
async fn kit_fixture_holding(document: &XmlSnapshot) -> KitFixtureApp {
    use semio_framework_plugin::PluginApp;
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<XmlAnyEditor>, _>(async { semio_framework_plugin::App { definition: create_xml_editor(), examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(document, STDIO_XML_DOCUMENT_SCHEMA) else {
        panic!("the example switch hands the host one whole document")
    };
    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.expect("the host loads the example document");
    app
}

/// 🕹️ Dispatches `action` with text-staged `args`, exactly as a rail sends it, and settles it through
/// the host's bounded publication loop.
async fn dispatch_settled(app: &mut KitFixtureApp, action: &str, args: &[(&str, &str)]) -> Result<(), Fault> {
    use semio_framework_plugin::PluginApp;
    let meta = semio_framework_plugin::artifact_app_laws::meta("local");
    let args = dsl::DslValue::object(args.iter().map(|(key, value)| ((*key).to_string(), dsl::DslValue::String((*value).to_string()))).collect::<Vec<_>>());
    app.handle_action(action, Some(&args), &meta).await?;
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, meta.instance_id).await.map(|_| ())
}

/// ⚖️ LAW: `set-node` — the verb the `TreeWindowKit` mints for `🪟️main` — reaches the document through this
/// editor's exact retained factory. Unregistered, the reactor refused it inside `s` with
/// `interactive-job.missing-factory` (S15, session 11).
#[semio_framework_async_macros::async_test]
async fn the_kit_verb_edits_the_document_through_its_exact_retained_factory() {
    let mut app = kit_fixture_holding(&xml_any_example_snapshot(crate::examples::demo::ID)).await;
    dispatch_settled(&mut app, "set-node", &[("nodeId", &main::encode_node_path(&[2, 0])), ("value", "Ada")]).await.expect("set-node settles");
    let after = app.snapshot().expect("xml snapshot");
    let root = after.doc.root.as_ref().expect("the example keeps its root element");
    assert_eq!(resolve_node(root, &[2, 0]), Some(&XmlNode::Text { text: "Ada".into() }));
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🪟️KitVerbLaws
