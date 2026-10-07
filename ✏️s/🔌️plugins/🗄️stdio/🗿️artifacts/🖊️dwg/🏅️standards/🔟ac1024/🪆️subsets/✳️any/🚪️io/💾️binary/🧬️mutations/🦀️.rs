//! dwg rep for stdio.dwg 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

use crate::standards::v_ac1024::subsets::any::schema::mutations::DwgMutation;
impl protocol::OpBinary for DwgMutation {
 fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{dsl::tagged_value_binary::encode_op(COMPONENT_PROTOCOL_SEMIO,dsl::tagged_value_binary::VariantTag::Field("mutation"),self)}
 fn decode_op(bytes:&[u8])->Result<Self,protocol::ProtocolError>{dsl::tagged_value_binary::decode_op(COMPONENT_PROTOCOL_SEMIO,dsl::tagged_value_binary::VariantTag::Field("mutation"),bytes)}
}
