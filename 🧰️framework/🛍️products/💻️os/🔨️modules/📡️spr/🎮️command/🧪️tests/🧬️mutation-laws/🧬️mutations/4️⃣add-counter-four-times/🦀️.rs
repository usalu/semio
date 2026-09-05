//! 4️⃣ Direct add-counter-four-times fixture payload and behavior.
use super::super::{AddCounterTwice, Counter, CounterMutation};
use crate::os_spr::{CompositeMutationKind, PlanError, Planner, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl_derive::DslRecord, dsl_derive::MutationLeaf, dsl_derive::CompositeMutation, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "add-counter-four-times")]
#[composite(snapshot = Counter, op = CounterMutation)]
pub struct AddCounterFourTimes { pub delta: i64 }

impl CompositeMutationKind<Counter, CounterMutation> for AddCounterFourTimes {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "counter", kind: "add-counter-four-times", record: "AddedCounterFourTimes" };
    fn plan(&self, base: &Counter, planner: &mut Planner<Counter, CounterMutation>) -> Result<(), PlanError> {
        AddCounterTwice { delta: self.delta }.plan(base, planner)?;
        let mid = *planner.base();
        AddCounterTwice { delta: self.delta }.plan(&mid, planner)
    }
    fn label(&self) -> String { format!("Add {} four times", self.delta) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_counter_leaf_contract() { super::super::super::assert_counter_leaf_descriptor::<AddCounterFourTimes>(include_str!("🔣️.json")); }
    #[test] fn plan_nests_two_twice_plans() { let base = 0; let mut planner = Planner::new(&base); AddCounterFourTimes { delta: 2 }.plan(&base, &mut planner).unwrap(); assert_eq!(planner.steps().len(), 4); }
}
