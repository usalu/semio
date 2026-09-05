//! ✌️ Direct add-counter-twice fixture payload and behavior.
use super::super::{AddCounter, Counter, CounterMutation};
use crate::os_spr::{CompositeMutationKind, PlanError, Planner, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl_derive::DslRecord, dsl_derive::MutationLeaf, dsl_derive::CompositeMutation, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "add-counter-twice")]
#[composite(snapshot = Counter, op = CounterMutation)]
pub struct AddCounterTwice { pub delta: i64 }

impl CompositeMutationKind<Counter, CounterMutation> for AddCounterTwice {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "counter", kind: "add-counter-twice", record: "AddedCounterTwice" };
    fn plan(&self, _base: &Counter, planner: &mut Planner<Counter, CounterMutation>) -> Result<(), PlanError> {
        planner.call(CounterMutation::AddCounter(AddCounter { delta: self.delta }))?;
        planner.call(CounterMutation::AddCounter(AddCounter { delta: self.delta }))
    }
    fn label(&self) -> String { format!("Add {} twice", self.delta) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_counter_leaf_contract() { super::super::super::assert_counter_leaf_descriptor::<AddCounterTwice>(include_str!("🔣️.json")); }
    #[test] fn plan_has_two_local_adds() { let base = 0; let mut planner = Planner::new(&base); AddCounterTwice { delta: 3 }.plan(&base, &mut planner).unwrap(); assert_eq!(planner.steps().len(), 2); }
}
