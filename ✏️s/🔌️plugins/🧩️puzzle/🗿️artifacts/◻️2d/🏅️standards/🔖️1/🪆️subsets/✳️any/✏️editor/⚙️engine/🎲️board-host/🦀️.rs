//! 🎲️ Puzzle 2d app engine — the themed board hosts: `BoardHost` constructors wired to this
//! plugin's metabolism icon table, plus the scene-sync, camera/LOD, hit-test, selection and
//! area-select laws they must satisfy.

use crate::editor::puzzle2d::engine::icons::puzzle_themed_icon_lookup;
use crate::editor::puzzle2d::engine::{BoardFillStage, BoardHost, BoardNaming};

/// 🪧️ The ids this plugin's boards mint and the labels its fill job reports: persisted document ids
/// (`puzzle2d.brush.<serial>`, `puzzle2d.fill.<serial>`) and job progress stages, owned by the puzzle 2d plugin and
/// installed on every puzzle board ([`BoardHost::set_naming`]) and fill ingress (`BoardFillSnapshotIngress::new`).
pub static PUZZLE_2D_BOARD_NAMING: BoardNaming = BoardNaming {
    brush_node_prefix: "puzzle2d.brush.",
    brush_edge_prefix: "puzzle2d.brush.edge.",
    fill_node_prefix: "puzzle2d.fill.",
    fill_edge_prefix: "puzzle2d.fill.edge.",
    fill_stage_label: puzzle_2d_fill_stage_label,
    fill_stale_operation_fault: "stale-puzzle2d-fill-operation",
};

/// 🚦️ The job progress stage a puzzle board's fill reports for one board fill stage.
fn puzzle_2d_fill_stage_label(stage: BoardFillStage) -> &'static str {
    match stage {
        BoardFillStage::ResetSources => "puzzle2d-fill-reset-sources",
        BoardFillStage::PrepareSources => "puzzle2d-fill-prepare-sources",
        BoardFillStage::SelectTarget => "puzzle2d-fill-select-target",
        BoardFillStage::ResetCandidates => "puzzle2d-fill-reset-candidates",
        BoardFillStage::PrepareCandidates => "puzzle2d-fill-prepare-candidates",
        BoardFillStage::ScanCompatibility => "puzzle2d-fill-scan-compatibility",
        BoardFillStage::SelectCandidate => "puzzle2d-fill-select-candidate",
        BoardFillStage::ConstructPreview => "puzzle2d-fill-construct-preview",
        BoardFillStage::ScanHostCollision => "puzzle2d-fill-scan-host-collision",
        BoardFillStage::ScanVirtualCollision => "puzzle2d-fill-scan-virtual-collision",
        BoardFillStage::AcceptCandidate => "puzzle2d-fill-accept-candidate",
        BoardFillStage::AcceptNodeId => "puzzle2d-fill-accept-node-id",
        BoardFillStage::AcceptEdgeId => "puzzle2d-fill-accept-edge-id",
        BoardFillStage::AcceptEdgeKind => "puzzle2d-fill-accept-edge-kind",
        BoardFillStage::AcceptNodeKind => "puzzle2d-fill-accept-node-kind",
        BoardFillStage::AcceptSourceHandle => "puzzle2d-fill-accept-source-handle",
        BoardFillStage::AcceptTargetHandle => "puzzle2d-fill-accept-target-handle",
        BoardFillStage::AcceptIcon => "puzzle2d-fill-accept-icon",
        BoardFillStage::AcceptVirtualNode => "puzzle2d-fill-accept-virtual-node",
        BoardFillStage::AcceptSourceConnection => "puzzle2d-fill-accept-source-connection",
        BoardFillStage::AcceptHandles => "puzzle2d-fill-accept-handles",
        BoardFillStage::AcceptHandleId => "puzzle2d-fill-accept-handle-id",
        BoardFillStage::AcceptHandleVirtual => "puzzle2d-fill-accept-handle-virtual",
        BoardFillStage::AcceptHandlePublish => "puzzle2d-fill-accept-handle-publish",
        BoardFillStage::PublishPlanPrefix => "puzzle2d-fill-publish-prefix",
        BoardFillStage::Complete => "puzzle2d-fill-complete",
    }
}

/// 🎲️ A `BoardHost` for the directed port graph, painting icons from this plugin's metabolism table.
pub fn puzzle_board_host() -> BoardHost {
    let mut h = BoardHost::new();
    h.style.icon_paint_cache.themed_icon_lookup = puzzle_themed_icon_lookup;
    h.style.set_naming(&PUZZLE_2D_BOARD_NAMING);
    h
}

/// 🎲️ The undirected ("normal") variant of [`puzzle_board_host`].
pub fn puzzle_board_host_normal() -> BoardHost {
    let mut h = BoardHost::new_normal();
    h.style.icon_paint_cache.themed_icon_lookup = puzzle_themed_icon_lookup;
    h.style.set_naming(&PUZZLE_2D_BOARD_NAMING);
    h
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;

//#endregion 🧪️Tests
