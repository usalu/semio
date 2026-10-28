//! ⚖️ An original typed projection proves canonical replacement bytes through its installed paged encoder.
use crate::{ArtifactOriginalOperationBytes,ArtifactOperationWire};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneRef,RetainedOwnedProjection,ValueError,ValueRefusalKind,retained_clone::retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
/// 🧳️ The real projection and original wire remain retained across every encoder, comparison and cancellation frontier.
pub struct OriginalOperationWireComparison<M:Sync>{alias:ManuallyDrop<Option<RetainedOwnedProjection<M>>>,encoder:ManuallyDrop<Option<ArtifactOriginalOperationBytes<M>>>,source:Option<(u64,usize,usize,usize)>,offset:usize,phase:u8,closing:bool}
impl<M:Sync> Default for OriginalOperationWireComparison<M>{fn default()->Self{Self{alias:ManuallyDrop::new(None),encoder:ManuallyDrop::new(None),source:None,offset:0,phase:0,closing:false}}}
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,text)}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
impl<M:Sync> OriginalOperationWireComparison<M>{
 fn validate(&self,input:RetainedCloneRef<'_,M>,wire:&[u8])->Result<(u64,usize,usize,usize),ValueError>{let source=(input.source_identity(),input.get()as*const M as usize,wire.as_ptr()as usize,wire.len());if self.source.is_some_and(|original|original!=source){return Err(invalid("canonical operation comparison changed its original typed input or wire"))}Ok(source)}
 /// 📏️ Quotes one genuine alias, original encoder action, bounded comparison or exact source return.
 pub fn demands(&self,input:RetainedCloneRef<'_,M>,wire:&[u8],body:usize)->Result<RetirementDemand,ValueError>{self.validate(input,wire)?;if self.closing{return self.close_demands(body)}let d=match self.phase{0=>RetainedOwnedProjection::<M>::constructor_demand(),1=>ArtifactOriginalOperationBytes::<M>::birth_demands(),2=>self.encoder.as_ref().unwrap().demands(body)?,3=>RetirementDemand{copy_bytes:64+2*size_of::<usize>(),depth:1,..Default::default()},4=>self.encoder.as_ref().unwrap().close_demands(body)?,5=>header::<Option<ArtifactOriginalOperationBytes<M>>>(),_=>return Ok(Default::default())};Ok(RetirementDemand{copy_bytes:d.copy_bytes+size_of::<u8>(),depth:d.depth.max(1),..d})}
 /// 🪡️ Re-encodes the same original projection and compares at most 64 canonical octets per funded action.
 pub fn advance(&mut self,input:RetainedCloneRef<'_,M>,wire:&[u8],codec:ArtifactOperationWire<M>,maximum_capacity:usize,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let source=self.validate(input,wire)?;if self.closing{return Err(invalid("closing canonical operation comparison cannot resume"))}if self.prepared(){return Ok(Default::default())}let d=self.demands(input,wire,grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(Default::default())}self.source=Some(source);let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<u8>(),..grant};let mut p=match self.phase{
  0=>{let (alias,p)=input.retain_owned(child)?;*self.alias=Some(alias);self.phase=1;p},
  1=>{let Some((encoder,p))=ArtifactOriginalOperationBytes::admit_paged(&mut self.alias,codec,wire.len(),maximum_capacity,child)?else{return Ok(Default::default())};*self.encoder=Some(encoder);self.phase=2;p},
  2=>{let encoder=self.encoder.as_mut().unwrap();let step=encoder.advance(child)?;if encoder.payload_ready(){self.phase=3;}step.progress()},
  3=>{let encoder=self.encoder.as_ref().unwrap();if encoder.payload_len()?!=wire.len(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"replacement input is not the admitted canonical operation wire"))}let mut bytes=[0;64];let count=encoder.read_payload(self.offset,&mut bytes)?;if bytes[..count]!=wire[self.offset..self.offset+count]{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"replacement input differs from its admitted canonical operation wire"))}self.offset+=count;if self.offset==wire.len(){self.phase=4;}RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes-size_of::<u8>(),..Default::default()}},
  4=>{let encoder=self.encoder.as_mut().unwrap();encoder.begin_close();let step=encoder.close_step(child)?;if encoder.terminal_is_empty(){self.phase=5;}step.progress()},
  5=>{self.encoder.take();self.phase=6;RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes-size_of::<u8>(),..Default::default()}},
  _=>return Err(invalid("canonical operation comparison phase changed")),
 };p.copied_bytes+=size_of::<u8>();Ok(p)}
 /// 🏁️ Equal canonical octets imply every encoding alias and output page has already returned.
 pub fn prepared(&self)->bool{self.phase==6&&!self.closing&&self.alias.is_none()&&self.encoder.is_none()}
 /// 🛑️ Cancellation stops encoding before the original typed receiver can release its source.
 pub fn begin_close(&mut self){self.closing=true;if let Some(encoder)=self.encoder.as_mut(){encoder.begin_close();}}
 /// 📏️ Quotes the same original encoder, unconsumed alias or final source-coordinate header.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(encoder)=self.encoder.as_ref(){return if encoder.terminal_is_empty(){Ok(header::<Option<ArtifactOriginalOperationBytes<M>>>())}else{encoder.close_demands(body)}}if let Some(alias)=self.alias.as_ref(){return if alias.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{alias.close_demand(body)}}Ok(header::<Option<(u64,usize,usize,usize)>>())}
 /// ♻️ Closes actual output pages and projection leases before the original source binding is forgotten.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.begin_close();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(encoder)=self.encoder.as_mut(){if encoder.terminal_is_empty(){self.encoder.take();return Ok(RetainedCloneStep::Progress(receipt(d)))}return encoder.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}if let Some(alias)=self.alias.as_mut(){if alias.terminal_is_empty(){self.alias.take();return Ok(RetainedCloneStep::Progress(receipt(d)))}return alias.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}self.source=None;Ok(RetainedCloneStep::Progress(receipt(d)))}
 /// 🫙️ Both the same original projection and all born encoded bytes must return before destruction.
 pub fn terminal_is_empty(&self)->bool{self.alias.is_none()&&self.encoder.is_none()&&self.source.is_none()}
}
impl<M:Sync> Drop for OriginalOperationWireComparison<M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"canonical operation comparison retains original typed or encoded owners");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
