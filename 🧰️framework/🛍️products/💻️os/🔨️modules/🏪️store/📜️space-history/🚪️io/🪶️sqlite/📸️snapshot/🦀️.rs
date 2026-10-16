//! 📜️ Literal space checkpoint, clock, author, member and alternative entities.
#[path="📏️preflight/🦀️.rs"]mod preflight;
#[path="🛂️validation/🦀️.rs"]mod validation;
#[path="🫴️receiving/🦀️.rs"]mod receiving;
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_os_spr::HybridLogicalTimestamp;
use crate::lifted_serialization::ArtifactSqliteSnapshotHost;
use semio_framework_os_store_space_schema::space_history_document::{S_SPACE_HISTORY_SCHEMA, SpaceAlternative, SpaceCheckpoint, SpaceHistorySnapshot, SpaceMemberPin};
use semio_framework_os_vcs::Author;
use semio_framework_io_sqlite_snapshot::{SnapshotEncoding, SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase, validate_sqlite_database_schema};
use semio_framework_io_sqlite_snapshot::artifact::{Cell, RowWriter};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_os_space_sqlite_cells as fields;
#[path = "🚦️native/🦀️.rs"]
mod native;
#[path = "⚖️settlement/🦀️.rs"]
pub mod settlement;
pub use native::decode_with as decode_native_cst;
#[path = "🛂️admission/🦀️.rs"]
pub mod admission;
/// 🚪️ The new explicitly authored SQLite coordinate of the real persisted history owner.
pub const SQLITE_SNAPSHOT_DIALECT: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: S_SPACE_HISTORY_SCHEMA, standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId("*") };
pub use semio_framework_os_store_space_schema::lifted_snapshot::{pair, rows, schema, word, write_rows};





fn semantic(value:&SpaceHistorySnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 schema(control)?;let total=rows(value,control,phase)?;let mut writer=RowWriter::borrowed(control,phase)?;write_rows(value,total,&mut writer)?;writer.finish_borrowed()
}
impl ArtifactSqliteSnapshotHost for SpaceHistorySnapshot {
    fn to_sqlite_database_receiving(&self,control:&mut SqliteSnapshotControl<'_>,owner:&mut semio_framework_io_control::NativeSnapshotDecodeOwner<'_,'_>)->Result<SqliteDatabase,ValueError>{receiving::project_receiving(Some(self),None,control,owner)}
    fn from_sqlite_database_receiving(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut semio_framework_io_control::NativeSnapshotEncodeOwner<'_,'_>)->Result<Self,ValueError>{receiving::reconstruct_receiving(None,Some(database),control,owner)}
    
    fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{preflight::check(self,encoding,control)}
    fn encode_sqlite_snapshot_native(&self, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_io_control::NativeSnapshotEncodeOwner<'_, '_>) -> Result<semio_framework_io_schema::IoPayload, ValueError> {
        semantic(self,SqliteSnapshotPhase::EncodeNative,control)?;
        native::encode(self, encoding, control, native_owner)
    }
    fn decode_sqlite_snapshot_native(payload: &semio_framework_io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_io_control::NativeSnapshotDecodeOwner<'_,'_>) -> Result<Self, ValueError> {
        schema(control)?;
        native::decode(payload, control, native_owner)
    }
    
    
    fn validate_sqlite_snapshot_subset(&self,_dialect:&semio_framework_artifact_reference::ArtifactDialect,_database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_io_schema::IoResult<()>{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0).map_err(semio_framework_io_schema::IoError::from_value_error)?;Err(semio_framework_io_schema::IoError::from_value_error(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"space history subset validation requires supplied directional owner")))}
    fn validate_sqlite_snapshot_subset_decoding(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,native:&mut semio_framework_io_control::NativeSnapshotDecodeOwner<'_,'_>)->semio_framework_io_schema::IoResult<()>{validation::decoding(self,dialect,database,control,native)}
    fn validate_sqlite_snapshot_subset_encoding(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,native:&mut semio_framework_io_control::NativeSnapshotEncodeOwner<'_,'_>)->semio_framework_io_schema::IoResult<()>{validation::encoding(self,dialect,database,control,native)}
}

