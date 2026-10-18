//! 🧬️ Scene types and delete records of the board scene: the fill text bytes, the fill capture fault, the scene and style state, the wheel plan, the pointer plan fault and the delete and property audit records. See [`super`].

use super::*;

pub const BOARD_FILL_TEXT_BYTES: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillCaptureFault {
    TextCapacity,
    NodeCapacity,
    HandleCapacity,
    KindCapacity,
    KindHandleCapacity,
    RuleCapacity,
    StaleNode,
    StaleHandle,
    StaleKind,
    StaleRule,
    GenerationExhausted,
}

/// 🧩️ BoardHost scene component: entities, camera, viewport and the cached world content of one board.
pub struct BoardSceneState {
    pub camera: Camera,
    pub nodes: BTreeMap<String, NodeData>,
    pub handles: BTreeMap<String, HandleData>,
    pub edges: BTreeMap<String, EdgeData>,
    pub wires: BTreeMap<String, WireData>,
    /// 🎯️ Fill-constraining rectangles, keyed by document id. Painted beneath every entity,
    /// hit-tested after all of them, and counted against the descriptor census like any other entity.
    pub regions: BTreeMap<String, RegionData>,
    pub width: u32,
    pub height: u32,
    pub dpr: f64,
    /// 🧿️ Bumped when drawable content changes (not camera); keys {@link BoardHost.world_content_cache}.
    pub content_scene_generation: u64,
    /// 🎨️ World-space node icons reused across pan/zoom. Keyed by generation and LOD; a pending
    /// retirement keeps the previous bake and the draw shifts each span onto the live node center.
    pub world_content_cache: RefCell<Option<WorldContentCache>>,
    pub interaction_revision: u64,
}

/// 🧩️ BoardHost style component: style catalogs, themes, LOD and grid policy.
pub struct BoardStyleState {
    /// Catalog keyed by `handle_kind` id (see `set_board_kind_catalogs_from_json`).
    pub handle_kinds: BTreeMap<String, HandleKindDef>,
    pub wire_kinds: BTreeMap<String, WireKindDef>,
    pub node_kinds: BTreeMap<String, NodeKindDef>,
    pub edge_kinds: BTreeMap<String, EdgeKindDef>,
    /// 🔺️ Registry of edge tip shapes keyed by catalog id (built-ins seeded at init).
    pub edge_tips: BTreeMap<String, EdgeTipDef>,
    /// 🔗️ Kind-compatibility rules for link gestures; empty = unrestricted.
    pub link_compat_rules: Vec<LinkCompatRule>,
    pub world_raster_tiling: String,
    pub canvas_theme: CanvasPalette,
    /// 👁️ When false, the LOD world grid is not stroked at all (snapping is unaffected — that is `grid_snap_enabled`).
    pub grid_visible: bool,
    /// 📐️ Positive multiplier for LOD world grid steps (`10` / `5` / `1` base world units per band).
    pub grid_factor: f64,
    /// 🧲️ When true, node drags snap to the finest visible LOD grid (step scales with `grid_factor`).
    pub grid_snap_enabled: bool,
    pub preserve_original_element_style: bool,
    /// 📶️ When true (default), camera zoom selects draw LOD; when false, optional `forced_draw_lod` pins the tier when set.
    pub automatic_lod: bool,
    pub forced_draw_lod: Option<BoardDrawLod>,
    pub icon_paint_cache: IconPaintCache,
    /// 🔍️ True while the wheel zoom gesture is active (skip grid + per-tile rebuild hot paths).
    pub wheel_zoom_active: bool,
    /// 📶️ LOD tier pinned for the active wheel gesture so pan/zoom does not rebuild {@link BoardHost.world_content_cache} on every band crossing.
    pub wheel_zoom_render_lod: Option<BoardDrawLod>,
    /// 📛️ Placement naming the owning domain installed ([`BoardHost::set_naming`]); [`NEUTRAL_BOARD_NAMING`] until then.
    pub naming: &'static BoardNaming,
    /// 📨️ Brush prefixes received as scene data ([`BoardHost::set_placement_naming_json`]); they override `naming`'s brush prefixes.
    pub placement_naming: Option<BoardPlacementNaming>,
    pub port_mode: GraphPortMode,
}

#[derive(Clone, Debug)]
pub struct BoardWheelPlan {
    pub revision: u64,
    pub expected: Camera,
    pub next: Camera,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardPointerPlanFault {
    ItemCredits,
    ByteCredits,
    Unsupported,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct BoardPointerDelta {
    pub id: BoardPointerSpan,
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardDeleteKind {
    Edge,
    Node,
    Handle,
    Wire,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct BoardDeleteEntry {
    pub kind: BoardDeleteKind,
    pub id: BoardPointerSpan,
}

pub struct BoardDeletePlan {
    pub revision: u64,
    pub bytes: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
    pub byte_len: u16,
    pub entries: Box<[Option<BoardDeleteEntry>; BOARD_POINTER_ITEM_CAPACITY]>,
    pub len: u16,
    pub(super) property_nodes: u16,
    pub(super) property_bytes: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardDeletePlanningPhase {
    SelectedEdges,
    Nodes,
    DiscoverNode,
    Handles,
    Wires,
    Edges,
    Handle,
    Node,
    SelectionEvent,
    Finish,
}

pub struct BoardDeletePlanningOperation {
    pub plan: BoardDeletePlan,
    pub phase: BoardDeletePlanningPhase,
    pub scan_after: Option<String>,
    pub node_after: Option<String>,
    pub handle_after: Option<String>,
    pub relation_after: Option<String>,
    pub node_id: Option<String>,
    pub handle_id: Option<String>,
    pub node_relevant: bool,
    pub property_audit: Option<BoardPropertyAudit>,
    pub select_builder: BoardPayloadBuilder,
    pub select_first: bool,
    pub fault: Option<BoardEventFault>,
    pub cancelling: bool,
}

pub(super) enum BoardPropertyAuditFrame {
    Array { values: Vec<semio_framework_graph_manifest::PropertyValue>, index: usize, pending: Option<usize> },
    Object { values: semio_framework_graph_manifest::PropertyBag, after: Option<String>, pending: Option<String> },
}

pub(crate) struct BoardPropertyAudit {
    pub kind: BoardDeleteKind,
    pub id: String,
    pub current: Option<semio_framework_graph_manifest::PropertyValue>,
    pub completed: Option<semio_framework_graph_manifest::PropertyValue>,
    pub stack: Box<[Option<BoardPropertyAuditFrame>; BOARD_POINTER_ITEM_CAPACITY]>,
    pub depth: u16,
    pub nodes: usize,
    pub bytes: usize,
    pub fault: Option<BoardEventFault>,
}
