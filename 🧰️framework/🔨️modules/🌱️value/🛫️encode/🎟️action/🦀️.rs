//! 🎟️ Encoder actions borrow the original caller's independent work loan.
use super::{NativeEncodeControl,ValueError,ValueRefusalKind};
pub use semio_framework_value_control_contract::{OriginalWorkAuthority,OriginalWorkActionIssuer,OriginalWorkIssuerIdentity,OriginalWorkLoan,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalSourceTurnIdentity};
pub(super) use semio_framework_value_control_contract::{OriginalWork as Action,OriginalWorkState as WorkState};
pub(super) fn prepare(action:&mut Action<'_>,additional:u64)->Result<bool,ValueError>{action.prepare(additional,None)}
pub(super) fn cancel(action:&mut Action<'_>)->Result<(),ValueError>{action.cancel()}
pub(super) fn commit(action:&mut Action<'_>,actual:u128)->Result<(),ValueError>{action.commit(actual)}
struct Scope<'owner,'call>{control:NativeEncodeControl<'call>,maximum:&'owner mut usize,scope_maximum:&'owner mut Option<usize>,ledger:&'owner mut Option<crate::native_receiving::NativeAllocationLedger>,completed:&'owner mut usize,total:&'owner mut usize,started:&'owner mut bool,stage:&'owner mut u64,depth:&'owner mut usize,work_state:&'owner mut WorkState}
impl Drop for Scope<'_, '_>{fn drop(&mut self){*self.maximum=self.control.maximum_bytes;*self.scope_maximum=self.control.scope_maximum;*self.ledger=self.control.ledger.take();*self.completed=self.control.completed;*self.total=self.control.total;*self.started=self.control.started;*self.stage=self.control.stage;*self.depth=self.control.depth;*self.work_state=self.control.action.state.take();}}
impl NativeEncodeControl<'_>{
 /// 🔭️ Reports committed concrete encoder effects, including effects before a refusal.
 pub fn actual_work(&self)->u128{self.action.actual()}
 /// 👁️ Reports whether the caller lent its original work recipient.
 pub fn has_work_recipient(&self)->bool{self.action.recipient.is_some()}
 /// 🪪️ Names the original work source for recovery.
 pub fn work_recipient_identity(&self)->Option<u64>{self.action.recipient.as_ref().map(|loan|loan.identity().get())}
 /// 🚦️ Reserves a bounded effect before changing its original output.
 pub fn prepare_work(&mut self,additional:u64)->Result<bool,ValueError>{
  prepare(&mut self.action,additional)
 }
 /// 🧾️ Commits actual output effects before any later fallible observer.
 pub fn commit_work(&mut self,actual:u128)->Result<(),ValueError>{
  commit(&mut self.action,actual)
 }
 /// ⏪️ Returns an unused reservation after a zero-effect refusal.
 pub fn cancel_work(&mut self)->Result<(),ValueError>{cancel(&mut self.action)}
 /// 🪢️ Borrows one caller recipient without replacing its physical or cancellation owners.
 pub fn with_work_recipient<T,E:From<ValueError>>(&mut self,loan:&mut OriginalWorkLoan<'_>,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<T,E>)->Result<T,E>{
  self.action.validate_recipient(loan).map_err(E::from)?;let mut work_loan=loan.reborrow();
  let mut forwarded=match &mut self.allocation{super::allocation::NativeEncodeAllocationBinding::Local=>None,super::allocation::NativeEncodeAllocationBinding::Forwarded(port)=>Some(&mut **port)};let mut scoped=self.scoped_allocation.as_mut().map(|port|&mut **port as &mut (dyn super::NativeSendAllocationLoan+'_));
  let mut construction=self.construction.as_mut().map(|loan|loan.reborrow());let control=NativeEncodeControl{scope_maximum:self.scope_maximum,retirement:self.retirement.as_deref_mut(),construction:construction.as_mut(),construction_identity:self.construction_identity,receiving:self.receiving,maximum_bytes:self.maximum_bytes,ledger:self.ledger.take(),completed:self.completed,total:self.total,started:self.started,stage:self.stage,depth:self.depth,source_depth:self.source_depth,callback:self.callback.as_mut().map(|port|&mut **port as &mut (dyn FnMut(super::NativeEncodeProgress)->bool+'_)),allocation:match forwarded.as_mut(){Some(port)=>super::allocation::NativeEncodeAllocationBinding::Forwarded(&mut **port),None=>super::allocation::NativeEncodeAllocationBinding::Local},scoped_allocation:scoped.as_mut().map(|port|&mut **port),work:self.work.as_mut().map(|port|&mut **port as &mut dyn FnMut(usize,usize)->Result<(),ValueError>),recorded_span:self.recorded_span.as_mut().map(|port|&mut **port as &mut (dyn crate::native_receiving::span::SendOriginalRecordedLoan<crate::native_receiving::span::OriginalSpanRequest,()>+'_)),recorded_checkpoint:self.recorded_checkpoint.as_mut().map(|port|&mut **port as &mut (dyn crate::native_receiving::span::SendOriginalRecordedLoan<crate::native_receiving::span::OriginalCheckpointRequest,bool>+'_)),source_turn:self.source_turn,source_witness:self.source_witness,action:Action{state:{let mut state=self.action.state.take();state.begin_turn();state},recipient:Some(&mut work_loan)}};
  let mut scope=Scope{control,maximum:&mut self.maximum_bytes,scope_maximum:&mut self.scope_maximum,ledger:&mut self.ledger,completed:&mut self.completed,total:&mut self.total,started:&mut self.started,stage:&mut self.stage,depth:&mut self.depth,work_state:&mut self.action.state};operation(&mut scope.control)
 }
}

