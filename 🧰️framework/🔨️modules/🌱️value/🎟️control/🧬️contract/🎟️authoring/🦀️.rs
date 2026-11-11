//! 🏦️ Synchronous authoring outside any Job turn spends one explicit finite stock and records every performed unit through one original receipt.
use super::{OriginalEffectAuthority,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete,OriginalWorkActionIdentity,OriginalWorkPerformedReceipt,OriginalSourceTurnIdentity,OriginalWork,OriginalWorkAuthority,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient,OriginalWorkSink,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,RetainedWorkPoll,ValueError,ValueRefusalKind};

struct AuthoringDriver;
struct AuthoringContext<'a>{remaining:&'a u128}
impl OriginalWorkRecipient for AuthoringDriver{
 type Context<'turn,'source>=AuthoringContext<'turn> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&AuthoringContext<'turn>)->Result<(),ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&AuthoringContext<'turn>)->Option<OriginalSourceTurnIdentity> where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,context:&mut AuthoringContext<'turn>,quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply> where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|u128::from(total)<=*context.remaining)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,_:&mut AuthoringContext<'turn>,check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(Ok(()))}
 fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut AuthoringContext<'turn>,check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.pending()}
}


type Call<'a>=dyn FnMut(&mut RetainedAllocationEffect<'_>)+'a;
struct CallFrame<'a,'b>{call:&'a mut Call<'b>,source:Option<OriginalSourceTurnIdentity>}
struct CallProducer;
impl OriginalEffectProducer for CallProducer{
 type Output=();type Failure=ValueError;type Context<'a,'b>=CallFrame<'a,'b> where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError> where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity> where 'b:'a{c.source}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,(),ValueError>)->Result<OriginalEffectReplyComplete<'r,(),ValueError>,ValueError> where 'b:'a{(c.call)(effect);Ok(reply.complete(Ok(())))}
 fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:&mut RetainedAllocationEffect<'_>,_:OriginalEffectReply<'r,(),ValueError>)->Result<OriginalEffectReplyComplete<'r,(),ValueError>,ValueError> where 'b:'a{Err(ValueError::invariant("synchronous authoring call has no unrecorded callback"))}
}
/// ⛽️ The one work unit the producer engine charges to move a recorded reply out of an invoked effect.
pub const ORIGINAL_INVOCATION_WORK:u64=1;
/// 📐️ Widens a caller's exact grant and work ceiling by the reply handoff that [`invoke_original`] pays on the same effect; the operation itself still sees only the caller's own extent.
pub fn invocation_extent(grant:RetainedCloneGrant,maximum_work:u64)->Result<(RetainedCloneGrant,u64),ValueError>{
 let handoff=OriginalEffectAuthority::<CallProducer>::transfer_reply_demand()?;
 let widened=RetainedCloneGrant{maximum_items:grant.maximum_items.max(1),maximum_copy_bytes:grant.maximum_copy_bytes.saturating_add(handoff.copy_bytes),maximum_depth:grant.maximum_depth.max(handoff.depth),..grant};
 Ok((widened,maximum_work.saturating_add(ORIGINAL_INVOCATION_WORK)))
}
/// 🧵️ Runs the operation as the receive callback of one real original producer invocation on an effect opened with [`invocation_extent`], then transfers and clears its recorded reply, so `effect.invocation()` is present for every nested reply-shaped owner. With `narrow` the operation sees exactly that grant and work; without it the whole widened extent, which also funds the metadata nested owners forward under the invocation. A child that spent past its extent leaves the handoff unfunded; the handoff unit is still recorded on the receipt, so the Work commit reports the whole overshoot and no work goes unpaid.
pub fn invoke_original<R>(effect:&mut RetainedAllocationEffect<'_>,narrow:Option<(RetainedCloneGrant,u64)>,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->Result<R,ValueError>{
 let mut operation=Some(operation);let mut output=None;let mut failure=None;
 let mut call=|effect:&mut RetainedAllocationEffect<'_>|{if let Some(operation)=operation.take(){match narrow{None=>output=Some(operation(effect)),Some((grant,maximum_work))=>match effect.reborrow(grant,maximum_work){Ok(mut inner)=>output=Some(operation(&mut inner)),Err(error)=>failure=Some(error)}}}};
 let mut authority=OriginalEffectAuthority::new(CallProducer);let mut frame=CallFrame{call:&mut call,source:effect.work_source_turn()};let(mut action,mut reply)=(None,None);
 if !authority.drive(&mut frame,effect,&mut action)?{return Err(ValueError::invariant("synchronous original call stayed pending"))}
 let mut refusal=None;
 match authority.transfer_reply_into(&mut frame,&mut action,&mut reply,effect).result{
  Ok(Some(_))=>{},
  Ok(None)=>{let charged=effect.performed(ORIGINAL_INVOCATION_WORK).map(drop);let taken=action.as_mut().ok_or_else(||ValueError::invariant("synchronous original action absent")).and_then(|open|open.transfer_reply(&mut reply).map(drop));let cleared=authority.finish(&mut action);refusal=charged.err().or(taken.err()).or(cleared.err());},
  Err(error)=>{let _=effect.performed(ORIGINAL_INVOCATION_WORK);if let Some(open)=action.as_mut(){let _=open.transfer_reply(&mut reply);}let _=authority.finish(&mut action);refusal=Some(error);},
 }
 let released=authority.into_producer().map(drop);
 if let Some(error)=refusal{return Err(error)}
 reply.take().ok_or_else(||ValueError::invariant("synchronous original reply absent"))??;
 released.map_err(|_|ValueError::invariant("synchronous original producer remains active"))?;
 if let Some(error)=failure{return Err(error)}
 output.ok_or_else(||ValueError::invariant("synchronous original call never ran"))
}
/// 🧾️ One authored effect: the closure's output, the work its receipt recorded and the physical allocation marker it left.
pub struct AuthoredEffect<R>{pub output:R,pub performed:u128,pub actual:Option<usize>}

