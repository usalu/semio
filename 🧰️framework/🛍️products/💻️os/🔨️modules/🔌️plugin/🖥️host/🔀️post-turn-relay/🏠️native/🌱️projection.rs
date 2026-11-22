//! 🔎️ Origin-scoped frozen instance projection under its actual Source loan.
use super::{GuestInstance,GuestRuntimes,NativeInstanceHome,NativeInstanceRetirement};
use semio_framework_job::{OriginalNativeBorrowed,OriginalNativeLease,OriginalTurnFailure,StepContext};
use semio_framework_job_return_escrow_publication::{original_escrow_length_valid,ORIGINAL_ESCROW_HEADER_BYTES,ORIGINAL_ESCROW_ROW_BYTES};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_plugin_host_wake::{FrozenCaptureProgress,GuestIssuedHistoryHome,GuestRelayAdmissionFailure,GuestRelayGuestSubloan,GuestRelayWakeDriver,HeaderValidationProgress,SreValidationProgress};

fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"frozen instance lacks its exact held native origin")}

impl<T:NativeInstanceRetirement> NativeInstanceHome<T>{
 /// 🧩️ Borrows disjoint original instance and payload only after this home holds its exact immutable origin.
 pub fn frozen_parts(&mut self)->Result<(&mut GuestInstance,&mut T),OriginalTurnFailure>{
  if self.origin.cache_identity==0||!self.origin_held{return Err(invalid().into())}
  let instance=self.instance.as_mut().ok_or_else(invalid)?;Ok((instance,&mut self.state))
 }
 /// 🎟️ Lends the actual Source-validated producer and exact immutable runtime under one native-bank guard.
 pub fn with_frozen_origin(mut loan:OriginalNativeBorrowed<'_,Self>,cx:&mut StepContext<'_>,run:impl FnOnce(&GuestRuntimes,OriginalNativeBorrowed<'_,Self>,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{
  loan.validate_context(cx)?;let home=loan.value_mut();if home.origin.cache_identity==0||!home.origin_held||home.instance.is_none(){return Err(invalid().into())}let origin=home.origin;let mut loan=Some(loan);let mut run=Some(run);let mut invoked=false;let available=(origin.resolve)(origin.cache_identity,origin.package_hash,&mut|runtime,compiled|{if invoked||compiled.package_hash!=origin.package_hash{return Err(invalid().into())}invoked=true;let run=run.take().ok_or_else(invalid)?;let loan=loan.take().ok_or_else(invalid)?;run(runtime,loan,cx)})?;if available!=invoked{return Err(invalid().into())}Ok(available)
 }
}

/// 🧊️ Selects the actual retained fixed-receipt frontier without taking it.
#[derive(Clone,Copy)]
pub enum FrozenReceiptFrontier{Unit,Scalar}

#[derive(Clone, Copy, Debug, Default)]
enum CaptureStep {
 #[default]
 Copy,Header,Sre,ReceiptOrigin,ClaimOrigin,StageClaim,ClaimSlot,VerifyClaim,ClaimDelta,CommitClaim,Coverage,OriginDelta,Purpose,Scope,Bootstrap,Ledger,Journal,Complete,
}

/// 🧾️ Progresses one genuine frozen header through its issued History and ledger before the caller can resume it.
#[derive(Debug, Default)]
pub struct GuestFrozenHistoryCapture {
 step:CaptureStep,
 row:usize,
 rows:usize,
}

impl GuestFrozenHistoryCapture {
 pub fn advance<T:NativeInstanceRetirement>(
  &mut self,driver:&GuestRelayWakeDriver,history:&mut OriginalNativeLease<GuestIssuedHistoryHome>,
  producer:&mut OriginalNativeLease<NativeInstanceHome<T>>,frontier:FrozenReceiptFrontier,
  loan:GuestRelayGuestSubloan,cleanup:bool,
 )->Result<bool,GuestRelayAdmissionFailure>{
  use CaptureStep::*;
  let step=self.step;
  let mut ready=false;
  match step{
   Bootstrap|Ledger=>{
    driver.dispatch_escrow_reconciliation_step(cleanup,|cx,mut issuer|{
     with_frozen_history_parts(history,producer,frontier,cx,|history,frozen,_,cx|{
      ready=if matches!(step,Bootstrap){frozen.header_len()!=200||issuer.apply_bootstrap_transition(history,&frozen,cx)?}
       else{issuer.apply(history,&frozen,cx)?};
      Ok(())
     })?;
     Ok(())
    })?;
   },
   Complete=>return Ok(true),
   _=>{
    driver.dispatch_operation_step(false,cleanup,|cx|{
     with_frozen_history_parts(history,producer,frontier,cx,|history,frozen,_,cx|{
      ready=match step{
       Copy=>{
        let length=frozen.escrow_len();
        let extent=u32::try_from(length).map_err(|_|ValueError::invariant("frozen receipt exceeds the original SRE1 width"))?;
        if !original_escrow_length_valid(extent){return Err(ValueError::invariant("frozen receipt has an invalid SRE1 extent").into())}
        let copied=GuestIssuedHistoryHome::capture_frozen_step(history,frozen,cx)?==FrozenCaptureProgress::WordsCopied;
        if copied{self.rows=(length-ORIGINAL_ESCROW_HEADER_BYTES)/ORIGINAL_ESCROW_ROW_BYTES;}
        copied
       },
       Header=>GuestIssuedHistoryHome::validate_header_step(history,&frozen,cx)?==HeaderValidationProgress::HeaderValidated,
       Sre=>GuestIssuedHistoryHome::validate_sre_step(history,&frozen,cx)?==SreValidationProgress::SreValidated,
       ReceiptOrigin=>GuestIssuedHistoryHome::find_receipt_origin_step(history,&frozen,loan,cx)?.is_some(),
       ClaimOrigin=>GuestIssuedHistoryHome::find_claim_origin_step(history,&frozen,self.row,cx)?.is_some(),
       StageClaim=>GuestIssuedHistoryHome::stage_claim_step(history,&frozen,self.row,cx)?,
       ClaimSlot=>GuestIssuedHistoryHome::find_claim_slot_step(history,&frozen,cx)?.is_some(),
       VerifyClaim=>GuestIssuedHistoryHome::verify_claim_step(history,&frozen,cx)?,
       ClaimDelta=>GuestIssuedHistoryHome::calculate_claim_delta_step(history,&frozen,cx)?,
       CommitClaim=>GuestIssuedHistoryHome::stage_verified_claim_step(history,&frozen,cx)?,
       Coverage=>GuestIssuedHistoryHome::verify_snapshot_coverage_step(history,&frozen,cx)?,
       OriginDelta=>GuestIssuedHistoryHome::aggregate_origin_delta_step(history,&frozen,cx)?,
       Purpose=>GuestIssuedHistoryHome::aggregate_purpose_step(history,&frozen,cx)?,
       Scope=>GuestIssuedHistoryHome::validate_origin_scope_step(history,&frozen,cx)?,
       Journal=>GuestIssuedHistoryHome::commit_journal_step(history,&frozen,cx)?,
       Bootstrap|Ledger|Complete=>unreachable!(),
      };
      Ok(())
     })?;
     Ok(())
    })?;
   },
  }
  if ready{self.complete_step();}
  Ok(matches!(self.step,Complete))
 }
 fn complete_step(&mut self){
  use CaptureStep::*;
  self.step=match self.step{
    Copy=>Header,Header=>Sre,Sre=>ReceiptOrigin,
    ReceiptOrigin=>if self.rows==0{Coverage}else{ClaimOrigin},
    ClaimOrigin=>StageClaim,StageClaim=>ClaimSlot,ClaimSlot=>VerifyClaim,VerifyClaim=>ClaimDelta,ClaimDelta=>CommitClaim,
    CommitClaim=>{self.row+=1;if self.row==self.rows{Coverage}else{ClaimOrigin}},
    Coverage=>OriginDelta,OriginDelta=>Purpose,Purpose=>Scope,Scope=>Bootstrap,Bootstrap=>Ledger,Ledger=>Journal,Journal|Complete=>Complete,
  };
 }
}

#[cfg(test)]
mod capture_tests{
 use super::*;

