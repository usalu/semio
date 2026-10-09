//! 🔤️ Ordered Semio text runs and inline marks as their own relational entities.

use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;

use semio_framework_os_kernel::sqlite_snapshot::artifact::reconstruct_text;
use crate::standards::v1::subsets::text::schema::snapshot::{SemioTextMark, SemioTextMarkKind, SemioTextRun, SemioTextSnapshot};
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};
use store::sqlite_snapshot::{SqliteDatabaseLimits,artifact::{RowWriter,Cell}};
#[path="🧮️semantic/🦀️.rs"]
mod semantic;

/// 🫳️ Visits each complete authored text row using the same owned or borrowed writer.
pub(crate)fn visit_rows(snapshot:&SemioTextSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key("semio_text_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,run)in snapshot.runs.iter().enumerate(){
  let id=out.insert("semio_text_run",&[Cell::Integer(1),Cell::Integer(integer(ordinal)?),Cell::Text(&run.language),Cell::Text(&run.content)])?;
  for(ordinal,mark)in run.marks.iter().enumerate(){out.insert("semio_text_mark",&[Cell::Integer(id),Cell::Integer(integer(ordinal)?),Cell::Text(kind(mark.kind)),Cell::Text(&mark.href)])?;}
 }Ok(())
}
/// 🎟️ Admits every typed SQL cell before native forecast or materialization.
pub(crate)fn admit_values(snapshot:&SemioTextSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
/// 🏛️ Admits exact layout before the native source producer begins ownership.
pub(crate)fn admit_layout(limits:SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Checks actual binary native primitives before typed field copies.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📃️ Checks actual document native primitives before typed field copies.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}


fn integer(value: usize) -> Result<i64,ValueError> { i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string())) }
fn kind(value: SemioTextMarkKind) -> &'static str { match value { SemioTextMarkKind::Bold => "bold", SemioTextMarkKind::Italic => "italic", SemioTextMarkKind::Code => "code", SemioTextMarkKind::Link => "link" } }
fn identity(row: &SqliteRow, columns: usize) -> Result<(),ValueError> { if row.values.len() != columns || row.integer(0)? != row.rowid { Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio text row identity or column count is invalid")) } else { Ok(()) } }

impl ArtifactSqliteSnapshot for SemioTextSnapshot {
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{crate::standards::v1::subsets::text::io::sqlite::snapshot::native_encoding::encode(self,encoding,control,native_owner)}
fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::text::io::sqlite::snapshot::native_decoding::decode(payload,control,native_control)}
    fn preflight_sqlite_snapshot_encoding(&self, _encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {let result=(||->Result<(),ValueError>{ admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut bound = Bound::file_only("", control)?; self.native_fields(&mut bound)?; bound.finish() })();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="text"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_text_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {self.project_sqlite_database(control)}
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {let result=(||->Result<Self,ValueError>{ semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) })();result}
}

impl SemioTextSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{
 use semio_framework_os_kernel::sqlite_snapshot::{artifact::{RowIdentity,RowIndex},transfer::reserve};
 use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
 control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;semio_framework_os_kernel::sqlite_snapshot::validate_sqlite_database_schema_controlled(database,declared_schema,SqliteSnapshotPhase::ReconstructSnapshot,control)?;
 let document=database.table("semio_text_document")?.single_row()?;identity(document,2)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio text document requires identifier 1"))}
 let runs=RowIndex::new_with_identity(database,"semio_text_run",5,&[],RowIdentity::Signed,control,"Semio text run document or identity is invalid")?;
 let marks=RowIndex::new_with_identity(database,"semio_text_mark",5,&[],RowIdentity::Signed,control,"Semio text mark run, identity or ordinal is invalid")?;
 let order=runs.ordered(2,control,"Semio text run ordinals must be contiguous and zero-based")?;
 for(count,&index)in runs.indices().iter().enumerate(){let row=runs.row(index)?;if row.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio text run document or identity is invalid"))}row.text(3)?;row.text(4)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,runs.len())?;}
 for(count,&index)in marks.indices().iter().enumerate(){let row=marks.row(index)?;if runs.get(row.integer(1)?,control)?.is_none()||row.integer(2)?<0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio text mark run, identity or ordinal is invalid"))}row.text(3)?;row.text(4)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,marks.len())?;}
 let grouped=marks.grouped_by(2,control,"Semio text mark ordinals must be contiguous and zero-based",|row|Ok((0,Some(row.integer(1)?))))?;
 let mut snapshot=Owned::new(Self{schema:String::new(),runs:Vec::new()});snapshot.get_mut().runs=reserve(order.len(),control)?;
 let mut completed=0;
 for index in order{
  let row=runs.row(index)?;let range=marks.range_by(&grouped,(0,Some(row.rowid)),control,|row|Ok((0,Some(row.integer(1)?))))?;
  let mut run=Owned::new(SemioTextRun{language:String::new(),content:String::new(),marks:Vec::new()});
  run.get_mut().marks=reserve(range.len(),control)?;run.get_mut().language=reconstruct_text(control,row.text(3)?)?;run.get_mut().content=reconstruct_text(control,row.text(4)?)?;
  for position in range{
   let row=marks.row(grouped[position])?;let kind=match row.text(3)?{"bold"=>SemioTextMarkKind::Bold,"italic"=>SemioTextMarkKind::Italic,"code"=>SemioTextMarkKind::Code,"link"=>SemioTextMarkKind::Link,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio text mark kind is invalid"))};
   let mut mark=Owned::new(SemioTextMark{kind,href:String::new()});mark.get_mut().href=reconstruct_text(control,row.text(4)?)?;run.get_mut().marks.push(mark.take());completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
  }
  snapshot.get_mut().runs.push(run.take());completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
 }
 snapshot.get_mut().schema=reconstruct_text(control,document.text(1)?)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(snapshot.take())
}
}

impl SemioTextSnapshot {
    /// 📏️ Checks this owner's explicit native fields before allocating an encoding.
    pub fn native_fields(&self, b: &mut Bound<'_, '_>) -> Result<(),ValueError> { b.text(&self.schema)?; b.entities(self.runs.len())?; for run in &self.runs { b.text(&run.language)?; b.text(&run.content)?; b.entities(run.marks.len())?; for mark in &run.marks { b.scalars(1)?; b.text(&mark.href)?; } } Ok(()) }
}

impl SemioTextSnapshot {
    /// 🪶️ Projects the owned subset into its declared relational writer while preserving refusals.
    pub fn project_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase,ValueError> {
        semantic::layout(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
