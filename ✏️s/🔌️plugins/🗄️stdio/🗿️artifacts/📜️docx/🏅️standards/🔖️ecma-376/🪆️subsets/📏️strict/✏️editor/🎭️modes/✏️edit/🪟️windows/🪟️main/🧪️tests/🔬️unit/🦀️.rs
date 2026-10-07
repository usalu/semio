use super::*;
use crate::schema::snapshot::{DocxBlock, DocxDocument};
use crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_docx;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_document_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_emits_one_page_per_top_level_block() {
    let document = build_minimal_docx(DocxDocument { body: vec![DocxBlock::paragraph("first"), DocxBlock::paragraph("second")], styles: Vec::new() });
    let stack = render(&document, semio_framework_plugin::UiPublicationRevision(23)).expect("render");
    assert_eq!(stack.children.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn nested_table_text_remains_editable_in_the_document_window() {
    use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🧱️base/🧫️fixtures/🧭️table-run-projection/🔣️.json")).unwrap();
    let mut snapshot = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_docx(crate::schema::snapshot::DocxDocument::default());
    let part_path = crate::standards::v_ecma_376::subsets::base::schema::inferences::document::main_document_path(&snapshot.opc).unwrap();
    snapshot.xml_part_mut(&part_path).unwrap().replace_document(xml_document_from_text(fixture["xml"].as_str().unwrap()).unwrap()).unwrap();
    let expected: Vec<Vec<String>> = serde_json::from_value(fixture["blocks"].clone()).unwrap();
    let pages = editable_pages(&snapshot).unwrap();
    assert_eq!(pages.iter().map(|page| page.text.as_str()).collect::<Vec<_>>(), expected.iter().flatten().map(String::as_str).collect::<Vec<_>>());
    assert!(pages.iter().all(|page| matches!(&page.arguments, Some(UiValue::Map(_)))));
    for locale in [Locale::En, Locale::De] {
        let rendered = render_windowed(&snapshot, &TreeWindows::unhosted(), locale, semio_framework_plugin::UiPublicationRevision::default()).unwrap();
        assert_eq!(rendered.children.len(), 5);
    }
}
