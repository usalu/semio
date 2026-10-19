//! 4️⃣ Direct add-counter-four-times fixture payload and behavior.
use super::super::{AddCounterTwice, Counter, CounterMutation};
use semio_framework_os_command::CompositeMutationKind;
use semio_framework_os_command::PlanError;
use semio_framework_os_command::Planner;
use semio_framework_os_command::SemanticDescriptor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_dsl_record_derive::DslRecord, dsl_derive::MutationLeaf, dsl_derive::CompositeMutation, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "add-counter-four-times")]
#[composite(snapshot = Counter, op = CounterMutation, reference = Counter)]
pub struct AddCounterFourTimes {
    pub delta: i64,
}

impl CompositeMutationKind<Counter, CounterMutation> for AddCounterFourTimes {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "counter", kind: "add-counter-four-times", record: "AddedCounterFourTimes" };

    fn label(&self) -> crate::LocalizedLabel {
        crate::LocalizedLabel::native(&format!("Add {} four times", self.delta), &format!("{} viermal hinzufügen", self.delta))
    }
}

impl semio_framework_os_command::ColdCompositeMutationKind<Counter, CounterMutation> for AddCounterFourTimes {
    fn plan(&self, base: &Counter, planner: &mut Planner<Counter, CounterMutation>) -> Result<(), PlanError> {
        AddCounterTwice { delta: self.delta }.plan(base, planner)?;
        let mid = *planner.base();
        AddCounterTwice { delta: self.delta }.plan(&mid, planner)
    }
}

#[cfg(test)]
#[path = "../../../../🧪️tests/🧬️mutation-laws-mutations-add-counter-four-times/🦀️.rs"]
mod tests;
