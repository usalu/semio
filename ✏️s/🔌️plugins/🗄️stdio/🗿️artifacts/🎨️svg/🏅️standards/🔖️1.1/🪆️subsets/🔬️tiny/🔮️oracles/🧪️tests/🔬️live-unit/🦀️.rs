mod tests {
    use super::*;

    fn tiny_doc() -> MarkupDoc {
        parse_markup(br#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1"><g style="fill:#000"><rect x="0" y="0" width="4" height="4"/></g><filter id="f1"><feTurbulence/></filter></svg>"#).expect("parses")
    }

    #[test]
    fn strip_non_tiny_removes_excluded_elements_and_attributes() {
        let mut doc = tiny_doc();
        apply(&mut doc, "strip-non-tiny", &Json::Object(Vec::new())).expect("strip applies");
        let rendered = String::from_utf8(write_markup(&doc).expect("writes")).expect("utf8");
        assert!(!rendered.contains("filter"), "the excluded <filter> subtree must be gone: {rendered}");
        assert!(!rendered.contains("style="), "the excluded style attribute must be gone: {rendered}");
        assert!(rendered.contains("<rect"), "retained geometry must survive: {rendered}");
    }

    #[test]
    fn insert_tiny_element_rejects_an_excluded_subtree() {
        let mut doc = tiny_doc();
        let node = Json::Object(vec![("kind".into(), Json::String("element".into())), ("name".into(), Json::String("linearGradient".into()))]);
        let params = Json::Object(vec![("parent".into(), Json::Array(Vec::new())), ("index".into(), Json::Number(0.0)), ("node".into(), node)]);
        assert!(apply(&mut doc, "insert-tiny-element", &params).is_err(), "a linearGradient is outside SVG Tiny 1.1");
    }

    #[test]
    fn set_tiny_attribute_rejects_a_forbidden_presentation_attribute() {
        let mut doc = tiny_doc();
        let params = Json::Object(vec![("path".into(), Json::Array(Vec::new())), ("name".into(), Json::String("opacity".into())), ("value".into(), Json::String("0.5".into()))]);
        assert!(apply(&mut doc, "set-tiny-attribute", &params).is_err(), "opacity is forbidden anywhere in SVG Tiny 1.1");
    }

    #[test]
    fn unknown_kind_is_an_error_not_a_no_op() {
        let mut doc = tiny_doc();
        assert!(apply(&mut doc, "set-element-name", &Json::Object(Vec::new())).is_err(), "a kind this subset does not declare must never be silently skipped");
    }
}
