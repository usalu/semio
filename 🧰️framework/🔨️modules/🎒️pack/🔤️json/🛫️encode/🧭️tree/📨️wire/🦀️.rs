//! 📨️ A canonical wire owns one original typed source through paid measurement, direct encoding and complete retirement.
use super::{ArtifactCanonicalJsonTree as Tree,ArtifactCanonicalJsonTreeCursor};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedOwnedProjection};
use std::mem::ManuallyDrop;
fn header()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
fn progress()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn refuses(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
/// 🧾️ Physical receipt accompanies every optional output, including denied or metadata-only turns.
pub struct ArtifactCanonicalJsonTreeWireStep{pub output:Option<Vec<u8>>,pub progress:RetainedCloneProgress}
/// 🧳️ The same typed root is measured and written without a value projection or payload clone.
pub struct ArtifactCanonicalJsonTreeWireCursor<T:Tree+RetireOwned>{original:ManuallyDrop<Option<T>>,original_close:ManuallyDrop<Option<ControlledRetirement<T>>>,#[doc(hidden)] pub source:ManuallyDrop<Option<RetainedCloneSource<T>>>,projection:ManuallyDrop<Option<RetainedOwnedProjection<dyn Tree>>>,tree:ManuallyDrop<Option<ArtifactCanonicalJsonTreeCursor>>,#[doc(hidden)] pub wire:ManuallyDrop<Vec<u8>>,scratch:[u8;64],#[doc(hidden)] pub length:usize,written:usize,#[doc(hidden)] pub phase:u8,#[doc(hidden)] pub closing:bool}
impl<T:Tree+RetireOwned> ArtifactCanonicalJsonTreeWireCursor<T>{
 /// 📥️ Transfers only an existing source header into inline custody; no backing is allocated here.
 pub fn new(original:T)->Self{Self{original:ManuallyDrop::new(Some(original)),original_close:ManuallyDrop::new(None),source:ManuallyDrop::new(None),projection:ManuallyDrop::new(None),tree:ManuallyDrop::new(None),wire:ManuallyDrop::new(Vec::new()),scratch:[0;64],length:0,written:0,phase:0,closing:false}}
 /// 🫴️ Borrows the exact typed source while its original authority remains live.
 pub fn original(&self)->Option<&T>{self.original.as_ref().or_else(||self.source.as_ref().and_then(|source|source.try_borrow().ok().map(|loan|loan.get())))}
 pub fn output_bytes_written(&self)->usize{self.written}
 pub fn measured_byte_length(&self)->usize{self.length}
 pub fn phase(&self)->&'static str{if self.closing{"closing"}else{match self.phase{0..=3=>"measure",4=>"allocate",5=>"initialize",6..=8=>"encode",9=>"sourceClose",10=>"handoff",_=>"complete"}}}
 pub fn begin_close(&mut self){self.closing=true;if let Some(tree)=self.tree.as_mut(){tree.begin_close()}}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.original_close.is_none()&&self.source.is_none()&&self.projection.is_none()&&self.tree.is_none()&&self.wire.capacity()==0}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(tree)=self.tree.as_ref(){return if tree.terminal_is_empty(){Ok(header())}else{tree.next_demand()}}
  if let Some(projection)=self.projection.as_ref(){return if projection.terminal_is_empty(){Ok(header())}else{projection.next_close_demand(body)}}
  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(header())}else{source.next_close_demand(body)}}
  if let Some(owner)=self.original_close.as_ref(){return if owner.terminal_is_empty(){Ok(header())}else{owner.selected_demand(body)}}
  if self.original.is_some(){return Ok(header())}
  Ok(RetirementDemand{release_bytes:self.wire.capacity(),depth:usize::from(self.wire.capacity()!=0),..Default::default()})
 }
 /// 📏️ Quotes the identical next selected body, allocator, alias or full-close event.
 pub fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demands(body)}
  Ok(match self.phase{
   0=>{let birth=RetainedCloneSource::<T>::owned_constructor_demand::<()>();RetirementDemand{capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()}},
   1|2|6|7|10=>header(),
   3|8=>self.tree.as_ref().map_or(Ok(header()),|tree|if tree.terminal_is_empty(){Ok(header())}else{tree.next_demand()})?,
   4=>RetirementDemand{capacity_bytes:self.length,depth:1,..Default::default()},
   5 if self.wire.len()<self.length=>RetirementDemand{copy_bytes:usize::from(body!=0),depth:1,..Default::default()},
   5=>header(),
   9=>self.source.as_ref().map_or(Ok(header()),|source|if source.terminal_is_empty(){Ok(header())}else{source.next_close_demand(body)})?,
   _=>Default::default(),
  })
 }
 fn project(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let(projection,receipt)=self.source.as_ref().ok_or_else(||refuses("original canonical wire source is absent"))?.project_owned(0,|root|root as&dyn Tree,grant)?;*self.projection=Some(projection);self.phase+=1;Ok(receipt)}
 fn admit_tree(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let projection=self.projection.take().ok_or_else(||refuses("original canonical wire projection is absent"))?;match ArtifactCanonicalJsonTreeCursor::admit(projection,grant){Ok((tree,receipt))=>{*self.tree=Some(tree);self.phase+=1;Ok(receipt)},Err((error,projection))=>{*self.projection=Some(projection);Err(error)}}}
 fn advance_one(&mut self,grant:RetainedCloneGrant)->Result<ArtifactCanonicalJsonTreeWireStep,ValueError>{
  let none=|progress|ArtifactCanonicalJsonTreeWireStep{output:None,progress};let demand=self.next_demand(grant.maximum_copy_bytes)?;if !permits(grant,demand){return Ok(none(Default::default()))}if self.closing{return self.close_one(grant).map(none)}
  match self.phase{
   0=>{let original=self.original.take().ok_or_else(||refuses("original canonical wire root is absent"))?;match RetainedCloneSource::admit_owned(original,(),grant){Ok((source,receipt))=>{*self.source=Some(source);self.phase=1;Ok(none(receipt))},Err((error,original,_))=>{*self.original=Some(original);Err(error)}}},
   1|6=>self.project(grant).map(none),
   2|7=>self.admit_tree(grant).map(none),
   3|8=>{
    let tree=self.tree.as_mut().ok_or_else(||refuses("original canonical wire tree is absent"))?;if tree.terminal_is_empty(){assert!(tree.is_complete());self.tree.take();if self.phase==8&&self.written!=self.length{return Err(refuses("original canonical wire source changed its measured length"))}self.phase+=1;return Ok(none(progress()))}
    let step=if self.phase==3{tree.advance(&mut self.scratch,grant)?}else{let end=self.written.saturating_add(64).min(self.length);tree.advance(&mut self.wire[self.written..end],grant)?};let receipt=step.ownership.progress();if self.phase==3{self.length=self.length.checked_add(step.written_bytes).ok_or_else(||refuses("original canonical wire measured length overflow").with_retained_progress(receipt))?}else{self.written=self.written.checked_add(step.written_bytes).filter(|length|*length<=self.length).ok_or_else(||refuses("original canonical wire source exceeded its measured output").with_retained_progress(receipt))?}Ok(none(receipt))
   },
   4=>{self.wire.try_reserve_exact(self.length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original canonical wire destination allocation refused"))?;self.phase=5;Ok(none(RetainedCloneProgress{retained_capacity_bytes:self.wire.capacity(),..progress()}))},
   5=>{let count=(self.length-self.wire.len()).min(grant.maximum_copy_bytes).min(64);if self.wire.len()==self.length{self.phase=6;return Ok(none(progress()))}if count==0{return Ok(none(Default::default()))}let length=self.wire.len()+count;self.wire.resize(length,0);Ok(none(RetainedCloneProgress{copied_bytes:count,..progress()}))},
   9=>{let source=self.source.as_mut().ok_or_else(||refuses("original canonical wire source close is absent"))?;if source.terminal_is_empty(){self.source.take();self.phase=10;return Ok(none(progress()))}source.close_step(grant).map(|step|none(step.progress()))},
   10=>{self.phase=11;Ok(ArtifactCanonicalJsonTreeWireStep{output:Some(std::mem::take(&mut*self.wire)),progress:progress()})},
   _=>Ok(none(Default::default())),
  }
 }
 /// ✍️ Performs one bounded event; any refusal leaves every remaining original owner available for paid close.
 pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<ArtifactCanonicalJsonTreeWireStep,ValueError>{let result=self.advance_one(grant);if result.is_err(){self.begin_close()}result}
 fn close_one(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if let Some(tree)=self.tree.as_mut(){if tree.terminal_is_empty(){self.tree.take();return Ok(progress())}return tree.close_step(grant).map(|step|step.progress())}
  if let Some(projection)=self.projection.as_mut(){if projection.terminal_is_empty(){self.projection.take();return Ok(progress())}return projection.close_step(grant).map(|step|step.progress())}
  if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source.take();return Ok(progress())}return source.close_step(grant).map(|step|step.progress())}
  if let Some(owner)=self.original_close.as_mut(){if owner.terminal_is_empty(){self.original_close.take();return Ok(progress())}return owner.step(grant).map(|step|step.progress())}
  if self.original.is_some(){let original=self.original.take().unwrap();match ControlledRetirement::new(original){Ok(owner)=>{*self.original_close=Some(owner);return Ok(progress())},Err((error,original))=>{*self.original=Some(original);return Err(error)}}}
  if self.wire.capacity()!=0{let bytes=self.wire.capacity();drop(std::mem::take(&mut*self.wire));return Ok(RetainedCloneProgress{released_bytes:bytes,..progress()})}Ok(Default::default())
 }
 /// ♻️ Closes the exact aliases, typed body and partial wire backing under the supplied actual grant.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{self.begin_close();let demand=self.close_demands(grant.maximum_copy_bytes)?;if !permits(grant,demand){return Ok(Default::default())}self.close_one(grant)}
}
impl<T:Tree+RetireOwned> Drop for ArtifactCanonicalJsonTreeWireCursor<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original canonical wire dropped before complete paid source and partial output closure")}}

#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod testkit {
    use super::*;
    pub fn refuses(reason:&'static str) ->ValueError {
        super::refuses(reason)
    }
}
