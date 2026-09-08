mod tests {
    use super::*;
    use crate::standards::v1_1::subsets::tiny::schema::SvgTinyBuilder;
    use crate::standards::v1_1::subsets::tiny::schema::{CODE_ATTRIBUTE, CODE_ELEMENT};
    use semio_framework_plugin::AnalyzeSource;
    use semio_framework_plugin::ArtifactBuilder as _;

    #[semio_framework_async_macros::async_test]
    async fn conforming_document_composes_and_stamps_tiny() {
        let text = r#"<svg xmlns="http://www.w3.org/2000/svg"><rect x="0" y="0" width="10" height="10"/></svg>"#;
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(text) }];
        let composed = SvgTinyComposerComposition::compose(&sources).expect("clean document must compose to tiny");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
        match &composed.snapshot.doc.root {
            Some(semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { attrs, .. }) => {
                assert!(attrs.iter().any(|a| a.name == "baseProfile" && a.value == "tiny"));
                assert!(attrs.iter().any(|a| a.name == "version" && a.value == "1.1"));
            }
            other => panic!("expected element root, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn blocklisted_element_fails_compose_with_real_diagnostic() {
        let text = r#"<svg xmlns="http://www.w3.org/2000/svg"><linearGradient id="g1"/></svg>"#;
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(text) }];
        let err = SvgTinyComposerComposition::compose(&sources).expect_err("a document with a linearGradient must not stamp tiny");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_ELEMENT && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn blocklisted_attribute_fails_compose_with_real_diagnostic() {
        let text = r#"<svg xmlns="http://www.w3.org/2000/svg"><rect x="0" y="0" width="10" height="10" filter="url(#f1)"/></svg>"#;
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(text) }];
        let err = SvgTinyComposerComposition::compose(&sources).expect_err("a document with a filter attribute must not stamp tiny");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_ATTRIBUTE && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_recheck_flags_no_hard_issue_on_a_clean_builder_document() {
        let snapshot = SvgTinyBuilder::empty().build().expect("empty document builds clean");
        let bytes = <SvgSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let diagnostics = SvgTinyValidator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "wire recheck must never report a hard violation for a builder-clean document: {diagnostics:?}");
    }
}
