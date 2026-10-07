//! 🚦️ Controlled typed PNG image records at the physical native boundary.
use crate::schema::snapshot::PngSnapshot;
use crate::store;
use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind};
fn invalid(message: String) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue,message) }
pub(crate) fn decode(payload: &store::io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>) -> Result<PngSnapshot,ValueError> {
 let snapshot=store::decode_sqlite_snapshot_record_native(payload,"stdio.png",PngSnapshot::__dsl_spec_producer(),|record,native| { let snapshot = PngSnapshot::__dsl_from_record_controlled(record,native)?; snapshot.validate().map_err(invalid)?; Ok(snapshot) },control)?;crate::standards::v1_2::subsets::any::io::sqlite::snapshot::admit(&snapshot,SqliteSnapshotPhase::DecodeNative,control)?;Ok(snapshot)
}
pub(crate) fn encode(snapshot: &PngSnapshot, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<store::io_schema::IoPayload,ValueError> {
 crate::standards::v1_2::subsets::any::io::sqlite::snapshot::admit(snapshot,SqliteSnapshotPhase::EncodeNative,control)?;
 store::encode_sqlite_snapshot_record_native(encoding,"stdio.png",PngSnapshot::__dsl_spec_producer(),|native|snapshot.__dsl_to_record_controlled(native),control)
}
pub(crate) fn preflight(snapshot: &PngSnapshot, _encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(),ValueError> { crate::standards::v1_2::subsets::any::io::sqlite::snapshot::admit(snapshot,SqliteSnapshotPhase::EncodeNative,control) }
