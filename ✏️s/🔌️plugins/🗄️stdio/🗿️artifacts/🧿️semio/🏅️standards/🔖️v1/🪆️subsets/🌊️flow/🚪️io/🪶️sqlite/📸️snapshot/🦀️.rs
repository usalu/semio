//! 🌊️ Named flow nodes, ordered parameters and port-addressed edge entities.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow};
use crate::standards::v1::subsets::flow::schema::snapshot::{SemioFlowSnapshot,FlowNode,FlowEdge,FlowParam,PortRef};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase}};
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
/// 🫳️ Visits the exact Flow entities through the caller's owned or borrowed row writer.
pub(crate)fn visit_rows(snapshot:&SemioFlowSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let phase=out.phase();let mut nodes=out.allocate_frontier(snapshot.nodes.len())?;
 for(ordinal,node)in snapshot.nodes.iter().enumerate(){out.checkpoint()?;nodes.push((node.id.as_str(),number(ordinal+1)?));}
 out.sort_frontier(&mut nodes,|a,b,c|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a.0,b.0,phase,c))?;
 for pair in nodes.windows(2){if out.compare_text(pair[0].0,pair[1].0)?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio flow node identifier"))}}
 let mut ids=out.allocate_frontier(snapshot.edges.len())?;for edge in &snapshot.edges{out.checkpoint()?;ids.push(edge.id.as_str());}
 out.sort_frontier(&mut ids,|a,b,c|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a,b,phase,c))?;
 for pair in ids.windows(2){if out.compare_text(pair[0],pair[1])?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio flow edge identifier"))}}
 out.insert_key("semio_flow_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,node)in snapshot.nodes.iter().enumerate(){
  let id=out.insert_float("semio_flow_node",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&node.id),Cell::Text(&node.kind),Cell::Text(&node.label),Cell::Real(node.position.x),Cell::Real(node.position.y)],float_columns("semio_flow_node"))?;
  for(ordinal,param)in node.params.iter().enumerate(){out.insert("semio_flow_parameter",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&param.key),Cell::Text(&param.value)])?;}
 }
 for(ordinal,edge)in snapshot.edges.iter().enumerate(){
  let source=node_reference(&nodes,&edge.from.node,out)?;let target=node_reference(&nodes,&edge.to.node,out)?;
  out.insert("semio_flow_edge",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&edge.id),Cell::Integer(source),Cell::Text(&edge.from.port),Cell::Integer(target),Cell::Text(&edge.to.port),Cell::Text(&edge.kind)])?;
 }Ok(())
}
/// 🔎️ Resolves the actual sorted borrowed node frontier with bounded literal comparisons.
fn node_reference(nodes:&[(&str,i64)],id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 let mut lo=0;let mut hi=nodes.len();while lo<hi{let mid=lo+(hi-lo)/2;match out.compare_text(nodes[mid].0,id)?{std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid,std::cmp::Ordering::Equal=>return Ok(nodes[mid].1)}}Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio flow edge node"))
}
/// 🎟️ Admits complete typed cells before native forecasting or materialization.
pub(crate)fn admit_values(snapshot:&SemioFlowSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
/// 🏛️ Admits the exact authored table and column layout before native ownership.
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts actual binary primitive cells before allocating typed fields.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📝️ Counts actual document primitive cells before allocating typed fields.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),ValueError>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio flow row identity or columns"))}else{Ok(())}}
impl ArtifactSqliteSnapshot for SemioFlowSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{crate::standards::v1::subsets::flow::io::sqlite::snapshot::native_encoding::encode(self,encoding,control)}
fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::flow::io::sqlite::snapshot::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="flow"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_flow_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {let result=(||->Result<Self,ValueError>{ semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) })();result}
}

