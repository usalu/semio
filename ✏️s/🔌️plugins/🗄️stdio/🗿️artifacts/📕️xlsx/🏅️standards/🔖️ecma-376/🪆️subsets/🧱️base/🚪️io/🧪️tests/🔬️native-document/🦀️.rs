//! 🧪️ Native workbook ingest and package authority laws.

use calamine::Reader;
use semio_plugin::{io::{AnalyzeSource, ComposeSource, Confidence}, ArtifactAnalysis, ArtifactBuilder, ArtifactComposition};
use semio_s_artifact_stdio_xlsx_base::standards::v_ecma_376::subsets::base::io::{
    export::serializers::encode_xlsx,
    import::deserializers::{decode_xlsx, sniff_xlsx_subset},
    XlsxAnalyzer, XlsxBuilder, XlsxComposerComposition,
};
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxXmlPart;
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text;
use semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::decode_zip;

const WORKBOOK: &[u8] = include_bytes!("../../../🧪️tests/🧫️fixtures/📕️reuse-marketplaces.xlsx");
const NATIVE_DOCUMENT_FIXTURES: &str = include_str!("../../../🧪️tests/🧫️fixtures/📕️native-document/🔣️.json");

#[test]
fn physical_workbook_requires_its_registered_media_type() {
    use semio_s_artifact_stdio_zip::opc::{decode_opc, encode_opc};

    let fixtures: serde_json::Value = serde_json::from_str(NATIVE_DOCUMENT_FIXTURES).unwrap();
    let fixture = &fixtures["workbookContentType"];
    let part = fixture["part"].as_str().unwrap();
    let package = decode_opc(WORKBOOK).unwrap();
    let snapshot = decode_xlsx(WORKBOOK).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let content_type = case["contentType"].as_str().unwrap();
        let accepted = case["accepted"].as_bool().unwrap();
        let mut input = package.clone();
        input.set_part(part, content_type, package.part_bytes(part).unwrap().to_vec());
        let bytes = encode_opc(&input).unwrap();
        assert_eq!(XlsxAnalyzer::sniff(&AnalyzeSource::Binary(&bytes)) == Confidence::High, accepted, "{content_type}");
        assert_eq!(decode_xlsx(&bytes).is_ok(), accepted, "{content_type}");

        let mut output = snapshot.clone();
        output.opc.content_types.set_override(part, content_type);
        output.xml_part_mut(part).unwrap().content_type = content_type.into();
        assert_eq!(encode_xlsx(&output).is_ok(), accepted, "{content_type}");
    }
}

#[test]
fn analyzer_stamps_the_subset_the_workbook_part_declares() {
    use semio_s_artifact_stdio_zip::opc::{decode_opc, encode_opc};

    let fixtures: serde_json::Value = serde_json::from_str(NATIVE_DOCUMENT_FIXTURES).unwrap();
    let fixture = &fixtures["workbookSubsets"];
    let part = fixture["part"].as_str().unwrap();
    let transitional = fixture["transitionalNamespace"].as_str().unwrap();
    let package = decode_opc(WORKBOOK).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mut input = package.clone();
        let text = String::from_utf8(package.part_bytes(part).unwrap().to_vec()).unwrap();
        input.set_part(part, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml", text.replace(transitional, case["namespace"].as_str().unwrap()).into_bytes());
        for (_, relationships) in input.relationships.groups_mut() {
            for relationship in relationships.iter_mut().filter(|relationship| relationship.rel_type.ends_with("/officeDocument")) {
                relationship.rel_type = case["relationshipType"].as_str().unwrap().into();
            }
        }
        let bytes = encode_opc(&input).unwrap();
        let expected = case["subset"].as_str().unwrap_or("*");
        assert_eq!(XlsxAnalyzer::analyze(&[AnalyzeSource::Binary(&bytes)]).dialect.subset.0, expected, "{name}");
        assert_eq!(sniff_xlsx_subset(&bytes), case["subset"].as_str(), "{name}");
    }
    let reference: calamine::Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(WORKBOOK)).unwrap();
    assert!(!reference.sheet_names().is_empty());
    assert_eq!(sniff_xlsx_subset(WORKBOOK), Some("transitional"));
    assert_eq!(XlsxAnalyzer::analyze(&[AnalyzeSource::Binary(WORKBOOK)]).dialect.subset.0, "transitional");
    assert_eq!(sniff_xlsx_subset(b"not a package"), None);
}

