//! 🔌 Natural STEP export retains the original IFC source and UTF-8 publication authority.
use super::{write_ifc4x3_step_pack, IfcSnapshot, TXT_DIALECT};
use semio_framework_os_kernel::io::{io_mechanism::{ArchiveChildren, IoRunControl, Serializer}, io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult}};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ValueError, ValueRefusalKind};
use std::mem::size_of;

/// 📤 A lossless admitted graph is encoded directly to the actual native UTF-8 artifact.
pub struct Ifc4x3IntoTxtNative;
impl Serializer<IfcSnapshot> for Ifc4x3IntoTxtNative {
    const INTO: semio_framework_artifact_reference::Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossless;
    async fn serialize(source: &IfcSnapshot, _: &ArchiveChildren, control: &mut IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        control.receive_nested::<Vec<u8>, IoOutcome<IoPayload>>(|slot, original| {
            *slot = Some(write_ifc4x3_step_pack(source, &mut original.snapshot_encode()?)?);
            let mut owner = original.snapshot_encode()?;
            let bytes = size_of::<IoOutcome<IoPayload>>();
            let demand = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: bytes, maximum_depth: 1, ..Default::default() };
            let remaining = owner.remaining_grant();
            if demand.maximum_items > remaining.maximum_items || demand.maximum_copy_bytes > remaining.maximum_copy_bytes || demand.maximum_depth > remaining.maximum_depth { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "STEP final outcome exceeds original grant")); }
            owner.native().checkpoint()?;
            owner.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
            Ok(IoOutcome { value: IoPayload::Binary(slot.take().unwrap()), diagnostics: Vec::new() })
        }).map_err(IoError::from_value_error)
    }
}
