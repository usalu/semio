//! 🕸️ Original Graph projection retains source-name ordinals and shared property tree pages.
use super::{SemioGraphSnapshot,SemioGraphPortKind,SemioValueEntry,number,float_columns,VALUES};
use crate::standards::v1::subsets::{base::io::sqlite::snapshot::projection::NameIndex,value::io::sqlite::snapshot::projection::{TreeScratch,visit_tree}};
use store::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::{Cell,RowWriter,ProjectionStorage,project_owned,owned_workspace}};
use semio_framework_value::{ValueError,ValueRefusalKind};

#[derive(semio_framework_value::RetireOwned)]
pub(crate)struct Scratch{nodes:NameIndex,edges:NameIndex,tree:TreeScratch}
impl Scratch{
 pub(crate)fn empty()->Self{Self{nodes:NameIndex::empty(),edges:NameIndex::empty(),tree:TreeScratch::empty()}}
 pub(crate)fn initialize(&mut self,snapshot:&SemioGraphSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  self.nodes.initialize(snapshot.nodes.len(),|index|snapshot.nodes[index].id.value.as_str(),phase,control,"duplicate Semio graph node identifier")?;
  self.edges.initialize(snapshot.edges.len(),|index|snapshot.edges[index].id.value.as_str(),phase,control,"duplicate Semio graph edge identifier")
 }
}

fn properties(entries:&[SemioValueEntry],table:&str,owner:i64,tree:&mut TreeScratch,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{for(ordinal,entry)in entries.iter().enumerate(){let value=visit_tree(&entry.value,VALUES,None,tree,out)?;out.insert(table,&[Cell::Integer(owner),Cell::Integer(number(ordinal)?),Cell::Text(&entry.key),Cell::Integer(value)])?;}Ok(())}

/// 🫳️ Visits every authored graph row and property through the same actual original scratch.
pub(crate)fn visit(snapshot:&SemioGraphSnapshot,scratch:&mut Scratch,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 if !scratch.nodes.ready()||!scratch.edges.ready(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"graph original name indexes are not ready"))}
 out.insert_key("semio_graph_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,node)in snapshot.nodes.iter().enumerate(){
  let id=out.insert_float("semio_graph_node",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&node.id.value),Cell::Text(&node.kind),Cell::Text(&node.label),Cell::Real(node.position.x),Cell::Real(node.position.y),Cell::Real(node.width),Cell::Real(node.height)],float_columns("semio_graph_node"))?;
  for(ordinal,port)in node.ports.iter().enumerate(){let port_id=out.insert("semio_graph_port",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&port.name),Cell::Text(match port.kind{SemioGraphPortKind::In=>"in",SemioGraphPortKind::Out=>"out",SemioGraphPortKind::InOut=>"in_out"}),Cell::Text(&port.category)])?;properties(&port.properties,"semio_graph_port_property",port_id,&mut scratch.tree,out)?;}
  properties(&node.properties,"semio_graph_property",id,&mut scratch.tree,out)?;
 }
 for(ordinal,edge)in snapshot.edges.iter().enumerate(){
  let source=scratch.nodes.lookup(&edge.source.value,|index|snapshot.nodes[index].id.value.as_str(),out,"dangling Semio graph edge node")?;let target=scratch.nodes.lookup(&edge.target.value,|index|snapshot.nodes[index].id.value.as_str(),out,"dangling Semio graph edge node")?;
  let id=out.insert("semio_graph_edge",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&edge.id.value),Cell::Integer(source),Cell::Integer(target),Cell::Text(&edge.kind),Cell::Text(&edge.label),edge.source_port.as_deref().map(Cell::Text).unwrap_or(Cell::Null),edge.target_port.as_deref().map(Cell::Text).unwrap_or(Cell::Null)])?;
  properties(&edge.properties,"semio_graph_edge_property",id,&mut scratch.tree,out)?;
 }
 Ok(())
}

#[derive(semio_framework_value::RetireOwned)]
struct Prefix{projection:ProjectionStorage,scratch:Scratch}

/// 🧮️ Preserves all original graph fields and scalar scratch across census, output and cancellation.
pub(super)fn project(snapshot:&SemioGraphSnapshot,sql:&str,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
 project_owned(control,||Prefix{projection:ProjectionStorage::empty(),scratch:Scratch::empty()},|prefix,control|{
  prefix.scratch.initialize(snapshot,SqliteSnapshotPhase::ProjectSnapshot,control)?;
  let mut census=RowWriter::census(&mut prefix.projection,sql,control)?;visit(snapshot,&mut prefix.scratch,&mut census)?;census.finish_census()?;
  let mut output=RowWriter::new_into(&mut prefix.projection,control)?;visit(snapshot,&mut prefix.scratch,&mut output)?;output.finish_into()
 },|prefix|prefix.projection.take_database())
}

/// 🎟️ Retains the same genuine graph scratch during native admission in the exact caller phase.
pub(super)fn admit(snapshot:&SemioGraphSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 owned_workspace(control,phase,Scratch::empty,|scratch,control|{
  scratch.initialize(snapshot,phase,control)?;let mut writer=RowWriter::borrowed(control,phase)?;visit(snapshot,scratch,&mut writer)?;writer.finish_borrowed()
 },|_|())
}
