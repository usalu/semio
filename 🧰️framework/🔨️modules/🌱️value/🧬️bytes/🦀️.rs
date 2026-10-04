//! 🧬️ Intrinsic octets have one owned value node and a canonical padded base64 literal.
use super::{DslValue,FromValue,ValueError};
/// 🧬️ Explicit field mapping for an intrinsic octet sequence.
pub fn to_value(bytes:&[u8])->DslValue{DslValue::Bytes(bytes.to_vec())}
/// 🛫️ Copies octets only after cumulative native output admission.
pub fn to_value_controlled(bytes:&[u8],control:&mut super::NativeEncodeControl<'_>)->Result<DslValue,ValueError>{control.copy_bytes(bytes).map(DslValue::Bytes)}
/// 🧬️ Accepts owned native octets or their canonical JSON array projection.
pub fn from_value(value:DslValue)->Result<Vec<u8>,ValueError>{match value{DslValue::Bytes(bytes)=>Ok(bytes),value@DslValue::Array(_)=>Vec::<u8>::from_value(value),_=>Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected intrinsic octets or their JSON array projection"))}}
/// 🛬️ Owns intrinsic octets under the same physical and typed construction budget.
pub fn from_value_controlled(value:&DslValue,control:&mut super::NativeDecodeControl<'_>)->Result<Vec<u8>,ValueError>{match value{DslValue::Bytes(bytes)=>control.copy_bytes(bytes),DslValue::Array(_)=>Vec::<u8>::from_value_controlled(value,control),_=>Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected intrinsic octets or their JSON array projection"))}}
/// 🔤️ Uses the single framework-owned standard base64 codec.
pub fn encode_base64(bytes:&[u8])->String{crate::base64_standard_encode(bytes)}
/// 🔤️ Uses the canonical decoder's strict padding and unused-bit checks.
pub fn decode_base64(text:&str)->Result<Vec<u8>,String>{crate::base64_standard_decode(text).map_err(|error|error.to_string())}

/// 🧬️ Optional intrinsic octets preserve absence independently from an empty sequence.
pub mod optional{
    use super::*;
    pub fn to_value(value:&Option<Vec<u8>>)->DslValue{value.as_deref().map_or(DslValue::Null,super::to_value)}
    /// 🛫️ Admits a present sequence while preserving absent and present-empty states.
    pub fn to_value_controlled(value:&Option<Vec<u8>>,control:&mut super::super::NativeEncodeControl<'_>)->Result<DslValue,ValueError>{match value{Some(bytes)=>super::to_value_controlled(bytes,control),None=><() as super::super::ToValue>::to_value_controlled(&(),control)}}
    /// 🛬️ Preserves explicit absence while admitting each present octet copy.
    pub fn from_value_controlled(value:&DslValue,control:&mut super::super::NativeDecodeControl<'_>)->Result<Option<Vec<u8>>,ValueError>{if value.is_null(){control.step()?;Ok(None)}else{super::from_value_controlled(value,control).map(Some)}}
    pub fn from_value(value:DslValue)->Result<Option<Vec<u8>>,ValueError>{if value.is_null(){Ok(None)}else{super::from_value(value).map(Some)}}
}
