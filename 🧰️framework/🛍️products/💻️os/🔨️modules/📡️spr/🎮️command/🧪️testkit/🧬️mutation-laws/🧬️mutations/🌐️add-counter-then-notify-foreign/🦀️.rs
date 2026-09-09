//! 🌐️ Direct add-counter-then-notify-foreign fixture payload and behavior.
use super::super::{foreign_step_fixture, AddCounter, Counter, CounterMutation};
use crate::os_spr::{CompositeMutationKind, PlanError, Planner, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl_derive::DslRecord, dsl_derive::MutationLeaf, dsl_derive::CompositeMutation, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "add-counter-then-notify-foreign")]
#[composite(snapshot = Counter, op = CounterMutation)]
pub struct AddCounterThenNotifyForeign {
    pub delta: i64,
    pub foreign_count: u8,
}

impl CompositeMutationKind<Counter, CounterMutation> for AddCounterThenNotifyForeign {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "counter", kind: "add-counter-then-notify-foreign", record: "AddedCounterThenNotifiedForeign" };
    fn plan(&self, _base: &Counter, planner: &mut Planner<Counter, CounterMutation>) -> Result<(), PlanError> {
        planner.call(CounterMutation::AddCounter(AddCounter { delta: self.delta }))?;
        for n in 0..self.foreign_count {
            planner.call_foreign(foreign_step_fixture(n))?;
        }
        Ok(())
    }
    fn label(&self) -> String {
        "Add then notify foreign".into()
    }
}

#[cfg(test)]
#[path = "../../../../🧪️tests/🧬️mutation-laws-mutations-add-counter-then-notify-foreign/🦀️.rs"]
mod tests;
