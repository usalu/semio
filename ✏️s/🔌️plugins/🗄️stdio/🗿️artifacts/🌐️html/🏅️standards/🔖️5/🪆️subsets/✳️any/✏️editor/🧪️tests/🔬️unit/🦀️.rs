use super::*;
use crate::standards::v5::subsets::any::schema::snapshot::{HtmlAttr, HtmlNode};

#[test]
fn text_edit_requires_an_explicit_text_value_and_allows_empty_documents() {
    assert!(html_command_from_action(HTML_KIT_ACTION_ID, None).is_err());
    let args = semio_framework_value::DslValue::object([("text".into(), semio_framework_value::DslValue::String(String::new()))]);
    assert_eq!(html_command_from_action(HTML_KIT_ACTION_ID, Some(&args)).expect("explicit empty text"), HtmlEditCommand::LoadText { text: String::new() });
}

#[test]
fn natural_file_route_exports_html5_and_reopens_the_same_document() {
    let source = b"<!doctype html><html><head><title>Natural Open Save</title></head><body><p>Edited body.</p></body></html>";
    let edited = crate::standards::v5::subsets::any::io::text::snapshot::parse_html_document(std::str::from_utf8(source).unwrap()).expect("HTML fixture");
    let bytes = <HtmlEditor as ArtifactEditor>::encode_natural_file(&edited).expect("HTML natural bytes");
    let oracle = semio_s_artifact_stdio_html_test_oracle::standards::v5::subsets::any::project_html_5;
    assert_eq!(oracle(&bytes).expect("html5ever reads exported HTML"), oracle(source).expect("html5ever reads expected HTML"));
    let reopened = <HtmlEditor as ArtifactEditor>::decode_natural_file(&bytes).expect("HTML natural bytes reopen");
    assert_eq!(reopened, edited);
}

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_html_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, HTML_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<HtmlEditor as ArtifactEditor>::DIALECT, HTML_DIALECT);
}

/// 🎯️ LAW: the editor declares the artifact kind it edits (the artifact's own `artifact_kind()`), which is
/// what the hub's one open-target rule (`app_opens_kind`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`)
/// pairs with this editor and the viewer of its dialect — so an HTML document can be created and opened as a hub document.
#[semio_framework_async_macros::async_test]
async fn the_editor_declares_the_artifact_kind_it_edits() {
    assert_eq!(create_html_editor().artifact_kinds, vec![crate::artifact_kind()]);
}

