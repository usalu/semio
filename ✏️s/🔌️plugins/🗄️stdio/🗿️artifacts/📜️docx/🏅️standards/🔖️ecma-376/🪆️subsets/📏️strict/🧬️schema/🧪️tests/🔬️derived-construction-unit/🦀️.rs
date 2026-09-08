mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn empty_builder_is_strict_conformant() {
        let snapshot = DocxStrictBuilderConstruction::empty().build().expect("empty strict builder must be conformant");
        assert!(snapshot.opc.part_bytes(MAIN_DOCUMENT_PART).is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn add_paragraph_stays_strict_conformant() {
        let snapshot = DocxStrictBuilderConstruction::empty().add_text_paragraph("Hello, strict world!").build().expect("must build");
        assert_eq!(snapshot.document.body.len(), 1);
        let bytes = snapshot.opc.part_bytes(MAIN_DOCUMENT_PART).unwrap();
        assert!(String::from_utf8_lossy(bytes).contains(STRICT_MAIN_NS));
    }

    #[semio_framework_async_macros::async_test]
    async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut snapshot = DocxStrictBuilderConstruction::empty().add_text_paragraph("clean").build().unwrap();
        snapshot.opc.set_part("word/legacyDrawing.xml", "application/xml", b"<v:shape xmlns:v=\"urn:schemas-microsoft-com:vml\"/>".to_vec());
        let (mutated, _diff) = DocxStrictBuilderConstruction::from_snapshot(DocxSnapshot::default()).mutate(DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }));
        let err = mutated.build().expect_err("VML content must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::standards::v_ecma_376::subsets::strict::schema::CODE_VML_PRESENT));
    }
}
