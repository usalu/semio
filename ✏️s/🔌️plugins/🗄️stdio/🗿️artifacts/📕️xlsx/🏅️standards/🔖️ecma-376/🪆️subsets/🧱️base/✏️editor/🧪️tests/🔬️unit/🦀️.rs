use super::*;
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxCell;

#[semio_framework_async_macros::async_test]
async fn create_xlsx_editor_builds_a_definition_for_the_editor_role() {
    let def = create_xlsx_editor();
    assert_eq!(def.role, semio_plugin::AppRole::Editor);
    assert_eq!(def.dialect, XLSX_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn new_workbook_provides_a_worksheet_readable_by_calamine() {
    use calamine::Reader;
    let snapshot = XlsxEditor::reference_birth().unwrap()().unwrap();
    let workbook = snapshot.project_workbook().unwrap();
    let bytes = XlsxEditor::encode_natural_file(&snapshot).unwrap();
    let reference: calamine::Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(bytes)).unwrap();
    assert_eq!(workbook.sheets.len(), 1);
    assert!(workbook.sheets[0].cells.is_empty());
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📊️sheet-grid/🔣️.json")).unwrap();
    let expected_name = fixture["newWorkbook"]["sheetName"].as_str().unwrap();
    assert_eq!(workbook.sheets[0].name, expected_name);
    assert_eq!(reference.sheet_names(), vec![expected_name.to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<XlsxEditor as ArtifactEditor>::DIALECT, XLSX_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_xlsx_editor();
    assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn render_value_resolves_shared_strings_and_exposes_editable_formula_source() {
    let strings = vec!["hello".to_string()];
    assert_eq!(render_xlsx_cell_value(&XlsxCellValue::SharedString(0), &strings, None), "hello");
    assert_eq!(render_xlsx_cell_value(&XlsxCellValue::SharedString(9), &strings, None), "#9");
    assert_eq!(render_xlsx_cell_value(&XlsxCellValue::Formula { expr: "SUM(A1:A2)".into(), cached: Some(Box::new(XlsxCellValue::Number(3.0))) }, &strings, None), "=SUM(A1:A2)");
    assert_eq!(render_xlsx_cell_value(&XlsxCellValue::Empty, &strings, None), "");
}

#[semio_framework_async_macros::async_test]
async fn parse_cell_value_detects_bool_and_number_before_falling_back_to_inline_string() {
    assert_eq!(parse_xlsx_cell_value("", None), XlsxCellValue::Empty);
    assert_eq!(parse_xlsx_cell_value("=SUM(A1:A2)", None), XlsxCellValue::Formula { expr: "SUM(A1:A2)".into(), cached: None });
    assert_eq!(parse_xlsx_cell_value("'=literal", None), XlsxCellValue::InlineString("=literal".into()));
    assert_eq!(parse_xlsx_cell_value("true", None), XlsxCellValue::Boolean(true));
    assert_eq!(parse_xlsx_cell_value("false", None), XlsxCellValue::Boolean(false));
    assert_eq!(parse_xlsx_cell_value("3.5", None), XlsxCellValue::Number(3.5));
    assert_eq!(parse_xlsx_cell_value("NaN", None), XlsxCellValue::InlineString("NaN".into()));
    assert_eq!(parse_xlsx_cell_value("hello", None), XlsxCellValue::InlineString("hello".into()));
}

#[semio_framework_async_macros::async_test]
async fn stable_cell_action_fixture_roundtrips_exact_text_through_parser_and_binary_codec() {
    const FIXTURE: &str = include_str!("../🧫️fixtures/📊️stable-cell-edit/🔣️.json");
    let oracle: serde_json::Value = serde_json::from_str(FIXTURE).expect("third-party JSON oracle parses the action fixture");
    let args = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(FIXTURE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("DSL JSON parses the action fixture");
    let command = XlsxEditor::command_from_action("set-cell", &mut Some(args), &mut None).expect("stable cell action parses");
    let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(native) = &command else { panic!("expected native command") };
    assert_eq!(
        native,
        &XlsxEditorCommand::SetCell {
            sheet_name: oracle["sheetName"].as_str().unwrap().to_string(),
            row: oracle["row"].as_u64().unwrap() as u32,
            column: oracle["column"].as_u64().unwrap() as u32,
            revision: oracle["revision"].as_str().unwrap().to_string(),
            value: oracle["value"].as_str().unwrap().to_string(),
        }
    );
    let encoded = protocol::OpBinary::encode_op(&command).expect("encode command");
    let decoded = <semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<XlsxEditorCommand> as protocol::OpBinary>::decode_op(&encoded).expect("decode command");
    assert_eq!(decoded, command);
}

#[semio_framework_async_macros::async_test]
async fn stable_cell_edit_targets_identity_and_rejects_a_stale_revision() {
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxCell;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSheet;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxWorkbook;
    let snapshot = semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet 1".into(), cells: vec![XlsxCell { row: 41, col: 7, value: XlsxCellValue::Number(1.0) }] }],
        ..Default::default()
    });
    let address = semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::cell_address::xlsx_cell_address(&snapshot, "Sheet 1", 41, 7).unwrap();
    let command = XlsxEditorCommand::SetCell { sheet_name: "Sheet 1".into(), row: 41, column: 7, revision: address.revision.clone(), value: "2".into() };
    let emit = xlsx_set_cell_emit(&snapshot, &command, None).expect("matching revision emits a mutation");
    assert_eq!(emit.artifact_mutations, vec![XlsxMutation::SetCell(set_cell::SetCell { address, value: XlsxCellValue::Number(2.0), node: None })]);
    let stale = XlsxEditorCommand::SetCell { sheet_name: "Sheet 1".into(), row: 41, column: 7, revision: "stale".into(), value: "2".into() };
    assert!(xlsx_set_cell_emit(&snapshot, &stale, None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn blank_cell_command_inserts_canonical_xml_saves_for_calamine_and_inverts_exactly() {
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSheet;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxWorkbook;
    use crate::standards::v_ecma_376::subsets::base::io::export::serializers::encode_xlsx;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx;
    use semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::apply_xlsx_mutation;
    use semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::cell_address::xlsx_cell_vacancy_address;
    use semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::insert_cell;
    use calamine::{Data, Reader};
    use protocol::{ColdMutation, Mutation};
    let base = build_minimal_xlsx(XlsxWorkbook { sheets: vec![XlsxSheet { name: "Blank".into(), cells: Vec::new() }], ..Default::default() });
    let vacancy = xlsx_cell_vacancy_address(&base, "Blank", 1, 0).unwrap();
    let command = XlsxEditorCommand::SetCell { sheet_name: "Blank".into(), row: 1, column: 0, revision: vacancy.worksheet.revision.clone(), value: "42".into() };
    let emitted = xlsx_set_cell_emit(&base, &command, None).unwrap();
    let mutation = XlsxMutation::InsertCell(insert_cell::InsertCell { address: vacancy, value: XlsxCellValue::Number(42.0), node: None });
    assert_eq!(emitted.artifact_mutations, vec![mutation.clone()]);
    let mut edited = base.clone();
    apply_xlsx_mutation(&mut edited, &mutation);
    assert_eq!(edited.project_workbook().unwrap().sheets[0].cells[0].value, XlsxCellValue::Number(42.0));
    let mut reference: calamine::Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(encode_xlsx(&edited).unwrap())).expect("Calamine opens canonical blank-cell save");
    assert!(matches!(reference.worksheet_range("Blank").unwrap().get_value((0, 0)), Some(Data::Float(value)) if *value == 42.0));
    for inverse in <XlsxMutation as ColdMutation<XlsxSnapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture") {
        apply_xlsx_mutation(&mut edited, &inverse);
    }
    assert_eq!(edited, base, "vacancy insertion inverse restores every authoritative OPC and XML field");
}

#[semio_framework_async_macros::async_test]
async fn sparse_row_insertion_preserves_formula_neighbors_and_stales_prior_vacancies() {
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxCell;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSheet;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxWorkbook;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx;
    use semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::apply_xlsx_mutation;
    use semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::cell_address::resolve_xlsx_cell_vacancy_address;
    use semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::cell_address::xlsx_cell_vacancy_address;
    use semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::insert_cell;
    let mut snapshot = build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet {
            name: "Sparse".into(),
            cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Formula { expr: "2+3".into(), cached: Some(Box::new(XlsxCellValue::Number(5.0))) } }, XlsxCell { row: 1, col: 2, value: XlsxCellValue::InlineString("right".into()) }],
        }],
        ..Default::default()
    });
    let old_vacancy = xlsx_cell_vacancy_address(&snapshot, "Sparse", 2, 0).unwrap();
    let middle = xlsx_cell_vacancy_address(&snapshot, "Sparse", 1, 1).unwrap();
    apply_xlsx_mutation(&mut snapshot, &XlsxMutation::InsertCell(insert_cell::InsertCell { address: middle, value: XlsxCellValue::InlineString("middle".into()), node: None }));
    let projected = snapshot.project_workbook().unwrap();
    assert_eq!(projected.sheets[0].cells.iter().map(|cell| (cell.row, cell.col)).collect::<Vec<_>>(), [(1, 0), (1, 1), (1, 2)]);
    assert_eq!(projected.sheets[0].cells[0].value, XlsxCellValue::Formula { expr: "2+3".into(), cached: Some(Box::new(XlsxCellValue::Number(5.0))) });
    assert!(resolve_xlsx_cell_vacancy_address(&snapshot, &old_vacancy).is_err(), "a structural edit invalidates a prior worksheet vacancy revision");
}

