mod tests {
    use super::*;
    use semio_s_artifact_stdio_zip::opc::{OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

    fn set_xml_part(snapshot: &mut PptxSnapshot, path: &str, content_type: &str, bytes: Vec<u8>) {
        let document = semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text(std::str::from_utf8(&bytes).unwrap()).unwrap();
        snapshot.opc.content_types.set_override(path, content_type);
        if let Some(part) = snapshot.xml_parts.iter_mut().find(|part| part.path == path) { part.document = document; part.content_type = content_type.into(); }
        else { snapshot.xml_parts.push(crate::schema::snapshot::PptxXmlPart { path: path.into(), content_type: content_type.into(), document }); }
    }

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
        opc.content_types.set_override("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml");
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "ppt/presentation.xml");
        let mut snapshot = PptxSnapshot { opc, xml_parts: Vec::new(), ..PptxSnapshot::default() };
        set_xml_part(&mut snapshot, "ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", TRANSITIONAL_PRESENTATION_XML.as_bytes().to_vec());
        snapshot
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
        set_xml_part(&mut snapshot, "ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", strict_xml.into_bytes());
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn strict_namespace_anywhere_in_package_is_hard() {
        let mut snapshot = transitional_snapshot();
        set_xml_part(&mut snapshot, "ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", b"<p:sld xmlns:p=\"http://purl.oclc.org/ooxml/presentationml/main\"/>".to_vec());
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRICT_NS_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn strict_relationship_base_is_hard() {
        let mut snapshot = transitional_snapshot();
        set_xml_part(&mut snapshot, "ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", b"<p:sld/>".to_vec());
        snapshot.opc.add_relationship("ppt/presentation.xml", "rId2", "http://purl.oclc.org/ooxml/officeDocument/relationships/slide", "slides/slide1.xml");
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRICT_NS_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn explicit_strict_conformance_attribute_is_soft() {
        let mut snapshot = transitional_snapshot();
        let with_conformance = TRANSITIONAL_PRESENTATION_XML.replace("<p:presentation ", "<p:presentation conformance=\"strict\" ");
        set_xml_part(&mut snapshot, "ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", with_conformance.into_bytes());
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTR && d.severity == Severity::Warning), "got {diagnostics:?}");
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_office_document_relationship_is_hard() {
        let mut snapshot = PptxSnapshot::default();
        snapshot.opc.relationships = Default::default();
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MAIN_NS && d.severity == Severity::Error), "got {diagnostics:?}");
    }
}
