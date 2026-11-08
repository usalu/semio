//! 🌉️ Bounded funded step loops that bridge a value to its native owner and back, the same bridge as the FEM snapshot.
use semio_framework_value::{DslValue,ValueError,ValueRefusalKind};
use semio_framework_value::{OriginalRetirementFailure,RetainedWorkPoll,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneGrant,RetainedCloneSourceTake,StagedSourceAdmission}};
use semio_framework_value_native_owned::owned::tree::NativeReceiveSlot;
use semio_framework_value_native_owned::{AuthoringWallet,OriginalReceivingPrimary};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeDecodeRetirementRecipient,OriginalWorkAuthority,OriginalConstructionAccept,OriginalConstructionAcceptComplete,OriginalConstructionAuthority,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionRecipient,OriginalSourceTurnIdentity,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient,NativeReceive};
use semio_framework_value::{RetainedCloneProgress,RetirementDemand,retained_clone::retained_clone_grant_admits};

const BRIDGE_STEPS:usize=1<<24;
const BRIDGE_STEP_WORK:u64=1<<16;
const BRIDGE_WORK:u128=1<<36;
fn bridge_stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:128}}
/// 🏦️ The explicit finite authoring stock of one synchronous bridge call.
fn bridge_wallet()->AuthoringWallet{AuthoringWallet::new(bridge_stock(),BRIDGE_WORK)}
fn bridge_failure(failure:OriginalRetirementFailure)->ValueError{let progress=failure.retained_progress();let(primary,work,receipt,..)=failure.into_parts();primary.or(work).or(receipt).unwrap_or_else(bridge_invalid).with_retained_progress(progress)}
fn bridge_exhausted()->ValueError{ValueError::literal(ValueRefusalKind::WorkLimit,"the energy snapshot bridge exhausted its bounded step budget")}
fn bridge_invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"the energy snapshot bridge lost its original source or receiver")}
fn bridge_primary(p:OriginalReceivingPrimary)->ValueError{let progress=p.progress;p.primary.or(p.quote_refusal).or(p.allocation_refusal).unwrap_or_else(bridge_invalid).with_retained_progress(progress)}

/// 🌉️ Steps one owned snapshot through paid retirement under a bounded local wallet; exhaustion fails typed.
pub fn retire_stepped<T:RetireOwned>(value:T)->Result<(),ValueError>{
 let mut owner=match ControlledRetirement::new(value){Ok(owner)=>owner,Err((error,refused))=>{std::mem::forget(refused);return Err(error)}};
 let mut wallet=bridge_wallet();
 for _ in 0..BRIDGE_STEPS{
  if owner.terminal_is_empty(){return Ok(())}
  let(turn,_)=wallet.run_settled(wallet.remaining(),BRIDGE_STEP_WORK,|effect|owner.step(effect))?.ok_or_else(bridge_exhausted)?;
  wallet.post(turn.result.map_err(bridge_failure)?.progress())?;
 }
 Err(bridge_exhausted())
}

/// 🎟️ One finite work loan per bridge step that admits observed plus additional native effects.
struct StepWork{quantum:u64}
impl OriginalWorkRecipient for StepWork{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity> where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply> where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|work|work<=self.quantum)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(Ok(()))}
 fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.pending()}
}

