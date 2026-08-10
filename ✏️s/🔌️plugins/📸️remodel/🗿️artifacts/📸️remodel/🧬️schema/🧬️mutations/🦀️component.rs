//! 🧬️ Remodel artifact — document mutation dispatch enum.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::{
    CalibrationState, CameraTrajectory, DenseCloud, DenseParams, FeatureParams, GeoParams, GeoProducts, GroundControlPoint, ImageAsset, IngestParams, MatchParams, MediaStream, MeshParams, MotionParams, MotionTrackSummary, QcReportSnapshot,
    ReconstructionJob, RemodelMesh, RemodelSnapshot, SfmParams, SparseCloud,
};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// @emoji 🧬️ The typed remodel document mutation — one LWW register setter per independent field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum RemodelMutation {
    SetStreams {
        streams: Vec<MediaStream>,
    },
    SetAsset {
        key: String,
        #[serde(default)]
        #[dsl(block)]
        value: Option<ImageAsset>,
    },
    SetCalibration {
        #[dsl(block)]
        calibration: CalibrationState,
    },
    SetGcps {
        gcps: Vec<GroundControlPoint>,
    },
    SetIngestParams {
        #[dsl(block)]
        params: IngestParams,
    },
    SetFeatureParams {
        #[dsl(block)]
        params: FeatureParams,
    },
    SetMatchParams {
        #[dsl(block)]
        params: MatchParams,
    },
    SetSfmParams {
        #[dsl(block)]
        params: SfmParams,
    },
    SetDenseParams {
        #[dsl(block)]
        params: DenseParams,
    },
    SetMeshParams {
        #[dsl(block)]
        params: MeshParams,
    },
    SetMotionParams {
        #[dsl(block)]
        params: MotionParams,
    },
    SetGeoParams {
        #[dsl(block)]
        params: GeoParams,
    },
    SetJob {
        #[dsl(block)]
        job: ReconstructionJob,
    },
    SetSparse {
        #[serde(default)]
        #[dsl(block)]
        sparse: Option<SparseCloud>,
    },
    SetDense {
        #[serde(default)]
        #[dsl(block)]
        dense: Option<DenseCloud>,
    },
    /// 📦️ Boxed: `RemodelMesh` (a full `MeshData` plus an optional watertight snapshot) is far larger
    /// than any sibling variant, and `clippy::large_enum_variant` flags the resulting size disparity
    /// across `RemodelMutation`/`RemodelDiff` — boxing keeps every other variant cheap to move.
    SetMeshResult {
        #[dsl(block)]
        mesh: Box<RemodelMesh>,
    },
    SetTrajectory {
        #[serde(default)]
        #[dsl(block)]
        trajectory: Option<CameraTrajectory>,
    },
    SetTracks {
        tracks: Vec<MotionTrackSummary>,
    },
    SetGeoProducts {
        #[serde(default)]
        #[dsl(block)]
        geo: Option<GeoProducts>,
    },
    SetQc {
        #[serde(default)]
        #[dsl(block)]
        qc: Option<QcReportSnapshot>,
    },
}

/// @emoji ▶️ Applies one mutation, returning the next projection.
pub fn apply_remodel_mutation(scene: &RemodelSnapshot, mutation: &RemodelMutation) -> RemodelSnapshot {
    let mut next = scene.clone();
    apply_remodel_mutation_in_place(&mut next, mutation);
    next
}

