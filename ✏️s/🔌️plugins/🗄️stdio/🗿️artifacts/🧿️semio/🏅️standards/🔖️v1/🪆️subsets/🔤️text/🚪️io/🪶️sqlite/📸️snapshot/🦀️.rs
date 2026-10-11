//! 🔤️ Ordered Semio text runs and inline marks as their own relational entities.

use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;

use crate::standards::v1::subsets::text::schema::snapshot::{SemioTextMark, SemioTextMarkKind, SemioTextRun, SemioTextSnapshot};
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};
use store::sqlite_snapshot::{SqliteDatabaseLimits,artifact::{RowWriter,Cell}};
#[path="💰️reconstruction/🦀️.rs"]
mod reconstruction;
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
fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{crate::standards::v1::subsets::text::io::sqlite::snapshot::native_decoding::decode(payload,control,native_control.native())}
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
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{reconstruction::reconstruct(database,control,declared_schema)}
}


impl SemioTextSnapshot {
    /// 📏️ Checks this owner's explicit native fields before allocating an encoding.
    pub fn native_fields(&self, b: &mut Bound<'_, '_>) -> Result<(),ValueError> { b.text(&self.schema)?; b.entities(self.runs.len())?; for run in &self.runs { b.text(&run.language)?; b.text(&run.content)?; b.entities(run.marks.len())?; for mark in &run.marks { b.scalars(1)?; b.text(&mark.href)?; } } Ok(()) }
}

impl SemioTextSnapshot {
    /// 🪶️ Projects the owned subset into its declared relational writer while preserving refusals.
    pub fn project_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase,ValueError> {
        semantic::layout(control.limits())?;crate::standards::v1::subsets::base::io::sqlite::snapshot::projection::project_rows_owned(Self::SQLITE_SCHEMA,control,|out|visit_rows(self,out))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
