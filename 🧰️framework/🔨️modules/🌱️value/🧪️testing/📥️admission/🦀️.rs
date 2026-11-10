//! 📥️ Test-only source admission drives each real stage from the caller's existing finite wallet.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_native_decoding::{native_decoding,native_decoding::NativeDecodeControl,native_decoding::OriginalReceivingPrimary};
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::retirement;
use crate::retirement::RetireOwned;
pub use semio_framework_value_testing_effect_bank::{EffectBank,Wallet as BankWallet,Wallet,WorkLoan};
#[path="driven.rs"]
mod driven_impl;
pub use driven_impl::{Authorities,bank_run,bank_run_with_source,driven,session_admit,session_driven,session_nested,session_open,session_run};
#[path="🪙️finite/🦀️.rs"]
mod finite;
pub use finite::{FiniteWork,finite_work};
pub use semio_framework_value_clone_source_testing::{admit_owned,FixtureAdmissionRecovery,FixtureSource,FixtureSourceBirth};

pub fn denials(g:RetainedCloneGrant,d:crate::RetirementDemand)->impl Iterator<Item=RetainedCloneGrant>{[(true,RetainedCloneGrant{maximum_items:0,..g}),(d.copy_bytes>0,RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..g}),(d.capacity_bytes>0,RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..g}),(d.release_bytes>0,RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..g}),(d.depth>0,RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..g})].into_iter().filter_map(|(yes,g)|yes.then_some(g))}
/// ♻️ Exclusively retains the actual controlled owner across every original borrowed close action.
pub struct FixtureRetirementProducer<'owner,T:RetireOwned>{owner:&'owner mut retirement::controlled::ControlledRetirement<T>}
impl<'owner,T:RetireOwned> FixtureRetirementProducer<'owner,T>{/// 🪢️ Takes the real exclusive owner loan, retaining it through Pending and reply settlement.
 pub fn new(owner:&'owner mut retirement::controlled::ControlledRetirement<T>)->Self{Self{owner}}
 /// 👁️ Observes the same original controlled owner without lending mutation.
 pub fn owner(&self)->&retirement::controlled::ControlledRetirement<T>{self.owner}}
impl<T:RetireOwned> OriginalEffectProducer for FixtureRetirementProducer<'_,T>{
 type Output=RetainedCloneStep;type Failure=semio_framework_value_retirement::OriginalRetirementFailure;
 type Context<'turn,'source>=(usize,Option<&'turn mut semio_framework_value_retirement::OriginalRetirementCloseLoan<'source>>) where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn receive<'r,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'source:'turn{match c.1.as_deref_mut(){Some(original)=>self.owner.step_original_into(original,c.0,effect,reply),None=>self.owner.step_into(c.0,effect,reply)}}
 fn recover<'r,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'source:'turn{match c.1.as_deref_mut(){Some(original)=>self.owner.recover_step_original_into(original,c.0,effect,reply),None=>self.owner.recover_step_into(c.0,effect,reply)}}
}
impl<T:RetireOwned> OriginalEffectBirthProducer for FixtureRetirementProducer<'_,T>{
 type Original<'loan>=semio_framework_value_retirement::OriginalRetirementCloseLoan<'loan>;
 fn requires_original_birth(&self)->bool{self.owner.requires_original_birth()}
 fn close_original_birth<'turn,'source,'loan>(&mut self,_:&mut Self::Context<'turn,'source>,original:&mut Self::Original<'loan>,grant:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,Self::Failure>where 'source:'turn{self.owner.step_original_birth(original,grant)}
}

