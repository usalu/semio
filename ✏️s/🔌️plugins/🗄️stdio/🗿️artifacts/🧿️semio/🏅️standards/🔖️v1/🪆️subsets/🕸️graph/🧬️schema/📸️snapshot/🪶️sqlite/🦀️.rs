//! 🕸️ Named graph nodes, ports, edges and explicitly typed property values.
use crate::standards::v1::subsets::base::schema::snapshot::sqlite::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow,insert_ieee754,insert_key_ieee754};
use crate::standards::v1::subsets::base::schema::snapshot::native_decoding::Owned;
use super::{SemioGraphSnapshot,SemioGraphNode,SemioGraphEdge,SemioGraphPort,SemioGraphPortKind,GraphNodeId,GraphEdgeId,SemioPoint2,SemioValueEntry};
use crate::standards::v1::subsets::value::schema::snapshot::sqlite::{project_value_tree,reconstruct_value_forest,ValueSqliteTables};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,Projection,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase}};
use std::collections::{BTreeMap,BTreeSet};
const VALUES:ValueSqliteTables=ValueSqliteTables{value:"semio_graph_value",list_element:"semio_graph_list_element",map_entry:"semio_graph_map_entry"};
fn number(value:usize)->Result<i64,String>{i64::try_from(value).map_err(|error|error.to_string())}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),String>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err("invalid Semio graph row identity or columns".into())}else{Ok(())}}
fn ordered<'a>(mut rows:Vec<SqliteRow<'a>>)->Result<Vec<SqliteRow<'a>>,String>{rows.sort_by_key(|row|row.integer(2).unwrap_or(-1));for(ordinal,row)in rows.iter().enumerate(){if row.integer(2)?!=number(ordinal)?{return Err("Semio graph relationship ordinals must be contiguous".into());}}Ok(rows)}
impl ArtifactSqliteSnapshot for SemioGraphSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::schema::snapshot::native_decoding::Owned::new(self));}

fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,String>{super::native_encoding::encode(self,encoding,control)}

 fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{super::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{let mut b=Bound::new("",control)?;self.native_fields(&mut b)?;b.finish()}

fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="graph"){return Err(String::from("Semio owned snapshot dialect differs from its dedicated semantic subset").into());}
let row=database.table("semio_graph_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(String::from("Semio owned document identity differs from projected semantic fields").into());}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
control.check_rows(self.nodes.len().checked_add(self.edges.len()).and_then(|count|count.checked_add(1)).ok_or("Semio graph row count overflow")?)?;let mut nodes=BTreeMap::new();for(ordinal,node)in self.nodes.iter().enumerate(){if ordinal%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;}if nodes.insert(node.id.value.as_str(),number(ordinal+1)?).is_some(){return Err("duplicate Semio graph node identifier".into());}}
let mut projection=Projection::new(Self::SQLITE_SCHEMA,control)?;projection.insert_key_float("semio_graph_document",1,&[Cell::Text(&self.schema)])?;
for(ordinal,node)in self.nodes.iter().enumerate(){let id=projection.insert_float("semio_graph_node",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&node.id.value),Cell::Text(&node.kind),Cell::Text(&node.label),Cell::Real(node.position.x),Cell::Real(node.position.y)])?;for(ordinal,port)in node.ports.iter().enumerate(){projection.insert_float("semio_graph_port",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&port.name),Cell::Text(match port.kind{SemioGraphPortKind::In=>"in",SemioGraphPortKind::Out=>"out",SemioGraphPortKind::InOut=>"in_out"})])?;}for(ordinal,property)in node.properties.iter().enumerate(){let value_id=project_value_tree(&property.value,VALUES,None,&mut projection)?;projection.insert_float("semio_graph_property",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&property.key),Cell::Integer(value_id)])?;}}
let mut ids=BTreeSet::new();for(ordinal,edge)in self.edges.iter().enumerate(){if !ids.insert(edge.id.value.as_str()){return Err("duplicate Semio graph edge identifier".into());}projection.insert_float("semio_graph_edge",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&edge.id.value),Cell::Integer(*nodes.get(edge.source.value.as_str()).ok_or("dangling Semio graph source")?),Cell::Integer(*nodes.get(edge.target.value.as_str()).ok_or("dangling Semio graph target")?),Cell::Text(&edge.kind),Cell::Text(&edge.label)])?;}projection.finish()
}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String> { Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) }
}

