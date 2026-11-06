//! 🎟️ Decoder work retains its original issuer, pending quote and actual accounting across turns.
use super::{NativeDecodeControl,ValueError,ValueRefusalKind};
pub use semio_framework_value_control_contract::{OriginalWorkAuthority,OriginalWorkIssuerIdentity,OriginalWorkActionIssuer,OriginalWorkLoan,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete};
/// 🫴️ The caller admits observed work against its genuine current loan and performs no debit here.
pub trait NativeDecodeWorkRecipient:Send{fn available_work(&mut self,observed_since_scope:u64)->Result<u64,ValueError>;fn admits(&mut self,observed_since_scope:u64,additional:u64)->Result<bool,ValueError>;}
/// 🪪️ An opaque identity can only be observed from the actual work source or its live loan.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeDecodeWorkIdentity(u64);
/// 🪪️ A source retains one non-reusable work authority across short turn loans.
pub struct NativeDecodeWorkIssuer{identity:u64,next_action:u64,outstanding:usize}
impl NativeDecodeWorkIssuer{
 /// 📏️ Quotes only the initialized original issuer fields.
 pub const fn birth_bytes()->usize{2*std::mem::size_of::<u64>()+std::mem::size_of::<usize>()}
 /// 🏷️ Creates an original caller-owned work issuer.
 pub fn new()->Self{static NEXT:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);Self{identity:NEXT.fetch_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|value|value.checked_add(1)).expect("native work issuer identity exhausted"),next_action:1,outstanding:0}}
 /// 🔗️ Delegates the original work authority to its actual turn backend.
 pub fn loan<'a>(&'a mut self,recipient:&'a mut dyn NativeDecodeWorkRecipient)->NativeDecodeWorkLoan<'a>{let identity=self.identity;NativeDecodeWorkLoan{identity,recipient,issuer:self}}
 /// 🪪️ Observes this actual work source without creating another authority.
 pub fn identity(&self)->NativeDecodeWorkIdentity{NativeDecodeWorkIdentity(self.identity)}
 /// ⏳️ Reports whether the original source still owns a quote or reservation.
 pub fn has_pending_work(&self)->bool{self.outstanding!=0}
 /// 🧮️ Counts the original source actions still awaiting resolution.
 pub fn outstanding(&self)->usize{self.outstanding}
}
impl Default for NativeDecodeWorkIssuer{fn default()->Self{Self::new()}}
impl Drop for NativeDecodeWorkIssuer{fn drop(&mut self){assert!(std::thread::panicking()||self.outstanding==0,"native work issuer retains original pending authority")}}
/// 🎟️ Couples an exclusive issuer borrow to the actual original work backend.
pub struct NativeDecodeWorkLoan<'a>{identity:u64,recipient:&'a mut dyn NativeDecodeWorkRecipient,issuer:&'a mut NativeDecodeWorkIssuer}
impl NativeDecodeWorkLoan<'_>{/// 🪪️ Observes the source identity carried by this actual live loan.
 pub fn identity(&self)->NativeDecodeWorkIdentity{NativeDecodeWorkIdentity(self.identity)}
 pub(super) fn reborrow(&mut self)->NativeDecodeWorkLoan<'_>{NativeDecodeWorkLoan{identity:self.identity,recipient:&mut *self.recipient,issuer:&mut *self.issuer}}}
