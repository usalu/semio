//! ⚙️ Board engine: the retained graph engine with camera, selection and hit testing, plus the directed port descriptors it exposes.
//!
//! Crate `semio-framework-os-infinite-board-engine`; a downward producer of the infinite board family.

use semio_framework_os_infinite_board_schema::{apply_board_palette_overlay, apply_force_layout_resolved, apply_hierarchical_inner, board_visible_option, board_visible_or_true, BoardNodeShape, BoardPalette, BoardPaletteOverlay, BoardSnapshot, BoardVisibility, decode_redraw_options_json, field, ForceAdmission, ForceGraphLayoutOptions, HierarchicalTreeLayoutOptions, LayoutControl, LayoutError, LayoutIoError, NodeDescriptor, RedrawLayoutOptions, resolve_node_id_endpoint, Selection};
use semio_framework_canvas::{Affine, Color, CubicBez, FillRule, OpaqueSceneRetirementStep, Point, RasterImage, Rect, Scene, Stroke, Vec2};
use std::sync::Arc;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::cell::{Cell, RefCell};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use serde::{Deserialize, Serialize};
use graph::{Directed, Directedness, EdgeId, ElementSemantics, GraphEdge, Handle, HandleId, HandleRole, Node, NodeId, NodeShape, Normal, orient_endpoints, Ported, PortModel, Undirected};
use semio_framework_value::{DslValue, NativeDecodeControl, NativeEncodeControl, ValueError, ValueRefusalKind};
use semio_framework_os_kernel::os_spr::causal::transition::HistoryFoldIndex;
use semio_framework_pack_json::JsonMemberPolicy;
use std::mem::ManuallyDrop;
use semio_framework_graph_manifest::PropertyBag;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::{deferred, deferred_birth_bytes_for, RetirementCursor, RetirementStep, RetireOwned, sequence, sequence_birth_bytes};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use semio_framework_canvas::{append_svg_document, SvgDocument};
use semio_framework_canvas::geom_sel::{inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box, world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox};
use semio_framework_canvas as canvas;
use semio_framework_graph_drawing::routing::{compute_edge_bezier_outward, handle_exterior_cap_peak, handle_outward_at_node_rim, handle_position_on_circle, handle_position_on_rectangle};
use geometry::{distance_between, distance_point_to_cubic_bezier, normalize_or_zero};
use semio_framework_graph_drawing::force;
use semio_framework_os_infinite_directed_presentation::*;

#[derive(Clone,Copy,Debug)]
pub(crate) struct RetainedPoint(pub Point);

impl std::ops::Deref for RetainedPoint{type Target=Point;fn deref(&self)->&Point{&self.0}}

semio_framework_value::artifact_retire_leaf!(Camera,InteractionMode,ProximityConnection,RetainedPoint);

semio_framework_value::artifact_retire_struct!(EngineSelectionOptions {method,mode,select_nodes,select_handles,select_edges});

struct PointBuffer(Vec<Point>);

struct PointRetirement{pub source:ManuallyDrop<Vec<Point>>,tail_bytes:usize}

impl RetireOwned for PointBuffer{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(PointRetirement{source:ManuallyDrop::new(self.0),tail_bytes:0})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<PointRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}

impl RetirementCursor for PointRetirement{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{
  if grant.maximum_items==0{return RetirementStep::BudgetExhausted}
  if !self.source.is_empty(){let remaining=size_of::<Point>()-self.tail_bytes;let bytes=remaining.min(grant.maximum_copy_bytes);if bytes==0{return RetirementStep::BudgetExhausted}self.tail_bytes+=bytes;if self.tail_bytes==size_of::<Point>(){self.source.pop();self.tail_bytes=0;}return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})}
  let bytes=self.source.capacity()*size_of::<Point>();if bytes>grant.maximum_release_bytes{return RetirementStep::BudgetExhausted}if bytes>0{drop(std::mem::take(&mut *self.source));return RetirementStep::Bytes(bytes)}RetirementStep::Complete
 }
 fn terminal_is_empty(&self)->bool{self.source.is_empty()&&self.source.capacity()==0&&self.tail_bytes==0}
 fn next_work_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(usize::from(!self.source.is_empty()))}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(if self.source.is_empty(){self.source.capacity()*size_of::<Point>()}else{0})}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}

impl Drop for PointRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original point backing abandoned");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.source)}}}}

impl RetireOwned for BoardEvent{
 fn retirement(self)->Box<dyn RetirementCursor>{match self{Self::HoverChanged{id}=>id.retirement(),Self::NodeMoved{id,x,y}=>(id,x,y).retirement(),Self::EdgeConnected{id,source,target}=>(id,source,target).retirement(),Self::EdgeRemoved{id}=>id.retirement(),Self::SelectionChanged{edge_ids,handle_ids,node_ids}=>(edge_ids,handle_ids,node_ids).retirement(),Self::PreselectChanged{edge_ids,handle_ids,node_ids,removed_edge_ids,removed_handle_ids,removed_node_ids}=>sequence(vec![deferred(edge_ids),deferred(handle_ids),deferred(node_ids),deferred(removed_edge_ids),deferred(removed_handle_ids),deferred(removed_node_ids)])}}
 fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::HoverChanged{id}=>id.retirement_birth_bytes(),Self::NodeMoved{id,x,y}=>(*id,*x,*y).retirement_birth_bytes(),Self::EdgeConnected{id,source,target}=>(*id,*source,*target).retirement_birth_bytes(),Self::EdgeRemoved{id}=>id.retirement_birth_bytes(),Self::SelectionChanged{edge_ids,handle_ids,node_ids}=>sequence_birth_bytes(&[deferred_birth_bytes_for(edge_ids),deferred_birth_bytes_for(handle_ids),deferred_birth_bytes_for(node_ids)]),Self::PreselectChanged{edge_ids,handle_ids,node_ids,removed_edge_ids,removed_handle_ids,removed_node_ids}=>sequence_birth_bytes(&[deferred_birth_bytes_for(edge_ids),deferred_birth_bytes_for(handle_ids),deferred_birth_bytes_for(node_ids),deferred_birth_bytes_for(removed_edge_ids),deferred_birth_bytes_for(removed_handle_ids),deferred_birth_bytes_for(removed_node_ids)])}}
 fn controlled_retirement_supported()->bool{true}
}

impl<P:GraphPortModel+Send+'static,D:Directedness+Send+'static> RetireOwned for GraphEngine<P,D> where P::Endpoint:RetireOwned {
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{camera,edges,edge_semantics,enforce_acyclic,events,handles,hover,interaction,nodes,selection,preselect,preselect_removed,selection_options,handle_pointer_picking,proximity_distance_world,proximity_distance_override,selection_preview_points,selection_preview_crossing,area_initial,area_points,area_screen_points,drag_start_positions,proximity_connection,next_edge_id,retirement_backing_credited_bytes,_directedness:_,_port:_}=self;sequence(vec![deferred(camera),deferred(edges),deferred(edge_semantics),deferred(enforce_acyclic),deferred(events),deferred(handles),deferred(hover),deferred(interaction),deferred(nodes),deferred(selection),deferred(preselect),deferred(preselect_removed),deferred(selection_options),deferred(handle_pointer_picking),deferred(proximity_distance_world),deferred(proximity_distance_override),deferred(PointBuffer(selection_preview_points)),deferred(selection_preview_crossing),deferred(area_initial),deferred(PointBuffer(area_points)),deferred(PointBuffer(area_screen_points)),deferred(drag_start_positions),deferred(proximity_connection),deferred(next_edge_id),deferred(retirement_backing_credited_bytes)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.camera),deferred_birth_bytes_for(&self.edges),deferred_birth_bytes_for(&self.edge_semantics),deferred_birth_bytes_for(&self.enforce_acyclic),deferred_birth_bytes_for(&self.events),deferred_birth_bytes_for(&self.handles),deferred_birth_bytes_for(&self.hover),deferred_birth_bytes_for(&self.interaction),deferred_birth_bytes_for(&self.nodes),deferred_birth_bytes_for(&self.selection),deferred_birth_bytes_for(&self.preselect),deferred_birth_bytes_for(&self.preselect_removed),deferred_birth_bytes_for(&self.selection_options),deferred_birth_bytes_for(&self.handle_pointer_picking),deferred_birth_bytes_for(&self.proximity_distance_world),deferred_birth_bytes_for(&self.proximity_distance_override),semio_framework_value::retirement::deferred_birth_bytes::<PointBuffer>(),deferred_birth_bytes_for(&self.selection_preview_crossing),deferred_birth_bytes_for(&self.area_initial),semio_framework_value::retirement::deferred_birth_bytes::<PointBuffer>(),semio_framework_value::retirement::deferred_birth_bytes::<PointBuffer>(),deferred_birth_bytes_for(&self.drag_start_positions),deferred_birth_bytes_for(&self.proximity_connection),deferred_birth_bytes_for(&self.next_edge_id),deferred_birth_bytes_for(&self.retirement_backing_credited_bytes)])}
 fn controlled_retirement_supported()->bool{P::Endpoint::controlled_retirement_supported()}
}

pub type UndirectedGraphEngine=GraphEngine<Normal,Undirected>;

/// ➡️ Node graph engine without ports; relationships are directed node pairs.
pub type DirectedGraphEngine = GraphEngine<Normal, Directed>;

/// ↔ Port graph engine; handle endpoints are unordered pairs.
pub type UndirectedPortGraphEngine = GraphEngine<Ported, Undirected>;

/// 🪢️ Port edge with handle endpoints (unordered).
pub type UndirectedEdge = GraphEdge<HandleId>;

semio_framework_value::artifact_retire_leaf!(CanvasPalette);

// #region 🔖️GraphPortMode
/// 🔌️ Runtime port-model axis: ported graphs use handles; normal graphs connect node ids directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum GraphPortMode {
    #[default]
    Ported,
    Normal,
}

impl GraphPortMode {
    pub fn has_ports(self) -> bool {
        self == GraphPortMode::Ported
    }
}
// #endregion 🔖️GraphPortMode

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardElementStyleKind {
    Original,
    Neutral,
    Hovered,
    Selected,
    Highlighted,
    Disabled,
}

#[derive(Clone, Debug)]
pub struct NodeData {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub shape: NodeShape,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
    pub scale: f64,
    pub draggable: bool,
    pub selected: bool,
    pub visible: bool,
    pub locked: bool,
    pub root: bool,
    pub style: Option<String>,
    pub text: Option<String>,
    pub icon_kind: Option<String>,
    pub node_kind: String,
    pub properties: semio_framework_graph_manifest::PropertyBag,
}

#[derive(Clone, Debug)]
pub struct WireKindDef {
    pub name: String,
    pub default_edge_kind: Option<String>,
}

