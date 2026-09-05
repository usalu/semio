//#region 🧬️ContributedMutationWireRoster
//! 🧬️ Transparent contributed-wire mutation roster.

use super::{WireTestDiff, WireTestSnapshot};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[path = "➕️add-value/🦀️.rs"]
mod add_value;
pub(crate) use add_value::AddValue;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::Mutations)]
#[serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = WireTestSnapshot, diff = WireTestDiff, schema = "wiretest.contributed.document")]
pub(crate) enum WireTestMutation {
    AddValue(AddValue),
}

impl protocol::OpBinary for WireTestMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| crate::store::PackError::Schema(error.to_string()).into())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| crate::store::PackError::Schema(error.to_string()).into())
    }
}
//#endregion 🧬️ContributedMutationWireRoster
