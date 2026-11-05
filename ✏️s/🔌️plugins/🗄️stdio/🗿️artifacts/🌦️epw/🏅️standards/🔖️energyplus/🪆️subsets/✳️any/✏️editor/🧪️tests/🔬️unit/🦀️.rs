use crate::editor::epw::modes::edit::windows::main;
use crate::{EpwMutation, EpwSnapshot, STDIO_EPW_DOCUMENT_SCHEMA};
use semio_plugin::ArtifactEditor;
use semio_plugin::Editor;
use crate::editor::epw::{epw_set_cell_emit};
use crate::apply_mutation;
use crate::editor::epw::*;
use semio_framework_test_fixtures::fixture_text;

#[semio_framework_async_macros::async_test]
async fn create_epw_editor_builds_a_definition_for_the_editor_role() {
    let def = create_epw_editor();
    assert_eq!(def.role, semio_plugin::AppRole::Editor);
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
    let args = semio_framework_value::DslValue::object([
        ("row".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(1))),
        ("column".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(2))),
        ("revision".into(), semio_framework_value::DslValue::String("row-revision".into())),
        ("value".into(), semio_framework_value::DslValue::String(String::new())),
    ]);
    let command = <EpwEditor as ArtifactEditor>::command_from_action("set-cell", Some((args).clone()), None).expect("complete cell address");
    assert_eq!(command, semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(EpwEditorCommand::SetCell { row: 1, column: main::EPW_TABLE_COLUMNS[2].into(), revision: "row-revision".into(), value: String::new() }));
    assert!(<EpwEditor as ArtifactEditor>::command_from_action("set-cell", None, None).is_err());
    let missing_value = semio_framework_value::DslValue::object([("row".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(1))), ("column".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(2)))]);
    assert!(<EpwEditor as ArtifactEditor>::command_from_action("set-cell", Some((missing_value).clone()), None).is_err());
}

#[test]
fn set_cell_matches_the_language_neutral_renderer_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/✏️set-cell/🔣️.json")).expect("EPW cell fixture");
    for case in fixture["cases"].as_array().expect("fixture cases") {
        let args: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(&case["args"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("typed fixture args");
        let command = <EpwEditor as ArtifactEditor>::command_from_action("set-cell", Some((args).clone()), None).expect("fixture command");
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

type KitFixtureApp = semio_plugin::VcsArtifactApp<semio_plugin::EditorApp<EpwEditor>>;

async fn kit_fixture_holding(document: &EpwSnapshot) -> (KitFixtureApp, semio_plugin::artifact_app_laws::OriginalFixtureScope) {
    use semio_plugin::PluginApp;
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::new_registered_app::<semio_plugin::EditorApp<EpwEditor>, _>(async { semio_plugin::App { definition: create_epw_editor(), examples: Vec::new() } }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    let semio_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(document, STDIO_EPW_DOCUMENT_SCHEMA) else { panic!("EPW fixture load must carry the document") };
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::load_document(&mut app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut identity, scope.cx()).await).expect("host loads EPW document");
    (app, scope)
}

#[semio_framework_async_macros::async_test]
async fn set_cell_reaches_the_document_through_the_registered_native_factory() {
    use semio_plugin::PluginApp;
    let snapshot = crate::standards::energyplus::subsets::any::schema::blank_epw_snapshot();
    let (mut app, mut scope) = kit_fixture_holding(&snapshot).await;
    let revision = epw_row_revision(&snapshot.records[0]);
    let args = semio_framework_value::DslValue::object([
        ("row".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(0))),
        ("column".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(0))),
        ("revision".into(), semio_framework_value::DslValue::String(revision)),
        ("value".into(), semio_framework_value::DslValue::String("2026".into())),
    ]);
    let meta = semio_plugin::artifact_app_laws::meta("local");
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app, "set-cell", Some(args.clone()), &meta, &mut identity, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await).expect("set-cell starts");
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).expect("set-cell settles");
    assert_eq!(app.snapshot().expect("EPW snapshot").records[0].year, "2026");
    semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("original caller paid app close");
}

