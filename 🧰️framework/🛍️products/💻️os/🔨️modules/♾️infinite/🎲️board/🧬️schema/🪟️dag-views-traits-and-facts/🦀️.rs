//! 🪟️ DAG views, traits and facts of the board schema: the evaluation status retirement, computing progress, channel and refusal views, the output source traits and the selection, hover and hit facts. See [`super`].

use super::*;

semio_framework_value::artifact_retire_struct!(DagSelectionDomains{nodes,edges,handles});

semio_framework_value::artifact_retire_struct!(DagChannelRef{widget_id,port,direction});

semio_framework_value::artifact_retire_leaf!(DagChannelDirection);

semio_framework_value::artifact_retire_struct!(DagWireTypeRefusal{source,source_types,target,target_types});

semio_framework_value::artifact_retire_struct!(DagHoverFacts{channel,refusal});

semio_framework_value::artifact_retire_struct!(DagScreenHit{node_id,channel,minimap,minimap_viewport,port_insert,handle,widget});

semio_framework_value::artifact_retire_struct!(DagScreenHitJson{node,draggable,handle,direction,widget,minimap,minimap_viewport,port_insert});

semio_framework_value::artifact_retire_struct!(DagComputingProgress{active,stale});

impl RetireOwned for DagNodeEvaluationStatus{
    fn retirement(self)->Box<dyn RetirementCursor>{match self{Self::Ok{}|Self::Queued{}|Self::Computing{}=>sequence(vec![]),Self::Error{message}=>sequence(vec![deferred(message)]),Self::Blocked{ports}=>sequence(vec![deferred(ports)])}}
    fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::Ok{}|Self::Queued{}|Self::Computing{}=>sequence_birth_bytes(&[]),Self::Error{message}=>sequence_birth_bytes(&[deferred_birth_bytes_for(message)]),Self::Blocked{ports}=>sequence_birth_bytes(&[deferred_birth_bytes_for(ports)])}}
    fn controlled_retirement_supported()->bool{true}
}

#[derive(Clone,Debug,Default,PartialEq,FromValue,ToValue)]
#[cfg_attr(any(test, feature = "testkit"),derive(Serialize,Deserialize))]
#[cfg_attr(any(test, feature = "testkit"),serde(deny_unknown_fields))]
#[value(deny_unknown_fields)]
pub struct DagComputingProgress {pub active:Option<String>,pub stale:Vec<String>}

#[derive(Clone,Copy)]
pub struct DagChannelView<'a>{pub widget_id:&'a str,pub port:&'a str,pub direction:DagChannelDirection}

#[derive(Clone,Copy)]
pub struct DagRefusalView<'a>{pub source:&'a str,pub source_types:&'a[String],pub target:&'a str,pub target_types:&'a[String]}

pub trait DagOutputSource{
    fn selected_node_count(&self)->usize;
    fn selected_node_id(&self,index:usize)->Option<&str>;
    fn selected_edge_count(&self)->usize;
    fn selected_edge_id(&self,index:usize)->Option<&str>;
    fn selected_channel_count(&self)->usize;
    fn selected_channel(&self,index:usize)->Option<DagChannelView<'_>>;
    fn selected_handle_identity(&self,index:usize)->Option<&str>;
    fn hovered_channel_view(&self)->Option<DagChannelView<'_>>;
    fn wire_refusal_view(&self)->Option<DagRefusalView<'_>>;
}

pub trait DagOutputCandidates{
    fn node_candidate_count(&self)->usize;
    fn node_candidate_id(&self,index:usize)->Option<&str>;
    fn edge_candidate_count(&self)->usize;
    fn edge_candidate_id(&self,index:usize)->Option<&str>;
    fn channel_candidate_count(&self)->usize;
    fn channel_candidate(&self,index:usize)->Option<DagChannelView<'_>>;
    fn handle_candidate_identity(&self,index:usize)->Option<&str>;
    fn hovered_channel_view(&self)->Option<DagChannelView<'_>>;
    fn wire_refusal_view(&self)->Option<DagRefusalView<'_>>;
}

#[derive(Clone,Debug,Default,PartialEq,Serialize,Deserialize,ToValue,FromValue)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct DagSelectionDomains {pub nodes:Vec<String>,pub edges:Vec<String>,pub handles:Vec<String>}

/// 🧭️ Declares the complete semantic input/output port direction.
#[derive(Clone,Copy,Debug,PartialEq,Eq,Serialize,Deserialize,ToValue,FromValue)]
#[serde(rename_all="lowercase")]
#[value(rename_all="lowercase")]
pub enum DagChannelDirection {In,Out}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize,ToValue,FromValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct DagChannelRef {pub widget_id:String,pub port:String,pub direction:DagChannelDirection}

impl DagChannelRef {
    pub fn is_input(&self)->bool{self.direction==DagChannelDirection::In}
    pub fn is_output(&self)->bool{self.direction==DagChannelDirection::Out}
}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize,ToValue,FromValue)]
#[serde(tag="status",rename_all="camelCase",deny_unknown_fields)]
#[value(tag="status",rename_all="camelCase",deny_unknown_fields)]
pub enum DagNodeEvaluationStatus {Ok{},Queued{},Computing{},Error{message:String},Blocked{ports:Vec<String>}}

/// 🚦️ Holds admitted per-node evaluation facts independently of physical field spelling.
pub type DagNodeStatuses=OrderedMap<DagNodeEvaluationStatus>;

/// 🚫️ Identifies a refused port pair and its declared semantic value types.
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize,ToValue,FromValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct DagWireTypeRefusal {pub source:String,pub source_types:Vec<String>,pub target:String,pub target_types:Vec<String>}

/// 🖱️ Projects hover and wire refusal facts from the same accepted graph state.
#[derive(Clone,Debug,Default,PartialEq,Serialize,Deserialize,ToValue,FromValue)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct DagHoverFacts {pub channel:Option<DagChannelRef>,pub refusal:Option<DagWireTypeRefusal>}

/// 🩺️ Classifies overlays and draggable node bodies from one accepted hit.
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize,ToValue,FromValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct DagScreenHit {
    pub node_id:Option<String>,pub channel:Option<DagChannelRef>,pub minimap:bool,pub minimap_viewport:bool,pub port_insert:bool,pub handle:bool,pub widget:bool,
}

impl DagScreenHit {
    pub fn is_screen_path(&self) -> bool {self.minimap||self.port_insert||self.handle||self.widget}
    pub fn is_draggable_body(&self) -> bool {
        self.node_id.is_some() && !self.is_screen_path()
    }
}

/// 🩻️ Projects the same hit classification across the renderer boundary.
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize,ToValue,FromValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct DagScreenHitJson {
    pub node:Option<String>,pub draggable:bool,pub handle:Option<String>,pub direction:Option<DagChannelDirection>,pub widget:bool,pub minimap:bool,pub minimap_viewport:bool,pub port_insert:bool,
}

/// 🔱️ Identifies the semantic selection relation inspected by a read source.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum DagSelectionDomain {Nodes,Edges}

/// 👁️ Borrows one candidate identity at a time without creating physical output.
pub trait DagSelectionSource {
    fn selection_candidate_count(&self,domain:DagSelectionDomain)->usize;
    fn selection_candidate_id(&self,domain:DagSelectionDomain,index:usize)->Option<&str>;
}
