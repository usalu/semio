//! 🚪️ IO stdio.xlsx (ecma-376/✳️strict) — reuses the ✳️any subset's `zip`/`xml` raw-codec DAG
//! leaves rather than duplicating them (same `XlsxSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! `✳️any/🚪️io` and pdf `1.7/✳️a/🚪️io` already established for this artifact family.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::XlsxSnapshot;
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::XlsxComposer as XlsxAnyComposer;
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::strict::schema::check_strict_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_STRICT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("strict") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_XML: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct XlsxStrictComposerComposition;

    impl ArtifactComposition for XlsxStrictComposerComposition {
        type Snapshot = XlsxSnapshot;
        const WRITES: Dialect = DIALECT_STRICT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_STRICT, DEP_ZIP, DEP_XML]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = semio_framework_plugin::resolve_ready(XlsxAnyComposer::compose(sources))?;
            let checks = check_strict_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("ISO/IEC 29500-1 Strict conformance violated: {} hard issue(s) -- not stamping the strict dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `ecma-376/strict` -- see the module doc comment for
    /// how this relates to (and honestly differs from) the composer's own pre-serialization hard gate
    /// above.
    pub struct XlsxStrictValidator;

    impl SubsetValidator for XlsxStrictValidator {
        const DIALECT: Dialect = DIALECT_STRICT;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes).await.ok(),
                IoPayload::Text(text) => <XlsxSnapshot as store::ArtifactDsl>::parse_dsl(text).await.ok(),
            };
            match decoded {
                Some(snapshot) => check_strict_conformance(&snapshot).await,
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.xlsx.strict.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "Xlsx Strict SubsetValidator: payload did not decode as an XlsxSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    async fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<XlsxStrictValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the ecma-376 standard's own `⚙️engine::register()`, which
    /// is already invoked from the artifact-level `crate::artifacts::xlsx::io_registry::register()`. The
    /// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
    /// (`crate::artifacts::xlsx::standards::v_ecma_376::engine::io_registry::entries()`), matching how `✳️any`'s
    /// own entry is registered.
    pub async fn register() {
        let _ = register_subset_validator(validator_entry().await);
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::XlsxWorkbook;
        use crate::artifacts::xlsx::standards::v_ecma_376::subsets::strict::schema::XlsxStrictBuilderConstruction as XlsxStrictBuilder;
        use crate::artifacts::xlsx::standards::v_ecma_376::subsets::strict::schema::{CODE_CONFORMANCE_ATTRIBUTE, CODE_NAMESPACE_MISMATCH};
        use semio_framework_plugin::{AnalyzeSource, ArtifactBuilder as _};

        /// 🩹 `encode_xlsx` (`⚙️engine/🦀️component.rs`) always calls `regenerate_workbook_parts`,
        /// which REBUILDS `xl/workbook.xml` from `snap.workbook` (the typed model) on every encode --
        /// it doesn't know about Strict mode, so it would silently overwrite whatever
        /// `XlsxStrictBuilder::new(...)`'s `stamp_strict_namespace` post-processing wrote into `opc`.
        /// Encoding the OPC package directly (bypassing the typed-model regeneration entirely) is how
        /// this test genuinely exercises a workbook whose XML matches what the strict builder seeded —
        /// same fix as docx's sibling `✳️strict` composer test.
        async fn conforming_pack_bytes(snapshot: &XlsxSnapshot) -> Vec<u8> {
            let raw = crate::artifacts::zip::opc::encode_opc(&snapshot.opc).expect("valid opc package encodes");
            let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<XlsxSnapshot as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).expect("valid envelope_id");
            store::semio_format::wrap_binary(&envelope, &raw)
        }

        #[semio_framework_async_macros::async_test]
        async fn conforming_builder_snapshot_composes_and_stamps_strict() {
            let snapshot = XlsxStrictBuilder::new(XlsxWorkbook::default()).build().expect("conforming strict construction must build");
            let bytes = conforming_pack_bytes(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let composed = XlsxStrictComposerComposition::compose(&sources).expect("clean document must compose to strict");
            assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
        }

        #[semio_framework_async_macros::async_test]
        async fn transitional_shaped_document_fails_compose_with_real_diagnostic() {
            let snapshot = crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_xlsx(XlsxWorkbook::default());
            let bytes = <XlsxSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let err = XlsxStrictComposerComposition::compose(&sources).expect_err("a Transitional-shaped workbook.xml must not stamp strict");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_NAMESPACE_MISMATCH && d.severity == Severity::Error), "got {:?}", err.diagnostics);
        }

        #[semio_framework_async_macros::async_test]
        async fn subset_validator_recheck_flags_hard_diagnostics_on_the_wire_payload() {
            // Documented writer scope cut (module doc comment): `encode_pack` -> `encode_xlsx` ->
            // `regenerate_workbook_parts` always re-emits Transitional-shaped bytes, so a round trip
            // honestly re-reports the Strict conformance-attribute violation -- not a false positive,
            // the wire bytes genuinely no longer declare Strict.
            let snapshot = XlsxStrictBuilder::new(XlsxWorkbook::default()).build().expect("build");
            let bytes = <XlsxSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let diagnostics = XlsxStrictValidator::validate(&IoPayload::Binary(bytes));
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTRIBUTE && d.severity == Severity::Error), "got {diagnostics:?}");
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