/// @emoji ▶️ Applies one mutation to the projection in place.
pub fn apply_remodel_mutation_in_place(next: &mut RemodelSnapshot, mutation: &RemodelMutation) {
    match mutation {
        RemodelMutation::SetStreams { streams } => super::set_streams::mutation::apply(next, streams),
        RemodelMutation::SetAsset { key, value } => super::set_asset::mutation::apply(next, key, value),
        RemodelMutation::SetCalibration { calibration } => super::set_calibration::mutation::apply(next, calibration),
        RemodelMutation::SetGcps { gcps } => super::set_gcps::mutation::apply(next, gcps),
        RemodelMutation::SetIngestParams { params } => super::set_ingest_params::mutation::apply(next, params),
        RemodelMutation::SetFeatureParams { params } => super::set_feature_params::mutation::apply(next, params),
        RemodelMutation::SetMatchParams { params } => super::set_match_params::mutation::apply(next, params),
        RemodelMutation::SetSfmParams { params } => super::set_sfm_params::mutation::apply(next, params),
        RemodelMutation::SetDenseParams { params } => super::set_dense_params::mutation::apply(next, params),
        RemodelMutation::SetMeshParams { params } => super::set_mesh_params::mutation::apply(next, params),
        RemodelMutation::SetMotionParams { params } => super::set_motion_params::mutation::apply(next, params),
        RemodelMutation::SetGeoParams { params } => super::set_geo_params::mutation::apply(next, params),
        RemodelMutation::SetJob { job } => super::set_job::mutation::apply(next, job),
        RemodelMutation::SetSparse { sparse } => super::set_sparse::mutation::apply(next, sparse),
        RemodelMutation::SetDense { dense } => super::set_dense::mutation::apply(next, dense),
        RemodelMutation::SetMeshResult { mesh } => super::set_mesh_result::mutation::apply(next, mesh),
        RemodelMutation::SetTrajectory { trajectory } => super::set_trajectory::mutation::apply(next, trajectory),
        RemodelMutation::SetTracks { tracks } => super::set_tracks::mutation::apply(next, tracks),
        RemodelMutation::SetGeoProducts { geo } => super::set_geo_products::mutation::apply(next, geo),
        RemodelMutation::SetQc { qc } => super::set_qc::mutation::apply(next, qc),
    }
}

/// @emoji ↩️ Computes the inverse mutations from pre-state.
pub fn inverse_remodel_mutation(base: &RemodelSnapshot, mutation: &RemodelMutation) -> Vec<RemodelMutation> {
    match mutation {
        RemodelMutation::SetStreams { .. } => super::set_streams::inverse::inverse(base),
        RemodelMutation::SetAsset { key, .. } => super::set_asset::inverse::inverse(base, key),
        RemodelMutation::SetCalibration { .. } => super::set_calibration::inverse::inverse(base),
        RemodelMutation::SetGcps { .. } => super::set_gcps::inverse::inverse(base),
        RemodelMutation::SetIngestParams { .. } => super::set_ingest_params::inverse::inverse(base),
        RemodelMutation::SetFeatureParams { .. } => super::set_feature_params::inverse::inverse(base),
        RemodelMutation::SetMatchParams { .. } => super::set_match_params::inverse::inverse(base),
        RemodelMutation::SetSfmParams { .. } => super::set_sfm_params::inverse::inverse(base),
        RemodelMutation::SetDenseParams { .. } => super::set_dense_params::inverse::inverse(base),
        RemodelMutation::SetMeshParams { .. } => super::set_mesh_params::inverse::inverse(base),
        RemodelMutation::SetMotionParams { .. } => super::set_motion_params::inverse::inverse(base),
        RemodelMutation::SetGeoParams { .. } => super::set_geo_params::inverse::inverse(base),
        RemodelMutation::SetJob { .. } => super::set_job::inverse::inverse(base),
        RemodelMutation::SetSparse { .. } => super::set_sparse::inverse::inverse(base),
        RemodelMutation::SetDense { .. } => super::set_dense::inverse::inverse(base),
        RemodelMutation::SetMeshResult { .. } => super::set_mesh_result::inverse::inverse(base),
        RemodelMutation::SetTrajectory { .. } => super::set_trajectory::inverse::inverse(base),
        RemodelMutation::SetTracks { .. } => super::set_tracks::inverse::inverse(base),
        RemodelMutation::SetGeoProducts { .. } => super::set_geo_products::inverse::inverse(base),
        RemodelMutation::SetQc { .. } => super::set_qc::inverse::inverse(base),
    }
}

impl Mutation<RemodelSnapshot> for RemodelMutation {
    type Diff = RemodelDiff;

