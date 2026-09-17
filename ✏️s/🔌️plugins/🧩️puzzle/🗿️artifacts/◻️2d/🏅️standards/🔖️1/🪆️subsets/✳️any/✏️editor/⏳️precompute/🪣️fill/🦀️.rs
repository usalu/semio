//! 🪣️ Puzzle 2d fill tool run (`📋️tool-run-contract.md` §2.7, §3.2, §3.3, §3.7): the run job streams the
//! document into the board engine's fill ingress, drives its `BoardFillJob` search and reports every
//! candidate the search decides as a `placement2d` trace record — `testing`, then `danger` (collision),
//! `warning` (port or kind rule) or `success` (placed). Every placement appends its `create_node` +
//! `connect_handles` ops and one entity to the tool run ledger; nothing touches the store before
//! finalize, where the revalidate job re-tests the provisional placements against the head.
//! Vocabulary source of record: `$defs.Puzzle2dFillRun` in `✳️any/🧬️schema/🔣️.json`.

use crate::editor::puzzle2d::engine::{
    BoardFillCandidateEvent, BoardFillCandidateVerdict, BoardFillCaptureFault, BoardFillIngressHandleText, BoardFillIngressKindText, BoardFillIngressRuleText, BoardFillIngressTemplateText, BoardFillJob, BoardFillPlacement, BoardFillSnapshot, BoardFillSnapshotIngress, BoardFillStage,
};
use crate::standards::v1::subsets::any::schema::mutations::text::{Puzzle2dMutation, Puzzle2dPlaySnapshot};
use crate::standards::v1::subsets::any::schema::mutations::{connect_handles, create_node};
use semio_framework_job::{Checkpoint, CommitCandidate, Generation, InteractiveJob, InteractiveJobCloseStep, JobFault, JobPayloadStream, Operation, OperationId, RetainedJobPayload, RevisionId, StepBudget, StepContext, StepOutcome, JOB_PAYLOAD_PAGE_BYTES};
use semio_framework_tool_run::{ToolRunCounter, ToolRunIdentity, ToolRunProgress, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTickWriter, ToolRunTraceSubject, ToolRunVerdict, TOOL_RUN_REASON_CONFLICT};
use serde_json::Value;
use std::collections::HashSet;
use std::sync::Arc;

//#region 🔖️Limits
/// 🚰️ Estimated pending tick bytes past which a run job flushes, so one tick stays within one job payload page.
pub(crate) const FILL_RUN_TICK_FLUSH_BYTES: usize = 8 * 1024;
/// 🧬️ Provisional ops one placement appends: `create_node` then `connect_handles`.
pub(crate) const FILL_RUN_OPS_PER_PLACEMENT: usize = 2;
/// 🎯️ Target regions one run reads as its placement constraint. A board is painted by hand, so this
/// admits far more than any authored document holds while keeping the run's own owner fixed.
pub(crate) const FILL_RUN_TARGET_REGION_SLOTS: usize = 256;
/// 🆔️ Prefix of the node ids the board engine mints for fill placements (`puzzle2d.fill.<serial>`).
pub(crate) const FILL_RUN_NODE_ID_PREFIX: &str = "puzzle2d.fill.";
const FILL_RUN_CAPTURE_UNITS: usize = 256;
const FILL_RUN_SEARCH_BATCH: usize = 64;
const FILL_RUN_DEADLINE_CHECK: usize = 16;
//#endregion 🔖️Limits

//#region 🔖️Vocabulary
/// 🧭️ Stage of a 2d fill run, the index into `ToolRunDefinition.stages`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FillRunStage {
    Capture,
    Search,
    Test,
    Place,
    Retract,
}

impl FillRunStage {
    pub const ALL: [Self; 5] = [Self::Capture, Self::Search, Self::Test, Self::Place, Self::Retract];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Capture => "capture",
            Self::Search => "search",
            Self::Test => "test",
            Self::Place => "place",
            Self::Retract => "retract",
        }
    }

    /// 🏷️ The terminology field carrying this stage's caption.
    pub fn label(self) -> fn(&crate::editor::puzzle2d::terminology::Puzzle2dLabels) -> semio_framework_plugin::LabelText {
        match self {
            Self::Capture => |labels| labels.fill_stage_capture,
            Self::Search => |labels| labels.fill_stage_search,
            Self::Test => |labels| labels.fill_stage_test,
            Self::Place => |labels| labels.fill_stage_place,
            Self::Retract => |labels| labels.fill_stage_retract,
        }
    }

    /// 🔭️ The run stage an engine search stage belongs to.
    fn of(stage: BoardFillStage) -> Self {
        match stage {
            BoardFillStage::ConstructPreview | BoardFillStage::ScanHostCollision | BoardFillStage::ScanVirtualCollision => Self::Test,
            BoardFillStage::AcceptCandidate
            | BoardFillStage::AcceptNodeId
            | BoardFillStage::AcceptEdgeId
            | BoardFillStage::AcceptEdgeKind
            | BoardFillStage::AcceptNodeKind
            | BoardFillStage::AcceptSourceHandle
            | BoardFillStage::AcceptTargetHandle
            | BoardFillStage::AcceptIcon
            | BoardFillStage::AcceptVirtualNode
            | BoardFillStage::AcceptSourceConnection
            | BoardFillStage::AcceptHandles
            | BoardFillStage::AcceptHandleId
            | BoardFillStage::AcceptHandleVirtual
            | BoardFillStage::AcceptHandlePublish
            | BoardFillStage::PublishPlanPrefix => Self::Place,
            _ => Self::Search,
        }
    }
}

/// 🔢️ Counter of a 2d fill run, the index into `ToolRunDefinition.counters`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FillRunCounter {
    Tested,
    Accepted,
    Collisions,
    Rejected,
}

impl FillRunCounter {
    pub const ALL: [Self; 4] = [Self::Tested, Self::Accepted, Self::Collisions, Self::Rejected];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Tested => "tested",
            Self::Accepted => "accepted",
            Self::Collisions => "collisions",
            Self::Rejected => "rejected",
        }
    }

    pub fn label(self) -> fn(&crate::editor::puzzle2d::terminology::Puzzle2dLabels) -> semio_framework_plugin::LabelText {
        match self {
            Self::Tested => |labels| labels.fill_counter_tested,
            Self::Accepted => |labels| labels.fill_counter_accepted,
            Self::Collisions => |labels| labels.fill_counter_collisions,
            Self::Rejected => |labels| labels.fill_counter_rejected,
        }
    }
}

/// 🏷️ Reason code of a 2d fill trace record or step.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FillRunReason {
    Fits,
    HostCollision,
    VirtualCollision,
    PortIncompatible,
    KindIncompatible,
    NoOpenHandle,
    NoCompatibleKind,
    NoFreePlacement,
    ArtifactCapacity,
    RequestedReached,
    Retracted,
    OutsideTargetRegion,
}

