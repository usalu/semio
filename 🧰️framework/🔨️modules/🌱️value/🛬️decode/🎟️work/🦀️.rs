//! 🎟️ Decoder effects borrow one actual caller work loan independently of physical storage grants.
use super::{NativeDecodeControl,ValueError,ValueRefusalKind};
/// 🫴️ The caller admits observed work against its genuine current loan and performs no debit here.
pub trait NativeDecodeWorkRecipient:Send{fn admits(&mut self,observed_since_scope:u64,additional:u64)->Result<bool,ValueError>;}
pub(super) struct Work<'a>{pub actual:u64,pub baseline:u64,pub reserved:u64,pub recipient:Option<&'a mut dyn NativeDecodeWorkRecipient>}
impl Work<'_>{
 pub fn empty(actual:u64)->Self{Self{actual,baseline:actual,reserved:0,recipient:None}}
}
struct Scope<'owner,'call>{control:NativeDecodeControl<'call>,maximum:&'owner mut usize,scope_maximum:&'owner mut Option<usize>,owned:&'owner mut usize,completed:&'owner mut usize,total:&'owner mut usize,started:&'owner mut bool,stage:&'owner mut u64,depth:&'owner mut usize,actual:&'owner mut u64}
impl Drop for Scope<'_, '_>{fn drop(&mut self){*self.maximum=self.control.maximum_bytes;*self.scope_maximum=self.control.scope_maximum;*self.owned=self.control.owned_bytes;*self.completed=self.control.completed;*self.total=self.control.total;*self.started=self.control.started;*self.stage=self.control.stage;*self.depth=self.control.depth;*self.actual=self.control.work.actual;}}
impl NativeDecodeControl<'_>{
 /// 🔭️ Counts committed defining effects, including those preceding an owned failure.
 pub fn actual_work(&self)->u64{self.work.actual}
 /// 🚦️ Preflights a known bounded effect against the same live caller loan before touching its owner.
 pub fn prepare_work(&mut self,additional:u64)->Result<bool,ValueError>{
  if self.work.reserved!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native decoder already has a pending work preflight"))}
  self.work.actual.checked_add(additional).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"native decode actual work overflow"))?;
  let observed=self.work.actual.checked_sub(self.work.baseline).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native decode work baseline exceeds actual effects"))?;
  let recipient=self.work.recipient.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native decode work requires its original caller loan"))?;
  if !recipient.admits(observed,additional)?{return Ok(false)}self.work.reserved=additional;Ok(true)
 }
 /// 🧾️ Records the actual selected effect before any fallible physical receipt or result fold.
 pub fn commit_work(&mut self,actual:u64)->Result<(),ValueError>{
  let admitted=self.work.recipient.is_some()&&actual<=self.work.reserved;
  let next=self.work.actual.checked_add(actual).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"native decode actual work overflow"))?;self.work.reserved=0;self.work.actual=next;
  if !admitted{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native decode effect exceeded its original work preflight"))}Ok(())
 }
 /// 🪢️ Borrows a real work payer for one synchronous scope and preserves actual observations on errors and unwind.
 pub fn with_work_recipient<T,E:From<ValueError>>(&mut self,recipient:&mut dyn NativeDecodeWorkRecipient,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{
  if self.work.recipient.is_some()||self.work.reserved!=0{return Err(E::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"native decode cannot replace its original work loan")))}
  let control=NativeDecodeControl{ledger:self.ledger,scope_maximum:self.scope_maximum,construction:self.construction.as_deref_mut().map(|p|p as &mut dyn super::NativeDecodeConstructionRecipient),receiving:self.receiving,maximum_bytes:self.maximum_bytes,owned_bytes:self.owned_bytes,completed:self.completed,total:self.total,started:self.started,stage:self.stage,depth:self.depth,callback:&mut *self.callback,retirement:self.retirement.as_deref_mut(),allocation:match &mut self.allocation{super::allocation::Binding::Local=>super::allocation::Binding::Local,super::allocation::Binding::Forwarded(p)=>super::allocation::Binding::Forwarded(&mut **p)},work:Work{actual:self.work.actual,baseline:self.work.actual,reserved:0,recipient:Some(recipient)}};
  let mut scope=Scope{control,maximum:&mut self.maximum_bytes,scope_maximum:&mut self.scope_maximum,owned:&mut self.owned_bytes,completed:&mut self.completed,total:&mut self.total,started:&mut self.started,stage:&mut self.stage,depth:&mut self.depth,actual:&mut self.work.actual};operation(&mut scope.control)
 }
}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
