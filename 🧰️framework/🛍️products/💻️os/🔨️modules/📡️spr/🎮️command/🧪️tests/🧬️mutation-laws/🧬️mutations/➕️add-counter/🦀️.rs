//! ➕️ Direct add-counter fixture payload and behavior.
use super::super::{Counter, CounterDiff, CounterMutation};
use crate::os_spr::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl_derive::DslRecord, dsl_derive::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "add-counter")]
pub struct AddCounter { pub delta: i64 }

impl MutationKind<Counter, CounterMutation> for AddCounter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "counter", kind: "add-counter", record: "AddedCounter" };
    fn diff(&self, _base: &Counter) -> MutationOutcome<CounterDiff> { MutationOutcome::new(CounterDiff { deltas: vec![self.delta] }) }
    fn inverse(&self, _base: &Counter) -> Vec<CounterMutation> {
        if self.delta == i64::MIN { vec![CounterMutation::AddCounter(AddCounter { delta: 1 }), CounterMutation::AddCounter(AddCounter { delta: i64::MAX })] }
        else { vec![CounterMutation::AddCounter(AddCounter { delta: -self.delta })] }
    }
    fn label(&self) -> String { format!("Add {}", self.delta) }
    fn target(&self) -> Vec<String> { vec!["value".into()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_counter_leaf_contract() { super::super::super::assert_counter_leaf_descriptor::<AddCounter>(include_str!("🔣️.json")); }
    
}
