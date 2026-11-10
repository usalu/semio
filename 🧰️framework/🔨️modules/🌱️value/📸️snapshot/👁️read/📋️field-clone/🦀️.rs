//! 📋️ Clones one actual native field while retaining its original captured Store issuer.
use super::SnapshotRead;
use semio_framework_value::{RetirementDemand,RetainedWorkOutcome,RetainedAllocationEffect,OriginalRetirementPoll,RetirementPollExit,OriginalRetirementFailure,ValueError,ValueRefusalKind,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedClone,RetainedCloneOwner,RetainedCloneSource,RetainedCloneSourceAdmission,RetainedNativeFieldProjection,RetainedOwnedProjectionAdmission};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
use std::mem::ManuallyDrop;
/// 📋️ Clones one native field of a captured read; the projection stays an alias of the original captured source until closed.
pub struct SnapshotReadFieldClone<T:RetireOwned+Sync,U:RetainedClone>{read:ManuallyDrop<Option<SnapshotRead<T>>>,admission:Option<RetainedCloneSourceAdmission<T,SnapshotRead<T>>>,source:Option<RetainedCloneSource<T,SnapshotRead<T>>>,projection:ManuallyDrop<RetainedOwnedProjectionAdmission<U>>,cursor:ManuallyDrop<RetainedCloneOwner<U>>,output:ManuallyDrop<Option<U>>,read_close:Option<ControlledRetirement<SnapshotRead<T>>>,output_close:Option<ControlledRetirement<U>>,witness:RetainedNativeFieldProjection<T,U>,revision:[u8;32],generation:u64,phase:u8,closing:bool,pending:bool,progress:RetainedCloneProgress,work:u128}
fn pending_refused(exit:RetirementPollExit)->OriginalRetirementFailure{match exit{RetirementPollExit::Failed(error)=>error,RetirementPollExit::Pending(_)=>ValueError::literal(ValueRefusalKind::InvariantViolated,"original native field child left its producer reply pending inside advance").into()}}
impl<T:RetireOwned+Sync,U:RetainedClone> SnapshotReadFieldClone<T,U>{
 /// 🪪️ Creates the clone of one actual native field; the accessor invariant (bounded, callback-free, nonallocating, nonunwinding) is carried by the unsafely constructed witness.
 pub fn new(read:SnapshotRead<T>,witness:RetainedNativeFieldProjection<T,U>,revision:[u8;32],generation:u64)->Self{Self{read:ManuallyDrop::new(Some(read)),admission:None,source:None,projection:ManuallyDrop::new(Default::default()),cursor:ManuallyDrop::new(Default::default()),output:ManuallyDrop::new(None),read_close:None,output_close:None,witness,revision,generation,phase:0,closing:false,pending:false,progress:Default::default(),work:0}}
 pub fn original(&self)->Option<&SnapshotRead<T>>{self.read.as_ref().or_else(||self.admission.as_ref().and_then(RetainedCloneSourceAdmission::original_authority)).or_else(||self.source.as_ref().and_then(RetainedCloneSource::original_authority))}
 pub fn validate_current(&self)->Result<(),ValueError>{if self.original().is_some_and(|read|read.commit_authority_matches(self.generation,self.revision)){Ok(())}else{Err(ValueError::literal(ValueRefusalKind::Canceled,"original native field read changed at its issuer"))}}

