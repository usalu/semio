//#region 🧬️Std2AnyMutationRoster
//! 🧬️ Std2Any declaration-channel direct mutation roster.
use crate::app::declarations::fixture::{Std2AnySnapshot, Std2AnyDiff};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[path = "📝️set-value/🦀️.rs"]
mod set_value;
pub(crate) use set_value::SetValue;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::Mutations)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
#[mutations(snapshot = Std2AnySnapshot, diff = Std2AnyDiff, schema = "semio.testkit.w1c-fixture.std2-any/v1")]
pub(crate) enum Std2AnyMutation { SetValue(SetValue) }
//#endregion 🧬️Std2AnyMutationRoster
