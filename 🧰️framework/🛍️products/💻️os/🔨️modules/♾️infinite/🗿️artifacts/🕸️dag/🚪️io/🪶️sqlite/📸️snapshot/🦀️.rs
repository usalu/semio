//! 🕸️ Actual persisted DAG semantic provider and literal controlled native authority.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::DagSnapshot;
use semio_framework_value::ValueError;
use store::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SnapshotEncoding};
use store::os_store::serialization::{SnapshotReceivingOutcome, SnapshotReceivingOutput};
#[path="./🛫️projection/🦀️.rs"]
mod projection;
#[path="./🛬️reconstruction/🦀️.rs"]
mod reconstruction;
#[path="./🔍️rows/🦀️.rs"]
mod rows;
#[path="./🪆️values/🦀️.rs"]
mod values;
#[path="./📏️preflight/🦀️.rs"]
mod preflight;
impl store::os_store::serialization::ArtifactSqliteSnapshotHost for DagSnapshot {
    fn decode_sqlite_snapshot_native(
        payload: &store::io_schema::IoPayload,
        control: &mut SqliteSnapshotControl<'_>,
        native: &mut store::NativeSnapshotDecodeOwner<'_, '_>,
        output: &mut Option<SnapshotReceivingOutput<Self>>,
    ) -> SnapshotReceivingOutcome {
        store::decode_sqlite_snapshot_record_native(
            payload,
            <Self as pack::snapshot::ArtifactDsl>::envelope_id(),
            Self::__dsl_spec_producer(),
            &mut |record, snapshot_output, native| {
                *snapshot_output = Some(Self::__dsl_from_record_controlled(record, native)?);
                Ok(())
            },
            control,
            native,
            output,
        )
    }

    fn encode_sqlite_snapshot_native(
        &self,
        encoding: SnapshotEncoding,
        control: &mut SqliteSnapshotControl<'_>,
        native: &mut store::NativeSnapshotEncodeOwner<'_, '_>,
        output: &mut Option<SnapshotReceivingOutput<store::io_schema::IoPayload>>,
    ) -> SnapshotReceivingOutcome {
        let before = control.performed_work();
        let checked = preflight::preflight(self, encoding, control);
        let preflight_work = u128::from(control.performed_work() - before);
        if let Err(error) = checked {
            return semio_framework_value::RetainedWorkOutcome {
                result: Err(store::sqlite_snapshot::SqliteStageFailure::NotStarted(error)),
                work: preflight_work,
            };
        }
        let mut outcome = store::encode_sqlite_snapshot_record_native(
            encoding,
            <Self as pack::snapshot::ArtifactDsl>::envelope_id(),
            Self::__dsl_spec_producer(),
            &mut |native| self.__dsl_to_record_controlled(native),
            control,
            native,
            output,
        );
        outcome.work += preflight_work;
        outcome
    }

    fn preflight_sqlite_snapshot_encoding(&self, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        preflight::preflight(self, encoding, control)
    }

    fn retire_sqlite_snapshot(self) {
        <Self as semio_framework_dsl_record::DslField>::retire_decoded(self)
    }
}
impl pack::snapshot::ArtifactSqliteSnapshot for DagSnapshot {
const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{projection::project(self,control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{reconstruction::reconstruct(database,control)}
}
