//! 🔁️ Original history uses the same Raster semantic candidate and cached point inverse as publication.
use super::*;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,ArtifactOwnedValueRetirementFactory,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetirementPollExit};
use std::sync::Arc;
use semio_framework_value::retained_clone::{RetainedCloneRef,RetainedCloneClose,retained_clone_grant_admits};
use semio_s_artifact_raster_raster_host::{RasterOneItemApply,RasterPreparedMutation};
use store::os_store::replay_preparation::{ArtifactReplayPreparation,ArtifactReplayPreparationFactory,ArtifactReplayPreparationRequest,ArtifactReplayPreparationContext,native::{NativeSemanticPreparation,SemanticProjection,SemanticEdit,SemanticEditStep}};
use store::os_store::one_item::{PagedOneItemPostMode,PagedOneItemPostRead,PagedOneItemPost};
use std::mem::{size_of,ManuallyDrop};

fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn perform(effect:&mut RetainedAllocationEffect<'_>,units:u64)->Result<(),ValueError>{if effect.performed(units)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Raster replay action exceeds its work ceiling"))}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}

#[derive(Default)]
struct ScratchProjection{value:ManuallyDrop<Option<RasterSnapshot>>,close:RetainedCloneClose,closing:bool}
impl SemanticProjection<RasterSnapshot> for ScratchProjection{
 fn birth_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
 fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.value.is_some()))}
 fn birth_demand(&self)->RetirementDemand{header::<RasterSnapshot>()}
 fn begin(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{let d=self.birth_demand();if !retained_clone_grant_admits(effect.grant(),d){return Err(invalid("Raster replay scratch header is unfunded"))}if self.value.is_some()||self.closing{return Err(invalid("Raster replay scratch header already began"))}perform(effect,1)?;*self.value=Some(RasterSnapshot{schema:String::new(),id:String::new(),title:None,layers:Vec::new(),assets:semio_s_artifact_raster_raster_model::RasterOwnedMap::new()});Ok(progress(d))}
 fn demands(&self,_:RetainedCloneRef<'_,RasterSnapshot>,_:usize,_:u64)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:1,..Default::default()})}
 fn advance(&mut self,_:RetainedCloneRef<'_,RasterSnapshot>,_:usize,_:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,OriginalRetirementFailure>{if self.value.is_none()||self.closing{return Err(invalid("Raster replay scratch owner is absent").into())}Ok(RetainedCloneStep::Complete(Default::default()))}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{Ok(header::<RasterSnapshot>())}
 fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(RasterSnapshot,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if self.closing||self.value.is_none()||!retained_clone_grant_admits(effect.grant(),d){return Ok(None)}perform(effect,1)?;Ok(self.value.take().map(|value|(value,progress(d))))}
 fn begin_close(&mut self){self.closing=true}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close.next_owner_demands::<RasterSnapshot>(self.value.is_some(),&None,body)}
 fn close_work_demand(&self,_:usize)->Result<Option<u64>,ValueError>{Ok(Some(u64::from(self.closing&&(self.value.is_some()||!self.close.is_empty()))))}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
   if !self.closing{return Ok(RetainedCloneStep::Progress(Default::default()))}
   if !self.close.is_empty(){return RetirementPollExit::resume(self.close.step_granted(effect))}
   Ok(self.close.begin_granted(&mut self.value,effect).result?.unwrap_or(RetainedCloneStep::Complete(Default::default())))
  })();
  RetirementPollExit::settle(effect,result)
 }
 fn terminal_is_empty(&self)->bool{self.closing&&self.value.is_none()&&self.close.is_empty()}
}
impl Drop for ScratchProjection{fn drop(&mut self){assert!(std::thread::panicking()||self.value.is_none()&&self.close.is_empty(),"Raster replay scratch owner requires original close");}}