impl FillRunReason {
    pub const ALL: [Self; 12] = [
        Self::Fits,
        Self::HostCollision,
        Self::VirtualCollision,
        Self::PortIncompatible,
        Self::KindIncompatible,
        Self::NoOpenHandle,
        Self::NoCompatibleKind,
        Self::NoFreePlacement,
        Self::ArtifactCapacity,
        Self::RequestedReached,
        Self::Retracted,
        Self::OutsideTargetRegion,
    ];

    pub fn code(self) -> u16 {
        self as u16
    }

    pub fn from_code(code: u16) -> Option<Self> {
        Self::ALL.get(usize::from(code)).copied()
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Fits => "fits",
            Self::HostCollision => "host-collision",
            Self::VirtualCollision => "virtual-collision",
            Self::PortIncompatible => "port-incompatible",
            Self::KindIncompatible => "kind-incompatible",
            Self::NoOpenHandle => "no-open-handle",
            Self::NoCompatibleKind => "no-compatible-kind",
            Self::NoFreePlacement => "no-free-placement",
            Self::ArtifactCapacity => "artifact-capacity",
            Self::RequestedReached => "requested-reached",
            Self::Retracted => "retracted",
            Self::OutsideTargetRegion => "outside-target-region",
        }
    }

    /// 🚥️ A fit and a reached request succeed, a collision is danger, a retraction informs, every rule and stall warns.
    pub fn verdict(self) -> ToolRunVerdict {
        match self {
            Self::Fits | Self::RequestedReached => ToolRunVerdict::Success,
            Self::HostCollision | Self::VirtualCollision => ToolRunVerdict::Danger,
            Self::Retracted => ToolRunVerdict::Testing,
            _ => ToolRunVerdict::Warning,
        }
    }

    pub fn label(self) -> fn(&crate::editor::puzzle2d::terminology::Puzzle2dLabels) -> semio_framework_plugin::LabelText {
        match self {
            Self::Fits => |labels| labels.fill_reason_fits,
            Self::HostCollision => |labels| labels.fill_reason_host_collision,
            Self::VirtualCollision => |labels| labels.fill_reason_virtual_collision,
            Self::PortIncompatible => |labels| labels.fill_reason_port_incompatible,
            Self::KindIncompatible => |labels| labels.fill_reason_kind_incompatible,
            Self::NoOpenHandle => |labels| labels.fill_reason_no_open_handle,
            Self::NoCompatibleKind => |labels| labels.fill_reason_no_compatible_kind,
            Self::NoFreePlacement => |labels| labels.fill_reason_no_free_placement,
            Self::ArtifactCapacity => |labels| labels.fill_reason_artifact_capacity,
            Self::RequestedReached => |labels| labels.fill_reason_requested_reached,
            Self::Retracted => |labels| labels.fill_reason_retracted,
            Self::OutsideTargetRegion => |labels| labels.fill_reason_outside_target_region,
        }
    }

    /// 🔎️ The reason an engine candidate verdict names; `Testing` names none.
    fn of_candidate(verdict: BoardFillCandidateVerdict) -> Option<Self> {
        match verdict {
            BoardFillCandidateVerdict::Testing => None,
            BoardFillCandidateVerdict::HostCollision => Some(Self::HostCollision),
            BoardFillCandidateVerdict::VirtualCollision => Some(Self::VirtualCollision),
            BoardFillCandidateVerdict::PortIncompatible => Some(Self::PortIncompatible),
            BoardFillCandidateVerdict::RuleIncompatible => Some(Self::KindIncompatible),
            BoardFillCandidateVerdict::Fits => Some(Self::Fits),
        }
    }
}

/// 🔑️ One provisional placement of a run: its trace key and its kind (the `placement2d` shape index).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FillRunPlacementKey {
    pub key: u64,
    pub shape: u32,
}

/// 📸️ Resume point a run reports through `StepOutcome::CheckpointReady`, little-endian:
/// `requested u64 | tested u64 | collisions u64 | rejected u64 | nextKey u64 | placements u32 | (key u64, shape u32)*`.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct FillRunCheckpoint {
    pub requested: u64,
    pub tested: u64,
    pub collisions: u64,
    pub rejected: u64,
    pub next_key: u64,
    pub placements: Vec<FillRunPlacementKey>,
}

impl FillRunCheckpoint {
    pub const HEADER_BYTES: usize = 44;
    pub const PLACEMENT_BYTES: usize = 12;

    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::HEADER_BYTES + Self::PLACEMENT_BYTES * self.placements.len());
        for value in [self.requested, self.tested, self.collisions, self.rejected, self.next_key] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes.extend_from_slice(&(self.placements.len() as u32).to_le_bytes());
        for placement in &self.placements {
            bytes.extend_from_slice(&placement.key.to_le_bytes());
            bytes.extend_from_slice(&placement.shape.to_le_bytes());
        }
        bytes
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let u64_at = |at: usize| bytes.get(at..at + 8).map(|slice| u64::from_le_bytes(slice.try_into().expect("eight bytes")));
        let u32_at = |at: usize| bytes.get(at..at + 4).map(|slice| u32::from_le_bytes(slice.try_into().expect("four bytes")));
        let count = u32_at(40)? as usize;
        if bytes.len() != Self::HEADER_BYTES + Self::PLACEMENT_BYTES * count {
            return None;
        }
        let placements = (0..count).map(|index| Self::HEADER_BYTES + Self::PLACEMENT_BYTES * index).map(|at| Some(FillRunPlacementKey { key: u64_at(at)?, shape: u32_at(at + 8)? })).collect::<Option<Vec<_>>>()?;
        Some(Self { requested: u64_at(0)?, tested: u64_at(8)?, collisions: u64_at(16)?, rejected: u64_at(24)?, next_key: u64_at(32)?, placements })
    }
}
//#endregion 🔖️Vocabulary

//#region 🧱️Placement
/// 🆔️ Stable provisional entity of a placed node: the first eight little-endian bytes of its id digest.
pub(crate) fn fill_run_entity(node_id: &str) -> u64 {
    u64::from_le_bytes(semio_framework_hash::hash(node_id.as_bytes()).as_bytes()[..8].try_into().expect("eight digest bytes"))
}

/// 🗂️ The node kind rows a fill places: the document's own `meta.kindCatalogs.nodes`, else the engine catalog
/// rows of its manifest (`board_kind_catalogs_json`), else the kinds the placed nodes imply
/// (`inferred_node_kind_rows` — Concrete Forest names no manifest and carries no catalog, and a fill
/// that finds no kind places nothing).
fn fill_kind_rows(document: &Value) -> Vec<Value> {
    if let Some(rows) = crate::editor::puzzle2d::kind_catalog_entries(document, "nodes").filter(|rows| !rows.is_empty()) {
        return rows.to_vec();
    }
    let manifest_rows = crate::editor::puzzle2d::board_kind_catalogs_json(document).and_then(|json| serde_json::from_str::<Value>(&json).ok()).and_then(|catalogs| catalogs.get("nodeKinds").and_then(Value::as_array).cloned()).unwrap_or_default();
    if manifest_rows.iter().any(|row| row.get("handles").and_then(Value::as_array).is_some_and(|templates| !templates.is_empty())) {
        return manifest_rows;
    }
    let inferred = crate::editor::puzzle2d::inferred_node_kind_rows(document);
    if inferred.is_empty() { manifest_rows } else { inferred }
}

