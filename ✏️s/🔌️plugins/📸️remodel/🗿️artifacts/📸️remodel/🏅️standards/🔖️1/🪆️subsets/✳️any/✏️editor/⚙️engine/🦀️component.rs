//! ⚙️ Remodel app engine — the `RemodelPlayApp`'s own compute: bridges between the document's
//! `RemodelSnapshot` param/result types and the ten sibling photogrammetry topic modules
//! (`images`/`video`/`camera`/`feature`/`sfm`/`dense`/`mesh`/`motion`/`geo`/`reconstruction`), none of
//! which reference `RemodelSnapshot` themselves (pure numeric/geometry algorithms).
//!
//! 🧭️ Relocated from the artifact's `⚙️engine/` (26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES,
//! #2553): an artifact is a schema plus IO, never an engine — behaviour belongs to the app that edits
//! it. Every function here takes or returns one of the ten topic modules' own types, which is exactly
//! why it cannot live in `🧬️schema/` (artifacts must never depend on apps) and instead lives beside the
//! topic modules it bridges.

use crate::artifacts::remodel::{CameraPosePreview, DenseResolution, ImageAsset, QcReportSnapshot, ReconstructionParams, ReconstructionStage, RobustLossKind, VideoCodec as DocumentVideoCodec, WatertightReportSnapshot};
use crate::editor::remodel::engine::{camera as remodel_camera, geo as remodel_geo, images as remodel_image, mesh as remodel_mesh, reconstruction as remodel_engine, sfm as remodel_sfm, video as remodel_video};
use base64::Engine as _;

//#region 🔖️EngineMapping
/// ⚙️ Builds `remodel_engine::EngineParams` from the document's 8 param sub-structs. Fields with no
/// engine-side counterpart (`SfmParams::ransac_iterations` — the LO-RANSAC solver doesn't expose an
/// iteration cap; `GeoParams::origin_*` — no georeferencing-origin knob exists on the engine side;
/// `EngineParams::assumed_focal_ratio` — no document field feeds it, calibration is not yet a wired
/// stage) are documented simplifications, not oversights.
pub fn build_engine_params(params: &ReconstructionParams) -> remodel_engine::EngineParams {
    let mut engine_params = remodel_engine::EngineParams::default();
    engine_params.ingest.stride = params.ingest.frame_sample_stride.max(1);
    engine_params.ingest.max_frames = params.ingest.max_frames;
    engine_params.ingest.min_sharpness = params.ingest.min_sharpness;
    engine_params.target_feature_count = params.feature.target_count as usize;
    engine_params.match_ratio = params.matching.ratio_test;
    engine_params.match_mutual = params.matching.cross_check;
    engine_params.sequential_window = params.matching.sequential_window.max(1) as usize;
    engine_params.sfm.ransac_threshold_px = f64::from(params.sfm.ransac_threshold_px);
    engine_params.sfm.min_track_length = params.sfm.min_track_length as usize;
    engine_params.sfm.ba_max_iterations = params.sfm.ba_max_iterations as usize;
    engine_params.sfm.robust_loss = match params.sfm.robust_loss {
        RobustLossKind::L2 => remodel_sfm::RobustLoss::Trivial,
        RobustLossKind::Huber => remodel_sfm::RobustLoss::Huber(f64::from(params.sfm.huber_delta_px)),
        RobustLossKind::Cauchy => remodel_sfm::RobustLoss::Cauchy(f64::from(params.sfm.huber_delta_px)),
    };
    engine_params.dense.window_radius = params.dense.window_radius_px as i32;
    engine_params.dense.iterations = match params.dense.resolution {
        DenseResolution::Low => 2,
        DenseResolution::Medium => 4,
        DenseResolution::High => 8,
    };
    engine_params.dense_source_views = params.dense.min_view_consistency.max(1) as usize;
    engine_params.tsdf_voxel_size = f64::from(params.mesh.tsdf_voxel_size_mm) / 1000.0;
    engine_params.tsdf_truncation = f64::from(params.mesh.tsdf_truncation_mm) / 1000.0;
    engine_params.mesh.target_triangles = if params.mesh.decimate_target_triangles == 0 { usize::MAX } else { params.mesh.decimate_target_triangles as usize };
    engine_params.mesh.taubin_iterations = params.mesh.smoothing_iterations as usize;
    engine_params.mesh.atlas_size = params.mesh.texture_size;
    engine_params.mesh.guarantee_watertight = params.mesh.guarantee_watertight;
    engine_params.mesh.hole_fill_max_boundary_verts = params.mesh.hole_fill_max_boundary_verts as usize;
    engine_params.mesh.self_intersection_check = params.mesh.self_intersection_check;
    engine_params.texture_enabled = params.mesh.texture_enabled;
    engine_params.motion_enabled = params.motion.enabled;
    engine_params.geo_enabled = params.geo.enabled;
    engine_params.geo_cell_size = f64::from(params.geo.dsm_cell_m);
    engine_params
}

