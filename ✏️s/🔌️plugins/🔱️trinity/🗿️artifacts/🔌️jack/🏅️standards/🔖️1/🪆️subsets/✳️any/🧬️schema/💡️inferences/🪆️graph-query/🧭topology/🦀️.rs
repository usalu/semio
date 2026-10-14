//! 🧭️ Topology traverses borrowed graph identities without projecting unrelated content.
use crate::{JackGraphQuery,retained_jack_graph};
use semio_framework_schema_composition::ChildRefFields;
use semio_framework_value::{ValueError,ValueRefusalKind};
use std::collections::{BTreeMap,VecDeque};
#[derive(Clone,Debug,PartialEq,value_derive::ToValue,value_derive::FromValue)]
#[value(rename_all="camelCase")]
pub struct JackTopology {pub topo_order:Vec<String>,pub depth:BTreeMap<String,u32>,pub cycle_free:bool,pub node_count:u32}
impl Default for JackTopology {fn default()->Self{Self{topo_order:Vec::new(),depth:BTreeMap::new(),cycle_free:true,node_count:0}}}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn limit(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::WorkLimit,message)}
/// 🔎️ Exact child admission precedes every borrowed entity read.
pub fn infer_jack_topology(selected:ChildRefFields<'_>,graph:&dyn JackGraphQuery)->Result<JackTopology,ValueError>{
 let graph=retained_jack_graph(selected,graph)?;
 let count=graph.node_count();
 let node_count=u32::try_from(count).map_err(|_|limit("Jack inference node count exceeds u32"))?;
 let mut adjacency=BTreeMap::<String,Vec<String>>::new();
 let mut indegree=BTreeMap::<String,u32>::new();
 for index in 0..count{let id=graph.node_id(index).ok_or_else(||invalid("Jack graph capability omitted a declared node"))?;if indegree.contains_key(id){return Err(invalid("Jack inference requires unique node identities"))}adjacency.insert(id.to_string(),Vec::new());indegree.insert(id.to_string(),0);}
 let mut edge_ids=std::collections::BTreeSet::new();
 for index in 0..graph.edge_count(){
  let edge=graph.edge(index).ok_or_else(||invalid("Jack graph capability omitted a declared edge"))?;
  if !edge_ids.insert(edge.id){return Err(invalid("Jack inference requires unique edge identities"))}
  let Some(children)=adjacency.get_mut(edge.source)else{return Err(invalid("Jack inference edge requires retained source"))};
  let Some(degree)=indegree.get_mut(edge.target)else{return Err(invalid("Jack inference edge requires retained target"))};
  children.push(edge.target.to_string());*degree=degree.checked_add(1).ok_or_else(||limit("Jack inference indegree overflow"))?;
 }
 let mut ready=indegree.iter().filter(|(_,degree)|**degree==0).map(|(id,_)|id.clone()).collect::<Vec<_>>();
 ready.sort_unstable_by(|left,right|crate::identity_order(left,right));
 let mut queue=VecDeque::from(ready);
 let mut depth=queue.iter().map(|id|(id.clone(),0u32)).collect::<BTreeMap<_,_>>();
 let mut topo_order=Vec::new();
 while let Some(id)=queue.pop_front(){
  topo_order.push(id.clone());
  let node_depth=*depth.get(&id).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Jack frontier depth absent"))?;
  let mut newly_zero=Vec::new();
  for child in adjacency.get(&id).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Jack frontier adjacency absent"))?{
   let candidate=node_depth.checked_add(1).ok_or_else(||limit("Jack inference depth overflow"))?;
   let held=depth.entry(child.clone()).or_insert(0);*held=(*held).max(candidate);
   let remaining=indegree.get_mut(child).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Jack frontier child absent"))?;
   *remaining=remaining.checked_sub(1).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Jack indegree underflow"))?;
   if *remaining==0{newly_zero.push(child.clone())}
  }
  newly_zero.sort_unstable_by(|left,right|crate::identity_order(left,right));queue.extend(newly_zero);
 }
 depth.retain(|id,_|topo_order.contains(id));
 Ok(JackTopology{cycle_free:topo_order.len()==count,topo_order,depth,node_count})
}
