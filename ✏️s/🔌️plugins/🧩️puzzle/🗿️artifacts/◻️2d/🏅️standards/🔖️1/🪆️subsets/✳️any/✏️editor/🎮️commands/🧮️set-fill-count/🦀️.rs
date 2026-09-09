//! 🖌️ `set-fill-count` command — the whole 2d brush-fill session.
//!
//! 🧵️ The session lives inside one retained tool job: [`Puzzle2dFillSessionWork`] owns the capture
//! ingress, the `BoardFillJob` search and the placement-apply cursor for the lifetime of that job,
//! and publishes exactly one `Emit` with every accepted placement. There is no process-global session registry, no worker
//! pool and no self-dispatched continuation effect: an `ArtifactApp` is a set of associated fns with
//! no live instance, so a session keyed by `app_instance_id` in a `static` slot table could never be
//! reached from a retained work — and a retained work never sees the `ArtifactView` such a key comes
//! from. What survives across dispatches instead is the checkpoint the design already had: the
//! `Puzzle2dFillRuntime` in `Config` (count, seed, accepted count, lifecycle) plus the placements
//! already committed to the document, which is exactly what `brushFillSessionStep` resumes from.

use crate::editor::puzzle2d::config::{Puzzle2dConfig, Puzzle2dFillLifecycle, Puzzle2dFillRuntime, Puzzle2dFillText};
use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::window::{self, Puzzle2dWindowConfig};
use crate::editor::puzzle2d::Puzzle2dPlayApp;
use crate::standards::v1::subsets::any::schema::mutations::text::{Puzzle2dMutation, Puzzle2dPlaySnapshot};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{EditorApp, Emit, Fault};
use serde_json::Value;

/// 🪣️ The eight verbs that drive one fill session.
pub const PUZZLE2D_FILL_SESSION_ACTIONS: &[&str] = &["setFillCount", "brushFillSessionBegin", "brushFillSessionStep", "brushFillSessionClear", "brushFillSessionAdopt", "brushFillSessionCancel", "brushFillSessionRetry", "brushFillSessionDiscard"];

/// 🔎️ Whether `action` opens or resumes a search rather than only moving the runtime lifecycle.
pub fn is_fill_search_action(action: &str) -> bool {
    matches!(action, "setFillCount" | "brushFillSessionBegin" | "brushFillSessionRetry" | "brushFillSessionStep")
}

pub fn is_fill_session_action(action: &str) -> bool {
    PUZZLE2D_FILL_SESSION_ACTIONS.contains(&action)
}

/// 🍰️ Per-`step()` chunk sizes and the fixed chunk ceiling of every fill stage. The declared extent
/// IS the enforced ceiling — a capture or apply that would run past its budget faults, and a search
/// that would is published as a resumable `CheckpointReady` runtime instead of running unbounded.
const PUZZLE2D_FILL_CAPTURE_UNITS_PER_STEP: usize = 2_048;
const PUZZLE2D_FILL_CAPTURE_CHUNKS: usize = 256;
const PUZZLE2D_FILL_SEARCH_UNITS_PER_STEP: usize = 512;
const PUZZLE2D_FILL_SEARCH_CHUNKS: usize = 512;
const PUZZLE2D_FILL_APPLY_UNITS_PER_STEP: usize = 4_096;
const PUZZLE2D_FILL_APPLY_CHUNKS: usize = 256;
const PUZZLE2D_FILL_CONTROL_CHUNKS: usize = 4;
const PUZZLE2D_FILL_OUTCOME_CLOSE_UNITS: usize = 4_096;
const PUZZLE2D_FILL_STEP_FUEL: u64 = 1;

