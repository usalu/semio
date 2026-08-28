//! 🔢️ Direct add-counter-sequence fixture payload and behavior.
use super::super::{AddCounter, Counter, CounterMutation};
use crate::os_spr::{CompositeMutationKind, PlanError, Planner, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl_derive::DslRecord, dsl_derive::MutationLeaf, dsl_derive::CompositeMutation)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "add-counter-sequence")]
#[composite(snapshot = Counter, op = CounterMutation)]
pub struct AddCounterSequence { pub deltas: Vec<i64> }

impl CompositeMutationKind<Counter, CounterMutation> for AddCounterSequence {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "counter", kind: "add-counter-sequence", record: "AddedCounterSequence" };
    fn plan(&self, _base: &Counter, planner: &mut Planner<Counter, CounterMutation>) -> Result<(), PlanError> {
        for delta in &self.deltas { planner.call(CounterMutation::AddCounter(AddCounter { delta: *delta }))?; }
        Ok(())
    }
    fn label(&self) -> String { "Add counter sequence".into() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_counter_leaf_contract() { super::super::super::assert_counter_leaf_descriptor::<AddCounterSequence>(include_str!("🔣️.json")); }
    
}

