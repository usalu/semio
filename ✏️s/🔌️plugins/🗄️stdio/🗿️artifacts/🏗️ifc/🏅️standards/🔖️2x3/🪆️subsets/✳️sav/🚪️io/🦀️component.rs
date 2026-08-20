//! 🚪️ IO stdio.ifc.2x3 (2x3/✳️sav) — reuses the ✳️any subset's `binary`/`txt` raw-codec DAG
//! leaves. Registration flows through `🎹️composer::register`, not per-leaf `register()`.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::Ifc2x3Composer as Ifc2x3AnyComposer;
    use crate::artifacts::ifc::standards::v2x3::subsets::sav::schema::check_sav_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_SAV: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("sav") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct Ifc2x3SavComposerComposition;

    impl ArtifactComposition for Ifc2x3SavComposerComposition {
        type Snapshot = Ifc2x3Snapshot;
        const WRITES: Dialect = DIALECT_SAV;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_SAV, DEP_TXT]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = semio_framework_plugin::resolve_ready(Ifc2x3AnyComposer::compose(sources))?;
            let checks = check_sav_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("Structural Analysis View conformance violated: {} hard issue(s) -- not stamping the sav dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct Ifc2x3SavValidator;

    impl SubsetValidator for Ifc2x3SavValidator {
        const DIALECT: Dialect = DIALECT_SAV;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes).await.ok(),
                IoPayload::Text(text) => <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text).await.ok(),
            };
            match decoded {
                Some(snapshot) => check_sav_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.ifc.2x3.sav.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "Ifc2x3Sav SubsetValidator: payload did not decode as an Ifc2x3Snapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<Ifc2x3SavValidator>)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::ifc::standards::v2x3::subsets::sav::schema::Ifc2x3SavBuilderConstruction as Ifc2x3SavBuilder;
        use crate::artifacts::ifc::standards::v2x3::subsets::sav::schema::CODE_NO_ANALYSIS_MODEL;
        use semio_framework_plugin::AnalyzeSource;
        use semio_framework_plugin::ArtifactBuilder as _;

        #[semio_framework_async_macros::async_test]
        async fn conforming_builder_snapshot_composes_and_stamps_sav() {
            let snapshot = Ifc2x3SavBuilder::new().build().await.expect("clean SAV document must build");
            let bytes = <Ifc2x3Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let composed = Ifc2x3SavComposerComposition::compose(&sources).await.expect("clean document must compose to sav");
            assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
        }

        #[semio_framework_async_macros::async_test]
        async fn no_analysis_model_fails_compose_with_real_diagnostic() {
            let mut snapshot = Ifc2x3SavBuilder::new().build().await.expect("build");
            snapshot.document.instances.retain(|i| !i.is_type("IFCSTRUCTURALANALYSISMODEL"));
            let bytes = <Ifc2x3Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let err = Ifc2x3SavComposerComposition::compose(&sources).await.expect_err("a document with no analysis model must not stamp sav");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_NO_ANALYSIS_MODEL && d.severity == Severity::Error), "got {:?}", err.diagnostics);
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
