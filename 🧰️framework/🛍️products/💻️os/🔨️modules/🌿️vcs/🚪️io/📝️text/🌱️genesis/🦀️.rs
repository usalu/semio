//! 🌱️ Native stored genesis Pack text framing.
use crate::os_vcs::io::binary::genesis::{AdmittedArtifactGenesis,ArtifactGenesisCodec};
use semio_framework_value::{DslValue,FromValue,ToValue,ValueError,ValueRefusalKind,NativeDecodeControl,NativeEncodeControl};
pub fn encode_genesis_hex_controlled(pack:&[u8],control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{
 let capacity=pack.len().checked_mul(2).filter(|n|*n<=isize::MAX as usize).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Genesis hex capacity overflow"))?;
 control.begin_stage(pack.len())?;control.charge(capacity)?;let mut hex=String::new();hex.try_reserve_exact(capacity).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Genesis hex allocation refused"))?;
 const DIGITS:&[u8;16]=b"0123456789abcdef";for byte in pack{hex.push(DIGITS[(byte>>4)as usize]as char);hex.push(DIGITS[(byte&15)as usize]as char);control.advance(1)?;}control.checkpoint()?;Ok(hex)
}
pub fn decode_genesis_hex_controlled(hex:&str,control:&mut NativeDecodeControl<'_>)->Result<Vec<u8>,ValueError>{
 if hex.is_empty()||hex.len()&1!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Native genesis Pack requires nonempty lowercase hex"));}
 control.begin_stage(hex.len()/2)?;control.charge(hex.len()/2)?;let mut pack=Vec::new();pack.try_reserve_exact(hex.len()/2).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Genesis Pack allocation refused"))?;
 let nibble=|b|match b{b'0'..=b'9'=>Some(b-b'0'),b'a'..=b'f'=>Some(b-b'a'+10),_=>None};
 for pair in hex.as_bytes().chunks_exact(2){let high=nibble(pair[0]).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Native genesis hex invalid"))?;let low=nibble(pair[1]).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Native genesis hex invalid"))?;pack.push(high<<4|low);control.advance(1)?;}control.checkpoint()?;Ok(pack)
}
impl<P> ToValue for AdmittedArtifactGenesis<P>{fn to_value(&self)->DslValue{DslValue::String(encode_genesis_hex_controlled(self.pack(),&mut NativeEncodeControl::new(usize::MAX,&mut |_|true)).expect("admitted native genesis hex"))}}
impl<P:ArtifactGenesisCodec> FromValue for AdmittedArtifactGenesis<P>{fn from_value(value:DslValue)->Result<Self,ValueError>{let hex=String::from_value(value)?;let pack=decode_genesis_hex_controlled(&hex,&mut NativeDecodeControl::new(usize::MAX,&mut |_|true))?;Self::from_stored_pack(pack)}}
