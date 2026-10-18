//! 🎯️ Source, candidate and preview of the board fill: the fill source, virtual nodes and handles, candidates, the job state, the preview, verdicts, candidate events and the placement record. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug)]
pub struct BoardFillSource {
    pub id: BoardFillText,
    pub node_kind: BoardFillText,
    pub handle_kind: BoardFillText,
    pub wire_kind: BoardFillText,
    pub edge_kind: BoardFillText,
    pub slot: [f64; 2],
    pub(super) weight: f64,
    pub owner: BoardFillSourceOwner,
    pub rejected: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BoardFillSourceCaptureStage {
    Id,
    NodeKind,
    HandleKind,
    WireKind,
    EdgeKind,
    Slot,
    Weight,
    Publish,
}

pub(super) struct BoardFillSourceCapture {
    pub value: BoardFillSource,
    pub stage: BoardFillSourceCaptureStage,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum BoardFillSourceOwner {
    Host(usize),
    Virtual(usize),
}

#[derive(Clone, Copy, Debug)]
pub(super) struct BoardFillVirtualNode {
    pub id: BoardFillText,
    pub bounds: [f64; 4],
}

#[derive(Clone, Copy, Debug)]
pub struct BoardFillVirtualHandle {
    pub id: BoardFillText,
    pub node_kind: BoardFillText,
    pub handle_kind: BoardFillText,
    pub wire_kind: BoardFillText,
    pub edge_kind: BoardFillText,
    pub slot: [f64; 2],
    pub(super) weight: f64,
    pub(super) connected: bool,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct BoardFillCandidate {
    pub(super) kind_index: usize,
    pub target_handle_index: usize,
    pub(super) weight: f64,
    pub rejected: bool,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct BoardFillCandidatePreview {
    pub source_id: BoardFillText,
    pub(super) kind_index: usize,
    pub target_handle_index: usize,
    pub x: f64,
    pub y: f64,
    pub bounds: [f64; 4],
}

pub struct BoardFillJobState {
    pub snapshot: BoardFillSnapshot,
    pub stage: BoardFillStage,
    pub(super) max_count: u32,
    pub(super) rng_state: u64,
    pub sources: BoardFillFixedPages<BoardFillSource, { BOARD_FILL_SOURCE_CAPACITY.div_ceil(BOARD_FILL_PAGE_ITEMS) }>,
    pub(super) source_scan_cursor: usize,
    pub(super) source_capture: Option<BoardFillSourceCapture>,
    pub(super) target_selection_cursor: usize,
    pub(super) target_best: Option<(f64, usize)>,
    pub(super) current_target: Option<usize>,
    pub candidates: BoardFillFixedPages<BoardFillCandidate, { BOARD_FILL_CANDIDATE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>,
    pub(super) kind_cursor: usize,
    pub(super) template_cursor: usize,
    pub(super) compatibility_candidate: Option<BoardFillCandidate>,
    pub(super) compatibility_cursor: usize,
    pub(super) compatibility_matched: bool,
    pub(super) candidate_selection_cursor: usize,
    pub(super) candidate_best: Option<(f64, usize)>,
    pub(super) current_candidate: Option<usize>,
    pub(super) current_preview: Option<BoardFillCandidatePreview>,
    pub(super) host_collision_cursor: usize,
    pub(super) virtual_collision_cursor: usize,
    pub(super) virtual_nodes: BoardFillFixedPages<BoardFillVirtualNode, { BOARD_FILL_PLACEMENT_CAPACITY.div_ceil(BOARD_FILL_PAGE_ITEMS) }>,
    pub virtual_handles: BoardFillFixedPages<BoardFillVirtualHandle, { (BOARD_FILL_PLACEMENT_CAPACITY * BOARD_FILL_KIND_HANDLE_CAPACITY).div_ceil(BOARD_FILL_PAGE_ITEMS) }>,
    pub(super) pending_placement: Option<BoardFillPlacement>,
    pub(super) accept_pending_virtual_node: Option<BoardFillVirtualNode>,
    pub(super) accept_handle_template: Option<BoardFillTemplateSnapshot>,
    pub(super) accept_handle_id: Option<BoardFillText>,
    pub(super) accept_handle_pending_virtual: Option<BoardFillVirtualHandle>,
    pub(super) accept_handle_pending_placement: Option<BoardFillPlacementHandle>,
    pub(super) accept_handle_cursor: usize,
    pub(super) accepted_count: u32,
    pub(super) next_serial: u64,
    pub(super) stalled: bool,
    pub(super) rejection: Option<BoardFillText>,
    pub(super) search_count: u64,
    pub(super) preview_sequence: u64,
    pub candidate_event: Option<BoardFillCandidateEvent>,
}

/// 📡️ Latest bounded fill search projection, separate from authoritative placements.
#[derive(Clone, Copy, Debug)]
pub struct BoardFillPreview {
    pub sequence: u64,
    pub generation: u64,
    pub stage: BoardFillStage,
    pub accepted_count: u32,
    pub target_handle_id: Option<BoardFillText>,
    pub candidate_node_kind_id: Option<BoardFillText>,
    pub host_collision_cursor: usize,
    pub virtual_collision_cursor: usize,
    pub tested_collision_id: Option<BoardFillText>,
    pub rejection: Option<BoardFillText>,
    pub search_count: u64,
}

/// ⚖️ What one fill search transition decided about one candidate placement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillCandidateVerdict {
    Testing,
    HostCollision,
    VirtualCollision,
    PortIncompatible,
    RuleIncompatible,
    Fits,
}

/// 🔎️ One candidate verdict with the candidate's kind index (capture order), slot center and axis-aligned
/// collision footprint `[min_x, min_y, max_x, max_y]`, taken once per transition by
/// [`BoardFillJob::take_candidate_event`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoardFillCandidateEvent {
    pub verdict: BoardFillCandidateVerdict,
    pub kind_index: usize,
    pub position: [f64; 2],
    pub bounds: [f64; 4],
}

#[derive(Clone, Copy, Debug)]
pub(super) struct BoardFillPlacementHandle {
    pub id: BoardFillText,
    pub(super) template: BoardFillTemplateSnapshot,
}

/// 🧩️ Typed placement transferred to the owning domain's document applier without a JSON callback.
pub struct BoardFillPlacement {
    pub node_id: BoardFillText,
    pub edge_id: BoardFillText,
    pub edge_kind: BoardFillText,
    pub node_kind: BoardFillText,
    pub source_handle_id: BoardFillText,
    pub target_handle_id: BoardFillText,
    pub target_handle_index: usize,
    pub x: f64,
    pub y: f64,
    pub shape: &'static str,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
    pub icon_kind: Option<BoardFillText>,
    pub handles: BoardFillFixedPages<BoardFillPlacementHandle, { BOARD_FILL_KIND_HANDLE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>,
}
