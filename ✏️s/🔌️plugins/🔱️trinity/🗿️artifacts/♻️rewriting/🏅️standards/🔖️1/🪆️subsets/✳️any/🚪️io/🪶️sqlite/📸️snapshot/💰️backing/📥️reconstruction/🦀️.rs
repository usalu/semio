//! 🪆️ full typed parent reconstruction; the composed Semio child remains a literal handle.
use super::{indexes::{Rows,invalid},properties::{Reader,retire}};
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;
use crate::standards::v1::subsets::any::schema::model::LayoutPoint;
use crate::standards::v1::subsets::any::schema::rule::{Pattern, Lhs, Rhs, Assignment, ParameterSpec, ParameterKind};
use crate::standards::v1::subsets::any::schema::layout::RuleLayout;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::content::JackContentChild;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::graph::Camera;
use semio_framework_graph_manifest::PropertyBag;
use semio_framework_value::{ValueError,DecodedValue};
use semio_os_kernel::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,transfer,artifact::{FloatColumn,read_binary64}};
fn pattern(id:i64,read:&mut Reader<'_,'_,'_>)->Result<Pattern,ValueError>{
 let row=read.rows.take(11,id,read.control)?;
 Ok(Pattern{left_var:read.text(row,1)?,left_kind:read.text(row,2)?,edge_var:read.optional(row,3)?,edge_kind:read.optional(row,4)?,right_var:read.optional(row,5)?,right_kind:read.optional(row,6)?})
}
fn jack(id:i64,read:&mut Reader<'_,'_,'_>)->Result<JackSnapshot,ValueError>{
 let row=read.rows.take(17,id,read.control)?;
 let schema=read.text(row,1)?;let name=read.text(row,2)?;let manifest_id=read.text(row,3)?;let root_node_id=read.optional(row,4)?;let query=read.text(row,5)?;
 if schema!=JackSnapshot::SCHEMA{return Err(invalid("Jack document schema identity"))}
 let camera=read.rows.body(18,id,read.control)?;
 let floats=&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4)];
 let camera=Camera{x:read_binary64(camera,2,floats)?,y:read_binary64(camera,3,floats)?,zoom:read_binary64(camera,4,floats)?};read.scalar()?;read.scalar()?;read.scalar()?;
 let child=read.rows.body(19,id,read.control)?;
 let child_id=read.text(child,2)?;let artifact_id=read.text(child,3)?;let artifact_kind=read.text(child,4)?;let standard=read.text(child,5)?;let subset=read.text(child,6)?;
 if artifact_kind!="s.stdio.semio"||standard!="v1"||subset!="graph"{return Err(invalid("Jack child requires its exact Semio graph dialect"))}
 let content=JackContentChild::new(child_id,semio_framework_artifact_reference::ArtifactRef{artifact_id,dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind,standard,subset}});
 Ok(JackSnapshot{schema,name,manifest_id,camera,content,root_node_id,query})
}
pub(in super::super) fn reconstruct(database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<RewritingSnapshot,ValueError>{
 let rows=Rows::new(database,c)?;let mut read=Reader::new(rows,c);
 let document=read.rows.root(0,read.control)?;let document_id=document.rowid;
 let graph=DecodedValue::new(jack(document.integer(1)?,&mut read)?,retire);
 let lhs=read.rows.body(9,document_id,read.control)?;let lhs=DecodedValue::new(Lhs{pattern:pattern(lhs.integer(2)?,&mut read)?,where_clause:read.optional(lhs,3)?},retire);
 let rhs_id=read.rows.body(10,document_id,read.control)?.rowid;
 let mut rhs=DecodedValue::new(Rhs::default(),retire);
 for(table,is_merge)in[(12,false),(13,true)]{
  let rows=read.rows.children(table,rhs_id,true,read.control)?;let mut values=transfer::reserve(rows.len(),read.control)?;
  for row in rows{values.push(pattern(row.integer(3)?,&mut read)?);}
  if is_merge{rhs.get_mut().merge=values}else{rhs.get_mut().create=values}
 }
 let rows=read.rows.children(14,rhs_id,true,read.control)?;rhs.get_mut().delete=transfer::reserve(rows.len(),read.control)?;for row in rows{rhs.get_mut().delete.push(read.text(row,3)?);}
 let rows=read.rows.children(15,rhs_id,true,read.control)?;rhs.get_mut().set=transfer::reserve(rows.len(),read.control)?;
 for row in rows{let var=read.text(row,3)?;let prop=read.text(row,4)?;let value=DecodedValue::new(read.property(row.integer(5)?)?,retire);rhs.get_mut().set.push(Assignment{var,prop,value:value.take()});}
 let rows=read.rows.children(16,rhs_id,true,read.control)?;rhs.get_mut().parameters=transfer::reserve(rows.len(),read.control)?;
 for row in rows{let name=read.text(row,3)?;let kind=match row.text(4)?{"string"=>ParameterKind::String,"number"=>ParameterKind::Number,"boolean"=>ParameterKind::Boolean,_=>return Err(invalid("Rewriting parameter kind is undeclared"))};let default=DecodedValue::new(read.property(row.integer(5)?)?,retire);rhs.get_mut().parameters.push(ParameterSpec{name,kind,default:default.take()});}
 let rows=read.rows.children(3,document_id,false,read.control)?;
 let mut bindings=DecodedValue::new(transfer::reserve(rows.len(),read.control)?,retire);
 for row in rows{let key=read.text(row,2)?;let value=DecodedValue::new(read.property(row.integer(3)?)?,retire);bindings.get_mut().push((key,value.take()));}
 transfer::heap_sort(bindings.get_mut(),SqliteSnapshotPhase::ReconstructSnapshot,read.control,|a,b,c|transfer::compare_text(&a.0,&b.0,SqliteSnapshotPhase::ReconstructSnapshot,c))?;
 if bindings.get().windows(2).any(|pair|pair[0].0==pair[1].0){return Err(invalid("Rewriting parameter binding keys must be unique"))}
 let rows=read.rows.children(1,document_id,false,read.control)?;
 let mut layout=transfer::reserve(rows.len(),read.control)?;
 for row in rows{let key=read.text(row,2)?;let columns=&[FloatColumn::Binary64(3),FloatColumn::Binary64(4)];let point=LayoutPoint{x:read_binary64(row,3,columns)?,y:read_binary64(row,4,columns)?};read.scalar()?;read.scalar()?;layout.push((key,point));}
 transfer::heap_sort(&mut layout,SqliteSnapshotPhase::ReconstructSnapshot,read.control,|a,b,c|transfer::compare_text(&a.0,&b.0,SqliteSnapshotPhase::ReconstructSnapshot,c))?;
 if layout.windows(2).any(|pair|pair[0].0==pair[1].0){return Err(invalid("Rewriting layout keys must be unique"))}
 let value=DecodedValue::new(RewritingSnapshot{working_graph:graph.take(),lhs:lhs.take(),rhs:rhs.take(),parameter_bindings:PropertyBag::from_admitted(bindings.take()),rule_layout:RuleLayout::from_admitted(layout)},retire);
 read.rows.finish(read.control)?;Ok(value.take())
}
