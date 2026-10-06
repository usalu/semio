//! 🪶️ Authored semantic SQLite and actual controlled native owner capability.
use crate::standards::v_raw::subsets::any::schema::snapshot::{BinarySnapshot};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SnapshotEncoding,ValueError}};
#[path="💰️backing/🦀️.rs"] mod backing;
pub(crate)fn semantic(snapshot:&BinarySnapshot,control:&mut SqliteSnapshotControl<'_>,phase:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase)->Result<(),ValueError>{backing::semantic(snapshot,control,phase)}
pub(crate)fn native_cells(schema:&str,count:usize,bytes:impl IntoIterator<Item=Result<u8,ValueError>>,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{backing::native_cells(schema,count,bytes,control)}
impl ArtifactSqliteSnapshot for BinarySnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{backing::project(self,control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{backing::reconstruct(database,control)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v_raw::subsets::any::io::sqlite::snapshot::native::decode(payload,control)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{crate::standards::v_raw::subsets::any::io::sqlite::snapshot::native::encode(self,encoding,control)}
 fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{crate::standards::v_raw::subsets::any::io::sqlite::snapshot::native::preflight(self,encoding,control)}
 fn retire_sqlite_snapshot(self){<Self as semio_framework_value::FromValue>::retire_decoded(self)}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;


#[path = "🚦️native/🦀️.rs"]
pub(crate) mod native;
