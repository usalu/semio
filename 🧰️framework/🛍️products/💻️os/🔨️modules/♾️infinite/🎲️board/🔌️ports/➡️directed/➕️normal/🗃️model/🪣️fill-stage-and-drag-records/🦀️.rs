//! 🪣️ Fill stage and drag records of the board model: the fill stage, the transform, region and paint drag records, the drop preview snapshot and the pick target JSON. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillStage {
    ResetSources,
    PrepareSources,
    SelectTarget,
    ResetCandidates,
    PrepareCandidates,
    ScanCompatibility,
    SelectCandidate,
    ConstructPreview,
    ScanHostCollision,
    ScanVirtualCollision,
    AcceptCandidate,
    AcceptNodeId,
    AcceptEdgeId,
    AcceptEdgeKind,
    AcceptNodeKind,
    AcceptSourceHandle,
    AcceptTargetHandle,
    AcceptIcon,
    AcceptVirtualNode,
    AcceptSourceConnection,
    AcceptHandles,
    AcceptHandleId,
    AcceptHandleVirtual,
    AcceptHandlePublish,
    PublishPlanPrefix,
    Complete,
}

impl BoardFillStage {
    pub fn id(self) -> &'static str {
        match self {
            Self::ResetSources => "reset-sources",
            Self::PrepareSources => "prepare-sources",
            Self::SelectTarget => "select-target",
            Self::ResetCandidates => "reset-candidates",
            Self::PrepareCandidates => "prepare-candidates",
            Self::ScanCompatibility => "scan-compatibility",
            Self::SelectCandidate => "select-candidate",
            Self::ConstructPreview => "construct-preview",
            Self::ScanHostCollision => "scan-host-collision",
            Self::ScanVirtualCollision => "scan-virtual-collision",
            Self::AcceptCandidate => "accept-candidate",
            Self::AcceptNodeId => "accept-node-id",
            Self::AcceptEdgeId => "accept-edge-id",
            Self::AcceptEdgeKind => "accept-edge-kind",
            Self::AcceptNodeKind => "accept-node-kind",
            Self::AcceptSourceHandle => "accept-source-handle",
            Self::AcceptTargetHandle => "accept-target-handle",
            Self::AcceptIcon => "accept-icon",
            Self::AcceptVirtualNode => "accept-virtual-node",
            Self::AcceptSourceConnection => "accept-source-connection",
            Self::AcceptHandles => "accept-handles",
            Self::AcceptHandleId => "accept-handle-id",
            Self::AcceptHandleVirtual => "accept-handle-virtual",
            Self::AcceptHandlePublish => "accept-handle-publish",
            Self::PublishPlanPrefix => "publish-plan-prefix",
            Self::Complete => "complete",
        }
    }
}

/// 🔄️ One live rotate-ring gesture. The node centres and handle angles captured at grab time are
/// the base every frame re-derives from, so the preview never accumulates float drift and a
/// cancel restores the exact pre-gesture geometry.
#[derive(Clone, Debug)]
pub struct BoardTransformDrag {
    pub gesture: GestureStage,
    pub pivot: Point,
    pub grab: Point,
    pub radius_world: f64,
    pub radians: f64,
    pub start_positions: BTreeMap<String, (f64, f64)>,
    pub start_handle_angles: BTreeMap<String, f64>,
}

/// 🚚️ A live target-region gesture: which region, which grip, and the bounds it was grabbed at.
/// Every frame re-derives from `start_bounds`, so the preview never accumulates float drift and a
/// cancel restores the exact pre-gesture rectangle.
#[derive(Clone, Debug)]
pub struct BoardRegionDrag {
    pub gesture: GestureStage,
    pub id: String,
    pub grip: RegionGrip,
    pub grab: Point,
    pub start_bounds: [f64; 4],
    pub bounds: [f64; 4],
    pub delta: Vec2,
}

/// 🖍️ A live area-brush paint: the grid-snapped anchor and the corner the pointer is at. A release
/// that never left the anchor commits the configured brush extent instead of a zero-area sliver.
#[derive(Clone, Debug)]
pub struct BoardRegionPaint {
    pub anchor: Point,
    pub corner: Point,
    pub dragged: bool,
}

#[derive(Clone, Debug)]
pub struct DropPreviewSnapshot {
    pub node_kind_id: String,
    pub x: f64,
    pub y: f64,
    pub shape: NodeShape,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
    pub icon_kind: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct BoardPickTargetJson {
    pub domain: String,
    pub id: String,
    pub generality: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
