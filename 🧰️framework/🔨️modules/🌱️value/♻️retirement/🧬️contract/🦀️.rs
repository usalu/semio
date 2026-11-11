//! ♻️ Neutral retained-owner retirement and typed factories.
pub use semio_framework_value_control_contract::{ValueError,ValueRefusalKind,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
pub use semio_framework_value_control_contract::{NativeAllocationRefusal,RetirementDemand,RetainedWorkOutcome,RetainedWorkPoll};

pub use semio_framework_value_control_contract::{NativeAllocationRequest,NativeAllocationBudget,NativeAllocationEvent,NativeAllocationIdentity,NativeAllocationRefusalHandle,NativeAllocationRefusalVault,OriginalWorkActionIdentity,OriginalWorkActionProof,OriginalWorkState,OriginalWork,OriginalWorkBorrow,OriginalWorkAuthority,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkPerformedReceipt,OriginalWorkPartialOutcome,OriginalSourceTurnIdentity,OriginalSourceTurnIssuer,OriginalSourceTurn,OriginalEffectBirthContext,retained_clone_grant_admits};
#[path="♻️failure/🦀️.rs"]mod original_failure;
pub use original_failure::{forward_original_value_reply,original_refusal_transfer_demand,transfer_original_refusal,OriginalFailureStorageBirth,OriginalFailureStorageBirthPhase,OriginalFailurePerformedScope,OriginalFailureStorageStep,OriginalFailureFunding,refill_original_failure_storage,OriginalRetirementFailure,OriginalFailureStorage,OriginalFailureStoragePool,run_with_failure_storage,run_with_failure_storage_transaction};
#[path="♻️original-close/🦀️.rs"]mod original_close;
pub use original_close::{OriginalRetirementCloseLoan,OriginalRetirementClosePort};

/// ♻️ Retains original retirement progress, work and independently owned failures.
pub type OriginalRetirementOutcome<T=RetainedCloneStep>=RetainedWorkOutcome<T,OriginalRetirementFailure>;
/// ⏳️ Retains an unfinished original effect without inventing a physical receipt.
pub type OriginalRetirementPoll<T=RetainedCloneStep>=RetainedWorkPoll<T,OriginalRetirementFailure>;
/// 🚦️ Internal producer control flow preserves Pending separately from genuine owned failures.
#[doc(hidden)]
pub enum RetirementPollExit{Pending(u128),Failed(OriginalRetirementFailure)}
impl From<ValueError> for RetirementPollExit{fn from(error:ValueError)->Self{Self::Failed(error.into())}}
impl From<OriginalRetirementFailure> for RetirementPollExit{fn from(error:OriginalRetirementFailure)->Self{Self::Failed(error)}}
impl RetirementPollExit{
 /// 🪆️ Forwards a child poll into a ladder: `Pending` stays pending and a settled failure is kept.
 pub fn resume<T>(poll:OriginalRetirementPoll<T>)->Result<T,Self>{match poll{RetainedWorkPoll::Pending{work}=>Err(Self::Pending(work)),RetainedWorkPoll::Complete(turn)=>turn.result.map_err(Self::Failed)}}
 /// 🧾️ Records work a grant-shaped child reported into the caller's effect; an unfunded unit is a typed work-limit failure.
 pub fn pay(effect:&mut RetainedAllocationEffect<'_>,work:u128)->Result<(),Self>{let admitted=u64::try_from(work).is_ok_and(|work|work==0||effect.performed(work).unwrap_or(false));if admitted{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"retirement close exceeds its work ceiling").into())}}
 /// 🧾️ Settles a close ladder against its effect once: a progressing turn that recorded no work pays one unit, and the poll carries exactly the effect's performed work.
 pub fn settle(effect:&mut RetainedAllocationEffect<'_>,result:Result<RetainedCloneStep,Self>)->OriginalRetirementPoll{
  let result=match result{
   Err(Self::Pending(_))=>return RetainedWorkPoll::Pending{work:effect.performed_work()},
   Err(Self::Failed(failure))=>Err(failure),
   Ok(step) if effect.performed_work()==0&&step.progress()!=RetainedCloneProgress::default()=>match Self::pay(effect,1){Ok(())=>Ok(step),Err(Self::Failed(failure))=>Err(failure),Err(Self::Pending(_))=>unreachable!("paying work never suspends")},
   Ok(step)=>Ok(step),
  };
  RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:effect.performed_work()})
 }
 pub fn settled(poll:OriginalRetirementPoll)->Result<OriginalRetirementOutcome,Self>{match poll{RetainedWorkPoll::Pending{work}=>Err(Self::Pending(work)),RetainedWorkPoll::Complete(turn)=>Ok(turn)}}
 pub fn finish<T>(result:Result<T,Self>,work:u128)->OriginalRetirementPoll<T>{match result{Ok(value)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(value),work}),Err(Self::Failed(error))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work}),Err(Self::Pending(child))=>RetainedWorkPoll::Pending{work:work.checked_add(child).expect("bounded original retirement invocation work")}}}
}
/// 🧾️ Retains both independently owned errors and the actual physical receipt after work settlement.
pub fn settle_original_retirement_work(turn:RetainedWorkOutcome,settlement:Result<(),ValueError>)->RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>{
 let work=turn.work;let receipt=turn.result.as_ref().map(|step|step.progress()).unwrap_or_else(|error|error.retained_progress());
 let result=match (turn.result,settlement){(Ok(step),Ok(()))=>Ok(step),(Err(primary),Ok(()))=>Err(OriginalRetirementFailure::from_primary(primary)),(Ok(_),Err(refusal))=>Err(OriginalRetirementFailure::from_work_refusal(refusal.with_retained_progress(receipt))),(Err(primary),Err(refusal))=>Err(OriginalRetirementFailure::from_primary_and_work(primary,refusal))};
 RetainedWorkOutcome{result,work}
}
/// 🧾️ Reserves paid collision custody and original work before running one local effect.
pub fn run_original_retirement_work(control:&mut dyn OriginalRetirementControl,quote:u64,effect:impl FnOnce(&mut dyn OriginalRetirementControl)->OriginalRetirementOutcome)->OriginalRetirementOutcome{
 let mut storage=control.failure_storage_pool().take();if storage.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
 let mut pending=false;let mut started=false;let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
  match control.prepare_work(quote){Ok(true)=>{pending=true},Ok(false)=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>return RetainedWorkOutcome{result:Err(OriginalRetirementFailure::from_work_refusal(error)),work:0}}
  started=true;let turn=effect(control);crate::run_with_failure_storage(&mut storage,||turn,|work|{let result=control.commit_work(work);pending=false;result},|step|step.progress()).expect("paid original failure node remains reserved")
 }));if pending&&!started{control.cancel_prepared_work()}if let Some(node)=storage{control.failure_storage_pool().put(node)}match result{Ok(turn)=>turn,Err(panic)=>std::panic::resume_unwind(panic)}
}
/// 🧾️ Reserves paid collision custody and work before one original decoder-loan effect.
pub fn run_original_retirement_loan(original:&mut crate::OriginalRetirementCloseLoan<'_>,quote:u64,effect:impl FnOnce(&mut crate::OriginalRetirementCloseLoan<'_>)->OriginalRetirementOutcome)->OriginalRetirementOutcome{
 let mut storage=original.failure_storage_pool().take();if storage.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
 let mut pending=false;let mut started=false;let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
  match original.prepare_work(quote){Ok(true)=>{pending=true},Ok(false)=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>return RetainedWorkOutcome{result:Err(OriginalRetirementFailure::from_work_refusal(error)),work:0}}
  started=true;let turn=effect(original);crate::run_with_failure_storage(&mut storage,||turn,|work|{let result=original.commit_work(work);pending=false;result},|step|step.progress()).expect("paid original failure node remains reserved")
 }));if pending&&!started{original.cancel_prepared_work()}if let Some(node)=storage{original.failure_storage_pool().put(node)}match result{Ok(turn)=>turn,Err(panic)=>std::panic::resume_unwind(panic)}
}
/// 🪪️ Original decoding custody supplies its unchanged identity and funded cleanup allocations.
pub trait OriginalRetirementControl {
 fn failure_storage_birth(&mut self)->&mut OriginalFailureStorageBirth;
 fn failure_storage_performed_scope(&mut self)->Result<crate::OriginalFailurePerformedScope<'_>,ValueError>;
 fn settle_failure_storage_performed(&mut self)->OriginalFailureStorageStep<(),ValueError>;
 fn prepare_failure_storage_work(&mut self,work:u64)->OriginalFailureStorageStep<bool,ValueError>;
 fn commit_failure_storage_work(&mut self,work:u128)->Result<(),ValueError>;
 fn cancel_failure_storage_work(&mut self)->Result<(),ValueError>;
 fn finish_failure_storage_work(&mut self)->Result<(),ValueError>;

