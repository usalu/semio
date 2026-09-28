mod tests {
    use super::*;
    use crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx;
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxSheet, XlsxWorkbook};
    use semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn attr(name: &str, value: &str) -> XmlAttr {
        XmlAttr { name: name.into(), value: value.into() }
    }

    /// 📄️ A real one-sheet package (`build_minimal_xlsx`: root officeDocument relationship, workbook-owned
    /// worksheet + shared-strings relationships, every XML part in the XML authority lane) whose workbook root
    /// declares exactly the given namespaces and `conformance` attribute.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snapshot_with_workbook(xmlns: &str, xmlns_r: &str, conformance: Option<&str>) -> XlsxSnapshot {
        let mut snapshot = build_minimal_xlsx(XlsxWorkbook { sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: Vec::new() }], shared_strings: Vec::new() });
        let part = snapshot.xml_part_mut(WORKBOOK_PART).expect("the minimal package carries its workbook part");
        let Some(XmlNode::Element { attrs, .. }) = &mut part.document.root else { panic!("the workbook part has a root element") };
        attrs.retain(|a| !matches!(a.name.as_str(), "xmlns" | "xmlns:r" | "conformance"));
        attrs.extend([attr("xmlns", xmlns), attr("xmlns:r", xmlns_r)]);
        attrs.extend(conformance.map(|c| attr("conformance", c)));
        snapshot
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_transitional_workbook_has_no_diagnostics() {
        let snapshot = snapshot_with_workbook(TRANSITIONAL_SML_NS, TRANSITIONAL_R_NS, None);
        snapshot.validate_authority().expect("the probe package is a valid XML authority");
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
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
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    /// 🏷️ The worksheet role comes from the workbook's worksheet relationship, not from the part's path or its
    /// declared content type: a worksheet the package types as plain `application/xml` is the soft violation.
    #[semio_framework_async_macros::async_test]
    async fn worksheet_wrong_content_type_is_soft() {
        let mut snapshot = snapshot_with_workbook(TRANSITIONAL_SML_NS, TRANSITIONAL_R_NS, None);
        snapshot.opc.content_types.set_override("xl/worksheets/sheet1.xml", "application/xml");
        snapshot.xml_part_mut("xl/worksheets/sheet1.xml").expect("the one-sheet package carries its worksheet part").content_type = "application/xml".into();
        snapshot.validate_authority().expect("the retyped worksheet stays a valid XML authority");
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_WORKSHEET_CONTENT_TYPE && d.severity == Severity::Warning), "got {diagnostics:?}");
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }
}
