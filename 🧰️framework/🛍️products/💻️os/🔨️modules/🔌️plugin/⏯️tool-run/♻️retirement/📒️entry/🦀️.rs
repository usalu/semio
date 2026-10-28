//! 📒️ A removed run retains its original typed sources and native presentation until funded physical closure.
use super::*;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::size_of;

struct Metadata{tool_id:String,actor:semio_framework_value::SharedUtf8,definition:ToolRunDefinition,settings_values:ToolRunSettingsValues,window:Option<(String,String)>,window_bodies:Vec<String>,payload:Option<Arc<Vec<u8>>>,trace_keys:ToolRunTraceKeys,entity_marks:Vec<(u32,u64)>,entities:Arc<BTreeSet<u64>>,trace:ToolRunTraceStore,recent_trace:std::collections::VecDeque<u64>,counters:Vec<ToolRunCounter>,steps:ToolRunStepRing,checkpoint:Option<Vec<u8>>,announced:Option<(ToolRunState,u64,String)>,port:ToolRunJobPort}
semio_framework_value::artifact_retire_struct!(Metadata{tool_id,actor,definition,settings_values,window,window_bodies,payload,trace_keys,entity_marks,entities,trace,recent_trace,counters,steps,checkpoint,announced,port});
pub(super) struct ToolRunEntryRetirement<A:ArtifactApp>{operations:crate::original_operations::OriginalToolRunOperations<A::Snapshot,A::Mutation>,native:crate::original_native_tick::OriginalToolRunNativeState,job:Option<ToolRunJobSlot<A::Config>>,finalize:Option<ToolRunFinalize<A>>,aliases:[Option<Arc<A::Snapshot>>;2],displaced:Vec<Arc<A::Snapshot>>,running:Option<A::Snapshot>,provisional:Vec<A::Mutation>,operation:Option<A::Mutation>,member:Option<ToolRunMemberRun>,retirement:Option<Box<dyn store::ErasedSnapshotRetirement>>,metadata:Option<ControlledRetirement<Metadata>>}
fn scalar<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
impl<A:ArtifactApp> ToolRunEntryRetirement<A>{
 pub(super) fn capture_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>()+size_of::<ToolRunEntry<A>>(),capacity_bytes:size_of::<Self>(),release_bytes:size_of::<ToolRunEntry<A>>(),depth:1,..Default::default()}}
 pub(super) fn capture(entry:Box<ToolRunEntry<A>>)->Box<Self>{
  let ToolRunEntry{retired_next,operations,native,tool_id,actor,definition,slot:_,identity:_,base,base_generation:_,settings_generation:_,settings_values,window,window_bodies,payload,trace_keys,provisional,entity_marks,entities,overlay,refold,trace,recent_trace,stage:_,completed:_,total:_,counters,steps,step_sequence:_,sequence:_,conflicts:_,rate_origin:_,units_per_second:_,job,checkpoint,pending_step:_,finalize,announced,port,displaced,member}=*entry;
  assert!(retired_next.is_none(),"original entry chain is detached before paid capture");
  let metadata=Metadata{tool_id,actor,definition,settings_values,window,window_bodies,payload,trace_keys,entity_marks,entities,trace,recent_trace,counters,steps,checkpoint,announced,port};
  Box::write(Box::new_uninit(),Self{operations,native,job,finalize,aliases:[Some(base),Some(overlay)],displaced,running:refold.and_then(|refold|refold.running),provisional,operation:None,member,retirement:None,metadata:Some(ControlledRetirement::new(metadata).unwrap_or_else(|_|unreachable!("native run metadata declares every retirement field")))})
 }
 pub(super) fn demand(&self,store:&ArtifactStore<A::Snapshot,A::Mutation>,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.native.terminal_is_empty(){return self.native.next_close_demands(body)}
  if !self.operations.terminal_is_empty(){return self.operations.close_demands(store,body)}
  if let Some(job)=self.job.as_ref(){return if job.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ToolRunJobSlot<A::Config>>>(),release_bytes:job.scaffold_release_bytes(),depth:1,..Default::default()})}else{job.next_demands(body)}}
  if let Some(finalize)=self.finalize.as_ref(){
   if let Some(publication)=finalize.publication.as_ref(){return if publication.terminal_is_empty(){Ok(scalar::<Option<store::os_store::one_item_publication::ArtifactStoreBatchPublication<A::Snapshot,A::Mutation>>>())}else{publication.retirement_demands(body)}}
   if !finalize.ingress.terminal_is_empty(){return finalize.ingress.close_demands(body)}
   return Ok(scalar::<Option<ToolRunFinalize<A>>>());
  }
  if self.retirement.is_some(){return tool_erased_retirement_demand(&self.retirement,body)}
  if self.aliases.iter().any(Option::is_some){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Arc<A::Snapshot>>>(),capacity_bytes:semio_framework_value::factory::FactorySharedRetirement::<A::Snapshot>::constructor_capacity_bytes(),depth:2,..Default::default()})}
  if !self.displaced.is_empty(){return Ok(scalar::<[Option<Arc<A::Snapshot>>;2]>())}
  if self.running.is_some(){store.owned_snapshot_retirement_factory()?;return Ok(RetirementDemand{capacity_bytes:semio_framework_value::FactoryOwnedRetirement::<A::Snapshot>::constructor_capacity_bytes(),depth:1,..Default::default()})}
  if self.operation.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<A::Mutation>>(),capacity_bytes:store.owned_mutation_retirement_birth_bytes()?,depth:2,..Default::default()})}
  if !self.provisional.is_empty(){return Ok(scalar::<(Option<A::Mutation>,Vec<A::Mutation>)>())}
  if let Some(d)=ledger_empty_backing_demand(&self.displaced)?{return Ok(d)}
  if let Some(d)=ledger_empty_backing_demand(&self.provisional)?{return Ok(d)}
  if self.member.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original composed run member retains its native close authority"))}
  if let Some(metadata)=self.metadata.as_ref(){return if metadata.terminal_is_empty(){Ok(scalar::<Option<ControlledRetirement<Metadata>>>())}else{metadata.selected_demand(body)}}
  Ok(Default::default())
 }
 pub(super) fn step(&mut self,store:&mut ArtifactStore<A::Snapshot,A::Mutation>,cx:&mut semio_framework_job::StepContext<'_>)->Result<RetainedCloneStep,semio_framework_job::OriginalTurnFailure>{
  let d=self.demand(store,cx.retained_grant().maximum_copy_bytes)?;
  if !self.native.terminal_is_empty(){return semio_framework_job::close_original_retained_step(cx,d,|grant|self.native.close_step(grant)).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if !self.operations.terminal_is_empty(){let before=cx.retained_progress();self.operations.close_step(store,cx)?;let after=cx.retained_progress();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}))}
  if let Some(job)=self.job.as_mut().filter(|job|!job.terminal_is_empty()){job.begin_close();return job.close_step(cx)}
  let step=semio_framework_job::close_original_retained_step(cx,d,|grant|{
   if self.job.is_some(){self.job=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))}
   if let Some(finalize)=self.finalize.as_mut(){
    if let Some(publication)=finalize.publication.as_mut(){if publication.terminal_is_empty(){finalize.publication=None;}else{publication.begin_close();return publication.close_step(grant)}}
    else if !finalize.ingress.terminal_is_empty(){return finalize.ingress.close_step(grant)}else{self.finalize=None;}
    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
   }
   if self.retirement.is_some(){return close_tool_erased_retirement(&mut self.retirement,grant)}
   if let Some(alias)=self.aliases.iter_mut().find(|alias|alias.is_some()){return store.retire_snapshot_alias(alias,grant).map(|result|match result{Some((owner,p))=>{self.retirement=Some(owner);RetainedCloneStep::Progress(p)},None=>RetainedCloneStep::Progress(Default::default())})}
   if let Some(alias)=self.displaced.pop(){self.aliases[0]=Some(alias);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
   if self.running.is_some(){let factory=store.owned_snapshot_retirement_factory()?;return semio_framework_value::FactoryOwnedRetirement::<A::Snapshot>::admit_original(&mut self.running,factory,grant).map(|result|match result{Some((owner,p))=>{self.retirement=Some(owner);RetainedCloneStep::Progress(p)},None=>RetainedCloneStep::Progress(Default::default())})}
   if self.operation.is_some(){return store.retire_owned_mutation(&mut self.operation,grant).map(|result|match result{Some((owner,p))=>{self.retirement=Some(owner);RetainedCloneStep::Progress(p)},None=>RetainedCloneStep::Progress(Default::default())})}
   if let Some(operation)=self.provisional.pop(){self.operation=Some(operation);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
   if let Some(step)=close_ledger_backing(&mut self.displaced,grant)?{return Ok(step)}if let Some(step)=close_ledger_backing(&mut self.provisional,grant)?{return Ok(step)}
   if let Some(metadata)=self.metadata.as_mut(){if !metadata.terminal_is_empty(){return metadata.step(grant)}self.metadata=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
   Ok(RetainedCloneStep::Complete(Default::default()))
  })?;Ok(RetainedCloneStep::Progress(step.progress()))
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.native.terminal_is_empty()&&self.operations.terminal_is_empty()&&self.job.is_none()&&self.finalize.is_none()&&self.aliases.iter().all(Option::is_none)&&ledger_backing_is_empty(&self.displaced)&&self.running.is_none()&&ledger_backing_is_empty(&self.provisional)&&self.operation.is_none()&&self.member.is_none()&&self.retirement.is_none()&&self.metadata.is_none()}
}
impl<A:ArtifactApp> Drop for ToolRunEntryRetirement<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original run entry must retire every typed source and presentation");}}
