//! 🛬️ Persisted child headers receive from their original fields without inventing local materialization.
use super::ArtifactChild;
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_value::{OriginalRetirementCloseLoan, OriginalRetirementFailure, OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneStep, RetainedWorkPoll, RetirementDemand, ValueError, retained_clone::RetainedCloneRef};
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeReceive, native_decoding::NativeReceiver, native_decoding::NativeReceivingFailure, native_decoding::OriginalReceivingPrimary,native_decoding::owned::OriginalSourceTurnWitness,native_decoding::OriginalEffectReply,native_decoding::OriginalEffectReplyComplete,native_decoding::owned::NativeAdvanceSelection, native_decoding::owned::NativeRecordFields, native_decoding::owned::tree::NativeValueSource, native_decoding::owned::mapped::{NativeMappedReceiver,NativeReceiveMap}};
use std::marker::PhantomData;

#[derive(semio_framework_value::RetireOwned,semio_framework_value::NativeReceive)]
#[value(rename_all="camelCase")]
struct PersistedChildHeader { child_id:String,target:ArtifactRef }

/// 🪆️ Receives exactly the two persisted fields while retaining all child receiver custody.
pub struct ArtifactChildNativeReceiver<I:NativeValueSource+?Sized,S:Send+'static> {
    header:NativeMappedReceiver<I,PersistedChildHeader,ArtifactChild<S>>,
    snapshot:PhantomData<fn()->S>,
}

unsafe impl<S:Send+'static> NativeRecordFields for ArtifactChild<S> {
    fn native_schema_count()->usize { PersistedChildHeader::native_schema_count() }
    fn native_schema_field(index:usize)->Option<semio_framework_value_native_owned::native_decoding::owned::NativeRecordSchema> { PersistedChildHeader::native_schema_field(index) }
}

impl<S:Send+'static> NativeReceiveMap<PersistedChildHeader> for ArtifactChild<S> {
    fn from_native(header:PersistedChildHeader)->Self { Self::new(header.child_id,header.target) }
}

impl<I:NativeValueSource+?Sized,S:Send+'static> NativeReceive<I> for ArtifactChild<S> {
    type Receiver=ArtifactChildNativeReceiver<I,S>;
    fn receiver()->Self::Receiver { ArtifactChildNativeReceiver { header:NativeMappedReceiver::new(),snapshot:PhantomData } }
}

impl<I:NativeValueSource+?Sized,S:Send+'static> NativeReceiver<I,ArtifactChild<S>> for ArtifactChildNativeReceiver<I,S> {
    fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError> { self.header.demands(original,body,maximum) }
    fn select_advance(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum:u64)->Result<NativeAdvanceSelection,ValueError> { self.header.select_advance(original,body,maximum) }
    fn advance_nested(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure> { self.header.advance_nested(original,control,witness,body,maximum,grant) }
    fn advance_local<'reply>(&mut self,original:RetainedCloneRef<'_,I>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError> { self.header.advance_local(original,body,effect,reply) }
    fn recover_advance_local<'reply>(&mut self,original:RetainedCloneRef<'_,I>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError> { self.header.recover_advance_local(original,body,effect,reply) }
    fn output_ready(&self)->bool { self.header.output_ready() }
    fn take_demand(&self)->Result<RetirementDemand,ValueError> { self.header.take_demand() }
    fn take_work_demand(&self)->Result<u64,ValueError> { self.header.take_work_demand() }
    fn take<'reply>(&mut self,output:&mut Option<ArtifactChild<S>>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError> { self.header.take(output,effect,reply) }
    fn recover_take<'reply>(&mut self,output:&mut Option<ArtifactChild<S>>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError> { self.header.recover_take(output,effect,reply) }
    fn advance_pending(&self)->bool { self.header.advance_pending() }
    fn is_closing(&self)->bool { self.header.is_closing() }
    fn begin_close_demand(&self)->Result<RetirementDemand,ValueError> { self.header.begin_close_demand() }
    fn begin_close_work_demand(&self)->Result<u64,ValueError> { self.header.begin_close_work_demand() }
    fn begin_close_into<'reply>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError> { self.header.begin_close_into(effect,reply) }
    fn recover_begin_close_into<'reply>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError> { self.header.recover_begin_close_into(effect,reply) }
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError> { self.header.close_demands(body) }
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError> { self.header.close_work_demand(body) }
    fn close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError> { self.header.close_step(body,effect,reply) }
    fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError> { self.header.recover_close_step(body,effect,reply) }
    fn requires_original_control(&self)->bool { self.header.requires_original_control() }
    fn requires_original_birth(&self)->bool { self.header.requires_original_birth() }
    fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll { self.header.close_original_birth_step(original,grant) }
    fn close_original_step<'reply>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError> { self.header.close_original_step(original,body,effect,reply) }
    fn recover_close_original_step<'reply>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError> { self.header.recover_close_original_step(original,body,effect,reply) }
    fn terminal_is_empty(&self)->bool { self.header.terminal_is_empty() }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
