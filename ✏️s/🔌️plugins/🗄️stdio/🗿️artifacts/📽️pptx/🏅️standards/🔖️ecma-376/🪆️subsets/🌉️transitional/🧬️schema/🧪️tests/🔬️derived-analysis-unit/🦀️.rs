mod tests {
    use super::*;
    use semio_s_artifact_stdio_zip::opc::{OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

    const TRANSITIONAL_PRESENTATION_XML: &str = concat!(
        r#"<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
        r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
        r#"<p:sldIdLst/>"#,
        "</p:presentation>",
    );

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn transitional_snapshot() -> PptxSnapshot {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", TRANSITIONAL_PRESENTATION_XML.as_bytes().to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "ppt/presentation.xml");
        PptxSnapshot { opc, ..PptxSnapshot::default() }
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_transitional_snapshot_reports_nothing() {
        let diagnostics = check_transitional_conformance(&transitional_snapshot());
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn strict_main_ns_on_root_part_is_hard() {
        let mut snapshot = transitional_snapshot();
        let strict_xml = TRANSITIONAL_PRESENTATION_XML.replace(TRANSITIONAL_MAIN_NS, "http://purl.oclc.org/ooxml/presentationml/main");
        snapshot.opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", strict_xml.into_bytes());
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn strict_namespace_anywhere_in_package_is_hard() {
        let mut snapshot = transitional_snapshot();
        snapshot.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", b"<p:sld xmlns:p=\"http://purl.oclc.org/ooxml/presentationml/main\"/>".to_vec());
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRICT_NS_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn strict_relationship_base_is_hard() {
        let mut snapshot = transitional_snapshot();
        snapshot.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", b"<p:sld/>".to_vec());
        snapshot.opc.add_relationship("ppt/presentation.xml", "rId2", "http://purl.oclc.org/ooxml/officeDocument/relationships/slide", "slides/slide1.xml");
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRICT_NS_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn explicit_strict_conformance_attribute_is_soft() {
        let mut snapshot = transitional_snapshot();
        let with_conformance = TRANSITIONAL_PRESENTATION_XML.replace("<p:presentation ", "<p:presentation conformance=\"strict\" ");
        snapshot.opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", with_conformance.into_bytes());
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTR && d.severity == Severity::Warning), "got {diagnostics:?}");
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_office_document_relationship_is_hard() {
        let snapshot = PptxSnapshot::default();
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MAIN_NS && d.severity == Severity::Error), "got {diagnostics:?}");
    }
}