/// 🔢️ The first engine serial a run may mint without reusing a fill node id `id` already holds.
fn fill_serial_after(id: &str) -> u64 {
    id.strip_prefix(FILL_RUN_NODE_ID_PREFIX).and_then(|serial| serial.parse::<u64>().ok()).map_or(1, |serial| serial.saturating_add(1))
}

/// 🧬️ The `create_node` and `connect_handles` document mutations one engine placement contributes.
fn fill_placement_mutations(placement: &BoardFillPlacement) -> Result<[Puzzle2dMutation; 2], &'static str> {
    let rectangle = placement.shape == "rectangle";
    let handles = (0..placement.handle_count())
        .map(|index| {
            let (id, handle_kind, angle, radius) = placement.handle(index).ok_or("puzzle2d-fill-apply-handle")?;
            Ok(crate::Puzzle2dHandle { id: id.to_string(), handle_kind: Some(handle_kind.to_string()), angle, radius, ..Default::default() })
        })
        .collect::<Result<Vec<_>, &'static str>>()?;
    let node = crate::Puzzle2dNode {
        id: placement.node_id.as_str().to_string(),
        node_kind: Some(placement.node_kind.as_str().to_string()),
        shape: Some(placement.shape.to_string()),
        x: placement.x,
        y: placement.y,
        radius: (!rectangle).then_some(placement.radius),
        width: rectangle.then_some(placement.width),
        height: rectangle.then_some(placement.height),
        text: Some(placement.node_id.as_str().to_string()),
        icon_kind: placement.icon_kind.as_ref().map(|icon| icon.as_str().to_string()),
        anchor: crate::Puzzle2dNodeAnchor::Fixed,
        handles,
        ..Default::default()
    };
    let edge = connect_handles(placement.edge_id.as_str().to_string(), placement.source_handle_id.as_str().to_string(), placement.target_handle_id.as_str().to_string(), Some(placement.edge_kind.as_str().to_string()), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None);
    Ok([create_node(node, None), edge])
}

/// 🎯️ The normalized bounds of every visible target region the document declares, capped at
/// [`FILL_RUN_TARGET_REGION_SLOTS`] — a document painted past that ceiling constrains fill by its
/// first regions rather than faulting the run, and the excess is visible in the board itself.
pub(crate) fn fill_visible_region_bounds(fixture: &Value) -> Vec<[f64; 4]> {
    crate::editor::puzzle2d::fixture_target_regions(fixture)
        .iter()
        .filter(|region| region.get("hidden").and_then(Value::as_bool) != Some(true))
        .take(FILL_RUN_TARGET_REGION_SLOTS)
        .map(crate::editor::puzzle2d::puzzle2d_region_bounds)
        .collect()
}

/// 🎯️ Whether a placement's own footprint lies fully inside ANY visible target region. An empty set
/// is unconstrained, so a board with no region fills exactly as it did before regions existed.
pub(crate) fn fill_regions_admit(regions: &[[f64; 4]], bounds: [f64; 4]) -> bool {
    regions.is_empty() || regions.iter().any(|region| bounds[0] >= region[0] && bounds[1] >= region[1] && bounds[2] <= region[2] && bounds[3] <= region[3])
}

/// 📦️ Axis-aligned collision footprint `[min_x, min_y, max_x, max_y]` of a board node, exactly as the fill
/// capture feeds it to the engine: a circle spans `radius · scale`, a rectangle half its scaled extents.
pub(crate) fn fill_node_bounds(x: f64, y: f64, scale: Option<f64>, rectangle: bool, radius_or_width: Option<f64>, height: Option<f64>) -> [f64; 4] {
    let finite = |value: Option<f64>| value.filter(|value| value.is_finite()).unwrap_or(1.0).max(f64::EPSILON);
    let scale = finite(scale);
    let (half_x, half_y) = if rectangle { (finite(radius_or_width) * scale * 0.5, finite(height) * scale * 0.5) } else { (finite(radius_or_width) * scale, finite(radius_or_width) * scale) };
    [x - half_x, y - half_y, x + half_x, y + half_y]
}

/// 🚧️ The AABB collision test every fill placement is decided by, widened by `slack` on each side.
/// `slack` is the net of the two placement-tuning settings (`contactTolerance - overlapBudget`,
/// `Puzzle2dConfig`): a positive contact tolerance GROWS both footprints so a placement that merely
/// grazes a neighbour is refused, a positive overlap budget SHRINKS them so a dense board can be
/// packed deliberately. `slack` is clamped so a budget can never shrink a footprint past its centre.
fn fill_bounds_overlap_with(left: [f64; 4], right: [f64; 4], slack: f64) -> bool {
    let inset = |bounds: [f64; 4]| {
        let half_x = (bounds[2] - bounds[0]) * 0.5;
        let half_y = (bounds[3] - bounds[1]) * 0.5;
        let dx = slack.max(-half_x);
        let dy = slack.max(-half_y);
        [bounds[0] - dx, bounds[1] - dy, bounds[2] + dx, bounds[3] + dy]
    };
    let (left, right) = (inset(left), inset(right));
    left[0] <= right[2] && left[2] >= right[0] && left[1] <= right[3] && left[3] >= right[1]
}

fn fill_bounds_overlap(left: [f64; 4], right: [f64; 4]) -> bool {
    fill_bounds_overlap_with(left, right, 0.0)
}

fn fill_run_fault(context: &mut StepContext<'_>, code: &str) -> StepOutcome {
    match context.payload_from_bytes(JobPayloadStream::Fault, code.as_bytes()) {
        Ok(detail) => StepOutcome::Fault(JobFault { detail }),
        Err(rejected) => {
            drop(rejected.into_source());
            StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) })
        }
    }
}

fn close_outcome(outcome: &mut StepOutcome) {
    while !outcome.terminal_is_empty() {
        let _ = outcome.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
    }
}

fn close_job_slot(slot: &mut Option<BoardFillJob>) {
    let Some(job) = slot.as_mut() else { return };
    InteractiveJob::begin_close(job);
    if matches!(InteractiveJob::close_step(job, 1, JOB_PAYLOAD_PAGE_BYTES), InteractiveJobCloseStep::Complete) && InteractiveJob::terminal_is_empty(job) {
        *slot = None;
    }
}

/// ⏱️ The engine search runs under a fuel-only clock: the run job enforces the wall deadline between batches.
fn fill_run_monotonic_zero() -> Option<u64> {
    Some(0)
}
//#endregion 🧱️Placement

//#region 🔬️CaptureCursor
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

fn capture_fault_code(fault: BoardFillCaptureFault) -> &'static str {
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

