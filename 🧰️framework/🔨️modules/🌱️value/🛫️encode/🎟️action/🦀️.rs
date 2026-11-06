//! 🎟️ Encoder actions borrow the original caller's independent work loan.
use super::{NativeEncodeControl,ValueError,ValueRefusalKind};
/// 🫴️ Admits concrete encoder effects and receives each actual commit before later observers.
pub trait NativeEncodeWorkRecipient{
 /// 🚦️ Checks the original caller’s work before the effect; `observed_since_scope` includes prior commits, so a recipient that debits on commit must not debit them again.
 fn admits(&mut self,observed_since_scope:u64,additional:u64)->Result<bool,ValueError>;
 /// 🧾️ Posts actual work or explicitly delegates its settlement to the enclosing caller.
 fn committed(&mut self,actual:u64)->Result<(),ValueError>;
}
static WORK_ISSUER_IDS:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);
/// 🪪️ Retains original work authority across short callback loans.
pub struct NativeEncodeWorkIssuer{identity:u64}
impl NativeEncodeWorkIssuer{
 /// 📏️ Quotes only the initialized original issuer fields.
 pub const fn birth_bytes()->usize{std::mem::size_of::<u64>()}
 /// 🆕️ Creates one original work authority.
 pub fn new()->Self{Self{identity:WORK_ISSUER_IDS.try_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|id|id.checked_add(1)).expect("work issuer identity exhausted")}}
 /// 🫴️ Lends this authority to its current work recipient.
 pub fn loan<'a>(&'a mut self,recipient:&'a mut dyn NativeEncodeWorkRecipient)->NativeEncodeWorkLoan<'a>{NativeEncodeWorkLoan{identity:self.identity,recipient}}
}
impl Default for NativeEncodeWorkIssuer{fn default()->Self{Self::new()}}
/// 🎟️ Binds a short recipient borrow to its persistent original authority.
pub struct NativeEncodeWorkLoan<'a>{identity:u64,recipient:&'a mut dyn NativeEncodeWorkRecipient}
pub(super) struct Action<'a>{pub actual:u64,pub baseline:u64,pub reserved:u64,pub identity:Option<u64>,pub recipient:Option<&'a mut dyn NativeEncodeWorkRecipient>}
impl Action<'_>{pub fn empty(actual:u64)->Self{Self{actual,baseline:actual,reserved:0,identity:None,recipient:None}}}
pub(super) fn prepare(action:&mut Action<'_>,additional:u64)->Result<bool,ValueError>{if additional==0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native encode work reservation must be nonzero"))}if action.reserved!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native encoder already has a pending work reservation"))}action.actual.checked_add(additional).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"native encode work overflow"))?;let observed=action.actual.checked_sub(action.baseline).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native encode work baseline exceeds actual effects"))?;let recipient=action.recipient.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native encode work requires its original caller loan"))?;if !recipient.admits(observed,additional)?{return Ok(false)}action.reserved=additional;Ok(true)}
pub(super) fn cancel(action:&mut Action<'_>)->Result<(),ValueError>{if action.recipient.is_none()||action.reserved==0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native encoder has no pending work reservation"))}action.reserved=0;Ok(())}
pub(super) fn commit(action:&mut Action<'_>,actual:u64)->Result<(),ValueError>{let admitted=action.recipient.is_some()&&actual<=action.reserved;let next=action.actual.checked_add(actual).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"native encode work overflow"))?;action.reserved=0;action.actual=next;if !admitted{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native encode effect exceeded its original work reservation"))}match action.recipient.as_deref_mut(){Some(recipient)=>recipient.committed(actual),None=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native encoder lost its original work recipient"))}}
struct Scope<'owner,'call>{control:NativeEncodeControl<'call>,maximum:&'owner mut usize,scope_maximum:&'owner mut Option<usize>,owned:&'owner mut usize,completed:&'owner mut usize,total:&'owner mut usize,started:&'owner mut bool,stage:&'owner mut u64,depth:&'owner mut usize,actual:&'owner mut u64,reserved:&'owner mut u64}
impl Drop for Scope<'_, '_>{fn drop(&mut self){*self.maximum=self.control.maximum_bytes;*self.scope_maximum=self.control.scope_maximum;*self.owned=self.control.owned_bytes;*self.completed=self.control.completed;*self.total=self.control.total;*self.started=self.control.started;*self.stage=self.control.stage;*self.depth=self.control.depth;*self.actual=self.control.action.actual;*self.reserved=self.control.action.reserved;}}
impl NativeEncodeControl<'_>{
 /// 🔭️ Reports committed concrete encoder effects, including effects before a refusal.
 pub fn actual_work(&self)->u64{self.action.actual}
 /// 👁️ Reports whether the caller lent its original work recipient.
 pub fn has_work_recipient(&self)->bool{self.action.recipient.is_some()}
 /// 🪪️ Names the original work source for recovery.
 pub fn work_recipient_identity(&self)->Option<u64>{self.action.identity}
 /// 🚦️ Reserves a bounded effect before changing its original output.
 pub fn prepare_work(&mut self,additional:u64)->Result<bool,ValueError>{
  prepare(&mut self.action,additional)
 }
 /// 🧾️ Commits actual output effects before any later fallible observer.
 pub fn commit_work(&mut self,actual:u64)->Result<(),ValueError>{
  commit(&mut self.action,actual)
 }
 /// ⏪️ Returns an unused reservation after a zero-effect refusal.
 pub fn cancel_work(&mut self)->Result<(),ValueError>{cancel(&mut self.action)}
 /// 🪢️ Borrows one caller recipient without replacing its physical or cancellation owners.
 pub fn with_work_recipient<T,E:From<ValueError>>(&mut self,loan:&mut NativeEncodeWorkLoan<'_>,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<T,E>)->Result<T,E>{
  if self.action.recipient.is_some()||self.action.reserved!=0{return Err(E::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"native encode cannot replace its original work loan")))}
  let mut forwarded=match &mut self.allocation{super::allocation::Binding::Local=>None,super::allocation::Binding::Forwarded(port)=>Some(port.reborrow())};let mut scoped=self.scoped_allocation.as_mut().map(|port|port.reborrow());
  let control=NativeEncodeControl{scope_maximum:self.scope_maximum,retirement:self.retirement.as_deref_mut(),construction:self.construction.as_mut().map(|port|&mut **port as &mut dyn super::NativeEncodeConstructionRecipient),construction_identity:self.construction_identity,receiving:self.receiving,maximum_bytes:self.maximum_bytes,owned_bytes:self.owned_bytes,completed:self.completed,total:self.total,started:self.started,stage:self.stage,depth:self.depth,source_depth:self.source_depth,callback:&mut *self.callback,allocation:match forwarded.as_mut(){Some(port)=>super::allocation::Binding::Forwarded(port),None=>super::allocation::Binding::Local},scoped_allocation:scoped.as_mut(),work:self.work.as_mut().map(|port|&mut **port as &mut dyn FnMut(usize,usize)->Result<(),ValueError>),action:Action{actual:self.action.actual,baseline:self.action.actual,reserved:0,identity:Some(loan.identity),recipient:Some(&mut *loan.recipient)}};
  let mut scope=Scope{control,maximum:&mut self.maximum_bytes,scope_maximum:&mut self.scope_maximum,owned:&mut self.owned_bytes,completed:&mut self.completed,total:&mut self.total,started:&mut self.started,stage:&mut self.stage,depth:&mut self.depth,actual:&mut self.action.actual,reserved:&mut self.action.reserved};operation(&mut scope.control)
 }
}

