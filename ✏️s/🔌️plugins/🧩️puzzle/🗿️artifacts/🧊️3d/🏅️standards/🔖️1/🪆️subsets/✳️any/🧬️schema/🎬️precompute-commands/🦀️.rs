//! 🎬️ Precompute commands: the brush and fill readouts, the run stage, counter and reason vocabularies, the typed command envelope and the session errors.

use super::*;

//#region 🔖️PrecomputeModel
#[derive(Debug, Clone, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct BrushCompatibleCandidate {
    pub object_kind_id: String,
    pub source_vortex_index: usize,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
pub struct BrushPreviewState {
    pub target_vortex_full_id: String,
    pub object_kind_id: String,
    pub source_vortex_index: usize,
    pub mesh_url: String,
    pub origin: Vec3,
    pub orientation: Quat,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "testing"), serde(skip_serializing_if = "Option::is_none"))]
    pub scale: Option<semio_framework_value::DslValue>,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BrushPlacePayload {
    pub target_vortex_full_id: String,
    pub object_kind_id: String,
    pub source_vortex_index: usize,
    pub origin: Vec3,
    pub orientation: Quat,
    #[cfg_attr(any(test, feature = "testing"), serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub scale: Option<semio_framework_value::DslValue>,
}

/// 🎯️ A suggestion-popup preview accepted as-is becomes a placement at the exact same pose — the one
/// field `BrushPreviewState` carries that `BrushPlacePayload` doesn't (`mesh_url`, resolvable again
/// from `object_kind_id` via the kind catalog) is simply dropped.
impl From<BrushPreviewState> for BrushPlacePayload {
    fn from(preview: BrushPreviewState) -> Self {
        Self { target_vortex_full_id: preview.target_vortex_full_id, object_kind_id: preview.object_kind_id, source_vortex_index: preview.source_vortex_index, origin: preview.origin, orientation: preview.orientation, scale: preview.scale }
    }
}

/// 🎯️ Public so `Puzzle3dEngineOutcome::BrushCandidates` can hand this back to callers (the app's
/// brush slot) as a typed value instead of the JSON string the old `brush_candidates` wasm-bindgen
/// method returned.
#[derive(Debug, Clone, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct BrushCollisionFreeResult {
    pub free: Vec<BrushCompatibleCandidate>,
    pub unknown_pending: bool,
    #[value(default)]
    pub resume_candidate_index: usize,
}

/// 🧭️ Stage of a brush suggestions tool run, the index into `ToolRunDefinition.stages`
/// (`$defs.Puzzle3dBrushSuggestionsRun`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrushSuggestionsRunStage {
    Prepare,
    Target,
    Test,
    Idle,
}

impl BrushSuggestionsRunStage {
    pub const ALL: [Self; 4] = [Self::Prepare, Self::Target, Self::Test, Self::Idle];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Prepare => "prepare",
            Self::Target => "target",
            Self::Test => "test",
            Self::Idle => "idle",
        }
    }
}

/// 🔢️ Counter of a brush suggestions tool run, the index into `ToolRunDefinition.counters`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrushSuggestionsRunCounter {
    Tested,
    Free,
    Collisions,
}

impl BrushSuggestionsRunCounter {
    pub const ALL: [Self; 3] = [Self::Tested, Self::Free, Self::Collisions];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Tested => "tested",
            Self::Free => "free",
            Self::Collisions => "collisions",
        }
    }
}

/// 🏷️ Reason code of a brush suggestions trace record or step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrushSuggestionsRunReason {
    Free,
    Collision,
    PoseUnavailable,
    TargetMissing,
    SuggestionsBlocked,
    SearchComplete,
}

impl BrushSuggestionsRunReason {
    pub const ALL: [Self; 6] = [Self::Free, Self::Collision, Self::PoseUnavailable, Self::TargetMissing, Self::SuggestionsBlocked, Self::SearchComplete];

    pub fn code(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Free => "free",
            Self::Collision => "collision",
            Self::PoseUnavailable => "pose-unavailable",
            Self::TargetMissing => "target-missing",
            Self::SuggestionsBlocked => "suggestions-blocked",
            Self::SearchComplete => "search-complete",
        }
    }

    /// 🚥️ A free candidate and a finished search succeed, an overlap is a collision, the rest warn.
    pub fn verdict(self) -> semio_framework_tool_run::ToolRunVerdict {
        match self {
            Self::Free | Self::SearchComplete => semio_framework_tool_run::ToolRunVerdict::Success,
            Self::Collision => semio_framework_tool_run::ToolRunVerdict::Danger,
            Self::PoseUnavailable | Self::TargetMissing | Self::SuggestionsBlocked => semio_framework_tool_run::ToolRunVerdict::Warning,
        }
    }
}

