//! 🧬️ Window preview retains one original press projection and the installed semantic producer across granted turns.
use super::*;
use semio_framework_value::retained_clone::{RetainedCloneRef,RetainedOwnedProjection,RetainedOwnedProjectionAdmission};
use semio_framework_value::{OriginalRetirementControl,OriginalRetirementFailure,ReplyStash,RetainedAllocationEffect,RetirementPollExit as Exit};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativePreparedDecodeControl};
use store::ErasedSnapshotRetirement as _;
use crate::store_owners::WindowConfigStore;
use store::os_store::{ArtifactDerivedOperation,ArtifactDerivedOperationMode,snapshot_read::{ArtifactDerivedSnapshot,admit_original_snapshot_alias}};
/// 🧳️ Original native preview state keeps source and displaced owners until exact closure.
pub struct WindowSemanticPreview<O:WindowConfigOwner>{
 pub base:Option<ArtifactDerivedSnapshot<O::State>>,pub input:Option<RetainedOwnedProjection<O::Mutation>>,pub producer:Option<ArtifactDerivedOperation<O::State,O::Mutation>>,pub displaced:Option<ArtifactDerivedSnapshot<O::State>>,pub projecting:RetainedOwnedProjectionAdmission<O::Mutation>,pub reply:ReplyStash<ValueError>,pub take_reply:ReplyStash<ValueError>,pub active:Option<Box<dyn store::ErasedSnapshotRetirement>>,
 pub snapshots:Option<Arc<dyn store::ArtifactOwnedValueRetirementFactory<O::State>>>,pub mutations:Option<Arc<dyn store::ArtifactOwnedValueRetirementFactory<O::Mutation>>>,pub factory_close:[Option<semio_framework_value::FactoryAuthority>;2],pub applied:bool,pub cancelled:bool,
}
fn preview_funds(g:RetainedCloneGrant,d:RetirementDemand)->bool{semio_framework_value::retained_clone::retained_clone_grant_admits(g,d)}
impl<O:WindowConfigOwner> WindowSemanticPreview<O>{
 pub fn birth_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>()+size_of::<ArtifactDerivedSnapshot<O::State>>()+2*size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(),depth:1,..Default::default()}}
 pub fn begin(store:&WindowConfigStore<O>,g:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{let d=Self::birth_demand();if !preview_funds(g,d){return Ok(None);}Ok(Some((Self{base:Some(store.derived_snapshot_head()),input:None,producer:None,displaced:None,projecting:Default::default(),reply:Default::default(),take_reply:Default::default(),active:None,snapshots:Some(Arc::clone(store.owned_snapshot_retirement_factory()?)),mutations:Some(Arc::clone(store.owned_mutation_retirement_factory()?)),factory_close:[None,None],applied:false,cancelled:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 fn typed(original:RetainedCloneRef<'_,WindowConfigMutation>)->Result<RetainedCloneRef<'_,O::Mutation>,ValueError>{
  original.try_project(1,|original|{if original.window_kind_id!=O::WINDOW_KIND_ID{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"window preview requires its actual typed original press operation"))}original.mutation.as_any().downcast_ref::<O::Mutation>().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"window preview requires its actual typed original press operation"))})
 }
 pub fn input_demand(&self,input:&RetainedOwnedProjection<WindowConfigMutation>)->Result<RetirementDemand,ValueError>{
  if self.projecting.terminal_is_empty(){return self.projecting.demand(Self::typed(input.borrow()?)?)}
  let mut d=self.projecting.take_demand()?;d.copy_bytes+=size_of::<bool>();Ok(d)
 }
 pub fn input_work_demand(&self,input:&RetainedOwnedProjection<WindowConfigMutation>)->Result<u64,ValueError>{
  if self.projecting.terminal_is_empty(){self.projecting.work_demand(Self::typed(input.borrow()?)?)}else{self.projecting.take_work_demand()}
 }
 pub fn admit_input(&mut self,input:&RetainedOwnedProjection<WindowConfigMutation>,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneProgress>,ValueError>{
  if self.cancelled||self.input.is_some()||self.producer.is_some()||self.displaced.is_some()||self.active.is_some(){return Ok(None);}
  let d=self.input_demand(input)?;if !preview_funds(effect.grant(),d)||effect.remaining_work_units()?<self.input_work_demand(input)?{return Ok(None);}
  let admission=&mut self.projecting;let taking=!admission.terminal_is_empty();
  let answer=if taking{let stash=&mut self.take_reply;let output=&mut self.input;stash.run(|reply,recover|(if recover{admission.recover_take(reply)}else{admission.take(output,effect,reply)}).map(drop))?}else{let source=Self::typed(input.borrow()?)?;let stash=&mut self.reply;stash.run(|reply,recover|(if recover{admission.recover_admit(reply)}else{admission.admit(source,effect,reply)}).map(drop))?};
  let Some(answer)=answer else{return Ok(Some(Default::default()))};let mut progress=answer?.progress();
  if taking&&self.input.is_some(){self.applied=false;progress.copied_bytes+=size_of::<bool>();}
  Ok(Some(progress))
 }
 pub fn cancel(&mut self){self.cancelled=true;if let Some(owner)=self.producer.as_mut(){owner.cancel();}}
 pub fn needs_cancellation(&self)->bool{!self.cancelled}
 pub fn input_held(&self)->bool{self.cancelled||self.input.is_some()||self.producer.is_some()||self.displaced.is_some()||self.active.is_some()}
 pub fn work_pending(&self)->bool{self.cancelled||!self.projecting.terminal_is_empty()||self.input.is_some()||self.producer.is_some()||self.displaced.is_some()||self.active.is_some()}
 pub fn leaf_ready(&self)->bool{self.applied&&self.input.is_none()&&self.producer.is_none()&&self.displaced.is_none()&&self.active.is_none()}
 pub fn demand(&self,store:&WindowConfigStore<O>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{self.demand_phase(store,body,Some(maximum_work_units))}
 pub fn close_demand(&self,store:&WindowConfigStore<O>,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.cancelled{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"window preview cleanup requires its actual cancellation phase"))}self.demand_phase(store,body,None)
 }
 fn demand_phase(&self,store:&WindowConfigStore<O>,body:usize,maximum_work_units:Option<u64>)->Result<RetirementDemand,ValueError>{
  if let Some(active)=self.active.as_ref(){return semio_framework_value::factory_ticket_demands(active,body);}
  if let Some(owner)=self.producer.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ArtifactDerivedOperation<O::State,O::Mutation>>>(),depth:1,..Default::default()})}else if owner.is_closing(){owner.next_demand(body)}else if owner.is_finished(){Ok(RetirementDemand{copy_bytes:size_of::<store::os_store::ArtifactReplayPrepared<O::State,O::Mutation>>(),depth:1,..Default::default()})}else{store.derived_operation_demands(owner,body,maximum_work_units.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"active window semantics require their actual remaining work"))?)};}
   if self.cancelled&&!self.projecting.terminal_is_empty(){return self.projecting.close_demand(body);}
 if self.displaced.is_some()||(self.cancelled&&self.base.is_some()){return Ok(store::os_store::snapshot_read::original_snapshot_alias_retirement_demand::<O::State>());}
  if let Some(input)=self.input.as_ref(){return if self.cancelled{if input.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<RetainedOwnedProjection<O::Mutation>>>(),depth:1,..Default::default()})}else{input.close_demand(body)}}else{store.derived_operation_birth_demand(self.base.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"window preview original base absent"))?)};}
  if self.cancelled{if self.snapshots.is_some()||self.mutations.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(),depth:1,..Default::default()});}if let Some(factory)=self.factory_close.iter().find_map(Option::as_ref){return factory.demands(body);}}
  Ok(Default::default())
 }
 pub fn close_step(&mut self,store:&WindowConfigStore<O>,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
  if !self.cancelled{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"window preview cleanup requires its actual cancellation phase"))}
  let body=effect.grant().maximum_copy_bytes;Self::folded(self.step_effect(store,body,None,effect,None,None))
 }
 pub fn close_controlled(&mut self,store:&WindowConfigStore<O>,body:usize,effect:&mut RetainedAllocationEffect<'_>,control:&mut dyn OriginalRetirementControl)->Result<RetainedCloneStep,ValueError>{
  if !self.cancelled{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"window preview cleanup requires its actual cancellation phase"))}
  Self::folded(self.step_effect(store,body,None,effect,None,Some(control)))
 }
 pub fn advance(&mut self,store:&WindowConfigStore<O>,body:usize,g:RetainedCloneGrant,receiving:&mut NativeDecodeControl<'_>)->Result<RetainedCloneStep,ValueError>{
  let maximum=receiving.available_work_units()?;let d=self.demand_phase(store,body,Some(maximum))?;if !preview_funds(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(owner)=self.producer.as_mut().filter(|owner|!owner.terminal_is_empty()&&!owner.is_closing()&&!owner.is_finished()){return store.step_derived_operation(owner,body,g,receiving,&mut||false).map_err(OriginalRetirementFailure::into_value_error);}
  let this=&mut *self;
  match receiving.with_allocated_effect(d,maximum,|native,effect|this.step_effect(store,body,Some(maximum),effect,Some(native),None))?{Some(turn)=>Self::folded(turn),None=>Ok(RetainedCloneStep::Progress(Default::default()))}
 }
 fn folded(turn:Result<RetainedCloneStep,Exit>)->Result<RetainedCloneStep,ValueError>{match turn{Ok(step)=>Ok(step),Err(Exit::Pending(_))=>Ok(RetainedCloneStep::Progress(Default::default())),Err(Exit::Failed(failure))=>Err(failure.into_value_error())}}
 fn step_effect(&mut self,store:&WindowConfigStore<O>,body:usize,maximum_work_units:Option<u64>,effect:&mut RetainedAllocationEffect<'_>,mut receiving:Option<&mut NativeDecodeControl<'_>>,mut control:Option<&mut dyn OriginalRetirementControl>)->Result<RetainedCloneStep,Exit>{
  let g=effect.grant();let d=self.demand_phase(store,body,maximum_work_units)?;if !preview_funds(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let moved=||RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()});
  if self.active.is_some(){return Exit::resume(match receiving.as_deref_mut(){Some(receiving)=>receiving.with_retirement(g,|control|Ok::<_,ValueError>(semio_framework_value::close_factory_ticket_controlled(&mut self.active,body,effect,control)))?,None=>match control.as_deref_mut(){Some(control)=>semio_framework_value::close_factory_ticket_controlled(&mut self.active,body,effect,control),None=>semio_framework_value::close_factory_ticket(&mut self.active,body,effect)}});}
  if let Some(owner)=self.producer.as_mut(){
   if owner.terminal_is_empty(){self.producer=None;Exit::pay(effect,1)?;return Ok(moved());}
   if owner.is_closing(){return Exit::resume(match receiving.as_deref_mut(){Some(receiving)=>receiving.with_retirement(g,|control|Ok::<_,ValueError>(owner.close_step_controlled(body,effect,control)))?,None=>match control.as_deref_mut(){Some(control)=>owner.close_step_controlled(body,effect,control),None=>owner.close_step(body,effect)}});}
   if owner.is_finished(){let Some(((next,messages),p))=store.take_derived_operation(owner,g)?else{return Ok(RetainedCloneStep::Progress(Default::default()));};assert!(messages.is_empty()&&messages.capacity()==0,"provisional semantic producer closes its diagnostics before preview transfer");if let Some(next)=next{self.displaced=self.base.replace(next);}self.applied=true;Exit::pay(effect,1)?;return Ok(RetainedCloneStep::Progress(p));}
   return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"window semantic producer requires its original receiving authority").into());
  }
  if self.cancelled&&!self.projecting.terminal_is_empty(){return Exit::resume(self.projecting.close_step(body,effect)).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if d.capacity_bytes!=0{if let Some(control)=control.as_deref_mut(){control.charge(d.capacity_bytes)?;}}
  let original=if self.displaced.is_some(){&mut self.displaced}else if self.cancelled&&self.base.is_some(){&mut self.base}else{
   if let Some(input)=self.input.as_mut(){
    if self.cancelled{if input.terminal_is_empty(){self.input=None;Exit::pay(effect,1)?;return Ok(moved());}return Exit::resume(input.close_step(body,effect));}
    let p=store.begin_derived_operation(&mut self.producer,self.base.as_ref().unwrap(),&mut self.input,0,ArtifactDerivedOperationMode::Provisional,g)?.unwrap_or_default();Exit::pay(effect,1)?;return Ok(RetainedCloneStep::Progress(p));
   }
   if self.cancelled{
    if let Some(factory)=self.snapshots.take(){self.factory_close[0]=Some(semio_framework_value::FactoryAuthority::new(factory));Exit::pay(effect,1)?;return Ok(moved());}
    if let Some(factory)=self.mutations.take(){self.factory_close[1]=Some(semio_framework_value::FactoryAuthority::new(factory));Exit::pay(effect,1)?;return Ok(moved());}
    if let Some(index)=self.factory_close.iter().position(Option::is_some){let owner=self.factory_close[index].as_mut().unwrap();let step=Exit::resume(owner.step(body,effect))?;if owner.terminal_is_empty(){self.factory_close[index]=None;}return Ok(step);}
   }
   return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(Default::default())}else{RetainedCloneStep::Progress(Default::default())});
  };
  let Some((owner,p))=admit_original_snapshot_alias(original,self.snapshots.as_ref().unwrap(),g)?else{return Ok(RetainedCloneStep::Progress(Default::default()));};
  *effect.actual()=Some(p.retained_capacity_bytes);self.active=Some(owner);Exit::pay(effect,1)?;Ok(RetainedCloneStep::Progress(p))
 }
 pub fn take_base(&mut self,g:RetainedCloneGrant)->Result<Option<(ArtifactDerivedSnapshot<O::State>,RetainedCloneProgress)>,ValueError>{if !self.leaf_ready()||self.cancelled{return Ok(None);}let d=RetirementDemand{copy_bytes:size_of::<ArtifactDerivedSnapshot<O::State>>(),depth:1,..Default::default()};if !preview_funds(g,d){return Ok(None);}let Some(base)=self.base.take()else{return Ok(None)};self.cancelled=true;Ok(Some((base,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn terminal_is_empty(&self)->bool{self.cancelled&&self.base.is_none()&&self.input.is_none()&&self.projecting.terminal_is_empty()&&self.producer.is_none()&&self.displaced.is_none()&&self.active.is_none()&&self.snapshots.is_none()&&self.mutations.is_none()&&self.factory_close.iter().all(Option::is_none)}
}
impl<O:WindowConfigOwner> Drop for WindowSemanticPreview<O>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"window semantic preview dropped original owners before paid closure");}}

