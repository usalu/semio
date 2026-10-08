mod tests {
    use super::*;
    use crate::standards::v1_1::subsets::basic::schema::conformance::CODE_FILTER_PRIMITIVE;
    use crate::schema::snapshot::SvgNode;

    #[semio_framework_async_macros::async_test]
    async fn empty_builder_injects_profile_and_builds_clean() {
        let snapshot = SvgBasicBuilderConstruction::empty().build().expect("empty document builds clean");
        match &snapshot.doc.root {
            Some(SvgNode::Element { attrs, .. }) => {
                assert!(attrs.iter().any(|a| a.name == "baseProfile" && a.value.text() == Some("basic")));
                assert!(attrs.iter().any(|a| a.name == "version" && a.value.text() == Some("1.1")));
            }
            other => panic!("expected element root, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn hard_violation_read_from_text_still_fails_build() {
        let text = r#"<svg xmlns="http://www.w3.org/2000/svg"><filter id="f1"><feMorphology/></filter></svg>"#;
        let err = SvgBasicBuilderConstruction::from_text(text).expect("parses").build().expect_err("a feMorphology primitive must fail build()");
        assert!(err.iter().any(|d| d.code.0 == CODE_FILTER_PRIMITIVE));
    }

    #[semio_framework_async_macros::async_test]
    async fn from_text_round_trips_through_basic_build() {
        let text = r#"<svg xmlns="http://www.w3.org/2000/svg"><circle cx="5" cy="5" r="5"/></svg>"#;
        let built = SvgBasicBuilderConstruction::from_text(text).expect("parses").build().expect("conforming document builds");
        assert!(matches!(built.doc.root, Some(SvgNode::Element { .. })));
    }
}
