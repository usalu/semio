use super::tests::{Value, assert_fixture_descriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[path = "🔢️set-value/🦀️.rs"] mod set_value;
pub use set_value::SetValue;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::Mutations)]
#[serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = Value, diff = Value, schema = "presence.fixture")]
pub(crate) enum ValueMutation {
    SetValue(SetValue),
}
