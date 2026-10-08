use super::*;

#[test]
fn text_edit_requires_an_explicit_change_set_and_allows_an_empty_one() {
    assert!(txt_command_from_action(TXT_KIT_ACTION_ID, None).is_err());
    let args = semio_framework_value::DslValue::object([("revision".into(), semio_framework_value::DslValue::String("revision".into())), ("splices".into(), semio_framework_value::DslValue::String("[]".into()))]);
    assert_eq!(txt_command_from_action(TXT_KIT_ACTION_ID, Some(&args)).expect("explicit empty change set"), TxtEditorCommand::SpliceText { revision: "revision".into(), splices: "[]".into() });
    let whole_draft = semio_framework_value::DslValue::object([("revision".into(), semio_framework_value::DslValue::String("revision".into())), ("text".into(), semio_framework_value::DslValue::String("whole draft".into()))]);
    assert!(txt_command_from_action(TXT_KIT_ACTION_ID, Some(&whole_draft)).is_err(), "a whole draft is no gesture");
}

#[semio_framework_async_macros::async_test]
async fn create_txt_editor_builds_a_definition_for_the_editor_role() {
    let def = create_txt_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, TXT_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<TxtEditor as ArtifactEditor>::DIALECT, TXT_EDITOR_DIALECT);
}

