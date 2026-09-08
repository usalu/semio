mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn svg_root(attrs: Vec<XmlAttr>, children: Vec<XmlNode>) -> SvgSnapshot {
        let mut snapshot = SvgSnapshot::default();
        snapshot.doc.root = Some(XmlNode::Element { name: "svg".into(), attrs, children });
        snapshot
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn attr(name: &str, value: &str) -> XmlAttr {
        XmlAttr { name: name.into(), value: value.into() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn elem(name: &str, attrs: Vec<XmlAttr>) -> XmlNode {
        XmlNode::Element { name: name.into(), attrs, children: vec![] }
    }

    #[semio_framework_async_macros::async_test]
    async fn fully_conforming_document_reports_no_diagnostics() {
        let snapshot = svg_root(vec![attr("baseProfile", "tiny"), attr("version", "1.1")], vec![elem("rect", vec![attr("x", "0"), attr("y", "0"), attr("width", "10"), attr("height", "10")])]);
        let diagnostics = check_svg_tiny_conformance(&snapshot);
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn blocklisted_element_is_hard() {
        let snapshot = svg_root(vec![attr("baseProfile", "tiny"), attr("version", "1.1")], vec![elem("linearGradient", vec![])]);
        let diagnostics = check_svg_tiny_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ELEMENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn filter_primitive_by_prefix_is_hard() {
        let snapshot = svg_root(vec![attr("baseProfile", "tiny"), attr("version", "1.1")], vec![elem("feGaussianBlur", vec![])]);
        let diagnostics = check_svg_tiny_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ELEMENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn blocklisted_attribute_is_hard() {
        let snapshot = svg_root(vec![attr("baseProfile", "tiny"), attr("version", "1.1")], vec![elem("rect", vec![attr("opacity", "0.5")])]);
        let diagnostics = check_svg_tiny_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ATTRIBUTE && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_base_profile_is_soft() {
        let snapshot = svg_root(vec![], vec![]);
        let diagnostics = check_svg_tiny_conformance(&snapshot);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code.0, CODE_BASE_PROFILE);
        assert_eq!(diagnostics[0].severity, Severity::Warning);
    }

    #[semio_framework_async_macros::async_test]
    async fn external_href_is_soft() {
        let snapshot = svg_root(vec![attr("baseProfile", "tiny"), attr("version", "1.1")], vec![elem("use", vec![attr("xlink:href", "http://example.com/sprite.svg#icon")])]);
        let diagnostics = check_svg_tiny_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_EXTERNAL_HREF && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn same_document_fragment_href_is_clean() {
        let snapshot = svg_root(vec![attr("baseProfile", "tiny"), attr("version", "1.1")], vec![elem("use", vec![attr("href", "#icon")])]);
        let diagnostics = check_svg_tiny_conformance(&snapshot);
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }
}
