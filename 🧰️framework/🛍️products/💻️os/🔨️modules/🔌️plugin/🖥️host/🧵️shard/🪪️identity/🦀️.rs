//! 🪪️ A shard retains the original caller's receiving ledger, allocation port and return slot.
use semio_plugin_host_guest_runtime::{GuestInstance, GuestRuntime, GuestRuntimes};
use semio_plugin_guest_contract::{PluginHostError,TurnFault};
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{NativeAllocationAuthority,NativeAllocationAuthorityPort,NativeAllocationEvent,NativeAllocationReply,NativeAllocationReplyComplete,NativeAllocationRefusal,NativeAllocationRequest,OriginalSourceTurnIdentity,RetainedCloneGrant,RetainedCloneProgress},native_encoding::{NativeEncodeControl,NativeEncodeRetirementRecipient,NativeForwardedEncodeReceipt}};
use semio_os_vcs::io::binary::entity_identity::control::{EntityIdentityAuthority,Observer};
use std::pin::Pin;

/// 🎟️ Explicit independent receiving policy authored before the shard starts.
#[derive(Clone,Copy,Debug,semio_framework_value::serde::Serialize,semio_framework_value::serde::Deserialize)]
#[serde(crate="semio_framework_value::serde",rename_all="camelCase",deny_unknown_fields)]
pub struct ShardIdentityPolicy{pub maximum_bytes:usize,pub grant:RetainedCloneGrant}

/// 🏭️ The caller transfers a new consuming identity only at original actor registration.
pub type OriginalShardIdentityIssuer=Box<dyn FnMut(semio_framework_actor::ActorId)->Result<OriginalShardIdentity,ShardIdentityIssueRefused>+Send>;

/// 🛑️ Refusal returns the caller's original ports before any actor publication.
pub struct ShardIdentityIssueRefused{pub error:ValueError,pub observer:Box<Observer<'static>>}
impl std::fmt::Debug for ShardIdentityIssueRefused{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("ShardIdentityIssueRefused").field("error",&self.error).finish_non_exhaustive()}}

