//! 🚪️ IO stdio.ifc.2x3 (2x3/✳️cv20) — reuses the ✳️any subset's `binary`/`txt` raw-codec DAG
//! leaves rather than duplicating them (same `Ifc2x3Snapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()`.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::Ifc2x3Composer as Ifc2x3AnyComposer;
    use crate::artifacts::ifc::standards::v2x3::subsets::cv20::schema::check_cv20_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_CV20: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("cv20") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct Ifc2x3Cv20ComposerComposition;

    impl ArtifactComposition for Ifc2x3Cv20ComposerComposition {
        type Snapshot = Ifc2x3Snapshot;
        const WRITES: Dialect = DIALECT_CV20;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_CV20, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = Ifc2x3AnyComposer::compose(sources)?;
            let checks = check_cv20_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("Coordination View 2.0 conformance violated: {} hard issue(s) -- not stamping the cv20 dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct Ifc2x3Cv20Validator;

    impl SubsetValidator for Ifc2x3Cv20Validator {
        const DIALECT: Dialect = DIALECT_CV20;

        fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_cv20_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.ifc.2x3.cv20.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "Ifc2x3Cv20 SubsetValidator: payload did not decode as an Ifc2x3Snapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<Ifc2x3Cv20Validator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator`. Called from the `2x3` standard's own
    /// `⚙️engine::register()`. The `ComposerEntry` itself is registered separately via the standard's
    /// own `composer::entries()` aggregation.
    pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::ifc::standards::v2x3::subsets::cv20::schema::Ifc2x3Cv20BuilderConstruction as Ifc2x3Cv20Builder;
        use crate::artifacts::ifc::standards::v2x3::subsets::cv20::schema::CODE_VIEW_DEFINITION;
        use semio_framework_plugin::AnalyzeSource;
        use semio_framework_plugin::ArtifactBuilder as _;

        #[test]
        fn conforming_builder_snapshot_composes_and_stamps_cv20() {
            let snapshot = Ifc2x3Cv20Builder::new().build().expect("clean CV2.0 document must build");
            let bytes = <Ifc2x3Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let composed = Ifc2x3Cv20ComposerComposition::compose(&sources).expect("clean document must compose to cv20");
            assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
        }

        #[test]
        fn wrong_view_definition_fails_compose_with_real_diagnostic() {
            let mut snapshot = Ifc2x3Cv20Builder::new().build().expect("build");
            snapshot.document.header.file_description[0] = crate::artifacts::step::engine::part21::Part21Value::List(vec![crate::artifacts::step::engine::part21::Part21Value::Str("ViewDefinition [StructuralAnalysisView]".into())]);
            let bytes = <Ifc2x3Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let err = Ifc2x3Cv20ComposerComposition::compose(&sources).expect_err("wrong ViewDefinition must not stamp cv20");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_VIEW_DEFINITION && d.severity == Severity::Error), "got {:?}", err.diagnostics);
        }

        #[test]
        fn subset_validator_recheck_is_clean_for_a_conforming_document() {
            let snapshot = Ifc2x3Cv20Builder::new().build().expect("build");
            let bytes = <Ifc2x3Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let diagnostics = Ifc2x3Cv20Validator::validate(&IoPayload::Binary(bytes));
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "wire recheck must never report a hard violation for a builder-clean document: {diagnostics:?}");
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
