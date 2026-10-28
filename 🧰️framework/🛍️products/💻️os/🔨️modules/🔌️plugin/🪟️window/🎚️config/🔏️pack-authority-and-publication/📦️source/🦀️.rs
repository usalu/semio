//! 📦️ Native Window input borrows exact pair slices from either original split owners or one framed owner.
use semio_framework_value::{ValueError,ValueRefusalKind};
/// 📦️ Original source owners remain intact until their native readers finish or cancel.
#[derive(semio_framework_value::RetireOwned)]
pub enum WindowConfigPackSource{Pair(semio_framework_os_store_text_io::ArtifactPackFiles),Framed(Vec<u8>)}
impl WindowConfigPackSource{
 pub fn pair(&self)->Result<(&[u8],&[u8]),ValueError>{
  match self{Self::Pair(files)=>Ok((&files.pack,&files.spr)),Self::Framed(bytes)=>{
   let mut length=0u64;let mut prefix=None;
   for index in 0..10{let byte=*bytes.get(index).ok_or_else(||invalid("Window source frame length is truncated"))?;if index==9&&byte>1{return Err(invalid("Window source frame length overflows u64"))}length|=u64::from(byte&127)<<(index*7);if byte&128==0{if index!=0&&byte==0{return Err(invalid("Window source frame length is not canonical"))}prefix=Some(index+1);break}}
   let start=prefix.ok_or_else(||invalid("Window source frame length is unterminated"))?;let length=usize::try_from(length).map_err(|_|invalid("Window source frame length exceeds this platform"))?;let end=start.checked_add(length).filter(|end|*end<=bytes.len()).ok_or_else(||invalid("Window source frame Pack is truncated"))?;Ok((&bytes[start..end],&bytes[end..]))
  }}
 }
 pub fn take_pair_pack(&mut self)->Option<Vec<u8>>{match self{Self::Pair(files)=>Some(std::mem::take(&mut files.pack)),Self::Framed(_)=>None}}
}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}

#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn original_framed_views_follow_neutral_vectors_without_copying(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🔣️.json")).unwrap();
  for vector in fixture["valid"].as_array().unwrap(){
   let mut bytes:Vec<u8>=vector["prefix"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect();let start=bytes.len();let length=vector["packLength"].as_u64().unwrap()as usize;bytes.resize(start+length,42);bytes.extend(vector["spr"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8));let pointer=bytes.as_ptr();let source=WindowConfigPackSource::Framed(bytes);let(pack,spr)=source.pair().unwrap();assert_eq!(pack.len(),length);assert_eq!(pack.as_ptr(),pointer.wrapping_add(start));assert_eq!(spr.as_ptr(),pointer.wrapping_add(start+length));assert!(pack.iter().all(|b|*b==42));
  }
  for vector in fixture["invalid"].as_array().unwrap(){let bytes=vector.as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect();assert!(WindowConfigPackSource::Framed(bytes).pair().is_err(),"{vector}");}
 }
}
