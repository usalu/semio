use crate::standards::v5::subsets::any::schema::mutations::{splice_source, HtmlMutation};
use semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;
use crate::{HTML_DIALECT, STDIO_HTML_DOCUMENT_SCHEMA};
use semio_plugin::ArtifactEditor;
use semio_plugin::Editor;
use semio_plugin::EditorApp;
use semio_plugin::Fault;
use semio_plugin::InteractiveJobClassification;
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;
use crate::editor::html::{HTML_KIT_ACTION_ID, html_example_snapshot, html_command_from_action, html_emit};
use crate::editor::html::*;
use semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::schema::snapshot::{HtmlAttr, HtmlNode};

#[test]
fn text_edit_requires_an_explicit_text_value_and_allows_empty_documents() {
    assert!(html_command_from_action(HTML_KIT_ACTION_ID, None).is_err());
    let only_revision = semio_framework_value::DslValue::object([("revision".into(), semio_framework_value::DslValue::String("r".into()))]);
    assert!(html_command_from_action(HTML_KIT_ACTION_ID, Some(&only_revision)).is_err());
    let args = semio_framework_value::DslValue::object([("revision".into(), semio_framework_value::DslValue::String("r".into())), ("splices".into(), semio_framework_value::DslValue::String("[]".into()))]);
    assert_eq!(html_command_from_action(HTML_KIT_ACTION_ID, Some(&args)).expect("explicit empty change set"), HtmlEditCommand::SpliceText { revision: "r".into(), splices: "[]".into() });
}

#[test]
fn natural_file_route_exports_html5_and_reopens_the_same_document() {
    let source = b"<!doctype html><html><head><title>Natural Open Save</title></head><body><p>Edited body.</p></body></html>";
    let edited = semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::io::text::snapshot::parse_html_document(std::str::from_utf8(source).unwrap()).expect("HTML fixture");
    let bytes = <HtmlEditor as ArtifactEditor>::encode_natural_file(&edited).expect("HTML natural bytes");
    let oracle = semio_s_artifact_stdio_html_test_oracle::standards::v5::subsets::any::project_html_5;
    assert_eq!(oracle(&bytes).expect("html5ever reads exported HTML"), oracle(source).expect("html5ever reads expected HTML"));
    let reopened = <HtmlEditor as ArtifactEditor>::decode_natural_file(&bytes).expect("HTML natural bytes reopen");
    assert_eq!(reopened, edited);
}

#[test]
fn natural_file_import_refuses_invalid_utf8_and_keeps_padded_text_through_the_dsl() {
    assert!(<HtmlEditor as ArtifactEditor>::decode_natural_file(&[0x3c, 0x70, 0x3e, 0xff]).is_err());
    let source = "<!doctype html><html><head><title>t</title></head><body><p>  padded  </p></body></html>";
    let edited = semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::io::text::snapshot::parse_html_document(source).expect("HTML fixture");
    let printed = <HtmlSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&edited);
    assert_eq!(<HtmlSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&printed).expect("DSL reopens"), edited);
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
    use semio_plugin::PluginApp;
    let registry = semio_plugin::AppActionRegistry::from_definition(&create_html_editor());
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| semio_plugin::VcsArtifactApp::<EditorApp<HtmlEditor>>::with_registry(EditorApp::<HtmlEditor>::default(), registry, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await.expect("the reference snapshot births"));
    semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::editing::fixture_mounted_owner_policy(), scope.cx()).expect("original caller paid app close");
    assert!(app.close_terminal_is_empty(), "registered fixture reaches its exact terminal-empty witness");
}

/// ⚖️ LAW: the curated example parses into a document that differs from the genesis one, and an
/// unknown or empty id falls back to the genesis document.
#[semio_framework_async_macros::async_test]
async fn the_curated_example_carries_visible_content() {
    let example = html_example_snapshot(semio_s_artifact_stdio_html_inferences::examples::demo::ID);
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
type KitFixtureApp = semio_plugin::VcsArtifactApp<EditorApp<HtmlEditor>>;

/// 🧪️ One registered fixture app holding `document`, loaded exactly as a host applies the example
/// switch's `Effect::LoadDocument`.
async fn kit_fixture_holding(document: &HtmlSnapshot) -> (KitFixtureApp, semio_plugin::artifact_app_laws::OriginalFixtureScope) {
    use semio_plugin::PluginApp;
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::new_registered_app::<EditorApp<HtmlEditor>, _>(async { semio_plugin::App { definition: create_html_editor(), examples: Vec::new() } }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    let semio_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(document, STDIO_HTML_DOCUMENT_SCHEMA) else { panic!("the example switch hands the host one whole document") };
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::load_document(&mut app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut identity, scope.cx()).await).expect("the host loads the example document");
    (app, scope)
}

/// 🕹️ Dispatches `action` with text-staged `args`, exactly as a rail sends it, and settles it through
/// the host's bounded publication loop.
async fn dispatch_settled(app: &mut KitFixtureApp, action: &str, args: &[(&str, &str)], cx: &mut semio_plugin::StepContext<'_>) -> Result<(), Fault> {
    use semio_plugin::PluginApp;
    let meta = semio_plugin::artifact_app_laws::meta("local");
    let args = semio_framework_value::DslValue::object(args.iter().map(|(key, value)| ((*key).to_string(), semio_framework_value::DslValue::String((*value).to_string()))).collect::<Vec<_>>());
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut *app, action, Some(args.clone()), &meta, &mut identity, cx, semio_s_artifact_stdio_contract::editing::fixture_mounted_owner_policy().close).await).map_err(|failure| failure.primary_fault().cloned().unwrap_or_else(|| semio_plugin::Fault::from(format!("{failure:?}"))))?;
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::settle_registered_typed_operation(app, meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, cx).await).map(|_| ()).map_err(|failure| semio_plugin::Fault::from(format!("{failure:?}")))
}

