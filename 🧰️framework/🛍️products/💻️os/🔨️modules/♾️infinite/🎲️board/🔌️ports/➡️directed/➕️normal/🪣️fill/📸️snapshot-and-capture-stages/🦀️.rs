//! 📸️ Snapshot and capture stages of the board fill: the node, handle, template, kind and rule snapshots with their capture stages, the fill snapshot, its capture and ingress records and the capture step. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillShape {
    Circle,
    Rectangle,
}

#[derive(Clone, Copy, Debug)]
pub struct BoardFillNodeSnapshot {
    pub id: BoardFillText,
    pub bounds: [f64; 4],
}

pub struct BoardFillNodeCapture {
    pub id: BoardFillText,
    pub bounds: Option<[f64; 4]>,
}

#[derive(Clone, Copy, Debug)]
pub struct BoardFillHandleSnapshot {
    pub id: BoardFillText,
    pub node_kind: BoardFillText,
    pub handle_kind: BoardFillText,
    pub wire_kind: BoardFillText,
    pub edge_kind: BoardFillText,
    pub slot: [f64; 2],
    pub visible: bool,
    pub(super) connected: bool,
    pub(super) weight: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillHandleCaptureStage {
    NodeKind,
    HandleKind,
    Slot,
    WireKind,
    EdgeKind,
    Visible,
    Connected,
    Weight,
    Publish,
}

pub struct BoardFillHandleCapture {
    pub handle: BoardFillHandleSnapshot,
    pub(super) edge_key: Option<BoardFillText>,
    pub stage: BoardFillHandleCaptureStage,
}

#[derive(Clone, Copy, Debug)]
pub struct BoardFillTemplateSnapshot {
    pub handle_kind: BoardFillText,
    pub wire_kind: BoardFillText,
    pub edge_kind: BoardFillText,
    pub angle: f64,
    pub radius: Option<f64>,
    pub(super) weight: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillTemplateCaptureStage {
    HandleKind,
    WireKind,
    EdgeKind,
    Angle,
    Radius,
    Weight,
    Publish,
}

pub struct BoardFillTemplateCapture {
    pub value: BoardFillTemplateSnapshot,
    pub stage: BoardFillTemplateCaptureStage,
}

pub struct BoardFillKindSnapshot {
    pub id: BoardFillText,
    pub shape: BoardFillShape,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
    pub icon: Option<BoardFillText>,
    pub(super) weight: f64,
    pub handles: BoardFillFixedPages<BoardFillTemplateSnapshot, { BOARD_FILL_KIND_HANDLE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillKindCaptureStage {
    Icon,
    Shape,
    Radius,
    Width,
    Height,
    Weight,
    Templates,
    Publish,
}

pub struct BoardFillKindCapture {
    pub value: BoardFillKindSnapshot,
    pub stage: BoardFillKindCaptureStage,
    pub(super) template: Option<BoardFillTemplateCapture>,
}

#[derive(Clone, Copy, Debug)]
pub struct BoardFillRuleSnapshot {
    pub source: BoardFillText,
    pub target: BoardFillText,
    pub bidirectional: bool,
    pub specificity: CompatSpecificity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillRuleCaptureStage {
    Target,
    Bidirectional,
    Specificity,
    Publish,
}

pub struct BoardFillRuleCapture {
    pub value: BoardFillRuleSnapshot,
    pub stage: BoardFillRuleCaptureStage,
}

/// 📸️ Fixed-page immutable input transferred after staged board capture.
pub struct BoardFillSnapshot {
        pub(super) original_source: Option<semio_framework_action_bus::OriginalToolSourceBinding>,
pub nodes: BoardFillFixedPages<BoardFillNodeSnapshot, { BOARD_FILL_NODE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>,
    pub handles: BoardFillFixedPages<BoardFillHandleSnapshot, { BOARD_FILL_HANDLE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>,
    pub(super) kinds: BoardFillFixedPages<BoardFillKindSnapshot, { BOARD_FILL_KIND_CAPACITY / BOARD_FILL_PAGE_ITEMS }>,
    pub(super) rules: BoardFillFixedPages<BoardFillRuleSnapshot, { BOARD_FILL_RULE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>,
    pub suggestion_offset: f64,
    pub naming: &'static BoardNaming,
}

impl Drop for BoardFillSnapshot {
    fn drop(&mut self) {
        assert!((self.nodes.terminal_is_empty() && self.handles.terminal_is_empty() && self.kinds.terminal_is_empty() && self.rules.terminal_is_empty()) || std::thread::panicking(), "Board fill snapshot must transfer or close every exact page before Drop");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillCapturePhase {
    Nodes,
    Handles,
    Kinds,
    Rules,
    Complete,
    Closing,
}

/// 📷️ Retained UI capture cursor; one host node, handle, kind scalar, template, or rule per call.
pub struct BoardFillSnapshotCapture {
    pub snapshot: Option<BoardFillSnapshot>,
    pub phase: BoardFillCapturePhase,
    pub(super) last_key: Option<BoardFillText>,
    pub node: Option<BoardFillNodeCapture>,
    pub handle: Option<BoardFillHandleCapture>,
    pub kind: Option<BoardFillKindCapture>,
    pub(super) kind_handle_cursor: usize,
    pub rule: Option<BoardFillRuleCapture>,
    pub(super) rule_cursor: usize,
    pub closing: bool,
}

/// 📥️ Worker-owned fixed-page ingress for immutable artifact snapshot leases.
pub struct BoardFillSnapshotIngress {
    pub snapshot: Option<BoardFillSnapshot>,
    pub node: Option<BoardFillNodeSnapshot>,
    pub handle: Option<BoardFillHandleSnapshot>,
    pub kind: Option<BoardFillKindSnapshot>,
    pub(super) template: Option<BoardFillTemplateSnapshot>,
    pub rule: Option<BoardFillRuleSnapshot>,
    pub closing: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillIngressHandleText {
    Id,
    NodeKind,
    HandleKind,
    WireKind,
    EdgeKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillIngressKindText {
    Id,
    Icon,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillIngressTemplateText {
    HandleKind,
    WireKind,
    EdgeKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillIngressRuleText {
    Source,
    Target,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillCaptureStep {
    Pending,
    Complete,
    Fault(BoardFillCaptureFault),
}
