//! 🏭️ Encoder construction lends one original physical recipient without replacing its grants.
use super::{NativeEncodeControl,ValueError,ValueRefusalKind};
use crate::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand};
/// 🫴️ Quotes the original physical grant and records every submitted effect, including refusals.
pub trait NativeEncodeConstructionRecipient{
 fn submitted(&self)->RetainedCloneProgress;
 fn grant(&mut self,demand:RetirementDemand)->Result<RetainedCloneGrant,ValueError>;
 fn accept(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>;
}
static CONSTRUCTION_ISSUER_IDS:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);
/// 🪪️ Retains the original physical source across short recipient loans.
pub struct NativeEncodeConstructionIssuer{identity:u64}
impl NativeEncodeConstructionIssuer{
 /// 📏️ Quotes only the initialized original issuer fields.
 pub const fn birth_bytes()->usize{std::mem::size_of::<u64>()}
 /// 🆕️ Creates a distinct original physical source.
 pub fn new()->Self{Self{identity:CONSTRUCTION_ISSUER_IDS.try_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|id|id.checked_add(1)).expect("construction issuer identity exhausted")}}
 /// 🫴️ Lends this source to its current construction recipient.
 pub fn loan<'a>(&'a mut self,recipient:&'a mut dyn NativeEncodeConstructionRecipient)->NativeEncodeConstructionLoan<'a>{NativeEncodeConstructionLoan{identity:self.identity,recipient}}
}
impl Default for NativeEncodeConstructionIssuer{fn default()->Self{Self::new()}}
/// 🎟️ Binds one construction recipient borrow to its durable source.
pub struct NativeEncodeConstructionLoan<'a>{identity:u64,recipient:&'a mut dyn NativeEncodeConstructionRecipient}
struct Scope<'owner,'call>{control:NativeEncodeControl<'call>,maximum:&'owner mut usize,scope_maximum:&'owner mut Option<usize>,owned:&'owner mut usize,completed:&'owner mut usize,total:&'owner mut usize,started:&'owner mut bool,stage:&'owner mut u64,depth:&'owner mut usize,actual:&'owner mut u64,reserved:&'owner mut u64}
impl Drop for Scope<'_, '_>{fn drop(&mut self){*self.maximum=self.control.maximum_bytes;*self.scope_maximum=self.control.scope_maximum;*self.owned=self.control.owned_bytes;*self.completed=self.control.completed;*self.total=self.control.total;*self.started=self.control.started;*self.stage=self.control.stage;*self.depth=self.control.depth;*self.actual=self.control.action.actual;*self.reserved=self.control.action.reserved;}}
impl NativeEncodeControl<'_>{
 /// 🪪️ Names the original physical source for recovery.
 pub fn construction_recipient_identity(&self)->Option<u64>{self.construction_identity}
 /// 🔭️ Reads the original physical recipient’s actual submitted total.
 pub fn construction_submitted(&self)->Result<RetainedCloneProgress,ValueError>{self.construction.as_deref().map(NativeEncodeConstructionRecipient::submitted).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native encoder requires its original construction recipient"))}
 /// 🎟️ Quotes the same original physical recipient before a concrete effect.
 pub fn construction_grant(&mut self,demand:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native encoder requires its original construction recipient"))?.grant(demand)}
 /// 🧾️ Submits an already-performed effect under its previously issued original grant.
 pub fn submit_constructed_progress(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native encoder requires its original construction recipient"))?.accept(grant,progress)}
 /// 🪢️ Borrows the actual physical recipient through this synchronous original owner.
 pub fn with_construction_recipient<T,E:From<ValueError>>(&mut self,loan:&mut NativeEncodeConstructionLoan<'_>,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<T,E>)->Result<T,E>{
  if self.construction.is_some(){return Err(E::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"native encoder cannot replace its original construction recipient")))}
  let mut forwarded=match &mut self.allocation{super::allocation::Binding::Local=>None,super::allocation::Binding::Forwarded(port)=>Some(port.reborrow())};let mut scoped=self.scoped_allocation.as_mut().map(|port|port.reborrow());
  let control=NativeEncodeControl{scope_maximum:self.scope_maximum,retirement:self.retirement.as_deref_mut(),construction:Some(&mut *loan.recipient),construction_identity:Some(loan.identity),receiving:self.receiving,maximum_bytes:self.maximum_bytes,owned_bytes:self.owned_bytes,completed:self.completed,total:self.total,started:self.started,stage:self.stage,depth:self.depth,source_depth:self.source_depth,callback:&mut *self.callback,allocation:match forwarded.as_mut(){Some(port)=>super::allocation::Binding::Forwarded(port),None=>super::allocation::Binding::Local},scoped_allocation:scoped.as_mut(),work:self.work.as_mut().map(|port|&mut **port as &mut dyn FnMut(usize,usize)->Result<(),ValueError>),action:super::action::Action{actual:self.action.actual,baseline:self.action.baseline,reserved:self.action.reserved,identity:self.action.identity,recipient:self.action.recipient.as_mut().map(|port|&mut **port as &mut dyn super::NativeEncodeWorkRecipient)}};
  let mut scope=Scope{control,maximum:&mut self.maximum_bytes,scope_maximum:&mut self.scope_maximum,owned:&mut self.owned_bytes,completed:&mut self.completed,total:&mut self.total,started:&mut self.started,stage:&mut self.stage,depth:&mut self.depth,actual:&mut self.action.actual,reserved:&mut self.action.reserved};operation(&mut scope.control)
 }
}
