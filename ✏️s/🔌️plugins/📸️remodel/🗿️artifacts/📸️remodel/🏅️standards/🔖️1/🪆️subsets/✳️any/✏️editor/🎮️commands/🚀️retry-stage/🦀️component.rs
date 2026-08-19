//! 🚀️ 🚀️ Remodel play app commands command — `retry-stage`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::editor::remodel::engine::{build_engine_params, build_qc_snapshot, camera_pose_preview, raster_to_png_asset, reconstruction as remodel_engine, watertight_snapshot};
use crate::editor::remodel::decode_still_image;
use crate::artifacts::remodel::mutations::{create_asset, replace_geo_products, replace_job, replace_mesh_result, replace_qc, replace_sparse, replace_trajectory};
use crate::artifacts::remodel::schema::next_remodel_id;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{CameraPosePreview, CameraTrajectory, GeoProducts, ImageAsset, MeshSource, PackedF32, ReconstructionJob, ReconstructionStage, RemodelMesh, RemodelSnapshot, SparseCloud};
use base64::Engine as _;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
/// ⚙️ Bounded units of engine work performed per internal `advance()` call within one synchronous
/// `RunReconstruction` — small enough that no single `advance` call does an unreasonable burst of work.
const RECONSTRUCTION_STEP_BUDGET: usize = 8;
/// 🛑️ Pure-function totality safety valve for the synchronous reconstruction loop (see the module doc
/// comment): a real project's total ticks is bounded by its frame/point/triangle counts, never this —
/// this only guards against an engine bug spinning `handle()` forever.
const REMODEL_MAX_RECONSTRUCTION_TICKS: u32 = 200_000;
//#endregion 🔖️Constants

