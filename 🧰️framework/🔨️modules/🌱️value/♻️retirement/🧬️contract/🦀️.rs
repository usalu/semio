//! ♻️ Neutral retained-owner retirement and typed factories.
use crate::{ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
pub use semio_framework_value_control_contract::{RetirementDemand,RetainedWorkOutcome,OriginalRetirementFailure,OriginalFailureStorage};
/// ♻️ Retains original retirement progress, work and independently owned failures.
pub type OriginalRetirementOutcome<T=RetainedCloneStep>=RetainedWorkOutcome<T,OriginalRetirementFailure>;
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
  started=true;let turn=effect(control);semio_framework_value_control_contract::run_with_failure_storage(&mut storage,||turn,|work|{let result=control.commit_work(work);pending=false;result},|step|step.progress()).expect("paid original failure node remains reserved")
 }));if pending&&!started{control.cancel_prepared_work()}if let Some(node)=storage{control.failure_storage_pool().put(node)}match result{Ok(turn)=>turn,Err(panic)=>std::panic::resume_unwind(panic)}
}
/// 🧾️ Reserves paid collision custody and work before one original decoder-loan effect.
pub fn run_original_retirement_loan(original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,quote:u64,effect:impl FnOnce(&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>)->OriginalRetirementOutcome)->OriginalRetirementOutcome{
 let mut storage=original.failure_storage_pool().take();if storage.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
 let mut pending=false;let mut started=false;let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
  match original.prepare_work(quote){Ok(true)=>{pending=true},Ok(false)=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>return RetainedWorkOutcome{result:Err(OriginalRetirementFailure::from_work_refusal(error)),work:0}}
  started=true;let turn=effect(original);semio_framework_value_control_contract::run_with_failure_storage(&mut storage,||turn,|work|{let result=original.commit_work(work);pending=false;result},|step|step.progress()).expect("paid original failure node remains reserved")
 }));if pending&&!started{original.cancel_prepared_work()}if let Some(node)=storage{original.failure_storage_pool().put(node)}match result{Ok(turn)=>turn,Err(panic)=>std::panic::resume_unwind(panic)}
}
/// 🪪️ Original decoding custody supplies its unchanged identity and funded cleanup allocations.
pub trait OriginalRetirementControl {
    /// ⛽️ Admits original caller work before a selected retirement effect.
    fn prepare_work(&mut self,additional:u64)->Result<bool,ValueError>;
    /// 🧾️ Posts the actual selected work to that same loan.
    fn commit_work(&mut self,actual:u64)->Result<(),ValueError>;
    /// ⏪️ Releases an unused pending work reservation without posting effects.
    fn cancel_prepared_work(&mut self);
    fn ledger_identity(&self) -> u64;
    fn retirement_recipient_identity(&self) -> Option<u64>;
    fn charge(&mut self, bytes: usize) -> Result<(), ValueError>;
    fn quote_failure_storage(&mut self,demand:RetirementDemand,grant:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError>;
    fn post_failure_storage(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>;
    fn cancel_failure_storage(&mut self)->Result<(),ValueError>;
    fn failure_storage_pool(&mut self)->&mut semio_framework_value_control_contract::OriginalFailureStoragePool;
}
impl OriginalRetirementControl for semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>{
 fn prepare_work(&mut self,work:u64)->Result<bool,ValueError>{self.prepare_work(work)}
 fn commit_work(&mut self,work:u64)->Result<(),ValueError>{self.commit_work(work)}
 fn cancel_prepared_work(&mut self){self.cancel_prepared_work()}
 fn ledger_identity(&self)->u64{self.ledger_identity()}
 fn retirement_recipient_identity(&self)->Option<u64>{self.retirement_recipient_identity()}
 fn charge(&mut self,bytes:usize)->Result<(),ValueError>{self.charge(bytes)}
 fn quote_failure_storage(&mut self,d:RetirementDemand,g:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError>{self.quote_failure_storage(d,g)}
 fn post_failure_storage(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{self.post_failure_storage(g,p)}
 fn cancel_failure_storage(&mut self)->Result<(),ValueError>{self.cancel_failure_storage()}
 fn failure_storage_pool(&mut self)->&mut semio_framework_value_control_contract::OriginalFailureStoragePool{self.failure_storage_pool()}
}
/// 📏️ Quotes the physical birth of one prepaid original failure node.
pub fn original_failure_storage_birth_demand()->RetirementDemand{RetirementDemand{capacity_bytes:OriginalFailureStorage::birth_bytes(),depth:1,..Default::default()}}
/// ⛽️ Quotes the allocator attempt for one selected original failure node.
pub const fn original_failure_storage_birth_work_demand()->u64{1}
/// 📤️ Quotes the actual selected spare node release.
pub fn original_failure_storage_release_demand(pool:&semio_framework_value_control_contract::OriginalFailureStoragePool)->RetirementDemand{if pool.is_empty(){RetirementDemand::default()}else{RetirementDemand{release_bytes:OriginalFailureStorage::birth_bytes(),depth:1,..Default::default()}}}
/// ⛽️ Quotes the selected spare-node release action.
pub fn original_failure_storage_release_work_demand(pool:&semio_framework_value_control_contract::OriginalFailureStoragePool)->u64{u64::from(!pool.is_empty())}
/// 🎟️ Refills one original failure node as its own paid, physically posted turn.
pub trait OriginalFailureFunding{
 fn quote(&mut self,demand:RetirementDemand,grant:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError>;
 fn post(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>;
 fn cancel(&mut self)->Result<(),ValueError>;
 fn pool(&mut self)->&mut semio_framework_value_control_contract::OriginalFailureStoragePool;
 fn prepare(&mut self,work:u64)->Result<bool,ValueError>;
 fn commit(&mut self,work:u64)->Result<(),ValueError>;
}
impl OriginalFailureFunding for semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>{
 fn quote(&mut self,d:RetirementDemand,g:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError>{self.quote_failure_storage(d,g)}
 fn post(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{self.post_failure_storage(g,p)}
 fn cancel(&mut self)->Result<(),ValueError>{self.cancel_failure_storage()}
 fn pool(&mut self)->&mut semio_framework_value_control_contract::OriginalFailureStoragePool{self.failure_storage_pool()}
 fn prepare(&mut self,w:u64)->Result<bool,ValueError>{self.prepare_work(w)}
 fn commit(&mut self,w:u64)->Result<(),ValueError>{self.commit_work(w)}
}
impl OriginalFailureFunding for dyn OriginalRetirementControl+'_ {
 fn quote(&mut self,d:RetirementDemand,g:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError>{self.quote_failure_storage(d,g)}
 fn post(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{self.post_failure_storage(g,p)}
 fn cancel(&mut self)->Result<(),ValueError>{self.cancel_failure_storage()}
 fn pool(&mut self)->&mut semio_framework_value_control_contract::OriginalFailureStoragePool{self.failure_storage_pool()}
 fn prepare(&mut self,w:u64)->Result<bool,ValueError>{self.prepare_work(w)}
 fn commit(&mut self,w:u64)->Result<(),ValueError>{self.commit_work(w)}
}
pub fn refill_original_failure_storage(funding:&mut (impl OriginalFailureFunding+?Sized),grant:RetainedCloneGrant)->OriginalRetirementOutcome{
 let bytes=OriginalFailureStorage::birth_bytes();let demand=RetirementDemand{capacity_bytes:bytes,depth:1,..Default::default()};
 if !semio_framework_value_control_contract::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
 let quoted=match funding.quote(demand,grant){Ok(quoted)=>quoted,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
 if !semio_framework_value_control_contract::retained_clone_grant_admits(quoted,demand){return match funding.cancel(){Ok(())=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>RetainedWorkOutcome{result:Err(OriginalRetirementFailure::from_causes(None,None,Some(error))),work:0}}}
 match funding.prepare(1){Ok(true)=>{},Ok(false)=>return match funding.cancel(){Ok(())=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>RetainedWorkOutcome{result:Err(OriginalRetirementFailure::from_causes(None,None,Some(error))),work:0}},Err(error)=>{let receipt=funding.cancel().err();return RetainedWorkOutcome{result:Err(OriginalRetirementFailure::from_causes(None,Some(error),receipt)),work:0}}}
 let turn=OriginalFailureStorage::admit(quoted.maximum_capacity_bytes);let (primary,progress)=match turn.result{Ok((node,progress))=>{funding.pool().put(node);(None,progress)},Err(error)=>(Some(error),RetainedCloneProgress::default())};
 let receipt=funding.post(quoted,progress).err();let work_refusal=funding.commit(turn.work).err();
 let result=if primary.is_none()&&receipt.is_none()&&work_refusal.is_none(){Ok(RetainedCloneStep::Progress(progress))}else{Err(OriginalRetirementFailure::from_causes(primary,work_refusal,receipt).with_retained_progress(progress))};RetainedWorkOutcome{result,work:turn.work}
}
pub trait ErasedSnapshotRetirement: Send {
    /// 🪪️ Visits a checked original receiving frame without moving its retained owner.
    fn visit_original(&mut self,_identity:std::any::TypeId,_visitor:&mut dyn FnMut(&mut dyn std::any::Any))->bool{false}
    /// ⛽️ Quotes actual original closure work before the selected action.
    fn next_computational_work(&self,_body:usize)->Result<u64,ValueError>{Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"original retirement lacks a work quote"))}
    /// 🧷️ Native owners override this port to preserve the caller's actual decoder ledger and recipient.
    fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementOutcome{let d=match self.next_demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};if !semio_framework_value_control_contract::retained_clone_grant_admits(grant,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}if let Err(error)=original.charge(d.capacity_bytes){return RetainedWorkOutcome{result:Err(error.into()),work:0}}self.close_step(grant)}
    fn requires_original_control(&self) -> bool { false }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> OriginalRetirementOutcome;
    /// 🧵️ Composite owners forward actual caller decoding custody to decoder-bound children.
    fn close_step_controlled(&mut self, grant: RetainedCloneGrant, _control: &mut dyn OriginalRetirementControl) -> OriginalRetirementOutcome { self.close_step(grant) }
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

