//! 🪶️ Authored semantic SQLite and actual controlled native owner capability.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::standards::v1_2::subsets::any::schema::snapshot::PngSnapshot;
use semio_os_kernel::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SnapshotEncoding,SqliteSnapshotPhase,ValueError}};
#[path="💰️backing/🦀️.rs"] mod backing;
pub fn admit(snapshot:&PngSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{backing::admit(snapshot,phase,control)}
impl ArtifactSqliteSnapshotHost for PngSnapshot{
 
 
 
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_os_kernel::NativeSnapshotDecodeOwner<'_, '_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<Self>>)->store::os_store::serialization::SnapshotReceivingOutcome {crate::standards::v1_2::subsets::any::io::binary::snapshot::native::decode(payload,control,native_control,output)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_os_kernel::NativeSnapshotEncodeOwner<'_, '_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<store::io_schema::IoPayload>>)->store::os_store::serialization::SnapshotReceivingOutcome {crate::standards::v1_2::subsets::any::io::binary::snapshot::native::encode(self,encoding,control,native_owner,output)}
 fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{crate::standards::v1_2::subsets::any::io::binary::snapshot::native::preflight(self,encoding,control)}
 fn retire_sqlite_snapshot(self){<Self as semio_framework_value::FromValue>::retire_decoded(self)}
}
impl pack::snapshot::ArtifactSqliteSnapshot for PngSnapshot {
const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{backing::project(self,control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{backing::reconstruct(database,control)}
}


