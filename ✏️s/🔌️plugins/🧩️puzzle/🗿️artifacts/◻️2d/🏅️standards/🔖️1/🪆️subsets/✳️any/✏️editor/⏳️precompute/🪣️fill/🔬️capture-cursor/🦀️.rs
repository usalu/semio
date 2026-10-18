//! 🔬️ Capture cursor: the artifact capture stages and fields and the fill capture cursor with its fault codes. See [`super`].

use super::*;

//#region 🔬️CaptureCursor
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ArtifactFillCaptureStage {
    Nodes,
    Handles,
    Kinds,
    Rules,
    Complete,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ArtifactNodeCaptureField {
    Begin,
    Id,
    X,
    Y,
    Scale,
    Shape,
    ExtentX,
    ExtentY,
    Bound0,
    Bound1,
    Bound2,
    Bound3,
    Publish,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ArtifactHandleCaptureField {
    Begin,
    Id,
    ScanEdges,
    NodeKind,
    HandleKind,
    WireKind,
    EdgeKind,
    X,
    Y,
    Angle,
    Shape,
    ExtentX,
    ExtentY,
    Radius,
    NodeVisible,
    HandleVisible,
    Visible,
    Connected,
    SlotX,
    SlotY,
    Weight,
    Publish,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ArtifactKindCaptureField {
    Begin,
    Id,
    Shape,
    Scale,
    Radius,
    Width,
    Height,
    IconBegin,
    Icon,
    Weight,
    TemplateBegin,
    TemplateHandleKind,
    TemplateWireKind,
    TemplateEdgeKind,
    TemplateAngle,
    TemplateRadius,
    TemplateWeight,
    TemplatePublish,
    Publish,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ArtifactRuleCaptureField {
    Begin,
    Source,
    Target,
    Bidirectional,
    Specificity,
    Publish,
}

pub(super) struct ArtifactFillCaptureCursor {
    pub(super) stage: ArtifactFillCaptureStage,
    pub(super) node: usize,
    pub(super) node_field: ArtifactNodeCaptureField,
    pub(super) node_x: f64,
    pub(super) node_y: f64,
    pub(super) node_scale: f64,
    pub(super) node_rectangle: bool,
    pub(super) node_extent_x: f64,
    pub(super) node_extent_y: f64,
    pub(super) handle_node: usize,
    pub(super) handle: usize,
    pub(super) handle_field: ArtifactHandleCaptureField,
    pub(super) handle_edge: usize,
    pub(super) handle_connected: bool,
    pub(super) handle_x: f64,
    pub(super) handle_y: f64,
    pub(super) handle_angle: f64,
    pub(super) handle_rectangle: bool,
    pub(super) handle_extent_x: f64,
    pub(super) handle_extent_y: f64,
    pub(super) handle_radius: f64,
    pub(super) handle_node_visible: bool,
    pub(super) handle_visible: bool,
    pub(super) kind: usize,
    pub(super) template: usize,
    pub(super) kind_field: ArtifactKindCaptureField,
    pub(super) kind_size: f64,
    pub(super) rule: usize,
    pub(super) rule_field: ArtifactRuleCaptureField,
    pub(super) byte: usize,
}

impl ArtifactFillCaptureCursor {
    pub(super) fn new() -> Self {
        Self {
            stage: ArtifactFillCaptureStage::Nodes,
            node: 0,
            node_field: ArtifactNodeCaptureField::Begin,
            node_x: 0.0,
            node_y: 0.0,
            node_scale: 1.0,
            node_rectangle: false,
            node_extent_x: 0.0,
            node_extent_y: 0.0,
            handle_node: 0,
            handle: 0,
            handle_field: ArtifactHandleCaptureField::Begin,
            handle_edge: 0,
            handle_connected: false,
            handle_x: 0.0,
            handle_y: 0.0,
            handle_angle: 0.0,
            handle_rectangle: false,
            handle_extent_x: 0.0,
            handle_extent_y: 0.0,
            handle_radius: 0.0,
            handle_node_visible: true,
            handle_visible: true,
            kind: 0,
            template: 0,
            kind_field: ArtifactKindCaptureField::Begin,
            kind_size: 0.0,
            rule: 0,
            rule_field: ArtifactRuleCaptureField::Begin,
            byte: 0,
        }
    }
}

pub(super) fn capture_fault_code(fault: BoardFillCaptureFault) -> &'static str {
    match fault {
        BoardFillCaptureFault::TextCapacity => "puzzle2d-fill-capture-text-capacity",
        BoardFillCaptureFault::NodeCapacity => "puzzle2d-fill-capture-node-capacity",
        BoardFillCaptureFault::HandleCapacity => "puzzle2d-fill-capture-handle-capacity",
        BoardFillCaptureFault::KindCapacity => "puzzle2d-fill-capture-kind-capacity",
        BoardFillCaptureFault::KindHandleCapacity => "puzzle2d-fill-capture-kind-handle-capacity",
        BoardFillCaptureFault::RuleCapacity => "puzzle2d-fill-capture-rule-capacity",
        BoardFillCaptureFault::StaleNode => "puzzle2d-fill-capture-stale-node",
        BoardFillCaptureFault::StaleHandle => "puzzle2d-fill-capture-stale-handle",
        BoardFillCaptureFault::StaleKind => "puzzle2d-fill-capture-stale-kind",
        BoardFillCaptureFault::StaleRule => "puzzle2d-fill-capture-stale-rule",
        BoardFillCaptureFault::GenerationExhausted => "puzzle2d-fill-capture-generation-exhausted",
    }
}
//#endregion 🔬️CaptureCursor