semio_plugin::history_edit_acceptance_law!("stdio", crate::editor::epw::EpwEditor, || semio_plugin::App { definition: crate::editor::epw::create_epw_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️energyplus/🪆️subsets/✳️any");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_field() {
    
    use semio_s_artifact_stdio_contract::editing::{SnapshotEditEvent, SnapshotEditingEditor};
    let base = EpwSnapshot { records: vec![Default::default(), Default::default()], ..EpwSnapshot::default() };
    let emit = |event: SnapshotEditEvent| <EpwEditor as SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let header = emit(SnapshotEditEvent::SetValue { path: "/designConditions".into(), value: semio_framework_value::DslValue::String("sizing".into()) }).expect("a header line edit resolves");
    let [mutation @ EpwMutation::SetDesignConditions(_)] = header.artifact_mutations.as_slice() else { panic!("a header line raises its own kind") };
    let mut state = base.clone();
    apply_mutation(&mut state, mutation);
    assert_eq!(state.design_conditions, "sizing");
    let cell = emit(SnapshotEditEvent::SetValue { path: "/records/1/dryBulbTemp".into(), value: semio_framework_value::DslValue::String("21.5".into()) }).expect("a column edit resolves");
    let [mutation @ EpwMutation::SetRecordField(_)] = cell.artifact_mutations.as_slice() else { panic!("a column edit raises the record-field kind") };
    apply_mutation(&mut state, mutation);
    assert_eq!(state.records[1].dry_bulb_temp, "21.5");
    let removed = emit(SnapshotEditEvent::RemoveValue { path: "/records/0".into() }).expect("a record removal resolves");
    assert!(matches!(removed.artifact_mutations.as_slice(), [EpwMutation::RemoveRecord(_)]));
    assert_eq!(emit(SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).err().expect("no kind").code.0, "snapshot-edit.unsupported-path");
}

#[path="📸️native-export/🦀️.rs"]
mod native_export;

const NATURAL_EPW: &str = include_str!("../../../🖼️assets/🎬️demo/🧪️example/🌦️.epw");

#[test]
fn natural_file_open_edit_save_reopen_preserves_headers_and_comments() {
    let opened = <EpwEditor as ArtifactEditor>::decode_natural_file(NATURAL_EPW.as_bytes()).expect("a real EPW opens");
    let comment_lines: Vec<&str> = NATURAL_EPW.lines().filter(|line| line.starts_with("COMMENTS ")).collect();
    assert_eq!(comment_lines, [opened.comments_1.as_str(), opened.comments_2.as_str()]);
    let mut edited = opened.clone();
    apply_mutation(&mut edited, &EpwMutation::SetRecordField(crate::standards::energyplus::subsets::any::schema::mutations::set_record_field::SetRecordField { record_index: 0, field_index: 6, value: "21.5".into() }));
    assert_ne!(edited, opened, "the hourly edit changed the document");
    let saved = <EpwEditor as ArtifactEditor>::encode_natural_file(&edited).expect("save");
    let reopened = <EpwEditor as ArtifactEditor>::decode_natural_file(&saved).expect("reopen");
    assert_eq!(reopened, edited);
    assert_eq!(reopened.comments_1, opened.comments_1);
    assert_eq!(reopened.comments_2, opened.comments_2);
    assert_eq!(reopened.design_conditions, opened.design_conditions);
    let before: Vec<&str> = NATURAL_EPW.split("\r\n").collect();
    let after: Vec<&str> = std::str::from_utf8(&saved).expect("utf8").split("\r\n").collect();
    let changed: Vec<usize> = (0..before.len().min(after.len())).filter(|index| before[*index] != after[*index]).collect();
    assert_eq!(changed, [8], "only the first data record differs");
}

#[test]
fn natural_file_refuses_truncated_and_non_utf8_input_with_a_named_reason() {
    let truncated = NATURAL_EPW.lines().take(3).collect::<Vec<_>>().join("\r\n");
    let error = <EpwEditor as ArtifactEditor>::decode_natural_file(truncated.as_bytes()).expect_err("truncated header is refused");
    assert!(error.to_string().contains("at least 8 header lines") && error.to_string().contains("mindestens 8 Kopfzeilen"), "{error}");
    let error = <EpwEditor as ArtifactEditor>::decode_natural_file(&[0xff, 0xfe, 0x00]).expect_err("binary is refused");
    assert!(error.to_string().contains("UTF-8"), "{error}");
}

#[test]
fn natural_file_matches_the_independent_csv_oracle() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📖️natural-open/🔣️.json")).expect("csv oracle fixture");
    let opened = <EpwEditor as ArtifactEditor>::decode_natural_file(NATURAL_EPW.as_bytes()).expect("a real EPW opens");
    let expected: Vec<Vec<&str>> = oracle["records"].as_array().expect("records").iter().map(|row| row.as_array().expect("row").iter().map(|cell| cell.as_str().expect("cell")).collect()).collect();
    assert_eq!(opened.records.len(), expected.len());
    for (record, row) in opened.records.iter().zip(&expected) {
        assert_eq!(record.fields().iter().map(String::as_str).collect::<Vec<_>>(), *row);
    }
    let saved = String::from_utf8(<EpwEditor as ArtifactEditor>::encode_natural_file(&opened).expect("save")).expect("text");
    let header: Vec<&str> = saved.split("\r\n").take(8).collect();
    let expected_header: Vec<&str> = oracle["headerLines"].as_array().expect("header").iter().map(|line| line.as_str().expect("line")).collect();
    assert_eq!(header, expected_header);
}
