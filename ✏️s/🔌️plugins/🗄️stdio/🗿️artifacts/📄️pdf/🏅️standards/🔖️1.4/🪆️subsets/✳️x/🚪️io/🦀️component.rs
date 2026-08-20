//! 🚪️ IO stdio.pdf (1.4/✳️x) — reuses the ✳️any subset's `binary`/`deflate` raw-codec DAG
//! leaves rather than duplicating them (same `PdfSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! `✳️any/🚪️io` already established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::PdfComposer as PdfAnyComposer;
    use crate::artifacts::pdf::standards::v1_4::subsets::x::schema::check_pdf_x_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_X: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("x") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct PdfXComposerComposition;

    impl ArtifactComposition for PdfXComposerComposition {
        type Snapshot = PdfSnapshot;
        const WRITES: Dialect = DIALECT_X;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_X, DEP_BINARY, DEP_DEFLATE]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = semio_framework_plugin::resolve_ready(PdfAnyComposer::compose(sources))?;
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(check_pdf_x_conformance(&inner.snapshot));
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct PdfXValidator;

    impl SubsetValidator for PdfXValidator {
        const DIALECT: Dialect = DIALECT_X;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).await.ok(),
                IoPayload::Text(text) => <PdfSnapshot as store::ArtifactDsl>::parse_dsl(text).await.ok(),
            };
            match decoded {
                Some(snapshot) => check_pdf_x_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.pdf.x.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "PDF/X (1.4) SubsetValidator: payload did not decode as a PdfSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<PdfXValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator`. Called from 1.4's own `⚙️engine::register()`.
    /// The `ComposerEntry` itself is registered separately via this standard's own
    /// `composer::entries()` aggregation.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::pdf::standards::v1_4::subsets::x::schema::CODE_SCHEMA_GAP;
        use semio_framework_plugin::AnalyzeSource;

        #[semio_framework_async_macros::async_test]
        async fn compose_always_carries_the_schema_gap_diagnostic() {
            let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&PdfSnapshot::default());
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let composed = PdfXComposerComposition::compose(&sources).await.expect("pass-through compose never fails on conformance grounds");
            assert!(composed.diagnostics.iter().any(|d| d.code.0 == CODE_SCHEMA_GAP), "got {:?}", composed.diagnostics);
        }

        #[semio_framework_async_macros::async_test]
        async fn subset_validator_reports_the_schema_gap_diagnostic() {
            let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&PdfSnapshot::default());
            let diagnostics = PdfXValidator::validate(&IoPayload::Binary(bytes.await));
            assert!(diagnostics.await.iter().any(|d| d.code.0 == CODE_SCHEMA_GAP), "got {diagnostics:?}");
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
