//! 🤝️ The original work issuer lends the actual reservation reply slot to its original backend.
use super::{OriginalWorkActionIssuer,OriginalWorkActionReservation,OriginalWorkActionProof,OriginalWorkScope,OriginalWorkIssuerIdentity,OriginalWorkActionIdentity,ValueError,refused};
/// 📨️ The original issued action supplies this borrowed reply destination before its backend runs.
pub struct OriginalWorkQuote<'a>{identity:OriginalWorkActionIdentity,work:u64,entered:&'a mut bool,reply:&'a mut Option<Result<bool,ValueError>>}
/// 🧾️ Only completing the borrowed original reply can produce this return token.
pub struct OriginalWorkQuoteComplete<'a>{_reply:&'a mut Option<Result<bool,ValueError>>}
impl<'a> OriginalWorkQuote<'a>{
 pub fn action_identity(&self)->OriginalWorkActionIdentity{self.identity}
 pub fn reserved_work(&self)->u64{self.work}
 pub fn complete(self,result:Result<bool,ValueError>)->OriginalWorkQuoteComplete<'a>{assert!(self.reply.is_none(),"original work quote already answered");*self.reply=Some(result);OriginalWorkQuoteComplete{_reply:self.reply}}
}
/// 📨️ The original producer receives the resident performed-work receipt before validation.
pub struct OriginalWorkPartialCheck<'a>{identity:OriginalWorkActionIdentity,receipt:super::OriginalWorkPerformedIdentity,source_turn:Option<crate::OriginalSourceTurnIdentity>,delta:u128,total:u128,entered:&'a mut bool,reply:&'a mut Option<Result<(),ValueError>>}
/// 🧾️ Only a recorded or pending original partial verdict can return this token.
pub struct OriginalWorkPartialCheckComplete<'a>{_reply:&'a mut Option<Result<(),ValueError>>}
impl<'a> OriginalWorkPartialCheck<'a>{
 pub fn action_identity(&self)->OriginalWorkActionIdentity{self.identity}
 pub fn receipt_identity(&self)->super::OriginalWorkPerformedIdentity{self.receipt}
 pub fn source_turn(&self)->Option<crate::OriginalSourceTurnIdentity>{self.source_turn}
 pub fn delta(&self)->u128{self.delta}
 pub fn total(&self)->u128{self.total}
 pub fn complete(self,result:Result<(),ValueError>)->OriginalWorkPartialCheckComplete<'a>{assert!(self.reply.is_none(),"original partial Work reply occupied");*self.reply=Some(result);OriginalWorkPartialCheckComplete{_reply:self.reply}}
 pub fn pending(self)->OriginalWorkPartialCheckComplete<'a>{assert!(self.reply.is_none(),"original partial Work reply occupied");OriginalWorkPartialCheckComplete{_reply:self.reply}}
}
/// 🧭️ A resident backend validates each borrowed original turn before answering its issued action.
pub trait OriginalWorkRecipient:Send{
 type Context<'turn,'source>:?Sized+Send where 'source:'turn;
 fn validate_context<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn;
 fn source_turn<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity> where 'source:'turn;
 fn admits<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply> where 'source:'turn;
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn;
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn;
}
trait OriginalWorkPort:Send{fn validate(&self)->Result<(),ValueError>;fn admits<'a>(&mut self,quote:OriginalWorkQuote<'a>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'a>;fn settle_partial<'a>(&mut self,check:OriginalWorkPartialCheck<'a>)->Result<OriginalWorkPartialCheckComplete<'a>,ValueError>;fn recover_partial<'a>(&mut self,check:OriginalWorkPartialCheck<'a>)->Result<OriginalWorkPartialCheckComplete<'a>,ValueError>;}
struct OriginalWorkBorrow<'loan,'turn,'source: 'turn,P:OriginalWorkRecipient>{recipient:&'loan mut P,context:&'loan mut P::Context<'turn,'source>,source_turn:Option<crate::OriginalSourceTurnIdentity>}
impl<'loan,'turn,'source: 'turn,P:OriginalWorkRecipient> OriginalWorkPort for OriginalWorkBorrow<'loan,'turn,'source,P>{fn validate(&self)->Result<(),ValueError>{self.recipient.validate_context(self.context)?;if self.recipient.source_turn(self.context)!=self.source_turn{return Err(refused())}Ok(())}fn admits<'a>(&mut self,quote:OriginalWorkQuote<'a>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'a>{if let Err(error)=self.recipient.validate_context(self.context){return quote.complete(Err(error))}if self.recipient.source_turn(self.context)!=self.source_turn{return quote.complete(Err(refused()))}*quote.entered=true;self.recipient.admits(self.context,quote,observed,additional)}fn settle_partial<'a>(&mut self,check:OriginalWorkPartialCheck<'a>)->Result<OriginalWorkPartialCheckComplete<'a>,ValueError>{self.validate()?;*check.entered=true;Ok(self.recipient.settle_partial(self.context,check))}fn recover_partial<'a>(&mut self,check:OriginalWorkPartialCheck<'a>)->Result<OriginalWorkPartialCheckComplete<'a>,ValueError>{self.validate()?;Ok(self.recipient.recover_partial(self.context,check))}}
/// 🏛️ Owns the actual issuer and its one original producer across every borrowed turn.
pub struct OriginalWorkAuthority<P:OriginalWorkRecipient>{issuer:OriginalWorkActionIssuer,recipient:P}
impl<P:OriginalWorkRecipient> OriginalWorkAuthority<P>{
 pub fn new(recipient:P)->Self{Self{issuer:OriginalWorkActionIssuer::new(),recipient}}
 /// 📏️ Quotes the actual resident producer and authority output slots.
 pub fn birth_demand()->Result<crate::RetirementDemand,crate::ValueError>{crate::work_action::authority_birth_demand::<P,Self>()}
 /// 🏛️ Mints the actual issuer before transferring the original paid producer slot.
 pub fn admit_from<'turn,'source>(input:&mut Option<P>,output:&mut Option<Self>,context:&P::Context<'turn,'source>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<crate::RetainedCloneProgress>,crate::ValueError> where 'source:'turn{let fail=|error|crate::RetainedWorkOutcome{result:Err(error),work:0};if output.is_some(){return fail(crate::ValueError::invariant("original authority output is occupied"))}let Some(recipient)=input.as_ref()else{return crate::RetainedWorkOutcome{result:Ok(None),work:0}};if let Err(error)=recipient.validate_context(context){return fail(error)}let source=recipient.source_turn(context);if source.is_some()&&source!=effect.work_source_turn(){return fail(crate::ValueError::invariant("original authority Source differs from its Work"))}crate::work_action::paid_authority_birth(input,output,effect,OriginalWorkActionIssuer::new,|issuer,recipient|Self{issuer,recipient})}

 pub fn identity(&self)->OriginalWorkIssuerIdentity{self.issuer.identity()}
 pub fn has_live_actions(&self)->bool{self.issuer.has_live_actions()}
 pub fn inspect<R>(&self,query:impl FnOnce(&P)->R)->R{query(&self.recipient)}
 pub fn with_loan<'turn,'source,R>(&mut self,context:&mut P::Context<'turn,'source>,operation:impl FnOnce(&mut OriginalWorkLoan<'_>)->R)->Result<R,ValueError> where 'source:'turn{self.recipient.validate_context(context)?;let source_turn=self.recipient.source_turn(context);let mut port=OriginalWorkBorrow{recipient:&mut self.recipient,context,source_turn};let mut loan=OriginalWorkLoan{issuer:&mut self.issuer,recipient:&mut port,source_turn};Ok(operation(&mut loan))}
 /// 📤️ Returns the same idle producer directly into its paid caller slot.
 pub fn return_into<'turn,'source>(input:&mut Option<Self>,output:&mut Option<P>,context:&P::Context<'turn,'source>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<crate::RetainedCloneProgress>,crate::ValueError> where 'source:'turn{let fail=|error|crate::RetainedWorkOutcome{result:Err(error),work:0};if output.is_some(){return fail(crate::ValueError::invariant("original producer return slot is occupied"))}let Some(authority)=input.as_ref()else{return crate::RetainedWorkOutcome{result:Ok(None),work:0}};if authority.issuer.has_live_actions(){return fail(crate::ValueError::invariant("original authority retains live actions"))}if let Err(error)=authority.recipient.validate_context(context){return fail(error)}let source=authority.recipient.source_turn(context);if source.is_some()&&source!=effect.work_source_turn(){return fail(crate::ValueError::invariant("original authority return changed Source Work"))}crate::work_action::paid_authority_return(input,output,effect,Self::into_recipient)}
 pub fn into_recipient(self)->Result<P,Self>{if self.issuer.has_live_actions(){Err(self)}else{Ok(self.recipient)}}
}
/// 🫴️ One exclusive loan borrows the issuer and the same resident producer.
pub struct OriginalWorkLoan<'a>{issuer:&'a mut OriginalWorkActionIssuer,recipient:&'a mut dyn OriginalWorkPort,source_turn:Option<crate::OriginalSourceTurnIdentity>}
impl OriginalWorkLoan<'_>{
 pub fn identity(&self)->OriginalWorkIssuerIdentity{self.issuer.identity()}
 pub fn reborrow(&mut self)->OriginalWorkLoan<'_>{OriginalWorkLoan{issuer:&mut *self.issuer,recipient:&mut *self.recipient,source_turn:self.source_turn}}
 pub fn has_live_actions(&self)->bool{self.issuer.has_live_actions()}
 pub fn source_turn(&self)->Option<crate::OriginalSourceTurnIdentity>{self.source_turn}
 pub fn validate_context(&self)->Result<(),ValueError>{self.recipient.validate()}
 pub fn issue(&mut self,scope:&mut OriginalWorkScope,work:u64)->Result<OriginalWorkActionReservation,ValueError>{self.recipient.validate()?;let mut action=self.issuer.issue(scope,work)?;action.source_turn=self.source_turn;Ok(action)}
 pub fn owns_reservation(&self,scope:&OriginalWorkScope,action:&OriginalWorkActionReservation)->bool{action.source_turn==self.source_turn&&self.issuer.owns_reservation(scope,action)}
 pub fn admits(&mut self,scope:&OriginalWorkScope,action:&mut OriginalWorkActionReservation,observed:u64,additional:u64)->Result<bool,ValueError>{
  self.recipient.validate()?;if !self.owns_reservation(scope,action)||action.reserved_work()!=additional||action.quote_started{return Err(refused())}
  action.quote_started=true;action.quote_entered=false;action.quote_observed=Some(observed);let quote=OriginalWorkQuote{identity:action.action_identity(),work:action.reserved_work(),entered:&mut action.quote_entered,reply:&mut action.quote_reply};let complete=self.recipient.admits(quote,observed,additional);drop(complete);let reply=action.quote_reply.take().ok_or_else(refused)?;action.quote_consumed=true;reply
 }
 pub fn recover(&mut self,scope:&OriginalWorkScope,action:&mut OriginalWorkActionReservation)->Result<Option<Result<bool,ValueError>>,ValueError>{self.recipient.validate()?;if !self.owns_reservation(scope,action)||!action.quote_started||action.quote_consumed{return Err(refused())}if !action.quote_entered&&action.quote_reply.is_none(){let observed=action.quote_observed.ok_or_else(refused)?;let additional=action.reserved_work();let quote=OriginalWorkQuote{identity:action.action_identity(),work:additional,entered:&mut action.quote_entered,reply:&mut action.quote_reply};let complete=self.recipient.admits(quote,observed,additional);drop(complete)}let reply=action.quote_reply.take();if reply.is_some(){action.quote_consumed=true}Ok(reply)}
 pub fn cancel(&mut self,scope:&mut OriginalWorkScope,action:OriginalWorkActionReservation)->Result<(),(ValueError,OriginalWorkActionReservation)>{if let Err(error)=self.recipient.validate(){return Err((error,action))}if action.source_turn!=self.source_turn{return Err((refused(),action))}self.issuer.cancel(scope,action)}
 pub fn commit(&mut self,scope:&mut OriginalWorkScope,action:OriginalWorkActionReservation,actual:u128,output:&mut Option<OriginalWorkActionProof>)->Result<bool,(ValueError,OriginalWorkActionReservation)>{if let Err(error)=self.recipient.validate(){return Err((error,action))}if action.source_turn!=self.source_turn{return Err((refused(),action))}self.issuer.commit(scope,action,actual,output)}
 /// 📨️ Resolves only the resident partial receipt against the same original producer.
 pub fn settle_partial(&mut self,scope:&OriginalWorkScope,action:&mut OriginalWorkActionReservation)->Result<Option<super::OriginalWorkPartialOutcome>,ValueError>{self.recipient.validate()?;if !self.owns_reservation(scope,action)||!action.quote_consumed{return Err(refused())}let identity=action.identity;let source_turn=action.source_turn;let receipt=action.partial.as_mut().ok_or_else(refused)?;if receipt.unaccounted.is_some(){return Err(refused())}if receipt.reply.is_none(){let first=!receipt.started||!receipt.entered;receipt.started=true;let receipt_identity=super::OriginalWorkPerformedIdentity{action:identity,serial:receipt.serial};let check=OriginalWorkPartialCheck{identity,receipt:receipt_identity,source_turn,delta:receipt.delta,total:receipt.total,entered:&mut receipt.entered,reply:&mut receipt.reply};let result=if first{self.recipient.settle_partial(check)}else{self.recipient.recover_partial(check)}?;drop(result)}let Some(result)=receipt.reply.take()else{return Ok(None)};let outcome=super::OriginalWorkPartialOutcome{identity:super::OriginalWorkPerformedIdentity{action:identity,serial:receipt.serial},delta:receipt.delta,total:receipt.total,admitted:receipt.admitted,source_refusal:result.err()};action.partial=None;Ok(Some(outcome))}
 pub fn validates(&self,scope:&OriginalWorkScope,proof:&OriginalWorkActionProof,actual:u128)->Result<bool,ValueError>{self.recipient.validate()?;Ok(proof.action.source_turn==self.source_turn&&self.issuer.validates(scope,proof,actual))}
 pub fn validates_finished(&self,scope:&OriginalWorkScope,proof:&OriginalWorkActionProof,actual:u128)->Result<bool,ValueError>{self.recipient.validate()?;Ok(proof.action.source_turn==self.source_turn&&self.issuer.validates_finished(scope,proof,actual))}
 pub fn finish_retained(&mut self,scope:&mut OriginalWorkScope,proof:&mut Option<OriginalWorkActionProof>)->Result<(),ValueError>{self.recipient.validate()?;if proof.as_ref().is_some_and(|proof|proof.action.source_turn!=self.source_turn){return Err(refused())}self.issuer.finish_retained(scope,proof)}
 pub fn finish(&mut self,scope:&mut OriginalWorkScope,proof:&mut Option<OriginalWorkActionProof>)->Result<(),ValueError>{self.recipient.validate()?;if proof.as_ref().is_some_and(|proof|proof.action.source_turn!=self.source_turn){return Err(refused())}self.issuer.finish(scope,proof)}
}
