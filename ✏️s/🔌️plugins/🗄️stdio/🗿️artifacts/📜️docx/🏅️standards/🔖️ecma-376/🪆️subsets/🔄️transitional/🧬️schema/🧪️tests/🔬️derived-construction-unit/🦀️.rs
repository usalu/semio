mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn empty_builder_is_transitional_conformant() {
        DocxTransitionalBuilderConstruction::empty().build().expect("empty transitional builder must be conformant");
    }

    #[semio_framework_async_macros::async_test]
    async fn add_paragraph_stays_transitional_conformant() {
        let snapshot = DocxTransitionalBuilderConstruction::empty().add_text_paragraph("Hello, transitional world!").build().expect("must build");
        assert_eq!(snapshot.project_document().unwrap().body.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut snapshot = DocxTransitionalBuilderConstruction::empty().add_text_paragraph("clean").build().unwrap();
        snapshot.xml_parts.push(crate::schema::snapshot::DocxXmlPart {
            path: "word/strict-extra.xml".into(),
            content_type: "application/xml".into(),
            document: semio_s_artifact_stdio_xml::schema::snapshot::xml_document_from_text("<w:styles xmlns:w=\"http://purl.oclc.org/ooxml/wordprocessingml/main\"/>").unwrap(),
        });
        snapshot.opc.content_types.set_override("word/strict-extra.xml", "application/xml");
        let (mutated, _diff) = DocxTransitionalBuilderConstruction::from_snapshot(DocxSnapshot::default()).mutate(DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }));
        let err = mutated.build().expect_err("mixed-in strict namespace must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::standards::v_ecma_376::subsets::transitional::schema::CODE_STRICT_NS_PRESENT));
    }
}
