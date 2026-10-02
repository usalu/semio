//! 🌊️ Named flow nodes, ordered parameters and port-addressed edge entities.
use crate::standards::v1::subsets::base::schema::snapshot::sqlite::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow,insert_ieee754,insert_key_ieee754};
use super::{SemioFlowSnapshot,FlowNode,FlowEdge,FlowParam,PortRef,SemioPoint2};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,Projection,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase}};
use std::collections::{BTreeMap,BTreeSet};
fn number(value:usize)->Result<i64,String>{i64::try_from(value).map_err(|error|error.to_string())}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),String>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err("invalid Semio flow row identity or columns".into())}else{Ok(())}}
impl ArtifactSqliteSnapshot for SemioFlowSnapshot{
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,String>{super::native_encoding::encode(self,encoding,control)}
fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{super::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{let mut b=Bound::new("",control)?;self.native_fields(&mut b)?;b.finish()}

fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="flow"){return Err(String::from("Semio owned snapshot dialect differs from its dedicated semantic subset").into());}
let row=database.table("semio_flow_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(String::from("Semio owned document identity differs from projected semantic fields").into());}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
control.check_rows(self.nodes.len().checked_add(self.edges.len()).and_then(|count|count.checked_add(1)).ok_or("Semio flow row count overflow")?)?;let mut nodes=BTreeMap::new();for(ordinal,node)in self.nodes.iter().enumerate(){if ordinal%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;}if nodes.insert(node.id.as_str(),number(ordinal+1)?).is_some(){return Err("duplicate Semio flow node identifier".into());}}
let mut projection=Projection::new(Self::SQLITE_SCHEMA,control)?;projection.insert_key_float("semio_flow_document",1,&[Cell::Text(&self.schema)])?;
for(ordinal,node)in self.nodes.iter().enumerate(){let id=projection.insert_float("semio_flow_node",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&node.id),Cell::Text(&node.kind),Cell::Text(&node.label),Cell::Real(node.position.x),Cell::Real(node.position.y)])?;for(ordinal,param)in node.params.iter().enumerate(){projection.insert_float("semio_flow_parameter",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&param.key),Cell::Text(&param.value)])?;}}
let mut ids=BTreeSet::new();for(ordinal,edge)in self.edges.iter().enumerate(){if !ids.insert(edge.id.as_str()){return Err("duplicate Semio flow edge identifier".into());}projection.insert_float("semio_flow_edge",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&edge.id),Cell::Integer(*nodes.get(edge.from.node.as_str()).ok_or("dangling Semio flow source node")?),Cell::Text(&edge.from.port),Cell::Integer(*nodes.get(edge.to.node.as_str()).ok_or("dangling Semio flow target node")?),Cell::Text(&edge.to.port),Cell::Text(&edge.kind)])?;}projection.finish()
}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String> { Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) }
}

