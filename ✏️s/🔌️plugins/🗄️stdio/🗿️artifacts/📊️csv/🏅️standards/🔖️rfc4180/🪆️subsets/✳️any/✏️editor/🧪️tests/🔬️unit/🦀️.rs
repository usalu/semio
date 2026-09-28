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
    let command = CsvEditorCommand::SetCell { row: 2, column: 5, revision: "révision %20".into(), value: "a \\s value %20\nGrüße 🌍".into() };
    let printed = <CsvEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <CsvEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
}

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip_preserves_example_ids_without_escape_reinterpretation() {
    let command = CsvEditorCommand::SetActiveExample { example_id: "literal\\s%20\n日本語".into() };
    let printed = <CsvEditorCommand as protocol::OpText>::print_op(&command);
    let parsed = <CsvEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
    assert_eq!(parsed, command);
}

#[test]
fn set_cell_requires_its_full_address_and_accepts_an_empty_value() {
    let args = dsl::DslValue::object([
        ("row".into(), dsl::DslValue::Number(dsl::Number::UInt(1))),
        ("column".into(), dsl::DslValue::Number(dsl::Number::UInt(2))),
        ("revision".into(), dsl::DslValue::String("rev".into())),
        ("value".into(), dsl::DslValue::String(String::new())),
    ]);
    assert_eq!(csv_command_from_action(CSV_KIT_ACTION_ID, Some(&args)).expect("complete cell address"), CsvEditorCommand::SetCell { row: 1, column: 2, revision: "rev".into(), value: String::new() });
    assert!(csv_command_from_action(CSV_KIT_ACTION_ID, None).is_err());
    let missing_value = dsl::DslValue::object([("row".into(), dsl::DslValue::Number(dsl::Number::UInt(1))), ("column".into(), dsl::DslValue::Number(dsl::Number::UInt(2)))]);
    assert!(csv_command_from_action(CSV_KIT_ACTION_ID, Some(&missing_value)).is_err());
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
        assert_eq!(csv_command_from_action(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, Some(&args)).expect("declared verb"), CsvEditorCommand::SetActiveExample { example_id: "demo".into() });
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
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(document, STDIO_CSV_DOCUMENT_SCHEMA) else { panic!("the example switch hands the host one whole document") };
    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.expect("the host loads the example document");
    app
}

/// 🕹️ Dispatches `action` with typed renderer `args`, exactly as the editable cell sends them, and settles it through
/// the host's bounded publication loop.
async fn dispatch_settled(app: &mut KitFixtureApp, action: &str, args: dsl::DslValue) -> Result<(), Fault> {
    use semio_framework_plugin::PluginApp;
    let meta = semio_framework_plugin::artifact_app_laws::meta("local");
    app.handle_action(action, Some(&args), &meta).await?;
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, meta.instance_id).await.map(|_| ())
}

/// ⚖️ LAW: `set-cell` — the verb the `TableWindowKit` mints for `🪟️main` — reaches the document through this
/// editor's exact retained factory. Unregistered, the reactor refused it inside `s` with
/// `interactive-job.missing-factory` (S15, session 11).
#[semio_framework_async_macros::async_test]
async fn the_kit_verb_edits_the_document_through_its_exact_retained_factory() {
    let source = csv_example_snapshot(crate::examples::demo::ID);
    let mut app = kit_fixture_holding(&source).await;
    let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(app.test_document_revision());
    let args = dsl::DslValue::object([
        ("row".into(), dsl::DslValue::Number(dsl::Number::UInt(0))),
        ("column".into(), dsl::DslValue::Number(dsl::Number::UInt(0))),
        ("revision".into(), dsl::DslValue::String(revision)),
        ("value".into(), dsl::DslValue::String("Zeta".into())),
    ]);
    dispatch_settled(&mut app, "set-cell", args).await.expect("set-cell settles");
    let after = app.snapshot().expect("csv snapshot");
    assert_eq!(after.records[grid_row_to_record_index(after.has_header, 0)].fields[0].value, "Zeta");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}

