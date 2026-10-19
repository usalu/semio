//! 📏️ Limits, vocabulary and placement: the fill run limits, the run stage, counter and reason vocabulary, the checkpoint and the placement helpers. See [`super`].

use super::*;

//#region 🔖️Limits
/// 🚰️ Estimated pending tick bytes past which a run job flushes, so one tick stays within one job payload page.
pub const FILL_RUN_TICK_FLUSH_BYTES: usize = 8 * 1024;

/// 🧬️ Provisional ops one placement appends: `create_node` then `connect_handles`.
pub const FILL_RUN_OPS_PER_PLACEMENT: usize = 2;

/// 🎯️ Target regions one run reads as its placement constraint. A board is painted by hand, so this
/// admits far more than any authored document holds while keeping the run's own owner fixed.
pub const FILL_RUN_TARGET_REGION_SLOTS: usize = 256;

pub(super) const FILL_RUN_CAPTURE_UNITS: usize = 256;

pub(super) const FILL_RUN_SEARCH_BATCH: usize = 64;

pub(super) const FILL_RUN_DEADLINE_CHECK: usize = 16;
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
    pub fn label(self) -> fn(&crate::editor::puzzle2d::terminology::Puzzle2dLabels) -> semio_framework_ui_locale::LabelText {
        match self {
            Self::Capture => |labels| labels.fill_stage_capture,
            Self::Search => |labels| labels.fill_stage_search,
            Self::Test => |labels| labels.fill_stage_test,
            Self::Place => |labels| labels.fill_stage_place,
            Self::Retract => |labels| labels.fill_stage_retract,
        }
    }

    /// 🔭️ The run stage an engine search stage belongs to.
    pub(super) fn of(stage: BoardFillStage) -> Self {
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

    pub fn label(self) -> fn(&crate::editor::puzzle2d::terminology::Puzzle2dLabels) -> semio_framework_ui_locale::LabelText {
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

    pub fn label(self) -> fn(&crate::editor::puzzle2d::terminology::Puzzle2dLabels) -> semio_framework_ui_locale::LabelText {
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
    pub(super) fn of_candidate(verdict: BoardFillCandidateVerdict) -> Option<Self> {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct FillRunPlacementKey {
    pub key: u64,
    pub shape: u32,
}

/// 📸️ Resume point a run reports through `JobTurn::Checkpoint`, little-endian:
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
pub fn fill_run_entity(node_id: &semio_framework_value::paged::PagedUtf8<{ usize::MAX }>) -> u64 {
    use semio_framework_value::paged::Utf8Text;
    let mut hasher=semio_framework_hash::Hasher::new();
    for index in 0..node_id.text_chunk_count(){hasher.update(node_id.text_chunk(index).expect("native text chunk").as_bytes());}
    u64::from_le_bytes(hasher.finalize().as_bytes()[..8].try_into().expect("eight digest bytes"))
}

/// 🗂️ The node kind rows a fill places: the document's own `meta.kindCatalogs.nodes`, else the engine catalog
/// rows of its manifest (`board_kind_catalogs_json`), else the kinds the placed nodes imply
/// (`inferred_node_kind_rows` — Concrete Forest names no manifest and carries no catalog, and a fill
/// that finds no kind places nothing).
pub fn fill_kind_rows(document: &Value) -> Vec<Value> {
    if let Some(rows) = crate::editor::puzzle2d::snapshot::kind_catalog_entries(document, "nodes").filter(|rows| !rows.is_empty()) {
        return rows.to_vec();
    }
    let manifest_rows = crate::editor::puzzle2d::snapshot::board_kind_catalogs_json(document).and_then(|json| semio_framework_pack_json::parse(&json,semio_framework_pack_json::JsonMemberPolicy::Reject).ok()).and_then(|catalogs| catalogs.get("nodeKinds").and_then(Value::as_array).cloned()).unwrap_or_default();
    if manifest_rows.iter().any(|row| row.get("handles").and_then(Value::as_array).is_some_and(|templates| !templates.is_empty())) {
        return manifest_rows;
    }
    let inferred = crate::editor::puzzle2d::snapshot::inferred_node_kind_rows(document);
    if inferred.is_empty() { manifest_rows } else { inferred }
}

/// 🔢️ The first engine serial a run may mint without reusing a fill node id `id` already holds.
pub(super) fn fill_serial_after(id: &str) -> u64 {
    id.strip_prefix(PUZZLE_2D_BOARD_NAMING.fill_node_prefix).and_then(|serial| serial.parse::<u64>().ok()).map_or(1, |serial| serial.saturating_add(1))
}

/// 🧬️ The `create_node` and `connect_handles` document mutations one engine placement contributes.
pub(super) fn fill_placement_mutations(placement: &BoardFillPlacement) -> Result<[Puzzle2dMutation; 2], &'static str> {
    let rectangle = placement.shape == "rectangle";
    let handles = (0..placement.handle_count())
        .map(|index| {
            let (id, handle_kind, angle, radius) = placement.handle(index).ok_or("puzzle2d-fill-apply-handle")?;
            Ok(crate::standards::v1::subsets::any::schema::records::Puzzle2dHandle { id: id.into(), handle_kind: Some(handle_kind.into()), angle, radius, ..Default::default() })
        })
        .collect::<Result<semio_framework_value_persistent::list::PagedList<_, { usize::MAX }>, &'static str>>()?;
    let node = crate::standards::v1::subsets::any::schema::records::Puzzle2dNode {
        id: placement.node_id.as_str().into(),
        node_kind: Some(placement.node_kind.as_str().into()),
        shape: Some(placement.shape.into()),
        x: placement.x,
        y: placement.y,
        radius: (!rectangle).then_some(placement.radius),
        width: rectangle.then_some(placement.width),
        height: rectangle.then_some(placement.height),
        text: Some(placement.node_id.as_str().into()),
        icon_kind: placement.icon_kind.as_ref().map(|icon| icon.as_str().into()),
        anchor: crate::standards::v1::subsets::any::schema::records::Puzzle2dNodeAnchor::Fixed,
        handles,
        ..Default::default()
    };
    let edge = connect_handles(placement.edge_id.as_str().into(), placement.source_handle_id.as_str().into(), placement.target_handle_id.as_str().into(), Some(placement.edge_kind.as_str().into()), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None, None);
    Ok([create_node(node, None), edge])
}

/// 🎯️ The normalized bounds of every visible target region the document declares, capped at
/// [`FILL_RUN_TARGET_REGION_SLOTS`] — a document painted past that ceiling constrains fill by its
/// first regions rather than faulting the run, and the excess is visible in the board itself.
pub fn fill_visible_region_bounds(snapshot: &Value) -> Vec<[f64; 4]> {
    crate::editor::puzzle2d::snapshot::snapshot_target_regions(snapshot)
        .iter()
        .filter(|region| region.get("hidden").and_then(Value::as_bool) != Some(true))
        .take(FILL_RUN_TARGET_REGION_SLOTS)
        .map(crate::editor::puzzle2d::snapshot::puzzle2d_region_bounds)
        .collect()
}

/// 🎯️ Whether a placement's own footprint lies fully inside ANY visible target region. An empty set
/// is unconstrained, so a board with no region fills exactly as it did before regions existed.
pub fn fill_regions_admit(regions: &[[f64; 4]], bounds: [f64; 4]) -> bool {
    regions.is_empty() || regions.iter().any(|region| bounds[0] >= region[0] && bounds[1] >= region[1] && bounds[2] <= region[2] && bounds[3] <= region[3])
}

/// 📦️ Axis-aligned collision footprint `[min_x, min_y, max_x, max_y]` of a board node, exactly as the fill
/// capture feeds it to the engine: a circle spans `radius · scale`, a rectangle half its scaled extents.
pub fn fill_node_bounds(x: f64, y: f64, scale: Option<f64>, rectangle: bool, radius_or_width: Option<f64>, height: Option<f64>) -> [f64; 4] {
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
pub fn fill_bounds_overlap_with(left: [f64; 4], right: [f64; 4], slack: f64) -> bool {
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

pub(super) fn fill_run_fault(code: &str) -> JobTurn {
    JobTurn::Fault(code.as_bytes().to_vec())
}

/// ⏱️ The engine search runs under a fuel-only clock: the run job enforces the wall deadline between batches.
pub fn fill_run_monotonic_zero() -> Option<u64> {
    Some(0)
}
//#endregion 🧱️Placement
