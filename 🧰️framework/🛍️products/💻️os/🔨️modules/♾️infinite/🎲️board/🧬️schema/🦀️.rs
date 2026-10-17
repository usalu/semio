//! 🧬️ Board schema: layout contracts, palette, dag input records, layout inference algorithms and the text io codecs.
//!
//! Crate `semio-framework-os-infinite-board-schema`; a downward producer of the infinite board family.

use std::collections::{BTreeMap, HashMap, HashSet};
use semio_framework_canvas::Circle;
use semio_framework_graph_drawing::force::ForceLayoutOptions as CoreForceLayoutOptions;
use serde::{Deserialize, Serialize};
use semio_framework_value::{DslValue, FromValue, NativeDecodeControl, NativeEncodeControl, Number, ToValue, ValueError, ValueRefusalKind};
use graph::{EdgeId, HandleId, NodeId};
use semio_framework_pack_json::{JsonGrammarCursor, JsonMemberPolicy, JsonWriteNode, JsonWriteSource};
use semio_framework_value::ordered::{OrderedMap, SharedOwner, UpdateCursor};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::{deferred, deferred_birth_bytes_for, RetirementCursor, RetireOwned, sequence, sequence_birth_bytes};
use geometry::Vec2;
use semio_framework_graph_drawing::force;
use semio_framework_value::retirement::*;

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

pub fn decode_board_palette_overlay_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<BoardPaletteOverlay,ValueError>{
    let mut value=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();
    if let DslValue::Object(fields)=value.get_mut(){for (_,field)in fields{control.step()?;if let DslValue::Array(components)=field{for component in components{control.step()?;if let DslValue::Number(Number::Float(number))=component{if number.is_finite()&&number.fract()==0.0&&*number>=0.0&&*number<=255.0{*component=DslValue::Number(Number::UInt(*number as u64));}}}}}}
    BoardPaletteOverlay::from_value_controlled(value.get(),control)
}

pub(crate) fn admit(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<DagComputingProgress,ValueError>{
    closed(value,&["active","stale"])?;
    let active=value.get("active").ok_or_else(refuse)?;if !active.is_null(){identifier(active)?;}
    identifiers(value.get("stale").ok_or_else(refuse)?,control)?;
    DagComputingProgress::from_value_controlled(value,control)
}

/// 📥️ Rejects incomplete, malformed and duplicate physical facts under the original caller control.
pub fn decode_dag_computing_progress_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<DagComputingProgress,ValueError>{
    let raw=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();admit(raw.get(),control)
}

#[derive(Clone,Copy)]
pub enum DagOutputKind{Nodes,Edges,Selection,Channels,Hover}

pub struct DagOutputPreparation{pub kind:DagOutputKind,pub nodes:Vec<usize>,pub edges:Vec<usize>,pub channels:Vec<usize>,pub field:usize,pub index:usize,pub phase:u8,pub complete:bool,pub progress:semio_framework_value::retained_clone::RetainedCloneProgress}

impl DagOutputPreparation{
    pub fn new(kind:DagOutputKind)->Self{Self{kind,nodes:Vec::new(),edges:Vec::new(),channels:Vec::new(),field:match kind{DagOutputKind::Edges=>1,DagOutputKind::Channels=>2,_=>0},index:0,phase:0,complete:matches!(kind,DagOutputKind::Hover),progress:Default::default()}}
    pub fn normal_step_progress(&self)->semio_framework_value::retained_clone::RetainedCloneProgress{self.progress}
    pub fn step<S:DagOutputCandidates+?Sized>(&mut self,source:&S,units:usize,control:&mut semio_framework_value::NativeEncodeControl<'_>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<bool,ValueError>{
        self.progress=Default::default();if units==0||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}for _ in 0..1{control.checkpoint()?;if self.complete{return Ok(true)}let count=match self.field{0=>source.node_candidate_count(),1=>source.edge_candidate_count(),_=>source.channel_candidate_count()};let output=match self.field{0=>&mut self.nodes,1=>&mut self.edges,_=>&mut self.channels};
            let bytes=if self.phase==0{count.checked_mul(size_of::<usize>()).ok_or_else(absent)?}else{0};if bytes>grant.maximum_capacity_bytes{return Ok(false)}control.admit_turn_capacity(grant.maximum_capacity_bytes)?;let before=control.owned_bytes();
            if self.phase==0{*output=control.allocate_vec(count)?;self.phase=1;}
            else if self.index==count{self.field+=1;self.index=0;self.phase=0;self.complete=self.field==match self.kind{DagOutputKind::Nodes=>1,DagOutputKind::Edges=>2,_=>3};}
            else{let admitted=match self.field{0=>source.node_candidate_id(self.index).is_some(),1=>source.edge_candidate_id(self.index).is_some(),_=>source.channel_candidate(self.index).is_some()&&source.handle_candidate_identity(self.index).is_some()};if admitted{output.push(self.index)}self.index+=1;}
            self.progress.copied_items=1;self.progress.retained_capacity_bytes=control.owned_bytes()-before;control.step()?;
        }Ok(self.complete)
    }
    pub fn view<'a,S:DagOutputCandidates+?Sized>(&'a self,source:&'a S)->DagPreparedOutput<'a,S>{assert!(self.complete);DagPreparedOutput{source,prepared:self}}
}

pub struct DagPreparedOutput<'a,S:DagOutputCandidates+?Sized>{pub source:&'a S,prepared:&'a DagOutputPreparation}

impl<S:DagOutputCandidates+?Sized> DagOutputSource for DagPreparedOutput<'_,S>{
    fn selected_node_count(&self)->usize{self.prepared.nodes.len()}
    fn selected_node_id(&self,index:usize)->Option<&str>{self.source.node_candidate_id(*self.prepared.nodes.get(index)?)}
    fn selected_edge_count(&self)->usize{self.prepared.edges.len()}
    fn selected_edge_id(&self,index:usize)->Option<&str>{self.source.edge_candidate_id(*self.prepared.edges.get(index)?)}
    fn selected_channel_count(&self)->usize{self.prepared.channels.len()}
    fn selected_channel(&self,index:usize)->Option<DagChannelView<'_>>{self.source.channel_candidate(*self.prepared.channels.get(index)?)}
    fn selected_handle_identity(&self,index:usize)->Option<&str>{self.source.handle_candidate_identity(*self.prepared.channels.get(index)?)}
    fn hovered_channel_view(&self)->Option<DagChannelView<'_>>{self.source.hovered_channel_view()}
    fn wire_refusal_view(&self)->Option<DagRefusalView<'_>>{self.source.wire_refusal_view()}
}

semio_framework_value::artifact_retire_leaf!(DagOutputKind);

semio_framework_value::artifact_retire_struct!(DagOutputPreparation{kind,nodes,edges,channels,field,index,phase,complete,progress});

pub struct DagJsonSource<'a,S:DagOutputSource+?Sized>{pub source:&'a S,pub kind:DagOutputKind}

pub fn absent()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"DAG publication source changed")}

pub fn channel<'a>(value:DagChannelView<'a>,path:&[usize])->Result<JsonWriteNode<'a>,ValueError>{match path{[]=>Ok(JsonWriteNode::Object(3)),[0]=>Ok(JsonWriteNode::String(value.widget_id)),[1]=>Ok(JsonWriteNode::String(value.port)),[2]=>Ok(JsonWriteNode::String(match value.direction{DagChannelDirection::In=>"in",DagChannelDirection::Out=>"out"})),_=>Err(absent())}}

pub fn refusal<'a>(value:DagRefusalView<'a>,path:&[usize])->Result<JsonWriteNode<'a>,ValueError>{match path{[]=>Ok(JsonWriteNode::Object(4)),[0]=>Ok(JsonWriteNode::String(value.source)),[1]=>Ok(JsonWriteNode::Array(value.source_types.len())),[2]=>Ok(JsonWriteNode::String(value.target)),[3]=>Ok(JsonWriteNode::Array(value.target_types.len())),[1,index]=>value.source_types.get(*index).map(|text|JsonWriteNode::String(text)).ok_or_else(absent),[3,index]=>value.target_types.get(*index).map(|text|JsonWriteNode::String(text)).ok_or_else(absent),_=>Err(absent())}}

impl<S:DagOutputSource+?Sized> JsonWriteSource for DagJsonSource<'_,S>{
    fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{match self.kind{
        DagOutputKind::Nodes=>match path{[]=>Ok(JsonWriteNode::Array(self.source.selected_node_count())),[index]=>self.source.selected_node_id(*index).map(JsonWriteNode::String).ok_or_else(absent),_=>Err(absent())},
        DagOutputKind::Edges=>match path{[]=>Ok(JsonWriteNode::Array(self.source.selected_edge_count())),[index]=>self.source.selected_edge_id(*index).map(JsonWriteNode::String).ok_or_else(absent),_=>Err(absent())},
        DagOutputKind::Selection=>match path{[]=>Ok(JsonWriteNode::Object(3)),[0]=>Ok(JsonWriteNode::Array(self.source.selected_node_count())),[1]=>Ok(JsonWriteNode::Array(self.source.selected_edge_count())),[2]=>Ok(JsonWriteNode::Array(self.source.selected_channel_count())),[0,index]=>self.source.selected_node_id(*index).map(JsonWriteNode::String).ok_or_else(absent),[1,index]=>self.source.selected_edge_id(*index).map(JsonWriteNode::String).ok_or_else(absent),[2,index]=>self.source.selected_handle_identity(*index).map(JsonWriteNode::String).ok_or_else(absent),_=>Err(absent())},
        DagOutputKind::Channels=>match path{[]=>Ok(JsonWriteNode::Array(self.source.selected_channel_count())),[index,rest @ ..]=>channel(self.source.selected_channel(*index).ok_or_else(absent)?,rest)},
        DagOutputKind::Hover=>match path{[]=>Ok(JsonWriteNode::Object(2)),[0,rest @ ..]=>match self.source.hovered_channel_view(){Some(value)=>channel(value,rest),None if rest.is_empty()=>Ok(JsonWriteNode::Null),_=>Err(absent())},[1,rest @ ..]=>match self.source.wire_refusal_view(){Some(value)=>refusal(value,rest),None if rest.is_empty()=>Ok(JsonWriteNode::Null),_=>Err(absent())},_=>Err(absent())},
    }}
    fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
        let keys:&[&str]=match(self.kind,path){(DagOutputKind::Selection,[])=>&["nodes","edges","handles"],(DagOutputKind::Channels,[_])|(DagOutputKind::Hover,[0])=>&["widgetId","port","direction"],(DagOutputKind::Hover,[])=>&["channel","refusal"],(DagOutputKind::Hover,[1])=>&["source","sourceTypes","target","targetTypes"],_=>return Err(absent())};keys.get(index).copied().ok_or_else(absent)
    }
}

pub const DAG_SELECTION_TEXT_MAX_OUTPUT_BYTES: usize = 65_536;

/// ⛽️ One cancellable physical-unit grant for a retained DAG cursor.
#[derive(Clone, Copy, Debug)]
pub struct DagSelectionTextGrant {
    pub fuel: u8,
    pub now_milliseconds: u64,
    pub deadline_milliseconds: u64,
    pub cancelled: bool,
    pub interrupted: bool,
}

/// 🚧️ Fail-closed retained DAG cursor faults.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagSelectionTextFault {
    Cancelled,
    Interrupted,
    Deadline,
    NoFuel,
    Limit,
    Sealed,
}

/// 📬️ A census, byte, progress, or terminal result from one DAG cursor grant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagSelectionTextStep {
    Progress { completed: usize, total: usize },
    Census { bytes: usize },
    Byte(u8),
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DagSelectionJsonPhase {
    CensusNode,
    CensusText,
    Open,
    Seek,
    Separator,
    QuoteOpen,
    Text,
    Escape,
    QuoteClose,
    Close,
    Complete,
}

/// 🎯️ Exact-preflight JSON array encoder that inspects or emits at most one unit per grant.
pub struct DagSelectionJsonCursor {
    pub kind: DagSelectionDomain,
    pub phase: DagSelectionJsonPhase,
    node_cursor: usize,
    text_cursor: usize,
    selected_count: usize,
    emitted_count: usize,
    census_bytes: usize,
    pub output_cursor: usize,
    escape: [u8; 6],
    escape_length: u8,
    escape_cursor: u8,
}

impl Default for DagSelectionJsonCursor {
    fn default() -> Self {
        Self {
            kind: DagSelectionDomain::Nodes,
            phase: DagSelectionJsonPhase::CensusNode,
            node_cursor: 0,
            text_cursor: 0,
            selected_count: 0,
            emitted_count: 0,
            census_bytes: 2,
            output_cursor: 0,
            escape: [0; 6],
            escape_length: 0,
            escape_cursor: 0,
        }
    }
}

impl DagSelectionJsonCursor {
    pub fn edges() -> Self {
        Self { kind: DagSelectionDomain::Edges, ..Self::default() }
    }

    fn guard(grant: DagSelectionTextGrant) -> Result<(), DagSelectionTextFault> {
        if grant.cancelled {
            Err(DagSelectionTextFault::Cancelled)
        } else if grant.interrupted {
            Err(DagSelectionTextFault::Interrupted)
        } else if grant.now_milliseconds >= grant.deadline_milliseconds {
            Err(DagSelectionTextFault::Deadline)
        } else if grant.fuel == 0 {
            Err(DagSelectionTextFault::NoFuel)
        } else {
            Ok(())
        }
    }

    pub fn count(&self, source: &impl DagSelectionSource) -> usize { source.selection_candidate_count(self.kind) }