/// ⏱️ The inner search context spends exactly one fuel unit — the granularity the engine's own
/// batch driver used — and is never clock-bounded: the retained job already
/// enforces the wall-clock step deadline before it calls [`Puzzle2dFillSessionWork::step`], and a
/// checkpoint replay must re-derive exactly the same cursor, which a wall clock would break.
fn puzzle2d_fill_monotonic_zero() -> Option<u64> {
    Some(0)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ArtifactFillCaptureStage {
    Nodes,
    Handles,
    Kinds,
    Rules,
    Complete,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ArtifactNodeCaptureField {
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
enum ArtifactHandleCaptureField {
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
enum ArtifactKindCaptureField {
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
enum ArtifactRuleCaptureField {
    Begin,
    Source,
    Target,
    Bidirectional,
    Specificity,
    Publish,
}

struct ArtifactFillCaptureCursor {
    stage: ArtifactFillCaptureStage,
    node: usize,
    node_field: ArtifactNodeCaptureField,
    node_x: f64,
    node_y: f64,
    node_scale: f64,
    node_rectangle: bool,
    node_extent_x: f64,
    node_extent_y: f64,
    handle_node: usize,
    handle: usize,
    handle_field: ArtifactHandleCaptureField,
    handle_edge: usize,
    handle_connected: bool,
    handle_x: f64,
    handle_y: f64,
    handle_angle: f64,
    handle_rectangle: bool,
    handle_extent_x: f64,
    handle_extent_y: f64,
    handle_radius: f64,
    handle_node_visible: bool,
    handle_visible: bool,
    kind: usize,
    template: usize,
    kind_field: ArtifactKindCaptureField,
    kind_size: f64,
    rule: usize,
    rule_field: ArtifactRuleCaptureField,
    byte: usize,
}

impl ArtifactFillCaptureCursor {
    fn new() -> Self {
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

struct FillPlacementApplyCursor {
    placement: Option<crate::editor::puzzle2d::engine::BoardFillPlacement>,
    handles: [Option<FillPlacementHandleOwner>; crate::editor::puzzle2d::engine::BOARD_FILL_KIND_HANDLE_CAPACITY],
    handle: Option<FillPlacementHandleOwner>,
    node: Option<FillPlacementNodeOwner>,
    edge: Option<FillPlacementEdgeOwner>,
    handle_cursor: usize,
    text_byte: usize,
    stage: FillPlacementApplyStage,
}

#[derive(Clone, Copy)]
struct FillPlacementHandleOwner {
    id: crate::editor::puzzle2d::engine::BoardFillText,
    handle_kind: crate::editor::puzzle2d::engine::BoardFillText,
    angle: f64,
    radius: Option<f64>,
}

#[derive(Clone, Copy)]
struct FillPlacementNodeOwner {
    id: crate::editor::puzzle2d::engine::BoardFillText,
    node_kind: crate::editor::puzzle2d::engine::BoardFillText,
    shape: crate::editor::puzzle2d::engine::BoardFillCommitShape,
    x: f64,
    y: f64,
    radius: f64,
    width: f64,
    height: f64,
    icon_kind: Option<crate::editor::puzzle2d::engine::BoardFillText>,
}

#[derive(Clone, Copy)]
struct FillPlacementEdgeOwner {
    id: crate::editor::puzzle2d::engine::BoardFillText,
    source: crate::editor::puzzle2d::engine::BoardFillText,
    target: crate::editor::puzzle2d::engine::BoardFillText,
    edge_kind: crate::editor::puzzle2d::engine::BoardFillText,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FillPlacementApplyStage {
    BeginHandle,
    HandleId,
    HandleKind,
    HandleAngle,
    HandleRadius,
    HandlePublish,
    NodeBegin,
    NodeId,
    NodeKind,
    NodeTarget,
    NodeShape,
    NodeX,
    NodeY,
    NodeText,
    NodeAnchor,
    NodeGeometry,
    NodeHeight,
    NodeIconBegin,
    NodeIcon,
    EdgeBegin,
    EdgeId,
    EdgeKind,
    EdgeSource,
    EdgeTarget,
    Publish,
    Close,
}

enum FillPlacementApplyStep {
    Pending,
    Complete,
}

fn copy_fill_text_one(source: &crate::editor::puzzle2d::engine::BoardFillText, destination: &mut crate::editor::puzzle2d::engine::BoardFillText, byte: &mut usize) -> Result<bool, &'static str> {
    let source = source.as_str().as_bytes();
    let Some(value) = source.get(*byte).copied() else {
        *byte = 0;
        return Ok(true);
    };
    destination.try_push_byte(value).map_err(|_| "puzzle2d-fill-fixed-text")?;
    *byte += 1;
    if *byte == source.len() {
        *byte = 0;
        return Ok(true);
    }
    Ok(false)
}

fn try_document_str(text: &str) -> Result<String, &'static str> {
    let mut output = String::new();
    output.try_reserve_exact(text.len()).map_err(|_| "puzzle2d-fill-document-text")?;
    output.push_str(text);
    Ok(output)
}

fn try_document_text(text: crate::editor::puzzle2d::engine::BoardFillText) -> Result<String, &'static str> {
    try_document_str(text.as_str())
}

#[derive(Clone, Copy)]
enum FillPlacementPublishHandles<'a> {
    Commit(&'a [Option<crate::editor::puzzle2d::engine::BoardFillCommitHandle>; crate::editor::puzzle2d::engine::BOARD_FILL_KIND_HANDLE_CAPACITY]),
    Cursor(&'a [Option<FillPlacementHandleOwner>; crate::editor::puzzle2d::engine::BOARD_FILL_KIND_HANDLE_CAPACITY]),
}

struct FillPlacementPublishView<'a> {
    node_id: &'a crate::editor::puzzle2d::engine::BoardFillText,
    edge_id: &'a crate::editor::puzzle2d::engine::BoardFillText,
    edge_kind: &'a crate::editor::puzzle2d::engine::BoardFillText,
    node_kind: &'a crate::editor::puzzle2d::engine::BoardFillText,
    source_handle_id: &'a crate::editor::puzzle2d::engine::BoardFillText,
    target_handle_id: &'a crate::editor::puzzle2d::engine::BoardFillText,
    x: f64,
    y: f64,
    shape: crate::editor::puzzle2d::engine::BoardFillCommitShape,
    radius: f64,
    width: f64,
    height: f64,
    icon_kind: Option<&'a crate::editor::puzzle2d::engine::BoardFillText>,
    handles: FillPlacementPublishHandles<'a>,
    handle_count: usize,
}

impl<'a> FillPlacementPublishView<'a> {
    fn from_commit(placement: &'a crate::editor::puzzle2d::engine::BoardFillCommitPlacement) -> Self {
        Self {
            node_id: &placement.node_id,
            edge_id: &placement.edge_id,
            edge_kind: &placement.edge_kind,
            node_kind: &placement.node_kind,
            source_handle_id: &placement.source_handle_id,
            target_handle_id: &placement.target_handle_id,
            x: placement.x,
            y: placement.y,
            shape: placement.shape,
            radius: placement.radius,
            width: placement.width,
            height: placement.height,
            icon_kind: placement.icon_kind.as_ref(),
            handles: FillPlacementPublishHandles::Commit(&placement.handles),
            handle_count: placement.handle_count,
        }
    }

    fn from_cursor(node: &'a FillPlacementNodeOwner, edge: &'a FillPlacementEdgeOwner, handles: &'a [Option<FillPlacementHandleOwner>; crate::editor::puzzle2d::engine::BOARD_FILL_KIND_HANDLE_CAPACITY], handle_count: usize) -> Self {
        Self {
            node_id: &node.id,
            edge_id: &edge.id,
            edge_kind: &edge.edge_kind,
            node_kind: &node.node_kind,
            source_handle_id: &edge.source,
            target_handle_id: &edge.target,
            x: node.x,
            y: node.y,
            shape: node.shape,
            radius: node.radius,
            width: node.width,
            height: node.height,
            icon_kind: node.icon_kind.as_ref(),
            handles: FillPlacementPublishHandles::Cursor(handles),
            handle_count,
        }
    }

    fn handle(&self, index: usize) -> Option<crate::editor::puzzle2d::engine::BoardFillCommitHandle> {
        match self.handles {
            FillPlacementPublishHandles::Commit(handles) => *handles.get(index)?,
            FillPlacementPublishHandles::Cursor(handles) => handles.get(index)?.map(|handle| crate::editor::puzzle2d::engine::BoardFillCommitHandle { id: handle.id, handle_kind: handle.handle_kind, angle: handle.angle, radius: handle.radius }),
        }
    }
}

/// 📤️ Pre-credits the final event destination before materializing the fixed terminal owner.
fn publish_fixed_placement(placement: &FillPlacementPublishView<'_>, mutations: &mut Vec<Puzzle2dMutation>) -> Result<(), &'static str> {
    if placement.handle_count > crate::editor::puzzle2d::engine::BOARD_FILL_KIND_HANDLE_CAPACITY {
        return Err("puzzle2d-fill-apply-handle-capacity");
    }
    let shape = match placement.shape {
        crate::editor::puzzle2d::engine::BoardFillCommitShape::Circle => "circle",
        crate::editor::puzzle2d::engine::BoardFillCommitShape::Rectangle => "rectangle",
    };
    let mut required_bytes = size_of::<Puzzle2dMutation>()
        .checked_mul(2)
        .and_then(|bytes| bytes.checked_add(size_of::<crate::Puzzle2dHandle>().checked_mul(placement.handle_count)?))
        .and_then(|bytes| bytes.checked_add(placement.node_id.as_str().len().checked_mul(2)?))
        .and_then(|bytes| bytes.checked_add(placement.edge_id.as_str().len()))
        .and_then(|bytes| bytes.checked_add(placement.edge_kind.as_str().len()))
        .and_then(|bytes| bytes.checked_add(placement.node_kind.as_str().len()))
        .and_then(|bytes| bytes.checked_add(placement.source_handle_id.as_str().len()))
        .and_then(|bytes| bytes.checked_add(placement.target_handle_id.as_str().len()))
        .and_then(|bytes| bytes.checked_add(placement.icon_kind.map_or(0, |icon| icon.as_str().len())))
        .and_then(|bytes| bytes.checked_add(shape.len()))
        .ok_or("puzzle2d-fill-apply-backing")?;
    for index in 0..placement.handle_count {
        let handle = placement.handle(index).ok_or("puzzle2d-fill-apply-handle-owner")?;
        required_bytes = required_bytes.checked_add(handle.id.as_str().len()).and_then(|bytes| bytes.checked_add(handle.handle_kind.as_str().len())).ok_or("puzzle2d-fill-apply-backing")?;
    }
    if required_bytes > semio_framework_job::JOB_PAYLOAD_PAGE_BYTES * 2 {
        return Err("puzzle2d-fill-apply-backing");
    }
    mutations.try_reserve_exact(2).map_err(|_| "puzzle2d-fill-mutation-page")?;
    let mut handles = Vec::new();
    handles.try_reserve_exact(placement.handle_count).map_err(|_| "puzzle2d-fill-apply-backing")?;
    for index in 0..placement.handle_count {
        let handle = placement.handle(index).ok_or("puzzle2d-fill-apply-handle-owner")?;
        handles.push(crate::Puzzle2dHandle { id: try_document_text(handle.id)?, handle_kind: Some(try_document_text(handle.handle_kind)?), angle: handle.angle, radius: handle.radius, ..Default::default() });
    }
    let node = crate::Puzzle2dNode {
        id: try_document_text(*placement.node_id)?,
        node_kind: Some(try_document_text(*placement.node_kind)?),
        shape: Some(try_document_str(shape)?),
        x: placement.x,
        y: placement.y,
        radius: matches!(placement.shape, crate::editor::puzzle2d::engine::BoardFillCommitShape::Circle).then_some(placement.radius),
        width: matches!(placement.shape, crate::editor::puzzle2d::engine::BoardFillCommitShape::Rectangle).then_some(placement.width),
        height: matches!(placement.shape, crate::editor::puzzle2d::engine::BoardFillCommitShape::Rectangle).then_some(placement.height),
        text: Some(try_document_text(*placement.node_id)?),
        icon_kind: placement.icon_kind.copied().map(try_document_text).transpose()?,
        anchor: crate::Puzzle2dNodeAnchor::Fixed,
        handles,
        ..Default::default()
    };
    let edge = crate::standards::v1::subsets::any::schema::mutations::connect_handles(
        try_document_text(*placement.edge_id)?,
        try_document_text(*placement.source_handle_id)?,
        try_document_text(*placement.target_handle_id)?,
        Some(try_document_text(*placement.edge_kind)?),
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        None,
        None,
    );
    mutations.push(crate::standards::v1::subsets::any::schema::mutations::create_node(node, None));
    mutations.push(edge);
    Ok(())
}

fn publish_commit_candidate(candidate: &semio_framework_job::CommitCandidate, mutations: &mut Vec<Puzzle2dMutation>) -> Option<Result<crate::editor::puzzle2d::engine::BoardFillResult, &'static str>> {
    let candidate = crate::editor::puzzle2d::engine::BoardFillCommitCandidate::from_commit_candidate(candidate)?;
    if let Some(placement) = candidate.placement.as_ref() {
        if let Err(code) = publish_fixed_placement(&FillPlacementPublishView::from_commit(placement), mutations) {
            return Some(Err(code));
        }
    }
    Some(Ok(candidate.result))
}

impl FillPlacementApplyCursor {
    fn new(placement: crate::editor::puzzle2d::engine::BoardFillPlacement) -> Self {
        Self { placement: Some(placement), handles: std::array::from_fn(|_| None), handle: None, node: None, edge: None, handle_cursor: 0, text_byte: 0, stage: FillPlacementApplyStage::BeginHandle }
    }

    fn step(&mut self, mutations: &mut Vec<Puzzle2dMutation>) -> Result<FillPlacementApplyStep, &'static str> {
        let placement = self.placement.as_ref().ok_or("puzzle2d-fill-apply-owner")?;
        match self.stage {
            FillPlacementApplyStage::BeginHandle => {
                if self.handle_cursor < placement.handle_count() {
                    self.handle = Some(FillPlacementHandleOwner { id: crate::editor::puzzle2d::engine::BoardFillText::empty(), handle_kind: crate::editor::puzzle2d::engine::BoardFillText::empty(), angle: 0.0, radius: None });
                    self.stage = FillPlacementApplyStage::HandleId;
                } else {
                    self.stage = FillPlacementApplyStage::NodeBegin;
                }
            }
            FillPlacementApplyStage::HandleId => {
                let source = placement.fixed_handle_id(self.handle_cursor).ok_or("puzzle2d-fill-apply-handle")?;
                let destination = &mut self.handle.as_mut().ok_or("puzzle2d-fill-apply-handle-owner")?.id;
                if copy_fill_text_one(source, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::HandleKind;
                }
            }
            FillPlacementApplyStage::HandleKind => {
                let source = placement.fixed_handle_kind(self.handle_cursor).ok_or("puzzle2d-fill-apply-handle")?;
                let destination = &mut self.handle.as_mut().ok_or("puzzle2d-fill-apply-handle-owner")?.handle_kind;
                if copy_fill_text_one(source, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::HandleAngle;
                }
            }
            FillPlacementApplyStage::HandleAngle => {
                self.handle.as_mut().ok_or("puzzle2d-fill-apply-handle-owner")?.angle = placement.fixed_handle_angle(self.handle_cursor).ok_or("puzzle2d-fill-apply-handle")?;
                self.stage = FillPlacementApplyStage::HandleRadius;
            }
            FillPlacementApplyStage::HandleRadius => {
                self.handle.as_mut().ok_or("puzzle2d-fill-apply-handle-owner")?.radius = placement.fixed_handle_radius(self.handle_cursor).ok_or("puzzle2d-fill-apply-handle")?;
                self.stage = FillPlacementApplyStage::HandlePublish;
            }
            FillPlacementApplyStage::HandlePublish => {
                let handle = self.handle.take().ok_or("puzzle2d-fill-apply-handle-owner")?;
                let slot = self.handles.get_mut(self.handle_cursor).ok_or("puzzle2d-fill-apply-handle-capacity")?;
                if slot.is_some() {
                    self.handle = Some(handle);
                    return Err("puzzle2d-fill-apply-handle-owner");
                }
                *slot = Some(handle);
                self.handle_cursor += 1;
                self.stage = FillPlacementApplyStage::BeginHandle;
            }
            FillPlacementApplyStage::NodeBegin => {
                self.node = Some(FillPlacementNodeOwner {
                    id: crate::editor::puzzle2d::engine::BoardFillText::empty(),
                    node_kind: crate::editor::puzzle2d::engine::BoardFillText::empty(),
                    shape: crate::editor::puzzle2d::engine::BoardFillCommitShape::Circle,
                    x: 0.0,
                    y: 0.0,
                    radius: 0.0,
                    width: 0.0,
                    height: 0.0,
                    icon_kind: None,
                });
                self.stage = FillPlacementApplyStage::NodeId;
            }
            FillPlacementApplyStage::NodeId => {
                let destination = &mut self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?.id;
                if copy_fill_text_one(&placement.node_id, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::NodeKind;
                }
            }
            FillPlacementApplyStage::NodeKind => {
                let destination = &mut self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?.node_kind;
                if copy_fill_text_one(&placement.node_kind, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::NodeTarget;
                }
            }
            FillPlacementApplyStage::NodeTarget => {
                self.stage = FillPlacementApplyStage::NodeShape;
            }
            FillPlacementApplyStage::NodeShape => {
                self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?.shape = match placement.shape {
                    "circle" => crate::editor::puzzle2d::engine::BoardFillCommitShape::Circle,
                    "rectangle" => crate::editor::puzzle2d::engine::BoardFillCommitShape::Rectangle,
                    _ => return Err("puzzle2d-fill-node-shape"),
                };
                self.stage = FillPlacementApplyStage::NodeX;
            }
            FillPlacementApplyStage::NodeX => {
                self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?.x = placement.x;
                self.stage = FillPlacementApplyStage::NodeY;
            }
            FillPlacementApplyStage::NodeY => {
                self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?.y = placement.y;
                self.stage = FillPlacementApplyStage::NodeText;
            }
            FillPlacementApplyStage::NodeText => {
                self.stage = FillPlacementApplyStage::NodeAnchor;
            }
            FillPlacementApplyStage::NodeAnchor => {
                self.stage = FillPlacementApplyStage::NodeGeometry;
            }
            FillPlacementApplyStage::NodeGeometry => {
                let node = self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?;
                if placement.shape == "rectangle" {
                    node.width = placement.width;
                    self.stage = FillPlacementApplyStage::NodeHeight;
                } else {
                    node.radius = placement.radius;
                    self.stage = FillPlacementApplyStage::NodeIconBegin;
                }
            }
            FillPlacementApplyStage::NodeHeight => {
                self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?.height = placement.height;
                self.stage = FillPlacementApplyStage::NodeIconBegin;
            }
            FillPlacementApplyStage::NodeIconBegin => {
                if placement.icon_kind.as_ref().is_some() {
                    self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?.icon_kind = Some(crate::editor::puzzle2d::engine::BoardFillText::empty());
                    self.stage = FillPlacementApplyStage::NodeIcon;
                } else {
                    self.stage = FillPlacementApplyStage::EdgeBegin;
                }
            }
            FillPlacementApplyStage::NodeIcon => {
                let source = placement.icon_kind.as_ref().ok_or("puzzle2d-fill-node-icon")?;
                let destination = self.node.as_mut().ok_or("puzzle2d-fill-node-owner")?.icon_kind.as_mut().ok_or("puzzle2d-fill-node-icon-owner")?;
                if copy_fill_text_one(source, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::EdgeBegin;
                }
            }
            FillPlacementApplyStage::EdgeBegin => {
                self.edge = Some(FillPlacementEdgeOwner {
                    id: crate::editor::puzzle2d::engine::BoardFillText::empty(),
                    source: crate::editor::puzzle2d::engine::BoardFillText::empty(),
                    target: crate::editor::puzzle2d::engine::BoardFillText::empty(),
                    edge_kind: crate::editor::puzzle2d::engine::BoardFillText::empty(),
                });
                self.stage = FillPlacementApplyStage::EdgeId;
            }
            FillPlacementApplyStage::EdgeId => {
                let destination = &mut self.edge.as_mut().ok_or("puzzle2d-fill-edge-owner")?.id;
                if copy_fill_text_one(&placement.edge_id, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::EdgeKind;
                }
            }
            FillPlacementApplyStage::EdgeKind => {
                let destination = &mut self.edge.as_mut().ok_or("puzzle2d-fill-edge-owner")?.edge_kind;
                if copy_fill_text_one(&placement.edge_kind, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::EdgeSource;
                }
            }
            FillPlacementApplyStage::EdgeSource => {
                let destination = &mut self.edge.as_mut().ok_or("puzzle2d-fill-edge-owner")?.source;
                if copy_fill_text_one(&placement.source_handle_id, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::EdgeTarget;
                }
            }
            FillPlacementApplyStage::EdgeTarget => {
                let destination = &mut self.edge.as_mut().ok_or("puzzle2d-fill-edge-owner")?.target;
                if copy_fill_text_one(&placement.target_handle_id, destination, &mut self.text_byte)? {
                    self.stage = FillPlacementApplyStage::Publish;
                }
            }
            FillPlacementApplyStage::Publish => {
                let node = self.node.as_ref().ok_or("puzzle2d-fill-node-owner")?;
                let edge = self.edge.as_ref().ok_or("puzzle2d-fill-edge-owner")?;
                publish_fixed_placement(&FillPlacementPublishView::from_cursor(node, edge, &self.handles, self.handle_cursor), mutations)?;
                self.node = None;
                self.edge = None;
                self.handles = std::array::from_fn(|_| None);
                self.stage = FillPlacementApplyStage::Close;
            }
            FillPlacementApplyStage::Close => {
                let placement = self.placement.as_mut().ok_or("puzzle2d-fill-apply-owner")?;
                if placement.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
                    self.placement = None;
                    return Ok(FillPlacementApplyStep::Complete);
                }
            }
        }
        Ok(FillPlacementApplyStep::Pending)
    }

    fn close_step(&mut self) -> bool {
        if self.handle.take().is_some() {
            return false;
        }
        if let Some(slot) = self.handles.iter_mut().rev().find(|slot| slot.is_some()) {
            *slot = None;
            return false;
        }
        if self.node.take().is_some() {
            return false;
        }
        if self.edge.take().is_some() {
            return false;
        }
        if let Some(placement) = self.placement.as_mut() {
            if placement.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
                self.placement = None;
            }
            return false;
        }
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.placement.is_none() && self.handles.iter().all(Option::is_none) && self.handle.is_none() && self.node.is_none() && self.edge.is_none()
    }
}

impl Drop for FillPlacementApplyCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Puzzle2d placement apply must reach exact terminal-empty before Drop");
    }
}

