//! 🌐️ Direct add-counter-then-notify-foreign fixture payload and behavior.
use super::super::{foreign_step_fixture, AddCounter, Counter, CounterMutation};
use crate::os_spr::{CompositeMutationKind, PlanError, Planner, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_dsl_record_derive::DslRecord, dsl_derive::MutationLeaf, dsl_derive::CompositeMutation, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
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
    fn may_emit_foreign_steps(&self) -> bool { true }
    fn foreign_step_source<'a>(&'a self, _: &'a Counter, index: usize) -> Result<Option<crate::os_spr::ForeignStepSource<'a>>, semio_framework_value::ValueError> {
        if self.foreign_count > crate::os_spr::MAX_PLAN_DEPTH { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "foreign plan exceeds its original depth")); }
        if index >= usize::from(self.foreign_count) { return Ok(None); }
        const TARGETS: [&str;8] = ["artifact-0","artifact-1","artifact-2","artifact-3","artifact-4","artifact-5","artifact-6","artifact-7"];
        const LABELS: [&str;8] = ["Recolor widget 0","Recolor widget 1","Recolor widget 2","Recolor widget 3","Recolor widget 4","Recolor widget 5","Recolor widget 6","Recolor widget 7"];
        const PAYLOADS: [[u8;1];8] = [[0],[1],[2],[3],[4],[5],[6],[7]];
        Ok(Some(crate::os_spr::ForeignStepSource { artifact_id: TARGETS[index], artifact_kind: "s.demo.widget", dialect: None, mutation_id: "widget.doc#set-color", payload: &PAYLOADS[index], label: LABELS[index] }))
    }
    fn label(&self) -> crate::LocalizedLabel {
        crate::LocalizedLabel::native("Add then notify foreign", "Hinzufügen und Fremddokument benachrichtigen")
    }
}

#[cfg(test)]
#[path = "../../../../🧪️tests/🧬️mutation-laws-mutations-add-counter-then-notify-foreign/🦀️.rs"]
mod tests;