    pub fn item<'a>(&self, source: &'a impl DagSelectionSource, index: usize) -> Option<&'a str> { source.selection_candidate_id(self.kind,index) }

    fn escape(byte: u8) -> ([u8; 6], u8) {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        match byte {
            b'"' => ([b'\\', b'"', 0, 0, 0, 0], 2),
            b'\\' => ([b'\\', b'\\', 0, 0, 0, 0], 2),
            b'\x08' => ([b'\\', b'b', 0, 0, 0, 0], 2),
            b'\x0c' => ([b'\\', b'f', 0, 0, 0, 0], 2),
            b'\n' => ([b'\\', b'n', 0, 0, 0, 0], 2),
            b'\r' => ([b'\\', b'r', 0, 0, 0, 0], 2),
            b'\t' => ([b'\\', b't', 0, 0, 0, 0], 2),
            0x00..=0x1f => ([b'\\', b'u', b'0', b'0', HEX[usize::from(byte >> 4)], HEX[usize::from(byte & 0x0f)]], 6),
            _ => ([byte, 0, 0, 0, 0, 0], 1),
        }
    }

    pub fn step(&mut self, source: &impl DagSelectionSource, grant: DagSelectionTextGrant) -> Result<DagSelectionTextStep, DagSelectionTextFault> {
        Self::guard(grant)?;
        let count = self.count(source);
        match self.phase {
            DagSelectionJsonPhase::CensusNode => {
                if self.node_cursor == count {
                    if self.census_bytes > DAG_SELECTION_TEXT_MAX_OUTPUT_BYTES {
                        return Err(DagSelectionTextFault::Limit);
                    }
                    self.node_cursor = 0;
                    self.text_cursor = 0;
                    self.phase = DagSelectionJsonPhase::Open;
                    return Ok(DagSelectionTextStep::Census { bytes: self.census_bytes });
                }
                if self.item(source, self.node_cursor).is_some() {
                    self.census_bytes = self.census_bytes.checked_add(2 + usize::from(self.selected_count != 0)).ok_or(DagSelectionTextFault::Limit)?;
                    self.phase = DagSelectionJsonPhase::CensusText;
                } else {
                    self.node_cursor += 1;
                }
                Ok(DagSelectionTextStep::Progress { completed: self.node_cursor, total: count })
            }
            DagSelectionJsonPhase::CensusText => {
                let text = self.item(source, self.node_cursor).ok_or(DagSelectionTextFault::Limit)?.as_bytes();
                if self.text_cursor == text.len() {
                    self.selected_count += 1;
                    self.node_cursor += 1;
                    self.text_cursor = 0;
                    self.phase = DagSelectionJsonPhase::CensusNode;
                } else {
                    self.census_bytes = self.census_bytes.checked_add(usize::from(Self::escape(text[self.text_cursor]).1)).ok_or(DagSelectionTextFault::Limit)?;
                    self.text_cursor += 1;
                }
                Ok(DagSelectionTextStep::Progress { completed: self.node_cursor, total: count })
            }
            DagSelectionJsonPhase::Open => {
                self.phase = DagSelectionJsonPhase::Seek;
                self.output_cursor += 1;
                Ok(DagSelectionTextStep::Byte(b'['))
            }
            DagSelectionJsonPhase::Seek => {
                if self.node_cursor == count {
                    self.phase = DagSelectionJsonPhase::Close;
                } else if self.item(source, self.node_cursor).is_some() {
                    self.phase = if self.emitted_count == 0 { DagSelectionJsonPhase::QuoteOpen } else { DagSelectionJsonPhase::Separator };
                } else {
                    self.node_cursor += 1;
                }
                Ok(DagSelectionTextStep::Progress { completed: self.node_cursor, total: count })
            }
            DagSelectionJsonPhase::Separator => {
                self.phase = DagSelectionJsonPhase::QuoteOpen;
                self.output_cursor += 1;
                Ok(DagSelectionTextStep::Byte(b','))
            }
            DagSelectionJsonPhase::QuoteOpen => {
                self.phase = DagSelectionJsonPhase::Text;
                self.output_cursor += 1;
                Ok(DagSelectionTextStep::Byte(b'"'))
            }
            DagSelectionJsonPhase::Text => {
                let text = self.item(source, self.node_cursor).ok_or(DagSelectionTextFault::Limit)?.as_bytes();
                if self.text_cursor == text.len() {
                    self.phase = DagSelectionJsonPhase::QuoteClose;
                    return Ok(DagSelectionTextStep::Progress { completed: self.output_cursor, total: self.census_bytes });
                }
                let (escape, length) = Self::escape(text[self.text_cursor]);
                if length == 1 {
                    self.text_cursor += 1;
                    self.output_cursor += 1;
                    Ok(DagSelectionTextStep::Byte(escape[0]))
                } else {
                    self.escape = escape;
                    self.escape_length = length;
                    self.escape_cursor = 0;
                    self.phase = DagSelectionJsonPhase::Escape;
                    Ok(DagSelectionTextStep::Progress { completed: self.output_cursor, total: self.census_bytes })
                }
            }
            DagSelectionJsonPhase::Escape => {
                let byte = self.escape[usize::from(self.escape_cursor)];
                self.escape_cursor += 1;
                self.output_cursor += 1;
                if self.escape_cursor == self.escape_length {
                    self.text_cursor += 1;
                    self.phase = DagSelectionJsonPhase::Text;
                }
                Ok(DagSelectionTextStep::Byte(byte))
            }
            DagSelectionJsonPhase::QuoteClose => {
                self.emitted_count += 1;
                self.node_cursor += 1;
                self.text_cursor = 0;
                self.phase = DagSelectionJsonPhase::Seek;
                self.output_cursor += 1;
                Ok(DagSelectionTextStep::Byte(b'"'))
            }
            DagSelectionJsonPhase::Close => {
                self.phase = DagSelectionJsonPhase::Complete;
                self.output_cursor += 1;
                Ok(DagSelectionTextStep::Byte(b']'))
            }
            DagSelectionJsonPhase::Complete if self.output_cursor == self.census_bytes => Ok(DagSelectionTextStep::Complete),
            DagSelectionJsonPhase::Complete => Err(DagSelectionTextFault::Limit),
        }
    }
}

fn refuse()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"invalid DAG input facts")}

pub fn closed(value:&DslValue,names:&[&str])->Result<(),ValueError>{let fields=value.as_object().ok_or_else(refuse)?;if fields.len()!=names.len()||fields.iter().any(|(key,_)|!names.contains(&key.as_str())){return Err(refuse())}Ok(())}

fn identifier(value:&DslValue)->Result<(),ValueError>{if value.as_str().is_none_or(str::is_empty){return Err(refuse())}Ok(())}

