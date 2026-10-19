use semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
use semio_framework_tool_run::{ToolRunIdentity, ToolRunTick, ToolRunTraceOp, ToolRunTracePage, ToolRunTraceSubject, ToolRunVerdict};
#[allow(unused_imports)]
use crate::editor::puzzle5d::view::engine_grip_kind;
#[allow(unused_imports)]
use crate::editor::puzzle5d::config::Puzzle5dConfig;
#[allow(unused_imports)]
use crate::editor::puzzle5d::precompute::geometry::Puzzle5dPointGrid;
#[allow(unused_imports)]
use crate::editor::puzzle5d::entities::{collect_mesh_urls, grips_from_templates, puzzle5d_grip_full_id, resolve_part_mesh_url, world_grip_position, Puzzle5dDocument, Puzzle5dFastener, Puzzle5dGrip, Puzzle5dGrip2d, Puzzle5dGrip3d, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, PUZZLE5D_BOARD_PLACEMENT_GAP, PUZZLE5D_DEFAULT_PART_RADIUS, PUZZLE5D_FALLBACK_MESH_KIND};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::play_snapshot::Puzzle5dPlaySnapshot;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::{connect_grips, create_part, Puzzle5dMutation};
#[allow(unused_imports)]
use crate::puzzle_job::JobTurn;
#[allow(unused_imports)]
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, StepBudget, StepContext};
use semio_framework_job_payload::{RetainedJobPayload};
#[allow(unused_imports)]
use semio_framework_plugin::{Fault, ToolRunJob};
#[allow(unused_imports)]
use semio_framework_tool_run::TOOL_RUN_TRACE_PAGE_OPS_MAX;
#[allow(unused_imports)]
use semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfig;
#[allow(unused_imports)]
use semio_s_artifact_puzzle_3d_editor_modes::windows::main as world3d;
#[allow(unused_imports)]
use semio_s_artifact_puzzle_3d_editor::puzzle3d_scene_snapshot_from_document;
#[allow(unused_imports)]
use {semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::ConnectVortices, semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation};
#[allow(unused_imports)]
use semio_s_artifact_puzzle_3d_model::BrushPreviewState;
#[allow(unused_imports)]
use {semio_s_artifact_puzzle_3d_model::Puzzle3dAttraction, semio_s_artifact_puzzle_3d_model::Puzzle3dCatalogObjectKind, semio_s_artifact_puzzle_3d_model::Puzzle3dCatalogVortexKind, semio_s_artifact_puzzle_3d_model::Puzzle3dCatalogVortexTemplate, semio_s_artifact_puzzle_3d_model::Puzzle3dKindCatalogs, semio_s_artifact_puzzle_3d_model::Puzzle3dMeta, semio_s_artifact_puzzle_3d_model::Puzzle3dObject, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot, semio_s_artifact_puzzle_3d_model::Puzzle3dRepresentation, semio_s_artifact_puzzle_3d_model::Puzzle3dScale, semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot, semio_s_artifact_puzzle_3d_model::Puzzle3dVortex, semio_s_artifact_puzzle_3d_model::PUZZLE_3D_SCHEMA};
#[allow(unused_imports)]
use std::collections::{HashMap, VecDeque};
use crate::editor::puzzle5d::precompute::*;
use semio_framework_tool_run::ToolRunId;
use semio_framework_tool_run::ToolRunStep;
use semio_framework_tool_run::ToolRunStepKind;

fn upsert(key: u64) -> ToolRunTraceOp {
    ToolRunTraceOp::Upsert { key, verdict: ToolRunVerdict::Testing, reason: 0, subject: ToolRunTraceSubject::Instance3d { mesh: 1, position: [key as f32, 0.0, 0.0], rotation: [0.0, 0.0, 0.0, 1.0], scale: 1.0 } }
}

/// 📃️ A translated tick too large for one job payload page splits into consecutive ticks that each fit one page,
/// replay every trace record in order, carry the retraction, placements and entities only on the first and the
/// steps and plugin payload only on the last; a tick that fits stays one tick byte for byte.
#[test]
fn a_tick_over_one_payload_page_splits_into_in_order_pages() {
    let identity = ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 7 }, [3; 32]);
    let step = ToolRunStep { sequence: 1, kind: ToolRunStepKind::Info, stage: 0, reason: 0, subject: None, repeat: 1, args: Vec::new() };
    let ops: Vec<ToolRunTraceOp> = (0..2_000).map(upsert).collect();
    let tick = ToolRunTick { identity, sequence: 9, progress: None, steps: vec![step.clone()], trace: vec![ToolRunTracePage { identity, page: 0, ops: ops.clone() }], append_ops: vec![vec![1, 2, 3], vec![4]], append_entities: vec![11], retract_to: Some(2), payload: Some(vec![5, 6]),payload_update:Some(semio_framework_tool_run::ToolRunPayloadUpdate::Replace) };
    let whole = tick.encode().expect("whole tick encodes");
    assert!(whole.len() > JOB_PAYLOAD_PAGE_BYTES, "the sample must overflow one page");
    let pages = puzzle5d_planner_tick_pages(tick).expect("split");
    assert!(pages.len() > 1);
    let ticks: Vec<ToolRunTick> = pages.iter().map(|bytes| {
        assert!(bytes.len() <= JOB_PAYLOAD_PAGE_BYTES);
        ToolRunTick::decode(bytes).expect("page decodes")
    }).collect();
    assert_eq!(ticks.iter().flat_map(|tick| tick.trace.iter().flat_map(|page| page.ops.iter().copied())).collect::<Vec<_>>(), ops);
    assert_eq!((ticks[0].append_ops.len(), ticks[0].append_entities.as_slice(), ticks[0].retract_to), (2, &[11][..], Some(2)));
    assert!(ticks[1..].iter().all(|tick| tick.append_ops.is_empty() && tick.append_entities.is_empty() && tick.retract_to.is_none()));
    assert_eq!((ticks.last().expect("last").steps.clone(), ticks.last().expect("last").payload.clone()), (vec![step], Some(vec![5, 6])));
    assert!(ticks[..ticks.len() - 1].iter().all(|tick| tick.steps.is_empty() && tick.payload.is_none()));
    let small = ToolRunTick { identity, sequence: 1, progress: None, steps: Vec::new(), trace: vec![ToolRunTracePage { identity, page: 0, ops: vec![upsert(1)] }], append_ops: Vec::new(), append_entities: Vec::new(), retract_to: None, payload: None,payload_update:None };
    assert_eq!(puzzle5d_planner_tick_pages(small.clone()).expect("small"), vec![small.encode().expect("small encodes")]);
}
