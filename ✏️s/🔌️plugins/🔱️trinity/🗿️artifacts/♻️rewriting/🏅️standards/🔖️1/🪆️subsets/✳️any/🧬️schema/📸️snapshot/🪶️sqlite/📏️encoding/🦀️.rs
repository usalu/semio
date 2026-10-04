//! 📏️ borrowed Rewriting bounds retain actual fields and pay only concrete traversal frontiers.
use crate::RewritingSnapshot;
use crate::standards::v1::subsets::any::schema::{Pattern,Rhs};
use semio_s_artifact_trinity_jack::{JackSnapshot,PropertyDef};
use semio_framework_graph::manifest::PropertyValue;
use semio_framework_value::{ValueError,ValueType};
use semio_framework_os_kernel::sqlite_snapshot::{SqliteSnapshotControl,artifact::NativeEncodingBound};
type Bound<'a,'p>=NativeEncodingBound<'a,'p>;
fn text(value:&str,bound:&mut Bound<'_,'_>)->Result<(),ValueError>{bound.add(128)?;bound.repeated(value.len(),6)}
fn optional(value:&Option<String>,bound:&mut Bound<'_,'_>)->Result<(),ValueError>{match value{Some(value)=>text(value,bound),None=>bound.add(128)}}
fn pattern(value:&Pattern,bound:&mut Bound<'_,'_>)->Result<(),ValueError>{bound.add(512)?;text(&value.left_var,bound)?;text(&value.left_kind,bound)?;for value in[&value.edge_var,&value.edge_kind,&value.right_var,&value.right_kind]{optional(value,bound)?;}Ok(())}
fn property(value:&PropertyValue,bound:&mut Bound<'_,'_>)->Result<(),ValueError>{
 let mut pending=bound.allocate_frontier(1)?;bound.push_frontier(&mut pending,value)?;
 while let Some(value)=pending.pop(){bound.add(256)?;match value{
  PropertyValue::Null|PropertyValue::Bool(_)=>{},PropertyValue::Number(_)=>bound.add(1100)?,PropertyValue::String(value)=>text(value,bound)?,
  PropertyValue::Array(values)=>for value in values{bound.push_frontier(&mut pending,value)?;},
  PropertyValue::Object(values)=>for(key,value)in values.iter(){text(key,bound)?;bound.push_frontier(&mut pending,value)?;},
 }}
 Ok(())
}
fn declarations(values:&[PropertyDef],bound:&mut Bound<'_,'_>)->Result<(),ValueError>{
 for value in values{bound.add(512)?;text(&value.name,bound)?;optional(&value.expr,bound)?;let mut kind=&value.value_type;loop{bound.add(256)?;match kind{ValueType::List(value)=>kind=value,ValueType::Schema(value)=>{text(value,bound)?;break},_=>break}}}
 Ok(())
}
fn jack(value:&JackSnapshot,bound:&mut Bound<'_,'_>)->Result<(),ValueError>{
 bound.add(2048)?;for value in[&value.schema,&value.name,&value.query,&value.content.child_id,&value.content.target.artifact_id,&value.content.target.dialect.artifact_kind,&value.content.target.dialect.standard,&value.content.target.dialect.subset]{text(value,bound)?;}
 optional(&value.manifest_id,bound)?;optional(&value.root_node_id,bound)?;bound.repeated(3,1100)?;
 for kind in &value.manifest.node_kinds{bound.add(512)?;text(&kind.name,bound)?;for port in &kind.port_kinds{text(port,bound)?;}declarations(&kind.properties,bound)?;}
 for kind in &value.manifest.edge_kinds{bound.add(512)?;text(&kind.name,bound)?;declarations(&kind.properties,bound)?;}
 for kind in &value.manifest.port_kinds{bound.add(512)?;text(&kind.name,bound)?;declarations(&kind.properties,bound)?;}
 Ok(())
}
fn rhs(value:&Rhs,bound:&mut Bound<'_,'_>)->Result<(),ValueError>{
 bound.add(1024)?;for values in[&value.create,&value.merge]{for value in values{pattern(value,bound)?;}}
 for value in &value.delete{text(value,bound)?;}
 for value in &value.set{bound.add(512)?;text(&value.var,bound)?;text(&value.prop,bound)?;property(&value.value,bound)?;}
 for value in &value.parameters{bound.add(512)?;text(&value.name,bound)?;property(&value.default,bound)?;}
 Ok(())
}
pub(super) fn preflight(value:&RewritingSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 let mut bound=NativeEncodingBound::new(control)?;bound.add(2048)?;jack(&value.working_graph,&mut bound)?;pattern(&value.lhs.pattern,&mut bound)?;optional(&value.lhs.where_clause,&mut bound)?;rhs(&value.rhs,&mut bound)?;
 for(key,value)in value.parameter_bindings.iter(){text(key,&mut bound)?;property(value,&mut bound)?;}
 for(key,_)in value.rule_layout.iter(){text(key,&mut bound)?;bound.repeated(2,1100)?;}
 bound.finish()
}
