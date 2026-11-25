//! 💾️ Controlled binary representation of authored chart mutations.
use crate::ChangeChartValue;
impl protocol::OpBinary for ChangeChartValue{
 fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{Ok(protocol::os_pack::pack_rt::encode_pack_value(&semio_framework_value::ToValue::to_value(self)))}
 fn decode_op(bytes:&[u8])->Result<Self,protocol::ProtocolError>{let malformed=|detail:String|protocol::ProtocolError::Malformed{what:"chart-mutation",offset:0,detail};let value=protocol::os_pack::pack_rt::decode_pack_value(bytes).map_err(|error|malformed(error.to_string()))?;semio_framework_value::FromValue::from_value(protocol::os_pack::pack_rt::renormalize_whole_number_floats(value)).map_err(|error|malformed(error.to_string()))}
}
protocol::original_pack_value_operation_receiving!(ChangeChartValue);