/// 🎯️ LAW: the editor declares the artifact kind it edits (the artifact's own `artifact_kind()`), which is
/// what the hub's one open-target rule (`app_opens_kind`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`)
/// pairs with this editor and the viewer of its dialect — so a text document can be created and opened as a hub document.
#[semio_framework_async_macros::async_test]
async fn the_editor_declares_the_artifact_kind_it_edits() {
    assert_eq!(create_txt_editor().artifact_kinds, vec![crate::artifact_kind()]);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_text_window() {
    let def = create_txt_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn native_body_codec_preserves_crlf_extra_carriage_returns_and_empty_text() {
    for source in ["a\nb\n", "a\nb", "", "a\r\r\nb\r\n"] {
        assert_eq!(TxtSnapshot::from_body(source).to_body(), source);
    }
}

#[test]
fn natural_file_route_exports_utf8_and_reopens_the_same_document() {
    let edited = TxtSnapshot::from_body("Natural Open Save\r\nGrüße 🌍\r\n");
    let bytes = <TxtEditor as ArtifactEditor>::encode_natural_file(&edited).expect("TXT natural bytes");
    let (lines, trailing_newline, is_crlf) = semio_s_artifact_stdio_txt_test_oracle::standards::v_utf_8::subsets::any::bstr_split(&bytes).expect("bstr reads exported TXT");
    assert_eq!(lines, edited.lines);
    assert!(trailing_newline);
    assert!(is_crlf);
    let reopened = <TxtEditor as ArtifactEditor>::decode_natural_file(&bytes).expect("TXT natural bytes reopen");
    assert_eq!(reopened, edited);
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = TxtEditorCommand::SpliceText { revision: "revision %20 Grüße 🌍".into(), splices: r#"[{"offset":0,"delete":1,"insert":"hello\r\nworld %20 Grüße 🌍"}]"#.into() };
    let printed = <TxtEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <TxtEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
    assert!(<TxtEditorCommand as protocol::OpText>::parse_op("splice-text revision=€0 splices=00").is_err());
}

/// 🧪️ The change set JSON of one range.
fn one_range(offset: usize, delete: usize, insert: &str) -> String {
    serde_json::json!([{ "offset": offset, "delete": delete, "insert": insert }]).to_string()
}

#[semio_framework_async_macros::async_test]
async fn direct_text_edit_is_revision_guarded_and_noop_preserving() {
    let snapshot = TxtSnapshot::from_body("a\r\r\nb\r\n");
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
    let noop = txt_emit(&TxtEditorCommand::SpliceText { revision: revision.clone(), splices: "[]".into() }, &snapshot, None).expect("empty change set");
    assert!(noop.artifact_mutations.is_empty());
    let unchanged = txt_emit(&TxtEditorCommand::SpliceText { revision: revision.clone(), splices: one_range(0, 1, "a") }, &snapshot, None).expect("a range that changes nothing");
    assert!(unchanged.artifact_mutations.is_empty());
    assert!(txt_emit(&TxtEditorCommand::SpliceText { revision: "stale".into(), splices: "[]".into() }, &snapshot, None).is_err());
    let emit = txt_emit(&TxtEditorCommand::SpliceText { revision, splices: one_range(0, 1, "x") }, &snapshot, None).expect("valid replacement");
    assert!(matches!(emit.artifact_mutations.as_slice(), [TxtMutation::SpliceText(splice)] if splice.splices == vec![TextSplice { offset: 0, delete: 1, insert: "x".into() }]));
    let mut next = snapshot.clone();
    for mutation in &emit.artifact_mutations {
        let outcome = <TxtMutation as protocol::Mutation<TxtSnapshot>>::diff(mutation, &next);
        assert!(outcome.messages().is_empty(), "native mutation {mutation:?} refused: {:?}", outcome.messages());
        next = protocol::apply_diff(outcome.diff(), &next).expect("native mutation applies");
    }
    assert_eq!(next.to_body(), "x\r\r\nb\r\n");
}

/// ⚖️ LAW: for ANY two native documents of one line ending, the one range that carries the first body to the second (its scalars
/// differing between the shared ends) publishes ONE `splice-text` that applies without a message and ends exactly at the second body;
/// a range that would switch the line ending or leave the native shape is refused by the editor, never journaled.
#[test]
fn every_range_edit_lands_on_native_documents_only() {
    const BODIES: [&str; 19] = ["", "a", "a\n", "a\r\n", "\n", "\r\n", "a\nb", "a\nb\n", "a\r\nb", "a\r\r\nb\r\n", "x\r\r\ny\r\n", "a\r", "a\n\n", "\r\n\r\n", "alpha\nbeta", "Hello, stdio.txt!\n", " ", " \n", "\r"];
    for old_body in BODIES {
        let snapshot = TxtSnapshot::from_body(old_body);
        assert_eq!(crate::schema::mutation_support::native_snapshot_error(&snapshot), None, "corpus body {old_body:?} is native");
        let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
        for new_body in BODIES {
            let (before, after): (Vec<char>, Vec<char>) = (old_body.chars().collect(), new_body.chars().collect());
            let prefix = before.iter().zip(&after).take_while(|(left, right)| left == right).count();
            let suffix = before[prefix..].iter().rev().zip(after[prefix..].iter().rev()).take_while(|(left, right)| left == right).count();
            let splices = if before == after { "[]".to_string() } else { one_range(prefix, before.len() - prefix - suffix, &after[prefix..after.len() - suffix].iter().collect::<String>()) };
            let target = TxtSnapshot::from_body(new_body);
            let reachable = crate::schema::mutation_support::native_snapshot_error(&target).is_none() && (new_body.is_empty() || target.line_ending == snapshot.line_ending || !old_body.contains("\r\n") && !new_body.contains("\r\n"));
            match txt_emit(&TxtEditorCommand::SpliceText { revision: revision.clone(), splices }, &snapshot, None) {
                Ok(emit) => {
                    let mut next = snapshot.clone();
                    for mutation in &emit.artifact_mutations {
                        let outcome = <TxtMutation as protocol::Mutation<TxtSnapshot>>::diff(mutation, &next);
                        assert!(outcome.messages().is_empty(), "{old_body:?} → {new_body:?}: {mutation:?} refused: {:?}", outcome.messages());
                        next = protocol::apply_diff(outcome.diff(), &next).expect("native mutation applies");
                    }
                    assert_eq!(next.to_body(), new_body, "{old_body:?} → {new_body:?}");
                    assert!(emit.artifact_mutations.len() <= 1, "{old_body:?} → {new_body:?}: one edit is one mutation");
                }
                Err(_) => assert!(!reachable || old_body.contains("\r\n") != new_body.contains("\r\n"), "{old_body:?} → {new_body:?} is a native edit of one line ending"),
            }
        }
    }
}

/// ⚖️ LAW: an Apply is ONE `splice-text` carrying the editor's ranges verbatim, with no static description, so its history row is
/// labelled from its leaf; ranges in two places stay one mutation.
#[test]
fn an_applied_change_set_is_one_splice_text() {
    let snapshot = TxtSnapshot::from_body("alpha\nbeta\ngamma\n");
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
    let emit = |splices: String| txt_emit(&TxtEditorCommand::SpliceText { revision: revision.clone(), splices }, &snapshot, None).expect("a native change set");
    let changed = emit(one_range(6, 4, "BETA"));
    assert!(matches!(changed.artifact_mutations.as_slice(), [TxtMutation::SpliceText(splice)] if splice.splices == vec![TextSplice { offset: 6, delete: 4, insert: "BETA".into() }]), "{:?}", changed.artifact_mutations);
    assert_eq!(<TxtMutation as protocol::SemanticMutation<TxtSnapshot>>::label(&changed.artifact_mutations[0]), semio_framework_ui_locale::LocalizedLabel::native("Edit Text", "Text bearbeiten"), "the history row is labelled from its leaf in every supported locale");
    let two = emit(serde_json::json!([{ "offset": 0, "delete": 1, "insert": "A" }, { "offset": 11, "delete": 5, "insert": "G" }]).to_string());
    assert!(matches!(two.artifact_mutations.as_slice(), [TxtMutation::SpliceText(splice)] if splice.splices.len() == 2));
    assert!(txt_emit(&TxtEditorCommand::SpliceText { revision: revision.clone(), splices: serde_json::json!([{ "offset": 3, "delete": 1, "insert": "x" }, { "offset": 2, "delete": 0, "insert": "y" }]).to_string() }, &snapshot, None).is_err(), "unordered ranges are no change set");
    assert!(txt_emit(&TxtEditorCommand::SpliceText { revision, splices: "not json".into() }, &snapshot, None).is_err());
}

/// ⚖️ LAW: an invariant refusal is FATAL (the store's contract for `mutation.invariant`), so the bounded preparation refuses the
/// whole edit instead of journaling the leaf's empty diff.
#[test]
fn invariant_refusals_are_fatal() {
    let terminated = TxtSnapshot::from_body("only\n");
    let outcome = <TxtMutation as protocol::Mutation<TxtSnapshot>>::diff(&TxtMutation::RemoveLine(RemoveLineMutation { index: 0 }), &terminated);
    assert_eq!(outcome.messages().iter().map(|message| (message.code.0.as_str(), message.level)).collect::<Vec<_>>(), vec![("mutation.invariant", semio_framework_diagnostic::Severity::Fatal)]);
    assert_eq!(outcome.diff(), &Default::default());
}

//#region 🎬️ExampleSwitchLaws
/// ⚖️ LAW: the example picker's verb is declared on the app itself, which is what
/// `appSwitchesExamples` (`🛠️ShellHelpers/🟦️.tsx`) reads before it offers the picker or announces a
/// boot example.
#[semio_framework_async_macros::async_test]
async fn the_editor_declares_the_example_pickers_verb() {
    let definition = create_txt_editor();
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
    let registry = semio_framework_plugin::AppActionRegistry::from_definition(&create_txt_editor());
    let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<TxtEditor>>::with_registry(EditorApp::<TxtEditor>::default(), registry, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
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
    let example = txt_example_snapshot(crate::examples::demo::ID);
    assert_ne!(example, TxtSnapshot::default());
    assert_eq!(txt_example_snapshot(""), TxtSnapshot::default());
    assert_eq!(txt_example_snapshot("no-such-example"), TxtSnapshot::default());
}

/// ⚖️ LAW: the shells' `{action, args}` pair resolves into the typed command, for every spelling of
/// the id the navbar, the palette and a replayed shell command use.
#[semio_framework_async_macros::async_test]
async fn the_shell_action_pair_resolves_into_the_typed_command() {
    for key in ["exampleId", "example_id", "id", "value"] {
        let args = semio_framework_value::DslValue::object([(key.to_string(), semio_framework_value::DslValue::String("demo".into()))]);
        assert_eq!(txt_command_from_action(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, Some(&args)).expect("declared verb"), TxtEditorCommand::SetActiveExample { example_id: "demo".into() });
    }
    assert!(txt_command_from_action("noSuchVerb", None).is_err());
}
//#endregion 🎬️ExampleSwitchLaws

//#region 🪟️KitVerbLaws
type KitFixtureApp = semio_framework_plugin::VcsArtifactApp<EditorApp<TxtEditor>>;

/// 🧪️ One registered fixture app holding `document`, loaded exactly as a host applies the example
/// switch's `Effect::LoadDocument`.
async fn kit_fixture_holding(document: &TxtSnapshot) -> KitFixtureApp {
    use semio_framework_plugin::PluginApp;
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<TxtEditor>, _>(async { semio_framework_plugin::App { definition: create_txt_editor(), examples: Vec::new() } }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(document, STDIO_TXT_DOCUMENT_SCHEMA) else { panic!("the example switch hands the host one whole document") };
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

/// ⚖️ LAW: `textEdit` — the verb the `TextWindowKit` mints for `🪟️main` — reaches the document through this
/// editor's exact retained factory. Unregistered, the reactor refused it inside `s` with
/// `interactive-job.missing-factory` (S15, session 11).
#[semio_framework_async_macros::async_test]
async fn the_kit_verb_edits_the_document_through_its_exact_retained_factory() {
    let mut app = kit_fixture_holding(&txt_example_snapshot(crate::examples::demo::ID)).await;
    let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(app.test_document_revision());
    let splices = one_range(0, txt_example_snapshot(crate::examples::demo::ID).to_body().chars().count(), "alpha\nbeta\n");
    dispatch_settled(&mut app, "textEdit", &[("revision", &revision), ("splices", &splices)]).await.expect("textEdit settles");
    let after = app.snapshot().expect("txt snapshot");
    assert_eq!(after.lines, vec!["alpha".to_string(), "beta".to_string()]);
    assert!(after.trailing_newline);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🪟️KitVerbLaws


semio_framework_plugin::history_edit_acceptance_law!("stdio", TxtEditor, || semio_framework_plugin::App { definition: create_txt_editor(), examples: Vec::new() }, "../..");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_field() {
    use semio_s_artifact_stdio_contract::editing::{SnapshotEditEvent, SnapshotEditingEditor};
    let base = TxtSnapshot::from_body("one\ntwo\n");
    let emit = |event: SnapshotEditEvent| <TxtEditor as SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let line = emit(SnapshotEditEvent::SetValue { path: "/lines/1".into(), value: semio_framework_value::DslValue::String("TWO".into()) }).expect("a line edit resolves");
    assert!(matches!(line.artifact_mutations.as_slice(), [TxtMutation::SetLine(set)] if (set.index, set.text.as_str()) == (1, "TWO")));
    let inserted = emit(SnapshotEditEvent::InsertValue { path: "/lines/-".into(), value: semio_framework_value::DslValue::String("three".into()) }).expect("a line insertion resolves");
    assert!(matches!(inserted.artifact_mutations.as_slice(), [TxtMutation::InsertLine(insert)] if insert.index == 2));
    let removed = emit(SnapshotEditEvent::RemoveValue { path: "/lines/0".into() }).expect("a line removal resolves");
    assert!(matches!(removed.artifact_mutations.as_slice(), [TxtMutation::RemoveLine(_)]));
    assert_eq!(emit(SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).expect_err("no kind").code.0, "snapshot-edit.unsupported-path");
}
