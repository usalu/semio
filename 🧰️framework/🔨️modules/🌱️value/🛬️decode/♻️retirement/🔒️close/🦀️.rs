//! 🔒️ Cleanup borrows only the original decoder identity and paid retirement capacity.
use super::NativeRetirementDecodeControl;
use super::super::{NativeDecodeControl,NativeForwardedDecodeControl};
use crate::ValueError;
mod sealed { pub trait Sealed {} }

/// ♻️ Native cleanup cannot acquire a decode checkpoint, source parser or replacement receiving authority.
pub trait NativeDecodeCloseControl:sealed::Sealed {
 fn ledger_identity(&self)->crate::NativeAllocationIdentity;
 fn retirement_recipient_identity(&self)->Option<u64>;
 fn charge(&mut self,bytes:usize)->Result<(),ValueError>;
}
impl sealed::Sealed for NativeDecodeControl<'_> {}
impl NativeDecodeCloseControl for NativeDecodeControl<'_> {
 fn ledger_identity(&self)->crate::NativeAllocationIdentity { NativeDecodeControl::ledger_identity(self) }
 fn retirement_recipient_identity(&self)->Option<u64> { NativeDecodeControl::retirement_recipient_identity(self) }
 fn charge(&mut self,bytes:usize)->Result<(),ValueError> { NativeDecodeControl::charge(self,bytes) }
}
impl sealed::Sealed for NativeRetirementDecodeControl<'_, '_> {}
impl NativeDecodeCloseControl for NativeRetirementDecodeControl<'_, '_> {
 fn ledger_identity(&self)->crate::NativeAllocationIdentity { NativeRetirementDecodeControl::ledger_identity(self) }
 fn retirement_recipient_identity(&self)->Option<u64> { NativeRetirementDecodeControl::retirement_recipient_identity(self) }
 fn charge(&mut self,bytes:usize)->Result<(),ValueError> { NativeRetirementDecodeControl::charge(self,bytes) }
}
impl sealed::Sealed for NativeForwardedDecodeControl<'_> {}
impl NativeDecodeCloseControl for NativeForwardedDecodeControl<'_> {
 fn ledger_identity(&self)->crate::NativeAllocationIdentity { NativeDecodeControl::ledger_identity(self) }
 fn retirement_recipient_identity(&self)->Option<u64> { NativeDecodeControl::retirement_recipient_identity(self) }
 fn charge(&mut self,bytes:usize)->Result<(),ValueError> { NativeDecodeControl::charge(self,bytes) }
}
impl sealed::Sealed for semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>{}
impl NativeDecodeCloseControl for semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>{fn ledger_identity(&self)->crate::NativeAllocationIdentity{self.ledger_identity()}fn retirement_recipient_identity(&self)->Option<u64>{self.retirement_recipient_identity()}fn charge(&mut self,bytes:usize)->Result<(),ValueError>{self.charge(bytes)}}
impl semio_framework_value_retirement_contract::OriginalRetirementClosePort for NativeRetirementDecodeControl<'_, '_>{
 fn original_work(&mut self)->semio_framework_value_control_contract::OriginalWorkBorrow<'_>{semio_framework_value_control_contract::OriginalWorkBorrow::new(&mut self.original.work)}
 fn failure_storage_birth(&mut self)->&mut semio_framework_value_retirement_contract::OriginalFailureStorageBirth{semio_framework_value_retirement::OriginalRetirementControl::failure_storage_birth(self)}
 fn failure_storage_performed_scope(&mut self)->Result<semio_framework_value_retirement_contract::OriginalFailurePerformedScope<'_>,ValueError>{semio_framework_value_retirement::OriginalRetirementControl::failure_storage_performed_scope(self)}
 fn settle_failure_storage_performed(&mut self)->semio_framework_value_retirement_contract::OriginalFailureStorageStep<(),ValueError>{semio_framework_value_retirement::OriginalRetirementControl::settle_failure_storage_performed(self)}
 fn prepare_failure_storage_work(&mut self,work:u64)->semio_framework_value_retirement_contract::OriginalFailureStorageStep<bool,ValueError>{semio_framework_value_retirement::OriginalRetirementControl::prepare_failure_storage_work(self,work)}
 fn commit_failure_storage_work(&mut self,work:u128)->Result<(),ValueError>{semio_framework_value_retirement::OriginalRetirementControl::commit_failure_storage_work(self,work)}
 fn cancel_failure_storage_work(&mut self)->Result<(),ValueError>{semio_framework_value_retirement::OriginalRetirementControl::cancel_failure_storage_work(self)}
 fn finish_failure_storage_work(&mut self)->Result<(),ValueError>{semio_framework_value_retirement::OriginalRetirementControl::finish_failure_storage_work(self)}
 fn charge(&mut self,bytes:usize)->Result<(),ValueError>{NativeRetirementDecodeControl::charge(self,bytes)}
 fn prepare_work(&mut self,additional:u64)->Result<bool,ValueError>{self.original.prepare_work(additional)}
 fn commit_work(&mut self,actual:u128)->Result<(),ValueError>{self.original.commit_work(actual)}
 fn cancel_prepared_work(&mut self){self.original.cancel_prepared_work()}
 fn quote_failure_storage(&mut self,demand:crate::RetirementDemand,grant:crate::RetainedCloneGrant)->semio_framework_value_retirement_contract::OriginalFailureStorageStep<crate::RetainedCloneGrant>{NativeRetirementDecodeControl::quote_failure_storage(self,demand,grant)}
 fn post_failure_storage(&mut self,grant:crate::RetainedCloneGrant,progress:crate::RetainedCloneProgress)->semio_framework_value_retirement_contract::OriginalFailureStorageStep<()>{NativeRetirementDecodeControl::post_failure_storage(self,grant,progress)}
 fn cancel_failure_storage(&mut self)->semio_framework_value_retirement_contract::OriginalFailureStorageStep<()>{NativeRetirementDecodeControl::cancel_failure_storage(self)}
 fn failure_storage_pool(&mut self)->&mut semio_framework_value_retirement_contract::OriginalFailureStoragePool{NativeRetirementDecodeControl::failure_storage_pool(self)}
}
impl NativeRetirementDecodeControl<'_, '_>{
 /// 🫴️ Loans the exact actual decoder identity and already admitted cleanup callback through neutral erasure.
 pub fn with_original_close<T,E:From<ValueError>>(&mut self,operation:impl FnOnce(&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>)->Result<T,E>)->Result<T,E>{let ledger=self.ledger_identity();let recipient=self.retirement_recipient_identity();let mut original=unsafe{semio_framework_value_retirement_contract::OriginalRetirementCloseLoan::from_original(ledger,recipient,self)};operation(&mut original)}
}