fn capture_fault_code(fault: crate::editor::puzzle2d::engine::BoardFillCaptureFault) -> &'static str {
    match fault {
        crate::editor::puzzle2d::engine::BoardFillCaptureFault::TextCapacity => "puzzle2d-fill-capture-text-capacity",
        crate::editor::puzzle2d::engine::BoardFillCaptureFault::NodeCapacity => "puzzle2d-fill-capture-node-capacity",
        crate::editor::puzzle2d::engine::BoardFillCaptureFault::HandleCapacity => "puzzle2d-fill-capture-handle-capacity",
        crate::editor::puzzle2d::engine::BoardFillCaptureFault::KindCapacity => "puzzle2d-fill-capture-kind-capacity",
        crate::editor::puzzle2d::engine::BoardFillCaptureFault::KindHandleCapacity => "puzzle2d-fill-capture-kind-handle-capacity",
        crate::editor::puzzle2d::engine::BoardFillCaptureFault::RuleCapacity => "puzzle2d-fill-capture-rule-capacity",
        crate::editor::puzzle2d::engine::BoardFillCaptureFault::StaleNode => "puzzle2d-fill-capture-stale-node",
        crate::editor::puzzle2d::engine::BoardFillCaptureFault::StaleHandle => "puzzle2d-fill-capture-stale-handle",
        crate::editor::puzzle2d::engine::BoardFillCaptureFault::StaleKind => "puzzle2d-fill-capture-stale-kind",
        crate::editor::puzzle2d::engine::BoardFillCaptureFault::StaleRule => "puzzle2d-fill-capture-stale-rule",
        crate::editor::puzzle2d::engine::BoardFillCaptureFault::GenerationExhausted => "puzzle2d-fill-capture-generation-exhausted",
    }
}

