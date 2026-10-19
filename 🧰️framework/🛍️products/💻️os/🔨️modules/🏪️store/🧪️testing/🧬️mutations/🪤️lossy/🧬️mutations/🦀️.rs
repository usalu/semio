use super::{DemoSnapshot, LossyDiff, assert_fixture_descriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[path = "🔢️set-n/🦀️.rs"]
mod set_n;
pub use set_n::SetN;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_os_dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
#[mutations(snapshot = DemoSnapshot, diff = LossyDiff, schema = "lossy.doc", reference = DemoSnapshot, reference_diff = LossyDiff)]
pub enum LossyMutation {
    SetN(SetN),
}

impl semio_framework_os_spr::OpBinary for LossyMutation {
    fn encode_op(&self) -> Result<Vec<u8>, semio_framework_os_spr::ProtocolError> {
        let Self::SetN(value) = self;
        semio_framework_os_spr::OpBinary::encode_op(value)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, semio_framework_os_spr::ProtocolError> {
        <SetN as semio_framework_os_spr::OpBinary>::decode_op(bytes).map(Self::SetN)
    }
}
