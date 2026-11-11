//! ⏪️ Original actor retirement quotes and admits each retained physical frontier independently.
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework::{ActionArgDef, ArgSchema, FaultFrom};
use semio_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(test)]
use semio_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use store::{ArtifactStore, Mutation, MutationDiff};
use semio_framework_value::DslValue;
use semio_plugin_decl_emission::Emit;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_ui_locale::LocalizedLabel;
use protocol::{OpBinary, OpText};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_time_travel::{TimeTravelBase, TimeTravelEvent, TimeTravelSession, TimeTravelStage, TimeTravelTarget};
use crate::{TimeTravelEditor, TimeTravelLedger, TimeTravelMemberMetadata, TimeTravelOwners, TimeTravelStoreState};
#[cfg(test)]
use semio_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_plugin_window_config::WindowConfigOwner;
use semio_ui_contract::*;
use store::ErasedSnapshotRetirement;
use semio_framework_value::{RetainedAllocationEffect,RetirementDemand, ValueError, ValueRefusalKind, FactoryAuthority, FactoryOwnedRetirement, close_factory_ticket, factory_ticket_demands};
use std::mem::{size_of, size_of_val};
fn lifted(poll:semio_framework_value::RetainedWorkPoll<RetainedCloneStep,ValueError>)->Result<RetainedCloneStep,ValueError>{settle_value_poll(poll)}
fn original_refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,message)}
#[doc(hidden)]
pub fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"actor retirement depth overflow"))?;Ok(demand)}
#[doc(hidden)]
pub fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn settle_value_poll(poll:semio_framework_value::RetainedWorkPoll)->Result<RetainedCloneStep,ValueError>{match poll{semio_framework_value::RetainedWorkPoll::Pending{..}=>Ok(RetainedCloneStep::Progress(Default::default())),semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result}}
#[doc(hidden)]
pub fn progress(step:RetainedCloneStep)->RetainedCloneStep{RetainedCloneStep::Progress(step.progress())}
impl<A:ArtifactApp> TimeTravelLedger<A> {
 fn session_transfer_copy_bytes()->usize{size_of::<TimeTravelSession>()*3+size_of::<ControlledRetirement<TimeTravelSession>>()*2+size_of::<Option<ControlledRetirement<TimeTravelSession>>>()}
 fn session_removal_copy_bytes()->usize{size_of::<Option<ControlledRetirement<TimeTravelSession>>>()*2+size_of::<bool>()}
 pub(super) fn pending_command_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Ok(Default::default())}
  let Some(owner)=self.pending_command.as_ref()else{return Ok(Default::default())};
  if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}
  nested(owner.retirement_demands(body)?)
 }
 pub(super) fn pending_command_retirement_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneStep>,ValueError>{
  let grant=effect.grant();
  if !self.closing||self.pending_command.is_none(){return Ok(None)}
  let demand=self.pending_command_retirement_demands(grant.maximum_copy_bytes)?;
  if !permits(grant,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
  let owner=self.pending_command.as_mut().unwrap();
  if owner.terminal_is_empty(){drop(self.pending_command.take());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))}
  let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=store::os_store::snapshot_read::settle_close_poll(owner.close_step(&mut effect.nested(child)))?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"actor original received command")?;Ok(Some(progress(step)))
 }
 pub(super) fn session_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.session_retirement.is_none()&&(!self.closing||self.session_retired){return Ok(Default::default());}
  let Some(owner)=self.session_retirement.as_ref()else{return Ok(RetirementDemand{copy_bytes:Self::session_transfer_copy_bytes(),depth:1,..Default::default()});};
  if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:Self::session_removal_copy_bytes(),depth:1,..Default::default()});}
  nested(owner.next_demand(body)?)
 }
 pub(super) fn session_retirement_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneStep>,ValueError>{
  let grant=effect.grant();
  if self.session_retirement.is_none()&&(!self.closing||self.session_retired){return Ok(None);}
  let demand=self.session_retirement_demands(grant.maximum_copy_bytes)?;
  if !permits(grant,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
  if let Some(owner)=self.session_retirement.as_mut(){
   if owner.terminal_is_empty(){drop(self.session_retirement.take());self.session_retired=self.closing;return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})));}
   let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let step=store::os_store::snapshot_read::settle_close_poll(owner.step(grant.maximum_copy_bytes,&mut effect.nested(child)))?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original time travel session")?;return Ok(Some(progress(step)));
  }
  let original=std::mem::take(&mut self.session);
  match ControlledRetirement::new(original){Ok(owner)=>{self.session_retirement=Some(owner);Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))},Err((error,original))=>{self.session=original;Err(error)}}
 }
}

