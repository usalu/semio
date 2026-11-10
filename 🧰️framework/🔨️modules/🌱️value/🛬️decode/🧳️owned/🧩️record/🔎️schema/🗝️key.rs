//! 🗝️ The original schema-key frame uses one incrementally initialized native backing.
use super::*;
use semio_framework_value_native_decoding::native_decoding::{OriginalEffectReply,OriginalEffectReplyComplete};

/// 🚦️ Distinguishes preflight denial, an unresolved original action and a settled result.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum NativeSchemaKeyPoll{Denied,Pending{work:u128},Ready{work:u128,progress:RetainedCloneProgress}}
/// 🧭️ Selects a separately funded phase of the original acknowledgement owner.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeSchemaKeyMetadata{Bind,CancelBinding,Prepare,CloseMetadata,Rearm,CloseAuthority}
impl<I:NativeValueSource+?Sized> NativeRecordMatcher<I>{
 /// 📐️ Quotes the initial actual Source-bound key authority.
 pub fn key_start_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing||!self.key_backing.initialized(){return Err(invalid())}Ok(NativeSchemaKeyFrame::<I>::start_demand())}
 /// 🌱️ Seats the original key producer under the same actual Work effect.
 pub fn key_start_into<'r>(&mut self,input:RetainedCloneRef<'_,I>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.key_start_demand()?;self.key_backing.validate_effect(effect)?;self.key_backing.get_mut().ok_or_else(invalid)?.start_into(input,self.key_source,effect,reply)}
 /// 📐️ Quotes the frozen original key Source action and its Work without any callback authority.
 pub fn key_action_quote(&self,input:RetainedCloneRef<'_,I>)->Result<(RetirementDemand,u64),ValueError>{if self.closing{return Err(invalid())}self.key_backing.get().ok_or_else(invalid)?.quote(input)}
 /// 🧵️ Drives or resumes the actual original key Source action.
 pub fn key_action_advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:crate::OriginalSourceTurnWitness<'_>,body:usize,maximum:u64,grant:RetainedCloneGrant)->Result<NativeSchemaKeyPoll,ValueError>{if self.closing{return Err(invalid())}let binding=control.original_work_borrow().binding_identity()?;self.key_backing.validate_work_binding(binding)?;let state=self.key_backing.get_mut().ok_or_else(invalid)?.advance(input,control,witness,body,maximum,grant)?;Ok(match state{crate::source::key_frame::NativeSchemaKeyFramePoll::Denied=>NativeSchemaKeyPoll::Denied,crate::source::key_frame::NativeSchemaKeyFramePoll::Pending{work}=>NativeSchemaKeyPoll::Pending{work},crate::source::key_frame::NativeSchemaKeyFramePoll::Ready{work,progress}=>NativeSchemaKeyPoll::Ready{work,progress}})}
 /// ⏳️ Reports a frozen key request that has not reached Ready.
 pub fn key_request_unsettled(&self)->bool{self.key_backing.get().is_some_and(NativeSchemaKeyFrame::pending)}
 /// 🛑️ Reports a checkpoint interruption that needs its original recovery owner.
 pub fn key_checkpoint_unresolved(&self)->bool{self.key_backing.get().is_some_and(NativeSchemaKeyFrame::checkpoint_unresolved)}
 /// 📬️ Reports an outcome retained for separately paid publication.
 pub fn key_action_result_ready(&self)->bool{self.key_backing.get().is_some_and(NativeSchemaKeyFrame::result_ready)}
 /// 🧾️ Borrows the actual resident acknowledgement owner for phase selection.
 pub(crate) fn key_acknowledgement(&self)->Result<&semio_framework_value_native_decoding::NativeResidentSuccessAck<ValueError>,ValueError>{Ok(self.key_backing.get().ok_or_else(invalid)?.acknowledgement())}
 /// ⏳️ Reports that the exact original metadata Source Post remains to be authenticated.
 pub fn key_post_pending(&self)->Result<bool,ValueError>{Ok(self.key_acknowledgement()?.source_post_pending())}
 /// ♻️ Selects the real remaining acknowledgement metadata before parent close.
 pub fn key_close_metadata(&self)->Result<Option<NativeSchemaKeyMetadata>,ValueError>{let owner=self.key_acknowledgement()?;if owner.source_post_pending(){return Ok(None)}if owner.metadata_close_demand().is_ok(){return Ok(Some(NativeSchemaKeyMetadata::CloseMetadata))}if owner.cancel_unprepared_binding_demand().is_ok(){return Ok(Some(NativeSchemaKeyMetadata::CancelBinding))}if owner.authority_close_demand().is_ok(){return Ok(Some(NativeSchemaKeyMetadata::CloseAuthority))}Ok(None)}
 /// 📐️ Quotes only the selected actual owner transition.
 pub fn key_metadata_demand(&self,operation:NativeSchemaKeyMetadata)->Result<RetirementDemand,ValueError>{let owner=self.key_acknowledgement()?;match operation{NativeSchemaKeyMetadata::Bind=>owner.bind_semantic_demand(),NativeSchemaKeyMetadata::CancelBinding=>owner.cancel_unprepared_binding_demand(),NativeSchemaKeyMetadata::Prepare=>owner.prepare_authority_demand(),NativeSchemaKeyMetadata::CloseMetadata=>owner.metadata_close_demand(),NativeSchemaKeyMetadata::Rearm=>owner.rearm_authority_demand(),NativeSchemaKeyMetadata::CloseAuthority=>owner.authority_close_demand()}}
 /// 📬️ Performs the selected transition under its real construction and Work binding.
 pub fn key_metadata_into<'r>(&mut self,operation:NativeSchemaKeyMetadata,binding:semio_framework_value_native_decoding::NativeReceivingConstructionBinding,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.key_backing.validate_effect(effect)?;if !binding.matches_effect(effect){return Err(ValueError::invariant("schema acknowledgement construction binding changed"))}let owner=self.key_backing.get_mut().ok_or_else(invalid)?.acknowledgement_mut();match operation{NativeSchemaKeyMetadata::Bind=>owner.bind_semantic_into(binding,effect,reply),NativeSchemaKeyMetadata::CancelBinding=>owner.cancel_unprepared_binding_into(binding,effect,reply),NativeSchemaKeyMetadata::Prepare=>owner.prepare_authority_into(binding,effect,reply),NativeSchemaKeyMetadata::CloseMetadata=>owner.close_metadata_into(binding,effect,reply),NativeSchemaKeyMetadata::Rearm=>owner.rearm_authority_into(binding,effect,reply),NativeSchemaKeyMetadata::CloseAuthority=>owner.close_authority_into(binding,effect,reply)}}
 /// 🛂️ Accepts only the original published Source Post and finished Work proof.
 pub fn key_accept_post(&mut self,proof:&semio_framework_value_native_decoding::OriginalConstructionAcceptedProof<'_>)->Result<bool,ValueError>{self.key_backing.validate_work_binding(proof.work_binding())?;self.key_backing.get_mut().ok_or_else(invalid)?.acknowledgement_mut().accept_close(proof)}
 /// 🧵️ Drives the same private acknowledgement with the actual decoder Source.
 pub fn key_advance_acknowledgement(&mut self,control:&mut NativeDecodeControl<'_>)->Result<semio_framework_value_native_decoding::native_decoding::NativeReceivingEffectState,ValueError>{self.key_backing.validate_work_binding(control.original_work_borrow().binding_identity()?)?;self.key_backing.get_mut().ok_or_else(invalid)?.advance_acknowledgement(control)}
 /// 📐️ Quotes direct retention of every original key-action outcome cause.
 pub(crate) fn key_action_result_demand(&self)->Result<RetirementDemand,ValueError>{self.key_backing.get().ok_or_else(invalid)?.effect_result_demand()}
 /// 📤️ Transfers the whole original key-action outcome to the caller-resident slot.
 pub(crate) fn key_take_action_result_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.key_backing.validate_effect(effect)?;self.key_backing.get_mut().ok_or_else(invalid)?.take_effect_result_into(effect,reply)}
 /// 📐️ Quotes release of a fully settled frozen key request.
 pub(crate) fn key_clear_request_demand(&self)->RetirementDemand{NativeSchemaKeyFrame::<I>::clear_request_demand()}
 /// 🧹️ Clears only a settled request after its published outcome is relinquished.
 pub(crate) fn key_clear_request_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.key_backing.validate_effect(effect)?;self.key_backing.get_mut().ok_or_else(invalid)?.clear_request_into(effect,reply)}
 /// 👁️ Borrows a fully settled key only while its original projection remains resident.
 pub fn key_value(&self)->Result<Option<crate::source::NativeSourceKeyValue<'_>>,ValueError>{self.key_backing.get().ok_or_else(invalid)?.key()}
 /// 👁️ Observes a settled key refusal without moving its original owner.
 pub fn key_error_ready(&self)->Result<bool,ValueError>{Ok(self.key_backing.get().ok_or_else(invalid)?.result().is_some_and(Result::is_err))}
 /// 📐️ Quotes direct movement of the original owned key refusal.
 pub(crate) fn key_error_demand(&self)->Result<RetirementDemand,ValueError>{if !self.key_error_ready()?{return Err(invalid())}Ok(NativeSchemaKeyFrame::<I>::transfer_error_demand())}
 /// 📤️ Moves the exact original key refusal into the caller-resident slot.
 pub(crate) fn key_error_into<'r>(&mut self,output:&mut Option<ValueError>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.key_error_demand()?;self.key_backing.validate_effect(effect)?;self.key_backing.get_mut().ok_or_else(invalid)?.transfer_error_into(output,effect,reply)}
 /// 📐️ Quotes clearing a fully consumed native key reference.
 pub(crate) fn key_clear_demand(&self)->Result<RetirementDemand,ValueError>{if self.key_value()?.is_none(){return Err(invalid())}Ok(NativeSchemaKeyFrame::<I>::release_key_demand())}
 /// 🧹️ Clears only the settled key reference under its original Source and Work effect.
 pub(crate) fn key_clear_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.key_clear_demand()?;self.key_backing.validate_effect(effect)?;self.key_backing.get_mut().ok_or_else(invalid)?.release_key_into(effect,reply)}
 /// 📐️ Quotes binding the settled key to its initial resident matcher frame.
 pub fn key_bind_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing||self.key.is_some()||self.pending.is_some()||!self.frames.is_empty()||self.key_value()?.is_none(){return Err(invalid())}let bytes=size_of::<Option<Key>>().checked_add(size_of::<Option<SchemaFrame>>()).and_then(|value|value.checked_add(size_of::<Option<Result<RetainedCloneStep,ValueError>>>())).ok_or_else(invalid)?;Ok(fixed(bytes))}
 /// 🧷️ Installs only the settled key and fixed native root descriptor under the actual Work effect.
 pub fn key_bind_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{let demand=self.key_bind_demand()?;self.key_backing.validate_effect(effect)?;if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}let key=match self.key_value()?.ok_or_else(invalid)?{crate::source::NativeSourceKeyValue::Name(name)=>Key::Name(name as *const str),crate::source::NativeSourceKeyValue::Ordinal(value)=>Key::Ordinal(value)};if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"schema key binding exceeded Work"))))}self.key=Some(key);self.pending=Some(SchemaFrame{fields:self.fields,count:self.count,index:0,root:None,role:0,before:NativeFieldMatch::Missing,best:NativeFieldMatch::Missing});Ok(reply.complete(Ok(RetainedCloneStep::Progress(receipt(demand)))))}
 /// 📐️ Quotes the next genuine key-frame backing birth phase.
 pub fn key_birth_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing||self.key.is_some(){return Err(invalid())}self.key_backing.birth_demand()}
 /// ⏱️ Quotes the next genuine key-frame backing birth Work.
 pub fn key_birth_work_demand(&self)->Result<u64,ValueError>{self.key_birth_demand()?;Ok(self.key_backing.birth_work_demand())}
 /// 🌱️ Drives one paid native key-frame backing birth phase.
 pub fn key_birth_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.key_birth_demand()?;self.key_backing.birth_into(effect,reply)}
 /// 🌱️ Drives the same native birth under an original retirement reply.
 pub fn key_birth_retirement_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,semio_framework_value_retirement::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,semio_framework_value_retirement::OriginalRetirementFailure>,ValueError>{self.key_birth_demand()?;self.key_backing.birth_retirement_into(effect,reply)}
 /// 👁️ Reports publication only after every vacant field is initialized.
 pub fn key_backing_initialized(&self)->bool{self.key_backing.initialized()}
 pub(super) fn key_release_proof(&self)->Result<Option<crate::retained_clone::RetainedCursorRelease<NativeSchemaKeyFrame<I>>>,ValueError>{if self.key.is_some(){return Err(invalid())}match self.key_backing.retained(){Some(frame)=>{let proof=frame.native_release_witness().ok_or_else(invalid)?;if !proof.validates(frame){return Err(invalid())}Ok(Some(proof))},None=>Ok(None)}}
 /// 📐️ Quotes the next native backing release after the original key frame settles.
 pub fn key_backing_release_demand(&self)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}let proof=self.key_release_proof()?;self.key_backing.release_demand(proof.as_ref())}
 /// ⏱️ Quotes the next native backing release Work.
 pub fn key_backing_release_work_demand(&self)->Result<u64,ValueError>{self.key_backing_release_demand()?;Ok(self.key_backing.release_work_demand())}
 /// ♻️ Releases the actual partial or terminal backing on the same Work effect.
 pub fn key_backing_release_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{if !self.closing{return Err(invalid())}let proof=self.key_release_proof()?;self.key_backing.release_into(proof,effect,reply)}
}
