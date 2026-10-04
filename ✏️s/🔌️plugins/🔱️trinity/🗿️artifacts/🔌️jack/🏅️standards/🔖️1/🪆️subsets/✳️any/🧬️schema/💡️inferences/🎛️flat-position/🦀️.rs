//! 🎛️ Derived positions read actual rich Semio endpoints and declared numeric offset fields.
use crate::JackSnapshot;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphEdge;
use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
use std::collections::{BTreeMap,BTreeSet};
#[derive(Clone,Copy,Debug,Default,PartialEq,value_derive::ToValue,value_derive::FromValue)]
#[value(rename_all="camelCase")]
pub struct JackFlatPositionUv {pub u:f64,pub v:f64}
#[derive(Clone,Debug,Default,PartialEq,value_derive::ToValue,value_derive::FromValue)]
#[value(rename_all="camelCase")]
pub struct JackFlatPosition {pub positions:BTreeMap<String,JackFlatPositionUv>}
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn offset(edge:&SemioGraphEdge,key:&str)->Result<f64,ValueError>{
 let mut values=edge.properties.iter().filter(|entry|entry.key==key);
 let Some(value)=values.next()else{return Ok(0.0)};
 if values.next().is_some(){return Err(invalid("Jack flattened offset requires unique property key"))}
 match &value.value {
  SemioValue::Float{lexeme}=>semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::read_binary64_lexeme(lexeme),
  SemioValue::Int{lexeme}=>lexeme.parse::<f64>().map_err(|_|invalid("Jack flattened integer lexeme invalid")),
  _=>Err(invalid("Jack flattened offset requires numeric intrinsic family")),
 }
}
/// 🪆️ Owner and endpoint failures propagate before a deterministic connected-component walk.
pub fn compute_flat_position(snapshot:&JackSnapshot)->Result<JackFlatPosition,ValueError>{
 crate::standards::v1::subsets::any::schema::inferences::topology::compute_topology(snapshot)?;
 let owner=crate::jack_content_for_handle(&snapshot.content)?;
 let graph=owner.snapshot();
 let nodes=graph.nodes.iter().map(|node|node.id.value.as_str()).collect::<BTreeSet<_>>();
 let edges=graph.edges.iter().map(|edge|(edge.id.value.as_str(),edge)).collect::<BTreeMap<_,_>>();
 let offsets=edges.iter().map(|(id,edge)|Ok((*id,(offset(edge,"u")?,offset(edge,"v")?)))).collect::<Result<BTreeMap<_,_>,ValueError>>()?;
 let mut flat=BTreeMap::<String,(f64,f64)>::new();
 if let Some(root)=snapshot.root_node_id.as_deref().or_else(||nodes.first().copied()){
  if !nodes.contains(root){return Err(invalid("Jack flattened root requires retained node"))}
  extend(&edges,&offsets,&mut flat,root);
 }
 while flat.len()<nodes.len(){
  let remaining=nodes.iter().copied().filter(|id|!flat.contains_key(*id)).collect::<BTreeSet<_>>();
  let seed=remaining.iter().copied().find(|id|!edges.values().any(|edge|edge.target.value==*id&&remaining.contains(edge.source.value.as_str()))).or_else(||remaining.first().copied()).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Jack flattened component seed absent"))?;
  extend(&edges,&offsets,&mut flat,seed);
 }
 Ok(JackFlatPosition{positions:flat.into_iter().map(|(id,(u,v))|(id,JackFlatPositionUv{u,v})).collect()})
}
fn extend(edges:&BTreeMap<&str,&SemioGraphEdge>,offsets:&BTreeMap<&str,(f64,f64)>,flat:&mut BTreeMap<String,(f64,f64)>,seed:&str){
 if flat.contains_key(seed){return}
 flat.insert(seed.to_string(),(0.0,0.0));
 let mut stack=vec![seed.to_string()];
 while let Some(source)=stack.pop(){
  let(held_u,held_v)=flat[&source];
  for(id,edge)in edges{
   if edge.source.value!=source||flat.contains_key(&edge.target.value){continue}
   let(u,v)=offsets[id];flat.insert(edge.target.value.clone(),(held_u+u,held_v+v));stack.push(edge.target.value.clone());
  }
 }
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