#[derive(Clone, Debug)]
pub struct NodeKindDef {
    pub name: String,
    pub scale: f64,
    pub shape: NodeShape,
    pub handles: Vec<NodeKindHandleTemplate>,
    pub icon: Option<String>,
    pub color_fill: Option<Color>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveUtility {
    Select,
    Brush,
    /// 🖍️ Paints target regions: a click-drag rectangle, or a click that drops one of the
    /// configured brush extent. Never picks, never marquees.
    AreaBrush,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeStrokePattern {
    Solid,
    Dashed,
    Dotted,
}

#[derive(Clone, Debug)]
pub struct EdgeKindDef {
    pub name: String,
    pub color: Option<Color>,
    pub stroke_width: f64,
    pub pattern: EdgeStrokePattern,
    pub source_tip: Option<String>,
    pub target_tip: Option<String>,
    pub directed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompatSpecificity {
    General = 0,
    Node = 1,
    Edge = 2,
    Handle = 3,
    Wire = 4,
}

#[derive(Clone, Debug)]
pub struct LinkCompatRule {
    pub source: String,
    pub target: String,
    pub bidirectional: bool,
    pub important: bool,
    pub specificity: CompatSpecificity,
}

#[derive(Clone, Debug)]
pub struct EdgeData {
    pub id: String,
    pub source: String,
    pub target: String,
    pub selected: bool,
    pub visible: bool,
    pub locked: bool,
    pub style: Option<String>,
    pub edge_kind: String,
    pub source_tip: Option<String>,
    pub target_tip: Option<String>,
    pub properties: semio_framework_graph_manifest::PropertyBag,
}

#[derive(Clone, Debug)]
pub struct WireData {
    pub id: String,
    pub source: String,
    pub target: Option<String>,
    pub end_x: Option<f64>,
    pub end_y: Option<f64>,
    pub selected: bool,
    pub visible: bool,
    pub locked: bool,
    pub style: Option<String>,
    pub wire_kind: String,
    pub properties: semio_framework_graph_manifest::PropertyBag,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectionOptions {
    pub method: String,
    pub mode: String,
    pub select_nodes: bool,
    pub select_edges: bool,
    pub select_handles: bool,
}

/// 🪪️ One pointer gesture's identity and the selection change its press staged. The press changes the
/// engine selection at once; its `select` row waits for the release, so `select` and the gesture record
/// leave as ONE batch tagged with `id`, and a cancel restores `restore` and publishes nothing.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GestureStage {
    pub id: String,
    pub select: Option<(Vec<String>, Option<String>)>,
    pub restore: Option<BTreeSet<String>>,
}

#[derive(Clone, Debug, Default)]
pub enum Interaction {
    #[default]
    None,
    Pan { origin: Camera, start_screen: Point },
    DragNodes { offset: Vec2, primary_id: String, start_positions: BTreeMap<String, (f64, f64)>, proximity_pair: Option<(String, String)>, gesture: GestureStage, delta: Vec2 },
    SelectionPending { initial_ids: BTreeSet<String>, start: Point, start_screen: Point },
    Selection { initial_ids: BTreeSet<String>, points: Vec<Point>, screen_points: Vec<Point>, start: Point, start_screen: Point },
    LinkAtSourceHandle { source_id: String, start_screen: Point },
    LinkDragSnap { source_id: String, target_id: Option<String>, end_world: Point },
    LinkTargetNode { source_id: String, target_node_id: String },
    ExternalLinkPreview { source_id: String, end_world: Point, compatible_node_ids: Vec<String>, ring_node_id: Option<String>, ring_handle_ids: Vec<String> },
}

#[derive(Clone, Copy, Debug)]
pub struct CanvasPalette {
    pub raster_clear: Color,
    pub grid_minor_stroke: Color,
    pub edge_stroke: Color,
    pub edge_stroke_hovered: Color,
    pub edge_stroke_selected: Color,
    pub edge_stroke_selection_exit: Color,
    pub edge_stroke_disabled: Color,
    pub node_fill: Color,
    pub node_stroke: Color,
    pub node_fill_hovered: Color,
    pub node_stroke_hovered: Color,
    pub node_fill_selected: Color,
    pub node_stroke_selected: Color,
    pub node_fill_selection_exit: Color,
    pub node_stroke_selection_exit: Color,
    pub node_fill_disabled: Color,
    pub node_stroke_disabled: Color,
    pub node_stroke_computing: Color,
    pub node_stroke_stale: Color,
    pub node_stroke_error: Color,
    pub node_stroke_blocked: Color,
    pub indirect_handle_fill: Color,
    pub indirect_handle_stroke: Color,
    pub handle_fill: Color,
    pub handle_stroke: Color,
    pub handle_fill_hovered: Color,
    pub handle_stroke_hovered: Color,
    pub handle_fill_selected: Color,
    pub handle_stroke_selected: Color,
    pub handle_fill_selection_exit: Color,
    pub handle_stroke_selection_exit: Color,
    pub handle_fill_disabled: Color,
    pub handle_stroke_disabled: Color,
    pub wire_stroke: Color,
    pub wire_stroke_hovered: Color,
    pub wire_stroke_selected: Color,
    pub wire_stroke_highlighted: Color,
    pub wire_stroke_disabled: Color,
    pub selection_preview_fill: Color,
    pub selection_preview_stroke: Color,
    pub label_fill: Color,
    pub label_fill_hovered: Color,
    pub label_halo: Color,
    pub minimap_widget_panel_fill: Color,
    pub minimap_widget_panel_stroke: Color,
    pub minimap_widget_viewport_fill: Color,
    pub minimap_widget_viewport_stroke: Color,
    pub minimap_widget_viewport_stroke_hovered: Color,
}

impl CanvasPalette {
    /// 🎨️ Builds a palette from centralized board theme tokens.
    pub fn from_board_palette(t: &ui_styling::BoardPalette) -> Self {
        Self {
            raster_clear: Color::new(t.raster_clear),
            grid_minor_stroke: Color::new(t.grid_minor_stroke),
            edge_stroke: Color::new(t.edge_stroke),
            edge_stroke_hovered: Color::new(t.edge_stroke_hovered),
            edge_stroke_selected: Color::new(t.edge_stroke_selected),
            edge_stroke_selection_exit: Color::new(t.edge_stroke_selection_exit),
            edge_stroke_disabled: Color::new(t.edge_stroke_disabled),
            node_fill: Color::new(t.node_fill),
            node_stroke: Color::new(t.node_stroke),
            node_fill_hovered: Color::new(t.node_fill_hovered),
            node_stroke_hovered: Color::new(t.node_stroke_hovered),
            node_fill_selected: Color::new(t.node_fill_selected),
            node_stroke_selected: Color::new(t.node_stroke_selected),
            node_fill_selection_exit: Color::new(t.node_fill_selection_exit),
            node_stroke_selection_exit: Color::new(t.node_stroke_selection_exit),
            node_fill_disabled: Color::new(t.node_fill_disabled),
            node_stroke_disabled: Color::new(t.node_stroke_disabled),
            node_stroke_computing: Color::new(t.node_stroke_computing),
            node_stroke_stale: Color::new(t.node_stroke_stale),
            node_stroke_error: Color::new(t.node_stroke_error),
            node_stroke_blocked: Color::new(t.node_stroke_blocked),
            indirect_handle_fill: Color::new(t.indirect_handle_fill),
            indirect_handle_stroke: Color::new(t.indirect_handle_stroke),
            handle_fill: Color::new(t.handle_fill),
            handle_stroke: Color::new(t.handle_stroke),
            handle_fill_hovered: Color::new(t.handle_fill_hovered),
            handle_stroke_hovered: Color::new(t.handle_stroke_hovered),
            handle_fill_selected: Color::new(t.handle_fill_selected),
            handle_stroke_selected: Color::new(t.handle_stroke_selected),
            handle_fill_selection_exit: Color::new(t.handle_fill_selection_exit),
            handle_stroke_selection_exit: Color::new(t.handle_stroke_selection_exit),
            handle_fill_disabled: Color::new(t.handle_fill_disabled),
            handle_stroke_disabled: Color::new(t.handle_stroke_disabled),
            wire_stroke: Color::new(t.wire_stroke),
            wire_stroke_hovered: Color::new(t.wire_stroke_hovered),
            wire_stroke_selected: Color::new(t.wire_stroke_selected),
            wire_stroke_highlighted: Color::new(t.wire_stroke_highlighted),
            wire_stroke_disabled: Color::new(t.wire_stroke_disabled),
            selection_preview_fill: Color::new(t.selection_preview_fill),
            selection_preview_stroke: Color::new(t.selection_preview_stroke),
            label_fill: Color::new(t.label_fill),
            label_fill_hovered: Color::new(t.label_fill_hovered),
            label_halo: Color::new(t.label_halo),
            minimap_widget_panel_fill: Color::new(t.minimap_widget_panel_fill),
            minimap_widget_panel_stroke: Color::new(t.minimap_widget_panel_stroke),
            minimap_widget_viewport_fill: Color::new(t.minimap_widget_viewport_fill),
            minimap_widget_viewport_stroke: Color::new(t.minimap_widget_viewport_stroke),
            minimap_widget_viewport_stroke_hovered: Color::new(t.minimap_widget_viewport_stroke_hovered),
        }
    }

    /// 🎨️ Applies a typed overlay to the centralized default palette.
    pub fn apply_overlay(&mut self, overlay:&BoardPaletteOverlay) {
        let defaults=Self::default();
        let base=BoardPalette {
            raster_clear:{let c=defaults.raster_clear.to_rgba8();[c.r,c.g,c.b,c.a]},
            grid_minor_stroke:{let c=defaults.grid_minor_stroke.to_rgba8();[c.r,c.g,c.b,c.a]},
            edge_stroke:{let c=defaults.edge_stroke.to_rgba8();[c.r,c.g,c.b,c.a]},
            edge_stroke_hovered:{let c=defaults.edge_stroke_hovered.to_rgba8();[c.r,c.g,c.b,c.a]},
            edge_stroke_selected:{let c=defaults.edge_stroke_selected.to_rgba8();[c.r,c.g,c.b,c.a]},
            edge_stroke_selection_exit:{let c=defaults.edge_stroke_selection_exit.to_rgba8();[c.r,c.g,c.b,c.a]},
            edge_stroke_disabled:{let c=defaults.edge_stroke_disabled.to_rgba8();[c.r,c.g,c.b,c.a]},
            node_fill:{let c=defaults.node_fill.to_rgba8();[c.r,c.g,c.b,c.a]},
            node_stroke:{let c=defaults.node_stroke.to_rgba8();[c.r,c.g,c.b,c.a]},
            node_fill_hovered:{let c=defaults.node_fill_hovered.to_rgba8();[c.r,c.g,c.b,c.a]},
            node_stroke_hovered:{let c=defaults.node_stroke_hovered.to_rgba8();[c.r,c.g,c.b,c.a]},
            node_fill_selected:{let c=defaults.node_fill_selected.to_rgba8();[c.r,c.g,c.b,c.a]},
            node_stroke_selected:{let c=defaults.node_stroke_selected.to_rgba8();[c.r,c.g,c.b,c.a]},
            node_fill_selection_exit:{let c=defaults.node_fill_selection_exit.to_rgba8();[c.r,c.g,c.b,c.a]},
            node_stroke_selection_exit:{let c=defaults.node_stroke_selection_exit.to_rgba8();[c.r,c.g,c.b,c.a]},
            node_fill_disabled:{let c=defaults.node_fill_disabled.to_rgba8();[c.r,c.g,c.b,c.a]},
            node_stroke_disabled:{let c=defaults.node_stroke_disabled.to_rgba8();[c.r,c.g,c.b,c.a]},
            node_stroke_computing:{let c=defaults.node_stroke_computing.to_rgba8();[c.r,c.g,c.b,c.a]},
            node_stroke_stale:{let c=defaults.node_stroke_stale.to_rgba8();[c.r,c.g,c.b,c.a]},
            node_stroke_error:{let c=defaults.node_stroke_error.to_rgba8();[c.r,c.g,c.b,c.a]},
            node_stroke_blocked:{let c=defaults.node_stroke_blocked.to_rgba8();[c.r,c.g,c.b,c.a]},
            indirect_handle_fill:{let c=defaults.indirect_handle_fill.to_rgba8();[c.r,c.g,c.b,c.a]},
            indirect_handle_stroke:{let c=defaults.indirect_handle_stroke.to_rgba8();[c.r,c.g,c.b,c.a]},
            handle_fill:{let c=defaults.handle_fill.to_rgba8();[c.r,c.g,c.b,c.a]},
            handle_stroke:{let c=defaults.handle_stroke.to_rgba8();[c.r,c.g,c.b,c.a]},
            handle_fill_hovered:{let c=defaults.handle_fill_hovered.to_rgba8();[c.r,c.g,c.b,c.a]},
            handle_stroke_hovered:{let c=defaults.handle_stroke_hovered.to_rgba8();[c.r,c.g,c.b,c.a]},
            handle_fill_selected:{let c=defaults.handle_fill_selected.to_rgba8();[c.r,c.g,c.b,c.a]},
            handle_stroke_selected:{let c=defaults.handle_stroke_selected.to_rgba8();[c.r,c.g,c.b,c.a]},
            handle_fill_selection_exit:{let c=defaults.handle_fill_selection_exit.to_rgba8();[c.r,c.g,c.b,c.a]},
            handle_stroke_selection_exit:{let c=defaults.handle_stroke_selection_exit.to_rgba8();[c.r,c.g,c.b,c.a]},
            handle_fill_disabled:{let c=defaults.handle_fill_disabled.to_rgba8();[c.r,c.g,c.b,c.a]},
            handle_stroke_disabled:{let c=defaults.handle_stroke_disabled.to_rgba8();[c.r,c.g,c.b,c.a]},
            wire_stroke:{let c=defaults.wire_stroke.to_rgba8();[c.r,c.g,c.b,c.a]},
            wire_stroke_hovered:{let c=defaults.wire_stroke_hovered.to_rgba8();[c.r,c.g,c.b,c.a]},
            wire_stroke_selected:{let c=defaults.wire_stroke_selected.to_rgba8();[c.r,c.g,c.b,c.a]},
            wire_stroke_highlighted:{let c=defaults.wire_stroke_highlighted.to_rgba8();[c.r,c.g,c.b,c.a]},
            wire_stroke_disabled:{let c=defaults.wire_stroke_disabled.to_rgba8();[c.r,c.g,c.b,c.a]},
            selection_preview_fill:{let c=defaults.selection_preview_fill.to_rgba8();[c.r,c.g,c.b,c.a]},
            selection_preview_stroke:{let c=defaults.selection_preview_stroke.to_rgba8();[c.r,c.g,c.b,c.a]},
            label_fill:{let c=defaults.label_fill.to_rgba8();[c.r,c.g,c.b,c.a]},
            label_fill_hovered:{let c=defaults.label_fill_hovered.to_rgba8();[c.r,c.g,c.b,c.a]},
            label_halo:{let c=defaults.label_halo.to_rgba8();[c.r,c.g,c.b,c.a]},
            minimap_widget_panel_fill:{let c=defaults.minimap_widget_panel_fill.to_rgba8();[c.r,c.g,c.b,c.a]},
            minimap_widget_panel_stroke:{let c=defaults.minimap_widget_panel_stroke.to_rgba8();[c.r,c.g,c.b,c.a]},
            minimap_widget_viewport_fill:{let c=defaults.minimap_widget_viewport_fill.to_rgba8();[c.r,c.g,c.b,c.a]},
            minimap_widget_viewport_stroke:{let c=defaults.minimap_widget_viewport_stroke.to_rgba8();[c.r,c.g,c.b,c.a]},
            minimap_widget_viewport_stroke_hovered:{let c=defaults.minimap_widget_viewport_stroke_hovered.to_rgba8();[c.r,c.g,c.b,c.a]},
        };
        let next=apply_board_palette_overlay(&base,overlay);
        *self=Self {
            raster_clear:{let c=next.raster_clear;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            grid_minor_stroke:{let c=next.grid_minor_stroke;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            edge_stroke:{let c=next.edge_stroke;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            edge_stroke_hovered:{let c=next.edge_stroke_hovered;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            edge_stroke_selected:{let c=next.edge_stroke_selected;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            edge_stroke_selection_exit:{let c=next.edge_stroke_selection_exit;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            edge_stroke_disabled:{let c=next.edge_stroke_disabled;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            node_fill:{let c=next.node_fill;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            node_stroke:{let c=next.node_stroke;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            node_fill_hovered:{let c=next.node_fill_hovered;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            node_stroke_hovered:{let c=next.node_stroke_hovered;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            node_fill_selected:{let c=next.node_fill_selected;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            node_stroke_selected:{let c=next.node_stroke_selected;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            node_fill_selection_exit:{let c=next.node_fill_selection_exit;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            node_stroke_selection_exit:{let c=next.node_stroke_selection_exit;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            node_fill_disabled:{let c=next.node_fill_disabled;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            node_stroke_disabled:{let c=next.node_stroke_disabled;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            node_stroke_computing:{let c=next.node_stroke_computing;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            node_stroke_stale:{let c=next.node_stroke_stale;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            node_stroke_error:{let c=next.node_stroke_error;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            node_stroke_blocked:{let c=next.node_stroke_blocked;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            indirect_handle_fill:{let c=next.indirect_handle_fill;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            indirect_handle_stroke:{let c=next.indirect_handle_stroke;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            handle_fill:{let c=next.handle_fill;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            handle_stroke:{let c=next.handle_stroke;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            handle_fill_hovered:{let c=next.handle_fill_hovered;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            handle_stroke_hovered:{let c=next.handle_stroke_hovered;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            handle_fill_selected:{let c=next.handle_fill_selected;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            handle_stroke_selected:{let c=next.handle_stroke_selected;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            handle_fill_selection_exit:{let c=next.handle_fill_selection_exit;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            handle_stroke_selection_exit:{let c=next.handle_stroke_selection_exit;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            handle_fill_disabled:{let c=next.handle_fill_disabled;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            handle_stroke_disabled:{let c=next.handle_stroke_disabled;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            wire_stroke:{let c=next.wire_stroke;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            wire_stroke_hovered:{let c=next.wire_stroke_hovered;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            wire_stroke_selected:{let c=next.wire_stroke_selected;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            wire_stroke_highlighted:{let c=next.wire_stroke_highlighted;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            wire_stroke_disabled:{let c=next.wire_stroke_disabled;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            selection_preview_fill:{let c=next.selection_preview_fill;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            selection_preview_stroke:{let c=next.selection_preview_stroke;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            label_fill:{let c=next.label_fill;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            label_fill_hovered:{let c=next.label_fill_hovered;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            label_halo:{let c=next.label_halo;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            minimap_widget_panel_fill:{let c=next.minimap_widget_panel_fill;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            minimap_widget_panel_stroke:{let c=next.minimap_widget_panel_stroke;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            minimap_widget_viewport_fill:{let c=next.minimap_widget_viewport_fill;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            minimap_widget_viewport_stroke:{let c=next.minimap_widget_viewport_stroke;Color::from_rgba8(c[0],c[1],c[2],c[3])},
            minimap_widget_viewport_stroke_hovered:{let c=next.minimap_widget_viewport_stroke_hovered;Color::from_rgba8(c[0],c[1],c[2],c[3])},
        };
    }
}

pub enum CachedIconBody {
    Vector(Scene),
    Raster(Arc<RasterImage>),
}

struct CachedIconPaint {
    pub bx: f64,
    pub by: f64,
    pub bw: f64,
    pub bh: f64,
    pub body: CachedIconBody,
}

const ICON_PAINT_CACHE_CAPACITY: usize = 256;

#[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
const ICON_PAINT_CACHE_KEY_BYTE_CAPACITY: usize = 256;

#[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
const ICON_PAINT_SOURCE_BYTE_CAPACITY: usize = 16 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
struct IconPaintToken {
    pub slot: u16,
    pub generation: u64,
}

struct IconPaintSlot {
    pub key: Option<String>,
    epoch: u64,
    pub generation: u64,
    pub value: Option<CachedIconPaint>,
}

struct IconPaintRegistry {
    pub slots: Box<[IconPaintSlot; ICON_PAINT_CACHE_CAPACITY]>,
    epoch: u64,
    pub faulted: bool,
}

impl Default for IconPaintRegistry {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| IconPaintSlot { key: None, epoch: 0, generation: 0, value: None }), epoch: 1, faulted: false }
    }
}

impl IconPaintRegistry {
    #[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
    pub fn index(&self, key: &str) -> Option<usize> {
        self.slots.iter().position(|slot| slot.epoch == self.epoch && slot.key.as_deref() == Some(key) && slot.value.is_some())
    }

    #[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
    pub fn get(&self, key: &str) -> Option<&CachedIconPaint> {
        self.slots.get(self.index(key)?)?.value.as_ref()
    }

    #[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
    pub fn reserve(&mut self, key: &str) -> Option<IconPaintToken> {
        if key.len() > ICON_PAINT_CACHE_KEY_BYTE_CAPACITY || self.get(key).is_some() {
            self.faulted = true;
            return None;
        }
        let Some(index) = self.slots.iter().position(|slot| slot.key.is_none() && slot.value.is_none()) else {
            self.faulted = true;
            return None;
        };
        let slot = &mut self.slots[index];
        slot.generation = slot.generation.wrapping_add(1).max(1);
        slot.epoch = self.epoch;
        slot.key = Some(key.to_owned());
        Some(IconPaintToken { slot: index as u16, generation: slot.generation })
    }

    #[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
    fn publish(&mut self, token: IconPaintToken, value: CachedIconPaint) {
        let slot = self.slots.get_mut(usize::from(token.slot)).expect("reserved icon cache slot remains present");
        assert_eq!(slot.generation, token.generation, "reserved icon cache generation remains current");
        assert_eq!(slot.epoch, self.epoch, "reserved icon cache epoch remains current");
        assert!(slot.key.is_some() && slot.value.is_none(), "reserved icon cache slot remains unpublished");
        slot.value = Some(value);
    }

    #[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
    fn abort(&mut self, token: IconPaintToken) {
        let slot = self.slots.get_mut(usize::from(token.slot)).expect("reserved icon cache slot remains present");
        assert_eq!(slot.generation, token.generation, "aborted icon cache generation remains current");
        assert!(slot.value.is_none(), "only an unpublished icon cache reservation can abort");
        slot.key = None;
        slot.epoch = 0;
        slot.generation = slot.generation.wrapping_add(1).max(1);
    }

    fn invalidate(&mut self) {
        self.epoch = self.epoch.wrapping_add(1).max(1);
    }
}

/// 🖼️ Center and available screen extents for fitting one icon without coordinate conversion.
#[derive(Clone, Copy)]
pub struct IconScreenRect {
    pub center: Point,
    pub width: f64,
    pub height: f64,
}

/// 🖼️ Shared SVG/raster icon decode cache for board and DAG hosts.
pub struct IconPaintCache {
    pub cache: RefCell<ManuallyDrop<IconPaintRegistry>>,
    retirement_cursor: Cell<u16>,
    retirement_credited_bytes: Cell<usize>,
    retirement_scene: Cell<Option<semio_framework_canvas::OpaqueSceneRetirementToken>>,
    pub closing: Cell<bool>,
    registry_released:Cell<bool>,
    pub themed_icon_lookup: semio_framework_canvas::icon_codec::ThemedSvgLookup,
}

/// 📸️ One icon-retirement turn with current credit and physical release split.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconPaintRetirementStep {
    Blocked,
    Pending { released_items: usize, credited_bytes: usize, released_bytes: usize },
    Complete,
}

pub struct CachedIconPaintLease<'a> {
    pub cache: std::cell::Ref<'a, ManuallyDrop<IconPaintRegistry>>,
    pub slot: usize,
}

impl CachedIconPaintLease<'_> {
    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        let paint = self.cache.slots[self.slot].value.as_ref().expect("leased icon cache slot remains published");
        (paint.bx, paint.by, paint.bw, paint.bh)
    }

    pub fn body(&self) -> &CachedIconBody {
        &self.cache.slots[self.slot].value.as_ref().expect("leased icon cache slot remains published").body
    }
}

impl Default for IconPaintCache {
    fn default() -> Self {
        Self {
            cache: RefCell::new(ManuallyDrop::new(IconPaintRegistry::default())),
            retirement_cursor: Cell::new(0),
            retirement_credited_bytes: Cell::new(0),
            retirement_scene: Cell::new(None),
            closing: Cell::new(false),
            registry_released:Cell::new(false),
            themed_icon_lookup: |_| None,
        }
    }
}

impl Clone for IconPaintCache {
    fn clone(&self) -> Self {
        Self {
            cache: RefCell::new(ManuallyDrop::new(IconPaintRegistry::default())),
            retirement_cursor: Cell::new(0),
            retirement_credited_bytes: Cell::new(0),
            retirement_scene: Cell::new(None),
            closing: Cell::new(false),
            registry_released:Cell::new(false),
            themed_icon_lookup: self.themed_icon_lookup,
        }
    }
}

impl Drop for IconPaintCache {
    fn drop(&mut self) {
        if self.registry_released.get(){return}
        let terminal = self.terminal_is_empty();
        let never_admitted = self.retirement_scene.get().is_none() && self.cache.get_mut().slots.iter().all(|slot| slot.key.is_none() && slot.value.is_none());
        debug_assert!(terminal || never_admitted || std::thread::panicking(), "IconPaintCache with admitted resources must reach terminal-empty through close_step before release");
        if terminal || never_admitted {
            unsafe { ManuallyDrop::drop(self.cache.get_mut()) };
        }
    }
}

struct IconOwnedRetirement {
    pub source:ManuallyDrop<Option<IconPaintCache>>,
    pub key:Option<semio_framework_value::retirement::controlled::ControlledRetirement<String>>,
    image:Option<semio_framework_value::retirement::shared::SharedControlledRetirement<RasterImage>>,
    metadata:Option<semio_framework_value::retirement::controlled::ControlledRetirement<(f64,f64,f64,f64)>>,
    pub cursor:usize,
}

impl IconOwnedRetirement {
    fn terminal(&self)->bool{self.source.is_none()&&self.key.is_none()&&self.image.is_none()&&self.metadata.is_none()}
    pub fn demand(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
        let deeper=|depth:usize|depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original icon owner depth overflow"));
        if let Some(owner)=self.key.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:deeper(owner.next_depth_demand()?)?})}
        if let Some(owner)=self.metadata.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:deeper(owner.next_depth_demand()?)?})}
        if let Some(owner)=self.image.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:deeper(owner.next_depth_demand()?)?})}
        let Some(source)=self.source.as_ref() else{return Ok(RetirementDemand::default())};
        if source.retirement_scene.get().is_some()||source.retirement_credited_bytes.get()!=0{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"existing icon scene authority requires its original full receipt"))}
        if self.cursor==ICON_PAINT_CACHE_CAPACITY{return Ok(RetirementDemand{release_bytes:size_of::<[IconPaintSlot;ICON_PAINT_CACHE_CAPACITY]>(),depth:1,..Default::default()})}
        if matches!(source.cache.borrow().slots[self.cursor].value.as_ref().map(|value|&value.body),Some(CachedIconBody::Vector(_))){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original vector icon producer has not declared full scene retirement authority"))}
        Ok(RetirementDemand{depth:1,..Default::default()})
    }
    pub fn step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>{
        use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::{controlled::ControlledRetirement,shared::SharedControlledRetirement}};
        let empty=RetainedCloneProgress::default();if self.terminal(){return Ok(RetainedCloneStep::Complete(empty))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}
        let demand=self.demand(grant.maximum_copy_bytes)?;if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original icon retirement exceeds admitted depth"))}if grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(empty))}
        macro_rules! child{($field:ident,$method:ident)=>{if let Some(owner)=self.$field.as_mut(){if owner.terminal_is_empty(){self.$field=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=owner.$method(child)?;return Ok(RetainedCloneStep::Progress(semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original icon child")?.progress()))}}}
        child!(key,step);child!(metadata,step);child!(image,step);
        let source=self.source.as_mut().unwrap();
        if self.cursor==ICON_PAINT_CACHE_CAPACITY{unsafe{ManuallyDrop::drop(source.cache.get_mut())}source.registry_released.set(true);drop(self.source.take());return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..empty}))}
        let slot=&mut source.cache.get_mut().slots[self.cursor];
        if let Some(key)=slot.key.take(){self.key=Some(ControlledRetirement::new(key).unwrap_or_else(|_|unreachable!("original icon key is supported")));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
        if let Some(value)=slot.value.take(){let CachedIconPaint{bx,by,bw,bh,body}=value;let CachedIconBody::Raster(image)=body else{unreachable!("original vector scene authority is checked before source transfer")};self.metadata=Some(ControlledRetirement::new((bx,by,bw,bh)).unwrap_or_else(|_|unreachable!("icon numeric metadata is supported")));self.image=Some(SharedControlledRetirement::lease(image));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
        self.cursor+=1;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))
    }
}

impl semio_framework_value::retirement::RetirementCursor for IconOwnedRetirement {
    fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep{use semio_framework_value::{RetirementStep,retained_clone::{RetainedCloneProgress,RetainedCloneStep}};match self.step(grant){Err(error)=>RetirementStep::Failure(error),Ok(RetainedCloneStep::Complete(progress))if progress==RetainedCloneProgress::default()=>RetirementStep::Complete,Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress)}}
    fn terminal_is_empty(&self)->bool{self.terminal()}
    fn next_work_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.demand(0)?.copy_bytes)}
    fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.demand(copy).ok().map(|demand|demand.capacity_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.demand(0).ok().map(|demand|demand.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.demand(0)?.depth)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal().then_some(size_of::<Self>())}
}

impl Drop for IconOwnedRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal(),"original icon cache abandoned before physical retirement");if self.terminal(){unsafe{ManuallyDrop::drop(&mut self.source)}}}}

impl semio_framework_value::retirement::RetireOwned for IconPaintCache {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(IconOwnedRetirement{source:ManuallyDrop::new(Some(self)),key:None,image:None,metadata:None,cursor:0})}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<IconOwnedRetirement>())}
    fn controlled_retirement_supported()->bool{true}
}

impl IconPaintCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&self) {
        assert!(self.retirement_scene.get().is_none(), "icon cache invalidation cannot detach an admitted scene retirement");
        assert_eq!(self.retirement_credited_bytes.get(), 0, "icon cache invalidation cannot detach admitted byte credit");
        self.cache.borrow_mut().invalidate();
        self.retirement_cursor.set(0);
    }

    pub fn close_step(&self) -> bool {
        matches!(self.close_page(1, usize::MAX), IconPaintRetirementStep::Complete)
    }

    pub fn close_page(&self, maximum_items: usize, maximum_bytes: usize) -> IconPaintRetirementStep {
        self.closing.set(true);
        if maximum_items == 0 || maximum_bytes == 0 {
            return IconPaintRetirementStep::Blocked;
        }
        if let Some(token) = self.retirement_scene.get() {
            return match semio_framework_canvas::advance_opaque_scene_retirement(token, maximum_items, maximum_bytes) {
                semio_framework_canvas::OpaqueSceneRetirementStep::Blocked => IconPaintRetirementStep::Blocked,
                semio_framework_canvas::OpaqueSceneRetirementStep::Pending { released_items, credited_bytes, released_bytes } => IconPaintRetirementStep::Pending { released_items, credited_bytes, released_bytes },
                semio_framework_canvas::OpaqueSceneRetirementStep::Complete { released_items, credited_bytes, released_bytes } => {
                    self.retirement_scene.set(None);
                    IconPaintRetirementStep::Pending { released_items, credited_bytes, released_bytes }
                }
                semio_framework_canvas::OpaqueSceneRetirementStep::Fault => {
                    self.cache.borrow_mut().faulted = true;
                    IconPaintRetirementStep::Blocked
                }
            };
        }
        let index = usize::from(self.retirement_cursor.get());
        if index == ICON_PAINT_CACHE_CAPACITY {
            return IconPaintRetirementStep::Complete;
        }
        let mut cache = self.cache.borrow_mut();
        let slot = &mut cache.slots[index];
        let released_bytes = slot.key.as_ref().map_or(0, String::capacity).saturating_add(match slot.value.as_ref().map(|value| &value.body) {
            Some(CachedIconBody::Raster(image)) if Arc::strong_count(image) == 1 => image.retirement_exclusive_backing_bytes(),
            Some(CachedIconBody::Raster(_)) | Some(CachedIconBody::Vector(_)) | None => 0,
        });
        let remaining_bytes = released_bytes.saturating_sub(self.retirement_credited_bytes.get());
        let credited_bytes = maximum_bytes.min(remaining_bytes);
        self.retirement_credited_bytes.set(self.retirement_credited_bytes.get().saturating_add(credited_bytes));
        if self.retirement_credited_bytes.get() != released_bytes {
            return IconPaintRetirementStep::Pending { released_items: 0, credited_bytes, released_bytes: 0 };
        }
        if let Some(CachedIconPaint { body: CachedIconBody::Vector(_), .. }) = slot.value.as_ref() {
            let Some(token) = semio_framework_canvas::reserve_opaque_scene_retirement() else {
                cache.faulted = true;
                return IconPaintRetirementStep::Blocked;
            };
            let paint = slot.value.take().expect("vector icon retirement slot remains occupied");
            let CachedIconBody::Vector(scene) = paint.body else {
                unreachable!("vector icon retirement was witnessed before ownership transfer");
            };
            semio_framework_canvas::publish_opaque_scene_retirement(token, scene);
            self.retirement_scene.set(Some(token));
        } else {
            slot.value = None;
        }
        slot.key = None;
        slot.epoch = 0;
        slot.generation = slot.generation.wrapping_add(1).max(1);
        self.retirement_cursor.set((index + 1) as u16);
        self.retirement_credited_bytes.set(0);
        IconPaintRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.registry_released.get() || self.closing.get()
            && usize::from(self.retirement_cursor.get()) == ICON_PAINT_CACHE_CAPACITY
            && self.retirement_credited_bytes.get() == 0
            && self.retirement_scene.get().is_none()
            && self.cache.borrow().slots.iter().all(|slot| slot.key.is_none() && slot.value.is_none())
    }

    pub fn faulted(&self) -> bool {
        self.cache.borrow().faulted
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn has_retiring_scene(&self) -> bool {
        self.retirement_scene.get().is_some()
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn occupied_slots(&self) -> usize {
        self.cache.borrow().slots.iter().filter(|slot| slot.key.is_some()).count()
    }

    #[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
    fn icon_vector_cache_key(tag: &str, svg: &str, fg: Color, bg: Color) -> String {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        svg.hash(&mut hasher);
        let hx = hasher.finish();
        let f = fg.to_rgba8();
        let b = bg.to_rgba8();
        format!("v8|{tag}|{hx:x}|{}|{:02x}{:02x}{:02x}{:02x}|{:02x}{:02x}{:02x}{:02x}", svg.len(), f.r, f.g, f.b, f.a, b.r, b.g, b.b, b.a)
    }

    #[cfg(any(any(test, feature = "testkit"), not(all(target_arch = "wasm32", target_env = "p2"))))]
    fn icon_raster_cache_key(rgba: &Arc<[u8]>, w: u32, h: u32) -> String {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        rgba.as_ref().hash(&mut hasher);
        let hx = hasher.finish();
        format!("v8|r|{w}x{h}|{hx:x}|{}", rgba.len())
    }

    /// 🖌️ Builds (or reuses a cached) icon paint — rasterizes SVG via `usvg`/`vello_svg`
    /// or decodes raster bytes via `image`, producing real pixels/vector paint for `Scene`.
    /// Host/browser only: a `wasm32-wasip2` guest has no display to paint onto, so this
    /// target has its own arm below that returns `None` unconditionally — the same value
    /// every caller here already treats as "nothing to paint", so no caller
    /// (`append_icon_at_screen_rect`, `paint_scene`, `build_vector_scene`, and trinity's own
    /// `TrinityBridge::paint_scene`, which is unreachable repo-wide as of this ticket, see
    /// `🔍️research/📓️intrinsic-size-wiring.md`) needs to change. Ticket
    /// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn get_or_build(&self, encoded: &str, fg: Color, bg: Color, preserve_original_style: bool) -> Option<CachedIconPaintLease<'_>> {
        if self.closing.get() {
            return None;
        }
        if encoded.len() > ICON_PAINT_SOURCE_BYTE_CAPACITY {
            self.cache.borrow_mut().faulted = true;
            return None;
        }
        let resolved = semio_framework_canvas::icon_codec::resolve_icon_kind(encoded, self.themed_icon_lookup);
        let key = match &resolved {
            semio_framework_canvas::icon_codec::ResolvedIcon::None => return None,
            semio_framework_canvas::icon_codec::ResolvedIcon::SvgThemed(s) | semio_framework_canvas::icon_codec::ResolvedIcon::SvgPlain(s) => Self::icon_vector_cache_key(if preserve_original_style { "p" } else { "t" }, s.as_str(), fg, bg),
            semio_framework_canvas::icon_codec::ResolvedIcon::RasterRgba8 { rgba, w, h } => Self::icon_raster_cache_key(rgba, *w, *h),
        };
        {
            let g = self.cache.borrow();
            if let Some(slot) = g.index(&key) {
                return Some(CachedIconPaintLease { cache: g, slot });
            }
        }
        let token = self.cache.borrow_mut().reserve(&key)?;
        let (bx, by, bw, bh, body) = match resolved {
            semio_framework_canvas::icon_codec::ResolvedIcon::None => {
                self.cache.borrow_mut().abort(token);
                return None;
            }
            semio_framework_canvas::icon_codec::ResolvedIcon::SvgThemed(s) => {
                let Some(doc) = SvgDocument::parse_icons(s.trim()).ok() else {
                    self.cache.borrow_mut().abort(token);
                    return None;
                };
                let (bx, by, bw, bh) = doc.content_bounds();
                if !(bw > 0.0 && bh > 0.0 && bw.is_finite() && bh.is_finite()) {
                    self.cache.borrow_mut().abort(token);
                    return None;
                }
                let mut s = Scene::new();
                if preserve_original_style {
                    append_svg_document(&mut s, &doc);
                } else {
                    doc.render_themed(&mut s, fg, bg);
                }
                (bx, by, bw, bh, CachedIconBody::Vector(s))
            }
            semio_framework_canvas::icon_codec::ResolvedIcon::SvgPlain(s) => {
                let Some(doc) = SvgDocument::parse_icons(s.trim()).ok() else {
                    self.cache.borrow_mut().abort(token);
                    return None;
                };
                let (bx, by, bw, bh) = doc.content_bounds();
                if !(bw > 0.0 && bh > 0.0 && bw.is_finite() && bh.is_finite()) {
                    self.cache.borrow_mut().abort(token);
                    return None;
                }
                let mut s = Scene::new();
                if preserve_original_style {
                    append_svg_document(&mut s, &doc);
                } else {
                    doc.render_themed(&mut s, fg, bg);
                }
                (bx, by, bw, bh, CachedIconBody::Vector(s))
            }
            semio_framework_canvas::icon_codec::ResolvedIcon::RasterRgba8 { rgba, w, h } => {
                let bx = 0.0_f64;
                let by = 0.0_f64;
                let bw = f64::from(w);
                let bh = f64::from(h);
                let img = RasterImage::rgba8(w, h, Arc::new(rgba.as_ref().to_vec()));
                (bx, by, bw, bh, CachedIconBody::Raster(Arc::new(img)))
            }
        };
        self.cache.borrow_mut().publish(token, CachedIconPaint { bx, by, bw, bh, body });
        let cache = self.cache.borrow();
        let slot = cache.index(&key).expect("published icon cache key remains indexed");
        Some(CachedIconPaintLease { cache, slot })
    }

    /// 🚫️ `wasm32-wasip2` arm: icon *painting* (rasterizing SVG/raster icon sources to
    /// `Scene` pixels/paths) is host-only by nature — a WASI guest component has no display
    /// to paint onto, and every real caller of icon painting on this target is either a
    /// browser bridge already excluded from `wasm32-wasip2` (`target_arch = "wasm32"` is TRUE
    /// for `wasm32-wasip2`, so those bridges use the narrower `not(target_env = "p2")` gate)
    /// or, for `semio-s-plugin-trinity`'s `TrinityBridge::paint_scene`, unreachable from any
    /// caller repo-wide. `None` here is not a stub for exercised behavior — it is the value
    /// every caller already treats as "nothing to paint", so nothing regresses if it is
    /// literally the only value this target ever produces. Ticket
    /// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`,
    /// `🔍️research/📓️intrinsic-size-wiring.md`.
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    pub fn get_or_build(&self, _encoded: &str, _fg: Color, _bg: Color, _preserve_original_style: bool) -> Option<CachedIconPaintLease<'_>> {
        None
    }

    /// 🖼️ Paints an icon centered in a screen-space rectangle.
    pub fn append_icon_at_screen_rect(&self, scene: &mut Scene, icon_kind: &str, rect: IconScreenRect, fg: Color, bg: Color, preserve_original_style: bool) {
        let IconScreenRect { center, width: avail_w, height: avail_h } = rect;
        let Some(paint) = self.get_or_build(icon_kind, fg, bg, preserve_original_style) else {
            return;
        };
        let (bx, by, bw, bh) = paint.bounds();
        if !(avail_w > 0.0 && avail_h > 0.0) {
            return;
        }
        let fit_inset = ui_styling::metrics::icon::FIT_INSET;
        let sx_half = avail_w * fit_inset * 0.5;
        let sy_half = avail_h * fit_inset * 0.5;
        let cx = bx + bw * 0.5;
        let cy = by + bh * 0.5;
        let scale = (2.0 * sx_half / bw).min(2.0 * sy_half / bh);
        let aff = Affine::IDENTITY.translate((center.x - scale * cx, center.y - scale * cy)) * Affine::IDENTITY.scale(scale);
        let clip_inset = ui_styling::metrics::icon::CLIP_INSET;
        let hw = avail_w * clip_inset * 0.5;
        let hh = avail_h * clip_inset * 0.5;
        let clip_r = Rect::from_points(Point::new(center.x - hw, center.y - hh), Point::new(center.x + hw, center.y + hh));
        scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &clip_r);
        match paint.body() {
            CachedIconBody::Vector(icon_scene) => {
                scene.append(icon_scene, Some(aff));
            }
            CachedIconBody::Raster(img) => {
                scene.draw_image(img, aff);
            }
        }
        scene.pop_layer();
    }

    /// 🎨️ Themed SVG icon fg/bg from centralized canvas tokens (not node chrome stroke/fill).
    pub fn board_icon_paint_colors(canvas_theme: &CanvasPalette) -> (Color, Color) {
        let rgba = canvas_theme.raster_clear.to_rgba8();
        let lum = f64::from(rgba.r) * 0.299 + f64::from(rgba.g) * 0.587 + f64::from(rgba.b) * 0.114;
        let canvas = if lum < 128.0 { &ui_styling::CANVAS_DARK } else { &ui_styling::CANVAS_LIGHT };
        (Color::new(canvas.icon_fg), Color::new(canvas.icon_bg))
    }
}
// #endregion 🔖️Icons

impl Default for CanvasPalette {
    fn default() -> Self {
        Self::from_board_palette(&ui_styling::BOARD_LIGHT)
    }
}
// #endregion types

/// ➡️ Port graph engine with directed handle endpoints.
pub type DirectedPortGraphEngine = GraphEngine<Ported, Directed>;

/// ⚙️ Directed port board engine alias.
pub type BoardEngine = DirectedPortGraphEngine;

/// 🪢️ Cubic edge connecting two handles (legacy field names).
#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    pub id: EdgeId,
    pub source_handle: HandleId,
    pub target_handle: HandleId,
}

// #region 🔖️EdgeEndpointResolution
/// 🔗️ Resolves a ported edge endpoint to a node id (handle lookup, then node id).
fn resolve_endpoint_node_id(endpoint_id: &str, handle_to_node: &std::collections::HashMap<String, String>) -> String {
    handle_to_node.get(endpoint_id).cloned().unwrap_or_else(|| endpoint_id.to_string())
}
// #endregion 🔖️EdgeEndpointResolution

/// 🌱️ Owns semantic descriptor values independently of serialized syntax.
#[derive(Clone, Debug, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct EdgeDescriptor {
    pub id: String,
    pub source: String,
    pub target: String,
    /// 🧩️ Semantic edge-kind id for compatibility at `edge` specificity.
    #[serde(default)]
    #[value(default)]
    pub edge_kind: Option<String>,
    /// 🔺️ Per-instance source tip id from the edge tip registry (`none` disables).
    #[serde(default)]
    #[value(default)]
    pub source_tip: Option<String>,
    /// 🔺️ Per-instance target tip id from the edge tip registry (`none` disables).
    #[serde(default)]
    #[value(default)]
    pub target_tip: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub style: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub user_data: Option<semio_framework_value::DslValue>,
    #[serde(default)]
    #[value(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub locked: Option<bool>,
}

/// 🧵️ Transient cubic link from a handle to another handle or a free world point (descriptor + link gesture).
///
/// 🌱️ Owns semantic descriptor values independently of serialized syntax.
#[derive(Clone, Debug, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WireDescriptor {
    pub id: String,
    pub source: String,
    /// 🧩️ Semantic wire-kind id (defaults from catalog when omitted in fixtures).
    #[serde(default)]
    #[value(default)]
    pub wire_kind: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub target: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub end_x: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub end_y: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub style: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub user_data: Option<semio_framework_value::DslValue>,
    #[serde(default)]
    #[value(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub locked: Option<bool>,
}

/// 🎯️ One `targetRegions` row of the owning document, as the descriptor spells it. Derived, not
/// hand-written: unlike its node/edge siblings a region carries no free-form `userData`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct RegionDescriptor {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    #[serde(default)]
    #[value(default)]
    pub label: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub hidden: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub locked: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub selected: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct SceneDescriptor {
    pub nodes: Vec<NodeDescriptor>,
    pub handles: Vec<HandleDescriptor>,
    pub edges: Vec<EdgeDescriptor>,
    #[serde(default)]
    #[value(default)]
    pub wires: Vec<WireDescriptor>,
    /// 🎯️ Fill-constraining rectangles painted beneath every entity and hit-tested after
    /// all of them — absent on a board that declares none.
    #[serde(default)]
    #[value(default)]
    pub regions: Vec<RegionDescriptor>,
    /// 💠️ JS‑authored ids to paint with secondary “left selection” chrome (not in current `selected` flags).
    #[serde(default)]
    #[value(default)]
    pub selection_exit_highlight_ids: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct HandleKindDef {
    pub name: String,
    pub color: Color,
    pub default_wire_kind: Option<String>,
    pub scale: f64,
}

#[derive(Clone, Debug)]
pub struct NodeKindHandleTemplate {
    pub handle_kind: String,
    pub angle: f64,
    pub radius: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct HandleData {
    pub id: String,
    pub node_id: String,
    pub angle: f64,
    pub radius: f64,
    pub scale: f64,
    pub selected: bool,
    pub visible: bool,
    pub locked: bool,
    pub style: Option<String>,
    pub handle_kind: String,
    /// Parsed from descriptor `color` when set (overrides catalog fill).
    pub color_fill: Option<Color>,
    /// 🏷️ Runtime host encoding: `typst:`, `emoji:`, `image:data:…`, catalog id, or inline SVG for detail LOD.
    pub icon_kind: Option<String>,
    pub properties: PropertyBag,
}
// #endregion 🔖️HandleKinds

// #region 🔖️HandleDescriptor
/// 🌱️ Owns semantic descriptor values independently of serialized syntax.
#[derive(Clone, Debug, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HandleDescriptor {
    pub id: String,
    pub node_id: String,
    pub angle: f64,
    #[serde(default)]
    #[value(default)]
    pub radius: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub style: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub handle_kind: Option<String>,
    /// CSS `#rgb` / `#rrggbb` / `#rrggbbaa` overriding catalog color for this handle.
    #[serde(default)]
    #[value(default)]
    pub color: Option<String>,
    /// 🏷️ Runtime host encoding: `typst:`, `emoji:`, `image:data:…`, catalog id, or inline SVG for detail LOD.
    #[serde(default)]
    #[value(default)]
    pub icon_kind: Option<String>,
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
    pub scale: Option<f64>,
}

pub fn decode_board_scene_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<SceneDescriptor,ValueError>{
    let raw=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();
    let mut scene=SceneDescriptor::from_value_controlled(raw.get(),control)?.guard_decoded();
    macro_rules! normalize {($field:ident)=>{if let Some(rows)=raw.get().get(stringify!($field)).and_then(|value|value.as_array()){for (entry,row)in scene.get_mut().$field.iter_mut().zip(rows){control.step()?;entry.visible=board_visible_option(&BoardVisibility{hidden:row.get("hidden").and_then(|value|value.as_bool()),visible:row.get("visible").and_then(|value|value.as_bool()),locked:None});}}};}
    normalize!(nodes);normalize!(handles);normalize!(edges);normalize!(wires);
    Ok(scene.take())
}

/// 📸️ Both physical inputs are admitted before the pure redraw pipeline executes.
pub fn redraw_snapshot_json(source:&str,options:&str,decode:&mut NativeDecodeControl<'_>,work:&mut LayoutControl<'_>,encode:&mut NativeEncodeControl<'_>)->Result<String,LayoutIoError>{
 let mut snapshot=decode_board_snapshot_json(source,decode)?;
 let opts=decode_redraw_options_json(options,decode)?;
 redraw_layout(&mut snapshot,&opts,work)?;Ok(semio_framework_pack_json::to_json_string_controlled(&snapshot,encode)?)
}

/// ⚛️ Explicit physical force-only entry, with optional empty option text admitted as declared defaults.
pub fn force_snapshot_json(source:&str,options:&str,decode:&mut NativeDecodeControl<'_>,work:&mut LayoutControl<'_>,encode:&mut NativeEncodeControl<'_>)->Result<String,LayoutIoError>{
 let mut snapshot=decode_board_snapshot_json(source,decode)?;
 let opts:ForceGraphLayoutOptions=if options.trim().is_empty(){ForceGraphLayoutOptions::default()}else{{let admission:ForceAdmission=semio_framework_pack_json::from_json_str_controlled(options,semio_framework_pack_json::JsonMemberPolicy::Reject,decode)?;admission.admit(decode)?}};
 if snapshot.schema.as_str()=="board.ports.directed.v1"{force::apply_ported_force_layout(&mut snapshot,&opts,work)?;}else{force::apply_force_layout(&mut snapshot,&opts,work)?;}
 Ok(semio_framework_pack_json::to_json_string_controlled(&snapshot,encode)?)
}

/// 🔗️ Physical snap entry shares the exclusive snapshot admission contract.
pub fn snap_snapshot_json(source:&str,decode:&mut NativeDecodeControl<'_>,work:&mut LayoutControl<'_>,encode:&mut NativeEncodeControl<'_>)->Result<String,LayoutIoError>{let mut snapshot=decode_board_snapshot_json(source,decode)?;snap_edge_handles(&mut snapshot,work)?;Ok(semio_framework_pack_json::to_json_string_controlled(&snapshot,encode)?)}

pub fn decode_board_snapshot_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<BoardSnapshot,ValueError>{semio_framework_pack_json::from_json_str_controlled(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)}

// #region 🔖️GraphExtension
/// 🧩️ Extension hook for domain-specific graph behavior.
pub trait GraphExtension: canvas::CanvasExtension {}
// #endregion 🔖️GraphExtension

// #region 🔖️RenderScene
/// 🎨️ Encodes a batch of edge curves into a stroked vello scene.
pub fn encode_board_stroke_scene(curves: &[CubicBez], stroke_width: f64) -> canvas::Scene {
    let mut scene = canvas::Scene::new();
    let stroke = canvas::Stroke::new(stroke_width);
    for curve in curves {
        scene.stroke(&stroke, canvas::Affine::IDENTITY, canvas::Color::new(ui_styling::CANVAS_LIGHT.icon_bg), None, curve);
    }
    scene
}
// #endregion 🔖️RenderScene

/// 🧭️ Camera state in world units with a zoom scalar suitable for a WASM host bridge.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 🎯️ Semantic board event emitted after interaction or selection changes.
#[derive(Clone, Debug, PartialEq)]
pub enum BoardEvent {
    HoverChanged { id: Option<u64> },
    NodeMoved { id: NodeId, x: f64, y: f64 },
    EdgeConnected { id: EdgeId, source: HandleId, target: HandleId },
    EdgeRemoved { id: EdgeId },
    SelectionChanged { edge_ids: Vec<EdgeId>, handle_ids: Vec<HandleId>, node_ids: Vec<NodeId> },
    PreselectChanged { edge_ids: Vec<EdgeId>, handle_ids: Vec<HandleId>, node_ids: Vec<NodeId>, removed_edge_ids: Vec<EdgeId>, removed_handle_ids: Vec<HandleId>, removed_node_ids: Vec<NodeId> },
}

/// 🖼️ Minimal render snapshot suitable for a host-side drawing layer or tests.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderSnapshot {
    pub edges: Vec<CubicBez>,
    pub handles: Vec<(HandleId, Point, f64)>,
    pub nodes: Vec<(NodeId, Point, f64)>,
    pub pending_edge: Option<CubicBez>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum HitObject<E> {
    Edge(EdgeId),
    Endpoint(E),
    Node(NodeId),
}

/// 🎯️ One graph pick target with generality rank (lower = more general).
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct GraphPickTarget {
    pub domain: String,
    pub id: u64,
    pub generality: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum InteractionMode {
    DragNode {
        node_id: NodeId,
        offset: Vec2,
    },
    DragNodes {
        primary_id: NodeId,
        offset: Vec2,
    },
    DrawEdge {
        anchor_handle: HandleId,
        anchor_is_source: bool,
        fixed_target: Option<HandleId>,
        cursor: Point,
        reconnecting: Option<EdgeId>,
        snap_target: Option<HandleId>,
    },
    SelectionPending {
        start: Point,
        start_screen: Point,
    },
    AreaSelect {
        start: Point,
        start_screen: Point,
    },
    Pan {
        start_screen: Point,
        cam_x: f64,
        cam_y: f64,
        zoom: f64,
    },
    #[default]
    Idle,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ProximityConnection {
    pub source: HandleId,
    pub target: HandleId,
}

/// 🔗️ Resolved wire endpoints for an edge: source/target world position, their owning nodes (if any), and cap radii.
type WireEndpoints<'a> = (Point, Point, Option<&'a Node>, Option<&'a Node>, f64, f64);

pub fn handle_position(node: &Node, handle: &Handle) -> Point {
    match node.shape {
        NodeShape::Circle => handle_position_on_circle(node.center, node.radius, handle.angle),
        NodeShape::Rectangle => handle_position_on_rectangle(node.center, node.width, node.height, handle.angle),
    }
}

pub fn distance(left: Point, right: Point) -> f64 {
    distance_between(left, right)
}

pub const DEFAULT_PROXIMITY_DISTANCE_WORLD: f64 = ui_styling::metrics::board::PROXIMITY_DISTANCE_WORLD;

fn node_contains_point(node: &Node, point: Point) -> bool {
    match node.shape {
        NodeShape::Circle => distance(point, node.center) <= node.radius,
        NodeShape::Rectangle => {
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            point.x >= node.center.x - hw && point.x <= node.center.x + hw && point.y >= node.center.y - hh && point.y <= node.center.y + hh
        }
    }
}

// #region 🔖️GraphPortModel
/// 🔌️ Port model with graph selection semantics.
pub trait GraphPortModel: PortModel {
    fn select_endpoint(selection: &mut Selection, endpoint: Self::Endpoint);
}

impl GraphPortModel for Normal {
    fn select_endpoint(selection: &mut Selection, endpoint: Self::Endpoint) {
        selection.node_ids.insert(endpoint);
    }
}

impl GraphPortModel for Ported {
    fn select_endpoint(selection: &mut Selection, endpoint: Self::Endpoint) {
        selection.handle_ids.insert(endpoint);
    }
}
// #endregion 🔖️GraphPortModel
// #endregion 🔖️Kinds

pub const SELECTION_CLICK_MAX_DISTANCE_PX: f64 = ui_styling::metrics::board::SELECTION_CLICK_MAX_DISTANCE_PX;

pub const SELECTION_LASSO_MIN_POINT_DISTANCE_PX: f64 = ui_styling::metrics::board::SELECTION_LASSO_MIN_POINT_DISTANCE_PX;

pub const GRID_WORLD_LARGE: f64 = ui_styling::metrics::board::GRID_WORLD_LARGE;

pub const GRID_WORLD_MEDIUM: f64 = ui_styling::metrics::board::GRID_WORLD_MEDIUM;

pub const GRID_WORLD_SMALL: f64 = ui_styling::metrics::board::GRID_WORLD_SMALL;

pub const GRID_WORLD_MICRO: f64 = ui_styling::metrics::board::GRID_WORLD_MICRO;

pub const GRID_FACTOR_DEFAULT: f64 = ui_styling::metrics::board::GRID_FACTOR_DEFAULT;

pub const SELECTION_MARQUEE_DRAG_THRESHOLD_PX: f64 = ui_styling::metrics::board::SELECTION_MARQUEE_MAX_DISTANCE_PX;

/// 🚫️ How near a refused port a wire drag must come before the surface names the refusal. A snap
/// tolerance of zero (proximity connection off) must still explain the port the pointer is on, so
/// this floor is independent of `proximity_distance_world`.
pub const DRAW_EDGE_TYPE_REFUSAL_WORLD_TOLERANCE: f64 = 12.0;

/// 🎯️ Normalizes `default` to `replace` for merge-mode strings.
pub fn normalize_selection_mode(mode: &str) -> String {
    if mode == "default" {
        "replace".into()
    } else {
        mode.to_string()
    }
}

/// 🎯️ Maps shift/ctrl modifiers to marquee selection mode (ctrl+shift → invertive).
pub fn pick_merge_mode_for_modifiers(ctrl_or_meta: bool, shift: bool, option_mode: &str) -> String {
    if ctrl_or_meta && shift {
        return "invertive".into();
    }
    if ctrl_or_meta {
        return "subtractive".into();
    }
    if shift {
        return "additive".into();
    }
    normalize_selection_mode(option_mode)
}

/// 🎯️ Applies pick merge mode for a single id.
pub fn merge_pick_into_selection(initial: &BTreeSet<String>, hit_id: &str, mode: &str) -> BTreeSet<String> {
    let mut next = initial.clone();
    match mode {
        "additive" => {
            next.insert(hit_id.to_string());
        }
        "subtractive" => {
            next.remove(hit_id);
        }
        "replace" => {
            next.clear();
            next.insert(hit_id.to_string());
        }
        _ => {
            if next.contains(hit_id) {
                next.remove(hit_id);
            } else {
                next.insert(hit_id.to_string());
            }
        }
    }
    next
}

/// 🎯️ Applies pick merge mode for a marquee hit set.
pub fn merge_ids_into_selection(initial: &BTreeSet<String>, hits: &BTreeSet<String>, mode: &str) -> BTreeSet<String> {
    if mode == "replace" {
        return hits.clone();
    }
    let mut next = initial.clone();
    for id in hits {
        match mode {
            "additive" => {
                next.insert(id.clone());
            }
            "subtractive" => {
                next.remove(id);
            }
            _ => {
                if next.contains(id) {
                    next.remove(id);
                } else {
                    next.insert(id.clone());
                }
            }
        }
    }
    next
}

pub const SELECTION_DRAG_DIRECTION_THRESHOLD_PX: f64 = ui_styling::metrics::board::SELECTION_DRAG_DIRECTION_PX;

/// 🎯️ Drag left→right = enclosing/full; right→left = crossing/partial (rectangle endpoints).
pub fn selection_drag_enclosing_rectangle(start: Point, end: Point) -> bool {
    end.x >= start.x
}

/// 🎯️ Lasso uses the first horizontal step; rectangle compares start vs end.
pub fn selection_drag_enclosing(method: &str, start: Point, points: &[Point]) -> bool {
    if method == "lasso" {
        for point in points.iter().skip(1) {
            let dx = point.x - start.x;
            if dx.abs() < SELECTION_DRAG_DIRECTION_THRESHOLD_PX {
                continue;
            }
            return dx > 0.0;
        }
    }
    let end = points.last().copied().unwrap_or(start);
    selection_drag_enclosing_rectangle(start, end)
}

/// 🧿️ Builds the world-space marquee shape for rectangle or lasso drags.
pub fn selection_drag_shape(method: &str, start: Point, points: &[Point]) -> Option<(WorldBox, bool, Vec<Point>)> {
    let last = points.last().copied().unwrap_or(start);
    let enclosing = selection_drag_enclosing(method, start, points);
    if method == "lasso" && points.len() >= 3 {
        let poly = points.to_vec();
        let b = world_box_from_points(&poly)?;
        return Some((b, enclosing, poly));
    }
    let b = world_box_from_points(&[start, last])?;
    let poly = vec![Point::new(b.min_x, b.min_y), Point::new(b.max_x, b.min_y), Point::new(b.max_x, b.max_y), Point::new(b.min_x, b.max_y)];
    Some((b, enclosing, poly))
}

/// 🧿️ Screen-space overlay points for the shared `SelectionMarquee` overlay.
pub fn selection_screen_overlay_points(method: &str, start_screen: Point, screen_points: &[Point]) -> Option<Vec<Point>> {
    if screen_points.len() < 2 {
        return None;
    }
    let last = *screen_points.last().unwrap_or(&start_screen);
    Some(if method == "lasso" { screen_points.to_vec() } else { vec![start_screen, Point::new(last.x, start_screen.y), last, Point::new(start_screen.x, last.y)] })
}

/// 🧿️ Returns sorted ids for the next preselect set and removed anchor ids.
pub fn area_preselect_ids(anchor: &BTreeSet<String>, ids: &[String]) -> (Vec<String>, Vec<String>) {
    let next: BTreeSet<String> = ids.iter().cloned().collect();
    let mut sorted: Vec<_> = next.iter().cloned().collect();
    sorted.sort();
    let mut removed: Vec<_> = anchor.difference(&next).cloned().collect();
    removed.sort();
    (sorted, removed)
}

fn node_rect_bounds(center: Point, width: f64, height: f64) -> WorldBox {
    let hw = width * 0.5;
    let hh = height * 0.5;
    WorldBox { min_x: center.x - hw, min_y: center.y - hh, max_x: center.x + hw, max_y: center.y + hh }
}

fn node_circle_bounds(center: Point, radius: f64) -> WorldBox {
    WorldBox { min_x: center.x - radius, min_y: center.y - radius, max_x: center.x + radius, max_y: center.y + radius }
}

/// 🎯️ Tests whether a graph node body intersects or is contained by the marquee shape.
pub fn selection_contains_node_bounds(node: &Node, box_: WorldBox, enclosing: bool, polygon: &[Point], lasso: bool) -> bool {
    let bounds = match node.shape {
        NodeShape::Rectangle => node_rect_bounds(node.center, node.width, node.height),
        NodeShape::Circle => node_circle_bounds(node.center, node.radius),
    };
    if enclosing {
        if lasso {
            polygon_contains_world_box(polygon, bounds)
        } else {
            world_box_contains_box(box_, bounds)
        }
    } else if lasso {
        polygon_intersects_world_box(polygon, bounds)
    } else {
        world_boxes_overlap(box_, bounds)
    }
}

/// 🎯️ Tests whether a port handle intersects or is contained by the marquee shape.
pub fn selection_contains_handle_point(pos: Point, pad: f64, box_: WorldBox, enclosing: bool, polygon: &[Point], lasso: bool) -> bool {
    let bounds = WorldBox { min_x: pos.x - pad, min_y: pos.y - pad, max_x: pos.x + pad, max_y: pos.y + pad };
    if enclosing {
        if lasso {
            polygon_contains_world_box(polygon, bounds)
        } else {
            world_box_contains_box(box_, bounds)
        }
    } else if lasso {
        polygon_intersects_world_box(polygon, bounds)
    } else {
        world_boxes_overlap(box_, bounds)
    }
}

/// 🎯️ Tests whether a cubic edge intersects or is contained by the marquee shape.
pub fn selection_contains_edge_curve(curve: CubicBez, box_: WorldBox, enclosing: bool, polygon: &[Point], lasso: bool) -> bool {
    const STEPS: usize = 24;
    let mut samples = Vec::with_capacity(STEPS + 1);
    for i in 0..=STEPS {
        let t = i as f64 / STEPS as f64;
        samples.push(curve.eval(t));
    }
    if enclosing {
        if lasso {
            samples.iter().all(|&p| point_in_polygon(p, polygon))
        } else {
            samples.iter().all(|&p| world_box_contains_point(box_, p))
        }
    } else if lasso {
        (1..samples.len()).any(|i| segment_intersects_polygon(samples[i - 1], samples[i], polygon))
    } else {
        (1..samples.len()).any(|i| segment_intersects_world_box(samples[i - 1], samples[i], box_))
    }
}
// #endregion 🔖️SelectionMarquee

// #region 🔖️TransformGumball
pub const TRANSFORM_RING_GAP_PX: f64 = ui_styling::metrics::board::TRANSFORM_RING_GAP_PX;

pub const TRANSFORM_RING_MIN_RADIUS_PX: f64 = ui_styling::metrics::board::TRANSFORM_RING_MIN_RADIUS_PX;

pub const TRANSFORM_RING_HIT_TOLERANCE_PX: f64 = ui_styling::metrics::board::TRANSFORM_RING_HIT_TOLERANCE_PX;

pub const TRANSFORM_ROTATE_SNAP_RADIANS: f64 = ui_styling::metrics::board::TRANSFORM_ROTATE_SNAP_DEGREES * std::f64::consts::PI / 180.0;

/// 🕹️ Which selection-gumball handles a board surface offers. `move` is the board's native node
/// drag, `rotate` the ring around the selection centroid; scale is deliberately absent — a board
/// node's size comes from its kind catalog, never from a free drag (same law as puzzle-3d's gumball).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransformGumballFlags {
    pub move_enabled: bool,
    pub rotate_enabled: bool,
}

impl Default for TransformGumballFlags {
    fn default() -> Self {
        Self { move_enabled: true, rotate_enabled: true }
    }
}

/// 📐️ The arithmetic mean of the selected node centres — the pivot both the in-canvas rotate
/// preview and the guest's `rotateSelection` reducer turn about, so the preview and the committed
/// document agree exactly. Locked members still count toward the pivot, only their positions stay.
pub fn transform_pivot_of(centers: &[Point]) -> Option<Point> {
    if centers.is_empty() {
        return None;
    }
    let count = centers.len() as f64;
    let sum = centers.iter().fold((0.0, 0.0), |(x, y), p| (x + p.x, y + p.y));
    Some(Point::new(sum.0 / count, sum.1 / count))
}

/// 🔄️ Turns `point` about `pivot` by `radians` counter-clockwise in board world space.
pub fn rotate_point_about(pivot: Point, point: Point, radians: f64) -> Point {
    let (sin, cos) = radians.sin_cos();
    let dx = point.x - pivot.x;
    let dy = point.y - pivot.y;
    Point::new(pivot.x + dx * cos - dy * sin, pivot.y + dx * sin + dy * cos)
}

/// ⭕️ World radius of the rotate ring: the farthest selected corner plus a constant screen gap, so
/// the grab ring keeps the same pixel clearance at every zoom.
pub fn transform_ring_radius_world(pivot: Point, corners: &[Point], zoom: f64) -> f64 {
    let zoom = if zoom.is_finite() && zoom > 1e-9 { zoom } else { 1.0 };
    let reach = corners.iter().fold(0.0_f64, |far, corner| far.max(distance_between(pivot, *corner)));
    (reach + TRANSFORM_RING_GAP_PX / zoom).max(TRANSFORM_RING_MIN_RADIUS_PX / zoom)
}

/// 🎯️ Whether `point` lies on the rotate ring band — the gumball outranks nodes, handles and edges,
/// so this is asked before any other hit test while the ring is drawn.
pub fn transform_ring_hit(pivot: Point, radius_world: f64, zoom: f64, point: Point) -> bool {
    let zoom = if zoom.is_finite() && zoom > 1e-9 { zoom } else { 1.0 };
    (distance_between(pivot, point) - radius_world).abs() <= TRANSFORM_RING_HIT_TOLERANCE_PX / zoom
}

/// 📐️ Signed delta between two ring grabs, normalized to `(-π, π]` so a drag across the seam does
/// not jump a full turn.
pub fn transform_ring_angle_delta(pivot: Point, from: Point, to: Point) -> f64 {
    let start = (from.y - pivot.y).atan2(from.x - pivot.x);
    let end = (to.y - pivot.y).atan2(to.x - pivot.x);
    let mut delta = end - start;
    while delta > std::f64::consts::PI {
        delta -= std::f64::consts::TAU;
    }
    while delta <= -std::f64::consts::PI {
        delta += std::f64::consts::TAU;
    }
    delta
}

/// 🧲️ Quantizes a live rotation to the snap step when the grid-snap modifier is on.
pub fn snap_transform_angle(radians: f64, snap: bool) -> f64 {
    if !snap || !radians.is_finite() {
        return radians;
    }
    (radians / TRANSFORM_ROTATE_SNAP_RADIANS).round() * TRANSFORM_ROTATE_SNAP_RADIANS
}
// #endregion 🔖️TransformGumball

// #region 🔖️TargetRegions
pub const REGION_GRIP_PX: f64 = ui_styling::metrics::board::REGION_GRIP_PX;

pub const REGION_HIT_TOLERANCE_PX: f64 = ui_styling::metrics::board::REGION_HIT_TOLERANCE_PX;

pub const REGION_MIN_EXTENT_WORLD: f64 = ui_styling::metrics::board::REGION_MIN_EXTENT_WORLD;

pub const REGION_LABEL_INSET_PX: f64 = ui_styling::metrics::board::REGION_LABEL_INSET_PX;

/// 🎯️ One axis-aligned board rectangle constraining fill placement — the engine's own copy of the
/// document's `targetRegions` row. `width`/`height` are measured from `x`/`y`; a negative extent is
/// normalized by every reader through [`region_bounds`], so the corner a brush started from never
/// reaches a hit test or a paint.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RegionData {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub label: Option<String>,
    pub hidden: bool,
    pub locked: bool,
    pub selected: bool,
}

/// 📐️ Normalized `[min_x, min_y, max_x, max_y]` of a rectangle stated as corner-plus-extent.
pub fn region_bounds(x: f64, y: f64, width: f64, height: f64) -> [f64; 4] {
    let (min_x, max_x) = if width < 0.0 { (x + width, x) } else { (x, x + width) };
    let (min_y, max_y) = if height < 0.0 { (y + height, y) } else { (y, y + height) };
    [min_x, min_y, max_x, max_y]
}

/// 🤏️ Which part of a region rectangle a pointer grabbed. Corners outrank edges and edges outrank
/// the body, so a press in the overlap of two grips always resolves to the more specific one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionGrip {
    Body,
    West,
    East,
    North,
    South,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl RegionGrip {
    /// 🏷️ Wire name — the probe reads it back out of `data-board-interaction-json`.
    pub fn name(self) -> &'static str {
        match self {
            Self::Body => "body",
            Self::West => "west",
            Self::East => "east",
            Self::North => "north",
            Self::South => "south",
            Self::NorthWest => "northWest",
            Self::NorthEast => "northEast",
            Self::SouthWest => "southWest",
            Self::SouthEast => "southEast",
        }
    }

    /// 📐️ Whether this grip moves the minimum/maximum edge on each axis: `(min_x, max_x, min_y, max_y)`.
    /// `Body` moves all four, which is exactly a translation.
    pub fn edges(self) -> (bool, bool, bool, bool) {
        match self {
            Self::Body => (true, true, true, true),
            Self::West => (true, false, false, false),
            Self::East => (false, true, false, false),
            Self::North => (false, false, true, false),
            Self::South => (false, false, false, true),
            Self::NorthWest => (true, false, true, false),
            Self::NorthEast => (false, true, true, false),
            Self::SouthWest => (true, false, false, true),
            Self::SouthEast => (false, true, false, true),
        }
    }
}

/// 🎯️ The grip `point` lands on, or `None` when it misses the rectangle and its grip band entirely.
/// The band is a constant SCREEN width, so grips stay grabbable at every zoom.
pub fn region_grip_at(bounds: [f64; 4], zoom: f64, point: Point) -> Option<RegionGrip> {
    let zoom = if zoom.is_finite() && zoom > 1e-9 { zoom } else { 1.0 };
    let tolerance = REGION_HIT_TOLERANCE_PX / zoom;
    let [min_x, min_y, max_x, max_y] = bounds;
    if point.x < min_x - tolerance || point.x > max_x + tolerance || point.y < min_y - tolerance || point.y > max_y + tolerance {
        return None;
    }
    let west = (point.x - min_x).abs() <= tolerance;
    let east = (point.x - max_x).abs() <= tolerance;
    let north = (point.y - min_y).abs() <= tolerance;
    let south = (point.y - max_y).abs() <= tolerance;
    Some(match (west, east, north, south) {
        (true, _, true, _) => RegionGrip::NorthWest,
        (_, true, true, _) => RegionGrip::NorthEast,
        (true, _, _, true) => RegionGrip::SouthWest,
        (_, true, _, true) => RegionGrip::SouthEast,
        (true, _, _, _) => RegionGrip::West,
        (_, true, _, _) => RegionGrip::East,
        (_, _, true, _) => RegionGrip::North,
        (_, _, _, true) => RegionGrip::South,
        _ => RegionGrip::Body,
    })
}

/// 🚚️ Applies a world drag delta to the edges this grip owns and answers the new normalized bounds.
/// Neither extent may collapse: an edge pushed past its opposite is clamped at [`REGION_MIN_EXTENT_WORLD`],
/// so a resize can never mint a zero- or negative-area region the fill rule would read as "nothing fits".
pub fn region_grip_drag(bounds: [f64; 4], grip: RegionGrip, dx: f64, dy: f64) -> [f64; 4] {
    let (move_min_x, move_max_x, move_min_y, move_max_y) = grip.edges();
    let [mut min_x, mut min_y, mut max_x, mut max_y] = bounds;
    if move_min_x {
        min_x += dx;
    }
    if move_max_x {
        max_x += dx;
    }
    if move_min_y {
        min_y += dy;
    }
    if move_max_y {
        max_y += dy;
    }
    if grip != RegionGrip::Body {
        if max_x - min_x < REGION_MIN_EXTENT_WORLD {
            if move_min_x {
                min_x = max_x - REGION_MIN_EXTENT_WORLD;
            } else {
                max_x = min_x + REGION_MIN_EXTENT_WORLD;
            }
        }
        if max_y - min_y < REGION_MIN_EXTENT_WORLD {
            if move_min_y {
                min_y = max_y - REGION_MIN_EXTENT_WORLD;
            } else {
                max_y = min_y + REGION_MIN_EXTENT_WORLD;
            }
        }
    }
    [min_x, min_y, max_x, max_y]
}

/// 🧲️ Quantizes one world scalar to `step`; a non-positive or absent step leaves it alone. The one
/// snap definition the area brush, the region move and the region resize all share.
pub fn snap_region_scalar(value: f64, step: Option<f64>) -> f64 {
    match step {
        Some(step) if step.is_finite() && step > 1e-9 && value.is_finite() => (value / step).round() * step,
        _ => value,
    }
}
// #endregion 🔖️TargetRegions

// #region 🔖️Engine

/// 🎯️ Engine-local area-select options.
#[derive(Clone, Debug, PartialEq)]
pub struct EngineSelectionOptions {
    pub method: String,
    pub mode: String,
    pub select_nodes: bool,
    pub select_handles: bool,
    pub select_edges: bool,
}

impl Default for EngineSelectionOptions {
    fn default() -> Self {
        Self { method: "rectangle".into(), mode: "replace".into(), select_nodes: true, select_handles: true, select_edges: true }
    }
}

/// ⚙️ Retained graph engine parameterized by port model and directedness.
#[derive(Clone, Debug)]
pub struct GraphEngine<P: GraphPortModel, D: Directedness> {
    pub camera: Camera,
    pub edges: HistoryFoldIndex<EdgeId, GraphEdge<P::Endpoint>>,
    pub edge_semantics: HistoryFoldIndex<EdgeId, ElementSemantics>,
    pub enforce_acyclic: bool,
    pub events: Vec<BoardEvent>,
    pub handles: HistoryFoldIndex<HandleId, Handle>,
    pub hover: Option<u64>,
    pub interaction: InteractionMode,
    pub nodes: HistoryFoldIndex<NodeId, Node>,
    pub selection: Selection,
    pub preselect: Selection,
    pub preselect_removed: Selection,
    pub selection_options: EngineSelectionOptions,
    pub handle_pointer_picking: bool,
    pub proximity_distance_world: f64,
    proximity_distance_override: Option<f64>,
    pub selection_preview_points: Vec<Point>,
    pub selection_preview_crossing: bool,
    area_initial: Selection,
    area_points: Vec<Point>,
    area_screen_points: Vec<Point>,
    drag_start_positions: HistoryFoldIndex<NodeId, RetainedPoint>,
    proximity_connection: Option<ProximityConnection>,
    next_edge_id: u64,
    retirement_backing_credited_bytes: usize,
    _directedness: std::marker::PhantomData<D>,
    _port: std::marker::PhantomData<P>,
}

/// 🧱️ One graph-backing retirement turn with current credit and physical release split.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GraphEngineBackingRetirementStep {
    Blocked,
    Pending { credited_bytes: usize, released_bytes: usize },
    Complete,
}

impl<P: GraphPortModel, D: Directedness> Default for GraphEngine<P, D> {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            edges: HistoryFoldIndex::new(),
            edge_semantics: HistoryFoldIndex::new(),
            enforce_acyclic: false,
            events: Vec::new(),
            handles: HistoryFoldIndex::new(),
            hover: None,
            interaction: InteractionMode::default(),
            nodes: HistoryFoldIndex::new(),
            selection: Selection::default(),
            preselect: Selection::default(),
            preselect_removed: Selection::default(),
            selection_options: EngineSelectionOptions::default(),
            handle_pointer_picking: true,
            proximity_distance_world: DEFAULT_PROXIMITY_DISTANCE_WORLD,
            proximity_distance_override: None,
            selection_preview_points: Vec::new(),
            selection_preview_crossing: false,
            area_initial: Selection::default(),
            area_points: Vec::new(),
            area_screen_points: Vec::new(),
            drag_start_positions: HistoryFoldIndex::new(),
            proximity_connection: None,
            next_edge_id: 1000,
            retirement_backing_credited_bytes: 0,
            _directedness: std::marker::PhantomData,
            _port: std::marker::PhantomData,
        }
    }
}

impl<P: GraphPortModel, D: Directedness> GraphEngine<P, D> {
    /// 🧹️ Releases one retained graph owner or scalar from the engine per call.
    pub fn close_step(&mut self) -> bool {
        if self.edges.pop_first().is_some()
            || self.edge_semantics.pop_first().is_some()
            || self.events.pop().is_some()
            || self.handles.pop_first().is_some()
            || self.nodes.pop_first().is_some()
            || self.selection.edge_ids.pop_first().is_some()
            || self.selection.handle_ids.pop_first().is_some()
            || self.selection.node_ids.pop_first().is_some()
            || self.preselect.edge_ids.pop_first().is_some()
            || self.preselect.handle_ids.pop_first().is_some()
            || self.preselect.node_ids.pop_first().is_some()
            || self.preselect_removed.edge_ids.pop_first().is_some()
            || self.preselect_removed.handle_ids.pop_first().is_some()
            || self.preselect_removed.node_ids.pop_first().is_some()
            || self.selection_preview_points.pop().is_some()
            || self.area_initial.edge_ids.pop_first().is_some()
            || self.area_initial.handle_ids.pop_first().is_some()
            || self.area_initial.node_ids.pop_first().is_some()
            || self.area_points.pop().is_some()
            || self.area_screen_points.pop().is_some()
            || self.drag_start_positions.pop_first().is_some()
            || self.selection_options.method.pop().is_some()
            || self.selection_options.mode.pop().is_some()
        {
            return false;
        }
        self.hover = None;
        self.interaction = InteractionMode::default();
        self.proximity_distance_override = None;
        self.proximity_connection = None;
        true
    }

    /// 🔍️ Proves every non-scalar graph owner governed by `close_step` is empty.
    pub fn terminal_is_empty(&self) -> bool {
        self.edges.is_empty()
            && self.edge_semantics.is_empty()
            && self.events.is_empty()
            && self.handles.is_empty()
            && self.nodes.is_empty()
            && self.selection.edge_ids.is_empty()
            && self.selection.handle_ids.is_empty()
            && self.selection.node_ids.is_empty()
            && self.preselect.edge_ids.is_empty()
            && self.preselect.handle_ids.is_empty()
            && self.preselect.node_ids.is_empty()
            && self.preselect_removed.edge_ids.is_empty()
            && self.preselect_removed.handle_ids.is_empty()
            && self.preselect_removed.node_ids.is_empty()
            && self.selection_preview_points.is_empty()
            && self.area_initial.edge_ids.is_empty()
            && self.area_initial.handle_ids.is_empty()
            && self.area_initial.node_ids.is_empty()
            && self.area_points.is_empty()
            && self.area_screen_points.is_empty()
            && self.drag_start_positions.is_empty()
            && self.selection_options.method.is_empty()
            && self.selection_options.mode.is_empty()
            && self.hover.is_none()
            && self.proximity_distance_override.is_none()
            && self.proximity_connection.is_none()
    }

    /// 🧹️ Releases one empty engine vector backing only when the caller grants its full retained capacity.
    pub fn close_backing_page(&mut self, maximum_bytes: usize) -> GraphEngineBackingRetirementStep {
        if !self.terminal_is_empty() {
            return GraphEngineBackingRetirementStep::Blocked;
        }
        macro_rules! retire_backing {
            ($field:expr, $element:ty) => {
                if $field.capacity() != 0 {
                    let released_bytes = $field.capacity().saturating_mul(std::mem::size_of::<$element>());
                    let remaining_bytes = released_bytes.saturating_sub(self.retirement_backing_credited_bytes);
                    let credited_bytes = maximum_bytes.min(remaining_bytes);
                    self.retirement_backing_credited_bytes = self.retirement_backing_credited_bytes.saturating_add(credited_bytes);
                    if self.retirement_backing_credited_bytes != released_bytes {
                        return GraphEngineBackingRetirementStep::Pending { credited_bytes, released_bytes: 0 };
                    }
                    drop(std::mem::take($field));
                    self.retirement_backing_credited_bytes = 0;
                    return GraphEngineBackingRetirementStep::Pending { credited_bytes, released_bytes };
                }
            };
        }
        retire_backing!(&mut self.events, BoardEvent);
        retire_backing!(&mut self.selection_preview_points, Point);
        retire_backing!(&mut self.area_points, Point);
        retire_backing!(&mut self.area_screen_points, Point);
        for selection in [&mut self.selection,&mut self.preselect,&mut self.preselect_removed,&mut self.area_initial]{
            for values in [&mut selection.edge_ids,&mut selection.handle_ids,&mut selection.node_ids]{
                if !values.terminal_is_empty(){let bytes=values.next_close_release_byte_demand().expect("finite numeric selection backing demand");let credited_bytes=maximum_bytes.min(bytes.saturating_sub(self.retirement_backing_credited_bytes));self.retirement_backing_credited_bytes+=credited_bytes;if self.retirement_backing_credited_bytes<bytes{return GraphEngineBackingRetirementStep::Pending{credited_bytes,released_bytes:0}}let progress=values.close_copy_step(1,self.retirement_backing_credited_bytes).expect("admitted numeric selection backing release");if progress.progressed{self.retirement_backing_credited_bytes=0;}return GraphEngineBackingRetirementStep::Pending{credited_bytes,released_bytes:progress.released_allocation_bytes};}
            }
        }
        debug_assert_eq!(self.retirement_backing_credited_bytes, 0);
        GraphEngineBackingRetirementStep::Complete
    }
}

impl<P: GraphPortModel, D: Directedness> GraphEngine<P, D> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.camera = Camera { x, y, zoom };
    }

    pub fn create_node(&mut self, id: NodeId, x: f64, y: f64, radius: f64, draggable: bool) {
        self.nodes.insert(id, Node { center: Point::new(x, y), draggable, height: radius * 2.0, id, radius, shape: NodeShape::Circle, width: radius * 2.0, kind: None, label: None, properties: PropertyBag::new() });
    }

    pub fn create_rect_node(&mut self, id: NodeId, x: f64, y: f64, width: f64, height: f64, draggable: bool) {
        let hw = width * 0.5;
        let hh = height * 0.5;
        self.nodes.insert(id, Node { center: Point::new(x, y), draggable, height, id, radius: hw.max(hh).max(ui_styling::radii::NODE_MIN), shape: NodeShape::Rectangle, width, kind: None, label: None, properties: PropertyBag::new() });
    }

    pub fn update_node(&mut self, id: NodeId, x: f64, y: f64, radius: f64) {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.center = Point::new(x, y);
            node.radius = radius;
            if node.shape == NodeShape::Circle {
                node.width = radius * 2.0;
                node.height = radius * 2.0;
            }
        }
    }

    pub fn set_next_edge_id(&mut self, id: u64) {
        self.next_edge_id = id;
    }

    pub fn remove_node(&mut self, id: NodeId) {
        self.nodes.remove(&id);
        if P::HAS_PORTS {
            let removed_handles: Vec<HandleId> = self.handles.values().filter(|handle| handle.node_id == id).map(|handle| handle.id).collect();
            for handle_id in removed_handles {
                self.remove_handle(handle_id);
            }
        } else {
            let removed_edges: Vec<EdgeId> = self.edges.values().filter(|edge| P::endpoint_as_u64(edge.source) == id || P::endpoint_as_u64(edge.target) == id).map(|edge| edge.id).collect();
            for edge_id in removed_edges {
                self.edges.remove(&edge_id);
                self.selection.edge_ids.remove(&edge_id);
            }
        }
        self.selection.node_ids.remove(&id);
        self.push_selection_event();
    }

    pub fn create_handle(&mut self, id: HandleId, node_id: NodeId, angle: f64) {
        if P::HAS_PORTS {
            self.handles.insert(id, Handle { angle, id, node_id, radius: ui_styling::radii::HANDLE_DEFAULT, role: HandleRole::Any, kind: None, value_types: Vec::new(), properties: PropertyBag::new() });
        }
    }

    pub fn set_handle_role(&mut self, id: HandleId, role: HandleRole) {
        if let Some(handle) = self.handles.get_mut(&id) {
            handle.role = role;
        }
    }

    /// 🔤️ Declares the value schemas a port carries, so `is_valid_connection` can refuse a wire the
    /// target port could never read. The board never invents these — they arrive with the port.
    ///
    /// @see `🧫️fixtures/🔌️port-types/🔣️.json` — the law both node-graph renderers answer
    pub fn set_handle_value_types(&mut self, id: HandleId, value_types: Vec<String>) {
        if let Some(handle) = self.handles.get_mut(&id) {
            handle.value_types = value_types;
        }
    }

    /// 🔌️ The ONE port-compatibility rule the board enforces: refuse only when both ends declare and
    /// the declared sets are disjoint.
    pub fn port_value_types_compatible(source: &[String], target: &[String]) -> bool {
        if source.is_empty() || target.is_empty() {
            return true;
        }
        source.iter().any(|provided| target.iter().any(|accepted| accepted == provided))
    }

    /// 🔌️ Whether a wire drawn from `source_hid` may land on `target_hid` given only the declared
    /// port types — the predicate a drag preview asks to paint a snap target as not-droppable.
    pub fn handles_type_compatible(&self, source_hid: HandleId, target_hid: HandleId) -> bool {
        let (Some(source), Some(target)) = (self.handles.get(&source_hid), self.handles.get(&target_hid)) else {
            return true;
        };
        Self::port_value_types_compatible(&source.value_types, &target.value_types)
    }

    pub fn create_edge(&mut self, id: EdgeId, source: P::Endpoint, target: P::Endpoint) {
        let (source, target) = orient_endpoints::<P::Endpoint, D>(source, target);
        self.edges.insert(id, GraphEdge { id, source, target });
        if id >= self.next_edge_id {
            self.next_edge_id = id + 1;
        }
    }

    pub fn remove_edge(&mut self, id: EdgeId) {
        if self.edges.remove(&id).is_some() {
            self.edge_semantics.remove(&id);
            self.selection.edge_ids.remove(&id);
            self.events.push(BoardEvent::EdgeRemoved { id });
        }
    }

    pub fn set_edge_semantics(&mut self, id: EdgeId, kind: Option<String>, properties: PropertyBag) {
        self.edge_semantics.insert(id, ElementSemantics { kind, properties });
    }

    pub fn set_selection_options(&mut self, method: &str, mode: &str, select_nodes: bool, select_handles: bool, select_edges: bool) {
        self.selection_options.method = method.to_string();
        self.selection_options.mode = normalize_selection_mode(mode);
        self.selection_options.select_nodes = select_nodes;
        self.selection_options.select_handles = select_handles;
        self.selection_options.select_edges = select_edges;
    }

    pub fn selection_preview_points(&self) -> &[Point] {
        &self.selection_preview_points
    }

    pub fn selection_preview_crossing(&self) -> bool {
        self.selection_preview_crossing
    }

    pub fn selection_preview_method(&self) -> &str {
        self.selection_options.method.as_str()
    }

    pub fn cancel_area_select(&mut self) -> bool {
        let prev = std::mem::replace(&mut self.interaction, InteractionMode::Idle);
        let cancelled = matches!(prev, InteractionMode::SelectionPending { .. } | InteractionMode::AreaSelect { .. });
        if cancelled {
            self.selection = self.area_initial.clone();
            self.clear_preselect();
            self.selection_preview_points.clear();
            self.selection_preview_crossing = false;
            self.push_selection_event();
        }
        cancelled
    }

    pub fn pointer_cancel_screen(&mut self) -> bool {
        let interaction = std::mem::replace(&mut self.interaction, InteractionMode::Idle);
        let cancelled = !matches!(interaction, InteractionMode::Idle);
        match interaction {
            InteractionMode::DragNode { .. } | InteractionMode::DragNodes { .. } => {
                for (id, start) in &self.drag_start_positions {
                    if let Some(node) = self.nodes.get_mut(id) {
                        node.center = start.0;
                    }
                }
            }
            InteractionMode::SelectionPending { .. } | InteractionMode::AreaSelect { .. } => {
                self.selection = self.area_initial.clone();
            }
            InteractionMode::DrawEdge { .. } | InteractionMode::Pan { .. } | InteractionMode::Idle => {}
        }
        self.proximity_connection = None;
        self.drag_start_positions.clear();
        self.clear_preselect();
        self.area_points.clear();
        self.area_screen_points.clear();
        self.selection_preview_points.clear();
        self.selection_preview_crossing = false;
        self.events.clear();
        cancelled
    }

    pub fn select_all(&mut self) {
        self.selection = Selection::default();
        if self.selection_options.select_nodes {
            self.selection.node_ids = self.nodes.keys().copied().collect();
        }
        if self.selection_options.select_handles && P::HAS_PORTS {
            self.selection.handle_ids = self.handles.keys().copied().collect();
        }
        if self.selection_options.select_edges {
            self.selection.edge_ids = self.edges.keys().copied().collect();
        }
        self.clear_preselect();
        self.push_selection_event();
    }

    pub fn delete_selection(&mut self) {
        let node_ids: Vec<_> = self.selection.node_ids.iter().copied().collect();
        for id in node_ids {
            self.remove_node(id);
        }
        let edge_ids: Vec<_> = self.selection.edge_ids.iter().copied().collect();
        for id in edge_ids {
            self.remove_edge(id);
        }
        if P::HAS_PORTS {
            let handle_ids: Vec<_> = self.selection.handle_ids.iter().copied().collect();
            for id in handle_ids {
                self.remove_handle(id);
            }
        }
        self.clear_preselect();
    }

    pub fn pointer_down(&mut self, x: f64, y: f64, extend_selection: bool) {
        self.pointer_down_screen(x, y, x, y, 0, extend_selection, false, false);
    }

    /// 📦️ Starts a group drag when `point` lies inside the padded union bounds of draggable selected nodes.
    pub fn try_begin_selection_union_drag_at(&mut self, point: Point, pad_world: f64) -> bool {
        let members: Vec<NodeId> = self.selection.node_ids.iter().copied().filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable)).collect();
        if members.is_empty() {
            return false;
        }
        let mut corners = Vec::new();
        for id in &self.selection.node_ids {
            let Some(node) = self.nodes.get(id) else {
                continue;
            };
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            corners.push(Point::new(node.center.x - hw, node.center.y - hh));
            corners.push(Point::new(node.center.x + hw, node.center.y + hh));
        }
        let Some(bounds) = world_box_from_points(&corners) else {
            return false;
        };
        if !world_box_contains_point(inflate_world_box(bounds, pad_world), point) {
            return false;
        }
        let primary_id = members
            .iter()
            .min_by(|a, b| {
                let da = self.nodes.get(a).map_or(f64::INFINITY, |n| distance_between(point, n.center));
                let db = self.nodes.get(b).map_or(f64::INFINITY, |n| distance_between(point, n.center));
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
            .unwrap_or(members[0]);
        let Some(primary) = self.nodes.get(&primary_id) else {
            return false;
        };
        self.drag_start_positions.clear();
        for id in &members {
            if let Some(node) = self.nodes.get(id) {
                self.drag_start_positions.insert(*id, RetainedPoint(node.center));
            }
        }
        self.interaction = InteractionMode::DragNodes { primary_id, offset: point - primary.center };
        self.hover = None;
        true
    }

    /// 🫳️ Starts dragging a selected draggable node (or its multi-selection group) from `point`.
    pub fn try_begin_selected_node_drag_at(&mut self, node_id: NodeId, point: Point) -> bool {
        if !self.selection.node_ids.contains(&node_id) {
            return false;
        }
        let Some(node) = self.nodes.get(&node_id) else {
            return false;
        };
        if !node.draggable {
            return false;
        }
        let members: Vec<NodeId> = self.selection.node_ids.iter().copied().filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable)).collect();
        let drag_group = members.contains(&node_id) && members.len() > 1;
        self.drag_start_positions.clear();
        for id in if drag_group { members.as_slice() } else { std::slice::from_ref(&node_id) } {
            if let Some(n) = self.nodes.get(id) {
                self.drag_start_positions.insert(*id, RetainedPoint(n.center));
            }
        }
        if drag_group {
            self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center };
        } else {
            self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center };
        }
        self.hover = None;
        true
    }

    /// 🫳️ Selects a draggable node and starts moving it from `point`.
    pub fn pointer_down_on_draggable_node_at(&mut self, node_id: NodeId, point: Point, shift: bool, ctrl_or_meta: bool) {
        let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
        let merge_from_modifiers = ctrl_or_meta || shift;
        let members_before: Vec<NodeId> = self.selection.node_ids.iter().copied().filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable)).collect();
        let drag_group_before = members_before.contains(&node_id) && members_before.len() > 1;
        let force_pick_merge = (merge_mode == "replace" && !drag_group_before) || merge_mode == "subtractive" || (merge_mode == "invertive" && merge_from_modifiers);
        if !drag_group_before || force_pick_merge {
            self.apply_pick_with_mode(HitObject::Node(node_id), merge_mode.as_str());
        }
        if let Some(node) = self.nodes.get(&node_id) {
            if node.draggable {
                let members: Vec<NodeId> = self.selection.node_ids.iter().copied().filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable)).collect();
                let drag_group = members.contains(&node_id) && members.len() > 1;
                self.drag_start_positions.clear();
                for id in if drag_group { members.as_slice() } else { std::slice::from_ref(&node_id) } {
                    if let Some(n) = self.nodes.get(id) {
                        self.drag_start_positions.insert(*id, RetainedPoint(n.center));
                    }
                }
                if drag_group {
                    self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center };
                } else {
                    self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center };
                }
            }
        }
        self.update_hover(Some(node_id));
    }

    /// 🪝️ Merges a handle into the engine selection without starting edge draw.
    pub fn select_handle_with_mode(&mut self, handle_id: HandleId, mode: &str) {
        if !self.selection_options.select_handles || !P::HAS_PORTS {
            return;
        }
        let current: BTreeSet<String> = self.selection.handle_ids.iter().map(|id| id.to_string()).collect();
        let next = merge_pick_into_selection(&current, &handle_id.to_string(), mode);
        self.selection.handle_ids = next.iter().filter_map(|s| s.parse().ok()).collect();
        self.clear_preselect();
        self.push_selection_event();
    }

    /// 🖱️ Flat pointer-event fields (screen/world coords, button, modifiers) mirror this repo's shared WASM host-bridge pointer contract, matched by every sibling board/editor/layout crate's `pointer_*_screen` — not bundled into a struct, to keep the JS-callable surface a plain positional arg list.
    /// Primary (`button == 0`) selects and may start drag / marquee / edge-draw. Secondary (`button == 2`) only updates selection/hover for context menus — never moves nodes.
    ///
    /// 🖱️ Secondary/middle empty presses must not clear selection — context menus and pan gestures own those buttons.
    #[allow(clippy::too_many_arguments, reason = "flat args mirror the shared WASM host-bridge pointer-event contract used across all `pointer_*_screen` methods in this repo")]
    pub fn pointer_down_screen(&mut self, screen_x: f64, screen_y: f64, world_x: f64, world_y: f64, button: u8, shift: bool, ctrl_or_meta: bool, _alt: bool) {
        self.proximity_connection = None;
        self.selection_preview_points.clear();
        self.selection_preview_crossing = false;
        let point = Point::new(world_x, world_y);
        let screen = Point::new(screen_x, screen_y);
        let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
        let merge_from_modifiers = ctrl_or_meta || shift;
        let primary = button == 0;
        match self.hit_test(point) {
            Some(HitObject::Node(node_id)) => {
                let members_before: Vec<NodeId> = self.selection.node_ids.iter().copied().filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable)).collect();
                let drag_group_before = members_before.contains(&node_id) && members_before.len() > 1;
                let force_pick_merge = (merge_mode == "replace" && !drag_group_before) || merge_mode == "subtractive" || (merge_mode == "invertive" && merge_from_modifiers);
                if !drag_group_before || force_pick_merge {
                    self.apply_pick_with_mode(HitObject::Node(node_id), merge_mode.as_str());
                }
                if primary {
                    if let Some(node) = self.nodes.get(&node_id) {
                        if node.draggable {
                            let members: Vec<NodeId> = self.selection.node_ids.iter().copied().filter(|id| self.nodes.get(id).is_some_and(|n| n.draggable)).collect();
                            let drag_group = members.contains(&node_id) && members.len() > 1;
                            self.drag_start_positions.clear();
                            for id in if drag_group { members.as_slice() } else { std::slice::from_ref(&node_id) } {
                                if let Some(n) = self.nodes.get(id) {
                                    self.drag_start_positions.insert(*id, RetainedPoint(n.center));
                                }
                            }
                            if drag_group {
                                self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center };
                            } else {
                                self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center };
                            }
                        }
                    }
                } else {
                    self.interaction = InteractionMode::Idle;
                }
                self.update_hover(Some(node_id));
            }
            Some(HitObject::Endpoint(ep)) => {
                self.apply_pick_with_mode(HitObject::Endpoint(ep), merge_mode.as_str());
                let hid = P::endpoint_as_u64(ep);
                self.update_hover(Some(hid));
                if primary && P::HAS_PORTS {
                    self.begin_draw_edge_from_handle(hid, point);
                } else {
                    self.interaction = InteractionMode::Idle;
                }
            }
            Some(HitObject::Edge(edge_id)) => {
                self.apply_pick_with_mode(HitObject::Edge(edge_id), merge_mode.as_str());
                self.update_hover(Some(edge_id));
                self.interaction = InteractionMode::Idle;
            }
            None if primary => {
                self.area_initial = self.selection.clone();
                self.interaction = InteractionMode::SelectionPending { start: point, start_screen: screen };
                self.update_hover(None);
            }
            None => {
                self.update_hover(None);
                self.interaction = InteractionMode::Idle;
            }
        }
    }