/// 🔬️ The staged document capture of one run: the field cursor, the engine ingress it feeds, the suggestion
/// offset every open handle's slot is pushed out by, the first engine serial past the document's fill ids and the
/// number of open handles it streamed.
struct FillCapture {
    capture: ArtifactFillCaptureCursor,
    ingress: Option<BoardFillSnapshotIngress>,
    suggestion_offset: f64,
    serial: u64,
    open_handles: u64,
}

impl FillCapture {
    fn new(suggestion_offset: f64) -> Self {
        Self { capture: ArtifactFillCaptureCursor::new(), ingress: Some(BoardFillSnapshotIngress::new(suggestion_offset)), suggestion_offset, serial: 1, open_handles: 0 }
    }

    /// 🔬️ Up to `units` capture units; the captured snapshot once the document is fully streamed.
    fn advance(&mut self, document: &Value, kinds: &[Value], units: usize) -> Result<Option<BoardFillSnapshot>, &'static str> {
        for _ in 0..units {
            if let Some(snapshot) = self.capture_one(document, kinds)? {
                return Ok(Some(snapshot));
            }
        }
        Ok(None)
    }

    /// 🚰️ One close unit of the ingress; `true` once nothing is left.
    fn close_one(&mut self) -> bool {
        let Some(ingress) = self.ingress.as_mut() else { return true };
        ingress.begin_close();
        if matches!(ingress.close_step(1, JOB_PAYLOAD_PAGE_BYTES), InteractiveJobCloseStep::Complete) && ingress.terminal_is_empty() {
            self.ingress = None;
        }
        self.ingress.is_none()
    }
}

