//! 🪶️ Authored semantic SQLite and actual controlled native owner capability.
use crate::standards::v_raw::subsets::any::schema::snapshot::{BinarySnapshot};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SnapshotEncoding,ValueError}};
#[path="💰️backing/🦀️.rs"] mod backing;
pub(crate)fn semantic(snapshot:&BinarySnapshot,control:&mut SqliteSnapshotControl<'_>,phase:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase)->Result<(),ValueError>{backing::semantic(snapshot,control,phase)}
pub(crate)fn native_cells(schema:&str,count:usize,bytes:impl IntoIterator<Item=Result<u8,ValueError>>,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{backing::native_cells(schema,count,bytes,control)}
impl ArtifactSqliteSnapshot for BinarySnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn to_sqlite_database_receiving(&self,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->Result<SqliteDatabase,ValueError>{receiving::project_receiving(self,control,owner)}
 fn from_sqlite_database_receiving(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotEncodeOwner<'_,'_>)->Result<Self,ValueError>{receiving::reconstruct_receiving(database,control,owner)}
 fn validate_sqlite_snapshot_subset_decoding(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,_database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,native:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->store::io_schema::IoResult<()>{receiving::validate_subset(dialect,control,||native.native().checkpoint())}
 fn validate_sqlite_snapshot_subset_encoding(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,_database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,native:&mut store::NativeSnapshotEncodeOwner<'_,'_>)->store::io_schema::IoResult<()>{receiving::validate_subset(dialect,control,||native.native().checkpoint())}

 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{backing::project(self,control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{backing::reconstruct(database,control)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{crate::standards::v_raw::subsets::any::io::sqlite::snapshot::native::decode(payload,control,native_owner)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{crate::standards::v_raw::subsets::any::io::sqlite::snapshot::native::encode(self,encoding,control,native_owner)}
 fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{crate::standards::v_raw::subsets::any::io::sqlite::snapshot::native::preflight(self,encoding,control)}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;


#[path = "🚦️native/🦀️.rs"]
pub(crate) mod native;

#[path="🫴️receiving/🦀️.rs"]
mod receiving;
/// 🫴️ Shares the original paid raw carrier binder with the required pack receiver.
pub(crate)fn bind_raw_pack(bytes:&[u8],slot:&mut Option<BinarySnapshot>,native:&mut semio_framework_value::NativeDecodeControl<'_>,body:&mut store::NativeSnapshotBodyWallet)->Result<(),ValueError>{
 receiving::bind(receiving::Carrier::Raw(bytes),crate::STDIO_BINARY_DOCUMENT_SCHEMA,slot,native,body)
}

