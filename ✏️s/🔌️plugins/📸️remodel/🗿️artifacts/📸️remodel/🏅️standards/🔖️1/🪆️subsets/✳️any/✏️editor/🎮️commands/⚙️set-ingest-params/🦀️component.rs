//! ⚙️ ⚙️ Remodel play app commands command — `set-ingest-params`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::mutations::{update_dense_params, update_feature_params, update_geo_params, update_ingest_params, update_match_params, update_mesh_params, update_motion_params, update_sfm_params};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{DenseParams, DenseResolution, FeatureDetector, FeatureParams, GeoParams, IngestParams, MatchParams, MatcherKind, MeshParams, MotionParams, RemodelSnapshot, RobustLossKind, SfmParams};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "ingest-params")]
pub struct SetIngestParams {
    pub frame_sample_stride: u32,
    pub max_frames: u32,
    pub downscale_long_edge_px: u32,
    pub min_sharpness: f32,
}

pub fn handle(payload: &SetIngestParams, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![update_ingest_params(IngestParams {
        frame_sample_stride: payload.frame_sample_stride,
        max_frames: payload.max_frames,
        downscale_long_edge_px: payload.downscale_long_edge_px,
        min_sharpness: payload.min_sharpness,
    })]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::remodel::commands::{set_dense_params, set_feature_params, set_geo_params, set_match_params, set_mesh_params, set_motion_params, set_sfm_params};
    use crate::editor::remodel::testkit::{app, dispatch};
    use crate::editor::remodel::RemodelCommand;

    #[test]
    fn set_sfm_params_command_materializes_typed_fields_into_operations() {
        let mut app = app();
        let result = dispatch(&mut app, RemodelCommand::SetSfmParams(set_sfm_params::SetSfmParams { ransac_iterations: 500, ransac_threshold_px: 1.5, min_track_length: 4, ba_max_iterations: 20, robust_loss: "cauchy".into(), huber_delta_px: 2.5 }));
        assert_eq!(result.mutations.len(), 1, "typed command produces one SetSfmParams operation");
        let params = app.snapshot().expect("materialize projection").params.sfm;
        assert_eq!(params.ransac_iterations, 500);
        assert_eq!(params.min_track_length, 4);
        assert_eq!(params.ba_max_iterations, 20);
        assert_eq!(params.robust_loss, RobustLossKind::Cauchy);
    }

    #[test]
    fn set_geo_params_command_materializes_typed_fields_into_operations() {
        let mut app = app();
        dispatch(&mut app, RemodelCommand::SetGeoParams(set_geo_params::SetGeoParams { enabled: true, origin_lon: None, origin_lat: None, origin_alt: None, gsd_m: 0.02, dsm_cell_m: 0.2, dtm_filter_radius_m: 2.0, ortho_max_px: 2048 }));
        let params = app.snapshot().expect("materialize projection").params.geo;
        assert!(params.enabled);
        assert_eq!(params.gsd_m, 0.02);
        assert_eq!(params.dsm_cell_m, 0.2);
        assert_eq!(params.ortho_max_px, 2048);
    }

    #[test]
    fn set_mesh_params_command_materializes_watertight_knobs() {
        let mut app = app();
        dispatch(
            &mut app,
            RemodelCommand::SetMeshParams(set_mesh_params::SetMeshParams {
                tsdf_voxel_size_mm: 3.0,
                tsdf_truncation_mm: 20.0,
                decimate_target_triangles: 200_000,
                smoothing_iterations: 2,
                texture_enabled: true,
                texture_size: 2048,
                guarantee_watertight: false,
                hole_fill_max_boundary_verts: 256,
                self_intersection_check: true,
            }),
        );
        let params = app.snapshot().expect("materialize projection").params.mesh;
        assert_eq!(params.tsdf_voxel_size_mm, 3.0);
        assert!(!params.guarantee_watertight);
        assert_eq!(params.hole_fill_max_boundary_verts, 256);
        assert!(params.self_intersection_check);
    }

    #[test]
    fn set_ingest_params_command_materializes_min_sharpness() {
        let mut app = app();
        dispatch(&mut app, RemodelCommand::SetIngestParams(SetIngestParams { frame_sample_stride: 5, max_frames: 200, downscale_long_edge_px: 1600, min_sharpness: 0.42 }));
        assert_eq!(app.snapshot().expect("materialize projection").params.ingest.min_sharpness, 0.42);
    }

    /// 🔤️ The three string-keyed enum fields fall back to their documented defaults on an unknown value
    /// rather than failing the dispatch.
    #[test]
    fn unknown_enum_keywords_fall_back_to_the_documented_defaults() {
        let mut app = app();
        dispatch(&mut app, RemodelCommand::SetFeatureParams(set_feature_params::SetFeatureParams { detector: "nonsense".into(), target_count: 10, octaves: 1, edge_threshold: 1.0 }));
        dispatch(&mut app, RemodelCommand::SetMatchParams(set_match_params::SetMatchParams { matcher: "nonsense".into(), ratio_test: 0.5, cross_check: false, sequential_window: 1, max_pairs_per_frame: 1, loop_closure: false }));
        dispatch(&mut app, RemodelCommand::SetDenseParams(set_dense_params::SetDenseParams { resolution: "nonsense".into(), window_radius_px: 1, min_view_consistency: 1, confidence_threshold: 0.1, max_points: 10 }));
        let params = app.snapshot().expect("materialize projection").params;
        assert_eq!(params.feature.detector, FeatureDetector::Orb);
        assert_eq!(params.matching.matcher, MatcherKind::BruteForce);
        assert_eq!(params.dense.resolution, DenseResolution::Medium);
    }

    #[test]
    fn set_motion_params_command_materializes_typed_fields() {
        let mut app = app();
        dispatch(&mut app, RemodelCommand::SetMotionParams(set_motion_params::SetMotionParams { enabled: true, max_tracks: 32, track_window_px: 11, min_track_quality: 0.4, min_track_length_frames: 7 }));
        let params = app.snapshot().expect("materialize projection").params.motion;
        assert!(params.enabled);
        assert_eq!(params.max_tracks, 32);
        assert_eq!(params.min_track_length_frames, 7);
    }
}
//#endregion 🧪️Tests
