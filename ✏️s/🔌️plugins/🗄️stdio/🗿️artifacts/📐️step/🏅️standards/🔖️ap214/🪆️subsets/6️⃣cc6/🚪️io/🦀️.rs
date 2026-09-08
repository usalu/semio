//! 🚪️ IO stdio.step (ap214/6️⃣cc6) — reuses the 🧱️base subset's import/export DAG leaves (same
//! `StepSnapshot` type, same catalog DAG edges) rather than duplicating them. Registration flows
//! through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator, and the
//! `SubsetValidator` directly), not per-leaf `register()` — same pattern `🧱️base/🚪️io` established.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_ap214::engine::ladder::ensure_file_schema;
    use crate::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot;
    use crate::standards::v_ap214::subsets::base::schema::StepComposer as StepAnyComposer;
    use crate::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_SELF: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("cc6") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct StepCc6ComposerComposition;

    impl ArtifactComposition for StepCc6ComposerComposition {
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
            snapshot = StepSnapshot::from_part21_document(&doc);
            let checks = check_cc6_conformance(&snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("ISO 10303-214 CC6 (advanced B-Rep, top of the ladder) conformance violated: {} hard issue(s) -- not stamping the cc6 dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `ap214/cc6` -- see the module doc comment for how
    /// this relates to (and honestly differs from) the composer's own pre-serialization hard gate.
    pub struct StepCc6Validator;

    impl SubsetValidator for StepCc6Validator {
        const DIALECT: Dialect = DIALECT_SELF;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <StepSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <StepSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_cc6_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.step.cc6.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "StepCc6Validator: payload did not decode as a StepSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<StepCc6Validator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the ap214 standard's own `⚙️engine::register()`. The
    /// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
    /// (`crate::standards::v_ap214::engine::io_registry::entries()`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