//#region 🪣️Session
/// 🧵️ Where one fill session currently is. `Control` applies the verb's runtime transition, `Capture`
/// streams the document into the engine's fixed ingress, `Search` drives the `BoardFillJob`, `Apply`
/// drains one checkpoint's pending placement into mutations before handing the checkpoint back.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle2dFillStage {
    Control,
    Capture,
    Search,
    Apply,
    Complete,
    Closing,
}

/// 🪣️ One brush-fill session as a retained tool work. Every owner it holds — ingress, search job,
/// checkpoint, placement cursor, retained outcome — is released through [`Puzzle2dFillSessionWork::
/// close_step`] so each engine owner reaches its exact terminal-empty state before `Drop`.
pub struct Puzzle2dFillSessionWork {
    tool_id: &'static str,
    stage: Puzzle2dFillStage,
    operation: semio_framework_job::Operation,
    preview_sequence: u64,
    maximum_count: u32,
    capture: ArtifactFillCaptureCursor,
    capture_chunks: usize,
    search_chunks: usize,
    apply_chunks: usize,
    suggestion_offset: f64,
    ingress: Option<crate::editor::puzzle2d::engine::BoardFillSnapshotIngress>,
    search: Option<crate::editor::puzzle2d::engine::BoardFillJob>,
    closing_search: Option<crate::editor::puzzle2d::engine::BoardFillJob>,
    checkpoint: Option<crate::editor::puzzle2d::engine::BoardFillCheckpoint>,
    apply: Option<FillPlacementApplyCursor>,
    outcome: Option<semio_framework_job::StepOutcome>,
    runtime: Option<Puzzle2dFillRuntime>,
    mutations: Vec<Puzzle2dMutation>,
    effects: Vec<Effect>,
    view_state: Option<semio_framework_plugin::ViewModel>,
    window_config: Puzzle2dWindowConfig,
    closing: bool,
}

