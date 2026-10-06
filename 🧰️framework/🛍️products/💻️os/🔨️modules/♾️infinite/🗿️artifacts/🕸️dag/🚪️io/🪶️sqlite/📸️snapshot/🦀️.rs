//! 🕸️ Actual persisted DAG semantic provider and literal controlled native authority.
use crate::DagSnapshot;
use semio_framework_value::ValueError;
use store::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SnapshotEncoding};
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
impl store::ArtifactSqliteSnapshot for DagSnapshot {
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{projection::project(self,control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{reconstruction::reconstruct(database,control)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|Self::__dsl_from_record_controlled(record,native),control)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{preflight::preflight(self,encoding,control)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)}
 fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{preflight::preflight(self,encoding,control)}
 fn retire_sqlite_snapshot(self){<Self as semio_framework_dsl_record::DslField>::retire_decoded(self)}
}