/// 🧾️ The change set JSON the editor sends: `(offset, delete, insert)` ranges of Unicode scalars of the source it started from.
fn splices_json(ranges: &[(usize, usize, &str)]) -> String {
    serde_json::Value::Array(ranges.iter().map(|(offset, delete, insert)| serde_json::json!({ "offset": offset, "delete": delete, "insert": insert })).collect()).to_string()
}

fn source_of(snapshot: &HtmlSnapshot) -> String {
    semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::io::text::snapshot::write_html_document(snapshot)
}

fn parsed(source: &str) -> HtmlSnapshot {
    semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::io::text::snapshot::parse_html_document(source).expect("HTML fixture")
}

/// ⚖️ LAW: `textEdit` — the verb the `TextWindowKit` mints for `🪟️main` — reaches the document through this
/// editor's exact retained factory as ONE undoable `splice-source` edit; one undo restores the committed document.
#[semio_framework_async_macros::async_test]
async fn the_kit_verb_edits_the_document_through_its_exact_retained_factory_and_undoes() {
    use semio_plugin::PluginApp;
    let base = parsed("<!DOCTYPE html>\n<html><head><title>T</title></head><body><p id=\"a\">Hello</p></body></html>");
    let (mut app, mut scope) = kit_fixture_holding(&base).await;
    let edits = app.edit_transactions().len();
    let offset = source_of(&base).find("Hello").expect("the text");
    let changes = splices_json(&[(source_of(&base)[..offset].chars().count(), 5, "World")]);
    dispatch_settled(&mut app, "textEdit", &[("revision", &semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&base)), ("splices", &changes)], scope.cx()).await.expect("textEdit settles");
    assert_eq!(app.snapshot().expect("html snapshot"), parsed(&source_of(&base).replace("Hello", "World")));
    assert_eq!(app.edit_transactions().len(), edits + 1, "one Apply, one edit");
    let rows: Vec<_> = app.history_snapshot().await.expect("history").upserts.into_iter().filter(|row| row.edit_id.is_some()).collect();
    let row = rows.iter().max_by_key(|row| row.seq).expect("the applied change set's row");
    assert_eq!(row.mutations.len(), 1, "one change set is one leaf: {:?}", row.mutations);
    assert_eq!(row.mutations[0].label.resolve(semio_ui_locale::Terminology::Native, semio_ui_locale::Locale::En), "Edit source");
    assert_eq!(row.mutations[0].label.resolve(semio_ui_locale::Terminology::Native, semio_ui_locale::Locale::De), "Quelltext bearbeiten");
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::settle_history_verb(&mut app, "undo", semio_plugin::artifact_app_laws::meta("local").instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
    assert_eq!(app.snapshot().expect("html snapshot"), base, "one undo restores the committed document");
    semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("original caller paid app close");
}

/// ⚖️ LAW: a change set whose edited source is not well-formed HTML is refused loudly with `stdio.html.invalid-source`
/// and leaves the document untouched.
#[semio_framework_async_macros::async_test]
async fn text_edit_refuses_source_that_is_not_html_and_keeps_the_document() {
    let base = parsed("<!DOCTYPE html>\n<html><head></head><body><p>x</p></body></html>");
    let (mut app, mut scope) = kit_fixture_holding(&base).await;
    let offset = source_of(&base).find("</body>").expect("the body end");
    let changes = splices_json(&[(source_of(&base)[..offset].chars().count(), 0, "<div>")]);
    let fault = dispatch_settled(&mut app, "textEdit", &[("revision", &semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&base)), ("splices", &changes)], scope.cx()).await.expect_err("invalid source is refused");
    assert!(format!("{fault:?}").contains("stdio.html.invalid-source"), "{fault:?}");
    assert_eq!(app.snapshot().expect("html snapshot"), base);
    semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("original caller paid app close");
}

