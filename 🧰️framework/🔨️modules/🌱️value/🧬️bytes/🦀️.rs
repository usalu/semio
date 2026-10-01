//! 🧬️ Intrinsic octets have one owned value node and a canonical padded base64 literal.
use super::{DslValue,FromValue,ValueError};
/// 🧬️ Explicit field mapping for an intrinsic octet sequence.
pub fn to_value(bytes:&[u8])->DslValue{DslValue::Bytes(bytes.to_vec())}
/// 🧬️ Accepts owned native octets or their canonical JSON array projection.
pub fn from_value(value:DslValue)->Result<Vec<u8>,ValueError>{match value{DslValue::Bytes(bytes)=>Ok(bytes),value@DslValue::Array(_)=>Vec::<u8>::from_value(value),_=>Err(ValueError::new("expected intrinsic octets or their JSON array projection"))}}
/// 🔤️ Uses the single framework-owned standard base64 codec.
pub fn encode_base64(bytes:&[u8])->String{crate::base64_standard_encode(bytes)}
/// 🔤️ Uses the canonical decoder's strict padding and unused-bit checks.
pub fn decode_base64(text:&str)->Result<Vec<u8>,String>{crate::base64_standard_decode(text).map_err(|error|error.to_string())}

/// 🧬️ Optional intrinsic octets preserve absence independently from an empty sequence.
pub mod optional{
    use super::*;
    pub fn to_value(value:&Option<Vec<u8>>)->DslValue{value.as_deref().map_or(DslValue::Null,super::to_value)}
    pub fn from_value(value:DslValue)->Result<Option<Vec<u8>>,ValueError>{if value.is_null(){Ok(None)}else{super::from_value(value).map(Some)}}
}
