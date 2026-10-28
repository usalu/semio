//! 🪪️ Original pending checkpoint commitments borrow the actual ledger and pending edit IDs.
use super::*;
use semio_framework_hash::Hasher;
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
use crate::ArtifactHistoryKey;
use std::mem::size_of;
/// 🚩️ A pending checkpoint hashes its original authored message and authors before any graph publication.
pub struct OriginalPendingCheckpointInput<'a>{pub generation:u64,pub parent:Option<&'a str>,pub prior_change_ids:&'a[String],pub changes:&'a ArtifactHistoryLedger<Change>,pub pending_id:&'a str,pub pending_edits:&'a[String],pub pending_description:Option<&'a str>,pub saved_at:&'a str,pub timestamp:&'a str,pub message:Option<&'a str>,pub authors:&'a[Author]}
impl OriginalPendingCheckpointInput<'_>{fn identity(&self)->(u64,[usize;22]){(self.generation,[self.parent.map_or(0,|s|s.as_ptr()as usize),self.parent.map_or(0,str::len),self.prior_change_ids.as_ptr()as usize,self.prior_change_ids.len(),self.changes as*const _ as usize,self.changes.len(),self.pending_id.as_ptr()as usize,self.pending_id.len(),self.pending_edits.as_ptr()as usize,self.pending_edits.len(),self.saved_at.as_ptr()as usize,self.saved_at.len(),self.timestamp.as_ptr()as usize,self.timestamp.len(),self.changes.last_key().map_or(0,|key|key.index as usize),self.changes.last_key().map_or(0,|key|key.generation as usize),self.message.map_or(0,|value|value.as_ptr()as usize),self.message.map_or(0,str::len),self.authors.as_ptr()as usize,self.authors.len(),self.pending_description.map_or(0,|value|value.as_ptr()as usize),self.pending_description.map_or(0,str::len)])}}
#[derive(Default)]
struct ChangeJson{phase:u8,offset:usize,item:usize,quote:u8,escape:usize}
impl ChangeJson{
 fn literal(&mut self,text:&[u8])->Option<u8>{if let Some(byte)=text.get(self.offset){self.offset+=1;Some(*byte)}else{self.offset=0;self.phase+=1;None}}
 fn quoted(&mut self,text:&str)->Option<u8>{match self.quote{0=>{self.quote=1;Some(b'"')},1=>{if let Some(byte)=text.as_bytes().get(self.offset){if let Some(next)=semio_framework_pack_json::canonical_escaped_byte(*byte,self.escape){self.escape+=1;return Some(next)}self.escape=0;self.offset+=1;return self.quoted(text)}self.quote=2;Some(b'"')},_=>{self.quote=0;self.offset=0;None}}}
 fn next(&mut self,id:&str,edits:&[String],description:Option<&str>,saved:&str)->Option<u8>{loop{let byte=match self.phase{0=>self.literal(b"{\"id\":"),1=>{let byte=self.quoted(id);if byte.is_none(){self.phase=2}byte},2=>self.literal(b",\"editIds\":["),3=>{if self.item==edits.len(){self.phase=5;None}else{let byte=self.quoted(&edits[self.item]);if byte.is_none(){self.item+=1;self.phase=4}byte}},4=>{self.phase=3;(self.item<edits.len()).then_some(b',')},5=>self.literal(b"]"),6 if description.is_none()=>{self.phase=9;None},6=>self.literal(b",\"description\":"),7=>{if let Some(text)=description{let byte=self.quoted(text);if byte.is_none(){self.phase=9}byte}else{self.literal(b"null")}},8=>{self.phase=9;None},9=>self.literal(b",\"savedAt\":"),10=>{let byte=self.quoted(saved);if byte.is_none(){self.phase=11}byte},11=>self.literal(b"}"),_=>return None};if byte.is_some(){return byte}}}
}
/// 🧮️ Two fixed hash states retain no copied source or variable sized mirror.
pub struct OriginalPendingCheckpointIdentity{outer:Hasher,change:Hasher,json:ChangeJson,source:Option<(u64,[usize;22])>,phase:u8,index:usize,offset:usize,lookup_offset:usize,search:Option<ArtifactHistoryKey>,found:Option<ArtifactHistoryKey>,change_digest:[u8;32],digest:Option<[u8;32]>}
impl Default for OriginalPendingCheckpointIdentity{fn default()->Self{Self{outer:Hasher::new(),change:Hasher::new(),json:Default::default(),source:None,phase:0,index:0,offset:0,lookup_offset:0,search:None,found:None,change_digest:[0;32],digest:None}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"branch checkpoint commitment changed its original source")}
impl OriginalPendingCheckpointIdentity{
 /// 📏️ One fixed hash action and at most 64 original bytes fit an independently denied grant.
 pub fn demands(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Hasher>()+128,depth:1,..Default::default()}}
 /// 🔏️ Reborrows actual ledger rows one at a time and hashes canonical JSON without gathering it.
 pub fn advance(&mut self,input:&OriginalPendingCheckpointInput<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let source=input.identity();if self.source.is_some_and(|original|original!=source){return Err(invalid())}if self.digest.is_some(){return Ok(Default::default())}let d=self.demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(Default::default())}self.source=Some(source);
  match self.phase{
   0=>{let text=input.parent.unwrap_or("").as_bytes();let end=(self.offset+64).min(text.len());self.outer.update(&text[self.offset..end]);self.offset=end;if end==text.len(){self.outer.update(&[0]);self.offset=0;self.phase=1;}},
   1 if self.index==input.prior_change_ids.len()+1=>{self.outer.update(&[0]);self.offset=0;self.index=0;self.phase=10;},
   1=>{self.search=input.changes.last_key();self.found=None;self.lookup_offset=0;self.phase=2;},
   2=>{let wanted=if self.index==input.prior_change_ids.len(){input.pending_id}else{input.prior_change_ids[self.index].as_str()};if let Some(key)=self.search{let row=input.changes.get_key(key).ok_or_else(invalid)?;let end=(self.lookup_offset+64).min(wanted.len());if row.id.len()!=wanted.len()||row.id.as_bytes()[self.lookup_offset..end]!=wanted.as_bytes()[self.lookup_offset..end]{self.search=input.changes.previous_key(key);self.lookup_offset=0;}else if end==wanted.len(){self.found=Some(key);self.lookup_offset=0;self.phase=3;}else{self.lookup_offset=end;}}else{self.lookup_offset=0;self.phase=9;}},
   9=>{let wanted=if self.index==input.prior_change_ids.len(){input.pending_id}else{input.prior_change_ids[self.index].as_str()};let end=(self.lookup_offset+64).min(wanted.len());if wanted.len()!=input.pending_id.len()||wanted.as_bytes()[self.lookup_offset..end]!=input.pending_id.as_bytes()[self.lookup_offset..end]{self.outer.update(&[0;32]);self.index+=1;self.lookup_offset=0;self.phase=1;}else if end==wanted.len(){self.lookup_offset=0;self.phase=3;}else{self.lookup_offset=end;}},
   3=>{let(id,edits,description,saved)=if let Some(key)=self.found{let row=input.changes.get_key(key).ok_or_else(invalid)?;(row.id.as_str(),row.edit_ids.as_slice(),row.description.as_deref(),row.saved_at.as_str())}else{(input.pending_id,input.pending_edits,input.pending_description,input.saved_at)};let mut bytes=[0;64];let mut count=0;while count<bytes.len(){let Some(byte)=self.json.next(id,edits,description,saved)else{self.phase=4;break};bytes[count]=byte;count+=1;}self.change.update(&bytes[..count]);},
   4=>{self.change_digest=*self.change.finalize().as_bytes();self.change=Hasher::new();self.phase=8;},
   8=>{self.outer.update(&self.change_digest);self.json=Default::default();self.index+=1;self.phase=1;},
   10=>{let text=input.message.unwrap_or("").as_bytes();let end=(self.offset+64).min(text.len());self.outer.update(&text[self.offset..end]);self.offset=end;if end==text.len(){self.outer.update(&[0]);self.offset=0;self.phase=11;}},
   11 if self.index==input.authors.len()=>{self.outer.update(&[0]);self.phase=5;},
   11=>{let text=input.authors[self.index].id.as_bytes();let end=(self.offset+64).min(text.len());self.outer.update(&text[self.offset..end]);self.offset=end;if end==text.len(){self.outer.update(&[0]);self.offset=0;self.index+=1;}},
   5=>{let text=input.timestamp.as_bytes();let end=(self.offset+64).min(text.len());self.outer.update(&text[self.offset..end]);self.offset=end;if end==text.len(){self.digest=Some(*self.outer.finalize().as_bytes());}},
   _=>return Err(invalid()),
  }Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})
 }
 /// 🏁️ The content commitment remains a fixed value until its original publisher consumes it.
 pub fn digest(&self)->Option<[u8;32]>{self.digest}
 /// 🛑️ Cancellation forgets borrowed coordinates only after a funded fixed state reset.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedCloneProgress{let d=RetirementDemand{copy_bytes:size_of::<Option<(u64,[usize;22])>>()+size_of::<Option<[u8;32]>>(),depth:1,..Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Default::default()}self.source=None;self.digest=None;RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
 /// 🫙️ No original source coordinates or output commitment remain retained.
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.digest.is_none()}
}

#[cfg(test)]
#[path="tests.rs"]
mod tests;
#[cfg(test)]
#[path="commit-identity-tests.rs"]
mod commit_identity_tests;

#[path="pending.rs"]
pub mod pending;

#[path="facts.rs"]
pub mod facts;
#[path="commit-facts.rs"]
pub mod commit_facts;

#[path="existing.rs"]
pub mod existing;

#[path="pin-order.rs"]
pub mod pin_order;

#[path="pin-fields.rs"]
pub mod pin_fields;

#[path="repin-facts.rs"]
pub mod repin_facts;

#[path="repin-alternative.rs"]
pub mod repin_alternative;