pub fn map_engine_stage(stage: remodel_engine::EngineStage) -> ReconstructionStage {
    match stage {
        remodel_engine::EngineStage::Idle => ReconstructionStage::Idle,
        remodel_engine::EngineStage::ExtractingFeatures => ReconstructionStage::ExtractingFeatures,
        remodel_engine::EngineStage::MatchingFeatures => ReconstructionStage::MatchingFeatures,
        remodel_engine::EngineStage::EstimatingPoses => ReconstructionStage::EstimatingPoses,
        remodel_engine::EngineStage::BundleAdjusting => ReconstructionStage::BundleAdjusting,
        remodel_engine::EngineStage::DenseStereo => ReconstructionStage::DenseStereo,
        remodel_engine::EngineStage::FusingVolume => ReconstructionStage::FusingVolume,
        remodel_engine::EngineStage::ExtractingSurface => ReconstructionStage::ExtractingSurface,
        remodel_engine::EngineStage::CleaningMesh => ReconstructionStage::CleaningMesh,
        remodel_engine::EngineStage::Texturing => ReconstructionStage::Texturing,
        remodel_engine::EngineStage::Done => ReconstructionStage::Done,
        remodel_engine::EngineStage::Failed => ReconstructionStage::Failed,
    }
}

/// 📍️ World-space camera center: `inverse().t` of the world→camera `Se3` (i.e. `-R⁻¹·t`).
fn camera_world_position(pose: &remodel_camera::CameraPose) -> [f32; 3] {
    let center = pose.0.inverse().t;
    [center[0] as f32, center[1] as f32, center[2] as f32]
}

/// 🔭️ A `CameraPosePreview` snapshot of one registration-order camera pose. `camera_id` isn't
/// recoverable from `ReconstructionEngine::sparse_preview` (it only exposes poses by registration
/// order, not by originating stream/frame/calibration id) — a documented simplification.
pub fn camera_pose_preview(index: u32, pose: &remodel_camera::CameraPose) -> CameraPosePreview {
    let translation = camera_world_position(pose);
    let quat = pose.0.inverse().r.to_quat();
    CameraPosePreview { camera_id: format!("cam-{index}"), rotation_wxyz: [quat.w as f32, quat.x as f32, quat.y as f32, quat.z as f32], translation }
}

pub fn watertight_snapshot(report: &remodel_mesh::WatertightReport) -> WatertightReportSnapshot {
    WatertightReportSnapshot {
        vertex_count: report.vertex_count as u32,
        triangle_count: report.triangle_count as u32,
        boundary_edge_count: report.boundary_edge_count as u32,
        boundary_loop_count: report.boundary_loop_count as u32,
        non_manifold_edge_count: report.non_manifold_edge_count as u32,
        non_manifold_vertex_count: report.non_manifold_vertex_count as u32,
        connected_components: report.connected_components as u32,
        consistently_oriented: report.consistently_oriented,
        euler_characteristic: report.euler_characteristic,
        genus: report.genus,
        signed_volume: report.signed_volume,
        self_intersection_pairs: report.self_intersection_pairs.map(|value| value as u32),
        closed_fallback_used: report.closed_fallback_used,
        is_closed: report.is_closed,
        is_two_manifold: report.is_two_manifold,
        is_watertight: report.is_watertight,
    }
}

