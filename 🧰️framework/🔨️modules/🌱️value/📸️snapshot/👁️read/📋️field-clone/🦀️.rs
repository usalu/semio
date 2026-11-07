//! 📋️ Clones one actual native field while retaining its original captured Store issuer.
use super::SnapshotRead;
use semio_framework_value::{RetirementDemand,RetainedWorkOutcome,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::retained_clone::{RetainedClone,RetainedCloneOwner,RetainedCloneSource,RetainedCloneSourceAdmission,RetainedOwnedProjection};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
use std::mem::ManuallyDrop;
pub struct SnapshotReadFieldClone<T:RetireOwned+Sync,U:RetainedClone>{read:ManuallyDrop<Option<SnapshotRead<T>>>,admission:Option<RetainedCloneSourceAdmission<T,SnapshotRead<T>>>,source:Option<RetainedCloneSource<T,SnapshotRead<T>>>,projection:Option<RetainedOwnedProjection<U>>,cursor:ManuallyDrop<RetainedCloneOwner<U>>,output:ManuallyDrop<Option<U>>,read_close:Option<ControlledRetirement<SnapshotRead<T>>>,output_close:Option<ControlledRetirement<U>>,project:for<'a>fn(&'a T)->&'a U,discriminator:usize,revision:[u8;32],generation:u64,phase:u8,closing:bool,progress:RetainedCloneProgress,work:u64}
impl<T:RetireOwned+Sync,U:RetainedClone> SnapshotReadFieldClone<T,U>{
 pub fn new(read:SnapshotRead<T>,project:for<'a>fn(&'a T)->&'a U,discriminator:usize,revision:[u8;32],generation:u64)->Self{Self{read:ManuallyDrop::new(Some(read)),admission:None,source:None,projection:None,cursor:ManuallyDrop::new(Default::default()),output:ManuallyDrop::new(None),read_close:None,output_close:None,project,discriminator,revision,generation,phase:0,closing:false,progress:Default::default(),work:0}}
 pub fn original(&self)->Option<&SnapshotRead<T>>{self.read.as_ref().or_else(||self.admission.as_ref().and_then(RetainedCloneSourceAdmission::original_authority)).or_else(||self.source.as_ref().and_then(RetainedCloneSource::original_authority))}
 pub fn validate_current(&self)->Result<(),ValueError>{if self.original().is_some_and(|read|read.commit_authority_matches(self.generation,self.revision)){Ok(())}else{Err(ValueError::literal(ValueRefusalKind::Canceled,"original native field read changed at its issuer"))}}

 pub fn next_source_demands(&self,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original native field source is closing"))}self.validate_current()?;
  if self.source.is_some(){return Ok(Default::default())}if self.phase<=1{return self.next_demands(0,maximum_work_units)}Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native field source is absent after admission"))
 }
 pub fn prepare_source(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool,semio_framework_value::OriginalRetirementFailure>{
  self.progress=Default::default();self.work=0;if let Err(error)=self.next_source_demands(effect.maximum_work_units()){return RetainedWorkOutcome{result:Err(error.into()),work:0}}if self.source.is_some(){return RetainedWorkOutcome{result:Ok(true),work:0}}
  let turn=self.advance(effect);let result=match turn.result{Ok(None)=>Ok(self.source.is_some()),Ok(Some(output))=>{self.restore_output(output);Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"source admission unexpectedly transferred its native field").into())},Err(error)=>Err(error)};
  RetainedWorkOutcome{result,work:turn.work}
 }
 pub fn project_original<V:?Sized+Sync>(&self,discriminator:usize,project:for<'a>fn(&'a T)->&'a V,grant:RetainedCloneGrant)->RetainedWorkOutcome<(RetainedOwnedProjection<V>,RetainedCloneProgress)>{
  if self.closing{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::Canceled,"original native source is closing")),work:0}}
  if let Err(error)=self.validate_current(){return RetainedWorkOutcome{result:Err(error),work:0}}
  match self.source.as_ref(){Some(source)=>source.project_owned(discriminator,project,grant),None=>RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original native source admission has not completed")),work:0}}
 }

 pub fn revision(&self)->[u8;32]{self.revision}
 pub fn generation(&self)->u64{self.generation}
 pub fn complete(&self)->bool{matches!(self.phase,7|8)}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn last_turn_work(&self)->u64{self.work}
 pub fn next_demands(&self,copy:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.next_close_demands(copy)}self.validate_current()?;
  match self.phase{
   1=>{let admission=self.admission.as_ref().unwrap();if admission.phase()=="ready"{Ok(RetirementDemand{depth:1,..Default::default()})}else{admission.next_demand()}},
   2=>Ok(RetainedOwnedProjection::<U>::constructor_demand()),
   3=>self.cursor.advance_demands(self.projection.as_ref().unwrap().borrow()?,copy,maximum_work_units),
   4=>self.cursor.take_demand(),
   5=>self.cursor.selected_close_demand(copy),
   6=>self.projection.as_ref().unwrap().next_close_demand(copy),
   _=>Ok(RetirementDemand{depth:1,..Default::default()}),
  }
 }
 pub fn advance(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<U>,semio_framework_value::OriginalRetirementFailure>{self.progress=Default::default();self.work=0;let result=self.advance_original(effect).map_err(|error|{let original=error.retained_progress();if original!=Default::default(){self.progress=original}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}}
 fn advance_original(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<Option<U>,semio_framework_value::OriginalRetirementFailure>{
  let grant=effect.grant();
  if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original native field clone is closing").into())}let demand=self.next_demands(grant.maximum_copy_bytes,effect.maximum_work_units())?;
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None)}
  self.work=1;self.progress.copied_items=1;
  match self.phase{
   0=>{let read=self.read.take().unwrap();match read.source_admission(){Ok(admission)=>{self.admission=Some(admission);self.phase=1},Err((error,read))=>{*self.read=Some(read);return Err(error.into())}}},
   1=>{let admission=self.admission.as_mut().unwrap();let turn=if admission.phase()=="ready"{admission.take_output(&mut self.source,grant)}else{admission.advance(effect)};self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if admission.terminal_is_empty(){self.admission=None;self.phase=2}},
   2=>{let project=self.project;let turn=self.source.as_ref().unwrap().project_owned(self.discriminator,project,grant);self.work=turn.work;self.progress=Default::default();let(projection,progress)=turn.result?;self.projection=Some(projection);self.progress=progress;self.phase=3},
   3=>{let turn=self.cursor.advance(self.projection.as_ref().unwrap().borrow()?,grant.maximum_copy_bytes,effect.maximum_work_units(),effect);self.work=turn.work;self.progress=Default::default();let step=turn.result?;self.progress=step.progress();if matches!(step,RetainedCloneStep::Complete(_)){self.phase=4}},
   4=>{let turn=self.cursor.take(grant);self.work=turn.work;self.progress=Default::default();if let Some((output,progress))=turn.result?{*self.output=Some(output);self.progress=progress;self.phase=5}},
   5=>{let turn=self.cursor.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if self.cursor.terminal_is_empty(){self.phase=6}},
   6=>{let projection=self.projection.as_mut().unwrap();let turn=projection.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if projection.terminal_is_empty(){self.projection=None;self.phase=7}},
   7=>{self.phase=8;return Ok(self.output.take())},
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native field output was transferred").into()),
  }Ok(None)
 }
 pub fn restore_output(&mut self,output:U){assert_eq!(self.phase,8);*self.output=Some(output);self.phase=7}
 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(admission)=self.admission.as_ref(){return admission.next_return_demand()}
  if !self.cursor.terminal_is_empty(){return self.cursor.selected_close_demand(copy)}
  if let Some(projection)=self.projection.as_ref(){return projection.next_close_demand(copy)}
  if let Some(source)=self.source.as_ref(){return source.next_close_demand(copy)}
  if let Some(owner)=self.read_close.as_ref(){return owner.selected_demand(copy)}
  if let Some(owner)=self.output_close.as_ref(){return owner.selected_demand(copy)}
  Ok(RetirementDemand{depth:usize::from(self.read.is_some()||self.output.is_some()),..Default::default()})
 }
 pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementOutcome{let grant=effect.grant();self.progress=Default::default();self.work=0;let result=self.close_original(grant).map_err(|error|{let original=error.retained_progress();if original!=Default::default(){self.progress=original}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}}
 fn close_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>{
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native field close was not begun").into())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=self.next_close_demands(grant.maximum_copy_bytes)?;
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}self.work=1;
  if let Some(admission)=self.admission.as_mut(){let mut returned=None;let turn=admission.return_inputs(&mut returned,grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if let Some((read,()))=returned{*self.read=Some(read)}if admission.terminal_is_empty(){self.admission=None}return Ok(RetainedCloneStep::Progress(self.progress))}
  if !self.cursor.terminal_is_empty(){let turn=self.cursor.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(projection)=self.projection.as_mut(){let turn=projection.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if projection.terminal_is_empty(){self.projection=None}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(source)=self.source.as_mut(){let turn=source.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if source.terminal_is_empty(){self.source=None}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(read)=self.read.take(){self.read_close=Some(ControlledRetirement::new(read).unwrap_or_else(|_|unreachable!("original captured read supports controlled retirement")));self.progress=RetainedCloneProgress{copied_items:1,..Default::default()};return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(owner)=self.read_close.as_mut(){let result=owner.step(grant);self.work=owner.last_turn_work();self.progress=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.read_close=None}return Ok(RetainedCloneStep::Progress(step.progress()))}
  if let Some(output)=self.output.take(){self.output_close=Some(ControlledRetirement::new(output).unwrap_or_else(|_|unreachable!("original cloned field supports controlled retirement")));self.progress=RetainedCloneProgress{copied_items:1,..Default::default()};return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(owner)=self.output_close.as_mut(){let result=owner.step(grant);self.work=owner.last_turn_work();self.progress=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.output_close=None}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
  self.work=0;Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.read.is_none()&&self.admission.is_none()&&self.source.is_none()&&self.projection.is_none()&&self.cursor.terminal_is_empty()&&self.output.is_none()&&self.read_close.is_none()&&self.output_close.is_none()}
}
impl<T:RetireOwned+Sync,U:RetainedClone> Drop for SnapshotReadFieldClone<T,U>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native field clone retains its original captured read or output");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.read);ManuallyDrop::drop(&mut self.output);ManuallyDrop::drop(&mut self.cursor)}}}}

