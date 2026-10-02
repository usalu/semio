//! 🪐️ Literal ordered Space metadata, timestamp words and per-occurrence dialect identities.
use super::{SSpaceSnapshot,SpaceArtifactRow,SpaceArtifactDialect};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,validate_sqlite_database_schema,artifact::{Projection,Cell,NativeEncodingBound}}};
fn count(length:usize)->Result<usize,String>{length.checked_mul(2).and_then(|n|n.checked_add(1)).ok_or_else(||"Space occurrence row count overflow".into())}
fn words(value:u64)->[Cell<'static>;2]{[Cell::Integer((value>>32)as i64),Cell::Integer((value&0xffffffff)as i64)]}
fn timestamp(row:&SqliteRow,high:usize,low:usize)->Result<u64,String>{let high=u32::try_from(row.integer(high)?).map_err(|_|"Space timestamp high word exceeds unsigned32")?;let low=u32::try_from(row.integer(low)?).map_err(|_|"Space timestamp low word exceeds unsigned32")?;Ok((u64::from(high)<<32)|u64::from(low))}
impl ArtifactSqliteSnapshot for SSpaceSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,String>{control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;control.check_rows(count(self.artifacts.len())?)?;store::encode_sqlite_snapshot_record_native(encoding,"s.space",Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{
  let maximum=control.limits().max_rows;store::decode_sqlite_snapshot_record_native(payload,"s.space",Self::__dsl_spec_producer(),|record,native|{
   let length=match record.get(2){Some(dsl::FieldValue::List(rows))=>rows.len(),_=>return Err(store::TextError::new("Space native occurrence list missing",dsl::TextSpan::at(1,1)))};
   let rows=count(length).map_err(|e|store::TextError::new(e,dsl::TextSpan::at(1,1)))?;if rows>maximum{return Err(store::TextError::new("Space native occurrence rows exceed caller limit",dsl::TextSpan::at(1,1)))}
   Self::__dsl_from_record_controlled(record,native)
  },control)
 }
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{
  let mut bound=NativeEncodingBound::new(control)?;bound.add(4096)?;for value in[&self.schema,&self.space_id]{bound.repeated(value.len(),6)?;}for row in &self.artifacts{bound.add(1024)?;for value in[&row.id,&row.name,&row.kind_id,&row.schema,&row.created_by,&row.updated_by,&row.dialect.artifact_kind,&row.dialect.standard,&row.dialect.subset]{bound.repeated(value.len(),6)?;}}bound.finish()
 }
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,self.artifacts.len())?;control.check_rows(count(self.artifacts.len())?)?;let mut bytes=8usize.checked_add(self.schema.len()).and_then(|n|n.checked_add(self.space_id.len())).ok_or("Space document value overflow")?;control.check_value_bytes(bytes)?;
  for(index,row)in self.artifacts.iter().enumerate(){bytes=bytes.checked_add(64).ok_or("Space occurrence value overflow")?;for value in[&row.id,&row.name,&row.kind_id,&row.schema,&row.created_by,&row.updated_by,&row.dialect.artifact_kind,&row.dialect.standard,&row.dialect.subset]{bytes=bytes.checked_add(value.len()).ok_or("Space occurrence text overflow")?;control.check_value_bytes(bytes)?;}if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,index+1,self.artifacts.len())?;}}
  let mut p=Projection::new(Self::SQLITE_SCHEMA,control)?;p.insert_key("space_document",1,&[Cell::Text(&self.schema),Cell::Text(&self.space_id)])?;
  for(index,row)in self.artifacts.iter().enumerate(){let ordinal=i64::try_from(index).map_err(|_|"Space ordinal exceeds integer64")?;let id=ordinal.checked_add(1).ok_or("Space occurrence identity overflow")?;let created=words(row.created_at_ms);let updated=words(row.updated_at_ms);
   p.insert_key("space_artifact",id,&[Cell::Integer(1),Cell::Integer(ordinal),Cell::Text(&row.id),Cell::Text(&row.name),Cell::Text(&row.kind_id),Cell::Text(&row.schema),created[0],created[1],Cell::Text(&row.created_by),updated[0],updated[1],Cell::Text(&row.updated_by)])?;
   p.insert_key("space_artifact_dialect",id,&[Cell::Text(&row.dialect.artifact_kind),Cell::Text(&row.dialect.standard),Cell::Text(&row.dialect.subset)])?;
  }p.finish()
 }
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{
  control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits()).map_err(|e|e.to_string())?;
  if database.tables.len()!=3{return Err("Space requires its three literal tables".into())}let documents=database.table("space_document")?;let rows=database.table("space_artifact")?;let dialects=database.table("space_artifact_dialect")?;let root=documents.single_row()?;if rows.rows.len()!=dialects.rows.len(){return Err("Space occurrence and dialect ownership differs".into())}
  let maximum=control.limits().max_value_bytes;let mut progress=|state:protocol::native_decoding::NativeDecodeProgress|control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,state.completed,state.total).is_ok();let mut native=dsl::NativeDecodeControl::new(maximum,&mut progress);native.begin_stage(count(rows.rows.len())?)?;
  for(table,width)in[(documents,3),(rows,13),(dialects,4)]{for row in &table.rows{native.step()?;if row.values.len()!=width||row.integer(0)?!=row.rowid{return Err("Space row identity differs".into())}}}
  let mut ordered=native.allocate_vec::<Option<&SqliteRow>>(rows.rows.len())?;ordered.resize(rows.rows.len(),None);native.begin_stage(rows.rows.len())?;for row in &rows.rows{native.step()?;let ordinal=usize::try_from(row.integer(2)?).map_err(|_|"Space ordinal differs")?;if row.integer(1)?!=root.rowid||ordinal>=ordered.len()||ordered[ordinal].replace(row).is_some(){return Err("Space occurrence owner or ordinal differs".into())}}
  native.charge(dialects.rows.len().checked_mul(96).ok_or("Space dialect lookup overflow")?)?;let mut lookup=std::collections::BTreeMap::new();native.begin_stage(dialects.rows.len())?;for row in &dialects.rows{native.step()?;if lookup.insert(row.rowid,row).is_some(){return Err("Space dialect identity repeats".into())}}
  let schema=native.copy_text(root.text(1)?)?;let space_id=native.copy_text(root.text(2)?)?;let mut artifacts=native.allocate_vec::<SpaceArtifactRow>(ordered.len())?;native.begin_stage(ordered.len())?;
  for row in ordered{native.step()?;let row=row.ok_or("Space occurrence ordinal missing")?;let dialect=lookup.remove(&row.rowid).ok_or("Space dialect owner missing")?;artifacts.push(SpaceArtifactRow{id:native.copy_text(row.text(3)?)?,name:native.copy_text(row.text(4)?)?,kind_id:native.copy_text(row.text(5)?)?,schema:native.copy_text(row.text(6)?)?,dialect:SpaceArtifactDialect{artifact_kind:native.copy_text(dialect.text(1)?)?,standard:native.copy_text(dialect.text(2)?)?,subset:native.copy_text(dialect.text(3)?)?},created_at_ms:timestamp(row,7,8)?,created_by:native.copy_text(row.text(9)?)?,updated_at_ms:timestamp(row,10,11)?,updated_by:native.copy_text(row.text(12)?)?});}
  if !lookup.is_empty(){return Err("Space unowned dialect entities".into())}native.checkpoint()?;Ok(Self{schema,space_id,artifacts})
 }
 fn validate_sqlite_snapshot_subset(&self,dialect:&store::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.space.space"||dialect.standard!="1"||dialect.subset!="*"{return Err(String::from("Space does not own this semantic subset").into())}if Self::from_sqlite_database(database,control)?!=*self{return Err(String::from("Space owned document state differs").into())}Ok(store::io_schema::IoOutcome::clean(()))
 }
}
