//! 📤️ Owned return words preserve original refusal kinds without a physical result allocation.
use semio_framework_value::{ValueError,ValueRefusalKind};

#[derive(Debug,PartialEq,Eq)]
pub enum OwnedReturn{Bytes{pointer:u32,length:u32},Refusal(ValueRefusalKind)}

/// 🔓️ Validates the reserved scalar extent before any result storage is read or admitted.
pub fn decode(word:u64)->Result<OwnedReturn,ValueError>{
    let pointer=word as u32;let length=(word>>32)as u32;
    if length!=u32::MAX{return Ok(OwnedReturn::Bytes{pointer,length});}
    if !(1..=8).contains(&pointer){return Err(super::refusal(4));}
    Ok(OwnedReturn::Refusal(super::refusal(pointer).kind))
}

/// 🔒️ Emits the exact original kind in the maximal-extent reserved scalar representation.
pub fn encode_refusal(kind:ValueRefusalKind)->u64{(u64::from(u32::MAX)<<32)|u64::from(super::refusal_code(kind))}
