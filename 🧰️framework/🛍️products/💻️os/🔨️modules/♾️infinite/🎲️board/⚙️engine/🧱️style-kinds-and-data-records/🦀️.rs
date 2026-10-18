//! 🧱️ Style kinds and data records of the board engine: the port mode, element style kinds, node, wire, edge and kind definitions, selection options, gesture stage and interaction. See [`super`].

use super::*;

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
