//! 🚪️ IO stdio.xml (1.0/✳️valid) — reuses the ✳️any subset's `txt` raw-codec DAG leaf rather than
//! duplicating it (same `XmlSnapshot` type, same catalog DAG edges). Registration flows through
//! `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator, and the
//! `SubsetValidator` directly), not per-leaf `register()` — same pattern `✳️any/🚪️io` already
//! established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::xml::standards::v1_0::subsets::any::schema::snapshot::XmlSnapshot;
    use crate::artifacts::xml::standards::v1_0::subsets::any::schema::XmlComposer as XmlAnyComposer;
    use crate::artifacts::xml::standards::v1_0::subsets::valid::schema::check_valid_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_VALID: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("valid") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct XmlValidComposerComposition;

    impl ArtifactComposition for XmlValidComposerComposition {
        type Snapshot = XmlSnapshot;
        const WRITES: Dialect = DIALECT_VALID;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_VALID, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = XmlAnyComposer::compose(sources)?;
            let checks = check_valid_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("XML 1.0 validity violated: {} hard issue(s) -- not stamping the valid dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `1.0/valid`.
    pub struct XmlValidValidator;

    impl SubsetValidator for XmlValidValidator {
        const DIALECT: Dialect = DIALECT_VALID;

        fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <XmlSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <XmlSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_valid_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.xml.valid.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "XML valid SubsetValidator: payload did not decode as an XmlSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<XmlValidValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the 1.0 standard's own `⚙️engine::register()`. The
    /// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
    /// (`crate::artifacts::xml::standards::v1_0::engine::io_registry::entries()`).
    pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::AnalyzeSource;

        fn conforming_xml_text() -> String {
            "<!DOCTYPE root>\n<root/>".to_string()
        }

        #[test]
        fn conforming_document_composes_and_stamps_valid() {
            let text = conforming_xml_text();
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&text) }];
            let composed = XmlValidComposerComposition::compose(&sources).expect("clean document must compose to valid");
            assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
        }

        #[test]
        fn missing_doctype_fails_compose_with_real_diagnostic() {
            let text = "<root/>".to_string();
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&text) }];
            let err = XmlValidComposerComposition::compose(&sources).expect_err("a document without a doctype must not stamp valid");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == "stdio.xml.valid.doctype-missing" && d.severity == Severity::Error), "got {:?}", err.diagnostics);
        }

        #[test]
        fn root_name_mismatch_fails_compose_with_real_diagnostic() {
            let text = "<!DOCTYPE book>\n<root/>".to_string();
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&text) }];
            let err = XmlValidComposerComposition::compose(&sources).expect_err("a doctype/root name mismatch must not stamp valid");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == "stdio.xml.valid.root-name-mismatch" && d.severity == Severity::Error), "got {:?}", err.diagnostics);
        }

        #[test]
        fn subset_validator_recheck_flags_only_soft_diagnostics_for_a_clean_document() {
            let text = conforming_xml_text();
            let snapshot = <XmlSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parses");
            let bytes = <XmlSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let diagnostics = XmlValidValidator::validate(&IoPayload::Binary(bytes));
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "wire recheck must never report a hard violation for a composer-clean document: {diagnostics:?}");
        }

        #[test]
        fn negative_no_doctype_example_fails_compose_with_declared_hard_code() {
            let text = crate::artifacts::xml::standards::v1_0::subsets::valid::examples::no_doctype::PRIMARY_TEXT;
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(text) }];
            let err = XmlValidComposerComposition::compose(&sources).expect_err("missing doctype must not stamp valid");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == "stdio.xml.valid.doctype-missing" && d.severity == Severity::Error), "got {:?}", err.diagnostics);
        }

        //#region 🧪️SubsetRoundtrip
        struct XmlValidRoundtrip;

        impl store::os_store::test_support::SubsetRoundtripSpec for XmlValidRoundtrip {
            type Snapshot = XmlSnapshot;
            type Mutation = crate::artifacts::xml::standards::v1_0::subsets::any::schema::mutations::XmlMutation;
            type Inference = crate::artifacts::xml::standards::v1_0::subsets::any::schema::inferences::XmlInference;

            fn dialect() -> store::os_io::ArtifactDialect {
                store::os_io::ArtifactDialect { artifact_kind: "s.stdio.xml".into(), standard: "1.0".into(), subset: "valid".into() }
            }

            fn fidelity() -> store::os_store::test_support::IoFidelityClass {
                store::os_store::test_support::IoFidelityClass::Canonical
            }

            fn drops() -> &'static [&'static str] {
                &[]
            }

            fn is_derived() -> bool {
                true
            }

            fn parse_native(asset: &store::os_store::test_support::ExampleAsset<'_>) -> Result<Self::Snapshot, String> {
                let text = asset.text.ok_or_else(|| "xml valid requires dsl text".to_string())?;
                <XmlSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| e.to_string())
            }

            fn export_native(snapshot: &Self::Snapshot) -> Result<Vec<u8>, String> {
                Ok(store::ArtifactDsl::print_dsl(snapshot).into_bytes())
            }

            fn reimport_native(bytes: &[u8]) -> Result<Self::Snapshot, String> {
                let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
                <XmlSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| e.to_string())
            }

            fn infer(snapshot: &Self::Snapshot) -> Self::Inference {
                use protocol::Inference;
                Self::Inference::infer(snapshot)
            }

            fn sample_mutations(snapshot: &Self::Snapshot) -> Vec<Self::Mutation> {
                vec![crate::artifacts::xml::standards::v1_0::subsets::any::schema::mutations::XmlMutation::SetSnapshot { snapshot: snapshot.clone() }]
            }

            fn validate_payload(bytes: &[u8]) -> Result<(), Vec<String>> {
                let text = std::str::from_utf8(bytes).map_err(|e| vec![e.to_string()])?;
                let snapshot = <XmlSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| vec![e.to_string()])?;
                let hard: Vec<String> = check_valid_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).map(|d| d.code.0).collect();
                if hard.is_empty() {
                    Ok(())
                } else {
                    Err(hard)
                }
            }

            fn validate_negative(bytes: &[u8]) -> Result<Vec<String>, String> {
                let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
                let snapshot = <XmlSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| e.to_string())?;
                Ok(check_valid_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).map(|d| d.code.0).collect())
            }
        }

        #[test]
        fn xml_valid_subset_integrated_roundtrip() {
            let text = crate::artifacts::xml::standards::v1_0::subsets::any::examples::demo::PRIMARY_TEXT;
            let positive = store::os_store::test_support::ExampleAsset { bytes: text.as_bytes(), text: Some(text), provenance: "✳️any/📚️examples/🎬️demo (conforming doctype for valid)" };
            let negative_text = crate::artifacts::xml::standards::v1_0::subsets::valid::examples::no_doctype::PRIMARY_TEXT;
            let negative = store::os_store::test_support::ExampleAsset { bytes: negative_text.as_bytes(), text: Some(negative_text), provenance: "✳️valid/📚️examples/🚫️no-doctype" };
            store::os_store::test_support::assert_subset_roundtrip::<XmlValidRoundtrip>(&positive, Some(&negative));
        }
        //#endregion 🧪️SubsetRoundtrip
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
