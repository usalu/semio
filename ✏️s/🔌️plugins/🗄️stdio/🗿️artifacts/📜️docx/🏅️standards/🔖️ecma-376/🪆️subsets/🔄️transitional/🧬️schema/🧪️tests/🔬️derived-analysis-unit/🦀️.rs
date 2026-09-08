mod tests {
    use super::*;
    use semio_s_artifact_stdio_zip::opc::{OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn transitional_document_bytes() -> Vec<u8> {
        format!(r#"<w:document xmlns:w="{TRANSITIONAL_MAIN_NS}"><w:body/></w:document>"#).into_bytes()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snapshot_with_main_part(rel_type: &str, doc_bytes: Vec<u8>) -> DocxSnapshot {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", doc_bytes);
        opc.add_relationship("", "rId1", rel_type, "word/document.xml");
        DocxSnapshot::from_parts(opc, Default::default())
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_transitional_document_has_no_hard_diagnostics() {
        let snapshot = snapshot_with_main_part(REL_TYPE_OFFICE_DOCUMENT, transitional_document_bytes());
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
        assert!(diagnostics.iter().all(|d| d.code.0 != CODE_CONFORMANCE_ATTR), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_transitional_namespace_is_hard() {
        let snapshot = snapshot_with_main_part(REL_TYPE_OFFICE_DOCUMENT, b"<w:document><w:body/></w:document>".to_vec());
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MAIN_NS_MISSING && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn strict_namespace_anywhere_is_hard() {
        let mut snapshot = snapshot_with_main_part(REL_TYPE_OFFICE_DOCUMENT, transitional_document_bytes());
        snapshot.opc.set_part("word/styles.xml", "application/xml", b"<w:styles xmlns:w=\"http://purl.oclc.org/ooxml/wordprocessingml/main\"/>".to_vec());
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRICT_NS_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn strict_relationship_base_anywhere_is_hard() {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", transitional_document_bytes());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "word/document.xml");
        opc.add_relationship("word/document.xml", "rId2", "http://purl.oclc.org/ooxml/officeDocument/relationships/image", "media/image1.png");
        let snapshot = DocxSnapshot::from_parts(opc, Default::default());
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRICT_NS_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn conformance_strict_attribute_present_is_soft() {
        let doc = format!(r#"<w:document xmlns:w="{TRANSITIONAL_MAIN_NS}" conformance="strict"><w:body/></w:document>"#).into_bytes();
        let snapshot = snapshot_with_main_part(REL_TYPE_OFFICE_DOCUMENT, doc);
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTR && d.severity == Severity::Warning), "got {diagnostics:?}");
    }
}
