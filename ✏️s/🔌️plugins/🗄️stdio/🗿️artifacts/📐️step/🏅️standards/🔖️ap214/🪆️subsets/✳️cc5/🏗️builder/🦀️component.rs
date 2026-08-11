//! 🏗️ StepCc5Builder (ap214/✳️cc5) — a local `ArtifactBuilder` (until SDK Wave 3, same as
//! the artifact-level `StepBuilder`) whose `build()` re-runs `check_cc5_conformance` as a hard
//! gate, unconditionally, regardless of which construction path produced the in-flight snapshot
//! -- so a hard ISO 10303-214 CC5 (faceted B-Rep) violation can never leave this builder as an `Ok(StepSnapshot)`.

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::step::{StepDiff, StepMutation, StepSnapshot};
use crate::artifacts::step::standards::v_ap214::subsets::cc5::analyzer::check_cc5_conformance;

//#region 🔖️Builder
#[derive(Clone, Debug, Default)]
pub struct StepCc5Builder {
    snapshot: StepSnapshot,
}

impl ArtifactBuilder for StepCc5Builder {
    type Snapshot = StepSnapshot;
    type Mutation = StepMutation;
    type Diff = StepDiff;

    fn empty() -> Self {
        Self { snapshot: StepSnapshot::default() }
    }

    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot }
    }

    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<StepSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }

    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<StepSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }

    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = crate::artifacts::step::schema::mutations::apply_step_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }

    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <StepDiff as protocol::MutationDiff<StepSnapshot>>::apply(&diff, &self.snapshot);
        self
    }

    /// 🛡️ The real construction gate: however `self.snapshot` got here, a hard ISO 10303-214 CC5 (faceted B-Rep)
    /// violation fails `build()` -- soft diagnostics (missing PRODUCT chain) pass through
    /// silently at this layer (the composer, not the builder, is the facet that surfaces them as
    /// advisory `Diagnostic`s on a successful `Composition`); the `Err` path is only taken for
    /// hard ones.
    fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
        let hard: Vec<Diagnostic> = check_cc5_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
        if hard.is_empty() {
            Ok(self.snapshot)
        } else {
            Err(hard)
        }
    }
}
//#endregion 🔖️Builder

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::step::standards::v_ap214::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
    use crate::artifacts::step::standards::v_ap214::subsets::cc5::analyzer::CODE_LADDER;

    fn conforming_snapshot() -> StepSnapshot {
        StepSnapshot::from_part21_document(Part21Document {
                header: Part21Header { file_schema: vec![Part21Value::List(vec![Part21Value::Str("AUTOMOTIVE_DESIGN".into())])], ..Part21Header::default() },
                instances: vec![
                    Part21Instance { id: 1, entities: vec![("PRODUCT".into(), vec![])] },
                    Part21Instance { id: 2, entities: vec![("PRODUCT_DEFINITION_FORMATION".into(), vec![])] },
                    Part21Instance { id: 3, entities: vec![("PRODUCT_DEFINITION".into(), vec![])] },
                ],
            },
        )
    }

    #[test]
    fn conforming_construction_builds() {
        let snapshot = StepCc5Builder::from_snapshot(conforming_snapshot()).build().expect("conforming construction must build");
        assert!(crate::artifacts::step::standards::v_ap214::engine::ladder::has_product_definition_chain(&snapshot.to_part21_document()));
    }

    #[test]
    fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut snapshot = conforming_snapshot();
        let mut doc = snapshot.to_part21_document();
        doc.instances.push(crate::artifacts::step::standards::v_ap214::engine::part21::Part21Instance {
            id: 99,
            entities: vec![("ADVANCED_BREP_SHAPE_REPRESENTATION".into(), vec![])],
        });
        snapshot = StepSnapshot::from_part21_document(doc);
        let (mutated, _diff) = StepCc5Builder::from_snapshot(StepSnapshot::default()).mutate(StepMutation::SetSnapshot { snapshot });
        let err = mutated.build().expect_err("an ADVANCED_BREP_SHAPE_REPRESENTATION instance above rung 5 must fail build()");
        assert!(err.iter().any(|d| d.code.0 == CODE_LADDER));
    }
}
