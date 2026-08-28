//! 🐛️ Deliberately incorrect missing-target command used to prove a law assertion fails.

use super::super::{CounterDiff, CounterMutation};
use crate::os_spr::{MutationKind, MutationOutcome, OpText, SemanticDescriptor};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl_derive::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
pub struct AddUncheckedCounter {}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<i64, CounterMutation> for AddUncheckedCounter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "unchecked-counter", kind: "add-unchecked-counter", record: "AddedUncheckedCounter" };
    fn diff(&self, _base: &i64) -> MutationOutcome<CounterDiff> { MutationOutcome::new(CounterDiff::delta(1)) }
    fn inverse(&self, _base: &i64) -> Vec<CounterMutation> { vec![Self {}.into()] }
    fn label(&self) -> String { "Add to unchecked counter".into() }
}
//#endregion ⚙️Behavior

//#region 📜️Text
impl OpText for AddUncheckedCounter {
    fn print_op(&self) -> String { "add-unchecked-counter".into() }
    fn parse_op(line: &str) -> Result<Self, crate::os_dsl::TextError> {
        if line == "add-unchecked-counter" { Ok(Self {}) } else { Err(crate::os_dsl::TextError::new("expected add-unchecked-counter", crate::os_dsl::TextSpan::at(1, 1))) }
    }
}
//#endregion 📜️Text

//#region 🧪️Contract
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_fixture_contract() { super::super::super::tests::assert_leaf::<AddUncheckedCounter>(2, CounterMutation::AddUncheckedCounter, include_str!("🔣️.json")); }
}
//#endregion 🧪️Contract