impl SemioFlowSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,String> {

control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits()).map_err(|error|error.to_string())?;let document=single_float_row(database,"semio_flow_document")?;identity(document,2)?;if document.rowid!=1{return Err("invalid Semio flow document identifier".into());}
let ordered=ordered_float_rows(database,"semio_flow_node",2,control)?;let mut names=BTreeMap::new();let mut native_ids=BTreeSet::new();for row in &ordered{identity(row,8)?;if row.integer(1)?!=1||!native_ids.insert(row.text(3)?)||names.insert(row.rowid,row.text(3)?).is_some(){return Err("invalid Semio flow node ownership or identity".into());}}
let mut parameters=BTreeMap::<i64,Vec<SqliteRow<'_>>>::new();let mut ids=BTreeSet::new();let mut completed=0usize;for row in float_rows(database,"semio_flow_parameter",control)?{identity(row,5)?;if !names.contains_key(&row.integer(1)?)||!ids.insert(row.rowid){return Err("invalid Semio flow parameter owner or identity".into());}row.integer(2)?;parameters.entry(row.integer(1)?).or_default().push(row);completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
let mut nodes=Vec::new();for row in ordered{let mut ordered_params=parameters.remove(&row.rowid).unwrap_or_default();ordered_params.sort_by_key(|row|row.integer(2).unwrap_or(-1));let mut params=Vec::new();for(ordinal,param)in ordered_params.into_iter().enumerate(){if param.integer(2)?!=number(ordinal)?{return Err("Semio flow parameter ordinals must be contiguous".into());}params.push(FlowParam{key:reconstruct_text(control,param.text(3)?)?,value:reconstruct_text(control,param.text(4)?)?});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}nodes.push(FlowNode{id:reconstruct_text(control,row.text(3)?)?,kind:reconstruct_text(control,row.text(4)?)?,label:reconstruct_text(control,row.text(5)?)?,position:SemioPoint2{x:row.real(6)?,y:row.real(7)?},params});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
let mut edges=Vec::new();let mut ids=BTreeSet::new();let mut native_ids=BTreeSet::new();for row in ordered_float_rows(database,"semio_flow_edge",2,control)?{identity(row,9)?;if row.integer(1)?!=1||!ids.insert(row.rowid)||!native_ids.insert(row.text(3)?){return Err("invalid Semio flow edge ownership or identity".into());}edges.push(FlowEdge{id:reconstruct_text(control,row.text(3)?)?,from:PortRef{node:reconstruct_text(control,names.get(&row.integer(4)?).ok_or("dangling Semio flow source node")?)?,port:reconstruct_text(control,row.text(5)?)?},to:PortRef{node:reconstruct_text(control,names.get(&row.integer(6)?).ok_or("dangling Semio flow target node")?)?,port:reconstruct_text(control,row.text(7)?)?},kind:reconstruct_text(control,row.text(8)?)?});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(Self{schema:reconstruct_text(control,document.text(1)?)?,nodes,edges})
    }
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_flow_node"=>&[FloatColumn::Binary64(6),FloatColumn::Binary64(7)],_=>&[]}}
trait FloatProjection { fn insert_float(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,String>; fn insert_key_float(&mut self,table:&str,key:i64,cells:&[Cell<'_>])->Result<(),String>; }
impl FloatProjection for Projection<'_,'_> { fn insert_float(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,String>{insert_ieee754(self,table,cells,float_columns(table))} fn insert_key_float(&mut self,table:&str,key:i64,cells:&[Cell<'_>])->Result<(),String>{insert_key_ieee754(self,table,key,cells,float_columns(table))} }
fn float_rows<'a>(db:&'a SqliteDatabase,table:&str,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,String>{let table_rows=db.table(table)?;control.check_rows(table_rows.rows.len())?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,table_rows.rows.len())?;let mut result=Vec::new();for(count,row)in table_rows.rows.iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,table_rows.rows.len())?;}}Ok(result)}
fn ordered_float_rows<'a>(db:&'a SqliteDatabase,table:&str,ordinal:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,String>{let mut result=Vec::new();for(count,row)in semio_framework_os_kernel::sqlite_snapshot::artifact::ordered_row_refs(db.table(table)?,ordinal,control)?.into_iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,0)?;}}Ok(result)}
fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,String>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioFlowSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),String>{b.text(&self.schema)?;b.entities(self.nodes.len())?;for node in &self.nodes{b.text(&node.id)?;b.text(&node.kind)?;b.text(&node.label)?;b.scalars(2)?;b.entities(node.params.len())?;for param in &node.params{b.text(&param.key)?;b.text(&param.value)?;}}b.entities(self.edges.len())?;for edge in &self.edges{b.text(&edge.id)?;b.text(&edge.from.node)?;b.text(&edge.from.port)?;b.text(&edge.to.node)?;b.text(&edge.to.port)?;b.text(&edge.kind)?;}Ok(())}
}
