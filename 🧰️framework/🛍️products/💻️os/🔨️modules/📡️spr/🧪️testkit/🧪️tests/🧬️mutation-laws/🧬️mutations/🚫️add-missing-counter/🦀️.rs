//! 🚫️ Missing-target command preserving the law helper's positive error fixture.

use super::super::{CounterDiff, CounterMutation};
use crate::os_spr::{MutationKind, MutationOutcome, OpText, SemanticDescriptor};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl_derive::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
pub struct AddMissingCounter {}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<i64, CounterMutation> for AddMissingCounter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "missing-counter", kind: "add-missing-counter", record: "AddedMissingCounter" };
    fn diff(&self, _base: &i64) -> MutationOutcome<CounterDiff> { MutationOutcome::error("mutation.target-missing", "target absent", ["thing"]) }
    fn inverse(&self, _base: &i64) -> Vec<CounterMutation> { Vec::new() }
    fn label(&self) -> String { "Add to missing counter".into() }
}
//#endregion ⚙️Behavior

//#region 📜️Text
impl OpText for AddMissingCounter {
    fn print_op(&self) -> String { "add-missing-counter".into() }
    fn parse_op(line: &str) -> Result<Self, crate::os_dsl::TextError> {
        if line == "add-missing-counter" { Ok(Self {}) } else { Err(crate::os_dsl::TextError::new("expected add-missing-counter", crate::os_dsl::TextSpan::at(1, 1))) }
    }
}
//#endregion 📜️Text

//#region 🧪️Contract
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_fixture_contract() { super::super::super::tests::assert_leaf::<AddMissingCounter>(1, CounterMutation::AddMissingCounter, include_str!("🔣️.json")); }
}
//#endregion 🧪️Contract
