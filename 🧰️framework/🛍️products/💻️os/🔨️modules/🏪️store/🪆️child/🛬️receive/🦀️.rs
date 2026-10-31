//! 🛬️ Persisted child headers receive from their original fields without inventing local materialization.
use super::ArtifactChild;
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_value::{RetainedWorkOutcome, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, ValueError, retained_clone::{RetainedCloneRef, retained_clone_grant_admits}};
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeReceive, native_decoding::NativeReceiver, native_decoding::OriginalReceivingPrimary, native_decoding::owned::NativeRecordFields, native_decoding::owned::tree::NativeValueSource, native_decoding::owned::tree::child_grant, native_decoding::owned::tree::nested, native_decoding::owned::tree::add_receipt};
use std::{marker::PhantomData,mem::size_of};

#[derive(semio_framework_value::RetireOwned,semio_framework_value::NativeReceive)]
#[value(rename_all="camelCase")]
struct PersistedChildHeader { child_id:String,target:ArtifactRef }

/// 🪆️ Receives exactly the two persisted fields while retaining all child receiver custody.
pub struct ArtifactChildNativeReceiver<I:NativeValueSource+?Sized,S:Send+'static> {
    header:PersistedChildHeaderNativeReceiver<I>,
    snapshot:PhantomData<fn()->S>,
}

impl<S:Send+'static> NativeRecordFields for ArtifactChild<S> {
    fn native_schema_count()->usize { PersistedChildHeader::native_schema_count() }
    fn native_schema_field(index:usize)->Option<semio_framework_value_native_owned::native_decoding::owned::NativeRecordSchema> { PersistedChildHeader::native_schema_field(index) }
}

impl<I:NativeValueSource+?Sized,S:Send+'static> NativeReceive<I> for ArtifactChild<S> {
    type Receiver=ArtifactChildNativeReceiver<I,S>;
    fn receiver()->Self::Receiver { ArtifactChildNativeReceiver { header:<PersistedChildHeader as NativeReceive<I>>::receiver(),snapshot:PhantomData } }
}

impl<I:NativeValueSource+?Sized,S:Send+'static> NativeReceiver<I,ArtifactChild<S>> for ArtifactChildNativeReceiver<I,S> {
    fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError> { self.header.demands(original,body) }
    fn advance(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary> { self.header.advance(original,control,body,grant) }
    fn output_ready(&self)->bool { self.header.output_ready() }
    fn take_demand(&self)->Result<RetirementDemand,ValueError> { nested(self.header.take_demand()?,2*size_of::<ArtifactChild<S>>()) }
    fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(ArtifactChild<S>,RetainedCloneProgress)>> {
        let mut work=0;
        let result=(||->Result<Option<(ArtifactChild<S>,RetainedCloneProgress)>,ValueError> {
            let demand=self.take_demand()?;
            if !retained_clone_grant_admits(grant,demand) { return Ok(None); }
            let overhead=2*size_of::<ArtifactChild<S>>();
            let turn=self.header.take(child_grant(grant,overhead));
            work=turn.work;
            let Some((header,progress))=turn.result? else { return Ok(None); };
            Ok(Some((ArtifactChild::new(header.child_id,header.target),add_receipt(progress,overhead)?)))
        })();
        RetainedWorkOutcome { result,work }
    }
    fn begin_close(&mut self) { self.header.begin_close(); }
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError> { self.header.close_demands(body) }
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome { self.header.close_step(grant) }
    fn terminal_is_empty(&self)->bool { self.header.terminal_is_empty() }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