#[semio_framework_async_macros::async_test]
async fn unchanged_cell_drafts_preserve_types_and_cached_values() {
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxCell;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSheet;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxWorkbook;
    const FIXTURE: &str = include_str!("../../../🧪️tests/🧫️fixtures/✍️unchanged-cell-draft/🔣️.json");
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
    let shared_strings: Vec<String> = serde_json::from_value(fixture["sharedStrings"].clone()).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let original: XlsxCellValue = semio_framework_pack_json::from_json_str(&case["value"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let snapshot = semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
            sheets: vec![XlsxSheet { name: "Sheet".into(), cells: vec![XlsxCell { row: 1, col: 0, value: original.clone() }] }],
            shared_strings: shared_strings.clone(),
        });
        let revision = semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::cell_address::xlsx_cell_address(&snapshot, "Sheet", 1, 0).unwrap().revision;
        let command = XlsxEditorCommand::SetCell { sheet_name: "Sheet".into(), row: 1, column: 0, revision, value: case["draft"].as_str().unwrap().into() };
        let emitted = xlsx_set_cell_emit(&snapshot, &command, None).unwrap();
        assert!(emitted.artifact_mutations.is_empty(), "unchanged draft must not rewrite {}", case["id"]);
        assert_eq!(snapshot.project_workbook().unwrap().sheets[0].cells[0].value, original);
    }
    let conflict = &fixture["sharedStringConflict"];
    let index = conflict["index"].as_u64().unwrap() as usize;
    let value = XlsxCellValue::SharedString(index);
    let mut snapshot = semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook { sheets: vec![XlsxSheet { name: "Sheet".into(), cells: vec![XlsxCell { row: 1, col: 0, value }] }], shared_strings });
    let revision = semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::cell_address::xlsx_cell_address(&snapshot, "Sheet", 1, 0).unwrap().revision;
    let command = XlsxEditorCommand::SetCell { sheet_name: "Sheet".into(), row: 1, column: 0, revision, value: conflict["draft"].as_str().unwrap().into() };
    semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::apply_xlsx_mutation(
        &mut snapshot,
        &XlsxMutation::SetSharedString(semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::set_shared_string::SetSharedString { index: 1, value: "unrelated change".into(), node: None }),
    );
    assert!(xlsx_set_cell_emit(&snapshot, &command, None).is_ok(), "unrelated shared strings do not invalidate this cell");
    semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::apply_xlsx_mutation(
        &mut snapshot,
        &XlsxMutation::SetSharedString(semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::set_shared_string::SetSharedString { index, value: conflict["replacement"].as_str().unwrap().into(), node: None }),
    );
    assert!(xlsx_set_cell_emit(&snapshot, &command, None).is_err(), "referenced text changes invalidate the draft");
}