    pub fn pointer_move(&mut self, x: f64, y: f64) {
        self.pointer_move_screen(x, y, x, y, false, false, false);
    }

    #[allow(clippy::too_many_arguments, reason = "flat args mirror the shared WASM host-bridge pointer-event contract used across all `pointer_*_screen` methods in this repo")]
    pub fn pointer_move_screen(&mut self, screen_x: f64, screen_y: f64, world_x: f64, world_y: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        let point = Point::new(world_x, world_y);
        let screen = Point::new(screen_x, screen_y);
        match std::mem::replace(&mut self.interaction, InteractionMode::Idle) {
            InteractionMode::Pan { start_screen, cam_x, cam_y, zoom } => {
                let dx = (screen.x - start_screen.x) / zoom;
                let dy = (screen.y - start_screen.y) / zoom;
                self.set_camera(cam_x - dx, cam_y - dy, zoom);
                self.interaction = InteractionMode::Pan { start_screen, cam_x, cam_y, zoom };
            }
            InteractionMode::DragNode { node_id, offset } => {
                if alt {
                    self.proximity_distance_override = Some(0.0);
                }
                if let Some(node) = self.nodes.get_mut(&node_id) {
                    node.center = point - offset;
                    self.events.push(BoardEvent::NodeMoved { id: node_id, x: node.center.x(), y: node.center.y() });
                }
                self.update_node_drag_proximity(&[node_id]);
                if alt {
                    self.proximity_distance_override = None;
                    self.proximity_connection = None;
                }
                self.interaction = InteractionMode::DragNode { node_id, offset };
            }
            InteractionMode::DragNodes { primary_id, offset } => {
                let Some((px0, py0)) = self.drag_start_positions.get(&primary_id).map(|p| (p.x, p.y)) else {
                    self.interaction = InteractionMode::Idle;
                    return;
                };
                let nx = point.x - offset.x;
                let ny = point.y - offset.y;
                let dx = nx - px0;
                let dy = ny - py0;
                let dragged_ids: Vec<NodeId> = self.drag_start_positions.keys().copied().collect();
                if alt {
                    self.proximity_distance_override = Some(0.0);
                }
                for (id, start) in &self.drag_start_positions {
                    if let Some(node) = self.nodes.get_mut(id) {
                        node.center = Point::new(start.x + dx, start.y + dy);
                        self.events.push(BoardEvent::NodeMoved { id: *id, x: node.center.x, y: node.center.y });
                    }
                }
                self.update_node_drag_proximity(&dragged_ids);
                if alt {
                    self.proximity_distance_override = None;
                    self.proximity_connection = None;
                }
                self.interaction = InteractionMode::DragNodes { primary_id, offset };
            }
            InteractionMode::DrawEdge { anchor_handle, anchor_is_source, fixed_target, reconnecting, .. } => {
                if alt {
                    self.proximity_distance_override = Some(0.0);
                }
                let snap_target = if alt { None } else { self.nearest_wire_snap_handle(anchor_handle, anchor_is_source, fixed_target, reconnecting, point) };
                if alt {
                    self.proximity_distance_override = None;
                }
                self.interaction = InteractionMode::DrawEdge { anchor_handle, anchor_is_source, fixed_target, cursor: point, reconnecting, snap_target };
                let hover = snap_target.or_else(|| {
                    self.hit_test(point).map(|hit| match hit {
                        HitObject::Edge(id) => id,
                        HitObject::Endpoint(ep) => P::endpoint_as_u64(ep),
                        HitObject::Node(id) => id,
                    })
                });
                self.update_hover(hover);
            }
            InteractionMode::SelectionPending { start, start_screen } => {
                if distance_between(screen, start_screen) < SELECTION_MARQUEE_DRAG_THRESHOLD_PX {
                    self.interaction = InteractionMode::SelectionPending { start, start_screen };
                } else {
                    let area_points = vec![start, point];
                    let area_screen_points = vec![start_screen, screen];
                    self.area_points = area_points.clone();
                    self.area_screen_points = area_screen_points.clone();
                    self.apply_area_preselect(start, &area_points, shift, ctrl_or_meta);
                    self.sync_selection_screen_overlay(start_screen, &area_screen_points);
                    self.interaction = InteractionMode::AreaSelect { start, start_screen };
                }
            }
            InteractionMode::AreaSelect { start, start_screen } => {
                let mut points = self.area_points.clone();
                let mut screen_points = self.area_screen_points.clone();
                let last_screen = screen_points.last().copied().unwrap_or(start_screen);
                let add_point = self.selection_options.method == "lasso" || distance_between(screen, last_screen) >= SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
                if add_point {
                    points.push(point);
                    screen_points.push(screen);
                } else if !points.is_empty() {
                    let last = points.len() - 1;
                    points[last] = point;
                    let ls = screen_points.len() - 1;
                    screen_points[ls] = screen;
                }
                let points_for_preselect = points.clone();
                let screen_for_overlay = screen_points.clone();
                self.apply_area_preselect(start, &points_for_preselect, shift, ctrl_or_meta);
                self.sync_selection_screen_overlay(start_screen, &screen_for_overlay);
                self.area_points = points;
                self.area_screen_points = screen_points;
                self.interaction = InteractionMode::AreaSelect { start, start_screen };
            }
            InteractionMode::Idle => {
                self.interaction = InteractionMode::Idle;
                self.update_hover(self.hit_test(point).map(|hit| match hit {
                    HitObject::Edge(id) => id,
                    HitObject::Endpoint(ep) => P::endpoint_as_u64(ep),
                    HitObject::Node(id) => id,
                }));
            }
        }
    }

