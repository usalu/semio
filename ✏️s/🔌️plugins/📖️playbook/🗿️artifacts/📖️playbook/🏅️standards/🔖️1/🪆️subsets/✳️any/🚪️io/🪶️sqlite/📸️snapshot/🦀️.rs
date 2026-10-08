//! 📖️ Individually authored persisted Playbook document and its typed flow child address.
use crate::standards::v1::subsets::any::schema::snapshot::PlaybookSnapshot;
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,validate_sqlite_database_schema,artifact::{Cell,RowWriter,Reconstruction}}};
use semio_framework_value::{ValueError,ValueRefusalKind,NativeDecodeControl,NativeEncodeControl};
use semio_framework_dsl_record::{DslField,FieldValue,RecordValue,native_encoding::EncodedRecord};
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn extent(limits:SqliteDatabaseLimits)->Result<(),ValueError>{if PlaybookSnapshot::SQLITE_SCHEMA.len()>limits.max_schema_bytes||limits.max_tables<2||limits.max_columns<6||limits.max_rows<2{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Playbook authored schema extent exceeds caller limits"))}Ok(())}
fn visit(s:&PlaybookSnapshot,p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 p.insert("playbook_document",&[Cell::Text(&s.schema),Cell::Text(&s.id),Cell::Text(&s.version),s.title.as_deref().map(Cell::Text).unwrap_or(Cell::Null)])?;
 let f=&s.flow;let t=&f.target;let d=&t.dialect;p.insert("playbook_flow_child",&[Cell::Text(&f.child_id),Cell::Text(&t.artifact_id),Cell::Text(&d.artifact_kind),Cell::Text(&d.standard),Cell::Text(&d.subset)])?;Ok(())
}
fn admit(s:&PlaybookSnapshot,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{extent(c.limits())?;let mut p=RowWriter::borrowed(c,SqliteSnapshotPhase::ProjectSnapshot)?;visit(s,&mut p)?;p.finish_borrowed()}
fn shape(record:&RecordValue,count:u16)->Result<(),ValueError>{if record.fields.keys().copied().eq(0..count){Ok(())}else{Err(invalid("Playbook native record field map differs"))}}
fn text(record:&RecordValue,id:u16)->Result<&str,ValueError>{match record.fields.get(&id){Some(FieldValue::Text(value))=>Ok(value),_=>Err(invalid("Playbook required native text role differs"))}}
fn record(value:Option<&FieldValue>)->Result<&RecordValue,ValueError>{match value{Some(FieldValue::Record(value))=>Ok(value),_=>Err(invalid("Playbook required native record role differs"))}}
fn admit_record(source:&RecordValue,limits:SqliteDatabaseLimits,n:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
 extent(limits)?;n.step()?;shape(source,5)?;let child=record(source.fields.get(&4))?;shape(child,2)?;let target=record(child.fields.get(&1))?;shape(target,4)?;
 let title=match source.fields.get(&3){Some(FieldValue::Absent)=>"",Some(FieldValue::Text(value))=>value,_=>return Err(invalid("Playbook optional native title role differs"))};
 let mut bytes=16usize;for value in[text(source,0)?,text(source,1)?,text(source,2)?,title,text(child,0)?,text(target,0)?,text(target,1)?,text(target,2)?,text(target,3)?]{n.step()?;bytes=bytes.checked_add(value.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Playbook semantic extent overflow"))?;}
 if bytes>limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Playbook semantic value limit exceeded"))}Ok(())
}
fn field<T:DslField>(value:&T,n:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{n.scoped_stage(|n|{n.begin_stage(0)?;value.to_value_controlled(n)})}
fn project_record(s:&PlaybookSnapshot,n:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{
 n.scoped_stage(|n|{n.begin_stage(5)?;let mut record=EncodedRecord::new(5,n)?;record.insert(0,field(&s.schema,n)?)?;n.step()?;record.insert(1,field(&s.id,n)?)?;n.step()?;record.insert(2,field(&s.version,n)?)?;n.step()?;
 let title=match &s.title{Some(value)=>field(value,n)?,None=>FieldValue::Absent};record.insert(3,title)?;n.step()?;record.insert(4,field(&s.flow,n)?)?;n.step()?;Ok(record.take())})
}

impl ArtifactSqliteSnapshot for PlaybookSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{admit(self,c)}
 fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{let limits=c.limits();extent(limits)?;store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),crate::standards::v1::subsets::any::io::text::snapshot::PlaybookPackRecord::__dsl_spec_producer(),|record,n|{admit_record(record,limits,n)?;crate::standards::v1::subsets::any::io::text::snapshot::PlaybookPackRecord::__dsl_from_record_controlled(record,n).map(crate::standards::v1::subsets::any::io::text::snapshot::PlaybookPackRecord::into_snapshot)},c)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{admit(self,c)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),crate::standards::v1::subsets::any::io::text::snapshot::PlaybookPackRecord::__dsl_spec_producer(),|n|project_record(self,n),c)}
 fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{extent(c.limits())?;let mut p=RowWriter::new(Self::SQLITE_SCHEMA,c)?;visit(self,&mut p)?;p.finish()}
 fn from_sqlite_database(d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{validate_sqlite_database_schema(d,Self::SQLITE_SCHEMA,c.limits())?;c.check_database(d,SqliteSnapshotPhase::ReconstructSnapshot)?;let p=d.table("playbook_document")?.single_row()?;let f=d.table("playbook_flow_child")?.single_row()?;if p.rowid!=1||f.rowid!=1||p.integer(0)?!=1||f.integer(0)?!=1||p.values.len()!=5||f.values.len()!=6{return Err(invalid("Playbook requires exact identity-1 parent and child rows"))}let mut r=Reconstruction::new(c)?;let schema=r.text(p.text(1)?)?;let id=r.text(p.text(2)?)?;let version=r.text(p.text(3)?)?;let title=p.optional_text(4)?.map(|s|r.text(s)).transpose()?;let flow=store::ArtifactChild::new(r.text(f.text(1)?)?,semio_framework_artifact_reference::ArtifactRef{artifact_id:r.text(f.text(2)?)?,dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:r.text(f.text(3)?)?,standard:r.text(f.text(4)?)?,subset:r.text(f.text(5)?)?}});r.checkpoint()?;Ok(Self{schema,id,version,title,flow})}
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1).map_err(store::io_schema::IoError::from_value_error)?;if dialect.artifact_kind!="s.playbook.playbook"||dialect.standard!="1"||dialect.subset!="*"{return Err(store::io_schema::IoError::from_value_error(invalid("Playbook dialect differs from its owned wildcard")))}if d.table("playbook_document").map_err(store::io_schema::IoError::from_value_error)?.single_row().map_err(store::io_schema::IoError::from_value_error)?.text(1).map_err(store::io_schema::IoError::from_value_error)?!=self.schema{return Err(store::io_schema::IoError::from_value_error(invalid("Playbook projected schema differs")))}c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1).map_err(store::io_schema::IoError::from_value_error)?;Ok(store::io_schema::IoOutcome::clean(()))}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

