//! 🌊️ Original Flow projection retains only scalar source ordinals and complete SQL ownership.
use super::{SemioFlowSnapshot,number,float_columns};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::projection::NameIndex;
use store::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::{Cell,ProjectionStorage,RowWriter,project_owned,owned_workspace}};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}

/// 🗂️ Scalar addresses borrow actual authored names only while comparing or resolving them.
#[derive(semio_framework_value::RetireOwned)]
pub(crate)struct Scratch{nodes:NameIndex,edges:NameIndex}
impl Scratch{
 pub(crate)fn empty()->Self{Self{nodes:NameIndex::empty(),edges:NameIndex::empty()}}
 pub(crate)fn initialize(&mut self,snapshot:&SemioFlowSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  self.nodes.initialize(snapshot.nodes.len(),|index|snapshot.nodes[index].id.as_str(),phase,control,"duplicate Semio flow node identifier")?;
  self.edges.initialize(snapshot.edges.len(),|index|snapshot.edges[index].id.as_str(),phase,control,"duplicate Semio flow edge identifier")
 }
 fn reference(&self,snapshot:&SemioFlowSnapshot,id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{self.nodes.lookup(id,|index|snapshot.nodes[index].id.as_str(),out,"dangling Semio flow edge node")}
}
/// 🫳️ Emits the same authored cells in census and output while every source index remains original.
pub(crate)fn visit(snapshot:&SemioFlowSnapshot,scratch:&Scratch,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 if !scratch.nodes.ready()||!scratch.edges.ready(){return Err(invalid("Flow projection scratch is not ready"))}
 out.insert_key("semio_flow_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,node)in snapshot.nodes.iter().enumerate(){let id=out.insert_float("semio_flow_node",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&node.id),Cell::Text(&node.kind),Cell::Text(&node.label),Cell::Real(node.position.x),Cell::Real(node.position.y)],float_columns("semio_flow_node"))?;for(ordinal,param)in node.params.iter().enumerate(){out.insert("semio_flow_parameter",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&param.key),Cell::Text(&param.value)])?;}}
 for(ordinal,edge)in snapshot.edges.iter().enumerate(){let source=scratch.reference(snapshot,&edge.from.node,out)?;let target=scratch.reference(snapshot,&edge.to.node,out)?;out.insert("semio_flow_edge",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&edge.id),Cell::Integer(source),Cell::Text(&edge.from.port),Cell::Integer(target),Cell::Text(&edge.to.port),Cell::Text(&edge.kind)])?;}
 Ok(())
}

#[derive(semio_framework_value::RetireOwned)]
struct Prefix{projection:ProjectionStorage,scratch:Scratch}

/// 🎟️ Retains the same original source ordinals during native semantic admission in the caller's actual phase.
pub(super)fn admit(snapshot:&SemioFlowSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 owned_workspace(control,phase,Scratch::empty,|scratch,control|{
  scratch.initialize(snapshot,phase,control)?;let mut writer=RowWriter::borrowed(control,phase)?;visit(snapshot,scratch,&mut writer)?;writer.finish_borrowed()
 },|_|())
}

/// 🧮️ Admits the actual complete original prefix before any schema, name index or row construction.
pub(super)fn project(snapshot:&SemioFlowSnapshot,sql:&str,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
 project_owned(control,||Prefix{projection:ProjectionStorage::empty(),scratch:Scratch::empty()},|prefix,control|{
  prefix.scratch.initialize(snapshot,SqliteSnapshotPhase::ProjectSnapshot,control)?;
  let mut census=RowWriter::census(&mut prefix.projection,sql,control)?;visit(snapshot,&prefix.scratch,&mut census)?;census.finish_census()?;
  let mut output=RowWriter::new_into(&mut prefix.projection,control)?;visit(snapshot,&prefix.scratch,&mut output)?;output.finish_into()
 },|prefix|prefix.projection.take_database())
}
