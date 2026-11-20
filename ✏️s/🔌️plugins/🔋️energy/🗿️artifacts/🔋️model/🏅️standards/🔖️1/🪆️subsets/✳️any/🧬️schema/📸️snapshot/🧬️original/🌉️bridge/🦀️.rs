//! 🌉️ Bounded funded step loops that bridge a value to its native owner and back, the same bridge as the FEM snapshot.
use semio_framework_value::{DslValue,ValueError,ValueRefusalKind};
use semio_framework_value::{OriginalRetirementFailure,RetainedWorkPoll,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedCloneGrant,StagedSourceAdmission}};
use semio_framework_value::RetainedAllocationEffect;
use semio_framework_value_native_owned::{OriginalSourceTurn,OriginalSourceTurnIssuer};
use semio_framework_value_native_owned::native_decoding::NativeReceivingFailure;
use semio_framework_value_native_owned::owned::tree::NativeReceiveSlot;
use semio_framework_value_native_owned::{AuthoringWallet,OriginalReceivingPrimary};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeDecodeRetirementRecipient,OriginalWorkAuthority,OriginalConstructionAccept,OriginalConstructionAcceptComplete,OriginalConstructionAuthority,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionRecipient,OriginalSourceTurnIdentity,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient,NativeReceive};
use semio_framework_value::{RetainedCloneProgress,RetirementDemand,retained_clone::retained_clone_grant_admits};

const BRIDGE_STEPS:usize=1<<24;
const BRIDGE_STEP_WORK:u64=1<<16;
const BRIDGE_WORK:u128=1<<36;
const BRIDGE_BODY:usize=1<<30;
fn bridge_stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:128}}
/// 🏦️ The explicit finite authoring stock of one synchronous bridge call.
fn bridge_wallet()->AuthoringWallet{AuthoringWallet::new(bridge_stock(),BRIDGE_WORK)}
fn bridge_failure(failure:OriginalRetirementFailure)->ValueError{let progress=failure.retained_progress();let(primary,work,receipt,..)=failure.into_parts();primary.or(work).or(receipt).unwrap_or_else(bridge_invalid).with_retained_progress(progress)}
fn bridge_exhausted()->ValueError{ValueError::literal(ValueRefusalKind::WorkLimit,"the energy snapshot bridge exhausted its bounded step budget")}
fn bridge_invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"the energy snapshot bridge lost its original source or receiver")}
fn bridge_native(failure:NativeReceivingFailure)->ValueError{match failure{
 NativeReceivingFailure::Primary(primary)=>bridge_primary(primary),
 NativeReceivingFailure::ValueEffect(e)=>e.child.or(e.producer_refusal).or(e.work_refusal).unwrap_or_else(||bridge_primary(e.receiving)),
 NativeReceivingFailure::RetirementEffect(e)=>e.child.map(bridge_failure).or(e.producer_refusal).or(e.work_refusal).unwrap_or_else(||bridge_primary(e.receiving)),
 NativeReceivingFailure::ReceivingEffect(e)=>e.child.map(bridge_primary).or(e.producer_refusal).or(e.work_refusal).unwrap_or_else(||bridge_primary(e.receiving)),
}}
/// 📬️ Runs one reply-shaped step to completion inside its own action; a reply left pending is refused typed.
fn settled<T,E:From<ValueError>>(effect:&mut RetainedAllocationEffect<'_>,call:impl for<'r,'e,'a> FnOnce(&'e mut RetainedAllocationEffect<'a>,OriginalEffectReply<'r,T,E>)->Result<OriginalEffectReplyComplete<'r,T,E>,ValueError>)->RetainedWorkPoll<T,E>{
 let before=effect.performed_work();let mut slot=None;
 let result=(||->Result<Result<T,E>,ValueError>{let done=call(&mut *effect,OriginalEffectReply::borrow(&mut slot)?)?;drop(done);slot.take().ok_or_else(bridge_invalid)})();
 RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:match result{Ok(result)=>result,Err(error)=>Err(error.into())},work:effect.performed_work().saturating_sub(before)})
}
fn bridge_primary(p:OriginalReceivingPrimary)->ValueError{let progress=p.progress;p.primary.or(p.quote_refusal).or(p.allocation_refusal).unwrap_or_else(bridge_invalid).with_retained_progress(progress)}

