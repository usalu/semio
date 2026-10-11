//! 🌊️ Framework Flow relational and native capability for its actual persisted owner.
use super::*;
use crate::os_store::ArtifactSqliteSnapshot;
use crate::store::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SnapshotEncoding,ValueError};
impl ArtifactSqliteSnapshot for FlowHostSnapshot{
 fn to_sqlite_database_receiving(&self,control:&mut SqliteSnapshotControl<'_>,owner:&mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_,'_>)->Result<SqliteDatabase,ValueError>{owner.native().checkpoint()?;self.to_sqlite_database(control)}
 fn from_sqlite_database_receiving(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_,'_>)->Result<Self,ValueError>{owner.native().checkpoint()?;Self::from_sqlite_database(database,control)}
 fn validate_sqlite_snapshot_subset_decoding(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_,'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{owner.native().checkpoint().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)?;self.validate_sqlite_snapshot_subset(dialect,database,control)}
 fn validate_sqlite_snapshot_subset_encoding(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_,'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{owner.native().checkpoint().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)?;self.validate_sqlite_snapshot_subset(dialect,database,control)}
 const SQLITE_SCHEMA:&'static str=include_str!("../../🗄️.sql");
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{flow_sql_projection::project(self,control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{flow_sql_reconstruction::reconstruct(database,control)}
 fn decode_sqlite_snapshot_native(payload:&crate::store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>{flow_native_carrier::decode(payload,control,native_owner)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut crate::os_store::NativeSnapshotEncodeOwner<'_,'_>)->Result<crate::store::io_schema::IoPayload,ValueError>{flow_native_carrier::encode(self,encoding,control,native_owner)}
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{flow_native_bound::preflight(self,control)}
 fn retire_sqlite_snapshot(self){self.retire_cold();}
}
