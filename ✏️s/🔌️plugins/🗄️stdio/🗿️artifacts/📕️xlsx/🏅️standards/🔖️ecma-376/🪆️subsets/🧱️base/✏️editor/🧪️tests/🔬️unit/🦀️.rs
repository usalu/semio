use super::*;

#[semio_framework_async_macros::async_test]
async fn create_xlsx_editor_builds_a_definition_for_the_editor_role() {
    let def = create_xlsx_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, XLSX_DIALECT.into());
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
    assert_eq!(render_xlsx_cell_value(&XlsxCellValue::SharedString(0), &strings), "hello");
    assert_eq!(render_xlsx_cell_value(&XlsxCellValue::SharedString(9), &strings), "#9");
    assert_eq!(render_xlsx_cell_value(&XlsxCellValue::Formula { expr: "SUM(A1:A2)".into(), cached: Some(Box::new(XlsxCellValue::Number(3.0))) }, &strings), "=SUM(A1:A2)");
    assert_eq!(render_xlsx_cell_value(&XlsxCellValue::Empty, &strings), "");
}

#[semio_framework_async_macros::async_test]
async fn parse_cell_value_detects_bool_and_number_before_falling_back_to_inline_string() {
    assert_eq!(parse_xlsx_cell_value(""), XlsxCellValue::Empty);
    assert_eq!(parse_xlsx_cell_value("=SUM(A1:A2)"), XlsxCellValue::Formula { expr: "SUM(A1:A2)".into(), cached: None });
    assert_eq!(parse_xlsx_cell_value("'=literal"), XlsxCellValue::InlineString("=literal".into()));
    assert_eq!(parse_xlsx_cell_value("true"), XlsxCellValue::Boolean(true));
    assert_eq!(parse_xlsx_cell_value("false"), XlsxCellValue::Boolean(false));
    assert_eq!(parse_xlsx_cell_value("3.5"), XlsxCellValue::Number(3.5));
    assert_eq!(parse_xlsx_cell_value("NaN"), XlsxCellValue::InlineString("NaN".into()));
    assert_eq!(parse_xlsx_cell_value("hello"), XlsxCellValue::InlineString("hello".into()));
}

#[semio_framework_async_macros::async_test]
async fn stable_cell_action_fixture_roundtrips_exact_text_through_parser_and_binary_codec() {
    const FIXTURE: &str = include_str!("../../🧫️fixtures/📊️stable-cell-edit/🔣️.json");
    let oracle: serde_json::Value = serde_json::from_str(FIXTURE).expect("third-party JSON oracle parses the action fixture");
    let args = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(FIXTURE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("DSL JSON parses the action fixture");
    let command = XlsxEditor::command_from_action("set-cell", Some(&args)).expect("stable cell action parses");
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
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCell, XlsxSheet, XlsxWorkbook};
    let snapshot = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet 1".into(), cells: vec![XlsxCell { row: 41, col: 7, value: XlsxCellValue::Number(1.0) }] }],
        ..Default::default()
    });
    let address = crate::standards::v_ecma_376::subsets::base::schema::mutations::cell_address::xlsx_cell_address(&snapshot, "Sheet 1", 41, 7).unwrap();
    let command = XlsxEditorCommand::SetCell { sheet_name: "Sheet 1".into(), row: 41, column: 7, revision: address.revision.clone(), value: "2".into() };
    let emit = xlsx_set_cell_emit(&snapshot, &command).expect("matching revision emits a mutation");
    assert_eq!(emit.artifact_mutations, vec![XlsxMutation::SetCell(set_cell::SetCell { address, value: XlsxCellValue::Number(2.0) })]);
    let stale = XlsxEditorCommand::SetCell { sheet_name: "Sheet 1".into(), row: 41, column: 7, revision: "stale".into(), value: "2".into() };
    assert!(xlsx_set_cell_emit(&snapshot, &stale).is_err());
}