impl Puzzle2dFillSessionWork {
    /// 🪪️ One work per admitted verb — the retained job's decode phase refuses a payload whose
    /// action id does not equal [`crate::retained_command::PuzzleCommandWork::tool_id`].
    pub fn new(tool_id: &'static str) -> Self {
        Self {
            tool_id,
            stage: Puzzle2dFillStage::Control,
            operation: semio_framework_job::Operation::new(semio_framework_job::OperationId(0), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), 0),
            preview_sequence: 0,
            maximum_count: 0,
            capture: ArtifactFillCaptureCursor::new(),
            capture_chunks: 0,
            search_chunks: 0,
            apply_chunks: 0,
            suggestion_offset: 0.0,
            ingress: None,
            search: None,
            closing_search: None,
            checkpoint: None,
            apply: None,
            outcome: None,
            runtime: None,
            mutations: Vec::new(),
            effects: Vec::new(),
            view_state: None,
            window_config: Puzzle2dWindowConfig::default(),
            closing: false,
        }
    }

    fn progress(stage: &'static str, en: &'static str, de: &'static str) -> crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle2dPlayApp>> {
        crate::retained_command::PuzzleCommandWorkStep::Progress { stage, en, de }
    }

    fn runtime_mut(&mut self) -> Result<&mut Puzzle2dFillRuntime, Fault> {
        self.runtime.as_mut().ok_or_else(|| Fault::from("puzzle2d-fill-runtime-owner"))
    }

    fn text(code: &'static str) -> Result<Puzzle2dFillText, Fault> {
        Puzzle2dFillText::try_from_str(code).ok_or_else(|| Fault::from("puzzle2d-fill-runtime-text-capacity"))
    }

    /// 🩹️ A fault is a published lifecycle, not a dropped session: the runtime carries the code so
    /// the panel can offer `brushFillSessionRetry`/`brushFillSessionDiscard`.
    fn fault(&mut self, code: &'static str) -> Result<(), Fault> {
        let text = Self::text(code)?;
        let runtime = self.runtime_mut()?;
        runtime.fill_job_fault_code = Some(text);
        runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Faulted;
        self.stage = Puzzle2dFillStage::Complete;
        Ok(())
    }

    fn begin_search(&mut self, count: u32, seed: u64) -> Result<(), Fault> {
        if count > fill::PUZZLE2D_FILL_COUNT_MAX {
            return self.fault("puzzle2d-fill-count-capacity");
        }
        self.maximum_count = count;
        self.operation = semio_framework_job::Operation::new(self.operation.operation, self.operation.base_revision, self.operation.generation, seed);
        self.suggestion_offset = self.window_config.suggestion_offset;
        self.ingress = Some(crate::editor::puzzle2d::engine::BoardFillSnapshotIngress::new(self.window_config.suggestion_offset));
        self.capture = ArtifactFillCaptureCursor::new();
        let stage = Self::text("capture")?;
        let operation = self.operation;
        let runtime = self.runtime_mut()?;
        runtime.fill_job_operation = operation.operation.0;
        runtime.fill_job_generation = operation.generation.0;
        runtime.fill_job_base_revision = operation.base_revision.0;
        runtime.fill_job_seed = seed;
        runtime.fill_job_checkpoint_sequence = 0;
        runtime.fill_job_accepted_count = 0;
        runtime.fill_job_search_count = 0;
        runtime.fill_job_stage = stage;
        runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Capturing;
        runtime.fill_job_fault_code = None;
        self.stage = Puzzle2dFillStage::Capture;
        Ok(())
    }

    fn publish_preview(&mut self, preview: crate::editor::puzzle2d::engine::BoardFillPreview) -> Result<(), Fault> {
        let Some(stage) = Puzzle2dFillText::try_from_str(preview.stage.id()) else {
            return self.fault("puzzle2d-fill-stage-capacity");
        };
        let runtime = self.runtime_mut()?;
        runtime.fill_job_stage = stage;
        runtime.fill_job_accepted_count = u64::from(preview.accepted_count);
        runtime.fill_job_search_count = preview.search_count;
        runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Running;
        Ok(())
    }

    /// 🧵️ One `BoardFillJob::step` under a fuel-only context bound to this retained operation.
    fn search_step_one(&mut self) -> Option<semio_framework_job::StepOutcome> {
        let Self { search, preview_sequence, operation, .. } = self;
        let job = search.as_mut()?;
        let budget = semio_framework_job::StepBudget::new(PUZZLE2D_FILL_STEP_FUEL, u64::MAX);
        let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, budget, semio_framework_job::root_cancel_token(), puzzle2d_fill_monotonic_zero, preview_sequence);
        Some(semio_framework_job::InteractiveJob::step(job, &mut context))
    }

    /// 🚰️ Drains a retained outcome's payload pages before it is released — the engine's outcomes
    /// assert exact terminal-emptiness on `Drop`.
    fn drain_outcome(&mut self) -> Result<(), Fault> {
        for _ in 0..PUZZLE2D_FILL_OUTCOME_CLOSE_UNITS {
            if self.outcome.as_ref().is_none_or(semio_framework_job::StepOutcome::terminal_is_empty) {
                self.outcome = None;
                return Ok(());
            }
            if let Some(outcome) = self.outcome.as_mut() {
                let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
        }
        Err(Fault::from("puzzle2d-fill-outcome-close-budget"))
    }

    fn absorb_outcome(&mut self, outcome: semio_framework_job::StepOutcome) -> Result<(), Fault> {
        match &outcome {
            semio_framework_job::StepOutcome::PreviewReady(_) => {
                let preview = self.search.as_mut().and_then(crate::editor::puzzle2d::engine::BoardFillJob::take_preview);
                if let Some(preview) = preview {
                    self.publish_preview(preview)?;
                }
            }
            semio_framework_job::StepOutcome::CheckpointReady(_) => {
                let taken = self.search.as_mut().and_then(crate::editor::puzzle2d::engine::BoardFillJob::take_checkpoint);
                match taken {
                    Some(checkpoint) => {
                        let accepted = checkpoint.accepted_count();
                        self.checkpoint = Some(checkpoint);
                        let runtime = self.runtime_mut()?;
                        runtime.fill_job_checkpoint_sequence = runtime.fill_job_checkpoint_sequence.saturating_add(1);
                        runtime.fill_job_accepted_count = u64::from(accepted);
                        runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::CheckpointReady;
                        self.stage = Puzzle2dFillStage::Apply;
                    }
                    None => self.fault("puzzle2d-fill-checkpoint-missing")?,
                }
            }
            semio_framework_job::StepOutcome::Complete(candidate) => {
                let published = publish_commit_candidate(candidate, &mut self.mutations);
                match published {
                    Some(Ok(result)) => {
                        let runtime = self.runtime_mut()?;
                        runtime.fill_job_accepted_count = u64::from(result.accepted_count);
                        runtime.fill_job_search_count = result.search_count;
                        runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::AwaitingAdoption;
                        self.stage = Puzzle2dFillStage::Complete;
                    }
                    Some(Err(code)) => self.fault(code)?,
                    None => self.fault("puzzle2d-fill-result-missing")?,
                }
            }
            semio_framework_job::StepOutcome::Cancelled => {
                self.runtime_mut()?.fill_job_lifecycle = Puzzle2dFillLifecycle::Cancelled;
                self.stage = Puzzle2dFillStage::Complete;
            }
            semio_framework_job::StepOutcome::Fault(_) => {
                let code = self.search.as_mut().and_then(crate::editor::puzzle2d::engine::BoardFillJob::take_fault).unwrap_or("puzzle2d-fill-worker-fault");
                self.fault(code)?;
            }
            semio_framework_job::StepOutcome::Yield => {}
        }
        self.outcome = Some(outcome);
        self.drain_outcome()
    }

    /// 🔁️ Hands an applied checkpoint back to the search so the engine resumes from its own state.
    fn adopt_checkpoint(&mut self) -> Result<(), Fault> {
        let Some(checkpoint) = self.checkpoint.take() else { return Ok(()) };
        let handback = match self.search.as_mut() {
            Some(job) => job.adopt_checkpoint(checkpoint).err(),
            None => Some(checkpoint),
        };
        if let Some(checkpoint) = handback {
            self.closing_search = Some(checkpoint.into_closing_job());
            return self.fault("puzzle2d-fill-checkpoint-stale");
        }
        self.runtime_mut()?.fill_job_lifecycle = Puzzle2dFillLifecycle::Running;
        self.stage = Puzzle2dFillStage::Search;
        Ok(())
    }

    fn apply_one(&mut self) -> Result<(), Fault> {
        if self.apply.is_none() {
            let placement = self.checkpoint.as_mut().and_then(crate::editor::puzzle2d::engine::BoardFillCheckpoint::take_pending_placement);
            match placement {
                Some(placement) => {
                    self.apply = Some(FillPlacementApplyCursor::new(placement));
                    self.runtime_mut()?.fill_job_lifecycle = Puzzle2dFillLifecycle::Applying;
                }
                None => return self.adopt_checkpoint(),
            }
        }
        let stepped = {
            let Self { apply, mutations, .. } = self;
            let Some(cursor) = apply.as_mut() else { return Ok(()) };
            cursor.step(mutations)
        };
        match stepped {
            Ok(FillPlacementApplyStep::Pending) => Ok(()),
            Ok(FillPlacementApplyStep::Complete) => {
                self.apply = None;
                Ok(())
            }
            Err(code) => self.fault(code),
        }
    }

    /// 🏁️ The one publication a session makes: every accepted placement, plus the runtime the next
    /// verb resumes from, plus the tool activation `setFillCount` requests.
    fn complete(&mut self) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle2dPlayApp>>, Fault> {
        let runtime = self.runtime.take().ok_or_else(|| Fault::from("puzzle2d-fill-runtime-owner"))?;
        let mut next_window_config = self.window_config.clone();
        next_window_config.fill_count = runtime.fill_count;
        let window_config_mutations = if next_window_config != self.window_config { vec![window::addressed_config(self.view_state.as_ref().ok_or_else(|| Fault::from("puzzle2d-fill-window-context"))?, next_window_config)?] } else { Vec::new() };
        let artifact_mutations = std::mem::take(&mut self.mutations);
        let effects = std::mem::take(&mut self.effects);
        self.stage = Puzzle2dFillStage::Complete;
        Ok(crate::retained_command::PuzzleCommandWorkStep::Complete(Emit { artifact_mutations, window_config_mutations, coalesce_key: None, effects, ui_scope: UiDirtyScope::Full, ..Default::default() }))
    }

    /// 🚰️ Releases one engine owner per call, innermost first: the checkpoint hands its state back
    /// to the search before the search itself closes, matching the order the engine's own `Drop`
    /// assertions require.
    fn close_one(&mut self) -> bool {
        if let Some(outcome) = self.outcome.as_mut() {
            if outcome.terminal_is_empty() {
                self.outcome = None;
            } else {
                let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            return true;
        }
        if let Some(cursor) = self.apply.as_mut() {
            if cursor.close_step() {
                self.apply = None;
            }
            return true;
        }
        if let Some(checkpoint) = self.checkpoint.take() {
            let handback = match self.search.as_mut() {
                Some(job) => job.adopt_checkpoint(checkpoint).err(),
                None => Some(checkpoint),
            };
            if let Some(checkpoint) = handback {
                self.closing_search = Some(checkpoint.into_closing_job());
            }
            return true;
        }
        if self.closing_search.is_some() {
            Self::close_job_slot(&mut self.closing_search);
            return true;
        }
        if self.search.is_some() {
            Self::close_job_slot(&mut self.search);
            return true;
        }
        if let Some(ingress) = self.ingress.as_mut() {
            ingress.begin_close();
            if matches!(ingress.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) && ingress.terminal_is_empty() {
                self.ingress = None;
            }
            return true;
        }
        if self.mutations.pop().is_some() {
            return true;
        }
        if self.effects.pop().is_some() {
            return true;
        }
        self.runtime.take().is_some()
    }

    fn close_job_slot(slot: &mut Option<crate::editor::puzzle2d::engine::BoardFillJob>) {
        let Some(job) = slot.as_mut() else { return };
        semio_framework_job::InteractiveJob::begin_close(job);
        if matches!(semio_framework_job::InteractiveJob::close_step(job, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) && semio_framework_job::InteractiveJob::terminal_is_empty(job) {
            *slot = None;
        }
    }

    fn session_is_empty(&self) -> bool {
        self.outcome.is_none() && self.apply.is_none() && self.checkpoint.is_none() && self.closing_search.is_none() && self.search.is_none() && self.ingress.is_none() && self.mutations.is_empty() && self.effects.is_empty() && self.runtime.is_none()
    }
}

//#endregion 🪣️Session

//#region 🔬️Capture
/// 🔬️ Streams the document into the engine's fixed-capacity fill ingress one field — and, for text,
/// one byte — per call, so a capture never allocates and never outruns its step budget.
impl Puzzle2dFillSessionWork {
    fn nodes(document: &Value) -> Result<&[Value], &'static str> {
        document.get("nodes").and_then(Value::as_array).map(Vec::as_slice).ok_or("puzzle2d-fill-capture-nodes")
    }

    fn edges(document: &Value) -> Result<&[Value], &'static str> {
        document.get("edges").and_then(Value::as_array).map(Vec::as_slice).ok_or("puzzle2d-fill-capture-edges")
    }

    fn node_kinds(document: &Value) -> Result<&[Value], &'static str> {
        document.get("meta").and_then(|meta| meta.get("kindCatalogs")).and_then(|catalogs| catalogs.get("nodes")).and_then(Value::as_array).map(Vec::as_slice).ok_or("puzzle2d-fill-capture-node-kinds")
    }

    fn rules(document: &Value) -> Option<&[Value]> {
        document.get("meta").and_then(|meta| meta.get("kindCompatibility")).or_else(|| document.get("kindCompatibility")).and_then(Value::as_array).map(Vec::as_slice)
    }

    fn finite(value: Option<f64>, default: f64) -> f64 {
        value.filter(|value| value.is_finite()).unwrap_or(default)
    }

    fn capture_node_one(&mut self, document: &Value) -> Result<(), &'static str> {
        let nodes = Self::nodes(document)?;
        let Some(node) = nodes.get(self.capture.node) else {
            if self.capture.node_field != ArtifactNodeCaptureField::Begin {
                return Err("puzzle2d-fill-capture-stale-node");
            }
            self.capture.stage = ArtifactFillCaptureStage::Handles;
            return Ok(());
        };
        match self.capture.node_field {
            ArtifactNodeCaptureField::Begin => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.begin_node().map_err(capture_fault_code)?;
                self.capture.node_field = ArtifactNodeCaptureField::Id;
            }
            ArtifactNodeCaptureField::Id => {
                let value = node.get("id").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-node-id")?;
                if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_node_id_byte(byte).map_err(capture_fault_code)?;
                    self.capture.byte += 1;
                } else {
                    self.capture.byte = 0;
                    self.capture.node_field = ArtifactNodeCaptureField::X;
                }
            }
            ArtifactNodeCaptureField::X => {
                self.capture.node_x = Self::finite(node.get("x").and_then(Value::as_f64), 0.0);
                self.capture.node_field = ArtifactNodeCaptureField::Y;
            }
            ArtifactNodeCaptureField::Y => {
                self.capture.node_y = Self::finite(node.get("y").and_then(Value::as_f64), 0.0);
                self.capture.node_field = ArtifactNodeCaptureField::Scale;
            }
            ArtifactNodeCaptureField::Scale => {
                self.capture.node_scale = Self::finite(node.get("scale").and_then(Value::as_f64), 1.0).max(f64::EPSILON);
                self.capture.node_field = ArtifactNodeCaptureField::Shape;
            }
            ArtifactNodeCaptureField::Shape => {
                self.capture.node_rectangle = node.get("shape").and_then(Value::as_str) == Some("rectangle");
                self.capture.node_field = ArtifactNodeCaptureField::ExtentX;
            }
            ArtifactNodeCaptureField::ExtentX => {
                let field = if self.capture.node_rectangle { "width" } else { "radius" };
                let extent = Self::finite(node.get(field).and_then(Value::as_f64), 1.0).max(f64::EPSILON) * self.capture.node_scale;
                self.capture.node_extent_x = if self.capture.node_rectangle { extent * 0.5 } else { extent };
                self.capture.node_field = ArtifactNodeCaptureField::ExtentY;
            }
            ArtifactNodeCaptureField::ExtentY => {
                self.capture.node_extent_y = if self.capture.node_rectangle { Self::finite(node.get("height").and_then(Value::as_f64), 1.0).max(f64::EPSILON) * self.capture.node_scale * 0.5 } else { self.capture.node_extent_x };
                self.capture.node_field = ArtifactNodeCaptureField::Bound0;
            }
            ArtifactNodeCaptureField::Bound0 => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_node_bound(0, self.capture.node_x - self.capture.node_extent_x).map_err(capture_fault_code)?;
                self.capture.node_field = ArtifactNodeCaptureField::Bound1;
            }
            ArtifactNodeCaptureField::Bound1 => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_node_bound(1, self.capture.node_y - self.capture.node_extent_y).map_err(capture_fault_code)?;
                self.capture.node_field = ArtifactNodeCaptureField::Bound2;
            }
            ArtifactNodeCaptureField::Bound2 => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_node_bound(2, self.capture.node_x + self.capture.node_extent_x).map_err(capture_fault_code)?;
                self.capture.node_field = ArtifactNodeCaptureField::Bound3;
            }
            ArtifactNodeCaptureField::Bound3 => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_node_bound(3, self.capture.node_y + self.capture.node_extent_y).map_err(capture_fault_code)?;
                self.capture.node_field = ArtifactNodeCaptureField::Publish;
            }
            ArtifactNodeCaptureField::Publish => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.publish_node().map_err(capture_fault_code)?;
                self.capture.node += 1;
                self.capture.node_field = ArtifactNodeCaptureField::Begin;
            }
        }
        Ok(())
    }

    fn capture_handle_one(&mut self, document: &Value) -> Result<(), &'static str> {
        let nodes = Self::nodes(document)?;
        let Some(node) = nodes.get(self.capture.handle_node) else {
            self.capture.stage = ArtifactFillCaptureStage::Kinds;
            return Ok(());
        };
        let handles = node.get("handles").and_then(Value::as_array).ok_or("puzzle2d-fill-capture-handles")?;
        let Some(handle) = handles.get(self.capture.handle) else {
            if self.capture.handle_field != ArtifactHandleCaptureField::Begin {
                return Err("puzzle2d-fill-capture-stale-handle");
            }
            self.capture.handle_node += 1;
            self.capture.handle = 0;
            self.capture.handle_edge = 0;
            self.capture.handle_connected = false;
            return Ok(());
        };
        match self.capture.handle_field {
            ArtifactHandleCaptureField::Begin => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.begin_handle().map_err(capture_fault_code)?;
                self.capture.handle_field = ArtifactHandleCaptureField::Id;
            }
            ArtifactHandleCaptureField::Id => {
                let value = handle.get("id").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-handle-id")?;
                if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_handle_text_byte(crate::editor::puzzle2d::engine::BoardFillIngressHandleText::Id, byte).map_err(capture_fault_code)?;
                    self.capture.byte += 1;
                } else {
                    self.capture.byte = 0;
                    self.capture.handle_field = ArtifactHandleCaptureField::ScanEdges;
                }
            }
            ArtifactHandleCaptureField::ScanEdges => {
                let id = handle.get("id").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-handle-id")?;
                let edges = Self::edges(document)?;
                if let Some(edge) = edges.get(self.capture.handle_edge) {
                    self.capture.handle_edge += 1;
                    if edge.get("source").and_then(Value::as_str) == Some(id) || edge.get("target").and_then(Value::as_str) == Some(id) {
                        self.capture.handle_connected = true;
                    }
                } else {
                    self.capture.handle_field = ArtifactHandleCaptureField::NodeKind;
                }
            }
            ArtifactHandleCaptureField::NodeKind | ArtifactHandleCaptureField::HandleKind | ArtifactHandleCaptureField::WireKind | ArtifactHandleCaptureField::EdgeKind => {
                let (value, field, next) = match self.capture.handle_field {
                    ArtifactHandleCaptureField::NodeKind => (node.get("nodeKind").and_then(Value::as_str).unwrap_or(""), crate::editor::puzzle2d::engine::BoardFillIngressHandleText::NodeKind, ArtifactHandleCaptureField::HandleKind),
                    ArtifactHandleCaptureField::HandleKind => (handle.get("handleKind").and_then(Value::as_str).unwrap_or("port"), crate::editor::puzzle2d::engine::BoardFillIngressHandleText::HandleKind, ArtifactHandleCaptureField::WireKind),
                    ArtifactHandleCaptureField::WireKind => (handle.get("wireKind").and_then(Value::as_str).unwrap_or("wire.link"), crate::editor::puzzle2d::engine::BoardFillIngressHandleText::WireKind, ArtifactHandleCaptureField::EdgeKind),
                    ArtifactHandleCaptureField::EdgeKind => (handle.get("edgeKind").and_then(Value::as_str).unwrap_or(""), crate::editor::puzzle2d::engine::BoardFillIngressHandleText::EdgeKind, ArtifactHandleCaptureField::X),
                    _ => return Err("puzzle2d-fill-capture-handle-field"),
                };
                if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_handle_text_byte(field, byte).map_err(capture_fault_code)?;
                    self.capture.byte += 1;
                } else {
                    self.capture.byte = 0;
                    self.capture.handle_field = next;
                }
            }
            ArtifactHandleCaptureField::X => {
                self.capture.handle_x = Self::finite(node.get("x").and_then(Value::as_f64), 0.0);
                self.capture.handle_field = ArtifactHandleCaptureField::Y;
            }
            ArtifactHandleCaptureField::Y => {
                self.capture.handle_y = Self::finite(node.get("y").and_then(Value::as_f64), 0.0);
                self.capture.handle_field = ArtifactHandleCaptureField::Angle;
            }
            ArtifactHandleCaptureField::Angle => {
                self.capture.handle_angle = Self::finite(handle.get("angle").and_then(Value::as_f64), 0.0);
                self.capture.handle_field = ArtifactHandleCaptureField::Shape;
            }
            ArtifactHandleCaptureField::Shape => {
                self.capture.handle_rectangle = node.get("shape").and_then(Value::as_str) == Some("rectangle");
                self.capture.handle_field = ArtifactHandleCaptureField::ExtentX;
            }
            ArtifactHandleCaptureField::ExtentX => {
                let field = if self.capture.handle_rectangle { "width" } else { "radius" };
                self.capture.handle_extent_x = Self::finite(node.get(field).and_then(Value::as_f64), 1.0).max(f64::EPSILON);
                self.capture.handle_field = ArtifactHandleCaptureField::ExtentY;
            }
            ArtifactHandleCaptureField::ExtentY => {
                self.capture.handle_extent_y = if self.capture.handle_rectangle { Self::finite(node.get("height").and_then(Value::as_f64), 1.0).max(f64::EPSILON) } else { self.capture.handle_extent_x };
                self.capture.handle_field = ArtifactHandleCaptureField::Radius;
            }
            ArtifactHandleCaptureField::Radius => {
                self.capture.handle_radius = if self.capture.handle_rectangle { self.capture.handle_extent_x.max(self.capture.handle_extent_y) * 0.5 } else { self.capture.handle_extent_x };
                self.capture.handle_field = ArtifactHandleCaptureField::NodeVisible;
            }
            ArtifactHandleCaptureField::NodeVisible => {
                self.capture.handle_node_visible = node.get("visible").and_then(Value::as_bool) != Some(false);
                self.capture.handle_field = ArtifactHandleCaptureField::HandleVisible;
            }
            ArtifactHandleCaptureField::HandleVisible => {
                self.capture.handle_visible = handle.get("visible").and_then(Value::as_bool) != Some(false);
                self.capture.handle_field = ArtifactHandleCaptureField::Visible;
            }
            ArtifactHandleCaptureField::Visible => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_handle_visible(self.capture.handle_node_visible && self.capture.handle_visible).map_err(capture_fault_code)?;
                self.capture.handle_field = ArtifactHandleCaptureField::Connected;
            }
            ArtifactHandleCaptureField::Connected => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_handle_connected(self.capture.handle_connected).map_err(capture_fault_code)?;
                self.capture.handle_field = ArtifactHandleCaptureField::SlotX;
            }
            ArtifactHandleCaptureField::SlotX => {
                let distance = self.capture.handle_radius + self.suggestion_offset;
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_handle_slot(0, self.capture.handle_x + self.capture.handle_angle.cos() * distance).map_err(capture_fault_code)?;
                self.capture.handle_field = ArtifactHandleCaptureField::SlotY;
            }
            ArtifactHandleCaptureField::SlotY => {
                let distance = self.capture.handle_radius + self.suggestion_offset;
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_handle_slot(1, self.capture.handle_y + self.capture.handle_angle.sin() * distance).map_err(capture_fault_code)?;
                self.capture.handle_field = ArtifactHandleCaptureField::Weight;
            }
            ArtifactHandleCaptureField::Weight => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_handle_weight(1.0).map_err(capture_fault_code)?;
                self.capture.handle_field = ArtifactHandleCaptureField::Publish;
            }
            ArtifactHandleCaptureField::Publish => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.publish_handle().map_err(capture_fault_code)?;
                self.capture.handle += 1;
                self.capture.handle_edge = 0;
                self.capture.handle_connected = false;
                self.capture.handle_field = ArtifactHandleCaptureField::Begin;
            }
        }
        Ok(())
    }

    fn capture_kind_one(&mut self, document: &Value) -> Result<(), &'static str> {
        let kinds = Self::node_kinds(document)?;
        let Some(kind) = kinds.get(self.capture.kind) else {
            if self.capture.kind_field != ArtifactKindCaptureField::Begin {
                return Err("puzzle2d-fill-capture-stale-kind");
            }
            self.capture.stage = ArtifactFillCaptureStage::Rules;
            return Ok(());
        };
        let templates = kind.get("handles").and_then(Value::as_array).ok_or("puzzle2d-fill-capture-kind-handles")?;
        let template = templates.get(self.capture.template);
        match self.capture.kind_field {
            ArtifactKindCaptureField::Begin => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.begin_kind().map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::Id;
            }
            ArtifactKindCaptureField::Id => {
                let value = kind.get("id").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-kind-id")?;
                if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_kind_text_byte(crate::editor::puzzle2d::engine::BoardFillIngressKindText::Id, byte).map_err(capture_fault_code)?;
                    self.capture.byte += 1;
                } else {
                    self.capture.byte = 0;
                    self.capture.kind_field = ArtifactKindCaptureField::Shape;
                }
            }
            ArtifactKindCaptureField::Shape => {
                let rectangle = kind.get("shape").and_then(Value::as_str) == Some("rectangle");
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_kind_rectangle(rectangle).map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::Scale;
            }
            ArtifactKindCaptureField::Scale => {
                self.capture.kind_size = 96.0 * Self::finite(kind.get("scale").and_then(Value::as_f64), 1.0).max(f64::EPSILON);
                self.capture.kind_field = ArtifactKindCaptureField::Radius;
            }
            ArtifactKindCaptureField::Radius => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_kind_radius(self.capture.kind_size * 0.5).map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::Width;
            }
            ArtifactKindCaptureField::Width => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_kind_width(self.capture.kind_size).map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::Height;
            }
            ArtifactKindCaptureField::Height => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_kind_height(self.capture.kind_size).map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::IconBegin;
            }
            ArtifactKindCaptureField::IconBegin => {
                if kind.get("icon").and_then(Value::as_str).is_some() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.begin_kind_icon().map_err(capture_fault_code)?;
                    self.capture.kind_field = ArtifactKindCaptureField::Icon;
                } else {
                    self.capture.kind_field = ArtifactKindCaptureField::Weight;
                }
            }
            ArtifactKindCaptureField::Icon => {
                let value = kind.get("icon").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-kind-icon")?;
                if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_kind_text_byte(crate::editor::puzzle2d::engine::BoardFillIngressKindText::Icon, byte).map_err(capture_fault_code)?;
                    self.capture.byte += 1;
                } else {
                    self.capture.byte = 0;
                    self.capture.kind_field = ArtifactKindCaptureField::Weight;
                }
            }
            ArtifactKindCaptureField::Weight => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_kind_weight(1.0).map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::TemplateBegin;
            }
            ArtifactKindCaptureField::TemplateBegin => {
                if template.is_some() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.begin_kind_handle().map_err(capture_fault_code)?;
                    self.capture.kind_field = ArtifactKindCaptureField::TemplateHandleKind;
                } else {
                    self.capture.kind_field = ArtifactKindCaptureField::Publish;
                }
            }
            ArtifactKindCaptureField::TemplateHandleKind | ArtifactKindCaptureField::TemplateWireKind | ArtifactKindCaptureField::TemplateEdgeKind => {
                let template = template.ok_or("puzzle2d-fill-capture-stale-template")?;
                let (value, field, next) = match self.capture.kind_field {
                    ArtifactKindCaptureField::TemplateHandleKind => {
                        (template.get("handleKind").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-template-kind")?, crate::editor::puzzle2d::engine::BoardFillIngressTemplateText::HandleKind, ArtifactKindCaptureField::TemplateWireKind)
                    }
                    ArtifactKindCaptureField::TemplateWireKind => {
                        (template.get("wireKind").and_then(Value::as_str).unwrap_or("wire.link"), crate::editor::puzzle2d::engine::BoardFillIngressTemplateText::WireKind, ArtifactKindCaptureField::TemplateEdgeKind)
                    }
                    ArtifactKindCaptureField::TemplateEdgeKind => (template.get("edgeKind").and_then(Value::as_str).unwrap_or(""), crate::editor::puzzle2d::engine::BoardFillIngressTemplateText::EdgeKind, ArtifactKindCaptureField::TemplateAngle),
                    _ => return Err("puzzle2d-fill-capture-template-field"),
                };
                if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_kind_handle_text_byte(field, byte).map_err(capture_fault_code)?;
                    self.capture.byte += 1;
                } else {
                    self.capture.byte = 0;
                    self.capture.kind_field = next;
                }
            }
            ArtifactKindCaptureField::TemplateAngle => {
                let template = template.ok_or("puzzle2d-fill-capture-stale-template")?;
                let angle = Self::finite(template.get("angle").and_then(Value::as_f64), 0.0);
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_kind_handle_angle(angle).map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::TemplateRadius;
            }
            ArtifactKindCaptureField::TemplateRadius => {
                let template = template.ok_or("puzzle2d-fill-capture-stale-template")?;
                let radius = template.get("radius").and_then(Value::as_f64).filter(|value| value.is_finite() && *value > 0.0);
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_kind_handle_radius(radius).map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::TemplateWeight;
            }
            ArtifactKindCaptureField::TemplateWeight => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_kind_handle_weight(1.0).map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::TemplatePublish;
            }
            ArtifactKindCaptureField::TemplatePublish => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.publish_kind_handle().map_err(capture_fault_code)?;
                self.capture.template += 1;
                self.capture.kind_field = ArtifactKindCaptureField::TemplateBegin;
            }
            ArtifactKindCaptureField::Publish => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.publish_kind().map_err(capture_fault_code)?;
                self.capture.kind += 1;
                self.capture.template = 0;
                self.capture.kind_field = ArtifactKindCaptureField::Begin;
            }
        }
        Ok(())
    }

    fn capture_rule_one(&mut self, document: &Value) -> Result<(), &'static str> {
        let Some(rules) = Self::rules(document) else {
            if self.capture.rule_field != ArtifactRuleCaptureField::Begin {
                return Err("puzzle2d-fill-capture-stale-rule");
            }
            self.capture.stage = ArtifactFillCaptureStage::Complete;
            return Ok(());
        };
        let Some(rule) = rules.get(self.capture.rule) else {
            if self.capture.rule_field != ArtifactRuleCaptureField::Begin {
                return Err("puzzle2d-fill-capture-stale-rule");
            }
            self.capture.stage = ArtifactFillCaptureStage::Complete;
            return Ok(());
        };
        match self.capture.rule_field {
            ArtifactRuleCaptureField::Begin => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.begin_rule().map_err(capture_fault_code)?;
                self.capture.rule_field = ArtifactRuleCaptureField::Source;
            }
            ArtifactRuleCaptureField::Source | ArtifactRuleCaptureField::Target => {
                let (value, field, next) = match self.capture.rule_field {
                    ArtifactRuleCaptureField::Source => (rule.get("source").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-rule-source")?, crate::editor::puzzle2d::engine::BoardFillIngressRuleText::Source, ArtifactRuleCaptureField::Target),
                    ArtifactRuleCaptureField::Target => {
                        (rule.get("target").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-rule-target")?, crate::editor::puzzle2d::engine::BoardFillIngressRuleText::Target, ArtifactRuleCaptureField::Bidirectional)
                    }
                    _ => return Err("puzzle2d-fill-capture-rule-field"),
                };
                if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_rule_text_byte(field, byte).map_err(capture_fault_code)?;
                    self.capture.byte += 1;
                } else {
                    self.capture.byte = 0;
                    self.capture.rule_field = next;
                }
            }
            ArtifactRuleCaptureField::Bidirectional => {
                let bidirectional = rule.get("bidirectional").and_then(Value::as_bool).unwrap_or(false);
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_rule_bidirectional(bidirectional).map_err(capture_fault_code)?;
                self.capture.rule_field = ArtifactRuleCaptureField::Specificity;
            }
            ArtifactRuleCaptureField::Specificity => {
                let specificity = rule.get("specificity").and_then(Value::as_str).unwrap_or("handle");
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.set_rule_specificity(specificity).map_err(capture_fault_code)?;
                self.capture.rule_field = ArtifactRuleCaptureField::Publish;
            }
            ArtifactRuleCaptureField::Publish => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.publish_rule().map_err(capture_fault_code)?;
                self.capture.rule += 1;
                self.capture.rule_field = ArtifactRuleCaptureField::Begin;
            }
        }
        Ok(())
    }

    fn capture_one(&mut self, document: &Value) -> Result<(), &'static str> {
        match self.capture.stage {
            ArtifactFillCaptureStage::Nodes => self.capture_node_one(document),
            ArtifactFillCaptureStage::Handles => self.capture_handle_one(document),
            ArtifactFillCaptureStage::Kinds => self.capture_kind_one(document),
            ArtifactFillCaptureStage::Rules => self.capture_rule_one(document),
            ArtifactFillCaptureStage::Complete => {
                let snapshot = self.ingress.as_mut().and_then(crate::editor::puzzle2d::engine::BoardFillSnapshotIngress::take_snapshot).ok_or("puzzle2d-fill-capture-snapshot")?;
                self.ingress = None;
                self.search = Some(crate::editor::puzzle2d::engine::BoardFillJob::with_operation(snapshot, self.maximum_count, self.operation));
                Ok(())
            }
        }
    }
}
//#endregion 🔬️Capture

