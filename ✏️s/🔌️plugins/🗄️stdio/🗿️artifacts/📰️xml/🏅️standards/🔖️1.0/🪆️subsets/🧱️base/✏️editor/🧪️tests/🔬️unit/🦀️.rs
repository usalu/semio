use super::*;

#[test]
fn set_node_requires_a_complete_address_and_allows_an_explicit_empty_text_node() {
    assert!(xml_any_command_from_action(XML_ANY_KIT_ACTION_ID, None).is_err());
    let missing_value = semio_framework_value::DslValue::object([("nodeId".into(), semio_framework_value::DslValue::String("0".into()))]);
    assert!(xml_any_command_from_action(XML_ANY_KIT_ACTION_ID, Some(&missing_value)).is_err());
    let args = semio_framework_value::DslValue::object([("nodeId".into(), semio_framework_value::DslValue::String("0".into())), ("revision".into(), semio_framework_value::DslValue::String("revision".into())), ("value".into(), semio_framework_value::DslValue::String(String::new()))]);
    assert!(matches!(xml_any_command_from_action(XML_ANY_KIT_ACTION_ID, Some(&args)), Ok(XmlAnyEditorCommand::SetNode { value, .. }) if value.is_empty()));
}

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

#[test]
fn natural_file_route_exports_xml_and_reopens_the_same_document() {
    use quick_xml::events::Event;
    let source = br#"<?xml version="1.0"?><document><title>Natural Open Save</title><body>Edited</body></document>"#;
    let edited = XmlSnapshot::import_utf8(source).expect("XML fixture");
    let bytes = <XmlAnyEditor as ArtifactEditor>::encode_natural_file(&edited).expect("XML natural bytes");
    let mut reader = quick_xml::reader::Reader::from_reader(bytes.as_slice());
    let mut text = Vec::new();
    loop {
        match reader.read_event().expect("quick-xml reads exported XML") {
            Event::Text(value) => text.push(value.xml10_content().expect("quick-xml decodes exported text").into_owned()),
            Event::Eof => break,
            _ => {}
        }
    }
    assert!(text.iter().any(|value| value == "Natural Open Save"));
    let reopened = <XmlAnyEditor as ArtifactEditor>::decode_natural_file(&bytes).expect("XML natural bytes reopen");
    assert_eq!(reopened, edited);
}

#[semio_framework_async_macros::async_test]
async fn decode_node_id_roundtrips_root_and_nested() {
    assert_eq!(decode_node_id(main::XML_ROOT_NODE_ID).unwrap(), Vec::<usize>::new());
    assert_eq!(decode_node_id(&main::encode_node_path(&[0, 2])).unwrap(), vec![0, 2]);
    assert!(decode_node_id("bad").is_err());
}