#[semio_framework_async_macros::async_test]
async fn unchanged_cell_draft_fixture_matches_independent_spreadsheet_values_and_formulas() {
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxCell;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSheet;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxWorkbook;
    use crate::standards::v_ecma_376::subsets::base::io::export::serializers::encode_xlsx;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx;
    use calamine::{Data, Reader};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/✍️unchanged-cell-draft/🔣️.json")).unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    let cells =
        cases.iter().enumerate().map(|(index, case)| XlsxCell { row: index as u32 + 1, col: 0, value: semio_framework_pack_json::from_json_str(&case["value"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap() }).collect();
    let snapshot = build_minimal_xlsx(XlsxWorkbook { sheets: vec![XlsxSheet { name: "Sheet".into(), cells }], shared_strings: serde_json::from_value(fixture["sharedStrings"].clone()).unwrap() });
    let mut reference: calamine::Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(encode_xlsx(&snapshot).unwrap())).unwrap();
    let values = reference.worksheet_range("Sheet").unwrap();
    let formulas = reference.worksheet_formula("Sheet").unwrap();
    for (index, case) in cases.iter().enumerate() {
        let value = values.get_value((index as u32, 0)).unwrap_or(&Data::Empty);
        assert_eq!(value.to_string(), case["reference"].as_str().unwrap(), "{}", case["id"]);
        let kind = match value {
            Data::String(_) => "string",
            Data::Float(_) | Data::Int(_) => "number",
            Data::Bool(_) => "boolean",
            Data::Empty => "empty",
            _ => panic!("unexpected independent cell type for {}", case["id"]),
        };
        assert_eq!(kind, case["referenceKind"].as_str().unwrap(), "{}", case["id"]);
        assert_eq!(formulas.get_value((index as u32, 0)).map(String::as_str).unwrap_or_default(), case["formula"].as_str().unwrap_or_default());
    }
}

fn canonical_save_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🧬️canonical-xml-save/🔣️.json")).expect("neutral canonical save fixture")
}

fn canonical_fixture_zip(case: &serde_json::Value) -> Vec<u8> {
    use std::io::{Cursor, Write};
    use zip::write::SimpleFileOptions;
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for part in case["parts"].as_array().unwrap() {
        writer.start_file(part["path"].as_str().unwrap(), options).unwrap();
        writer.write_all(part["text"].as_str().unwrap().as_bytes()).unwrap();
    }
    for part in case["binaryParts"].as_array().unwrap() {
        writer.start_file(part["path"].as_str().unwrap(), options).unwrap();
        let bytes: Vec<u8> = serde_json::from_value(part["bytes"].clone()).unwrap();
        writer.write_all(&bytes).unwrap();
    }
    writer.finish().unwrap().into_inner()
}

