//! 🧬️ Editable chart values use the existing event-sourced mutation protocol.
use crate::{ChartSnapshot, ChartDiff, ChartEdit};
use protocol::{DslValue, FromValue, ToValue, Mutation, MutationDiff, MutationOutcome, MutationLeafDescriptor, MutationInvertibility, MutationDiffParticipation, MutationOutcomeClass, MutationComposition, MutationLanguageSurface};

#[path="📦️codec/🦀️.rs"]
mod codec;

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
        let before = crate::diff::read_path(&base.chart, &self.path).cloned();
        if !crate::diff::valid_path(&self.path) { return MutationOutcome::error("print.chart.path", "invalid chart address", self.path.clone()); }
        let diff = ChartDiff { edits: vec![ChartEdit { path: self.path.clone(), before: before.clone(), after: self.value.clone() }] };
        match diff.apply(base) {
            Ok(_) if match(&before,&self.value){(Some(a),Some(b))=>crate::diff::chart_values_equal(a,b),(None,None)=>true,_=>false} => MutationOutcome::empty(),
            Ok(_) => MutationOutcome::new(diff),
            Err(error) => MutationOutcome::error(error.code, error.message, error.target),
        }
    }
    fn inverse(&self, base: &ChartSnapshot) -> Vec<Self> {
        if self.diff(base).diff().edits.is_empty() { return Vec::new(); }
        let array_parent = self.path.len() > 1 && matches!(crate::diff::read_path(&base.chart, &self.path[..self.path.len()-1]), Some(DslValue::Array(_)));
        let path = if array_parent { self.path[..self.path.len()-1].to_vec() } else { self.path.clone() };
        vec![Self { value: crate::diff::read_path(&base.chart, &path).cloned(), path }]
    }
    fn conflict_target(&self) -> Vec<String> { std::iter::once("chart".into()).chain(self.path.clone()).collect() }
}
