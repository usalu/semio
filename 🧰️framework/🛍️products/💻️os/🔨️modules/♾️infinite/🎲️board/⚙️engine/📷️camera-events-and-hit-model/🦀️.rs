//! 📷️ Camera, events and hit model of the board engine: the camera, board events, render snapshot, hit objects, pick targets, interaction modes, handle geometry and the port model trait. See [`super`].

use super::*;

// #region 🔖️HandleDescriptor
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
pub(super) enum HitObject<E> {
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
pub(super) struct ProximityConnection {
    pub source: HandleId,
    pub target: HandleId,
}

/// 🔗️ Resolved wire endpoints for an edge: source/target world position, their owning nodes (if any), and cap radii.
pub(super) type WireEndpoints<'a> = (Point, Point, Option<&'a Node>, Option<&'a Node>, f64, f64);

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

pub(super) fn node_contains_point(node: &Node, point: Point) -> bool {
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
