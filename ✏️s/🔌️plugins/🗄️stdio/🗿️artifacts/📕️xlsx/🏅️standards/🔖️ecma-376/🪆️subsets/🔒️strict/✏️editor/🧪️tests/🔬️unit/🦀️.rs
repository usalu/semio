use super::*;

fn fixture() -> XlsxSnapshot {
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxCell;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSheet;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxWorkbook;
    build_minimal_xlsx(XlsxWorkbook { sheets: vec![XlsxSheet { name: "Sheet 1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Boolean(true) }] }], ..Default::default() })
}

#[semio_framework_async_macros::async_test]
async fn strict_editor_declares_its_dialect_and_main_window() {
    let definition = create_xlsx_strict_editor();
    assert_eq!(definition.role, semio_plugin::AppRole::Editor);
    assert_eq!(definition.dialect, XLSX_STRICT_DIALECT.into());
    assert_eq!(<XlsxStrictEditor as ArtifactEditor>::DIALECT, XLSX_STRICT_DIALECT);
    assert!(definition.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn strict_cell_action_preserves_unicode_and_requires_a_complete_address() {
    let fixture = serde_json::json!({ "sheetName": "Tabelle %20", "row": 4, "column": 2, "revision": "0123456789abcdef", "value": "Grüße\n\\s %20" });
    let arguments = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(&fixture.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let command = XlsxStrictEditor::command_from_action("set-cell", &mut Some(arguments), &mut None).unwrap();
    let encoded = protocol::OpBinary::encode_op(&command).unwrap();
    assert_eq!(<semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<XlsxStrictEditorCommand> as protocol::OpBinary>::decode_op(&encoded).unwrap(), command);
    let incomplete = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(r#"{"row":4,"column":2,"value":"x"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(XlsxStrictEditor::command_from_action("set-cell", &mut Some(incomplete), &mut None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn strict_cell_edit_uses_the_canonical_lineage_revision() {
    let snapshot = fixture();
    let address = semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::cell_address::xlsx_cell_address(&snapshot, "Sheet 1", 1, 0).unwrap();
    let command = XlsxStrictEditorCommand::SetCell { sheet_name: "Sheet 1".into(), row: 1, column: 0, revision: address.revision.clone(), value: "false".into() };
    let emit = xlsx_set_cell_emit(&snapshot, &command, None).unwrap();
    assert_eq!(emit.artifact_mutations, vec![XlsxMutation::SetCell(set_cell::SetCell { address, value: XlsxCellValue::Boolean(false), node: None })]);
    assert!(xlsx_set_cell_emit(&snapshot, &XlsxStrictEditorCommand::SetCell { sheet_name: "Sheet 1".into(), row: 1, column: 0, revision: "stale".into(), value: "false".into() }, None).is_err());
    assert!(xlsx_set_cell_emit(&snapshot, &XlsxStrictEditorCommand::SetCell { sheet_name: "Sheet 1".into(), row: 99, column: 0, revision: command_revision(&command), value: "false".into() }, None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn new_workbook_provides_an_editable_sheet_and_roundtrips_through_its_native_codec() {
    let snapshot = XlsxStrictEditor::reference_birth().unwrap()().unwrap();
    let workbook = snapshot.project_workbook().unwrap();
    assert_eq!(workbook.sheets.len(), 1);
    let sheet = &workbook.sheets[0];
    assert!(sheet.cells.is_empty());
    let vacancy = semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::cell_address::xlsx_cell_vacancy_address(&snapshot, &sheet.name, 1, 0).unwrap();
    let command = XlsxStrictEditorCommand::SetCell { sheet_name: sheet.name.clone(), row: 1, column: 0, revision: vacancy.worksheet.revision, value: "=1+2".into() };
    let emitted = xlsx_set_cell_emit(&snapshot, &command, None).unwrap();
    assert_eq!(emitted.artifact_mutations.len(), 1);
    let mut edited = snapshot;
    let outcome = protocol::ColdMutation::diff(&emitted.artifact_mutations[0], &edited);
    edited = protocol::apply_diff_cold(outcome.diff(), &edited).unwrap();
    let bytes = XlsxStrictEditor::encode_natural_file(&edited).unwrap();
    let reopened = XlsxStrictEditor::decode_natural_file(&bytes).unwrap();
    assert_eq!(reopened.project_workbook().unwrap().sheets[0].cells[0].value, XlsxCellValue::Formula { expr: "1+2".into(), cached: None });
}

fn command_revision(command: &XlsxStrictEditorCommand) -> String {
    let XlsxStrictEditorCommand::SetCell { revision, .. } = command;
    revision.clone()
}

semio_plugin::history_edit_acceptance_law!("stdio", super::XlsxStrictEditor, || semio_plugin::App { definition: super::create_xlsx_strict_editor(), examples: Vec::new() }, "../../../🧱️base");

/// 🔌️ The natural-file hooks of this editor pick the controlled decoder above one scheduler step and carry the dialect gate the whole-buffer decoder applies; the stepped paged intake itself is proven in the base subset's `controlled-import` laws, whose `admit_*`/`build_*` functions these hooks only parameterise.
#[test]
fn natural_decode_hooks_select_the_controlled_decoder_and_the_strict_gate() {
    use semio_s_artifact_stdio_xlsx_base::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_xlsx as decode_base_xlsx;
    use semio_plugin::{NaturalFileDecodeOwner, ToolExecutionContract};
    let contract = ToolExecutionContract::resumable(0, 8_192, 4_096, 32 << 20, 7_500, 1, 1);
    assert_eq!(<XlsxStrictEditor as ArtifactEditor>::PAGED_NATURAL_SOURCE_MAXIMUM_BYTES, 16 * 1024 * 1024);
    let input = include_bytes!("../../../../🧱️base/🧪️tests/🧫️fixtures/📏️large-grid/📦️package.xlsx");
    assert!(matches!(XlsxStrictEditor::build_natural_file_decode_owner(input.to_vec(), contract).unwrap(), NaturalFileDecodeOwner::Controlled(_)));
    assert!(matches!(XlsxStrictEditor::build_natural_file_decode_owner(vec![0; 4_096], contract).unwrap(), NaturalFileDecodeOwner::Synchronous(_)));
    let refusal = io::natural_conformance(&decode_base_xlsx(input).unwrap()).expect_err("openpyxl writes Transitional namespaces").to_string();
    assert_eq!(io::decode_xlsx(input).expect_err("the whole-buffer decoder applies the same gate").to_string(), refusal);
}

/// 📄️ Opens `input` through the paged natural route of this editor, exactly as the base subset's laws do, and returns what the controlled decoder published or refused.
fn paged_natural_open(input: &[u8]) -> Result<XlsxSnapshot, String> {
    use semio_framework_value::{RetainedCloneGrant, RetainedCloneStep, RetirementPollExit};
    use semio_plugin::artifact_app_laws::OriginalWorkFixture;
    use semio_plugin::{NaturalFileDecodeOwner, NaturalFileDecodeStep, NaturalFilePagedSource, ToolExecutionContract};
    let contract = ToolExecutionContract::resumable(0, 8_192, 4_096, 32 << 20, 7_500, 1, 1);
    let stock = OriginalWorkFixture::stock();
    let mut scope = OriginalWorkFixture::hosted(1 << 44);
    let mut source = Some(NaturalFilePagedSource::retain(semio_framework_value::paged::PagedBytes::try_from_slice(input).unwrap()));
    let mut owner = None;
    let denied = RetainedCloneGrant { maximum_items: 0, ..stock };
    assert!(!scope.turn(denied, |cx| <XlsxStrictEditor as ArtifactEditor>::admit_paged_natural_file_decode_owner(&mut source, contract, &mut owner, cx)).unwrap());
    assert!(scope.turn(stock, |cx| <XlsxStrictEditor as ArtifactEditor>::admit_paged_natural_file_decode_owner(&mut source, contract, &mut owner, cx)).unwrap());
    let Some(NaturalFileDecodeOwner::Controlled(mut decoder)) = owner else { panic!("the workbook needs the controlled decoder") };
    let mut turns = 0;
    let outcome = loop {
        turns += 1;
        assert!(turns < 100_000, "the controlled decoder finishes");
        match scope.turn(stock, |cx| Ok(decoder.advance(cx))).unwrap() {
            Ok(NaturalFileDecodeStep::Complete { snapshot, .. }) => break Ok(snapshot),
            Ok(_) => {}
            Err(error) => break Err(error.to_string()),
        }
    };
    decoder.begin_close();
    for _ in 0..200_000 {
        if decoder.terminal_is_empty() { break; }
        scope.turn(stock, |cx| {
            let demand = decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
            semio_plugin::__job::close_original_effect_step(cx, demand, |effect| {
                let result = decoder.close_chained(effect).map(|step| RetainedCloneStep::Progress(step.progress().unwrap_or_default())).map_err(Into::into);
                RetirementPollExit::settle(effect, result)
            })?;
            Ok(())
        }).unwrap();
    }
    drop(decoder);
    scope.finish();
    outcome
}

#[test]
fn paged_natural_route_refuses_a_transitional_workbook_with_the_strict_gate_of_the_whole_buffer_decoder() {
    let input = include_bytes!("../../../../🧱️base/🧪️tests/🧫️fixtures/📏️large-grid/📦️package.xlsx");
    let expected = io::decode_xlsx(input).expect_err("openpyxl writes the transitional namespaces").to_string();
    let refusal = paged_natural_open(input).expect_err("the strict gate refuses the paged route too");
    assert!(refusal.contains(&expected), "{refusal} must carry {expected}");
}
