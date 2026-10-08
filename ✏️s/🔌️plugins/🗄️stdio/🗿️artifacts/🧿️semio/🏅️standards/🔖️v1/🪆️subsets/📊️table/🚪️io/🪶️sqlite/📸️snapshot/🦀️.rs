//! 📊️ Ordered declaration hints and independent contiguous native cell occurrences.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableSnapshot,SemioTableColumn,SemioTableRow,SemioTableCellKind};
use crate::standards::v1::subsets::value::io::sqlite::snapshot::{project_value_tree,reconstruct_value_forest,ValueSqliteTables};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase}};
use semio_framework_os_kernel::sqlite_snapshot::transfer;

use store::sqlite_snapshot::SqliteDatabaseLimits;
#[path="🧮️semantic/🦀️.rs"]
pub(crate)mod semantic;
/// 🎟️ Admits every typed cell before native forecasting or allocation.
pub(crate)fn admit_values(snapshot:&SemioTableSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
/// 🏛️ Checks the exact authored layout before native ownership.
pub(crate)fn admit_layout(limits:SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts actual binary primitive cells before typed construction.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📃️ Counts actual document primitive cells before typed construction.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}
/// 📊️ Visits independent declarations and every cell without imposing a rectangular shape.
pub(crate)fn visit_rows(snapshot:&SemioTableSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key("semio_table_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,column)in snapshot.columns.iter().enumerate(){out.insert("semio_table_column",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&column.name),Cell::Text(kind(column.kind))])?;}
 for(ordinal,row)in snapshot.rows.iter().enumerate(){let id=out.insert("semio_table_row",&[Cell::Integer(1),Cell::Integer(number(ordinal)?)])?;for(column,value)in row.cells.iter().enumerate(){let value_id=project_value_tree(value,VALUES,None,out)?;out.insert("semio_table_cell",&[Cell::Integer(id),Cell::Integer(number(column)?),Cell::Integer(value_id)])?;}}Ok(())
}

const VALUES:ValueSqliteTables=ValueSqliteTables{value:"semio_table_value",list_element:"semio_table_list_element",map_entry:"semio_table_map_entry"};
fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity(row:&SqliteRow,columns:usize)->Result<(),ValueError>{if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio table row identity or columns"))}else{Ok(())}}
fn kind(value:SemioTableCellKind)->&'static str{match value{SemioTableCellKind::Null=>"null",SemioTableCellKind::Bool=>"bool",SemioTableCellKind::Int=>"int",SemioTableCellKind::Float=>"float",SemioTableCellKind::Str=>"str",SemioTableCellKind::Bytes=>"bytes"}}
impl ArtifactSqliteSnapshot for SemioTableSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}

fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io::IoPayload,ValueError>{crate::standards::v1::subsets::table::io::sqlite::snapshot::native_encoding::encode(self,encoding,control)}

 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::table::io::sqlite::snapshot::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{(|| -> Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})()}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{(|| -> Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="table"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_table_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA)}
}