/// ✅️ Maps `remodel_geo::QualityReport` (plus counts only the plugin can compute — accepted vs.
/// registered frames, whether any GCPs exist) onto the document's `QcReportSnapshot`.
/// `dense_coverage_ratio` stays `0.0`: the engine doesn't compute a density/overlap raster today —
/// a documented gap.
pub fn build_qc_snapshot(quality: &remodel_geo::QualityReport, registered_count: usize, accepted_count: usize, gcp_count: usize) -> QcReportSnapshot {
    let mut warnings = Vec::new();
    if let Some(watertight) = &quality.watertight {
        if !watertight.is_watertight {
            warnings.push("Mesh is not watertight.".to_string());
        }
    }
    if gcp_count > 0 && quality.gcp_checkpoint_rmse.is_none() {
        warnings.push("Ground control points are set but checkpoint RMSE is not yet computed.".to_string());
    }
    QcReportSnapshot {
        reprojection_rms_px: quality.reprojection_rms_px,
        gcp_checkpoint_rmse: quality.gcp_checkpoint_rmse,
        watertight: quality.watertight.as_ref().map(watertight_snapshot),
        mean_track_length: quality.track_stats.mean_track_length as f32,
        registered_frame_ratio: if accepted_count == 0 { 0.0 } else { registered_count as f32 / accepted_count as f32 },
        dense_coverage_ratio: 0.0,
        warnings,
    }
}

/// 🗺️ Normalizes a `remodel_geo::Raster`'s valid cells to a 16-bit grayscale PNG `ImageAsset` (invalid
/// cells encode as `0`) — `remodel_image::encode_png_gray16` is the shared 16-bit writer every other
/// raster-shaped asset in this codebase uses.
pub fn raster_to_png_asset(raster: &remodel_geo::Raster) -> ImageAsset {
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    for (index, &value) in raster.values.iter().enumerate() {
        if raster.valid.get(index).copied().unwrap_or(false) {
            min = min.min(value);
            max = max.max(value);
        }
    }
    if !min.is_finite() || !max.is_finite() {
        min = 0.0;
        max = 0.0;
    }
    let span = (max - min).max(1e-6);
    let data: Vec<u16> = raster.values.iter().enumerate().map(|(index, &value)| if raster.valid.get(index).copied().unwrap_or(false) { (((value - min) / span) * 65535.0).round().clamp(0.0, 65535.0) as u16 } else { 0 }).collect();
    let bytes = remodel_image::encode_png_gray16(&data, raster.width, raster.height).unwrap_or_default();
    ImageAsset { mime: "image/png".into(), data: base64::engine::general_purpose::STANDARD.encode(bytes), width: raster.width, height: raster.height }
}

pub fn video_codec_to_artifact(codec: remodel_video::VideoCodec) -> DocumentVideoCodec {
    match codec {
        remodel_video::VideoCodec::Avc => DocumentVideoCodec::Avc,
        remodel_video::VideoCodec::Hevc => DocumentVideoCodec::Hevc,
        remodel_video::VideoCodec::Vp9 => DocumentVideoCodec::Vp9,
        remodel_video::VideoCodec::Av1 => DocumentVideoCodec::Av1,
        remodel_video::VideoCodec::Mjpeg => DocumentVideoCodec::Mjpeg,
        remodel_video::VideoCodec::Unknown(_) => DocumentVideoCodec::Unknown,
    }
}

/// 🏷️ `(codec, width, height, duration_ms, container)` from either container family's probe.
pub fn describe_video_probe(probe: &remodel_video::VideoProbe) -> (remodel_video::VideoCodec, u32, u32, f64, &'static str) {
    match probe {
        remodel_video::VideoProbe::Mp4(info) => (info.codec, info.width, info.height, info.duration_ms, "mp4"),
        remodel_video::VideoProbe::Avi(info) => {
            let duration_ms = if info.fps > 0.0 { f64::from(info.frame_count) / info.fps * 1000.0 } else { 0.0 };
            (info.codec, info.width, info.height, duration_ms, "avi")
        }
    }
}
//#endregion 🔖️EngineMapping

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raster_to_png_asset_normalizes_valid_cells_only() {
        let mut raster = remodel_geo::Raster::new(2, 2, 1.0, [0.0, 0.0]);
        raster.set(0, 0, 0.0);
        raster.set(1, 1, 10.0);
        let asset = raster_to_png_asset(&raster);
        assert_eq!(asset.mime, "image/png");
        assert_eq!(asset.width, 2);
        assert_eq!(asset.height, 2);
        assert!(!asset.data.is_empty());
    }
}
//#endregion 🧪️Tests
