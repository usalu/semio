//! 🌊️ Framework Flow relational and native capability for its actual persisted owner.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use super::*;
use crate::os_store::serialization::{ArtifactSqliteSnapshotHost,SnapshotReceivingOutcome,SnapshotReceivingOutput};
use crate::store::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SnapshotEncoding,ValueError};
impl ArtifactSqliteSnapshotHost for FlowHostSnapshot{
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>,output:&mut Option<SnapshotReceivingOutput<Self>>)->SnapshotReceivingOutcome {flow_native_carrier::decode(payload,control,native_owner,output)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut crate::os_store::NativeSnapshotEncodeOwner<'_,'_>,output:&mut Option<SnapshotReceivingOutput<store::io_schema::IoPayload>>)->SnapshotReceivingOutcome {flow_native_carrier::encode(self,encoding,control,native_owner,output)}
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{flow_native_bound::preflight(self,control)}
 fn retire_sqlite_snapshot(self){self.retire_cold();}
}
impl pack::snapshot::ArtifactSqliteSnapshot for FlowHostSnapshot {
const SQLITE_SCHEMA:&'static str=include_str!("../../🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{flow_sql_projection::project(self,control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{flow_sql_reconstruction::reconstruct(database,control)}
}