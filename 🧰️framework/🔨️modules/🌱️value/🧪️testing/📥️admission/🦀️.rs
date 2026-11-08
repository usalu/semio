//! 📥️ Test-only source admission drives each real stage from the caller's existing finite wallet.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_native_decoding::{native_decoding,native_decoding::NativeDecodeControl,native_decoding::OriginalReceivingPrimary};
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::retirement;
use crate::retirement::RetireOwned;
pub use semio_framework_value_testing_effect_bank::{EffectBank,Wallet as BankWallet,Wallet,WorkLoan};
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
