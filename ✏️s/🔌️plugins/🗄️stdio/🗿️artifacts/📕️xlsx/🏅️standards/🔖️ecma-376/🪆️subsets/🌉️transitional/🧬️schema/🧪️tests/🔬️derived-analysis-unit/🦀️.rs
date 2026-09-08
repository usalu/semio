mod tests {
    use super::*;
    use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDocument, xml_document_to_text};
    use semio_s_artifact_stdio_zip::opc::OpcPackage;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn attr(name: &str, value: &str) -> XmlAttr {
        XmlAttr { name: name.into(), value: value.into() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn workbook_xml(xmlns: &str, xmlns_r: &str, conformance: Option<&str>) -> Vec<u8> {
        let mut attrs = vec![attr("xmlns", xmlns), attr("xmlns:r", xmlns_r)];
        if let Some(c) = conformance {
            attrs.push(attr("conformance", c));
        }
        let doc = XmlDocument { root: Some(XmlNode::Element { name: "workbook".into(), attrs, children: vec![XmlNode::Element { name: "sheets".into(), attrs: vec![], children: vec![] }] }), doctype: None, declaration: None, prolog: Vec::new() };
        xml_document_to_text(&doc).into_bytes()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snapshot_with_workbook(xmlns: &str, xmlns_r: &str, conformance: Option<&str>) -> XlsxSnapshot {
        let mut opc = OpcPackage::empty();
        opc.set_part(WORKBOOK_PART, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml", workbook_xml(xmlns, xmlns_r, conformance));
        XlsxSnapshot::from_parts(opc, Default::default())
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_transitional_workbook_has_no_hard_diagnostics() {
        let snapshot = snapshot_with_workbook(TRANSITIONAL_SML_NS, TRANSITIONAL_R_NS, None);
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn strict_namespace_is_hard() {
        let snapshot = snapshot_with_workbook("http://purl.oclc.org/ooxml/spreadsheetml/main", "http://purl.oclc.org/ooxml/officeDocument/relationships", None);
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_NAMESPACE_MISMATCH && d.severity == Severity::Error), "got {diagnostics:?}");
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_RELATIONSHIPS_NAMESPACE_MISMATCH && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn explicit_strict_conformance_attribute_is_hard() {
        let snapshot = snapshot_with_workbook(TRANSITIONAL_SML_NS, TRANSITIONAL_R_NS, Some("strict"));
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTRIBUTE && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn explicit_transitional_conformance_attribute_is_fine() {
        let snapshot = snapshot_with_workbook(TRANSITIONAL_SML_NS, TRANSITIONAL_R_NS, Some("transitional"));
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn worksheet_wrong_content_type_is_soft() {
        let mut snapshot = snapshot_with_workbook(TRANSITIONAL_SML_NS, TRANSITIONAL_R_NS, None);
        snapshot.opc.set_part("xl/worksheets/sheet1.xml", "application/xml", b"<worksheet/>".to_vec());
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_WORKSHEET_CONTENT_TYPE && d.severity == Severity::Warning), "got {diagnostics:?}");
    }
}
