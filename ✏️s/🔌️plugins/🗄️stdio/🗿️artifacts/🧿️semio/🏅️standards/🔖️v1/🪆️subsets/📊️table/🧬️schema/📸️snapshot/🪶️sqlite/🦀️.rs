//! 📊️ Declared column kinds and positionally aligned, independently typed native cell values.
use crate::standards::v1::subsets::base::schema::snapshot::sqlite::native::Bound;
use crate::standards::v1::subsets::base::schema::snapshot::native_decoding::Owned;
use super::{SemioTableSnapshot,SemioTableColumn,SemioTableRow,SemioTableCellKind};
use crate::standards::v1::subsets::value::schema::snapshot::sqlite::{project_value_tree,reconstruct_value_forest,ValueSqliteTables};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,Projection,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase}};
use std::collections::{BTreeMap,BTreeSet};
const VALUES:ValueSqliteTables=ValueSqliteTables{value:"semio_table_value",list_element:"semio_table_list_element",map_entry:"semio_table_map_entry"};
fn number(value:usize)->Result<i64,String>{i64::try_from(value).map_err(|error|error.to_string())}
fn identity(row:&SqliteRow,columns:usize)->Result<(),String>{if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err("invalid Semio table row identity or columns".into())}else{Ok(())}}
fn kind(value:SemioTableCellKind)->&'static str{match value{SemioTableCellKind::Null=>"null",SemioTableCellKind::Bool=>"bool",SemioTableCellKind::Int=>"int",SemioTableCellKind::Float=>"float",SemioTableCellKind::Str=>"str",SemioTableCellKind::Bytes=>"bytes"}}
impl ArtifactSqliteSnapshot for SemioTableSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::schema::snapshot::native_decoding::Owned::new(self));}

fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,String>{super::native_encoding::encode(self,encoding,control)}

 fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{super::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{let mut b=Bound::new("",control)?;self.native_fields(&mut b)?;b.finish()}

fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="table"){return Err(String::from("Semio owned snapshot dialect differs from its dedicated semantic subset").into());}
let row=database.table("semio_table_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(String::from("Semio owned document identity differs from projected semantic fields").into());}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
let mut projection=Projection::new(Self::SQLITE_SCHEMA,control)?;projection.insert_key("semio_table_document",1,&[Cell::Text(&self.schema)])?;let mut names=BTreeSet::new();for(ordinal,column)in self.columns.iter().enumerate(){if !names.insert(column.name.as_str()){return Err("duplicate Semio table column name".into());}projection.insert("semio_table_column",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&column.name),Cell::Text(kind(column.kind))])?;}
for(ordinal,row)in self.rows.iter().enumerate(){if row.cells.len()!=self.columns.len(){return Err("Semio table cells must align with declared columns".into());}let id=projection.insert("semio_table_row",&[Cell::Integer(1),Cell::Integer(number(ordinal)?)])?;for(column,value)in row.cells.iter().enumerate(){let value_id=project_value_tree(value,VALUES,None,&mut projection)?;projection.insert("semio_table_cell",&[Cell::Integer(id),Cell::Integer(number(column+1)?),Cell::Integer(value_id)])?;}}projection.finish()
}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String> { Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) }
}

impl SemioTableSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,String> {

control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits()).map_err(|error|error.to_string())?;let document=database.table("semio_table_document")?.single_row()?;identity(document,2)?;if document.rowid!=1{return Err("invalid Semio table document identifier".into());}
let ordered_columns=database.table("semio_table_column")?.ordered_rows(2)?;let mut column_positions=BTreeMap::new();let mut names=BTreeSet::new();let mut columns=Vec::new();for(ordinal,row)in ordered_columns.into_iter().enumerate(){identity(row,5)?;if row.integer(1)?!=1||column_positions.insert(row.rowid,ordinal).is_some()||!names.insert(row.text(3)?){return Err("invalid Semio table column ownership or identity".into());}let kind=match row.text(4)?{"null"=>SemioTableCellKind::Null,"bool"=>SemioTableCellKind::Bool,"int"=>SemioTableCellKind::Int,"float"=>SemioTableCellKind::Float,"str"=>SemioTableCellKind::Str,"bytes"=>SemioTableCellKind::Bytes,_=>return Err("unknown Semio table column kind".into())};columns.push(SemioTableColumn{name:reconstruct_text(control,row.text(3)?)?,kind});}
let ordered_rows=database.table("semio_table_row")?.ordered_rows(2)?;let mut row_positions=BTreeMap::new();for(ordinal,row)in ordered_rows.iter().enumerate(){identity(row,3)?;if row.integer(1)?!=1||row_positions.insert(row.rowid,ordinal).is_some(){return Err("invalid Semio table row ownership or identity".into());}}
let mut cells=BTreeMap::new();let mut ids=BTreeSet::new();let mut completed=0usize;for row in &database.table("semio_table_cell")?.rows{identity(row,4)?;let owner=*row_positions.get(&row.integer(1)?).ok_or("dangling Semio table row")?;let column=*column_positions.get(&row.integer(2)?).ok_or("dangling Semio table column")?;if !ids.insert(row.rowid)||cells.insert((owner,column),row.integer(3)?).is_some(){return Err("duplicate Semio table cell identity or position".into());}completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
let expected=ordered_rows.len().checked_mul(columns.len()).ok_or("Semio table dimensions overflow")?;if cells.len()!=expected{return Err("Semio table cells must align with declared columns".into());}let roots=cells.values().copied().collect::<Vec<_>>();let mut values=Owned::new(reconstruct_value_forest(database,VALUES,&roots,None,control)?);values.get_mut().reverse();let mut rows=Owned::new(Vec::new());for _ in ordered_rows{let mut cells=Owned::new(Vec::new());for _ in &columns{cells.get_mut().push(values.get_mut().pop().ok_or("missing Semio table cell value")?);}rows.get_mut().push(SemioTableRow{cells:cells.take()});}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(Self{schema:reconstruct_text(control,document.text(1)?)?,columns,rows:rows.take()})
    }
}

impl SemioTableSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),String>{b.text(&self.schema)?;b.entities(self.columns.len())?;for column in &self.columns{b.text(&column.name)?;b.scalars(1)?;}b.entities(self.rows.len())?;for row in &self.rows{crate::standards::v1::subsets::value::schema::snapshot::sqlite::native_values(&row.cells,b)?;}Ok(())}
}
