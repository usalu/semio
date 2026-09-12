mod tests {
    use super::*;

    fn basic_doc() -> MarkupDoc {
        parse_markup(br#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1"><defs><clipPath id="shape"><path d="M0 0H8V8H0Z"/></clipPath><clipPath id="lettering"><text>hi</text></clipPath></defs><g clip-path="url(#shape)"><rect x="0" y="0" width="8" height="8"/></g></svg>"#).expect("parses")
    }

    fn element_node(name: &str) -> Json {
        Json::Object(vec![("kind".into(), Json::String("element".into())), ("name".into(), Json::String(name.into()))])
    }

    #[test]
    fn insert_basic_element_accepts_a_retained_filter_primitive() {
        let mut doc = basic_doc();
        let filter = Json::Object(vec![
            ("kind".into(), Json::String("element".into())),
            ("name".into(), Json::String("filter".into())),
            ("attrs".into(), Json::Array(vec![Json::Object(vec![("name".into(), Json::String("id".into())), ("value".into(), Json::String("blur".into()))])])),
            ("children".into(), Json::Array(vec![element_node("feGaussianBlur")])),
        ]);
        let params = Json::Object(vec![("parent".into(), Json::Array(vec![Json::Number(0.0)])), ("index".into(), Json::Number(0.0)), ("node".into(), filter)]);
        apply(&mut doc, "insert-basic-element", &params).expect("feGaussianBlur is retained by SVG Basic 1.1");
    }

    #[test]
    fn insert_basic_element_rejects_an_excluded_primitive() {
        let mut doc = basic_doc();
        let filter = Json::Object(vec![("kind".into(), Json::String("element".into())), ("name".into(), Json::String("filter".into())), ("children".into(), Json::Array(vec![element_node("feTurbulence")]))]);
        let params = Json::Object(vec![("parent".into(), Json::Array(vec![Json::Number(0.0)])), ("index".into(), Json::Number(0.0)), ("node".into(), filter)]);
        assert!(apply(&mut doc, "insert-basic-element", &params).is_err(), "feTurbulence is outside SVG Basic 1.1");
    }

    #[test]
    fn set_clip_path_reference_rejects_a_clip_path_that_clips_to_text() {
        let mut doc = basic_doc();
        let params = Json::Object(vec![("path".into(), Json::Array(vec![Json::Number(1.0)])), ("clipPathId".into(), Json::String("lettering".into()))]);
        assert!(apply(&mut doc, "set-clip-path-reference", &params).is_err(), "SVG Basic 1.1 does not support clipping to text");
    }

    #[test]
    fn set_clip_path_reference_accepts_a_shape_only_clip_path() {
        let mut doc = basic_doc();
        let params = Json::Object(vec![("path".into(), Json::Array(vec![Json::Number(1.0)])), ("clipPathId".into(), Json::String("shape".into()))]);
        apply(&mut doc, "set-clip-path-reference", &params).expect("a shape-only clipPath is legal in SVG Basic 1.1");
    }

    #[test]
    fn insert_clip_path_shape_rejects_a_text_shape() {
        let mut doc = basic_doc();
        let params = Json::Object(vec![("clipPathId".into(), Json::String("shape".into())), ("index".into(), Json::Number(0.0)), ("node".into(), element_node("text"))]);
        assert!(apply(&mut doc, "insert-clip-path-shape", &params).is_err(), "adding text to a clip path would clip to text");
    }

    #[test]
    fn unknown_kind_is_an_error_not_a_no_op() {
        let mut doc = basic_doc();
        assert!(apply(&mut doc, "strip-non-tiny", &Json::Object(Vec::new())).is_err(), "a sibling subset's kind must never be silently skipped here");
    }
}
