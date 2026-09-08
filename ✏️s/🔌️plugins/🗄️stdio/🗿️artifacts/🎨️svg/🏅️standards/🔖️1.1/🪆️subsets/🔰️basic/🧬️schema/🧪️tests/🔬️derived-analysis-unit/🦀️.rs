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
    fn elem(name: &str, attrs: Vec<XmlAttr>, children: Vec<XmlNode>) -> XmlNode {
        XmlNode::Element { name: name.into(), attrs, children }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base_attrs() -> Vec<XmlAttr> {
        vec![attr("baseProfile", "basic"), attr("version", "1.1")]
    }

    #[semio_framework_async_macros::async_test]
    async fn fully_conforming_document_reports_no_diagnostics() {
        let snapshot = svg_root(base_attrs(), vec![elem("rect", vec![attr("x", "0"), attr("y", "0"), attr("width", "10"), attr("height", "10")], vec![])]);
        let diagnostics = check_svg_basic_conformance(&snapshot);
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn blocklisted_filter_primitive_is_hard() {
        let snapshot = svg_root(base_attrs(), vec![elem("filter", vec![attr("id", "f1")], vec![elem("feTurbulence", vec![], vec![])])]);
        let diagnostics = check_svg_basic_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_FILTER_PRIMITIVE && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn lighting_child_primitive_is_hard() {
        let snapshot = svg_root(base_attrs(), vec![elem("filter", vec![attr("id", "f1")], vec![elem("feDiffuseLighting", vec![], vec![elem("feDistantLight", vec![], vec![])])])]);
        let diagnostics = check_svg_basic_conformance(&snapshot);
        assert_eq!(diagnostics.iter().filter(|d| d.code.0 == CODE_FILTER_PRIMITIVE).count(), 2, "expected both feDiffuseLighting and feDistantLight flagged: {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn clip_path_referencing_text_is_hard() {
        let snapshot = svg_root(base_attrs(), vec![elem("clipPath", vec![attr("id", "c1")], vec![elem("text", vec![], vec![XmlNode::Text { text: "hi".into() }])]), elem("rect", vec![attr("clip-path", "url(#c1)")], vec![])]);
        let diagnostics = check_svg_basic_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CLIP_PATH_TEXT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn clip_path_referencing_shapes_only_is_clean() {
        let snapshot = svg_root(base_attrs(), vec![elem("clipPath", vec![attr("id", "c1")], vec![elem("circle", vec![attr("cx", "5"), attr("cy", "5"), attr("r", "5")], vec![])]), elem("rect", vec![attr("clip-path", "url(#c1)")], vec![])]);
        let diagnostics = check_svg_basic_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.code.0 != CODE_CLIP_PATH_TEXT), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn nested_svg_is_soft() {
        let snapshot = svg_root(base_attrs(), vec![elem("svg", vec![attr("width", "5"), attr("height", "5")], vec![])]);
        let diagnostics = check_svg_basic_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_NESTED_SVG && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_base_profile_is_soft() {
        let snapshot = svg_root(vec![], vec![]);
        let diagnostics = check_svg_basic_conformance(&snapshot);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code.0, CODE_BASE_PROFILE);
        assert_eq!(diagnostics[0].severity, Severity::Warning);
    }
}