    pub fn pointer_up(&mut self, x: f64, y: f64) {
        self.pointer_up_screen(x, y, x, y, false, false, false);
    }

    /// 🔌️ Dragging a wired endpoint off its port and releasing over nothing is the
    /// reconnect gesture's other outcome: the wire is CUT. Leaving it attached made
    /// wire deletion unreachable from the pointer altogether.
    #[allow(clippy::too_many_arguments, reason = "flat args mirror the shared WASM host-bridge pointer-event contract used across all `pointer_*_screen` methods in this repo")]
    pub fn pointer_up_screen(&mut self, screen_x: f64, screen_y: f64, world_x: f64, world_y: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        let point = Point::new(world_x, world_y);
        let screen = Point::new(screen_x, screen_y);
        let grabbed = std::mem::replace(&mut self.interaction, InteractionMode::Idle);
        let node_drag_proximity = self.proximity_connection.take();
        match grabbed {
            InteractionMode::DrawEdge { anchor_handle, anchor_is_source, fixed_target, reconnecting, snap_target, .. } => {
                let endpoint = snap_target.filter(|hid| self.wire_snap_still_active(*hid, point)).or_else(|| {
                    self.hit_test(point).and_then(|hit| match hit {
                        HitObject::Endpoint(ep) => Some(P::endpoint_as_u64(ep)),
                        _ => None,
                    })
                });
                if !alt {
                    if let Some(hit_hid) = endpoint {
                        let (source_hid, target_handle) = if let Some(tgt) = fixed_target {
                            (hit_hid, tgt)
                        } else if anchor_is_source {
                            (anchor_handle, hit_hid)
                        } else {
                            (hit_hid, anchor_handle)
                        };
                        self.try_connect_handles(source_hid, target_handle, reconnecting);
                    } else if let Some(edge_id) = reconnecting {
                        self.remove_edge(edge_id);
                    }
                }
            }
            InteractionMode::DragNodes { .. } | InteractionMode::DragNode { .. } => {
                if !alt {
                    if let Some(conn) = node_drag_proximity {
                        self.try_connect_handles(conn.source, conn.target, None);
                    }
                }
            }
            InteractionMode::SelectionPending { start, start_screen } => {
                let merge_from_modifiers = ctrl_or_meta || shift;
                if !merge_from_modifiers {
                    self.selection = Selection::default();
                    self.push_selection_event();
                } else {
                    let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                    let next = self.resolve_area_hits(&self.area_initial_string_set(), start, &[start], merge_mode.as_str());
                    self.commit_selection_from_hits(&next);
                }
                let _ = (start_screen, start);
                self.clear_preselect();
                self.selection_preview_points.clear();
                self.selection_preview_crossing = false;
            }
            InteractionMode::AreaSelect { start, start_screen } => {
                let mut points = self.area_points.clone();
                let mut screen_points = self.area_screen_points.clone();
                points.push(point);
                screen_points.push(screen);
                let end_screen = screen_points.last().copied().unwrap_or(start_screen);
                let click_only = distance_between(start_screen, end_screen) < SELECTION_CLICK_MAX_DISTANCE_PX;
                let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                let next = if click_only { BTreeSet::new() } else { self.resolve_area_hits(&self.area_initial_string_set(), start, &points, merge_mode.as_str()) };
                self.commit_selection_from_hits(&next);
                self.clear_preselect();
                self.selection_preview_points.clear();
                self.selection_preview_crossing = false;
            }
            InteractionMode::Pan { .. } => {}
            other => {
                self.interaction = other;
            }
        }
    }

