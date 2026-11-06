//! ⚙️ The actual original operation retains its scope, reservation and committed work through borrowed turns.
use super::{OriginalWorkScope,OriginalWorkActionReservation,OriginalWorkActionProof,OriginalWorkActionIdentity,OriginalWorkLoan,ValueError,ValueRefusalKind};
#[derive(Debug)]
pub struct OriginalWorkState{baseline:u128,quote_pending:bool,scope:Option<OriginalWorkScope>,action:Option<OriginalWorkActionReservation>}
impl OriginalWorkState{
 pub fn fresh()->Self{Self{scope:Some(OriginalWorkScope::new()),..Self::vacant()}}
 fn vacant()->Self{Self{baseline:0,quote_pending:false,scope:None,action:None}}
 pub fn take(&mut self)->Self{std::mem::replace(self,Self::vacant())}
 fn clear(&mut self){self.quote_pending=false}
 pub fn has_live_actions(&self)->bool{self.scope.as_ref().is_some_and(OriginalWorkScope::has_live_actions)}
 pub fn begin_turn(&mut self){self.baseline=self.actual()}
 pub fn quote_pending(&self)->bool{self.quote_pending}
 pub fn actual(&self)->u128{self.scope.as_ref().map_or(0,OriginalWorkScope::actual_work)}
 pub fn reserved(&self)->u64{self.action.as_ref().map_or(0,OriginalWorkActionReservation::reserved_work)}
 pub fn issuer_identity(&self)->Option<u64>{self.action.as_ref().map(|action|action.issuer_identity().get())}
 pub fn action_identity(&self)->Option<OriginalWorkActionIdentity>{self.action.as_ref().map(OriginalWorkActionReservation::action_identity)}
 /// 📊️ Reads the exact claimed work for this same original action.
 pub fn performed_total_for_action(&self,identity:Option<OriginalWorkActionIdentity>)->Result<u128,ValueError>{if identity.is_none()||identity!=self.action_identity(){return Err(invalid())}let scope=self.scope.as_ref().ok_or_else(invalid)?;let action=self.action.as_ref().ok_or_else(invalid)?;if !action.live||action.identity.scope!=scope.identity||action.performed_issued.is_some(){return Err(invalid())}Ok(action.paid)}
 /// 👁️ Reports whether this original action recorded a producer attempt.
 pub fn performed_seen_for_action(&self,identity:Option<OriginalWorkActionIdentity>)->Result<bool,ValueError>{self.performed_total_for_action(identity)?;Ok(self.action.as_ref().ok_or_else(invalid)?.partial_seen)}
 /// ⏳️ Reports the exact original action pending partial receipt.
 pub fn partial_pending_for_action(&self,identity:Option<OriginalWorkActionIdentity>)->Result<bool,ValueError>{self.performed_total_for_action(identity)?;Ok(self.action.as_ref().ok_or_else(invalid)?.partial.is_some())}
 /// 🎟️ Issues a caller-owned receipt for this exact original action before producer work.
 pub fn issue_performed_for_action(&mut self,identity:Option<OriginalWorkActionIdentity>,slot:&mut Option<OriginalWorkPerformedReceipt>)->Result<(),ValueError>{if self.quote_pending||identity.is_none()||identity!=self.action_identity()||slot.is_some(){return Err(invalid())}let scope=self.scope.as_ref().ok_or_else(invalid)?;let action=self.action.as_mut().ok_or_else(invalid)?;if !action.live||action.identity.scope!=scope.identity||action.observed.is_some()||action.partial.is_some()||action.performed_issued.is_some(){return Err(invalid())}let serial=action.next_performed;let next=serial.checked_add(1).ok_or_else(invalid)?;action.next_performed=next;action.performed_issued=Some(serial);*slot=Some(OriginalWorkPerformedReceipt{identity:action.identity,serial,source_turn:action.source_turn,base:action.paid,ceiling:action.reserved,delta:0,unaccounted:None,performed:false,claimed:false});Ok(())}
 /// 🧾️ Claims the same producer receipt and invoices its exact work without a Source callback.
 pub fn claim_performed_for_action(&mut self,identity:Option<OriginalWorkActionIdentity>,slot:&mut Option<OriginalWorkPerformedReceipt>)->Result<(),ValueError>{if identity.is_none()||identity!=self.action_identity(){return Err(invalid())}let receipt=slot.as_mut().ok_or_else(invalid)?;let scope=self.scope.as_mut().ok_or_else(invalid)?;let action=self.action.as_mut().ok_or_else(invalid)?;if receipt.claimed||receipt.identity!=action.identity||receipt.source_turn!=action.source_turn||action.identity.scope!=scope.identity||action.performed_issued!=Some(receipt.serial)||action.paid!=receipt.base||action.partial.is_some()||receipt.unaccounted.is_some(){return Err(invalid())}if receipt.performed{let total=action.paid.checked_add(receipt.delta).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original performed work total overflow"))?;let actual=scope.actual.checked_add(receipt.delta).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original scope work total overflow"))?;action.partial=Some(super::OriginalWorkPartialReceipt{serial:receipt.serial,delta:receipt.delta,total,admitted:total<=u128::from(action.reserved),unaccounted:None,started:false,entered:false,reply:None});action.paid=total;action.partial_seen=true;scope.actual=actual}action.performed_issued=None;receipt.claimed=true;drop(slot.take());Ok(())}

}
/// 🪪️ Opaque identity of one original performed-work receipt.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalWorkPerformedIdentity{pub(super) action:OriginalWorkActionIdentity,pub(super) serial:u64}
impl OriginalWorkPerformedIdentity{
 /// 🪪️ Observes the same original action without transferring its authority.
 pub fn action_identity(self)->OriginalWorkActionIdentity{self.action}
}
/// 🧾️ Caller-owned evidence of actual work against one original issued action.
pub struct OriginalWorkPerformedReceipt{identity:OriginalWorkActionIdentity,serial:u64,source_turn:Option<crate::OriginalSourceTurnIdentity>,base:u128,ceiling:u64,delta:u128,unaccounted:Option<u64>,performed:bool,claimed:bool}
impl OriginalWorkPerformedReceipt{
 /// 🪪️ Identifies this original performed-work receipt.
 pub fn identity(&self)->OriginalWorkPerformedIdentity{OriginalWorkPerformedIdentity{action:self.identity,serial:self.serial}}
 /// ⏱️ Observes the exact staged work before the receipt is claimed.
 pub fn performed_work(&self)->u128{self.delta}
 /// 🚧️ Retains a real producer delta that could not fit the wider counter.
 pub fn unaccounted_work(&self)->Option<u64>{self.unaccounted}
}
impl Drop for OriginalWorkPerformedReceipt{fn drop(&mut self){assert!(std::thread::panicking()||self.claimed,"original performed-work receipt requires its same action")}}
mod sealed_work_sink{pub trait Sealed{}}
/// ⏱️ A sealed receipt records actual work only against its original action.
pub trait OriginalWorkSink:sealed_work_sink::Sealed+Send{fn performed(&mut self,delta:u64)->Result<bool,ValueError>;fn observed_work(&self)->u128;fn remaining_work_units(&self)->Result<u64,ValueError>;fn identity(&self)->OriginalWorkPerformedIdentity;fn source_turn(&self)->Option<crate::OriginalSourceTurnIdentity>;}
impl sealed_work_sink::Sealed for OriginalWorkPerformedReceipt{}
impl OriginalWorkSink for OriginalWorkPerformedReceipt{
 fn performed(&mut self,delta:u64)->Result<bool,ValueError>{if self.claimed||self.unaccounted.is_some(){return Err(invalid())}let Some(next)=self.delta.checked_add(u128::from(delta))else{self.unaccounted=Some(delta);return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original performed-work receipt overflow"))};let Some(total)=self.base.checked_add(next)else{self.unaccounted=Some(delta);return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original performed-work total overflow"))};self.delta=next;self.performed=true;Ok(total<=u128::from(self.ceiling))}
 fn observed_work(&self)->u128{self.base+self.delta}
 fn remaining_work_units(&self)->Result<u64,ValueError>{if self.claimed||self.unaccounted.is_some(){return Err(invalid())}let total=self.base.checked_add(self.delta).ok_or_else(invalid)?;let remaining=u128::from(self.ceiling).checked_sub(total).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original work receipt exceeded its ceiling"))?;u64::try_from(remaining).map_err(|_|invalid())}
 fn identity(&self)->OriginalWorkPerformedIdentity{OriginalWorkPerformedReceipt::identity(self)}
 fn source_turn(&self)->Option<crate::OriginalSourceTurnIdentity>{self.source_turn}
}
/// 🧵️ Borrows one original work owner without exposing or replacing its state or issuer.
pub struct OriginalWorkTurn<'turn,'source>{original:&'turn mut OriginalWork<'source>}
impl<'turn,'source> OriginalWorkTurn<'turn,'source>{
 /// 🔗️ Binds the actual original work owner for one shorter turn.
 pub fn new(original:&'turn mut OriginalWork<'source>)->Self{Self{original}}
 /// 📊️ Observes the original committed work count.
 pub fn actual(&self)->u128{self.original.actual()}
 /// 🎟️ Begins or recovers the same issuer-bound action in its caller-owned slot.
 pub fn prepare(&mut self,amount:u64,action:&mut Option<OriginalWorkActionIdentity>)->Result<Option<bool>,ValueError>{self.original.prepare_for_action(amount,action)}
 /// 📊️ Reads claimed work from the same original action.
 pub fn performed_total_for_action(&self,identity:Option<OriginalWorkActionIdentity>)->Result<u128,ValueError>{self.original.state.performed_total_for_action(identity)}
 /// 👁️ Reports the same action original producer attempt.
 pub fn performed_seen_for_action(&self,identity:Option<OriginalWorkActionIdentity>)->Result<bool,ValueError>{self.original.state.performed_seen_for_action(identity)}
 /// ⏳️ Reports the same action pending partial receipt.
 pub fn partial_pending_for_action(&self,identity:Option<OriginalWorkActionIdentity>)->Result<bool,ValueError>{self.original.state.partial_pending_for_action(identity)}
 /// 🔐️ Validates the actual Source and the same original action before effect entry.
 pub fn validate_action_source(&self,identity:Option<OriginalWorkActionIdentity>)->Result<(),ValueError>{self.original.validate_action_source(identity)}
 /// ⏱️ Lends the same original action while the actual producer records CPU.
 pub fn issue_performed_for_action(&mut self,identity:Option<OriginalWorkActionIdentity>,slot:&mut Option<OriginalWorkPerformedReceipt>)->Result<(),ValueError>{self.original.state.issue_performed_for_action(identity,slot)}
 pub fn claim_performed_for_action(&mut self,identity:Option<OriginalWorkActionIdentity>,slot:&mut Option<OriginalWorkPerformedReceipt>)->Result<(),ValueError>{self.original.state.claim_performed_for_action(identity,slot)}
 /// 📥️ Installs the actual partial outcome before returning to the caller.
 pub fn settle_partial_for_action_into(&mut self,identity:Option<OriginalWorkActionIdentity>,output:&mut Option<super::OriginalWorkPartialOutcome>)->Result<bool,ValueError>{self.original.settle_partial_for_action_into(identity,output)}
 /// 📨️ Commits that exact action and retains its original proof.
 pub fn commit(&mut self,actual:u128,action:Option<OriginalWorkActionIdentity>,proof:&mut Option<OriginalWorkActionProof>)->Result<bool,ValueError>{self.original.commit_for_action(actual,action,proof)}
 /// 🔎️ Validates an existing proof against this same original issuer and scope.
 pub fn validates(&self,proof:&OriginalWorkActionProof,actual:u128)->Result<bool,ValueError>{self.original.validates(proof,actual)}
 /// ↩️ Cancels only the same original outstanding action.
 pub fn cancel(&mut self,action:&mut Option<OriginalWorkActionIdentity>)->Result<(),ValueError>{self.original.cancel_for_action(action)}
 /// 🏁️ Returns the proof to the same original issuer.
 pub fn finish(&mut self,proof:&mut Option<OriginalWorkActionProof>,action:&mut Option<OriginalWorkActionIdentity>)->Result<(),ValueError>{self.original.finish_for_action(proof,action)}
}
pub struct OriginalWork<'a>{pub state:OriginalWorkState,pub recipient:Option<&'a mut OriginalWorkLoan<'a>>}
impl std::ops::Deref for OriginalWork<'_>{type Target=OriginalWorkState;fn deref(&self)->&OriginalWorkState{&self.state}}
impl std::ops::DerefMut for OriginalWork<'_>{fn deref_mut(&mut self)->&mut OriginalWorkState{&mut self.state}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original work requires its original issuer and resolved quote")}
impl OriginalWork<'_>{
 pub fn empty()->Self{Self{state:OriginalWorkState::fresh(),recipient:None}}
 pub fn validate_recipient(&self,loan:&OriginalWorkLoan<'_>)->Result<(),ValueError>{loan.validate_context()?;if self.recipient.is_some()||self.issuer_identity().is_some_and(|id|id!=loan.identity().get())||(self.issuer_identity().is_some()&&(self.action_identity().is_none()||!loan.has_live_actions())){Err(invalid())}else{Ok(())}}
 /// 🔐️ Validates the borrowed Source and actual original action before effect entry.
 pub fn validate_action_source(&self,identity:Option<OriginalWorkActionIdentity>)->Result<(),ValueError>{if identity.is_none()||identity!=self.state.action_identity()||self.quote_pending||!self.same_issuer(){return Err(invalid())}self.recipient.as_ref().ok_or_else(invalid)?.validate_context()}
 pub fn same_issuer(&self)->bool{match(self.recipient.as_ref(),self.state.scope.as_ref(),self.state.action.as_ref()){(Some(loan),Some(scope),Some(action))=>loan.owns_reservation(scope,action),_=>false}}
 fn cancel_action(&mut self)->Result<(),ValueError>{let state=&mut self.state;let loan=self.recipient.as_deref_mut().ok_or_else(invalid)?;let scope=state.scope.as_mut().ok_or_else(invalid)?;let action=state.action.take().ok_or_else(invalid)?;match loan.cancel(scope,action){Ok(())=>{state.clear();Ok(())},Err((error,action))=>{state.action=Some(action);Err(error)}}}
 pub fn commit_retained(&mut self,actual:u128,output:&mut Option<OriginalWorkActionProof>)->Result<bool,ValueError>{if !self.same_issuer()||self.quote_pending{return Err(invalid())}let state=&mut self.state;let loan=self.recipient.as_deref_mut().ok_or_else(invalid)?;let scope=state.scope.as_mut().ok_or_else(invalid)?;let action=state.action.take().ok_or_else(invalid)?;let result=loan.commit(scope,action,actual,output);match result{Ok(admitted)=>{state.clear();Ok(admitted)},Err((error,action))=>{state.action=Some(action);Err(error)}}}
 /// 📨️ Settles the same resident partial receipt through the original producer.
 fn settle_partial_action(&mut self,identity:Option<OriginalWorkActionIdentity>)->Result<Option<super::OriginalWorkPartialOutcome>,ValueError>{if identity.is_none()||identity!=self.state.action_identity()||!self.same_issuer()||self.quote_pending{return Err(invalid())}let state=&mut self.state;let loan=self.recipient.as_deref_mut().ok_or_else(invalid)?;let scope=state.scope.as_ref().ok_or_else(invalid)?;let action=state.action.as_mut().ok_or_else(invalid)?;loan.settle_partial(scope,action)}
 /// 📥️ Retains the exact original partial outcome in the caller-owned slot.
 pub fn settle_partial_for_action_into(&mut self,identity:Option<OriginalWorkActionIdentity>,output:&mut Option<super::OriginalWorkPartialOutcome>)->Result<bool,ValueError>{if output.is_some(){return Err(invalid())}match self.settle_partial_action(identity)?{Some(outcome)=>{*output=Some(outcome);Ok(true)},None=>Ok(false)}}
 pub fn finish_proof(&mut self,output:&mut Option<OriginalWorkActionProof>)->Result<(),ValueError>{let loan=self.recipient.as_deref_mut().ok_or_else(invalid)?;let scope=self.state.scope.as_mut().ok_or_else(invalid)?;loan.finish(scope,output)}
 pub fn validates(&self,proof:&OriginalWorkActionProof,actual:u128)->Result<bool,ValueError>{match(self.recipient.as_ref(),self.state.scope.as_ref()){(Some(loan),Some(scope))=>loan.validates(scope,proof,actual),_=>Ok(false)}}
 pub fn commit(&mut self,actual:u128)->Result<(),ValueError>{let mut proof=None;let result=self.commit_retained(actual,&mut proof);if proof.is_some(){self.finish_proof(&mut proof)?}match result{Ok(true)=>Ok(()),Ok(false)=>Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original effect exceeded its original work preflight")),Err(error)=>Err(error)}}
 pub fn prepare_for_action(&mut self,additional:u64,action:&mut Option<OriginalWorkActionIdentity>)->Result<Option<bool>,ValueError>{
  if self.state.action.is_some(){if *action!=self.state.action_identity()||!self.same_issuer()||self.state.reserved()!=additional{return Err(invalid())}if !self.quote_pending{return Ok(Some(true))}let result=self.recover_quote();if self.state.action.is_none(){*action=None}return result}
  if action.is_some(){return Err(invalid())}let result=self.prepare(additional,Some(&mut *action)).map(Some);if self.state.action.is_none(){*action=None}result
 }
 pub fn cancel_for_action(&mut self,action:&mut Option<OriginalWorkActionIdentity>)->Result<(),ValueError>{if action.is_none()||*action!=self.state.action_identity(){return Err(invalid())}if self.quote_pending{if !self.same_issuer(){return Err(invalid())}self.cancel_action()?}else{self.cancel()?}*action=None;Ok(())}
 pub fn commit_for_action(&mut self,actual:u128,action:Option<OriginalWorkActionIdentity>,output:&mut Option<OriginalWorkActionProof>)->Result<bool,ValueError>{if action.is_none()||action!=self.state.action_identity(){return Err(invalid())}self.commit_retained(actual,output)}
 pub fn finish_for_action(&mut self,proof:&mut Option<OriginalWorkActionProof>,action:&mut Option<OriginalWorkActionIdentity>)->Result<(),ValueError>{if action.is_none()||proof.as_ref().map(OriginalWorkActionProof::action_identity)!=*action{return Err(invalid())}self.finish_proof(proof)?;*action=None;Ok(())}
 pub fn prepare(&mut self,additional:u64,output:Option<&mut Option<OriginalWorkActionIdentity>>)->Result<bool,ValueError>{
  if self.state.action.is_some()||output.as_ref().is_some_and(|slot|slot.is_some()){return Err(invalid())}
  self.state.actual().checked_add(u128::from(additional)).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original actual work overflow"))?;let observed=u64::try_from(self.state.actual().checked_sub(self.state.baseline).ok_or_else(invalid)?).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"original turn work exceeds its per-turn extent"))?;
  let state=&mut self.state;let loan=self.recipient.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original work requires its original caller loan"))?;let scope=state.scope.as_mut().ok_or_else(invalid)?;let action=loan.issue(scope,additional)?;if let Some(output)=output{*output=Some(action.action_identity())}state.action=Some(action);state.quote_pending=true;
  let result=loan.admits(scope,state.action.as_mut().expect("original work action issued"),observed,additional);match result{Ok(true)=>{self.state.quote_pending=false;Ok(true)},Ok(false)=>{self.cancel_action()?;Ok(false)},Err(error)=>{self.cancel_action()?;Err(error)}}
 }
 pub fn recover_quote(&mut self)->Result<Option<bool>,ValueError>{
  if !self.quote_pending||!self.same_issuer(){return Err(invalid())}
  let state=&mut self.state;let loan=self.recipient.as_deref_mut().ok_or_else(invalid)?;let reply=loan.recover(state.scope.as_ref().ok_or_else(invalid)?,state.action.as_mut().ok_or_else(invalid)?)?;
  match reply{None=>Ok(None),Some(Ok(true))=>{self.state.quote_pending=false;Ok(Some(true))},Some(Ok(false))=>{self.cancel_action()?;Ok(Some(false))},Some(Err(error))=>{self.cancel_action()?;Err(error)}}
 }
 pub fn cancel(&mut self)->Result<(),ValueError>{if !self.same_issuer()||self.quote_pending{return Err(invalid())}self.cancel_action()}
}
