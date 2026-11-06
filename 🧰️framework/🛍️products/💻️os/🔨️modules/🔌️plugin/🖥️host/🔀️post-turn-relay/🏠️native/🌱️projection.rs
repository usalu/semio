//! 🔎️ Origin-scoped frozen instance projection under its actual Source loan.
use super::{GuestInstance,GuestRuntimes,NativeInstanceHome,NativeInstanceRetirement};
use semio_framework_job::{OriginalNativeBorrowed,OriginalTurnFailure,StepContext};
use semio_framework_value::{ValueError,ValueRefusalKind};

fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"frozen instance lacks its exact held native origin")}

impl<T:NativeInstanceRetirement> NativeInstanceHome<T>{
 pub(crate) fn frozen_instance(&mut self)->Result<&mut GuestInstance,OriginalTurnFailure>{
  if self.origin.cache_identity==0||!self.origin_held{return Err(invalid().into())}
  self.instance.as_mut().ok_or_else(||invalid().into())
 }
 pub(crate) fn frozen_parts(&mut self)->Result<(&mut GuestInstance,&mut T),OriginalTurnFailure>{
  if self.origin.cache_identity==0||!self.origin_held{return Err(invalid().into())}
  let instance=self.instance.as_mut().ok_or_else(invalid)?;Ok((instance,&mut self.state))
 }
 pub(crate) fn with_frozen_origin(mut loan:OriginalNativeBorrowed<'_,Self>,cx:&mut StepContext<'_>,run:impl FnOnce(&GuestRuntimes,OriginalNativeBorrowed<'_,Self>,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{
  loan.validate_context(cx)?;let home=loan.value_mut();if home.origin.cache_identity==0||!home.origin_held||home.instance.is_none(){return Err(invalid().into())}let origin=home.origin;let mut loan=Some(loan);let mut run=Some(run);let mut invoked=false;let available=(origin.resolve)(origin.cache_identity,origin.package_hash,&mut|runtime,compiled|{if invoked||compiled.package_hash!=origin.package_hash{return Err(invalid().into())}invoked=true;let run=run.take().ok_or_else(invalid)?;let loan=loan.take().ok_or_else(invalid)?;run(runtime,loan,cx)})?;if available!=invoked{return Err(invalid().into())}Ok(available)
 }
}

/// 🧊️ Selects the actual retained fixed-receipt frontier without taking it.
pub(crate) enum FrozenReceiptFrontier{Unit,Scalar}
/// 🤝️ Lends frozen producer bytes and the distinct same-Source History under the held compiled origin.
pub(crate) fn with_frozen_history<T:NativeInstanceRetirement>(
 history:&mut semio_framework_job::OriginalNativeLease<semio_plugin_host_wake::GuestIssuedHistoryHome>,
 producer:&mut semio_framework_job::OriginalNativeLease<NativeInstanceHome<T>>,
 frontier:FrozenReceiptFrontier,cx:&mut StepContext<'_>,
 receive:impl for<'a,'b> FnOnce(OriginalNativeBorrowed<'a,semio_plugin_host_wake::GuestIssuedHistoryHome>,semio_plugin_host_guest_runtime::GuestFrozenEscrowBorrow<'b>,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>,
)->Result<bool,OriginalTurnFailure>{
 let mut delivered=false;let paired=history.with_native_pair_borrow(producer,cx,|history,producer,cx|{
  NativeInstanceHome::with_frozen_origin(producer,cx,|runtime,producer,cx|{
   delivered=match frontier{
    FrozenReceiptFrontier::Unit=>runtime.with_original_unit_frozen_escrow(producer,NativeInstanceHome::frozen_instance,cx,|frozen,cx|receive(history,frozen,cx))?,
    FrozenReceiptFrontier::Scalar=>runtime.with_original_scalar_frozen_escrow(producer,NativeInstanceHome::frozen_instance,cx,|frozen,cx|receive(history,frozen,cx))?,
   };Ok(())
  })?;Ok(())
 })?;Ok(paired&&delivered)
}

/// 🧾️ Reconciles one frozen Source slice without re-entering the Driver lock.
pub(crate) fn reconcile_frozen_history<T:NativeInstanceRetirement>(driver:&semio_plugin_host_wake::GuestRelayWakeDriver,history:&mut semio_framework_job::OriginalNativeLease<semio_plugin_host_wake::GuestIssuedHistoryHome>,producer:&mut semio_framework_job::OriginalNativeLease<NativeInstanceHome<T>>,frontier:FrozenReceiptFrontier,cleanup:bool,run:impl for<'a,'b,'c> FnOnce(semio_plugin_host_wake::GuestEscrowReconcile<'a>,OriginalNativeBorrowed<'b,semio_plugin_host_wake::GuestIssuedHistoryHome>,semio_plugin_host_guest_runtime::GuestFrozenEscrowBorrow<'c>,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,semio_plugin_host_wake::GuestRelayAdmissionFailure>{let mut done=false;driver.dispatch_escrow_reconciliation_step(cleanup,|cx,session|{done=with_frozen_history(history,producer,frontier,cx,|history,frozen,cx|run(session,history,frozen,cx))?;Ok(())})?;Ok(done)}