    pub fn render_snapshot(&self) -> RenderSnapshot {
        let mut snapshot = RenderSnapshot::default();
        for node in self.nodes.values() {
            snapshot.nodes.push((node.id, node.center, node.radius));
        }
        if P::HAS_PORTS {
            for handle in self.handles.values() {
                if let Some(node) = self.nodes.get(&handle.node_id) {
                    snapshot.handles.push((handle.id, handle_position(node, handle), handle.radius));
                }
            }
        }
        for edge in self.edges.values() {
            if let Some(curve) = self.edge_curve(edge.id) {
                snapshot.edges.push(curve);
            }
        }
        if let InteractionMode::DrawEdge { anchor_handle, anchor_is_source, fixed_target, cursor, snap_target, .. } = self.interaction {
            snapshot.pending_edge = self.draw_edge_preview_curve(anchor_handle, anchor_is_source, fixed_target, cursor, snap_target);
        } else if let Some(conn) = self.proximity_connection {
            snapshot.pending_edge = self.proximity_preview_curve(conn);
        }
        let _stroke_scene = encode_board_stroke_scene(&snapshot.edges, 2.0);
        let _ = _stroke_scene.path_count();
        snapshot
    }

    pub fn drain_events(&mut self) -> Vec<BoardEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn edge_curve(&self, edge_id: EdgeId) -> Option<CubicBez> {
        let edge = self.edges.get(&edge_id)?;
        let (source_position, target_position, source_node, target_node, source_cap_radius, target_cap_radius) = self.endpoint_wire_nodes(edge)?;
        Some(self.wire_bezier_between(source_position, target_position, source_node, target_node, source_cap_radius, target_cap_radius))
    }

