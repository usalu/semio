//! 🛬️ Persisted child headers receive from their original fields without inventing local materialization.
use super::ArtifactChild;
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_value::{RetainedWorkOutcome, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, ValueError, retained_clone::RetainedCloneRef};
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeReceive, native_decoding::NativeReceiver, native_decoding::OriginalReceivingPrimary, native_decoding::owned::NativeRecordFields, native_decoding::owned::tree::NativeValueSource, native_decoding::owned::mapped::{NativeMappedReceiver,NativeReceiveMap}};
use semio_framework_value::native_decoding::owned::{NativeClosePermit,NativeRecoveryLoan};
use std::marker::PhantomData;

#[derive(semio_framework_value::RetireOwned,semio_framework_value::NativeReceive)]
#[value(rename_all="camelCase")]
struct PersistedChildHeader { child_id:String,target:ArtifactRef }

/// 🪆️ Receives exactly the two persisted fields while retaining all child receiver custody.
pub struct ArtifactChildNativeReceiver<I:NativeValueSource+?Sized,S:Send+'static> {
    header:NativeMappedReceiver<I,PersistedChildHeader,ArtifactChild<S>>,
    snapshot:PhantomData<fn()->S>,
}

impl<S:Send+'static> NativeRecordFields for ArtifactChild<S> {
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
    fn advance_work_demand(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum:u64)->Result<u64,ValueError> { self.header.advance_work_demand(original,body,maximum) }
    fn take_work_demand(&self)->Result<u64,ValueError> { self.header.take_work_demand() }
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError> { self.header.close_work_demand(body) }
    fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError> { self.header.demands(original,body,maximum) }
    fn advance(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,maximum:u64,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary> { self.header.advance(original,control,body,maximum,grant) }
    fn output_ready(&self)->bool { self.header.output_ready() }
    fn take_demand(&self)->Result<RetirementDemand,ValueError> { self.header.take_demand() }
    fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(ArtifactChild<S>,RetainedCloneProgress)>> { self.header.take(grant) }
 fn has_pending_recovery(&self)->bool{self.header.has_pending_recovery()}
 fn pending_recovery(&mut self)->Option<NativeRecoveryLoan<'_>>{self.header.pending_recovery()}
 fn resume_recovery(&mut self,control:&mut NativeDecodeControl<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{self.header.resume_recovery(control)}
 fn begin_close_admitted(&mut self,_permit:NativeClosePermit)->Result<(),ValueError>{self.header.begin_close()}
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError> { self.header.close_demands(body) }
    fn close_step(&mut self,grant:RetainedCloneGrant,actual:&mut Option<usize>)->semio_framework_value::OriginalRetirementOutcome { self.header.close_step(grant,actual) }
    fn terminal_is_empty(&self)->bool { self.header.terminal_is_empty() }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
