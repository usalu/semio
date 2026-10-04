//! ♻️ full thirty-table owner projection with paid borrowed frontiers.
use crate::{LayoutPoint, RewritingSnapshot};
use crate::standards::v1::subsets::any::schema::{Pattern,ParameterKind};
use semio_s_artifact_trinity_jack::{JackSnapshot,PropertyDef,PropertyKind,PortDirection};
use semio_framework_graph::manifest::PropertyValue;
use semio_framework_value::{ValueError,ValueRefusalKind,ValueType};
use semio_framework_os_kernel::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,transfer,artifact::{Projection,Cell,FloatColumn,insert_ieee754}};
const SQL:&str=include_str!("../../🗄️.sql");
fn limit(message:&str)->ValueError{ValueError::new(ValueRefusalKind::WorkLimit,message)}
fn invariant(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvariantViolated,message)}
fn ordinal(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|_|limit("Rewriting ordinal exceeds signed SQLite range"))}
fn optional(value:&Option<String>)->Cell<'_>{value.as_deref().map(Cell::Text).unwrap_or(Cell::Null)}
fn add(total:&mut usize,count:usize,c:&SqliteSnapshotControl<'_>)->Result<(),ValueError>{*total=total.checked_add(count).ok_or_else(||limit("Rewriting row count overflow"))?;c.check_rows(*total)}
fn push<'a>(values:&mut Vec<&'a PropertyValue>,value:&'a PropertyValue,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 if values.len()==values.capacity(){let count=values.capacity().checked_mul(2).and_then(|n|n.checked_add(1)).ok_or_else(||limit("Rewriting frontier count overflow"))?;let mut next=transfer::reserve(count,c)?;next.append(values);*values=next;}
 values.push(value);Ok(())
}
fn property_rows(value:&PropertyValue,total:&mut usize,c:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{
 let mut stack=transfer::reserve(1,c)?;stack.push(value);
 while let Some(value)=stack.pop(){
  add(total,match value{PropertyValue::Null=>1,PropertyValue::Bool(_)|PropertyValue::Number(_)|PropertyValue::String(_)=>2,PropertyValue::Array(values)=>values.len().checked_add(1).ok_or_else(||limit("Rewriting array count overflow"))?,PropertyValue::Object(values)=>values.len().checked_add(1).ok_or_else(||limit("Rewriting object count overflow"))?},c)?;
  match value{PropertyValue::Array(values)=>for value in values{push(&mut stack,value,c)?;},PropertyValue::Object(values)=>for value in values.values(){push(&mut stack,value,c)?;},_=>{}}
  c.checkpoint(phase,*total,0)?;
 }
 Ok(())
}
fn declaration_rows(values:&[PropertyDef],total:&mut usize,c:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{
 for value in values{
  add(total,1,c)?;let mut ty=&value.value_type;
  loop{add(total,1,c)?;match ty{ValueType::List(inner)=>{add(total,1,c)?;ty=inner},ValueType::Schema(_)=>{add(total,1,c)?;break},_=>break}c.checkpoint(phase,*total,0)?;}
 }
 Ok(())
}
pub(super) fn forecast(value:&RewritingSnapshot,c:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<usize,ValueError>{
 c.checkpoint(phase,0,0)?;let mut total=7; c.check_rows(total)?;
 add(&mut total,value.rule_layout.len(),c)?;add(&mut total,value.parameter_bindings.len(),c)?;
 for property in value.parameter_bindings.values(){property_rows(property,&mut total,c,phase)?;}
 for pattern in value.rhs.create.iter().chain(&value.rhs.merge){let _=pattern;add(&mut total,2,c)?;}
 add(&mut total,value.rhs.delete.len(),c)?;add(&mut total,value.rhs.set.len(),c)?;add(&mut total,value.rhs.parameters.len(),c)?;
 for value in &value.rhs.set{property_rows(&value.value,&mut total,c,phase)?;}
 for value in &value.rhs.parameters{property_rows(&value.default,&mut total,c,phase)?;}
 for kind in &value.working_graph.manifest.node_kinds{add(&mut total,1,c)?;add(&mut total,kind.port_kinds.len(),c)?;declaration_rows(&kind.properties,&mut total,c,phase)?;}
 for kind in &value.working_graph.manifest.edge_kinds{add(&mut total,1,c)?;declaration_rows(&kind.properties,&mut total,c,phase)?;}
 for kind in &value.working_graph.manifest.port_kinds{add(&mut total,1,c)?;declaration_rows(&kind.properties,&mut total,c,phase)?;}
 c.checkpoint(phase,total,total)?;Ok(total)
}
enum Edge<'a>{Array(i64,usize),Object(i64,usize,&'a str)}
fn property(value:&PropertyValue,out:&mut Projection<'_,'_>,total:usize)->Result<i64,ValueError>{
 let mut stack=out.allocate_frontier(1)?;stack.push((None,value));let mut root=None;
 while let Some((edge,value))=stack.pop(){
  let variant=match value{PropertyValue::Null=>"null",PropertyValue::Bool(_)=>"bool",PropertyValue::Number(_)=>"number",PropertyValue::String(_)=>"string",PropertyValue::Array(_)=>"array",PropertyValue::Object(_)=>"object"};
  let id=out.insert("rewriting_value",&[Cell::Text(variant)])?;
  match edge{None=>root=Some(id),Some(Edge::Array(parent,n))=>{out.insert("rewriting_array_element",&[Cell::Integer(parent),Cell::Integer(ordinal(n)?),Cell::Integer(id)])?;},Some(Edge::Object(parent,n,key))=>{out.insert("rewriting_object_member",&[Cell::Integer(parent),Cell::Integer(ordinal(n)?),Cell::Text(key),Cell::Integer(id)])?;}}
  match value{
   PropertyValue::Null=>{},PropertyValue::Bool(value)=>{out.insert("rewriting_boolean",&[Cell::Integer(id),Cell::Integer(i64::from(*value))])?;},
   PropertyValue::Number(value)=>{insert_ieee754(out,"rewriting_number",&[Cell::Integer(id),Cell::Real(*value)],&[FloatColumn::Binary64(2)])?;},
   PropertyValue::String(value)=>{out.insert("rewriting_string",&[Cell::Integer(id),Cell::Text(value)])?;},
   PropertyValue::Array(values)=>for(n,value)in values.iter().enumerate().rev(){out.push_frontier(&mut stack,(Some(Edge::Array(id,n)),value))?;},
   PropertyValue::Object(values)=>for(n,(key,value))in values.iter().enumerate().rev(){out.push_frontier(&mut stack,(Some(Edge::Object(id,n,key)),value))?;},
  }
  out.checkpoint_total(total)?;
 }
 root.ok_or_else(||invariant("Rewriting property walk lost its root"))
}
fn pattern(value:&Pattern,out:&mut Projection<'_,'_>)->Result<i64,ValueError>{out.insert("rewriting_pattern",&[Cell::Text(&value.left_var),Cell::Text(&value.left_kind),optional(&value.edge_var),optional(&value.edge_kind),optional(&value.right_var),optional(&value.right_kind)])}
fn project_type(mut value:&ValueType,out:&mut Projection<'_,'_>,total:usize)->Result<i64,ValueError>{
 let mut root=None;let mut parent=None;
 loop{
  let variant=match value{ValueType::Boolean=>"boolean",ValueType::Integer=>"integer",ValueType::Decimal=>"decimal",ValueType::Text=>"text",ValueType::List(_)=>"list",ValueType::Schema(_)=>"schema",ValueType::Any=>"any"};
  let id=out.insert("jack_value_type",&[Cell::Text(variant)])?;if root.is_none(){root=Some(id)}
  if let Some(parent)=parent{out.insert("jack_value_type_list",&[Cell::Integer(parent),Cell::Integer(id)])?;}
  out.checkpoint_total(total)?;
  match value{ValueType::List(inner)=>{parent=Some(id);value=inner},ValueType::Schema(value)=>{out.insert("jack_value_type_schema",&[Cell::Integer(id),Cell::Text(value)])?;break},_=>break}
 }
 root.ok_or_else(||invariant("Jack value type walk lost its root"))
}
fn declarations(values:&[PropertyDef],table:&str,parent:i64,out:&mut Projection<'_,'_>,total:usize)->Result<(),ValueError>{
 for(n,value)in values.iter().enumerate(){let ty=project_type(&value.value_type,out,total)?;out.insert(table,&[Cell::Integer(parent),Cell::Integer(ordinal(n)?),Cell::Text(&value.name),Cell::Text(match value.kind{PropertyKind::Data=>"data",PropertyKind::Derived=>"derived"}),optional(&value.expr),Cell::Integer(ty)])?;out.checkpoint_total(total)?;}
 Ok(())
}
fn jack(value:&JackSnapshot,out:&mut Projection<'_,'_>,total:usize)->Result<i64,ValueError>{
 let id=out.insert("jack_document",&[Cell::Text(&value.schema),Cell::Text(&value.name),optional(&value.manifest_id),optional(&value.root_node_id),Cell::Text(&value.query)])?;
 insert_ieee754(out,"jack_camera",&[Cell::Integer(id),Cell::Real(value.camera.x),Cell::Real(value.camera.y),Cell::Real(value.camera.zoom)],&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4)])?;
 let child=&value.content;let target=&child.target;
 out.insert("jack_content_child",&[Cell::Integer(id),Cell::Text(&child.child_id),Cell::Text(&target.artifact_id),Cell::Text(&target.dialect.artifact_kind),Cell::Text(&target.dialect.standard),Cell::Text(&target.dialect.subset)])?;
 for(n,kind)in value.manifest.node_kinds.iter().enumerate(){let kind_id=out.insert("jack_node_kind",&[Cell::Integer(id),Cell::Integer(ordinal(n)?),Cell::Text(&kind.name)])?;for(n,port)in kind.port_kinds.iter().enumerate(){out.insert("jack_node_kind_port",&[Cell::Integer(kind_id),Cell::Integer(ordinal(n)?),Cell::Text(port)])?;}declarations(&kind.properties,"jack_node_property",kind_id,out,total)?;}
 for(n,kind)in value.manifest.edge_kinds.iter().enumerate(){let kind_id=out.insert("jack_edge_kind",&[Cell::Integer(id),Cell::Integer(ordinal(n)?),Cell::Text(&kind.name)])?;declarations(&kind.properties,"jack_edge_property",kind_id,out,total)?;}
 for(n,kind)in value.manifest.port_kinds.iter().enumerate(){let kind_id=out.insert("jack_port_kind",&[Cell::Integer(id),Cell::Integer(ordinal(n)?),Cell::Text(&kind.name),Cell::Text(match kind.direction{PortDirection::In=>"in",PortDirection::Out=>"out"})])?;declarations(&kind.properties,"jack_port_property",kind_id,out,total)?;}
 out.checkpoint_total(total)?;Ok(id)
}
pub(in super::super) fn project(value:&RewritingSnapshot,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
 let total=forecast(value,c,SqliteSnapshotPhase::ProjectSnapshot)?;let mut out=Projection::new(SQL,c)?;
 let working=jack(&value.working_graph,&mut out,total)?;let document=out.insert("rewriting_document",&[Cell::Integer(working)])?;
 for(key,LayoutPoint{x,y})in value.rule_layout.iter(){insert_ieee754(&mut out,"rewriting_layout",&[Cell::Integer(document),Cell::Text(key),Cell::Real(*x),Cell::Real(*y)],&[FloatColumn::Binary64(3),FloatColumn::Binary64(4)])?;}
 for(key,value)in value.parameter_bindings.iter(){let root=property(value,&mut out,total)?;out.insert("rewriting_binding",&[Cell::Integer(document),Cell::Text(key),Cell::Integer(root)])?;}
 let lhs_pattern=pattern(&value.lhs.pattern,&mut out)?;out.insert("rewriting_lhs",&[Cell::Integer(document),Cell::Integer(lhs_pattern),optional(&value.lhs.where_clause)])?;
 let rhs=out.insert("rewriting_rhs",&[Cell::Integer(document)])?;
 for(table,patterns)in[("rewriting_create",&value.rhs.create),("rewriting_merge",&value.rhs.merge)]{for(n,value)in patterns.iter().enumerate(){let pattern=pattern(value,&mut out)?;out.insert(table,&[Cell::Integer(rhs),Cell::Integer(ordinal(n)?),Cell::Integer(pattern)])?;out.checkpoint_total(total)?;}}
 for(n,value)in value.rhs.delete.iter().enumerate(){out.insert("rewriting_delete",&[Cell::Integer(rhs),Cell::Integer(ordinal(n)?),Cell::Text(value)])?;out.checkpoint_total(total)?;}
 for(n,value)in value.rhs.set.iter().enumerate(){let root=property(&value.value,&mut out,total)?;out.insert("rewriting_assignment",&[Cell::Integer(rhs),Cell::Integer(ordinal(n)?),Cell::Text(&value.var),Cell::Text(&value.prop),Cell::Integer(root)])?;out.checkpoint_total(total)?;}
 for(n,value)in value.rhs.parameters.iter().enumerate(){let root=property(&value.default,&mut out,total)?;out.insert("rewriting_parameter",&[Cell::Integer(rhs),Cell::Integer(ordinal(n)?),Cell::Text(&value.name),Cell::Text(match value.kind{ParameterKind::String=>"string",ParameterKind::Number=>"number",ParameterKind::Boolean=>"boolean"}),Cell::Integer(root)])?;out.checkpoint_total(total)?;}
 out.checkpoint_total(total)?;out.finish()
}
