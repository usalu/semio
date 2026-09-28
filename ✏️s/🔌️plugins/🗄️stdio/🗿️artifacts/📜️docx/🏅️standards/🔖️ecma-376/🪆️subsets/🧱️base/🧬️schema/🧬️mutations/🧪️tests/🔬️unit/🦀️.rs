use super::*;
use protocol::command::DiffAlgebra;
use protocol::{DiffCodec, MutationDiff};

fn namespace_formatting_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/🏷️namespace-formatting/🔣️.json")).unwrap()
}

fn namespace_formatting_snapshot(case: &serde_json::Value) -> DocxSnapshot {
    use semio_s_artifact_stdio_xml::schema::snapshot::xml_document_from_text;
    let mut snapshot = crate::engine::build_minimal_docx(DocxDocument { body: Vec::new(), styles: vec![DocxStyle { id: "Heading".into(), name: "Heading title".into(), based_on: Some("Normal".into()) }] });
    snapshot.xml_part_mut("word/document.xml").unwrap().document = xml_document_from_text(case["documentXml"].as_str().unwrap()).unwrap();
    snapshot.xml_part_mut("word/styles.xml").unwrap().document = xml_document_from_text(case["stylesXml"].as_str().unwrap()).unwrap();
    snapshot
}

fn word_namespace_oracle(xml: &str, namespace: &str) -> Vec<(String, Vec<(String, String)>)> {
    use quick_xml::{XmlVersion, events::Event, name::ResolveResult, reader::NsReader};
    let mut reader = NsReader::from_str(xml);
    let mut result = Vec::new();
    loop {
        match reader.read_event().unwrap() {
            Event::Start(event) | Event::Empty(event) => {
                let (resolved, local) = reader.resolver().resolve_element(event.name());
                if !matches!(resolved, ResolveResult::Bound(value) if value.as_ref() == namespace.as_bytes()) {
                    continue;
                }
                let name = String::from_utf8(local.as_ref().to_vec()).unwrap();
                let attrs = event
                    .attributes()
                    .map(|attr| attr.unwrap())
                    .filter_map(|attr| {
                        let (resolved, local) = reader.resolver().resolve_attribute(attr.key);
                        matches!(resolved, ResolveResult::Bound(value) if value.as_ref() == namespace.as_bytes()).then(|| (String::from_utf8(local.as_ref().to_vec()).unwrap(), attr.normalized_value(XmlVersion::Explicit1_0).unwrap().into_owned()))
                    })
                    .collect();
                result.push((name, attrs));
            }
            Event::Eof => return result,
            _ => {}
        }
    }
}

#[test]
fn namespace_projection_matches_independent_expanded_names_and_false_flags() {
    for case in namespace_formatting_fixture()["cases"].as_array().unwrap() {
        let namespace = case["namespace"].as_str().unwrap();
        let oracle = word_namespace_oracle(case["documentXml"].as_str().unwrap(), namespace);
        let expected: Vec<bool> = serde_json::from_value(case["format"].clone()).unwrap();
        for (index, property) in ["b", "i", "u"].iter().enumerate() {
            let (_, attrs) = oracle.iter().find(|(name, _)| name == property).unwrap();
            let value = attrs.iter().find(|(name, _)| name == "val").map(|(_, value)| value.as_str());
            assert_eq!(!matches!(value, Some("0" | "false" | "off" | "none")), expected[index]);
        }
        let document = project(&namespace_formatting_snapshot(case));
        let DocxBlock::Paragraph(paragraph) = &document.body[0] else { panic!("paragraph") };
        assert_eq!(paragraph.style, case["paragraphStyle"].as_str().map(str::to_string), "{}", case["id"]);
        assert_eq!(paragraph.runs[0].text, "Grüße 文字");
        assert_eq!([paragraph.runs[0].bold, paragraph.runs[0].italic, paragraph.runs[0].underline].as_slice(), expected);
        let ids: Vec<String> = serde_json::from_value(case["styleIds"].clone()).unwrap();
        assert_eq!(document.styles.iter().map(|style| style.id.clone()).collect::<Vec<_>>(), ids);
        let oracle_ids = word_namespace_oracle(case["stylesXml"].as_str().unwrap(), namespace)
            .into_iter()
            .filter(|(name, _)| name == "style")
            .filter_map(|(_, attrs)| attrs.into_iter().find(|(name, _)| name == "styleId").map(|(_, value)| value))
            .collect::<Vec<_>>();
        assert_eq!(oracle_ids, ids);
    }
    println!("[DEBUG] DOCX semantic projection matches six independent namespace and explicit formatting cases");
}

