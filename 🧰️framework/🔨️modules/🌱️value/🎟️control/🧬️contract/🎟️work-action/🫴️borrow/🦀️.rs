//! 🫴️ A sealed borrow lends the actual [OriginalWork] owner across neutral cleanup erasure.
use crate::{OriginalWork,OriginalWorkState,OriginalWorkActionIdentity,OriginalWorkActionProof,OriginalWorkPerformedReceipt,OriginalWorkPartialOutcome,OriginalWorkSink,RetainedAllocationEffect,RetainedCloneGrant,ValueError,ValueRefusalKind};
/// 🌐️ Identifies the same actual Work owner and Source before any action is issued.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalWorkBindingIdentity{scope:crate::OriginalWorkScopeIdentity,issuer:crate::OriginalWorkIssuerIdentity,source:Option<crate::OriginalSourceTurnIdentity>}
impl OriginalWorkBindingIdentity{
 /// 🧬️ Observes only the sealed effect receipt, without constructing a Work issuer or credit.
 pub fn from_effect(effect:&RetainedAllocationEffect<'_>)->Self{let action=effect.work_identity().action_identity();Self{scope:action.scope,issuer:action.issuer,source:effect.work_source_turn()}}
 /// 🌐️ Borrows the actual optional Source dispatch without exposing its work authority.
 pub fn source_turn(&self)->Option<crate::OriginalSourceTurnIdentity>{self.source}
 /// 🪪️ Validates the same actual loan before issuing a separate physical action.
 pub fn validates_loan(&self,loan:&crate::OriginalWorkLoan<'_>)->Result<bool,ValueError>{loan.validate_context()?;Ok(self.issuer==loan.identity()&&self.source==loan.source_turn())}
 /// 🧷️ Matches a genuine effect from the same original Work owner across action boundaries.
 pub fn matches_effect(&self,effect:&RetainedAllocationEffect<'_>)->bool{let action=effect.work_identity().action_identity();self.scope==action.scope&&self.issuer==action.issuer&&self.source==effect.work_source_turn()}

}
trait Port{
 fn binding(&self)->Result<OriginalWorkBindingIdentity,ValueError>;
 fn state(&self)->&OriginalWorkState;
 fn state_mut(&mut self)->&mut OriginalWorkState;
 fn validate(&self,action:Option<OriginalWorkActionIdentity>)->Result<(),ValueError>;
 fn prepare(&mut self,additional:u64,action:&mut Option<OriginalWorkActionIdentity>)->Result<Option<bool>,ValueError>;
 fn settle(&mut self,action:Option<OriginalWorkActionIdentity>,output:&mut Option<OriginalWorkPartialOutcome>)->Result<bool,ValueError>;
 fn commit(&mut self,actual:u128,action:Option<OriginalWorkActionIdentity>,proof:&mut Option<OriginalWorkActionProof>)->Result<bool,ValueError>;
 fn cancel(&mut self,action:&mut Option<OriginalWorkActionIdentity>)->Result<(),ValueError>;
 fn finish(&mut self,proof:&mut Option<OriginalWorkActionProof>,action:&mut Option<OriginalWorkActionIdentity>)->Result<(),ValueError>;
 fn finish_retained(&mut self,proof:&mut Option<OriginalWorkActionProof>,action:&mut Option<OriginalWorkActionIdentity>)->Result<(),ValueError>;
 fn validates_finished(&self,proof:&OriginalWorkActionProof,actual:u128)->Result<bool,ValueError>;
 fn validates(&self,proof:&OriginalWorkActionProof,actual:u128)->Result<bool,ValueError>;
}
impl Port for OriginalWork<'_>{
 fn binding(&self)->Result<OriginalWorkBindingIdentity,ValueError>{let loan=self.recipient.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original Work binding requires its actual loan"))?;loan.validate_context()?;Ok(OriginalWorkBindingIdentity{scope:self.state.scope_identity().ok_or_else(||ValueError::invariant("original Work scope absent"))?,issuer:loan.identity(),source:loan.source_turn()})}

 fn state(&self)->&OriginalWorkState{&self.state}
 fn state_mut(&mut self)->&mut OriginalWorkState{&mut self.state}
 fn validate(&self,action:Option<OriginalWorkActionIdentity>)->Result<(),ValueError>{self.validate_action_source(action)}
 fn prepare(&mut self,additional:u64,action:&mut Option<OriginalWorkActionIdentity>)->Result<Option<bool>,ValueError>{self.prepare_for_action(additional,action)}
 fn settle(&mut self,action:Option<OriginalWorkActionIdentity>,output:&mut Option<OriginalWorkPartialOutcome>)->Result<bool,ValueError>{self.settle_partial_for_action_into(action,output)}
 fn commit(&mut self,actual:u128,action:Option<OriginalWorkActionIdentity>,proof:&mut Option<OriginalWorkActionProof>)->Result<bool,ValueError>{self.commit_for_action(actual,action,proof)}
 fn cancel(&mut self,action:&mut Option<OriginalWorkActionIdentity>)->Result<(),ValueError>{self.cancel_for_action(action)}
 fn finish(&mut self,proof:&mut Option<OriginalWorkActionProof>,action:&mut Option<OriginalWorkActionIdentity>)->Result<(),ValueError>{self.finish_for_action(proof,action)}
 fn finish_retained(&mut self,proof:&mut Option<OriginalWorkActionProof>,action:&mut Option<OriginalWorkActionIdentity>)->Result<(),ValueError>{self.finish_retained_for_action(proof,action)}
 fn validates_finished(&self,proof:&OriginalWorkActionProof,actual:u128)->Result<bool,ValueError>{OriginalWork::validates_finished(self,proof,actual)}
 fn validates(&self,proof:&OriginalWorkActionProof,actual:u128)->Result<bool,ValueError>{OriginalWork::validates(self,proof,actual)}
}
/// 🧷️ Retains neither a replacement issuer nor a replacement computational wallet.
pub struct OriginalWorkBorrow<'a>{port:&'a mut dyn Port,ceiling:Option<u128>}
impl<'a> OriginalWorkBorrow<'a>{
 /// 🔄️ Reborrows the same private original port with its unchanged absolute ceiling.
 pub fn reborrow(&mut self)->OriginalWorkBorrow<'_>{OriginalWorkBorrow{port:&mut*self.port,ceiling:self.ceiling}}
 /// 🔗️ Borrows only the defining original Work state and its actual Source recipient.
 pub fn new(original:&'a mut OriginalWork<'_>)->Self{Self{port:original,ceiling:None}}
 /// 🪟️ Narrows this borrowed turn against the same actual cumulative Work without creating credit.
 pub fn with_maximum_work(self,maximum:u64)->Result<Self,ValueError>{let ceiling=self.actual().checked_add(u128::from(maximum)).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original work window overflow"))?;Ok(self.with_ceiling(Some(ceiling)))}
 /// 📉️ Narrows the same original absolute work ceiling without replenishing it.
 pub fn with_ceiling(mut self,ceiling:Option<u128>)->Self{self.ceiling=match(self.ceiling,ceiling){(Some(a),Some(b))=>Some(a.min(b)),(a,b)=>a.or(b)};self}
 fn remaining_window(&self)->u128{self.ceiling.map_or(u128::MAX,|ceiling|ceiling.saturating_sub(self.actual()))}
 pub fn actual(&self)->u128{self.port.state().actual()}
 /// 🧬️ Reads only the validated original scope, loan issuer and live Source binding.
 pub fn binding_identity(&self)->Result<OriginalWorkBindingIdentity,ValueError>{self.port.binding()}
 pub fn action_identity(&self)->Option<OriginalWorkActionIdentity>{self.port.state().action_identity()}
 pub fn prepare(&mut self,additional:u64,action:&mut Option<OriginalWorkActionIdentity>)->Result<Option<bool>,ValueError>{if action.is_none()&&u128::from(additional)>self.remaining_window(){return Ok(Some(false))}self.port.prepare(additional,action)}
 pub fn validate_action_source(&self,action:Option<OriginalWorkActionIdentity>)->Result<(),ValueError>{self.port.validate(action)}

 /// 🧾️ Issues one genuine receipt only after the original action Source validates.
 pub fn issue_performed(&mut self,action:Option<OriginalWorkActionIdentity>,receipt:&mut Option<OriginalWorkPerformedReceipt>)->Result<(),ValueError>{self.port.validate(action)?;let maximum=self.remaining_window();self.port.state_mut().issue_performed_for_action(action,receipt)?;receipt.as_mut().expect("actual performed receipt issued").narrow_remaining(maximum);Ok(())}
 /// 🧾️ Claims actual performed work before any fallible Source callback can run.
 pub fn claim_performed(&mut self,action:Option<OriginalWorkActionIdentity>,receipt:&mut Option<OriginalWorkPerformedReceipt>)->Result<(),ValueError>{self.port.state_mut().claim_performed_for_action(action,receipt)}
 pub fn performed_total(&self,action:Option<OriginalWorkActionIdentity>)->Result<u128,ValueError>{self.port.state().performed_total_for_action(action)}
 pub fn performed_seen(&self,action:Option<OriginalWorkActionIdentity>)->Result<bool,ValueError>{self.port.state().performed_seen_for_action(action)}
 pub fn partial_pending(&self,action:Option<OriginalWorkActionIdentity>)->Result<bool,ValueError>{self.port.state().partial_pending_for_action(action)}
 pub fn settle_partial_into(&mut self,action:Option<OriginalWorkActionIdentity>,output:&mut Option<OriginalWorkPartialOutcome>)->Result<bool,ValueError>{self.port.settle(action,output)}
 pub fn commit(&mut self,actual:u128,action:Option<OriginalWorkActionIdentity>,proof:&mut Option<OriginalWorkActionProof>)->Result<bool,ValueError>{self.port.commit(actual,action,proof)}
 pub fn cancel(&mut self,action:&mut Option<OriginalWorkActionIdentity>)->Result<(),ValueError>{self.port.cancel(action)}
 pub fn validates(&self,proof:&OriginalWorkActionProof,actual:u128)->Result<bool,ValueError>{self.port.validates(proof,actual)}
 pub fn finish(&mut self,proof:&mut Option<OriginalWorkActionProof>,action:&mut Option<OriginalWorkActionIdentity>)->Result<(),ValueError>{self.port.finish(proof,action)}
 /// 🧾️ Returns the original action without moving its nonclone proof.
 pub fn finish_retained(&mut self,proof:&mut Option<OriginalWorkActionProof>,action:&mut Option<OriginalWorkActionIdentity>)->Result<(),ValueError>{self.port.finish_retained(proof,action)}
 /// 🔎️ Validates that same returned proof against this live Work Source.
 pub fn validates_finished(&self,proof:&OriginalWorkActionProof,actual:u128)->Result<bool,ValueError>{self.port.validates_finished(proof,actual)}
 /// 🛡️ Validates the actual Source before issuing a caller-resident performed receipt.
 pub fn performed_scope<'scope>(&'scope mut self,action:Option<OriginalWorkActionIdentity>,receipt:&'scope mut Option<OriginalWorkPerformedReceipt>,claim:&'scope mut Option<Result<u128,ValueError>>)->Result<OriginalWorkPerformedScope<'scope>,ValueError>{
  if claim.is_some(){return Err(ValueError::invariant("original performed claim remains occupied"))}self.port.validate(action)?;let ceiling=self.ceiling;self.port.state_mut().issue_performed_for_action(action,receipt)?;let observed=OriginalWorkSink::observed_work(receipt.as_ref().expect("issued original work receipt"));Ok(OriginalWorkPerformedScope{state:self.port.state_mut(),action,receipt,claim,ceiling,observed})
 }
}
/// ⏱️ Claims the exact original receipt before any enclosing Source loan can unwind.
semio_framework_value_control_reply::original_work_performed_scope_engine!();