    fn wire_bezier_between(&self, source_point: Point, target_point: Point, source_node: Option<&Node>, target_node: Option<&Node>, source_cap_radius: f64, target_cap_radius: f64) -> CubicBez {
        let chord = normalize_or_zero(target_point - source_point);
        let source_out = source_node.and_then(|node| handle_outward_at_node_rim(source_point, node.center, node.shape, node.radius, node.width, node.height)).filter(|outward| outward.hypot() > f64::EPSILON).unwrap_or(chord);
        let target_out = target_node.and_then(|node| handle_outward_at_node_rim(target_point, node.center, node.shape, node.radius, node.width, node.height)).filter(|outward| outward.hypot() > f64::EPSILON).unwrap_or(-chord);
        let source_wire = if source_node.is_some() { handle_exterior_cap_peak(source_point, source_out, source_cap_radius) } else { source_point };
        let target_wire = if target_node.is_some() { handle_exterior_cap_peak(target_point, target_out, target_cap_radius) } else { target_point };
        compute_edge_bezier_outward(source_wire, target_wire, source_out, target_out)
    }

    fn draw_edge_preview_curve(&self, anchor_handle: HandleId, anchor_is_source: bool, fixed_target: Option<HandleId>, cursor: Point, snap_target: Option<HandleId>) -> Option<CubicBez> {
        if let Some(fixed_tgt) = fixed_target {
            let tgt_h = self.handles.get(&fixed_tgt)?;
            let tgt_node = self.nodes.get(&tgt_h.node_id)?;
            let target_point = handle_position(tgt_node, tgt_h);
            if let Some(snap) = snap_target {
                let snap_h = self.handles.get(&snap)?;
                let snap_node = self.nodes.get(&snap_h.node_id)?;
                let source_point = handle_position(snap_node, snap_h);
                return Some(self.wire_bezier_between(source_point, target_point, Some(snap_node), Some(tgt_node), snap_h.radius, tgt_h.radius));
            }
            return Some(self.wire_bezier_between(cursor, target_point, None, Some(tgt_node), 0.0, tgt_h.radius));
        }
        let anchor = self.handles.get(&anchor_handle)?;
        let anchor_node = self.nodes.get(&anchor.node_id)?;
        let anchor_point = handle_position(anchor_node, anchor);
        if anchor_is_source {
            if let Some(snap) = snap_target {
                let snap_h = self.handles.get(&snap)?;
                let snap_node = self.nodes.get(&snap_h.node_id)?;
                let snap_point = handle_position(snap_node, snap_h);
                return Some(self.wire_bezier_between(anchor_point, snap_point, Some(anchor_node), Some(snap_node), anchor.radius, snap_h.radius));
            }
            Some(self.wire_bezier_between(anchor_point, cursor, Some(anchor_node), None, anchor.radius, 0.0))
        } else if let Some(snap) = snap_target {
            let snap_h = self.handles.get(&snap)?;
            let snap_node = self.nodes.get(&snap_h.node_id)?;
            let snap_point = handle_position(snap_node, snap_h);
            Some(self.wire_bezier_between(snap_point, anchor_point, Some(snap_node), Some(anchor_node), snap_h.radius, anchor.radius))
        } else {
            Some(self.wire_bezier_between(cursor, anchor_point, None, Some(anchor_node), 0.0, anchor.radius))
        }
    }

