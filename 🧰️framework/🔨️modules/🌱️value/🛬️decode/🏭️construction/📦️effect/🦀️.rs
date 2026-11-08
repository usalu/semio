//! 🧭️ Native control lends its actual authorities to the original receiving frame.
use super::{NativeDecodeControl,OriginalConstructionLoan};
use crate::{RetainedWorkOutcome,RetirementDemand,RetainedCloneGrant,RetainedCloneStep,ValueError,ValueRefusalKind};
pub use semio_framework_value_native_receiving_frame::{NativeReceivingConstructionBinding,NativeReceivingSuccessIdentity,NativeReceivingEffect,NativeReceivingEffectPhase,NativeReceivingEffectPoll,NativeReceivingEffectState,NativeReceivingEffectOutcome,NativeEffectCapturePlan,NativeReceivingEffectRetirement,NativeReceivingEffectError,NativeReceivingEffectFailure,OriginalEffectAction,OriginalEffectAuthority,OriginalEffectCallbackMode,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete};

semio_framework_value_native_receiving_frame::native_resident_success_actor!();

impl semio_framework_value_native_receiving_frame::NativeReceivingEffectHost for NativeDecodeControl<'_>{
 fn original_work(&mut self)->semio_framework_value_control_contract::OriginalWorkBorrow<'_>{semio_framework_value_control_contract::OriginalWorkBorrow::new(&mut self.work)}
 fn begin_construction(&mut self,demand:RetirementDemand,action:&mut Option<semio_framework_value_control_contract::OriginalConstructionAction>)->Result<semio_framework_value_control_contract::OriginalConstructionIssuerIdentity,ValueError>{let work=semio_framework_value_control_contract::OriginalWorkBorrow::new(&mut self.work);let recipient=self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native receiving requires its original construction recipient"))?;recipient.begin(demand,action,&work)?;Ok(recipient.identity())}
 fn work_issuer(&self)->Option<semio_framework_value_control_contract::OriginalWorkIssuerIdentity>{self.work.recipient.as_ref().map(|loan|loan.identity())}
 fn construction(&mut self)->Result<OriginalConstructionLoan<'_>,ValueError>{self.construction.as_deref_mut().map(|loan|loan.reborrow()).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native receiving requires its original construction recipient"))}
 fn publish_receiving_completion(&mut self,receiving:&mut crate::OriginalReceivingPrimary,payout:&semio_framework_value_control_contract::OriginalConstructionAction,proof:&semio_framework_value_control_contract::OriginalWorkActionProof)->Result<bool,ValueError>{let work=semio_framework_value_control_contract::OriginalWorkBorrow::new(&mut self.work);let recipient=self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native receiving requires its original construction recipient"))?;receiving.publish_receipt_completion(recipient,payout,&work,proof)}
 fn checkpoint(&mut self)->Result<(),ValueError>{NativeDecodeControl::checkpoint(self)}
 fn has_retirement(&self)->bool{self.retirement.is_some()}
 fn observe_retirement(&mut self)->Result<(),ValueError>{let owned_bytes=self.owned_bytes();let _=(self.callback)(super::super::NativeDecodeProgress{completed:self.completed,total:self.total,owned_bytes});Ok(())}
 fn pending_work(&self)->u64{NativeDecodeControl::pending_work(self)}
 fn has_pending_work(&self)->bool{NativeDecodeControl::has_pending_work(self)}
 fn work_quote_pending(&self)->bool{NativeDecodeControl::work_quote_pending(self)}
 fn validate_work_proof(&self,proof:&semio_framework_value_control_contract::OriginalWorkActionProof,actual:u128)->Result<bool,ValueError>{NativeDecodeControl::validate_work_proof(self,proof,actual)}
 fn prepare_allocation_effect(&mut self,bytes:usize,actual:&Option<usize>,state:&mut Option<super::super::allocation::NativeDecodeAllocationProgress>,cancellable:bool)->super::super::allocation::NativeDecodeAllocationAdmission{NativeDecodeControl::prepare_allocation_effect(self,bytes,actual,state,cancellable)}
 fn recover_allocation_callback(&mut self,state:&mut super::super::allocation::NativeDecodeAllocationProgress)->Result<bool,ValueError>{NativeDecodeControl::recover_allocation_callback(self,state)}
 fn settle_allocation_progress(&mut self,state:&mut super::super::allocation::NativeDecodeAllocationProgress,actual:Option<usize>)->Result<bool,ValueError>{NativeDecodeControl::settle_allocation_progress(self,state,actual)}}
semio_framework_value_native_receiving_frame::native_receiving_control_driver!();
impl NativeDecodeControl<'_>{
 /// 🪛 Runs one closure inside one original Work action on this control's own recipient; `None` means the quote was not admitted and nothing ran or was charged. The control handed to the closure carries the live work action: children may charge, checkpoint and read bytes but must never prepare_work.
 pub fn with_effect<R>(&mut self,grant:RetainedCloneGrant,maximum_work:u64,run:impl FnOnce(&mut Self,&mut crate::RetainedAllocationEffect<'_>)->R)->Result<Option<R>,ValueError>{
  let Some(mut open)=semio_framework_value_control_contract::OriginalEffectRun::begin(&mut self.work,grant,maximum_work)?else{return Ok(None)};
  let output={let mut effect=open.effect()?;run(self,&mut effect)};
  open.finish(&mut self.work,&mut 0)?;Ok(Some(output))
 }
}
