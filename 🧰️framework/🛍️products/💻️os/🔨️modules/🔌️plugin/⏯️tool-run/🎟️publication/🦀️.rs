//! 🎟️ Original actor and transaction custody outlives every denied ToolRun publication.
use super::*;
use semio_framework_value::{SharedUtf8,retirement::{RetireOwned,controlled::ControlledRetirement}};
use std::mem::{size_of,ManuallyDrop};
pub(super) struct ToolRunPublicationIngress{pub actor:Option<ControlledRetirement<SharedUtf8>>,pub transaction:ManuallyDrop<Option<protocol::TransactionRef>>,transaction_close:Option<ControlledRetirement<protocol::TransactionRef>>}
impl Default for ToolRunPublicationIngress{fn default()->Self{Self{actor:None,transaction:ManuallyDrop::new(None),transaction_close:None}}}
fn original_control_demand<T:RetireOwned>(owner:&ControlledRetirement<T>,body:usize)->Result<RetirementDemand,ValueError>{if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<T>>>(),depth:1,..Default::default()});}Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original ToolRun ingress child depth overflow"))?})}
fn close_original_control<T:RetireOwned>(slot:&mut Option<ControlledRetirement<T>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let owner=slot.as_mut().unwrap();if owner.terminal_is_empty(){drop(slot.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<ControlledRetirement<T>>>(),..Default::default()}));}let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let step=owner.step(child)?;let step=admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original ToolRun publication ingress")?;Ok(RetainedCloneStep::Progress(step.progress()))}
impl ToolRunPublicationIngress{
 pub fn terminal_is_empty(&self)->bool{self.actor.is_none()&&self.transaction.is_none()&&self.transaction_close.is_none()}
 pub fn admit_actor(&mut self,actor:&str,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{semio_framework_plugin_mounted_owner::actor_capture::admit_actor_capture(&mut self.actor,actor,grant)}
 pub fn admit_transaction(&mut self,actor:&str,clock:&protocol::HybridLogicalTimestamp,app:&str,tool:&str,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
  if self.transaction.is_some()||self.transaction_close.is_some(){return Ok(None);}
  let placement=size_of::<Option<protocol::TransactionRef>>();if grant.maximum_copy_bytes<placement{return Ok(None);}
  let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-placement,..grant};
  let Some((transaction,mut progress))=::replication::mutation::transaction_admission::admit_tool_transaction(actor,clock,app,tool,child)?else{return Ok(None)};
  *self.transaction=Some(transaction);progress.copied_bytes+=placement;Ok(Some(progress))
 }
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(owner)=&self.actor{return original_control_demand(owner,body);}
  if let Some(owner)=&self.transaction_close{return original_control_demand(owner,body);}
  if self.transaction.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<protocol::TransactionRef>>()+size_of::<Option<ControlledRetirement<protocol::TransactionRef>>>(),depth:1,..Default::default()});}
  Ok(Default::default())
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.close_demands(grant.maximum_copy_bytes)?;if !tool_grant_funds(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.actor.is_some(){return close_original_control(&mut self.actor,grant);}
  if self.transaction_close.is_some(){return close_original_control(&mut self.transaction_close,grant);}
  self.transaction_close=Some(ControlledRetirement::new(self.transaction.take().unwrap()).unwrap_or_else(|_|unreachable!("original TransactionRef owns its native retirement issuer")));
  Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
 }
}
impl Drop for ToolRunPublicationIngress{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original ToolRun publication ingress requires granted terminal closure");}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