    /// ⛽️ Admits original caller work before a selected retirement effect.
    fn prepare_work(&mut self,additional:u64)->Result<bool,ValueError>;
    /// 🧾️ Posts the actual selected work to that same loan.
    fn commit_work(&mut self,actual:u128)->Result<(),ValueError>;
    /// ⏪️ Releases an unused pending work reservation without posting effects.
    fn cancel_prepared_work(&mut self);
    fn ledger_identity(&self) -> semio_framework_value_control_contract::NativeAllocationIdentity;
    fn retirement_recipient_identity(&self) -> Option<u64>;
    fn charge(&mut self, bytes: usize) -> Result<(), ValueError>;
    fn quote_failure_storage(&mut self,demand:RetirementDemand,grant:RetainedCloneGrant)->OriginalFailureStorageStep<RetainedCloneGrant>;
    fn post_failure_storage(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->OriginalFailureStorageStep<()>;
    fn cancel_failure_storage(&mut self)->OriginalFailureStorageStep<()>;
    fn failure_storage_pool(&mut self)->&mut crate::OriginalFailureStoragePool;
}
impl OriginalRetirementControl for crate::OriginalRetirementCloseLoan<'_>{
 fn failure_storage_birth(&mut self)->&mut OriginalFailureStorageBirth{self.failure_storage_birth()}
 fn failure_storage_performed_scope(&mut self)->Result<crate::OriginalFailurePerformedScope<'_>,ValueError>{self.failure_storage_performed_scope()}
 fn settle_failure_storage_performed(&mut self)->OriginalFailureStorageStep<(),ValueError>{self.settle_failure_storage_performed()}
 fn prepare_failure_storage_work(&mut self,work:u64)->OriginalFailureStorageStep<bool,ValueError>{self.prepare_failure_storage_work(work)}
 fn commit_failure_storage_work(&mut self,work:u128)->Result<(),ValueError>{self.commit_failure_storage_work(work)}
 fn cancel_failure_storage_work(&mut self)->Result<(),ValueError>{self.cancel_failure_storage_work()}
 fn finish_failure_storage_work(&mut self)->Result<(),ValueError>{self.finish_failure_storage_work()}