#[semio_framework_async_macros::async_test]
async fn blank_cell_command_inserts_canonical_xml_saves_for_calamine_and_inverts_exactly() {
    use crate::schema::snapshot::{XlsxSheet, XlsxWorkbook};
    use crate::standards::v_ecma_376::subsets::base::io::export::serializers::encode_xlsx;
    use crate::schema::construction::build_minimal_xlsx;
    use crate::standards::v_ecma_376::subsets::base::schema::mutations::{apply_xlsx_mutation, cell_address::xlsx_cell_vacancy_address, insert_cell};
    use calamine::{Data, Reader};
    use protocol::Mutation;
    let base = build_minimal_xlsx(XlsxWorkbook { sheets: vec![XlsxSheet { name: "Blank".into(), cells: Vec::new() }], ..Default::default() });
    let vacancy = xlsx_cell_vacancy_address(&base, "Blank", 1, 0).unwrap();
    let command = XlsxEditorCommand::SetCell { sheet_name: "Blank".into(), row: 1, column: 0, revision: vacancy.worksheet.revision.clone(), value: "42".into() };
    let emitted = xlsx_set_cell_emit(&base, &command).unwrap();
    let mutation = XlsxMutation::InsertCell(insert_cell::InsertCell { address: vacancy, value: XlsxCellValue::Number(42.0) });
    assert_eq!(emitted.artifact_mutations, vec![mutation.clone()]);
    let mut edited = base.clone();
    apply_xlsx_mutation(&mut edited, &mutation);
    assert_eq!(edited.project_workbook().unwrap().sheets[0].cells[0].value, XlsxCellValue::Number(42.0));
    let mut reference: calamine::Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(encode_xlsx(&edited).unwrap())).expect("Calamine opens canonical blank-cell save");
    assert!(matches!(reference.worksheet_range("Blank").unwrap().get_value((0, 0)), Some(Data::Float(value)) if *value == 42.0));
    for inverse in <XlsxMutation as Mutation<XlsxSnapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture") {
        apply_xlsx_mutation(&mut edited, &inverse);
    }
    assert_eq!(edited, base, "vacancy insertion inverse restores every authoritative OPC and XML field");
}

#[semio_framework_async_macros::async_test]
async fn sparse_row_insertion_preserves_formula_neighbors_and_stales_prior_vacancies() {
    use crate::schema::snapshot::{XlsxCell, XlsxSheet, XlsxWorkbook};
    use crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx;
    use crate::standards::v_ecma_376::subsets::base::schema::mutations::{
        apply_xlsx_mutation,
        cell_address::{resolve_xlsx_cell_vacancy_address, xlsx_cell_vacancy_address},
        insert_cell,
    };
    let mut snapshot = build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet {
            name: "Sparse".into(),
            cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Formula { expr: "2+3".into(), cached: Some(Box::new(XlsxCellValue::Number(5.0))) } }, XlsxCell { row: 1, col: 2, value: XlsxCellValue::InlineString("right".into()) }],
        }],
        ..Default::default()
    });
    let old_vacancy = xlsx_cell_vacancy_address(&snapshot, "Sparse", 2, 0).unwrap();
    let middle = xlsx_cell_vacancy_address(&snapshot, "Sparse", 1, 1).unwrap();
    apply_xlsx_mutation(&mut snapshot, &XlsxMutation::InsertCell(insert_cell::InsertCell { address: middle, value: XlsxCellValue::InlineString("middle".into()) }));
    let projected = snapshot.project_workbook().unwrap();
    assert_eq!(projected.sheets[0].cells.iter().map(|cell| (cell.row, cell.col)).collect::<Vec<_>>(), [(1, 0), (1, 1), (1, 2)]);
    assert_eq!(projected.sheets[0].cells[0].value, XlsxCellValue::Formula { expr: "2+3".into(), cached: Some(Box::new(XlsxCellValue::Number(5.0))) });
    assert!(resolve_xlsx_cell_vacancy_address(&snapshot, &old_vacancy).is_err(), "a structural edit invalidates a prior worksheet vacancy revision");
}