pub(super) struct Work<'a>{pub actual:u64,pub baseline:u64,pub reserved:u64,pub reserved_issuer:Option<u64>,pub reserved_action:Option<u64>,pub quote_pending:bool,pub recipient:Option<&'a mut NativeDecodeWorkLoan<'a>>}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native work requires its original issuer and resolved quote")}
impl Work<'_>{
 pub fn empty(actual:u64)->Self{Self{actual,baseline:actual,reserved:0,reserved_issuer:None,reserved_action:None,quote_pending:false,recipient:None}}
 fn same_issuer(&self)->bool{self.recipient.as_ref().is_some_and(|loan|Some(loan.identity)==self.reserved_issuer&&self.reserved_action.is_some()&&loan.issuer.outstanding!=0)}
 fn clear(&mut self){if let Some(loan)=self.recipient.as_deref_mut(){loan.issuer.outstanding-=1}self.reserved=0;self.reserved_issuer=None;self.reserved_action=None;self.quote_pending=false}
 pub fn commit(&mut self,actual:u64)->Result<(),ValueError>{if !self.same_issuer()||self.quote_pending{return Err(invalid())}let next=self.actual.checked_add(actual).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"native decode actual work overflow"))?;let admitted=actual<=self.reserved;self.actual=next;self.clear();if !admitted{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native decode effect exceeded its original work preflight"))}Ok(())}
 fn cancel(&mut self)->Result<(),ValueError>{if !self.same_issuer()||self.quote_pending{return Err(invalid())}self.clear();Ok(())}
}
struct Scope<'owner,'call>{control:NativeDecodeControl<'call>,maximum:&'owner mut usize,scope_maximum:&'owner mut Option<usize>,ledger:&'owner mut Option<semio_framework_value_native_encoding::native_receiving::NativeAllocationLedger>,completed:&'owner mut usize,total:&'owner mut usize,started:&'owner mut bool,stage:&'owner mut u64,depth:&'owner mut usize,actual:&'owner mut u64,reserved:&'owner mut u64,reserved_issuer:&'owner mut Option<u64>,reserved_action:&'owner mut Option<u64>,quote_pending:&'owner mut bool}
impl Drop for Scope<'_, '_>{fn drop(&mut self){*self.maximum=self.control.maximum_bytes;*self.scope_maximum=self.control.scope_maximum;*self.ledger=self.control.ledger.take();*self.completed=self.control.completed;*self.total=self.control.total;*self.started=self.control.started;*self.stage=self.control.stage;*self.depth=self.control.depth;*self.actual=self.control.work.actual;*self.reserved=self.control.work.reserved;*self.reserved_issuer=self.control.work.reserved_issuer;*self.reserved_action=self.control.work.reserved_action;*self.quote_pending=self.control.work.quote_pending;self.control.work.reserved=0;self.control.work.reserved_issuer=None;self.control.work.reserved_action=None;self.control.work.quote_pending=false;}}
impl NativeDecodeControl<'_>{
 /// 🔭️ Borrows the actual scoped CPU ceiling without issuing, reserving or spending another action.
 pub fn available_work_units(&mut self)->Result<u64,ValueError>{if self.work.reserved_issuer.is_some(){if !self.work.same_issuer()||self.work.quote_pending{return Err(invalid())}return Ok(self.work.reserved)}let observed=self.work.actual.checked_sub(self.work.baseline).ok_or_else(invalid)?;let loan=self.work.recipient.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native work selection requires its actual caller loan"))?;loan.recipient.available_work(observed)}
 /// 🧮️ Counts committed defining effects, including those preceding an owned failure.
 pub fn actual_work(&self)->u64{self.work.actual}
 /// 👁️ Reports whether the original caller supplied a work loan.
 pub fn has_work_recipient(&self)->bool{self.work.recipient.is_some()}
 /// ⏳️ Reports a pending original quote or admitted work reservation.
 pub fn has_pending_work(&self)->bool{self.work.reserved_issuer.is_some()}
 /// 🎟️ Checks whether the same original work issuer already admitted this child effect.
 pub fn prepared_work_admits(&self,amount:u64)->bool{self.work.same_issuer()&&!self.work.quote_pending&&self.work.reserved>=amount}
 /// 📏️ Reports the exact work quantity still held by the original source.
 pub fn pending_work(&self)->u64{self.work.reserved}
 /// 🚦️ Reports an original work quote callback that has not yet resolved.
 pub fn work_quote_pending(&self)->bool{self.work.quote_pending}
 /// 🚦️ Retains the original quote boundary before invoking its work backend.
 pub fn prepare_work(&mut self,additional:u64)->Result<bool,ValueError>{if self.work.reserved_issuer.is_some(){return Err(invalid())}self.work.actual.checked_add(additional).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"native decode actual work overflow"))?;let observed=self.work.actual.checked_sub(self.work.baseline).ok_or_else(invalid)?;let loan=self.work.recipient.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native decode work requires its original caller loan"))?;let next=loan.issuer.next_action.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native work action identity exhausted"))?;let count=loan.issuer.outstanding.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native work action count exhausted"))?;let action=loan.issuer.next_action;loan.issuer.next_action=next;loan.issuer.outstanding=count;self.work.reserved=additional;self.work.reserved_issuer=Some(loan.identity);self.work.reserved_action=Some(action);self.work.quote_pending=true;let result=loan.recipient.admits(observed,additional);match result{Ok(true)=>{self.work.quote_pending=false;Ok(true)},Ok(false)=>{self.work.clear();Ok(false)},Err(error)=>{self.work.clear();Err(error)}}}
 /// 🪪️ Resolves an interrupted original quote without issuing it again.
 pub fn resolve_work_quote(&mut self,result:Result<bool,ValueError>)->Result<Result<bool,ValueError>,(ValueError,Result<bool,ValueError>)>{if !self.work.quote_pending||!self.work.same_issuer(){return Err((invalid(),result))}match result{Ok(true)=>{self.work.quote_pending=false;Ok(Ok(true))},Ok(false)=>{self.work.clear();Ok(Ok(false))},Err(error)=>{self.work.clear();Ok(Err(error))}}}
 /// ⏪️ Releases only an original reservation whose owner proves no effect occurred.
 pub fn cancel_work(&mut self)->Result<(),ValueError>{self.work.cancel()}
 pub(crate) fn cancel_prepared_work(&mut self){if self.work.reserved_issuer.is_some(){self.work.cancel().expect("original work cancellation requires its issuer and resolved quote")}}
 /// 🧾️ Records exact actual work before any fallible physical receipt or result fold.
 pub fn commit_work(&mut self,actual:u64)->Result<(),ValueError>{self.work.commit(actual)}
 /// 🪤️ Reborrows the same source work issuer, including any held reservation.
 pub fn with_work_recipient<T,E:From<ValueError>>(&mut self,recipient:&mut NativeDecodeWorkLoan<'_>,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{
  if self.work.recipient.is_some()||self.work.reserved_issuer.is_some_and(|id|id!=recipient.identity)||(self.work.reserved_issuer.is_some()&&(self.work.reserved_action.is_none()||recipient.issuer.outstanding==0)){return Err(E::from(invalid()))}
  let mut construction=self.construction.as_mut().map(|loan|loan.reborrow());let mut forwarded=match &mut self.allocation{super::allocation::Binding::Local=>None,super::allocation::Binding::Forwarded(loan)=>Some(&mut **loan)};let mut work_loan=recipient.reborrow();
  let control=NativeDecodeControl{ledger:self.ledger.take(),scope_maximum:self.scope_maximum,construction:construction.as_mut(),receiving:self.receiving,maximum_bytes:self.maximum_bytes,completed:self.completed,total:self.total,started:self.started,stage:self.stage,depth:self.depth,callback:&mut *self.callback,retirement:self.retirement.as_deref_mut(),allocation:match forwarded.as_mut(){None=>super::allocation::Binding::Local,Some(loan)=>super::allocation::Binding::Forwarded(&mut **loan)},work:Work{actual:self.work.actual,baseline:self.work.actual,reserved:self.work.reserved,reserved_issuer:self.work.reserved_issuer,reserved_action:self.work.reserved_action,quote_pending:self.work.quote_pending,recipient:Some(&mut work_loan)}};
  let mut scope=Scope{control,maximum:&mut self.maximum_bytes,scope_maximum:&mut self.scope_maximum,ledger:&mut self.ledger,completed:&mut self.completed,total:&mut self.total,started:&mut self.started,stage:&mut self.stage,depth:&mut self.depth,actual:&mut self.work.actual,reserved:&mut self.work.reserved,reserved_issuer:&mut self.work.reserved_issuer,reserved_action:&mut self.work.reserved_action,quote_pending:&mut self.work.quote_pending};operation(&mut scope.control)
 }
}
