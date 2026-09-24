use super::*;

#[semio_framework_async_macros::async_test]
async fn create_csv_editor_builds_a_definition_for_the_editor_role() {
    let def = create_csv_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, CSV_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<CsvEditor as ArtifactEditor>::DIALECT, CSV_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_table_window() {
    let def = create_csv_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn grid_row_offsets_by_one_when_has_header() {
    assert_eq!(grid_row_to_record_index(true, 0), 1);
    assert_eq!(grid_row_to_record_index(false, 0), 0);
    assert_eq!(grid_row_to_record_index(true, 3), 4);
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip() {
    let command = CsvEditorCommand::SetCell { row: 2, column: 5, value: "a value".into() };
    let printed = <CsvEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <CsvEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
}

//#region 🎬️ExampleSwitchLaws
/// ⚖️ LAW: the example picker's verb is declared on the app itself, which is what
/// `appSwitchesExamples` (`🛠️ShellHelpers/🟦️.tsx`) reads before it offers the picker or announces a
/// boot example.
#[semio_framework_async_macros::async_test]
async fn the_editor_declares_the_example_pickers_verb() {
    let definition = create_csv_editor();
    let action = definition.actions.iter().find(|action| action.id == semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID).expect("declared example verb");
    assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    assert!(action.semantics.effects.destructive);
    assert!(action.args.iter().any(|arg| arg.id == "exampleId"));
}

/// ⚖️ LAW: the example switch's bounded-first-step proof joins its exact registered factory. A
/// registry-backed controller PANICS at construction (`interactive-job.catalog-authority` /
/// `interactive-job.catalog-incomplete`) whenever the retained roster, the publication contract, the
/// `Migrated` classification and `OpBinary::TOOL_JOB_IDS` disagree — so constructing one is the proof.
#[semio_framework_async_macros::async_test]
async fn the_example_switch_joins_its_exact_retained_factory() {
    use semio_framework_plugin::PluginApp;
    let registry = semio_framework_plugin::AppActionRegistry::from_definition(&create_csv_editor());
    let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<CsvEditor>>::with_registry(EditorApp::<CsvEditor>::default(), registry).await;
    while !app.close_terminal_is_empty() {
        if matches!(app.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("registered fixture close"), semio_framework_plugin::PluginCloseStep::Complete) {
            break;
        }
    }
    assert!(app.close_terminal_is_empty(), "registered fixture reaches its exact terminal-empty witness");
}

/// ⚖️ LAW: the curated example parses into a document with real content, and an unknown or empty id
/// falls back to the genesis document.
#[semio_framework_async_macros::async_test]
async fn the_curated_example_carries_visible_content() {
    let example = csv_example_snapshot(crate::examples::demo::ID);
    assert_ne!(example, CsvSnapshot::default());
    assert!(!example.records.is_empty());
    assert_eq!(csv_example_snapshot(""), CsvSnapshot::default());
    assert_eq!(csv_example_snapshot("no-such-example"), CsvSnapshot::default());
}

/// ⚖️ LAW: the shells' `{action, args}` pair resolves into the typed command, for every spelling of
/// the id the navbar, the palette and a replayed shell command use.
#[semio_framework_async_macros::async_test]
async fn the_shell_action_pair_resolves_into_the_typed_command() {
    for key in ["exampleId", "example_id", "id", "value"] {
        let args = dsl::DslValue::object([(key.to_string(), dsl::DslValue::String("demo".into()))]);
        assert_eq!(
            csv_command_from_action(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, Some(&args)).expect("declared verb"),
            CsvEditorCommand::SetActiveExample { example_id: "demo".into() }
        );
    }
    assert!(csv_command_from_action("noSuchVerb", None).is_err());
}
//#endregion 🎬️ExampleSwitchLaws

//#region 🪟️KitVerbLaws
type KitFixtureApp = semio_framework_plugin::VcsArtifactApp<EditorApp<CsvEditor>>;

/// 🧪️ One registered fixture app holding `document`, loaded exactly as a host applies the example
/// switch's `Effect::LoadDocument`.
async fn kit_fixture_holding(document: &CsvSnapshot) -> KitFixtureApp {
    use semio_framework_plugin::PluginApp;
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<CsvEditor>, _>(async { semio_framework_plugin::App { definition: create_csv_editor(), examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(document, STDIO_CSV_DOCUMENT_SCHEMA) else {
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

/// ⚖️ LAW: `set-cell` — the verb the `TableWindowKit` mints for `🪟️main` — reaches the document through this
/// editor's exact retained factory. Unregistered, the reactor refused it inside `s` with
/// `interactive-job.missing-factory` (S15, session 11).
#[semio_framework_async_macros::async_test]
async fn the_kit_verb_edits_the_document_through_its_exact_retained_factory() {
    let mut app = kit_fixture_holding(&csv_example_snapshot(crate::examples::demo::ID)).await;
    dispatch_settled(&mut app, "set-cell", &[("row", "0"), ("column", "0"), ("value", "Zeta")]).await.expect("set-cell settles");
    let after = app.snapshot().expect("csv snapshot");
    assert_eq!(after.records[grid_row_to_record_index(after.has_header, 0)].fields[0].value, "Zeta");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🪟️KitVerbLaws
