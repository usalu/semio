//! 🏗️ The authored native S installation binds Jack behavior to its real rich Semio member store.
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
#[path="🚪️io/🦀️.rs"]
pub mod io;
use semio_s_artifact_trinity_jack as jack;
use semio_s_artifact_trinity_jack_semio_content as content;
use jack::standards::v1::subsets::any::schema::composition::JackComposition;
use jack::standards::v1::subsets::any::schema::content::JackContentFactory;
use jack::standards::v1::subsets::any::schema::content_read::{JackContentRead, JackContentProjection, JackContentLayout};
use semio_s_artifact_trinity_jack_graph_query::{JackContentVisual, JackGraphQuery};
use semio_framework_schema_composition::ChildRefFields;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_plugin::{Fault,FaultOrigin,FaultCode,app::{ChildContentView,ChildEmitPreparation}};
use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use semio_s_artifact_stdio_semio_base::schema::geometry::read_binary64_lexeme;

/// 🧩️ Every native Jack runtime installs this composition explicitly, including its actual member roster.
#[derive(Default,Clone)]
pub struct SemioJackComposition;

/// 🧸️ A live read retains the exact original published store lease and all five borrowed identity coordinates.
pub struct SemioJackPublishedRead<'a>{identity:ChildRefFields<'a>,snapshot:store::os_store::snapshot_read::SnapshotReadRef<'a,SemioGraphSnapshot>}

fn fault(code:&'static str,message:impl Into<String>)->Fault{Fault::new(FaultOrigin::App,FaultCode::new(code),message.into())}
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

impl JackContentFactory for SemioJackComposition{
 fn child_from_working(nodes:Vec<jack::standards::v1::subsets::any::schema::graph::Node>,edges:Vec<jack::standards::v1::subsets::any::schema::graph::Edge>)->jack::standards::v1::subsets::any::schema::content::JackContentChild{<content::SemioJackContent as JackContentFactory>::child_from_working(nodes,edges)}
}

impl JackComposition for SemioJackComposition{
 type Members=semio_s_artifact_stdio_semio::SemioMembers;
 type Read<'a>=SemioJackPublishedRead<'a>;
 fn initial_snapshot()->jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot{
  let asset:semio_framework_pack_json::Value=semio_framework_pack_json::from_str(include_str!("🖼️assets/🎬️demo/🔣️.query.json")).expect("authored query scene JSON");
  let scene:jack::standards::v1::subsets::any::schema::ast::JackQueryGraphSnapshot=semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&asset)).expect("authored query scene schema");
  let mut snapshot=jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot::with_content::<Self>(scene.schema,scene.name,scene.manifest_id,scene.manifest,scene.camera,jack::standards::v1::subsets::any::schema::content::JackWorkingScene{nodes:scene.nodes,edges:scene.edges},scene.root_node_id);
  snapshot.query=scene.query;snapshot.resolve_manifest().expect("authored scene manifest");snapshot
 }
 fn example_snapshot(id:&str)->Option<jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot>{matches!(id,"demo"|"nakagin"|"nakagin-capsule-tower"|"branch-chain").then(Self::initial_snapshot)}
 fn initial_viewer_snapshot()->jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot{jack::standards::v1::subsets::any::schema::snapshot::empty_trinity_graph_snapshot::<Self>()}
 fn bind<'a>(snapshot:&jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot,children:&'a ChildContentView)->Result<Self::Read<'a>,Fault>{
  let selected=jack::standards::v1::subsets::any::schema::content::jack_content_identity(&snapshot.content);
  let identity=children.reference_fields("content",selected.child_id).ok_or_else(||fault("trinity.jack.content-missing","the selected graph member is not published"))?;
  if identity!=selected{return Err(fault("trinity.jack.content-dialect","the published graph member differs from the exact selected address"))}
  if identity.artifact_kind!="s.stdio.semio"||identity.standard!="v1"||identity.subset!="graph"{return Err(fault("trinity.jack.content-dialect","the authored Semio composition requires its declared graph dialect"))}
  let snapshot=children.typed_read::<SemioGraphSnapshot>("content",identity.child_id)?;
  Ok(SemioJackPublishedRead{identity,snapshot})
 }
 fn genesis_child_pack(snapshot:&jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot,slot:&str,child_id:&str)->Result<Option<Vec<u8>>,ValueError>{
  if slot!="content"||child_id!=snapshot.content.child_id{return Ok(None)}
  if snapshot.content.local_owner::<jack::standards::v1::subsets::any::schema::content::JackContentOwner>().is_some(){return jack::standards::v1::subsets::any::schema::composition::genesis_jack_child_pack(snapshot,slot,child_id)}
  for build in [Self::initial_snapshot as fn()->jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot,Self::initial_viewer_snapshot]{
   let bundled=build();
   let admitted=jack::standards::v1::subsets::any::schema::content::jack_content_identity(&snapshot.content)==jack::standards::v1::subsets::any::schema::content::jack_content_identity(&bundled.content);
   let result=if admitted{jack::standards::v1::subsets::any::schema::composition::genesis_jack_child_pack(&bundled,slot,child_id)}else{Ok(None)};
   jack::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(bundled)?;
   if admitted{return result}
  }
  Ok(None)
 }
 fn examples()->&'static[semio_framework_plugin::ExampleSource]{static EXAMPLES:std::sync::LazyLock<Vec<semio_framework_plugin::ExampleSource>>=std::sync::LazyLock::new(||vec![semio_framework_plugin::ExampleSource::deferred("demo",jack::examples::demo::label(),"file",".query.json",include_bytes!("🖼️assets/🎬️demo/🔣️.query.json"),||{let snapshot=SemioJackComposition::initial_snapshot();let text=snapshot.to_json();jack::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(snapshot).expect("authored example source retirement");text.expect("authored parent JSON")} )]);EXAMPLES.as_slice()}
 fn io_entries()->&'static[store::io::io_mechanism::IoEntry]{io::entries()}
}