 pub fn next_source_demands(&self,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original native field source is closing"))}self.validate_current()?;
  if self.source.is_some(){return Ok(Default::default())}if self.phase<=1{return self.next_demands(0,maximum_work_units)}Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native field source is absent after admission"))
 }
 pub fn prepare_source(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool,OriginalRetirementFailure>{
  self.progress=Default::default();self.work=0;if let Err(error)=self.next_source_demands(effect.maximum_work_units()){return RetainedWorkOutcome{result:Err(error.into()),work:0}}if self.source.is_some(){return RetainedWorkOutcome{result:Ok(true),work:0}}
  let turn=self.advance(effect);let result=match turn.result{Ok(None)=>Ok(self.source.is_some()),Ok(Some(output))=>{self.restore_output(output);Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"source admission unexpectedly transferred its native field").into())},Err(error)=>Err(error)};
  RetainedWorkOutcome{result,work:turn.work}
 }
 /// 📥️ Projects another native field of the original source into the caller's admission slot.
 pub fn project_original<'reply,V:?Sized+Sync>(&self,witness:&RetainedNativeFieldProjection<T,V>,output:&mut RetainedOwnedProjectionAdmission<V>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{
  if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original native source is closing"))}self.validate_current()?;
  let source=self.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original native source admission has not completed"))?;
  witness.admit_into(source.try_borrow()?,output,effect,reply)
 }

 pub fn revision(&self)->[u8;32]{self.revision}
 pub fn generation(&self)->u64{self.generation}
 pub fn complete(&self)->bool{matches!(self.phase,7|8)}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn last_turn_work(&self)->u128{self.work}
 pub fn next_demands(&self,copy:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.next_close_demands(copy)}self.validate_current()?;
  match self.phase{
   1=>{let admission=self.admission.as_ref().unwrap();if admission.phase()=="ready"{Ok(RetirementDemand{depth:1,..Default::default()})}else{admission.next_demand()}},
   2=>self.witness.demand(self.source.as_ref().unwrap().try_borrow()?,&self.projection),
   3=>self.cursor.advance_demands(self.projection.borrow()?,copy,maximum_work_units),
   4=>self.cursor.take_demand(),
   5=>self.cursor.selected_close_demand(copy),
   6=>self.projection.close_demand(copy),
   _=>Ok(RetirementDemand{depth:1,..Default::default()}),
  }
 }
 pub fn advance(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<U>,OriginalRetirementFailure>{self.progress=Default::default();self.work=0;let result=self.advance_original(effect).map_err(|error|{let original=error.retained_progress();if original!=Default::default(){self.progress=original}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}}
 fn settle(&mut self,effect:&mut RetainedAllocationEffect<'_>,before:u128,slot:Option<Result<RetainedCloneStep,OriginalRetirementFailure>>)->Result<Option<RetainedCloneStep>,OriginalRetirementFailure>{
  self.work=effect.performed_work().saturating_sub(before);self.progress=Default::default();self.pending=slot.is_none();
  match slot{Some(Ok(step))=>{self.progress=step.progress();Ok(Some(step))},Some(Err(error))=>Err(error),None=>Ok(None)}
 }
 fn advance_original(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<U>,OriginalRetirementFailure>{
  let grant=effect.grant();
  if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original native field clone is closing").into())}let demand=self.next_demands(grant.maximum_copy_bytes,effect.maximum_work_units())?;
  if grant.maximum_items==0||effect.maximum_work_units()==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None)}
  self.progress.copied_items=1;
  match self.phase{
   0=>{crate::pay_pure_work(effect,&mut self.work)?;let read=self.read.take().unwrap();match read.source_admission(){Ok(admission)=>{self.admission=Some(admission);self.phase=1},Err((error,read))=>{*self.read=Some(read);return Err(error.into())}}},
   1=>{let admission=self.admission.as_mut().unwrap();let turn=if admission.phase()=="ready"{admission.take_output(&mut self.source,effect)}else{admission.advance(effect)};self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if admission.terminal_is_empty(){self.admission=None;self.phase=2}},
   2=>{let before=effect.performed_work();let mut slot=None;{let reply=OriginalEffectReply::borrow(&mut slot)?;let source=self.source.as_ref().unwrap().try_borrow()?;drop(self.witness.admit_into(source,&mut self.projection,effect,reply)?);}self.work=effect.performed_work().saturating_sub(before);self.progress=Default::default();match slot.take(){Some(Ok(step))=>{if matches!(step,RetainedCloneStep::Complete(_)){self.progress=step.progress();self.phase=3}},None=>{},Some(Err(error))=>return Err(error.into())}},
   3=>{let before=effect.performed_work();let mut slot=None;{let reply=OriginalEffectReply::borrow(&mut slot)?;let source=self.projection.borrow()?;let owner=&mut *self.cursor;drop(if self.pending{owner.recover_advance_into(source,grant.maximum_copy_bytes,effect.maximum_work_units(),effect,reply)?}else{owner.advance_into(source,grant.maximum_copy_bytes,effect.maximum_work_units(),effect,reply)?});}if let Some(step)=self.settle(effect,before,slot.take())?{if matches!(step,RetainedCloneStep::Complete(_)){self.phase=4}}},
   4=>{let before=effect.performed_work();let mut slot=None;let mut output=None;{let reply=OriginalEffectReply::borrow(&mut slot)?;drop(if self.pending{self.cursor.recover_take(&mut output,effect,reply)?}else{self.cursor.take(&mut output,effect,reply)?});}self.work=effect.performed_work().saturating_sub(before);self.progress=Default::default();self.pending=slot.is_none();match slot.take(){Some(Ok(RetainedCloneStep::Complete(progress)))=>{*self.output=output.take();self.progress=progress;self.phase=5},Some(Ok(RetainedCloneStep::Progress(_)))|None=>{},Some(Err(error))=>return Err(error.into())}},
   5=>{let before=effect.performed_work();let mut slot=None;{let reply=OriginalEffectReply::borrow(&mut slot)?;drop(if self.pending{self.cursor.recover_close_into(grant.maximum_copy_bytes,effect,reply)?}else{self.cursor.close_into(grant.maximum_copy_bytes,effect,reply)?});}self.settle(effect,before,slot.take())?;if self.cursor.terminal_is_empty(){self.phase=6}},
   6=>{let turn=RetirementPollExit::settled(self.projection.close_step(grant.maximum_copy_bytes,effect)).map_err(pending_refused)?;self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if self.projection.terminal_is_empty(){self.phase=7}},
   7=>{crate::pay_pure_work(effect,&mut self.work)?;self.phase=8;return Ok(self.output.take())},
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native field output was transferred").into()),
  }Ok(None)
 }
 pub fn restore_output(&mut self,output:U){assert_eq!(self.phase,8);*self.output=Some(output);self.phase=7}
 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(admission)=self.admission.as_ref(){return admission.next_return_demand()}
  if !self.cursor.terminal_is_empty(){return self.cursor.selected_close_demand(copy)}
  if !self.projection.terminal_is_empty(){return self.projection.close_demand(copy)}
  if let Some(source)=self.source.as_ref(){return source.next_close_demand(copy)}
  if let Some(owner)=self.read_close.as_ref(){return owner.selected_demand(copy)}
  if let Some(owner)=self.output_close.as_ref(){return owner.selected_demand(copy)}
  Ok(RetirementDemand{depth:usize::from(self.read.is_some()||self.output.is_some()),..Default::default()})
 }
 pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.progress=Default::default();self.work=0;let result=self.close_original(effect).map_err(|error|match error{RetirementPollExit::Failed(error)=>{let original=error.retained_progress();if original!=Default::default(){self.progress=original}RetirementPollExit::Failed(error.with_retained_progress(self.progress))},pending=>pending});let work=self.work;RetirementPollExit::finish(result,work)}
 fn close_original(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  let grant=effect.grant();let body=grant.maximum_copy_bytes;
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original native field close was not begun").into())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=self.next_close_demands(body)?;
  if grant.maximum_items==0||effect.maximum_work_units()==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(admission)=self.admission.as_mut(){let mut returned=None;let turn=admission.return_inputs(&mut returned,effect);self.work+=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if let Some((read,()))=returned{*self.read=Some(read)}if admission.terminal_is_empty(){self.admission=None}return Ok(RetainedCloneStep::Progress(self.progress))}
  if !self.cursor.terminal_is_empty(){let before=effect.performed_work();let mut slot=None;{let reply=OriginalEffectReply::borrow(&mut slot)?;drop(if self.pending{self.cursor.recover_close_into(body,effect,reply)?}else{self.cursor.close_into(body,effect,reply)?});}self.settle(effect,before,slot.take())?;return Ok(RetainedCloneStep::Progress(self.progress))}
  if !self.projection.terminal_is_empty(){let turn=RetirementPollExit::settled(self.projection.close_step(body,effect))?;self.work+=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(source)=self.source.as_mut(){let turn=RetirementPollExit::settled(source.close_step(body,effect))?;self.work+=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if source.terminal_is_empty(){self.source=None}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(read)=self.read.take(){crate::pay_pure_work(effect,&mut self.work)?;self.read_close=Some(ControlledRetirement::new(read).unwrap_or_else(|_|unreachable!("original captured read supports controlled retirement")));self.progress=RetainedCloneProgress{copied_items:1,..Default::default()};return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(owner)=self.read_close.as_mut(){let turn=RetirementPollExit::settled(owner.step(body,effect))?;self.work+=turn.work;self.progress=owner.step_progress();let step=turn.result?;if owner.terminal_is_empty(){self.read_close=None}return Ok(RetainedCloneStep::Progress(step.progress()))}
  if let Some(output)=self.output.take(){crate::pay_pure_work(effect,&mut self.work)?;self.output_close=Some(ControlledRetirement::new(output).unwrap_or_else(|_|unreachable!("original cloned field supports controlled retirement")));self.progress=RetainedCloneProgress{copied_items:1,..Default::default()};return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(owner)=self.output_close.as_mut(){let turn=RetirementPollExit::settled(owner.step(body,effect))?;self.work+=turn.work;self.progress=owner.step_progress();let step=turn.result?;if owner.terminal_is_empty(){self.output_close=None}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.read.is_none()&&self.admission.is_none()&&self.source.is_none()&&self.projection.terminal_is_empty()&&self.cursor.terminal_is_empty()&&self.output.is_none()&&self.read_close.is_none()&&self.output_close.is_none()}
}
impl<T:RetireOwned+Sync,U:RetainedClone> Drop for SnapshotReadFieldClone<T,U>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native field clone retains its original captured read or output");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.read);ManuallyDrop::drop(&mut self.output);ManuallyDrop::drop(&mut self.cursor);ManuallyDrop::drop(&mut self.projection)}}}}

#[cfg(test)]
mod laws{
//! 🧪️ Native field cloning keeps the captured issuer and exact heap receipts across refusal and cancellation.
use crate as snapshot_read;
use crate::{SnapshotRead,SnapshotReadFieldClone,SnapshotReadRegistryHandle};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_value::ErasedSnapshotRetirement;
use std::sync::Arc;
type Root=(Vec<String>,u64);
fn field(root:&Root)->&Vec<String>{&root.0}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn observe<T>(call:impl FnOnce()->T)->(T,usize,usize){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(call);assert!(!heap.overflowed);(value,heap.requested_bytes,heap.released_bytes)}
fn receipt(p:RetainedCloneProgress,g:RetainedCloneGrant,born:usize,released:usize){assert!(p.fits(g));assert_eq!((born,released),(p.retained_capacity_bytes,p.released_bytes));}
#[test]
fn native_field_clone_retains_original_store_issuer_and_receipts(){
 let corpus:serde_json::Value=serde_json::from_str::<serde_json::Value>(include_str!("../🧪️tests/🧫️fixtures/🔣️.json")).unwrap()["fieldClone"].clone();
 for law in corpus.as_array().unwrap(){
  let values:Vec<String>=serde_json::from_value(law["field"].clone()).unwrap();let expected=values.clone();let root=Arc::new((values,73));let pointer=Arc::as_ptr(&root);
  let registry=SnapshotReadRegistryHandle::new();let generation=law["generation"].as_u64().unwrap();let revision=[law["revisionByte"].as_u64().unwrap()as u8;32];assert!(registry.publish_authority(generation,revision));
  let lease=registry.try_issue(Arc::clone(&root)).unwrap_or_else(|_|panic!("genuine Store issuer must issue original field read"));let read=SnapshotRead::new(root,lease);let identity=read.lease_identity();
  let mut work=SnapshotReadFieldClone::new(read,field,1,revision,generation);let mut output=None;

  if law["cancelAfter"].is_null(){
   let mut ready=false;for _ in 0..100{let g=grant(work.next_source_demands().unwrap());let(turn,born,released)=observe(||work.prepare_source(g));ready=turn.result.unwrap();receipt(work.step_progress(),g,born,released);if ready{break}}assert!(ready);assert!(!work.complete());
   let g=grant(semio_framework_value::retained_clone::RetainedOwnedProjection::<Vec<String>>::constructor_demand());let(turn,born,released)=observe(||work.project_original(2,field,g));let(mut projection,p)=turn.result.unwrap();receipt(p,g,born,released);assert_eq!(projection.borrow().unwrap().get()as*const _,field(work.original().unwrap().get())as*const _);
   for _ in 0..10000{if projection.terminal_is_empty(){break}let g=grant(projection.next_close_demand(4).unwrap());let(turn,born,released)=observe(||projection.close_step(g));receipt(turn.result.unwrap().progress(),g,born,released)}assert!(projection.terminal_is_empty());
  }

  for index in 0..10000{
   assert_eq!(work.original().unwrap().lease_identity(),identity);assert_eq!(work.original().unwrap().get()as*const _,pointer);
   if law["cancelAfter"].as_u64()==Some(index){break}
   let d=work.next_demands(4).unwrap();let g=grant(d);
   for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes-=1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,3 if d.release_bytes>0=>denied.maximum_release_bytes-=1,4 if d.depth>0=>denied.maximum_depth-=1,_=>continue}if semio_framework_value::retained_clone::retained_clone_grant_admits(denied,work.next_demands(denied.maximum_copy_bytes).unwrap()){continue}let(turn,born,released)=observe(||work.advance(denied));assert!(turn.result.unwrap().is_none());assert_eq!(turn.work,0);assert_eq!((born,released),(0,0));}
   let(turn,born,released)=observe(||work.advance(g));let value=turn.result.unwrap();receipt(work.step_progress(),g,born,released);
   if let Some(value)=value{assert_eq!(value,expected);output=Some(value);break}
  }
  if law["cancelAfter"].is_null(){assert!(output.is_some());}
  if let Some(output)=output{work.restore_output(output)}
  assert!(registry.publish_authority(generation+1,[0;32]));assert!(work.validate_current().is_err());work.begin_close();
  let mut closed=(RetainedCloneProgress::default(),0usize,0usize);for _ in 0..10000{if work.terminal_is_empty(){break}let g=grant(work.next_close_demands(4).unwrap());let(turn,born,released)=observe(||work.close_step(g));let step=turn.result.unwrap();receipt(step.progress(),g,born,released);closed=(closed.0.checked_add(step.progress()).unwrap(),closed.1+born,closed.2+released);}assert_eq!((closed.0.retained_capacity_bytes,closed.0.released_bytes),(closed.1,closed.2),"dropping terminal-empty owners leaves the exact close receipt total unchanged");
  assert!(work.terminal_is_empty());drop(work);
  let mut active=None;
  for _ in 0..10000{
   if active.is_some(){finish_box(&mut active);}
   if !registry.has_returned(){break}
   let g=close_grant();
   let(turn,born,released)=observe(||registry.try_admit_one_returned::<Root,_>(g,|root,grant|semio_framework_value::retirement::arc_custody::admit_shared_retirement(root,grant,false)));
   let(owner,p)=turn.result.unwrap_or_else(|_|panic!("original returned field read must retire"));receipt(p,g,born,released);active=owner;
  }
  finish_box(&mut active);assert!(!registry.has_returned());finish_registry(registry);
 }
}

fn close_grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:4096}}
fn finish_box(owner:&mut Option<Box<dyn ErasedSnapshotRetirement>>){for _ in 0..10000{if owner.is_none(){return}let d=snapshot_read::artifact_retirement_box_demands(owner.as_ref().unwrap(),4096).unwrap();let g=grant(d);let(turn,born,released)=observe(||snapshot_read::artifact_retirement_box_close_step(owner,g));receipt(turn.result.unwrap().progress(),g,born,released)}panic!("original field test box did not close")}
fn finish_registry(registry:SnapshotReadRegistryHandle){let mut owner=Some(snapshot_read::SnapshotReadRegistryAliasRetirement::new(registry));for _ in 0..10000{if owner.is_none(){return}let d=snapshot_read::snapshot_registry_alias_demands(&owner).unwrap();let g=grant(d);let(step,born,released)=observe(||snapshot_read::snapshot_registry_alias_close_step(&mut owner,g).unwrap());receipt(step.progress(),g,born,released)}panic!("original field test issuer did not close")}
}