/// 🏗️ One funded step wallet that admits each construction demand once against the bounded step grant.
struct StepRecipient{grant:RetainedCloneGrant,submitted:RetainedCloneProgress}
impl StepRecipient{
 fn submitted_progress(&self)->RetainedCloneProgress{self.submitted}
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
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity> where 'source:'turn{None}
 fn grant<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalConstructionGrant<'reply>)->OriginalConstructionGrantComplete<'reply> where 'source:'turn{let result=self.grant_for(quote.demand());quote.complete(result)}
 fn accept<'reply,'turn,'source>(&mut self,_:&mut (),receipt:OriginalConstructionAccept<'reply>)->OriginalConstructionAcceptComplete<'reply> where 'source:'turn{let result=self.accept_for(receipt.grant(),receipt.progress());receipt.complete(result)}
 fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.submitted_progress()}
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
 let mut slot=NativeReceiveSlot::<DslValue,T>::default();
 let mut output=None;
 let received=(||{
  for _ in 0..BRIDGE_STEPS{
   if source.is_some(){break}
   let staged=admission.as_mut().ok_or_else(bridge_invalid)?;
   native.checkpoint()?;
   native.charge(staged.next_demand()?.capacity_bytes)?;
   let turn=wallet.run(wallet.remaining(),BRIDGE_STEP_WORK,|effect|staged.advance(&mut source,effect))?.ok_or_else(bridge_exhausted)?.output;
   wallet.post(turn.result?.progress())?;
   if staged.terminal_is_empty(){admission=None;}
  }
  for _ in 0..BRIDGE_STEPS{
   native.checkpoint()?;
   if slot.ready(){
    let(turn,_)=wallet.run_settled(wallet.remaining(),BRIDGE_STEP_WORK,|effect|slot.take(&mut output,effect))?.ok_or_else(bridge_exhausted)?;
    wallet.post(turn.result.map_err(bridge_primary)?.progress())?;
    return output.take().ok_or_else(bridge_invalid);
   }
   let original=source.as_ref().ok_or_else(bridge_invalid)?.borrow();
   let grant=wallet.remaining();
   let mut polled=None;
   let mut recipient=OriginalConstructionAuthority::new(StepRecipient{grant,submitted:Default::default()});let mut work=OriginalWorkAuthority::new(StepWork{quantum:u64::try_from(wallet.remaining_work()).unwrap_or(u64::MAX).min(1<<30)});work.with_loan(&mut (),|loan|native.with_work_recipient(loan,|native|recipient.with_loan(&mut (),|construction|native.with_construction_recipient(construction,|native|{polled=Some(slot.advance(original,native,grant.maximum_copy_bytes,BRIDGE_STEP_WORK,grant));Ok::<(),ValueError>(())}))))???;
   match polled.ok_or_else(bridge_invalid)?{RetainedWorkPoll::Pending{..}=>return Err(bridge_exhausted()),RetainedWorkPoll::Complete(turn)=>{wallet.charge_work(turn.work)?;wallet.post(turn.result.map_err(bridge_primary)?.progress())?}}
  }
  Err(bridge_exhausted())
 })();
 slot.begin_close();
 let mut closed:Result<(),ValueError>=Ok(());
 for _ in 0..BRIDGE_STEPS{
  if slot.terminal_is_empty(){break}
  match wallet.run_settled(wallet.remaining(),BRIDGE_STEP_WORK,|effect|slot.close_step(effect)){
   Ok(Some((turn,_)))=>match turn.result.map_err(bridge_failure){Ok(step)=>{if let Err(error)=wallet.post(step.progress()){closed=closed.and(Err(error));break}}Err(error)=>{closed=closed.and(Err(error));break}},
   Ok(None)=>{closed=closed.and(Err(bridge_exhausted()));break}
   Err(error)=>{closed=closed.and(Err(error));break}
  }
 }
 if let Some(staged)=admission.take(){closed=closed.and(retire_stepped(staged));}
 if let Some(mut held)=source.take(){
  for _ in 0..BRIDGE_STEPS{
   match wallet.run_settled(wallet.remaining(),BRIDGE_STEP_WORK,|effect|held.take_authority(effect)){
    Ok(Some((turn,_)))=>match turn.result.map_err(bridge_failure){
     Ok(RetainedCloneSourceTake::Ready(original,_))=>{closed=closed.and(retire_stepped(original));break}
     Ok(_)=>{}
     Err(error)=>{closed=closed.and(Err(error));break}
    },
    Ok(None)=>{closed=closed.and(Err(bridge_exhausted()));break}
    Err(error)=>{closed=closed.and(Err(error));break}
   }
  }
 }
 let grant=wallet.remaining();
 let drained:Result<(),ValueError>=native.with_retirement(grant,|native|{while native.has_retirement_owner(){let(turn,_)=wallet.run_settled(wallet.remaining(),BRIDGE_STEP_WORK,|effect|native.close_retirement_recipient(effect))?.ok_or_else(bridge_exhausted)?;wallet.post(turn.result.map_err(bridge_failure)?.progress())?;}Ok::<(),ValueError>(())});
 let output=received?;closed?;drained?;Ok(output)
}
