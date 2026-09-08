mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn empty_builder_is_transitional_conformant() {
        DocxTransitionalBuilderConstruction::empty().build().expect("empty transitional builder must be conformant");
    }

    #[semio_framework_async_macros::async_test]
    async fn add_paragraph_stays_transitional_conformant() {
        let snapshot = DocxTransitionalBuilderConstruction::empty().add_text_paragraph("Hello, transitional world!").build().expect("must build");
        assert_eq!(snapshot.document.body.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut snapshot = DocxTransitionalBuilderConstruction::empty().add_text_paragraph("clean").build().unwrap();
        snapshot.opc.set_part("word/styles.xml", "application/xml", b"<w:styles xmlns:w=\"http://purl.oclc.org/ooxml/wordprocessingml/main\"/>".to_vec());
        let (mutated, _diff) = DocxTransitionalBuilderConstruction::from_snapshot(DocxSnapshot::default()).mutate(DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }));
        let err = mutated.build().expect_err("mixed-in strict namespace must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::standards::v_ecma_376::subsets::transitional::schema::CODE_STRICT_NS_PRESENT));
    }
}