/// 🧾️ Consuming custody survives actor yields and failures without creating another authority.
pub struct OriginalShardIdentity{birth:RetainedCloneProgress,policy:ShardIdentityPolicy,receipt:Option<NativeForwardedEncodeReceipt>,pub recipient:Pin<Box<NativeEncodeRetirementRecipient>>,pub observer:Box<Observer<'static>>,allocate:NativeAllocationAuthority<ShardAllocationPort>}
/// 🛂️ The stable original port refuses any physical receiving frontier the independently authored policy cannot fund.
struct ShardAllocationPort{policy:ShardIdentityPolicy}
impl ShardAllocationPort{fn admits(&self,event:NativeAllocationEvent)->Result<(),ValueError>{match event{NativeAllocationEvent::Admit(request)|NativeAllocationEvent::Quote(request)=>{let grant=self.policy.grant;if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<NativeAllocationRequest>()||grant.maximum_capacity_bytes<request.bytes||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original caller shard policy refuses physical receiving frontier"))}Ok(())},_=>Ok(())}}}
impl NativeAllocationAuthorityPort for ShardAllocationPort{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity> where 'source:'turn{None}
 fn receive<'reply,'turn,'source>(&mut self,_:&mut (),event:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{Ok(reply.complete(self.admits(event).map_err(NativeAllocationRefusal::from)))}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut (),_:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{Ok(reply.pending())}
}
/// 📬️ The actual guest outcome and its original receiving decision remain available together.
pub struct OriginalShardActorAdmission{pub call:Result<semio_framework_actor::retained_context::OriginalActorAdmissionReply,TurnFault>,pub receiving:Result<semio_framework_actor::RetainedTurnReceipt,ValueError>}
impl OriginalShardIdentity{
 /// 🏭️ The original port admits the stable return slot before its physical birth.
 pub fn issue(policy:ShardIdentityPolicy,mut observer:Box<Observer<'static>>)->Result<Self,ShardIdentityIssueRefused>{
  let bytes=size_of::<NativeEncodeRetirementRecipient>();
  let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_capacity_bytes:bytes,maximum_release_bytes:0,maximum_depth:1};
  if !(policy.grant.maximum_items>=demand.maximum_items&&policy.grant.maximum_copy_bytes>=demand.maximum_copy_bytes&&policy.grant.maximum_capacity_bytes>=demand.maximum_capacity_bytes&&policy.grant.maximum_release_bytes>=demand.maximum_release_bytes&&policy.grant.maximum_depth>=demand.maximum_depth){return Err(ShardIdentityIssueRefused{error:ValueError::literal(ValueRefusalKind::OwnershipLimit,"original shard receiving policy refuses its return slot"),observer})}
  let mut allocate=NativeAllocationAuthority::new(ShardAllocationPort{policy});
  let mut recipient=Box::pin(NativeEncodeRetirementRecipient::new());
  let admitted=allocate.with_send_loan(&mut (),|loan|{
   let mut native=NativeEncodeControl::new_forwarded(policy.maximum_bytes,&mut *observer,loan);
   native.checkpoint().and_then(|_|native.charge(bytes))?;
   native.install_retirement_recipient(recipient.as_mut().get_mut()).expect("new original shard return slot is empty and uninstalled");
   Ok(native.detach().unwrap_or_else(|_|panic!("new original shard receiving receipt is valid between stages")))
  }).and_then(|receipt:Result<NativeForwardedEncodeReceipt,ValueError>|receipt);
  match admitted{
   Ok(receipt)=>Ok(Self{birth:RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:bytes,released_bytes:0},policy,receipt:Some(receipt),recipient,observer,allocate}),
   Err(error)=>Err(ShardIdentityIssueRefused{error,observer}),
  }
 }
 /// 🧾️ Reports the actual stable recipient's admitted inline copy and whole physical birth.
 pub fn birth_progress(&self)->RetainedCloneProgress{self.birth}
 /// 📏️ Borrows the original cumulative admission without copying its authority.
 pub fn owned_bytes(&self)->usize{self.receipt.as_ref().expect("original shard identity exists between loans").owned_bytes()}
 /// 🎟️ Returns the independently authored policy without inferring it from work or owned bytes.
 pub fn policy(&self)->ShardIdentityPolicy{self.policy}
 /// 🫴️ Lends the original receiving loan to one authoring operation and returns the cumulative receipt to its unique custody.
 pub fn with_identity<R>(&mut self,run:impl AsyncFnOnce(&mut EntityIdentityAuthority<'_>)->R)->Result<R,ValueError>{
  let receipt=self.receipt.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original shard identity is already loaned"))?;
  let Self{allocate,recipient,observer,receipt:returned,..}=self;
  let mut held=Some(receipt);
  let outcome=allocate.with_send_loan(&mut (),|loan|{
   let receipt=held.take().expect("original shard receipt enters its unique loan");
   match receipt.bind(loan,recipient.as_mut().get_mut()){
    Ok(continuation)=>{
     let mut identity:EntityIdentityAuthority<'_>=EntityIdentityAuthority::resume_forwarded(continuation,&mut **observer);
     let output=semio_framework_async::block_on(run(&mut identity));
     let receipt=identity.pause_forwarded().unwrap_or_else(|_|panic!("original shard loan retains its forwarded authority")).detach().unwrap_or_else(|_|panic!("original shard loan ends between synchronous receiving hops"));
     *returned=Some(receipt);Ok(output)
    },
    Err((error,receipt))=>{*returned=Some(receipt);Err(error)}
   }
  });
  match outcome{Ok(result)=>result,Err(error)=>{if let Some(receipt)=held.take(){*returned=Some(receipt)}Err(error)}}
 }
 /// 🤝️ One issued checkout borrows the original identity and receives its raw delta before readiness or parent return.
 pub async fn admit_actor_original<T>(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,original:&mut semio_framework_actor::retained_turn::OriginalActorInput<T>,mut budget:semio_framework::kernel::Budget,policy:RetainedCloneGrant)->Result<Option<OriginalShardActorAdmission>,ValueError>{
  if original.issued_input()!=Some(&budget.retained){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Host budget differs from its issued Actor input"))}
  let Some(input)=original.checkout_guest(policy)?else{return Ok(None)};budget.retained=input;budget.retained_policy=original.guest_policy(input)?;
  let call=self.with_identity(async move|identity|runtime.admit_actor_context(instance,input,budget,identity).await)?;
  let receiving=receive_actor_admission(original,&call);
  Ok(Some(OriginalShardActorAdmission{call,receiving}))
 }
 /// 🧳️ Admits the actual instance retained inside the original input without borrowing a second owner.
 pub async fn admit_owned_actor_original(&mut self,runtime:&GuestRuntimes,original:&mut semio_framework_actor::retained_turn::OriginalActorInput<GuestInstance>,mut budget:semio_framework::kernel::Budget,policy:RetainedCloneGrant)->Result<Option<OriginalShardActorAdmission>,ValueError>{
  if original.issued_input()!=Some(&budget.retained){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Host budget differs from its issued instance input"))}
  let Some(input)=original.checkout_guest(policy)?else{return Ok(None)};budget.retained=input;budget.retained_policy=original.guest_policy(input)?;
  let instance=original.original_mut_for_guest(input)?;let call=self.with_identity(async move|identity|runtime.admit_actor_context(instance,input,budget,identity).await)?;
  let receiving=receive_actor_admission(original,&call);Ok(Some(OriginalShardActorAdmission{call,receiving}))
 }
 /// 📸️ Resumes the original identity through the entire asynchronous checkpoint callback.
 pub async fn checkpoint(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance)->Result<Vec<u8>,PluginHostError>{
  self.with_identity(async|identity|runtime.checkpoint(instance,identity).await).map_err(PluginHostError::NativeIo)?
 }
 /// ▶️ Restores through the same receiving port, cumulative ledger and cancellation observer.
 pub async fn restore(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,state:&[u8])->Result<(),PluginHostError>{
  self.with_identity(async|identity|runtime.restore(instance,state,identity).await).map_err(PluginHostError::NativeIo)?
 }
 /// 🏃️ Retains the same caller admission after successful, yielded and refused actor turns.
 pub async fn execute_turn(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,events:&mut [semio_framework::kernel::Event],budget:semio_framework::kernel::Budget)->Result<semio_framework::kernel::TurnResult,TurnFault>{
  self.with_identity(async move|identity|runtime.execute_turn(instance,events,budget,identity).await).map_err(semio_plugin_guest_contract::identity_turn_fault)?
 }
}

fn receive_actor_admission<T>(original:&mut semio_framework_actor::retained_turn::OriginalActorInput<T>,call:&Result<semio_framework_actor::retained_context::OriginalActorAdmissionReply,TurnFault>)->Result<semio_framework_actor::RetainedTurnReceipt,ValueError>{match call{Ok(reply)=>{let phase=if reply.refusal.is_none()&&reply.ready{semio_framework_actor::retained_turn::ActorGuestPhase::Ready}else{original.phase()};original.receive_guest_spent(reply.input,reply.spent,reply.work_units_spent,phase)?;},Err(TurnFault::ActorCall(reply))=>{original.receive_guest_spent(reply.input,reply.spent,reply.work_units_spent,original.phase())?;},Err(_)=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original guest trap has no returned raw receipt; its checkout remains held")),}original.return_actor_turn()}

/// 🏭️ Creates each actor's original ports once from the root's unchanged independent policy.
pub fn native_identity_issuer(policy:ShardIdentityPolicy,mut observer_issuer:impl FnMut(semio_framework_actor::ActorId)->Box<Observer<'static>>+Send+'static)->OriginalShardIdentityIssuer{
 Box::new(move |actor|{
  let observer=observer_issuer(actor);
  OriginalShardIdentity::issue(policy,observer)
 })
}

