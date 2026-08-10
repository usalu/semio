//! ⚙️ Remodel play app commands — the 8 reconstruction-parameter setters, one per `ReconstructionParams`
//! sub-struct. Each row takes the sub-struct's fields flat (the palette arg form's shape) and emits the
//! single LWW `Set<Group>Params` operation that replaces it.

use crate::apps::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{DenseParams, DenseResolution, FeatureDetector, FeatureParams, GeoParams, IngestParams, MatchParams, MatcherKind, MeshParams, MotionParams, RemodelSnapshot, RobustLossKind, SfmParams};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetIngestParams
pub mod set_ingest_params {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "ingest-params")]
    pub struct SetIngestParams {
        pub frame_sample_stride: u32,
        pub max_frames: u32,
        pub downscale_long_edge_px: u32,
        pub min_sharpness: f32,
    }

    pub fn handle(payload: &SetIngestParams, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![RemodelMutation::SetIngestParams {
            params: IngestParams { frame_sample_stride: payload.frame_sample_stride, max_frames: payload.max_frames, downscale_long_edge_px: payload.downscale_long_edge_px, min_sharpness: payload.min_sharpness },
        }]))
    }
}
//#endregion 🔖️SetIngestParams

//#region 🔖️SetFeatureParams
pub mod set_feature_params {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "feature-params")]
    pub struct SetFeatureParams {
        pub detector: String,
        pub target_count: u32,
        pub octaves: u32,
        pub edge_threshold: f32,
    }

    pub fn handle(payload: &SetFeatureParams, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![RemodelMutation::SetFeatureParams {
            params: FeatureParams {
                detector: match payload.detector.as_str() {
                    "akaze" => FeatureDetector::Akaze,
                    "harris" => FeatureDetector::Harris,
                    _ => FeatureDetector::Orb,
                },
                target_count: payload.target_count,
                octaves: payload.octaves,
                edge_threshold: payload.edge_threshold,
            },
        }]))
    }
}
//#endregion 🔖️SetFeatureParams

//#region 🔖️SetMatchParams
pub mod set_match_params {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "match-params")]
    pub struct SetMatchParams {
        pub matcher: String,
        pub ratio_test: f32,
        pub cross_check: bool,
        pub sequential_window: u32,
        pub max_pairs_per_frame: u32,
        pub loop_closure: bool,
    }

    pub fn handle(payload: &SetMatchParams, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![RemodelMutation::SetMatchParams {
            params: MatchParams {
                matcher: if payload.matcher == "kd-tree" { MatcherKind::KdTree } else { MatcherKind::BruteForce },
                ratio_test: payload.ratio_test,
                cross_check: payload.cross_check,
                sequential_window: payload.sequential_window,
                max_pairs_per_frame: payload.max_pairs_per_frame,
                loop_closure: payload.loop_closure,
            },
        }]))
    }
}
//#endregion 🔖️SetMatchParams

//#region 🔖️SetSfmParams
pub mod set_sfm_params {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "sfm-params")]
    pub struct SetSfmParams {
        pub ransac_iterations: u32,
        pub ransac_threshold_px: f32,
        pub min_track_length: u32,
        pub ba_max_iterations: u32,
        pub robust_loss: String,
        pub huber_delta_px: f32,
    }

    pub fn handle(payload: &SetSfmParams, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![RemodelMutation::SetSfmParams {
            params: SfmParams {
                ransac_iterations: payload.ransac_iterations,
                ransac_threshold_px: payload.ransac_threshold_px,
                min_track_length: payload.min_track_length,
                ba_max_iterations: payload.ba_max_iterations,
                robust_loss: match payload.robust_loss.as_str() {
                    "l2" => RobustLossKind::L2,
                    "cauchy" => RobustLossKind::Cauchy,
                    _ => RobustLossKind::Huber,
                },
                huber_delta_px: payload.huber_delta_px,
            },
        }]))
    }
}
//#endregion 🔖️SetSfmParams