#[test]
fn set_cell_rejects_stale_revisions_and_addresses_without_mutation() {
    let source = csv_example_snapshot(crate::examples::demo::ID);
    assert!(csv_emit(&CsvEditorCommand::SetCell { row: 0, column: 0, revision: "stale".into(), value: "x".into() }, &source).is_err());
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&source);
    assert!(csv_emit(&CsvEditorCommand::SetCell { row: u32::MAX, column: 0, revision: revision.clone(), value: "x".into() }, &source).is_err());
    assert!(csv_emit(&CsvEditorCommand::SetCell { row: 0, column: u32::MAX, revision, value: "x".into() }, &source).is_err());
}
/// ⚖️ LAW (agent lane, ticket 26/09/23 G12 session 14c): an agent's `set-cell` without a `revision` is admitted against the
/// document's revision — it previews the edit exactly as with the token the rendered cell carries, a stale token is still
/// refused — while the shell lane keeps refusing the omission.
#[semio_framework_async_macros::async_test]
async fn an_agent_set_cell_without_a_revision_previews_against_the_document_revision_and_the_shell_lane_still_requires_it() {
    let source = csv_example_snapshot(crate::examples::demo::ID);
    let mut app = kit_fixture_holding(&source).await;
    let definition = create_csv_editor();
    let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(app.test_document_revision());
    let address = |revision: Option<String>| {
        dsl::DslValue::object(
            [("row".to_string(), dsl::DslValue::Number(dsl::Number::UInt(0))), ("column".to_string(), dsl::DslValue::Number(dsl::Number::UInt(0))), ("value".to_string(), dsl::DslValue::String("Zeta".into()))]
                .into_iter()
                .chain(revision.map(|revision| ("revision".to_string(), dsl::DslValue::String(revision)))),
        )
    };
    let omitted = semio_framework_plugin::artifact_app_laws::agent_preview(&mut app, &definition, "set-cell", &address(None)).await.expect("the agent lane admits an omitted revision");
    let bound = semio_framework_plugin::artifact_app_laws::agent_preview(&mut app, &definition, "set-cell", &address(Some(revision))).await.expect("the rendered cell's revision");
    assert!(omitted > 0 && omitted == bound, "omitted {omitted} vs bound {bound} document ops");
    assert!(semio_framework_plugin::artifact_app_laws::agent_preview(&mut app, &definition, "set-cell", &address(Some("stale".into()))).await.is_err(), "a stale token is refused on the agent lane too");
    assert!(dispatch_settled(&mut app, "set-cell", address(None)).await.is_err(), "the shell lane still requires the rendered cell's revision");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🪟️KitVerbLaws

#[test]
fn structural_command_codecs_roundtrip_unicode_and_empty_headers() {
    for command in [
        CsvEditorCommand::AddRow { revision: "rév %20".into() },
        CsvEditorCommand::RemoveRow { row: 7, revision: "rév %20".into() },
        CsvEditorCommand::AddColumn { revision: "rév %20".into() },
        CsvEditorCommand::RemoveColumn { column: 3, revision: "rév %20".into() },
        CsvEditorCommand::SetHeader { column: 2, revision: "rév %20".into(), value: r"\s\n日本語".into() },
        CsvEditorCommand::SetHeader { column: 0, revision: "rév %20".into(), value: String::new() },
    ] {
        let encoded = <CsvEditorCommand as protocol::OpBinary>::encode_op(&command).expect("encode");
        assert_eq!(<CsvEditorCommand as protocol::OpBinary>::decode_op(&encoded).expect("decode"), command);
    }
}

#[test]
fn blank_csv_can_build_edit_and_remove_a_table_through_structural_mutations() {
    fn apply(snapshot: &mut CsvSnapshot, command: CsvEditorCommand) {
        let mut emitted = csv_emit(&command, snapshot).expect("structural edit");
        assert_eq!(emitted.artifact_mutations.len(), 1);
        let mutation = emitted.artifact_mutations.pop().expect("one mutation");
        crate::schema::mutations::apply_csv_mutation(snapshot, &mutation);
    }
    let mut snapshot = CsvSnapshot::default();
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
    apply(&mut snapshot, CsvEditorCommand::AddColumn { revision });
    assert_eq!(snapshot.records.len(), 1);
    assert_eq!(snapshot.records[0].fields.len(), 1);
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
    apply(&mut snapshot, CsvEditorCommand::SetHeader { column: 0, revision, value: "Name".into() });
    assert_eq!(snapshot.records[0].fields[0].value, "Name");
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
    apply(&mut snapshot, CsvEditorCommand::AddRow { revision });
    assert_eq!(snapshot.records.len(), 2);
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
    apply(&mut snapshot, CsvEditorCommand::SetCell { row: 0, column: 0, revision, value: "Ada".into() });
    assert_eq!(snapshot.records[1].fields[0].value, "Ada");
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
    apply(&mut snapshot, CsvEditorCommand::RemoveRow { row: 0, revision });
    assert_eq!(snapshot.records.len(), 1);
    let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
    apply(&mut snapshot, CsvEditorCommand::RemoveColumn { column: 0, revision });
    assert!(snapshot.records[0].fields.is_empty());
}
