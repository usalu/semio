//! 🧭️ Topology traverses actual retained Semio endpoints without projecting unrelated value families.
use crate::JackSnapshot;
use semio_framework_value::{ValueError,ValueRefusalKind};
use std::collections::{BTreeMap,VecDeque};
#[derive(Clone,Debug,PartialEq,value_derive::ToValue,value_derive::FromValue)]
#[value(rename_all="camelCase")]
pub struct JackTopology {pub topo_order:Vec<String>,pub depth:BTreeMap<String,u32>,pub cycle_free:bool,pub node_count:u32}
impl Default for JackTopology {fn default()->Self{Self{topo_order:Vec::new(),depth:BTreeMap::new(),cycle_free:true,node_count:0}}}
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn limit(message:&str)->ValueError{ValueError::new(ValueRefusalKind::WorkLimit,message)}
/// 🪆️ Owner refusal and exact node references precede deterministic Kahn traversal.
pub fn compute_topology(snapshot:&JackSnapshot)->Result<JackTopology,ValueError>{
 let owner=crate::jack_content_for_handle(&snapshot.content)?;
 let graph=owner.snapshot();
 let node_count=u32::try_from(graph.nodes.len()).map_err(|_|limit("Jack inference node count exceeds u32"))?;
 let mut adjacency=BTreeMap::<String,Vec<String>>::new();
 let mut indegree=BTreeMap::<String,u32>::new();
 for node in &graph.nodes{if indegree.contains_key(&node.id.value){return Err(invalid("Jack inference requires unique node identities"))}adjacency.insert(node.id.value.clone(),Vec::new());indegree.insert(node.id.value.clone(),0);}
 let mut edge_ids=std::collections::BTreeSet::new();
 for edge in &graph.edges{
  if !edge_ids.insert(&edge.id.value){return Err(invalid("Jack inference requires unique edge identities"))}
  let Some(children)=adjacency.get_mut(&edge.source.value)else{return Err(invalid("Jack inference edge requires retained source"))};
  let Some(degree)=indegree.get_mut(&edge.target.value)else{return Err(invalid("Jack inference edge requires retained target"))};
  children.push(edge.target.value.clone());*degree=degree.checked_add(1).ok_or_else(||limit("Jack inference indegree overflow"))?;
 }
 let mut queue=indegree.iter().filter(|(_,degree)|**degree==0).map(|(id,_)|id.clone()).collect::<VecDeque<_>>();
 let mut depth=queue.iter().map(|id|(id.clone(),0u32)).collect::<BTreeMap<_,_>>();
 let mut topo_order=Vec::new();
 while let Some(id)=queue.pop_front(){
  topo_order.push(id.clone());
  let node_depth=*depth.get(&id).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Jack frontier depth absent"))?;
  let mut newly_zero=Vec::new();
  for child in adjacency.get(&id).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Jack frontier adjacency absent"))?{
   let candidate=node_depth.checked_add(1).ok_or_else(||limit("Jack inference depth overflow"))?;
   let held=depth.entry(child.clone()).or_insert(0);*held=(*held).max(candidate);
   let remaining=indegree.get_mut(child).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Jack frontier child absent"))?;
   *remaining=remaining.checked_sub(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Jack indegree underflow"))?;
   if *remaining==0{newly_zero.push(child.clone())}
  }
  newly_zero.sort();queue.extend(newly_zero);
 }
 depth.retain(|id,_|topo_order.contains(id));
 Ok(JackTopology{cycle_free:topo_order.len()==graph.nodes.len(),topo_order,depth,node_count})
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
