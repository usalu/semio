//! 🛬️ Rewriting preserves its original declared envelope and record inside the actual receiving frame.
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;
use semio_os_kernel::{
    NativeSnapshotDecodeOwner,
    os_store::serialization::{SnapshotReceivingOutcome, SnapshotReceivingOutput},
    sqlite_snapshot::SqliteSnapshotControl,
};

pub(super) fn decode(
    payload: &store::io::IoPayload,
    control: &mut SqliteSnapshotControl<'_>,
    native_owner: &mut NativeSnapshotDecodeOwner<'_, '_>,
    output: &mut Option<SnapshotReceivingOutput<RewritingSnapshot>>,
) -> SnapshotReceivingOutcome {
    store::decode_sqlite_snapshot_record_native(
        payload,
        "trinity.rewriting",
        RewritingSnapshot::__dsl_spec_producer(),
        &mut |record, snapshot_output, native| {
            *snapshot_output = Some(RewritingSnapshot::__dsl_from_record_controlled(record, native)?);
            Ok(())
        },
        control,
        native_owner,
        output,
    )
}