impl SemioTableSnapshot {
    /// 🧩️ Projects owned semantic fields with typed relational refusals.
    pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()} 

    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,ValueError> {

control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;semio_framework_os_kernel::sqlite_snapshot::validate_sqlite_database_schema_controlled(database,declared_schema,SqliteSnapshotPhase::ReconstructSnapshot,control)?;let document=database.table("semio_table_document")?.single_row()?;identity(document,2)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio table document identifier"));}
let mut result=Owned::new(Self{schema:String::new(),columns:Vec::new(),rows:Vec::new()});let column_rows=ordered(&database.table("semio_table_column")?.rows,5,control)?;let ordered_rows=ordered(&database.table("semio_table_row")?.rows,3,control)?;
result.get_mut().columns=transfer::reserve(column_rows.len(),control)?;for(at,row)in column_rows.iter().enumerate(){let kind=match row.text(4)?{"null"=>SemioTableCellKind::Null,"bool"=>SemioTableCellKind::Bool,"int"=>SemioTableCellKind::Int,"float"=>SemioTableCellKind::Float,"str"=>SemioTableCellKind::Str,"bytes"=>SemioTableCellKind::Bytes,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio table column kind"))};result.get_mut().columns.push(SemioTableColumn{name:reconstruct_text(control,row.text(3)?)?,kind});if(at+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,at+1,column_rows.len())?;}}
let row_ids=identities(&database.table("semio_table_row")?.rows,3,control)?;let cell_rows=&database.table("semio_table_cell")?.rows;identities(cell_rows,4,control)?;
let mut cells=transfer::reserve(cell_rows.len(),control)?;for(at,row)in cell_rows.iter().enumerate(){if row_ids.binary_search(&row.integer(1)?).is_err(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio table row"));}cells.push(row);if(at+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,at+1,cell_rows.len())?;}}
transfer::heap_sort(&mut cells,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.integer(1)?.cmp(&b.integer(1)?).then(a.integer(2)?.cmp(&b.integer(2)?))))?;
let mut roots=transfer::reserve(cells.len(),control)?;let mut widths=transfer::reserve(ordered_rows.len(),control)?;let mut completed=0usize;
for row in &ordered_rows{let start=cells.partition_point(|cell|cell.integer(1).is_ok_and(|parent|parent<row.rowid));let end=cells.partition_point(|cell|cell.integer(1).is_ok_and(|parent|parent<=row.rowid));widths.push(end-start);for(ordinal,cell)in cells[start..end].iter().enumerate(){if cell.integer(2)?!=number(ordinal)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio table cell ordinals require contiguous occurrences"));}roots.push(cell.integer(3)?);completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,cells.len())?;}}}
if completed!=cells.len(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unconsumed Semio table cell"));}
let mut values=Owned::new(reconstruct_value_forest(database,VALUES,&roots,None,control)?);let count=values.get_mut().len();for(at,left)in(0..count/2).enumerate(){values.get_mut().swap(left,count-1-left);if(at+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,at+1,count/2)?;}}
let mut rows=Owned::new(transfer::reserve(widths.len(),control)?);for width in widths{let mut cells=Owned::new(transfer::reserve(width,control)?);for _ in 0..width{cells.get_mut().push(values.get_mut().pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing Semio table cell value"))?);completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}rows.get_mut().push(SemioTableRow{cells:cells.take()});}
if !values.get_mut().is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unconsumed Semio table cell value"));}result.get_mut().rows=rows.take();result.get_mut().schema=reconstruct_text(control,document.text(1)?)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(result.take())
    }
}

impl SemioTableSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.columns.len())?;for column in &self.columns{b.text(&column.name)?;b.scalars(1)?;}b.entities(self.rows.len())?;for row in &self.rows{crate::standards::v1::subsets::value::io::sqlite::snapshot::native_values(&row.cells,b)?;}Ok(())}
}

fn ordered<'a>(rows:&'a[SqliteRow],columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{
 identities(rows,columns,control)?;let mut ordered=transfer::reserve(rows.len(),control)?;for(at,row)in rows.iter().enumerate(){if row.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio table document parent"));}ordered.push(row);if(at+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,at+1,rows.len())?;}}
 transfer::heap_sort(&mut ordered,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.integer(2)?.cmp(&b.integer(2)?)))?;for(at,row)in ordered.iter().enumerate(){if row.integer(2)?!=number(at)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio table row ordinals require contiguous occurrences"));}if(at+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,at+1,rows.len())?;}}Ok(ordered)
}
fn identities(rows:&[SqliteRow],columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<i64>,ValueError>{
 let mut ids=transfer::reserve(rows.len(),control)?;for(at,row)in rows.iter().enumerate(){identity(row,columns)?;ids.push(row.rowid);if(at+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,at+1,rows.len())?;}}
 transfer::heap_sort(&mut ids,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.cmp(b)))?;for(at,id)in ids.iter().enumerate(){if at>0&&ids[at-1]==*id{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio table row identity"));}if(at+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,at+1,rows.len())?;}}Ok(ids)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
