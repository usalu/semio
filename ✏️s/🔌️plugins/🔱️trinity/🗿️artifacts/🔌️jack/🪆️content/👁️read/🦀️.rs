//! 👁️ Consumer-owned reads preserve the rich original child behind borrowed views.
use crate::{Edge,GraphEffect,JackGraphQuery,JackSnapshot,Node};
pub use crate::graph_query::{JackContentNodeView,JackContentPortView,JackContentEdgeView,JackContentVisual};
use semio_framework_value::{ValueError,retirement::RetirementCursor};
use semio_framework_plugin::{Fault,app::{ChildContentView,ChildEmitPreparation}};

/// 📍️ One decoded move keeps the concrete operation representation owned by composition.
pub struct JackContentMove{pub node_id:String,pub x:f64,pub y:f64}

/// 🧩️ Layout consumes move behavior without knowing the selected child's operation enum.
pub trait JackContentMoveCodec:Send+Sync{
    fn encode_move(&self,node_id:&str,x:f64,y:f64)->Result<Vec<u8>,ValueError>;
    fn decode_move(&self,bytes:&[u8])->Result<Option<JackContentMove>,ValueError>;
}

/// 🔎️ Rich UI reads, admitted query projections and concrete edit preparation share one retained child.
/// 🧱️ Projection validates every entity grant before transferring its owned query value.
pub trait JackContentProjection:JackGraphQuery{
    fn projection_counts(&self)->(usize,usize);
    fn query_node_id(&self,index:usize)->Option<&str>;
    fn query_edge_id(&self,index:usize)->Option<&str>;
    fn query_node(&self,index:usize,maximum_bytes:usize)->Result<Option<Node>,ValueError>;
    fn query_edge(&self,index:usize,maximum_bytes:usize)->Result<Option<Edge>,ValueError>;
}

/// 🧸️ The selected published child exposes all behaviors through the same retained address.
pub trait JackContentLayout:JackContentVisual{
    fn move_codec(&self)->std::sync::Arc<dyn JackContentMoveCodec>;
}

/// 🧸️ The same published child retains reads, layout operation behavior and effects.
pub trait JackContentRead:JackGraphQuery+JackContentLayout+JackContentProjection{
    fn prepare_effects(&self,effects:&[GraphEffect])->Result<Vec<ChildEmitPreparation>,ValueError>;
}

/// 🧳️ The original rich child supplies pack bytes and its own bounded retirement authority.
pub trait JackOwnedContent:JackContentRead+Send+Sync{
    fn genesis_pack(&self)->Result<Vec<u8>,ValueError>;
    fn retirement_birth_bytes(&self)->Option<usize>;
    fn into_retirement(self:Box<Self>)->Box<dyn RetirementCursor>;
}

/// 🏗️ The authored native installation selects members, live bindings, genesis and format assembly.
pub trait JackComposition:Default+Clone+Send+Sync+'static{
    type Members:store::SpaceMember+store::MemberFactory+Send+'static;
    type Read<'a>:JackContentRead+'a;
    fn child_from_working(nodes:Vec<Node>,edges:Vec<Edge>)->crate::JackContentChild;
    fn initial_snapshot()->JackSnapshot;
    fn example_snapshot(id:&str)->Option<JackSnapshot>;
    fn initial_viewer_snapshot()->JackSnapshot;
    fn bind<'a>(snapshot:&JackSnapshot,children:&'a ChildContentView)->Result<Self::Read<'a>,Fault>;
    fn genesis_child_pack(snapshot:&JackSnapshot,slot:&str,child_id:&str)->Result<Option<Vec<u8>>,ValueError>;
    fn examples()->&'static[semio_framework_plugin::ExampleSource];
    fn io_entries()->&'static[store::io::io_mechanism::IoEntry];
}
