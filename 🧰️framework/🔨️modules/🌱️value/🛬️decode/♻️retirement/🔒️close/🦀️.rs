//! 🔒️ Cleanup borrows only the original decoder identity and paid retirement capacity.
use super::NativeRetirementDecodeControl;
use super::super::{NativeDecodeControl,NativeForwardedDecodeControl};
use crate::ValueError;
mod sealed { pub trait Sealed {} }

/// ♻️ Native cleanup cannot acquire a decode checkpoint, source parser or replacement receiving authority.
pub trait NativeDecodeCloseControl:sealed::Sealed {
 fn ledger_identity(&self)->u64;
 fn retirement_recipient_identity(&self)->Option<u64>;
 fn charge(&mut self,bytes:usize)->Result<(),ValueError>;
}
impl sealed::Sealed for NativeDecodeControl<'_> {}
impl NativeDecodeCloseControl for NativeDecodeControl<'_> {
 fn ledger_identity(&self)->u64 { NativeDecodeControl::ledger_identity(self) }
 fn retirement_recipient_identity(&self)->Option<u64> { NativeDecodeControl::retirement_recipient_identity(self) }
 fn charge(&mut self,bytes:usize)->Result<(),ValueError> { NativeDecodeControl::charge(self,bytes) }
}
impl sealed::Sealed for NativeRetirementDecodeControl<'_, '_> {}
impl NativeDecodeCloseControl for NativeRetirementDecodeControl<'_, '_> {
 fn ledger_identity(&self)->u64 { NativeRetirementDecodeControl::ledger_identity(self) }
 fn retirement_recipient_identity(&self)->Option<u64> { NativeRetirementDecodeControl::retirement_recipient_identity(self) }
 fn charge(&mut self,bytes:usize)->Result<(),ValueError> { NativeRetirementDecodeControl::charge(self,bytes) }
}
impl sealed::Sealed for NativeForwardedDecodeControl<'_> {}
impl NativeDecodeCloseControl for NativeForwardedDecodeControl<'_> {
 fn ledger_identity(&self)->u64 { NativeDecodeControl::ledger_identity(self) }
 fn retirement_recipient_identity(&self)->Option<u64> { NativeDecodeControl::retirement_recipient_identity(self) }
 fn charge(&mut self,bytes:usize)->Result<(),ValueError> { NativeDecodeControl::charge(self,bytes) }
}
impl sealed::Sealed for semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>{}
impl NativeDecodeCloseControl for semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>{fn ledger_identity(&self)->u64{self.ledger_identity()}fn retirement_recipient_identity(&self)->Option<u64>{self.retirement_recipient_identity()}fn charge(&mut self,bytes:usize)->Result<(),ValueError>{self.charge(bytes)}}
impl NativeRetirementDecodeControl<'_, '_>{
 /// 🫴️ Loans the exact actual decoder identity and already admitted cleanup callback through neutral erasure.
 pub fn with_original_close<T>(&mut self,operation:impl FnOnce(&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>)->Result<T,ValueError>)->Result<T,ValueError>{let ledger=self.ledger_identity();let recipient=self.retirement_recipient_identity();let mut charge=|bytes|self.charge(bytes);let mut original=unsafe{semio_framework_value_control_contract::OriginalRetirementCloseLoan::from_original(ledger,recipient,&mut charge)};operation(&mut original)}
}
