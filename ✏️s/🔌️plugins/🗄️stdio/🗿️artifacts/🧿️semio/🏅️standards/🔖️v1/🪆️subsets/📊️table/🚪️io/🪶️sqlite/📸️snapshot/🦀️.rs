//! 📊️ Ordered declaration hints and independent contiguous native cell occurrences.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native::Bound;
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::Owned;
use crate::schema::snapshot::SemioTableSnapshot;
use crate::schema::snapshot::SemioTableColumn;
use crate::schema::snapshot::SemioTableRow;
use crate::schema::snapshot::SemioTableCellKind;
use semio_s_artifact_stdio_semio_value_model::io::sqlite::snapshot::project_value_tree;
use semio_s_artifact_stdio_semio_value_model::io::sqlite::snapshot::ValueSqliteTables;
use semio_framework_os_kernel::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{artifact::{Cell,RowWriter},SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase}};

use store::sqlite_snapshot::SqliteDatabaseLimits;
#[path="🧮️semantic/🦀️.rs"]
pub(crate)mod semantic;
#[path="🫙️projection/🦀️.rs"]
mod projection;
#[path="💰️reconstruction/🦀️.rs"]
mod reconstruction;
/// 🎟️ Admits every typed cell before native forecasting or allocation.
pub(crate)fn admit_values(snapshot:&SemioTableSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;projection::admit(snapshot,phase,control)}
/// 🏛️ Checks the exact authored layout before native ownership.
pub(crate)fn admit_layout(limits:SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts actual binary primitive cells before typed construction.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📃️ Counts actual document primitive cells before typed construction.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}
/// 📊️ Visits independent declarations and every cell without imposing a rectangular shape.
pub fn visit_rows(snapshot:&SemioTableSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key("semio_table_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,column)in snapshot.columns.iter().enumerate(){out.insert("semio_table_column",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&column.name),Cell::Text(kind(column.kind))])?;}
 for(ordinal,row)in snapshot.rows.iter().enumerate(){let id=out.insert("semio_table_row",&[Cell::Integer(1),Cell::Integer(number(ordinal)?)])?;for(column,value)in row.cells.iter().enumerate(){let value_id=project_value_tree(value,VALUES,None,out)?;out.insert("semio_table_cell",&[Cell::Integer(id),Cell::Integer(number(column)?),Cell::Integer(value_id)])?;}}Ok(())
}

const VALUES:ValueSqliteTables=ValueSqliteTables{value:"semio_table_value",list_element:"semio_table_list_element",map_entry:"semio_table_map_entry"};
fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity(row:&SqliteRow,columns:usize)->Result<(),ValueError>{if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio table row identity or columns"))}else{Ok(())}}
fn kind(value:SemioTableCellKind)->&'static str{match value{SemioTableCellKind::Null=>"null",SemioTableCellKind::Bool=>"bool",SemioTableCellKind::Int=>"int",SemioTableCellKind::Float=>"float",SemioTableCellKind::Str=>"str",SemioTableCellKind::Bytes=>"bytes"}}
impl ArtifactSqliteSnapshotHost for SemioTableSnapshot{
fn retire_sqlite_snapshot(self){drop(Owned::new(self));}

fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{native_encoding::encode(self,encoding,control,native_owner)}

 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{native_decoding::decode(payload,control,native_control.native())}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{(|| -> Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})()}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{(|| -> Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="table"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_table_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}




}
impl pack::snapshot::ArtifactSqliteSnapshot for SemioTableSnapshot {
const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA)}
}

impl SemioTableSnapshot {
    /// 🧩️ Projects owned semantic fields with typed relational refusals.
    pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{semantic::layout(control.limits())?;projection::project(self,Self::SQLITE_SCHEMA,control)}

    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{reconstruction::reconstruct(database,control,declared_schema)}
}

impl SemioTableSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.columns.len())?;for column in &self.columns{b.text(&column.name)?;b.scalars(1)?;}b.entities(self.rows.len())?;for row in &self.rows{semio_s_artifact_stdio_semio_value_model::io::sqlite::snapshot::native_values(&row.cells,b)?;}Ok(())}
}




#[path = "🛫️native/🦀️.rs"]
pub mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub mod native_decoding;