//#region 🎬️ExampleSwitchLaws
/// ⚖️ LAW: the example picker's verb is declared on the app itself, which is what
/// `appSwitchesExamples` (`🛠️ShellHelpers/🟦️.tsx`) reads before it offers the picker or announces a
/// boot example.
#[semio_framework_async_macros::async_test]
async fn the_editor_declares_the_example_pickers_verb() {
    let definition = create_html_editor();
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
    let registry = semio_framework_plugin::AppActionRegistry::from_definition(&create_html_editor());
    let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<HtmlEditor>>::with_registry(EditorApp::<HtmlEditor>::default(), registry, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
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
    let example = html_example_snapshot(crate::examples::demo::ID);
    assert_ne!(example, HtmlSnapshot::default());
    assert_eq!(html_example_snapshot(""), HtmlSnapshot::default());
    assert_eq!(html_example_snapshot("no-such-example"), HtmlSnapshot::default());
}

/// ⚖️ LAW: the shells' `{action, args}` pair resolves into the typed command, for every spelling of
/// the id the navbar, the palette and a replayed shell command use.
#[semio_framework_async_macros::async_test]
async fn the_shell_action_pair_resolves_into_the_typed_command() {
    for key in ["exampleId", "example_id", "id", "value"] {
        let args = semio_framework_value::DslValue::object([(key.to_string(), semio_framework_value::DslValue::String("demo".into()))]);
        assert_eq!(html_command_from_action(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, Some(&args)).expect("declared verb"), HtmlEditCommand::SetActiveExample { example_id: "demo".into() });
    }
    assert!(html_command_from_action("noSuchVerb", None).is_err());
}
//#endregion 🎬️ExampleSwitchLaws

//#region 🪟️KitVerbLaws
type KitFixtureApp = semio_framework_plugin::VcsArtifactApp<EditorApp<HtmlEditor>>;

/// 🧪️ One registered fixture app holding `document`, loaded exactly as a host applies the example
/// switch's `Effect::LoadDocument`.
async fn kit_fixture_holding(document: &HtmlSnapshot) -> KitFixtureApp {
    use semio_framework_plugin::PluginApp;
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<HtmlEditor>, _>(async { semio_framework_plugin::App { definition: create_html_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(document, STDIO_HTML_DOCUMENT_SCHEMA) else { panic!("the example switch hands the host one whole document") };
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

/// ⚖️ LAW: `textEdit` — the verb the `TextWindowKit` mints for `🪟️main` — reaches the host through this
/// editor's exact retained factory and loads the parsed document. Unregistered, the reactor refused it inside `s` with
/// `interactive-job.missing-factory` (S15, session 11).
#[semio_framework_async_macros::async_test]
async fn the_kit_verb_loads_the_document_through_its_exact_retained_factory() {
    let target = html_example_snapshot(crate::examples::demo::ID);
    let mut app = kit_fixture_holding(&HtmlSnapshot::default()).await;
    dispatch_settled(&mut app, "textEdit", &[("text", &<HtmlSnapshot as store::ArtifactDsl>::print_dsl(&target))]).await.expect("textEdit settles");
    assert_eq!(app.snapshot().expect("html snapshot"), target);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}

/// ⚖️ LAW: `textEdit` with text that is not this artifact's DSL is refused loudly with
/// `stdio.html.invalid-text` and leaves the document untouched.
#[semio_framework_async_macros::async_test]
async fn text_edit_refuses_text_that_is_not_the_artifacts_dsl() {
    let example = html_example_snapshot(crate::examples::demo::ID);
    let mut app = kit_fixture_holding(&example).await;
    let fault = dispatch_settled(&mut app, "textEdit", &[("text", "not a dsl document")]).await.expect_err("invalid text is refused");
    assert!(format!("{fault:?}").contains("stdio.html.invalid-text"), "{fault:?}");
    assert_eq!(app.snapshot().expect("html snapshot"), example);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}

/// ⚖️ LAW: the window's DSL envelope is a whole-document dump, so an Apply is a load and publishes no mutation row.
#[test]
fn an_applied_envelope_is_a_load_and_publishes_no_mutation() {
    let next = html_example_snapshot(crate::examples::demo::ID);
    let emit = html_emit(&HtmlEditCommand::LoadText { text: <HtmlSnapshot as store::ArtifactDsl>::print_dsl(&next) }).expect("the envelope parses");
    assert!(emit.artifact_mutations.is_empty());
    assert!(matches!(emit.effects.as_slice(), [semio_framework_plugin::Effect::LoadDocument { .. }]));
}
//#endregion 🪟️KitVerbLaws

//#region 🪟️DetailsLaws
/// ⚖️ LAW: replacing the whole source through the details pane is a document load, not an edit, so no mutation is published.
#[test]
fn replacing_the_whole_source_is_refused_as_a_load() {
    use semio_s_artifact_stdio_contract::editing::{snapshot_edit_source, SnapshotEditEvent, SnapshotEditingEditor};
    let event = SnapshotEditEvent::ReplaceSource { source: snapshot_edit_source(&html_example_snapshot(crate::examples::demo::ID)) };
    assert_eq!(<HtmlEditor as SnapshotEditingEditor>::snapshot_edit_emit(&event, &HtmlSnapshot::default()).expect_err("a load, not an edit").code.0, "snapshot-edit.unsupported-path");
}
//#endregion 🪟️DetailsLaws
semio_framework_plugin::history_edit_acceptance_law!("stdio", HtmlEditor, || semio_framework_plugin::App { definition: create_html_editor(), examples: Vec::new() }, "../..");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_node() {
    use semio_s_artifact_stdio_contract::editing::{SnapshotEditEvent, SnapshotEditingEditor};
    let base = HtmlSnapshot { root: HtmlNode::Element { name: "div".into(), attributes: vec![HtmlAttr::new("class", "a")], children: vec![HtmlNode::Text { text: "hi".into() }, HtmlNode::Comment { text: "c".into() }] }, ..HtmlSnapshot::default() };
    let emit = |event: SnapshotEditEvent| <HtmlEditor as SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let text = emit(SnapshotEditEvent::SetValue { path: "/root/children/0/text".into(), value: semio_framework_value::DslValue::String("bye".into()) }).expect("a text edit resolves");
    assert!(matches!(text.artifact_mutations.as_slice(), [HtmlMutation::SetText(_)]));
    let attribute = emit(SnapshotEditEvent::SetValue { path: "/root/attributes/0/value".into(), value: semio_framework_value::DslValue::String("b".into()) }).expect("an attribute edit resolves");
    assert!(matches!(attribute.artifact_mutations.as_slice(), [HtmlMutation::SetAttribute(_)]));
    let renamed = emit(SnapshotEditEvent::SetValue { path: "/root/name".into(), value: semio_framework_value::DslValue::String("section".into()) }).expect("an element rename resolves");
    assert!(matches!(renamed.artifact_mutations.as_slice(), [HtmlMutation::SetElementName(_)]));
    let removed = emit(SnapshotEditEvent::RemoveValue { path: "/root/children/1".into() }).expect("a node removal resolves");
    assert!(matches!(removed.artifact_mutations.as_slice(), [HtmlMutation::RemoveNode(_)]));
    assert_eq!(emit(SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).expect_err("no kind").code.0, "snapshot-edit.unsupported-path");
}
