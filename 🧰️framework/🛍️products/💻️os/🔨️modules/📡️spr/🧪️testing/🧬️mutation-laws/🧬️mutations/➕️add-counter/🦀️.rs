//! ➕️ Lawful signed counter addition used by the testkit's own law tests.

use super::super::{CounterDiff, CounterMutation};
use semio_framework_os_spr::{MutationKind, MutationOutcome, OpText, SemanticDescriptor};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl_derive::MutationLeaf, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct AddCounter {
    pub delta: i64,
}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<i64, CounterMutation> for AddCounter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "counter", kind: "add-counter", record: "AddedCounter" };


}

impl semio_framework_os_spr::ColdMutationKind<i64, CounterMutation> for AddCounter {
    fn diff(&self, _base: &i64) -> MutationOutcome<CounterDiff> {
        MutationOutcome::new(CounterDiff::delta(self.delta))
    }
    fn inverse(&self, _base: &i64) -> Result<Vec<CounterMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        CounterDiff::from_wide(-i128::from(self.delta)).deltas.into_iter().rev().map(|delta| Self { delta }.into()).collect()
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add {}", self.delta), &format!("{} hinzufügen", self.delta))
    }
}
//#endregion ⚙️Behavior

//#region 📜️Text
impl OpText for AddCounter {
    fn print_op(&self) -> String {
        format!("add-counter {}", self.delta)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let error = || semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected add-counter <i64>", semio_framework_diagnostic::TextSpan::at(1, 1));
        let delta = line.strip_prefix("add-counter ").ok_or_else(error)?.parse::<i64>().map_err(|_| error())?;
        Ok(Self { delta })
    }
}
//#endregion 📜️Text
