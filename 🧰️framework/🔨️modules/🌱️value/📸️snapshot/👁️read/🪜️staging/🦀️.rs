//! 🪜️ A staged Store-read source seal that any retired owner can hold between turns.
use crate::SnapshotRead;
use semio_framework_value::{RetainedWorkOutcome,RetirementDemand,ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneSource,RetainedCloneSourceAdmission};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep};

/// 🎟️ Holds the unmoved read inside one staged admission until its source is published; a close returns the read or hands the source to its own retirement.
pub struct SnapshotReadSourceStaging<T:RetireOwned+Sync>(Option<RetainedCloneSourceAdmission<T,SnapshotRead<T>>>);
impl<T:RetireOwned+Sync> Default for SnapshotReadSourceStaging<T>{fn default()->Self{Self(None)}}
impl<T:RetireOwned+Sync> SnapshotReadSourceStaging<T>{
 pub fn is_staged(&self)->bool{self.0.is_some()}
 /// 🧬️ Moves the read into staging; a refusal returns the exact read.
 pub fn begin(&mut self,read:SnapshotRead<T>)->Result<(),(ValueError,SnapshotRead<T>)>{self.0=Some(read.source_admission()?);Ok(())}
 pub fn next_demand(&self)->Result<RetirementDemand,ValueError>{self.0.as_ref().map_or(Ok(Default::default()),|admission|if admission.phase()=="ready"{Ok(RetirementDemand{depth:1,..Default::default()})}else{admission.next_demand()})}
 /// 🪜️ Runs one stage, or publishes the source into the caller's slot once every stage is paid.
 pub fn step(&mut self,output:&mut Option<RetainedCloneSource<T,SnapshotRead<T>>>,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  let Some(admission)=self.0.as_mut()else{return RetainedWorkOutcome{result:Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default())),work:0}};
  let turn=if admission.phase()=="ready"{admission.take_output(output,grant)}else{admission.advance(grant)};
  if output.is_some()&&admission.terminal_is_empty(){self.0=None;}
  turn
 }
 /// ↩️ Returns the unmoved read (or the published source) toward the caller's slots under a close grant.
 pub fn close_step(&mut self,read:&mut Option<SnapshotRead<T>>,output:&mut Option<RetainedCloneSource<T,SnapshotRead<T>>>,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  let Some(admission)=self.0.as_mut()else{return RetainedWorkOutcome{result:Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default())),work:0}};
  let turn=if admission.phase()=="ready"{admission.take_output(output,grant)}else{let mut inputs=None;let turn=admission.return_inputs(&mut inputs,grant);if let Some((returned,()))=inputs{*read=Some(returned);}turn};
  if admission.terminal_is_empty(){self.0=None;}
  turn
 }
 pub fn terminal_is_empty(&self)->bool{self.0.is_none()}
}
struct StagingCursor<T:RetireOwned+Sync>{staging:SnapshotReadSourceStaging<T>,base:Option<SnapshotRead<T>>,source:Option<RetainedCloneSource<T,SnapshotRead<T>>>}
impl<T:RetireOwned+Sync> RetireOwned for SnapshotReadSourceStaging<T>{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(StagingCursor{staging:self,base:None,source:None})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<StagingCursor<T>>())}
 fn controlled_retirement_supported()->bool{true}
}
impl<T:RetireOwned+Sync> RetirementCursor for StagingCursor<T>{
 fn next_demand(&self,_body:usize)->Result<RetirementDemand,ValueError>{if self.staging.is_staged(){self.staging.next_demand()}else{Ok(RetirementDemand{depth:usize::from(!self.terminal_is_empty()),..Default::default()})}}
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{
  if self.staging.is_staged(){
   let turn=self.staging.close_step(&mut self.base,&mut self.source,grant);
   return match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)};
  }
  if grant.maximum_items==0{return RetirementStep::BudgetExhausted;}
  if let Some(source)=self.source.take(){return RetirementStep::Child(source.retirement());}
  if let Some(base)=self.base.take(){return RetirementStep::Child(base.retirement());}
  RetirementStep::Complete(0)
 }
 fn terminal_is_empty(&self)->bool{self.staging.terminal_is_empty()&&self.base.is_none()&&self.source.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
}
