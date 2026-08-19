//! 🚪️ IO stdio.pdf (1.7/✳️e) — reuses the ✳️any subset's `binary`/`deflate` raw-codec DAG
//! leaves rather than duplicating them (same `PdfSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! established by `✳️a/🚪️io` and `✳️any/🚪️io` for this artifact. ISO 24517-1:2008 (PDF/E-1).
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::PdfComposer as PdfAnyComposer;
    use crate::artifacts::pdf::standards::v1_7::subsets::e::schema::check_e_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_E: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("e") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct PdfEComposerComposition;

    impl ArtifactComposition for PdfEComposerComposition {
        type Snapshot = PdfSnapshot;
        const WRITES: Dialect = DIALECT_E;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_E, DEP_BINARY, DEP_DEFLATE]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = semio_framework_plugin::resolve_ready(PdfAnyComposer::compose(sources))?;
            let checks = check_e_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("PDF/E-1 conformance violated: {} hard issue(s) -- not stamping the e dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct PdfEValidator;

    impl SubsetValidator for PdfEValidator {
        const DIALECT: Dialect = DIALECT_E;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <PdfSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_e_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.pdf.e.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "PDF/E SubsetValidator: payload did not decode as a PdfSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    async fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<PdfEValidator>)
    }

    pub async fn register() {
        let _ = register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::pdf::standards::v1_7::subsets::e::schema::PdfEBuilderConstruction as PdfEBuilder;
        use semio_framework_plugin::AnalyzeSource;
        use semio_framework_plugin::ArtifactBuilder as _;

        #[semio_framework_async_macros::async_test]
        async fn conforming_builder_snapshot_composes_and_stamps_e() {
            let snapshot = PdfEBuilder::new().add_page(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfPage::new(100.0, 100.0)).build().unwrap();
            let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let composed = PdfEComposerComposition::compose(&sources).expect("clean document must compose to e");
            assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
        }

        #[semio_framework_async_macros::async_test]
        async fn subset_validator_recheck_runs_the_same_check() {
            let snapshot = PdfEBuilder::new().add_page(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfPage::new(50.0, 50.0)).build().unwrap();
            let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let diagnostics = PdfEValidator::validate(&IoPayload::Binary(bytes));
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "wire recheck must never report a hard violation for a builder-clean document: {diagnostics:?}");
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
