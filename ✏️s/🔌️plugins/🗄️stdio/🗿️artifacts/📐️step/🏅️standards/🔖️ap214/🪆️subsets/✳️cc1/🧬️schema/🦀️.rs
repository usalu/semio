//! 🧬️ StepSnapshot schema (ap214/✳️cc1) — reuses the ✳️base subset's `StepSnapshot` verbatim
//! (the SAME Rust type, same `stdio.step` schema id). ISO 10303-214 CC1 (config data only) is a validation-gated
//! dialect STAMP on top of that existing schema, not a new one — see D4's Tier-1 "same snapshot
//! type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/✳️cc1/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition.

pub use crate::artifacts::step::standards::v_ap214::subsets::base::schema::*;

//#region 🧬️Mutations
/// 🧬️ This subset's OWN mutation vocabulary — one kind per ISO 10303-214 CC1 (config data only) conformance
/// rule, derived from `check_cc1_conformance` below rather than copied from a sibling class, and
/// NOT the `✳️base` subset's generic ISO 10303-21 graph editing. The module re-exports `✳️base`'s
/// `StepMutation`/`apply_step_mutation` as well, since this explicit declaration shadows the glob
/// re-export those names used to arrive through.
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
//#endregion 🧬️Mutations
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::step::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance;
    use crate::artifacts::step::{StepDiff, StepMutation, StepSnapshot};
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct StepCc1BuilderConstruction {
        snapshot: StepSnapshot,
        diagnostics: Vec<Diagnostic>,
    }

    impl ArtifactBuilder for StepCc1BuilderConstruction {
        type Snapshot = StepSnapshot;
        type Mutation = StepMutation;
        type Diff = StepDiff;

        fn empty() -> Self {
            Self { snapshot: StepSnapshot::default(), diagnostics: Vec::new() }
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }

        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<StepSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<StepSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::step::schema::mutations::apply_step_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <StepDiff as protocol::MutationDiff<StepSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ The real construction gate: however `self.snapshot` got here, a hard ISO 10303-214 CC1 (config data only)
        /// violation fails `build()` -- soft diagnostics (missing PRODUCT chain) pass through
        /// silently at this layer (the composer, not the builder, is the facet that surfaces them as
        /// advisory `Diagnostic`s on a successful `Composition`); the `Err` path is only taken for
        /// hard ones.
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let Self { snapshot, mut diagnostics } = self;
            diagnostics.extend(check_cc1_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)));
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
        use crate::artifacts::step::standards::v_ap214::subsets::cc1::schema::CODE_SHAPE_REPRESENTATION_PRESENT;

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn conforming_snapshot() -> StepSnapshot {
            StepSnapshot::from_part21_document(Part21Document {
                header: Part21Header { file_schema: vec![Part21Value::List(vec![Part21Value::Str("AUTOMOTIVE_DESIGN".into())])], ..Part21Header::default() },
                instances: vec![
                    Part21Instance { id: 1, entities: vec![("PRODUCT".into(), vec![])] },
                    Part21Instance { id: 2, entities: vec![("PRODUCT_DEFINITION_FORMATION".into(), vec![])] },
                    Part21Instance { id: 3, entities: vec![("PRODUCT_DEFINITION".into(), vec![])] },
                ],
            })
        }

        #[semio_framework_async_macros::async_test]
        async fn conforming_construction_builds() {
            let snapshot = StepCc1BuilderConstruction::from_snapshot(conforming_snapshot()).build().expect("conforming construction must build");
            assert!(crate::artifacts::step::standards::v_ap214::engine::ladder::has_product_definition_chain(&snapshot.to_part21_document()));
        }

        #[semio_framework_async_macros::async_test]
        async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
            let mut snapshot = conforming_snapshot();
            let mut doc = snapshot.to_part21_document();
            doc.instances.push(Part21Instance { id: 99, entities: vec![("ADVANCED_BREP_SHAPE_REPRESENTATION".into(), vec![])] });
            snapshot = StepSnapshot::from_part21_document(doc);
            let (mutated, _diff) = StepCc1BuilderConstruction::from_snapshot(StepSnapshot::default()).mutate(StepMutation::SetSnapshot(crate::artifacts::step::standards::v_ap214::subsets::base::schema::mutations::set_snapshot::SetSnapshot { snapshot }));
            let err = mutated.build().expect_err("CC1 allows no *_SHAPE_REPRESENTATION instance at all, so an ADVANCED_BREP_SHAPE_REPRESENTATION must fail build()");
            assert!(err.iter().any(|d| d.code.0 == CODE_SHAPE_REPRESENTATION_PRESENT));
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::step::standards::v_ap214::engine::ladder::{file_schema_contains, has_product_definition_chain, ladder_violations};
    use crate::artifacts::step::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot;
    use crate::artifacts::step::standards::v_ap214::subsets::base::schema::StepAnalyzer as StepAnyAnalyzer;
    pub use crate::artifacts::step::standards::v_ap214::subsets::base::schema::StepParts;
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("cc1") };

    /// 🔢️ Maximum ladder rung CC1 permits — no real rung (2..=6) is ever `<= 1`, so this hard-flags
    /// every `*_SHAPE_REPRESENTATION` instance, matching "CC1 allows none".
    pub const MAX_RUNG: u8 = 1;

    //#region 🔖️Conformance
    pub const CODE_FILE_SCHEMA: &str = "stdio.step.cc1.file-schema-automotive-design";
    pub const CODE_PRODUCT_CHAIN: &str = "stdio.step.cc1.product-definition-chain";
    pub const CODE_SHAPE_REPRESENTATION_PRESENT: &str = "stdio.step.cc1.shape-representation-present";

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real ISO 10303-214 CC1 conformance checks against one already-decoded `StepSnapshot`.
    /// Shared single source of truth: `StepCc1Composer::compose` hard-gates on this (pre-
    /// serialization, authoritative), `StepCc1Builder::build` hard-gates on this too, and the
    /// registered `SubsetValidator` (from `🎹️composer::register`) re-runs it post-hoc against the wire
    /// payload for the D5 validate-on-build hook.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_cc1_conformance(snapshot: &StepSnapshot) -> Vec<Diagnostic> {
        let doc = snapshot.to_part21_document();
        let mut out = Vec::new();
        if !file_schema_contains(&doc, "AUTOMOTIVE_DESIGN") {
            out.push(hard(CODE_FILE_SCHEMA, "FILE_SCHEMA does not declare AUTOMOTIVE_DESIGN -- ISO 10303-214 requires the AP214 EXPRESS schema".into()));
        }
        for (id, type_name, rung) in ladder_violations(&doc, MAX_RUNG) {
            out.push(hard(CODE_SHAPE_REPRESENTATION_PRESENT, format!("instance #{id} is a {type_name} (ladder rung {rung}) -- CC1 (config data only) allows no *_SHAPE_REPRESENTATION instance at all")));
        }
        if !has_product_definition_chain(&doc) {
            out.push(soft(CODE_PRODUCT_CHAIN, "no PRODUCT + PRODUCT_DEFINITION_FORMATION + PRODUCT_DEFINITION chain found -- real AP214 config data normally carries one".into()));
        }
        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.step` (ap214/✳️cc1): delegates the real parse to the ✳️base subset's
    /// analyzer (same `StepSnapshot`), then folds real CC1 conformance diagnostics on top. `sniff`
    /// also delegates -- subset-level sniff is "is this recognizable as a STEP file at all", the same
    /// probe every ap214 dialect shares; conformance is a separate, heavier question answered by
    /// `analyze`/`check_cc1_conformance`, not by `sniff`.
    pub struct StepCc1AnalyzerAnalysis;

    impl ArtifactAnalysis for StepCc1AnalyzerAnalysis {
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
                let checks = check_cc1_conformance(snapshot);
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

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn base_doc() -> Part21Document {
            Part21Document {
                header: Part21Header { file_schema: vec![Part21Value::List(vec![Part21Value::Str("AUTOMOTIVE_DESIGN".into())])], ..Part21Header::default() },
                instances: vec![
                    Part21Instance { id: 1, entities: vec![("PRODUCT".into(), vec![])] },
                    Part21Instance { id: 2, entities: vec![("PRODUCT_DEFINITION_FORMATION".into(), vec![])] },
                    Part21Instance { id: 3, entities: vec![("PRODUCT_DEFINITION".into(), vec![])] },
                ],
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn conforming_config_only_document_reports_no_diagnostics() {
            let snapshot = StepSnapshot::from_part21_document(base_doc());
            let diagnostics = check_cc1_conformance(&snapshot);
            assert!(diagnostics.is_empty(), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn missing_file_schema_is_hard() {
            let mut doc = base_doc();
            doc.header.file_schema = vec![];
            let snapshot = StepSnapshot::from_part21_document(doc);
            let diagnostics = check_cc1_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_FILE_SCHEMA && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn missing_product_chain_is_soft() {
            let mut doc = base_doc();
            doc.instances.clear();
            let snapshot = StepSnapshot::from_part21_document(doc);
            let diagnostics = check_cc1_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_PRODUCT_CHAIN && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn any_named_shape_representation_subtype_is_hard() {
            let mut doc = base_doc();
            doc.instances.push(Part21Instance { id: 4, entities: vec![("GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION".into(), vec![])] });
            let snapshot = StepSnapshot::from_part21_document(doc);
            let diagnostics = check_cc1_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_SHAPE_REPRESENTATION_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn bare_shape_representation_base_type_is_also_hard() {
            // Suffix match catches the un-subtyped base type too -- CC1 allows NO representation.
            let mut doc = base_doc();
            doc.instances.push(Part21Instance { id: 4, entities: vec![("SHAPE_REPRESENTATION".into(), vec![])] });
            let snapshot = StepSnapshot::from_part21_document(doc);
            let diagnostics = check_cc1_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_SHAPE_REPRESENTATION_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec StepCc1BuilderFacets {
        construction: StepCc1BuilderConstruction,
        analysis: StepCc1AnalyzerAnalysis,
        composition: super::io::derived_composition::StepCc1ComposerComposition,
    }
    builder: StepCc1Builder,
    analyzer: StepCc1Analyzer,
    composer: StepCc1Composer,
);
//#endregion 🧬️DerivedArtifactFacets
