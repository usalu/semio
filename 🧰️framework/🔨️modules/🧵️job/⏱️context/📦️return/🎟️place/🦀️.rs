//! 🎟️ One original source prepays fixed return capture and receiving without a context-free callback.
use crate::{StepContext,OriginalTurnFailure,OriginalSourceBinding};
use crate::original_job_continuation::{OriginalReturnClaimCredit,OriginalReturnWitness,OriginalReturnPhase,claim_original_return,receive_original_return_witness};
use semio_framework_value_retained_clone::{ValueError,ValueRefusalKind};

/// 🪙️ Only the genuine source can hold these non-clone currencies until its exact return is received.
pub struct OriginalCallerReturnPlace{credit:Option<OriginalReturnClaimCredit>,witness:Option<OriginalReturnWitness>,identity:Option<OriginalSourceBinding>}
impl OriginalCallerReturnPlace{
 pub const fn empty()->Self{Self{credit:None,witness:None,identity:None}}
 pub fn terminal_is_empty(&self)->bool{self.credit.is_none()&&self.witness.is_none()&&self.identity.is_none()}
 pub fn is_admitted(&self)->bool{self.credit.is_some()}
 pub fn admit(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if let Some(credit)=self.credit.as_ref(){credit.qualify_source(cx,self.identity.unwrap())?;return Ok(credit.has_unused_capture()&&self.witness.is_none())}
  let parent=cx.original_source_parent()?;let bound=cx.return_budget.as_ref().is_some_and(|budget|budget.get().is_bound());
  if !parent.admit_return_origin(cx)?||!bound{return Ok(false)}
  let identity=OriginalSourceBinding::borrow(cx);let result=claim_original_return(cx,&mut self.credit);
  if self.credit.is_some(){self.identity=Some(identity)}result
 }
 /// 📮️ Records only this fixed protocol capture; it does not execute or price external work.
 pub fn record_return(&mut self)->Result<(),ValueError>{
  if self.witness.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original return place already captured its return"))}
  let credit=self.credit.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original return place has no held source credit"))?;
  self.witness=Some(credit.issue_return(OriginalReturnPhase::Capture)?);Ok(())
 }
 pub fn settle_return(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let Some(credit)=self.credit.as_ref()else{return Ok(true)};credit.qualify_source(cx,self.identity.unwrap())?;
  let captured=self.witness.is_some();
  if !captured{let credit=self.credit.as_mut().unwrap();self.witness=Some(if credit.has_unused_capture(){credit.issue_success_return()?}else{credit.issue_return(OriginalReturnPhase::Receive)?})}
  receive_original_return_witness(cx,self.credit.as_mut().unwrap(),&mut self.witness)?;
  if self.credit.as_ref().unwrap().terminal_is_empty(){self.credit.take();self.identity=None;return Ok(true)}
  Ok(false)
 }
}
impl Drop for OriginalCallerReturnPlace{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original return place requires same-source settlement of held currencies")}}
