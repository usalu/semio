use super::*;

#[test]
fn text_edit_requires_an_explicit_revision_and_change_set_and_allows_an_empty_one() {
    assert!(md_command_from_action(MD_KIT_ACTION_ID, None).is_err());
    let only_revision = semio_framework_value::DslValue::object([("revision".into(), semio_framework_value::DslValue::String("r".into()))]);
    assert!(md_command_from_action(MD_KIT_ACTION_ID, Some(&only_revision)).is_err());
    let args = semio_framework_value::DslValue::object([("revision".into(), semio_framework_value::DslValue::String("r".into())), ("splices".into(), semio_framework_value::DslValue::String("[]".into()))]);
    assert_eq!(md_command_from_action(MD_KIT_ACTION_ID, Some(&args)).expect("explicit empty change set"), MdEditCommand::SpliceText { revision: "r".into(), splices: "[]".into() });
}

#[test]
fn natural_file_route_exports_commonmark_and_reopens_the_same_document() {
    let edited = MdSnapshot::from_text("# Natural Open Save\n\nEdited body.\n");
    let bytes = <MdEditor as ArtifactEditor>::encode_natural_file(&edited).expect("Markdown natural bytes");
    let oracle = semio_s_artifact_stdio_md_test_oracle::standards::v_commonmark::subsets::any::project_md;
    assert_eq!(oracle(&bytes).expect("Comrak reads exported Markdown"), oracle(b"# Natural Open Save\n\nEdited body.\n").expect("Comrak reads expected Markdown"));
    let reopened = <MdEditor as ArtifactEditor>::decode_natural_file(&bytes).expect("Markdown natural bytes reopen");
    assert_eq!(reopened, edited);
}

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_md_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, MD_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<MdEditor as ArtifactEditor>::DIALECT, MD_DIALECT);
}

//#region 🎬️ExampleSwitchLaws
/// ⚖️ LAW: the example picker's verb is declared on the app itself, which is what
/// `appSwitchesExamples` (`🛠️ShellHelpers/🟦️.tsx`) reads before it offers the picker or announces a
/// boot example.
#[semio_framework_async_macros::async_test]
async fn the_editor_declares_the_example_pickers_verb() {
    let definition = create_md_editor();
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
    let registry = semio_framework_plugin::AppActionRegistry::from_definition(&create_md_editor());
    let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<MdEditor>>::with_registry(EditorApp::<MdEditor>::default(), registry, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::editing::fixture_mounted_owner_policy());
    assert!(app.close_terminal_is_empty(), "registered fixture reaches its exact terminal-empty witness");
}

/// ⚖️ LAW: the curated example parses into a document that differs from the genesis one, and an
/// unknown or empty id falls back to the genesis document.
#[semio_framework_async_macros::async_test]
async fn the_curated_example_carries_visible_content() {
    let example = md_example_snapshot(crate::examples::demo::ID);
    assert_ne!(example, MdSnapshot::default());
    assert_eq!(md_example_snapshot(""), MdSnapshot::default());
    assert_eq!(md_example_snapshot("no-such-example"), MdSnapshot::default());
}

/// ⚖️ LAW: the shells' `{action, args}` pair resolves into the typed command, for every spelling of
/// the id the navbar, the palette and a replayed shell command use.
#[semio_framework_async_macros::async_test]
async fn the_shell_action_pair_resolves_into_the_typed_command() {
    for key in ["exampleId", "example_id", "id", "value"] {
        let args = semio_framework_value::DslValue::object([(key.to_string(), semio_framework_value::DslValue::String("demo".into()))]);
        assert_eq!(md_command_from_action(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, Some(&args)).expect("declared verb"), MdEditCommand::SetActiveExample { example_id: "demo".into() });
    }
    assert!(md_command_from_action("noSuchVerb", None).is_err());
}
//#endregion 🎬️ExampleSwitchLaws

//#region 🪟️KitVerbLaws
type KitFixtureApp = semio_framework_plugin::VcsArtifactApp<EditorApp<MdEditor>>;