    fn active_proximity_distance_world(&self) -> f64 {
        self.proximity_distance_override.unwrap_or(self.proximity_distance_world)
    }

    fn proximity_enabled(&self) -> bool {
        self.active_proximity_distance_world() > 0.0
    }

    fn wire_snap_drag_tolerance_world(&self, handle_radius: f64) -> f64 {
        if !self.proximity_enabled() {
            return 0.0;
        }
        self.active_proximity_distance_world() + handle_radius
    }

    fn wire_snap_still_active(&self, handle_id: HandleId, cursor: Point) -> bool {
        if !self.proximity_enabled() {
            return false;
        }
        let Some(handle) = self.handles.get(&handle_id) else {
            return false;
        };
        let Some(node) = self.nodes.get(&handle.node_id) else {
            return false;
        };
        let pos = handle_position(node, handle);
        distance(cursor, pos) <= self.wire_snap_drag_tolerance_world(handle.radius)
    }

    fn nearest_wire_snap_handle(&self, anchor_handle: HandleId, anchor_is_source: bool, fixed_target: Option<HandleId>, reconnecting: Option<EdgeId>, cursor: Point) -> Option<HandleId> {
        if !P::HAS_PORTS || !self.proximity_enabled() {
            return None;
        }
        let mut best: Option<(f64, HandleId)> = None;
        for handle in self.handles.values() {
            let candidate = handle.id;
            if candidate == anchor_handle {
                continue;
            }
            let (source_hid, target_hid) = if let Some(fixed) = fixed_target {
                if anchor_is_source {
                    (candidate, fixed)
                } else {
                    (fixed, candidate)
                }
            } else if anchor_is_source {
                (anchor_handle, candidate)
            } else {
                (candidate, anchor_handle)
            };
            if !self.is_valid_connection(source_hid, target_hid, reconnecting, true) {
                continue;
            }
            let Some(node) = self.nodes.get(&handle.node_id) else {
                continue;
            };
            let pos = handle_position(node, handle);
            let d = distance(cursor, pos);
            let tol = self.wire_snap_drag_tolerance_world(handle.radius);
            if d <= tol && best.as_ref().is_none_or(|(best_d, _)| d < *best_d) {
                best = Some((d, candidate));
            }
        }
        best.map(|(_, id)| id)
    }

