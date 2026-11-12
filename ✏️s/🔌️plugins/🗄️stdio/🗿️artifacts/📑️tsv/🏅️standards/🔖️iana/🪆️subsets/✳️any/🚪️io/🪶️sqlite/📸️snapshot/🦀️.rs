//! 🪶️ Authored semantic SQLite and actual controlled native owner capability.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::standards::iana::subsets::any::schema::snapshot::{TsvSnapshot,LineEnding};
use semio_os_kernel::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SnapshotEncoding,ValueError}};
#[path="💰️backing/🦀️.rs"] mod backing;
pub(super) fn semantic(snapshot:&TsvSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:semio_os_kernel::sqlite_snapshot::SqliteSnapshotPhase)->Result<usize,ValueError>{backing::semantic(snapshot,control,phase)}
#[path="🛂️admission/🦀️.rs"]pub mod admission;
impl ArtifactSqliteSnapshotHost for TsvSnapshot{
fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,_database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{
  control.checkpoint(semio_os_kernel::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot,0,0).map_err(store::io_schema::IoError::from_value_error)?;
  if dialect.subset=="*"{return Ok(store::io_schema::IoOutcome::clean(()))}
  Err(store::io_schema::IoError::from_value_error(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"TSV subset has no authored semantic validator")))
 }
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut semio_os_kernel::NativeSnapshotDecodeOwner<'_, '_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<Self>>)->store::os_store::serialization::SnapshotReceivingOutcome{native::decode(payload,control,native_control,output)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_os_kernel::NativeSnapshotEncodeOwner<'_, '_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<store::io_schema::IoPayload>>)->store::os_store::serialization::SnapshotReceivingOutcome{native::encode(self,encoding,control,native_owner,output)}
 fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{native::preflight(self,encoding,control)}
 fn retire_sqlite_snapshot(self){<Self as semio_framework_value::FromValue>::retire_decoded(self)}
}
impl pack::snapshot::ArtifactSqliteSnapshot for TsvSnapshot {
const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{backing::project(self,control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{backing::reconstruct(database,control)}
}



#[path = "🚦️native/🦀️.rs"]
pub(crate) mod native;
