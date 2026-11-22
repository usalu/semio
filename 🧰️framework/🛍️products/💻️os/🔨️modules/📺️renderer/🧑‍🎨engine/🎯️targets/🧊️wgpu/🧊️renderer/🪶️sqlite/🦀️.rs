//! 🪶️ Raw Socket text owns one explicit relational entity and its exact UTF-8 carriers.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use super::{NativeSocketProbeSnapshot,store};
use store::{os_store::{serialization::{ArtifactSqliteSnapshotHost,SnapshotReceivingOutcome,SnapshotReceivingOutput},ArtifactNativeSnapshot,NativeSnapshotInput,NativeSnapshotEncoding},sqlite_snapshot::{SqliteStageFailure,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase as Phase,SnapshotEncoding,artifact::{Projection,Cell,reconstruct_text},validate_sqlite_database_schema_controlled},io_schema::IoPayload};
use pack::snapshot::ArtifactPack;
use semio_framework_value::{NativeEncodeControl, ValueError, ValueRefusalKind as Kind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_artifact_reference::{Dialect,StandardId,SubsetId};

pub const SCHEMA:&str="native.socket-grant.probe/v1";
pub const DIALECT:Dialect=Dialect{artifact_kind:"native.socket-grant.probe",standard:StandardId("1"),subset:SubsetId("*")};

fn semantics(length:usize,phase:Phase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 control.checkpoint(phase,0,1)?;control.check_rows(1)?;control.check_value_bytes(length.checked_add(8).ok_or_else(||ValueError::new(Kind::OwnershipLimit,"Socket semantic bytes overflow"))?)
}
fn physical(length:usize,phase:Phase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 semantics(length,phase,control)?;if length>control.limits().max_file_bytes{return Err(ValueError::new(Kind::OwnershipLimit,"Socket native file exceeds caller limit"))}Ok(())
}
fn refused(error:ValueError)->SnapshotReceivingOutcome{semio_framework_value::RetainedWorkOutcome{result:Err(SqliteStageFailure::NotStarted(error)),work:0}}

impl ArtifactNativeSnapshot for NativeSocketProbeSnapshot{
 fn decode_native_snapshot(payload:NativeSnapshotInput<'_>,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
  control.checkpoint()?;let text=match payload{NativeSnapshotInput::Binary(bytes)=>control.borrow_text(bytes)?,NativeSnapshotInput::Text(text)=>text};control.copy_text(text).map(Self)
 }
 fn encode_native_snapshot(&self,encoding:NativeSnapshotEncoding,control:&mut NativeEncodeControl<'_>)->Result<IoPayload,ValueError>{
  match encoding{NativeSnapshotEncoding::Binary=>control.copy_bytes(self.0.as_bytes()).map(IoPayload::Binary),NativeSnapshotEncoding::Text=>control.copy_text(&self.0).map(IoPayload::Text)}
 }
}

impl ArtifactSqliteSnapshotHost for NativeSocketProbeSnapshot{
 fn decode_sqlite_snapshot_native(payload:&IoPayload,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>,output:&mut Option<SnapshotReceivingOutput<Self>>)->SnapshotReceivingOutcome{
  let length=match payload{IoPayload::Binary(bytes)=>bytes.len(),IoPayload::Text(text)=>text.len()};
  if let Err(error)=physical(length,Phase::DecodeNative,control){return refused(error)}
  store::decode_sqlite_snapshot_raw_native(payload,&mut |text,slot,owner|store::native_snapshot_turn(|work|{*slot=Some(Self(work.measured(owner.native(),|native|native.copy_text(text))?));Ok(())}),control,native_owner,output)
 }
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut store::NativeSnapshotEncodeOwner<'_,'_>,output:&mut Option<SnapshotReceivingOutput<IoPayload>>)->SnapshotReceivingOutcome{
  if let Err(error)=self.preflight_sqlite_snapshot_encoding(encoding,control){return refused(error)}
  store::encode_sqlite_snapshot_raw_native(encoding,&mut |_,slot,owner|store::native_snapshot_turn(|work|{*slot=Some(work.measured_encode(owner.native(),|native|native.copy_text(&self.0))?);Ok(())}),control,native_owner,output)
 }
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{physical(self.0.len(),Phase::EncodeNative,control)}
}
impl pack::snapshot::ArtifactSqliteSnapshot for NativeSocketProbeSnapshot {
const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
  semantics(self.0.len(),Phase::ProjectSnapshot,control)?;let mut projection=Projection::new(Self::SQLITE_SCHEMA,control)?;projection.insert_key("socket_probe",1,&[Cell::Text(&self.0)])?;projection.finish()
 }
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  let phase=Phase::ReconstructSnapshot;control.checkpoint(phase,0,1)?;validate_sqlite_database_schema_controlled(database,Self::SQLITE_SCHEMA,phase,control)?;control.check_database(database,phase)?;
  let rows=&database.table("socket_probe")?.rows;if rows.len()!=1||rows[0].rowid!=1||rows[0].values.len()!=2||rows[0].integer(0)?!=1{return Err(ValueError::new(Kind::InvalidValue,"Socket requires one declared text entity"))}
  reconstruct_text(control,rows[0].text(1)?).map(Self)
 }
}