/// 🌉️ Steps one owned snapshot through paid retirement under a bounded local wallet; exhaustion fails typed.
pub fn retire_stepped<T:RetireOwned>(value:T)->Result<(),ValueError>{
 let mut owner=match ControlledRetirement::new(value){Ok(owner)=>owner,Err((error,refused))=>{std::mem::forget(refused);return Err(error)}};
 let mut wallet=bridge_wallet();
 for _ in 0..BRIDGE_STEPS{
  if owner.terminal_is_empty(){return Ok(())}
  let(turn,_)=wallet.run_settled(wallet.remaining(),BRIDGE_STEP_WORK,|effect|owner.step(BRIDGE_BODY,effect))?.ok_or_else(bridge_exhausted)?;
  wallet.post(turn.result.map_err(bridge_failure)?.progress())?;
 }
 Err(bridge_exhausted())
}

/// 🎟️ One finite work loan per bridge step that admits observed plus additional native effects.
struct StepWork{quantum:u64,turn:OriginalSourceTurnIdentity}
impl OriginalWorkRecipient for StepWork{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity> where 'source:'turn{Some(self.turn)}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply> where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|work|work<=self.quantum)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(Ok(()))}
 fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.pending()}
}

fn source_effect<R>(
    native: &mut NativeDecodeControl<'_>,
    turn: &OriginalSourceTurn,
    wallet: &mut AuthoringWallet,
    operation: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> R,
) -> Result<R, ValueError> {
    let quantum = u64::try_from(wallet.remaining_work()).unwrap_or(u64::MAX);
    let grant = wallet.remaining();
    let mut work = OriginalWorkAuthority::new(StepWork { quantum, turn: turn.identity() });
    let effect = work.with_loan(&mut (), |loan| {
        native.with_work_recipient(loan, |native| {
            native.with_effect_reported(grant, BRIDGE_STEP_WORK.min(quantum.saturating_sub(1)), |_, effect| operation(effect))
        })
    })??.ok_or_else(bridge_exhausted)?;
    wallet.charge_work(effect.1)?;
    Ok(effect.0)
}

/// 🏗️ One funded step wallet that admits each construction demand once against the bounded step grant.
struct StepRecipient{turn:OriginalSourceTurnIdentity}
struct StepStock{grant:RetainedCloneGrant,submitted:RetainedCloneProgress}
impl StepStock{
 fn grant_for(&mut self,demand:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{
  let left=RetainedCloneGrant{maximum_items:self.grant.maximum_items.saturating_sub(self.submitted.copied_items),maximum_copy_bytes:self.grant.maximum_copy_bytes.saturating_sub(self.submitted.copied_bytes),maximum_capacity_bytes:self.grant.maximum_capacity_bytes.saturating_sub(self.submitted.retained_capacity_bytes),maximum_release_bytes:self.grant.maximum_release_bytes.saturating_sub(self.submitted.released_bytes),..self.grant};
  if retained_clone_grant_admits(left,demand){Ok(left)}else{Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"the energy snapshot bridge step grant does not admit a construction"))}
 }
 fn accept_for(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{
  self.submitted=self.submitted.checked_add(progress)?;
  if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"a bridge construction receipt exceeds its grant").with_retained_progress(progress))}
  Ok(())
 }
}
impl OriginalConstructionRecipient for StepRecipient{
 type Context<'turn,'source>=StepStock where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&StepStock)->Result<(),ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&StepStock)->Option<OriginalSourceTurnIdentity> where 'source:'turn{Some(self.turn)}
 fn grant<'reply,'turn,'source>(&mut self,stock:&mut StepStock,quote:OriginalConstructionGrant<'reply>)->OriginalConstructionGrantComplete<'reply> where 'source:'turn{let result=stock.grant_for(quote.demand());quote.complete(result)}
 fn accept<'reply,'turn,'source>(&mut self,stock:&mut StepStock,receipt:OriginalConstructionAccept<'reply>)->OriginalConstructionAcceptComplete<'reply> where 'source:'turn{let result=stock.accept_for(receipt.grant(),receipt.progress());receipt.complete(result)}
 fn submitted<'turn,'source>(&self,stock:&StepStock)->RetainedCloneProgress where 'source:'turn{stock.submitted}
}

