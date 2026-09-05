//! 🧬️ Transparent roster for the lawful and intentionally unlawful law-test commands.

use crate::os_spr::{MutationLeaf, OpText};

//#region 🧬️Leaves
#[path = "➕️add-counter/🦀️.rs"] mod add_counter;
#[path = "🚫️add-missing-counter/🦀️.rs"] mod add_missing_counter;
#[path = "🐛️add-unchecked-counter/🦀️.rs"] mod add_unchecked_counter;
#[path = "👁️add-observed-counter/🦀️.rs"] mod add_observed_counter;
#[path = "⛔️add-rejected-counter/🦀️.rs"] mod add_rejected_counter;
pub use add_counter::AddCounter;
pub use add_missing_counter::AddMissingCounter;
pub use add_unchecked_counter::AddUncheckedCounter;
pub use add_observed_counter::AddObservedCounter;
pub use add_rejected_counter::AddRejectedCounter;
//#endregion 🧬️Leaves

//#region 🧬️Aggregate
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl_derive::Mutations, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = i64, diff = super::CounterDiff, schema = "testkit.counter")]
pub enum CounterMutation {
    AddCounter(AddCounter),
    AddMissingCounter(AddMissingCounter),
    AddUncheckedCounter(AddUncheckedCounter),
    AddObservedCounter(AddObservedCounter),
    AddRejectedCounter(AddRejectedCounter),
}
//#endregion 🧬️Aggregate

//#region 📜️Text
impl OpText for CounterMutation {
    fn print_op(&self) -> String {
        match self {
            Self::AddCounter(value) => value.print_op(),
            Self::AddMissingCounter(value) => value.print_op(),
            Self::AddUncheckedCounter(value) => value.print_op(),
            Self::AddObservedCounter(value) => value.print_op(),
            Self::AddRejectedCounter(value) => value.print_op(),
        }
    }

    fn parse_op(line: &str) -> Result<Self, crate::os_dsl::TextError> {
        let opcode = line.split_once(' ').map_or(line, |(opcode, _)| opcode);
        match opcode {
            value if Some(value) == AddCounter::DESCRIPTOR.text_opcode => AddCounter::parse_op(line).map(Into::into),
            value if Some(value) == AddMissingCounter::DESCRIPTOR.text_opcode => AddMissingCounter::parse_op(line).map(Into::into),
            value if Some(value) == AddUncheckedCounter::DESCRIPTOR.text_opcode => AddUncheckedCounter::parse_op(line).map(Into::into),
            value if Some(value) == AddObservedCounter::DESCRIPTOR.text_opcode => AddObservedCounter::parse_op(line).map(Into::into),
            value if Some(value) == AddRejectedCounter::DESCRIPTOR.text_opcode => AddRejectedCounter::parse_op(line).map(Into::into),
            _ => Err(crate::os_dsl::TextError::new("unknown counter operation", crate::os_dsl::TextSpan::at(1, 1))),
        }
    }
}
//#endregion 📜️Text