#[test]
fn namespace_formatting_and_style_edits_preserve_qualified_attributes_and_inverse() {
    use semio_s_artifact_stdio_xml::schema::snapshot::xml_document_to_text;
    for case in namespace_formatting_fixture()["cases"].as_array().unwrap() {
        let before = namespace_formatting_snapshot(case);
        let namespace = case["namespace"].as_str().unwrap();
        let address = docx_xml_address(&before, "word/document.xml", vec![0, 0, 1]).unwrap();
        let mutation = DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { address, bold: false, italic: false, underline: false });
        let prepared = prepare_addressed_xml_mutation(&before, &mutation).unwrap();
        let mut after = before.clone();
        apply_addressed_xml_mutation_in_place(&mut after, &mutation).unwrap();
        let output = xml_document_to_text(&after.xml_part("word/document.xml").unwrap().document);
        let oracle = word_namespace_oracle(&output, namespace);
        for marker in case["retainedMarkers"].as_array().unwrap() {
            assert!(output.contains(marker.as_str().unwrap()), "{}: {output}", case["id"]);
        }
        for (property, value) in [("b", "0"), ("i", "0"), ("u", "none")] {
            let props = oracle.iter().filter(|(name, _)| name == property).collect::<Vec<_>>();
            assert_eq!(props.len(), 1, "{}: {output}", case["id"]);
            assert!(props[0].1.contains(&("val".to_string(), value.to_string())), "{}: {output}", case["id"]);
        }
        let reopened = crate::engine::decode_docx(&crate::engine::encode_docx(&after).unwrap()).unwrap();
        assert_eq!(reopened, after);
        apply_addressed_xml_mutation_in_place(&mut after, &prepared.inverse).unwrap();
        assert_eq!(after, before);
        let address = docx_top_level_block_address(&before, 0).unwrap();
        let mut styled = before.clone();
        let mutation = DocxMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { address, style_id: Some("Heading".into()) });
        if case["styleIds"].as_array().unwrap().is_empty() {
            assert!(prepare_addressed_xml_mutation(&before, &mutation).is_err());
        } else {
            let prepared = prepare_addressed_xml_mutation(&before, &mutation).unwrap();
            apply_addressed_xml_mutation_in_place(&mut styled, &mutation).unwrap();
            let output = xml_document_to_text(&styled.xml_part("word/document.xml").unwrap().document);
            let oracle = word_namespace_oracle(&output, namespace);
            for marker in case["retainedMarkers"].as_array().unwrap() {
                assert!(output.contains(marker.as_str().unwrap()), "{}: {output}", case["id"]);
            }
            assert!(oracle.iter().any(|(name, attrs)| name == "pStyle" && attrs.contains(&("val".into(), "Heading".into()))));
            apply_addressed_xml_mutation_in_place(&mut styled, &prepared.inverse).unwrap();
            assert_eq!(styled, before);
        }
    }
    println!("[DEBUG] DOCX direct formatting false, style identity, namespace-qualified attributes, save and compact inverse agree");
}

fn project(snapshot: &DocxSnapshot) -> DocxDocument {
    snapshot.project_document().expect("canonical DOCX projects")
}

fn apply(snapshot: &mut DocxSnapshot, mutation: &DocxMutation) {
    let outcome = apply_docx_mutation(snapshot, mutation);
    assert!(outcome.messages().is_empty(), "mutation was refused: {:?}", outcome.messages());
}

