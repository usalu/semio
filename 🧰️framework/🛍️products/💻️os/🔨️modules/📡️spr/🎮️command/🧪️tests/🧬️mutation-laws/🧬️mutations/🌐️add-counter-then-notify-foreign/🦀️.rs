//! 🌐️ Direct add-counter-then-notify-foreign fixture payload and behavior.
use super::super::{AddCounter, Counter, CounterMutation, foreign_step_fixture};
use crate::os_spr::{CompositeMutationKind, PlanError, Planner, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl_derive::DslRecord, dsl_derive::MutationLeaf, dsl_derive::CompositeMutation)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "add-counter-then-notify-foreign")]
#[composite(snapshot = Counter, op = CounterMutation)]
pub struct AddCounterThenNotifyForeign { pub delta: i64, pub foreign_count: u8 }

impl CompositeMutationKind<Counter, CounterMutation> for AddCounterThenNotifyForeign {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "counter", kind: "add-counter-then-notify-foreign", record: "AddedCounterThenNotifiedForeign" };
    fn plan(&self, _base: &Counter, planner: &mut Planner<Counter, CounterMutation>) -> Result<(), PlanError> {
        planner.call(CounterMutation::AddCounter(AddCounter { delta: self.delta }))?;
        for n in 0..self.foreign_count { planner.call_foreign(foreign_step_fixture(n))?; }
        Ok(())
    }
    fn label(&self) -> String { "Add then notify foreign".into() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_counter_leaf_contract() { super::super::super::assert_counter_leaf_descriptor::<AddCounterThenNotifyForeign>(include_str!("🔣️.json")); }
    #[test] fn plan_keeps_local_add_before_foreign_steps() { let base = 0; let mut planner = Planner::new(&base); AddCounterThenNotifyForeign { delta: 2, foreign_count: 2 }.plan(&base, &mut planner).unwrap(); assert_eq!(planner.steps().len(), 3); }
}