 fn prepare_work(&mut self,work:u64)->Result<bool,ValueError>{self.prepare_work(work)}
 fn commit_work(&mut self,work:u128)->Result<(),ValueError>{self.commit_work(work)}
 fn cancel_prepared_work(&mut self){self.cancel_prepared_work()}
 fn ledger_identity(&self)->semio_framework_value_control_contract::NativeAllocationIdentity{self.ledger_identity()}
 fn retirement_recipient_identity(&self)->Option<u64>{self.retirement_recipient_identity()}
 fn charge(&mut self,bytes:usize)->Result<(),ValueError>{self.charge(bytes)}
 fn quote_failure_storage(&mut self,d:RetirementDemand,g:RetainedCloneGrant)->OriginalFailureStorageStep<RetainedCloneGrant>{self.quote_failure_storage(d,g)}
 fn post_failure_storage(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->OriginalFailureStorageStep<()>{self.post_failure_storage(g,p)}
 fn cancel_failure_storage(&mut self)->OriginalFailureStorageStep<()>{self.cancel_failure_storage()}
 fn failure_storage_pool(&mut self)->&mut crate::OriginalFailureStoragePool{self.failure_storage_pool()}
}
/// 📏️ Quotes the physical birth of one prepaid original failure node.
pub fn original_failure_storage_birth_demand()->RetirementDemand{OriginalFailureStorage::birth_demand()}
/// ⛽️ Quotes the allocator attempt for one selected original failure node.
pub const fn original_failure_storage_birth_work_demand()->u64{1}
/// 📤️ Quotes the actual selected spare node release.
pub fn original_failure_storage_release_demand(pool:&crate::OriginalFailureStoragePool)->RetirementDemand{if pool.is_empty(){RetirementDemand::default()}else{RetirementDemand{release_bytes:OriginalFailureStorage::birth_bytes(),depth:1,..Default::default()}}}
/// ⛽️ Quotes the selected spare-node release action.
pub fn original_failure_storage_release_work_demand(pool:&crate::OriginalFailureStoragePool)->u64{u64::from(!pool.is_empty())}
/// 🎟️ Refills one original failure node as its own paid, physically posted turn.
impl OriginalFailureFunding for dyn OriginalRetirementControl+'_{
 fn birth(&mut self)->&mut OriginalFailureStorageBirth{self.failure_storage_birth()}
 fn performed_scope(&mut self)->Result<crate::OriginalFailurePerformedScope<'_>,ValueError>{self.failure_storage_performed_scope()}
 fn settle_performed(&mut self)->OriginalFailureStorageStep<(),ValueError>{self.settle_failure_storage_performed()}
 fn quote(&mut self,d:RetirementDemand,g:RetainedCloneGrant)->OriginalFailureStorageStep<RetainedCloneGrant>{self.quote_failure_storage(d,g)}
 fn post(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->OriginalFailureStorageStep<()>{self.post_failure_storage(g,p)}
 fn cancel(&mut self)->OriginalFailureStorageStep<()>{self.cancel_failure_storage()}
 fn pool(&mut self)->&mut crate::OriginalFailureStoragePool{self.failure_storage_pool()}
 fn prepare(&mut self,work:u64)->OriginalFailureStorageStep<bool,ValueError>{self.prepare_failure_storage_work(work)}
 fn commit(&mut self,work:u128)->Result<(),ValueError>{self.commit_failure_storage_work(work)}
 fn cancel_work(&mut self)->Result<(),ValueError>{self.cancel_failure_storage_work()}
 fn finish_work(&mut self)->Result<(),ValueError>{self.finish_failure_storage_work()}
}
pub trait ErasedSnapshotRetirement: Send + std::any::Any {
    /// 🎟️ This pure query only declares an explicit native lifecycle for the actual terminal header.
    fn native_header_release(&self)->Option<RetirementHeaderRelease>{None}
    /// 📥️ Records the actual ordinary child reply in its caller-owned slot before return.
    fn close_into<'reply>(&mut self,body:usize,_effect:&mut crate::RetainedAllocationEffect<'_>,_reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,std::any::type_name::<Self>()))}
    /// 🔁️ Recovers only the existing original invocation without replaying a child.
    fn recover_close_into<'reply>(&mut self,body:usize,_effect:&mut crate::RetainedAllocationEffect<'_>,_reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"retirement owner has no borrowed close recovery"))}
    /// 🧵️ Lends the same original controller into the actual child reply producer.
    fn close_original_into<'reply>(&mut self,_original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.requires_original_control(){return Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"retirement owner has no borrowed original close producer"))}self.close_into(body,effect,reply)}
    /// 🪡️ Recovers the original-controller child under its unchanged invocation.
    fn recover_close_original_into<'reply>(&mut self,_original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.requires_original_control(){return Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"retirement owner has no borrowed original close recovery"))}self.recover_close_into(body,effect,reply)}

    /// 👶️ Selects a real source-owned birth separately from ordinary closure effects.
    fn requires_original_birth(&self)->bool{false}
    fn close_original_birth_step(&mut self,_original:&mut crate::OriginalRetirementCloseLoan<'_>,_grant:RetainedCloneGrant)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"retirement owner has no selected original birth").into()),work:0})}
    /// 🪪️ Visits a checked original receiving frame without moving its retained owner.
    fn visit_original(&mut self,_identity:std::any::TypeId,_visitor:&mut dyn FnMut(&mut dyn std::any::Any))->bool{false}
    /// ⛽️ Quotes actual original closure work before the selected action.
    fn next_computational_work(&self,_body:usize)->Result<u64,ValueError>{Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original retirement lacks a work quote"))}
    /// 🧷️ Native owners override this port to preserve the caller's actual decoder ledger and recipient.
    fn close_original_step(&mut self,_original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.close_step(body,effect)}
    fn requires_original_control(&self) -> bool { false }
    /// 📮️ A reply-only owner lends the resident stash that keeps its unanswered turn between drivers.
    fn resident_reply(&mut self)->Option<&mut crate::ReplyStash<crate::OriginalRetirementFailure>>{None}
    fn close_step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->OriginalRetirementPoll;
    /// 🧵️ Composite owners forward actual caller decoding custody to decoder-bound children.
    fn close_step_controlled(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>, _control: &mut dyn OriginalRetirementControl)->OriginalRetirementPoll {let grant=effect.grant(); self.close_step(body,effect) }
    fn terminal_is_empty(&self) -> bool;
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;
    fn next_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_depth_demand(&self) -> Result<usize, ValueError>;
    /// 🎟️ Quotes the original next action once, preserving its actual body hint through every nested owner.
    fn next_demand(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { copy_bytes: self.next_copy_byte_demand()?, capacity_bytes: self.next_capacity_byte_demand(maximum_body_bytes)?, release_bytes: self.next_release_byte_demand()?, depth: self.next_depth_demand()? })
    }
}


#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod poll_tests;

pub use semio_framework_value_control_contract as retained_clone;
#[path="../../📬️reply-stash/🦀️.rs"]
mod reply_stash;
pub use reply_stash::ReplyStash;
#[path="../🎟️tickets/🦀️.rs"]
pub mod tickets;
pub use tickets::*;

/// ⏱️ Records the defining bounded operation through its original performed receipt.
pub fn record_retirement_work(effect:&mut crate::RetainedAllocationEffect<'_>,delta:u64)->Result<u128,(crate::ValueError,u128)>{let before=effect.performed_work();let result=effect.performed(delta);let work=effect.performed_work()-before;match result{Ok(true)=>Ok(work),Ok(false)=>Err((crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit,"retirement operation exceeded its original work action"),work)),Err(error)=>Err((error,work))}}

#[path="🎟️header/🦀️.rs"]mod header_release;
pub use header_release::RetirementHeaderRelease;

#[path="🔗️arc-custody/🦀️.rs"]
mod shared_controlled_owner;
