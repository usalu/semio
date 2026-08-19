//! 🧬️ StepSnapshot schema (ap214/✳️cc6) — reuses the ✳️any subset's `StepSnapshot` verbatim
//! (the SAME Rust type, same `stdio.step` schema id). ISO 10303-214 CC6 (advanced B-Rep, top of the ladder) is a validation-gated
//! dialect STAMP on top of that existing schema, not a new one — see D4's Tier-1 "same snapshot
//! type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/✳️cc6/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition.

pub use crate::artifacts::step::standards::v_ap214::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::step::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance;
    use crate::artifacts::step::{StepDiff, StepMutation, StepSnapshot};
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct StepCc6BuilderConstruction {
        snapshot: StepSnapshot,
        diagnostics: Vec<Diagnostic>,
    }

    impl ArtifactBuilder for StepCc6BuilderConstruction {
        type Snapshot = StepSnapshot;
        type Mutation = StepMutation;
        type Diff = StepDiff;

        async fn empty() -> Self {
            Self { snapshot: StepSnapshot::default(), diagnostics: Vec::new() }
        }

        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }

        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<StepSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<StepSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::step::schema::mutations::apply_step_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <StepDiff as protocol::MutationDiff<StepSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ The real construction gate: however `self.snapshot` got here, a hard ISO 10303-214 CC6 (advanced B-Rep, top of the ladder)
        /// violation fails `build()` -- soft diagnostics (missing PRODUCT chain) pass through
        /// silently at this layer (the composer, not the builder, is the facet that surfaces them as
        /// advisory `Diagnostic`s on a successful `Composition`); the `Err` path is only taken for
        /// hard ones.
        async fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let Self { snapshot, mut diagnostics } = self;
            diagnostics.extend(check_cc6_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)));
            if diagnostics.is_empty() {
                Ok(snapshot)
            } else {
                Err(diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::step::standards::v_ap214::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};

        async fn conforming_snapshot() -> StepSnapshot {
            StepSnapshot::from_part21_document(Part21Document {
                header: Part21Header { file_schema: vec![Part21Value::List(vec![Part21Value::Str("AUTOMOTIVE_DESIGN".into())])], ..Part21Header::default() },
                instances: vec![
                    Part21Instance { id: 1, entities: vec![("PRODUCT".into(), vec![])] },
                    Part21Instance { id: 2, entities: vec![("PRODUCT_DEFINITION_FORMATION".into(), vec![])] },
                    Part21Instance { id: 3, entities: vec![("PRODUCT_DEFINITION".into(), vec![])] },
                ],
            })
        }

        #[test]
        async fn conforming_construction_builds() {
            let snapshot = StepCc6BuilderConstruction::from_snapshot(conforming_snapshot()).build().expect("conforming construction must build");
            assert!(crate::artifacts::step::standards::v_ap214::engine::ladder::has_product_definition_chain(&snapshot.to_part21_document()));
        }

        #[test]
        async fn top_of_ladder_representation_injected_via_raw_mutate_still_builds() {
            // CC6 is the top rung -- ADVANCED_BREP_SHAPE_REPRESENTATION (rung 6) is never a
            // violation here, however it entered the snapshot; this documents that invariant
            // honestly rather than asserting a hard failure that can never happen at cc6.
            let mut snapshot = conforming_snapshot();
            let mut doc = snapshot.to_part21_document();
            doc.instances.push(Part21Instance { id: 99, entities: vec![("ADVANCED_BREP_SHAPE_REPRESENTATION".into(), vec![])] });
            snapshot = StepSnapshot::from_part21_document(doc);
            let (mutated, _diff) = StepCc6BuilderConstruction::from_snapshot(StepSnapshot::default()).mutate(StepMutation::SetSnapshot { snapshot });
            mutated.build().expect("cc6 is the top of the ladder -- ADVANCED_BREP_SHAPE_REPRESENTATION is never a violation");
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::step::standards::v_ap214::engine::ladder::{file_schema_contains, has_product_definition_chain, ladder_violations};
    use crate::artifacts::step::standards::v_ap214::subsets::any::schema::snapshot::StepSnapshot;
    use crate::artifacts::step::standards::v_ap214::subsets::any::schema::StepAnalyzer as StepAnyAnalyzer;
    pub use crate::artifacts::step::standards::v_ap214::subsets::any::schema::StepParts;
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("cc6") };

    /// 🔢️ Maximum ladder rung cc6 permits (see `⚙️engine::ladder::ladder_rung_of`).
    pub const MAX_RUNG: u8 = 6;

    //#region 🔖️Conformance
    pub const CODE_FILE_SCHEMA: &str = "stdio.step.cc6.file-schema-automotive-design";
    pub const CODE_PRODUCT_CHAIN: &str = "stdio.step.cc6.product-definition-chain";
    pub const CODE_LADDER: &str = "stdio.step.cc6.representation-above-rung";

    async fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    async fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real ISO 10303-214 CC6 (advanced B-Rep, top of the ladder) conformance checks against one already-decoded `StepSnapshot`. Shared
    /// single source of truth: `StepCc6Composer::compose` hard-gates on this (pre-serialization,
    /// authoritative), `StepCc6Builder::build` hard-gates on this too, and the registered
    /// `SubsetValidator` (from `🎹️composer::register`) re-runs it post-hoc against the wire payload
    /// for the D5 validate-on-build hook.
    pub async fn check_cc6_conformance(snapshot: &StepSnapshot) -> Vec<Diagnostic> {
        let doc = snapshot.to_part21_document();
        let mut out = Vec::new();
        if !file_schema_contains(&doc, "AUTOMOTIVE_DESIGN") {
            out.push(hard(CODE_FILE_SCHEMA, "FILE_SCHEMA does not declare AUTOMOTIVE_DESIGN -- ISO 10303-214 requires the AP214 EXPRESS schema".into()));
        }
        for (id, type_name, rung) in ladder_violations(&doc, MAX_RUNG) {
            out.push(hard(CODE_LADDER, format!("instance #{id} is a {type_name} (ladder rung {rung}) -- exceeds cc6's max rung 6")));
        }
        if !has_product_definition_chain(&doc) {
            out.push(soft(CODE_PRODUCT_CHAIN, "no PRODUCT + PRODUCT_DEFINITION_FORMATION + PRODUCT_DEFINITION chain found -- real AP214 data normally carries one".into()));
        }
        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.step` (ap214/✳️cc6): delegates the real parse to the ✳️any subset's
    /// analyzer (same `StepSnapshot`), then folds real ISO 10303-214 CC6 (advanced B-Rep, top of the ladder) conformance diagnostics on top.
    /// `sniff` also delegates -- subset-level sniff is "is this recognizable as a STEP file at all",
    /// the same probe every ap214 dialect shares; conformance is a separate, heavier question
    /// answered by `analyze`/`check_cc6_conformance`, not by `sniff`.
    pub struct StepCc6AnalyzerAnalysis;

    impl ArtifactAnalysis for StepCc6AnalyzerAnalysis {
        type Parts = StepParts;
        const DIALECT: Dialect = DIALECT;

        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            StepAnyAnalyzer::sniff(source)
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = StepAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_cc6_conformance(snapshot);
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
    mod tests {
        use super::*;
        use crate::artifacts::step::standards::v_ap214::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};

        async fn base_doc() -> Part21Document {
            Part21Document {
                header: Part21Header { file_schema: vec![Part21Value::List(vec![Part21Value::Str("AUTOMOTIVE_DESIGN".into())])], ..Part21Header::default() },
                instances: vec![
                    Part21Instance { id: 1, entities: vec![("PRODUCT".into(), vec![])] },
                    Part21Instance { id: 2, entities: vec![("PRODUCT_DEFINITION_FORMATION".into(), vec![])] },
                    Part21Instance { id: 3, entities: vec![("PRODUCT_DEFINITION".into(), vec![])] },
                ],
            }
        }

        #[test]
        async fn conforming_document_reports_no_diagnostics() {
            let snapshot = StepSnapshot::from_part21_document(base_doc());
            let diagnostics = check_cc6_conformance(&snapshot);
            assert!(diagnostics.is_empty(), "got {diagnostics:?}");
        }

        #[test]
        async fn missing_file_schema_is_hard() {
            let mut doc = base_doc();
            doc.header.file_schema = vec![];
            let snapshot = StepSnapshot::from_part21_document(doc);
            let diagnostics = check_cc6_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_FILE_SCHEMA && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        async fn missing_product_chain_is_soft() {
            let mut doc = base_doc();
            doc.instances.clear();
            let snapshot = StepSnapshot::from_part21_document(doc);
            let diagnostics = check_cc6_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_PRODUCT_CHAIN && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[test]
        async fn representation_at_max_rung_is_clean() {
            let mut doc = base_doc();
            doc.instances.push(Part21Instance { id: 4, entities: vec![("ADVANCED_BREP_SHAPE_REPRESENTATION".into(), vec![])] });
            let snapshot = StepSnapshot::from_part21_document(doc);
            let diagnostics = check_cc6_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.code.0 != CODE_LADDER), "got {diagnostics:?}");
        }

        #[test]
        async fn top_of_ladder_has_no_representation_that_can_exceed_it() {
            // CC6 is the top rung -- every real *_SHAPE_REPRESENTATION subtype classifies at <= 6, so
            // there is no honest "above" fixture; this documents that invariant rather than fabricating one.
            assert_eq!(MAX_RUNG, 6);
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec StepCc6BuilderFacets {
        construction: StepCc6BuilderConstruction,
        analysis: StepCc6AnalyzerAnalysis,
        composition: super::io::derived_composition::StepCc6ComposerComposition,
    }
    builder: StepCc6Builder,
    analyzer: StepCc6Analyzer,
    composer: StepCc6Composer,
);
//#endregion 🧬️DerivedArtifactFacets
