//! 🧳️ Actual Semio graph owners retain rich values, pack, mutations and controlled retirement.
use super::jack;
use jack::{JackContentChild,JackContentVisual,JackContentProjection,JackContentLayout,JackContentRead,JackOwnedContent,JackGraphQuery};
use jack::{Node,Edge,Port,PortDirection,PropertyBag,PropertyValue,JackSnapshot};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor};
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_base::schema::geometry::binary64_lexeme;
use semio_s_artifact_stdio_semio_base::schema::geometry::read_binary64_lexeme;
use semio_s_artifact_stdio_semio_graph::schema::snapshot::{GraphEdgeId, GraphNodeId, SemioGraphEdge, SemioGraphNode, SemioGraphPort, SemioGraphPortKind, SemioGraphSnapshot, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueEntry;
use std::sync::Arc;

use {semio_s_artifact_stdio_semio_graph::schema::mutations::add_edge_property::AddEdgeProperty, semio_s_artifact_stdio_semio_graph::schema::mutations::add_node_property::AddNodeProperty, semio_s_artifact_stdio_semio_graph::schema::mutations::change_node_label::ChangeNodeLabel, semio_s_artifact_stdio_semio_graph::schema::mutations::create_edge::CreateEdge, semio_s_artifact_stdio_semio_graph::schema::mutations::create_node::CreateNode, semio_s_artifact_stdio_semio_graph::schema::mutations::delete_edge::DeleteEdge, semio_s_artifact_stdio_semio_graph::schema::mutations::delete_node::DeleteNode, semio_s_artifact_stdio_semio_graph::schema::mutations::move_node::MoveNode, semio_s_artifact_stdio_semio_graph::schema::mutations::remove_edge_property::RemoveEdgeProperty, semio_s_artifact_stdio_semio_graph::schema::mutations::remove_node_property::RemoveNodeProperty, semio_s_artifact_stdio_semio_graph::schema::mutations::set_edge_property::SetEdgeProperty, semio_s_artifact_stdio_semio_graph::schema::mutations::set_node_property::SetNodeProperty, semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation};


pub struct SemioJackOwnedContent{identity:JackContentChild,snapshot:Option<SemioGraphSnapshot>}
impl SemioJackOwnedContent{pub fn new(identity:JackContentChild,snapshot:SemioGraphSnapshot)->Self{Self{identity,snapshot:Some(snapshot)}}pub fn snapshot(&self)->&SemioGraphSnapshot{self.snapshot.as_ref().expect("original live graph owner")}}
impl RetireOwned for SemioJackOwnedContent{
 fn retirement(self)->Box<dyn RetirementCursor>{semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.identity),semio_framework_value::retirement::deferred(self.snapshot)])}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.identity),semio_framework_value::retirement::deferred_birth_bytes_for(&self.snapshot)])}
 fn controlled_retirement_supported()->bool{true}
}
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

