mod tests {
    use super::*;
    use semio_s_artifact_stdio_zip::opc::{OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

    const STRICT_PRESENTATION_XML: &str = concat!(
        r#"<p:presentation xmlns:a="http://purl.oclc.org/ooxml/drawingml/main" xmlns:p="http://purl.oclc.org/ooxml/presentationml/main" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships" conformance="strict">"#,
        r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
        r#"<p:sldIdLst/>"#,
        "</p:presentation>",
    );

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn strict_snapshot() -> PptxSnapshot {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", STRICT_PRESENTATION_XML.as_bytes().to_vec());
        opc.add_relationship("", "rId1", "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument", "ppt/presentation.xml");
        PptxSnapshot { opc, ..PptxSnapshot::default() }
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_strict_snapshot_reports_nothing() {
        let diagnostics = check_strict_conformance(&strict_snapshot());
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn transitional_main_ns_on_root_part_is_hard() {
        let mut snapshot = strict_snapshot();
        let transitional_xml = STRICT_PRESENTATION_XML.replace(STRICT_MAIN_NS, TRANSITIONAL_MAIN_NS);
        snapshot.opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", transitional_xml.into_bytes());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MAIN_NS && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn transitional_namespace_anywhere_in_package_is_hard() {
        let mut snapshot = strict_snapshot();
        snapshot.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", format!("<p:sld xmlns:p=\"{TRANSITIONAL_MAIN_NS}\"/>").into_bytes());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_TRANSITIONAL_NS_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn vml_markup_is_hard() {
        let mut snapshot = strict_snapshot();
        snapshot.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", format!("<v:shape xmlns:v=\"{VML_NS}\"/>").into_bytes());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_VML_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn transitional_relationship_base_is_hard() {
        let mut snapshot = strict_snapshot();
        snapshot.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", b"<p:sld/>".to_vec());
        snapshot.opc.add_relationship("ppt/presentation.xml", "rId2", REL_TYPE_OFFICE_DOCUMENT, "slides/slide1.xml");
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_REL_BASE && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_conformance_attribute_is_soft() {
        let mut snapshot = strict_snapshot();
        let no_conformance = STRICT_PRESENTATION_XML.replace(" conformance=\"strict\"", "");
        snapshot.opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", no_conformance.into_bytes());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTR && d.severity == Severity::Warning), "got {diagnostics:?}");
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn alternate_content_markup_is_soft() {
        let mut snapshot = strict_snapshot();
        snapshot.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", b"<mc:AlternateContent/>".to_vec());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ALTERNATE_CONTENT && d.severity == Severity::Warning), "got {diagnostics:?}");
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_office_document_relationship_is_hard() {
        let snapshot = PptxSnapshot::default();
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MAIN_NS && d.severity == Severity::Error), "got {diagnostics:?}");
    }
}