struct ReplayEdit{apply:Option<RasterOneItemApply>,prepared:ManuallyDrop<Option<RasterPreparedMutation>>,old:ManuallyDrop<Option<RasterSnapshot>>,close:RetainedCloneClose,context:ArtifactReplayPreparationContext,closing:bool,done:bool,last_work:u64}
impl ReplayEdit{fn new(context:ArtifactReplayPreparationContext)->Self{Self{apply:None,prepared:ManuallyDrop::new(None),old:ManuallyDrop::new(None),close:Default::default(),context,closing:false,done:false,last_work:0}}}
impl SemanticEdit<RasterSnapshot,RasterMutation> for ReplayEdit{
 fn birth_work_demand(&self)->u64{1}
 fn last_turn_work(&self)->u64{self.last_work}
 fn birth_demand(&self)->RetirementDemand{header::<Option<RasterOneItemApply>>()}
 fn begin(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{self.last_work=0;let d=self.birth_demand();if !retained_clone_grant_admits(effect.grant(),d){return Err(invalid("Raster replay semantic header is unfunded"))}if self.apply.is_some()||self.closing{return Err(invalid("Raster replay semantic candidate already began"))}perform(effect,1)?;self.apply=Some(RasterOneItemApply::new());Ok(progress(d))}
 fn post_mode(&self)->PagedOneItemPostMode{PagedOneItemPostMode::Mutable}
 fn demands(&self,before:RetainedCloneRef<'_,RasterSnapshot>,input:RetainedCloneRef<'_,RasterMutation>,_:PagedOneItemPostRead<'_,RasterSnapshot>,body:usize)->Result<RetirementDemand,ValueError>{if self.prepared.is_some(){return Ok(header::<(RasterSnapshot,RasterSnapshot,Option<Vec<RasterMutation>>)>())}self.apply.as_ref().ok_or_else(||invalid("Raster replay semantic candidate is absent"))?.advance_demands(before.get(),input.get(),body)}
 fn advance(&mut self,before:RetainedCloneRef<'_,RasterSnapshot>,input:RetainedCloneRef<'_,RasterMutation>,post:PagedOneItemPost<'_,RasterSnapshot>,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<SemanticEditStep<RasterMutation>,ValueError>{
  self.last_work=0;if self.closing||self.done{return Err(invalid("Raster replay semantic candidate is closed"))}
  if self.prepared.is_none(){let apply=self.apply.as_mut().ok_or_else(||invalid("Raster replay semantic candidate is absent"))?;let d=apply.advance_demands(before.get(),input.get(),body)?;if !retained_clone_grant_admits(grant,d){return Ok(SemanticEditStep::Progress(Default::default()))}let turn=apply.advance_replay_controlled(before.get(),input.get(),self.context,grant,receiving);self.last_work=turn.work;let(output,receipt)=turn.result?;if let Some(output)=output{*self.prepared=Some(output)}return Ok(SemanticEditStep::Progress(receipt))}
  let d=header::<(RasterSnapshot,RasterSnapshot,Option<Vec<RasterMutation>>)>() ;if !retained_clone_grant_admits(grant,d)||!receiving.prepare_work(1)?{return Ok(SemanticEditStep::Progress(Default::default()))}if self.prepared.as_ref().is_none_or(|value|value.inverse.is_none()){return Err(invalid("Raster replay semantic inverse is absent"))}let post=post.mutable()?;let RasterPreparedMutation{snapshot,inverse}=self.prepared.take().unwrap();*self.old=Some(std::mem::replace(post,snapshot));self.done=true;self.last_work=1;let receipt=progress(d);receiving.commit_work(1).map_err(|error|error.with_retained_progress(receipt))?;Ok(SemanticEditStep::Complete(receipt,inverse.unwrap(),Vec::new(),false))
 }
 fn begin_close(&mut self){self.closing=true}
 fn close_demand(&self,_:&Arc<dyn ArtifactOwnedValueRetirementFactory<RasterMutation>>,body:usize)->Result<RetirementDemand,ValueError>{if let Some(apply)=self.apply.as_ref(){return if apply.terminal_is_empty(){Ok(header::<Option<RasterOneItemApply>>())}else{apply.close_demands(body)}}if self.prepared.is_some(){return self.close.next_owner_demands::<RasterPreparedMutation>(true,&None,body)}self.close.next_owner_demands::<RasterSnapshot>(self.old.is_some(),&None,body)}
 fn close_work_demand(&self,_:&Arc<dyn ArtifactOwnedValueRetirementFactory<RasterMutation>>,_:usize)->Result<u64,ValueError>{Ok(u64::from(self.closing&&!self.terminal_is_empty()))}
 fn close_step(&mut self,_:&Arc<dyn ArtifactOwnedValueRetirementFactory<RasterMutation>>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
   let grant=effect.grant();self.last_work=0;
   if !self.closing{return Ok(RetainedCloneStep::Progress(Default::default()))}
   if let Some(apply)=self.apply.as_mut(){
    if apply.terminal_is_empty(){let d=header::<Option<RasterOneItemApply>>();if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}RetirementPollExit::pay(effect,1)?;self.apply.take();self.last_work=1;return Ok(RetainedCloneStep::Progress(progress(d)))}
    let turn=apply.close_step(grant);self.last_work=u64::try_from(turn.work).unwrap_or(u64::MAX);RetirementPollExit::pay(effect,turn.work)?;return Ok(turn.result?)
   }
   if !self.close.is_empty(){return RetirementPollExit::resume(self.close.step_granted(effect))}
   let step=if self.prepared.is_some(){self.close.begin_granted(&mut self.prepared,effect)}else{self.close.begin_granted(&mut self.old,effect)}.result?;
   Ok(step.unwrap_or(RetainedCloneStep::Complete(Default::default())))
  })();
  RetirementPollExit::settle(effect,result)
 }
 fn terminal_is_empty(&self)->bool{self.closing&&self.apply.is_none()&&self.prepared.is_none()&&self.old.is_none()&&self.close.is_empty()}
}
impl Drop for ReplayEdit{fn drop(&mut self){assert!(std::thread::panicking()||self.apply.is_none()&&self.prepared.is_none()&&self.old.is_none()&&self.close.is_empty(),"Raster replay semantic owner requires original close");}}
type Producer=NativeSemanticPreparation<RasterSnapshot,RasterMutation,ScratchProjection,ReplayEdit>;
impl ArtifactReplayPreparationFactory<RasterSnapshot,RasterMutation> for RasterStorePreparationFactory{
 fn preparation_birth_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Producer>()+size_of::<ArtifactReplayPreparationRequest<RasterSnapshot,RasterMutation>>(),capacity_bytes:size_of::<Producer>(),depth:1,..Default::default()}}
 fn begin(&self,request:ArtifactReplayPreparationRequest<RasterSnapshot,RasterMutation>,grant:RetainedCloneGrant)->Result<(Box<dyn ArtifactReplayPreparation<RasterSnapshot,RasterMutation>>,RetainedCloneProgress),(ValueError,ArtifactReplayPreparationRequest<RasterSnapshot,RasterMutation>)>{let d=ArtifactReplayPreparationFactory::preparation_birth_demand(self);if !retained_clone_grant_admits(grant,d){return Err((invalid("Raster original replay producer is unfunded"),request))}let context=request.context;Ok((Box::new(Producer::new(request,ScratchProjection::default(),ReplayEdit::new(context))),progress(d)))}
}
