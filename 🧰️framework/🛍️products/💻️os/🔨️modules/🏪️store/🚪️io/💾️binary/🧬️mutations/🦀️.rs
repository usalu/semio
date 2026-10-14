//! 💾️ Space-history structured binary mutation encoding.
use semio_framework_value::{ToValue,FromValue};
use crate::os_spr::OpBinary;
use crate::os_pack::pack_rt;
use crate::os_pack::pack_rt::renormalize_whole_number_floats;
use crate::os_store::SpaceHistoryMutation;
impl OpBinary for SpaceHistoryMutation {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        let value = ToValue::to_value(self);
        Ok(pack_rt::encode_pack_value(&value))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        let value = pack_rt::decode_pack_value(bytes).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "space history op", offset: 0, detail: error.to_string() })?;
        FromValue::from_value(renormalize_whole_number_floats(value)).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "space history op", offset: 0, detail: error.to_string() })
    }
}
