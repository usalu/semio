//! 🚦️ Controlled typed PNG image records at the physical native boundary.
use crate::schema::snapshot::PngSnapshot;
use crate::store;
use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind};
fn invalid(message: String) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue,message) }
pub(crate) fn decode(payload: &store::io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>) -> Result<PngSnapshot,ValueError> {
 let snapshot=store::decode_sqlite_snapshot_record_native(payload,"stdio.png",PngSnapshot::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| { let snapshot = PngSnapshot::__dsl_from_record_controlled(record,native)?; snapshot.validate().map_err(invalid)?; Ok(snapshot) })(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)?;crate::standards::v1_2::subsets::any::io::sqlite::snapshot::admit(&snapshot,SqliteSnapshotPhase::DecodeNative,control)?;Ok(snapshot)
}
pub(crate) fn encode(snapshot: &PngSnapshot, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>) -> Result<store::io_schema::IoPayload,ValueError> {
 crate::standards::v1_2::subsets::any::io::sqlite::snapshot::admit(snapshot,SqliteSnapshotPhase::EncodeNative,control)?;
 store::encode_sqlite_snapshot_record_native(encoding,"stdio.png",PngSnapshot::__dsl_spec_producer(),|native|snapshot.__dsl_to_record_controlled(native),control,native_owner)
}
pub(crate) fn preflight(snapshot: &PngSnapshot, _encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(),ValueError> { crate::standards::v1_2::subsets::any::io::sqlite::snapshot::admit(snapshot,SqliteSnapshotPhase::EncodeNative,control) }
