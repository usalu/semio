//! 🚪️ IO stdio.step (ap214/✳️cc4) — reuses the ✳️any subset's import/export DAG leaves (same
//! `StepSnapshot` type, same catalog DAG edges) rather than duplicating them. Registration flows
//! through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator, and the
//! `SubsetValidator` directly), not per-leaf `register()` — same pattern `✳️any/🚪️io` established.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::step::standards::v_ap214::engine::ladder::ensure_file_schema;
    use crate::artifacts::step::standards::v_ap214::subsets::any::schema::snapshot::StepSnapshot;
    use crate::artifacts::step::standards::v_ap214::subsets::any::schema::StepComposer as StepAnyComposer;
    use crate::artifacts::step::standards::v_ap214::subsets::cc4::schema::check_cc4_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_SELF: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("cc4") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct StepCc4ComposerComposition;

    impl ArtifactComposition for StepCc4ComposerComposition {
        type Snapshot = StepSnapshot;
        const WRITES: Dialect = DIALECT_SELF;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_SELF, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = StepAnyComposer::compose(sources)?;
            let mut snapshot = inner.snapshot;
            let mut doc = snapshot.to_part21_document();
            ensure_file_schema(&mut doc, "AUTOMOTIVE_DESIGN");
            snapshot = StepSnapshot::from_part21_document(doc);
            let checks = check_cc4_conformance(&snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("ISO 10303-214 CC4 (manifold surfaces with topology) conformance violated: {} hard issue(s) -- not stamping the cc4 dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `ap214/cc4` -- see the module doc comment for how
    /// this relates to (and honestly differs from) the composer's own pre-serialization hard gate.
    pub struct StepCc4Validator;

    impl SubsetValidator for StepCc4Validator {
        const DIALECT: Dialect = DIALECT_SELF;

        fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <StepSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <StepSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_cc4_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.step.cc4.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "StepCc4Validator: payload did not decode as a StepSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<StepCc4Validator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the ap214 standard's own `⚙️engine::register()`. The
    /// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
    /// (`crate::artifacts::step::standards::v_ap214::engine::io_registry::entries()`).
    pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::step::standards::v_ap214::engine::part21::{Part21Document, Part21Header, Part21Instance};
        use semio_framework_plugin::AnalyzeSource;

        fn clean_bytes() -> Vec<u8> {
            let doc = Part21Document {
                header: Part21Header { file_schema: vec![], ..Part21Header::default() },
                instances: vec![
                    Part21Instance { id: 1, entities: vec![("PRODUCT".into(), vec![])] },
                    Part21Instance { id: 2, entities: vec![("PRODUCT_DEFINITION_FORMATION".into(), vec![])] },
                    Part21Instance { id: 3, entities: vec![("PRODUCT_DEFINITION".into(), vec![])] },
                ],
            };
            <StepSnapshot as store::ArtifactPack>::encode_pack(&StepSnapshot::from_part21_document(doc))
        }

        #[test]
        fn composer_injects_file_schema_and_stamps_clean_document() {
            let bytes = clean_bytes();
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let composed = StepCc4ComposerComposition::compose(&sources).expect("a document with no illegal representation must compose to cc4");
            assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
            assert!(crate::artifacts::step::standards::v_ap214::engine::ladder::file_schema_contains(&composed.snapshot.to_part21_document(), "AUTOMOTIVE_DESIGN"), "composer must inject FILE_SCHEMA=AUTOMOTIVE_DESIGN");
        }

        #[test]
        fn subset_validator_recheck_flags_missing_file_schema_on_the_raw_wire_payload() {
            // Unlike `compose`, `validate` never runs `ensure_file_schema` -- a wire payload that
            // skipped this subset's own composer genuinely lacks the injection.
            let bytes = clean_bytes();
            let diagnostics = StepCc4Validator::validate(&IoPayload::Binary(bytes));
            assert!(diagnostics.iter().any(|d| d.code.0 == crate::artifacts::step::standards::v_ap214::subsets::cc4::schema::CODE_FILE_SCHEMA), "got {diagnostics:?}");
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
