//! 🚪️ IO stdio.xlsx (ecma-376/✳️transitional) — reuses the ✳️base subset's `zip`/`xml` raw-codec
//! DAG leaves rather than duplicating them (same `XlsxSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! `✳️base/🚪️io` and `✳️strict/🚪️io` already established for this artifact family.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::XlsxSnapshot;
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::XlsxComposer as XlsxAnyComposer;
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::schema::check_transitional_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_TRANSITIONAL: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_XML: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct XlsxTransitionalComposerComposition;

    impl ArtifactComposition for XlsxTransitionalComposerComposition {
        type Snapshot = XlsxSnapshot;
        const WRITES: Dialect = DIALECT_TRANSITIONAL;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_TRANSITIONAL, DEP_ZIP, DEP_XML]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = XlsxAnyComposer::compose(sources)?;
            let checks = check_transitional_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("ISO/IEC 29500-4 Transitional conformance violated: {} hard issue(s) -- not stamping the transitional dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct XlsxTransitionalValidator;

    impl SubsetValidator for XlsxTransitionalValidator {
        const DIALECT: Dialect = DIALECT_TRANSITIONAL;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <XlsxSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_transitional_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.xlsx.transitional.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "Xlsx Transitional SubsetValidator: payload did not decode as an XlsxSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<XlsxTransitionalValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry. Called from the
    /// ecma-376 standard's own `⚙️engine::register()`. The `ComposerEntry` itself is registered
    /// separately by the standard-level composer aggregator
    /// (`crate::artifacts::xlsx::standards::v_ecma_376::engine::io_registry::entries()`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::XlsxWorkbook;
        use crate::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::schema::XlsxTransitionalBuilderConstruction as XlsxTransitionalBuilder;
        use crate::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::schema::CODE_NAMESPACE_MISMATCH;
        use semio_framework_plugin::{AnalyzeSource, ArtifactBuilder as _};

        #[semio_framework_async_macros::async_test]
        async fn conforming_builder_snapshot_composes_and_stamps_transitional() {
            let snapshot = XlsxTransitionalBuilder::new(XlsxWorkbook::default()).build().expect("conforming transitional construction must build");
            let bytes = <XlsxSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let composed = XlsxTransitionalComposerComposition::compose(&sources).expect("clean document must compose to transitional");
            assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
        }

        #[semio_framework_async_macros::async_test]
        async fn strict_shaped_document_fails_compose_with_real_diagnostic() {
            // ⚠️ `ArtifactPack::encode_pack` (-> `⚙️engine::encode_xlsx`) regenerates workbook.xml as
            // Transitional-shaped on every call (documented writer scope cut, see ✳️strict's composer
            // module doc comment) -- so to genuinely exercise a Strict-shaped payload here, feed the
            // raw OPC bytes through the DSL text (hex) path instead, which routes straight to the real
            // `engine::decode_xlsx` without an intervening regenerate. Same technique the PDF/A 1.7
            // pilot's composer tests use for the analogous reason.
            use crate::artifacts::xlsx::standards::v_ecma_376::subsets::strict::schema::stamp_strict_namespace;
            let strict_snapshot = stamp_strict_namespace(crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_xlsx(XlsxWorkbook::default()));
            // 🩹 `engine::encode_xlsx` itself regenerates workbook.xml as Transitional-shaped (the
            // very thing this comment above warns about) -- encoding the OPC package directly, NOT
            // through `encode_xlsx`, is what actually avoids the regenerate.
            let opc_bytes = crate::artifacts::zip::opc::encode_opc(&strict_snapshot.opc).expect("encode strict-shaped opc bytes");
            let hex: String = opc_bytes.iter().map(|b| format!("{b:02x}")).collect();
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&hex) }];
            let err = XlsxTransitionalComposerComposition::compose(&sources).expect_err("a Strict-shaped workbook.xml must not stamp transitional");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_NAMESPACE_MISMATCH && d.severity == Severity::Error), "got {:?}", err.diagnostics);
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