/// 🏦️ The explicit finite stock of a synchronous authoring path: no job turn grants it, so the caller names it and every effect debits it.
pub struct AuthoringWallet{stock:RetainedCloneGrant,work:u128}
/// 🪢️ One open original Work action between its quote and its settlement; callers that need their own owner alive while the effect runs use `begin`, `effect` and `finish` instead of the closure form.
pub struct OriginalEffectRun{action:Option<OriginalWorkActionIdentity>,receipt:Option<OriginalWorkPerformedReceipt>,actual:Option<usize>,grant:RetainedCloneGrant,maximum:u64}
impl OriginalEffectRun{
 /// 🎟️ Quotes and opens the action; `None` means the quote was not admitted and nothing was opened.
 pub fn begin(work:&mut OriginalWork<'_>,grant:RetainedCloneGrant,maximum_work:u64)->Result<Option<Self>,ValueError>{
  let mut action=None;
  if work.prepare_for_action(maximum_work,&mut action)?!=Some(true){if action.is_some(){work.cancel_for_action(&mut action)?}return Ok(None)}
  let mut receipt=None;if let Err(error)=work.issue_performed_for_action(action,&mut receipt){let mut slot=action;let _=work.cancel_for_action(&mut slot);return Err(error)}
  Ok(Some(Self{action,receipt,actual:None,grant,maximum:maximum_work}))
 }
 /// 🪢️ Lends the effect over the performed receipt; the receipt and marker are locals of this run, never fields of the owner.
 pub fn effect(&mut self)->Result<RetainedAllocationEffect<'_>,ValueError>{let sink=self.receipt.as_mut().ok_or_else(||ValueError::invariant("authoring receipt absent"))?;Ok(RetainedAllocationEffect::new(self.grant,self.maximum,&mut self.actual,sink))}
 /// 🧾️ Reads the physical marker the closure left.
 pub fn actual(&self)->Option<usize>{self.actual}
 /// ✅️ Claims, settles and commits the performed work exactly once; `spent` is set before any settlement can refuse.
 pub fn finish(mut self,work:&mut OriginalWork<'_>,spent:&mut u128)->Result<(u128,Option<usize>),ValueError>{
  let mut proof=None;let action=self.action;let mut slot=action;
  let performed=OriginalWorkSink::observed_work(self.receipt.as_ref().ok_or_else(||ValueError::invariant("authoring receipt absent"))?);*spent=performed;work.claim_performed_for_action(action,&mut self.receipt)?;
  if performed==0{work.cancel_for_action(&mut slot)?}else{
   let mut partial=None;if !work.settle_partial_for_action_into(action,&mut partial)?{return Err(ValueError::invariant("authoring partial receipt is pending"))}
   let partial=partial.take().ok_or_else(||ValueError::invariant("authoring partial outcome absent"))?;if partial.total()!=performed||partial.source_refusal().is_some(){return Err(ValueError::invariant("authoring partial outcome differs from its performed work"))}
   let committed=work.commit_for_action(performed,action,&mut proof)?;work.finish_for_action(&mut proof,&mut slot)?;if !committed{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"authoring work commit was refused"))}
  }
  Ok((performed,self.actual))
 }
}
/// 🪢️ Closure form of the one original Work sequence; `None` means the quote was not admitted and nothing ran.
pub fn run_original_effect<R>(work:&mut OriginalWork<'_>,grant:RetainedCloneGrant,maximum_work:u64,spent:&mut u128,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->Result<Option<AuthoredEffect<R>>,ValueError>{
 let Some(mut run)=OriginalEffectRun::begin(work,grant,maximum_work)?else{return Ok(None)};
 let output={let mut effect=run.effect()?;operation(&mut effect)};
 let (performed,actual)=run.finish(work,spent)?;
 Ok(Some(AuthoredEffect{output,performed,actual}))
}
impl AuthoringWallet{
 /// 🆕️ Names the whole physical stock and computational work this authoring path may ever spend.
 pub const fn new(stock:RetainedCloneGrant,work:u128)->Self{Self{stock,work}}
 /// 📏️ Reads the physical stock still unspent.
 pub fn remaining(&self)->RetainedCloneGrant{self.stock}
 /// ⛽️ Reads the work still unspent.
 pub fn remaining_work(&self)->u128{self.work}
 /// ⛽️ Debits work that happened outside `run` (for example a construction loan the caller drove itself); more than the stock is refused typed and drains it.
 pub fn charge_work(&mut self,work:u128)->Result<(),ValueError>{match self.work.checked_sub(work){Some(left)=>{self.work=left;Ok(())},None=>{self.work=0;Err(ValueError::literal(ValueRefusalKind::WorkLimit,"authoring work exceeds its explicit stock"))}}}
 /// 🧾️ Debits independently observed physical progress; progress beyond the stock is refused, never ignored.
 pub fn post(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{
  if !progress.fits(self.stock){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"authoring stock cannot fund the observed physical progress").with_retained_progress(progress))}
  self.stock.maximum_items-=progress.copied_items;self.stock.maximum_copy_bytes-=progress.copied_bytes;self.stock.maximum_capacity_bytes-=progress.retained_capacity_bytes;self.stock.maximum_release_bytes-=progress.released_bytes;Ok(())
 }
 /// 🪙️ Runs one closure inside one original effect over a performed-work receipt settled against this wallet; `None` means the work quote was not admitted and nothing ran.
 pub fn run<R>(&mut self,grant:RetainedCloneGrant,maximum_work:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->Result<Option<AuthoredEffect<R>>,ValueError>{self.run_with(grant,maximum_work,false,operation)}
 /// 🧵️ [`Self::run`] for steps whose nested retirements are reply-shaped: the closure runs under [`invoke_original`] on an effect widened by its reply handoff, which the wallet stock never pays.
 pub fn run_invoked<R>(&mut self,grant:RetainedCloneGrant,maximum_work:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->Result<Option<AuthoredEffect<R>>,ValueError>{self.run_with(grant,maximum_work,true,operation)}
 fn run_with<R>(&mut self,grant:RetainedCloneGrant,maximum_work:u64,invoked:bool,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->Result<Option<AuthoredEffect<R>>,ValueError>{
  if grant.restricted_by(self.stock)!=grant{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"authoring grant exceeds its explicit stock"))}
  if u128::from(maximum_work)>self.work{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"authoring work exceeds its explicit stock"))}
  let remaining=self.work+u128::from(invoked)*u128::from(ORIGINAL_INVOCATION_WORK);let mut authority=OriginalWorkAuthority::new(AuthoringDriver);let mut spent=0u128;
  let(effect_grant,effect_work)=if invoked{invocation_extent(grant,maximum_work)?}else{(grant,maximum_work)};
  let settled=authority.with_loan(&mut AuthoringContext{remaining:&remaining},|loan|{
   let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);
   let taken=run_original_effect(&mut work,effect_grant,effect_work,&mut spent,|effect|if invoked{invoke_original(effect,None,operation)}else{Ok(operation(effect))})?;
   Ok::<_,ValueError>(taken)
  });
  self.work=self.work.saturating_sub(spent);
  let settled=settled??;
  if authority.has_live_actions(){return Err(ValueError::invariant("authoring left a live work action"))}
  match settled{None=>Ok(None),Some(done)=>Ok(Some(AuthoredEffect{output:done.output?,performed:done.performed,actual:done.actual}))}
 }
 /// 🏁️ [`Self::run`] for a poll-returning step: a suspended poll cannot be awaited synchronously and is refused typed.
 pub fn run_settled<T,E>(&mut self,grant:RetainedCloneGrant,maximum_work:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<T,E>)->Result<Option<(RetainedWorkOutcome<T,E>,Option<usize>)>,ValueError>{
  match self.run_invoked(grant,maximum_work,operation)?{
   None=>Ok(None),
   Some(done)=>match done.output{RetainedWorkPoll::Complete(turn)=>Ok(Some((turn,done.actual))),RetainedWorkPoll::Pending{..}=>Err(ValueError::literal(ValueRefusalKind::WorkLimit,"synchronous authoring cannot await a pending original effect"))},
  }
 }
}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
