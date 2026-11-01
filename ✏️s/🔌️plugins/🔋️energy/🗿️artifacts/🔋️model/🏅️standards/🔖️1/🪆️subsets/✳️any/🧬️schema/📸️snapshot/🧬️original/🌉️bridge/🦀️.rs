//! 🌉️ Bounded funded step loops that bridge a value to its native owner and back, the same bridge as the FEM snapshot.
use semio_framework_value::{DslValue,ValueError,ValueRefusalKind};
use semio_framework_value::{retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneGrant,RetainedCloneSourceTake,StagedSourceAdmission}};
use semio_framework_value_native_owned::OriginalReceivingPrimary;
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeDecodeRetirementRecipient,NativeReceive,NativeReceiver};

const BRIDGE_STEPS:usize=1<<24;
fn bridge_wallet()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:128}}
fn bridge_exhausted()->ValueError{ValueError::literal(ValueRefusalKind::WorkLimit,"the energy snapshot bridge exhausted its bounded step budget")}
fn bridge_invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"the energy snapshot bridge lost its original source or receiver")}
fn bridge_primary(p:OriginalReceivingPrimary)->ValueError{let progress=p.progress;p.primary.or(p.quote_refusal).or(p.allocation_refusal).unwrap_or_else(bridge_invalid).with_retained_progress(progress)}

/// 🌉️ Steps one owned snapshot through paid retirement under a bounded local wallet; exhaustion fails typed.
pub fn retire_stepped<T:RetireOwned>(value:T)->Result<(),ValueError>{
 let mut owner=match ControlledRetirement::new(value){Ok(owner)=>owner,Err((error,refused))=>{std::mem::forget(refused);return Err(error)}};
 for _ in 0..BRIDGE_STEPS{if owner.terminal_is_empty(){return Ok(())}owner.step(bridge_wallet())?;}
 Err(bridge_exhausted())
}

/// 🌉️ Receives one canonical document value into its native owner by driving the unchanged original receiver in bounded funded steps; the same bridge as the FEM snapshot.
pub fn receive_stepped<T:NativeReceive<DslValue>+RetireOwned>(value:DslValue)->Result<T,ValueError>{
 let wallet=bridge_wallet();
 let mut recipient=NativeDecodeRetirementRecipient::new();
 let mut callback=|_|true;
 let mut native=NativeDecodeControl::new(1<<40,&mut callback);
 native.install_retirement_recipient(&mut recipient)?;
 let mut admission=Some(StagedSourceAdmission::owned(value));
 let mut source=None;
 let mut receiver=<T as NativeReceive<DslValue>>::receiver();
 let received=(||{
  for _ in 0..BRIDGE_STEPS{
   if source.is_some(){break}
   let staged=admission.as_mut().ok_or_else(bridge_invalid)?;
   native.checkpoint()?;
   native.charge(staged.next_demand()?.capacity_bytes)?;
   staged.advance(&mut source,wallet).result?;
   if staged.terminal_is_empty(){admission=None;}
  }
  for _ in 0..BRIDGE_STEPS{
   native.checkpoint()?;
   if receiver.output_ready(){return Ok(receiver.take(wallet).result?.ok_or_else(bridge_invalid)?.0)}
   let original=source.as_ref().ok_or_else(bridge_invalid)?.borrow();
   receiver.advance(original,&mut native,wallet.maximum_copy_bytes,wallet).result.map_err(bridge_primary)?;
  }
  Err(bridge_exhausted())
 })();
 receiver.begin_close();
 let mut closed:Result<(),ValueError>=Ok(());
 for _ in 0..BRIDGE_STEPS{if receiver.terminal_is_empty(){break}if let Err(error)=receiver.close_step(wallet).result{closed=closed.and(Err(error));break}}
 if let Some(staged)=admission.take(){closed=closed.and(retire_stepped(staged));}
 if let Some(mut held)=source.take(){
  for _ in 0..BRIDGE_STEPS{
   match held.take_authority(wallet).result{
    Ok(RetainedCloneSourceTake::Ready(original,_))=>{closed=closed.and(retire_stepped(original));break}
    Ok(_)=>{}
    Err(error)=>{closed=closed.and(Err(error));break}
   }
  }
 }
 let drained=native.with_retirement(wallet,|native|{while native.has_retirement_owner(){native.close_retirement_recipient(wallet).result?;}Ok(())});
 let output=received?;closed?;drained?;Ok(output)
}