fn independent_xml_events(text: &str) -> Vec<String> {
    use quick_xml::{events::Event, Reader, XmlVersion};
    let mut reader = Reader::from_str(text);
    reader.config_mut().expand_empty_elements = true;
    let mut events = Vec::new();
    loop {
        let event = match reader.read_event().expect("independent XML parser") {
            Event::Start(element) => {
                let attrs: Vec<_> = element
                    .attributes()
                    .map(|attribute| {
                        let attribute = attribute.unwrap();
                        (attribute.key.0.to_string(), attribute.normalized_value(XmlVersion::Explicit1_0).unwrap().into_owned())
                    })
                    .collect();
                format!("start:{}:{}", element.name().0, serde_json::to_string(&attrs).unwrap())
            }
            Event::End(element) => format!("end:{}", element.name().0),
            Event::Text(text) => format!("text:{}", text.xml10_content()),
            Event::CData(text) => format!("cdata:{}", text.xml10_content()),
            Event::Comment(text) => format!("comment:{}", &*text),
            Event::PI(text) => format!("pi:{}", &*text),
            Event::Decl(text) => format!("declaration:{}", &*text),
            Event::DocType(text) => format!("doctype:{}", &*text),
            Event::GeneralRef(text) => format!("reference:{}", &*text),
            Event::Eof => break,
            Event::Empty(_) => unreachable!("empty elements are expanded"),
        };
        events.push(event);
    }
    events
}

fn assert_independent_canonical_package(bytes: &[u8], case: &serde_json::Value, edited: bool) {
    use std::io::{Cursor, Read};
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("independent ZIP reader");
    let mut actual_names: Vec<String> = archive.file_names().map(str::to_string).collect();
    let mut expected_names: Vec<String> = case["parts"].as_array().unwrap().iter().chain(case["binaryParts"].as_array().unwrap()).map(|part| part["path"].as_str().unwrap().to_string()).collect();
    actual_names.sort();
    expected_names.sort();
    assert_eq!(actual_names, expected_names, "{} preserves every part identity", case["id"]);
    for part in case["parts"].as_array().unwrap() {
        let path = part["path"].as_str().unwrap();
        let mut actual = String::new();
        archive.by_name(path).unwrap().read_to_string(&mut actual).unwrap();
        let expected = if edited && path == case["sheetPath"].as_str().unwrap() { case["expectedSheetXml"].as_str().unwrap() } else { part["text"].as_str().unwrap() };
        assert_eq!(independent_xml_events(&actual), independent_xml_events(expected), "{} preserves XML field/order at {path}", case["id"]);
    }
    for part in case["binaryParts"].as_array().unwrap() {
        let mut actual = Vec::new();
        archive.by_name(part["path"].as_str().unwrap()).unwrap().read_to_end(&mut actual).unwrap();
        assert_eq!(actual, serde_json::from_value::<Vec<u8>>(part["bytes"].clone()).unwrap());
    }
}

/// 🔮️ The package calamine can read: every part verbatim except cell-level `extLst`, which calamine 0.36 refuses
/// (`cells_reader.rs`: "v, f, or is") although ECMA-376 Part 1 §18.3.1.4 allows it — the values it answers are unaffected.
fn calamine_value_projection(bytes: &[u8]) -> Vec<u8> {
    use std::io::{Cursor, Read, Write};
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("package ZIP");
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).expect("package entry");
        let name = entry.name().to_string();
        let mut content = Vec::new();
        entry.read_to_end(&mut content).expect("package entry bytes");
        if let Ok(mut text) = String::from_utf8(content.clone()) {
            for prefix in ["", "s:"] {
                let (open, close, cell_end) = (format!("<{prefix}extLst>"), format!("</{prefix}extLst>"), format!("</{prefix}c>"));
                let mut from = 0;
                while let Some(found) = text[from..].find(&open) {
                    let start = from + found;
                    let Some(end) = text[start..].find(&close).map(|end| start + end + close.len()) else { break };
                    if text[end..].starts_with(&cell_end) {
                        text.replace_range(start..end, "");
                        from = start;
                    } else {
                        from = end;
                    }
                }
            }
            content = text.into_bytes();
        }
        writer.start_file(name, options).expect("projection entry");
        writer.write_all(&content).expect("projection bytes");
    }
    writer.finish().expect("projection ZIP").into_inner()
}

