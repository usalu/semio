mod tests {
    use super::*;
    use crate::standards::v1_1::subsets::tiny::schema::CODE_ELEMENT;
    use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;

    #[semio_framework_async_macros::async_test]
    async fn empty_builder_injects_profile_and_builds_clean() {
        let snapshot = SvgTinyBuilderConstruction::empty().build().expect("empty document builds clean");
        match &snapshot.doc.root {
            Some(XmlNode::Element { attrs, .. }) => {
                assert!(attrs.iter().any(|a| a.name == "baseProfile" && a.value == "tiny"));
                assert!(attrs.iter().any(|a| a.name == "version" && a.value == "1.1"));
            }
            other => panic!("expected element root, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut snapshot = SvgTinyBuilderConstruction::empty().build().unwrap();
        if let Some(XmlNode::Element { children, .. }) = snapshot.doc.root.as_mut() {
            children.push(XmlNode::Element { name: "script".into(), attrs: vec![], children: vec![XmlNode::Text { text: "alert(1)".into() }] });
        }
        let (mutated, _diff) = SvgTinyBuilderConstruction::from_snapshot(SvgSnapshot::default()).mutate(SvgTinyMutation::SetSnapshot(crate::standards::v1_1::subsets::tiny::schema::mutations::set_snapshot::SetSnapshot { snapshot }));
        let err = mutated.build().expect_err("a <script> element must fail build()");
        assert!(err.iter().any(|d| d.code.0 == CODE_ELEMENT));
    }

    #[semio_framework_async_macros::async_test]
    async fn from_text_round_trips_through_tiny_build() {
        let text = r#"<svg xmlns="http://www.w3.org/2000/svg"><circle cx="5" cy="5" r="5"/></svg>"#;
        let built = SvgTinyBuilderConstruction::from_text(text).expect("parses").build().expect("conforming document builds");
        assert!(matches!(built.doc.root, Some(XmlNode::Element { .. })));
    }
}
