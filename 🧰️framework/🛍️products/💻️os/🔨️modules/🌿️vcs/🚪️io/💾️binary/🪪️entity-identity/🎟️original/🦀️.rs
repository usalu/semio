//! 🆔️ Native history identity cursors hash actual names and ID lists in bounded borrowed fragments.
use super::*;
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress};
use std::mem::size_of;
/// 📜️ The canonical preimage retains the exact existing identity taxonomy and separator order.
pub enum OriginalHistoryIdentityInput<'a>{Change{edit_ids:&'a[String],description:Option<&'a str>},Alternative{name:&'a str,checkpoint_id:&'a str}}
#[derive(Clone,Copy)]
enum Ids<'a>{Many(&'a[String]),One(&'a str)}
impl<'a> Ids<'a>{fn len(self)->usize{match self{Self::Many(ids)=>ids.len(),Self::One(_)=>1}}fn get(self,index:usize)->Option<&'a str>{match self{Self::Many(ids)=>ids.get(index).map(String::as_str),Self::One(id)=>(index==0).then_some(id)}}fn identity(self)->(usize,usize){match self{Self::Many(ids)=>(ids.as_ptr()as usize,ids.len()),Self::One(id)=>(id.as_ptr()as usize,id.len())}}}
impl OriginalHistoryIdentityInput<'_>{fn parts(&self)->(&'static str,Option<&str>,Ids<'_>,Option<&str>){match self{Self::Change{edit_ids,description}=>("change",None,Ids::Many(edit_ids),*description),Self::Alternative{name,checkpoint_id}=>("alternative",Some(name),Ids::One(checkpoint_id),None)}}fn identity(&self)->[usize;7]{let(prefix,name,ids,description)=self.parts();[prefix.len(),name.map_or(0,|s|s.as_ptr()as usize),name.map_or(0,str::len),ids.identity().0,ids.identity().1,description.map_or(0,|s|s.as_ptr()as usize),description.map_or(0,str::len)]}}
/// 🧮️ A single fixed hash state retains no source clone or variable output until identity admission.
pub struct OriginalHistoryIdentity{hash:Hasher,source:Option<(u64,[usize;7])>,phase:u8,index:usize,offset:usize,digest:Option<[u8;32]>}
impl Default for OriginalHistoryIdentity{fn default()->Self{Self{hash:Hasher::new(),source:None,phase:0,index:0,offset:0,digest:None}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"history identity changed its original source")}
impl OriginalHistoryIdentity{
 /// 📏️ Every hash event updates one fixed owner and reads at most 64 original octets.
 pub fn demands(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Hasher>()+128,depth:1,..Default::default()}}
 /// 🔏️ The caller lends its original fields and actual generation for exactly one bounded hash action.
 pub fn advance(&mut self,generation:u64,input:&OriginalHistoryIdentityInput<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let source=(generation,input.identity());if self.source.is_some_and(|original|original!=source){return Err(invalid())}if self.digest.is_some(){return Ok(Default::default())}let d=self.demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(Default::default())}self.source=Some(source);let(prefix,name,ids,description)=input.parts();match self.phase{
  0=>{self.hash.update(prefix.as_bytes());self.hash.update(&[0]);self.phase=if name.is_some(){1}else{3};},
  1=>{let bytes=name.ok_or_else(invalid)?.as_bytes();let end=(self.offset+64).min(bytes.len());self.hash.update(&bytes[self.offset..end]);self.offset=end;if end==bytes.len(){self.offset=0;self.phase=2;}},
  2=>{self.hash.update(&[0]);self.phase=3;},
  3 if self.index==ids.len()=>self.phase=if name.is_some(){7}else{5},
  3=>{let bytes=ids.get(self.index).ok_or_else(invalid)?.as_bytes();let end=(self.offset+64).min(bytes.len());self.hash.update(&bytes[self.offset..end]);self.offset=end;if end==bytes.len(){self.offset=0;self.index+=1;self.phase=4;}},
  4=>{if self.index<ids.len(){self.hash.update(&[0]);}self.phase=3;},
  5=>{self.hash.update(&[0]);self.phase=6;},
  6=>{let bytes=description.unwrap_or("").as_bytes();let end=(self.offset+64).min(bytes.len());self.hash.update(&bytes[self.offset..end]);self.offset=end;if end==bytes.len(){self.phase=7;}},
  7=>self.digest=Some(*self.hash.finalize().as_bytes()),
  _=>return Err(invalid()),
 }Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})}
 /// 🏁️ The complete fixed commitment can be printed only through the caller's identity authority.
 pub fn digest(&self)->Option<[u8;32]>{self.digest}
 /// 🛑️ Paid cancellation clears only the fixed source coordinates and result commitment.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedCloneProgress{let d=RetirementDemand{copy_bytes:128,depth:1,..Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Default::default()}self.source=None;self.digest=None;RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.digest.is_none()}
}
/// 🔑️ The original identity control admits only the exact final identifier after bounded hashing completes.
pub fn print_identity_digest(prefix:&str,digest:&[u8;32],control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{let length=add(prefix.len(),17)?;control.begin_stage(0)?;let mut output=control.allocate_vec(length)?;control.begin_stage(length)?;control.advance(length)?;output.extend_from_slice(prefix.as_bytes());output.push(b'-');const HEX:&[u8;16]=b"0123456789abcdef";for byte in &digest[..8]{output.push(HEX[(byte>>4)as usize]);output.push(HEX[(byte&15)as usize]);}String::from_utf8(output).map_err(|_|invalid())}

#[cfg(test)]
#[path="tests.rs"]
mod tests;
