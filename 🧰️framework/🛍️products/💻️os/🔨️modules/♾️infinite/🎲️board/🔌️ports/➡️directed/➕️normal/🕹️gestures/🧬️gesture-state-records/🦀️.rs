//! 🧬️ Gesture state records of the board gestures: the brush, gestures, commit and closing state, the close phase, delete operation, removed entity, property container and retirement records and the pointer commit operation. See [`super`].

use super::*;

//#endregion 🎯️TargetRegions

/// 🧩️ BoardHost brush component: brush candidates, previews and suggestion state.
pub struct BoardBrushState {
    pub suggestion_offset: f64,
    pub brush_node_size: f64,
    pub brush_slot_source_id: Option<String>,
    pub brush_candidates: BrushCandidatePage,
    pub brush_candidate_index: usize,
    pub brush_preview: Option<BrushPreviewSnapshot>,
    pub drop_preview: Option<DropPreviewSnapshot>,
    pub brush_candidates_emit_key: Option<String>,
    pub brush_preview_emit_key: Option<String>,
    pub brush_placement_serial: u64,
    pub brush_node_kind_weights: HashMap<String, f64>,
    pub brush_handle_kind_weights: HashMap<String, f64>,
    /// ⌥️ Alt held while brushing — enables suggestion offset and commit-on-leave.
    pub brush_alt_pressed: bool,
    /// ✨️ Suggestions menu opened a slot outside brush utility — use suggestion offset and highlight source handle.
    pub brush_slot_suggestions_active: bool,
}

/// 🧩️ BoardHost gestures component: transform, region and link gesture state.
pub struct BoardGesturesState {
    /// Screen-space polyline preview (CSS px) while dragging a handle link before drop.
    pub link_screen_preview: Option<Vec<Point>>,
    /// 🕹️ Which selection-gumball handles the select utility offers (`setTransformGumballFlag`).
    pub transform_flags: TransformGumballFlags,
    /// 🔄️ The live rotate-ring gesture, `None` whenever the ring is merely drawn.
    pub transform_drag: Option<BoardTransformDrag>,
    /// 🚚️ The live region move/resize gesture (`None` when no region is being dragged).
    pub region_drag: Option<BoardRegionDrag>,
    /// 🖍️ The live area-brush rectangle (`None` unless the areaBrush utility is painting).
    pub region_paint: Option<BoardRegionPaint>,
    /// 🖍️ World width/height an area-brush CLICK (no drag) paints, from the utility's own steppers.
    pub area_brush_extent: (f64, f64),
}

/// 🧩️ BoardHost commit component: pending pointer commit and delete operations.
pub struct BoardCommitState {
    pub pending_delete_planning: Option<BoardDeletePlanningOperation>,
    pub pending_delete_operation: Option<BoardDeleteOperation>,
    pub pending_pointer_commit: Option<BoardPointerCommitOperation>,
    pub queued_pointer_commit: Option<BoardPointerPlan>,
    pub cancelled_pointer_plan_retirement: Option<BoardPointerPlanRetirement>,
    pub pointer_publication: Option<BoardPointerPublication>,
}

/// 🧩️ BoardHost closing component: close and opaque scene retirement state.
pub struct BoardClosingState {
    pub opaque_scene_retirement: Cell<Option<OpaqueSceneRetirementToken>>,
    pub opaque_scene_fault: Cell<bool>,
    pub close_phase: BoardHostClosePhase,
    pub close_entity_retirement: Option<BoardEntityRetirement>,
    pub close_strings: [Option<String>; 16],
    pub close_string_len: u8,
    pub close_node_handles: Option<Vec<NodeKindHandleTemplate>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardHostClosePhase {
    Events,
    Pointer,
    WorldScene,
    Icons,
    Nodes,
    Handles,
    Edges,
    Wires,
    Regions,
    Selection,
    Preselect,
    PreselectRemoved,
    SelectionExit,
    Highlighted,
    Interaction,
    HandleKinds,
    WireKinds,
    NodeKinds,
    EdgeKinds,
    EdgeTips,
    LinkRules,
    Previews,
    Strings,
    Weights,
    Done,
}

pub(crate) struct BoardDeleteOperation {
    pub plan: BoardDeletePlan,
    pub select_event: Option<BoardOwnedEvent>,
    pub mutation_cursor: u16,
    pub publication_cursor: u16,
    pub remaining_claimed_items: u16,
    pub remaining_claimed_bytes: usize,
    pub claimed: bool,
    pub retiring_entity: Option<BoardEntityRetirement>,
    pub cancelling: bool,
}

pub enum BoardRemovedEntity {
    Node(NodeData),
    Edge(EdgeData),
    Handle(HandleData),
    Wire(WireData),
}

pub(super) enum BoardPropertyContainer {
    Array(Vec<semio_framework_graph_manifest::PropertyValue>),
    Object(semio_framework_graph_manifest::PropertyBag),
}

pub(super) struct BoardPropertyRetirement {
    pub current: Option<semio_framework_graph_manifest::PropertyValue>,
    pub stack: Box<[Option<BoardPropertyContainer>; BOARD_POINTER_ITEM_CAPACITY]>,
    pub depth: u16,
    pub faulted: bool,
}

pub struct BoardEntityRetirement {
    pub entity: Option<BoardRemovedEntity>,
    pub strings: Box<[Option<String>; 16]>,
    pub(super) string_len: u8,
    pub properties: Option<BoardPropertyRetirement>,
}

pub struct BoardPointerPublication {
    pub bytes: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
    pub len: u16,
}

pub struct BoardPointerCommitOperation {
    pub plan: BoardPointerPlan,
    pub phase: u8,
    pub cursor: u16,
    pub changed: bool,
    pub cancelling: bool,
    pub faulted: bool,
    pub scan_after: Option<String>,
    pub points: Vec<Point>,
    pub screen_points: Vec<Point>,
    pub overlay_points: Vec<Point>,
    pub brush_candidates: BrushCandidatePage,
    pub retiring_ids: BTreeSet<String>,
    pub retiring_points: Vec<Point>,
    pub retiring_screen_points: Vec<Point>,
    pub retiring_overlay_points: Vec<Point>,
    pub retiring_signature_a: Vec<String>,
    pub retiring_signature_b: Vec<String>,
    pub retiring_signature_c: Vec<String>,
    pub retiring_gestures: [Option<String>; 2],
}