/// 🧭️ Stage of a fill tool run, the index into `ToolRunDefinition.stages` (`$defs.Puzzle3dFillRun`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FillRunStage {
    Prepare,
    Search,
    Test,
    Lock,
    Retract,
}

impl FillRunStage {
    pub const ALL: [Self; 5] = [Self::Prepare, Self::Search, Self::Test, Self::Lock, Self::Retract];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Prepare => "prepare",
            Self::Search => "search",
            Self::Test => "test",
            Self::Lock => "lock",
            Self::Retract => "retract",
        }
    }
}

/// 🔢️ Counter of a fill tool run, the index into `ToolRunDefinition.counters`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FillRunCounter {
    Tested,
    Locked,
    Collisions,
    Rejected,
    Marked,
}

impl FillRunCounter {
    pub const ALL: [Self; 5] = [Self::Tested, Self::Locked, Self::Collisions, Self::Rejected, Self::Marked];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Tested => "tested",
            Self::Locked => "locked",
            Self::Collisions => "collisions",
            Self::Rejected => "rejected",
            Self::Marked => "marked",
        }
    }
}

/// 🏷️ Reason code of a fill run trace record or step; `id` spells the planner's own refusal string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FillRunReason {
    Fits,
    SolidOverlap,
    OutsideTargetVolume,
    MeshUnavailable,
    MissingPreview,
    MissingTarget,
    BroadPhaseEntryMissing,
    PlacedMeshUnavailable,
    StaleSpatialQuery,
    PlacementKindMissing,
    PlacementVortexMissing,
    PlacementMeshMissing,
    PlacementRejected,
    PlacementStateMissing,
    PlacementSpatialStateMissing,
    StaleSpatialMutation,
    Rejected,
    NoOpenVortex,
    NoCompatibleKind,
    NoFreePlacement,
    ArtifactCapacity,
    RequestedReached,
    Retracted,
    VortexExhausted,
}

impl FillRunReason {
    pub const ALL: [Self; 24] = [
        Self::Fits,
        Self::SolidOverlap,
        Self::OutsideTargetVolume,
        Self::MeshUnavailable,
        Self::MissingPreview,
        Self::MissingTarget,
        Self::BroadPhaseEntryMissing,
        Self::PlacedMeshUnavailable,
        Self::StaleSpatialQuery,
        Self::PlacementKindMissing,
        Self::PlacementVortexMissing,
        Self::PlacementMeshMissing,
        Self::PlacementRejected,
        Self::PlacementStateMissing,
        Self::PlacementSpatialStateMissing,
        Self::StaleSpatialMutation,
        Self::Rejected,
        Self::NoOpenVortex,
        Self::NoCompatibleKind,
        Self::NoFreePlacement,
        Self::ArtifactCapacity,
        Self::RequestedReached,
        Self::Retracted,
        Self::VortexExhausted,
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
            Self::SolidOverlap => "solid-overlap",
            Self::OutsideTargetVolume => "outside-target-volume",
            Self::MeshUnavailable => "mesh-unavailable",
            Self::MissingPreview => "missing-preview",
            Self::MissingTarget => "missing-target",
            Self::BroadPhaseEntryMissing => "broad-phase-entry-missing",
            Self::PlacedMeshUnavailable => "placed-mesh-unavailable",
            Self::StaleSpatialQuery => "stale-spatial-query",
            Self::PlacementKindMissing => "placement-kind-missing",
            Self::PlacementVortexMissing => "placement-vortex-missing",
            Self::PlacementMeshMissing => "placement-mesh-missing",
            Self::PlacementRejected => "placement-rejected",
            Self::PlacementStateMissing => "placement-state-missing",
            Self::PlacementSpatialStateMissing => "placement-spatial-state-missing",
            Self::StaleSpatialMutation => "stale-spatial-mutation",
            Self::Rejected => "rejected",
            Self::NoOpenVortex => "no-open-vortex",
            Self::NoCompatibleKind => "no-compatible-kind",
            Self::NoFreePlacement => "no-free-placement",
            Self::ArtifactCapacity => "artifact-capacity",
            Self::RequestedReached => "requested-reached",
            Self::Retracted => "retracted",
            Self::VortexExhausted => "vortex-exhausted",
        }
    }

    pub fn of_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|candidate| candidate.id() == id)
    }

    /// 🔎️ The reason a planner refusal string names; an undeclared refusal is a plain `rejected`.
    pub fn of_refusal(reason: &str) -> Self {
        Self::ALL[..Self::Rejected as usize].iter().copied().find(|candidate| candidate.id() == reason).unwrap_or(Self::Rejected)
    }

    /// 🚥️ `fits` succeeds, `solid-overlap` is a collision, every other candidate refusal is a rule warning,
    /// the stalls warn, reaching the request succeeds, a retraction informs and a vortex without any collision-free
    /// candidate is marked as danger.
    pub fn verdict(self) -> semio_framework_tool_run::ToolRunVerdict {
        match self {
            Self::Fits | Self::RequestedReached => semio_framework_tool_run::ToolRunVerdict::Success,
            Self::SolidOverlap | Self::VortexExhausted => semio_framework_tool_run::ToolRunVerdict::Danger,
            Self::Retracted => semio_framework_tool_run::ToolRunVerdict::Testing,
            _ => semio_framework_tool_run::ToolRunVerdict::Warning,
        }
    }
}