#[semio_framework_async_macros::async_test]
async fn block_run_style_and_part_mutations_apply_and_inverse() {
    let mut base = crate::engine::build_minimal_docx(DocxDocument {
        body: vec![
            DocxBlock::paragraph("first"),
            DocxBlock::Table(DocxTable { rows: vec![DocxTableRow { cells: vec![DocxTableCell { blocks: vec![DocxBlock::paragraph("cell")], ..Default::default() }], ..Default::default() }], ..Default::default() }),
        ],
        styles: vec![DocxStyle { id: "Normal".into(), name: "Normal".into(), based_on: None }],
    });
    base.opc.set_part("word/media/original.bin", "application/octet-stream", vec![1, 2, 3]);
    base.validate_authority().expect("fixture authority");

    let run_address = docx_block_run_address(&base, &DocxBlockPath { segments: vec![], index: 0 }, 0).expect("run address");
    let cases = [
        DocxMutation::InsertBlock(insert_block::InsertBlock { path: DocxBlockPath { segments: vec![], index: 1 }, block: DocxBlock::paragraph("inserted") }),
        DocxMutation::SetRunText(set_run_text::SetRunText { address: run_address.clone(), text: "changed".into() }),
        DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { address: run_address, bold: true, italic: true, underline: true }),
        DocxMutation::InsertStyle(insert_style::InsertStyle { style: DocxStyle { id: "Heading1".into(), name: "Heading 1".into(), based_on: Some("Normal".into()) } }),
        DocxMutation::SetStyleName(set_style_name::SetStyleName { id: "Normal".into(), name: "Body".into() }),
        DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: "Normal".into(), based_on: Some("Heading1".into()) }),
        DocxMutation::SetPart(set_part::SetPart { path: "word/media/new.bin".into(), content_type: "application/octet-stream".into(), bytes: vec![4, 5, 6] }),
        DocxMutation::RemovePart(remove_part::RemovePart { path: "word/media/original.bin".into() }),
    ];

    for mutation in cases {
        let mut next = base.clone();
        apply(&mut next, &mutation);
        assert_ne!(next, base, "mutation must change authored state: {mutation:?}");
        for inverse in Mutation::inverse(&mutation, &base) {
            apply(&mut next, &inverse);
        }
        assert_eq!(next, base, "inverse must restore exact XML/OPC authority: {mutation:?}");
    }

    let nested = DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path: table_path(1, 0, 0, 0), block: DocxBlock::paragraph("changed cell") });
    let mut changed = base.clone();
    apply(&mut changed, &nested);
    let DocxBlock::Table(table) = &project(&changed).body[1] else { panic!("table") };
    assert_eq!(table.rows[0].cells[0].blocks[0], DocxBlock::paragraph("changed cell"));
}

#[semio_framework_async_macros::async_test]
async fn every_mutation_has_exact_diff_inverse_and_text_binary_replay() {
    for mutation in demo_mutation_cases() {
        let base = fixture();
        let direct = Mutation::diff(&mutation, &base);
        let via_diff = MutationDiff::apply(direct.diff(), &base).expect("diff applies");
        let mut via_mutation = base.clone();
        apply(&mut via_mutation, &mutation);
        assert_eq!(via_diff, via_mutation, "mutation and diff disagree for {mutation:?}");

        let inverse = DiffAlgebra::inverse(direct.diff(), &base);
        assert_eq!(MutationDiff::apply(&inverse, &via_diff).expect("inverse applies"), base);

        let text = mutation.print_op();
        assert_eq!(DocxMutation::parse_op(&text).expect("text replay"), mutation);
        let bytes = mutation.encode_op().expect("binary encode");
        assert_eq!(DocxMutation::decode_op(&bytes).expect("binary replay"), mutation);
    }
}

