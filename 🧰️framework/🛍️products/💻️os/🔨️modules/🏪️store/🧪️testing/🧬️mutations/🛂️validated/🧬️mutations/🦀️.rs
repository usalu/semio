use super::{DemoDiff, DemoSnapshot, assert_fixture_descriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[path = "🔢️set-n/🦀️.rs"]
mod set_n;
pub use set_n::SetN;
#[path = "↩️assign-n/🦀️.rs"]
mod assign_n;
pub use assign_n::AssignN;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::Mutations, semio_framework_dsl_record_derive::DslEnum)]
#[serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = DemoSnapshot, diff = DemoDiff, schema = "validated.doc")]
pub(crate) enum ValidatedMutation {
    SetN(SetN),
    AssignN(AssignN),
}