fn assert_independent_spreadsheet_values(bytes: Vec<u8>, case: &serde_json::Value, edited: bool) {
    use calamine::{Data, Reader};
    let mut reference: calamine::Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(calamine_value_projection(&bytes))).expect("Calamine opens neutral package");
    assert_eq!(reference.sheet_names(), ["Data"]);
    let values = reference.worksheet_range("Data").unwrap();
    let formulas = reference.worksheet_formula("Data").unwrap();
    for (column, expected) in case["expectedValues"].as_array().unwrap().iter().enumerate() {
        let value = values.get_value((0, column as u32)).unwrap_or(&Data::Empty);
        let kind = match value {
            Data::Int(_) | Data::Float(_) => "number",
            Data::String(_) => "string",
            Data::Bool(_) => "boolean",
            Data::Error(_) => "error",
            other => panic!("unexpected independent fixture value {other:?}"),
        };
        assert_eq!(kind, expected["kind"].as_str().unwrap(), "{} {}", case["id"], expected["address"]);
        assert_eq!(value.to_string(), expected[if edited { "after" } else { "before" }].as_str().unwrap(), "{} {}", case["id"], expected["address"]);
        assert_eq!(formulas.get_value((0, column as u32)).map(String::as_str).unwrap_or_default(), expected["formula"].as_str().unwrap());
    }
}