#[test]
fn physical_workbook_ingestion_and_export_preserve_the_document() {
    let input = AnalyzeSource::Binary(WORKBOOK);
    assert_eq!(XlsxAnalyzer::sniff(&input), Confidence::High);
    let analysis = XlsxAnalyzer::analyze(&[input]);
    assert_eq!(analysis.confidence, Confidence::High);
    assert!(analysis.diagnostics.is_empty(), "{:?}", analysis.diagnostics);
    let snapshot = analysis.parts.snapshot.unwrap();
    assert_eq!(XlsxBuilder::from_binary(WORKBOOK).unwrap().build().unwrap(), snapshot);

    let output = encode_xlsx(&snapshot).unwrap();
    assert_eq!(decode_xlsx(&output).unwrap().opc.content_types, snapshot.opc.content_types);
    let mut source: calamine::Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(WORKBOOK)).unwrap();
    let mut exported: calamine::Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(output)).unwrap();
    assert_eq!(source.sheet_names(), exported.sheet_names());
    for name in source.sheet_names() {
        assert_eq!(source.worksheet_range(&name).unwrap(), exported.worksheet_range(&name).unwrap());
    }
}

#[test]
fn composition_reads_physical_and_packed_zip_sources() {
    let zip = decode_zip(WORKBOOK).unwrap();
    let packed = pack::snapshot::ArtifactPack::encode_pack(&zip);
    let expected = decode_xlsx(WORKBOOK).unwrap().project_workbook().unwrap();
    let dialect = XlsxComposerComposition::reads().iter().copied().find(|dialect| dialect.artifact_kind == "s.stdio.zip").unwrap();
    for bytes in [WORKBOOK, packed.as_slice()] {
        let source = ComposeSource { dialect, payload: AnalyzeSource::Binary(bytes) };
        let composed = XlsxComposerComposition::compose(&[source]).unwrap();
        assert_eq!(composed.snapshot.project_workbook().unwrap(), expected);
    }
}

#[test]
fn export_keeps_the_default_content_type_for_custom_xml() {
    let mut snapshot = decode_xlsx(WORKBOOK).unwrap();
    let part = XlsxXmlPart {
        path: "customXml/item1.xml".into(),
        content_type: "application/xml".into(),
        document: xml_document_from_text("<item/>").unwrap(),
    };
    snapshot.xml_parts.push(part.clone());
    let exported = decode_xlsx(&encode_xlsx(&snapshot).unwrap()).unwrap();
    assert_eq!(exported.opc.content_types, snapshot.opc.content_types);
    assert_eq!(exported.xml_part(&part.path), Some(&part));
}