/// 🌉️ Receives one canonical document value into its native owner by driving the unchanged original receiver in bounded funded steps; the same bridge as the FEM snapshot.
pub fn receive_stepped<T:NativeReceive<DslValue>+RetireOwned>(value:DslValue)->Result<T,ValueError>{
 let mut wallet=bridge_wallet();
 let mut recipient=NativeDecodeRetirementRecipient::new();
 let mut callback=|_|true;
 let mut native=NativeDecodeControl::new(bridge_stock().maximum_capacity_bytes,&mut callback);
 native.install_retirement_recipient(&mut recipient)?;
 let mut admission=Some(StagedSourceAdmission::owned(value));
 let mut source=None;
 let issuer=OriginalSourceTurnIssuer::new();
 let turn:OriginalSourceTurn=issuer.issue().map_err(|_|bridge_invalid())?;
 let mut slot=NativeReceiveSlot::<DslValue,T>::default();
 let mut output=None;
 let received=(||{
  for _ in 0..BRIDGE_STEPS{
   let Some(staged)=admission.as_mut() else{break};
   native.checkpoint()?;
   native.charge(staged.next_owned_demand()?.capacity_bytes)?;
   let turn=source_effect(&mut native,&turn,&mut wallet,|effect|staged.advance_owned(&mut source,effect))?;
   wallet.post(turn.result?.progress())?;
   if staged.terminal_is_empty(){admission=None;}
  }
  if admission.is_some(){return Err(bridge_exhausted())}
  if source.as_ref().is_none_or(|source|source.publication_pending()){return Err(bridge_invalid())}
  let mut work=OriginalWorkAuthority::new(StepWork{quantum:u64::try_from(wallet.remaining_work()).unwrap_or(u64::MAX),turn:turn.identity()});
  let mut construction=OriginalConstructionAuthority::new(StepRecipient{turn:turn.identity()});
  let mut stock=StepStock{grant:wallet.remaining(),submitted:Default::default()};
  let mut observed_idle=false;
  let mut observed_pending=false;
  for _ in 0..BRIDGE_STEPS{
   native.checkpoint()?;
   if slot.ready(){
    let(turn,_)=wallet.run_settled(wallet.remaining(),BRIDGE_STEP_WORK,|effect|slot.take(&mut output,effect))?.ok_or_else(bridge_exhausted)?;
    wallet.post(turn.result.map_err(bridge_primary)?.progress())?;
    return output.take().ok_or_else(bridge_invalid);
   }
   let original=source.as_ref().ok_or_else(bridge_invalid)?.borrow();
   let stock_for_turn=stock.grant;
   let poll=work.with_loan(&mut (),|loan|{
    native.with_work_recipient(loan,|native|{
     construction.with_loan(&mut stock,|construction|{
      let grant=stock_for_turn;
      native.with_construction_recipient(construction,|native|{
       Ok::<_,ValueError>(slot.advance(original,native,turn.witness(),BRIDGE_BODY,BRIDGE_STEP_WORK,grant))
      })
     })?
    })
   })??;
   match poll{
    RetainedWorkPoll::Pending{work}=>{if !observed_pending{observed_pending=true;eprintln!("[DEBUG] Energy receiver pending work={work} stock={stock_for_turn:?}");}wallet.charge_work(work)?;},
    RetainedWorkPoll::Complete(turn)=>{
     wallet.charge_work(turn.work)?;
     let progress=turn.result.map_err(bridge_native)?.progress();
     if progress==RetainedCloneProgress::default()&&turn.work==0&&!observed_idle{observed_idle=true;eprintln!("[DEBUG] Energy receiver idle demand={:?}, next work={:?}, stock={stock_for_turn:?}",slot.demands(original,BRIDGE_BODY,BRIDGE_STEP_WORK),slot.advance_work_demand(original,BRIDGE_BODY));}
     wallet.post(progress)?;
     stock=StepStock{grant:wallet.remaining(),submitted:Default::default()};
    }
   }
  }
  Err(bridge_exhausted())
 })();
 eprintln!("[DEBUG] Energy receive result={:?} ready={} pending={}",received.as_ref().err(),slot.ready(),slot.advance_pending());
 let mut closed:Result<(),ValueError>=Ok(());
 for _ in 0..BRIDGE_STEPS{
  if slot.is_closing(){break}
  match wallet.run_settled(wallet.remaining(),BRIDGE_STEP_WORK,|effect|settled(effect,|effect,reply|slot.begin_close_into(effect,reply))){
   Ok(Some((turn,_)))=>match turn.result.map_err(bridge_failure){Ok(step)=>{if let Err(error)=wallet.post(step.progress()){closed=closed.and(Err(error));break}}Err(error)=>{closed=closed.and(Err(error));break}},
   Ok(None)=>{closed=closed.and(Err(bridge_exhausted()));break}
   Err(error)=>{closed=closed.and(Err(error));break}
  }
 }
 for _ in 0..BRIDGE_STEPS{
  if slot.terminal_is_empty(){break}
  match wallet.run_settled(wallet.remaining(),BRIDGE_STEP_WORK,|effect|settled(effect,|effect,reply|slot.close_into(BRIDGE_BODY,effect,reply))){
   Ok(Some((turn,_)))=>match turn.result.map_err(bridge_failure){Ok(step)=>{if let Err(error)=wallet.post(step.progress()){closed=closed.and(Err(error));break}}Err(error)=>{closed=closed.and(Err(error));break}},
   Ok(None)=>{closed=closed.and(Err(bridge_exhausted()));break}
   Err(error)=>{closed=closed.and(Err(error));break}
  }
 }
 if let Some(staged)=admission.as_mut(){
  let mut returned=None;
  let mut recovery=bridge_wallet();
  let release=(||{
   for _ in 0..BRIDGE_STEPS{
    if staged.terminal_is_empty(){return Ok(())}
    let turn=source_effect(&mut native,&turn,&mut recovery,|effect|staged.close_owned_into(&mut returned,&mut source,effect))?;
    recovery.post(turn.result?.progress())?;
   }
   Err(bridge_exhausted())
  })();
  closed=closed.and(release);
  if let Some(original)=returned{closed=closed.and(retire_stepped(original));}
 }
 if let Some(mut held)=source.take(){
  let mut handed:Option<DslValue>=None;
  for _ in 0..BRIDGE_STEPS{
   if handed.is_some()||held.terminal_is_empty(){break}
   match wallet.run_settled(wallet.remaining(),BRIDGE_STEP_WORK,|effect|settled(effect,|effect,reply|held.take_authority_into(&mut handed,BRIDGE_BODY,effect,reply))){
    Ok(Some((turn,_)))=>match turn.result.map_err(bridge_failure){
     Ok(step)=>{if let Err(error)=wallet.post(step.progress()){closed=closed.and(Err(error));break}}
     Err(error)=>{closed=closed.and(Err(error));break}
    },
    Ok(None)=>{closed=closed.and(Err(bridge_exhausted()));break}
    Err(error)=>{closed=closed.and(Err(error));break}
   }
  }
  if let Some(original)=handed.take(){closed=closed.and(retire_stepped(original));}
  for _ in 0..BRIDGE_STEPS{
   if held.terminal_is_empty(){break}
   match wallet.run_settled(wallet.remaining(),BRIDGE_STEP_WORK,|effect|settled(effect,|effect,reply|held.close_into(BRIDGE_BODY,effect,reply))){
    Ok(Some((turn,_)))=>match turn.result.map_err(bridge_failure){Ok(step)=>{if let Err(error)=wallet.post(step.progress()){closed=closed.and(Err(error));break}}Err(error)=>{closed=closed.and(Err(error));break}},
    Ok(None)=>{closed=closed.and(Err(bridge_exhausted()));break}
    Err(error)=>{closed=closed.and(Err(error));break}
   }
  }
 }
 let grant=wallet.remaining();
 let drained:Result<(),ValueError>=native.with_retirement(grant,|native|{while native.has_retirement_owner(){let(turn,_)=wallet.run_settled(wallet.remaining(),BRIDGE_STEP_WORK,|effect|native.close_retirement_recipient(BRIDGE_BODY,effect))?.ok_or_else(bridge_exhausted)?;wallet.post(turn.result.map_err(bridge_failure)?.progress())?;}Ok::<(),ValueError>(())});
 eprintln!("[DEBUG] Energy close result={closed:?} decoder={drained:?} slot_empty={} original_birth={} controlled={}",slot.terminal_is_empty(),slot.requires_original_birth(),slot.requires_original_control());
 let output=received?;closed?;drained?;Ok(output)
}