#[test]
fn set_node_rejects_every_noncanonical_address_before_emitting() {
    let snapshot = XmlSnapshot::default();
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
    let separator = semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR;
    for node_id in [String::new(), "0".into(), format!("${separator}{separator}0"), format!("${separator}${separator}0")] {
        let command = XmlAnyEditorCommand::SetNode { node_id, revision: revision.clone(), value: "text".into() };
        assert!(xml_any_emit(&command, &snapshot, None).is_err());
    }
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = XmlAnyEditorCommand::SetNode { node_id: main::encode_node_path(&[0, 2]), revision: "revision %20 Grüße 🌍".into(), value: "hello world %20 Grüße 🌍".into() };
    let printed = <XmlAnyEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <XmlAnyEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
    assert!(<XmlAnyEditorCommand as protocol::OpText>::parse_op("set-node node-id=00 revision=€0 value=00").is_err());
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
    let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<XmlAnyEditor>>::with_registry(EditorApp::<XmlAnyEditor>::default(), registry, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    while !app.close_terminal_is_empty() {
        if matches!(app.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("registered fixture close"), semio_framework_plugin::PluginCloseStep::Complete) {
            break;
        }
    }
    assert!(app.close_terminal_is_empty(), "registered fixture reaches its exact terminal-empty witness");
}

/// ⚖️ LAW: the natural-source draft applies what it shows. The curated catalog's rendered source (declaration, DOCTYPE,
/// comment, processing instruction, escaped text, CDATA) is a no-op through the root `set-node` when unchanged, and a text
/// change is exactly the net text leaf whose document quick-xml reads back with the edited text. (S18's served
/// matrix saw every source apply refused on a build whose demo asset no longer decoded — the editor had opened an empty
/// document.)
#[test]
fn the_natural_source_draft_applies_what_it_shows() {
    use quick_xml::events::Event;
    let snapshot = crate::schema::demo_xml_snapshot();
    let source = crate::standards::v1_0::subsets::base::io::text::snapshot::xml_document_to_text_checked(&snapshot.doc).expect("the curated catalog prints");
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
    let apply = |value: String| xml_any_emit(&XmlAnyEditorCommand::SetNode { node_id: main::XML_ROOT_NODE_ID.into(), revision: revision.clone(), value }, &snapshot, None);
    assert!(apply(source.clone()).expect("the unchanged source applies").artifact_mutations.is_empty(), "the unchanged source is a no-op");
    let edited = source.replacen("Tom", "Tim", 1);
    assert_ne!(edited, source, "the curated catalog carries the text the law edits");
    let emit = apply(edited).expect("a text change applies");
    assert!(matches!(emit.artifact_mutations.as_slice(), [XmlMutation::SetText(_)]), "a source text change is one net set-text leaf: {:?}", emit.artifact_mutations);
    let mut applied = snapshot.clone();
    for leaf in &emit.artifact_mutations {
        crate::schema::mutations::apply_xml_mutation(&mut applied, leaf);
    }
    let printed = crate::standards::v1_0::subsets::base::io::text::snapshot::xml_document_to_text_checked(&applied.doc).expect("the applied document prints");
    let mut reader = quick_xml::reader::Reader::from_str(&printed);
    let (mut inside, mut item) = (false, String::new());
    loop {
        match reader.read_event().expect("quick-xml reads the applied document") {
            Event::Start(start) if start.local_name().as_ref() == b"item" => inside = true,
            Event::End(end) if end.local_name().as_ref() == b"item" => inside = false,
            Event::Text(text) if inside => item.push_str(std::str::from_utf8(&text).expect("utf-8 text")),
            Event::GeneralRef(reference) if inside => item.push_str(quick_xml::escape::resolve_xml_entity(std::str::from_utf8(&reference).expect("utf-8 reference")).expect("a predefined entity")),
            Event::Eof => break,
            _ => {}
        }
    }
    assert_eq!(item, "Tim & Jerry", "quick-xml reads the edited text node in the applied document");
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
        let args = semio_framework_value::DslValue::object([(key.to_string(), semio_framework_value::DslValue::String("demo".into()))]);
        assert_eq!(xml_any_command_from_action(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, Some(&args)).expect("declared verb"), XmlAnyEditorCommand::SetActiveExample { example_id: "demo".into() });
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
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<XmlAnyEditor>, _>(async { semio_framework_plugin::App { definition: create_xml_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(document, STDIO_XML_DOCUMENT_SCHEMA) else { panic!("the example switch hands the host one whole document") };
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
    let mut app = kit_fixture_holding(&xml_any_example_snapshot(crate::examples::demo::ID)).await;
    let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(app.test_document_revision());
    dispatch_settled(&mut app, "set-node", &[("nodeId", &main::encode_node_path(&[2, 0])), ("revision", &revision), ("value", "Ada")]).await.expect("set-node settles");
    let after = app.snapshot().expect("xml snapshot");
    let root = after.doc.root.as_ref().expect("the example keeps its root element");
    assert_eq!(resolve_node(root, &[2, 0]), Some(&XmlNode::Text { text: "Ada".into() }));
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🪟️KitVerbLaws

semio_framework_plugin::history_edit_acceptance_law!("stdio", XmlAnyEditor, || semio_framework_plugin::App { definition: create_xml_editor(), examples: Vec::new() }, "../..");
