mod tests {
    use super::*;
    use crate::standards::v1_1::subsets::tiny::schema::conformance::CODE_ELEMENT;
    use crate::schema::snapshot::SvgNode;

    #[semio_framework_async_macros::async_test]
    async fn empty_builder_injects_profile_and_builds_clean() {
        let snapshot = SvgTinyBuilderConstruction::empty().build().expect("empty document builds clean");
        match &snapshot.doc.root {
            Some(SvgNode::Element { attrs, .. }) => {
                assert!(attrs.iter().any(|a| a.name == "baseProfile" && a.value.text() == Some("tiny")));
                assert!(attrs.iter().any(|a| a.name == "version" && a.value.text() == Some("1.1")));
            }
            other => panic!("expected element root, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn hard_violation_read_from_text_still_fails_build() {
        let text = r#"<svg xmlns="http://www.w3.org/2000/svg"><script>alert(1)</script></svg>"#;
        let err = SvgTinyBuilderConstruction::from_text(text).expect("parses").build().expect_err("a <script> element must fail build()");
        assert!(err.iter().any(|d| d.code.0 == CODE_ELEMENT));
    }

    #[semio_framework_async_macros::async_test]
    async fn from_text_round_trips_through_tiny_build() {
        let text = r#"<svg xmlns="http://www.w3.org/2000/svg"><circle cx="5" cy="5" r="5"/></svg>"#;
        let built = SvgTinyBuilderConstruction::from_text(text).expect("parses").build().expect("conforming document builds");
        assert!(matches!(built.doc.root, Some(SvgNode::Element { .. })));
    }
}
