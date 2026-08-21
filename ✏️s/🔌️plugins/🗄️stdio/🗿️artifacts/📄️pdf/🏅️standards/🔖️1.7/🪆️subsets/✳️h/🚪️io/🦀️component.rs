//! 🚪️ IO stdio.pdf (1.7/✳️h) — reuses the ✳️any subset's `binary`/`deflate` raw-codec DAG
//! leaves rather than duplicating them (same `PdfSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! established by `✳️a/🚪️io` and `✳️any/🚪️io` for this artifact. AIIM/ASTM PDF Healthcare Best Practices Guide.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::PdfComposer as PdfAnyComposer;
    use crate::artifacts::pdf::standards::v1_7::subsets::h::schema::check_h_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_H: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("h") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct PdfHComposerComposition;

    impl ArtifactComposition for PdfHComposerComposition {
        type Snapshot = PdfSnapshot;
        const WRITES: Dialect = DIALECT_H;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_H, DEP_BINARY, DEP_DEFLATE]
        }

        /// ✅ Always `Ok` -- PDF/H has no hard checks to gate on (see module doc comment). Advisory
        /// diagnostics from `check_h_conformance` are folded onto the successful `Composition`.
        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = semio_framework_plugin::resolve_ready(PdfAnyComposer::compose(sources))?;
            let checks = check_h_conformance(&inner.snapshot);
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(checks);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct PdfHValidator;

    impl SubsetValidator for PdfHValidator {
        const DIALECT: Dialect = DIALECT_H;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <PdfSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_h_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.pdf.h.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "PDF/H SubsetValidator: payload did not decode as a PdfSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<PdfHValidator>)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::pdf::standards::v1_7::subsets::h::schema::PdfHBuilderConstruction as PdfHBuilder;
        use semio_framework_plugin::AnalyzeSource;
        use semio_framework_plugin::ArtifactBuilder as _;

        #[semio_framework_async_macros::async_test]
        async fn compose_always_succeeds_even_with_zero_setup() {
            let snapshot = PdfSnapshot::default();
            let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let composed = PdfHComposerComposition::compose(&sources).await.expect("PDF/H never hard-gates");
            assert!(composed.diagnostics.iter().any(|d| d.code.0 == crate::artifacts::pdf::standards::v1_7::subsets::h::schema::CODE_INFO_TITLE_OR_AUTHOR));
        }

        #[semio_framework_async_macros::async_test]
        async fn conforming_builder_snapshot_composes_with_fewer_advisories() {
            let snapshot = PdfHBuilder::new()
                .await
                .add_page(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfPage::new(100.0, 100.0))
                .await
                .set_info(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfInfo { title: Some("A Chart".into()), author: Some("Dr. X".into()), ..Default::default() })
                .await
                .build()
                .await
                .unwrap();
            let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let composed = PdfHComposerComposition::compose(&sources).await.expect("PDF/H never hard-gates");
            assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error && d.severity != Severity::Fatal), "got {:?}", composed.diagnostics);
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
