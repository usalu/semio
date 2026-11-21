//! 🪜️ A staged Store-read source seal that any retired owner can hold between turns.
use crate::SnapshotRead;
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError};
use semio_framework_value::retained_clone::{OriginalEffectAction,OriginalProjectionFrame,OriginalProjectionFramePhase,RetainedCloneSource,RetainedCloneSourceAdmission,retained_clone_grant_admits};
use semio_framework_value::retirement::{admit_retirement_cursor,retirement_cursor_birth_demand,retirement_cursor_birth_work,RetireOwned,RetirementCursor,RetirementStep};

/// 🎟️ Holds the unmoved read inside one staged admission until its source is published; a close returns the read or hands the source to its own retirement.
pub struct SnapshotReadSourceStaging<T:RetireOwned+Sync>{
 admission:Option<RetainedCloneSourceAdmission<T,SnapshotRead<T>>>,
 projection:Option<OriginalProjectionFrame<T,SnapshotRead<T>>>,
 action:Option<OriginalEffectAction<RetainedCloneStep,ValueError>>,
}
impl<T:RetireOwned+Sync> Default for SnapshotReadSourceStaging<T>{
 fn default()->Self{Self{admission:None,projection:None,action:None}}
}
impl<T:RetireOwned+Sync> SnapshotReadSourceStaging<T>{
 pub fn is_staged(&self)->bool{self.admission.is_some()}
 /// 🧬️ Moves the read into staging and returns it on refusal.
 pub fn begin(&mut self,read:SnapshotRead<T>)->Result<(),(ValueError,SnapshotRead<T>)>{
  if self.is_staged(){return Err((ValueError::invariant("snapshot read already staging"),read))}
  self.admission=Some(read.source_admission()?);Ok(())
 }
 pub fn next_demand(&self)->Result<RetirementDemand,ValueError>{
  let Some(admission)=self.admission.as_ref()else{return Ok(Default::default())};
  if let Some(stage)=admission.projection_stage(&self.projection)?{return Ok(admission.projection_stage_quote(&self.projection,stage)?.0)}
  if admission.projection_result()==Some(false){return Ok(admission.take_projection_failure_demand())}
  if admission.phase()=="ready"{return Ok(admission.take_output_demand())}
  admission.next_demand()
 }
 pub fn next_advance_work_demand(&self)->Result<u64,ValueError>{
  let Some(admission)=self.admission.as_ref()else{return Ok(0)};
  if let Some(stage)=admission.projection_stage(&self.projection)?{return Ok(admission.projection_stage_quote(&self.projection,stage)?.1)}
  if admission.projection_result()==Some(false){return Ok(1)}
  if admission.phase()=="ready"{return Ok(admission.take_output_work_demand())}
  admission.advance_work_demand()
 }
 fn fail(error:ValueError)->RetainedWorkOutcome{RetainedWorkOutcome{result:Err(error),work:0}}
 fn pending(work:u128)->RetainedWorkOutcome{RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work}}
 fn projected(poll:RetainedWorkPoll<RetainedCloneStep,ValueError>)->RetainedWorkOutcome{
  match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{work}=>Self::pending(work)}
 }
 fn projected_failure(admission:&mut RetainedCloneSourceAdmission<T,SnapshotRead<T>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  let mut failure=None;
  let turn=admission.take_projection_failure(&mut failure,effect);
  RetainedWorkOutcome{result:turn.result.and_then(|step|match failure{Some(error)=>Err(error.with_retained_progress(step.progress())),None=>Ok(step)}),work:turn.work}
 }
 fn funded(effect:&RetainedAllocationEffect<'_>,d:Result<RetirementDemand,ValueError>,work:Result<u64,ValueError>)->Result<bool,ValueError>{
  Ok(retained_clone_grant_admits(effect.grant(),d?)&&effect.remaining_work_units()? >= work?)
 }
 /// 🪜️ Drives the actual captured read's projection before publishing the source.
 pub fn step(&mut self,output:&mut Option<RetainedCloneSource<T,SnapshotRead<T>>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  match Self::funded(effect,self.next_demand(),self.next_advance_work_demand()){
   Ok(true)=>{},Ok(false)=>return Self::pending(0),Err(error)=>return Self::fail(error),
  }
  let Some(admission)=self.admission.as_mut()else{return Self::pending(0)};
  let selected=match admission.projection_stage(&self.projection){Ok(value)=>value,Err(error)=>return Self::fail(error)};
  let turn=if let Some(stage)=selected{
   Self::projected(admission.project_staged(&mut self.projection,&mut self.action,stage,effect))
  }else if admission.projection_result()==Some(false){Self::projected_failure(admission,effect)}
  else if admission.phase()=="ready"{admission.take_output(output,effect)}else{admission.advance(effect)};
  if admission.terminal_is_empty(){self.admission=None}
  turn
 }
 pub fn close_demand(&self)->Result<RetirementDemand,ValueError>{
  let Some(admission)=self.admission.as_ref()else{return Ok(Default::default())};
  if let Some(frame)=self.projection.as_ref(){
   return match frame.phase()?{
    OriginalProjectionFramePhase::Drive if self.action.is_some()=>Ok(frame.projection_demand()),
    OriginalProjectionFramePhase::Drive|OriginalProjectionFramePhase::TransferProducer=>frame.producer_transfer_demand(),
    OriginalProjectionFramePhase::AcceptAnswer=>admission.projection_answer_demand(frame),
    OriginalProjectionFramePhase::ReturnOwner=>admission.return_projection_owner_demand(frame),
    OriginalProjectionFramePhase::Finish=>Ok(admission.finish_projection_demand()),
   }
  }
  if admission.phase()=="ready"{return Ok(admission.take_output_demand())}
  match admission.projection_result(){
   Some(true)=>return Ok(admission.discard_projection_demand()),
   Some(false)=>return Ok(admission.take_projection_failure_demand()),
   None=>{}
  }
  admission.next_return_demand()
 }
 /// ↩️ Releases any projected answer back to its original source before returning the read or published source.
 pub fn close_step(&mut self,read:&mut Option<SnapshotRead<T>>,output:&mut Option<RetainedCloneSource<T,SnapshotRead<T>>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  match Self::funded(effect,self.close_demand(),self.next_work_demand()){
   Ok(true)=>{},Ok(false)=>return Self::pending(0),Err(error)=>return Self::fail(error),
  }
  let Some(admission)=self.admission.as_mut()else{return Self::pending(0)};
  let turn=if let Some(frame)=self.projection.as_mut(){
   let phase=match frame.phase(){Ok(phase)=>phase,Err(error)=>return Self::fail(error)};
   match phase{
    OriginalProjectionFramePhase::Drive if self.action.is_some()=>Self::projected(admission.project_staged(&mut self.projection,&mut self.action,2,effect)),
    OriginalProjectionFramePhase::Drive|OriginalProjectionFramePhase::TransferProducer=>{
     let turn=frame.transfer_producer(effect);
     RetainedWorkOutcome{result:turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())),work:turn.work}
    }
    OriginalProjectionFramePhase::AcceptAnswer=>admission.accept_projection(frame,effect),
    OriginalProjectionFramePhase::ReturnOwner=>admission.return_projection_owner(frame,effect),
    OriginalProjectionFramePhase::Finish=>admission.finish_projection(&mut self.projection,effect),
   }
  }else if admission.phase()=="ready"{admission.take_output(output,effect)}
  else if let Some(success)=admission.projection_result(){
   if success{admission.discard_projection(effect)}else{Self::projected_failure(admission,effect)}
  }else{
   let mut inputs=None;let turn=admission.return_inputs(&mut inputs,effect);
   if let Some((returned,()))=inputs{*read=Some(returned);}
   turn
  };
  if admission.terminal_is_empty(){self.admission=None}
  turn
 }
 pub fn next_work_demand(&self)->Result<u64,ValueError>{
  let Some(admission)=self.admission.as_ref()else{return Ok(0)};
  if let Some(frame)=self.projection.as_ref(){return match frame.phase()?{
   OriginalProjectionFramePhase::Drive if self.action.is_some()=>frame.projection_work_demand(),
   OriginalProjectionFramePhase::AcceptAnswer=>Ok(2),
   _=>Ok(1),
  }}
  if admission.phase()=="ready"{return Ok(admission.take_output_work_demand())}
  if admission.projection_result().is_some(){return Ok(1)}
  admission.return_work_demand()
 }
 pub fn terminal_is_empty(&self)->bool{self.admission.is_none()&&self.projection.is_none()&&self.action.is_none()}
}
fn admit_child<R:RetireOwned>(slot:&mut Option<R>,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{let turn=admit_retirement_cursor(slot,effect);match turn.result{Ok(Some((cursor,progress)))=>RetirementStep::ChildProgress(cursor,progress,turn.work),Ok(None)=>RetirementStep::BudgetExhausted,Err(error)=>RetirementStep::Failure(error.into(),turn.work)}}
#[doc(hidden)]
pub struct StagingCursor<T:RetireOwned+Sync>{staging:SnapshotReadSourceStaging<T>,base:Option<SnapshotRead<T>>,source:Option<RetainedCloneSource<T,SnapshotRead<T>>>}
impl<T:RetireOwned+Sync> RetireOwned for SnapshotReadSourceStaging<T>{
 type RetirementCursor=StagingCursor<T>;
 fn retirement_seed(self)->Self::RetirementCursor{StagingCursor{staging:self,base:None,source:None}}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<StagingCursor<T>>())}
 fn controlled_retirement_supported()->bool{true}
}
impl<T:RetireOwned+Sync> RetirementCursor for StagingCursor<T>{
 fn next_demand(&self,_body:usize)->Result<RetirementDemand,ValueError>{
  if self.staging.is_staged(){return self.staging.close_demand()}
  if let Some(source)=self.source.as_ref(){return retirement_cursor_birth_demand(source)}
  if let Some(base)=self.base.as_ref(){return retirement_cursor_birth_demand(base)}
  Ok(Default::default())
 }
 fn close_into<'r>(&mut self,_body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:semio_framework_value::retained_clone::OriginalEffectReply<'r,RetirementStep,semio_framework_value::OriginalRetirementFailure>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,semio_framework_value::OriginalRetirementFailure>,semio_framework_value::ValueError>{Ok(reply.complete(Ok(self.close_sync(effect))))}
    fn recover_close_into<'r>(&mut self,_body:usize,_:&mut semio_framework_value::RetainedAllocationEffect<'_>,_:semio_framework_value::retained_clone::OriginalEffectReply<'r,RetirementStep,semio_framework_value::OriginalRetirementFailure>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,semio_framework_value::OriginalRetirementFailure>,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::invariant("synchronous close has no unrecorded callback"))}
 fn next_computational_work(&self,_body:usize)->Result<u64,ValueError>{if self.staging.is_staged(){return self.staging.next_work_demand()}Ok(if self.source.is_some()||self.base.is_some(){retirement_cursor_birth_work()}else{0})}
 fn terminal_is_empty(&self)->bool{self.staging.terminal_is_empty()&&self.base.is_none()&&self.source.is_none()}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
}
impl<T:RetireOwned+Sync> StagingCursor<T>{
 fn close_sync(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();
  if self.staging.is_staged(){
   let turn=self.staging.close_step(&mut self.base,&mut self.source,effect);
   return match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error.into(),turn.work)};
  }
  if grant.maximum_items==0||effect.maximum_work_units()==0{return RetirementStep::BudgetExhausted;}
  if self.source.is_some(){return admit_child(&mut self.source,effect);}
  if self.base.is_some(){return admit_child(&mut self.base,effect);}
  RetirementStep::Complete(0)
 }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
