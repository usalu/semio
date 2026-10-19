//! 🕸️ Named graph nodes, ports, edges and explicitly typed property values.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow};
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::Owned;
use crate::schema::snapshot::{SemioGraphSnapshot, SemioGraphNode, SemioGraphEdge, SemioGraphPort, SemioGraphPortKind};
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueEntry;
use semio_s_artifact_stdio_semio_value_model::io::sqlite::snapshot::project_value_tree;
use semio_s_artifact_stdio_semio_value_model::io::sqlite::snapshot::ValueSqliteTables;
use semio_framework_os_kernel::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{artifact::{Cell,RowWriter},SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase}};
use semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text;
const VALUES:ValueSqliteTables=ValueSqliteTables{value:"semio_graph_value",list_element:"semio_graph_list_element",map_entry:"semio_graph_map_entry"};
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
#[path="🫙️projection/🦀️.rs"]
pub(crate)mod projection;
#[path="💰️reconstruction/🦀️.rs"]
mod reconstruction;
/// 🫳️ Visits the exact Graph and property forest through one owned or borrowed writer.
pub fn visit_rows(snapshot:&SemioGraphSnapshot,projection:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 projection.check_rows(snapshot.nodes.len().checked_add(snapshot.edges.len()).and_then(|count|count.checked_add(1)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio graph row count overflow"))?)?;
 let phase=projection.phase();
 let mut nodes=projection.allocate_frontier(snapshot.nodes.len())?;
 for(ordinal,node)in snapshot.nodes.iter().enumerate(){projection.checkpoint()?;nodes.push((node.id.value.as_str(),number(ordinal+1)?));}
 projection.sort_frontier(&mut nodes,|a,b,c|compare_text(a.0,b.0,phase,c))?;projection.checkpoint()?;
 for pair in nodes.windows(2){if projection.compare_text(pair[0].0,pair[1].0)?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio graph node identifier"));}}
 let mut edges=projection.allocate_frontier(snapshot.edges.len())?;for edge in &snapshot.edges{projection.checkpoint()?;edges.push(edge.id.value.as_str());}projection.sort_frontier(&mut edges,|a,b,c|compare_text(a,b,phase,c))?;projection.checkpoint()?;
 for pair in edges.windows(2){if projection.compare_text(pair[0],pair[1])?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio graph edge identifier"));}}
 projection.insert_key("semio_graph_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,node)in snapshot.nodes.iter().enumerate(){
  let id=projection.insert_float("semio_graph_node",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&node.id.value),Cell::Text(&node.kind),Cell::Text(&node.label),Cell::Real(node.position.x),Cell::Real(node.position.y),Cell::Real(node.width),Cell::Real(node.height)],float_columns("semio_graph_node"))?;
  for(ordinal,port)in node.ports.iter().enumerate(){let port_id=projection.insert("semio_graph_port",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&port.name),Cell::Text(match port.kind{SemioGraphPortKind::In=>"in",SemioGraphPortKind::Out=>"out",SemioGraphPortKind::InOut=>"in_out"}),Cell::Text(&port.category)])?;project_properties(&port.properties,"semio_graph_port_property",port_id,projection)?;}
  project_properties(&node.properties,"semio_graph_property",id,projection)?;
 }
 for(ordinal,edge)in snapshot.edges.iter().enumerate(){
  let source=node_reference(&nodes,edge.source.value.as_str(),projection)?;
  let target=node_reference(&nodes,edge.target.value.as_str(),projection)?;
  let id=projection.insert("semio_graph_edge",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&edge.id.value),Cell::Integer(source),Cell::Integer(target),Cell::Text(&edge.kind),Cell::Text(&edge.label),edge.source_port.as_deref().map(Cell::Text).unwrap_or(Cell::Null),edge.target_port.as_deref().map(Cell::Text).unwrap_or(Cell::Null)])?;
  project_properties(&edge.properties,"semio_graph_edge_property",id,projection)?;
 }
 Ok(())
}
/// 🔎️ Resolves the actual sorted node frontier through bounded full literal comparisons.
fn node_reference(nodes:&[(&str,i64)],id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{let mut lo=0;let mut hi=nodes.len();while lo<hi{let mid=lo+(hi-lo)/2;match out.compare_text(nodes[mid].0,id)?{std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid,std::cmp::Ordering::Equal=>return Ok(nodes[mid].1)}}Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio graph edge node"))}
/// 🎟️ Admits every typed Graph cell before native forecasting or materialization.
pub(crate)fn admit_values(snapshot:&SemioGraphSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;projection::admit(snapshot,phase,control)}
/// 🏛️ Admits every authored table and companion column before native ownership.
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts the original Graph binary grammar before typed construction.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📝️ Counts the original Graph document grammar before typed construction.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),ValueError>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio graph row identity or columns"))}else{Ok(())}}
impl ArtifactSqliteSnapshotHost for SemioGraphSnapshot{
fn retire_sqlite_snapshot(self){drop(Owned::new(self));}

fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{native_encoding::encode(self,encoding,control,native_owner)}

 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{native_decoding::decode(payload,control,native_control.native())}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="graph"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_graph_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}




}
impl pack::snapshot::ArtifactSqliteSnapshot for SemioGraphSnapshot {
const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {let result=(||->Result<Self,ValueError>{ semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) })();result}
}

impl SemioGraphSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{reconstruction::reconstruct(database,control,declared_schema)}
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_graph_node"=>&[FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9)],_=>&[]}}
fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioGraphSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.nodes.len())?;for node in &self.nodes{b.text(&node.id.value)?;b.text(&node.kind)?;b.text(&node.label)?;b.scalars(4)?;b.entities(node.ports.len())?;for port in &node.ports{b.text(&port.name)?;b.scalars(1)?;b.text(&port.category)?;b.entities(port.properties.len())?;for property in &port.properties{b.text(&property.key)?;semio_s_artifact_stdio_semio_value_model::io::sqlite::snapshot::native_values(std::slice::from_ref(&property.value),b)?;}}b.entities(node.properties.len())?;for property in &node.properties{b.text(&property.key)?;semio_s_artifact_stdio_semio_value_model::io::sqlite::snapshot::native_values(std::slice::from_ref(&property.value),b)?;}}b.entities(self.edges.len())?;for edge in &self.edges{b.text(&edge.id.value)?;b.text(&edge.source.value)?;b.text(&edge.target.value)?;b.text(&edge.kind)?;b.text(&edge.label)?;b.scalars(2)?;if let Some(value)=&edge.source_port{b.text(value)?;}if let Some(value)=&edge.target_port{b.text(value)?;}b.entities(edge.properties.len())?;for property in &edge.properties{b.text(&property.key)?;semio_s_artifact_stdio_semio_value_model::io::sqlite::snapshot::native_values(std::slice::from_ref(&property.value),b)?;}}Ok(())}
}

impl SemioGraphSnapshot{
/// 🧮️ Projects full node, port and edge state with paid literal reference indexes.
pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
semantic::layout(control.limits())?;projection::project(self,Self::SQLITE_SCHEMA,control)
}
}
fn project_properties(properties:&[SemioValueEntry],table:&str,owner:i64,projection:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 for(ordinal,property)in properties.iter().enumerate(){let value_id=project_value_tree(&property.value,VALUES,None,projection)?;projection.insert(table,&[Cell::Integer(owner),Cell::Integer(number(ordinal)?),Cell::Text(&property.key),Cell::Integer(value_id)])?;}Ok(())
}



#[path = "🛫️native/🦀️.rs"]
pub mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub mod native_decoding;
