//! 🚪️ IO stdio.step (ap214/5️⃣cc5) — reuses the 🧱️base subset's import/export DAG leaves (same
//! `StepSnapshot` type, same catalog DAG edges) rather than duplicating them. Registration flows
//! through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator, and the
//! `SubsetValidator` directly), not per-leaf `register()` — same pattern `🧱️base/🚪️io` established.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_ap214::engine::ladder::ensure_file_schema;
    use crate::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot;
    use crate::standards::v_ap214::subsets::base::io::StepComposer as StepAnyComposer;
    use crate::standards::v_ap214::subsets::cc5::io::check_cc5_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::register_subset_validator,semio_framework_plugin::subset_validator_entry_of,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposeSource,semio_framework_plugin::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::SubsetValidator,semio_framework_plugin::SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_SELF: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("cc5") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct StepCc5ComposerComposition;

    impl ArtifactComposition for StepCc5ComposerComposition {
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
            let checks = check_cc5_conformance(&snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("ISO 10303-214 CC5 (faceted B-Rep) conformance violated: {} hard issue(s) -- not stamping the cc5 dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `ap214/cc5` -- see the module doc comment for how
    /// this relates to (and honestly differs from) the composer's own pre-serialization hard gate.
    pub struct StepCc5Validator;

    impl SubsetValidator for StepCc5Validator {
        const DIALECT: Dialect = DIALECT_SELF;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <StepSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <StepSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_cc5_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.step.cc5.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "StepCc5Validator: payload did not decode as a StepSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<StepCc5Validator>)
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

pub mod derived_construction {
    use crate::standards::v_ap214::subsets::cc5::io::check_cc5_conformance;
    use crate::{StepDiff, StepSnapshot};
    use crate::standards::v_ap214::subsets::cc5::schema::mutations::{StepCc5Mutation, apply_step_cc5_mutation};
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::Severity;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct StepCc5BuilderConstruction {
        snapshot: StepSnapshot,
        diagnostics: Vec<Diagnostic>,
    }

    impl ArtifactBuilder for StepCc5BuilderConstruction {
        type Snapshot = StepSnapshot;
        type Mutation = StepCc5Mutation;
        type Diff = StepDiff;

        fn empty() -> Self {
            Self { snapshot: StepSnapshot::default(), diagnostics: Vec::new() }
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<StepSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<StepSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_step_cc5_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <StepDiff as protocol::MutationDiff<StepSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ The real construction gate: however `self.snapshot` got here, a hard ISO 10303-214 CC5 (faceted B-Rep)
        /// violation fails `build()` -- soft diagnostics (missing PRODUCT chain) pass through
        /// silently at this layer (the composer, not the builder, is the facet that surfaces them as
        /// advisory `Diagnostic`s on a successful `Composition`); the `Err` path is only taken for
        /// hard ones.
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let Self { snapshot, mut diagnostics } = self;
            diagnostics.extend(check_cc5_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)));
            if diagnostics.is_empty() {
                Ok(snapshot)
            } else {
                Err(diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-construction-unit/🦀️.rs");
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v_ap214::engine::ladder::{file_schema_contains, has_product_definition_chain, ladder_violations};
    use crate::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot;
    use crate::standards::v_ap214::subsets::base::io::StepAnalyzer as StepAnyAnalyzer;
    pub use crate::standards::v_ap214::subsets::base::io::StepParts;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::Analysis,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoConfidence,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("cc5") };

    /// 🔢️ Maximum ladder rung cc5 permits (see `⚙️engine::ladder::ladder_rung_of`).
    use crate::standards::v_ap214::subsets::cc5::schema::MAX_RUNG;

    //#region 🔖️Conformance
    pub const CODE_FILE_SCHEMA: &str = "stdio.step.cc5.file-schema-automotive-design";
    pub const CODE_PRODUCT_CHAIN: &str = "stdio.step.cc5.product-definition-chain";
    pub const CODE_LADDER: &str = "stdio.step.cc5.representation-above-rung";

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real ISO 10303-214 CC5 (faceted B-Rep) conformance checks against one already-decoded `StepSnapshot`. Shared
    /// single source of truth: `StepCc5Composer::compose` hard-gates on this (pre-serialization,
    /// authoritative), `StepCc5Builder::build` hard-gates on this too, and the registered
    /// `SubsetValidator` (from `🎹️composer::register`) re-runs it post-hoc against the wire payload
    /// for the D5 validate-on-build hook.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_cc5_conformance(snapshot: &StepSnapshot) -> Vec<Diagnostic> {
        let doc = snapshot.to_part21_document();
        let mut out = Vec::new();
        if !file_schema_contains(&doc, "AUTOMOTIVE_DESIGN") {
            out.push(hard(CODE_FILE_SCHEMA, "FILE_SCHEMA does not declare AUTOMOTIVE_DESIGN -- ISO 10303-214 requires the AP214 EXPRESS schema".into()));
        }
        for (id, type_name, rung) in ladder_violations(&doc, MAX_RUNG) {
            out.push(hard(CODE_LADDER, format!("instance #{id} is a {type_name} (ladder rung {rung}) -- exceeds cc5's max rung 5")));
        }
        if !has_product_definition_chain(&doc) {
            out.push(soft(CODE_PRODUCT_CHAIN, "no PRODUCT + PRODUCT_DEFINITION_FORMATION + PRODUCT_DEFINITION chain found -- real AP214 data normally carries one".into()));
        }
        out
    }
    /// 🛡️ Checks owned CC5 facts with cancellation before diagnostic allocation.
    pub fn check_cc5_conformance_controlled(snapshot:&StepSnapshot,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<Vec<Diagnostic>,semio_framework_os_kernel::sqlite_snapshot::ValueError>{
        let facts=crate::standards::v_ap214::subsets::base::io::sqlite::snapshot::conformance_facts(snapshot,MAX_RUNG,control)?;let mut out=Vec::new();
        if !facts.file_schema{out.push(hard(CODE_FILE_SCHEMA,"FILE_SCHEMA does not declare AUTOMOTIVE_DESIGN -- ISO 10303-214 requires the AP214 EXPRESS schema".into()));}
        for(index,(id,type_name,rung))in facts.violations.into_iter().enumerate(){if index%256==0||type_name.len()>65536{control.checkpoint(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot,index,0)?;}out.push(hard(CODE_LADDER,format!("instance #{id} is a {type_name} (ladder rung {rung}) -- exceeds cc5's max rung 5")));}
        if !facts.product_chain{out.push(soft(CODE_PRODUCT_CHAIN,"no PRODUCT + PRODUCT_DEFINITION_FORMATION + PRODUCT_DEFINITION chain found -- real AP214 data normally carries one".into()));}Ok(out)
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.step` (ap214/5️⃣cc5): delegates the real parse to the 🧱️base subset's
    /// analyzer (same `StepSnapshot`), then folds real ISO 10303-214 CC5 (faceted B-Rep) conformance diagnostics on top.
    /// `sniff` also delegates -- subset-level sniff is "is this recognizable as a STEP file at all",
    /// the same probe every ap214 dialect shares; conformance is a separate, heavier question
    /// answered by `analyze`/`check_cc5_conformance`, not by `sniff`.
    pub struct StepCc5AnalyzerAnalysis;

    impl ArtifactAnalysis for StepCc5AnalyzerAnalysis {
        type Parts = StepParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            StepAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = StepAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_cc5_conformance(snapshot);
                if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                    confidence = IoConfidence::Low;
                }
                diagnostics.extend(checks);
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec StepCc5BuilderFacets {
        construction: StepCc5BuilderConstruction,
        analysis: StepCc5AnalyzerAnalysis,
        composition: super::io::derived_composition::StepCc5ComposerComposition,
    }
    builder: StepCc5Builder,
    analyzer: StepCc5Analyzer,
    composer: StepCc5Composer,
);

#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
