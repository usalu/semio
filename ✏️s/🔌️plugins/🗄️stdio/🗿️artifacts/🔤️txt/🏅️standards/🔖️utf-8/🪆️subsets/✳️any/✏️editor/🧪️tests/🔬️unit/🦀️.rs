use super::*;

#[test]
fn text_edit_requires_an_explicit_text_value_and_allows_empty_documents() {
    assert!(txt_command_from_action(TXT_KIT_ACTION_ID, None).is_err());
    let args = dsl::DslValue::object([("revision".into(), dsl::DslValue::String("revision".into())), ("text".into(), dsl::DslValue::String(String::new()))]);
    assert_eq!(txt_command_from_action(TXT_KIT_ACTION_ID, Some(&args)).expect("explicit empty text"), TxtEditorCommand::ReplaceText { revision: "revision".into(), text: String::new() });
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

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = TxtEditorCommand::ReplaceText { revision: "revision %20 Grüße 🌍".into(), text: "hello\r\nworld %20 Grüße 🌍".into() };
    let printed = <TxtEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <TxtEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
    assert!(<TxtEditorCommand as protocol::OpText>::parse_op("replace-text revision=€0 text=00").is_err());
}

#[semio_framework_async_macros::async_test]
async fn direct_text_edit_is_revision_guarded_and_noop_preserving() {
    let snapshot = TxtSnapshot::from_body("a\r\r\nb\r\n");
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
    let noop = txt_emit(&TxtEditorCommand::ReplaceText { revision: revision.clone(), text: snapshot.to_body() }, &snapshot, None).expect("same text");
    assert!(noop.artifact_mutations.is_empty());
    assert!(txt_emit(&TxtEditorCommand::ReplaceText { revision: "stale".into(), text: String::new() }, &snapshot, None).is_err());
    let emit = txt_emit(&TxtEditorCommand::ReplaceText { revision, text: "x\r\r\ny\r\n".into() }, &snapshot, None).expect("valid replacement");
    let mut next = snapshot.clone();
    for mutation in &emit.artifact_mutations {
        let outcome = <TxtMutation as protocol::Mutation<TxtSnapshot>>::diff(mutation, &next);
        assert!(outcome.messages().is_empty(), "native mutation {mutation:?} refused: {:?}", outcome.messages());
        next = protocol::MutationDiff::apply(outcome.diff(), &next).expect("native mutation applies");
    }
    assert_eq!(next.to_body(), "x\r\r\ny\r\n");
}

/// ⚖️ LAW: a whole-buffer replacement between ANY two native documents lowers to mutations that each apply without a message
/// (no refused leaf, no empty diff journaled) and end exactly at the replacement — the text round trip `to_body()` is the
/// oracle. The corpus crosses LF/CRLF, terminated/unterminated, empty, blank-line and bare-CR bodies.
#[test]
fn every_replacement_lowers_through_native_documents_only() {
    const BODIES: [&str; 19] = ["", "a", "a\n", "a\r\n", "\n", "\r\n", "a\nb", "a\nb\n", "a\r\nb", "a\r\r\nb\r\n", "x\r\r\ny\r\n", "a\r", "a\n\n", "\r\n\r\n", "alpha\nbeta", "Hello, stdio.txt!\n", " ", " \n", "\r"];
    for old_body in BODIES {
        let snapshot = TxtSnapshot::from_body(old_body);
        assert_eq!(native_snapshot_error(&snapshot), None, "corpus body {old_body:?} is native");
        let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
        for new_body in BODIES {
            let emit = txt_emit(&TxtEditorCommand::ReplaceText { revision: revision.clone(), text: new_body.into() }, &snapshot, None).expect("native replacement lowers");
            let mut next = snapshot.clone();
            for mutation in &emit.artifact_mutations {
                let outcome = <TxtMutation as protocol::Mutation<TxtSnapshot>>::diff(mutation, &next);
                assert!(outcome.messages().is_empty(), "{old_body:?} → {new_body:?}: {mutation:?} refused: {:?}", outcome.messages());
                next = protocol::MutationDiff::apply(outcome.diff(), &next).expect("native mutation applies");
            }
            assert_eq!(next.to_body(), new_body, "{old_body:?} → {new_body:?}");
            let mut expected = TxtSnapshot::from_body(new_body);
            expected.schema.clone_from(&snapshot.schema);
            assert_eq!(next, expected, "{old_body:?} → {new_body:?}");
        }
    }
}

/// ⚖️ LAW: an invariant refusal is FATAL (the store's contract for `mutation.invariant`), so the bounded preparation refuses the
/// whole edit instead of journaling the leaf's empty diff.
#[test]
fn invariant_refusals_are_fatal() {
    let terminated = TxtSnapshot::from_body("only\n");
    let outcome = <TxtMutation as protocol::Mutation<TxtSnapshot>>::diff(&TxtMutation::RemoveLine(RemoveLineMutation { index: 0 }), &terminated);
    assert_eq!(outcome.messages().iter().map(|message| (message.code.0.as_str(), message.level)).collect::<Vec<_>>(), vec![("mutation.invariant", protocol::Severity::Fatal)]);
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
    let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<TxtEditor>>::with_registry(EditorApp::<TxtEditor>::default(), registry).await;
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
        let args = dsl::DslValue::object([(key.to_string(), dsl::DslValue::String("demo".into()))]);
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
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<TxtEditor>, _>(async { semio_framework_plugin::App { definition: create_txt_editor(), examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(document, STDIO_TXT_DOCUMENT_SCHEMA) else { panic!("the example switch hands the host one whole document") };
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

/// ⚖️ LAW: `replace-text` — the verb the `TextWindowKit` mints for `🪟️main` — reaches the document through this
/// editor's exact retained factory. Unregistered, the reactor refused it inside `s` with
/// `interactive-job.missing-factory` (S15, session 11).
#[semio_framework_async_macros::async_test]
async fn the_kit_verb_edits_the_document_through_its_exact_retained_factory() {
    let mut app = kit_fixture_holding(&txt_example_snapshot(crate::examples::demo::ID)).await;
    let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(app.test_document_revision());
    dispatch_settled(&mut app, "textEdit", &[("revision", &revision), ("text", "alpha\nbeta\n")]).await.expect("replace-text settles");
    let after = app.snapshot().expect("txt snapshot");
    assert_eq!(after.lines, vec!["alpha".to_string(), "beta".to_string()]);
    assert!(after.trailing_newline);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🪟️KitVerbLaws
