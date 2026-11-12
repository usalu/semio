//! 🫴️ IFC Pack ingress keeps the literal frame in the caller's original receiving owner until the typed snapshot is published.
use super::{Frame, IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA, binding, reconstruct};
use semio_os_kernel::{ArtifactPackReceiving, NativeSnapshotDecodeOwner, NativeSnapshotFailure};
use semio_framework_value::{RetainedWorkOutcome, ValueError, ValueRefusalKind};
fn refusal(error: store::PackError) -> ValueError {
    error.into_value_error().unwrap_or_else(|_| ValueError::literal(ValueRefusalKind::InvariantViolated, "IFC Pack receiving lost its transport"))
}
impl ArtifactPackReceiving for IfcSnapshot {
    fn receive_pack(bytes: &[u8], output: &mut Option<Self>, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> RetainedWorkOutcome<(), NativeSnapshotFailure> {
        owner.receive::<Frame, Self, NativeSnapshotFailure>(output, &mut |slot, published, _marker, _unwind, native, _body| {
            let before = native.actual_work();
            let result = (|| -> Result<(), ValueError> {
                native.checkpoint()?;
                let maximum = store::sqlite_snapshot::SqliteDatabaseLimits::default().max_rows;
                let spec = Frame::__dsl_spec_producer().decode(native)?;
                let body = store::semio_format::unwrap_binary_controlled(bytes, STDIO_IFC_DOCUMENT_SCHEMA, store::semio_format::Component::Pack, 1, native).map_err(store::PackError::from).map_err(refusal)?;
                let (record, _) = store::pack_rt::decode_document_controlled(body, &spec, &store::PackDecodeOptions::default(), native).map_err(store::PackRefusal::into_value_error)?;
                *slot = Some(binding::frame(&record, native, maximum)?);
                let frame = slot.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "IFC receiving lost its literal frame"))?;
                *published = Some(reconstruct(frame, native)?);
                Ok(())
            })()
            .map_err(NativeSnapshotFailure::from);
            RetainedWorkOutcome { result, work: native.actual_work().saturating_sub(before) }
        })
    }
}