use semio_framework_value_retirement::{RetainedAllocationEffect,RetainedCloneProgress,RetainedWorkOutcome,RetirementDemand};
use semio_framework_value_control_contract::{OriginalEffectAuthority as DrivenAuthority,OriginalEffectProducer as DrivenProducer,OriginalEffectReply as DrivenReply,OriginalEffectReplyComplete as DrivenComplete,ValueError as Fault};
/// 🪝️ Names a reply-shaped call that a driven producer forwards to.
pub type DrivenFn<'t,T,F>=dyn for<'r,'e,'a> FnMut(&'e mut RetainedAllocationEffect<'a>,DrivenReply<'r,T,F>)->Result<DrivenComplete<'r,T,F>,Fault>+'t;
/// 🪝️ Lends the forwarded call to its producer as the action context.
pub struct DrivenCall<'t,T,F>(Option<&'t mut DrivenFn<'t,T,F>>);
struct Driven<T,F>{turn:OriginalSourceTurnIdentity,kinds:std::marker::PhantomData<fn()->(T,F)>}
impl<T:Send+'static,F:Send+'static> DrivenProducer for Driven<T,F>{
type Output=T;type Failure=F;
type Context<'turn,'source>=DrivenCall<'turn,T,F> where 'source:'turn;
fn validate_context<'turn,'source>(&self,_:&DrivenCall<'turn,T,F>)->Result<(),Fault>where 'source:'turn{Ok(())}
fn source_turn<'turn,'source>(&self,_:&DrivenCall<'turn,T,F>)->Option<OriginalSourceTurnIdentity>where 'source:'turn{Some(self.turn)}
fn receive<'r,'turn,'source>(&mut self,context:&mut DrivenCall<'turn,T,F>,effect:&mut RetainedAllocationEffect<'_>,reply:DrivenReply<'r,T,F>)->Result<DrivenComplete<'r,T,F>,Fault>where 'source:'turn{(context.0.take().ok_or_else(||Fault::invariant("driven call already spent"))?)(effect,reply)}
fn recover<'r,'turn,'source>(&mut self,_:&mut DrivenCall<'turn,T,F>,_:&mut RetainedAllocationEffect<'_>,_:DrivenReply<'r,T,F>)->Result<DrivenComplete<'r,T,F>,Fault>where 'source:'turn{Err(Fault::invariant("synchronous driven call has no pending action"))}
}
/// 🪝️ Fixes the higher-ranked shape of a forwarded call closure.
pub fn bind_reply<T,F,C:for<'r,'e,'a> FnMut(&'e mut RetainedAllocationEffect<'a>,DrivenReply<'r,T,F>)->Result<DrivenComplete<'r,T,F>,Fault>>(call:C)->C{call}
/// 📐️ Quotes the reply and producer transfers that a driven call adds to its grant.
pub fn driven_quotes<T:Send+'static,F:Send+'static>()->[RetirementDemand;2]{[DrivenAuthority::<Driven<T,F>>::transfer_reply_demand().expect("reply transfer quote"),DrivenAuthority::<Driven<T,F>>::producer_transfer_demand().expect("producer transfer quote")]}
/// 🪝️ Runs one reply-shaped call as an original producer action inside the caller's effect, so sealed retirement finds its invocation.
pub fn drive_reply<T:Send+'static,F:Send+'static>(effect:&mut RetainedAllocationEffect<'_>,turn:OriginalSourceTurnIdentity,call:&mut DrivenFn<'_,T,F>)->Result<RetainedWorkOutcome<T,F>,Fault>{
let mut context=DrivenCall(Some(call));let mut authority=Some(DrivenAuthority::new(Driven{turn,kinds:std::marker::PhantomData}));let mut action=None;let before=effect.performed_work();
if !authority.as_mut().unwrap().drive(&mut context,effect,&mut action)?{return Err(Fault::invariant("synchronous driven call stayed pending"))}
let work=effect.performed_work()-before;
let mut answer=None;if !action.as_mut().unwrap().transfer_reply(&mut answer)?{return Err(Fault::invariant("driven reply is unsettled"))}authority.as_mut().unwrap().finish(&mut action)?;
let mut producer=None;DrivenAuthority::transfer_producer(&mut authority,&context,&mut producer,effect).result?;producer.take();
Ok(RetainedWorkOutcome{result:answer.ok_or_else(||Fault::invariant("driven reply absent"))?,work})
}