static SPLICE_SOURCE: &str = include_str!("../../../🧪️tests/🧫️fixtures/✂️splice-source/🔣️.json");

/// ⚖️ LAW (language-agnostic corpus `🧫️fixtures/✂️splice-source`, cross-checked by html5ever): the ranges of a case carry the committed
/// document to exactly the parse of the source they mean, the leaf's concrete inverse carries it back, and a malformed result is refused.
#[test]
fn the_corpus_ranges_reach_exactly_the_source_they_mean_and_undo() {
    let corpus: serde_json::Value = serde_json::from_str(SPLICE_SOURCE).expect("splice-source corpus");
    let oracle = semio_s_artifact_stdio_html_test_oracle::standards::v5::subsets::any::project_html_5;
    for case in corpus["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("id");
        let base = parsed(case["before"].as_str().expect("before"));
        assert_eq!(source_of(&base), case["before"].as_str().expect("before"), "{id}: the corpus source is the source the window shows");
        let ranges = case["splices"].as_array().expect("splices").iter().map(|range| splice_source::SourceSplice { offset: range["offset"].as_u64().expect("offset") as u32, delete: range["delete"].as_u64().expect("delete") as u32, insert: range["insert"].as_str().expect("insert").into() }).collect();
        let leaf = HtmlMutation::SpliceSource(splice_source::SpliceSource { splices: ranges });
        let mut state = base.clone();
        let applied = semio_s_artifact_stdio_contract::apply_mutation(&mut state, &leaf);
        if case["refused"].as_bool() == Some(true) {
            assert!(applied.messages().iter().any(|message| message.level == semio_framework_diagnostic::Severity::Fatal), "{id}: a malformed source is refused");
            assert_eq!(state, base, "{id}: a refused edit leaves the document");
            continue;
        }
        let inverse = protocol::ColdMutation::inverse(&leaf, &base).expect("valid retained mutation inverse fixture");
        assert!(applied.messages().iter().all(|message| message.level != semio_framework_diagnostic::Severity::Fatal), "{id}: the ranges apply");
        let after = case["after"].as_str().expect("after");
        assert_eq!(state, parsed(after), "{id}: the ranges reach exactly the source they mean");
        assert_eq!(oracle(source_of(&state).as_bytes()).expect("html5ever reads the edited document"), oracle(after.as_bytes()).expect("html5ever reads the expected document"), "{id}: html5ever agrees");
        for row in inverse.iter().rev() {
            semio_s_artifact_stdio_contract::apply_mutation(&mut state, row);
        }
        assert_eq!(state, base, "{id}: the concrete inverse restores the committed document");
    }
}

/// ⚖️ LAW: an empty change set is no edit, a stale revision is refused, and a range beyond the source is refused by the leaf's own diff.
#[test]
fn empty_stale_and_unrepresentable_change_sets_publish_nothing() {
    let snapshot = parsed("<!DOCTYPE html>\n<html><head></head><body><p>x</p></body></html>");
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
    let command = |revision: &str, splices: String| HtmlEditCommand::SpliceText { revision: revision.into(), splices };
    assert!(html_emit(&command(&revision, "[]".into()), &snapshot, None).expect("empty").artifact_mutations.is_empty());
    assert_eq!(html_emit(&command("stale", "[]".into()), &snapshot, None).err().expect("stale").code.0, "stdio.html.stale-text-edit");
    assert!(html_emit(&command(&revision, splices_json(&[(10_000, 0, "x")])), &snapshot, None).is_err());
    let identical = splices_json(&[(2, 7, "DOCTYPE")]);
    assert!(html_emit(&command(&revision, identical), &snapshot, None).expect("identical").artifact_mutations.is_empty());
}
//#endregion 🪟️KitVerbLaws

//#region 🪟️DetailsLaws
/// ⚖️ LAW: replacing the whole source through the details pane is a document load, not an edit, so no mutation is published.
#[test]
fn replacing_the_whole_source_is_refused_as_a_load() {
    use semio_s_artifact_stdio_contract::editing::{snapshot_edit_source, SnapshotEditEvent, SnapshotEditingEditor};
    let event = SnapshotEditEvent::ReplaceSource { source: snapshot_edit_source(&html_example_snapshot(semio_s_artifact_stdio_html_inferences::examples::demo::ID)) };
    assert_eq!(<HtmlEditor as SnapshotEditingEditor>::snapshot_edit_emit(&event, &HtmlSnapshot::default()).err().expect("a load, not an edit").code.0, "snapshot-edit.unsupported-path");
}
//#endregion 🪟️DetailsLaws
semio_plugin::history_edit_acceptance_law!("stdio", HtmlEditor, || semio_plugin::App { definition: create_html_editor(), examples: Vec::new() }, "../..");

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
    assert_eq!(emit(SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).err().expect("no kind").code.0, "snapshot-edit.unsupported-path");
}
