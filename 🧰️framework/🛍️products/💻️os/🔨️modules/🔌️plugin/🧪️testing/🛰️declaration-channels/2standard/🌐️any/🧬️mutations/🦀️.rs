//#region 🧬️Std2AnyMutationRoster
//! 🧬️ Std2Any declaration-channel direct mutation roster.
use crate::declarations_fixture_tests::fixture::{Std2AnyDiff, Std2AnySnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[path = "📝️set-value/🦀️.rs"]
pub(crate) mod set_value;
pub(crate) use crate::declaration_fixture_mutations::std2_any::set_value::SetValue;

#[derive(semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::Mutations)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
#[mutations(snapshot = Std2AnySnapshot, diff = Std2AnyDiff, schema = "semio.testkit.w1c-fixture.std2-any/v1", reference = Std2AnySnapshot, reference_diff = Std2AnyDiff)]
pub(crate) enum Std2AnyMutation {
    SetValue(SetValue),
}
//#endregion 🧬️Std2AnyMutationRoster