    fn endpoint_wire_nodes(&self, edge: &GraphEdge<P::Endpoint>) -> Option<WireEndpoints<'_>> {
        if P::HAS_PORTS {
            let source_handle = self.handles.get(&P::endpoint_as_handle(edge.source)?)?;
            let target_handle = self.handles.get(&P::endpoint_as_handle(edge.target)?)?;
            let source_node = self.nodes.get(&source_handle.node_id)?;
            let target_node = self.nodes.get(&target_handle.node_id)?;
            let source_position = handle_position(source_node, source_handle);
            let target_position = handle_position(target_node, target_handle);
            return Some((source_position, target_position, Some(source_node), Some(target_node), source_handle.radius, target_handle.radius));
        }
        let source_node = self.nodes.get(&P::endpoint_as_u64(edge.source))?;
        let target_node = self.nodes.get(&P::endpoint_as_u64(edge.target))?;
        Some((source_node.center, target_node.center, Some(source_node), Some(target_node), 0.0, 0.0))
    }

    fn remove_handle(&mut self, id: HandleId) {
        self.handles.remove(&id);
        let removed_edges: Vec<EdgeId> = self.edges.values().filter(|edge| P::endpoint_as_u64(edge.source) == id || P::endpoint_as_u64(edge.target) == id).map(|edge| edge.id).collect();
        for edge_id in removed_edges {
            self.edges.remove(&edge_id);
            self.selection.edge_ids.remove(&edge_id);
        }
        self.selection.handle_ids.remove(&id);
    }

    fn apply_pick_with_mode(&mut self, hit: HitObject<P::Endpoint>, mode: &str) {
        match hit {
            HitObject::Node(id) => {
                let current: BTreeSet<String> = self.selection.node_ids.iter().map(|nid| nid.to_string()).collect();
                let next = merge_pick_into_selection(&current, &id.to_string(), mode);
                self.selection.node_ids = next.iter().filter_map(|s| s.parse().ok()).collect();
            }
            HitObject::Endpoint(ep) => {
                let id = P::endpoint_as_u64(ep);
                if P::HAS_PORTS {
                    let current: BTreeSet<String> = self.selection.handle_ids.iter().map(|hid| hid.to_string()).collect();
                    let next = merge_pick_into_selection(&current, &id.to_string(), mode);
                    self.selection.handle_ids = next.iter().filter_map(|s| s.parse().ok()).collect();
                } else {
                    let current: BTreeSet<String> = self.selection.node_ids.iter().map(|nid| nid.to_string()).collect();
                    let next = merge_pick_into_selection(&current, &id.to_string(), mode);
                    self.selection.node_ids = next.iter().filter_map(|s| s.parse().ok()).collect();
                }
            }
            HitObject::Edge(id) => {
                let current: BTreeSet<String> = self.selection.edge_ids.iter().map(|eid| eid.to_string()).collect();
                let next = merge_pick_into_selection(&current, &id.to_string(), mode);
                self.selection.edge_ids = next.iter().filter_map(|s| s.parse().ok()).collect();
            }
        }
        self.clear_preselect();
        self.push_selection_event();
    }

    fn clear_preselect(&mut self) {
        self.preselect = Selection::default();
        self.preselect_removed = Selection::default();
    }

    pub fn sync_selection_screen_overlay(&mut self, start_screen: Point, screen_points: &[Point]) {
        if screen_points.len() < 2 {
            self.selection_preview_points.clear();
            self.selection_preview_crossing = false;
            return;
        }
        self.selection_preview_crossing = !selection_drag_enclosing(self.selection_options.method.as_str(), start_screen, screen_points);
        self.selection_preview_points = selection_screen_overlay_points(self.selection_options.method.as_str(), start_screen, screen_points).unwrap_or_default();
    }

    fn area_initial_string_set(&self) -> BTreeSet<String> {
        let mut set = BTreeSet::new();
        for id in &self.area_initial.node_ids {
            set.insert(id.to_string());
        }
        for id in &self.area_initial.handle_ids {
            set.insert(id.to_string());
        }
        for id in &self.area_initial.edge_ids {
            set.insert(id.to_string());
        }
        set
    }

    fn resolve_area_hits(&self, initial: &BTreeSet<String>, start: Point, points: &[Point], merge_mode: &str) -> BTreeSet<String> {
        let Some((box_, enclosing, ref polygon)) = selection_drag_shape(self.selection_options.method.as_str(), start, points) else {
            return initial.clone();
        };
        let lasso = self.selection_options.method == "lasso";
        let mut hits = BTreeSet::new();
        if self.selection_options.select_nodes {
            for node in self.nodes.values() {
                if selection_contains_node_bounds(node, box_, enclosing, polygon, lasso) {
                    hits.insert(node.id.to_string());
                }
            }
        }
        if self.selection_options.select_handles && P::HAS_PORTS {
            for handle in self.handles.values() {
                if let Some(node) = self.nodes.get(&handle.node_id) {
                    let pos = handle_position(node, handle);
                    if selection_contains_handle_point(pos, handle.radius.max(6.0), box_, enclosing, polygon, lasso) {
                        hits.insert(handle.id.to_string());
                    }
                }
            }
        }
        if self.selection_options.select_edges {
            for edge in self.edges.keys() {
                if let Some(curve) = self.edge_curve(*edge) {
                    if selection_contains_edge_curve(curve, box_, enclosing, polygon, lasso) {
                        hits.insert(edge.to_string());
                    }
                }
            }
        }
        merge_ids_into_selection(initial, &hits, merge_mode)
    }

    fn selection_from_string_set(&self, ids: &BTreeSet<String>) -> Selection {
        let mut selection = Selection::default();
        for id in ids {
            if let Ok(nid) = id.parse::<NodeId>() {
                if self.nodes.contains_key(&nid) {
                    selection.node_ids.insert(nid);
                    continue;
                }
            }
            if let Ok(hid) = id.parse::<HandleId>() {
                if self.handles.contains_key(&hid) {
                    selection.handle_ids.insert(hid);
                    continue;
                }
            }
            if let Ok(eid) = id.parse::<EdgeId>() {
                if self.edges.contains_key(&eid) {
                    selection.edge_ids.insert(eid);
                }
            }
        }
        selection
    }

    fn apply_area_preselect(&mut self, start: Point, points: &[Point], shift: bool, ctrl_or_meta: bool) {
        let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
        let anchor = self.area_initial_string_set();
        let next_ids: Vec<String> = self.resolve_area_hits(&anchor, start, points, merge_mode.as_str()).into_iter().collect();
        let (sorted, removed) = area_preselect_ids(&anchor, &next_ids);
        let next = self.selection_from_string_set(&next_ids.iter().cloned().collect());
        let removed_sel = self.selection_from_string_set(&removed.iter().cloned().collect());
        if self.preselect == next && self.preselect_removed == removed_sel {
            return;
        }
        self.preselect = next;
        self.preselect_removed = removed_sel;
        let _ = sorted;
        self.push_preselect_event();
    }

    fn commit_selection_from_hits(&mut self, hits: &BTreeSet<String>) {
        self.selection = self.selection_from_string_set(hits);
        self.clear_preselect();
        self.push_selection_event();
    }

    fn push_preselect_event(&mut self) {
        self.events.push(BoardEvent::PreselectChanged {
            node_ids: self.preselect.node_ids.iter().copied().collect(),
            handle_ids: self.preselect.handle_ids.iter().copied().collect(),
            edge_ids: self.preselect.edge_ids.iter().copied().collect(),
            removed_node_ids: self.preselect_removed.node_ids.iter().copied().collect(),
            removed_handle_ids: self.preselect_removed.handle_ids.iter().copied().collect(),
            removed_edge_ids: self.preselect_removed.edge_ids.iter().copied().collect(),
        });
    }

    fn update_hover(&mut self, hover: Option<u64>) {
        if self.hover == hover {
            return;
        }
        self.hover = hover;
        self.events.push(BoardEvent::HoverChanged { id: hover });
    }

    fn push_selection_event(&mut self) {
        self.events.push(BoardEvent::SelectionChanged { edge_ids: self.selection.edge_ids.iter().copied().collect(), handle_ids: self.selection.handle_ids.iter().copied().collect(), node_ids: self.selection.node_ids.iter().copied().collect() });
    }

    fn begin_draw_edge_from_handle(&mut self, handle_id: HandleId, cursor: Point) {
        let Some(handle) = self.handles.get(&handle_id).cloned() else {
            self.interaction = InteractionMode::Idle;
            return;
        };
        let incoming = self.incoming_edge_for_handle(handle_id);
        let reconnect_from_target = |edge_id: EdgeId, target_hid: HandleId| -> Option<(HandleId, bool, Option<HandleId>, Option<EdgeId>)> {
            let src = P::endpoint_as_u64(self.edges.get(&edge_id)?.source);
            Some((src, true, Some(target_hid), Some(edge_id)))
        };
        let (anchor_handle, anchor_is_source, fixed_target, reconnecting) = match handle.role {
            HandleRole::Target => incoming.and_then(|e| reconnect_from_target(e, handle_id)).unwrap_or((handle_id, false, None, None)),
            HandleRole::Source => (handle_id, true, None, None),
            HandleRole::Any => incoming.and_then(|e| reconnect_from_target(e, handle_id)).unwrap_or((handle_id, true, None, None)),
        };
        self.interaction = InteractionMode::DrawEdge { anchor_handle, anchor_is_source, fixed_target, cursor, reconnecting, snap_target: None };
    }

    fn incoming_edge_for_handle(&self, handle_id: HandleId) -> Option<EdgeId> {
        self.edges.values().find(|edge| P::endpoint_as_u64(edge.target) == handle_id).map(|edge| edge.id)
    }

    fn displaced_incoming_edge(&self, target_hid: HandleId, reconnecting: Option<EdgeId>) -> Option<EdgeId> {
        self.edges.values().find(|edge| Some(edge.id) != reconnecting && P::endpoint_as_u64(edge.target) == target_hid).map(|edge| edge.id)
    }

    /// 🔌️ Every connection rule EXCEPT the declared port types — role, self-node, duplicate edge,
    /// single-incoming and acyclicity. Split out so a drag can tell "this port could never take a
    /// wire" apart from "this port takes wires, but not one carrying THIS value", which is the only
    /// refusal a user can act on and therefore the only one worth naming on screen.
    fn is_structurally_valid_connection(&self, source_hid: HandleId, target_hid: HandleId, reconnecting: Option<EdgeId>, allow_target_replace: bool) -> bool {
        if source_hid == target_hid {
            return false;
        }
        let Some(source_handle) = self.handles.get(&source_hid) else {
            return false;
        };
        let Some(target_handle) = self.handles.get(&target_hid) else {
            return false;
        };
        if source_handle.node_id == target_handle.node_id {
            return false;
        }
        if !matches!(source_handle.role, HandleRole::Source | HandleRole::Any) {
            return false;
        }
        if !matches!(target_handle.role, HandleRole::Target | HandleRole::Any) {
            return false;
        }
        if self.edges.values().any(|e| Some(e.id) != reconnecting && P::endpoint_as_u64(e.source) == source_hid && P::endpoint_as_u64(e.target) == target_hid) {
            return false;
        }
        if !allow_target_replace && self.edges.values().any(|e| Some(e.id) != reconnecting && P::endpoint_as_u64(e.target) == target_hid) {
            return false;
        }
        if self.enforce_acyclic {
            let src_node = source_handle.node_id;
            let tgt_node = target_handle.node_id;
            let excluding = reconnecting.or_else(|| if allow_target_replace { self.displaced_incoming_edge(target_hid, reconnecting) } else { None });
            if self.would_create_cycle_between_nodes(src_node, tgt_node, excluding) {
                return false;
            }
        }
        true
    }

    fn is_valid_connection(&self, source_hid: HandleId, target_hid: HandleId, reconnecting: Option<EdgeId>, allow_target_replace: bool) -> bool {
        self.is_structurally_valid_connection(source_hid, target_hid, reconnecting, allow_target_replace) && self.handles_type_compatible(source_hid, target_hid)
    }

    /// 🚫️ The port pair the live wire drag is hovering that ONLY the port-type rule refuses — the
    /// evidence a surface paints a snap target as not-droppable with, and names in the user's own
    /// language. `None` while the drag is over open canvas, over a port it can legally take, or over
    /// one refused for a structural reason a hint could not help with.
    pub fn wire_drag_type_refusal(&self) -> Option<(HandleId, HandleId)> {
        let InteractionMode::DrawEdge { anchor_handle, anchor_is_source, fixed_target, cursor, reconnecting, .. } = self.interaction else {
            return None;
        };
        self.nearest_type_refused_pair(anchor_handle, anchor_is_source, fixed_target, reconnecting, cursor)
    }

    fn nearest_type_refused_pair(&self, anchor_handle: HandleId, anchor_is_source: bool, fixed_target: Option<HandleId>, reconnecting: Option<EdgeId>, cursor: Point) -> Option<(HandleId, HandleId)> {
        if !P::HAS_PORTS {
            return None;
        }
        let mut best: Option<(f64, HandleId, HandleId)> = None;
        for handle in self.handles.values() {
            let candidate = handle.id;
            if candidate == anchor_handle {
                continue;
            }
            let (source_hid, target_hid) = if let Some(fixed) = fixed_target {
                if anchor_is_source {
                    (candidate, fixed)
                } else {
                    (fixed, candidate)
                }
            } else if anchor_is_source {
                (anchor_handle, candidate)
            } else {
                (candidate, anchor_handle)
            };
            if !self.is_structurally_valid_connection(source_hid, target_hid, reconnecting, true) || self.handles_type_compatible(source_hid, target_hid) {
                continue;
            }
            let Some(node) = self.nodes.get(&handle.node_id) else {
                continue;
            };
            let pos = handle_position(node, handle);
            let d = distance(cursor, pos);
            let tol = self.active_proximity_distance_world().max(DRAW_EDGE_TYPE_REFUSAL_WORLD_TOLERANCE) + handle.radius;
            if d <= tol && best.as_ref().is_none_or(|(best_d, _, _)| d < *best_d) {
                best = Some((d, source_hid, target_hid));
            }
        }
        best.map(|(_, source, target)| (source, target))
    }

    fn try_connect_handles(&mut self, source_hid: HandleId, target_hid: HandleId, reconnecting: Option<EdgeId>) -> bool {
        if !self.is_valid_connection(source_hid, target_hid, reconnecting, true) {
            return false;
        }
        let new_id = reconnecting.unwrap_or_else(|| {
            let id = self.next_edge_id;
            self.next_edge_id += 1;
            id
        });
        if let Some(old_id) = reconnecting.or_else(|| self.incoming_edge_for_handle(target_hid)) {
            self.remove_edge(old_id);
        }
        let (Some(src_ep), Some(tgt_ep)) = (P::try_handle_endpoint(source_hid), P::try_handle_endpoint(target_hid)) else {
            return false;
        };
        self.create_edge(new_id, src_ep, tgt_ep);
        self.events.push(BoardEvent::EdgeConnected { id: new_id, source: source_hid, target: target_hid });
        true
    }

    fn connection_pairs_for_handles(&self, a: &Handle, b: &Handle) -> Vec<(HandleId, HandleId)> {
        let mut pairs = Vec::new();
        match (a.role, b.role) {
            (HandleRole::Source, HandleRole::Target) => pairs.push((a.id, b.id)),
            (HandleRole::Target, HandleRole::Source) => pairs.push((b.id, a.id)),
            (HandleRole::Source, HandleRole::Any) => pairs.push((a.id, b.id)),
            (HandleRole::Any, HandleRole::Source) => pairs.push((b.id, a.id)),
            (HandleRole::Target, HandleRole::Any) => pairs.push((b.id, a.id)),
            (HandleRole::Any, HandleRole::Target) => pairs.push((a.id, b.id)),
            (HandleRole::Any, HandleRole::Any) => {
                pairs.push((a.id, b.id));
                pairs.push((b.id, a.id));
            }
            _ => {}
        }
        pairs
    }

    pub fn handle_has_incident_edge(&self, handle_id: HandleId) -> bool {
        self.edges.values().any(|edge| P::endpoint_as_u64(edge.source) == handle_id || P::endpoint_as_u64(edge.target) == handle_id)
    }

    fn update_node_drag_proximity(&mut self, dragged_ids: &[NodeId]) {
        self.proximity_connection = None;
        if !self.proximity_enabled() || !P::HAS_PORTS {
            return;
        }
        let dragged: BTreeSet<NodeId> = dragged_ids.iter().copied().collect();
        let mut best: Option<(f64, ProximityConnection)> = None;
        for dragged_handle in self.handles.values().filter(|h| dragged.contains(&h.node_id)) {
            if self.handle_has_incident_edge(dragged_handle.id) {
                continue;
            }
            for other_handle in self.handles.values().filter(|h| !dragged.contains(&h.node_id)) {
                for (source_hid, target_hid) in self.connection_pairs_for_handles(dragged_handle, other_handle) {
                    if !self.is_valid_connection(source_hid, target_hid, None, false) {
                        continue;
                    }
                    let Some(source_handle) = self.handles.get(&source_hid) else {
                        continue;
                    };
                    let Some(target_handle) = self.handles.get(&target_hid) else {
                        continue;
                    };
                    let Some(source_node) = self.nodes.get(&source_handle.node_id) else {
                        continue;
                    };
                    let Some(target_node) = self.nodes.get(&target_handle.node_id) else {
                        continue;
                    };
                    let src_pos = handle_position(source_node, source_handle);
                    let tgt_pos = handle_position(target_node, target_handle);
                    let d = distance(src_pos, tgt_pos);
                    let tol = self.active_proximity_distance_world() + source_handle.radius + target_handle.radius;
                    if d <= tol && best.as_ref().is_none_or(|(best_d, _)| d < *best_d) {
                        best = Some((d, ProximityConnection { source: source_hid, target: target_hid }));
                    }
                }
            }
        }
        if let Some((_, conn)) = best {
            self.proximity_connection = Some(conn);
        }
    }

    fn proximity_preview_curve(&self, conn: ProximityConnection) -> Option<CubicBez> {
        let source_handle = self.handles.get(&conn.source)?;
        let target_handle = self.handles.get(&conn.target)?;
        let source_node = self.nodes.get(&source_handle.node_id)?;
        let target_node = self.nodes.get(&target_handle.node_id)?;
        let source_position = handle_position(source_node, source_handle);
        let target_position = handle_position(target_node, target_handle);
        Some(self.wire_bezier_between(source_position, target_position, Some(source_node), Some(target_node), source_handle.radius, target_handle.radius))
    }

    fn would_create_cycle_between_nodes(&self, source: NodeId, target: NodeId, excluding: Option<EdgeId>) -> bool {
        if source == target {
            return true;
        }
        let mut adj: std::collections::HashMap<NodeId, Vec<NodeId>> = std::collections::HashMap::new();
        for edge in self.edges.values() {
            if excluding == Some(edge.id) {
                continue;
            }
            let Some(src_h) = self.handles.get(&P::endpoint_as_u64(edge.source)) else {
                continue;
            };
            let Some(tgt_h) = self.handles.get(&P::endpoint_as_u64(edge.target)) else {
                continue;
            };
            adj.entry(src_h.node_id).or_default().push(tgt_h.node_id);
        }
        adj.entry(source).or_default().push(target);
        Self::has_path_nodes(&adj, target, source)
    }

    fn has_path_nodes(adj: &std::collections::HashMap<NodeId, Vec<NodeId>>, from: NodeId, to: NodeId) -> bool {
        let mut seen = std::collections::HashSet::new();
        let mut stack = vec![from];
        while let Some(n) = stack.pop() {
            if n == to {
                return true;
            }
            if !seen.insert(n) {
                continue;
            }
            if let Some(next) = adj.get(&n) {
                for m in next {
                    stack.push(*m);
                }
            }
        }
        false
    }

    fn hit_test(&self, point: Point) -> Option<HitObject<P::Endpoint>> {
        if P::HAS_PORTS && self.handle_pointer_picking {
            for handle in self.handles.values().rev() {
                let node = self.nodes.get(&handle.node_id)?;
                if distance(point, handle_position(node, handle)) <= handle.radius + 6.0 {
                    if let Some(ep) = P::try_handle_endpoint(handle.id) {
                        return Some(HitObject::Endpoint(ep));
                    }
                }
            }
        }
        for node in self.nodes.values().rev() {
            if node_contains_point(node, point) {
                return Some(HitObject::Node(node.id));
            }
        }
        for edge in self.edges.values().rev() {
            if let Some(curve) = self.edge_curve(edge.id) {
                if distance_point_to_cubic_bezier(point, curve, 18) <= 8.0 {
                    return Some(HitObject::Edge(edge.id));
                }
            }
        }
        None
    }

    /// 🎯️ Returns every graph entity under a world point for pick disambiguation menus.
    pub fn hit_test_pick_targets(&self, point: Point) -> Vec<GraphPickTarget> {
        let mut out = Vec::new();
        for node in self.nodes.values().rev() {
            if node_contains_point(node, point) {
                out.push(GraphPickTarget { domain: "node".into(), id: node.id, generality: 0 });
            }
        }
        for edge in self.edges.values().rev() {
            if let Some(curve) = self.edge_curve(edge.id) {
                if distance_point_to_cubic_bezier(point, curve, 18) <= 8.0 {
                    out.push(GraphPickTarget { domain: "edge".into(), id: edge.id, generality: 1 });
                }
            }
        }
        if P::HAS_PORTS && self.handle_pointer_picking {
            for handle in self.handles.values().rev() {
                let node = match self.nodes.get(&handle.node_id) {
                    Some(node) => node,
                    None => continue,
                };
                if distance(point, handle_position(node, handle)) <= handle.radius + 6.0 {
                    if let Some(ep) = P::try_handle_endpoint(handle.id) {
                        out.push(GraphPickTarget { domain: "handle".into(), id: P::endpoint_as_u64(ep), generality: 2 });
                    }
                }
            }
        }
        out
    }
}
// #endregion 🔖️Engine

    /// 🕸️ Runs undirected force layout on a board snapshot with node-id edges.
    pub fn apply_force_layout(snapshot: &mut BoardSnapshot, opts: &ForceGraphLayoutOptions, control: &mut LayoutControl<'_>) -> Result<(), LayoutError> {
        apply_force_layout_resolved(snapshot, opts, control, resolve_node_id_endpoint)
    }

/// 🔌️ Resolves visible handle endpoints before the shared node geometry core.
pub fn apply_ported_force_layout(snapshot:&mut BoardSnapshot,opts:&ForceGraphLayoutOptions,control:&mut LayoutControl<'_>)->Result<(),LayoutError>{
 control.advance(snapshot.nodes.len() as u64)?;let mut handles=HashMap::new();for node in &snapshot.nodes {control.advance(1)?;if !board_visible_or_true(&node.visibility()){continue;}for h in node.handles.as_deref().unwrap_or(&[]){control.advance(1)?;if board_visible_or_true(&h.visibility()){handles.insert(h.id.clone(),node.id.clone());}}}
 apply_force_layout_resolved(snapshot,opts,control,|endpoint,ids|{let node=handles.get(endpoint).map(String::as_str).unwrap_or(endpoint);ids.contains_key(node).then(||node.to_owned())})
}

/// 🌳️ Refuses work before any original snapshot coordinates are changed.
pub fn apply_hierarchical_layout(snapshot:&mut BoardSnapshot,opts:&HierarchicalTreeLayoutOptions,control:&mut LayoutControl<'_>)->Result<(),LayoutError>{control.advance(snapshot.nodes.len() as u64)?;let mut next=snapshot.clone();apply_hierarchical_inner(&mut next,opts,control)?;*snapshot=next;Ok(())}

/// 🔗️ Last visible edge wins for shared handles; coincident centers preserve angles.
pub fn snap_edge_handles(snapshot:&mut BoardSnapshot,control:&mut LayoutControl<'_>)->Result<(),LayoutError>{
 if snapshot.schema.as_str()!="board.ports.directed.v1"{return Err(LayoutError::Schema(snapshot.schema.as_str().into()));}
 let mut locations=HashMap::new();let mut shapes=Vec::new();
 for (ni,node) in snapshot.nodes.iter().enumerate(){control.advance(1)?;if !board_visible_or_true(&node.visibility()){shapes.push(None);continue;}
 let shape=match(node.x,node.y,node.shape){(Some(x),Some(y),Some(BoardNodeShape::Rectangle))=>node.width.zip(node.height).map(|(w,h)|(Point::new(x,y),Some((w,h)))),(Some(x),Some(y),_)=>node.radius.map(|_|(Point::new(x,y),None)),_=>None};shapes.push(shape);
 for (hi,handle) in node.handles.as_deref().unwrap_or(&[]).iter().enumerate(){if board_visible_or_true(&handle.visibility()){locations.insert(handle.id.as_str(),(ni,hi));}}
 }
 let mut angles=HashMap::new();for edge in &snapshot.edges {control.advance(1)?;if !board_visible_or_true(&edge.visibility()){continue;}
 let (Some(source),Some(target))=(edge.source.as_deref(),edge.target.as_deref())else{continue;};let(Some(&(a,ha)),Some(&(b,hb)))=(locations.get(source),locations.get(target))else{continue;};
 let (Some((ca,sa)),Some((cb,sb)))=(shapes[a],shapes[b])else{continue;};if geometry::distance_between(ca,cb)<=1e-9{continue;}
 let angle=|center,toward,shape:Option<(f64,f64)>|match shape{Some((w,h))=>semio_framework_graph_drawing::routing::rectangle_handle_angle_toward(center,w,h,toward),None=>semio_framework_graph_drawing::routing::circle_handle_angle_toward(center,toward)};
 angles.insert((a,ha),angle(ca,cb,sa));angles.insert((b,hb),angle(cb,ca,sb));}
 control.advance(angles.len() as u64)?;for ((ni,hi),angle) in angles{snapshot.nodes[ni].handles.as_mut().expect("admitted handle location")[hi].angle=Some(angle);}Ok(())
}

/// 🔀️ Applies declared options atomically to the original admitted snapshot.
pub fn redraw_layout(snapshot:&mut BoardSnapshot,opts:&RedrawLayoutOptions,control:&mut LayoutControl<'_>)->Result<(),LayoutError>{
 let undirected=matches!(snapshot.schema.as_str(),"board.normal.undirected.v1"|"trinity.graph"|"reasoning.wires.identity.snapshot");
 if undirected&&opts.redraw_handles_after{return Err(LayoutError::InvalidOptions("normal undirected redraw does not support redrawHandlesAfter".into()));}
 control.advance(snapshot.nodes.len() as u64)?;let mut next=snapshot.clone();match opts.mode.as_str(){
 "force-graph"=>{let mut force=opts.force_graph.clone().unwrap_or_default();if opts.center_x.is_some(){force.center_x=opts.center_x;}if opts.center_y.is_some(){force.center_y=opts.center_y;}if let Some(seed)=opts.random_seed{force.random_seed=seed;}for id in &opts.locked_node_ids{if !force.locked_node_ids.contains(id){force.locked_node_ids.push(id.clone());}}if undirected{force::apply_force_layout(&mut next,&force,control)?;}else{force::apply_ported_force_layout(&mut next,&force,control)?;}},
 "hierarchical-tree"=>{let mut tree=opts.hierarchical_tree.clone().unwrap_or_default();if opts.center_x.is_some(){tree.center_x=opts.center_x;}if opts.center_y.is_some(){tree.center_y=opts.center_y;}for id in &opts.locked_node_ids{if !tree.locked_node_ids.contains(id){tree.locked_node_ids.push(id.clone());}}apply_hierarchical_layout(&mut next,&tree,control)?;},
 other=>return Err(LayoutError::InvalidOptions(format!("unknown redraw mode: {other}"))),}
 if opts.redraw_handles_after{snap_edge_handles(&mut next,control)?;}*snapshot=next;Ok(())
}
