use super::*;

#[semio_framework_async_macros::async_test]
async fn create_epw_editor_builds_a_definition_for_the_editor_role() {
    let def = create_epw_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, EPW_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<EpwEditor as ArtifactEditor>::DIALECT, EPW_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_epw_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[test]
fn set_cell_requires_its_full_address_and_accepts_an_empty_value() {
    let args = dsl::DslValue::object([
        ("row".into(), dsl::DslValue::Number(dsl::Number::UInt(1))),
        ("column".into(), dsl::DslValue::Number(dsl::Number::UInt(2))),
        ("revision".into(), dsl::DslValue::String("row-revision".into())),
        ("value".into(), dsl::DslValue::String(String::new())),
    ]);
    let command = <EpwEditor as ArtifactEditor>::command_from_action("set-cell", Some(&args)).expect("complete cell address");
    assert_eq!(command, semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(EpwEditorCommand::SetCell { row: 1, column: main::EPW_TABLE_COLUMNS[2].into(), revision: "row-revision".into(), value: String::new() }));
    assert!(<EpwEditor as ArtifactEditor>::command_from_action("set-cell", None).is_err());
    let missing_value = dsl::DslValue::object([("row".into(), dsl::DslValue::Number(dsl::Number::UInt(1))), ("column".into(), dsl::DslValue::Number(dsl::Number::UInt(2)))]);
    assert!(<EpwEditor as ArtifactEditor>::command_from_action("set-cell", Some(&missing_value)).is_err());
}

#[test]
fn set_cell_matches_the_language_neutral_renderer_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/✏️set-cell/🔣️.json")).expect("EPW cell fixture");
    for case in fixture["cases"].as_array().expect("fixture cases") {
        let args: dsl::DslValue = semio_s_artifact_stdio_contract::pack::json::from_json_str(&case["args"].to_string()).expect("typed fixture args");
        let command = <EpwEditor as ArtifactEditor>::command_from_action("set-cell", Some(&args)).expect("fixture command");
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(EpwEditorCommand::SetCell { row, column, revision, value }) = command else { panic!("fixture must produce the native cell command") };
        assert_eq!(u64::from(row), case["command"]["row"].as_u64().expect("row"));
        assert_eq!(column, case["command"]["column"].as_str().expect("column"));
        assert_eq!(revision, case["command"]["revision"].as_str().expect("revision"));
        assert_eq!(value, case["command"]["value"].as_str().expect("value"));
    }
}

#[test]
fn set_cell_refuses_stale_rows_and_unknown_columns() {
    let snapshot = EpwSnapshot { records: vec![crate::standards::energyplus::subsets::any::schema::snapshot::EpwRecord::default()], ..Default::default() };
    let revision = epw_row_revision(&snapshot.records[0]);
    assert!(epw_set_cell_emit(&snapshot, 1, main::EPW_TABLE_COLUMNS[0], &revision, "2026").is_err());
    assert!(epw_set_cell_emit(&snapshot, 0, "unknown", &revision, "2026").is_err());
    assert!(epw_set_cell_emit(&snapshot, 0, main::EPW_TABLE_COLUMNS[0], "stale", "2026").is_err());
}

type KitFixtureApp = semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<EpwEditor>>;

async fn kit_fixture_holding(document: &EpwSnapshot) -> KitFixtureApp {
    use semio_framework_plugin::PluginApp;
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<semio_framework_plugin::EditorApp<EpwEditor>, _>(async { semio_framework_plugin::App { definition: create_epw_editor(), examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(document, STDIO_EPW_DOCUMENT_SCHEMA) else { panic!("EPW fixture load must carry the document") };
    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.expect("host loads EPW document");
    app
}

#[semio_framework_async_macros::async_test]
async fn set_cell_reaches_the_document_through_the_registered_native_factory() {
    use semio_framework_plugin::PluginApp;
    let snapshot = crate::standards::energyplus::subsets::any::schema::blank_epw_snapshot();
    let mut app = kit_fixture_holding(&snapshot).await;
    let revision = epw_row_revision(&snapshot.records[0]);
    let args = dsl::DslValue::object([
        ("row".into(), dsl::DslValue::Number(dsl::Number::UInt(0))),
        ("column".into(), dsl::DslValue::Number(dsl::Number::UInt(0))),
        ("revision".into(), dsl::DslValue::String(revision)),
        ("value".into(), dsl::DslValue::String("2026".into())),
    ]);
    let meta = semio_framework_plugin::artifact_app_laws::meta("local");
    app.handle_action("set-cell", Some(&args), &meta).await.expect("set-cell starts");
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.expect("set-cell settles");
    assert_eq!(app.snapshot().expect("EPW snapshot").records[0].year, "2026");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