#[semio_framework_async_macros::async_test]
async fn canonical_xml_and_opc_diff_round_trip_and_absorb() {
    let base = fixture();
    let mut middle = base.clone();
    let address = docx_block_run_address(&base, &DocxBlockPath { segments: vec![], index: 0 }, 0).expect("run address");
    apply(&mut middle, &DocxMutation::SetRunText(set_run_text::SetRunText { address, text: "middle".into() }));
    let mut final_snapshot = middle.clone();
    apply(&mut final_snapshot, &DocxMutation::SetPart(set_part::SetPart { path: "word/media/final.bin".into(), content_type: "application/octet-stream".into(), bytes: vec![9, 8, 7] }));

    let first = DocxDiff::between(&base, &middle);
    let second = DocxDiff::between(&middle, &final_snapshot);
    assert!(first.xml_parts.is_some(), "text edit must be an XML-part diff");
    assert!(second.opc.is_some(), "binary part edit must be an OPC diff");
    let mut absorbed = first;
    absorbed.absorb(second);
    assert_eq!(absorbed.apply(&base).expect("absorbed diff applies"), final_snapshot);
    assert_eq!(absorbed.inverse(&base).apply(&final_snapshot).expect("inverse applies"), base);

    let text = absorbed.print_diff();
    assert_eq!(DocxDiff::parse_diff(&text).expect("text diff replay"), absorbed);
    let bytes = absorbed.encode_diff().expect("binary diff encode");
    assert_eq!(DocxDiff::decode_diff(&bytes).expect("binary diff replay"), absorbed);
}

#[test]
fn invalid_paragraph_style_and_last_table_row_removal_are_atomic() {
    let base = crate::engine::build_minimal_docx(DocxDocument {
        body: vec![
            DocxBlock::paragraph("body"),
            DocxBlock::Table(DocxTable { rows: vec![DocxTableRow { cells: vec![DocxTableCell { blocks: vec![DocxBlock::paragraph("only row")], ..Default::default() }], ..Default::default() }], ..Default::default() }),
        ],
        styles: vec![DocxStyle { id: "Normal".into(), name: "Normal".into(), based_on: None }],
    });
    let paragraph = docx_top_level_block_address(&base, 0).expect("paragraph address");
    let table = docx_top_level_block_address(&base, 1).expect("table address");
    for mutation in [DocxMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { address: paragraph, style_id: Some("Missing".into()) }), DocxMutation::RemoveTableRow(remove_table_row::RemoveTableRow { address: table, index: 0 })] {
        let mut snapshot = base.clone();
        let outcome = apply_docx_mutation(&mut snapshot, &mutation);
        assert!(!outcome.messages().is_empty(), "invalid edit must be refused");
        assert_eq!(snapshot, base, "refused canonical edit must leave authored state unchanged");
    }
}

#[test]
fn identical_run_insertion_invalidates_the_original_canonical_address() {
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;
    use semio_s_artifact_stdio_xml::schema::snapshot::xml_document_from_text;

    let authored: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧭️identical-run-displacement/🔣️.json")).unwrap();
    let before_xml = authored["beforeXml"].as_str().unwrap();
    let displaced_xml = authored["displacedXml"].as_str().unwrap();
    for (xml, expected) in [(before_xml, 3usize), (displaced_xml, 4usize)] {
        let mut reader = Reader::from_str(xml);
        let mut runs = 0usize;
        loop {
            match reader.read_event().unwrap() {
                Event::Start(event) if event.local_name().as_ref() == b"r" => runs += 1,
                Event::Eof => break,
                _ => {}
            }
        }
        assert_eq!(runs, expected, "independent quick-xml oracle confirms fixture structure");
    }

    let mut before = fixture();
    let part_path = authored["partPath"].as_str().unwrap();
    before.xml_part_mut(part_path).unwrap().document = xml_document_from_text(before_xml).unwrap();
    let target_path = authored["targetPath"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize).collect();
    let address = docx_xml_address(&before, part_path, target_path).unwrap();

    let mut displaced = before.clone();
    displaced.xml_part_mut(part_path).unwrap().document = xml_document_from_text(displaced_xml).unwrap();
    let unchanged = displaced.clone();
    assert!(resolve_docx_xml_address(&displaced, &address).is_err());
    assert!(prepare_addressed_xml_mutation(&displaced, &DocxMutation::SetRunText(set_run_text::SetRunText { address, text: "edited".into() })).is_err());
    assert_eq!(displaced, unchanged, "stale address refusal is atomic");
}