//#region 🔬️Capture
/// 🔬️ Streams the document into the engine's fixed-capacity fill ingress one field — and, for text,
/// one byte — per call, so a capture never allocates and never outruns its step budget.
impl FillCapture {
    fn nodes(document: &Value) -> Result<&[Value], &'static str> {
        document.get("nodes").and_then(Value::as_array).map(Vec::as_slice).ok_or("puzzle2d-fill-capture-nodes")
    }

    fn edges(document: &Value) -> Result<&[Value], &'static str> {
        document.get("edges").and_then(Value::as_array).map(Vec::as_slice).ok_or("puzzle2d-fill-capture-edges")
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
                    self.serial = self.serial.max(fill_serial_after(value));
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
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_handle_text_byte(BoardFillIngressHandleText::Id, byte).map_err(capture_fault_code)?;
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
                    ArtifactHandleCaptureField::NodeKind => (node.get("nodeKind").and_then(Value::as_str).unwrap_or(""), BoardFillIngressHandleText::NodeKind, ArtifactHandleCaptureField::HandleKind),
                    ArtifactHandleCaptureField::HandleKind => (handle.get("handleKind").and_then(Value::as_str).unwrap_or("port"), BoardFillIngressHandleText::HandleKind, ArtifactHandleCaptureField::WireKind),
                    ArtifactHandleCaptureField::WireKind => (handle.get("wireKind").and_then(Value::as_str).unwrap_or("wire.link"), BoardFillIngressHandleText::WireKind, ArtifactHandleCaptureField::EdgeKind),
                    ArtifactHandleCaptureField::EdgeKind => (handle.get("edgeKind").and_then(Value::as_str).unwrap_or(""), BoardFillIngressHandleText::EdgeKind, ArtifactHandleCaptureField::X),
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
                self.open_handles += u64::from(self.capture.handle_node_visible && self.capture.handle_visible && !self.capture.handle_connected);
                self.capture.handle += 1;
                self.capture.handle_edge = 0;
                self.capture.handle_connected = false;
                self.capture.handle_field = ArtifactHandleCaptureField::Begin;
            }
        }
        Ok(())
    }

    fn capture_kind_one(&mut self, kinds: &[Value]) -> Result<(), &'static str> {
        let Some(kind) = kinds.get(self.capture.kind) else {
            if self.capture.kind_field != ArtifactKindCaptureField::Begin {
                return Err("puzzle2d-fill-capture-stale-kind");
            }
            self.capture.stage = ArtifactFillCaptureStage::Rules;
            return Ok(());
        };
        let templates = kind.get("handles").and_then(Value::as_array).map_or(&[][..], Vec::as_slice);
        let template = templates.get(self.capture.template);
        match self.capture.kind_field {
            ArtifactKindCaptureField::Begin => {
                self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.begin_kind().map_err(capture_fault_code)?;
                self.capture.kind_field = ArtifactKindCaptureField::Id;
            }
            ArtifactKindCaptureField::Id => {
                let value = kind.get("id").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-kind-id")?;
                if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_kind_text_byte(BoardFillIngressKindText::Id, byte).map_err(capture_fault_code)?;
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
                    self.ingress.as_mut().ok_or("puzzle2d-fill-capture-ingress")?.push_kind_text_byte(BoardFillIngressKindText::Icon, byte).map_err(capture_fault_code)?;
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
                        (template.get("handleKind").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-template-kind")?, BoardFillIngressTemplateText::HandleKind, ArtifactKindCaptureField::TemplateWireKind)
                    }
                    ArtifactKindCaptureField::TemplateWireKind => {
                        (template.get("wireKind").and_then(Value::as_str).unwrap_or("wire.link"), BoardFillIngressTemplateText::WireKind, ArtifactKindCaptureField::TemplateEdgeKind)
                    }
                    ArtifactKindCaptureField::TemplateEdgeKind => (template.get("edgeKind").and_then(Value::as_str).unwrap_or(""), BoardFillIngressTemplateText::EdgeKind, ArtifactKindCaptureField::TemplateAngle),
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
                    ArtifactRuleCaptureField::Source => (rule.get("source").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-rule-source")?, BoardFillIngressRuleText::Source, ArtifactRuleCaptureField::Target),
                    ArtifactRuleCaptureField::Target => {
                        (rule.get("target").and_then(Value::as_str).ok_or("puzzle2d-fill-capture-rule-target")?, BoardFillIngressRuleText::Target, ArtifactRuleCaptureField::Bidirectional)
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

    fn capture_one(&mut self, document: &Value, kinds: &[Value]) -> Result<Option<BoardFillSnapshot>, &'static str> {
        match self.capture.stage {
            ArtifactFillCaptureStage::Nodes => self.capture_node_one(document).map(|()| None),
            ArtifactFillCaptureStage::Handles => self.capture_handle_one(document).map(|()| None),
            ArtifactFillCaptureStage::Kinds => self.capture_kind_one(kinds).map(|()| None),
            ArtifactFillCaptureStage::Rules => self.capture_rule_one(document).map(|()| None),
            ArtifactFillCaptureStage::Complete => self.ingress.as_mut().and_then(BoardFillSnapshotIngress::take_snapshot).map(Some).ok_or("puzzle2d-fill-capture-snapshot"),
        }
    }
}
//#endregion 🔬️Capture

//#region ⏯️RunJob
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FillRunOwed {
    Checkpoint,
    Complete,
}

/// 🎯️ The candidate the search is testing right now.
#[derive(Clone, Copy, Debug)]
struct FillRunLive {
    key: u64,
    subject: ToolRunTraceSubject,
}

/// ⏩️ A resumed run silently replays the deterministic search up to the provisional placements it continues.
struct FillRunReplay {
    expected: Vec<Vec<u8>>,
    retire: Vec<FillRunPlacementKey>,
    target: usize,
}

/// ⏯️ The puzzle 2d fill tool run job (`ToolRunDefinition.runJob`). One unit of fuel is one final candidate
/// verdict; ticks travel only in `PreviewReady` pages; `CheckpointReady` follows the tick of every placement
/// and of the completion, `Complete` follows the completion checkpoint. A job built from a checkpoint with
/// a new requested count replays the same search (seeded by the run id, serial after the document's fill
/// ids) without trace or ops, keeps every provisional placement it re-derives byte-identically, retracts
/// the rest and continues live — so a raised count continues and a lowered one retracts the tail.
pub(crate) struct Puzzle2dFillRunJob {
    document: Arc<Puzzle2dPlaySnapshot>,
    kinds: Vec<Value>,
    open_handles: u64,
    writer: ToolRunTickWriter,
    requested: u64,
    seed: u64,
    capture: Option<FillCapture>,
    search: Option<BoardFillJob>,
    closing_search: Option<BoardFillJob>,
    search_sequence: u64,
    replay: Option<FillRunReplay>,
    live: Option<FillRunLive>,
    next_key: u64,
    tested: u64,
    collisions: u64,
    rejected: u64,
    decided_since_placement: u64,
    placements: Vec<FillRunPlacementKey>,
    stage: FillRunStage,
    stall: Option<FillRunReason>,
    finished: bool,
    settled: bool,
    owed: Option<FillRunOwed>,
    progress_sequence: u64,
    closing: bool,
    /// 🎯️ Normalized `[min_x, min_y, max_x, max_y]` of every VISIBLE target region the run's base
    /// document declared, read once at construction. Empty means unconstrained — the same rule
    /// puzzle3d states as `world_volumes_contain_aabb`.
    target_regions: Vec<[f64; 4]>,
}

impl Puzzle2dFillRunJob {
    /// 🎬️ A run over `document` toward `requested` placements. `checkpoint` and `provisional` are the
    /// ledger's resume state (both empty for a fresh run).
    pub(crate) fn new(identity: ToolRunIdentity, document: Arc<Puzzle2dPlaySnapshot>, suggestion_offset: f64, requested: u32, checkpoint: Option<&[u8]>, provisional: &[Puzzle2dMutation]) -> Result<Self, &'static str> {
        let expected = provisional.iter().map(|op| protocol::OpBinary::encode_op(op).map_err(|_| "puzzle2d-fill-run-op-encode")).collect::<Result<Vec<_>, _>>()?;
        let retire = match checkpoint {
            Some(bytes) => FillRunCheckpoint::decode(bytes).ok_or("puzzle2d-fill-run-checkpoint")?.placements,
            None => Vec::new(),
        };
        let provisional_placements = expected.len() / FILL_RUN_OPS_PER_PLACEMENT;
        let kinds = fill_kind_rows(&document.0);
        let target_regions = fill_visible_region_bounds(&document.0);
        let mut job = Self {
            kinds,
            open_handles: 0,
            document,
            writer: ToolRunTickWriter::with_provisional_base(identity, expected.len() as u32),
            requested: u64::from(requested),
            seed: identity.id.run.max(1),
            capture: Some(FillCapture::new(suggestion_offset)),
            search: None,
            closing_search: None,
            search_sequence: 0,
            replay: None,
            live: None,
            next_key: 0,
            tested: 0,
            collisions: 0,
            rejected: 0,
            decided_since_placement: 0,
            placements: Vec::new(),
            stage: FillRunStage::Capture,
            stall: None,
            finished: false,
            settled: false,
            owed: None,
            progress_sequence: 0,
            closing: false,
            target_regions,
        };
        let target = provisional_placements.min(requested as usize);
        job.replay = (provisional_placements > 0).then_some(FillRunReplay { expected, retire, target });
        if target == 0 {
            job.end_replay();
        }
        job.finished = job.replay.is_none() && job.requested == 0;
        Ok(job)
    }

    /// 📟️ `[tested, accepted, collisions, rejected]`, in [`FillRunCounter::ALL`] order.
    pub(crate) fn counters(&self) -> [u64; 4] {
        [self.tested, self.placements.len() as u64, self.collisions, self.rejected]
    }

    pub(crate) fn checkpoint(&self) -> FillRunCheckpoint {
        FillRunCheckpoint { requested: self.requested, tested: self.tested, collisions: self.collisions, rejected: self.rejected, next_key: self.next_key, placements: self.placements.clone() }
    }

    fn replaying(&self) -> bool {
        self.replay.is_some()
    }

    /// 🪚️ Ends a replay: provisional placements past the ones re-derived are retracted and their records retired.
    fn end_replay(&mut self) {
        let Some(replay) = self.replay.take() else { return };
        let kept = self.placements.len();
        if kept * FILL_RUN_OPS_PER_PLACEMENT < replay.expected.len() {
            self.writer.retract_to((kept * FILL_RUN_OPS_PER_PLACEMENT) as u32);
            for placement in replay.retire.iter().skip(kept) {
                self.writer.retire(placement.key);
            }
            self.stage = FillRunStage::Retract;
            let _ = self.writer.step(ToolRunStepKind::Info, FillRunStage::Retract.index(), FillRunReason::Retracted.code(), None, &[ToolRunStepArg::Unsigned(kept as u64)]);
        }
        if kept as u64 >= self.requested {
            self.finished = true;
        }
    }

    fn subject(event: &BoardFillCandidateEvent) -> ToolRunTraceSubject {
        ToolRunTraceSubject::Placement2d { shape: event.kind_index as u32, position: [event.position[0] as f32, event.position[1] as f32], rotation: 0.0 }
    }

    fn decide(&mut self, context: &mut StepContext<'_>, live: FillRunLive, reason: FillRunReason) {
        if !self.replaying() {
            self.writer.upsert(live.key, reason.verdict(), reason.code(), live.subject);
            context.consume_fuel(1);
        }
    }

    fn observe(&mut self, context: &mut StepContext<'_>, event: BoardFillCandidateEvent) {
        let reason = FillRunReason::of_candidate(event.verdict);
        match reason {
            None | Some(FillRunReason::PortIncompatible | FillRunReason::KindIncompatible) => {
                let live = FillRunLive { key: self.next_key, subject: Self::subject(&event) };
                self.next_key += 1;
                self.tested += 1;
                if !self.replaying() {
                    self.writer.upsert(live.key, ToolRunVerdict::Testing, FillRunReason::Fits.code(), live.subject);
                }
                match reason {
                    Some(reason) => {
                        self.rejected += 1;
                        self.decide(context, live, reason);
                    }
                    None => self.live = Some(live),
                }
            }
            Some(FillRunReason::HostCollision | FillRunReason::VirtualCollision) => {
                if let Some(live) = self.live.take() {
                    self.collisions += 1;
                    self.decided_since_placement += 1;
                    self.decide(context, live, reason.expect("collision reason"));
                }
            }
            _ => {}
        }
    }

    /// 🧱️ Moves the engine's pending placement into provisional ops; `true` when a live tick is owed.
    fn accept(&mut self, context: &mut StepContext<'_>) -> Result<bool, &'static str> {
        let search = self.search.as_mut().ok_or("puzzle2d-fill-run-search-owner")?;
        let mut checkpoint = search.take_checkpoint().ok_or("puzzle2d-fill-checkpoint-missing")?;
        let placement = checkpoint.take_pending_placement();
        let adopted = search.adopt_checkpoint(checkpoint);
        let mut placement = placement.ok_or("puzzle2d-fill-placement-missing")?;
        let mutations = fill_placement_mutations(&placement);
        while !placement.close_step(1, JOB_PAYLOAD_PAGE_BYTES) {}
        drop(placement);
        if let Err(checkpoint) = adopted {
            self.closing_search = Some(checkpoint.into_closing_job());
            return Err("puzzle2d-fill-checkpoint-stale");
        }
        let [create, connect] = mutations?;
        if let Puzzle2dMutation::CreateNode(payload) = &create {
            let node = &payload.node;
            let rectangle = node.shape.as_deref() == Some("rectangle");
            let bounds = fill_node_bounds(node.x, node.y, node.scale, rectangle, if rectangle { node.width } else { node.radius }, node.height);
            if !fill_regions_admit(&self.target_regions, bounds) {
                if let Some(live) = self.live.take() {
                    self.rejected += 1;
                    self.decided_since_placement += 1;
                    self.decide(context, live, FillRunReason::OutsideTargetRegion);
                }
                return Ok(false);
            }
        }
        let entity = match &create {
            Puzzle2dMutation::CreateNode(payload) => fill_run_entity(&payload.node.id),
            _ => return Err("puzzle2d-fill-run-create-node"),
        };
        let ops = [protocol::OpBinary::encode_op(&create).map_err(|_| "puzzle2d-fill-run-op-encode")?, protocol::OpBinary::encode_op(&connect).map_err(|_| "puzzle2d-fill-run-op-encode")?];
        let live = self.live.take().ok_or("puzzle2d-fill-run-placement-untested")?;
        let shape = match live.subject {
            ToolRunTraceSubject::Placement2d { shape, .. } => shape,
            _ => u32::MAX,
        };
        let index = self.placements.len();
        self.decided_since_placement = 0;
        if let Some(replay) = self.replay.as_ref() {
            let offset = index * FILL_RUN_OPS_PER_PLACEMENT;
            if replay.expected.get(offset..offset + FILL_RUN_OPS_PER_PLACEMENT) == Some(&ops[..]) {
                self.placements.push(FillRunPlacementKey { key: live.key, shape });
                if self.placements.len() >= replay.target {
                    self.end_replay();
                }
                return Ok(false);
            }
            self.end_replay();
        }
        self.writer.upsert(live.key, ToolRunVerdict::Success, FillRunReason::Fits.code(), live.subject);
        let [create, connect] = ops;
        self.writer.append_op(create).and_then(|()| self.writer.append_op(connect)).map_err(|_| "puzzle2d-fill-run-provisional-ops")?;
        self.writer.append_entity(entity);
        context.consume_fuel(1);
        self.placements.push(FillRunPlacementKey { key: live.key, shape });
        if self.placements.len() as u64 >= self.requested {
            self.finished = true;
            self.settle();
        }
        Ok(true)
    }

    fn stall_reason(&self) -> FillRunReason {
        match self.decided_since_placement {
            _ if self.open_handles == 0 && self.placements.is_empty() => FillRunReason::NoOpenHandle,
            0 => FillRunReason::NoCompatibleKind,
            _ => FillRunReason::NoFreePlacement,
        }
    }

    fn stall(&mut self, context: &mut StepContext<'_>, reason: FillRunReason) {
        if let Some(live) = self.live.take() {
            self.rejected += 1;
            self.decide(context, live, FillRunReason::ArtifactCapacity);
        }
        self.end_replay();
        self.stall = Some(reason);
        self.finished = true;
    }

    fn settle(&mut self) {
        if std::mem::replace(&mut self.settled, true) {
            return;
        }
        let placed = [ToolRunStepArg::Unsigned(self.placements.len() as u64)];
        let reason = self.stall.unwrap_or(FillRunReason::RequestedReached);
        let kind = if self.stall.is_some() { ToolRunStepKind::Warning } else { ToolRunStepKind::Success };
        let _ = self.writer.step(kind, self.stage.index(), reason.code(), None, &placed);
    }

    fn progress(&mut self) -> ToolRunProgress {
        self.progress_sequence += 1;
        ToolRunProgress {
            identity: self.writer.identity(),
            sequence: self.progress_sequence,
            state: if self.finished { ToolRunState::Complete } else { ToolRunState::Running },
            stage: self.stage.index(),
            completed: self.placements.len() as u64,
            total: Some(self.requested),
            counters: FillRunCounter::ALL.iter().zip(self.counters()).map(|(counter, value)| ToolRunCounter { counter: counter.index(), value }).collect(),
            units_per_second: 0.0,
            conflicts: 0,
            steps: ToolRunStepRing::default(),
        }
    }

    fn flush(&mut self, context: &mut StepContext<'_>) -> Option<StepOutcome> {
        if self.writer.is_empty() && !self.finished {
            return None;
        }
        let progress = self.progress();
        self.writer.progress(progress);
        let bytes = self.writer.finish()?.encode().ok();
        Some(match bytes.map(|bytes| context.payload_from_bytes(JobPayloadStream::Preview, &bytes)) {
            Some(Ok(payload)) => StepOutcome::PreviewReady(payload),
            Some(Err(rejected)) => {
                drop(rejected.into_source());
                fill_run_fault(context, "puzzle2d-fill-run-tick-bytes")
            }
            None => fill_run_fault(context, "puzzle2d-fill-run-tick-encode"),
        })
    }

    fn flush_then(&mut self, context: &mut StepContext<'_>, owed: FillRunOwed) -> StepOutcome {
        match self.flush(context) {
            Some(preview @ StepOutcome::PreviewReady(_)) => {
                self.owed = Some(owed);
                preview
            }
            Some(fault) => fault,
            None => self.settle_owed(context, owed),
        }
    }

    fn settle_owed(&mut self, context: &mut StepContext<'_>, owed: FillRunOwed) -> StepOutcome {
        match owed {
            FillRunOwed::Complete => StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) }),
            FillRunOwed::Checkpoint => match context.payload_from_bytes(JobPayloadStream::CheckpointState, &self.checkpoint().encode()) {
                Ok(state) => {
                    if self.finished {
                        self.owed = Some(FillRunOwed::Complete);
                    }
                    StepOutcome::CheckpointReady(Checkpoint { state, applied_progress: self.placements.len() as u64 })
                }
                Err(rejected) => {
                    drop(rejected.into_source());
                    self.owed = Some(FillRunOwed::Checkpoint);
                    StepOutcome::Yield
                }
            },
        }
    }

    /// 🔎️ Engine transitions until a candidate event, a non-preview outcome, a stall or the wall deadline.
    fn search_batch(search: &mut BoardFillJob, sequence: &mut u64, context: &StepContext<'_>) -> (Option<StepOutcome>, Option<BoardFillCandidateEvent>) {
        let operation = search.operation();
        let mut inner = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX), context.cancel_token(), fill_run_monotonic_zero, sequence);
        for transition in 0..FILL_RUN_SEARCH_BATCH {
            if search.stage() == BoardFillStage::Complete {
                return (None, None);
            }
            let mut outcome = InteractiveJob::step(search, &mut inner);
            let _ = search.take_preview();
            let event = search.take_candidate_event();
            match outcome {
                StepOutcome::Yield | StepOutcome::PreviewReady(_) => {
                    close_outcome(&mut outcome);
                    if event.is_some() {
                        return (None, event);
                    }
                }
                other => return (Some(other), event),
            }
            if transition % FILL_RUN_DEADLINE_CHECK == FILL_RUN_DEADLINE_CHECK - 1 && context.deadline_exceeded() {
                return (None, None);
            }
        }
        (None, None)
    }
}

