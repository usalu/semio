//! 🔢️ Original Value projection owns graph ordinals and shared scalar tree frames through every result.
use super::{SemioValue,SemioValueSnapshot,ValueSqliteTables,VALUE_TABLES,number};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::projection::NameIndex;
use store::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::{Cell,ProjectionStorage,RowWriter,project_owned,owned_workspace}};
use semio_framework_value::{ValueError,ValueRefusalKind};
#[path="🌳️tree/🦀️.rs"]mod tree;
pub(crate)use tree::TreeScratch;
pub(crate)use tree::visit as visit_tree;
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}

/// 🔗️ Resolves an actual graph identity while keeping all source bytes borrowed.
pub(crate)trait References{fn lookup(&self,id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>;}

#[derive(semio_framework_value::RetireOwned)]
pub(crate)struct Scratch{nodes:NameIndex,tree:TreeScratch}
impl Scratch{
 pub(crate)fn empty()->Self{Self{nodes:NameIndex::empty(),tree:TreeScratch::empty()}}
 pub(crate)fn initialize(&mut self,snapshot:&SemioValueSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{self.nodes.initialize(snapshot.nodes.len(),|index|snapshot.nodes[index].id.value.as_str(),phase,control,"duplicate Semio value node identifier")}
}

struct Names<'a>{snapshot:&'a SemioValueSnapshot,index:&'a NameIndex}
impl References for Names<'_>{
 fn lookup(&self,id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{self.index.lookup(id,|index|self.snapshot.nodes[index].id.value.as_str(),out,"dangling Semio value reference")}
}
/// 🌳️ Visits the same complete graph using original scalar indexes and original paged traversal backing.
pub(crate)fn visit(snapshot:&SemioValueSnapshot,scratch:&mut Scratch,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 if !scratch.nodes.ready(){return Err(invalid("value projection graph scratch is not ready"))}
 let names=Names{snapshot,index:&scratch.nodes};let root=visit_tree(&snapshot.root,VALUE_TABLES,Some(&names),&mut scratch.tree,out)?;out.insert_key("semio_value_document",1,&[Cell::Text(&snapshot.schema),Cell::Integer(root)])?;
 for(ordinal,node)in snapshot.nodes.iter().enumerate(){let value=visit_tree(&node.value,VALUE_TABLES,Some(&names),&mut scratch.tree,out)?;out.insert_key("semio_value_node",number(ordinal+1)?,&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&node.id.value),Cell::Integer(value)])?;}
 Ok(())
}

#[derive(semio_framework_value::RetireOwned)]
struct Prefix{projection:ProjectionStorage,scratch:Scratch}

/// 🧮️ Captures every actual graph, schema and tree allocation before its first cancellable operation.
pub(super)fn project(snapshot:&SemioValueSnapshot,sql:&str,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
 project_owned(control,||Prefix{projection:ProjectionStorage::empty(),scratch:Scratch::empty()},|prefix,control|{
  prefix.scratch.initialize(snapshot,SqliteSnapshotPhase::ProjectSnapshot,control)?;
  let mut census=RowWriter::census(&mut prefix.projection,sql,control)?;visit(snapshot,&mut prefix.scratch,&mut census)?;census.finish_census()?;
  let mut output=RowWriter::new_into(&mut prefix.projection,control)?;visit(snapshot,&mut prefix.scratch,&mut output)?;output.finish_into()
 },|prefix|prefix.projection.take_database())
}

/// 🎟️ Keeps the same actual graph and tree scratch while native admission uses its original caller phase.
pub(super)fn admit(snapshot:&SemioValueSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 owned_workspace(control,phase,Scratch::empty,|scratch,control|{
  scratch.initialize(snapshot,phase,control)?;let mut writer=RowWriter::borrowed(control,phase)?;visit(snapshot,scratch,&mut writer)?;writer.finish_borrowed()
 },|_|())
}
