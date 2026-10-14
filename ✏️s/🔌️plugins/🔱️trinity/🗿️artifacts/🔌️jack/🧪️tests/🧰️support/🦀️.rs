//! 🧪️ Isolated intrinsic fixtures explicitly select their own retained query-domain producer.
use crate::*;
use semio_framework_value::{ValueError,retirement::{RetireOwned,RetirementCursor}};
#[derive(Default,Clone)]
pub(crate) struct FixtureComposition;
#[derive(semio_framework_value::RetireOwned)]
pub(crate) struct FixtureContent{identity:JackContentChild,graph:crate::ast::JackQueryGraphSnapshot}
impl JackGraphQuery for FixtureContent{
 fn identity(&self)->semio_framework_schema_composition::ChildRefFields<'_>{jack_content_identity(&self.identity)}
 fn node_count(&self)->usize{self.graph.nodes.len()}
 fn node_id(&self,index:usize)->Option<&str>{self.graph.nodes.get(index).map(|node|node.id.as_str())}
 fn edge_count(&self)->usize{self.graph.edges.len()}
 fn edge(&self,index:usize)->Option<JackGraphEndpoint<'_>>{self.graph.edges.get(index).map(|edge|JackGraphEndpoint{id:&edge.id,source:edge.source.split_once('@').map_or(edge.source.as_str(),|(node,_)|node),target:edge.target.split_once('@').map_or(edge.target.as_str(),|(node,_)|node)})}
 fn displacement(&self,id:&str)->Result<(f64,f64),ValueError>{let edge=self.graph.edges.iter().find(|edge|edge.id==id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"fixture edge missing"))?;let offset=|key:&str|match edge.properties.get(key){None=>Ok(0.0),Some(PropertyValue::Number(value))=>Ok(*value),_=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"fixture numeric displacement required"))};Ok((offset("u")?,offset("v")?))}
}
impl JackContentVisual for FixtureContent{
 fn visual_counts(&self)->(usize,usize){self.graph.visual_counts()}
 fn node_view(&self,index:usize)->Option<JackContentNodeView<'_>>{self.graph.node_view(index)}
 fn port_view(&self,node_index:usize,port_index:usize)->Option<JackContentPortView<'_>>{self.graph.port_view(node_index,port_index)}
 fn edge_view(&self,index:usize)->Option<JackContentEdgeView<'_>>{self.graph.edge_view(index)}
}
impl JackContentProjection for FixtureContent{
 fn projection_counts(&self)->(usize,usize){self.visual_counts()}
 fn query_node_id(&self,index:usize)->Option<&str>{self.node_id(index)}
 fn query_edge_id(&self,index:usize)->Option<&str>{self.graph.edges.get(index).map(|edge|edge.id.as_str())}
 fn query_node(&self,index:usize,maximum:usize)->Result<Option<Node>,ValueError>{let Some(node)=self.graph.nodes.get(index)else{return Ok(None)};crate::executor::fixture_node_owned_bytes(node,maximum)?;Ok(Some(node.clone()))}
 fn query_edge(&self,index:usize,maximum:usize)->Result<Option<Edge>,ValueError>{let Some(edge)=self.graph.edges.get(index)else{return Ok(None)};crate::executor::fixture_edge_owned_bytes(edge,maximum)?;Ok(Some(edge.clone()))}
}
struct FixtureMoveCodec;
impl JackContentMoveCodec for FixtureMoveCodec{
 fn encode_move(&self,_:&str,_:f64,_:f64)->Result<Vec<u8>,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"fixture has no member operation codec"))}
 fn decode_move(&self,_:&[u8])->Result<Option<JackContentMove>,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"fixture has no member operation codec"))}
}
impl JackContentLayout for FixtureContent{fn move_codec(&self)->std::sync::Arc<dyn JackContentMoveCodec>{std::sync::Arc::new(FixtureMoveCodec)}}
impl JackContentRead for FixtureContent{fn prepare_effects(&self,_:&[GraphEffect])->Result<Vec<semio_framework_plugin::app::ChildEmitPreparation>,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"fixture has no registered member emitter"))}}
impl JackOwnedContent for FixtureContent{
 fn genesis_pack(&self)->Result<Vec<u8>,ValueError>{Ok(semio_framework_pack_json::to_json_string(&self.graph).into_bytes())}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value::retirement::boxed_owner_birth_bytes::<Self>())}
 fn into_retirement(self:Box<Self>)->Box<dyn RetirementCursor>{RetireOwned::retirement(self)}
}
impl JackComposition for FixtureComposition{
 type Members=store::NoMembers;
 type Read<'a>=FixtureContent;
 fn child_from_working(nodes:Vec<Node>,edges:Vec<Edge>)->JackContentChild{
  let graph=crate::ast::JackQueryGraphSnapshot{schema:JackSnapshot::SCHEMA.into(),name:String::new(),manifest_id:None,manifest:Manifest::default(),camera:Camera::default(),nodes,edges,root_node_id:None,query:String::new()};
  let bytes=semio_framework_pack_json::to_json_string(&graph);let child_id=store::content_id("jack-intrinsic-fixture",bytes.as_bytes());let target=semio_framework_artifact_reference::ArtifactRef{artifact_id:child_id.clone(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.fixture.graph".into(),standard:"exact".into(),subset:"query".into()}};let handle=JackContentChild::new(child_id,target);let identity=JackContentChild::new(handle.child_id.clone(),handle.target.clone());jack_content_with_owner(handle,Box::new(FixtureContent{identity,graph})).unwrap()
 }
 fn initial_snapshot()->JackSnapshot{empty_trinity_graph_snapshot::<Self>()}
 fn initial_viewer_snapshot()->JackSnapshot{Self::initial_snapshot()}
 fn example_snapshot(_:&str)->Option<JackSnapshot>{None}
 fn bind<'a>(_:&JackSnapshot,_:&'a semio_framework_plugin::app::ChildContentView)->Result<Self::Read<'a>,semio_framework_plugin::Fault>{Err(semio_framework_plugin::Fault::from("intrinsic fixture declares no published member receiver"))}
 fn genesis_child_pack(snapshot:&JackSnapshot,slot:&str,id:&str)->Result<Option<Vec<u8>>,ValueError>{genesis_jack_child_pack(snapshot,slot,id)}
 fn examples()->&'static[semio_framework_plugin::ExampleSource]{&[]}
 fn io_entries()->&'static[store::io::io_mechanism::IoEntry]{&[]}
}

pub(crate) fn materialize_fixture(handle:&mut JackContentChild,nodes:Vec<Node>,edges:Vec<Edge>){let identity=JackContentChild::new(handle.child_id.clone(),handle.target.clone());let graph=crate::ast::JackQueryGraphSnapshot{schema:JackSnapshot::SCHEMA.into(),name:String::new(),manifest_id:None,manifest:Manifest::default(),camera:Camera::default(),nodes,edges,root_node_id:None,query:String::new()};let owner=JackContentOwner::new(identity.clone(),Box::new(FixtureContent{identity,graph})).unwrap();handle.set_local_owner(std::sync::Arc::new(owner));}
