use super::{DemoSnapshot, LossyDiff, assert_fixture_descriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[path = "🔢️set-n/🦀️.rs"] mod set_n;
pub use set_n::SetN;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::Mutations)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
#[mutations(snapshot = DemoSnapshot, diff = LossyDiff, schema = "lossy.doc")]
pub(crate) enum LossyMutation {
    SetN(SetN),
}

impl crate::os_spr::OpBinary for LossyMutation {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> { let Self::SetN(value) = self; crate::os_spr::OpBinary::encode_op(value) }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> { <SetN as crate::os_spr::OpBinary>::decode_op(bytes).map(Self::SetN) }
}