fn identifiers(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{for entry in value.as_array().ok_or_else(refuse)?{control.step()?;identifier(entry)?;}Ok(())}

fn input_channel(value:&DslValue)->Result<(),ValueError>{closed(value,&["widgetId","port","direction"])?;identifier(value.get("widgetId").ok_or_else(refuse)?)?;identifier(value.get("port").ok_or_else(refuse)?)?;if !matches!(value.get("direction").and_then(DslValue::as_str),Some("in"|"out")){return Err(refuse())}Ok(())}

/// 📥️ Refuses malformed selection before any semantic host changes.
pub fn decode_dag_selection_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<DagSelectionDomains,ValueError>{
    let raw=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();
    admit_dag_selection_value(raw.get(),control)
}

fn admit_dag_selection_value(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<DagSelectionDomains,ValueError>{
    closed(value,&["nodes","edges","handles"])?;
    for key in ["nodes","edges","handles"]{identifiers(value.get(key).ok_or_else(refuse)?,control)?;}
    DagSelectionDomains::from_value_controlled(value,control)
}

/// 🔌️ Admits a complete port list independently of graph selection mutation.
pub fn decode_dag_channels_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<Vec<DagChannelRef>,ValueError>{
    let raw=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();
    admit_dag_channels_value(raw.get(),control)
}

fn admit_dag_channels_value(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Vec<DagChannelRef>,ValueError>{
    for row in value.as_array().ok_or_else(refuse)?{control.step()?;input_channel(row)?;}
    Vec::<DagChannelRef>::from_value_controlled(value,control)
}

/// 🚦️ Admits every status row before replacing accepted host chrome.
pub fn decode_dag_node_statuses_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<DagNodeStatuses,ValueError>{
    let raw=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();
    admit_dag_node_statuses_value(raw.get(),control)
}

fn admit_dag_node_statuses_value(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<DagNodeStatuses,ValueError>{
    for (id,row)in value.as_object().ok_or_else(refuse)?{
        control.step()?;if id.is_empty(){return Err(refuse())}
        match row.get("status").and_then(DslValue::as_str){
            Some("ok"|"queued"|"computing")=>closed(row,&["status"] )?,
            Some("error")=>{closed(row,&["status","message"])?;if row.get("message").and_then(DslValue::as_str).is_none(){return Err(refuse())}},
            Some("blocked")=>{closed(row,&["status","ports"])?;identifiers(row.get("ports").ok_or_else(refuse)?,control)?},
            _=>return Err(refuse()),
        }
    }
    DagNodeStatuses::from_value_controlled(value,control)
}

/// 📤️ Publishes typed selection through the caller's cumulative output authority.
pub fn encode_dag_selection_json(value:&DagSelectionDomains,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{semio_framework_pack_json::to_json_string_controlled(value,control)}

/// 📤️ Publishes typed channels through the same explicit output authority.
pub fn encode_dag_channels_json(value:&Vec<DagChannelRef>,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{semio_framework_pack_json::to_json_string_controlled(value,control)}

/// 🎯️ Publishes projected semantic identities under explicit output authority.
pub fn encode_dag_node_ids_json(value:&Vec<String>,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{semio_framework_pack_json::to_json_string_controlled(value,control)}

/// 🖱️ Admits complete hover facts before publishing an interaction projection.
pub fn decode_dag_hover_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<DagHoverFacts,ValueError>{
    let raw=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();closed(raw.get(),&["channel","refusal"])?;
    let hovered=raw.get().get("channel").ok_or_else(refuse)?;if !hovered.is_null(){control.step()?;input_channel(hovered)?;}
    let refusal=raw.get().get("refusal").ok_or_else(refuse)?;if !refusal.is_null(){control.step()?;closed(refusal,&["source","sourceTypes","target","targetTypes"])?;for key in ["source","target"]{identifier(refusal.get(key).ok_or_else(refuse)?)?;}for key in ["sourceTypes","targetTypes"]{identifiers(refusal.get(key).ok_or_else(refuse)?,control)?;}}
    DagHoverFacts::from_value_controlled(raw.get(),control)
}

/// 📤️ Publishes the declared hover and refusal record under cumulative output authority.
pub fn encode_dag_hover_json(value:&DagHoverFacts,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{semio_framework_pack_json::to_json_string_controlled(value,control)}

#[derive(Clone,Copy)]
pub enum DagInputKind{Progress,Statuses,Selection,Channels}

pub enum DagInputFacts{Progress(DagComputingProgress),Statuses(DagNodeStatuses),Selection(DagSelectionDomains),Channels(Vec<DagChannelRef>)}

pub struct DagInputCursor{pub kind:DagInputKind,grammar:JsonGrammarCursor<DslValue>,pub raw:Option<DslValue>,pub facts:Option<DagInputFacts>,pending_status:Option<DagNodeEvaluationStatus>,pending_key:Option<SharedOwner<String>>,pending_value:Option<SharedOwner<DagNodeEvaluationStatus>>,pending_update:Option<UpdateCursor<DagNodeEvaluationStatus>>,pending_retired:Option<(DagNodeStatuses,UpdateCursor<DagNodeEvaluationStatus>)>,pub phase:u8,pub field:usize,pub index:usize,pub complete:bool,pub progress:RetainedCloneProgress}

impl DagInputCursor{
    pub fn new(kind:DagInputKind)->Self{Self{kind,grammar:JsonGrammarCursor::new(JsonMemberPolicy::Reject),raw:None,facts:None,pending_status:None,pending_key:None,pending_value:None,pending_update:None,pending_retired:None,phase:0,field:0,index:0,complete:false,progress:Default::default()}}
    pub fn take_retired_update(&mut self)->Option<(DagNodeStatuses,UpdateCursor<DagNodeEvaluationStatus>)>{self.pending_retired.take()}
    pub fn restore_retired_update(&mut self,value:(DagNodeStatuses,UpdateCursor<DagNodeEvaluationStatus>)){assert!(self.pending_retired.is_none());self.pending_retired=Some(value);}
    pub fn position(&self)->usize{self.grammar.position()}
    pub fn facts(&self)->Option<&DagInputFacts>{if self.complete{self.facts.as_ref()}else{None}}
    pub fn take_facts(&mut self)->Option<DagInputFacts>{if self.complete{self.facts.take()}else{None}}
    pub fn normal_step_progress(&self)->RetainedCloneProgress{self.progress}
    pub fn normal_step_demands(&self,source:&str)->Result<semio_framework_value::RetirementDemand,ValueError>{
        if self.raw.is_none(){return self.grammar.normal_step_demands(source).map_err(|error|error.into_value_error())}
        let raw=self.raw.as_ref().unwrap();if self.phase==0{match self.kind{DagInputKind::Progress=>closed(raw,&["active","stale"] )?,DagInputKind::Selection=>closed(raw,&["nodes","edges","handles"] )?,_=>{}}}let mut demand=semio_framework_value::RetirementDemand{depth:1,..Default::default()};
        let vector=|value:&DslValue,size:usize|->Result<usize,ValueError>{value.as_array().ok_or_else(refuse)?.len().checked_mul(size).ok_or_else(refuse)};
        demand.capacity_bytes=if self.phase==0{match self.kind{DagInputKind::Progress=>vector(raw.get("stale").ok_or_else(refuse)?,size_of::<String>())?,DagInputKind::Channels=>vector(raw,size_of::<DagChannelRef>())?,_=>0}}else{match self.facts.as_ref().unwrap(){
            DagInputFacts::Selection(_)if self.phase==1=>vector(raw.get(["nodes","edges","handles"][self.field]).ok_or_else(refuse)?,size_of::<String>())?,
            DagInputFacts::Statuses(_)if self.field<raw.as_object().ok_or_else(refuse)?.len()=>{let row=&raw.as_object().unwrap()[self.field].1;if self.phase==1&&row.as_object().ok_or_else(refuse)?.len()>2{return Err(refuse())}match self.phase{
                1 if row.get("status").and_then(DslValue::as_str)==Some("blocked")=>vector(row.get("ports").ok_or_else(refuse)?,size_of::<String>())?,
                3=>SharedOwner::<String>::allocation_bytes(),4=>SharedOwner::<DagNodeEvaluationStatus>::allocation_bytes(),
                6=>{let update=self.pending_update.as_ref().unwrap();demand.copy_bytes=update.next_copy_byte_demand();demand.depth=update.next_depth_demand();update.next_capacity_byte_demand()?},_=>0}},_=>0}};Ok(demand)
    }
    pub fn step(&mut self,source:&str,maximum_units:usize,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<bool,ValueError>{
        self.progress=Default::default();if maximum_units==0||grant.maximum_items==0{return Ok(false)}control.checkpoint()?;if self.pending_retired.is_some(){return Ok(false)}if self.complete{return Ok(true)}
        let demand=self.normal_step_demands(source)?;if demand.copy_bytes>grant.maximum_copy_bytes||demand.capacity_bytes>grant.maximum_capacity_bytes||demand.release_bytes>grant.maximum_release_bytes||demand.depth>grant.maximum_depth{return Ok(false)}
        control.admit_turn_capacity(grant.maximum_capacity_bytes)?;
        if self.raw.is_some(){let before=control.owned_bytes();self.project_one(control,grant)?;self.progress.copied_items=1;self.progress.retained_capacity_bytes=control.owned_bytes()-before;control.step()?;return Ok(self.complete)}
        let result=self.grammar.step(source,1,control,grant).map_err(|error|error.into_value_error());self.progress=self.grammar.normal_step_progress();self.raw=result?;Ok(false)
    }
    fn project_one(&mut self,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<(),ValueError>{
        let raw=self.raw.as_mut().unwrap();
        if self.phase==0{
            self.facts=Some(match self.kind{
                DagInputKind::Progress=>{closed(raw,&["active","stale"])?;let active=raw.get("active").ok_or_else(refuse)?;if !active.is_null(){identifier(active)?;}let stale=control.allocate_vec(array(field(raw,"stale")?)?.len())?;let active=match field(raw,"active")?{DslValue::Null=>None,value=>Some(take_text(value,false)?)};DagInputFacts::Progress(DagComputingProgress{active,stale})},
                DagInputKind::Selection=>{closed(raw,&["nodes","edges","handles"])?;for name in ["nodes","edges","handles"]{array(field(raw,name)?)?;}DagInputFacts::Selection(DagSelectionDomains::default())},
                DagInputKind::Channels=>DagInputFacts::Channels(control.allocate_vec(array(raw)?.len())?),
                DagInputKind::Statuses=>{object(raw)?;DagInputFacts::Statuses(DagNodeStatuses::new())},
            });self.phase=1;return Ok(())
        }
        match self.facts.as_mut().unwrap(){
            DagInputFacts::Progress(facts)=>{let source=array(field(raw,"stale")?)?;if self.index==source.len(){self.complete=true;}else{facts.stale.push(take_text(&mut source[self.index],true)?);self.index+=1;}},
            DagInputFacts::Selection(facts)=>{
                let name=["nodes","edges","handles"][self.field];let source=array(field(raw,name)?)?;let output=match self.field{0=>&mut facts.nodes,1=>&mut facts.edges,_=>&mut facts.handles};
                if self.phase==1{*output=control.allocate_vec(source.len())?;self.phase=2;}
                else if self.index==source.len(){self.field+=1;self.index=0;self.phase=1;self.complete=self.field==3;}
                else{output.push(take_text(&mut source[self.index],true)?);self.index+=1;}
            },
            DagInputFacts::Channels(facts)=>{
                let rows=array(raw)?;if self.index==rows.len(){self.complete=true;return Ok(())}let row=&mut rows[self.index];input_channel(row)?;
                let direction=match row.get("direction").and_then(DslValue::as_str){Some("in")=>DagChannelDirection::In,Some("out")=>DagChannelDirection::Out,_=>return Err(refuse())};
                let widget_id=take_text(field(row,"widgetId")?,true)?;let port=take_text(field(row,"port")?,true)?;facts.push(DagChannelRef{widget_id,port,direction});self.index+=1;
            },
            DagInputFacts::Statuses(facts)=>{
                let rows=object(raw)?;if self.field==rows.len(){self.complete=true;return Ok(())}let(id,row)=&mut rows[self.field];
                if self.phase==1{
                    if id.is_empty(){return Err(refuse())}let status=row.get("status").and_then(DslValue::as_str).ok_or_else(refuse)?;
                    let names:&[&str]=match status{"ok"|"queued"|"computing"=>&["status"],"error"=>&["status","message"],"blocked"=>&["status","ports"],_=>return Err(refuse())};closed(row,names)?;
                    self.pending_status=Some(match status{"ok"=>DagNodeEvaluationStatus::Ok{},"queued"=>DagNodeEvaluationStatus::Queued{},"computing"=>DagNodeEvaluationStatus::Computing{},"error"=>DagNodeEvaluationStatus::Error{message:take_text(field(row,"message")?,false)?},"blocked"=>DagNodeEvaluationStatus::Blocked{ports:control.allocate_vec(array(field(row,"ports")?)?.len())?},_=>return Err(refuse())});
                    self.phase=if matches!(self.pending_status,Some(DagNodeEvaluationStatus::Blocked{..})){2}else{3};self.index=0;
                }else if self.phase==2{
                    let ports=array(field(row,"ports")?)?;if self.index==ports.len(){self.phase=3;}else{let DagNodeEvaluationStatus::Blocked{ports:output}=self.pending_status.as_mut().unwrap()else{unreachable!()};output.push(take_text(&mut ports[self.index],true)?);self.index+=1;}
                }else if self.phase==3{
                    let bytes=SharedOwner::<String>::allocation_bytes();control.charge(bytes)?;let value=std::mem::take(id);
                    match SharedOwner::admit(value,grant){Ok((owner,_))=>self.pending_key=Some(owner),Err((error,value))=>{*id=value;return Err(error)}}self.phase=4;
                }else if self.phase==4{
                    let bytes=SharedOwner::<DagNodeEvaluationStatus>::allocation_bytes();control.charge(bytes)?;let value=self.pending_status.take().unwrap();
                    match SharedOwner::admit(value,grant){Ok((owner,_))=>self.pending_value=Some(owner),Err((error,value))=>{self.pending_status=Some(value);return Err(error)}}self.phase=5;
                }else if self.phase==5{self.pending_update=Some(facts.begin_set_shared(self.pending_key.take().unwrap(),self.pending_value.take().unwrap()));self.phase=6;}
                else if self.phase==6{
                    let cursor=self.pending_update.as_mut().unwrap();self.progress=match cursor.advance_insert_controlled(grant,control)?{RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>progress};if cursor.is_complete(){self.phase=7;}
                }else{
                    let mut cursor=self.pending_update.take().unwrap();let next=cursor.take_result().expect("completed admitted ordered status root");let previous=std::mem::replace(facts,next);self.pending_retired=Some((previous,cursor));self.field+=1;self.phase=1;
                }
            },
        }Ok(())
    }

}

impl RetireOwned for DagInputFacts{
    fn retirement(self)->Box<dyn RetirementCursor>{match self{Self::Progress(value)=>sequence(vec![deferred(value)]),Self::Statuses(value)=>sequence(vec![deferred(value)]),Self::Selection(value)=>sequence(vec![deferred(value)]),Self::Channels(value)=>sequence(vec![deferred(value)])}}
    fn retirement_birth_bytes(&self)->Option<usize>{let bytes=match self{Self::Progress(value)=>deferred_birth_bytes_for(value),Self::Statuses(value)=>deferred_birth_bytes_for(value),Self::Selection(value)=>deferred_birth_bytes_for(value),Self::Channels(value)=>deferred_birth_bytes_for(value)};sequence_birth_bytes(&[bytes])}
    fn controlled_retirement_supported()->bool{true}
}

impl RetireOwned for DagInputCursor{
    fn retirement(self)->Box<dyn RetirementCursor>{sequence(vec![deferred(self.grammar),deferred(self.raw),deferred(self.facts),deferred(self.pending_status),deferred(self.pending_key),deferred(self.pending_value),deferred(self.pending_update),deferred(self.pending_retired)])}
    fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.grammar),deferred_birth_bytes_for(&self.raw),deferred_birth_bytes_for(&self.facts),deferred_birth_bytes_for(&self.pending_status),deferred_birth_bytes_for(&self.pending_key),deferred_birth_bytes_for(&self.pending_value),deferred_birth_bytes_for(&self.pending_update),deferred_birth_bytes_for(&self.pending_retired)])}
    fn controlled_retirement_supported()->bool{true}
}

pub fn array(value:&mut DslValue)->Result<&mut Vec<DslValue>,ValueError>{match value{DslValue::Array(values)=>Ok(values),_=>Err(refuse())}}

fn object(value:&mut DslValue)->Result<&mut Vec<(String,DslValue)>,ValueError>{match value{DslValue::Object(values)=>Ok(values),_=>Err(refuse())}}

pub fn field<'a>(value:&'a mut DslValue,name:&str)->Result<&'a mut DslValue,ValueError>{object(value)?.iter_mut().find(|(key,_)|key==name).map(|(_,value)|value).ok_or_else(refuse)}

fn take_text(value:&mut DslValue,nonempty:bool)->Result<String,ValueError>{if value.as_str().is_none_or(|text|nonempty&&text.is_empty()){return Err(refuse())}let DslValue::String(text)=std::mem::replace(value,DslValue::Null)else{unreachable!()};Ok(text)}

#[derive(Debug)]
pub enum LayoutIoError {Admission(ValueError),Layout(LayoutError)}

impl std::fmt::Display for LayoutIoError {fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Admission(e)=>write!(f,"{e}"),Self::Layout(e)=>write!(f,"{e}")}}}

impl std::error::Error for LayoutIoError {}

impl From<ValueError> for LayoutIoError{fn from(e:ValueError)->Self{Self::Admission(e)}}

impl From<LayoutError> for LayoutIoError{fn from(e:LayoutError)->Self{Self::Layout(e)}}

struct Defaulted<T>(Option<T>);

impl<T> Default for Defaulted<T>{fn default()->Self{Self(None)}}

impl<T> Defaulted<T>{pub fn unwrap_or(self,value:T)->T{self.0.unwrap_or(value)}}

impl<T:semio_framework_value::FromValue> semio_framework_value::FromValue for Defaulted<T>{
 fn from_value(value:semio_framework_value::DslValue)->Result<Self,ValueError>{T::from_value(value).map(|value|Self(Some(value)))}
 fn from_value_controlled(value:&semio_framework_value::DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{T::from_value_controlled(value,control).map(|value|Self(Some(value)))}
 fn default_value_controlled(control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;Ok(Self(None))}
 fn retire_decoded(self){if let Some(value)=self.0{T::retire_decoded(value)}}
}

#[derive(FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ForceAdmission {
 #[value(default)]
 iterations:Defaulted<u32>,
 #[value(default)]
 ideal_edge_length:Defaulted<f64>,
 #[value(default)]
 repulsion_strength:Defaulted<f64>,
 #[value(default)]
 spring_strength:Defaulted<f64>,
 #[value(default)]
 gravity:Defaulted<f64>,
 #[value(default)]
 pub center_x:Option<f64>,
 #[value(default)]
 pub center_y:Option<f64>,
 #[value(default)]
 time_step:Defaulted<f64>,
 #[value(default)]
 velocity_damping:Defaulted<f64>,
 #[value(default)]
 max_speed:Defaulted<f64>,
 #[value(default)]
 pub random_seed:Defaulted<u64>,
 #[value(default)]
 barnes_hut_theta:Defaulted<f64>,
 #[value(default)]
 pairwise_repulsion_max_bodies:Defaulted<u32>,
 #[value(default)]
 pub locked_node_ids:Defaulted<Vec<String>>,
}

impl ForceAdmission {pub fn admit(self,control:&mut NativeDecodeControl<'_>)->Result<ForceGraphLayoutOptions,ValueError>{
 control.checkpoint()?;let defaults=ForceGraphLayoutOptions::default();Ok(ForceGraphLayoutOptions {
 iterations:self.iterations.unwrap_or(defaults.iterations),
 ideal_edge_length:self.ideal_edge_length.unwrap_or(defaults.ideal_edge_length),
 repulsion_strength:self.repulsion_strength.unwrap_or(defaults.repulsion_strength),
 spring_strength:self.spring_strength.unwrap_or(defaults.spring_strength),
 gravity:self.gravity.unwrap_or(defaults.gravity),
 center_x:self.center_x,
 center_y:self.center_y,
 time_step:self.time_step.unwrap_or(defaults.time_step),
 velocity_damping:self.velocity_damping.unwrap_or(defaults.velocity_damping),
 max_speed:self.max_speed.unwrap_or(defaults.max_speed),
 random_seed:self.random_seed.unwrap_or(defaults.random_seed),
 barnes_hut_theta:self.barnes_hut_theta.unwrap_or(defaults.barnes_hut_theta),
 pairwise_repulsion_max_bodies:self.pairwise_repulsion_max_bodies.unwrap_or(defaults.pairwise_repulsion_max_bodies),
 locked_node_ids:self.locked_node_ids.unwrap_or(defaults.locked_node_ids),
 })}}

#[derive(FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
struct TreeAdmission {
 #[value(default)]
 pub layer_spacing:Defaulted<f64>,
 #[value(default)]
 pub sibling_gap:Defaulted<f64>,
 #[value(default)]
 pub direction:Defaulted<String>,
 #[value(default)]
 pub center_x:Option<f64>,
 #[value(default)]
 pub center_y:Option<f64>,
 #[value(default)]
 pub locked_node_ids:Defaulted<Vec<String>>,
}

impl TreeAdmission {pub fn admit(self,control:&mut NativeDecodeControl<'_>)->Result<HierarchicalTreeLayoutOptions,ValueError>{
 let direction=match self.direction.0{Some(value)=>value,None=>{control.charge("downwards".len())?;control.checkpoint()?;default_direction()}};Ok(HierarchicalTreeLayoutOptions {
 layer_spacing:self.layer_spacing.unwrap_or(default_tree_layer_spacing()),
 sibling_gap:self.sibling_gap.unwrap_or(default_tree_sibling_gap()),
 direction:direction,
 center_x:self.center_x,
 center_y:self.center_y,
 locked_node_ids:self.locked_node_ids.unwrap_or(Vec::new()),
 })}}

#[derive(FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
struct DagAdmission {
 #[value(default)]
 pub layer_spacing:Defaulted<f64>,
 #[value(default)]
 pub sibling_gap:Defaulted<f64>,
 #[value(default)]
 pub orientation:Defaulted<DagLayoutOrientation>,
 #[value(default)]
 pub center_x:Option<f64>,
 #[value(default)]
 pub center_y:Option<f64>,
}

impl DagAdmission {pub fn admit(self,control:&mut NativeDecodeControl<'_>)->Result<DagLayoutOptions,ValueError>{
 control.checkpoint()?;let defaults=DagLayoutOptions::default();Ok(DagLayoutOptions {
 layer_spacing:self.layer_spacing.unwrap_or(defaults.layer_spacing),
 sibling_gap:self.sibling_gap.unwrap_or(defaults.sibling_gap),
 orientation:self.orientation.unwrap_or(defaults.orientation),
 center_x:self.center_x,
 center_y:self.center_y,
 })}}

#[derive(FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
struct RedrawAdmission {
 pub mode:String,
 #[value(default)] pub center_x:Option<f64>,
 #[value(default)] pub center_y:Option<f64>,
 #[value(default)] pub random_seed:Option<u64>,
 #[value(default)] pub redraw_handles_after:bool,
 #[value(default)] pub locked_node_ids:Vec<String>,
 #[value(default)] pub force_graph:Option<ForceAdmission>,
 #[value(default)] pub hierarchical_tree:Option<TreeAdmission>,
}

/// 📥️ Defaults are admitted in IO before publishing typed semantic options.
pub fn decode_redraw_options_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<RedrawLayoutOptions,ValueError>{let raw:RedrawAdmission=semio_framework_pack_json::from_json_str_controlled(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?;Ok(RedrawLayoutOptions{mode:raw.mode,center_x:raw.center_x,center_y:raw.center_y,random_seed:raw.random_seed,redraw_handles_after:raw.redraw_handles_after,locked_node_ids:raw.locked_node_ids,force_graph:raw.force_graph.map(|v|v.admit(control)).transpose()?,hierarchical_tree:raw.hierarchical_tree.map(|v|v.admit(control)).transpose()?})}

/// 📥️ Layered options use the same declared missing-field defaults under byte admission.
pub fn decode_dag_options_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<DagLayoutOptions,ValueError>{let raw:DagAdmission=semio_framework_pack_json::from_json_str_controlled(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?;raw.admit(control)}

/// ✅️ Selection snapshot maintained by the engine hot path.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Selection {
    pub edge_ids: semio_framework_value::numeric_scratch::NumericSet<EdgeId>,
    pub handle_ids: semio_framework_value::numeric_scratch::NumericSet<HandleId>,
    pub node_ids: semio_framework_value::numeric_scratch::NumericSet<NodeId>,
}

impl semio_framework_value::retirement::RetireOwned for Selection {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::{sequence,deferred};sequence(vec![deferred(self.edge_ids),deferred(self.handle_ids),deferred(self.node_ids)])}
    fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.edge_ids),deferred_birth_bytes_for(&self.handle_ids),deferred_birth_bytes_for(&self.node_ids)])}
    fn controlled_retirement_supported()->bool{true}
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct BoardPalette {
 pub raster_clear:[u8;4],
 pub grid_minor_stroke:[u8;4],
 pub edge_stroke:[u8;4],
 pub edge_stroke_hovered:[u8;4],
 pub edge_stroke_selected:[u8;4],
 pub edge_stroke_selection_exit:[u8;4],
 pub edge_stroke_disabled:[u8;4],
 pub node_fill:[u8;4],
 pub node_stroke:[u8;4],
 pub node_fill_hovered:[u8;4],
 pub node_stroke_hovered:[u8;4],
 pub node_fill_selected:[u8;4],
 pub node_stroke_selected:[u8;4],
 pub node_fill_selection_exit:[u8;4],
 pub node_stroke_selection_exit:[u8;4],
 pub node_fill_disabled:[u8;4],
 pub node_stroke_disabled:[u8;4],
 pub node_stroke_computing:[u8;4],
 pub node_stroke_stale:[u8;4],
 pub node_stroke_error:[u8;4],
 pub node_stroke_blocked:[u8;4],
 pub indirect_handle_fill:[u8;4],
 pub indirect_handle_stroke:[u8;4],
 pub handle_fill:[u8;4],
 pub handle_stroke:[u8;4],
 pub handle_fill_hovered:[u8;4],
 pub handle_stroke_hovered:[u8;4],
 pub handle_fill_selected:[u8;4],
 pub handle_stroke_selected:[u8;4],
 pub handle_fill_selection_exit:[u8;4],
 pub handle_stroke_selection_exit:[u8;4],
 pub handle_fill_disabled:[u8;4],
 pub handle_stroke_disabled:[u8;4],
 pub wire_stroke:[u8;4],
 pub wire_stroke_hovered:[u8;4],
 pub wire_stroke_selected:[u8;4],
 pub wire_stroke_highlighted:[u8;4],
 pub wire_stroke_disabled:[u8;4],
 pub selection_preview_fill:[u8;4],
 pub selection_preview_stroke:[u8;4],
 pub label_fill:[u8;4],
 pub label_fill_hovered:[u8;4],
 pub label_halo:[u8;4],
 pub minimap_widget_panel_fill:[u8;4],
 pub minimap_widget_panel_stroke:[u8;4],
 pub minimap_widget_viewport_fill:[u8;4],
 pub minimap_widget_viewport_stroke:[u8;4],
 pub minimap_widget_viewport_stroke_hovered:[u8;4],
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub struct BoardPaletteOverlay {
 pub raster_clear:Option<[u8;4]>,
 pub grid_minor_stroke:Option<[u8;4]>,
 pub edge_stroke:Option<[u8;4]>,
 pub edge_stroke_hovered:Option<[u8;4]>,
 pub edge_stroke_selected:Option<[u8;4]>,
 pub edge_stroke_selection_exit:Option<[u8;4]>,
 pub edge_stroke_disabled:Option<[u8;4]>,
 pub node_fill:Option<[u8;4]>,
 pub node_stroke:Option<[u8;4]>,
 pub node_fill_hovered:Option<[u8;4]>,
 pub node_stroke_hovered:Option<[u8;4]>,
 pub node_fill_selected:Option<[u8;4]>,
 pub node_stroke_selected:Option<[u8;4]>,
 pub node_fill_selection_exit:Option<[u8;4]>,
 pub node_stroke_selection_exit:Option<[u8;4]>,
 pub node_fill_disabled:Option<[u8;4]>,
 pub node_stroke_disabled:Option<[u8;4]>,
 pub node_stroke_computing:Option<[u8;4]>,
 pub node_stroke_stale:Option<[u8;4]>,
 pub node_stroke_error:Option<[u8;4]>,
 pub node_stroke_blocked:Option<[u8;4]>,
 pub indirect_handle_fill:Option<[u8;4]>,
 pub indirect_handle_stroke:Option<[u8;4]>,
 pub handle_fill:Option<[u8;4]>,
 pub handle_stroke:Option<[u8;4]>,
 pub handle_fill_hovered:Option<[u8;4]>,
 pub handle_stroke_hovered:Option<[u8;4]>,
 pub handle_fill_selected:Option<[u8;4]>,
 pub handle_stroke_selected:Option<[u8;4]>,
 pub handle_fill_selection_exit:Option<[u8;4]>,
 pub handle_stroke_selection_exit:Option<[u8;4]>,
 pub handle_fill_disabled:Option<[u8;4]>,
 pub handle_stroke_disabled:Option<[u8;4]>,
 pub wire_stroke:Option<[u8;4]>,
 pub wire_stroke_hovered:Option<[u8;4]>,
 pub wire_stroke_selected:Option<[u8;4]>,
 pub wire_stroke_highlighted:Option<[u8;4]>,
 pub wire_stroke_disabled:Option<[u8;4]>,
 pub selection_preview_fill:Option<[u8;4]>,
 pub selection_preview_stroke:Option<[u8;4]>,
 pub label_fill:Option<[u8;4]>,
 pub label_fill_hovered:Option<[u8;4]>,
 pub label_halo:Option<[u8;4]>,
 pub minimap_widget_panel_fill:Option<[u8;4]>,
 pub minimap_widget_panel_stroke:Option<[u8;4]>,
 pub minimap_widget_viewport_fill:Option<[u8;4]>,
 pub minimap_widget_viewport_stroke:Option<[u8;4]>,
 pub minimap_widget_viewport_stroke_hovered:Option<[u8;4]>,
}

pub fn apply_board_palette_overlay(base:&BoardPalette,overlay:&BoardPaletteOverlay)->BoardPalette { BoardPalette {
 raster_clear:overlay.raster_clear.unwrap_or(base.raster_clear),
 grid_minor_stroke:overlay.grid_minor_stroke.unwrap_or(base.grid_minor_stroke),
 edge_stroke:overlay.edge_stroke.unwrap_or(base.edge_stroke),
 edge_stroke_hovered:overlay.edge_stroke_hovered.unwrap_or(base.edge_stroke_hovered),
 edge_stroke_selected:overlay.edge_stroke_selected.unwrap_or(base.edge_stroke_selected),
 edge_stroke_selection_exit:overlay.edge_stroke_selection_exit.unwrap_or(base.edge_stroke_selection_exit),
 edge_stroke_disabled:overlay.edge_stroke_disabled.unwrap_or(base.edge_stroke_disabled),
 node_fill:overlay.node_fill.unwrap_or(base.node_fill),
 node_stroke:overlay.node_stroke.unwrap_or(base.node_stroke),
 node_fill_hovered:overlay.node_fill_hovered.unwrap_or(base.node_fill_hovered),
 node_stroke_hovered:overlay.node_stroke_hovered.unwrap_or(base.node_stroke_hovered),
 node_fill_selected:overlay.node_fill_selected.unwrap_or(base.node_fill_selected),
 node_stroke_selected:overlay.node_stroke_selected.unwrap_or(base.node_stroke_selected),
 node_fill_selection_exit:overlay.node_fill_selection_exit.unwrap_or(base.node_fill_selection_exit),
 node_stroke_selection_exit:overlay.node_stroke_selection_exit.unwrap_or(base.node_stroke_selection_exit),
 node_fill_disabled:overlay.node_fill_disabled.unwrap_or(base.node_fill_disabled),
 node_stroke_disabled:overlay.node_stroke_disabled.unwrap_or(base.node_stroke_disabled),
 node_stroke_computing:overlay.node_stroke_computing.unwrap_or(base.node_stroke_computing),
 node_stroke_stale:overlay.node_stroke_stale.unwrap_or(base.node_stroke_stale),
 node_stroke_error:overlay.node_stroke_error.unwrap_or(base.node_stroke_error),
 node_stroke_blocked:overlay.node_stroke_blocked.unwrap_or(base.node_stroke_blocked),
 indirect_handle_fill:overlay.indirect_handle_fill.unwrap_or(base.indirect_handle_fill),
 indirect_handle_stroke:overlay.indirect_handle_stroke.unwrap_or(base.indirect_handle_stroke),
 handle_fill:overlay.handle_fill.unwrap_or(base.handle_fill),
 handle_stroke:overlay.handle_stroke.unwrap_or(base.handle_stroke),
 handle_fill_hovered:overlay.handle_fill_hovered.unwrap_or(base.handle_fill_hovered),
 handle_stroke_hovered:overlay.handle_stroke_hovered.unwrap_or(base.handle_stroke_hovered),
 handle_fill_selected:overlay.handle_fill_selected.unwrap_or(base.handle_fill_selected),
 handle_stroke_selected:overlay.handle_stroke_selected.unwrap_or(base.handle_stroke_selected),
 handle_fill_selection_exit:overlay.handle_fill_selection_exit.unwrap_or(base.handle_fill_selection_exit),
 handle_stroke_selection_exit:overlay.handle_stroke_selection_exit.unwrap_or(base.handle_stroke_selection_exit),
 handle_fill_disabled:overlay.handle_fill_disabled.unwrap_or(base.handle_fill_disabled),
 handle_stroke_disabled:overlay.handle_stroke_disabled.unwrap_or(base.handle_stroke_disabled),
 wire_stroke:overlay.wire_stroke.unwrap_or(base.wire_stroke),
 wire_stroke_hovered:overlay.wire_stroke_hovered.unwrap_or(base.wire_stroke_hovered),
 wire_stroke_selected:overlay.wire_stroke_selected.unwrap_or(base.wire_stroke_selected),
 wire_stroke_highlighted:overlay.wire_stroke_highlighted.unwrap_or(base.wire_stroke_highlighted),
 wire_stroke_disabled:overlay.wire_stroke_disabled.unwrap_or(base.wire_stroke_disabled),
 selection_preview_fill:overlay.selection_preview_fill.unwrap_or(base.selection_preview_fill),
 selection_preview_stroke:overlay.selection_preview_stroke.unwrap_or(base.selection_preview_stroke),
 label_fill:overlay.label_fill.unwrap_or(base.label_fill),
 label_fill_hovered:overlay.label_fill_hovered.unwrap_or(base.label_fill_hovered),
 label_halo:overlay.label_halo.unwrap_or(base.label_halo),
 minimap_widget_panel_fill:overlay.minimap_widget_panel_fill.unwrap_or(base.minimap_widget_panel_fill),
 minimap_widget_panel_stroke:overlay.minimap_widget_panel_stroke.unwrap_or(base.minimap_widget_panel_stroke),
 minimap_widget_viewport_fill:overlay.minimap_widget_viewport_fill.unwrap_or(base.minimap_widget_viewport_fill),
 minimap_widget_viewport_stroke:overlay.minimap_widget_viewport_stroke.unwrap_or(base.minimap_widget_viewport_stroke),
 minimap_widget_viewport_stroke_hovered:overlay.minimap_widget_viewport_stroke_hovered.unwrap_or(base.minimap_widget_viewport_stroke_hovered),
}}

impl ToValue for BoardPaletteOverlay{fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<DslValue,ValueError>{let count=[self.raster_clear,self.grid_minor_stroke,self.edge_stroke,self.edge_stroke_hovered,self.edge_stroke_selected,self.edge_stroke_selection_exit,self.edge_stroke_disabled,self.node_fill,self.node_stroke,self.node_fill_hovered,self.node_stroke_hovered,self.node_fill_selected,self.node_stroke_selected,self.node_fill_selection_exit,self.node_stroke_selection_exit,self.node_fill_disabled,self.node_stroke_disabled,self.node_stroke_computing,self.node_stroke_stale,self.node_stroke_error,self.node_stroke_blocked,self.indirect_handle_fill,self.indirect_handle_stroke,self.handle_fill,self.handle_stroke,self.handle_fill_hovered,self.handle_stroke_hovered,self.handle_fill_selected,self.handle_stroke_selected,self.handle_fill_selection_exit,self.handle_stroke_selection_exit,self.handle_fill_disabled,self.handle_stroke_disabled,self.wire_stroke,self.wire_stroke_hovered,self.wire_stroke_selected,self.wire_stroke_highlighted,self.wire_stroke_disabled,self.selection_preview_fill,self.selection_preview_stroke,self.label_fill,self.label_fill_hovered,self.label_halo,self.minimap_widget_panel_fill,self.minimap_widget_panel_stroke,self.minimap_widget_viewport_fill,self.minimap_widget_viewport_stroke,self.minimap_widget_viewport_stroke_hovered].into_iter().filter(Option::is_some).count();let mut fields=<Vec<(String,DslValue)> as FromValue>::guard_decoded(control.allocate_vec(count)?);
if let Some(value)=self.raster_clear{let key=control.copy_text("rasterClear")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.grid_minor_stroke{let key=control.copy_text("gridMinorStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.edge_stroke{let key=control.copy_text("edgeStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.edge_stroke_hovered{let key=control.copy_text("edgeStrokeHovered")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.edge_stroke_selected{let key=control.copy_text("edgeStrokeSelected")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.edge_stroke_selection_exit{let key=control.copy_text("edgeStrokeSelectionExit")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.edge_stroke_disabled{let key=control.copy_text("edgeStrokeDisabled")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_fill{let key=control.copy_text("nodeFill")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke{let key=control.copy_text("nodeStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_fill_hovered{let key=control.copy_text("nodeFillHovered")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke_hovered{let key=control.copy_text("nodeStrokeHovered")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_fill_selected{let key=control.copy_text("nodeFillSelected")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke_selected{let key=control.copy_text("nodeStrokeSelected")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_fill_selection_exit{let key=control.copy_text("nodeFillSelectionExit")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke_selection_exit{let key=control.copy_text("nodeStrokeSelectionExit")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_fill_disabled{let key=control.copy_text("nodeFillDisabled")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke_disabled{let key=control.copy_text("nodeStrokeDisabled")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke_computing{let key=control.copy_text("nodeStrokeComputing")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke_stale{let key=control.copy_text("nodeStrokeStale")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke_error{let key=control.copy_text("nodeStrokeError")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.node_stroke_blocked{let key=control.copy_text("nodeStrokeBlocked")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.indirect_handle_fill{let key=control.copy_text("indirectHandleFill")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.indirect_handle_stroke{let key=control.copy_text("indirectHandleStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_fill{let key=control.copy_text("handleFill")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_stroke{let key=control.copy_text("handleStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_fill_hovered{let key=control.copy_text("handleFillHovered")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_stroke_hovered{let key=control.copy_text("handleStrokeHovered")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_fill_selected{let key=control.copy_text("handleFillSelected")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_stroke_selected{let key=control.copy_text("handleStrokeSelected")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_fill_selection_exit{let key=control.copy_text("handleFillSelectionExit")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_stroke_selection_exit{let key=control.copy_text("handleStrokeSelectionExit")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_fill_disabled{let key=control.copy_text("handleFillDisabled")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.handle_stroke_disabled{let key=control.copy_text("handleStrokeDisabled")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.wire_stroke{let key=control.copy_text("wireStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.wire_stroke_hovered{let key=control.copy_text("wireStrokeHovered")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.wire_stroke_selected{let key=control.copy_text("wireStrokeSelected")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.wire_stroke_highlighted{let key=control.copy_text("wireStrokeHighlighted")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.wire_stroke_disabled{let key=control.copy_text("wireStrokeDisabled")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.selection_preview_fill{let key=control.copy_text("selectionPreviewFill")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.selection_preview_stroke{let key=control.copy_text("selectionPreviewStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.label_fill{let key=control.copy_text("labelFill")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.label_fill_hovered{let key=control.copy_text("labelFillHovered")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.label_halo{let key=control.copy_text("labelHalo")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.minimap_widget_panel_fill{let key=control.copy_text("minimapWidgetPanelFill")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.minimap_widget_panel_stroke{let key=control.copy_text("minimapWidgetPanelStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.minimap_widget_viewport_fill{let key=control.copy_text("minimapWidgetViewportFill")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.minimap_widget_viewport_stroke{let key=control.copy_text("minimapWidgetViewportStroke")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
if let Some(value)=self.minimap_widget_viewport_stroke_hovered{let key=control.copy_text("minimapWidgetViewportStrokeHovered")?;let value=value.to_value_controlled(control)?;fields.get_mut().push((key,value));}
Ok(DslValue::Object(fields.take()))}
fn to_value(&self)->DslValue{let mut fields=Vec::new();
if let Some(value)=self.raster_clear{fields.push(("rasterClear".into(),value.to_value()));}
if let Some(value)=self.grid_minor_stroke{fields.push(("gridMinorStroke".into(),value.to_value()));}
if let Some(value)=self.edge_stroke{fields.push(("edgeStroke".into(),value.to_value()));}
if let Some(value)=self.edge_stroke_hovered{fields.push(("edgeStrokeHovered".into(),value.to_value()));}
if let Some(value)=self.edge_stroke_selected{fields.push(("edgeStrokeSelected".into(),value.to_value()));}
if let Some(value)=self.edge_stroke_selection_exit{fields.push(("edgeStrokeSelectionExit".into(),value.to_value()));}
if let Some(value)=self.edge_stroke_disabled{fields.push(("edgeStrokeDisabled".into(),value.to_value()));}
if let Some(value)=self.node_fill{fields.push(("nodeFill".into(),value.to_value()));}
if let Some(value)=self.node_stroke{fields.push(("nodeStroke".into(),value.to_value()));}
if let Some(value)=self.node_fill_hovered{fields.push(("nodeFillHovered".into(),value.to_value()));}
if let Some(value)=self.node_stroke_hovered{fields.push(("nodeStrokeHovered".into(),value.to_value()));}
if let Some(value)=self.node_fill_selected{fields.push(("nodeFillSelected".into(),value.to_value()));}
if let Some(value)=self.node_stroke_selected{fields.push(("nodeStrokeSelected".into(),value.to_value()));}
if let Some(value)=self.node_fill_selection_exit{fields.push(("nodeFillSelectionExit".into(),value.to_value()));}
if let Some(value)=self.node_stroke_selection_exit{fields.push(("nodeStrokeSelectionExit".into(),value.to_value()));}
if let Some(value)=self.node_fill_disabled{fields.push(("nodeFillDisabled".into(),value.to_value()));}
if let Some(value)=self.node_stroke_disabled{fields.push(("nodeStrokeDisabled".into(),value.to_value()));}
if let Some(value)=self.node_stroke_computing{fields.push(("nodeStrokeComputing".into(),value.to_value()));}
if let Some(value)=self.node_stroke_stale{fields.push(("nodeStrokeStale".into(),value.to_value()));}
if let Some(value)=self.node_stroke_error{fields.push(("nodeStrokeError".into(),value.to_value()));}
if let Some(value)=self.node_stroke_blocked{fields.push(("nodeStrokeBlocked".into(),value.to_value()));}
if let Some(value)=self.indirect_handle_fill{fields.push(("indirectHandleFill".into(),value.to_value()));}
if let Some(value)=self.indirect_handle_stroke{fields.push(("indirectHandleStroke".into(),value.to_value()));}
if let Some(value)=self.handle_fill{fields.push(("handleFill".into(),value.to_value()));}
if let Some(value)=self.handle_stroke{fields.push(("handleStroke".into(),value.to_value()));}
if let Some(value)=self.handle_fill_hovered{fields.push(("handleFillHovered".into(),value.to_value()));}
if let Some(value)=self.handle_stroke_hovered{fields.push(("handleStrokeHovered".into(),value.to_value()));}
if let Some(value)=self.handle_fill_selected{fields.push(("handleFillSelected".into(),value.to_value()));}
if let Some(value)=self.handle_stroke_selected{fields.push(("handleStrokeSelected".into(),value.to_value()));}
if let Some(value)=self.handle_fill_selection_exit{fields.push(("handleFillSelectionExit".into(),value.to_value()));}
if let Some(value)=self.handle_stroke_selection_exit{fields.push(("handleStrokeSelectionExit".into(),value.to_value()));}
if let Some(value)=self.handle_fill_disabled{fields.push(("handleFillDisabled".into(),value.to_value()));}
if let Some(value)=self.handle_stroke_disabled{fields.push(("handleStrokeDisabled".into(),value.to_value()));}
if let Some(value)=self.wire_stroke{fields.push(("wireStroke".into(),value.to_value()));}
if let Some(value)=self.wire_stroke_hovered{fields.push(("wireStrokeHovered".into(),value.to_value()));}
if let Some(value)=self.wire_stroke_selected{fields.push(("wireStrokeSelected".into(),value.to_value()));}
if let Some(value)=self.wire_stroke_highlighted{fields.push(("wireStrokeHighlighted".into(),value.to_value()));}
if let Some(value)=self.wire_stroke_disabled{fields.push(("wireStrokeDisabled".into(),value.to_value()));}
if let Some(value)=self.selection_preview_fill{fields.push(("selectionPreviewFill".into(),value.to_value()));}
if let Some(value)=self.selection_preview_stroke{fields.push(("selectionPreviewStroke".into(),value.to_value()));}
if let Some(value)=self.label_fill{fields.push(("labelFill".into(),value.to_value()));}
if let Some(value)=self.label_fill_hovered{fields.push(("labelFillHovered".into(),value.to_value()));}
if let Some(value)=self.label_halo{fields.push(("labelHalo".into(),value.to_value()));}
if let Some(value)=self.minimap_widget_panel_fill{fields.push(("minimapWidgetPanelFill".into(),value.to_value()));}
if let Some(value)=self.minimap_widget_panel_stroke{fields.push(("minimapWidgetPanelStroke".into(),value.to_value()));}
if let Some(value)=self.minimap_widget_viewport_fill{fields.push(("minimapWidgetViewportFill".into(),value.to_value()));}
if let Some(value)=self.minimap_widget_viewport_stroke{fields.push(("minimapWidgetViewportStroke".into(),value.to_value()));}
if let Some(value)=self.minimap_widget_viewport_stroke_hovered{fields.push(("minimapWidgetViewportStrokeHovered".into(),value.to_value()));}
DslValue::Object(fields)}}

impl FromValue for BoardPaletteOverlay{fn from_value(value:DslValue)->Result<Self,ValueError>{let mut accepted=|_|true;let mut control=NativeDecodeControl::new(usize::MAX,&mut accepted);Self::from_value_controlled(&value,&mut control)}
fn from_value_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let DslValue::Object(fields)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Board palette overlay must be an object"))};let mut output=Self::default();let mut seen=[false;48];for (key,value)in fields{control.step()?;let index=match key.as_str(){
"rasterClear"=>0,
"gridMinorStroke"=>1,
"edgeStroke"=>2,
"edgeStrokeHovered"=>3,
"edgeStrokeSelected"=>4,
"edgeStrokeSelectionExit"=>5,
"edgeStrokeDisabled"=>6,
"nodeFill"=>7,
"nodeStroke"=>8,
"nodeFillHovered"=>9,
"nodeStrokeHovered"=>10,
"nodeFillSelected"=>11,
"nodeStrokeSelected"=>12,
"nodeFillSelectionExit"=>13,
"nodeStrokeSelectionExit"=>14,
"nodeFillDisabled"=>15,
"nodeStrokeDisabled"=>16,
"nodeStrokeComputing"=>17,
"nodeStrokeStale"=>18,
"nodeStrokeError"=>19,
"nodeStrokeBlocked"=>20,
"indirectHandleFill"=>21,
"indirectHandleStroke"=>22,
"handleFill"=>23,
"handleStroke"=>24,
"handleFillHovered"=>25,
"handleStrokeHovered"=>26,
"handleFillSelected"=>27,
"handleStrokeSelected"=>28,
"handleFillSelectionExit"=>29,
"handleStrokeSelectionExit"=>30,
"handleFillDisabled"=>31,
"handleStrokeDisabled"=>32,
"wireStroke"=>33,
"wireStrokeHovered"=>34,
"wireStrokeSelected"=>35,
"wireStrokeHighlighted"=>36,
"wireStrokeDisabled"=>37,
"selectionPreviewFill"=>38,
"selectionPreviewStroke"=>39,
"labelFill"=>40,
"labelFillHovered"=>41,
"labelHalo"=>42,
"minimapWidgetPanelFill"=>43,
"minimapWidgetPanelStroke"=>44,
"minimapWidgetViewportFill"=>45,
"minimapWidgetViewportStroke"=>46,
"minimapWidgetViewportStrokeHovered"=>47,
_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Unknown board palette field"))};if seen[index]{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Duplicate board palette field"))}seen[index]=true;let color=<[u8;4] as FromValue>::from_value_controlled(value,control).map_err(|e|e.under(key))?;match index{
0=>output.raster_clear=Some(color),
1=>output.grid_minor_stroke=Some(color),
2=>output.edge_stroke=Some(color),
3=>output.edge_stroke_hovered=Some(color),
4=>output.edge_stroke_selected=Some(color),
5=>output.edge_stroke_selection_exit=Some(color),
6=>output.edge_stroke_disabled=Some(color),
7=>output.node_fill=Some(color),
8=>output.node_stroke=Some(color),
9=>output.node_fill_hovered=Some(color),
10=>output.node_stroke_hovered=Some(color),
11=>output.node_fill_selected=Some(color),
12=>output.node_stroke_selected=Some(color),
13=>output.node_fill_selection_exit=Some(color),
14=>output.node_stroke_selection_exit=Some(color),
15=>output.node_fill_disabled=Some(color),
16=>output.node_stroke_disabled=Some(color),
17=>output.node_stroke_computing=Some(color),
18=>output.node_stroke_stale=Some(color),
19=>output.node_stroke_error=Some(color),
20=>output.node_stroke_blocked=Some(color),
21=>output.indirect_handle_fill=Some(color),
22=>output.indirect_handle_stroke=Some(color),
23=>output.handle_fill=Some(color),
24=>output.handle_stroke=Some(color),
25=>output.handle_fill_hovered=Some(color),
26=>output.handle_stroke_hovered=Some(color),
27=>output.handle_fill_selected=Some(color),
28=>output.handle_stroke_selected=Some(color),
29=>output.handle_fill_selection_exit=Some(color),
30=>output.handle_stroke_selection_exit=Some(color),
31=>output.handle_fill_disabled=Some(color),
32=>output.handle_stroke_disabled=Some(color),
33=>output.wire_stroke=Some(color),
34=>output.wire_stroke_hovered=Some(color),
35=>output.wire_stroke_selected=Some(color),
36=>output.wire_stroke_highlighted=Some(color),
37=>output.wire_stroke_disabled=Some(color),
38=>output.selection_preview_fill=Some(color),
39=>output.selection_preview_stroke=Some(color),
40=>output.label_fill=Some(color),
41=>output.label_fill_hovered=Some(color),
42=>output.label_halo=Some(color),
43=>output.minimap_widget_panel_fill=Some(color),
44=>output.minimap_widget_panel_stroke=Some(color),
45=>output.minimap_widget_viewport_fill=Some(color),
46=>output.minimap_widget_viewport_stroke=Some(color),
47=>output.minimap_widget_viewport_stroke_hovered=Some(color),
_=>unreachable!()}}Ok(output)}}

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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BoardVisibility { pub hidden:Option<bool>, pub visible:Option<bool>, pub locked:Option<bool> }

pub fn board_visible_option(flags:&BoardVisibility)->Option<bool>{flags.hidden.map(|hidden|!hidden).or(flags.visible)}

pub fn board_visible_or_true(flags:&BoardVisibility)->bool{board_visible_option(flags).unwrap_or(true)}

pub fn board_locked_option(flags:&BoardVisibility)->Option<bool>{flags.locked}

fn node_repulsion_radius(node:&BoardNodeSnapshot)->f64 {
 if node.shape==Some(BoardNodeShape::Rectangle){let w=node.width.unwrap_or(40.0);let h=node.height.unwrap_or(40.0);return ((w*w+h*h).sqrt()*0.5).max(8.0);}
 node.radius.filter(|r|r.is_finite()&&*r>0.0).unwrap_or(32.0)
}

    fn core_opts(opts: &ForceGraphLayoutOptions, gx: f64, gy: f64) -> CoreForceLayoutOptions {
        CoreForceLayoutOptions {
            iterations: opts.iterations,
            ideal_edge_length: opts.ideal_edge_length,
            repulsion_strength: opts.repulsion_strength,
            spring_strength: opts.spring_strength,
            gravity: opts.gravity,
            center_x: gx,
            center_y: gy,
            time_step: opts.time_step,
            velocity_damping: opts.velocity_damping,
            max_speed: opts.max_speed,
            random_seed: opts.random_seed,
            barnes_hut_theta: opts.barnes_hut_theta,
            pairwise_repulsion_max_bodies: opts.pairwise_repulsion_max_bodies,
        }
    }

    /// ↔ Resolves edge endpoint strings to node ids (node-id graphs only).
    pub fn resolve_node_id_endpoint(endpoint_id: &str, id_to_index: &HashMap<String, usize>) -> Option<String> {
        id_to_index.contains_key(endpoint_id).then(|| endpoint_id.to_string())
    }

    /// 🕸️ Force layout with a custom endpoint→node-id resolver (ported graphs pass handle lookup here).
    pub fn apply_force_layout_resolved(snapshot: &mut BoardSnapshot, opts: &ForceGraphLayoutOptions, control: &mut LayoutControl<'_>, resolve_node_id: impl Fn(&str, &HashMap<String, usize>) -> Option<String>) -> Result<(), LayoutError> {
        if !["board.ports.directed.v1","board.normal.undirected.v1","trinity.graph","reasoning.wires.identity.snapshot"].contains(&snapshot.schema.as_str()) {return Err(LayoutError::Schema(snapshot.schema.as_str().into()));}
        control.advance(snapshot.nodes.len() as u64 + snapshot.edges.len() as u64)?;
        let edges=&snapshot.edges;
        let nodes=&mut snapshot.nodes;
        if nodes.is_empty() {
            return Ok(());
        }
        let locked_ids: HashSet<String> = opts.locked_node_ids.iter().cloned().collect();
        let mut id_to_index: HashMap<String, usize> = HashMap::new();
        let mut visible_node_indices: Vec<usize> = Vec::new();
        let mut optional_xy: Vec<Option<(f64, f64)>> = Vec::new();
        let mut is_locked: Vec<bool> = Vec::new();
        let mut positions: Vec<Vec2> = Vec::new();
        let mut radii: Vec<f64> = Vec::new();
        for (raw_idx, node) in nodes.iter().enumerate() {
            if !board_visible_or_true(&node.visibility()) {continue;}
            let nid=&node.id;
            let x_opt=node.x;
            let y_opt=node.y;
            let xy = match (x_opt, y_opt) {
                (Some(x), Some(y)) if x.is_finite() && y.is_finite() => Some((x, y)),
                _ => None,
            };
            id_to_index.insert(nid.to_string(), positions.len());
            visible_node_indices.push(raw_idx);
            optional_xy.push(xy);
            is_locked.push(locked_ids.contains(nid));
            positions.push(Vec2::ZERO);
            radii.push(node_repulsion_radius(node));
        }
        let n = positions.len();
        if n == 0 {
            return Ok(());
        }
        let mut sum = Vec2::ZERO;
        let mut finite_ct: u32 = 0;
        for (x, y) in optional_xy.iter().flatten() {
            sum += Vec2::new(*x, *y);
            finite_ct += 1;
        }
        let anchor = if finite_ct > 0 { sum / (finite_ct as f64) } else { Vec2::new(opts.center_x.unwrap_or(0.0), opts.center_y.unwrap_or(0.0)) };
        for i in 0..n {
            positions[i] = if let Some((x, y)) = optional_xy[i] { Vec2::new(x, y) } else { Vec2::ZERO };
        }
        let pin: Vec<Option<Vec2>> = (0..n).map(|i| if is_locked[i] { Some(positions[i]) } else { None }).collect();
        force::seed_positions(&mut positions, &pin, anchor, opts.random_seed);
        let mut edge_pairs: Vec<(usize, usize)> = Vec::new();
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        for e in edges {
            if !board_visible_or_true(&e.visibility()) {continue;}
            let (Some(src),Some(tgt))=(e.source.as_deref(),e.target.as_deref()) else {continue;};
            let Some(a) = resolve_node_id(src, &id_to_index) else {
                continue;
            };
            let Some(b) = resolve_node_id(tgt, &id_to_index) else {
                continue;
            };
            if a == b {
                continue;
            }
            let Some(&ia) = id_to_index.get(&a) else {
                continue;
            };
            let Some(&ib) = id_to_index.get(&b) else {
                continue;
            };
            let lo = ia.min(ib);
            let hi = ia.max(ib);
            if seen.insert((lo, hi)) {
                edge_pairs.push((lo, hi));
            }
        }
        let mut cx = 0.0f64;
        let mut cy = 0.0f64;
        for p in &positions {
            cx += p.x;
            cy += p.y;
        }
        cx /= n as f64;
        cy /= n as f64;
        let gx = opts.center_x.unwrap_or(cx);
        let gy = opts.center_y.unwrap_or(cy);
        let work=(n as u64).saturating_mul(n as u64).saturating_add(edge_pairs.len() as u64).saturating_add(n as u64);
        let mut failure=None;
        let completed=force::run_force_layout_controlled(&mut positions,&radii,&edge_pairs,&pin,&core_opts(opts,gx,gy),&mut |_|match control.advance(work){Ok(())=>true,Err(error)=>{failure=Some(error);false}});
        if !completed {return Err(failure.expect("refused layout work"));}
        control.advance(n as u64)?;
        for (idx, raw_idx) in visible_node_indices.into_iter().enumerate() {
            let Some(node) = nodes.get_mut(raw_idx) else {
                continue;
            };
            node.x=Some(positions[idx].x);
            node.y=Some(positions[idx].y);
        }
        Ok(())
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum TreeDirection {
        Downwards,
        Upwards,
        Right,
        Left,
    }

    impl TreeDirection {
        pub fn parse(s: &str) -> Result<Self, LayoutError> {
            match s.trim().to_ascii_lowercase().as_str() {
                "down" | "downwards" => Ok(Self::Downwards),
                "up" | "upwards" => Ok(Self::Upwards),
                "right" => Ok(Self::Right),
                "left" => Ok(Self::Left),
                _ => Err(LayoutError::InvalidOptions(format!("unknown hierarchical tree direction: {s}"))),
            }
        }
    }

    fn half_extent(node:&BoardNodeSnapshot)->f64 {if node.shape==Some(BoardNodeShape::Rectangle){return (node.width.unwrap_or(40.0).max(node.height.unwrap_or(40.0))*0.5).max(8.0);}node.radius.filter(|r|r.is_finite()&&*r>0.0).unwrap_or(ui_styling::radii::NODE_DEFAULT)}

    const TREE_SUPER_ID: &str = "__tree_super__";

    /** 🌲️ Buchheim et al. (GD 2002) tidy tree: O(n) Reingold–Tilford with even sibling spacing (after pymag-trees listing 12). */
    #[derive(Debug)]
    struct BuchheimNode {
        pub id: String,
        parent: Option<usize>,
        children: Vec<usize>,
        pub x: f64,
        pub y: f64,
        mod_: f64,
        pub thread: Option<usize>,
        ancestor: usize,
        pub change: f64,
        pub shift: f64,
        pub number: i32,
        synthetic: bool,
    }

    fn buchheim_left_brother(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
        let p = nodes[i].parent?;
        let ch = &nodes[p].children;
        let pos = ch.iter().position(|&c| c == i)?;
        if pos == 0 {
            return None;
        }
        Some(ch[pos - 1])
    }

    fn buchheim_leftmost_sibling(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
        let p = nodes[i].parent?;
        let ch = &nodes[p].children;
        if ch.first() == Some(&i) {
            return None;
        }
        ch.first().copied()
    }

    fn buchheim_next_right(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
        if let Some(t) = nodes[i].thread {
            return Some(t);
        }
        nodes[i].children.last().copied()
    }

    fn buchheim_next_left(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
        if let Some(t) = nodes[i].thread {
            return Some(t);
        }
        nodes[i].children.first().copied()
    }

    fn buchheim_ancestor(nodes: &[BuchheimNode], vil: usize, v: usize, default_ancestor: usize) -> usize {
        let par = nodes[v].parent.expect("buchheim ancestor needs parent");
        let pa = nodes[vil].ancestor;
        if nodes[par].children.contains(&pa) {
            pa
        } else {
            default_ancestor
        }
    }

    fn buchheim_move_subtree(nodes: &mut [BuchheimNode], wl: usize, wr: usize, shift: f64) {
        let subtrees = (nodes[wr].number - nodes[wl].number) as f64;
        if subtrees <= 0.0 {
            return;
        }
        nodes[wr].change -= shift / subtrees;
        nodes[wr].shift += shift;
        nodes[wl].change += shift / subtrees;
        nodes[wr].x += shift;
        nodes[wr].mod_ += shift;
    }

    fn buchheim_execute_shifts(nodes: &mut [BuchheimNode], v: usize) {
        let mut shift = 0.0f64;
        let mut change = 0.0f64;
        for &w in nodes[v].children.iter().rev() {
            nodes[w].x += shift;
            nodes[w].mod_ += shift;
            change += nodes[w].change;
            shift += nodes[w].shift + change;
        }
    }

    fn buchheim_apportion(nodes: &mut [BuchheimNode], v: usize, default_ancestor: usize, distance: f64) -> usize {
        let mut default_ancestor = default_ancestor;
        let w = match buchheim_left_brother(nodes, v) {
            Some(w) => w,
            None => return default_ancestor,
        };
        let mut vir = v;
        let mut vor = v;
        let mut vil = w;
        let mut vol = match buchheim_leftmost_sibling(nodes, v) {
            Some(s) => s,
            None => return default_ancestor,
        };
        let mut sir = nodes[v].mod_;
        let mut sor = nodes[v].mod_;
        let mut sil = nodes[vil].mod_;
        let mut sol = nodes[vol].mod_;
        loop {
            let vil_r = buchheim_next_right(nodes, vil);
            let vir_l = buchheim_next_left(nodes, vir);
            if vil_r.is_none() || vir_l.is_none() {
                break;
            }
            vil = vil_r.unwrap();
            vir = vir_l.unwrap();
            let vol_l = buchheim_next_left(nodes, vol);
            let vor_r = buchheim_next_right(nodes, vor);
            if vol_l.is_none() || vor_r.is_none() {
                break;
            }
            vol = vol_l.unwrap();
            vor = vor_r.unwrap();
            nodes[vor].ancestor = v;
            let shift = (nodes[vil].x + sil) - (nodes[vir].x + sir) + distance;
            if shift > 0.0 {
                let a = buchheim_ancestor(nodes, vil, v, default_ancestor);
                buchheim_move_subtree(nodes, a, v, shift);
                sir += shift;
                sor += shift;
            }
            sil += nodes[vil].mod_;
            sir += nodes[vir].mod_;
            sol += nodes[vol].mod_;
            sor += nodes[vor].mod_;
        }
        if let Some(vil_r) = buchheim_next_right(nodes, vil) {
            if buchheim_next_right(nodes, vor).is_none() {
                nodes[vor].thread = Some(vil_r);
                nodes[vor].mod_ += sil - sor;
            }
        } else if buchheim_next_left(nodes, vir).is_some() && buchheim_next_left(nodes, vol).is_none() {
            if let Some(vir_l) = buchheim_next_left(nodes, vir) {
                nodes[vol].thread = Some(vir_l);
                nodes[vol].mod_ += sir - sol;
            }
            default_ancestor = v;
        }
        default_ancestor
    }

    fn buchheim_first_walk(nodes: &mut [BuchheimNode], v: usize, distance: f64,control:&mut LayoutControl<'_>) -> Result<usize,LayoutError> {
        control.advance(1)?;
        if nodes[v].children.is_empty() {
            if buchheim_leftmost_sibling(nodes, v).is_some() {
                let lb = buchheim_left_brother(nodes, v).expect("leaf with leftmost sibling has left brother");
                nodes[v].x = nodes[lb].x + distance;
            } else {
                nodes[v].x = 0.0;
            }
            return Ok(v);
        }
        let mut default_ancestor = nodes[v].children[0];
        for &w in &nodes[v].children.clone() {
            buchheim_first_walk(nodes, w, distance,control)?;
            default_ancestor = buchheim_apportion(nodes, w, default_ancestor, distance);
        }
        buchheim_execute_shifts(nodes, v);
        let c0 = nodes[v].children[0];
        let c1 = *nodes[v].children.last().expect("internal node has children");
        let mid = (nodes[c0].x + nodes[c1].x) * 0.5;
        if let Some(w) = buchheim_left_brother(nodes, v) {
            nodes[v].x = nodes[w].x + distance;
            nodes[v].mod_ = nodes[v].x - mid;
        } else {
            nodes[v].x = mid;
        }
        Ok(v)
    }

    fn buchheim_second_walk(nodes: &mut [BuchheimNode], v: usize, m: f64, depth: i32, min_x: f64,control:&mut LayoutControl<'_>) -> Result<f64,LayoutError> {
        control.advance(1)?;
        nodes[v].x += m;
        nodes[v].y = depth as f64;
        let mut min_x = min_x.min(nodes[v].x);
        for &w in &nodes[v].children.clone() {
            min_x = buchheim_second_walk(nodes, w, m + nodes[v].mod_, depth + 1, min_x,control)?;
        }
        Ok(min_x)
    }

    fn buchheim_third_walk(nodes: &mut [BuchheimNode], v: usize, n: f64,control:&mut LayoutControl<'_>)->Result<(),LayoutError> {
        control.advance(1)?;
        nodes[v].x += n;
        for &c in &nodes[v].children.clone() {
            buchheim_third_walk(nodes, c, n,control)?;
        }
        Ok(())
    }

    fn run_buchheim_layout(control:&mut LayoutControl<'_>,id_to_node: &BTreeMap<String, BoardNodeSnapshot>, roots: &[String], directed: &[(String, String)], depth: &HashMap<String, i32>) -> Result<HashMap<String, (f64, f64)>, LayoutError> {
        let roots_set: HashSet<String> = roots.iter().cloned().collect();
        let mut incoming: HashMap<String, Vec<String>> = HashMap::new();
        for (u, v) in directed {
            incoming.entry(v.clone()).or_default().push(u.clone());
        }
        for v in incoming.values_mut() {
            v.sort();
            v.dedup();
        }
        let mut chosen_parent: HashMap<String, String> = HashMap::new();
        for id in id_to_node.keys() {
            if roots_set.contains(id) {
                continue;
            }
            let ps = incoming.get(id).cloned().unwrap_or_default();
            if ps.is_empty() {
                continue;
            }
            let best = ps
                .iter()
                .min_by_key(|p| {
                    let dp = depth.get(*p).copied().unwrap_or(0);
                    (dp, (*p).clone())
                })
                .expect("non-empty ps")
                .clone();
            chosen_parent.insert(id.clone(), best);
        }
        let mut ordered_ids: Vec<String> = id_to_node.keys().cloned().collect();
        ordered_ids.sort();
        let id_to_idx: HashMap<String, usize> = ordered_ids.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect();
        let super_idx = ordered_ids.len();
        let mut nodes: Vec<BuchheimNode> =
            ordered_ids.iter().map(|id| BuchheimNode { ancestor: 0, change: 0.0, children: vec![], id: id.clone(), mod_: 0.0, number: 0, parent: None, shift: 0.0, synthetic: false, thread: None, x: -1.0, y: 0.0 }).collect();
        nodes.push(BuchheimNode { ancestor: super_idx, change: 0.0, children: vec![], id: TREE_SUPER_ID.to_string(), mod_: 0.0, number: 0, parent: None, shift: 0.0, synthetic: true, thread: None, x: -1.0, y: 0.0 });
        for (i, oid) in ordered_ids.iter().enumerate() {
            let pidx = if roots_set.contains(oid) {
                super_idx
            } else {
                match chosen_parent.get(oid) {
                    Some(p) => *id_to_idx.get(p).ok_or_else(|| LayoutError::InvalidOptions(format!("missing parent index for {p}")))?,
                    None => super_idx,
                }
            };
            nodes[i].parent = Some(pidx);
        }
        for node in &mut nodes {
            node.children.clear();
        }
        for i in 0..super_idx {
            let pi = nodes[i].parent.ok_or_else(|| LayoutError::InvalidOptions("tree node missing parent".into()))?;
            nodes[pi].children.push(i);
        }
        for p in 0..=super_idx {
            let mut ch: Vec<usize> = nodes[p].children.clone();
            ch.sort_by_key(|&c| nodes[c].id.clone());
            nodes[p].children = ch;
        }
        for p in 0..=super_idx {
            if nodes[p].children.is_empty() {
                continue;
            }
            let ch = nodes[p].children.clone();
            for (k, &c) in ch.iter().enumerate() {
                nodes[c].number = (k + 1) as i32;
                nodes[c].ancestor = c;
            }
        }
        let dist = 1.0f64;
        buchheim_first_walk(&mut nodes, super_idx, dist,control)?;
        let min_x = buchheim_second_walk(&mut nodes, super_idx, 0.0, 0, f64::INFINITY,control)?;
        if min_x.is_finite() && min_x < 0.0 {
            buchheim_third_walk(&mut nodes, super_idx, -min_x,control)?;
        }
        let mut out: HashMap<String, (f64, f64)> = HashMap::new();
        for (i, n) in nodes.iter().enumerate() {
            if i == super_idx || n.synthetic {
                continue;
            }
            out.insert(n.id.clone(), (n.x, n.y));
        }
        Ok(out)
    }

    /// 🌳️ Writes node centers: Buchheim tidy-tree on a spanning forest (min-depth parent tie-break id), synthetic multi-root; super-root not serialized.
    pub fn apply_hierarchical_inner(snapshot: &mut BoardSnapshot, opts: &HierarchicalTreeLayoutOptions, control:&mut LayoutControl<'_>) -> Result<(), LayoutError> {
        let dir = TreeDirection::parse(&opts.direction)?;
        if snapshot.schema.as_str()!="board.ports.directed.v1" {return Err(LayoutError::Schema("schema must be board.ports.directed.v1".into()));}
        control.advance(snapshot.nodes.len() as u64 +snapshot.edges.len() as u64)?;
        let edges_json=&snapshot.edges;let nodes=&mut snapshot.nodes;
        if nodes.is_empty() {
            return Ok(());
        }
        let mut handle_to_node: HashMap<String, String> = HashMap::new();
        let mut id_to_node: BTreeMap<String, BoardNodeSnapshot> = BTreeMap::new();
        for node in nodes.iter() {
            if !board_visible_or_true(&node.visibility()){continue;}
            id_to_node.insert(node.id.clone(),node.clone());
            for handle in node.handles.as_deref().unwrap_or(&[]){if board_visible_or_true(&handle.visibility()){handle_to_node.insert(handle.id.clone(),node.id.clone());}}
        }
        if id_to_node.is_empty() {
            return Ok(());
        }
        let mut directed: Vec<(String, String)> = Vec::new();
        let mut seen_dir: HashSet<(String, String)> = HashSet::new();
        for e in edges_json {
            if !board_visible_or_true(&e.visibility()){continue;}
            let (Some(src_h),Some(tgt_h))=(e.source.as_deref(),e.target.as_deref()) else {continue;};
            let source_node_id=handle_to_node.get(src_h).cloned().unwrap_or_else(||src_h.into());
            let target_node_id=handle_to_node.get(tgt_h).cloned().unwrap_or_else(||tgt_h.into());
            if source_node_id == target_node_id {
                continue;
            }
            if !id_to_node.contains_key(&source_node_id) || !id_to_node.contains_key(&target_node_id) {
                continue;
            }
            if seen_dir.insert((source_node_id.clone(), target_node_id.clone())) {
                directed.push((source_node_id, target_node_id));
            }
        }
        let mut incoming_edge_count_by_node: HashMap<String, u32> = HashMap::new();
        for id in id_to_node.keys() {
            incoming_edge_count_by_node.insert(id.clone(), 0);
        }
        for (_source_nid, target_nid) in &directed {
            *incoming_edge_count_by_node.entry(target_nid.clone()).or_insert(0) += 1;
        }
        let mut roots: Vec<String> = Vec::new();
        for node in nodes.iter(){if board_visible_or_true(&node.visibility())&&node.root==Some(true){roots.push(node.id.clone());}}
        roots.sort();
        roots.dedup();
        if roots.is_empty() {
            for (id, &d) in &incoming_edge_count_by_node {
                if d == 0 {
                    roots.push(id.clone());
                }
            }
            roots.sort();
        }
        if roots.is_empty() {
            roots = id_to_node.keys().cloned().collect();
            roots.sort();
        }
        let mut depth: HashMap<String, i32> = HashMap::new();
        for r in &roots {
            depth.insert(r.clone(), 0);
        }
        let cap = directed.len().saturating_mul(3).saturating_add(nodes.len()).saturating_add(8);
        for _ in 0..cap {
            control.advance(directed.len() as u64)?;
            let mut changed = false;
            for (source_nid, target_nid) in &directed {
                let Some(&dp) = depth.get(source_nid) else {
                    continue;
                };
                let nd = dp + 1;
                let cur = *depth.get(target_nid).unwrap_or(&-1);
                if nd > cur {
                    depth.insert(target_nid.clone(), nd);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let max_depth = depth.values().copied().max().unwrap_or(0);
        for id in id_to_node.keys() {
            depth.entry(id.clone()).or_insert(max_depth + 1);
        }
        let raw = run_buchheim_layout(control,&id_to_node, &roots, &directed, &depth)?;
        let mean_half: f64 = id_to_node.values().map(half_extent).sum::<f64>() / id_to_node.len().max(1) as f64;
        let along_scale = (opts.sibling_gap + 2.0 * mean_half).max(8.0);
        let mut pos: HashMap<String, (f64, f64)> = HashMap::new();
        for (id, (bx, by)) in raw {
            let along = bx * along_scale;
            let orth = by * opts.layer_spacing;
            let (lx, ly) = match dir {
                TreeDirection::Downwards => (along, orth),
                TreeDirection::Upwards => (along, -orth),
                TreeDirection::Right => (orth, along),
                TreeDirection::Left => (-orth, along),
            };
            pos.insert(id, (lx, ly));
        }
        let mut minx = f64::INFINITY;
        let mut maxx = f64::NEG_INFINITY;
        let mut miny = f64::INFINITY;
        let mut maxy = f64::NEG_INFINITY;
        for (id, (x, y)) in &pos {
            let h = half_extent(id_to_node.get(id).unwrap());
            minx = minx.min(x - h);
            maxx = maxx.max(x + h);
            miny = miny.min(y - h);
            maxy = maxy.max(y + h);
        }
        if !minx.is_finite() {
            minx = 0.0;
            maxx = 1.0;
            miny = 0.0;
            maxy = 1.0;
        }
        let cx = (minx + maxx) * 0.5;
        let cy = (miny + maxy) * 0.5;
        let gx = opts.center_x.unwrap_or(0.0);
        let gy = opts.center_y.unwrap_or(0.0);
        let dx = gx - cx;
        let dy = gy - cy;
        let locked_set: HashSet<String> = opts.locked_node_ids.iter().cloned().collect();
        let mut pinned_world: HashMap<String, (f64, f64)> = HashMap::new();
        if !locked_set.is_empty() {
            for node in nodes.iter(){if board_visible_or_true(&node.visibility())&&locked_set.contains(&node.id)&&id_to_node.contains_key(&node.id){pinned_world.insert(node.id.clone(),(node.x.unwrap_or(0.0),node.y.unwrap_or(0.0)));}}
        }
        control.advance(pos.len() as u64)?;
        for (id, (x, y)) in pos {
            let (fx, fy) = if let Some(&(px, py)) = pinned_world.get(&id) { (px, py) } else { (x + dx, y + dy) };
            let idx = nodes.iter().position(|n| n.id==id).ok_or_else(|| LayoutError::InvalidOptions(format!("node index {id}")))?;
            nodes[idx].x=Some(fx);nodes[idx].y=Some(fy);
        }
        Ok(())
    }

/// 🌲️ Buchheim tidy-tree on string-labeled directed edges.
pub fn buchheim_positions(roots: &[String], directed: &[(String, String)], depth: &HashMap<String, i32>,control:&mut LayoutControl<'_>) -> Result<HashMap<String, (f64, f64)>,LayoutError> {
    control.advance(roots.len() as u64+directed.len() as u64)?;
    let roots_set: HashSet<String> = roots.iter().cloned().collect();
    let mut incoming: HashMap<String, Vec<String>> = HashMap::new();
    for (u, v) in directed {
        incoming.entry(v.clone()).or_default().push(u.clone());
    }
    for v in incoming.values_mut() {
        v.sort();
        v.dedup();
    }
    let mut chosen_parent: HashMap<String, String> = HashMap::new();
    let mut all_ids: HashSet<String> = HashSet::new();
    for (u, v) in directed {
        all_ids.insert(u.clone());
        all_ids.insert(v.clone());
    }
    for r in roots {
        all_ids.insert(r.clone());
    }
    for id in &all_ids {
        if roots_set.contains(id) {
            continue;
        }
        let ps = incoming.get(id).cloned().unwrap_or_default();
        if ps.is_empty() {
            continue;
        }
        let best = ps
            .iter()
            .min_by_key(|p| {
                let dp = depth.get(*p).copied().unwrap_or(0);
                (dp, (*p).clone())
            })
            .expect("non-empty ps")
            .clone();
        chosen_parent.insert(id.clone(), best);
    }
    let mut ordered_ids: Vec<String> = all_ids.into_iter().collect();
    ordered_ids.sort();
    if ordered_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let id_to_idx: HashMap<String, usize> = ordered_ids.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect();
    let super_idx = ordered_ids.len();
    let mut nodes: Vec<LayeredBuchheimNode> =
        ordered_ids.iter().map(|id| LayeredBuchheimNode { ancestor: 0, change: 0.0, children: vec![], id: id.clone(), mod_: 0.0, number: 0, parent: None, shift: 0.0, synthetic: false, thread: None, x: -1.0, y: 0.0 }).collect();
    nodes.push(LayeredBuchheimNode { ancestor: super_idx, change: 0.0, children: vec![], id: "__tree_super__".into(), mod_: 0.0, number: 0, parent: None, shift: 0.0, synthetic: true, thread: None, x: -1.0, y: 0.0 });
    for (i, oid) in ordered_ids.iter().enumerate() {
        let pidx = if roots_set.contains(oid) {
            super_idx
        } else {
            match chosen_parent.get(oid) {
                Some(p) => *id_to_idx.get(p).unwrap_or(&super_idx),
                None => super_idx,
            }
        };
        nodes[i].parent = Some(pidx);
    }
    for node in nodes.iter_mut() {
        node.children.clear();
    }
    for i in 0..super_idx {
        let pi = nodes[i].parent.expect("parent set for every non-super node in the loop above");
        nodes[pi].children.push(i);
    }
    for p in 0..=super_idx {
        let mut ch = nodes[p].children.clone();
        ch.sort_by_key(|&c| nodes[c].id.clone());
        nodes[p].children = ch;
    }
    for p in 0..=super_idx {
        if nodes[p].children.is_empty() {
            continue;
        }
        let ch = nodes[p].children.clone();
        for (k, &c) in ch.iter().enumerate() {
            nodes[c].number = (k + 1) as i32;
            nodes[c].ancestor = c;
        }
    }
    layered_buchheim_first_walk(&mut nodes, super_idx, 1.0,control)?;
    let min_x = layered_buchheim_second_walk(&mut nodes, super_idx, 0.0, 0, f64::INFINITY,control)?;
    if min_x.is_finite() && min_x < 0.0 {
        layered_buchheim_third_walk(&mut nodes, super_idx, -min_x,control)?;
    }
    let mut out = HashMap::new();
    for (i, n) in nodes.iter().enumerate() {
        if i == super_idx || n.synthetic {
            continue;
        }
        out.insert(n.id.clone(), (n.x, n.y));
    }
    Ok(out)
}

#[derive(Debug)]
struct LayeredBuchheimNode {
    pub id: String,
    parent: Option<usize>,
    children: Vec<usize>,
    pub x: f64,
    pub y: f64,
    mod_: f64,
    pub thread: Option<usize>,
    ancestor: usize,
    pub change: f64,
    pub shift: f64,
    pub number: i32,
    synthetic: bool,
}

fn layered_buchheim_left_brother(nodes: &[LayeredBuchheimNode], i: usize) -> Option<usize> {
    let p = nodes[i].parent?;
    let ch = &nodes[p].children;
    let pos = ch.iter().position(|&c| c == i)?;
    if pos == 0 {
        return None;
    }
    Some(ch[pos - 1])
}

fn layered_buchheim_leftmost_sibling(nodes: &[LayeredBuchheimNode], i: usize) -> Option<usize> {
    let p = nodes[i].parent?;
    let ch = &nodes[p].children;
    if ch.first() == Some(&i) {
        return None;
    }
    ch.first().copied()
}

fn layered_buchheim_next_right(nodes: &[LayeredBuchheimNode], i: usize) -> Option<usize> {
    if let Some(t) = nodes[i].thread {
        return Some(t);
    }
    nodes[i].children.last().copied()
}

fn layered_buchheim_next_left(nodes: &[LayeredBuchheimNode], i: usize) -> Option<usize> {
    if let Some(t) = nodes[i].thread {
        return Some(t);
    }
    nodes[i].children.first().copied()
}

fn layered_buchheim_move_subtree(nodes: &mut [LayeredBuchheimNode], wl: usize, wr: usize, shift: f64) {
    let subtrees = (nodes[wr].number - nodes[wl].number) as f64;
    if subtrees <= 0.0 {
        return;
    }
    nodes[wr].change -= shift / subtrees;
    nodes[wr].shift += shift;
    nodes[wl].change += shift / subtrees;
    nodes[wr].x += shift;
    nodes[wr].mod_ += shift;
}

fn layered_buchheim_execute_shifts(nodes: &mut [LayeredBuchheimNode], v: usize) {
    let mut shift = 0.0f64;
    let mut change = 0.0f64;
    for &w in nodes[v].children.iter().rev() {
        nodes[w].x += shift;
        nodes[w].mod_ += shift;
        change += nodes[w].change;
        shift += nodes[w].shift + change;
    }
}

fn layered_buchheim_apportion(nodes: &mut [LayeredBuchheimNode], v: usize, default_ancestor: usize, distance: f64) -> usize {
    let w = match layered_buchheim_left_brother(nodes, v) {
        Some(w) => w,
        None => return default_ancestor,
    };
    let mut vir = v;
    let mut vil = w;
    let mut vol = layered_buchheim_leftmost_sibling(nodes, v).unwrap_or(v);
    let mut vor = v;
    let mut sir = nodes[v].mod_;
    let mut sil = nodes[vil].mod_;
    loop {
        let vil_r = layered_buchheim_next_right(nodes, vil);
        let vir_l = layered_buchheim_next_left(nodes, vir);
        if vil_r.is_none() || vir_l.is_none() {
            break;
        }
        vil = vil_r.expect("checked Some above");
        vir = vir_l.expect("checked Some above");
        let vol_l = layered_buchheim_next_left(nodes, vol);
        let vor_r = layered_buchheim_next_right(nodes, vor);
        if vol_l.is_none() || vor_r.is_none() {
            break;
        }
        vol = vol_l.expect("checked Some above");
        vor = vor_r.expect("checked Some above");
        nodes[vor].ancestor = v;
        let shift = (nodes[vil].x + sil) - (nodes[vir].x + sir) + distance;
        if shift > 0.0 {
            layered_buchheim_move_subtree(nodes, default_ancestor, v, shift);
            sir += shift;
        }
        sil += nodes[vil].mod_;
        sir += nodes[vir].mod_;
    }
    default_ancestor
}

fn layered_buchheim_first_walk(nodes: &mut [LayeredBuchheimNode], v: usize, distance: f64,control:&mut LayoutControl<'_>) -> Result<usize,LayoutError> {
    control.advance(1)?;
    if nodes[v].children.is_empty() {
        if let Some(lb) = layered_buchheim_left_brother(nodes, v) {
            nodes[v].x = nodes[lb].x + distance;
        } else {
            nodes[v].x = 0.0;
        }
        return Ok(v);
    }
    let mut default_ancestor = nodes[v].children[0];
    for &w in nodes[v].children.clone().iter() {
        layered_buchheim_first_walk(nodes, w, distance,control)?;
        default_ancestor = layered_buchheim_apportion(nodes, w, default_ancestor, distance);
    }
    layered_buchheim_execute_shifts(nodes, v);
    let c0 = nodes[v].children[0];
    let c1 = *nodes[v].children.last().expect("children non-empty per the is_empty check above");
    let mid = (nodes[c0].x + nodes[c1].x) * 0.5;
    if let Some(w) = layered_buchheim_left_brother(nodes, v) {
        nodes[v].x = nodes[w].x + distance;
        nodes[v].mod_ = nodes[v].x - mid;
    } else {
        nodes[v].x = mid;
    }
    Ok(v)
}

fn layered_buchheim_second_walk(nodes: &mut [LayeredBuchheimNode], v: usize, m: f64, depth: i32, min_x: f64,control:&mut LayoutControl<'_>) -> Result<f64,LayoutError> {
    control.advance(1)?;
    nodes[v].x += m;
    nodes[v].y = depth as f64;
    let mut min_x = min_x.min(nodes[v].x);
    for &w in nodes[v].children.clone().iter() {
        min_x = layered_buchheim_second_walk(nodes, w, m + nodes[v].mod_, depth + 1, min_x,control)?;
    }
    Ok(min_x)
}

fn layered_buchheim_third_walk(nodes: &mut [LayeredBuchheimNode], v: usize, n: f64,control:&mut LayoutControl<'_>)->Result<(),LayoutError> {
    control.advance(1)?;
    nodes[v].x += n;
    for &c in nodes[v].children.clone().iter() {
        layered_buchheim_third_walk(nodes, c, n,control)?;
    }
    Ok(())
}

/// 🌲️ Returns positions before a typed snapshot owner commits any changed coordinates.
pub fn layered_positions(node_ids:&HashSet<String>,directed:&[(String,String)],opts:&DagLayoutOptions,control:&mut LayoutControl<'_>)->Result<HashMap<String,(f64,f64)>,LayoutError>{
 control.advance(node_ids.len() as u64 +directed.len() as u64)?;if node_ids.is_empty(){return Ok(HashMap::new());}
    let mut incoming: HashMap<String, u32> = HashMap::new();
    for id in node_ids {
        incoming.insert(id.clone(), 0);
    }
    for (_, v) in directed {
        *incoming.entry(v.clone()).or_insert(0) += 1;
    }
    let roots: Vec<String> = node_ids.iter().filter(|id| incoming.get(*id).copied().unwrap_or(0) == 0).cloned().collect();
    let roots = if roots.is_empty() { node_ids.iter().cloned().collect() } else { roots };
    let mut depth: HashMap<String, i32> = HashMap::new();
    for r in &roots {
        depth.insert(r.clone(), 0);
    }
    for _ in 0..directed.len().saturating_add(node_ids.len()).saturating_add(4) {
        control.advance(directed.len() as u64)?;
        let mut changed = false;
        for (u, v) in directed {
            let Some(&du) = depth.get(u) else {
                continue;
            };
            let nd = du + 1;
            if depth.get(v).copied().unwrap_or(-1) < nd {
                depth.insert(v.clone(), nd);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let pos = buchheim_positions(&roots, directed, &depth,control)?;
    let mut minx = f64::INFINITY;
    let mut maxx = f64::NEG_INFINITY;
    let mut miny = f64::INFINITY;
    let mut maxy = f64::NEG_INFINITY;
    for (x, y) in pos.values() {
        minx = minx.min(*x);
        maxx = maxx.max(*x);
        miny = miny.min(*y);
        maxy = maxy.max(*y);
    }
    let cx = (minx + maxx) * 0.5;
    let cy = (miny + maxy) * 0.5;
    let gx = opts.center_x.unwrap_or(0.0);
    let gy = opts.center_y.unwrap_or(0.0);
    let (dx, dy) = match opts.orientation {
        DagLayoutOrientation::LeftRight => (gx - cy * opts.layer_spacing, gy - cx * opts.sibling_gap),
        DagLayoutOrientation::TopBottom => (gx - cx * opts.sibling_gap, gy - cy * opts.layer_spacing),
    };
    let mut result=HashMap::new();
    for (id,(bx,by)) in pos {control.advance(1)?;let (x,y)=match opts.orientation {DagLayoutOrientation::LeftRight=>(by*opts.layer_spacing+dx,bx*opts.sibling_gap+dy),DagLayoutOrientation::TopBottom=>(bx*opts.sibling_gap+dx,by*opts.layer_spacing+dy)};result.insert(id,(x,y));}Ok(result)
}

/// ⚙️ Force-directed layout parameters for normal undirected node-id graphs.
#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase",deny_unknown_fields)]
pub struct ForceGraphLayoutOptions {
    #[value(default = "default_iterations")]
    pub iterations: u32,
    #[value(default = "default_ideal_edge_length")]
    pub ideal_edge_length: f64,
    #[value(default = "default_repulsion_strength")]
    pub repulsion_strength: f64,
    #[value(default = "default_spring_strength")]
    pub spring_strength: f64,
    #[value(default = "default_gravity")]
    pub gravity: f64,
    #[value(default)]
    pub center_x: Option<f64>,
    #[value(default)]
    pub center_y: Option<f64>,
    #[value(default = "default_time_step")]
    pub time_step: f64,
    #[value(default = "default_velocity_damping")]
    pub velocity_damping: f64,
    #[value(default = "default_max_speed")]
    pub max_speed: f64,
    #[value(default = "default_random_seed")]
    pub random_seed: u64,
    #[value(default = "default_barnes_hut_theta")]
    pub barnes_hut_theta: f64,
    #[value(default = "default_pairwise_repulsion_max_bodies")]
    pub pairwise_repulsion_max_bodies: u32,
    #[value(default)]
    pub locked_node_ids: Vec<String>,
}

fn default_iterations() -> u32 {
    420
}

fn default_ideal_edge_length() -> f64 {
    140.0
}

fn default_repulsion_strength() -> f64 {
    6500.0
}

fn default_spring_strength() -> f64 {
    0.028
}

fn default_gravity() -> f64 {
    0.0
}

fn default_time_step() -> f64 {
    0.85
}

fn default_velocity_damping() -> f64 {
    0.88
}

fn default_max_speed() -> f64 {
    48.0
}

fn default_random_seed() -> u64 {
    0x5eedfaced0
}

fn default_barnes_hut_theta() -> f64 {
    0.78
}

fn default_pairwise_repulsion_max_bodies() -> u32 {
    56
}

impl Default for ForceGraphLayoutOptions {
    fn default() -> Self {
        Self {
            iterations: default_iterations(),
            ideal_edge_length: default_ideal_edge_length(),
            repulsion_strength: default_repulsion_strength(),
            spring_strength: default_spring_strength(),
            gravity: default_gravity(),
            center_x: None,
            center_y: None,
            time_step: default_time_step(),
            velocity_damping: default_velocity_damping(),
            max_speed: default_max_speed(),
            random_seed: default_random_seed(),
            barnes_hut_theta: default_barnes_hut_theta(),
            pairwise_repulsion_max_bodies: default_pairwise_repulsion_max_bodies(),
            locked_node_ids: Vec::new(),
        }
    }
}

/// 🌳️ Buchheim tidy-tree knobs: rank gap, sibling breadth, growth-axis string, optional world anchor for the laid subtree.
#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase",deny_unknown_fields)]
pub struct HierarchicalTreeLayoutOptions {
    #[value(default = "default_tree_layer_spacing")]
    pub layer_spacing: f64,
    #[value(default = "default_tree_sibling_gap")]
    pub sibling_gap: f64,
    #[value(default = "default_direction")]
    pub direction: String,
    #[value(default)]
    pub center_x: Option<f64>,
    #[value(default)]
    pub center_y: Option<f64>,
    /// 📌️ Node ids whose incoming snapshot centers are kept; Buchheim still runs for placement of unlocked nodes.
    #[value(default)]
    pub locked_node_ids: Vec<String>,
}

pub(crate) fn default_tree_layer_spacing() -> f64 {
    120.0
}

pub(crate) fn default_tree_sibling_gap() -> f64 {
    28.0
}

pub(crate) fn default_direction() -> String {
    "downwards".into()
}

impl Default for HierarchicalTreeLayoutOptions {
    fn default() -> Self {
        Self { layer_spacing: default_tree_layer_spacing(), sibling_gap: default_tree_sibling_gap(), direction: default_direction(), center_x: None, center_y: None, locked_node_ids: Vec::new() }
    }
}

#[derive(Debug, Clone, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase",deny_unknown_fields)]
pub struct RedrawLayoutOptions {
    pub mode: String,
    #[value(default)]
    pub center_x: Option<f64>,
    #[value(default)]
    pub center_y: Option<f64>,
    #[value(default)]
    pub random_seed: Option<u64>,
    #[value(default)]
    pub redraw_handles_after: bool,
    #[value(default)]
    pub locked_node_ids: Vec<String>,
    #[value(default)]
    pub force_graph: Option<ForceGraphLayoutOptions>,
    #[value(default)]
    pub hierarchical_tree: Option<HierarchicalTreeLayoutOptions>,
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct LayoutProgress {pub completed:u64,pub limit:u64}

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum LayoutError {Cancelled,WorkLimit,Schema(String),InvalidOptions(String)}

impl std::fmt::Display for LayoutError {fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {match self{Self::Schema(value)=>write!(f,"schema refused: {value}"),other=>write!(f,"{other:?}")}}}

impl std::error::Error for LayoutError {}

pub struct LayoutControl<'a>{pub limit:u64,pub completed:u64,pub progress:&'a mut dyn FnMut(LayoutProgress)->bool}

impl<'a> LayoutControl<'a>{
 pub fn new(limit:u64,progress:&'a mut dyn FnMut(LayoutProgress)->bool)->Self{Self{limit,completed:0,progress}}
 pub fn advance(&mut self,work:u64)->Result<(),LayoutError>{let next=self.completed.checked_add(work).filter(|next|*next<=self.limit).ok_or(LayoutError::WorkLimit)?;if !(self.progress)(LayoutProgress{completed:next,limit:self.limit}){return Err(LayoutError::Cancelled);}self.completed=next;Ok(())}
}

/// 🧭️ Tree layout flow direction for layered DAG positions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum DagLayoutOrientation {
    #[default]
    LeftRight,
    TopBottom,
}

/// 🌲️ Layered DAG layout options for snapshot JSON. `ToValue`/`FromValue` added
/// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass) so
/// `FlowHost::reorganize` can route through `pack::json::from_json_str` instead of
/// `serde_json::from_str` — `FlowCoreError` only has `From<pack::json::JsonError>`, not
/// `From<serde_json::Error>` (see its own docstring), so the old call never actually compiled
/// with the `?` operator once that conversion impl was dropped.
#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase",deny_unknown_fields)]
pub struct DagLayoutOptions {
    #[value(default = "default_dag_layer_spacing")]
    pub layer_spacing: f64,
    #[value(default = "default_dag_sibling_gap")]
    pub sibling_gap: f64,
    #[value(default)]
    pub orientation: DagLayoutOrientation,
    #[value(default)]
    pub center_x: Option<f64>,
    #[value(default)]
    pub center_y: Option<f64>,
}

fn default_dag_layer_spacing() -> f64 {
    120.0
}

fn default_dag_sibling_gap() -> f64 {
    40.0
}

impl Default for DagLayoutOptions {
    fn default() -> Self {
        Self { layer_spacing: default_dag_layer_spacing(), sibling_gap: default_dag_sibling_gap(), orientation: DagLayoutOrientation::default(), center_x: None, center_y: None }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeTipGeometry {
    Arrow,
    FineArrow,
    Diamond,
    Circle,
    Bar,
}

#[derive(Clone, Debug)]
pub struct EdgeTipDef {
    pub geometry: EdgeTipGeometry,
    pub filled: bool,
    pub scale: f64,
}

/// 🔺️ Typed authored edge tip facts, independent of catalog serialization.
#[derive(Clone,Debug,semio_framework_value_derive::ToValue,semio_framework_value_derive::FromValue)]
pub struct EdgeTipCatalogEntry {pub id:String,#[value(default)]pub geometry:Option<EdgeTipGeometry>,#[value(default)]pub filled:Option<bool>,#[value(default)]pub scale:Option<f64>}

impl EdgeTipDef {
    pub fn from_catalog_entry(entry:&EdgeTipCatalogEntry)->Option<Self>{
        let Some(geometry)=entry.geometry else{return Self::builtin_for_id(&entry.id)};
        let filled=entry.filled.unwrap_or_else(||match geometry{EdgeTipGeometry::FineArrow|EdgeTipGeometry::Bar=>false,EdgeTipGeometry::Diamond=>!entry.id.contains("open"),_=>true});
        let scale=entry.scale.filter(|value|value.is_finite()&&*value>0.0).unwrap_or(1.0);
        Some(Self{geometry,filled,scale})
    }

    pub fn builtin_for_id(id: &str) -> Option<Self> {
        match id {
            "arrow" | "filled-arrow" => Some(Self { geometry: EdgeTipGeometry::Arrow, filled: true, scale: 1.0 }),
            "fine-arrow" => Some(Self { geometry: EdgeTipGeometry::FineArrow, filled: false, scale: 1.0 }),
            "filled-diamond" => Some(Self { geometry: EdgeTipGeometry::Diamond, filled: true, scale: 1.0 }),
            "open-diamond" => Some(Self { geometry: EdgeTipGeometry::Diamond, filled: false, scale: 1.0 }),
            _ => None,
        }
    }
}

pub fn builtin_edge_tips() -> BTreeMap<String, EdgeTipDef> {
    let ids = ["arrow", "filled-arrow", "fine-arrow", "filled-diamond", "open-diamond"];
    let mut m = BTreeMap::new();
    for id in ids {
        if let Some(def) = EdgeTipDef::builtin_for_id(id) {
            m.insert(id.to_string(), def);
        }
    }
    m
}

impl semio_framework_value::ToValue for EdgeTipGeometry{
fn to_value(&self)->semio_framework_value::DslValue{semio_framework_value::DslValue::String(match self{Self::Arrow=>"arrow",Self::FineArrow=>"fine-arrow",Self::Diamond=>"diamond",Self::Circle=>"circle",Self::Bar=>"bar"}.into())}
fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_value::DslValue,semio_framework_value::ValueError>{Ok(semio_framework_value::DslValue::String(control.copy_text(match self{Self::Arrow=>"arrow",Self::FineArrow=>"fine-arrow",Self::Diamond=>"diamond",Self::Circle=>"circle",Self::Bar=>"bar"})?))}}

impl semio_framework_value::FromValue for EdgeTipGeometry{
fn from_value(value:semio_framework_value::DslValue)->Result<Self,semio_framework_value::ValueError>{let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(usize::MAX,&mut accepted);Self::from_value_controlled(&value,&mut control)}
fn from_value_controlled(value:&semio_framework_value::DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{control.step()?;match value.as_str(){Some("arrow")=>Ok(Self::Arrow),Some("fine-arrow")=>Ok(Self::FineArrow),Some("diamond")=>Ok(Self::Diamond),Some("circle")=>Ok(Self::Circle),Some("bar")=>Ok(Self::Bar),_=>Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"Unknown edge tip geometry"))}}}

#[derive(Clone, Debug, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct CameraDescriptor {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

/// 🌱️ Owns semantic descriptor values independently of serialized syntax.
#[derive(Clone, Debug, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct NodeDescriptor {
    pub id: String,
    pub x: f64,
    pub y: f64,
    #[serde(default)]
    #[value(default)]
    pub draggable: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub style: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub text: Option<String>,
    /// 🏷️ Runtime host encoding: catalog id from the baked icon table or inline SVG (`<?xml` / `<svg` …) parsed at detail LOD.
    #[serde(default)]
    #[value(default)]
    pub icon_kind: Option<String>,
    /// 🧩️ Semantic node-kind id for compatibility rows at `node` specificity.
    #[serde(default)]
    #[value(default)]
    pub node_kind: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub user_data: Option<semio_framework_value::DslValue>,
    #[serde(default)]
    #[value(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub locked: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub root: Option<bool>,
    #[value(default)]
    pub shape: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub radius: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub width: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub height: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub scale: Option<f64>,
}

/// 🔗️ Requires both semantic endpoint identities without serialized object lookup.
pub fn board_edge_handle_ids<'a>(source:Option<&'a str>,target:Option<&'a str>)->Option<(&'a str,&'a str)>{Some((source?,target?))}
