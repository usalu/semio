//! 🏭️ Encoder construction lends one original physical recipient without replacing its grants.
use super::{NativeEncodeControl,ValueError,ValueRefusalKind};
use crate::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand};
pub use semio_framework_value_control_contract::{OriginalConstructionAuthority,OriginalConstructionRecipient,OriginalConstructionLoan,OriginalConstructionIssuerIdentity,OriginalConstructionAction,OriginalConstructionPhase,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionAccept,OriginalConstructionAcceptComplete};
struct Scope<'owner,'call>{control:NativeEncodeControl<'call>,maximum:&'owner mut usize,scope_maximum:&'owner mut Option<usize>,ledger:&'owner mut Option<crate::native_receiving::NativeAllocationLedger>,completed:&'owner mut usize,total:&'owner mut usize,started:&'owner mut bool,stage:&'owner mut u64,depth:&'owner mut usize,work_state:&'owner mut super::action::WorkState}
impl Drop for Scope<'_, '_>{fn drop(&mut self){*self.maximum=self.control.maximum_bytes;*self.scope_maximum=self.control.scope_maximum;*self.ledger=self.control.ledger.take();*self.completed=self.control.completed;*self.total=self.control.total;*self.started=self.control.started;*self.stage=self.control.stage;*self.depth=self.control.depth;*self.work_state=self.control.action.state.take();}}
/// 🎟️ Quotes one original construction action and either cancels it unspent or posts its actual progress, always settling the action.
fn quote_construction(loan:&mut OriginalConstructionLoan<'_>,work:&crate::OriginalWorkBorrow<'_>,demand:RetirementDemand,progress:Option<RetainedCloneProgress>)->Result<RetainedCloneGrant,ValueError>{
 let pending=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original construction reply is pending");
 let mut slot=None;loan.begin(demand,&mut slot,work)?;
 let outcome=(||{
  let action=slot.as_mut().ok_or_else(pending)?;loan.quote(action)?;
  let grant=loan.recover_grant(action)?.ok_or_else(pending)??;
  match progress{
   None=>loan.cancel(action,false,Default::default(),None)?,
   Some(progress)=>{if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original construction grant no longer admits the posted progress"))}loan.post(action,progress,(progress.retained_capacity_bytes!=0).then_some(progress.retained_capacity_bytes))?;loan.recover_accept(action)?.ok_or_else(pending)?;}
  }
  Ok(grant)
 })();
 if outcome.is_err(){if let Some(action)=slot.as_mut(){if matches!(action.phase(),OriginalConstructionPhase::Fresh|OriginalConstructionPhase::Granted){loan.cancel(action,false,Default::default(),None)?}}}
 if let Some(action)=slot.as_ref(){match action.phase(){OriginalConstructionPhase::GrantRefused=>loan.finish(&mut slot)?,OriginalConstructionPhase::Settled if action.accept_succeeded()==Some(true)=>loan.finish_accepted(&mut slot)?,OriginalConstructionPhase::Settled if action.recorded_accept_status().is_none()=>loan.finish(&mut slot)?,OriginalConstructionPhase::Settled if action.accept_succeeded()==Some(false)=>{let refusal=loan.finish_refused(&mut slot)?;return Err(refusal)},_=>{}}}
 outcome
}
impl NativeEncodeControl<'_>{
 /// 🪪️ Names the original physical source for recovery.
 pub fn construction_recipient_identity(&self)->Option<OriginalConstructionIssuerIdentity>{self.construction_identity}
 /// 🔭️ Reads the original physical recipient’s actual submitted total.
 pub fn construction_submitted(&self)->Result<RetainedCloneProgress,ValueError>{self.construction.as_deref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native encoder requires its original construction recipient"))?.submitted()}
 /// 🎟️ Quotes the real original recipient without introducing another payer.
 pub fn construction_grant(&mut self,demand:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{let loan=self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native encoder requires its original construction recipient"))?;quote_construction(loan,&crate::OriginalWorkBorrow::new(&mut self.action),demand,None)}
 /// 🧾️ Publishes already-performed fresh effects under the caller's previously issued grant.
 pub fn submit_constructed_progress(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{
  if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"constructed progress exceeds its original grant"))}
  let loan=self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native encoder requires its original construction recipient"))?;let demand=RetirementDemand{copy_bytes:progress.copied_bytes,capacity_bytes:progress.retained_capacity_bytes,release_bytes:progress.released_bytes,depth:usize::from(grant.maximum_depth>0)};quote_construction(loan,&crate::OriginalWorkBorrow::new(&mut self.action),demand,Some(progress)).map(|_|())
 }
 /// 🪢️ Borrows the actual physical recipient through this synchronous original owner.
 pub fn with_construction_recipient<T,E:From<ValueError>>(&mut self,loan:&mut OriginalConstructionLoan<'_>,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<T,E>)->Result<T,E>{
  if self.construction.is_some(){return Err(E::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"native encoder cannot replace its original construction recipient")))}
  let mut forwarded=match &mut self.allocation{super::allocation::NativeEncodeAllocationBinding::Local=>None,super::allocation::NativeEncodeAllocationBinding::Forwarded(port)=>Some(&mut **port)};let mut scoped=self.scoped_allocation.as_mut().map(|port|&mut **port as &mut (dyn super::NativeSendAllocationLoan+'_));
  let identity=loan.identity();let mut construction=loan.reborrow();let mut action_loan=self.action.recipient.as_mut().map(|loan|loan.reborrow());let control=NativeEncodeControl{scope_maximum:self.scope_maximum,retirement:self.retirement.as_deref_mut(),construction:Some(&mut construction),construction_identity:Some(identity),receiving:self.receiving,maximum_bytes:self.maximum_bytes,ledger:self.ledger.take(),completed:self.completed,total:self.total,started:self.started,stage:self.stage,depth:self.depth,source_depth:self.source_depth,callback:self.callback.as_mut().map(|port|&mut **port as &mut (dyn FnMut(super::NativeEncodeProgress)->bool+'_)),allocation:match forwarded.as_mut(){Some(port)=>super::allocation::NativeEncodeAllocationBinding::Forwarded(&mut **port),None=>super::allocation::NativeEncodeAllocationBinding::Local},scoped_allocation:scoped.as_mut().map(|port|&mut **port),work:self.work.as_mut().map(|port|&mut **port as &mut dyn FnMut(usize,usize)->Result<(),ValueError>),recorded_span:self.recorded_span.as_mut().map(|port|&mut **port as &mut (dyn crate::native_receiving::span::SendOriginalRecordedLoan<crate::native_receiving::span::OriginalSpanRequest,()>+'_)),recorded_checkpoint:self.recorded_checkpoint.as_mut().map(|port|&mut **port as &mut (dyn crate::native_receiving::span::SendOriginalRecordedLoan<crate::native_receiving::span::OriginalCheckpointRequest,bool>+'_)),source_turn:self.source_turn,action:super::action::Action{state:self.action.state.take(),recipient:action_loan.as_mut()}};
  let mut scope=Scope{control,maximum:&mut self.maximum_bytes,scope_maximum:&mut self.scope_maximum,ledger:&mut self.ledger,completed:&mut self.completed,total:&mut self.total,started:&mut self.started,stage:&mut self.stage,depth:&mut self.depth,work_state:&mut self.action.state};operation(&mut scope.control)
 }
}
