//! 🎛️ Derived positions consume retained endpoints and composition-owned numeric displacement.
use crate::{JackGraphQuery,JackGraphEndpoint,retained_jack_graph};
use semio_framework_schema_composition::ChildRefFields;
use semio_framework_value::{ValueError,ValueRefusalKind};
use std::collections::{BTreeMap,BTreeSet};
#[derive(Clone,Copy,Debug,Default,PartialEq,value_derive::ToValue,value_derive::FromValue)]
#[value(rename_all="camelCase")]
pub struct JackFlatPositionUv {pub u:f64,pub v:f64}
#[derive(Clone,Debug,Default,PartialEq,value_derive::ToValue,value_derive::FromValue)]
#[value(rename_all="camelCase")]
pub struct JackFlatPosition {pub positions:BTreeMap<String,JackFlatPositionUv>}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
/// 🧭️ Exact owner admission and topology validation precede a deterministic component walk.
pub fn infer_jack_flat_position(selected:ChildRefFields<'_>,root:Option<&str>,graph:&dyn JackGraphQuery)->Result<JackFlatPosition,ValueError>{
 crate::infer_jack_topology(selected,graph)?;
 let graph=retained_jack_graph(selected,graph)?;
 let nodes=(0..graph.node_count()).map(|index|graph.node_id(index).ok_or_else(||invalid("Jack graph capability omitted a declared node"))).collect::<Result<BTreeSet<_>,_>>()?;
 let mut edges=(0..graph.edge_count()).map(|index|graph.edge(index).ok_or_else(||invalid("Jack graph capability omitted a declared edge"))).collect::<Result<Vec<_>,_>>()?;
 edges.sort_unstable_by(|left,right|crate::identity_order(left.id,right.id));
 let offsets=edges.iter().map(|edge|Ok((edge.id,graph.displacement(edge.id)?))).collect::<Result<BTreeMap<_,_>,ValueError>>()?;
 let mut flat=BTreeMap::<String,(f64,f64)>::new();
 if let Some(root)=root.or_else(||nodes.iter().copied().min_by(|left,right|crate::identity_order(left,right))){
  if !nodes.contains(root){return Err(invalid("Jack flattened root requires retained node"))}
  extend(&edges,&offsets,&mut flat,root);
 }
 while flat.len()<nodes.len(){
  let remaining=nodes.iter().copied().filter(|id|!flat.contains_key(*id)).collect::<BTreeSet<_>>();
  let seed=remaining.iter().copied().filter(|id|!edges.iter().any(|edge|edge.target==*id&&remaining.contains(edge.source))).min_by(|left,right|crate::identity_order(left,right)).or_else(||remaining.iter().copied().min_by(|left,right|crate::identity_order(left,right))).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Jack flattened component seed absent"))?;
  extend(&edges,&offsets,&mut flat,seed);
 }
 Ok(JackFlatPosition{positions:flat.into_iter().map(|(id,(u,v))|(id,JackFlatPositionUv{u,v})).collect()})
}
fn extend(edges:&[JackGraphEndpoint<'_>],offsets:&BTreeMap<&str,(f64,f64)>,flat:&mut BTreeMap<String,(f64,f64)>,seed:&str){
 if flat.contains_key(seed){return}
 flat.insert(seed.to_string(),(0.0,0.0));
 let mut stack=vec![seed.to_string()];
 while let Some(source)=stack.pop(){
  let(held_u,held_v)=flat[&source];
  for edge in edges{
   if edge.source!=source||flat.contains_key(edge.target){continue}
   let(u,v)=offsets[edge.id];flat.insert(edge.target.to_string(),(held_u+u,held_v+v));stack.push(edge.target.to_string());
  }
 }
}
