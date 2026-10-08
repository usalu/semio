mod tests {
    use super::*;
    use semio_framework_plugin::io::AnalyzeSource;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn conforming_json_text() -> String {
        store::ArtifactDsl::print_dsl(&JsonSnapshot{schema:crate::STDIO_JSON_DOCUMENT_SCHEMA.into(),value:crate::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text("{\"a\":1,\"b\":[1,2,3]}").unwrap()})
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_document_composes_and_stamps_i_json() {
        let text = conforming_json_text();
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&text) }];
        let composed = JsonIJsonComposerComposition::compose(&sources).expect("clean document must compose to i-json");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn duplicate_member_name_fails_compose_with_real_diagnostic() {
        let text = store::ArtifactDsl::print_dsl(&JsonSnapshot{schema:crate::STDIO_JSON_DOCUMENT_SCHEMA.into(),value:crate::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text("{\"a\":1,\"a\":2}").unwrap()});
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&text) }];
        let err = JsonIJsonComposerComposition::compose(&sources).expect_err("a document with a duplicate member name must not stamp i-json");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == "stdio.json.i-json.duplicate-member-name" && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn unsafe_integer_fails_compose_with_real_diagnostic() {
        let text = store::ArtifactDsl::print_dsl(&JsonSnapshot{schema:crate::STDIO_JSON_DOCUMENT_SCHEMA.into(),value:crate::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text("{\"n\":9007199254740993}").unwrap()});
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&text) }];
        let err = JsonIJsonComposerComposition::compose(&sources).expect_err("a document with an unsafe integer must not stamp i-json");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == "stdio.json.i-json.unsafe-integer" && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_recheck_flags_only_soft_diagnostics_for_a_clean_document() {
        let text = "\"just a top-level string\"".to_string();
        let snapshot = JsonSnapshot{schema:crate::STDIO_JSON_DOCUMENT_SCHEMA.into(),value:crate::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text(&text).expect("file syntax")};
        let bytes = <JsonSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let diagnostics = JsonIJsonValidator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "wire recheck must never report a hard violation for a duplicate/overflow-free document: {diagnostics:?}");
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.json.i-json.top-level-scalar"), "got {diagnostics:?}");
    }
}