 #[test]
 fn every_claim_precedes_committing_the_frozen_history(){
  let corpus:serde_json::Value=serde_json::from_str(include_str!("escrow-capture.json")).unwrap();
  for case in corpus["cases"].as_array().unwrap(){
   let mut capture=GuestFrozenHistoryCapture{rows:case["rows"].as_u64().unwrap() as usize,..Default::default()};
   for expected in case["steps"].as_array().unwrap(){
    assert_eq!(format!("{:?}",capture.step),expected.as_str().unwrap());
    capture.complete_step();
   }
   assert!(matches!(capture.step,CaptureStep::Complete));
  }
 }
}
/// 🤝️ Couples the authenticated frozen producer, its controller and actual issuer without re-entering a native bank.
pub fn with_frozen_history_parts<T:NativeInstanceRetirement>(history:&mut semio_framework_job::OriginalNativeLease<semio_plugin_host_wake::GuestIssuedHistoryHome>,producer:&mut semio_framework_job::OriginalNativeLease<NativeInstanceHome<T>>,frontier:FrozenReceiptFrontier,cx:&mut StepContext<'_>,receive:impl for<'a,'b> FnOnce(OriginalNativeBorrowed<'a,semio_plugin_host_wake::GuestIssuedHistoryHome>,semio_plugin_host_guest_runtime::GuestFrozenEscrowBorrow<'b>,&mut T,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{let mut delivered=false;let paired=history.with_native_pair_borrow(producer,cx,|history,producer,cx|{NativeInstanceHome::with_frozen_origin(producer,cx,|runtime,producer,cx|{delivered=runtime.with_original_frozen_escrow_parts(matches!(frontier,FrozenReceiptFrontier::Scalar),producer,NativeInstanceHome::frozen_parts,cx,|frozen,state,cx|receive(history,frozen,state,cx))?;Ok(())})?;Ok(())})?;Ok(paired&&delivered)}

/// ✅️ Captures only the actual Core Deallocate return in the retained controller's exact preborn slot.
pub fn attest_header_return<T:NativeInstanceRetirement>(loan:OriginalNativeBorrowed<'_,NativeInstanceHome<T>>,scalar:bool,identity:semio_plugin_host_guest_runtime::GuestFrozenEscrowIdentity,cx:&mut StepContext<'_>,destination:impl for<'a> FnOnce(&'a mut T)->&'a mut Option<semio_plugin_host_guest_runtime::GuestHeaderDeallocateReturned>)->Result<bool,OriginalTurnFailure>{let mut done=false;let available=NativeInstanceHome::with_frozen_origin(loan,cx,|runtime,loan,cx|{done=if scalar{runtime.attest_original_scalar_header_deallocate(loan,|home|{let(instance,state)=home.frozen_parts()?;Ok((instance,destination(state)))},identity,cx)?}else{runtime.attest_original_unit_header_deallocate(loan,|home|{let(instance,state)=home.frozen_parts()?;Ok((instance,destination(state)))},identity,cx)?};Ok(())})?;Ok(available&&done)}

/// ⚠️ Captures a causal reference only from the actual failed frozen producer and exact retained destination.
pub fn attest_actor_failure_if_needed<T:NativeInstanceRetirement>(loan:OriginalNativeBorrowed<'_,NativeInstanceHome<T>>,scalar:bool,cx:&mut StepContext<'_>,needed:impl FnOnce(&T)->bool,destination:impl for<'a> FnOnce(&'a mut T)->&'a mut Option<semio_plugin_host_guest_runtime::GuestActorFailureCallReference>)->Result<bool,OriginalTurnFailure>{let mut done=true;let available=NativeInstanceHome::with_frozen_origin(loan,cx,|runtime,mut loan,cx|{if !needed(loan.value_mut().frozen_parts()?.1){return Ok(())}done=runtime.attest_original_actor_failure(scalar,loan,|home|{let(instance,state)=home.frozen_parts()?;Ok((instance,destination(state)))},cx)?;Ok(())})?;Ok(available&&done)}

/// 🌱️ Selects actual frozen SAB1 separately from a pending or Source-ready scalar receipt.
pub fn with_bootstrap_history_parts<T:NativeInstanceRetirement>(history:&mut semio_framework_job::OriginalNativeLease<semio_plugin_host_wake::GuestIssuedHistoryHome>,producer:&mut semio_framework_job::OriginalNativeLease<NativeInstanceHome<T>>,cx:&mut StepContext<'_>,receive:impl for<'a,'b> FnOnce(OriginalNativeBorrowed<'a,semio_plugin_host_wake::GuestIssuedHistoryHome>,semio_plugin_host_guest_runtime::GuestFrozenBootstrapBorrow<'b>,&mut T,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<Option<bool>,OriginalTurnFailure>{let mut selected=None;let paired=history.with_native_pair_borrow(producer,cx,|history,producer,cx|{NativeInstanceHome::with_frozen_origin(producer,cx,|runtime,producer,cx|{selected=runtime.with_original_bootstrap_parts(producer,NativeInstanceHome::frozen_parts,cx,|frozen,state,cx|receive(history,frozen,state,cx))?;Ok(())})?;Ok(())})?;Ok(if paired{selected}else{None})}
/// ✅️ Captures only the exact actual SAB1 Deallocate result in its retained controller slot.
pub fn attest_bootstrap_header_return<T:NativeInstanceRetirement>(loan:OriginalNativeBorrowed<'_,NativeInstanceHome<T>>,identity:semio_plugin_host_guest_runtime::GuestBootstrapHeaderIdentity,cx:&mut StepContext<'_>,destination:impl for<'a> FnOnce(&'a mut T)->&'a mut Option<semio_plugin_host_guest_runtime::GuestBootstrapHeaderDeallocateReturned>)->Result<bool,OriginalTurnFailure>{let mut done=false;let available=NativeInstanceHome::with_frozen_origin(loan,cx,|runtime,loan,cx|{done=runtime.attest_original_bootstrap_header_deallocate(loan,|home|{let(instance,state)=home.frozen_parts()?;Ok((instance,destination(state)))},identity,cx)?;Ok(())})?;Ok(available&&done)}
