mod tests {
    use super::*;
    use crate::schema::vocabulary::MAIN_DOCUMENT_PART;
    use crate::standards::v_ecma_376::subsets::strict::schema::conformance::STRICT_MAIN_NS;

    #[semio_framework_async_macros::async_test]
    async fn empty_builder_is_strict_conformant() {
        let snapshot = DocxStrictBuilderConstruction::empty().build().expect("empty strict builder must be conformant");
        assert!(snapshot.xml_part(MAIN_DOCUMENT_PART).is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn add_paragraph_stays_strict_conformant() {
        let snapshot = DocxStrictBuilderConstruction::empty().add_text_paragraph("Hello, strict world!").build().expect("must build");
        assert_eq!(snapshot.project_document().unwrap().body.len(), 1);
        assert!(xml_document_to_text(&snapshot.xml_part(MAIN_DOCUMENT_PART).unwrap().document.materialize_exact().unwrap()).contains(STRICT_MAIN_NS));
    }

    #[semio_framework_async_macros::async_test]
    async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut snapshot = DocxStrictBuilderConstruction::empty().add_text_paragraph("clean").build().unwrap();
        snapshot.xml_parts.try_push(DocxXmlPart {
            path: "word/legacyDrawing.xml".into(),
            content_type: "application/xml".into(),
            document: semio_s_artifact_stdio_xml::schema::snapshot::retained::RetainedXmlDocument::try_from_document(&semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text("<v:shape xmlns:v=\"urn:schemas-microsoft-com:vml\"/>").unwrap()).unwrap(),
        }).unwrap();
        snapshot.opc.content_types.set_override("word/legacyDrawing.xml", "application/xml");
        let mutated = DocxStrictBuilderConstruction::from_snapshot(snapshot);
        let err = mutated.build().expect_err("VML content must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::standards::v_ecma_376::subsets::strict::schema::conformance::CODE_VML_PRESENT));
    }
}