/// 🧪️ One registered fixture app holding `document`, loaded exactly as a host applies the example
/// switch's `Effect::LoadDocument`.
async fn kit_fixture_holding(document: &MdSnapshot) -> KitFixtureApp {
    use semio_framework_plugin::PluginApp;
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<MdEditor>, _>(async { semio_framework_plugin::App { definition: create_md_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(document, STDIO_MD_DOCUMENT_SCHEMA) else { panic!("the example switch hands the host one whole document") };
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

/// 🧾️ The change set JSON the editor sends: `(offset, delete, insert)` ranges of Unicode scalars of the source it started from.
fn splices_json(ranges: &[(usize, usize, &str)]) -> String {
    serde_json::Value::Array(ranges.iter().map(|(offset, delete, insert)| serde_json::json!({ "offset": offset, "delete": delete, "insert": insert })).collect()).to_string()
}

/// ⚖️ LAW: `textEdit` — the verb the `TextWindowKit` mints for `🪟️main` — reaches the document through this
/// editor's exact retained factory. Unregistered, the reactor refused it inside `s` with
/// `interactive-job.missing-factory` (S15, session 11).
#[semio_framework_async_macros::async_test]
async fn the_kit_verb_edits_the_document_through_its_exact_retained_factory() {
    let base = MdSnapshot::default();
    let target = MdSnapshot::from_text(&md_example_snapshot(crate::examples::demo::ID).to_text());
    let mut app = kit_fixture_holding(&base).await;
    let (revision, base_text) = (semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&base), base.to_text());
    let changes = splices_json(&[(0, base_text.chars().count(), &target.to_text())]);
    dispatch_settled(&mut app, "textEdit", &[("revision", &revision), ("splices", &changes)]).await.expect("textEdit settles");
    assert_eq!(app.snapshot().expect("md snapshot"), target);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🪟️KitVerbLaws

//#region ✂️SpliceLaws
/// ⚖️ LAW: the kit verb applies the editor's change set verbatim as ONE `splice-source` edit: one changed word is one row
/// labelled from the leaf in every supported locale, the document reads exactly the edited source, and ONE undo restores the
/// committed document.
#[semio_framework_async_macros::async_test]
async fn one_applied_change_set_is_one_splice_source_edit() {
    use semio_framework_plugin::PluginApp;
    let (before, after) = ("# Title\n\nFirst.\n\nSecond.\n", "# Title\n\nFirst, edited.\n\nSecond.\n");
    let base = MdSnapshot::from_text(before);
    let mut app = kit_fixture_holding(&base).await;
    let edits = app.edit_transactions().len();
    let offset = base.to_text().find("First.").expect("the paragraph") + "First".len();
    let changes = splices_json(&[(base.to_text()[..offset].chars().count(), 0, ", edited")]);
    dispatch_settled(&mut app, "textEdit", &[("revision", &semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&base)), ("splices", &changes)]).await.expect("the applied change set settles");
    assert_eq!(app.snapshot().expect("md snapshot"), MdSnapshot::from_text(after));
    assert_eq!(app.edit_transactions().len(), edits + 1, "one Apply, one edit");
    let rows: Vec<_> = app.history_snapshot().await.expect("history").upserts.into_iter().filter(|row| row.edit_id.is_some()).collect();
    let row = rows.iter().max_by_key(|row| row.seq).expect("the applied change set's row");
    assert_eq!(row.mutations.len(), 1, "one change set is one leaf: {:?}", row.mutations);
    assert_eq!(row.mutations[0].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Edit source");
    assert_eq!(row.mutations[0].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "Quelltext bearbeiten");
    semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app, "undo", semio_framework_plugin::artifact_app_laws::meta("local").instance_id).await;
    assert_eq!(app.snapshot().expect("md snapshot"), base, "one undo restores the committed document");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}

const SPLICE_SOURCE: &str = include_str!("../../../🧫️fixtures/✂️splice-source/🔣️.json");

/// ⚖️ LAW (corpus `🧫️fixtures/✂️splice-source`): the ranges of a row carry the committed document to exactly the parse of the
/// source they mean, and the leaf's own concrete inverse carries it back to the committed document.
#[test]
fn the_corpus_ranges_reach_exactly_the_source_they_mean_and_undo() {
    let corpus: serde_json::Value = serde_json::from_str(SPLICE_SOURCE).expect("splice-source corpus");
    for case in corpus["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("id");
        let base = MdSnapshot::from_text(case["before"].as_str().expect("before"));
        assert_eq!(base.to_text(), case["before"].as_str().expect("before"), "{id}: the corpus source is the source the window shows");
        let ranges = case["splices"].as_array().expect("splices").iter().map(|range| splice_source::SourceSplice { offset: range["offset"].as_u64().expect("offset") as u32, delete: range["delete"].as_u64().expect("delete") as u32, insert: range["insert"].as_str().expect("insert").into() }).collect();
        let leaf = MdMutation::SpliceSource(splice_source::SpliceSource { splices: ranges });
        let inverse = protocol::Mutation::inverse(&leaf, &base).expect("valid retained mutation inverse fixture");
        let mut state = base.clone();
        assert!(crate::apply_mutation(&mut state, &leaf).messages().iter().all(|message| message.level != semio_framework_diagnostic::Severity::Fatal), "{id}: the ranges apply");
        assert_eq!(state, MdSnapshot::from_text(case["after"].as_str().expect("after")), "{id}: the ranges reach exactly the source they mean");
        for row in inverse.iter().rev() {
            crate::apply_mutation(&mut state, row);
        }
        assert_eq!(state, base, "{id}: the concrete inverse restores the committed document");
    }
}

/// ⚖️ LAW: an empty change set is no edit, a stale revision is refused, and a range beyond the source is refused by the leaf's own
/// diff instead of being published.
#[test]
fn empty_stale_and_unrepresentable_change_sets_publish_nothing() {
    let snapshot = MdSnapshot::from_text("# Title\n\nBody.\n");
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
    let command = |revision: &str, splices: String| MdEditCommand::SpliceText { revision: revision.into(), splices };
    assert!(md_emit(&command(&revision, "[]".into()), &snapshot, None).expect("empty").artifact_mutations.is_empty());
    assert_eq!(md_emit(&command("stale", "[]".into()), &snapshot, None).expect_err("stale").code.0, "stdio.md.stale-text-edit");
    assert!(md_emit(&command(&revision, splices_json(&[(10_000, 0, "x")])), &snapshot, None).is_err());
    let identical = splices_json(&[(2, 5, "Title")]);
    assert!(md_emit(&command(&revision, identical), &snapshot, None).expect("identical").artifact_mutations.is_empty());
}

/// ⚖️ LAW: replacing the whole source through the details pane is a document load, not an edit, so no mutation is published.
#[test]
fn replacing_the_whole_source_is_refused_as_a_load() {
    use semio_s_artifact_stdio_contract::editing::{snapshot_edit_source, SnapshotEditEvent, SnapshotEditingEditor};
    let (base, next) = (MdSnapshot::from_text("# Title\n\nBody.\n"), MdSnapshot::from_text("# Other\n"));
    let event = SnapshotEditEvent::ReplaceSource { source: snapshot_edit_source(&next) };
    assert_eq!(<MdEditor as SnapshotEditingEditor>::snapshot_edit_emit(&event, &base).expect_err("a load, not an edit").code.0, "snapshot-edit.unsupported-path");
}
//#endregion ✂️SpliceLaws

semio_framework_plugin::history_edit_acceptance_law!("stdio", MdEditor, || semio_framework_plugin::App { definition: create_md_editor(), examples: Vec::new() }, "../..");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_block() {
    use semio_s_artifact_stdio_contract::editing::{SnapshotEditEvent, SnapshotEditingEditor};
    let base = MdSnapshot::from_text("# Title\n\nFirst.\n\n> quoted\n");
    let emit = |event: SnapshotEditEvent| <MdEditor as SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let inlines = emit(SnapshotEditEvent::SetValue { path: "/blocks/1/inlines/0/text".into(), value: semio_framework_value::DslValue::String("Second.".into()) }).expect("an inline edit resolves");
    assert!(matches!(inlines.artifact_mutations.as_slice(), [MdMutation::SetInlines(_)]));
    let removed = emit(SnapshotEditEvent::RemoveValue { path: "/blocks/0".into() }).expect("a block removal resolves");
    assert!(matches!(removed.artifact_mutations.as_slice(), [MdMutation::RemoveBlock(_)]));
    let nested = emit(SnapshotEditEvent::RemoveValue { path: "/blocks/2/blocks/0".into() }).expect("a nested block removal resolves");
    assert!(matches!(nested.artifact_mutations.as_slice(), [MdMutation::RemoveBlock(remove)] if remove.path.len() == 1));
    let level = emit(SnapshotEditEvent::SetValue { path: "/blocks/0/level".into(), value: semio_framework_value::DslValue::uint(2) }).expect("a heading level edit resolves");
    assert!(matches!(level.artifact_mutations.as_slice(), [MdMutation::ReplaceBlock(_)]));
    assert_eq!(emit(SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).expect_err("no kind").code.0, "snapshot-edit.unsupported-path");
}
