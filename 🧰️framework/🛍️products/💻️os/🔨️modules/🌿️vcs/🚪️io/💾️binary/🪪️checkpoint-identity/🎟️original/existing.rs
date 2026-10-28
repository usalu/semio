//! 🪪️ Existing checkpoint commitments hash original canonical rows and ordered composition pins.
use super::*;
use crate::Checkpoint;
use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceUriView;
/// 📚️ The enclosing original publisher retains the actual checkpoint, ledger and sorted pin owner.
pub struct OriginalCheckpointIdentityInput<'a>{pub generation:u64,pub checkpoint:&'a Checkpoint,pub changes:&'a ArtifactHistoryLedger<Change>,pub pins:&'a[CompositionPin]}
impl OriginalCheckpointIdentityInput<'_>{fn identity(&self)->(u64,[usize;6]){(self.generation,[self.checkpoint as*const _ as usize,self.changes as*const _ as usize,self.changes.len(),self.pins.as_ptr()as usize,self.pins.len(),self.checkpoint.change_ids.len()])}}
/// 🔏️ Two fixed hash states borrow original rows without gathering canonical JSON or URI strings.
pub struct OriginalCheckpointIdentity{outer:Hasher,change:Hasher,json:ChangeJson,source:Option<(u64,[usize;6])>,phase:u8,index:usize,offset:usize,lookup:usize,search:Option<ArtifactHistoryKey>,found:Option<ArtifactHistoryKey>,change_digest:[u8;32],digest:Option<[u8;32]>}
impl Default for OriginalCheckpointIdentity{fn default()->Self{Self{outer:Hasher::new(),change:Hasher::new(),json:Default::default(),source:None,phase:0,index:0,offset:0,lookup:0,search:None,found:None,change_digest:[0;32],digest:None}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"existing checkpoint commitment changed its original source")}
impl OriginalCheckpointIdentity{
 /// 📏️ Each event hashes at most 64 original bytes and fixed metadata without allocating.
 pub fn demands(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Hasher>()+256,depth:1,..Default::default()}}
 /// 🧮️ Canonical ordered pins are checked before any commitment is exposed.
 pub fn advance(&mut self,input:&OriginalCheckpointIdentityInput<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let source=input.identity();if self.source.is_some_and(|original|original!=source){return Err(invalid())}if self.digest.is_some(){return Ok(Default::default())}let d=self.demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(Default::default())}self.source=Some(source);let checkpoint=input.checkpoint;
  match self.phase{
   0 if self.index+1<input.pins.len()=>{let left=ArtifactReferenceUriView::new(&input.pins[self.index].child_ref);let right=ArtifactReferenceUriView::new(&input.pins[self.index+1].child_ref);let(mut a,mut b)=([0;64],[0;64]);let na=left.read_bytes(self.offset,&mut a);let nb=right.read_bytes(self.offset,&mut b);match a[..na].cmp(&b[..nb]){std::cmp::Ordering::Greater|std::cmp::Ordering::Equal if na==0=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"checkpoint pins require unique canonical URI order").with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),std::cmp::Ordering::Less=>{self.index+=1;self.offset=0;},std::cmp::Ordering::Greater=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"checkpoint pins require canonical URI order").with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),_=>{self.offset+=na;}}},
   0=>{self.index=0;self.offset=0;self.phase=1;},
   1=>{let text=checkpoint.parent_id.as_deref().unwrap_or("");self.hash_text(text);if self.offset==text.len(){self.outer.update(&[0]);self.offset=0;self.phase=2;}},
   2 if self.index==checkpoint.change_ids.len()=>{self.outer.update(&[0]);self.index=0;self.phase=6;},
   2=>{self.search=input.changes.last_key();self.lookup=0;self.phase=3;},
   3=>{let wanted=&checkpoint.change_ids[self.index];if let Some(key)=self.search{let row=input.changes.get_key(key).ok_or_else(invalid)?;let end=(self.lookup+64).min(wanted.len());if row.id.len()!=wanted.len()||row.id.as_bytes()[self.lookup..end]!=wanted.as_bytes()[self.lookup..end]{self.search=input.changes.previous_key(key);self.lookup=0;}else if end==wanted.len(){self.found=Some(key);self.phase=4;}else{self.lookup=end;}}else{self.outer.update(&[0;32]);self.index+=1;self.phase=2;}},
   4=>{let row=input.changes.get_key(self.found.ok_or_else(invalid)?).ok_or_else(invalid)?;let mut bytes=[0;64];let mut n=0;while n<64{let Some(byte)=self.json.next(&row.id,&row.edit_ids,row.description.as_deref(),&row.saved_at)else{self.phase=5;break};bytes[n]=byte;n+=1;}self.change.update(&bytes[..n]);},
   5=>{self.change_digest=*self.change.finalize().as_bytes();self.change=Hasher::new();self.phase=12;},
   12=>{self.outer.update(&self.change_digest);self.json=Default::default();self.found=None;self.index+=1;self.phase=2;},
   6=>{let text=checkpoint.message.as_deref().unwrap_or("");self.hash_text(text);if self.offset==text.len(){self.outer.update(&[0]);self.offset=0;self.phase=7;}},
   7 if self.index==checkpoint.authors.len()=>{self.outer.update(&[0]);self.index=0;self.phase=8;},
   7=>{let text=&checkpoint.authors[self.index].id;self.hash_text(text);if self.offset==text.len(){self.outer.update(&[0]);self.offset=0;self.index+=1;}},
   8=>{let text=&checkpoint.timestamp;self.hash_text(text);if self.offset==text.len(){self.offset=0;if input.pins.is_empty(){self.finish();}else{self.outer.update(&[0]);self.phase=9;}}},
   9=>{let view=ArtifactReferenceUriView::new(&input.pins[self.index].child_ref);let mut bytes=[0;64];let n=view.read_bytes(self.offset,&mut bytes);self.outer.update(&bytes[..n]);self.offset+=n;if self.offset==view.byte_len().ok_or_else(invalid)?{self.outer.update(&[0]);self.offset=0;self.phase=10;}},
   10=>{let text=&input.pins[self.index].checkpoint_id;self.hash_text(text);if self.offset==text.len(){self.outer.update(&[0]);self.offset=0;self.index+=1;if self.index==input.pins.len(){self.finish();}else{self.phase=9;}}},
   _=>return Err(invalid()),
  }Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})
 }
 fn hash_text(&mut self,text:&str){let end=(self.offset+64).min(text.len());self.outer.update(&text.as_bytes()[self.offset..end]);self.offset=end;}
 fn finish(&mut self){self.digest=Some(*self.outer.finalize().as_bytes());self.phase=11;}
 /// 🏁️ The enclosing native author derives its canonical checkpoint ID from this fixed commitment.
 pub fn digest(&self)->Option<[u8;32]>{self.digest}
 /// ♻️ Only borrowed coordinates and the fixed digest require a funded cancellation reset.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedCloneProgress{let d=self.close_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Default::default()}self.source=None;self.digest=None;RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
 /// 📐️ Quotes the exact original source-coordinate reset.
 pub fn close_demands(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<(u64,[usize;6])>>()+size_of::<Option<[u8;32]>>(),depth:1,..Default::default()}}
 /// 🫙️ No borrowed source coordinates or output digest remain retained.
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.digest.is_none()}
}

#[cfg(test)]
#[path="existing-tests.rs"]
mod tests;