impl InteractiveJob for Puzzle2dFillRunJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if let Some(owed) = self.owed.take() {
            return self.settle_owed(context, owed);
        }
        loop {
            if self.finished {
                self.settle();
                return self.flush_then(context, FillRunOwed::Checkpoint);
            }
            if context.fuel_exhausted() || context.deadline_exceeded() || self.writer.pending_bytes() >= FILL_RUN_TICK_FLUSH_BYTES {
                return self.flush(context).unwrap_or(StepOutcome::Yield);
            }
            if let Some(capture) = self.capture.as_mut() {
                match capture.advance(&self.document.0, &self.kinds, FILL_RUN_CAPTURE_UNITS) {
                    Ok(None) => {}
                    Ok(Some(snapshot)) => {
                        let serial = capture.serial;
                        self.open_handles = capture.open_handles;
                        self.capture = None;
                        self.search = Some(BoardFillJob::with_operation(snapshot, u32::MAX, Operation::new(OperationId(serial), RevisionId(0), Generation(0), self.seed)));
                        self.stage = FillRunStage::Search;
                    }
                    Err(code) => return fill_run_fault(context, code),
                }
                continue;
            }
            let Some(search) = self.search.as_mut() else { return fill_run_fault(context, "puzzle2d-fill-run-search-owner") };
            if search.stage() == BoardFillStage::Complete {
                let reason = self.stall_reason();
                self.stall(context, reason);
                continue;
            }
            let (outcome, event) = Self::search_batch(search, &mut self.search_sequence, context);
            let fault = match &outcome {
                Some(StepOutcome::Fault(_)) => search.take_fault(),
                _ => None,
            };
            self.stage = FillRunStage::of(search.stage());
            if let Some(event) = event {
                self.observe(context, event);
            }
            match outcome {
                None => {}
                Some(StepOutcome::Cancelled) => return StepOutcome::Cancelled,
                Some(mut outcome) => {
                    let checkpoint = matches!(outcome, StepOutcome::CheckpointReady(_));
                    close_outcome(&mut outcome);
                    match (checkpoint, fault) {
                        (true, _) => match self.accept(context) {
                            Ok(true) => return self.flush_then(context, FillRunOwed::Checkpoint),
                            Ok(false) => {}
                            Err(code) => return fill_run_fault(context, code),
                        },
                        (false, Some("placement-capacity")) => self.stall(context, FillRunReason::ArtifactCapacity),
                        (false, Some(code)) => return fill_run_fault(context, code),
                        (false, None) => return fill_run_fault(context, "puzzle2d-fill-run-search-outcome"),
                    }
                }
            }
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.closing = true;
        if maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some(capture) = self.capture.as_mut() {
            if capture.close_one() {
                self.capture = None;
            }
        } else if self.closing_search.is_some() {
            close_job_slot(&mut self.closing_search);
        } else if self.search.is_some() {
            close_job_slot(&mut self.search);
        } else {
            self.replay = None;
            self.placements = Vec::new();
            return InteractiveJobCloseStep::Complete;
        }
        InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.capture.is_none() && self.search.is_none() && self.closing_search.is_none()
    }
}
//#endregion ⏯️RunJob

