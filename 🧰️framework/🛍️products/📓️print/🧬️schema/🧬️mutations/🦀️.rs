//! 🧬️ Editable chart values use the existing event-sourced mutation protocol.
use crate::{ChartSnapshot, ChartDiff, ChartEdit};
use semio_framework_value::DslValue;
use protocol::{Mutation, MutationOutcome, MutationLeafDescriptor, MutationInvertibility, MutationDiffParticipation, MutationOutcomeClass, MutationComposition, MutationLanguageSurface};


#[derive(Clone, Debug, PartialEq)]
pub struct ChangeChartValue {
    pub path: Vec<String>,
    pub value: Option<DslValue>,
}

pub const CHANGE_CHART_VALUE: MutationLeafDescriptor = MutationLeafDescriptor {
    schema_version: 1, owner: "🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/🦀️.rs", semantic_kind: "change-chart-value", display_name: "Change Chart Value", emoji: "🎚️", aggregate_variant: "ChangeChartValue", payload_schema: include_str!("🔣️.json"), text_opcode: Some("change-chart-value"), binary_tag: Some(1), invertibility: MutationInvertibility::ExplicitMutation, diff_participation: MutationDiffParticipation::Detect, outcome_classes: &[MutationOutcomeClass::Applied, MutationOutcomeClass::NoOp, MutationOutcomeClass::Rejected], composition: MutationComposition::Atomic, required_language_surfaces: &[MutationLanguageSurface::Rust, MutationLanguageSurface::Typescript, MutationLanguageSurface::JsonSchema],
};

impl Mutation<ChartSnapshot> for ChangeChartValue {
    type Diff = ChartDiff;
    const DESCRIPTORS: &'static [MutationLeafDescriptor] = &[CHANGE_CHART_VALUE];
    fn descriptor(&self) -> &'static MutationLeafDescriptor { &CHANGE_CHART_VALUE }
    fn diff(&self, base: &ChartSnapshot) -> MutationOutcome<ChartDiff> {
        match ChartEdit::authored(&base.chart, &self.path, self.value.as_ref()) {
            Ok(Some(edit)) => MutationOutcome::new(ChartDiff { edits: vec![edit] }),
            Ok(None) => MutationOutcome::empty(),
            Err(error) => match error.code.as_str() {
                "mutation.apply.invalid-path" => MutationOutcome::fatal("mutation.invariant", error.message, error.target),
                "mutation.apply.missing-target" => MutationOutcome::error("mutation.target-missing", error.message, error.target),
                _ => MutationOutcome::error("mutation.target-mismatch", error.message, error.target),
            },
        }
    }
    fn inverse(&self, base: &ChartSnapshot) -> Result<Vec<Self>,semio_framework_value::ValueError> {
        let before = crate::diff::read_path(&base.chart, &self.path).cloned();
        if !crate::diff::valid_path(&self.path) || crate::diff::presence_equal(before.as_ref(), self.value.as_ref()) { return Ok(Vec::new()); }
        let parent_path = &self.path[..self.path.len() - 1];
        let index = self.path.last().and_then(|segment| crate::diff::array_index(segment));
        if let (None, Some(DslValue::Array(items)), Some(index)) = (&self.value, crate::diff::read_path(&base.chart, parent_path), index) {
            if index < items.len() {
                return Ok((index..items.len()).map(|slot| Self { path: parent_path.iter().cloned().chain(std::iter::once(slot.to_string())).collect(), value: Some(items[slot].clone()) }).collect());
            }
        }
        Ok(vec![Self { path: self.path.clone(), value: before }])
    }
    fn conflict_target(&self) -> Vec<String> { std::iter::once("chart".into()).chain(self.path.clone()).collect() }
}
