//! 🚪️ IO stdio.json (rfc8259/🛜️i-json) — reuses the ✳️any subset's `txt` raw-codec DAG leaf
//! rather than duplicating it (same `JsonSnapshot` type, same catalog DAG edges). Registration
//! flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator,
//! and the `SubsetValidator` directly), not per-leaf `register()` — same pattern `✳️any/🚪️io`
//! already established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::json::standards::v_rfc8259::subsets::base::schema::snapshot::JsonSnapshot;
    use crate::artifacts::json::standards::v_rfc8259::subsets::base::schema::JsonComposer as JsonAnyComposer;
    use crate::artifacts::json::standards::v_rfc8259::subsets::i_json::schema::check_i_json_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_I_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("i-json") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct JsonIJsonComposerComposition;

    impl ArtifactComposition for JsonIJsonComposerComposition {
        type Snapshot = JsonSnapshot;
        const WRITES: Dialect = DIALECT_I_JSON;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_I_JSON, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = JsonAnyComposer::compose(sources)?;
            let checks = check_i_json_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("I-JSON (RFC 7493) conformance violated: {} hard issue(s) -- not stamping the i-json dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `rfc8259/i-json`.
    pub struct JsonIJsonValidator;

    impl SubsetValidator for JsonIJsonValidator {
        const DIALECT: Dialect = DIALECT_I_JSON;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <JsonSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <JsonSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_i_json_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.json.i-json.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "I-JSON SubsetValidator: payload did not decode as a JsonSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<JsonIJsonValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the rfc8259 standard's own `⚙️engine::register()`. The
    /// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
    /// (`crate::artifacts::json::standards::v_rfc8259::subsets::base::io::io_registry::entries()`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::AnalyzeSource;

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn conforming_json_text() -> String {
            "{\"a\":1,\"b\":[1,2,3]}".to_string()
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
            let text = "{\"a\":1,\"a\":2}".to_string();
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&text) }];
            let err = JsonIJsonComposerComposition::compose(&sources).expect_err("a document with a duplicate member name must not stamp i-json");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == "stdio.json.i-json.duplicate-member-name" && d.severity == Severity::Error), "got {:?}", err.diagnostics);
        }

        #[semio_framework_async_macros::async_test]
        async fn unsafe_integer_fails_compose_with_real_diagnostic() {
            let text = "{\"n\":9007199254740993}".to_string();
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&text) }];
            let err = JsonIJsonComposerComposition::compose(&sources).expect_err("a document with an unsafe integer must not stamp i-json");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == "stdio.json.i-json.unsafe-integer" && d.severity == Severity::Error), "got {:?}", err.diagnostics);
        }

        #[semio_framework_async_macros::async_test]
        async fn subset_validator_recheck_flags_only_soft_diagnostics_for_a_clean_document() {
            let text = "\"just a top-level string\"".to_string();
            let snapshot = <JsonSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parses");
            let bytes = <JsonSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let diagnostics = JsonIJsonValidator::validate(&IoPayload::Binary(bytes)).await;
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "wire recheck must never report a hard violation for a duplicate/overflow-free document: {diagnostics:?}");
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.json.i-json.top-level-scalar"), "got {diagnostics:?}");
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
