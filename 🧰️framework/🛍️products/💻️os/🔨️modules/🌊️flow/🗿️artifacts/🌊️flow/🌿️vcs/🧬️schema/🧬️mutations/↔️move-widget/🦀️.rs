//! ↔️ Move Widget direct payload and owned behavior.
use super::super::{FlowHostSnapshot, FlowDiff, FlowDelta, FlowCollectionDelta, FlowMove, FlowMutation, flow_wire_index};
use crate::os_spr::{MutationKind, MutationOutcome, SemanticDescriptor, Identified};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, crate::os_dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "move-widget")]
pub struct MoveWidget { pub id: String, pub to_index: u32 }

//#endregion 🧬️Payload

//#region 🎮️Behavior
impl MutationKind<FlowHostSnapshot, FlowMutation> for MoveWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "widget", kind: "move-widget", record: "MovedWidget" };


    fn label(&self) -> crate::LocalizedLabel {
        crate::LocalizedLabel::native(&format!("Move widget {}", self.id), &format!("Widget {} verschieben", self.id))
    }
    fn target(&self) -> Vec<String> { vec!["widgets".into(), self.id.clone()] }
}

impl crate::os_spr::ColdMutationKind<FlowHostSnapshot, FlowMutation> for MoveWidget {
    fn diff(&self, base: &FlowHostSnapshot) -> MutationOutcome<FlowDiff> {
        let from = base.widgets.iter().position(|item| item.id() == &self.id).and_then(|index| flow_wire_index(index).ok()).unwrap_or(u32::MAX);
        MutationOutcome::new(FlowDiff::from(FlowDelta::Widgets(FlowCollectionDelta { moved: vec![FlowMove { id: self.id.clone(), from, to: self.to_index }], ..FlowCollectionDelta::default() })))
    }
    fn inverse(&self, base: &FlowHostSnapshot) -> Result<Vec<FlowMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        base.widgets.iter().position(|item| item.id() == &self.id).and_then(|index| flow_wire_index(index).ok().map(|to_index| FlowMutation::MoveWidget(Self { id: self.id.clone(), to_index }))).into_iter().collect()
    
    })())
}
}

//#endregion 🎮️Behavior
