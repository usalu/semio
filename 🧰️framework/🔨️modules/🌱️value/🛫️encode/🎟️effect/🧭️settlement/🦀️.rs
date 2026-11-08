//! 🧭️ Control lends its actual Work and construction authorities to the settlement owner.
use super::super::NativeEncodeControl;
use crate::{RetirementDemand,ValueError,ValueRefusalKind};
pub use semio_framework_value_native_outcome::{NativeEncodeEffectSettlement,NativeEncodeEffectPostPhase,NativeEncodeEffectReceipt,NativeEncodeEffectCancellation,NativeEncodeEffectRecovery};
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
semio_framework_value_native_outcome::native_encode_receipt_payout!();
impl NativeEncodeControl<'_>{
 /// 🎫️ Starts the exact original physical quote and Work admission.
 pub fn begin_effect_settlement(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>,demand:RetirementDemand,maximum_work:u64)->Result<bool,ValueError>{NativeEncodeEffectRecovery::new(&mut self.action,&mut self.construction,self.construction_identity).begin(slot,demand,maximum_work)}
 /// 🪢️ Recovers the same recorded physical quote.
 pub fn recover_effect_quote(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>)->Result<bool,ValueError>{NativeEncodeEffectRecovery::new(&mut self.action,&mut self.construction,self.construction_identity).recover_quote(slot)}
 /// ⛽️ Resumes the original admitted Work action.
 pub fn resume_effect_admission(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>)->Result<bool,ValueError>{NativeEncodeEffectRecovery::new(&mut self.action,&mut self.construction,self.construction_identity).resume_admission(slot)}
 /// ↩️ Cancels an unperformed original action.
 pub fn cancel_effect_settlement(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>)->Result<NativeEncodeEffectCancellation,ValueError>{NativeEncodeEffectRecovery::new(&mut self.action,&mut self.construction,self.construction_identity).cancel(slot)}
 /// ↩️ Returns a refused original quote.
 pub fn release_refused_effect(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>)->Result<NativeEncodeEffectCancellation,ValueError>{NativeEncodeEffectRecovery::new(&mut self.action,&mut self.construction,self.construction_identity).release_refused(slot)}
 /// 🧾️ Posts the same original physical receipt.
 pub fn settle_effect(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>)->Result<bool,ValueError>{NativeEncodeEffectRecovery::new(&mut self.action,&mut self.construction,self.construction_identity).settle(slot)}
 /// 🧾️ Returns the original child Work proof before funding its refused receipt.
 pub fn finish_refused_effect_work(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>)->Result<(),ValueError>{NativeEncodeEffectRecovery::new(&mut self.action,&mut self.construction,self.construction_identity).finish_refused_work(slot)}
 /// 🔓️ Publishes the actual retained receipt after its same-Source payout settles.
 pub fn publish_refused_effect_receipt(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>,payout:&semio_framework_value_control_contract::OriginalConstructionAction,proof:&semio_framework_value_control_contract::OriginalWorkActionProof)->Result<bool,ValueError>{NativeEncodeEffectRecovery::new(&mut self.action,&mut self.construction,self.construction_identity).publish_refused_receipt(slot,payout,proof)}
 /// 🫴️ Lends the actual disjoint Work and physical authorities to a scoped payout.
 pub fn with_original_effect_work_and_construction<R>(&mut self,operation:impl FnOnce(&mut semio_framework_value_control_contract::OriginalWorkBorrow<'_>,&mut semio_framework_value_control_contract::OriginalConstructionLoan<'_>)->Result<R,ValueError>)->Result<R,ValueError>{let recipient=self.construction.as_deref_mut().ok_or_else(||invalid("original construction source is absent"))?;let mut recipient=recipient.reborrow();let mut work=semio_framework_value_control_contract::OriginalWorkBorrow::new(&mut self.action);operation(&mut work,&mut recipient)}
 /// 🪢️ Recovers the recorded original physical reply.
 pub fn recover_effect_receipt(&mut self,slot:&mut Option<NativeEncodeEffectSettlement>)->Result<bool,ValueError>{self.settle_effect(slot)}
 /// 👁️ Visits the actual retained frame with its original effect authorities.
 pub fn visit_effect_original<T:Send+'static,O:Send+'static>(&mut self,recipient_identity:u64,visitor:&mut impl FnMut(&mut Option<T>,&mut NativeEncodeEffectRecovery<'_,'_>))->Result<bool,ValueError>{let recipient=self.retirement.as_deref_mut().ok_or_else(||invalid("original effect recovery recipient is absent"))?;if recipient.identity()!=recipient_identity{return Err(invalid("original effect recovery recipient changed"))}let mut recovery=NativeEncodeEffectRecovery::new(&mut self.action,&mut self.construction,self.construction_identity);Ok(crate::ErasedSnapshotRetirement::visit_original(recipient,std::any::TypeId::of::<(T,O)>(),&mut |original|{if let Some(frame)=original.downcast_mut::<Option<T>>(){visitor(frame,&mut recovery)}}))}
}
