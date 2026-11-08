//! 🎟️ Decoder work borrows the original caller's independent work loan.
use super::{NativeDecodeControl,ValueError};
pub use semio_framework_value_control_contract::{OriginalWorkAuthority,OriginalWorkIssuerIdentity,OriginalWorkActionIssuer,OriginalWorkLoan,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalSourceTurnIdentity};
pub(super) use semio_framework_value_control_contract::{OriginalWork as Work,OriginalWorkState as WorkState};
struct Scope<'owner,'call>{control:NativeDecodeControl<'call>,maximum:&'owner mut usize,scope_maximum:&'owner mut Option<usize>,ledger:&'owner mut Option<semio_framework_value_native_encoding::native_receiving::NativeAllocationLedger>,completed:&'owner mut usize,total:&'owner mut usize,started:&'owner mut bool,stage:&'owner mut u64,depth:&'owner mut usize,work_state:&'owner mut WorkState}
impl Drop for Scope<'_, '_>{fn drop(&mut self){*self.maximum=self.control.maximum_bytes;*self.scope_maximum=self.control.scope_maximum;*self.ledger=self.control.ledger.take();*self.completed=self.control.completed;*self.total=self.control.total;*self.started=self.control.started;*self.stage=self.control.stage;*self.depth=self.control.depth;*self.work_state=self.control.work.state.take();}}
impl NativeDecodeControl<'_>{
 /// 🧮️ Counts committed defining effects, including those preceding an owned failure.
 pub fn actual_work(&self)->u128{self.work.actual()}
 /// 👁️ Reports whether the original caller supplied a work loan.
 pub fn has_work_recipient(&self)->bool{self.work.recipient.is_some()}
 /// 🪪️ Names the original work source for recovery.
 pub fn work_recipient_identity(&self)->Option<u64>{self.work.recipient.as_ref().map(|loan|loan.identity().get())}
 /// ⏳️ Reports a pending original quote or admitted work reservation.
 pub fn has_pending_work(&self)->bool{self.work.state.has_live_actions()}
 /// 🎟️ Checks whether the same original work issuer already admitted this child effect.
 pub fn prepared_work_admits(&self,amount:u64)->bool{self.work.same_issuer()&&!self.work.state.quote_pending()&&self.work.state.reserved()>=amount}
 /// 📏️ Reports the exact work quantity still held by the original source.
 pub fn pending_work(&self)->u64{self.work.state.reserved()}
 /// 🚦️ Reports an original work quote callback that has not yet resolved.
 pub fn work_quote_pending(&self)->bool{self.work.state.quote_pending()}
 /// 🚦️ Retains the original quote boundary before invoking its work backend.
 pub fn prepare_work(&mut self,additional:u64)->Result<bool,ValueError>{self.work.prepare(additional,None)}
 /// 🪪️ Resolves an interrupted original quote without issuing it again.
 pub fn recover_work_quote(&mut self)->Result<Option<bool>,ValueError>{self.work.recover_quote()}
 /// ⏪️ Releases only an original reservation whose owner proves no effect occurred.
 pub fn cancel_work(&mut self)->Result<(),ValueError>{self.work.cancel()}
 pub(crate) fn cancel_prepared_work(&mut self){if self.work.state.has_live_actions(){self.work.cancel().expect("original work cancellation requires its issuer and resolved quote")}}
 /// 🧾️ Records exact actual work before any fallible physical receipt or result fold.
 pub fn commit_work(&mut self,actual:u128)->Result<(),ValueError>{self.work.commit(actual)}
 /// 🪤️ Reborrows the same source work issuer, including any held reservation.
 pub fn with_work_recipient<T,E:From<ValueError>>(&mut self,recipient:&mut OriginalWorkLoan<'_>,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{
  self.work.validate_recipient(recipient).map_err(E::from)?;let mut work_loan=recipient.reborrow();
  let mut construction=self.construction.as_mut().map(|loan|loan.reborrow());let mut forwarded=match &mut self.allocation{super::allocation::Binding::Local=>None,super::allocation::Binding::Forwarded(loan)=>Some(&mut **loan)};
  let control=NativeDecodeControl{ledger:self.ledger.take(),scope_maximum:self.scope_maximum,construction:construction.as_mut(),receiving:self.receiving,maximum_bytes:self.maximum_bytes,completed:self.completed,total:self.total,started:self.started,stage:self.stage,depth:self.depth,callback:&mut *self.callback,retirement:self.retirement.as_deref_mut(),allocation:match forwarded.as_mut(){None=>super::allocation::Binding::Local,Some(loan)=>super::allocation::Binding::Forwarded(&mut **loan)},work:Work{state:{let mut state=self.work.state.take();state.begin_turn();state},recipient:Some(&mut work_loan)}};
  let mut scope=Scope{control,maximum:&mut self.maximum_bytes,scope_maximum:&mut self.scope_maximum,ledger:&mut self.ledger,completed:&mut self.completed,total:&mut self.total,started:&mut self.started,stage:&mut self.stage,depth:&mut self.depth,work_state:&mut self.work.state};operation(&mut scope.control)
 }
}
impl<'a> NativeDecodeControl<'a>{
 /// 🎟️ Installs one explicit caller work loan before any decoder effect is admitted, so a control that outlives one synchronous scope keeps the same loan like its retirement recipient.
 pub fn install_work_recipient(&mut self,recipient:&'a mut OriginalWorkLoan<'a>)->Result<(),ValueError>{
  self.work.validate_recipient(recipient)?;self.work.recipient=Some(recipient);Ok(())
 }
}
