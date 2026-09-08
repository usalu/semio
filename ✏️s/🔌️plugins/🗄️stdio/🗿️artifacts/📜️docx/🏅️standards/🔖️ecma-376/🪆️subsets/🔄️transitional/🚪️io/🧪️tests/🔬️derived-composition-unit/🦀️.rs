mod tests {
    use super::*;
    use crate::standards::v_ecma_376::subsets::transitional::schema::CODE_STRICT_NS_PRESENT;
    use semio_framework_plugin::AnalyzeSource;
    use semio_s_artifact_stdio_zip::opc::{OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

    const TRANSITIONAL_MAIN_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn transitional_snapshot() -> DocxSnapshot {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", format!(r#"<w:document xmlns:w="{TRANSITIONAL_MAIN_NS}"><w:body/></w:document>"#).into_bytes());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "word/document.xml");
        DocxSnapshot::from_parts(opc, Default::default())
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_snapshot_composes_and_stamps_transitional() {
        let bytes = <DocxSnapshot as store::ArtifactPack>::encode_pack(&transitional_snapshot());
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = DocxTransitionalComposerComposition::compose(&sources).expect("clean transitional document must compose");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "got {:?}", composed.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn strict_namespace_present_fails_compose_with_real_diagnostic() {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", format!(r#"<w:document xmlns:w="{TRANSITIONAL_MAIN_NS}"><w:body/></w:document>"#).into_bytes());
        opc.set_part("word/styles.xml", "application/xml", b"<w:styles xmlns:w=\"http://purl.oclc.org/ooxml/wordprocessingml/main\"/>".to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "word/document.xml");
        let snapshot = DocxSnapshot::from_parts(opc, Default::default());
        let bytes = <DocxSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let err = DocxTransitionalComposerComposition::compose(&sources).expect_err("mixed-in strict namespace must not stamp transitional");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_STRICT_NS_PRESENT && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_rechecks_wire_payload() {
        let bytes = <DocxSnapshot as store::ArtifactPack>::encode_pack(&transitional_snapshot());
        let diagnostics = DocxTransitionalValidator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }
}