//#region 🔖️Run
/// 🚀️ Validates ≥2 accepted frames, builds an engine from the current document params, pushes every
/// stream's already-persisted frames into it, then loops `advance()` in-process until `Done`/`Failed`
/// and returns exactly one `Emit` carrying only the FINAL state — one call, one `Emit`, one undo step;
/// no coalesce key needed. Shared by all three rows in this group.
pub async fn run_whole_pipeline(doc: &ArtifactView<'_, RemodelSnapshot>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    let scene = doc.snapshot;
    let engine_params = build_engine_params(&scene.params);
    let mut engine = remodel_engine::ReconstructionEngine::new(&engine_params);
    let mut pushed = 0u32;
    for stream in &scene.streams {
        for frame_ref in &stream.frames {
            let Some(asset) = crate::artifacts::remodel::remodel_asset(&scene.assets, &frame_ref.asset_id) else { continue };
            let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(&asset.data) else { continue };
            if let Ok(image) = decode_still_image(&asset.mime, &bytes) {
                engine.push_frame(frame_ref.index, image, frame_ref.timestamp_ms);
                pushed += 1;
            }
        }
    }
    if pushed < 2 {
        return Ok(Emit::default()); // fewer than 2 accepted frames: too little to reconstruct from
    }
    let job_id = next_remodel_id("job");
    let mut last_progress = 0.0f32;
    let mut ticks = 0u32;
    loop {
        ticks += 1;
        if ticks > REMODEL_MAX_RECONSTRUCTION_TICKS {
            let job = ReconstructionJob {
                id: job_id,
                stage: ReconstructionStage::Failed,
                progress_0_1: last_progress,
                cancel_requested: false,
                stage_cursor: 0,
                started_at_ms: None,
                error: Some("reconstruction did not converge within the bounded tick budget".into()),
                camera_poses_preview: Vec::new(),
                sparse_point_cloud_preview: PackedF32::default(),
            };
            return Ok(Emit::mutations(vec![replace_job(job)]));
        }
        match engine.advance(RECONSTRUCTION_STEP_BUDGET) {
            remodel_engine::EngineStatus::Working { progress, .. } => {
                last_progress = progress;
            }
            remodel_engine::EngineStatus::Done => {
                let accepted_count = engine.frame_source().accepted_count();
                let preview = engine.sparse_preview();
                let quality = engine.take_quality();
                let mesh_data = engine.take_mesh();
                let geo_products = engine.take_geo_products();

                let registered_count = preview.camera_poses.len();
                let camera_previews: Vec<CameraPosePreview> = preview.camera_poses.iter().enumerate().map(|(index, pose)| camera_pose_preview(index as u32, pose)).collect();

                let job = ReconstructionJob {
                    id: job_id.clone(),
                    stage: ReconstructionStage::Done,
                    progress_0_1: 1.0,
                    cancel_requested: false,
                    stage_cursor: 0,
                    started_at_ms: None,
                    error: None,
                    camera_poses_preview: camera_previews.clone(),
                    sparse_point_cloud_preview: PackedF32::from_f32_slice(&preview.packed_points),
                };

                let mut operations = vec![replace_job(job)];
                operations.push(replace_sparse(Some(SparseCloud { points: PackedF32::from_f32_slice(&preview.packed_points), colors: None })));
                if !camera_previews.is_empty() {
                    operations.push(replace_trajectory(Some(CameraTrajectory { poses: camera_previews })));
                }
                if let Some(mesh_data) = mesh_data {
                    let watertight = quality.as_ref().and_then(|quality| quality.watertight.as_ref()).map(watertight_snapshot);
                    let mut texture_asset_id = None;
                    if let Some(texture) = &mesh_data.paint_texture_base64 {
                        let texture_size = scene.params.mesh.texture_size;
                        let asset_id = format!("mesh-texture-{job_id}");
                        operations.push(create_asset(asset_id.clone(), ImageAsset { mime: "image/png".into(), data: texture.clone(), width: texture_size, height: texture_size }));
                        texture_asset_id = Some(asset_id);
                    }
                    operations.push(replace_mesh_result(Box::new(RemodelMesh { mesh: crate::artifacts::remodel::mint_and_stash_mesh(mesh_data), source: MeshSource::Reconstructed, texture_asset_id, watertight })));
                }
                if let Some(quality) = &quality {
                    operations.push(replace_qc(Some(build_qc_snapshot(quality, registered_count, accepted_count, scene.gcps.len()))));
                }
                if let Some(geo) = geo_products {
                    let dsm_id = format!("geo-dsm-{job_id}");
                    let dtm_id = format!("geo-dtm-{job_id}");
                    operations.push(create_asset(dsm_id.clone(), raster_to_png_asset(&geo.dsm)));
                    operations.push(create_asset(dtm_id.clone(), raster_to_png_asset(&geo.dtm)));
                    operations.push(replace_geo_products(Some(GeoProducts { dsm_asset_id: Some(dsm_id), dtm_asset_id: Some(dtm_id), ortho_asset_id: None })));
                }
                return Ok(Emit::mutations(operations));
            }
            remodel_engine::EngineStatus::Failed(message) => {
                let job = ReconstructionJob {
                    id: job_id,
                    stage: ReconstructionStage::Failed,
                    progress_0_1: last_progress,
                    cancel_requested: false,
                    stage_cursor: 0,
                    started_at_ms: None,
                    error: Some(message),
                    camera_poses_preview: Vec::new(),
                    sparse_point_cloud_preview: PackedF32::default(),
                };
                return Ok(Emit::mutations(vec![replace_job(job)]));
            }
        }
    }
}
//#endregion 🔖️Run

//#region 🔖️RunReconstruction
//#endregion 🔖️RunReconstruction

//#region 🔖️RetryStage
//#endregion 🔖️RetryStage

//#region 🔖️RunStage
//#endregion 🔖️RunStage

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "retry-stage")]
pub struct RetryStage {
    pub stage: String,
}

/// 🔁️ A retry is a fresh whole run (see the module doc comment): resuming at `payload.stage` would
/// need the mid-pipeline engine state the pure-trait pivot removed, so the stage name is accepted and ignored.
pub async fn handle(_payload: &RetryStage, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    run_whole_pipeline(doc)
}
