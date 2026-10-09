//! 🎯️ Typed selection and evaluation facts shared by DAG host receivers.
use serde::{Deserialize,Serialize};
use semio_framework_value_derive::{FromValue,ToValue};
use semio_framework_value::ordered::OrderedMap;

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

#[path="📊️progress/🦀️.rs"]
pub mod progress;
pub use progress::DagComputingProgress;

#[path="📤️output/🦀️.rs"]
pub mod output;

#[path="♻️retirement/🦀️.rs"]
mod retirement;
