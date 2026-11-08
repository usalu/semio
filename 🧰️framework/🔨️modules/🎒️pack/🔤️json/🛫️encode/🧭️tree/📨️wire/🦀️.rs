//! 📨️ A canonical wire owns one original typed source through paid measurement, direct encoding and complete retirement.
use super::{ArtifactCanonicalJsonTree as Tree,ArtifactCanonicalJsonTreeCursor,settled,work_u64};
type Effect<'a,'b>=&'a mut semio_framework_value::RetainedAllocationEffect<'b>;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedCloneSourceAdmission,RetainedOwnedProjection};
use std::mem::ManuallyDrop;
fn header()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
fn progress()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
/// 💰️ Performs the one paid unit of a turn that delegates to no nested owner.
fn pay(effect:Effect<'_,'_>)->Result<(),ValueError>{if effect.performed(1)?{Ok(())}else{Err(refuses("original canonical wire exceeded its work authority"))}}
fn refuses(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
pub struct ArtifactCanonicalJsonTreeWireStep{pub output:Option<Vec<u8>>,pub progress:RetainedCloneProgress}
pub struct ArtifactCanonicalJsonTreeWireCursor<T:Tree+RetireOwned>{original:ManuallyDrop<Option<T>>,original_close:ManuallyDrop<Option<ControlledRetirement<T>>>,admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<T,T>>>,#[doc(hidden)] pub source:ManuallyDrop<Option<RetainedCloneSource<T>>>,projection:ManuallyDrop<Option<RetainedOwnedProjection<dyn Tree>>>,tree:ManuallyDrop<Option<ArtifactCanonicalJsonTreeCursor>>,#[doc(hidden)] pub wire:ManuallyDrop<Vec<u8>>,scratch:[u8;64],#[doc(hidden)] pub length:usize,written:usize,#[doc(hidden)] pub phase:u8,#[doc(hidden)] pub closing:bool,last_turn_work:u64}
impl<T:Tree+RetireOwned> ArtifactCanonicalJsonTreeWireCursor<T>{
 pub fn new(original:T)->Self{Self{original:ManuallyDrop::new(Some(original)),original_close:ManuallyDrop::new(None),admission:ManuallyDrop::new(None),source:ManuallyDrop::new(None),projection:ManuallyDrop::new(None),tree:ManuallyDrop::new(None),wire:ManuallyDrop::new(Vec::new()),scratch:[0;64],length:0,written:0,phase:0,closing:false,last_turn_work:0}}
 pub fn original(&self)->Option<&T>{self.original.as_ref().or_else(||self.admission.as_ref().and_then(RetainedCloneSourceAdmission::original_authority)).or_else(||self.source.as_ref().and_then(|source|source.try_borrow().ok().map(|loan|loan.get())))}
 pub fn last_turn_work(&self)->u64{self.last_turn_work}
 pub fn output_bytes_written(&self)->usize{self.written}
 pub fn measured_byte_length(&self)->usize{self.length}
 pub fn phase(&self)->&'static str{if self.closing{"closing"}else{match self.phase{0..=3=>"measure",4=>"allocate",5=>"initialize",6..=8=>"encode",9=>"sourceClose",10=>"handoff",_=>"complete"}}}
 pub fn begin_close(&mut self){self.closing=true;if let Some(tree)=self.tree.as_mut(){tree.begin_close()}}
 /// 📫️ Quotes handback of the actual typed root after both immutable traversal aliases have closed.
 pub fn original_return_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase!=9||self.tree.is_some()||self.projection.is_some(){return Err(refuses("canonical wire original return requires its completed immutable traversals"))}
  self.source.as_ref().ok_or_else(||refuses("canonical wire original return has no source"))?.next_take_demand(body)
 }
 /// 🫴️ Returns the original typed owner without a clone while retaining the independently funded wire.
 pub fn return_original(&mut self,receiving:&mut Option<T>,effect:Effect<'_,'_>)->semio_framework_value::RetainedWorkOutcome{
  use semio_framework_value::retained_clone::RetainedCloneSourceTake;
  use semio_framework_value::{RetainedCloneStep,RetainedWorkOutcome};
  let grant=effect.grant();self.last_turn_work=0;
  if receiving.is_some(){return RetainedWorkOutcome{result:Err(refuses("canonical wire original return receiver is occupied")),work:0}}
  let demand=match self.original_return_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  if !permits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  let Some(source)=self.source.as_mut()else{return RetainedWorkOutcome{result:Err(refuses("canonical wire original return has no source")),work:0}};
  let(take,work)=match settled(source.take_authority(effect)){Ok(turn)=>turn,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  self.last_turn_work=work_u64(work).unwrap_or(u64::MAX);
  let step=match take{RetainedCloneSourceTake::Pending(progress)=>RetainedCloneStep::Progress(progress),RetainedCloneSourceTake::Ready(original,progress)=>{*receiving=Some(original);assert!(self.source.as_ref().is_some_and(RetainedCloneSource::terminal_is_empty));self.source.take();self.phase=10;RetainedCloneStep::Complete(progress)}};
  RetainedWorkOutcome{result:Ok(step),work}
 }
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.original_close.is_none()&&self.admission.is_none()&&self.source.is_none()&&self.projection.is_none()&&self.tree.is_none()&&self.wire.capacity()==0}
 /// ♻️ Quotes paid closure without changing the current encoding intent or original custody.
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demands(body)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(admission)=self.admission.as_ref(){return admission.next_return_demand()}
  if let Some(tree)=self.tree.as_ref(){return if tree.terminal_is_empty(){Ok(header())}else{tree.next_demand()}}
  if let Some(projection)=self.projection.as_ref(){return if projection.terminal_is_empty(){Ok(header())}else{projection.next_close_demand(body)}}
  if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(header())}else{source.next_close_demand(body)}}
  if let Some(owner)=self.original_close.as_ref(){return if owner.terminal_is_empty(){Ok(header())}else{owner.selected_demand(body)}}
  if self.original.is_some(){return Ok(header())}
  Ok(RetirementDemand{release_bytes:self.wire.capacity(),depth:usize::from(self.wire.capacity()!=0),..Default::default()})
 }
 pub fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.close_demands(body)}
  Ok(match self.phase{
   0=>match self.admission.as_ref(){Some(admission)if admission.phase()!="ready"=>admission.next_demand()?,_=>header()},
   1|6=>semio_framework_value::retained_clone::RetainedOwnedProjection::<dyn Tree>::constructor_demand(),
   2|7|10=>header(),
   3|8=>self.tree.as_ref().map_or(Ok(header()),|tree|if tree.terminal_is_empty(){Ok(header())}else{tree.next_demand()})?,
   4=>RetirementDemand{capacity_bytes:self.length,depth:1,..Default::default()},
   5 if self.wire.len()<self.length=>RetirementDemand{copy_bytes:(self.length-self.wire.len()).min(body).min(64),depth:1,..Default::default()},
   5=>header(),
   9=>self.source.as_ref().map_or(Ok(header()),|source|if source.terminal_is_empty(){Ok(header())}else{source.next_close_demand(body)})?,
   _=>Default::default(),
  })
 }
 fn project(&mut self,effect:Effect<'_,'_>)->Result<RetainedCloneProgress,ValueError>{let source=self.source.as_ref().ok_or_else(||refuses("original canonical wire source is absent"))?;let turn=source.project_owned(0,|root|root as&dyn Tree,effect);self.last_turn_work=work_u64(turn.work)?;let(projection,receipt)=turn.result?;*self.projection=Some(projection);self.phase+=1;Ok(receipt)}
 fn admit_tree(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let projection=self.projection.take().ok_or_else(||refuses("original canonical wire projection is absent"))?;match ArtifactCanonicalJsonTreeCursor::admit(projection,grant){Ok((tree,receipt))=>{*self.tree=Some(tree);self.phase+=1;Ok(receipt)},Err((error,projection))=>{*self.projection=Some(projection);Err(error)}}}
 fn advance_one(&mut self,effect:Effect<'_,'_>)->Result<ArtifactCanonicalJsonTreeWireStep,ValueError>{
  let grant=effect.grant();let none=|progress|ArtifactCanonicalJsonTreeWireStep{output:None,progress};if self.terminal_is_empty()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(none(Default::default()))}let demand=self.next_demand(grant.maximum_copy_bytes).map_err(|error|{self.last_turn_work=1;error})?;if !permits(grant,demand){return Ok(none(Default::default()))}if self.closing{return self.close_one(effect).map(none)}
  self.last_turn_work=1;match self.phase{
   0=>{
    if self.admission.is_none(){let original=self.original.take().ok_or_else(||refuses("original canonical wire root is absent"))?;*self.admission=Some(RetainedCloneSource::<T>::admission_owned(original,()));pay(effect)?;return Ok(none(progress()))}
    let admission=self.admission.as_mut().ok_or_else(||refuses("original canonical wire admission is absent"))?;
    if admission.phase()=="ready"{let turn=admission.take_output(&mut self.source,grant);self.last_turn_work=work_u64(turn.work)?;let step=turn.result?;if admission.terminal_is_empty(){self.admission.take();self.phase=1;}Ok(none(step.progress()))}else{let turn=admission.advance(effect);self.last_turn_work=work_u64(turn.work)?;turn.result.map(|step|none(step.progress()))}
   },
   1|6=>self.project(effect).map(none),
   2|7=>{pay(effect)?;self.admit_tree(grant).map(none)},
   3|8=>{
    let tree=self.tree.as_mut().ok_or_else(||refuses("original canonical wire tree is absent"))?;if tree.terminal_is_empty(){assert!(tree.is_complete());self.tree.take();if self.phase==8&&self.written!=self.length{return Err(refuses("original canonical wire source changed its measured length"))}self.phase+=1;pay(effect)?;return Ok(none(progress()))}
    let original=if self.phase==3{tree.advance(&mut self.scratch,effect)}else{let end=self.written.saturating_add(64).min(self.length);tree.advance(&mut self.wire[self.written..end],effect)};self.last_turn_work=tree.last_turn_work();let step=original?;let receipt=step.ownership.progress();if self.phase==3{self.length=self.length.checked_add(step.written_bytes).ok_or_else(||refuses("original canonical wire measured length overflow").with_retained_progress(receipt))?}else{self.written=self.written.checked_add(step.written_bytes).filter(|length|*length<=self.length).ok_or_else(||refuses("original canonical wire source exceeded its measured output").with_retained_progress(receipt))?}Ok(none(receipt))
   },
   4=>{pay(effect)?;self.wire.try_reserve_exact(self.length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original canonical wire destination allocation refused"))?;self.phase=5;Ok(none(RetainedCloneProgress{retained_capacity_bytes:self.wire.capacity(),..progress()}))},
   5=>{let count=(self.length-self.wire.len()).min(grant.maximum_copy_bytes).min(64);if self.wire.len()==self.length{pay(effect)?;self.phase=6;return Ok(none(progress()))}if count==0{self.last_turn_work=0;return Ok(none(Default::default()))}pay(effect)?;let length=self.wire.len()+count;self.wire.resize(length,0);Ok(none(RetainedCloneProgress{copied_bytes:count,..progress()}))},
   9=>{let source=self.source.as_mut().ok_or_else(||refuses("original canonical wire source close is absent"))?;if source.terminal_is_empty(){self.source.take();self.phase=10;pay(effect)?;return Ok(none(progress()))}let(step,work)=settled(source.close_step(effect))?;self.last_turn_work=work_u64(work)?;Ok(none(step.progress()))},
   10=>{pay(effect)?;self.phase=11;Ok(ArtifactCanonicalJsonTreeWireStep{output:Some(std::mem::take(&mut*self.wire)),progress:progress()})},
   _=>{self.last_turn_work=0;Ok(none(Default::default()))},
  }
 }
 pub fn advance(&mut self,effect:Effect<'_,'_>)->Result<ArtifactCanonicalJsonTreeWireStep,ValueError>{self.last_turn_work=0;let result=self.advance_one(effect);if result.is_err(){self.begin_close()}result}
 fn close_one(&mut self,effect:Effect<'_,'_>)->Result<RetainedCloneProgress,ValueError>{
  if let Some(admission)=self.admission.as_mut(){let mut returned=None;let turn=admission.return_inputs(&mut returned,effect);self.last_turn_work=work_u64(turn.work)?;if let Some((original,()))=returned{*self.original=Some(original);}if admission.terminal_is_empty(){self.admission.take();}return turn.result.map(|step|step.progress())}
  if let Some(tree)=self.tree.as_mut(){if tree.terminal_is_empty(){self.last_turn_work=1;self.tree.take();pay(effect)?;return Ok(progress())}let result=tree.close_step(effect);self.last_turn_work=tree.last_turn_work();return result.map(|step|step.progress())}
  self.last_turn_work=1;
  if let Some(projection)=self.projection.as_mut(){if projection.terminal_is_empty(){self.projection.take();pay(effect)?;return Ok(progress())}let(step,work)=settled(projection.close_step(effect))?;self.last_turn_work=work_u64(work)?;return Ok(step.progress())}
  if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source.take();pay(effect)?;return Ok(progress())}let(step,work)=settled(source.close_step(effect))?;self.last_turn_work=work_u64(work)?;return Ok(step.progress())}
  if let Some(owner)=self.original_close.as_mut(){if owner.terminal_is_empty(){self.original_close.take();pay(effect)?;return Ok(progress())}let(step,work)=settled(owner.step(effect))?;self.last_turn_work=work_u64(owner.last_turn_work().max(work))?;return Ok(step.progress())}
  if let Some(original)=self.original.take(){*self.original_close=Some(ControlledRetirement::retain(original));pay(effect)?;return Ok(progress())}
  if self.wire.capacity()!=0{let bytes=self.wire.capacity();pay(effect)?;drop(std::mem::take(&mut*self.wire));return Ok(RetainedCloneProgress{released_bytes:bytes,..progress()})}self.last_turn_work=0;Ok(Default::default())
 }
 pub fn close_step(&mut self,effect:Effect<'_,'_>)->Result<RetainedCloneProgress,ValueError>{let grant=effect.grant();self.last_turn_work=0;self.begin_close();if self.terminal_is_empty()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(Default::default())}let demand=self.close_demands(grant.maximum_copy_bytes).map_err(|error|{self.last_turn_work=1;error})?;if !permits(grant,demand){return Ok(Default::default())}self.close_one(effect)}
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
