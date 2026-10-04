//! 🔑️ Typed media identity preserves exact scalar words, octets and literal container order.
use semio_framework_value::{DslValue,Number};
use semio_framework_hash::Sha256;
enum Part<'a>{Value(&'a DslValue),Name(&'a str)}
fn bytes(hash:&mut Sha256,value:&[u8]){hash.update(&(value.len()as u64).to_le_bytes());hash.update(value);}
/// 🔑️ Hash the actual borrowed intrinsic owner directly, without a JSON projection.
pub(super) fn fingerprint(schema:&str,value:&DslValue)->String{
 let mut hash=Sha256::new();hash.update(b"semio.media.intrinsic/v1\0");bytes(&mut hash,schema.as_bytes());let mut pending=vec![Part::Value(value)];
 while let Some(part)=pending.pop(){match part{
  Part::Name(name)=>bytes(&mut hash,name.as_bytes()),
  Part::Value(value)=>match value{
   DslValue::Null=>hash.update(&[0]),
   DslValue::Bool(value)=>hash.update(&[1,u8::from(*value)]),
   DslValue::Number(Number::UInt(value))=>{hash.update(&[2]);hash.update(&value.to_le_bytes());},
   DslValue::Number(Number::Int(value))=>{hash.update(&[3]);hash.update(&value.to_le_bytes());},
   DslValue::Number(Number::Float(value))=>{hash.update(&[4]);hash.update(&value.to_bits().to_le_bytes());},
   DslValue::String(value)=>{hash.update(&[5]);bytes(&mut hash,value.as_bytes());},
   DslValue::Bytes(value)=>{hash.update(&[6]);bytes(&mut hash,value);},
   DslValue::Array(items)=>{hash.update(&[7]);hash.update(&(items.len()as u64).to_le_bytes());for value in items.iter().rev(){pending.push(Part::Value(value));}},
   DslValue::Object(members)=>{hash.update(&[8]);hash.update(&(members.len()as u64).to_le_bytes());for(name,value)in members.iter().rev(){pending.push(Part::Value(value));pending.push(Part::Name(name));}}
  }
 }}
 semio_framework_hash::hex_lower(&hash.finalize())
}
