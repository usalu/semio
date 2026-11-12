//! 🫴️ Normal IFC route entries bind the actual original Pack receiver and final Pack publication owner.
use super::*;
use semio_os_kernel::{ArtifactPackReceiving, io::io_mechanism::IoEntryDirection};
use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
use std::mem::size_of;
fn invalid(message: &'static str) -> IoError {
    IoError::from_value_error(ValueError::literal(ValueRefusalKind::InvalidValue, message))
}
fn publish_header(control: &mut IoRunControl<'_, '_>) -> Result<(), ValueError> {
    let mut owner = control.snapshot_encode()?;
    let grant = owner.remaining_grant();
    let bytes = size_of::<IoOutcome<IoPayload>>();
    if grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth == 0 {
        return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "original IFC route final outcome exceeds caller grant"));
    }
    owner.native().checkpoint()?;
    owner.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })
}
/// 📤 Direct native IFC Packs enter through their source-owned original parser and close before route publication.
pub fn serializer<T: Serializer<IfcSnapshot>>() -> IoEntry {
    fn run<T: Serializer<IfcSnapshot>>(input: &IoPayload, control: &mut IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let IoPayload::Binary(bytes) = input else {
            return Err(invalid("IFC route requires its declared binary native Pack"));
        };
        nested::<IfcSnapshot, IoOutcome<IoPayload>>(control, |slot, original, work| {
            work.take(<IfcSnapshot as ArtifactPackReceiving>::receive_pack(bytes, slot, &mut original.snapshot_decode().map_err(IoError::from_value_error)?)).map_err(refused)?;
            semio_framework_async::poll::resolve_ready(T::serialize(slot.as_ref().unwrap(), &ArchiveChildren::empty(), original))
        })
    }
    IoEntry { owned_serializer: T::OWNED_FACTORY, from: crate::io::IFC4X3_DIALECT, into: T::INTO, fidelity: T::FIDELITY, direction: IoEntryDirection::Export, sniff: None, run: run::<T> }
}
/// 📥 Foreign imports retain the actual IFC graph and diagnostics until the sole final native Pack is encoded and its scratch closes.
pub fn deserializer<T: Deserializer<IfcSnapshot>>() -> IoEntry {
    fn sniff<T: Deserializer<IfcSnapshot>>(input: &IoPayload) -> semio_os_kernel::io::io_schema::Confidence {
        semio_framework_async::poll::resolve_ready(T::sniff(input))
    }
    fn run<T: Deserializer<IfcSnapshot>>(input: &IoPayload, control: &mut IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        if T::CONFORMANCE.is_some() {
            return Err(IoError::from_value_error(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original IFC route requires a separately funded conformance producer")));
        }
        nested::<IoOutcome<IfcSnapshot>, IoOutcome<IoPayload>>(control, |slot, original, _| {
            *slot = Some(semio_framework_async::poll::resolve_ready(T::deserialize(input, original))?);
            nested::<Vec<u8>, IoOutcome<IoPayload>>(original, |wire, original, work| {
                work.take(slot.as_ref().unwrap().value.encode_exchange_pack(wire, &mut original.snapshot_encode().map_err(IoError::from_value_error)?)).map_err(refused)?;
                publish_header(original).map_err(IoError::from_value_error)?;
                Ok(IoOutcome { value: IoPayload::Binary(wire.take().unwrap()), diagnostics: std::mem::take(&mut slot.as_mut().unwrap().diagnostics) })
            })
        })
    }
    IoEntry { owned_serializer: None, from: T::FROM, into: crate::io::IFC4X3_DIALECT, fidelity: T::FIDELITY, direction: IoEntryDirection::Import, sniff: Some(sniff::<T>), run: run::<T> }
}
