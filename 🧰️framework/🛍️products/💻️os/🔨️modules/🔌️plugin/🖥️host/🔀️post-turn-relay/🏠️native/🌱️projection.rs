//! 🔎️ Origin-scoped frozen instance projection under its actual Source loan.
use super::{GuestInstance,GuestRuntimes,NativeInstanceHome,NativeInstanceRetirement};
use semio_framework_job::{OriginalNativeBorrowed,OriginalTurnFailure,StepContext};
use semio_framework_value::{ValueError,ValueRefusalKind};

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
