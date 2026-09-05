//! ➕️ Lawful signed counter addition used by the testkit's own law tests.

use super::super::{CounterDiff, CounterMutation};
use crate::os_spr::{MutationKind, MutationOutcome, OpText, SemanticDescriptor};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl_derive::MutationLeaf, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct AddCounter { pub delta: i64 }
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<i64, CounterMutation> for AddCounter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "counter", kind: "add-counter", record: "AddedCounter" };
    fn diff(&self, _base: &i64) -> MutationOutcome<CounterDiff> { MutationOutcome::new(CounterDiff::delta(self.delta)) }
    fn inverse(&self, _base: &i64) -> Vec<CounterMutation> {
        CounterDiff::from_wide(-i128::from(self.delta)).deltas.into_iter().rev().map(|delta| Self { delta }.into()).collect()
    }
    fn label(&self) -> String { format!("Add {}", self.delta) }
}
//#endregion ⚙️Behavior

//#region 📜️Text
impl OpText for AddCounter {
    fn print_op(&self) -> String { format!("add-counter {}", self.delta) }
    fn parse_op(line: &str) -> Result<Self, crate::os_dsl::TextError> {
        let error = || crate::os_dsl::TextError::new("expected add-counter <i64>", crate::os_dsl::TextSpan::at(1, 1));
        let delta = line.strip_prefix("add-counter ").ok_or_else(error)?.parse::<i64>().map_err(|_| error())?;
        Ok(Self { delta })
    }
}
//#endregion 📜️Text

//#region 🧪️Contract
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_fixture_contract() { super::super::super::tests::assert_leaf::<AddCounter>(0, CounterMutation::AddCounter, include_str!("🔣️.json")); }
}
//#endregion 🧪️Contract
