//! ⛔️ Fatal command preserving the rejected-forward inverse-law test.

use super::super::{CounterDiff, CounterMutation};
use crate::os_spr::{MutationKind, MutationOutcome, OpText, SemanticDescriptor};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl_derive::MutationLeaf, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct AddRejectedCounter {}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<i64, CounterMutation> for AddRejectedCounter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "rejected-counter", kind: "add-rejected-counter", record: "AddedRejectedCounter" };
    fn diff(&self, _base: &i64) -> MutationOutcome<CounterDiff> { MutationOutcome::fatal("mutation.invariant", "boom", ["x"]) }
    fn inverse(&self, _base: &i64) -> Vec<CounterMutation> { Vec::new() }
    fn label(&self) -> String { "Add rejected counter".into() }
}
//#endregion ⚙️Behavior

//#region 📜️Text
impl OpText for AddRejectedCounter {
    fn print_op(&self) -> String { "add-rejected-counter".into() }
    fn parse_op(line: &str) -> Result<Self, crate::os_dsl::TextError> {
        if line == "add-rejected-counter" { Ok(Self {}) } else { Err(crate::os_dsl::TextError::new("expected add-rejected-counter", crate::os_dsl::TextSpan::at(1, 1))) }
    }
}
//#endregion 📜️Text

//#region 🧪️Contract
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_fixture_contract() { super::super::super::tests::assert_leaf::<AddRejectedCounter>(4, CounterMutation::AddRejectedCounter, include_str!("🔣️.json")); }
}
//#endregion 🧪️Contract
