//! ♻️ Neutral original cleanup loans preserve the exact borrowed physical recipient across retirement erasure.
use crate::{ValueError,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,OriginalFailureStoragePool};
/// 🧷️ This borrowed capability never owns a replacement ledger or independent capacity counter.
/// 🎟️ The original caller supplies the same capacity and computational work authority.
pub trait OriginalRetirementClosePort{
 fn charge(&mut self,bytes:usize)->Result<(),ValueError>;
 fn quote_failure_storage(&mut self,demand:RetirementDemand,grant:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError>;
 fn post_failure_storage(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>;
 fn cancel_failure_storage(&mut self)->Result<(),ValueError>;
 fn failure_storage_pool(&mut self)->&mut OriginalFailureStoragePool;
 fn prepare_work(&mut self,additional:u64)->Result<bool,ValueError>;
 fn commit_work(&mut self,actual:u64)->Result<(),ValueError>;
 fn cancel_prepared_work(&mut self);
}
pub struct OriginalRetirementCloseLoan<'a>{ledger:u64,recipient:Option<u64>,port:&'a mut dyn OriginalRetirementClosePort}
impl<'a> OriginalRetirementCloseLoan<'a>{
 /// 🎟️ Borrows the defining original decoder rather than issuing another physical currency.
 /// # Safety
 /// The ledger, recipient and port must belong to the same live original decoder retirement loan.
 /// Its caller grant and cumulative work and capacity admission must remain enforced by that port.
 pub unsafe fn from_original(ledger:u64,recipient:Option<u64>,port:&'a mut dyn OriginalRetirementClosePort)->Self{Self{ledger,recipient,port}}
 pub fn ledger_identity(&self)->u64{self.ledger}
 pub fn retirement_recipient_identity(&self)->Option<u64>{self.recipient}
 pub fn charge(&mut self,bytes:usize)->Result<(),ValueError>{self.port.charge(bytes)}
 pub fn quote_failure_storage(&mut self,demand:RetirementDemand,grant:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError>{self.port.quote_failure_storage(demand,grant)}
 pub fn post_failure_storage(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{self.port.post_failure_storage(grant,progress)}
 pub fn cancel_failure_storage(&mut self)->Result<(),ValueError>{self.port.cancel_failure_storage()}
 pub fn failure_storage_pool(&mut self)->&mut OriginalFailureStoragePool{self.port.failure_storage_pool()}
 pub fn prepare_work(&mut self,additional:u64)->Result<bool,ValueError>{self.port.prepare_work(additional)}
 pub fn commit_work(&mut self,actual:u64)->Result<(),ValueError>{self.port.commit_work(actual)}
 pub fn cancel_prepared_work(&mut self){self.port.cancel_prepared_work()}
}
