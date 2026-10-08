//! 🧬️ Editable chart values use the existing event-sourced mutation protocol.
use crate::{ChartSnapshot, ChartDiff, ChartEdit};
use semio_framework_value::{DslValue,FromValue,ToValue};
use protocol::{Mutation, MutationOutcome, MutationLeafDescriptor, MutationInvertibility, MutationDiffParticipation, MutationOutcomeClass, MutationComposition, MutationLanguageSurface};


#[derive(Clone, Debug, PartialEq,semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword="change-chart-value")]
pub struct ChangeChartValue {
    pub path: Vec<String>,
    pub value: Option<DslValue>,
}

impl ToValue for ChangeChartValue {
    fn to_value(&self)->DslValue{
        let mut fields=vec![("path".into(),self.path.to_value())];
        if let Some(value)=&self.value{fields.push(("value".into(),value.clone()));}
        DslValue::object(fields)
    }
}
impl FromValue for ChangeChartValue {
    fn from_value(value:DslValue)->Result<Self,semio_framework_value::ValueError>{
        let DslValue::Object(fields)=value else{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"chart mutation must be an object"));};
        let mut path=None;let mut next=None;
        for(key,value)in fields{match key.as_str(){"path"=>path=Some(Vec::<String>::from_value(value)?),"value"=>next=Some(value),_=>return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown chart mutation field {key}"))),}}
        Ok(Self{path:path.ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"chart mutation path is required"))?,value:next})
    }
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
            Err(error) => MutationOutcome::error(error.code, error.message, error.target),
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
