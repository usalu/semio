//! 👁️ Deliberately nondeterministic command retaining shared, nonserialized observation state.

use super::super::{CounterDiff, CounterMutation};
use crate::os_spr::{MutationKind, MutationOutcome, OpText, SemanticDescriptor};

//#region 🧬️Payload
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, dsl_derive::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(deny_unknown_fields)]
pub struct AddObservedCounter {
    #[serde(skip)]
    calls: std::rc::Rc<std::cell::Cell<i64>>,
}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<i64, CounterMutation> for AddObservedCounter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "observed-counter", kind: "add-observed-counter", record: "AddedObservedCounter" };
    fn diff(&self, _base: &i64) -> MutationOutcome<CounterDiff> {
        let count = self.calls.get();
        self.calls.set(count + 1);
        MutationOutcome::new(CounterDiff::delta(count))
    }
    fn inverse(&self, _base: &i64) -> Vec<CounterMutation> { Vec::new() }
    fn label(&self) -> String { "Add observed counter".into() }
}
//#endregion ⚙️Behavior

//#region 📜️Text
impl OpText for AddObservedCounter {
    fn print_op(&self) -> String { "add-observed-counter".into() }
    fn parse_op(line: &str) -> Result<Self, crate::os_dsl::TextError> {
        if line == "add-observed-counter" { Ok(Self::default()) } else { Err(crate::os_dsl::TextError::new("expected add-observed-counter", crate::os_dsl::TextSpan::at(1, 1))) }
    }
}
//#endregion 📜️Text

//#region 🧪️Contract
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_fixture_contract() { super::super::super::tests::assert_leaf::<AddObservedCounter>(3, CounterMutation::AddObservedCounter, include_str!("🔣️.json")); }
}
//#endregion 🧪️Contract
