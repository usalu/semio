//! 🪐️ Literal ordered Space metadata, timestamp words and per-occurrence dialect identities.
use crate::{SSpaceSnapshot,SpaceArtifactRow,SpaceArtifactDialect};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,validate_sqlite_database_schema,artifact::{RowWriter,Cell,NativeEncodingBound}}};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(message:&'static str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

fn count(length:usize)->Result<usize,ValueError>{length.checked_mul(2).and_then(|n|n.checked_add(1)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Space occurrence row count overflow"))}
fn words(value:u64)->[Cell<'static>;2]{[Cell::Integer((value>>32)as i64),Cell::Integer((value&0xffffffff)as i64)]}
fn timestamp(row:&SqliteRow,high:usize,low:usize)->Result<u64,ValueError>{let high=u32::try_from(row.integer(high)?).map_err(|_|invalid("Space timestamp high word exceeds unsigned32"))?;let low=u32::try_from(row.integer(low)?).map_err(|_|invalid("Space timestamp low word exceeds unsigned32"))?;Ok((u64::from(high)<<32)|u64::from(low))}
impl ArtifactSqliteSnapshot for SSpaceSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;store::encode_sqlite_snapshot_record_native(encoding,"s.space",Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  let maximum=control.limits().max_rows;let value=store::decode_sqlite_snapshot_record_native(payload,"s.space",Self::__dsl_spec_producer(),|record,native|{
   let length=match record.get(2){Some(semio_framework_dsl_record::FieldValue::List(rows))=>rows.len(),_=>return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Space native occurrence list missing"))};
   let rows=count(length)?;if rows>maximum{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit, "Space native occurrence rows exceed caller limit"))}
   Self::__dsl_from_record_controlled(record,native)
  },control)?;value.admit_sqlite_values(control,SqliteSnapshotPhase::DecodeNative)?;Ok(value)
 }
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  let mut bound=NativeEncodingBound::new(control)?;bound.add(4096)?;for value in[&self.schema,&self.space_id]{bound.repeated(value.len(),6)?;}for row in &self.artifacts{bound.add(1024)?;for value in[&row.id,&row.name,&row.kind_id,&row.schema,&row.created_by,&row.updated_by,&row.dialect.artifact_kind,&row.dialect.standard,&row.dialect.subset]{bound.repeated(value.len(),6)?;}}bound.finish()
 }
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{let mut p=RowWriter::new(Self::SQLITE_SCHEMA,control)?;self.write_sqlite_rows(&mut p)?;p.finish()}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits())?;
  if database.tables.len()!=3{return Err(invalid("Space requires its three literal tables"))}let documents=database.table("space_document")?;let rows=database.table("space_artifact")?;let dialects=database.table("space_artifact_dialect")?;let root=documents.single_row()?;if rows.rows.len()!=dialects.rows.len(){return Err(invalid("Space occurrence and dialect ownership differs"))}
  let maximum=control.limits().max_value_bytes;let mut progress=|state:semio_framework_value::native_decoding::NativeDecodeProgress|control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,state.completed,state.total).is_ok();let mut native=semio_framework_value::NativeDecodeControl::new(maximum,&mut progress);native.begin_stage(count(rows.rows.len())?)?;
  for(table,width)in[(documents,3),(rows,13),(dialects,4)]{for row in &table.rows{native.step()?;if row.values.len()!=width||row.integer(0)?!=row.rowid{return Err(invalid("Space row identity differs"))}}}
  let mut ordered=native.allocate_vec::<Option<&SqliteRow>>(rows.rows.len())?;ordered.resize(rows.rows.len(),None);native.begin_stage(rows.rows.len())?;for row in &rows.rows{native.step()?;let ordinal=usize::try_from(row.integer(2)?).map_err(|_|invalid("Space ordinal differs"))?;if row.integer(1)?!=root.rowid||ordinal>=ordered.len()||ordered[ordinal].replace(row).is_some(){return Err(invalid("Space occurrence owner or ordinal differs"))}}
  native.charge(dialects.rows.len().checked_mul(96).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Space dialect lookup overflow"))?)?;let mut lookup=std::collections::BTreeMap::new();native.begin_stage(dialects.rows.len())?;for row in &dialects.rows{native.step()?;if lookup.insert(row.rowid,row).is_some(){return Err(invalid("Space dialect identity repeats"))}}
  let schema=native.copy_text(root.text(1)?)?;let space_id=native.copy_text(root.text(2)?)?;let mut artifacts=native.allocate_vec::<SpaceArtifactRow>(ordered.len())?;native.begin_stage(ordered.len())?;
  for row in ordered{native.step()?;let row=row.ok_or_else(||invalid("Space occurrence ordinal missing"))?;let dialect=lookup.remove(&row.rowid).ok_or_else(||invalid("Space dialect owner missing"))?;artifacts.push(SpaceArtifactRow{id:native.copy_text(row.text(3)?)?,name:native.copy_text(row.text(4)?)?,kind_id:native.copy_text(row.text(5)?)?,schema:native.copy_text(row.text(6)?)?,dialect:SpaceArtifactDialect{artifact_kind:native.copy_text(dialect.text(1)?)?,standard:native.copy_text(dialect.text(2)?)?,subset:native.copy_text(dialect.text(3)?)?},created_at_ms:timestamp(row,7,8)?,created_by:native.copy_text(row.text(9)?)?,updated_at_ms:timestamp(row,10,11)?,updated_by:native.copy_text(row.text(12)?)?});}
  if !lookup.is_empty(){return Err(invalid("Space unowned dialect entities"))}native.checkpoint()?;Ok(Self{schema,space_id,artifacts})
 }
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0).map_err(store::io_schema::IoError::from_value_error)?;if dialect.artifact_kind!="s.space.space"||dialect.standard!="1"||dialect.subset!="*"{return Err(store::io_schema::IoError::from_value_error(invalid("Space does not own this semantic subset")))}if Self::from_sqlite_database(database,control).map_err(store::io_schema::IoError::from_value_error)?!=*self{return Err(store::io_schema::IoError::from_value_error(invalid("Space owned document state differs")))}Ok(store::io_schema::IoOutcome::clean(()))
 }
}

impl SSpaceSnapshot{
 fn write_sqlite_rows(&self,p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
  p.check_rows(count(self.artifacts.len())?)?;p.insert_key("space_document",1,&[Cell::Text(&self.schema),Cell::Text(&self.space_id)])?;
  for(index,row)in self.artifacts.iter().enumerate(){let ordinal=i64::try_from(index).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"Space ordinal exceeds integer64"))?;let id=ordinal.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Space occurrence identity overflow"))?;let created=words(row.created_at_ms);let updated=words(row.updated_at_ms);
   p.insert_key("space_artifact",id,&[Cell::Integer(1),Cell::Integer(ordinal),Cell::Text(&row.id),Cell::Text(&row.name),Cell::Text(&row.kind_id),Cell::Text(&row.schema),created[0],created[1],Cell::Text(&row.created_by),updated[0],updated[1],Cell::Text(&row.updated_by)])?;
   p.insert_key("space_artifact_dialect",id,&[Cell::Text(&row.dialect.artifact_kind),Cell::Text(&row.dialect.standard),Cell::Text(&row.dialect.subset)])?;
  }Ok(())
 }
 fn admit_sqlite_values(&self,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{let mut p=RowWriter::borrowed(control,phase)?;self.write_sqlite_rows(&mut p)?;p.finish_borrowed()}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
