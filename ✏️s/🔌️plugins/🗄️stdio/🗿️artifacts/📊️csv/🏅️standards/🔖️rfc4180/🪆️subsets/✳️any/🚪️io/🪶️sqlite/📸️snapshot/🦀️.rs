//! 🪶️ Authored semantic SQLite and actual controlled native owner capability.
use crate::standards::v_rfc4180::subsets::any::schema::snapshot::{CsvSnapshot,CsvRecord,CsvField};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SnapshotEncoding,ValueError}};
#[path="💰️backing/🦀️.rs"] mod backing;
pub(super) fn semantic(snapshot:&CsvSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase)->Result<usize,ValueError>{backing::semantic(snapshot,control,phase)}
#[path="🛂️admission/🦀️.rs"]pub(super)mod admission;
impl ArtifactSqliteSnapshot for CsvSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{backing::project(self,control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{backing::reconstruct(database,control)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v_rfc4180::subsets::any::io::sqlite::snapshot::native::decode(payload,control)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{crate::standards::v_rfc4180::subsets::any::io::sqlite::snapshot::native::encode(self,encoding,control)}
 fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{crate::standards::v_rfc4180::subsets::any::io::sqlite::snapshot::native::preflight(self,encoding,control)}
 fn retire_sqlite_snapshot(self){<Self as semio_framework_value::FromValue>::retire_decoded(self)}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;


#[path = "🚦️native/🦀️.rs"]
pub(crate) mod native;
