//! 📚️ Required original history receiving bounds belong below every product and typed consumer.
use serde::{Serialize,Deserialize};
use semio_framework_value::{ValueError,ValueRefusalKind};
#[derive(Clone,Copy,Debug,PartialEq,Eq,Serialize,Deserialize,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct HistoryReadLimits{
 pub file_bytes:u32,pub frame_body_bytes:u32,pub records:u32,pub maximum_fields:u32,pub maximum_text_bytes:u32,pub maximum_binary_bytes:u32,pub maximum_json_bytes:u32,pub maximum_json_allocation_bytes:u32,pub maximum_json_items:u32,pub maximum_json_depth:u16,
}
impl HistoryReadLimits{
 pub fn validate(self)->Result<(),ValueError>{if [self.file_bytes,self.frame_body_bytes,self.records,self.maximum_fields,self.maximum_text_bytes,self.maximum_binary_bytes,self.maximum_json_bytes,self.maximum_json_allocation_bytes,self.maximum_json_items].contains(&0)||self.maximum_json_depth==0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original history receiving bounds are invalid"))}Ok(())}
}