//#region 🔍️RevalidateJob
/// 🔍️ The puzzle 2d fill revalidation job (`ToolRunDefinition.revalidateJob`): re-tests every provisional
/// placement against the head document. A placement conflicts when its node id already exists, its source
/// handle is gone (neither in the head nor on a surviving earlier placement) or its footprint overlaps a head
/// node. Each placement is one unit of fuel and ends `success` (`fits`) or `danger` (`TOOL_RUN_REASON_CONFLICT`);
/// the last tick retracts to the first conflict and re-appends every later survivor's ops and entity with one
/// `danger` conflict step, and `Complete` follows on the next call.
pub(crate) struct Puzzle2dFillRevalidateJob {
    writer: ToolRunTickWriter,
    head: Arc<Puzzle2dPlaySnapshot>,
    ops: Vec<Puzzle2dMutation>,
    keys: Vec<FillRunPlacementKey>,
    head_cursor: usize,
    head_bounds: Vec<[f64; 4]>,
    head_ids: HashSet<String>,
    handles: HashSet<String>,
    conflicts: Vec<bool>,
    /// 🚧️ `contactTolerance - brushPlacementOverlapBudget`, the one number the AABB test is widened by.
    placement_slack: f64,
    finished: bool,
    completed: bool,
    closed: bool,
}

impl Puzzle2dFillRevalidateJob {
    pub(crate) fn new(identity: ToolRunIdentity, head: Arc<Puzzle2dPlaySnapshot>, provisional: &[Puzzle2dMutation], checkpoint: Option<&[u8]>, placement_slack: f64) -> Self {
        let keys = checkpoint.and_then(FillRunCheckpoint::decode).map(|checkpoint| checkpoint.placements).unwrap_or_default();
        Self {
            writer: ToolRunTickWriter::with_provisional_base(identity, provisional.len() as u32),
            head,
            ops: provisional.to_vec(),
            keys,
            head_cursor: 0,
            head_bounds: Vec::new(),
            head_ids: HashSet::new(),
            handles: HashSet::new(),
            conflicts: Vec::new(),
            placement_slack: if placement_slack.is_finite() { placement_slack } else { 0.0 },
            finished: false,
            completed: false,
            closed: false,
        }
    }

