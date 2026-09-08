mod tests {
    use super::*;
    use semio_s_artifact_stdio_zip::opc::{OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn strict_document_bytes() -> Vec<u8> {
        format!(r#"<w:document xmlns:w="{STRICT_MAIN_NS}" conformance="strict"><w:body/></w:document>"#).into_bytes()
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
    async fn conforming_strict_document_has_no_hard_diagnostics() {
        let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
        let snapshot = snapshot_with_main_part(&rel_type, strict_document_bytes());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_strict_namespace_is_hard() {
        let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
        let snapshot = snapshot_with_main_part(&rel_type, b"<w:document><w:body/></w:document>".to_vec());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MAIN_NS_MISSING && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn transitional_namespace_anywhere_is_hard() {
        let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
        let mut snapshot = snapshot_with_main_part(&rel_type, strict_document_bytes());
        snapshot.opc.set_part("word/styles.xml", "application/xml", format!(r#"<w:styles xmlns:w="{TRANSITIONAL_MAIN_NS}"/>"#).into_bytes());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_TRANSITIONAL_NS_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn vml_namespace_anywhere_is_hard() {
        let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
        let mut snapshot = snapshot_with_main_part(&rel_type, strict_document_bytes());
        snapshot.opc.set_part("word/header1.xml", "application/xml", format!(r#"<w:hdr xmlns:v="{VML_NS}"/>"#).into_bytes());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_VML_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn transitional_relationship_base_is_hard() {
        let snapshot = snapshot_with_main_part(REL_TYPE_OFFICE_DOCUMENT, strict_document_bytes());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_REL_BASE && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_conformance_attr_is_soft() {
        let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
        let doc = format!(r#"<w:document xmlns:w="{STRICT_MAIN_NS}"><w:body/></w:document>"#).into_bytes();
        let snapshot = snapshot_with_main_part(&rel_type, doc);
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTR && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn alternate_content_anywhere_is_soft() {
        let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
        let mut snapshot = snapshot_with_main_part(&rel_type, strict_document_bytes());
        snapshot.opc.set_part("word/document2.xml", "application/xml", b"<mc:AlternateContent/>".to_vec());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ALTERNATE_CONTENT && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_officedocument_relationship_is_hard() {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        let snapshot = DocxSnapshot::from_parts(opc, Default::default());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MAIN_NS_MISSING && d.severity == Severity::Error), "got {diagnostics:?}");
    }
}
