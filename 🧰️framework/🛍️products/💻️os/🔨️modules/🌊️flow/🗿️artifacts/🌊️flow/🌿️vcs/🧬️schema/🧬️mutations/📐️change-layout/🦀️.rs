//! 📐️ Change Layout direct payload and owned behavior.
use super::super::{FlowHostSnapshot, FlowDiff, FlowDelta, FlowLayoutEntry, FlowMutation};
use crate::os_spr::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, crate::os_dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "change-layout")]
pub struct ChangeLayout { pub entries: Vec<FlowLayoutEntry> }

//#endregion 🧬️Payload

//#region 🎮️Behavior
impl MutationKind<FlowHostSnapshot, FlowMutation> for ChangeLayout {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "layout", kind: "change-layout", record: "ChangedLayout" };
    fn diff(&self, _base: &FlowHostSnapshot) -> MutationOutcome<FlowDiff> {
        MutationOutcome::new(FlowDiff::from(FlowDelta::Layout(self.entries.clone())))
    }
    fn inverse(&self, base: &FlowHostSnapshot) -> Result<Vec<FlowMutation>, semio_framework_value::ValueError> {
        Ok(self
            .entries
            .iter()
            .enumerate()
            .map(|(at, entry)| {
                let previous = match self.entries[..at].iter().rev().find(|earlier| earlier.id == entry.id) {
                    Some(earlier) => earlier.layout.clone(),
                    None => base.layout.get(&entry.id).cloned(),
                };
                FlowMutation::ChangeLayout(Self { entries: vec![FlowLayoutEntry { id: entry.id.clone(), layout: previous }] })
            })
            .collect())
    }
    fn label(&self) -> crate::LocalizedLabel {
        crate::LocalizedLabel::native("Change layout", "Layout ändern")
    }
    fn target(&self) -> Vec<String> { vec!["layout".into()] }
}

//#endregion 🎮️Behavior

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