impl SemioGraphSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,String> {

control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits()).map_err(|error|error.to_string())?;let document=single_float_row(database,"semio_graph_document")?;identity(document,2)?;if document.rowid!=1{return Err("invalid Semio graph document identifier".into());}
let rows=ordered_float_rows(database,"semio_graph_node",2,control)?;let mut names=BTreeMap::new();let mut native_ids=BTreeSet::new();for row in &rows{identity(row,8)?;if row.integer(1)?!=1||names.insert(row.rowid,row.text(3)?).is_some()||!native_ids.insert(row.text(3)?){return Err("invalid Semio graph node ownership or identity".into());}}
let mut ports=BTreeMap::<i64,Vec<SqliteRow<'_>>>::new();let mut properties=BTreeMap::<i64,Vec<SqliteRow<'_>>>::new();let mut completed=0usize;
for(table,map)in [("semio_graph_port",&mut ports),("semio_graph_property",&mut properties)]{let mut ids=BTreeSet::new();for row in float_rows(database,table,control)?{identity(row,5)?;if !names.contains_key(&row.integer(1)?)||!ids.insert(row.rowid){return Err("invalid Semio graph relationship ownership or identity".into());}row.integer(2)?;map.entry(row.integer(1)?).or_default().push(row);completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}}
let mut roots=Vec::new();let mut property_rows=BTreeMap::new();for row in &rows{let ordered=ordered(properties.remove(&row.rowid).unwrap_or_default())?;for property in &ordered{roots.push(property.integer(4)?);}property_rows.insert(row.rowid,ordered);}let mut values=Owned::new(reconstruct_value_forest(database,VALUES,&roots,None,control)?);values.get_mut().reverse();
let mut nodes=Owned::new(Vec::new());for row in rows{let mut native_ports=Vec::new();for port in ordered(ports.remove(&row.rowid).unwrap_or_default())?{let kind=match port.text(4)?{"in"=>SemioGraphPortKind::In,"out"=>SemioGraphPortKind::Out,"in_out"=>SemioGraphPortKind::InOut,_=>return Err("unknown Semio graph port kind".into())};native_ports.push(SemioGraphPort{name:reconstruct_text(control,port.text(3)?)?,kind});}let mut native_properties=Owned::new(Vec::new());for property in property_rows.remove(&row.rowid).unwrap_or_default(){native_properties.get_mut().push(SemioValueEntry{key:reconstruct_text(control,property.text(3)?)?,value:values.get_mut().pop().ok_or("missing Semio graph property value")?});}nodes.get_mut().push(SemioGraphNode{id:GraphNodeId::new(reconstruct_text(control,row.text(3)?)?),kind:reconstruct_text(control,row.text(4)?)?,label:reconstruct_text(control,row.text(5)?)?,position:SemioPoint2{x:row.real(6)?,y:row.real(7)?},ports:native_ports,properties:native_properties.take()});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
let mut edges=Vec::new();let mut ids=BTreeSet::new();let mut native_ids=BTreeSet::new();for row in ordered_float_rows(database,"semio_graph_edge",2,control)?{identity(row,8)?;if row.integer(1)?!=1||!ids.insert(row.rowid)||!native_ids.insert(row.text(3)?){return Err("invalid Semio graph edge ownership or identity".into());}edges.push(SemioGraphEdge{id:GraphEdgeId::new(reconstruct_text(control,row.text(3)?)?),source:GraphNodeId::new(reconstruct_text(control,names.get(&row.integer(4)?).ok_or("dangling Semio graph source")?)?),target:GraphNodeId::new(reconstruct_text(control,names.get(&row.integer(5)?).ok_or("dangling Semio graph target")?)?),kind:reconstruct_text(control,row.text(6)?)?,label:reconstruct_text(control,row.text(7)?)?});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(Self{schema:reconstruct_text(control,document.text(1)?)?,nodes:nodes.take(),edges})
    }
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_graph_node"=>&[FloatColumn::Binary64(6),FloatColumn::Binary64(7)],_=>&[]}}
trait FloatProjection { fn insert_float(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,String>; fn insert_key_float(&mut self,table:&str,key:i64,cells:&[Cell<'_>])->Result<(),String>; }
impl FloatProjection for Projection<'_,'_> { fn insert_float(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,String>{insert_ieee754(self,table,cells,float_columns(table))} fn insert_key_float(&mut self,table:&str,key:i64,cells:&[Cell<'_>])->Result<(),String>{insert_key_ieee754(self,table,key,cells,float_columns(table))} }
fn float_rows<'a>(db:&'a SqliteDatabase,table:&str,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,String>{let table_rows=db.table(table)?;control.check_rows(table_rows.rows.len())?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,table_rows.rows.len())?;let mut result=Vec::new();for(count,row)in table_rows.rows.iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,table_rows.rows.len())?;}}Ok(result)}
fn ordered_float_rows<'a>(db:&'a SqliteDatabase,table:&str,ordinal:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,String>{let mut result=Vec::new();for(count,row)in semio_framework_os_kernel::sqlite_snapshot::artifact::ordered_row_refs(db.table(table)?,ordinal,control)?.into_iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,0)?;}}Ok(result)}
fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,String>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioGraphSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),String>{b.text(&self.schema)?;b.entities(self.nodes.len())?;for node in &self.nodes{b.text(&node.id.value)?;b.text(&node.kind)?;b.text(&node.label)?;b.scalars(2)?;b.entities(node.ports.len())?;for port in &node.ports{b.text(&port.name)?;b.scalars(1)?;}b.entities(node.properties.len())?;for property in &node.properties{b.text(&property.key)?;crate::standards::v1::subsets::value::schema::snapshot::sqlite::native_values(std::slice::from_ref(&property.value),b)?;}}b.entities(self.edges.len())?;for edge in &self.edges{b.text(&edge.id.value)?;b.text(&edge.source.value)?;b.text(&edge.target.value)?;b.text(&edge.kind)?;b.text(&edge.label)?;}Ok(())}
}
