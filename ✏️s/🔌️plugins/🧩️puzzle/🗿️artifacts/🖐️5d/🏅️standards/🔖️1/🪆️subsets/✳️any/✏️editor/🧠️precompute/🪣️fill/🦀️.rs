//! 🪣️ Puzzle 5d fill tool run jobs: the puzzle 3d fill planner's run and revalidate jobs over the 5d document,
//! behind the planner bridge's translation (`🧠️precompute`). The run's provisional 5d placements go to the planner
//! as its own `create_object`/`connect_vortices` pairs.

use crate::editor::puzzle5d::modes::edit::tools::fill::TOOL_ID;
use crate::editor::puzzle5d::precompute::{editor_part, puzzle3d_config, puzzle3d_object, puzzle3d_snapshot, puzzle3d_target_volume, puzzle5d_authored_kind_catalogs, Puzzle5dPlannerBoard, Puzzle5dPlannerToolRunJob};
use crate::editor::puzzle5d::{Puzzle5dDocument, Puzzle5dPlayApp};
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use semio_framework_plugin::{EditorApp, Fault, ToolRunJob, ToolRunJobRequest};
use semio_s_artifact_puzzle_3d::editor::puzzle3d::modes::edit::tools::fill as fill3d;
use semio_s_artifact_puzzle_3d::standards::v1::subsets::any::schema::mutations::{
    change_target_volume_hidden, change_target_volume_locked, connect_vortices, create_object, create_target_volume, delete_target_volume, move_target_volume, rotate_target_volume, scale_target_volume, Puzzle3dMutation,
};
use std::sync::Arc;

//#region 🔖️Build
/// 🧵️ Builds the fill run (`Run`) or finalize revalidation (`Revalidate`) job for the framework ledger; `None`
/// for any other tool.
pub fn build_run_job(request: ToolRunJobRequest<'_, EditorApp<Puzzle5dPlayApp>>) -> Result<Option<ToolRunJob>, Fault> {
    Ok(fill_run_job(request)?.map(|job| Box::new(job) as ToolRunJob))
}

/// 🧵️ The unboxed job behind [`build_run_job`].
pub(crate) fn fill_run_job(request: ToolRunJobRequest<'_, EditorApp<Puzzle5dPlayApp>>) -> Result<Option<Puzzle5dPlannerToolRunJob>, Fault> {
    if request.tool_id != TOOL_ID {
        return Ok(None);
    }
    let document: Puzzle5dDocument = serde_json::from_value(request.snapshot.0.clone()).map_err(|error| Fault::from(format!("puzzle5d-fill-run-document: {error}")))?;
    let provisional = puzzle3d_ops(request.provisional)?;
    let board = Puzzle5dPlannerBoard::new(&document, request.provisional)?;
    let snapshot = Arc::new(puzzle3d_snapshot(&document, puzzle5d_authored_kind_catalogs(&request.snapshot)?)?);
    let config = Arc::new(puzzle3d_config(&request.config));
    let inner = fill3d::build_run_job(ToolRunJobRequest {
        tool_id: fill3d::TOOL_ID,
        definition: request.definition,
        purpose: request.purpose,
        identity: request.identity,
        snapshot,
        config,
        window_id: request.window_id,
        window_config: request.window_config,
        checkpoint: request.checkpoint,
        provisional: &provisional,
        instance_owner: request.instance_owner,
        port: request.port,
        trace_keys: request.trace_keys,
        entity_marks: request.entity_marks,
    })?
    .ok_or_else(|| Fault::from("puzzle5d-fill-run-planner-missing"))?;
    Ok(Some(Puzzle5dPlannerToolRunJob::new(inner, board)))
}

/// 🧬️ A run's provisional 5d ops as the planner's own ops, pair for pair; any other op is not a fill placement.
fn puzzle3d_ops(provisional: &[Puzzle5dMutation]) -> Result<Vec<Puzzle3dMutation>, Fault> {
    provisional
        .iter()
        .map(|mutation| match mutation {
            Puzzle5dMutation::CreatePart(create) => Ok(create_object(puzzle3d_object(&editor_part(&create.part)?, None), None)),
            Puzzle5dMutation::ConnectGrips(connect) => Ok(connect_vortices(connect.id.clone(), connect.source.clone(), connect.target.clone(), connect.gap, connect.shift, connect.rise, connect.rotation, connect.turn, connect.tilt, connect.x, connect.y)),
            Puzzle5dMutation::CreateTargetVolume(create) => Ok(create_target_volume(puzzle3d_target_volume(&editor_target_volume(&create.target_volume)?), create.index)),
            Puzzle5dMutation::DeleteTargetVolume(delete) => Ok(delete_target_volume(delete.id.clone())),
            Puzzle5dMutation::MoveTargetVolume(moved) => Ok(move_target_volume(moved.id.clone(), moved.new_origin)),
            Puzzle5dMutation::RotateTargetVolume(rotated) => Ok(rotate_target_volume(rotated.id.clone(), rotated.new_orientation)),
            Puzzle5dMutation::ScaleTargetVolume(scaled) => Ok(scale_target_volume(scaled.id.clone(), scaled.new_scale.map(puzzle3d_scale))),
            Puzzle5dMutation::ChangeTargetVolumeHidden(changed) => Ok(change_target_volume_hidden(changed.id.clone(), changed.new_hidden)),
            Puzzle5dMutation::ChangeTargetVolumeLocked(changed) => Ok(change_target_volume_locked(changed.id.clone(), changed.new_locked)),
            _ => Err(Fault::from("puzzle5d-fill-run-provisional")),
        })
        .collect()
}

/// 🧊️ One schema target volume as the editor's own document type — the same `serde_json` hop
/// `editor_part` takes for a part, so the planner bridge reads one shape whichever side minted it.
fn editor_target_volume(volume: &crate::Puzzle5dTargetVolume) -> Result<crate::editor::puzzle5d::Puzzle5dTargetVolume, Fault> {
    serde_json::from_value(serde_json::Value::from(&dsl::ToValue::to_value(volume))).map_err(|error| Fault::from(format!("puzzle5d-planner-target-volume: {error}")))
}

/// 📏️ A schema-side `Puzzle5dScale` as the planner's own scale union.
fn puzzle3d_scale(scale: crate::Puzzle5dScale) -> semio_s_artifact_puzzle_3d::Puzzle3dScale {
    match scale {
        crate::Puzzle5dScale::Uniform(factor) => semio_s_artifact_puzzle_3d::Puzzle3dScale::Uniform(factor),
        crate::Puzzle5dScale::Vec3(axes) => semio_s_artifact_puzzle_3d::Puzzle3dScale::Vec3(axes),
    }
}

//#endregion 🔖️Build

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
//#endregion 🧪️Tests