#[test]
fn implicit_worksheet_coordinates_edit_and_undo_without_moving_siblings() {
    use semio_s_artifact_stdio_xlsx_base::editor::xlsx::standards::v_ecma_376::subsets::base::{XlsxEditor, XlsxEditorCommand};
    use protocol::{ColdMutation, OpBinary, OpText};
    use semio_framework_value::ToValue;
    use semio_s_artifact_stdio_contract::editing::{BoundedNativeEditingEditor, SnapshotEditEvent, SnapshotEditingCommand, SnapshotEditingEditor};
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::{
        construction::build_minimal_xlsx,
        snapshot::{XlsxCellValue, XlsxSheet, XlsxWorkbook},
    };
    use semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::{
        cell_address::{xlsx_cell_address, xlsx_cell_vacancy_address}, insert_cell::InsertCell, remove_cell::RemoveCell, XlsxMutation,
    };
    use semio_s_artifact_stdio_zip::opc::{decode_opc, encode_opc};

    let fixtures: serde_json::Value = serde_json::from_str(NATIVE_DOCUMENT_FIXTURES).unwrap();
    let fixture = &fixtures["implicitCoordinates"];
    let sheet = fixture["sheet"].as_str().unwrap();
    let expected: Vec<(u32, u32, f64)> = serde_json::from_value(fixture["cells"].clone()).unwrap();
    let seed = build_minimal_xlsx(XlsxWorkbook { sheets: vec![XlsxSheet { name: sheet.into(), cells: vec![] }], shared_strings: vec![] });
    let mut package = decode_opc(&encode_xlsx(&seed).unwrap()).unwrap();
    let part = &seed.worksheet_part_paths()[0];
    let content_type = package.content_types.resolve(part).unwrap().to_owned();
    package.set_part(part, &content_type, fixture["worksheet"].as_str().unwrap().as_bytes().to_vec());
    let input = encode_opc(&package).unwrap();
    let snapshot = decode_xlsx(&input).unwrap();
    assert_eq!(snapshot.xml_part(part).unwrap().document, xml_document_from_text(fixture["worksheet"].as_str().unwrap()).unwrap());
    let project = |snapshot: &semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot| {
        snapshot.project_workbook().unwrap().sheets[0].cells.iter().map(|cell| {
            let XlsxCellValue::Number(value) = cell.value else { panic!("expected numeric fixture cell") };
            (cell.row, cell.col, value)
        }).collect::<Vec<_>>()
    };
    let verify_external = |bytes: &[u8], cells: &[(u32, u32, f64)]| {
        let mut reference: calamine::Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(bytes)).unwrap();
        let result = reference.worksheet_range(sheet).unwrap();
        assert_eq!(result.used_cells().count(), cells.len());
        for &(row, column, value) in cells {
            assert_eq!(result.get_value((row - 1, column)), Some(&calamine::Data::Float(value)));
        }
    };
    assert_eq!(project(&snapshot), expected);
    verify_external(&input, &expected);
    let unchanged = decode_xlsx(&encode_xlsx(&snapshot).unwrap()).unwrap();
    assert_eq!(unchanged.xml_part(part), snapshot.xml_part(part));

    for edit in fixture["edits"].as_array().unwrap() {
        let row = edit["row"].as_u64().unwrap().try_into().unwrap();
        let column = edit["column"].as_u64().unwrap().try_into().unwrap();
        let address = || xlsx_cell_address(&snapshot, sheet, row, column).unwrap();
        let value = || XlsxCellValue::Number(edit["value"].as_f64().unwrap());
        let mutation = match edit["operation"].as_str().unwrap() {
            "set" => {
                let command = SnapshotEditingCommand::Native(XlsxEditorCommand::SetCell {
                    sheet_name: sheet.into(), row, column, revision: address().revision, value: edit["value"].to_string(),
                });
                let mut emitted = XlsxEditor::native_edit_mutations(&command, &snapshot).unwrap().artifact_mutations;
                assert_eq!(emitted.len(), 1);
                let mutation = emitted.pop().unwrap();
                assert!(matches!(mutation, XlsxMutation::SetCell(_)));
                mutation
            }
            "remove" => XlsxMutation::RemoveCell(RemoveCell { address: address(), keep_row: false }),
            "insert" => {
                let address = xlsx_cell_vacancy_address(&snapshot, sheet, row, column).unwrap();
                XlsxMutation::InsertCell(InsertCell { address, value: value(), node: None })
            }
            "insert-implicit" => {
                let before_column = edit["beforeColumn"].as_u64().unwrap().try_into().unwrap();
                let before = xlsx_cell_address(&snapshot, sheet, row, before_column).unwrap();
                let part_index = snapshot.xml_parts.iter().position(|part| part.path == before.part_path).unwrap();
                let children: String = before.node_path.iter().map(|index| format!("/children/{index}")).collect();
                let event = SnapshotEditEvent::InsertValue {
                    path: format!("/xmlParts/{part_index}/document/root{children}"),
                    value: xml_document_from_text(edit["node"].as_str().unwrap()).unwrap().root.unwrap().to_value(),
                };
                let mut emitted = XlsxEditor::snapshot_edit_emit(&event, &snapshot).unwrap().artifact_mutations;
                assert_eq!(emitted.len(), 1);
                let mutation = emitted.pop().unwrap();
                assert!(matches!(mutation, XlsxMutation::InsertCell(_)));
                mutation
            }
            other => panic!("unexpected verification operation {other}"),
        };
        let edited_cells: Vec<(u32, u32, f64)> = serde_json::from_value(edit["cells"].clone()).unwrap();
        let inverse = ColdMutation::inverse(&mutation, &snapshot).unwrap();
        let result = ColdMutation::diff(&mutation, &snapshot);
        let edited = protocol::apply_diff_cold(result.diff(), &snapshot).unwrap();
        assert_eq!(project(&edited), edited_cells);
        let exported = encode_xlsx(&edited).unwrap();
        verify_external(&exported, &edited_cells);
        assert_eq!(project(&decode_xlsx(&exported).unwrap()), edited_cells);
        assert_eq!(inverse.len(), 1);
        let undo = &inverse[0];
        let via_text = XlsxMutation::parse_op(&undo.print_op()).unwrap();
        let via_bytes = XlsxMutation::decode_op(&undo.encode_op().unwrap()).unwrap();
        assert_eq!(&via_text, undo);
        assert_eq!(&via_bytes, undo);
        let redo = ColdMutation::inverse(undo, &edited).unwrap();
        let restored = protocol::apply_diff_cold(ColdMutation::diff(undo, &edited).diff(), &edited).unwrap();
        assert_eq!(restored, snapshot);
        assert_eq!(redo.len(), 1);
        assert_eq!(protocol::apply_diff_cold(ColdMutation::diff(&redo[0], &restored).diff(), &restored).unwrap(), edited);
    }
}

#[test]
fn conflict_paths_lend_every_authored_mutation_segment() {
    use protocol::SemanticMutation;

    use semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::demo_mutation_cases;

    for mutation in demo_mutation_cases() {
        let expected = mutation.target();
        let Some(path) = mutation.original_conflict_target() else {
            assert!(expected.is_empty());
            continue;
        };
        assert_eq!(path.len(), expected.len());
        assert!(path.segment_len(path.len()).is_err());
        for (index, segment) in expected.iter().enumerate() {
            assert_eq!(path.segment_len(index).unwrap(), segment.len());
            let mut actual = Vec::new();
            let mut chunk = [0; 2];
            while actual.len() < segment.len() {
                let copied = path.read_segment(index, actual.len(), &mut chunk).unwrap();
                assert!(copied > 0);
                actual.extend_from_slice(&chunk[..copied]);
            }
            assert_eq!(actual, segment.as_bytes());
            assert!(path.read_segment(index, segment.len() + 1, &mut chunk).is_err());
        }
    }
}