impl SemioFlowSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{
 use semio_framework_os_kernel::sqlite_snapshot::{artifact::RowIndex,transfer::reserve};
 use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
 control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;semio_framework_os_kernel::sqlite_snapshot::validate_sqlite_database_schema_controlled(database,declared_schema,SqliteSnapshotPhase::ReconstructSnapshot,control)?;
 let document=single_float_row(database,"semio_flow_document")?;identity(document,2)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio flow document identifier"))}
 let nodes=RowIndex::new(database,"semio_flow_node",8,float_columns("semio_flow_node"),control,"invalid Semio flow node ownership or identity")?;
 let parameters=RowIndex::new(database,"semio_flow_parameter",5,float_columns("semio_flow_parameter"),control,"invalid Semio flow parameter owner or identity")?;
 let edges=RowIndex::new(database,"semio_flow_edge",9,float_columns("semio_flow_edge"),control,"invalid Semio flow edge ownership or identity")?;
 let order=nodes.ordered(2,control,"Semio flow node ordinals must be contiguous")?;let edge_order=edges.ordered(2,control,"Semio flow edge ordinals must be contiguous")?;
 nodes.unique_text(nodes.indices(),3,control,"invalid Semio flow node ownership or identity")?;edges.unique_text(edges.indices(),3,control,"invalid Semio flow edge ownership or identity")?;
 for(count,&index)in nodes.indices().iter().enumerate(){if nodes.row(index)?.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio flow node ownership or identity"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,nodes.len())?;}
 for(count,&index)in parameters.indices().iter().enumerate(){let row=parameters.row(index)?;if nodes.get(row.integer(1)?,control)?.is_none(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio flow parameter owner or identity"))}row.integer(2)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,parameters.len())?;}
 let grouped=parameters.grouped_by(2,control,"Semio flow parameter ordinals must be contiguous",|row|Ok((0,Some(row.integer(1)?))))?;
 let mut snapshot=Owned::new(Self{schema:String::new(),nodes:Vec::new(),edges:Vec::new()});snapshot.get_mut().nodes=reserve(order.len(),control)?;snapshot.get_mut().edges=reserve(edge_order.len(),control)?;let mut completed=0;
 for index in order{
  let row=nodes.row(index)?;let range=parameters.range_by(&grouped,(0,Some(row.rowid)),control,|row|Ok((0,Some(row.integer(1)?))))?;
  let position=SemioPoint2{x:row.real(6)?,y:row.real(7)?};let mut node=Owned::new(FlowNode{id:String::new(),kind:String::new(),label:String::new(),position,params:Vec::new()});node.get_mut().params=reserve(range.len(),control)?;
  node.get_mut().id=reconstruct_text(control,row.text(3)?)?;node.get_mut().kind=reconstruct_text(control,row.text(4)?)?;node.get_mut().label=reconstruct_text(control,row.text(5)?)?;
  for index in range{
   let row=parameters.row(grouped[index])?;let mut param=Owned::new(FlowParam{key:String::new(),value:String::new()});param.get_mut().key=reconstruct_text(control,row.text(3)?)?;param.get_mut().value=reconstruct_text(control,row.text(4)?)?;node.get_mut().params.push(param.take());completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
  }
  snapshot.get_mut().nodes.push(node.take());completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
 }
 for index in edge_order{
  let row=edges.row(index)?;if row.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio flow edge ownership or identity"))}
  let from=nodes.get(row.integer(4)?,control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio flow source node"))?;let to=nodes.get(row.integer(6)?,control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio flow target node"))?;
  let mut edge=Owned::new(FlowEdge{id:String::new(),from:PortRef{node:String::new(),port:String::new()},to:PortRef{node:String::new(),port:String::new()},kind:String::new()});
  edge.get_mut().id=reconstruct_text(control,row.text(3)?)?;edge.get_mut().from.node=reconstruct_text(control,from.text(3)?)?;edge.get_mut().from.port=reconstruct_text(control,row.text(5)?)?;edge.get_mut().to.node=reconstruct_text(control,to.text(3)?)?;edge.get_mut().to.port=reconstruct_text(control,row.text(7)?)?;edge.get_mut().kind=reconstruct_text(control,row.text(8)?)?;
  snapshot.get_mut().edges.push(edge.take());completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
 }
 snapshot.get_mut().schema=reconstruct_text(control,document.text(1)?)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(snapshot.take())
}
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_flow_node"=>&[FloatColumn::Binary64(6),FloatColumn::Binary64(7)],_=>&[]}}


fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioFlowSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.nodes.len())?;for node in &self.nodes{b.text(&node.id)?;b.text(&node.kind)?;b.text(&node.label)?;b.scalars(2)?;b.entities(node.params.len())?;for param in &node.params{b.text(&param.key)?;b.text(&param.value)?;}}b.entities(self.edges.len())?;for edge in &self.edges{b.text(&edge.id)?;b.text(&edge.from.node)?;b.text(&edge.from.port)?;b.text(&edge.to.node)?;b.text(&edge.to.port)?;b.text(&edge.kind)?;}Ok(())}
}

impl SemioFlowSnapshot{
/// 🧮️ Projects owned semantic rows under the caller's typed resource control.
pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
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
