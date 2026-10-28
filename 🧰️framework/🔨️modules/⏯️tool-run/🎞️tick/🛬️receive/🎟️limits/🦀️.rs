//! 📏️ Required declaration bounds remain unchanged from run capture through every receiving decoder.
use serde::{Serialize,Deserialize};
use semio_framework_value::{ValueError,ValueRefusalKind};

#[derive(Clone,Copy,Debug,PartialEq,Eq,Serialize,Deserialize,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ToolRunRecordReadLimits{
 pub maximum_file_bytes:u32,pub maximum_segment_bytes:u32,pub maximum_symbols:u32,pub maximum_depth:u16,pub maximum_items:u32,pub maximum_total_allocation_bytes:u32,pub maximum_symbol_utf8_bytes:u32,pub maximum_symbol_scalars:u32,pub maximum_cursor_allocation_bytes:u32,
}

impl ToolRunRecordReadLimits{
 pub fn validate(self)->Result<(),ValueError>{if [self.maximum_file_bytes,self.maximum_segment_bytes,self.maximum_symbols,self.maximum_items,self.maximum_total_allocation_bytes,self.maximum_cursor_allocation_bytes].contains(&0)||self.maximum_depth==0||self.maximum_cursor_allocation_bytes>self.maximum_total_allocation_bytes{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original ToolRun record receiving bounds are invalid"))}Ok(())}
 pub fn record_body(self)->pack::record::RetainedRecordBodyReadLimits{pack::record::RetainedRecordBodyReadLimits{pack:pack::PackLimits{max_file_len:u64::from(self.maximum_file_bytes),max_segment_len:u64::from(self.maximum_segment_bytes),max_symbols:self.maximum_symbols,max_depth:self.maximum_depth,max_items:u64::from(self.maximum_items),max_total_alloc:u64::from(self.maximum_total_allocation_bytes)},maximum_symbol_utf8_bytes:self.maximum_symbol_utf8_bytes as usize,maximum_symbol_scalars:self.maximum_symbol_scalars as usize,maximum_allocation_bytes:self.maximum_cursor_allocation_bytes as usize}}
}

pub use protocol::codec::HistoryReadLimits as ToolRunHistoryReadLimits;

#[derive(Clone,Copy,Debug,PartialEq,Eq,Serialize,Deserialize,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ToolRunReceivingLimits{pub tick:ToolRunRecordReadLimits,pub record_body:ToolRunRecordReadLimits,pub history:ToolRunHistoryReadLimits}
impl ToolRunReceivingLimits{
 pub const REPORTS:Self=Self{
  tick:ToolRunRecordReadLimits{maximum_file_bytes:262144,maximum_segment_bytes:262144,maximum_symbols:4096,maximum_depth:64,maximum_items:262144,maximum_total_allocation_bytes:16777216,maximum_symbol_utf8_bytes:262144,maximum_symbol_scalars:262144,maximum_cursor_allocation_bytes:4194304},
  record_body:ToolRunRecordReadLimits{maximum_file_bytes:1048576,maximum_segment_bytes:1048576,maximum_symbols:16384,maximum_depth:64,maximum_items:1048576,maximum_total_allocation_bytes:67108864,maximum_symbol_utf8_bytes:1048576,maximum_symbol_scalars:1048576,maximum_cursor_allocation_bytes:16777216},
  history:ToolRunHistoryReadLimits{file_bytes:67108864,frame_body_bytes:1048576,records:8192,maximum_fields:128,maximum_text_bytes:1048576,maximum_binary_bytes:1048576,maximum_json_bytes:1048576,maximum_json_allocation_bytes:8388608,maximum_json_items:65536,maximum_json_depth:64},
 };
 pub fn validate(self)->Result<(),ValueError>{self.tick.validate()?;self.record_body.validate()?;self.history.validate()}
}
