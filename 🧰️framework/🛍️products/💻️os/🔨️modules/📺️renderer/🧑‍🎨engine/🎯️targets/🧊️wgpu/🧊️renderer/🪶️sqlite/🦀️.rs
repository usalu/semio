//! 🪶️ Raw Socket text owns one explicit relational entity and its exact UTF-8 carriers.
use super::{NativeSocketProbeSnapshot,store};
use store::{os_store::{ArtifactSqliteSnapshot,ArtifactNativeSnapshot,NativeSnapshotInput,NativeSnapshotEncoding,ArtifactPack},sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase as Phase,SnapshotEncoding,artifact::{Projection,Cell,reconstruct_text},validate_sqlite_database_schema_controlled},io_schema::IoPayload};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind as Kind};
use semio_framework_artifact_reference::{Dialect,StandardId,SubsetId};

pub(super) const SCHEMA:&str="native.socket-grant.probe/v1";
pub(super) const DIALECT:Dialect=Dialect{artifact_kind:"native.socket-grant.probe",standard:StandardId("1"),subset:SubsetId("*")};

fn semantics(length:usize,phase:Phase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 control.checkpoint(phase,0,1)?;control.check_rows(1)?;control.check_value_bytes(length.checked_add(8).ok_or_else(||ValueError::new(Kind::OwnershipLimit,"Socket semantic bytes overflow"))?)
}
fn physical(length:usize,phase:Phase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 semantics(length,phase,control)?;if length>control.limits().max_file_bytes{return Err(ValueError::new(Kind::OwnershipLimit,"Socket native file exceeds caller limit"))}Ok(())
}
fn maximum(before:usize,control:&SqliteSnapshotControl<'_>)->Result<usize,ValueError>{
 before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||ValueError::new(Kind::OwnershipLimit,"Socket native admission overflow"))
}

impl ArtifactNativeSnapshot for NativeSocketProbeSnapshot{
 fn decode_native_snapshot(payload:NativeSnapshotInput<'_>,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
  control.checkpoint()?;let text=match payload{NativeSnapshotInput::Binary(bytes)=>control.borrow_text(bytes)?,NativeSnapshotInput::Text(text)=>text};control.copy_text(text).map(Self)
 }
 fn encode_native_snapshot(&self,encoding:NativeSnapshotEncoding,control:&mut NativeEncodeControl<'_>)->Result<IoPayload,ValueError>{
  match encoding{NativeSnapshotEncoding::Binary=>control.copy_bytes(self.0.as_bytes()).map(IoPayload::Binary),NativeSnapshotEncoding::Text=>control.copy_text(&self.0).map(IoPayload::Text)}
 }
}

impl ArtifactSqliteSnapshot for NativeSocketProbeSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
  semantics(self.0.len(),Phase::ProjectSnapshot,control)?;let mut projection=Projection::new(Self::SQLITE_SCHEMA,control)?;projection.insert_key("socket_probe",1,&[Cell::Text(&self.0)])?;projection.finish()
 }
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  let phase=Phase::ReconstructSnapshot;control.checkpoint(phase,0,1)?;validate_sqlite_database_schema_controlled(database,Self::SQLITE_SCHEMA,phase,control)?;control.check_database(database,phase)?;
  let rows=&database.table("socket_probe")?.rows;if rows.len()!=1||rows[0].rowid!=1||rows[0].values.len()!=2||rows[0].integer(0)?!=1{return Err(ValueError::new(Kind::InvalidValue,"Socket requires one declared text entity"))}
  reconstruct_text(control,rows[0].text(1)?).map(Self)
 }
 fn decode_sqlite_snapshot_native(payload:&IoPayload,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>{
 let native=native_owner.native();
  let input=match payload{IoPayload::Binary(bytes)=>NativeSnapshotInput::Binary(bytes),IoPayload::Text(text)=>NativeSnapshotInput::Text(text)};
  let length=match input{NativeSnapshotInput::Binary(bytes)=>bytes.len(),NativeSnapshotInput::Text(text)=>text.len()};physical(length,Phase::DecodeNative,control)?;
  let before=native.owned_bytes();let result=native.scoped_maximum(maximum(before,control)?,|native|Self::decode_native_snapshot(input,native));control.admit_allocation_bytes(native.owned_bytes()-before)?;result
 }
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::os_store::NativeSnapshotEncodeOwner<'_, '_>)->Result<IoPayload,ValueError>{
  let native=owner.native();
  self.preflight_sqlite_snapshot_encoding(encoding,control)?;let encoding=match encoding{SnapshotEncoding::Binary=>NativeSnapshotEncoding::Binary,SnapshotEncoding::Text=>NativeSnapshotEncoding::Text};
  let before=native.owned_bytes();let result=native.scoped_maximum(maximum(before,control)?,|native|self.encode_native_snapshot(encoding,native));control.admit_allocation_bytes(native.owned_bytes()-before)?;result
 }
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{physical(self.0.len(),Phase::EncodeNative,control)}
}

/// 📣️ Publishes the original document and its declared SQLite IO coordinate atomically.
pub(super) fn publish()->Result<(),store::io::ArtifactAssemblyRegistryError>{
 let(owner,codec)=NativeSocketProbeSnapshot::native_snapshot_registration().expect("Socket declares its original native IO owner");store::io::register_native_snapshot_codec(owner,codec)
}