    fn diff(&self, base: &RemodelSnapshot) -> RemodelDiff {
        match self {
            RemodelMutation::SetStreams { streams } => RemodelDiff {
                streams: Some(crate::artifacts::remodel::diff::RemodelMediaStreamList { values: streams.clone() }),
                ..Default::default()
            },
            RemodelMutation::SetAsset { key, value } => {
                let mut assets = base.assets.clone();
                match value {
                    Some(asset) => {
                        assets.insert(key.clone(), asset.clone());
                    }
                    None => {
                        assets.remove(key);
                    }
                }
                RemodelDiff { assets: Some(assets), ..Default::default() }
            }
            RemodelMutation::SetCalibration { calibration } => RemodelDiff {
                calibration: Some(calibration.clone()),
                ..Default::default()
            },
            RemodelMutation::SetGcps { gcps } => RemodelDiff {
                gcps: Some(crate::artifacts::remodel::diff::RemodelGcpList { values: gcps.clone() }),
                ..Default::default()
            },
            RemodelMutation::SetIngestParams { params } => {
                let mut next = base.params.clone();
                next.ingest = params.clone();
                RemodelDiff { params: Some(next), ..Default::default() }
            }
            RemodelMutation::SetFeatureParams { params } => {
                let mut next = base.params.clone();
                next.feature = params.clone();
                RemodelDiff { params: Some(next), ..Default::default() }
            }
            RemodelMutation::SetMatchParams { params } => {
                let mut next = base.params.clone();
                next.matching = params.clone();
                RemodelDiff { params: Some(next), ..Default::default() }
            }
            RemodelMutation::SetSfmParams { params } => {
                let mut next = base.params.clone();
                next.sfm = params.clone();
                RemodelDiff { params: Some(next), ..Default::default() }
            }
            RemodelMutation::SetDenseParams { params } => {
                let mut next = base.params.clone();
                next.dense = params.clone();
                RemodelDiff { params: Some(next), ..Default::default() }
            }
            RemodelMutation::SetMeshParams { params } => {
                let mut next = base.params.clone();
                next.mesh = params.clone();
                RemodelDiff { params: Some(next), ..Default::default() }
            }
            RemodelMutation::SetMotionParams { params } => {
                let mut next = base.params.clone();
                next.motion = params.clone();
                RemodelDiff { params: Some(next), ..Default::default() }
            }
            RemodelMutation::SetGeoParams { params } => {
                let mut next = base.params.clone();
                next.geo = params.clone();
                RemodelDiff { params: Some(next), ..Default::default() }
            }
            RemodelMutation::SetJob { job } => RemodelDiff { job: Some(job.clone()), ..Default::default() },
            RemodelMutation::SetSparse { sparse } => {
                let mut results = base.results.clone();
                results.sparse = sparse.clone();
                RemodelDiff { results: Some(results), ..Default::default() }
            }
            RemodelMutation::SetDense { dense } => {
                let mut results = base.results.clone();
                results.dense = dense.clone();
                RemodelDiff { results: Some(results), ..Default::default() }
            }
            RemodelMutation::SetMeshResult { mesh } => {
                let mut results = base.results.clone();
                results.mesh = (**mesh).clone();
                RemodelDiff { results: Some(results), ..Default::default() }
            }
            RemodelMutation::SetTrajectory { trajectory } => {
                let mut results = base.results.clone();
                results.trajectory = trajectory.clone();
                RemodelDiff { results: Some(results), ..Default::default() }
            }
            RemodelMutation::SetTracks { tracks } => {
                let mut results = base.results.clone();
                results.tracks = tracks.clone();
                RemodelDiff { results: Some(results), ..Default::default() }
            }
            RemodelMutation::SetGeoProducts { geo } => {
                let mut results = base.results.clone();
                results.geo = geo.clone();
                RemodelDiff { results: Some(results), ..Default::default() }
            }
            RemodelMutation::SetQc { qc } => {
                let mut results = base.results.clone();
                results.qc = qc.clone();
                RemodelDiff { results: Some(results), ..Default::default() }
            }
        }
    }

