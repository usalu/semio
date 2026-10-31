//! 🎟️ Original actor and transaction custody outlives every denied ToolRun publication.
use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};
#[cfg(test)]
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_close};
use semio_framework_value::{RetirementDemand, RetainedWorkOutcome, ValueError, ValueRefusalKind};
use semio_framework_plugin_mounted_owner::actor_capture::ActorCapture;
use semio_framework_ui_scene::SceneDoc;
use crate::{ToolRunMemberEmissionOwner, ToolRunMemberOwners, tool_grant_funds};
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_ui_contract::*;
use semio_framework_value::SharedUtf8;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::{size_of, ManuallyDrop};
use protocol::HybridLogicalTimestamp;
pub struct ToolRunPublicationIngress{pub actor_pending:ActorCapture,pub actor:Option<ControlledRetirement<SharedUtf8>>,pub transaction:ManuallyDrop<Option<protocol::TransactionRef>>,transaction_close:Option<ControlledRetirement<protocol::TransactionRef>>}
impl Default for ToolRunPublicationIngress{fn default()->Self{Self{actor_pending:ActorCapture::new(),actor:None,transaction:ManuallyDrop::new(None),transaction_close:None}}}
fn original_control_demand<T:RetireOwned>(owner:&ControlledRetirement<T>,body:usize)->Result<RetirementDemand,ValueError>{if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<T>>>(),depth:1,..Default::default()});}Ok({let mut demand=owner.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original ToolRun ingress child depth overflow"))?;demand})}
fn close_original_control<T:RetireOwned>(slot:&mut Option<ControlledRetirement<T>>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{
 let owner=slot.as_mut().unwrap();if owner.terminal_is_empty(){drop(slot.take());return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
 let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let step=owner.step(child);let work=owner.last_turn_work();let result=step.and_then(|step|admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original ToolRun publication ingress")).map(|step|RetainedCloneStep::Progress(step.progress()));RetainedWorkOutcome{result,work}
}

impl ToolRunPublicationIngress{
 pub fn terminal_is_empty(&self)->bool{self.actor_pending.terminal_is_empty()&&self.actor.is_none()&&self.transaction.is_none()&&self.transaction_close.is_none()}
 pub fn actor_ready(&self)->bool{self.actor.is_some()&&self.actor_pending.terminal_is_empty()}
 pub fn admit_actor(&mut self,actor:&str,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{self.actor_pending.advance(actor,&mut self.actor,grant)}
 pub fn admit_transaction(&mut self,actor:&str,clock:&HybridLogicalTimestamp,app:&str,tool:&str,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
  if self.transaction.is_some()||self.transaction_close.is_some(){return Ok(None);}
  let placement=size_of::<Option<protocol::TransactionRef>>();if grant.maximum_copy_bytes<placement{return Ok(None);}
  let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-placement,..grant};
  let Some((transaction,mut progress))=::replication::mutation::transaction_admission::admit_tool_transaction(actor,clock,app,tool,child)?else{return Ok(None)};
  *self.transaction=Some(transaction);progress.copied_bytes+=placement;Ok(Some(progress))
 }
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.actor_pending.terminal_is_empty(){return Ok(self.actor_pending.close_demand())}
  if let Some(owner)=&self.actor{return original_control_demand(owner,body);}
  if let Some(owner)=&self.transaction_close{return original_control_demand(owner,body);}
  if self.transaction.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<protocol::TransactionRef>>()+size_of::<Option<ControlledRetirement<protocol::TransactionRef>>>(),depth:1,..Default::default()});}
  Ok(Default::default())
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{
  if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  let demand=match self.close_demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if !tool_grant_funds(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if !self.actor_pending.terminal_is_empty(){return self.actor_pending.close_step(grant)}
  if self.actor.is_some(){return close_original_control(&mut self.actor,grant)}
  if self.transaction_close.is_some(){return close_original_control(&mut self.transaction_close,grant)}
  self.transaction_close=Some(ControlledRetirement::retain(self.transaction.take().unwrap()));
  RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }

}
impl Drop for ToolRunPublicationIngress{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original ToolRun publication ingress requires granted terminal closure");}}
#[doc(hidden)]
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
pub mod tests;
