//! 📸️ Declared nested Board snapshot facts independent of physical object maps.
use semio_framework_value_derive::{FromValue,ToValue};
use crate::infinite::board::{BoardVisibility,CameraDescriptor};
#[derive(Clone,Copy,Debug,PartialEq,Eq,ToValue,FromValue)]
#[value(rename_all="camelCase")]
pub enum BoardNodeShape { Circle, Rectangle }

/// 🧬️ Owns declared BoardHandleSnapshot semantic fields.
#[derive(Clone,Debug,Default,ToValue,FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct BoardHandleSnapshot {
    pub id: String,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub angle: Option<f64>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub radius: Option<f64>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub scale: Option<f64>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub handle_kind: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub color: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub icon_kind: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub text: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub label: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub selected: Option<bool>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub style: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub user_data: Option<semio_framework_value::DslValue>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub hidden: Option<bool>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub visible: Option<bool>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub locked: Option<bool>,
}
impl BoardHandleSnapshot { pub fn visibility(&self)->BoardVisibility {BoardVisibility{hidden:self.hidden,visible:self.visible,locked:self.locked}} }

/// 🧬️ Owns declared BoardNodeSnapshot semantic fields.
#[derive(Clone,Debug,Default,ToValue,FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct BoardNodeSnapshot {
    pub id: String,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub x: Option<f64>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub y: Option<f64>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub shape: Option<BoardNodeShape>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub radius: Option<f64>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub width: Option<f64>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub height: Option<f64>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub scale: Option<f64>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub text: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub label: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub icon_kind: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub node_kind: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub draggable: Option<bool>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub selected: Option<bool>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub style: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub user_data: Option<semio_framework_value::DslValue>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub root: Option<bool>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub hidden: Option<bool>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub visible: Option<bool>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub locked: Option<bool>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub handles: Option<Vec<BoardHandleSnapshot>>,
}
impl BoardNodeSnapshot { pub fn visibility(&self)->BoardVisibility {BoardVisibility{hidden:self.hidden,visible:self.visible,locked:self.locked}} }

/// 🧬️ Owns declared BoardEdgeSnapshot semantic fields.
#[derive(Clone,Debug,Default,ToValue,FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct BoardEdgeSnapshot {
    pub id: String,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub source: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub target: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub edge_kind: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub source_tip: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub target_tip: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub label: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub text: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub selected: Option<bool>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub style: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub user_data: Option<semio_framework_value::DslValue>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub hidden: Option<bool>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub visible: Option<bool>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub locked: Option<bool>,
}
impl BoardEdgeSnapshot { pub fn visibility(&self)->BoardVisibility {BoardVisibility{hidden:self.hidden,visible:self.visible,locked:self.locked}} }

/// 🧬️ Owns declared BoardRegionSnapshot semantic fields.
#[derive(Clone,Debug,Default,ToValue,FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct BoardRegionSnapshot {
    pub id: String,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub x: Option<f64>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub y: Option<f64>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub width: Option<f64>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub height: Option<f64>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub label: Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub hidden: Option<bool>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub locked: Option<bool>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub selected: Option<bool>,
}

/// 🎲️ Declared snapshot graph families admitted by the exclusive Board codec.
#[derive(Clone,Copy,Debug,PartialEq,Eq,ToValue,FromValue)]
pub enum BoardSnapshotSchema {
 #[value(rename="board.ports.directed.v1")]
 DirectedPorts,
 #[value(rename="board.normal.undirected.v1")]
 NormalUndirected,
 #[value(rename="trinity.graph")]
 TrinityGraph,
 #[value(rename="reasoning.wires.identity.snapshot")]
 WiresIdentity,
}
impl BoardSnapshotSchema{pub fn as_str(self)->&'static str{match self{Self::DirectedPorts=>"board.ports.directed.v1",Self::NormalUndirected=>"board.normal.undirected.v1",Self::TrinityGraph=>"trinity.graph",Self::WiresIdentity=>"reasoning.wires.identity.snapshot"}}}

/// 📸️ Admitted board snapshot with typed nested entities.
#[derive(Clone,Debug,ToValue,FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct BoardSnapshot {
 pub schema:BoardSnapshotSchema,
 #[value(default,skip_serializing_if="Option::is_none")]
 pub camera:Option<CameraDescriptor>,
 pub nodes:Vec<BoardNodeSnapshot>,
 pub edges:Vec<BoardEdgeSnapshot>,
 #[value(default,skip_serializing_if="Vec::is_empty")]
 pub target_regions:Vec<BoardRegionSnapshot>,
 #[value(default,skip_serializing_if="Option::is_none")]
 pub meta:Option<semio_framework_value::DslValue>,
}
