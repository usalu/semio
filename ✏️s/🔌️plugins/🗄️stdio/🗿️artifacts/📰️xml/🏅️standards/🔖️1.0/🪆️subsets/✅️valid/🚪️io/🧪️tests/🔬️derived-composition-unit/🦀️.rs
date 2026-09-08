mod tests {
    use super::*;
    use semio_framework_plugin::AnalyzeSource;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn conforming_xml_text() -> String {
        "<!DOCTYPE root>\n<root/>".to_string()
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_document_composes_and_stamps_valid() {
        let text = conforming_xml_text();
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&text) }];
        let composed = XmlValidComposerComposition::compose(&sources).expect("clean document must compose to valid");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_doctype_fails_compose_with_real_diagnostic() {
        let text = "<root/>".to_string();
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&text) }];
        let err = XmlValidComposerComposition::compose(&sources).expect_err("a document without a doctype must not stamp valid");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == "stdio.xml.valid.doctype-missing" && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn root_name_mismatch_fails_compose_with_real_diagnostic() {
        let text = "<!DOCTYPE book>\n<root/>".to_string();
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&text) }];
        let err = XmlValidComposerComposition::compose(&sources).expect_err("a doctype/root name mismatch must not stamp valid");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == "stdio.xml.valid.root-name-mismatch" && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_recheck_flags_only_soft_diagnostics_for_a_clean_document() {
        let text = conforming_xml_text();
        let snapshot = <XmlSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parses");
        let bytes = <XmlSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let diagnostics = XmlValidValidator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "wire recheck must never report a hard violation for a composer-clean document: {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn negative_no_doctype_example_fails_compose_with_declared_hard_code() {
        let text = crate::standards::v1_0::subsets::valid::examples::no_doctype::PRIMARY_TEXT;
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(text) }];
        let err = XmlValidComposerComposition::compose(&sources).expect_err("missing doctype must not stamp valid");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == "stdio.xml.valid.doctype-missing" && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }

    //#region 🧪️SubsetRoundtrip
    struct XmlValidRoundtrip;

    impl store::os_store::test_support::SubsetRoundtripSpec for XmlValidRoundtrip {
        type Snapshot = XmlSnapshot;
        type Mutation = crate::standards::v1_0::subsets::valid::schema::XmlValidMutation;
        type Inference = crate::standards::v1_0::subsets::base::schema::inferences::XmlInference;

        async fn dialect() -> store::os_io::ArtifactDialect {
            store::os_io::ArtifactDialect { artifact_kind: "s.stdio.xml".into(), standard: "1.0".into(), subset: "valid".into() }
        }

        async fn fidelity() -> store::os_store::test_support::IoFidelityClass {
            store::os_store::test_support::IoFidelityClass::Canonical
        }

        async fn drops() -> &'static [&'static str] {
            &[]
        }

        async fn is_derived() -> bool {
            true
        }

        async fn parse_native(asset: &store::os_store::test_support::ExampleAsset<'_>) -> Result<Self::Snapshot, String> {
            let text = asset.text.ok_or_else(|| "xml valid requires dsl text".to_string())?;
            <XmlSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| e.to_string())
        }

        async fn export_native(snapshot: &Self::Snapshot) -> Result<Vec<u8>, String> {
            Ok(store::ArtifactDsl::print_dsl(snapshot).into_bytes())
        }

        async fn reimport_native(bytes: &[u8]) -> Result<Self::Snapshot, String> {
            let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
            <XmlSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| e.to_string())
        }

        async fn infer(snapshot: &Self::Snapshot) -> Self::Inference {
            use protocol::Inference;
            Self::Inference::infer(snapshot)
        }

        async fn sample_mutations(snapshot: &Self::Snapshot) -> Vec<Self::Mutation> {
            vec![crate::standards::v1_0::subsets::valid::schema::XmlValidMutation::SetSnapshot(crate::standards::v1_0::subsets::valid::schema::valid_mutations::set_snapshot::SetSnapshot { snapshot: snapshot.clone() })]
        }

        async fn validate_payload(bytes: &[u8]) -> Result<(), Vec<String>> {
            let text = std::str::from_utf8(bytes).map_err(|e| vec![e.to_string()])?;
            let snapshot = <XmlSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| vec![e.to_string()])?;
            let hard: Vec<String> = check_valid_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).map(|d| d.code.0).collect();
            if hard.is_empty() { Ok(()) } else { Err(hard) }
        }

        async fn validate_negative(bytes: &[u8]) -> Result<Vec<String>, String> {
            let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
            let snapshot = <XmlSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| e.to_string())?;
            Ok(check_valid_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).map(|d| d.code.0).collect())
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn xml_valid_subset_integrated_roundtrip() {
        let text = crate::standards::v1_0::subsets::base::examples::demo::PRIMARY_TEXT;
        let positive = store::os_store::test_support::ExampleAsset { bytes: text.as_bytes(), text: Some(text), provenance: "✳️any/📚️examples/🎬️demo (conforming doctype for valid)" };
        let negative_text = crate::standards::v1_0::subsets::valid::examples::no_doctype::PRIMARY_TEXT;
        let negative = store::os_store::test_support::ExampleAsset { bytes: negative_text.as_bytes(), text: Some(negative_text), provenance: "✳️valid/📚️examples/🚫️no-doctype" };
        store::os_store::test_support::assert_subset_roundtrip::<XmlValidRoundtrip>(&positive, Some(&negative)).await;
    }
    //#endregion 🧪️SubsetRoundtrip
}