    fn inverse(&self, projection: &RemodelSnapshot) -> Vec<Self> {
        inverse_remodel_mutation(projection, self)
    }
}
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::remodel::{
        default_remodel_scene, CameraCalibration, CameraPosePreview, FrameRef, GcpObservation, MediaKind, MeshSource, PackedF32, PackedU8, ReconstructionStage, RigExtrinsic, TrackClass, VideoCodec, VideoSource, WatertightReportSnapshot,
    };
    use protocol::MutationDiff as _;

    //#region Operations
    #[test]
    fn set_streams_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let streams = vec![MediaStream { id: "s1".into(), name: "cam".into(), ..MediaStream::default() }];
        let operation = RemodelMutation::SetStreams { streams: streams.clone() };
        let next = apply_remodel_mutation(&scene, &operation);
        assert_eq!(next.streams, streams);
        assert_eq!(operation.diff(&scene).apply(&scene).streams, streams);
        let inverse = operation.inverse(&scene);
        assert_eq!(inverse, vec![RemodelMutation::SetStreams { streams: scene.streams.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_mutation(&current, operation));
        assert_eq!(reverted.streams, scene.streams);
    }

    #[test]
    fn set_asset_op_applies_and_reverts_including_absent_case() {
        let scene = default_remodel_scene();
        assert!(!scene.assets.contains_key("frame-1"));

        let asset = ImageAsset { mime: "image/jpeg".into(), data: "zzz".into(), width: 2, height: 2 };
        let insert_operation = RemodelMutation::SetAsset { key: "frame-1".into(), value: Some(asset.clone()) };
        let after_insert = apply_remodel_mutation(&scene, &insert_operation);
        assert_eq!(after_insert.assets.get("frame-1"), Some(&asset));
        assert_eq!(insert_operation.diff(&scene).apply(&scene).assets.get("frame-1"), Some(&asset));

        let insert_inverse = insert_operation.inverse(&scene);
        assert_eq!(insert_inverse, vec![RemodelMutation::SetAsset { key: "frame-1".into(), value: None }]);
        let reverted = insert_inverse.iter().fold(after_insert.clone(), |current, operation| apply_remodel_mutation(&current, operation));
        assert_eq!(reverted, scene);

        let remove_operation = RemodelMutation::SetAsset { key: "frame-1".into(), value: None };
        let remove_inverse = remove_operation.inverse(&after_insert);
        assert_eq!(remove_inverse, vec![RemodelMutation::SetAsset { key: "frame-1".into(), value: Some(asset.clone()) }]);
        let after_remove = apply_remodel_mutation(&after_insert, &remove_operation);
        assert!(!after_remove.assets.contains_key("frame-1"));
        let restored = remove_inverse.iter().fold(after_remove, |current, operation| apply_remodel_mutation(&current, operation));
        assert_eq!(restored.assets.get("frame-1"), Some(&asset));
    }

    #[test]
    fn set_calibration_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let mut calibration = scene.calibration.clone();
        calibration.cameras.push(CameraCalibration { id: "cam-1".into(), model: "pinhole".into(), ..CameraCalibration::default() });
        let operation = RemodelMutation::SetCalibration { calibration: calibration.clone() };
        let next = apply_remodel_mutation(&scene, &operation);
        assert_eq!(next.calibration, calibration);
        let inverse = operation.inverse(&scene);
        assert_eq!(inverse, vec![RemodelMutation::SetCalibration { calibration: scene.calibration.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_mutation(&current, operation));
        assert_eq!(reverted.calibration, scene.calibration);
    }

    #[test]
    fn set_gcps_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let gcps = vec![GroundControlPoint { id: "gcp-1".into(), name: "A".into(), world_position: [0.0, 0.0, 0.0], observations: Vec::new() }];
        let operation = RemodelMutation::SetGcps { gcps: gcps.clone() };
        let next = apply_remodel_mutation(&scene, &operation);
        assert_eq!(next.gcps, gcps);
        let inverse = operation.inverse(&scene);
        assert_eq!(inverse, vec![RemodelMutation::SetGcps { gcps: scene.gcps.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_mutation(&current, operation));
        assert_eq!(reverted.gcps, scene.gcps);
    }

    #[test]
    fn set_job_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let mut job = scene.job.clone();
        job.stage = ReconstructionStage::BundleAdjusting;
        job.progress_0_1 = 0.42;
        job.started_at_ms = Some(1000.0);
        let operation = RemodelMutation::SetJob { job: job.clone() };
        let next = apply_remodel_mutation(&scene, &operation);
        assert_eq!(next.job, job);
        let inverse = operation.inverse(&scene);
        assert_eq!(inverse, vec![RemodelMutation::SetJob { job: scene.job.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_mutation(&current, operation));
        assert_eq!(reverted.job, scene.job);
    }

    #[test]
    fn set_sparse_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let sparse = SparseCloud { points: PackedF32::from_f32_slice(&[0.0, 0.0, 0.0]), colors: Some(PackedU8::from_u8_slice(&[255, 255, 255])) };
        let operation = RemodelMutation::SetSparse { sparse: Some(sparse.clone()) };
        let next = apply_remodel_mutation(&scene, &operation);
        assert_eq!(next.results.sparse, Some(sparse));
        let inverse = operation.inverse(&scene);
        assert_eq!(inverse, vec![RemodelMutation::SetSparse { sparse: scene.results.sparse.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_mutation(&current, operation));
        assert_eq!(reverted.results.sparse, scene.results.sparse);
    }

    #[test]
    fn set_dense_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let dense = DenseCloud { positions: PackedF32::from_f32_slice(&[1.0, 2.0, 3.0]), colors: None, confidence: Some(PackedF32::from_f32_slice(&[0.8])), classification: Some(PackedU8::from_u8_slice(&[2])) };
        let operation = RemodelMutation::SetDense { dense: Some(dense.clone()) };
        let next = apply_remodel_mutation(&scene, &operation);
        assert_eq!(next.results.dense, Some(dense));
        let inverse = operation.inverse(&scene);
        assert_eq!(inverse, vec![RemodelMutation::SetDense { dense: scene.results.dense.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_mutation(&current, operation));
        assert_eq!(reverted.results.dense, scene.results.dense);
    }

    #[test]
    fn set_mesh_result_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let mesh = RemodelMesh {
            mesh: semio_framework::mesh_from_kind("box"),
            source: MeshSource::Reconstructed,
            texture_asset_id: Some("tex-1".into()),
            watertight: Some(WatertightReportSnapshot { is_watertight: true, is_two_manifold: true, is_closed: true, ..WatertightReportSnapshot::default() }),
        };
        let operation = RemodelMutation::SetMeshResult { mesh: Box::new(mesh.clone()) };
        let next = apply_remodel_mutation(&scene, &operation);
        assert_eq!(next.results.mesh, mesh);
        assert_eq!(operation.diff(&scene).apply(&scene).results.mesh, mesh);
        let inverse = operation.inverse(&scene);
        assert_eq!(inverse, vec![RemodelMutation::SetMeshResult { mesh: Box::new(scene.results.mesh.clone()) }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_mutation(&current, operation));
        assert_eq!(reverted.results.mesh, scene.results.mesh);
    }

    #[test]
    fn set_trajectory_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let trajectory = CameraTrajectory { poses: vec![CameraPosePreview { camera_id: "cam-1".into(), ..CameraPosePreview::default() }] };
        let operation = RemodelMutation::SetTrajectory { trajectory: Some(trajectory.clone()) };
        let next = apply_remodel_mutation(&scene, &operation);
        assert_eq!(next.results.trajectory, Some(trajectory));
        let inverse = operation.inverse(&scene);
        assert_eq!(inverse, vec![RemodelMutation::SetTrajectory { trajectory: scene.results.trajectory.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_mutation(&current, operation));
        assert_eq!(reverted.results.trajectory, scene.results.trajectory);
    }

    #[test]
    fn set_tracks_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let tracks = vec![MotionTrackSummary { id: "t1".into(), length: 12, class: TrackClass::Moving, mean_speed_m_s: 0.5 }];
        let operation = RemodelMutation::SetTracks { tracks: tracks.clone() };
        let next = apply_remodel_mutation(&scene, &operation);
        assert_eq!(next.results.tracks, tracks);
        let inverse = operation.inverse(&scene);
        assert_eq!(inverse, vec![RemodelMutation::SetTracks { tracks: scene.results.tracks.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_mutation(&current, operation));
        assert_eq!(reverted.results.tracks, scene.results.tracks);
    }

    #[test]
    fn set_geo_products_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let geo = GeoProducts { dsm_asset_id: Some("dsm".into()), dtm_asset_id: None, ortho_asset_id: Some("ortho".into()) };
        let operation = RemodelMutation::SetGeoProducts { geo: Some(geo.clone()) };
        let next = apply_remodel_mutation(&scene, &operation);
        assert_eq!(next.results.geo, Some(geo));
        let inverse = operation.inverse(&scene);
        assert_eq!(inverse, vec![RemodelMutation::SetGeoProducts { geo: scene.results.geo.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_mutation(&current, operation));
        assert_eq!(reverted.results.geo, scene.results.geo);
    }

    #[test]
    fn set_qc_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let qc = QcReportSnapshot { reprojection_rms_px: 0.6, warnings: vec!["w".into()], ..QcReportSnapshot::default() };
        let operation = RemodelMutation::SetQc { qc: Some(qc.clone()) };
        let next = apply_remodel_mutation(&scene, &operation);
        assert_eq!(next.results.qc, Some(qc));
        let inverse = operation.inverse(&scene);
        assert_eq!(inverse, vec![RemodelMutation::SetQc { qc: scene.results.qc.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_mutation(&current, operation));
        assert_eq!(reverted.results.qc, scene.results.qc);
    }

    /// 🔁️ The 8 `Set<Stage>Params` operations are mechanically identical (LWW-replace one
    /// `ReconstructionParams` sub-field, inverse restores the pre-edit value) — generated once per
    /// param family instead of copy-pasted, per the "concise code" rule.
    macro_rules! param_op_roundtrip_test {
        ($test_name:ident, $variant:ident, $field:ident, $params_ty:ty, $mutate:expr) => {
            #[test]
            fn $test_name() {
                let scene = default_remodel_scene();
                let mut params: $params_ty = scene.params.$field.clone();
                let mutate: fn(&mut $params_ty) = $mutate;
                mutate(&mut params);
                let operation = RemodelMutation::$variant { params: params.clone() };
                let next = apply_remodel_mutation(&scene, &operation);
                assert_eq!(next.params.$field, params);
                assert_eq!(operation.diff(&scene).apply(&scene).params.$field, params);
                let inverse = operation.inverse(&scene);
                assert_eq!(inverse, vec![RemodelMutation::$variant { params: scene.params.$field.clone() }]);
                let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_mutation(&current, operation));
                assert_eq!(reverted.params.$field, scene.params.$field);
            }
        };
    }

    param_op_roundtrip_test!(set_ingest_params_op_applies_and_reverts, SetIngestParams, ingest, IngestParams, |p| p.min_sharpness = 0.6);
    param_op_roundtrip_test!(set_feature_params_op_applies_and_reverts, SetFeatureParams, feature, FeatureParams, |p| p.target_count = 8000);
    param_op_roundtrip_test!(set_match_params_op_applies_and_reverts, SetMatchParams, matching, MatchParams, |p| p.ratio_test = 0.6);
    param_op_roundtrip_test!(set_sfm_params_op_applies_and_reverts, SetSfmParams, sfm, SfmParams, |p| p.ransac_iterations = 2000);
    param_op_roundtrip_test!(set_dense_params_op_applies_and_reverts, SetDenseParams, dense, DenseParams, |p| p.max_points = 100);
    param_op_roundtrip_test!(set_mesh_params_op_applies_and_reverts, SetMeshParams, mesh, MeshParams, |p| p.guarantee_watertight = false);
    param_op_roundtrip_test!(set_motion_params_op_applies_and_reverts, SetMotionParams, motion, MotionParams, |p| p.enabled = true);
    param_op_roundtrip_test!(set_geo_params_op_applies_and_reverts, SetGeoParams, geo, GeoParams, |p| p.enabled = true);
    //#endregion Operations

    //#region Convergence
    /// 🔀️ The CRDT convergence contract: two collaborators concurrently importing different frames
    /// (`SetAsset` on disjoint keys) must converge to an identical scene regardless of application
    /// order, and neither import may clobber the other's key. This is what makes `SetAsset` per-key
    /// rather than a whole-`assets`-map replace — a whole-map design would fail this test, since
    /// applying instance B's operation after instance A's would silently drop A's key if B's captured map
    /// snapshot predates A's insert.
    #[test]
    fn concurrent_set_asset_ops_converge_regardless_of_order() {
        let base = default_remodel_scene();
        let asset_a = ImageAsset { mime: "image/jpeg".into(), data: "frame-one".into(), width: 8, height: 8 };
        let asset_b = ImageAsset { mime: "image/jpeg".into(), data: "frame-two".into(), width: 8, height: 8 };
        let op_a = RemodelMutation::SetAsset { key: "frame-1".into(), value: Some(asset_a.clone()) };
        let op_b = RemodelMutation::SetAsset { key: "frame-2".into(), value: Some(asset_b.clone()) };

        let a_then_b = apply_remodel_mutation(&apply_remodel_mutation(&base, &op_a), &op_b);
        let b_then_a = apply_remodel_mutation(&apply_remodel_mutation(&base, &op_b), &op_a);

        assert_eq!(a_then_b, b_then_a, "concurrent SetAsset on disjoint keys must converge regardless of order");
        assert_eq!(a_then_b.assets.get("frame-1"), Some(&asset_a), "instance A's import must survive instance B's operation");
        assert_eq!(a_then_b.assets.get("frame-2"), Some(&asset_b), "instance B's import must survive instance A's operation");
        assert_eq!(a_then_b.assets.len(), base.assets.len() + 2);
    }

    /// 🔀️ Same convergence contract across two *disjoint operation families* at once (one instance tunes
    /// feature-detector params, the other adds a GCP) — proves field-granular LWW converges not just
    /// within one operation family but across the whole operation vocabulary.
    #[test]
    fn concurrent_edits_across_different_op_families_converge() {
        let base = default_remodel_scene();
        let mut feature_params = base.params.feature.clone();
        feature_params.target_count = 9000;
        let op_feature = RemodelMutation::SetFeatureParams { params: feature_params.clone() };
        let gcps = vec![GroundControlPoint { id: "gcp-1".into(), name: "Corner".into(), world_position: [1.0, 2.0, 3.0], observations: Vec::new() }];
        let op_gcp = RemodelMutation::SetGcps { gcps: gcps.clone() };

        let feature_then_gcp = apply_remodel_mutation(&apply_remodel_mutation(&base, &op_feature), &op_gcp);
        let gcp_then_feature = apply_remodel_mutation(&apply_remodel_mutation(&base, &op_gcp), &op_feature);

        assert_eq!(feature_then_gcp, gcp_then_feature);
        assert_eq!(feature_then_gcp.params.feature, feature_params);
        assert_eq!(feature_then_gcp.gcps, gcps);
    }
    //#endregion Convergence

    //#region 🔖️OpText
    /// 🏗️ Verbatim duplicate of the `rs` crate's own private test-only fixture builder (see that
    /// crate's `populated_scene_fixture` doc comment) — needed here so every `RemodelMutation`
    /// variant below can be exercised against a document that actually populates every field.
    fn populated_scene_fixture() -> RemodelSnapshot {
        let mut scene = default_remodel_scene();
        scene.streams.push(MediaStream {
            id: "stream-1".into(),
            name: "front".into(),
            kind: MediaKind::Video,
            camera_id: Some("cam-1".into()),
            sync_offset_ms: 12.5,
            fps_hint: 30.0,
            frames: vec![FrameRef { index: 0, timestamp_ms: 0.0, asset_id: "asset-1".into() }],
            source: Some(VideoSource { name: "front.mp4".into(), container: "mp4".into(), codec: VideoCodec::Avc, duration_ms: 6633.3, frame_count: 199, width: 1920, height: 1080 }),
        });
        scene.assets.insert("asset-1".into(), ImageAsset { mime: "image/jpeg".into(), data: "abcd".into(), width: 4, height: 4 });
        scene.calibration.cameras.push(CameraCalibration {
            id: "cam-1".into(),
            label: "Front".into(),
            model: "brownConrady".into(),
            fx: 1000.0,
            fy: 1000.0,
            cx: 512.0,
            cy: 384.0,
            skew: 0.0,
            distortion: [0.01, -0.02, 0.0, 0.0, 0.0],
            rms_reprojection_px: Some(0.4),
            locked: false,
        });
        scene.calibration.rig.push(RigExtrinsic::default());
        scene.gcps.push(GroundControlPoint { id: "gcp-1".into(), name: "Corner".into(), world_position: [1.0, 2.0, 3.0], observations: vec![GcpObservation { stream_id: "stream-1".into(), frame_index: 0, pixel: [10.0, 20.0] }] });
        scene.params.ingest.min_sharpness = 0.4;
        scene.params.mesh.texture_size = 4096;
        scene.job.stage = ReconstructionStage::BundleAdjusting;
        scene.job.progress_0_1 = 0.42;
        scene.job.started_at_ms = Some(1000.0);
        scene.job.error = Some("retry needed".into());
        scene.job.camera_poses_preview.push(CameraPosePreview { camera_id: "cam-1".into(), ..CameraPosePreview::default() });
        scene.job.sparse_point_cloud_preview = PackedF32::from_f32_slice(&[0.1, 0.2, 0.3]);
        scene.results.sparse = Some(SparseCloud { points: PackedF32::from_f32_slice(&[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]), colors: Some(PackedU8::from_u8_slice(&[255, 0, 0, 0, 255, 0])) });
        scene.results.dense =
            Some(DenseCloud { positions: PackedF32::from_f32_slice(&[0.0, 0.0, 0.0]), colors: Some(PackedU8::from_u8_slice(&[0, 0, 255])), confidence: Some(PackedF32::from_f32_slice(&[0.9])), classification: Some(PackedU8::from_u8_slice(&[2])) });
        scene.results.mesh = RemodelMesh {
            mesh: semio_framework::mesh_from_kind("box"),
            source: MeshSource::Reconstructed,
            texture_asset_id: Some("tex-1".into()),
            watertight: Some(WatertightReportSnapshot {
                vertex_count: 512,
                triangle_count: 1020,
                boundary_edge_count: 0,
                boundary_loop_count: 0,
                non_manifold_edge_count: 0,
                non_manifold_vertex_count: 0,
                connected_components: 1,
                consistently_oriented: true,
                euler_characteristic: 2,
                genus: Some(0),
                signed_volume: 12.5,
                self_intersection_pairs: Some(0),
                closed_fallback_used: false,
                is_closed: true,
                is_two_manifold: true,
                is_watertight: true,
            }),
        };
        scene.results.trajectory = Some(CameraTrajectory {
            poses: vec![
                CameraPosePreview { camera_id: "cam-1".into(), rotation_wxyz: [1.0, 0.0, 0.0, 0.0], translation: [0.0, 0.0, 0.0] },
                CameraPosePreview { camera_id: "cam-1".into(), rotation_wxyz: [0.999, 0.001, 0.0, 0.0], translation: [0.1, 0.0, 0.0] },
            ],
        });
        scene.results.tracks.push(MotionTrackSummary { id: "track-1".into(), length: 42, class: TrackClass::Moving, mean_speed_m_s: 1.2 });
        scene.results.geo = Some(GeoProducts { dsm_asset_id: Some("asset-dsm".into()), dtm_asset_id: Some("asset-dtm".into()), ortho_asset_id: Some("asset-ortho".into()) });
        scene.results.qc = Some(QcReportSnapshot {
            reprojection_rms_px: 0.5,
            gcp_checkpoint_rmse: Some(0.02),
            watertight: scene.results.mesh.watertight.clone(),
            mean_track_length: 6.0,
            registered_frame_ratio: 1.0,
            dense_coverage_ratio: 0.95,
            warnings: vec!["low overlap on frame 12".into()],
        });
        scene
    }

    /// ⚡️ One `assert_op_line_round_trip` per `RemodelMutation` variant, per the mechanism contract.
    #[test]
    fn every_operation_variant_roundtrips_through_op_text() {
        let scene = populated_scene_fixture();

        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetStreams { streams: scene.streams.clone() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetAsset { key: "asset-1".into(), value: scene.assets.get("asset-1").cloned() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetAsset { key: "asset-2".into(), value: None });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetCalibration { calibration: scene.calibration.clone() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetGcps { gcps: scene.gcps.clone() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetIngestParams { params: scene.params.ingest.clone() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetFeatureParams { params: scene.params.feature.clone() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetMatchParams { params: scene.params.matching.clone() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetSfmParams { params: scene.params.sfm.clone() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetDenseParams { params: scene.params.dense.clone() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetMeshParams { params: scene.params.mesh.clone() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetMotionParams { params: scene.params.motion.clone() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetGeoParams { params: scene.params.geo.clone() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetJob { job: scene.job.clone() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetSparse { sparse: scene.results.sparse.clone() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetSparse { sparse: None });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetDense { dense: scene.results.dense.clone() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetDense { dense: None });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetMeshResult { mesh: Box::new(scene.results.mesh.clone()) });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetTrajectory { trajectory: scene.results.trajectory.clone() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetTrajectory { trajectory: None });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetTracks { tracks: scene.results.tracks.clone() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetGeoProducts { geo: scene.results.geo.clone() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetGeoProducts { geo: None });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetQc { qc: scene.results.qc });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelMutation::SetQc { qc: None });
    }
    //#endregion 🔖️OpText
}
//#endregion 🧪️Tests

