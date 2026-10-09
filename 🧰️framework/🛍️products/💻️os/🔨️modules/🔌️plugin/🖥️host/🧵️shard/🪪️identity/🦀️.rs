//! 🪪️ A shard retains the original caller's receiving ledger, allocation port and return slot.
use crate::{GuestInstance,GuestRuntime,GuestRuntimes,PluginHostError,TurnFault};
use semio_framework_value::{ValueError,ValueRefusalKind,ErasedSnapshotRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},native_encoding::{NativeEncodeControl,NativeEncodeAllocation,NativeEncodeAllocationPort,NativeEncodeProgress,NativeEncodeRetirementRecipient,NativeForwardedEncodeReceipt}};
use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::{EntityIdentityAuthority,Observer};
use std::pin::Pin;

/// 🎟️ Explicit independent receiving policy authored before the shard starts.
#[derive(Clone,Copy,Debug,semio_framework_value::serde::Serialize,semio_framework_value::serde::Deserialize)]
#[serde(crate="semio_framework_value::serde",rename_all="camelCase",deny_unknown_fields)]
pub struct ShardIdentityPolicy{pub maximum_bytes:usize,pub grant:RetainedCloneGrant}

/// 🏭️ The caller transfers a new consuming identity only at original actor registration.
pub type OriginalShardIdentityIssuer=Box<dyn FnMut(semio_framework_actor::ActorId)->Result<OriginalShardIdentity,ShardIdentityIssueRefused>+Send>;

/// 🛑️ Refusal returns the caller's original ports before any actor publication.
pub struct ShardIdentityIssueRefused{pub error:ValueError,pub observer:Box<Observer<'static>>,pub allocate:Box<NativeEncodeAllocationPort<'static>>}
impl std::fmt::Debug for ShardIdentityIssueRefused{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("ShardIdentityIssueRefused").field("error",&self.error).finish_non_exhaustive()}}

/// 🧾️ Consuming custody survives actor yields and failures without creating another authority.
pub struct OriginalShardIdentity{birth:RetainedCloneProgress,policy:ShardIdentityPolicy,receipt:Option<NativeForwardedEncodeReceipt>,recipient:Pin<Box<NativeEncodeRetirementRecipient>>,observer:Box<Observer<'static>>,allocate:Box<NativeEncodeAllocationPort<'static>>}
impl OriginalShardIdentity{
 /// 🏭️ The original port admits the stable return slot before its physical birth.
 pub fn issue(policy:ShardIdentityPolicy,mut observer:Box<Observer<'static>>,mut allocate:Box<NativeEncodeAllocationPort<'static>>)->Result<Self,ShardIdentityIssueRefused>{
  let bytes=std::mem::size_of::<NativeEncodeRetirementRecipient>();
  let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_capacity_bytes:bytes,maximum_release_bytes:0,maximum_depth:1};
  if !(policy.grant.maximum_items>=demand.maximum_items&&policy.grant.maximum_copy_bytes>=demand.maximum_copy_bytes&&policy.grant.maximum_capacity_bytes>=demand.maximum_capacity_bytes&&policy.grant.maximum_release_bytes>=demand.maximum_release_bytes&&policy.grant.maximum_depth>=demand.maximum_depth){return Err(ShardIdentityIssueRefused{error:ValueError::literal(ValueRefusalKind::OwnershipLimit,"original shard receiving policy refuses its return slot"),observer,allocate})}
  let mut native=NativeEncodeControl::new_forwarded(policy.maximum_bytes,&mut *observer,&mut *allocate);
  let admitted=native.checkpoint().and_then(|_|native.charge(bytes));
  if let Err(error)=admitted{drop(native);return Err(ShardIdentityIssueRefused{error,observer,allocate})}
  let mut recipient=Box::pin(NativeEncodeRetirementRecipient::new());
  native.install_retirement_recipient(recipient.as_mut().get_mut()).expect("new original shard return slot is empty and uninstalled");
  let receipt=native.detach().unwrap_or_else(|_|panic!("new original shard receiving receipt is valid between stages"));
  Ok(Self{birth:RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:bytes,released_bytes:0},policy,receipt:Some(receipt),recipient,observer,allocate})
 }
 /// 🧾️ Reports the actual stable recipient's admitted inline copy and whole physical birth.
 pub fn birth_progress(&self)->RetainedCloneProgress{self.birth}
 /// 📏️ Borrows the original cumulative admission without copying its authority.
 pub fn owned_bytes(&self)->usize{self.receipt.as_ref().expect("original shard identity exists between loans").owned_bytes()}
 /// 🎟️ Returns the independently authored policy without inferring it from work or owned bytes.
 pub fn policy(&self)->ShardIdentityPolicy{self.policy}
 fn loan(&mut self)->Result<EntityIdentityAuthority<'_>,ValueError>{
  let receipt=self.receipt.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original shard identity is already loaned"))?;
  match receipt.bind(&mut *self.allocate,self.recipient.as_mut().get_mut()){
   Ok(continuation)=>Ok(EntityIdentityAuthority::resume_forwarded(continuation,&mut *self.observer)),
   Err((error,receipt))=>{self.receipt=Some(receipt);Err(error)}
  }
 }
 /// 📸️ Resumes the original identity through the entire asynchronous checkpoint callback.
 pub async fn checkpoint(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance)->Result<Vec<u8>,PluginHostError>{
  let mut identity=self.loan().map_err(PluginHostError::NativeIo)?;
  let result=runtime.checkpoint(instance,&mut identity).await;
  let receipt=identity.pause_forwarded().map_err(PluginHostError::NativeIo)?.detach().map_err(PluginHostError::NativeIo)?;
  self.receipt=Some(receipt);result
 }
 /// ▶️ Restores through the same receiving port, cumulative ledger and cancellation observer.
 pub async fn restore(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,state:&[u8])->Result<(),PluginHostError>{
  let mut identity=self.loan().map_err(PluginHostError::NativeIo)?;
  let result=runtime.restore(instance,state,&mut identity).await;
  let receipt=identity.pause_forwarded().map_err(PluginHostError::NativeIo)?.detach().map_err(PluginHostError::NativeIo)?;
  self.receipt=Some(receipt);result
 }
 /// 🏃️ Retains the same caller admission after successful, yielded and refused actor turns.
 pub async fn execute_turn(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,events:&[semio_framework::kernel::Event],budget:semio_framework::kernel::Budget)->Result<semio_framework::kernel::TurnResult,TurnFault>{
  let mut identity=self.loan().map_err(crate::identity_turn_fault)?;
  let result=runtime.execute_turn(instance,events,budget,&mut identity).await;
  let receipt=identity.pause_forwarded().map_err(crate::identity_turn_fault)?.detach().map_err(crate::identity_turn_fault)?;
  self.receipt=Some(receipt);result
 }
}

/// 🏭️ Creates each actor's original ports once from the root's unchanged independent policy.
pub fn native_identity_issuer(policy:ShardIdentityPolicy,mut observer_issuer:impl FnMut(semio_framework_actor::ActorId)->Box<Observer<'static>>+Send+'static)->OriginalShardIdentityIssuer{
 Box::new(move |actor|{
  let observer=observer_issuer(actor);
  let allocate=Box::new(move |request:NativeEncodeAllocation|{
   if policy.grant.maximum_items==0||policy.grant.maximum_copy_bytes<std::mem::size_of::<NativeEncodeAllocation>()||policy.grant.maximum_capacity_bytes<request.bytes||policy.grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original caller shard policy refuses physical receiving frontier"))}
   Ok(())
  });
  OriginalShardIdentity::issue(policy,observer,allocate)
 })
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