#[test]
fn canonical_xml_addresses_support_empty_and_default_namespaces_and_reject_unbound_prefixes() {
    use crate::schema::snapshot::DocxXmlPart;
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;
    use semio_s_artifact_stdio_xml::schema::snapshot::xml_document_from_text;

    let authored: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧭️xml-address-namespaces/🔣️.json")).unwrap();
    let part_path = authored["partPath"].as_str().unwrap();
    let target_path = authored["targetPath"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize).collect::<Vec<_>>();
    for case in authored["cases"].as_array().unwrap() {
        let xml = case["xml"].as_str().unwrap();
        let mut reader = Reader::from_str(xml);
        let mut saw_field = false;
        loop {
            match reader.read_event().unwrap() {
                Event::Start(event) if event.local_name().as_ref() == b"field" => saw_field = true,
                Event::Eof => break,
                _ => {}
            }
        }
        assert!(saw_field, "quick-xml oracle must observe the addressed fixture node");

        let mut snapshot = fixture();
        snapshot.xml_parts.push(DocxXmlPart { path: part_path.into(), content_type: "application/xml".into(), document: xml_document_from_text(xml).unwrap() });
        let result = docx_xml_address(&snapshot, part_path, target_path.clone());
        if let Some(expected) = case.get("expectedName") {
            assert_eq!(result.unwrap().expected_name, expected.as_str().unwrap());
        } else {
            assert!(result.unwrap_err().contains(case["errorContains"].as_str().unwrap()));
        }
    }
}

#[test]
fn run_text_edit_replaces_all_text_contributions_and_has_compact_exact_inverse() {
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;
    use semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_from_text, xml_document_to_text};

    let authored: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔤️run-text-fidelity/🔣️.json")).unwrap();
    let part_path = authored["partPath"].as_str().unwrap();
    for case in authored["cases"].as_array().unwrap() {
        let mut snapshot = fixture();
        snapshot.xml_part_mut(part_path).unwrap().document = xml_document_from_text(case["xml"].as_str().unwrap()).unwrap();
        let before = snapshot.clone();
        let target_path = case["targetPath"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize).collect();
        let address = docx_xml_address(&snapshot, part_path, target_path).unwrap();
        let mutation = DocxMutation::SetRunText(set_run_text::SetRunText { address, text: case["replacement"].as_str().unwrap().into() });
        let prepared = prepare_addressed_xml_mutation(&snapshot, &mutation).unwrap();
        assert!(matches!(prepared.inverse, DocxMutation::ReplaceXmlNode(_)), "inverse carries only the prior addressed node");
        assert!(prepared.diff.opc.is_none());
        let part_diff = prepared.diff.xml_parts.as_ref().unwrap();
        assert_eq!(part_diff.modified.len(), 1);
        assert!(part_diff.added.is_empty() && part_diff.removed.is_empty());

        apply_addressed_xml_mutation_in_place(&mut snapshot, &mutation).unwrap();
        let inverse = prepared.inverse.clone();
        let output = xml_document_to_text(&snapshot.xml_part(part_path).unwrap().document);
        let mut reader = Reader::from_str(&output);
        let mut local_names = Vec::new();
        let mut text_elements = 0usize;
        loop {
            match reader.read_event().unwrap() {
                Event::Start(event) | Event::Empty(event) => {
                    let local = String::from_utf8(event.local_name().as_ref().to_vec()).unwrap();
                    if local == "t" {
                        text_elements += 1;
                    }
                    local_names.push(local);
                }
                Event::Eof => break,
                _ => {}
            }
        }
        assert_eq!(text_elements, case["textElementCount"].as_u64().unwrap() as usize);
        for name in case["preservedLocalNames"].as_array().unwrap() {
            assert!(local_names.iter().any(|actual| actual == name.as_str().unwrap()), "quick-xml oracle lost {}", name.as_str().unwrap());
        }
        assert!(output.contains(case["replacement"].as_str().unwrap()));
        if case["replacement"].as_str().unwrap().starts_with(' ') {
            assert!(output.contains("xml:space=\"preserve\""));
        }
        apply_addressed_xml_mutation_in_place(&mut snapshot, &inverse).unwrap();
        assert_eq!(snapshot, before, "compact inverse restores exact canonical XML");
    }
}

