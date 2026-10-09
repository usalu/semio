//! 🛬️ Rewriting preserves its original declared envelope and record inside the actual receiving frame.
use crate::RewritingSnapshot;
use semio_framework_os_kernel::{NativeSnapshotDecodeOwner,sqlite_snapshot::SqliteSnapshotControl};
use semio_framework_value::ValueError;
pub(super) fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut NativeSnapshotDecodeOwner<'_, '_>)->Result<RewritingSnapshot,ValueError>{
 store::decode_sqlite_snapshot_record_native(payload,"trinity.rewriting",RewritingSnapshot::__dsl_spec_producer(),|record,output,native|{*output=Some(RewritingSnapshot::__dsl_from_record_controlled(record,native)?);Ok(())},control,native_owner)
}
