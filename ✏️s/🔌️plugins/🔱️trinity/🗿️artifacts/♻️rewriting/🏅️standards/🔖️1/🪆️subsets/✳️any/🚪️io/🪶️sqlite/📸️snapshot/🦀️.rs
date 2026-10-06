//! ♻️ typed thirty-table Rewriting capability awaiting its owning assertion gate.
use crate::RewritingSnapshot;
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SnapshotEncoding}};
use semio_framework_value::ValueError;
#[path="💰️backing/🦀️.rs"]
mod backing;
#[path="📏️encoding/🦀️.rs"]
mod encoding;
#[path="🚦️native/🦀️.rs"]
mod native;
impl ArtifactSqliteSnapshot for RewritingSnapshot {
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{backing::project(self,control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{backing::reconstruct(database,control)}
 fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  native::decode(payload,control)
 }
 fn encode_sqlite_snapshot_native(&self,format:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{
  encoding::preflight(self,control)?;
  store::encode_sqlite_snapshot_record_native(format,"trinity.rewriting",Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)
 }
 fn preflight_sqlite_snapshot_encoding(&self,_format:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{encoding::preflight(self,control)}
 fn retire_sqlite_snapshot(self){<Self as semio_framework_dsl_record::DslField>::retire_decoded(self)}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

