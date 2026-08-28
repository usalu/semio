//! 📐️ Change Layout direct payload and owned behavior.
use super::super::{FlowFixture, FlowDiff, FlowDelta, FlowLayoutEntry, FlowMutation, WidgetLayout};
use crate::os_spr::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, crate::os_dsl::DslRecord, crate::os_dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "change-layout")]
pub struct ChangeLayout { pub entries: Vec<FlowLayoutEntry> }

//#endregion 🧬️Payload

//#region 🎮️Behavior
impl MutationKind<FlowFixture, FlowMutation> for ChangeLayout {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "layout", kind: "change-layout", record: "ChangedLayout" };
    fn diff(&self, _base: &FlowFixture) -> MutationOutcome<FlowDiff> {
        MutationOutcome::new(FlowDiff::from(FlowDelta::Layout(self.entries.clone())))
    }
    fn inverse(&self, base: &FlowFixture) -> Vec<FlowMutation> {
        let mut layout: std::collections::BTreeMap<String, WidgetLayout> = base.layout.iter().map(|(id, layout)| (id.clone(), layout.clone())).collect();
        let mut inverse = Vec::with_capacity(self.entries.len());
        for entry in &self.entries {
            let previous = layout.get(&entry.id).cloned();
            inverse.push(FlowMutation::ChangeLayout(Self { entries: vec![FlowLayoutEntry { id: entry.id.clone(), layout: previous }] }));
            match &entry.layout {
                Some(value) => { layout.insert(entry.id.clone(), value.clone()); }
                None => { layout.remove(&entry.id); }
            }
        }
        inverse
    }
    fn label(&self) -> String { "Change layout".into() }
    fn target(&self) -> Vec<String> { vec!["layout".into()] }
}

//#endregion 🎮️Behavior

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::flow_direct_tests::assert_leaf_contract::<ChangeLayout>(8, FlowMutation::ChangeLayout, include_str!("🔣️.json")); }
}
//#endregion 🧪️Tests
