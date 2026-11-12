//! 🚦️ Controlled typed BMP image records at the physical native boundary.
use crate::schema::snapshot::BmpSnapshot;
use crate::store;
use semio_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind};
fn invalid(message: String) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue,message) }
pub(crate) fn decode(payload: &store::io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>,native_control: &mut semio_os_kernel::NativeSnapshotDecodeOwner<'_, '_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<BmpSnapshot>>)->store::os_store::serialization::SnapshotReceivingOutcome {
 store::decode_sqlite_snapshot_record_receiving(payload,"stdio.bmp",BmpSnapshot::__dsl_spec_producer(),&mut |record,snapshot_output,owner,control|semio_os_kernel::native_snapshot_turn(|work|work.measured(owner.native(),|native|{
  let snapshot=BmpSnapshot::__dsl_from_record_controlled(record,native)?;snapshot.validate().map_err(invalid)?;
  crate::standards::v_v3::subsets::any::io::sqlite::snapshot::admit(&snapshot,SqliteSnapshotPhase::DecodeNative,control)?;
  *snapshot_output=Some(snapshot);Ok::<(),ValueError>(())
 })),control,native_control,output)
}
pub(crate) fn encode(snapshot: &BmpSnapshot, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>,native_owner:&mut semio_os_kernel::NativeSnapshotEncodeOwner<'_, '_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<store::io_schema::IoPayload>>)->store::os_store::serialization::SnapshotReceivingOutcome {
 if let Err(error)=crate::standards::v_v3::subsets::any::io::sqlite::snapshot::admit(snapshot,SqliteSnapshotPhase::EncodeNative,control){return semio_framework_value::RetainedWorkOutcome{result:Err(store::sqlite_snapshot::SqliteStageFailure::NotStarted(error)),work:0}}
 store::encode_sqlite_snapshot_record_native(encoding,"stdio.bmp",BmpSnapshot::__dsl_spec_producer(),&mut |native|snapshot.__dsl_to_record_controlled(native),control,native_owner,output)
}
pub(crate) fn preflight(snapshot: &BmpSnapshot, _encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(),ValueError> { crate::standards::v_v3::subsets::any::io::sqlite::snapshot::admit(snapshot,SqliteSnapshotPhase::EncodeNative,control) }