impl crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle2dPlayApp>> for Puzzle2dFillSessionWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    /// 🪪️ The retained operation is the session identity — it replaces the `(app_instance_id,
    /// operation_id, generation)` key the removed process-global slot table used.
    fn bind_operation(&mut self, operation: semio_framework_job::Operation) {
        self.operation = operation;
    }

    fn bind_view_state(&mut self, view_state: Option<semio_framework_plugin::ViewModel>) {
        self.view_state = view_state;
    }

    fn bind_window_owners(&mut self, config: Option<semio_framework_plugin::WindowConfigSnapshot>, _transient: Option<semio_framework_plugin::WindowTransientSnapshot>) {
        self.window_config = window::config_from_snapshot(config.as_ref());
    }

    /// 📐️ A control verb costs its four runtime-transition steps. A search verb declares the exact
    /// per-stage chunk ceilings it is allowed to spend, which is also what
    /// [`Puzzle2dFillSessionWork::step`] enforces — the extent is the budget, not an estimate.
    fn extent(&self, command: &crate::editor::puzzle2d::Puzzle2dCommand, snapshot: &Puzzle2dPlaySnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        if command.action_id() != self.tool_id {
            return None;
        }
        if !is_fill_search_action(self.tool_id) {
            return Some(PUZZLE2D_FILL_CONTROL_CHUNKS);
        }
        if snapshot.0.get("schema").and_then(Value::as_str) != Some(crate::PUZZLE_2D_SCHEMA) {
            return None;
        }
        if requested_fill_count(self.tool_id, command.args()).is_some_and(|count| count > fill::PUZZLE2D_FILL_COUNT_MAX) {
            return None;
        }
        let items = PUZZLE2D_FILL_CAPTURE_CHUNKS.checked_add(PUZZLE2D_FILL_SEARCH_CHUNKS)?.checked_add(PUZZLE2D_FILL_APPLY_CHUNKS)?.checked_add(PUZZLE2D_FILL_CONTROL_CHUNKS)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn step(
        &mut self,
        command: &crate::editor::puzzle2d::Puzzle2dCommand,
        snapshot: &Puzzle2dPlaySnapshot,
        _config: &Puzzle2dConfig,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
    ) -> Result<crate::retained_command::PuzzleCommandWorkStep<EditorApp<Puzzle2dPlayApp>>, Fault> {
        if self.runtime.is_none() {
            self.runtime = Some(Puzzle2dFillRuntime::for_count(self.window_config.fill_count));
        }
        match self.stage {
            Puzzle2dFillStage::Control => {
                let action = self.tool_id;
                let mut runtime = self.runtime.take().ok_or_else(|| Fault::from("puzzle2d-fill-runtime-owner"))?;
                let mut effects = std::mem::take(&mut self.effects);
                let outcome = fill_session_control(action, command.args(), &mut runtime, &mut effects);
                self.runtime = Some(runtime);
                self.effects = effects;
                match outcome {
                    Ok(Some((count, seed))) => {
                        self.begin_search(count, seed)?;
                        if self.stage == Puzzle2dFillStage::Complete {
                            return self.complete();
                        }
                        Ok(Self::progress("puzzle2d-fill-capture", "Capturing board for fill", "Board wird für Füllung erfasst"))
                    }
                    Ok(None) => self.complete(),
                    Err(code) => {
                        self.fault(code)?;
                        self.complete()
                    }
                }
            }
            Puzzle2dFillStage::Capture => {
                if self.capture_chunks >= PUZZLE2D_FILL_CAPTURE_CHUNKS {
                    self.fault("puzzle2d-fill-capture-budget")?;
                    return self.complete();
                }
                self.capture_chunks = self.capture_chunks.saturating_add(1);
                for _ in 0..PUZZLE2D_FILL_CAPTURE_UNITS_PER_STEP {
                    if self.search.is_some() {
                        break;
                    }
                    if let Err(code) = self.capture_one(&snapshot.0) {
                        self.fault(code)?;
                        return self.complete();
                    }
                }
                if self.search.is_some() {
                    self.runtime_mut()?.fill_job_lifecycle = Puzzle2dFillLifecycle::Queued;
                    self.stage = Puzzle2dFillStage::Search;
                }
                Ok(Self::progress("puzzle2d-fill-capture", "Capturing board for fill", "Board wird für Füllung erfasst"))
            }
            Puzzle2dFillStage::Search => {
                if self.search_chunks >= PUZZLE2D_FILL_SEARCH_CHUNKS {
                    self.runtime_mut()?.fill_job_lifecycle = Puzzle2dFillLifecycle::CheckpointReady;
                    return self.complete();
                }
                self.search_chunks = self.search_chunks.saturating_add(1);
                for _ in 0..PUZZLE2D_FILL_SEARCH_UNITS_PER_STEP {
                    if self.stage != Puzzle2dFillStage::Search {
                        break;
                    }
                    let Some(outcome) = self.search_step_one() else {
                        self.fault("puzzle2d-fill-search-owner")?;
                        break;
                    };
                    self.absorb_outcome(outcome)?;
                }
                if self.stage == Puzzle2dFillStage::Complete {
                    return self.complete();
                }
                Ok(Self::progress("puzzle2d-fill-search", "Searching a fill placement", "Füllplatzierung wird gesucht"))
            }
            Puzzle2dFillStage::Apply => {
                if self.apply_chunks >= PUZZLE2D_FILL_APPLY_CHUNKS {
                    self.fault("puzzle2d-fill-apply-budget")?;
                    return self.complete();
                }
                self.apply_chunks = self.apply_chunks.saturating_add(1);
                for _ in 0..PUZZLE2D_FILL_APPLY_UNITS_PER_STEP {
                    if self.stage != Puzzle2dFillStage::Apply {
                        break;
                    }
                    self.apply_one()?;
                }
                if self.stage == Puzzle2dFillStage::Complete {
                    return self.complete();
                }
                Ok(Self::progress("puzzle2d-fill-apply", "Applying a fill placement", "Füllplatzierung wird angewendet"))
            }
            Puzzle2dFillStage::Complete => Err(Fault::from("puzzle2d-fill-complete-repolled")),
            Puzzle2dFillStage::Closing => Err(Fault::from("puzzle2d-fill-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle2dFillStage::Closing;
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.close_one() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.session_is_empty()
    }
}

//#region 🎛️Control
/// 🔢️ The count a search verb asks for, when the verb carries one in its arguments.
fn requested_fill_count(action: &str, args: Option<&Value>) -> Option<u32> {
    match action {
        "setFillCount" => {
            let value = args.and_then(|args| args.get("count").or_else(|| args.get("value"))).and_then(Value::as_f64)?;
            if !value.is_finite() || value < 0.0 {
                return None;
            }
            u32::try_from(value.round() as i64).ok()
        }
        "brushFillSessionBegin" => args.and_then(|args| args.get("maxCount")).and_then(Value::as_u64).and_then(|count| u32::try_from(count).ok()),
        _ => None,
    }
}

/// 🧹️ Returns the fill runtime to its idle shape, keeping the requested count.
fn discard_runtime(runtime: &mut Puzzle2dFillRuntime) {
    runtime.fill_job_operation = 0;
    runtime.fill_job_generation = 0;
    runtime.fill_job_seed = 0;
    runtime.fill_job_base_revision = 0;
    runtime.fill_job_checkpoint_sequence = 0;
    runtime.fill_job_accepted_count = 0;
    runtime.fill_job_search_count = 0;
    runtime.fill_job_stage = Puzzle2dFillText::default();
    runtime.fill_job_fault_code = None;
    runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Discarded;
}

/// 🧰️ Cancels whatever fill the runtime describes when the operator leaves the fill utility. There
/// is no live session to reach any more: discarding the
/// runtime IS discarding the session, because the runtime is the only thing a later verb resumes
/// from.
pub fn discard_fill_session(runtime: &mut Puzzle2dFillRuntime) {
    discard_runtime(runtime);
}

/// 🎛️ The runtime transition every fill verb performs. Returns `Some((count, seed))` when the verb
/// must additionally open or resume a search, `None` when the transition is the whole verb.
///
/// `brushFillSessionStep` is a real resumption, not a poll: the placements a previous session
/// accepted are already committed to the document, so the remaining count plus the stored seed is
/// the entire continuation state.
fn fill_session_control(action: &str, args: Option<&Value>, runtime: &mut Puzzle2dFillRuntime, effects: &mut Vec<Effect>) -> Result<Option<(u32, u64)>, &'static str> {
    match action {
        "setFillCount" => {
            let Some(value) = args.and_then(|args| args.get("count").or_else(|| args.get("value"))).and_then(Value::as_f64) else {
                return Err("puzzle2d-fill-count");
            };
            if !value.is_finite() || value < 0.0 || value.round() > f64::from(fill::PUZZLE2D_FILL_COUNT_MAX) {
                return Err("puzzle2d-fill-count-capacity");
            }
            let count = value.round() as u32;
            runtime.fill_count = count;
            effects.push(Effect::SetActiveTool { tool_id: fill::TOOL_ID.into() });
            Ok(Some((count, 1)))
        }
        "brushFillSessionBegin" => crate::editor::puzzle2d::commands::fill_session_begin::fill_session_begin(args, runtime),
        "brushFillSessionRetry" => Ok(Some((runtime.fill_count, runtime.fill_job_seed.max(1)))),
        "brushFillSessionStep" => crate::editor::puzzle2d::commands::fill_session_step::fill_session_step(runtime),
        "brushFillSessionAdopt" => {
            runtime.fill_job_lifecycle = if runtime.fill_job_fault_code.is_some() { Puzzle2dFillLifecycle::Faulted } else { Puzzle2dFillLifecycle::Completed };
            Ok(None)
        }
        "brushFillSessionCancel" => {
            runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Cancelled;
            runtime.fill_job_fault_code = None;
            Ok(None)
        }
        "brushFillSessionDiscard" => {
            discard_runtime(runtime);
            Ok(None)
        }
        "brushFillSessionClear" => {
            crate::editor::puzzle2d::commands::fill_session_clear::fill_session_clear(runtime);
            Ok(None)
        }
        _ => Err("puzzle2d-fill-action-unmapped"),
    }
}
//#endregion 🎛️Control

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