#[doc(hidden)]
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
pub mod tests;
#[cfg(test)]
pub use crate::original_retirement::tests::assert_session_original;
#[cfg(test)]
pub use crate::original_retirement::tests::assert_pending_command_original;
#[cfg(test)]
pub use crate::original_retirement::tests::assert_discard_publication_original;
#[cfg(test)]
pub use crate::original_retirement::tests::native_original_actor_grant;
impl<P,Mu> TimeTravelStoreState<P,Mu> where P: ::protocol::ArtifactPack + ::semio_os_kernel::os_store::serialization::ArtifactStoreSnapshot + ::semio_os_kernel::StoreState,Mu: ::semio_os_kernel::StoreMutationFor<P> {
 pub(crate) fn original_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(demand)=self.discard_effects_retirement_demands(body)?{return Ok(demand)}
  if let Some(owner)=self.preview_derived.as_ref().filter(|owner|owner.is_closing()){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<store::os_store::ArtifactDerivedOperation<P,Mu>>>(),depth:1,..Default::default()})}else{owner.next_demand(body)};}
  if self.preview_cancelled{if let Some(input)=self.preview_native_input.as_ref(){return if input.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value::retained_clone::RetainedOwnedProjection<Mu>>>(),depth:1,..Default::default()})}else{input.close_demand(body)};}if let Some(source)=self.preview_source.as_ref(){return if source.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<store::os_store::ArtifactMutationSource<Mu>>>(),depth:1,..Default::default()})}else{source.close_demand(body)};}}
  if let Some(owner)=self.preview_input_close.as_ref().or(self.preview_original_close.as_ref()){return factory_ticket_demands(owner,body);}
  if self.preview_cancelled&&self.preview_original.is_some(){return Ok(FactoryOwnedRetirement::<Mu>::original_admission_demands(&self.preview_original));}
  if self.preview_cancelled&&self.preview_input.is_some(){return Ok(RetirementDemand{capacity_bytes:size_of::<ControlledRetirement<protocol::InputReplacement>>(),depth:1,..Default::default()});}
  if self.preview_cancelled&&self.preview_base.is_some(){return Ok(store::os_store::snapshot_read::original_snapshot_alias_retirement_demand::<P>());}
  if let Some(owner)=self.snapshot_close.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(owner,body);}
  if self.snapshot_aliases.iter().any(Option::is_some){self.snapshot_issuers.as_ref().ok_or_else(||original_refusal("derived snapshot retains its original missing issuers"))?;return {let mut d=store::os_store::snapshot_read::original_snapshot_alias_retirement_demand::<P>();d.depth+=1;Ok(d)};}
  if self.snapshot_issuers.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
  if let Some(owner)=self.snapshot_issuer_close.iter().flatten().next(){return nested(owner.demands(body)?);}
  if let Some(owner)=self.retirements.back(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of_val(owner.as_ref()),depth:1,..Default::default()})}else{nested(factory_ticket_demands(owner,body)?)};}
  if self.retirements.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.retirements.capacity().checked_mul(size_of::<Box<dyn store::ErasedSnapshotRetirement>>()).ok_or_else(||original_refusal("actor original queue capacity overflow"))?,depth:1,..Default::default()});}
  if let Some(owner)=self.discarded_active.as_ref(){return nested(factory_ticket_demands(owner,body)?);}
  if self.discarded_pending.is_some(){self.discarded_factory.as_ref().ok_or_else(||original_refusal("actor original mutation retains its missing issuer"))?;let mut demand=FactoryOwnedRetirement::<Mu>::original_admission_demands(&self.discarded_pending);demand.depth+=1;return Ok(demand);}
  if !self.discarded.is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}
  if self.discarded_has_physical_capacity(){return Ok(RetirementDemand{release_bytes:self.discarded.capacity().checked_mul(size_of::<Mu>()).ok_or_else(||original_refusal("actor original mutation vector capacity overflow"))?,depth:1,..Default::default()});}
  if self.discarded_factory.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
  self.discarded_factory_close.as_ref().map_or(Ok(Default::default()),|owner|nested(owner.demands(body)?))
 }
 pub(crate) fn original_retirement_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneStep>,ValueError>{self.original_retirement_step_with_control(effect,None)}
 pub(crate) fn original_retirement_step_controlled(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<Option<RetainedCloneStep>,ValueError>{self.original_retirement_step_with_control(effect,Some(control))}
 fn original_retirement_step_with_control(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,mut control:Option<&mut dyn semio_framework_value::OriginalRetirementControl>)->Result<Option<RetainedCloneStep>,ValueError>{
  let grant=effect.grant();let body=grant.maximum_copy_bytes;
  if let Some(step)=self.discard_effects_retirement_step(effect)?{return Ok(Some(step))}
  if grant.maximum_items==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
  let demand=self.original_retirement_demands(grant.maximum_copy_bytes)?;if !permits(grant,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
  if let Some(owner)=self.preview_derived.as_mut().filter(|owner|owner.is_closing()){if owner.terminal_is_empty(){drop(self.preview_derived.take());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})));}return store::os_store::snapshot_read::settle_close_poll(match control.as_mut(){Some(control)=>owner.close_step_controlled(body,effect,*control),None=>owner.close_step(body,effect)}).map(Some);}
  if self.preview_cancelled{if let Some(input)=self.preview_native_input.as_mut(){if input.terminal_is_empty(){self.preview_native_input=None;return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})));}return store::os_store::snapshot_read::settle_close_poll(input.close_step(body,effect)).map(Some);}if let Some(source)=self.preview_source.as_mut(){source.begin_close();if source.terminal_is_empty(){self.preview_source=None;return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})));}return source.close_step(effect).map(Some);}}
  if self.preview_input_close.is_some(){return store::os_store::snapshot_read::settle_close_poll(semio_framework_value::close_factory_ticket(&mut self.preview_input_close,body,effect)).map(Some);}
  if self.preview_original_close.is_some(){return store::os_store::snapshot_read::settle_close_poll(semio_framework_value::close_factory_ticket(&mut self.preview_original_close,body,effect)).map(Some);}
  if self.preview_cancelled&&self.preview_original.is_some(){let factory=self.discarded_factory.as_ref().ok_or_else(||original_refusal("selected preview retains its original mutation cleanup issuer"))?;let mut issuer=Some(std::sync::Arc::clone(factory));return lifted(FactoryOwnedRetirement::<Mu>::admit_original(&mut self.preview_original,&mut issuer,&mut self.preview_original_close,effect)).map(|step|Some(progress(step)));}
  if self.preview_cancelled&&self.preview_input.is_some(){return Ok(Some(match semio_framework_value::retirement::admit_owned_retirement(&mut self.preview_input,effect).result?{Some((owner,p))=>{self.preview_input_close=Some(owner);RetainedCloneStep::Progress(p)},None=>RetainedCloneStep::Progress(Default::default())}));}
  if self.preview_cancelled&&self.preview_base.is_some(){let(snapshots,mutations)=self.snapshot_issuers.as_ref().ok_or_else(||original_refusal("selected preview retains its original base cleanup issuers"))?;let Some((owner,p))=store::os_store::snapshot_read::admit_original_snapshot_alias(&mut self.preview_base,snapshots,grant)?else{return Ok(Some(RetainedCloneStep::Progress(Default::default())))};*self.snapshot_close=Some(owner);return Ok(Some(RetainedCloneStep::Progress(p)));}
  if self.snapshot_close.is_some(){return store::os_store::snapshot_read::settle_close_poll(store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.snapshot_close,effect.grant().maximum_copy_bytes,effect)).map(|step|Some(progress(step)));}
  if let Some(slot)=self.snapshot_aliases.iter_mut().find(|slot|slot.is_some()){
   let(snapshots,mutations)=self.snapshot_issuers.as_ref().unwrap();
   let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
   let Some((owner,receipt))=store::os_store::snapshot_read::admit_original_snapshot_alias(slot,snapshots,child)?else{return Ok(Some(RetainedCloneStep::Progress(Default::default())))};
   *self.snapshot_close=Some(owner);return Ok(Some(RetainedCloneStep::Progress(receipt)));
  }
  if let Some((snapshots,mutations))=self.snapshot_issuers.take(){self.snapshot_issuer_close=[Some(FactoryAuthority::new(snapshots)),Some(FactoryAuthority::new(mutations))];return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})));}
  if let Some(index)=self.snapshot_issuer_close.iter().position(Option::is_some){let owner=self.snapshot_issuer_close[index].as_mut().unwrap();let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=store::os_store::snapshot_read::settle_close_poll(owner.step(body,&mut effect.nested(child)))?;if !step.progress().fits(child){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original snapshot issuer receipt exceeds original caller"))}if owner.terminal_is_empty(){drop(self.snapshot_issuer_close[index].take())}return Ok(Some(progress(step)));}
  if let Some(owner)=self.retirements.back_mut(){
   if owner.terminal_is_empty(){drop(self.retirements.pop_back());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})));}
   let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let step=store::os_store::snapshot_read::settle_close_poll(match control.as_mut(){Some(control)=>owner.close_step_controlled(body,&mut effect.nested(child),*control),None=>owner.close_step(body,&mut effect.nested(child))})?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"actor original snapshot frame")?;return Ok(Some(progress(step)));
  }
  if self.retirements.capacity()!=0{drop(std::mem::take(&mut self.retirements));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})));}
  if self.discarded_active.is_some(){return store::os_store::snapshot_read::settle_close_poll(close_factory_ticket(&mut self.discarded_active,body,&mut effect.nested(RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant}))).map(|step|Some(progress(step)));}
  if self.discarded_pending.is_some(){let mut issuer=Some(std::sync::Arc::clone(self.discarded_factory.as_ref().unwrap()));let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let step=lifted(FactoryOwnedRetirement::<Mu>::admit_original(&mut self.discarded_pending,&mut issuer,&mut self.discarded_active,&mut effect.nested(child)))?;if !step.progress().fits(child){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"actor mutation issuer exceeded its original admission"));}return Ok(Some(progress(step)));}
  if !self.discarded.is_empty(){self.discarded_pending=self.discarded.pop();return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})));}
  if self.discarded_has_physical_capacity(){drop(std::mem::take(&mut self.discarded));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})));}
  if let Some(factory)=self.discarded_factory.take(){self.discarded_factory_close=Some(FactoryAuthority::new(factory));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})));}
  if let Some(owner)=self.discarded_factory_close.as_mut(){let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let step=store::os_store::snapshot_read::settle_close_poll(owner.step(body,&mut effect.nested(child)))?;if !step.progress().fits(child){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"actor issuer exceeded its retained retirement grant"));}if owner.terminal_is_empty(){drop(self.discarded_factory_close.take());}return Ok(Some(progress(step)));}
  Ok(None)
 }
}