#[test]
fn identical_natural_run_text_is_a_no_op_without_restructuring_xml() {
    let mut snapshot = fixture();
    let before = snapshot.clone();
    let run = docx_top_level_run_at(&snapshot, 0, 0).unwrap();
    let mutation = DocxMutation::SetRunText(set_run_text::SetRunText { address: run.address, text: run.text });
    let prepared = prepare_addressed_xml_mutation(&snapshot, &mutation).unwrap();
    assert!(!prepared.changed);
    assert_eq!(prepared.diff, DocxDiff::default());
    apply_addressed_xml_mutation_in_place(&mut snapshot, &mutation).unwrap();
    assert_eq!(snapshot, before);
}

#[semio_framework_async_macros::async_test]
async fn snapshot_pack_retains_complete_xml_authority() {
    let snapshot = fixture();
    let bytes = store::ArtifactPack::encode_pack(&snapshot);
    let decoded = <DocxSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snapshot);
    assert_eq!(project(&decoded), project(&snapshot));
}

#[test]
fn kinds_const_matches_enum_variants_in_declaration_order() {
    let cases = demo_mutation_cases();
    assert_eq!(cases.len(), KINDS.len());
    for (mutation, kind) in cases.iter().zip(KINDS.iter()) {
        assert_eq!(mutation.print_op().split(' ').next().unwrap(), *kind);
    }
}

#[semio_framework_async_macros::async_test]
async fn top_level_table_projection_matches_independent_xml_text_order() {
    use quick_xml::{Reader, events::Event};
    use semio_s_artifact_stdio_xml::schema::snapshot::xml_document_from_text;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🧭️table-run-projection/🔣️.json")).unwrap();
    let xml = fixture["xml"].as_str().unwrap();
    let mut reader = Reader::from_str(xml);
    let mut oracle = Vec::new();
    loop {
        match reader.read_event().unwrap() {
            Event::Text(text) => oracle.push(text.xml10_content().into_owned()),
            Event::Eof => break,
            _ => {}
        }
    }
    let expected: Vec<Vec<String>> = serde_json::from_value(fixture["blocks"].clone()).unwrap();
    assert_eq!(oracle, expected.iter().flatten().cloned().collect::<Vec<_>>());
    let mut snapshot = crate::engine::build_minimal_docx(DocxDocument::default());
    let part_path = crate::standards::v_ecma_376::subsets::base::io::import::deserializers::main_document_path(&snapshot.opc).unwrap();
    snapshot.xml_part_mut(&part_path).unwrap().document = xml_document_from_text(xml).unwrap();
    assert_eq!(docx_top_level_block_count(&snapshot).unwrap(), expected.len());
    for (block, runs) in expected.iter().enumerate() {
        assert_eq!(docx_top_level_run_count(&snapshot, block).unwrap(), runs.len());
        for (index, text) in runs.iter().enumerate() {
            let run = docx_top_level_run_at(&snapshot, block, index).unwrap();
            assert_eq!(&run.text, text);
            assert_eq!(run.address.node_path, serde_json::from_value::<Vec<usize>>(fixture["paths"][block][index].clone()).unwrap());
            assert_eq!(run.address.expected_name, fixture["expectedName"].as_str().unwrap());
            assert_eq!(run.address.part_path, part_path);
            resolve_docx_xml_address(&snapshot, &run.address).unwrap();
        }
    }
    println!("[DEBUG] DOCX top-level projections retain all five paragraph/nested-table runs and canonical physical XML addresses");
}
