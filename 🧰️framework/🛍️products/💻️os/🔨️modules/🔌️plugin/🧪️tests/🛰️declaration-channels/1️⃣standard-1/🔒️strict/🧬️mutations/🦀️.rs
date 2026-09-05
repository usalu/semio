//#region 🧬️Std1StrictMutationRoster
//! 🧬️ Std1Strict declaration-channel direct mutation roster.
use crate::app::declarations::fixture::{Std1StrictSnapshot, Std1StrictDiff};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[path = "📝️set-value/🦀️.rs"]
mod set_value;
pub(crate) use set_value::SetValue;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::Mutations)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
#[mutations(snapshot = Std1StrictSnapshot, diff = Std1StrictDiff, schema = "semio.testkit.w1c-fixture.std1-strict/v1")]
pub(crate) enum Std1StrictMutation { SetValue(SetValue) }
//#endregion 🧬️Std1StrictMutationRoster
