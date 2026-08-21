//! 🚪️ IO stdio.ifc.2x3 (2x3/✳️cobie) — reuses the ✳️any subset's `binary`/`txt` raw-codec DAG
//! leaves. Registration flows through `🎹️composer::register`, not per-leaf `register()`.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::Ifc2x3Composer as Ifc2x3AnyComposer;
    use crate::artifacts::ifc::standards::v2x3::subsets::cobie::schema::check_cobie_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_COBIE: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("cobie") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct Ifc2x3CobieComposerComposition;

    impl ArtifactComposition for Ifc2x3CobieComposerComposition {
        type Snapshot = Ifc2x3Snapshot;
        const WRITES: Dialect = DIALECT_COBIE;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_COBIE, DEP_TXT]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = semio_framework_plugin::resolve_ready(Ifc2x3AnyComposer::compose(sources))?;
            let checks = check_cobie_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("Basic FM Handover (COBie) conformance violated: {} hard issue(s) -- not stamping the cobie dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct Ifc2x3CobieValidator;

    impl SubsetValidator for Ifc2x3CobieValidator {
        const DIALECT: Dialect = DIALECT_COBIE;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_cobie_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.ifc.2x3.cobie.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "Ifc2x3Cobie SubsetValidator: payload did not decode as an Ifc2x3Snapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<Ifc2x3CobieValidator>)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::ifc::standards::v2x3::subsets::cobie::schema::Ifc2x3CobieBuilderConstruction as Ifc2x3CobieBuilder;
        use crate::artifacts::ifc::standards::v2x3::subsets::cobie::schema::CODE_VIEW_DEFINITION;
        use semio_framework_plugin::AnalyzeSource;
        use semio_framework_plugin::ArtifactBuilder as _;

        #[semio_framework_async_macros::async_test]
        async fn conforming_builder_snapshot_composes_and_stamps_cobie() {
            let snapshot = Ifc2x3CobieBuilder::new().build().expect("clean COBie document must build");
            let bytes = <Ifc2x3Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let composed = Ifc2x3CobieComposerComposition::compose(&sources).expect("clean document must compose to cobie");
            assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
        }

        #[semio_framework_async_macros::async_test]
        async fn wrong_view_definition_fails_compose_with_real_diagnostic() {
            let mut snapshot = Ifc2x3CobieBuilder::new().build().expect("build");
            snapshot.document.header.file_description[0] = crate::artifacts::step::engine::part21::Part21Value::List(vec![crate::artifacts::step::engine::part21::Part21Value::Str("ViewDefinition [CoordinationView]".into())]);
            let bytes = <Ifc2x3Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let err = Ifc2x3CobieComposerComposition::compose(&sources).expect_err("wrong ViewDefinition must not stamp cobie");
            assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_VIEW_DEFINITION && d.severity == Severity::Error), "got {:?}", err.diagnostics);
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
