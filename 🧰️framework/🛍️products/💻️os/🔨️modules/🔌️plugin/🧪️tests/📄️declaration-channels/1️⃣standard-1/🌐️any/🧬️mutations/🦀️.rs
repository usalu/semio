//#region 🧬️Std1AnyMutationRoster
//! 🧬️ Std1Any declaration-channel direct mutation roster.
use crate::app::declarations::fixture::{Std1AnySnapshot, Std1AnyDiff};
use serde::{Deserialize, Serialize};

#[path = "📝️set-value/🦀️.rs"]
mod set_value;
pub(crate) use set_value::SetValue;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(deny_unknown_fields)]
#[mutations(snapshot = Std1AnySnapshot, diff = Std1AnyDiff, schema = "semio.testkit.w1c-fixture.std1-any/v1")]
pub(crate) enum Std1AnyMutation { SetValue(SetValue) }
//#endregion 🧬️Std1AnyMutationRoster
