//! 📷️ Actual PNG literal SQL and paid native capability over the authored semantic owner.
use crate::standards::v1_2::subsets::any::schema::snapshot::*;
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SnapshotEncoding,ValueError}};
#[path="💰️backing/🦀️.rs"] mod backing;
pub(super) fn admit(snapshot:&PngSnapshot,phase:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{backing::admit(snapshot,phase,control)}
pub(super) fn preflight(snapshot:&PngSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{backing::preflight(snapshot,control)}
impl ArtifactSqliteSnapshot for PngSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{backing::project(self,control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{backing::reconstruct(database,control)}
 fn decode_sqlite_snapshot_native(payload:&crate::store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v1_2::subsets::any::io::sqlite::snapshot::native::decode(payload,control)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<crate::store::io_schema::IoPayload,ValueError>{crate::standards::v1_2::subsets::any::io::sqlite::snapshot::native::encode(self,encoding,control)}
 fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{crate::standards::v1_2::subsets::any::io::sqlite::snapshot::native::preflight(self,encoding,control)}
 fn retire_sqlite_snapshot(self){<Self as semio_framework_value::FromValue>::retire_decoded(self)}
}

pub(super) fn admit_record(record:&semio_framework_dsl_record::RecordValue,limits:semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{backing::admit_record(record,limits,native)}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;


#[path = "🚦️native/🦀️.rs"]
pub(crate) mod native;
