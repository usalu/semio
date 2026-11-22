//! ⚖️ An original typed projection proves canonical replacement bytes through its installed paged encoder.
use crate::{ArtifactOriginalOperationBytes,ArtifactOperationWire};
use crate::completed;
use semio_framework_pack_json::{retained_alias,retained_alias_demand};
use semio_framework_value::{RetainedAllocationEffect,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneRef,RetainedOwnedProjection,ValueError,ValueRefusalKind,retained_clone::retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
#[path="pack-value.rs"]
mod pack_value;
use pack_value::PackValueComparison;
/// 🧳️ The real projection and original wire remain retained across every encoder, comparison and cancellation frontier.
pub struct OriginalOperationWireComparison<M:Sync>{alias:ManuallyDrop<Option<RetainedOwnedProjection<M>>>,encoder:ManuallyDrop<Option<ArtifactOriginalOperationBytes<M>>>,pack:Option<Box<PackValueComparison>>,pack_value:bool,source:Option<(u64,usize,usize,usize)>,offset:usize,phase:u8,closing:bool}
impl<M:Sync> Default for OriginalOperationWireComparison<M>{fn default()->Self{Self{alias:ManuallyDrop::new(None),encoder:ManuallyDrop::new(None),pack:None,pack_value:false,source:None,offset:0,phase:0,closing:false}}}
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,text)}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
impl<M:Sync> OriginalOperationWireComparison<M>{
 fn validate(&self,input:RetainedCloneRef<'_,M>,wire:&[u8])->Result<(u64,usize,usize,usize),ValueError>{let source=(input.source_identity(),input.get()as*const M as usize,wire.as_ptr()as usize,wire.len());if self.source.is_some_and(|original|original!=source){return Err(invalid("canonical operation comparison changed its original typed input or wire"))}Ok(source)}
 /// 📏️ Quotes one genuine alias, original encoder action, bounded comparison or exact source return.
 pub fn demands(&self,input:RetainedCloneRef<'_,M>,wire:&[u8],body:usize)->Result<RetirementDemand,ValueError>{
  self.validate(input,wire)?;if self.closing{return self.close_demands(body)}
  let d=match self.phase{
   0=>retained_alias_demand(input)?,
   1 if self.pack_value=>PackValueComparison::birth_demand(),
   1=>ArtifactOriginalOperationBytes::<M>::birth_demands(),
   2 if self.pack_value=>self.pack.as_ref().unwrap().demands()?,
   2=>self.encoder.as_ref().unwrap().demands(body)?,
   3=>RetirementDemand{copy_bytes:64+2*size_of::<usize>(),depth:1,..Default::default()},
   4 if self.pack_value=>self.pack.as_ref().unwrap().close_demands()?,
   4=>self.encoder.as_ref().unwrap().close_demands(body)?,
   5 if self.pack_value=>self.close_demands(body)?,
   5=>header::<Option<ArtifactOriginalOperationBytes<M>>>(),
   _=>return Ok(Default::default())
  };
  Ok(RetirementDemand{copy_bytes:d.copy_bytes+size_of::<u8>()+usize::from(self.phase==0)*size_of::<bool>(),depth:d.depth.max(1),..d})
 }
 /// 🪡️ Re-encodes the same original projection and compares at most 64 canonical octets per funded action.
 pub fn advance(&mut self,input:RetainedCloneRef<'_,M>,wire:&[u8],codec:ArtifactOperationWire<M>,maximum_capacity:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{let grant=effect.grant();let source=self.validate(input,wire)?;if self.closing{return Err(invalid("closing canonical operation comparison cannot resume"))}if self.prepared(){return Ok(Default::default())}let d=self.demands(input,wire,grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(Default::default())}let pack_value=codec.original_pack_value(input.get()).is_some();if self.phase!=0&&self.pack_value!=pack_value{return Err(invalid("replacement comparison changed its registered encoder"))}self.source=Some(source);let metadata=size_of::<u8>()+usize::from(self.phase==0)*size_of::<bool>();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-metadata,..grant};let mut p=match self.phase{
   0=>{let Some((alias,p))=retained_alias(input,&mut effect.nested(child))? else{return Ok(Default::default())};*self.alias=Some(alias);self.pack_value=pack_value;self.phase=1;p},
   1 if self.pack_value=>{let demand=PackValueComparison::birth_demand();if !effect.performed(1)?{return Err(invalid("replacement Pack comparison exceeded its Work"))}self.pack=Some(Box::new(PackValueComparison::new(wire.len())));self.phase=2;receipt(demand)},
  1=>{let Some((encoder,p))=ArtifactOriginalOperationBytes::admit_paged(&mut self.alias,codec,wire.len(),maximum_capacity,child)?else{return Ok(Default::default())};*self.encoder=Some(encoder);self.phase=2;p},
   2 if self.pack_value=>{
    let original=self.alias.as_ref().ok_or_else(||invalid("replacement Pack comparison lost its original projection"))?.borrow()?;
    let body=codec.original_pack_value(original.get()).ok_or_else(||invalid("replacement Pack comparison lost its registered projection"))?;
    let (step,p)=self.pack.as_mut().unwrap().advance(body,wire,&mut effect.nested(child))?;
    if step{self.phase=4;}
    p
   },
  2=>{let encoder=self.encoder.as_mut().unwrap();let step=encoder.advance(&mut effect.nested(child))?;if encoder.payload_ready(){self.phase=3;}step.progress()},
  3=>{let encoder=self.encoder.as_ref().unwrap();if encoder.payload_len()?!=wire.len(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"replacement input is not the admitted canonical operation wire"))}let mut bytes=[0;64];let count=encoder.read_payload(self.offset,&mut bytes)?;if bytes[..count]!=wire[self.offset..self.offset+count]{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"replacement input differs from its admitted canonical operation wire"))}self.offset+=count;if self.offset==wire.len(){self.phase=4;}RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes-size_of::<u8>(),..Default::default()}},
   4 if self.pack_value=>{let (closed,p)=self.close_pack(&mut effect.nested(child))?;if closed{self.phase=5;}p},
   4=>{let encoder=self.encoder.as_mut().unwrap();encoder.begin_close();let step=encoder.close_step(&mut effect.nested(child))?;if encoder.terminal_is_empty(){self.phase=5;}step.progress()},
   5 if self.pack_value=>{let p=self.close_alias(&mut effect.nested(child))?;if self.alias.is_none(){self.phase=6;}p},
  5=>{self.encoder.take();self.phase=6;RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes-size_of::<u8>(),..Default::default()}},
  _=>return Err(invalid("canonical operation comparison phase changed")),
  };p.copied_bytes+=metadata;Ok(p)}
 /// 🏁️ Equal canonical octets imply every encoding alias and output page has already returned.
 pub fn prepared(&self)->bool{self.phase==6&&!self.closing&&self.alias.is_none()&&self.encoder.is_none()&&self.pack.is_none()}
 /// 🛑️ Cancellation stops encoding before the original typed receiver can release its source.
 pub fn begin_close(&mut self){self.closing=true;if let Some(encoder)=self.encoder.as_mut(){encoder.begin_close();}}
 /// 📏️ Quotes the same original encoder, unconsumed alias or final source-coordinate header.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(pack)=self.pack.as_ref(){return pack.close_demands()}if let Some(encoder)=self.encoder.as_ref(){return if encoder.terminal_is_empty(){Ok(header::<Option<ArtifactOriginalOperationBytes<M>>>())}else{encoder.close_demands(body)}}if let Some(alias)=self.alias.as_ref(){return if alias.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{alias.close_demand(body)}}Ok(header::<Option<(u64,usize,usize,usize)>>())}
 /// ♻️ Closes actual output pages and projection leases before the original source binding is forgotten.
 pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{let grant=effect.grant();self.begin_close();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if self.pack.is_some(){return self.close_pack(effect).map(|(_,p)|RetainedCloneStep::Progress(p))}if let Some(encoder)=self.encoder.as_mut(){if encoder.terminal_is_empty(){self.encoder.take();return Ok(RetainedCloneStep::Progress(receipt(d)))}return encoder.close_step(effect).map(|step|RetainedCloneStep::Progress(step.progress()))}if self.alias.is_some(){return self.close_alias(effect).map(RetainedCloneStep::Progress)}self.source=None;Ok(RetainedCloneStep::Progress(receipt(d)))}
 fn close_pack(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<(bool,RetainedCloneProgress),ValueError>{
  let owner=self.pack.as_mut().unwrap();
  if owner.terminal_is_empty(){
   let demand=owner.close_demands()?;
   if !effect.performed(1)?{return Err(invalid("replacement Pack comparison close exceeded its Work"))}
   self.pack=None;
   return Ok((true,receipt(demand)))
  }
  Ok((false,owner.close(effect)?))
 }
 fn close_alias(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{
  let alias=self.alias.as_mut().unwrap();
  if alias.terminal_is_empty(){self.alias.take();return Ok(receipt(header::<Option<RetainedOwnedProjection<M>>>()))}
  completed(alias.close_step(effect.grant().maximum_copy_bytes,effect)).map(|step|step.progress())
 }
 /// 🫙️ Both the same original projection and all born encoded bytes must return before destruction.
 pub fn terminal_is_empty(&self)->bool{self.alias.is_none()&&self.encoder.is_none()&&self.pack.is_none()&&self.source.is_none()}
}
impl<M:Sync> Drop for OriginalOperationWireComparison<M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"canonical operation comparison retains original typed or encoded owners");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