#[cfg(test)]
#[test]
fn settled_source_admits_and_refuses() {
    assert!(receive_stepped::<bool>(DslValue::Bool(true)).unwrap());
    assert!(receive_stepped::<bool>(DslValue::String("wrong value kind".into())).is_err());
    let original = DslValue::Object(vec![("zone".into(), DslValue::Array(vec![DslValue::Bool(true), DslValue::String("north".into())]))]);
    let received = receive_stepped::<DslValue>(original.clone()).unwrap();
    assert_eq!(received, original);
    retire_stepped(received).unwrap();

    #[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
    struct Details { active: semio_framework_value::shared_issuance::SealedShared<bool> }
    #[derive(semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
    struct Record {
        label: semio_framework_value::shared_issuance::SealedShared<String>,
        #[value(flatten)]
        details: semio_framework_value::shared_issuance::SealedShared<Details>,
    }
    let record=receive_stepped::<Record>(DslValue::Object(vec![
        ("label".into(),DslValue::String("Retained label".into())),
        ("active".into(),DslValue::Bool(true)),
    ])).unwrap();
    assert_eq!(record.label.get(),"Retained label");
    assert!(*record.details.get().active.get());
    retire_stepped(record).unwrap();
    let result=receive_stepped::<Record>(DslValue::Object(vec![
        ("label".into(),DslValue::String("Retained label".into())),
        ("active".into(),DslValue::String("wrong nested value kind".into())),
    ]));
    assert!(matches!(result,Err(error) if error.kind==ValueRefusalKind::InvalidValue));
}