#[semio_framework_async_macros::async_test]
async fn canonical_save_fixtures_are_valid_independent_workbooks() {
    for case in canonical_save_fixture()["cases"].as_array().unwrap() {
        let bytes = canonical_fixture_zip(case);
        assert_independent_canonical_package(&bytes, case, false);
        if case["calamine"].as_bool().unwrap() {
            assert_independent_spreadsheet_values(bytes, case, false);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn canonical_xlsx_no_op_save_preserves_all_xml_fields_and_custom_part_paths() {
    use crate::standards::v_ecma_376::subsets::base::io::{export::serializers::encode_xlsx, import::deserializers::decode_xlsx};
    for case in canonical_save_fixture()["cases"].as_array().unwrap() {
        let original = canonical_fixture_zip(case);
        let snapshot = decode_xlsx(&original).unwrap_or_else(|error| panic!("{} import: {error}", case["id"]));
        let encoded = encode_xlsx(&snapshot).unwrap_or_else(|error| panic!("{} export: {error}", case["id"]));
        assert_independent_canonical_package(&encoded, case, false);
        if case["calamine"].as_bool().unwrap() {
            assert_independent_spreadsheet_values(encoded, case, false);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn natural_file_route_preserves_canonical_xlsx_xml_and_reopens() {
    let _arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
    use crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_xlsx;
    use semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::apply_xlsx_mutation;
    use semio_plugin::app::{Media, MediaPayload};
    use semio_plugin::{BuiltNode, Component, Effect, HistoryView, MediaClass, MediaForm, MediaType, TreeWindows, Trigger, UiValue};
    use semio_ui_locale::Locale;

    fn cell_commit(node: &BuiltNode) -> Option<&semio_plugin::plugin_app_close_prelude::UiMap> {
        let binding = node.bindings.iter().find(|binding| binding.trigger == Trigger::Commit && binding.action.name.as_str() == "set-cell");
        if let (Component::Input(_), Some(UiValue::Map(arguments))) = (&node.component, binding.and_then(|binding| binding.args.as_ref())) {
            return Some(arguments);
        }
        node.children.iter().find_map(cell_commit)
    }

    for case in canonical_save_fixture()["cases"].as_array().unwrap() {
        let codec = XlsxEditor::natural_file_codec().unwrap();
        let media = Media {
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            payload: MediaPayload::Intrinsic { schema: codec.format_kind.into(), value: semio_framework_value::DslValue::Bytes(canonical_fixture_zip(case)) },
        };
        let current = XlsxEditor::reference_birth().unwrap()().unwrap();
        let imported = XlsxEditor::import_media(semio_plugin::NATURAL_FILE_PORT, &media, &ArtifactView::new(&current, &HistoryView::empty()))
            .unwrap_or_else(|error| panic!("{} import: {error}", case["id"]));
        assert!(imported.artifact_mutations.is_empty());
        let [Effect::LoadDocument { pack, .. }] = imported.effects.as_slice() else { panic!("expected a single document load effect"); };
        let mut snapshot = <XlsxSnapshot as pack::snapshot::ArtifactPack>::decode_pack(pack).unwrap();
        let grid = main::render(&snapshot, Locale::En, &TreeWindows::unhosted(), semio_plugin::UiPublicationRevision(23)).unwrap();
        let mut arguments = semio_plugin::artifact_app_laws::observe_and_retire_fixture_tree(semio_plugin::built_to_component_tree(grid), |grid| {
            cell_commit(grid).expect("A1 commit binding").iter().map(|(key, value)| {
                let value = match value {
                    UiValue::Text(text) => serde_json::json!(text.as_str()),
                    UiValue::Number(number) => serde_json::json!(number as u32),
                    _ => panic!("unexpected argument type"),
                };
                (key.as_str().to_string(), value)
            }).collect::<serde_json::Map<_, _>>()
        });
        assert_eq!(arguments["sheetName"], "Data");
        assert_eq!(arguments["row"], 1);
        assert_eq!(arguments["column"], 0);
        arguments.insert("value".into(), serde_json::json!("99.25"));
        let args = semio_framework_pack_json::from_json_str(&serde_json::Value::Object(arguments).to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let command = XlsxEditor::command_from_action("set-cell", &mut Some(args), &mut None).unwrap();
        let emit = semio_plugin::artifact_app_laws::reduce_editor_command::<XlsxEditor>(&command, &snapshot).unwrap();
        let [mutation] = emit.artifact_mutations.as_slice() else { panic!("expected sole A1 edit"); };
        apply_xlsx_mutation(&mut snapshot, mutation);
        let encoded = <XlsxEditor as ArtifactEditor>::encode_natural_file(&snapshot).unwrap_or_else(|error| panic!("{} natural export: {error}", case["id"]));
        assert_independent_canonical_package(&encoded, case, true);
        let independently_reopened = decode_xlsx(&encoded).unwrap_or_else(|error| panic!("{} independent reopen: {error}", case["id"]));
        assert_eq!(independently_reopened.project_workbook().unwrap().sheets[0].cells[0].value, XlsxCellValue::Number(99.25));
        let reopened = <XlsxEditor as ArtifactEditor>::decode_natural_file(&encoded).unwrap_or_else(|error| panic!("{} natural reopen: {error}", case["id"]));
        assert_eq!(reopened.project_workbook().unwrap().sheets[0].cells[0].value, XlsxCellValue::Number(99.25));
        if case["calamine"].as_bool().unwrap() {
            assert_independent_spreadsheet_values(encoded, case, true);
        }
    }
}

semio_plugin::history_edit_acceptance_law!("stdio", super::XlsxEditor, || semio_plugin::App { definition: super::create_xlsx_editor(), examples: Vec::new() }, "../..");

#[test]
fn original_xlsx_replacement_projects_the_exact_pack_document() {
    use pack::record::BorrowedPackDocumentCursor;
    use semio_framework_value::{AuthoringWallet, RetainedCloneGrant};
    use semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::rename_sheet::RenameSheet;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📊️stable-cell-edit/🔣️.json")).unwrap();
    let mutation = XlsxMutation::RenameSheet(RenameSheet {
        name: fixture["sheetName"].as_str().unwrap().into(),
        new_name: fixture["value"].as_str().unwrap().into(),
    });
    let mut authored = protocol::OpBinary::encode_op(&mutation).unwrap();
    let codec = XlsxMutation::install_original_history_wire(store::os_store::ArtifactOperationWire::semantic::<XlsxSnapshot>());
    let source = codec.original_pack_value(&mutation).expect("installed Pack Value projection");
    let mut cursor = BorrowedPackDocumentCursor::with_maximum_file_len(authored.len());
    let grant = RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: 32768,
        maximum_capacity_bytes: 16 * 1024 * 1024,
        maximum_release_bytes: 16 * 1024 * 1024,
        maximum_depth: 64,
    };
    let mut wallet = AuthoringWallet::new(grant, 1 << 38);
    let mut projected = Vec::with_capacity(authored.len());
    for _ in 0..262144 {
        let mut byte = [0];
        let turn = wallet.run(grant, 1 << 20, |effect| cursor.advance(source, store::pack_rt::wire_value_spec(), &mut byte, effect)).unwrap().unwrap();
        let step = turn.output.expect("bounded replacement should encode");
        assert!(step.progress.fits(grant));
        projected.extend_from_slice(&byte[..step.written_bytes]);
        if step.complete { break; }
    }
    assert!(cursor.is_complete());
    for _ in 0..262144 {
        if cursor.terminal_is_empty() { break; }
        let turn = wallet.run(grant, 1 << 20, |effect| cursor.close(effect)).unwrap().unwrap();
        assert!(turn.output.expect("bounded replacement projection should retire").progress.fits(grant));
    }
    assert!(cursor.terminal_is_empty());
    assert_eq!(projected, authored);
    assert_eq!(<XlsxMutation as protocol::OpBinary>::decode_op(&projected).unwrap(), mutation);
    *authored.last_mut().unwrap() ^= 1;
    assert!(<XlsxMutation as protocol::OpBinary>::decode_op(&authored).is_err());
}

#[test]
fn generated_edit_proofs_name_the_factory_and_keep_exact_editor_type() {
    use semio_s_artifact_stdio_contract::editing::{BoundedNativeEditToolJobFactory, SnapshotEditToolJobFactory};
    use std::any::{type_name, TypeId};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📊️sheet-grid/🔣️.json")).unwrap();
    let proofs = <XlsxEditor as ArtifactEditor>::bounded_first_step_tool_proofs();
    for row in fixture["genericToolProofs"].as_array().unwrap() {
        let tool = row["tool"].as_str().unwrap();
        let proof = proofs.iter().find(|proof| proof.tool_id() == tool).unwrap();
        assert_eq!(proof.factory, row["factory"].as_str().unwrap());
        let (type_id, name) = match tool {
            "setSnapshotValue" => (TypeId::of::<SnapshotEditToolJobFactory<XlsxEditor>>(), type_name::<SnapshotEditToolJobFactory<XlsxEditor>>()),
            "set-cell" => (TypeId::of::<BoundedNativeEditToolJobFactory<XlsxEditor>>(), type_name::<BoundedNativeEditToolJobFactory<XlsxEditor>>()),
            _ => panic!("unexpected fixture tool"),
        };
        assert_eq!(proof.factory_type_id, Some(type_id));
        assert_eq!(proof.factory_type_name, Some(name));
    }
}

fn notation_locale(name: &str) -> Option<Locale> {
    match name {
        "en" => Some(Locale::En),
        "de" => Some(Locale::De),
        _ => None,
    }
}

#[semio_framework_async_macros::async_test]
async fn number_notation_fixture_parses_and_displays_by_locale_and_agrees_with_serde_json() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📊️sheet-grid/🔣️.json")).unwrap();
    for case in fixture["numberNotation"]["cases"].as_array().unwrap() {
        let locale = notation_locale(case["locale"].as_str().unwrap());
        let text = case["text"].as_str().unwrap();
        let parsed = parse_xlsx_cell_value(text, locale);
        match case["canonical"].as_str() {
            Some(canonical) => {
                let oracle: f64 = serde_json::from_str(canonical).unwrap();
                assert_eq!(parsed, XlsxCellValue::Number(oracle), "{text} in {locale:?}");
                assert_eq!(render_xlsx_cell_value(&parsed, &[], locale), case["display"].as_str().unwrap(), "{text} in {locale:?}");
            }
            None => assert_eq!(parsed, XlsxCellValue::InlineString(text.into()), "{text} in {locale:?}"),
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn german_decimal_comma_edit_stores_the_canonical_number() {
    let snapshot = semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet 1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(2.0) }] }],
        ..Default::default()
    });
    let address = xlsx_cell_address(&snapshot, "Sheet 1", 1, 0).unwrap();
    let command = XlsxEditorCommand::SetCell { sheet_name: "Sheet 1".into(), row: 1, column: 0, revision: address.revision.clone(), value: "1,5".into() };
    let german = xlsx_set_cell_emit(&snapshot, &command, Some(Locale::De)).unwrap();
    assert_eq!(german.artifact_mutations, vec![XlsxMutation::SetCell(set_cell::SetCell { address: address.clone(), value: XlsxCellValue::Number(1.5), node: None })]);
    let english = xlsx_set_cell_emit(&snapshot, &command, Some(Locale::En)).unwrap();
    assert_eq!(english.artifact_mutations, vec![XlsxMutation::SetCell(set_cell::SetCell { address, value: XlsxCellValue::InlineString("1,5".into()), node: None })]);
}

#[semio_framework_async_macros::async_test]
async fn canonical_cell_text_fixture_agrees_with_the_python_float_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🌐️canonical-cell-text/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let (locale, text, canonical) = (case["locale"].as_str().unwrap(), case["text"].as_str().unwrap(), case["canonical"].as_str().unwrap());
        assert_eq!(xlsx_canonical_cell_text(text, locale), canonical, "{text:?} typed in {locale:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn a_german_cell_draft_reduces_to_the_canonical_number_on_the_locale_free_native_path() {
    let snapshot = semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet 1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(2.0) }] }],
        ..Default::default()
    });
    let address = xlsx_cell_address(&snapshot, "Sheet 1", 1, 0).unwrap();
    for (locale, typed, expected) in [("de", "1,5", XlsxCellValue::Number(1.5)), ("de", "1.5", XlsxCellValue::InlineString("1.5".into())), ("en", "1,5", XlsxCellValue::InlineString("1,5".into())), ("", "1.5", XlsxCellValue::Number(1.5))] {
        let json = serde_json::json!({ "sheetName": "Sheet 1", "row": 1, "column": 0, "revision": address.revision, "value": typed, "locale": locale }).to_string();
        let args = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let command = XlsxEditor::command_from_action("set-cell", &mut Some(args), &mut None).unwrap();
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(native) = &command else { panic!("expected native command") };
        let emitted = xlsx_set_cell_emit(&snapshot, native, None).unwrap();
        assert_eq!(emitted.artifact_mutations, vec![XlsxMutation::SetCell(set_cell::SetCell { address: address.clone(), value: expected, node: None })], "{typed:?} typed in {locale:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn unchanged_localized_number_draft_emits_nothing() {
    let snapshot = semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet 1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.5) }] }],
        ..Default::default()
    });
    let address = xlsx_cell_address(&snapshot, "Sheet 1", 1, 0).unwrap();
    let command = XlsxEditorCommand::SetCell { sheet_name: "Sheet 1".into(), row: 1, column: 0, revision: address.revision, value: "1,5".into() };
    assert!(xlsx_set_cell_emit(&snapshot, &command, Some(Locale::De)).unwrap().artifact_mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn labels_resolve_in_both_languages_from_the_catalog() {
    assert_eq!(xlsx_editor_labels(Locale::En).row.as_str(), "Row");
    assert_eq!(xlsx_editor_labels(Locale::De).row.as_str(), "Zeile");
    assert_eq!(xlsx_editor_labels(Locale::En).column.as_str(), "Column");
    assert_eq!(xlsx_editor_labels(Locale::De).column.as_str(), "Spalte");
    assert_eq!(xlsx_editor_labels(Locale::En).unknown_body.fill(&[("body", "x")]).as_str(), "Unknown body: x");
    assert_eq!(xlsx_editor_labels(Locale::De).unknown_body.fill(&[("body", "x")]).as_str(), "Unbekannter Bereich: x");
}

#[semio_framework_async_macros::async_test]
async fn edit_layout_titles_come_from_the_localized_window_kind_label() {
    let semio_plugin::WindowLayoutNode::Stack { children, .. } = edit::layout().root else { panic!("expected a stack layout") };
    assert!(children.iter().all(|window| matches!(window, semio_plugin::WindowLayoutNode::Window { title: None, .. })));
}

#[semio_framework_async_macros::async_test]
async fn editing_an_input_recalculates_dependent_formula_caches_in_one_undoable_command_and_the_saved_xlsx() {
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx;
    use semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::apply_xlsx_mutation;
    use std::io::Read;
    let cached = |value: f64| XlsxCellValue::Formula { expr: "SUM(B2:B3)".into(), cached: Some(Box::new(XlsxCellValue::Number(value))) };
    let cell = |row, col, value| XlsxCell { row, col, value };
    let base = build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![cell(2, 1, XlsxCellValue::Number(1200.5)), cell(3, 1, XlsxCellValue::Number(350.25)), cell(4, 1, cached(1550.75))] }],
        ..Default::default()
    });
    let revision = |snapshot: &XlsxSnapshot, row| xlsx_cell_address(snapshot, "Sheet1", row, 1).unwrap().revision;
    let set = XlsxEditorCommand::SetCell { sheet_name: "Sheet1".into(), row: 3, column: 1, revision: revision(&base, 3), value: "400".into() };
    let emitted = xlsx_set_cell_emit(&base, &set, None).unwrap().artifact_mutations;
    assert_eq!(emitted.len(), 2, "the user's edit plus one recalculated formula cache");
    let XlsxMutation::SetCell(follow_up) = &emitted[1] else { panic!("recalculated cache must be a SetCell") };
    assert_eq!(follow_up.value, cached(1600.5));
    assert_eq!(follow_up.node, None);
    let mut edited = base.clone();
    for mutation in &emitted {
        apply_xlsx_mutation(&mut edited, mutation);
    }
    let projected = edited.project_workbook().unwrap();
    assert_eq!(projected.sheets[0].cells.iter().find(|cell| cell.row == 4).unwrap().value, cached(1600.5));
    let mut undone = edited.clone();
    for mutation in emitted.iter().rev() {
        let mut before = base.clone();
        for earlier in emitted.iter().take_while(|earlier| *earlier != mutation) {
            apply_xlsx_mutation(&mut before, earlier);
        }
        for inverse in <XlsxMutation as protocol::ColdMutation<XlsxSnapshot>>::inverse(mutation, &before).unwrap() {
            apply_xlsx_mutation(&mut undone, &inverse);
        }
    }
    assert_eq!(undone, base, "undoing the one command reverts the edit and the recalculated caches");

    let vacancy = xlsx_cell_vacancy_address(&edited, "Sheet1", 5, 1).unwrap();
    let typed = XlsxEditorCommand::SetCell { sheet_name: "Sheet1".into(), row: 5, column: 1, revision: vacancy.worksheet.revision.clone(), value: "=B4*2".into() };
    let inserted = xlsx_set_cell_emit(&edited, &typed, None).unwrap().artifact_mutations;
    assert!(matches!(inserted[0], XlsxMutation::InsertCell(_)));
    for mutation in &inserted {
        apply_xlsx_mutation(&mut edited, mutation);
    }
    let projected = edited.project_workbook().unwrap();
    assert_eq!(projected.sheets[0].cells.iter().find(|cell| cell.row == 5).unwrap().value, XlsxCellValue::Formula { expr: "B4*2".into(), cached: Some(Box::new(XlsxCellValue::Number(3201.0))) });

    let bytes = XlsxEditor::encode_natural_file(&edited).unwrap();
    let mut sheet = String::new();
    zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap().by_name("xl/worksheets/sheet1.xml").unwrap().read_to_string(&mut sheet).unwrap();
    assert!(sheet.contains("<f>SUM(B2:B3)</f><v>1600.5</v>"), "{sheet}");
    assert!(sheet.contains("<f>B4*2</f><v>3201</v>"), "{sheet}");
}
