//! 🛬️ Persisted child headers receive from their original fields without inventing local materialization.
use super::ArtifactChild;
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, ValueError, retained_clone::{RetainedCloneRef, retained_clone_grant_admits}};
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeReceive, native_decoding::NativeReceiver, native_decoding::owned::NativeRecordFields, native_decoding::owned::tree::NativeValueSource, native_decoding::owned::tree::child_grant, native_decoding::owned::tree::nested, native_decoding::owned::tree::add_receipt};
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
    fn native_field_memberships(key:semio_framework_value_native_owned::native_decoding::owned::NativeRecordKey<'_>)->usize { PersistedChildHeader::native_field_memberships(key) }
    fn native_field_key_bytes(key:semio_framework_value_native_owned::native_decoding::owned::NativeRecordKey<'_>)->usize { PersistedChildHeader::native_field_key_bytes(key) }
}

impl<I:NativeValueSource+?Sized,S:Send+'static> NativeReceive<I> for ArtifactChild<S> {
    type Receiver=ArtifactChildNativeReceiver<I,S>;
    fn receive_birth_demand()->RetirementDemand {
        nested(<PersistedChildHeader as NativeReceive<I>>::receive_birth_demand(),size_of::<Self::Receiver>()).expect("fixed child receiving birth fits usize")
    }
    fn begin_receive(grant:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError> {
        if !retained_clone_grant_admits(grant,<Self as NativeReceive<I>>::receive_birth_demand()) { return Err(semio_framework_value_native_owned::native_decoding::owned::tree::invalid()); }
        let overhead=size_of::<Self::Receiver>();
        let (header,progress)=<PersistedChildHeader as NativeReceive<I>>::begin_receive(child_grant(grant,overhead))?;
        Ok((ArtifactChildNativeReceiver { header,snapshot:PhantomData },add_receipt(progress,overhead)?))
    }
}

impl<I:NativeValueSource+?Sized,S:Send+'static> NativeReceiver<I,ArtifactChild<S>> for ArtifactChildNativeReceiver<I,S> {
    fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError> { self.header.demands(original,body) }
    fn advance(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> { self.header.advance(original,control,body,grant) }
    fn output_ready(&self)->bool { self.header.output_ready() }
    fn take_demand(&self)->Result<RetirementDemand,ValueError> { nested(self.header.take_demand()?,2*size_of::<ArtifactChild<S>>()) }
    fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(ArtifactChild<S>,RetainedCloneProgress)>,ValueError> {
        let demand=self.take_demand()?;
        if !retained_clone_grant_admits(grant,demand) { return Ok(None); }
        let overhead=2*size_of::<ArtifactChild<S>>();
        let Some((header,progress))=self.header.take(child_grant(grant,overhead))? else { return Ok(None); };
        Ok(Some((ArtifactChild::new(header.child_id,header.target),add_receipt(progress,overhead)?)))
    }
    fn begin_close(&mut self) { self.header.begin_close(); }
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError> { self.header.close_demands(body) }
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> { self.header.close_step(grant) }
    fn terminal_is_empty(&self)->bool { self.header.terminal_is_empty() }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
