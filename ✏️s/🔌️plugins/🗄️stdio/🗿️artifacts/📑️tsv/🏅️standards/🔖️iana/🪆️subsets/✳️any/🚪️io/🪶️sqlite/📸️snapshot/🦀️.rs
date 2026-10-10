//! 🪶️ Authored semantic SQLite and actual controlled native owner capability.
use crate::standards::iana::subsets::any::schema::snapshot::{TsvSnapshot,LineEnding};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SnapshotEncoding,ValueError}};
#[path="💰️backing/🦀️.rs"] mod backing;
#[path="🫴️receiving/🦀️.rs"]mod receiving;
pub(super) fn semantic(snapshot:&TsvSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase)->Result<usize,ValueError>{backing::semantic(snapshot,control,phase)}
#[path="🛂️admission/🦀️.rs"]pub(super)mod admission;
impl ArtifactSqliteSnapshot for TsvSnapshot{
fn validate_sqlite_snapshot_subset_decoding(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,_database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,native:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->store::io_schema::IoResult<()>{receiving::validate_subset(dialect,control,||native.native().checkpoint())}
 fn validate_sqlite_snapshot_subset_encoding(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,_database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,native:&mut store::NativeSnapshotEncodeOwner<'_,'_>)->store::io_schema::IoResult<()>{receiving::validate_subset(dialect,control,||native.native().checkpoint())}
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{backing::project(self,control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{backing::reconstruct(database,control)}
 fn to_sqlite_database_receiving(&self,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->Result<SqliteDatabase,ValueError>{receiving::project_receiving(self,control,owner)}
 fn from_sqlite_database_receiving(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotEncodeOwner<'_,'_>)->Result<Self,ValueError>{receiving::reconstruct_receiving(database,control,owner)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{crate::standards::iana::subsets::any::io::sqlite::snapshot::native::decode(payload,control,native_control)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{crate::standards::iana::subsets::any::io::sqlite::snapshot::native::encode(self,encoding,control,native_owner)}
 fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{crate::standards::iana::subsets::any::io::sqlite::snapshot::native::preflight(self,encoding,control)}
 fn retire_sqlite_snapshot(self){<Self as semio_framework_value::FromValue>::retire_decoded(self)}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;


#[path = "🚦️native/🦀️.rs"]
pub(crate) mod native;