#[semio_framework_async_macros::async_test]
async fn unchanged_cell_drafts_preserve_types_and_cached_values() {
    use crate::schema::snapshot::{XlsxCell, XlsxSheet, XlsxWorkbook};
    const FIXTURE: &str = include_str!("../../../🧫️fixtures/✍️unchanged-cell-draft/🔣️.json");
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
    let shared_strings: Vec<String> = serde_json::from_value(fixture["sharedStrings"].clone()).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let original: XlsxCellValue = semio_framework_pack_json::from_json_str(&case["value"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let snapshot = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
            sheets: vec![XlsxSheet { name: "Sheet".into(), cells: vec![XlsxCell { row: 1, col: 0, value: original.clone() }] }],
            shared_strings: shared_strings.clone(),
        });
        let revision = crate::standards::v_ecma_376::subsets::base::schema::mutations::cell_address::xlsx_cell_address(&snapshot, "Sheet", 1, 0).unwrap().revision;
        let command = XlsxEditorCommand::SetCell { sheet_name: "Sheet".into(), row: 1, column: 0, revision, value: case["draft"].as_str().unwrap().into() };
        let emitted = xlsx_set_cell_emit(&snapshot, &command).unwrap();
        assert!(emitted.artifact_mutations.is_empty(), "unchanged draft must not rewrite {}", case["id"]);
        assert_eq!(snapshot.project_workbook().unwrap().sheets[0].cells[0].value, original);
    }
    let conflict = &fixture["sharedStringConflict"];
    let index = conflict["index"].as_u64().unwrap() as usize;
    let value = XlsxCellValue::SharedString(index);
    let mut snapshot = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook { sheets: vec![XlsxSheet { name: "Sheet".into(), cells: vec![XlsxCell { row: 1, col: 0, value }] }], shared_strings });
    let revision = crate::standards::v_ecma_376::subsets::base::schema::mutations::cell_address::xlsx_cell_address(&snapshot, "Sheet", 1, 0).unwrap().revision;
    let command = XlsxEditorCommand::SetCell { sheet_name: "Sheet".into(), row: 1, column: 0, revision, value: conflict["draft"].as_str().unwrap().into() };
    crate::standards::v_ecma_376::subsets::base::schema::mutations::apply_xlsx_mutation(
        &mut snapshot,
        &XlsxMutation::SetSharedString(crate::standards::v_ecma_376::subsets::base::schema::mutations::set_shared_string::SetSharedString { index: 1, value: "unrelated change".into() }),
    );
    assert!(xlsx_set_cell_emit(&snapshot, &command).is_ok(), "unrelated shared strings do not invalidate this cell");
    crate::standards::v_ecma_376::subsets::base::schema::mutations::apply_xlsx_mutation(
        &mut snapshot,
        &XlsxMutation::SetSharedString(crate::standards::v_ecma_376::subsets::base::schema::mutations::set_shared_string::SetSharedString { index, value: conflict["replacement"].as_str().unwrap().into() }),
    );
    assert!(xlsx_set_cell_emit(&snapshot, &command).is_err(), "referenced text changes invalidate the draft");
}

#[semio_framework_async_macros::async_test]
async fn unchanged_cell_draft_fixture_matches_independent_spreadsheet_values_and_formulas() {
    use crate::schema::snapshot::{XlsxCell, XlsxSheet, XlsxWorkbook};
    use crate::standards::v_ecma_376::subsets::base::io::export::serializers::encode_xlsx;
    use crate::schema::construction::build_minimal_xlsx;
    use calamine::{Data, Reader};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/✍️unchanged-cell-draft/🔣️.json")).unwrap();
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
    serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️canonical-xml-save/🔣️.json")).expect("neutral canonical save fixture")
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
    use crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_xlsx;
    use crate::standards::v_ecma_376::subsets::base::schema::mutations::{apply_xlsx_mutation, cell_address::xlsx_cell_address, set_cell::SetCell};
    for case in canonical_save_fixture()["cases"].as_array().unwrap() {
        let mut snapshot = decode_xlsx(&canonical_fixture_zip(case)).unwrap_or_else(|error| panic!("{} import: {error}", case["id"]));
        let address = xlsx_cell_address(&snapshot, "Data", 1, 0).unwrap_or_else(|error| panic!("{} A1 address: {error}", case["id"]));
        apply_xlsx_mutation(&mut snapshot, &XlsxMutation::SetCell(SetCell { address, value: XlsxCellValue::Number(99.25) }));
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

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::XlsxEditor, || semio_framework_plugin::App { definition: super::create_xlsx_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base");