fn property_to_semio(value:&PropertyValue)->SemioValue{match value{
 PropertyValue::Null=>SemioValue::Null,
 PropertyValue::Bool(value)=>SemioValue::Bool{value:*value},
 PropertyValue::Number(value)=>SemioValue::Float{lexeme:binary64_lexeme(*value)},
 PropertyValue::String(value)=>SemioValue::Str{value:value.clone()},
 PropertyValue::Array(values)=>SemioValue::List{items:values.iter().map(property_to_semio).collect()},
 PropertyValue::Object(values)=>SemioValue::Map{entries:properties_to_semio(values)},
}}
fn properties_to_semio(values:&PropertyBag)->Vec<SemioValueEntry>{values.iter().map(|(key,value)|SemioValueEntry{key:key.clone(),value:property_to_semio(value)}).collect()}
fn property_from_semio(value:&SemioValue)->Result<PropertyValue,ValueError>{Ok(match value{
 SemioValue::Null=>PropertyValue::Null,
 SemioValue::Bool{value}=>PropertyValue::Bool(*value),
 SemioValue::Float{lexeme}=>PropertyValue::Number(read_binary64_lexeme(lexeme)?),
 SemioValue::Str{value}=>PropertyValue::String(value.clone()),
 SemioValue::List{items}=>PropertyValue::Array(items.iter().map(property_from_semio).collect::<Result<_,_>>()?),
 SemioValue::Map{entries}=>PropertyValue::Object(properties_from_semio(entries)?),
 SemioValue::Int{..}|SemioValue::Bytes{..}|SemioValue::Ref{..}=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Jack query projection cannot represent this intrinsic Semio value family")),
})}
fn properties_from_semio(values:&[SemioValueEntry])->Result<PropertyBag,ValueError>{let mut result=PropertyBag::new();for entry in values{if result.contains_key(&entry.key){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Jack query projection requires unique property keys"))}result.insert(entry.key.clone(),property_from_semio(&entry.value)?);}Ok(result)}
fn endpoint(node:&str,port:Option<&str>)->Result<String,ValueError>{if node.contains('@')||port.is_some_and(|port|port.contains('@')){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Jack query endpoint cannot represent a literal separator in an endpoint component"))}Ok(match port{Some(port)=>format!("{node}@{port}"),None=>node.to_string()})}

pub fn jack_content_snapshot_from_working(nodes:&[Node],edges:&[Edge])->SemioGraphSnapshot{
 SemioGraphSnapshot{schema:STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(),nodes:nodes.iter().map(|node|SemioGraphNode{id:GraphNodeId::new(node.id.clone()),kind:node.kind.clone(),label:node.name.clone(),position:SemioPoint2{x:node.x,y:node.y},width:node.width,height:node.height,ports:node.ports.iter().map(|port|SemioGraphPort{name:port.id.clone(),kind:match port.direction{PortDirection::In=>SemioGraphPortKind::In,PortDirection::Out=>SemioGraphPortKind::Out},category:port.kind.clone(),properties:properties_to_semio(&port.properties)}).collect(),properties:properties_to_semio(&node.properties)}).collect(),edges:edges.iter().map(|edge|{let(source,source_port)=edge.source.split_once('@').map_or((edge.source.as_str(),None),|(node,port)|(node,Some(port.to_string())));let(target,target_port)=edge.target.split_once('@').map_or((edge.target.as_str(),None),|(node,port)|(node,Some(port.to_string())));SemioGraphEdge{id:GraphEdgeId::new(edge.id.clone()),source:GraphNodeId::new(source),target:GraphNodeId::new(target),kind:edge.kind.clone(),label:String::new(),source_port,target_port,properties:properties_to_semio(&edge.properties)}}).collect()}
}

pub fn working_from_jack_content_snapshot(content:&SemioGraphSnapshot)->Result<(Vec<Node>,Vec<Edge>),ValueError>{
 let nodes=content.nodes.iter().map(jack_node_from_content).collect::<Result<_,ValueError>>()?;
 let edges=content.edges.iter().map(jack_edge_from_content).collect::<Result<_,ValueError>>()?;Ok((nodes,edges))
}

/// 🔎️ Projects one declared query node from the sole retained Semio child.
pub fn jack_node_from_content(node:&SemioGraphNode)->Result<Node,ValueError>{let ports=node.ports.iter().map(|port|Ok(Port{id:port.name.clone(),kind:port.category.clone(),direction:match port.kind{SemioGraphPortKind::In=>PortDirection::In,SemioGraphPortKind::Out=>PortDirection::Out,SemioGraphPortKind::InOut=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Jack query projection requires a single port direction"))},properties:properties_from_semio(&port.properties)?})).collect::<Result<_,ValueError>>()?;Ok(Node{id:node.id.value.clone(),kind:node.kind.clone(),name:node.label.clone(),x:node.position.x,y:node.position.y,width:node.width,height:node.height,ports,properties:properties_from_semio(&node.properties)?})}

/// 🔗️ Projects one representable query edge without copying the retained child graph.
pub fn jack_edge_from_content(edge:&SemioGraphEdge)->Result<Edge,ValueError>{if !edge.label.is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Jack query projection cannot represent an intrinsic edge label"))}Ok(Edge{id:edge.id.value.clone(),kind:edge.kind.clone(),source:endpoint(&edge.source.value,edge.source_port.as_deref())?,target:endpoint(&edge.target.value,edge.target_port.as_deref())?,properties:properties_from_semio(&edge.properties)?})}

pub fn jack_content_handle(snapshot:&SemioGraphSnapshot)->JackContentChild{use pack::snapshot::ArtifactPack;let bytes=SemioGraphSnapshot::encode_pack(snapshot);let child_id=store::content_id("jack-content",&bytes);let target=semio_framework_artifact_reference::ArtifactRef{artifact_id:child_id.clone(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.semio".into(),standard:"v1".into(),subset:"graph".into()}};store::os_store::artifact_child::ArtifactChild::new(child_id,target)}
pub fn jack_content_child_handle(nodes:&[Node],edges:&[Edge])->JackContentChild{jack_content_handle(&jack_content_snapshot_from_working(nodes,edges))}
pub fn jack_content_child_with_snapshot(snapshot:SemioGraphSnapshot)->JackContentChild{let handle=jack_content_handle(&snapshot);let identity=JackContentChild::new(handle.child_id.clone(),handle.target.clone());jack::jack_content_with_owner(handle,Box::new(SemioJackOwnedContent::new(identity,snapshot))).expect("authored original graph address")}
pub fn jack_content_child_with_owner(nodes:Vec<Node>,edges:Vec<Edge>)->JackContentChild{jack_content_child_with_snapshot(jack_content_snapshot_from_working(&nodes,&edges))}
pub fn materialize_jack_snapshot(handle:&mut JackContentChild,snapshot:SemioGraphSnapshot){let identity=JackContentChild::new(handle.child_id.clone(),handle.target.clone());handle.set_local_owner(Arc::new(jack::JackContentOwner::new(identity.clone(),Box::new(SemioJackOwnedContent::new(identity,snapshot))).expect("authored original graph address")))}
pub fn materialize_jack_content(handle:&mut JackContentChild,nodes:Vec<Node>,edges:Vec<Edge>){materialize_jack_snapshot(handle,jack_content_snapshot_from_working(&nodes,&edges))}
/// 🔌️ Splits a `node@port` endpoint into the graph's node and port fields.
fn split_endpoint(endpoint: &str) -> (GraphNodeId, Option<String>) {
    endpoint.split_once('@').map_or((GraphNodeId::new(endpoint), None), |(node, port)| (GraphNodeId::new(node), Some(port.to_string())))
}

/// 🏷️ The sorted insertion index of `key` in a property list kept in jack's key order.
fn property_index(entries: &[SemioValueEntry], key: &str) -> usize {
    entries.iter().position(|entry| entry.key.as_str() > key).unwrap_or(entries.len())
}

/// 🧮️ The content child's graph leaves that apply query `effects` to `base`, each against the state the earlier ones left:
/// a node delete first cuts every incident edge as its own row (every row stays point-invertible), a property set on an
/// existing key is `set-*-property`, a new key is `add-*-property` at its sorted index.
pub fn graph_leaves(base: &SemioGraphSnapshot, effects: &[jack::GraphEffect]) -> Vec<SemioGraphMutation> {
    use jack::GraphEffect;
    let mut work = base.clone();
    let mut leaves = Vec::with_capacity(effects.len());
    let push = |work: &mut SemioGraphSnapshot, leaf: SemioGraphMutation, leaves: &mut Vec<SemioGraphMutation>| {
        let outcome = <SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::diff(&leaf, work);
        if let Ok(next) = protocol::apply_diff(outcome.diff(), work) {
            *work = next;
        }
        leaves.push(leaf);
    };
    for effect in effects {
        match effect {
            GraphEffect::CreateNode(node) => {
                let content = jack_content_snapshot_from_working(std::slice::from_ref(node), &[]).nodes.remove(0);
                let SemioGraphNode { id, kind, label, position, width, height, ports, properties } = content;
                push(&mut work, SemioGraphMutation::CreateNode(CreateNode { id, kind, label, position, width, height, ports, properties, at: None }), &mut leaves);
            }
            GraphEffect::DeleteNode(id) => {
                let incident: Vec<GraphEdgeId> = work.edges.iter().filter(|edge| edge.source.value == *id || edge.target.value == *id).map(|edge| edge.id.clone()).collect();
                for edge in incident {
                    push(&mut work, SemioGraphMutation::DeleteEdge(DeleteEdge { id: edge }), &mut leaves);
                }
                push(&mut work, SemioGraphMutation::DeleteNode(DeleteNode { id: GraphNodeId::new(id.clone()) }), &mut leaves);
            }
            GraphEffect::CreateEdge(edge) => {
                let (source, source_port) = split_endpoint(&edge.source);
                let (target, target_port) = split_endpoint(&edge.target);
                push(&mut work, SemioGraphMutation::CreateEdge(CreateEdge { id: GraphEdgeId::new(edge.id.clone()), source, target, kind: edge.kind.clone(), label: String::new(), source_port, target_port, properties: properties_to_semio(&edge.properties), at: None }), &mut leaves);
            }
            GraphEffect::DeleteEdge(id) => push(&mut work, SemioGraphMutation::DeleteEdge(DeleteEdge { id: GraphEdgeId::new(id.clone()) }), &mut leaves),
            GraphEffect::RenameNode { id, name } => push(&mut work, SemioGraphMutation::ChangeNodeLabel(ChangeNodeLabel { id: GraphNodeId::new(id.clone()), new_label: name.clone() }), &mut leaves),
            GraphEffect::MoveNode { id, x, y } => push(&mut work, SemioGraphMutation::MoveNode(MoveNode { id: GraphNodeId::new(id.clone()), new_position: SemioPoint2 { x: *x, y: *y } }), &mut leaves),
            GraphEffect::SetProperty { entity: jack::EntityRef::Node(id), key, value } => {
                let existing = work.nodes.iter().find(|node| node.id.value == *id).map(|node| (node.properties.iter().any(|entry| entry.key == *key), property_index(&node.properties, key)));
                let leaf = match existing {
                    Some((true, _)) => SemioGraphMutation::SetNodeProperty(SetNodeProperty { node_id: GraphNodeId::new(id.clone()), key: key.clone(), value: property_to_semio(value) }),
                    other => SemioGraphMutation::AddNodeProperty(AddNodeProperty { node_id: GraphNodeId::new(id.clone()), index: other.map_or(0, |(_, index)| index), property: SemioValueEntry { key: key.clone(), value: property_to_semio(value) } }),
                };
                push(&mut work, leaf, &mut leaves);
            }
            GraphEffect::SetProperty { entity: jack::EntityRef::Edge(id), key, value } => {
                let existing = work.edges.iter().find(|edge| edge.id.value == *id).map(|edge| (edge.properties.iter().any(|entry| entry.key == *key), property_index(&edge.properties, key)));
                let leaf = match existing {
                    Some((true, _)) => SemioGraphMutation::SetEdgeProperty(SetEdgeProperty { edge_id: GraphEdgeId::new(id.clone()), key: key.clone(), value: property_to_semio(value) }),
                    other => SemioGraphMutation::AddEdgeProperty(AddEdgeProperty { edge_id: GraphEdgeId::new(id.clone()), index: other.map_or(0, |(_, index)| index), property: SemioValueEntry { key: key.clone(), value: property_to_semio(value) } }),
                };
                push(&mut work, leaf, &mut leaves);
            }
            GraphEffect::RemoveProperty { entity: jack::EntityRef::Node(id), key } => push(&mut work, SemioGraphMutation::RemoveNodeProperty(RemoveNodeProperty { node_id: GraphNodeId::new(id.clone()), key: key.clone() }), &mut leaves),
            GraphEffect::RemoveProperty { entity: jack::EntityRef::Edge(id), key } => push(&mut work, SemioGraphMutation::RemoveEdgeProperty(RemoveEdgeProperty { edge_id: GraphEdgeId::new(id.clone()), key: key.clone() }), &mut leaves),
        }
    }
    jack::retire_owned_to_terminal(work).expect("valid Jack working child retirement");
    leaves
}
const QUERY_ENTITY_COLLECTION_MAXIMUM:usize=128;
const QUERY_ENTITY_NESTING_MAXIMUM:usize=16;
fn add_owned_bytes(bytes:&mut usize,additional:usize,maximum:usize)->Result<(),ValueError>{*bytes=bytes.checked_add(additional).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"query entity byte count overflow"))?;if *bytes>maximum{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"query entity exceeds its byte grant"))}Ok(())}
fn content_property_bytes(value:&SemioValue,depth:usize,items:&mut usize,bytes:&mut usize,maximum:usize)->Result<(),ValueError>{
    use SemioValue;
    if depth>QUERY_ENTITY_NESTING_MAXIMUM{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"query entity exceeds its nesting admission"))}
    *items=items.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"query entity item count overflow"))?;
    if *items>QUERY_ENTITY_COLLECTION_MAXIMUM{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"query entity exceeds its collection admission"))}
    add_owned_bytes(bytes,size_of::<PropertyValue>(),maximum)?;
    match value{SemioValue::Null|SemioValue::Bool{..}=>Ok(()),SemioValue::Float{lexeme}=>read_binary64_lexeme(lexeme).map(|_|()),SemioValue::Str{value}=>add_owned_bytes(bytes,value.len(),maximum),SemioValue::List{items:values}=>{for value in values{content_property_bytes(value,depth+1,items,bytes,maximum)?;}Ok(())},SemioValue::Map{entries}=>{for (index,entry) in entries.iter().enumerate(){if entries[..index].iter().any(|previous|previous.key==entry.key){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Jack query projection requires unique property keys"))}add_owned_bytes(bytes,entry.key.len(),maximum)?;content_property_bytes(&entry.value,depth+1,items,bytes,maximum)?;}Ok(())},_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Jack query projection cannot represent this intrinsic Semio value family"))}
}
fn content_bag_bytes(values:&[SemioValueEntry],items:&mut usize,bytes:&mut usize,maximum:usize)->Result<(),ValueError>{for (index,entry) in values.iter().enumerate(){if values[..index].iter().any(|previous|previous.key==entry.key){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Jack query projection requires unique property keys"))}add_owned_bytes(bytes,entry.key.len(),maximum)?;content_property_bytes(&entry.value,0,items,bytes,maximum)?;}Ok(())}
fn content_node_bytes(node:&SemioGraphNode,maximum:usize)->Result<(),ValueError>{
    let mut bytes=size_of::<Node>();let mut items=node.ports.len();
    if node.ports.len()>QUERY_ENTITY_COLLECTION_MAXIMUM{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"query node exceeds its port admission"))}
    for value in[&node.id.value,&node.id.value,&node.kind,&node.label]{add_owned_bytes(&mut bytes,value.len(),maximum)?;}
    content_bag_bytes(&node.properties,&mut items,&mut bytes,maximum)?;
    for port in &node.ports{if matches!(port.kind,SemioGraphPortKind::InOut){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Jack query projection requires a single port direction"))}add_owned_bytes(&mut bytes,size_of::<Port>(),maximum)?;add_owned_bytes(&mut bytes,port.name.len(),maximum)?;add_owned_bytes(&mut bytes,port.category.len(),maximum)?;content_bag_bytes(&port.properties,&mut items,&mut bytes,maximum)?;}Ok(())
}
fn content_edge_bytes(edge:&SemioGraphEdge,maximum:usize)->Result<(),ValueError>{
    if !edge.label.is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Jack query projection cannot represent an intrinsic edge label"))}
    if [&edge.source.value,&edge.target.value].into_iter().any(|id|id.contains('@'))||[edge.source_port.as_deref(),edge.target_port.as_deref()].into_iter().flatten().any(|port|port.contains('@')){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Jack query endpoint cannot represent a literal separator in an endpoint component"))}
    let mut bytes=size_of::<Edge>();let mut items=0;
    for value in[&edge.id.value,&edge.id.value,&edge.kind,&edge.source.value,&edge.target.value]{add_owned_bytes(&mut bytes,value.len(),maximum)?;}
    for port in[edge.source_port.as_deref(),edge.target_port.as_deref()].into_iter().flatten(){add_owned_bytes(&mut bytes,port.len().checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"query entity byte count overflow"))?,maximum)?;}
    content_bag_bytes(&edge.properties,&mut items,&mut bytes,maximum)
}


struct SemioJackMoveCodec;
impl jack::JackContentMoveCodec for SemioJackMoveCodec{
 fn encode_move(&self,node_id:&str,x:f64,y:f64)->Result<Vec<u8>,ValueError>{let leaf=SemioGraphMutation::MoveNode(MoveNode{id:GraphNodeId::new(node_id),new_position:SemioPoint2{x,y}});protocol::OpBinary::encode_op(&leaf).map_err(|error|invalid(format!("{error:?}")))}
 fn decode_move(&self,bytes:&[u8])->Result<Option<jack::JackContentMove>,ValueError>{match <SemioGraphMutation as protocol::OpBinary>::decode_op(bytes){Ok(SemioGraphMutation::MoveNode(payload))=>Ok(Some(jack::JackContentMove{node_id:payload.id.value,x:payload.new_position.x,y:payload.new_position.y})),Ok(_)=>Ok(None),Err(error)=>Err(invalid(format!("{error:?}")))}}
}

pub fn visual_node(snapshot:&SemioGraphSnapshot,index:usize)->Option<jack::JackContentNodeView<'_>>{snapshot.nodes.get(index).map(|node|jack::JackContentNodeView{id:&node.id.value,kind:&node.kind,label:&node.label,x:node.position.x,y:node.position.y,width:node.width,height:node.height,port_count:node.ports.len()})}
pub fn visual_port(snapshot:&SemioGraphSnapshot,node_index:usize,port_index:usize)->Option<jack::JackContentPortView<'_>>{snapshot.nodes.get(node_index)?.ports.get(port_index).map(|port|jack::JackContentPortView{id:&port.name,label:&port.name,inbound:matches!(port.kind,SemioGraphPortKind::In|SemioGraphPortKind::InOut),outbound:matches!(port.kind,SemioGraphPortKind::Out|SemioGraphPortKind::InOut)})}
pub fn visual_edge(snapshot:&SemioGraphSnapshot,index:usize)->Option<jack::JackContentEdgeView<'_>>{snapshot.edges.get(index).map(|edge|jack::JackContentEdgeView{id:&edge.id.value,kind:&edge.kind,label:&edge.label,source:&edge.source.value,target:&edge.target.value,source_port:edge.source_port.as_deref(),target_port:edge.target_port.as_deref()})}
pub fn project_node(snapshot:&SemioGraphSnapshot,index:usize,maximum_bytes:usize)->Result<Option<Node>,ValueError>{let Some(node)=snapshot.nodes.get(index)else{return Ok(None)};content_node_bytes(node,maximum_bytes)?;jack_node_from_content(node).map(Some)}
pub fn project_edge(snapshot:&SemioGraphSnapshot,index:usize,maximum_bytes:usize)->Result<Option<Edge>,ValueError>{let Some(edge)=snapshot.edges.get(index)else{return Ok(None)};content_edge_bytes(edge,maximum_bytes)?;jack_edge_from_content(edge).map(Some)}
pub fn move_codec()->Arc<dyn jack::JackContentMoveCodec>{Arc::new(SemioJackMoveCodec)}
pub fn displacement(snapshot:&SemioGraphSnapshot,id:&str)->Result<(f64,f64),ValueError>{let edge=snapshot.edges.iter().find(|edge|edge.id.value==id).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"flattened layout requires retained edge"))?;let offset=|key:&str|{let mut values=edge.properties.iter().filter(|entry|entry.key==key);let Some(value)=values.next()else{return Ok(0.0)};if values.next().is_some(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"flattened offset requires unique key"))}match &value.value{SemioValue::Float{lexeme}=>read_binary64_lexeme(lexeme),SemioValue::Int{lexeme}=>lexeme.parse::<f64>().map_err(|_|ValueError::literal(ValueRefusalKind::InvalidValue,"flattened integer lexeme invalid")),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"flattened offset requires numeric value family"))}};Ok((offset("u")?,offset("v")?))}
impl JackGraphQuery for SemioJackOwnedContent{
 fn identity(&self)->semio_framework_schema_composition::ChildRefFields<'_>{jack::jack_content_identity(&self.identity)}
 fn node_count(&self)->usize{self.snapshot().nodes.len()}
 fn node_id(&self,index:usize)->Option<&str>{self.snapshot().nodes.get(index).map(|node|node.id.value.as_str())}
 fn edge_count(&self)->usize{self.snapshot().edges.len()}
 fn edge(&self,index:usize)->Option<jack::JackGraphEndpoint<'_>>{self.snapshot().edges.get(index).map(|edge|jack::JackGraphEndpoint{id:&edge.id.value,source:&edge.source.value,target:&edge.target.value})}
 fn displacement(&self,id:&str)->Result<(f64,f64),ValueError>{displacement(self.snapshot(),id)}
}
impl JackContentVisual for SemioJackOwnedContent{
 fn visual_counts(&self)->(usize,usize){(self.snapshot().nodes.len(),self.snapshot().edges.len())}
 fn node_view(&self,index:usize)->Option<jack::JackContentNodeView<'_>>{visual_node(self.snapshot(),index)}
 fn port_view(&self,node_index:usize,port_index:usize)->Option<jack::JackContentPortView<'_>>{visual_port(self.snapshot(),node_index,port_index)}
 fn edge_view(&self,index:usize)->Option<jack::JackContentEdgeView<'_>>{visual_edge(self.snapshot(),index)}
}
impl JackContentProjection for SemioJackOwnedContent{
 fn projection_counts(&self)->(usize,usize){self.visual_counts()}
 fn query_node_id(&self,index:usize)->Option<&str>{self.node_id(index)}
 fn query_edge_id(&self,index:usize)->Option<&str>{self.snapshot().edges.get(index).map(|edge|edge.id.value.as_str())}
 fn query_node(&self,index:usize,maximum_bytes:usize)->Result<Option<Node>,ValueError>{project_node(self.snapshot(),index,maximum_bytes)}
 fn query_edge(&self,index:usize,maximum_bytes:usize)->Result<Option<Edge>,ValueError>{project_edge(self.snapshot(),index,maximum_bytes)}
}
impl JackContentLayout for SemioJackOwnedContent{fn move_codec(&self)->Arc<dyn jack::JackContentMoveCodec>{move_codec()}}
impl JackContentRead for SemioJackOwnedContent{fn prepare_effects(&self,effects:&[jack::GraphEffect])->Result<Vec<semio_framework_plugin::app::ChildEmitPreparation>,ValueError>{let leaves=graph_leaves(self.snapshot(),effects);Ok(if leaves.is_empty(){Vec::new()}else{vec![semio_framework_plugin::app::ChildEmitPreparation::of::<SemioGraphSnapshot,_>("content",&self.identity.child_id,leaves)]})}}
impl JackOwnedContent for SemioJackOwnedContent{
 fn genesis_pack(&self)->Result<Vec<u8>,ValueError>{use pack::snapshot::ArtifactPack;Ok(SemioGraphSnapshot::encode_pack(self.snapshot()))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value::retirement::boxed_owner_birth_bytes::<Self>())}
 fn into_retirement(self:Box<Self>)->Box<dyn RetirementCursor>{RetireOwned::retirement(self)}
}
