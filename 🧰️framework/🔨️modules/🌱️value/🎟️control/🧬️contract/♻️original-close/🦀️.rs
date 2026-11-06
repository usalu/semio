//! ♻️ Neutral original cleanup loans preserve the exact borrowed physical recipient across retirement erasure.
use crate::{OriginalFailureStorageBirth,OriginalFailureStorageStep,ValueError,NativeAllocationRefusal,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,OriginalFailureStoragePool};
/// 🧷️ This borrowed capability never owns a replacement ledger or independent capacity counter.
/// 🎟️ The original caller supplies the same capacity and computational work authority.
pub trait OriginalRetirementClosePort{
 fn failure_storage_birth(&mut self)->&mut OriginalFailureStorageBirth;
 fn failure_storage_performed_scope(&mut self)->Result<crate::OriginalFailurePerformedScope<'_>,ValueError>;
 fn settle_failure_storage_performed(&mut self)->OriginalFailureStorageStep<(),ValueError>;
 fn prepare_failure_storage_work(&mut self,work:u64)->OriginalFailureStorageStep<bool,ValueError>;
 fn commit_failure_storage_work(&mut self,work:u128)->Result<(),ValueError>;
 fn cancel_failure_storage_work(&mut self)->Result<(),ValueError>;
 fn finish_failure_storage_work(&mut self)->Result<(),ValueError>;

 fn charge(&mut self,bytes:usize)->Result<(),ValueError>;
 fn quote_failure_storage(&mut self,demand:RetirementDemand,grant:RetainedCloneGrant)->OriginalFailureStorageStep<RetainedCloneGrant>;
 fn post_failure_storage(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->OriginalFailureStorageStep<()>;
 fn cancel_failure_storage(&mut self)->OriginalFailureStorageStep<()>;
 fn failure_storage_pool(&mut self)->&mut OriginalFailureStoragePool;
 fn prepare_work(&mut self,additional:u64)->Result<bool,ValueError>;
 fn commit_work(&mut self,actual:u128)->Result<(),ValueError>;
 fn cancel_prepared_work(&mut self);
}
pub struct OriginalRetirementCloseLoan<'a>{ledger:crate::NativeAllocationIdentity,recipient:Option<u64>,port:&'a mut dyn OriginalRetirementClosePort}
impl<'a> OriginalRetirementCloseLoan<'a>{
 /// 🎟️ Borrows the defining original decoder rather than issuing another physical currency.
 /// # Safety
 /// The ledger, recipient and port must belong to the same live original decoder retirement loan.
 /// Its caller grant and cumulative work and capacity admission must remain enforced by that port.
 pub unsafe fn from_original(ledger:crate::NativeAllocationIdentity,recipient:Option<u64>,port:&'a mut dyn OriginalRetirementClosePort)->Self{Self{ledger,recipient,port}}
 pub fn ledger_identity(&self)->crate::NativeAllocationIdentity{self.ledger}
 pub fn retirement_recipient_identity(&self)->Option<u64>{self.recipient}
 pub fn charge(&mut self,bytes:usize)->Result<(),ValueError>{self.port.charge(bytes)}
 pub fn failure_storage_birth(&mut self)->&mut OriginalFailureStorageBirth{self.port.failure_storage_birth()}
 pub fn failure_storage_performed_scope(&mut self)->Result<crate::OriginalFailurePerformedScope<'_>,ValueError>{self.port.failure_storage_performed_scope()}
 pub fn settle_failure_storage_performed(&mut self)->OriginalFailureStorageStep<(),ValueError>{self.port.settle_failure_storage_performed()}
 pub fn prepare_failure_storage_work(&mut self,work:u64)->OriginalFailureStorageStep<bool,ValueError>{self.port.prepare_failure_storage_work(work)}
 pub fn commit_failure_storage_work(&mut self,work:u128)->Result<(),ValueError>{self.port.commit_failure_storage_work(work)}
 pub fn cancel_failure_storage_work(&mut self)->Result<(),ValueError>{self.port.cancel_failure_storage_work()}
 pub fn finish_failure_storage_work(&mut self)->Result<(),ValueError>{self.port.finish_failure_storage_work()}
 pub fn quote_failure_storage(&mut self,demand:RetirementDemand,grant:RetainedCloneGrant)->OriginalFailureStorageStep<RetainedCloneGrant>{self.port.quote_failure_storage(demand,grant)}
 pub fn post_failure_storage(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->OriginalFailureStorageStep<()>{self.port.post_failure_storage(grant,progress)}
 pub fn cancel_failure_storage(&mut self)->OriginalFailureStorageStep<()>{self.port.cancel_failure_storage()}
 pub fn failure_storage_pool(&mut self)->&mut OriginalFailureStoragePool{self.port.failure_storage_pool()}
 pub fn prepare_work(&mut self,additional:u64)->Result<bool,ValueError>{self.port.prepare_work(additional)}
 pub fn commit_work(&mut self,actual:u128)->Result<(),ValueError>{self.port.commit_work(actual)}
 pub fn cancel_prepared_work(&mut self){self.port.cancel_prepared_work()}
}
