//! 🧾️ The original snapshot owner keeps physical wallet settlement beside callback custody.
use super::{NativeSnapshotEncodeOwner,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
use semio_framework_value::native_encoding::{NativeEncodeEffectSettlement,NativeEncodeEffectReceipt,NativeEncodeEffectRecovery};
use semio_framework_value::retirement::RetireOwned;

/// 🧳️ Owns the exact original action and its local wallet receipt across callback unwind.
pub struct NativeSnapshotEncodeSettlement{original:Option<NativeEncodeEffectSettlement>,grant:RetainedCloneGrant,before:RetainedCloneProgress,local_done:bool,local_failure:Option<ValueError>,operation_unwind:Option<semio_framework_value_control_contract::RetainedOwnerGate<Box<dyn std::any::Any+Send>>>}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
impl NativeSnapshotEncodeSettlement{
 /// 🧾️ Stores actual caller-owned effects before original post callbacks.
 pub fn record_effect(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{self.original.as_mut().ok_or_else(||invalid("original effect quote is absent"))?.record_effect(progress)}
 /// 🪢️ Borrows the original settlement for cursor binding and recovery.
 pub fn original(&self)->Option<&NativeEncodeEffectSettlement>{self.original.as_ref()}
 /// 🧾️ Completes remaining original posts without replaying a started callback.
 pub fn settle_retained(&mut self,recovery:&mut NativeEncodeEffectRecovery<'_,'_>)->Result<bool,ValueError>{if !self.local_done{return Err(invalid("snapshot local receipt is not recorded"))}recovery.settle(&mut self.original)}
 /// 🪢️ Reconciles an original quote retained before its callback returned.
 pub fn recover_quote(&mut self,recovery:&mut NativeEncodeEffectRecovery<'_,'_>)->Result<bool,ValueError>{if self.local_done{return Err(invalid("snapshot effect already changed its wallet"))}recovery.recover_quote(&mut self.original)}
 /// 🎟️ Continues original work admission after an interrupted quote.
 pub fn resume_retained_admission(&mut self,recovery:&mut NativeEncodeEffectRecovery<'_,'_>)->Result<bool,ValueError>{if self.local_done{return Err(invalid("snapshot effect already changed its wallet"))}recovery.resume_admission(&mut self.original)}
 /// ↩️ Cancels an original retained reservation before its effect.
 pub fn cancel_retained(&mut self,recovery:&mut NativeEncodeEffectRecovery<'_,'_>)->Result<semio_framework_value::native_encoding::NativeEncodeEffectCancellation,ValueError>{if self.local_done{return Err(invalid("snapshot effect already changed its wallet"))}let token=recovery.cancel(&mut self.original)?;self.local_done=true;Ok(token)}
 /// ↩️ Releases a refused original quote after its exact facts were observed.
 pub fn release_retained_refusal(&mut self,recovery:&mut NativeEncodeEffectRecovery<'_,'_>)->Result<semio_framework_value::native_encoding::NativeEncodeEffectCancellation,ValueError>{if self.local_done{return Err(invalid("snapshot effect already changed its wallet"))}let token=recovery.release_refused(&mut self.original)?;self.local_done=true;Ok(token)}
 /// 🧳️ Retains an operation unwind while independent original callbacks settle.
 pub fn retain_operation_unwind(&mut self,payload:Box<dyn std::any::Any+Send>)->Result<(),Box<dyn std::any::Any+Send>>{if self.operation_unwind.is_some(){return Err(payload)}self.operation_unwind=Some(semio_framework_value_control_contract::RetainedOwnerGate::new(payload));Ok(())}
 /// 👁️ Reports whether an operation unwind remains in this original frame.
 pub fn has_operation_unwind(&self)->bool{self.operation_unwind.is_some()}
 /// ▶️ Resumes the original unwind after completed settlement and receipt handoff.
 pub fn resume_operation_unwind(slot:&mut Option<Self>)->Result<(),ValueError>{let state=slot.as_mut().ok_or_else(||invalid("snapshot settlement is absent"))?;if state.original.is_some()||!state.local_done||state.local_failure.is_some(){return Err(invalid("snapshot settlement is not handed off"))}let payload=state.operation_unwind.take().ok_or_else(||invalid("snapshot operation unwind is absent"))?.into_inner();slot.take();std::panic::resume_unwind(payload)}
 /// 🎟️ Transfers the completed single-use original receipt.
 pub fn take_receipt(slot:&mut Option<Self>)->Result<NativeEncodeEffectReceipt,ValueError>{let state=slot.as_mut().ok_or_else(||invalid("snapshot settlement is absent"))?;if !state.local_done||state.local_failure.is_some(){return Err(invalid("snapshot wallet settlement is not accepted"))}let receipt=NativeEncodeEffectSettlement::take_receipt(&mut state.original)?;if state.operation_unwind.is_none(){slot.take();}Ok(receipt)}
}
impl Drop for NativeSnapshotEncodeSettlement{fn drop(&mut self){assert!(std::thread::panicking()||self.retirement_ready(),"original snapshot settlement remains unresolved")}}
impl RetireOwned for NativeSnapshotEncodeSettlement{
 type RetirementCursor=semio_framework_value::native_encoding::NativeEncodeRetirement<Self,<(Option<NativeEncodeEffectSettlement>,Option<ValueError>) as RetireOwned>::RetirementCursor>;
 fn retirement_ready(&self)->bool{self.operation_unwind.is_none()&&self.original.as_ref().is_none_or(RetireOwned::retirement_ready)}
 fn retirement_seed(self)->Self::RetirementCursor{use semio_framework_value::native_encoding::NativeEncodeRetirement;if !self.retirement_ready(){NativeEncodeRetirement::Pending(Some(self))}else{let mut state=self;NativeEncodeRetirement::Ready(RetireOwned::retirement_seed((state.original.take(),state.local_failure.take())))}}
 fn retirement_birth_bytes(&self)->Option<usize>{self.retirement_ready().then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
impl NativeSnapshotEncodeOwner<'_,'_>{
 /// 🎫️ Retains the caller wallet and original callback authority before quoting an effect.
 pub fn begin_original_settlement(&mut self,slot:&mut Option<NativeSnapshotEncodeSettlement>,demand:RetainedCloneGrant)->Result<bool,ValueError>{if slot.is_some()||!self.progress.fits(self.grant)||!(RetainedCloneProgress{copied_items:demand.maximum_items,copied_bytes:demand.maximum_copy_bytes,retained_capacity_bytes:demand.maximum_capacity_bytes,released_bytes:demand.maximum_release_bytes}).fits(self.remaining_grant())||demand.maximum_depth>self.grant.maximum_depth{return Err(invalid("snapshot settlement exceeds original wallet"))}*slot=Some(NativeSnapshotEncodeSettlement{original:None,grant:self.grant,before:self.progress,local_done:false,local_failure:None,operation_unwind:None});let result=self.native.begin_effect_settlement(&mut slot.as_mut().unwrap().original,semio_framework_value::RetirementDemand{copy_bytes:demand.maximum_copy_bytes,capacity_bytes:demand.maximum_capacity_bytes,release_bytes:demand.maximum_release_bytes,depth:demand.maximum_depth},1);if result.is_err()&&slot.as_ref().is_some_and(|state|state.original.is_none()){slot.take();}result}
 /// 🪢️ Reconciles an interrupted original quote without replay.
 pub fn recover_original_quote(&mut self,slot:&mut Option<NativeSnapshotEncodeSettlement>)->Result<bool,ValueError>{let state=slot.as_mut().ok_or_else(||invalid("snapshot settlement is absent"))?;if state.grant!=self.grant||state.before!=self.progress{return Err(invalid("snapshot original wallet changed"))}let result=self.native.recover_effect_quote(&mut state.original);if result.is_err()&&state.original.is_none(){slot.take();}result}
 /// 🎟️ Resumes work admission after reconciling an original quote.
 pub fn resume_original_admission(&mut self,slot:&mut Option<NativeSnapshotEncodeSettlement>)->Result<bool,ValueError>{let state=slot.as_mut().ok_or_else(||invalid("snapshot settlement is absent"))?;if state.grant!=self.grant||state.before!=self.progress{return Err(invalid("snapshot original wallet changed"))}let result=self.native.resume_effect_admission(&mut state.original);if result.is_err()&&state.original.is_none(){slot.take();}result}
 /// ↩️ Cancels a matching original reservation before any physical effect.
 pub fn cancel_original_settlement(&mut self,slot:&mut Option<NativeSnapshotEncodeSettlement>)->Result<semio_framework_value::native_encoding::NativeEncodeEffectCancellation,ValueError>{let state=slot.as_mut().ok_or_else(||invalid("snapshot settlement is absent"))?;if state.grant!=self.grant||state.before!=self.progress||state.local_done{return Err(invalid("snapshot original wallet changed"))}let token=self.native.cancel_effect_settlement(&mut state.original)?;state.local_done=true;if state.operation_unwind.is_none(){slot.take();}Ok(token)}
 /// ↩️ Releases a refused original quote through the same snapshot owner.
 pub fn release_original_refusal(&mut self,slot:&mut Option<NativeSnapshotEncodeSettlement>)->Result<semio_framework_value::native_encoding::NativeEncodeEffectCancellation,ValueError>{let state=slot.as_mut().ok_or_else(||invalid("snapshot settlement is absent"))?;if state.grant!=self.grant||state.before!=self.progress||state.local_done{return Err(invalid("snapshot original wallet changed"))}let token=self.native.release_refused_effect(&mut state.original)?;state.local_done=true;if state.operation_unwind.is_none(){slot.take();}Ok(token)}
 /// 🧾️ Records local physical effects once, then settles original callback phases.
 pub fn settle_original_effect(&mut self,slot:&mut Option<NativeSnapshotEncodeSettlement>)->Result<bool,ValueError>{let state=slot.as_mut().ok_or_else(||invalid("snapshot settlement is absent"))?;let original=state.original.as_ref().ok_or_else(||invalid("original effect quote is absent"))?;if !original.matches_original(self.native.work_recipient_identity(),self.native.construction_recipient_identity())||state.grant!=self.grant{return Err(invalid("snapshot original authority changed"))}let progress=original.progress().ok_or_else(||invalid("snapshot original effect is absent"))?;if !state.local_done{if self.progress!=state.before{return Err(invalid("snapshot original wallet changed"))}state.local_failure=self.record_progress(progress).err();state.local_done=true}self.native.settle_effect(&mut state.original)}
}