//#region 🔖️SetDenseParams
pub mod set_dense_params {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "dense-params")]
    pub struct SetDenseParams {
        pub resolution: String,
        pub window_radius_px: u32,
        pub min_view_consistency: u32,
        pub confidence_threshold: f32,
        pub max_points: u32,
    }

    pub fn handle(payload: &SetDenseParams, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![RemodelMutation::SetDenseParams {
            params: DenseParams {
                resolution: match payload.resolution.as_str() {
                    "low" => DenseResolution::Low,
                    "high" => DenseResolution::High,
                    _ => DenseResolution::Medium,
                },
                window_radius_px: payload.window_radius_px,
                min_view_consistency: payload.min_view_consistency,
                confidence_threshold: payload.confidence_threshold,
                max_points: payload.max_points,
            },
        }]))
    }
}
//#endregion 🔖️SetDenseParams

//#region 🔖️SetMeshParams
pub mod set_mesh_params {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "mesh-params")]
    pub struct SetMeshParams {
        pub tsdf_voxel_size_mm: f32,
        pub tsdf_truncation_mm: f32,
        pub decimate_target_triangles: u32,
        pub smoothing_iterations: u32,
        pub texture_enabled: bool,
        pub texture_size: u32,
        pub guarantee_watertight: bool,
        pub hole_fill_max_boundary_verts: u32,
        pub self_intersection_check: bool,
    }

    pub fn handle(payload: &SetMeshParams, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![RemodelMutation::SetMeshParams {
            params: MeshParams {
                tsdf_voxel_size_mm: payload.tsdf_voxel_size_mm,
                tsdf_truncation_mm: payload.tsdf_truncation_mm,
                decimate_target_triangles: payload.decimate_target_triangles,
                smoothing_iterations: payload.smoothing_iterations,
                texture_enabled: payload.texture_enabled,
                texture_size: payload.texture_size,
                guarantee_watertight: payload.guarantee_watertight,
                hole_fill_max_boundary_verts: payload.hole_fill_max_boundary_verts,
                self_intersection_check: payload.self_intersection_check,
            },
        }]))
    }
}
//#endregion 🔖️SetMeshParams

//#region 🔖️SetMotionParams
pub mod set_motion_params {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "motion-params")]
    pub struct SetMotionParams {
        pub enabled: bool,
        pub max_tracks: u32,
        pub track_window_px: u32,
        pub min_track_quality: f32,
        pub min_track_length_frames: u32,
    }

    pub fn handle(payload: &SetMotionParams, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![RemodelMutation::SetMotionParams {
            params: MotionParams { enabled: payload.enabled, max_tracks: payload.max_tracks, track_window_px: payload.track_window_px, min_track_quality: payload.min_track_quality, min_track_length_frames: payload.min_track_length_frames },
        }]))
    }
}
//#endregion 🔖️SetMotionParams

//#region 🔖️SetGeoParams
pub mod set_geo_params {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "geo-params")]
    pub struct SetGeoParams {
        pub enabled: bool,
        #[serde(default)]
        pub origin_lon: Option<f64>,
        #[serde(default)]
        pub origin_lat: Option<f64>,
        #[serde(default)]
        pub origin_alt: Option<f64>,
        pub gsd_m: f32,
        pub dsm_cell_m: f32,
        pub dtm_filter_radius_m: f32,
        pub ortho_max_px: u32,
    }

    pub fn handle(payload: &SetGeoParams, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![RemodelMutation::SetGeoParams {
            params: GeoParams {
                enabled: payload.enabled,
                origin_lon: payload.origin_lon,
                origin_lat: payload.origin_lat,
                origin_alt: payload.origin_alt,
                gsd_m: payload.gsd_m,
                dsm_cell_m: payload.dsm_cell_m,
                dtm_filter_radius_m: payload.dtm_filter_radius_m,
                ortho_max_px: payload.ortho_max_px,
            },
        }]))
    }
}
//#endregion 🔖️SetGeoParams

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::remodel::testkit::{app, dispatch};
    use crate::apps::remodel::RemodelCommand;

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
        dispatch(&mut app, RemodelCommand::SetIngestParams(set_ingest_params::SetIngestParams { frame_sample_stride: 5, max_frames: 200, downscale_long_edge_px: 1600, min_sharpness: 0.42 }));
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