impl JackGraphQuery for SemioJackPublishedRead<'_>{
 fn identity(&self)->ChildRefFields<'_>{self.identity}
 fn node_count(&self)->usize{self.snapshot.nodes.len()}
 fn node_id(&self,index:usize)->Option<&str>{self.snapshot.nodes.get(index).map(|node|node.id.value.as_str())}
 fn edge_count(&self)->usize{self.snapshot.edges.len()}
 fn edge(&self,index:usize)->Option<semio_s_artifact_trinity_jack_graph_query::JackGraphEndpoint<'_>>{self.snapshot.edges.get(index).map(|edge|semio_s_artifact_trinity_jack_graph_query::JackGraphEndpoint{id:&edge.id.value,source:&edge.source.value,target:&edge.target.value})}
 fn displacement(&self,id:&str)->Result<(f64,f64),ValueError>{content::displacement(&self.snapshot,id)}
}
impl JackContentVisual for SemioJackPublishedRead<'_>{
 fn visual_counts(&self)->(usize,usize){(self.snapshot.nodes.len(),self.snapshot.edges.len())}
 fn node_view(&self,index:usize)->Option<semio_s_artifact_trinity_jack_graph_query::JackContentNodeView<'_>>{content::visual_node(&self.snapshot,index)}
 fn port_view(&self,node_index:usize,port_index:usize)->Option<semio_s_artifact_trinity_jack_graph_query::JackContentPortView<'_>>{content::visual_port(&self.snapshot,node_index,port_index)}
 fn edge_view(&self,index:usize)->Option<semio_s_artifact_trinity_jack_graph_query::JackContentEdgeView<'_>>{content::visual_edge(&self.snapshot,index)}
}
impl JackContentProjection for SemioJackPublishedRead<'_>{
 fn projection_counts(&self)->(usize,usize){(self.snapshot.nodes.len(),self.snapshot.edges.len())}
 fn query_node_id(&self,index:usize)->Option<&str>{self.snapshot.nodes.get(index).map(|node|node.id.value.as_str())}
 fn query_edge_id(&self,index:usize)->Option<&str>{self.snapshot.edges.get(index).map(|edge|edge.id.value.as_str())}
 fn query_node(&self,index:usize,maximum_bytes:usize)->Result<Option<jack::standards::v1::subsets::any::schema::graph::Node>,ValueError>{content::project_node(&self.snapshot,index,maximum_bytes)}
 fn query_edge(&self,index:usize,maximum_bytes:usize)->Result<Option<jack::standards::v1::subsets::any::schema::graph::Edge>,ValueError>{content::project_edge(&self.snapshot,index,maximum_bytes)}
}
impl JackContentLayout for SemioJackPublishedRead<'_>{
 fn move_codec(&self)->std::sync::Arc<dyn jack::standards::v1::subsets::any::schema::content_read::JackContentMoveCodec>{content::move_codec()}
}
impl JackContentRead for SemioJackPublishedRead<'_>{
 fn prepare_effects(&self,effects:&[jack::standards::v1::subsets::any::schema::graph::GraphEffect])->Result<Vec<ChildEmitPreparation>,ValueError>{let leaves=content::graph_leaves(&self.snapshot,effects);Ok(if leaves.is_empty(){Vec::new()}else{vec![ChildEmitPreparation::of::<SemioGraphSnapshot,_>("content",self.identity.child_id,leaves)]})}
}

/// 🏗️ The S-owned native declaration installs its concrete content composition explicitly.
pub fn artifact<PA:jack::standards::v1::subsets::any::registration::ArtifactApps<SemioJackComposition>>()->semio_framework_plugin::app::declarations::ArtifactDeclaration<PA>{jack::standards::v1::subsets::any::registration::artifact::<PA,SemioJackComposition>()}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