    fn placement(&self, index: usize) -> Option<(&crate::Puzzle2dNode, &str)> {
        match self.ops.get(index * FILL_RUN_OPS_PER_PLACEMENT..(index + 1) * FILL_RUN_OPS_PER_PLACEMENT)? {
            [Puzzle2dMutation::CreateNode(create), Puzzle2dMutation::ConnectHandles(connect)] => Some((&create.node, connect.source.as_str())),
            _ => None,
        }
    }

    fn prepare_head_one(&mut self) -> bool {
        let Some(node) = self.head.0.get("nodes").and_then(Value::as_array).and_then(|nodes| nodes.get(self.head_cursor)) else { return false };
        self.head_cursor += 1;
        let rectangle = node.get("shape").and_then(Value::as_str) == Some("rectangle");
        let number = |key: &str| node.get(key).and_then(Value::as_f64);
        self.head_bounds.push(fill_node_bounds(number("x").unwrap_or(0.0), number("y").unwrap_or(0.0), number("scale"), rectangle, number(if rectangle { "width" } else { "radius" }), number("height")));
        if let Some(id) = node.get("id").and_then(Value::as_str) {
            self.head_ids.insert(id.to_string());
        }
        for handle in node.get("handles").and_then(Value::as_array).into_iter().flatten() {
            if let Some(id) = handle.get("id").and_then(Value::as_str) {
                self.handles.insert(id.to_string());
            }
        }
        true
    }

    fn test_placement(&mut self, index: usize) -> (bool, ToolRunTraceSubject) {
        let shape = self.keys.get(index).map_or(u32::MAX, |key| key.shape);
        let Some((node, source)) = self.placement(index) else {
            return (true, ToolRunTraceSubject::Placement2d { shape, position: [0.0, 0.0], rotation: 0.0 });
        };
        let subject = ToolRunTraceSubject::Placement2d { shape, position: [node.x as f32, node.y as f32], rotation: 0.0 };
        let rectangle = node.shape.as_deref() == Some("rectangle");
        let bounds = fill_node_bounds(node.x, node.y, node.scale, rectangle, if rectangle { node.width } else { node.radius }, node.height);
        let slack = self.placement_slack;
        let conflict = self.head_ids.contains(&node.id) || !self.handles.contains(source) || self.head_bounds.iter().any(|head| fill_bounds_overlap_with(bounds, *head, slack));
        let handles: Vec<String> = if conflict { Vec::new() } else { node.handles.iter().map(|handle| handle.id.clone()).collect() };
        self.handles.extend(handles);
        (conflict, subject)
    }

    fn finish(&mut self) -> Result<(), &'static str> {
        self.finished = true;
        let Some(first) = self.conflicts.iter().position(|conflict| *conflict) else { return Ok(()) };
        self.writer.retract_to((first * FILL_RUN_OPS_PER_PLACEMENT) as u32);
        for index in (first..self.conflicts.len()).filter(|index| !self.conflicts[*index]) {
            let offset = index * FILL_RUN_OPS_PER_PLACEMENT;
            for op in &self.ops[offset..offset + FILL_RUN_OPS_PER_PLACEMENT] {
                self.writer.append_op(protocol::OpBinary::encode_op(op).map_err(|_| "puzzle2d-fill-revalidate-op-encode")?).map_err(|_| "puzzle2d-fill-revalidate-provisional-ops")?;
            }
            let entity = self.placement(index).map(|(node, _)| fill_run_entity(&node.id)).ok_or("puzzle2d-fill-revalidate-placement")?;
            self.writer.append_entity(entity);
        }
        let conflicts = self.conflicts.iter().filter(|conflict| **conflict).count() as u64;
        self.writer.step(ToolRunStepKind::Danger, FillRunStage::Place.index(), TOOL_RUN_REASON_CONFLICT, None, &[ToolRunStepArg::Unsigned(conflicts)]).map_err(|_| "puzzle2d-fill-revalidate-step")
    }

    fn flush(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        let total = (self.ops.len() / FILL_RUN_OPS_PER_PLACEMENT) as u64;
        let conflicts = self.conflicts.iter().filter(|conflict| **conflict).count() as u64;
        self.writer.progress(ToolRunProgress {
            identity: self.writer.identity(),
            sequence: self.conflicts.len() as u64,
            state: ToolRunState::Finalizing,
            stage: FillRunStage::Place.index(),
            completed: self.conflicts.len() as u64,
            total: Some(total),
            counters: vec![ToolRunCounter { counter: FillRunCounter::Collisions.index(), value: conflicts }],
            units_per_second: 0.0,
            conflicts: conflicts as u32,
            steps: ToolRunStepRing::default(),
        });
        match self.writer.finish().and_then(|tick| tick.encode().ok()).map(|bytes| context.payload_from_bytes(JobPayloadStream::Preview, &bytes)) {
            Some(Ok(payload)) => StepOutcome::PreviewReady(payload),
            Some(Err(rejected)) => {
                drop(rejected.into_source());
                fill_run_fault(context, "puzzle2d-fill-revalidate-tick-bytes")
            }
            None => fill_run_fault(context, "puzzle2d-fill-revalidate-tick-encode"),
        }
    }
}

impl InteractiveJob for Puzzle2dFillRevalidateJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        loop {
            if self.completed {
                return StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) });
            }
            if self.finished {
                self.completed = true;
                return self.flush(context);
            }
            if context.fuel_exhausted() || context.deadline_exceeded() || self.writer.pending_bytes() >= FILL_RUN_TICK_FLUSH_BYTES {
                return if self.writer.is_empty() { StepOutcome::Yield } else { self.flush(context) };
            }
            if self.prepare_head_one() {
                continue;
            }
            let index = self.conflicts.len();
            if index * FILL_RUN_OPS_PER_PLACEMENT >= self.ops.len() {
                if let Err(code) = self.finish() {
                    return fill_run_fault(context, code);
                }
                continue;
            }
            let (conflict, subject) = self.test_placement(index);
            let key = self.keys.get(index).map_or(u64::MAX - index as u64, |key| key.key);
            self.writer.upsert(key, ToolRunVerdict::Testing, FillRunReason::Fits.code(), subject);
            let (verdict, reason) = if conflict { (ToolRunVerdict::Danger, TOOL_RUN_REASON_CONFLICT) } else { (ToolRunVerdict::Success, FillRunReason::Fits.code()) };
            self.writer.upsert(key, verdict, reason, subject);
            self.conflicts.push(conflict);
            context.consume_fuel(1);
        }
    }

    fn begin_close(&mut self) {
        self.closed = true;
    }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.closed = true;
        self.ops = Vec::new();
        self.head_bounds = Vec::new();
        self.head_ids = HashSet::new();
        self.handles = HashSet::new();
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closed && self.ops.is_empty() && self.head_bounds.is_empty()
    }
}
//#endregion 🔍️RevalidateJob

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