/// 📸️ Resume point a fill run job reports through `JobTurn::Checkpoint`: fixed 68-byte
/// little-endian layout `requested u64 | placements u64 | provisionalOps u32 | tested u64 | nextKey u64 |
/// inputs [u8; 32]`. `inputs` digests everything the planner's deterministic sequence depends on except
/// the requested count (base revision, contact tolerance, weights, collision meshes), so a rebuilt run job
/// replays to the checkpoint only when its prefix is provably the same sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FillRunCheckpoint {
    pub requested: u64,
    pub placements: u64,
    pub provisional_ops: u32,
    pub tested: u64,
    pub next_key: u64,
    pub inputs: [u8; 32],
}

pub fn empty_puzzle3d_snapshot() -> Puzzle3dSnapshot {
    Puzzle3dSnapshot::default()
}
//#endregion 🔖️PrecomputeModel

//#region 🔖️PrecomputeCommand
/// 🎯️ Typed command envelope for `crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession::dispatch`
/// — the headless replacement for the old per-action JSON-string wasm-bindgen methods. Declared here (not
/// app-side) because `#[derive(dsl::DslEnum)]`'s generated code needs `SceneConfig`/`BrushPlacePayload` in
/// scope by value, and because `🧬️mutations/💾️binary`'s `encode_engine_command`/`decode_engine_command`
/// wrap it exactly like it already does for `Puzzle3dMutation`. Field shapes mirror the exact payload each
/// old JSON-string method parsed: `SetScene` mirrors `set_scene`'s `SceneConfig` JSON body,
/// `UpdateKindWeights` mirrors `update_kind_weights`'s two JSON map bodies.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
pub enum Puzzle3dEngineCommand {
    #[dsl(key = "set-scene")]
    SetScene { scene: SceneConfig },
    #[dsl(key = "update-kind-weights")]
    UpdateKindWeights { object_weights: std::collections::BTreeMap<String, f64>, vortex_weights: std::collections::BTreeMap<String, f64> },
    #[dsl(key = "brush-preview")]
    BrushPreview { vortex_full_id: String, candidate_index: u32 },
}

/// 📬️ What `dispatch` hands back — the typed counterpart of what each old JSON-string method
/// returned (a `BrushPreviewState` JSON string, or nothing). Plain Rust, no
/// DSL/wasm-bindgen requirement — this only ever crosses the artifact <-> app boundary in-process.
#[derive(Debug, Clone, PartialEq)]
pub enum Puzzle3dEngineOutcome {
    Unit,
    BrushPreview(Option<BrushPreviewState>),
}
//#endregion 🔖️PrecomputeCommand

/// 🧯️ Puzzle 3d precompute session errors — JSON (de)serialization and brush/fill session state
/// failures. `Json` wraps `semio_framework_value::ValueError` (not `serde_json::Error`) since every production
/// (de)serialization call site now routes text through `dsl::os_pack::json::{from_json_str,
/// to_json_string}`, which are `ToValue`/`FromValue`-based, not `serde`-based.
#[derive(Debug)]
pub enum Puzzle3dError {
    Json(semio_framework_value::ValueError),
    FillSessionUnavailable,
}

impl std::fmt::Display for Puzzle3dError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "{error}"),
            Self::FillSessionUnavailable => formatter.write_str("fill session unavailable"),
        }
    }
}

impl std::error::Error for Puzzle3dError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => std::error::Error::source(error),
            Self::FillSessionUnavailable => None,
        }
    }
}

impl From<semio_framework_value::ValueError> for Puzzle3dError {
    fn from(error: semio_framework_value::ValueError) -> Self {
        Self::Json(error)
    }
}
