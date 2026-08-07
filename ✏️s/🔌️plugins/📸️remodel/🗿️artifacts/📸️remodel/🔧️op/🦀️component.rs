//! ⚡️ Remodel artifact — the operation vocabulary and its laws.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::{
    CalibrationState, CameraTrajectory, DenseCloud, DenseParams, FeatureParams, GeoParams, GeoProducts, GroundControlPoint, ImageAsset, IngestParams, MatchParams, MediaStream, MeshParams, MotionParams, MotionTrackSummary, QcReportSnapshot,
    ReconstructionJob, RemodelMesh, RemodelProjection, SfmParams, SparseCloud,
};
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Operations
/// 🔁️ The document mutation vocabulary — one field-granular LWW register setter per independent
/// `RemodelProjection` field/sub-field, so disjoint-field edits by concurrent instances converge cleanly.
/// There is no `setDocument` catch-all: reconstruction is field-granular (import a stream, tune one
/// param group, publish a partial result) and each operation carries its own inverse from the pre-edit state.
/// `SetAsset` is per-key (not a whole-map replace) so two peers importing different frames converge
/// without clobbering each other's assets — see `concurrent_set_asset_ops_converge_regardless_of_order`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum RemodelOperation {
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
    /// across `RemodelOperation`/`RemodelDiff` — boxing keeps every other variant cheap to move.
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
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for RemodelOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for RemodelOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs




pub fn apply_remodel_operation(scene: &RemodelProjection, operation: &RemodelOperation) -> RemodelProjection {
    let mut next = scene.clone();
    match operation {
        RemodelOperation::SetStreams { streams } => next.streams = streams.clone(),
        RemodelOperation::SetAsset { key, value } => match value {
            Some(value) => {
                next.assets.insert(key.clone(), value.clone());
            }
            None => {
                next.assets.remove(key);
            }
        },
        RemodelOperation::SetCalibration { calibration } => next.calibration = calibration.clone(),
        RemodelOperation::SetGcps { gcps } => next.gcps = gcps.clone(),
        RemodelOperation::SetIngestParams { params } => next.params.ingest = params.clone(),
        RemodelOperation::SetFeatureParams { params } => next.params.feature = params.clone(),
        RemodelOperation::SetMatchParams { params } => next.params.matching = params.clone(),
        RemodelOperation::SetSfmParams { params } => next.params.sfm = params.clone(),
        RemodelOperation::SetDenseParams { params } => next.params.dense = params.clone(),
        RemodelOperation::SetMeshParams { params } => next.params.mesh = params.clone(),
        RemodelOperation::SetMotionParams { params } => next.params.motion = params.clone(),
        RemodelOperation::SetGeoParams { params } => next.params.geo = params.clone(),
        RemodelOperation::SetJob { job } => next.job = job.clone(),
        RemodelOperation::SetSparse { sparse } => next.results.sparse = sparse.clone(),
        RemodelOperation::SetDense { dense } => next.results.dense = dense.clone(),
        RemodelOperation::SetMeshResult { mesh } => next.results.mesh = mesh.as_ref().clone(),
        RemodelOperation::SetTrajectory { trajectory } => next.results.trajectory = trajectory.clone(),
        RemodelOperation::SetTracks { tracks } => next.results.tracks = tracks.clone(),
        RemodelOperation::SetGeoProducts { geo } => next.results.geo = geo.clone(),
        RemodelOperation::SetQc { qc } => next.results.qc = qc.clone(),
    }
    next
}

impl Operation<RemodelProjection> for RemodelOperation {
    type Diff = RemodelDiff;

    fn diff(&self, _projection: &RemodelProjection) -> RemodelDiff {
        match self {
            RemodelOperation::SetStreams { streams } => RemodelDiff::SetStreams { streams: streams.clone() },
            RemodelOperation::SetAsset { key, value } => RemodelDiff::SetAsset { key: key.clone(), value: value.clone() },
            RemodelOperation::SetCalibration { calibration } => RemodelDiff::SetCalibration { calibration: calibration.clone() },
            RemodelOperation::SetGcps { gcps } => RemodelDiff::SetGcps { gcps: gcps.clone() },
            RemodelOperation::SetIngestParams { params } => RemodelDiff::SetIngestParams { params: params.clone() },
            RemodelOperation::SetFeatureParams { params } => RemodelDiff::SetFeatureParams { params: params.clone() },
            RemodelOperation::SetMatchParams { params } => RemodelDiff::SetMatchParams { params: params.clone() },
            RemodelOperation::SetSfmParams { params } => RemodelDiff::SetSfmParams { params: params.clone() },
            RemodelOperation::SetDenseParams { params } => RemodelDiff::SetDenseParams { params: params.clone() },
            RemodelOperation::SetMeshParams { params } => RemodelDiff::SetMeshParams { params: params.clone() },
            RemodelOperation::SetMotionParams { params } => RemodelDiff::SetMotionParams { params: params.clone() },
            RemodelOperation::SetGeoParams { params } => RemodelDiff::SetGeoParams { params: params.clone() },
            RemodelOperation::SetJob { job } => RemodelDiff::SetJob { job: job.clone() },
            RemodelOperation::SetSparse { sparse } => RemodelDiff::SetSparse { sparse: sparse.clone() },
            RemodelOperation::SetDense { dense } => RemodelDiff::SetDense { dense: dense.clone() },
            RemodelOperation::SetMeshResult { mesh } => RemodelDiff::SetMeshResult { mesh: mesh.clone() },
            RemodelOperation::SetTrajectory { trajectory } => RemodelDiff::SetTrajectory { trajectory: trajectory.clone() },
            RemodelOperation::SetTracks { tracks } => RemodelDiff::SetTracks { tracks: tracks.clone() },
            RemodelOperation::SetGeoProducts { geo } => RemodelDiff::SetGeoProducts { geo: geo.clone() },
            RemodelOperation::SetQc { qc } => RemodelDiff::SetQc { qc: qc.clone() },
        }
    }

    fn backwards(&self, projection: &RemodelProjection) -> Vec<Self> {
        vec![match self {
            RemodelOperation::SetStreams { .. } => RemodelOperation::SetStreams { streams: projection.streams.clone() },
            RemodelOperation::SetAsset { key, .. } => RemodelOperation::SetAsset { key: key.clone(), value: projection.assets.get(key).cloned() },
            RemodelOperation::SetCalibration { .. } => RemodelOperation::SetCalibration { calibration: projection.calibration.clone() },
            RemodelOperation::SetGcps { .. } => RemodelOperation::SetGcps { gcps: projection.gcps.clone() },
            RemodelOperation::SetIngestParams { .. } => RemodelOperation::SetIngestParams { params: projection.params.ingest.clone() },
            RemodelOperation::SetFeatureParams { .. } => RemodelOperation::SetFeatureParams { params: projection.params.feature.clone() },
            RemodelOperation::SetMatchParams { .. } => RemodelOperation::SetMatchParams { params: projection.params.matching.clone() },
            RemodelOperation::SetSfmParams { .. } => RemodelOperation::SetSfmParams { params: projection.params.sfm.clone() },
            RemodelOperation::SetDenseParams { .. } => RemodelOperation::SetDenseParams { params: projection.params.dense.clone() },
            RemodelOperation::SetMeshParams { .. } => RemodelOperation::SetMeshParams { params: projection.params.mesh.clone() },
            RemodelOperation::SetMotionParams { .. } => RemodelOperation::SetMotionParams { params: projection.params.motion.clone() },
            RemodelOperation::SetGeoParams { .. } => RemodelOperation::SetGeoParams { params: projection.params.geo.clone() },
            RemodelOperation::SetJob { .. } => RemodelOperation::SetJob { job: projection.job.clone() },
            RemodelOperation::SetSparse { .. } => RemodelOperation::SetSparse { sparse: projection.results.sparse.clone() },
            RemodelOperation::SetDense { .. } => RemodelOperation::SetDense { dense: projection.results.dense.clone() },
            RemodelOperation::SetMeshResult { .. } => RemodelOperation::SetMeshResult { mesh: Box::new(projection.results.mesh.clone()) },
            RemodelOperation::SetTrajectory { .. } => RemodelOperation::SetTrajectory { trajectory: projection.results.trajectory.clone() },
            RemodelOperation::SetTracks { .. } => RemodelOperation::SetTracks { tracks: projection.results.tracks.clone() },
            RemodelOperation::SetGeoProducts { .. } => RemodelOperation::SetGeoProducts { geo: projection.results.geo.clone() },
            RemodelOperation::SetQc { .. } => RemodelOperation::SetQc { qc: projection.results.qc.clone() },
        }]
    }
}
//#endregion 🔖️Operations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::remodel::{
        default_remodel_scene, CameraCalibration, CameraPosePreview, FrameRef, GcpObservation, MediaKind, MeshSource, PackedF32, PackedU8, ReconstructionStage, RigExtrinsic, TrackClass, VideoCodec, VideoSource, WatertightReportSnapshot,
    };
    use protocol::OperationDiff as _;

    //#region Operations
    #[test]
    fn set_streams_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let streams = vec![MediaStream { id: "s1".into(), name: "cam".into(), ..MediaStream::default() }];
        let operation = RemodelOperation::SetStreams { streams: streams.clone() };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.streams, streams);
        assert_eq!(operation.diff(&scene).apply(&scene).streams, streams);
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetStreams { streams: scene.streams.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.streams, scene.streams);
    }

    #[test]
    fn set_asset_op_applies_and_reverts_including_absent_case() {
        let scene = default_remodel_scene();
        assert!(!scene.assets.contains_key("frame-1"));

        let asset = ImageAsset { mime: "image/jpeg".into(), data: "zzz".into(), width: 2, height: 2 };
        let insert_operation = RemodelOperation::SetAsset { key: "frame-1".into(), value: Some(asset.clone()) };
        let after_insert = apply_remodel_operation(&scene, &insert_operation);
        assert_eq!(after_insert.assets.get("frame-1"), Some(&asset));
        assert_eq!(insert_operation.diff(&scene).apply(&scene).assets.get("frame-1"), Some(&asset));

        let insert_inverse = insert_operation.backwards(&scene);
        assert_eq!(insert_inverse, vec![RemodelOperation::SetAsset { key: "frame-1".into(), value: None }]);
        let reverted = insert_inverse.iter().fold(after_insert.clone(), |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted, scene);

        let remove_operation = RemodelOperation::SetAsset { key: "frame-1".into(), value: None };
        let remove_inverse = remove_operation.backwards(&after_insert);
        assert_eq!(remove_inverse, vec![RemodelOperation::SetAsset { key: "frame-1".into(), value: Some(asset.clone()) }]);
        let after_remove = apply_remodel_operation(&after_insert, &remove_operation);
        assert!(!after_remove.assets.contains_key("frame-1"));
        let restored = remove_inverse.iter().fold(after_remove, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(restored.assets.get("frame-1"), Some(&asset));
    }

    #[test]
    fn set_calibration_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let mut calibration = scene.calibration.clone();
        calibration.cameras.push(CameraCalibration { id: "cam-1".into(), model: "pinhole".into(), ..CameraCalibration::default() });
        let operation = RemodelOperation::SetCalibration { calibration: calibration.clone() };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.calibration, calibration);
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetCalibration { calibration: scene.calibration.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.calibration, scene.calibration);
    }

    #[test]
    fn set_gcps_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let gcps = vec![GroundControlPoint { id: "gcp-1".into(), name: "A".into(), world_position: [0.0, 0.0, 0.0], observations: Vec::new() }];
        let operation = RemodelOperation::SetGcps { gcps: gcps.clone() };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.gcps, gcps);
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetGcps { gcps: scene.gcps.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.gcps, scene.gcps);
    }

    #[test]
    fn set_job_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let mut job = scene.job.clone();
        job.stage = ReconstructionStage::BundleAdjusting;
        job.progress_0_1 = 0.42;
        job.started_at_ms = Some(1000.0);
        let operation = RemodelOperation::SetJob { job: job.clone() };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.job, job);
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetJob { job: scene.job.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.job, scene.job);
    }

    #[test]
    fn set_sparse_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let sparse = SparseCloud { points: PackedF32::from_f32_slice(&[0.0, 0.0, 0.0]), colors: Some(PackedU8::from_u8_slice(&[255, 255, 255])) };
        let operation = RemodelOperation::SetSparse { sparse: Some(sparse.clone()) };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.results.sparse, Some(sparse));
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetSparse { sparse: scene.results.sparse.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.results.sparse, scene.results.sparse);
    }

    #[test]
    fn set_dense_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let dense = DenseCloud { positions: PackedF32::from_f32_slice(&[1.0, 2.0, 3.0]), colors: None, confidence: Some(PackedF32::from_f32_slice(&[0.8])), classification: Some(PackedU8::from_u8_slice(&[2])) };
        let operation = RemodelOperation::SetDense { dense: Some(dense.clone()) };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.results.dense, Some(dense));
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetDense { dense: scene.results.dense.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.results.dense, scene.results.dense);
    }

    #[test]
    fn set_mesh_result_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let mesh = RemodelMesh {
            mesh: semio_framework_core::mesh_from_kind("box"),
            source: MeshSource::Reconstructed,
            texture_asset_id: Some("tex-1".into()),
            watertight: Some(WatertightReportSnapshot { is_watertight: true, is_two_manifold: true, is_closed: true, ..WatertightReportSnapshot::default() }),
        };
        let operation = RemodelOperation::SetMeshResult { mesh: Box::new(mesh.clone()) };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.results.mesh, mesh);
        assert_eq!(operation.diff(&scene).apply(&scene).results.mesh, mesh);
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetMeshResult { mesh: Box::new(scene.results.mesh.clone()) }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.results.mesh, scene.results.mesh);
    }

    #[test]
    fn set_trajectory_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let trajectory = CameraTrajectory { poses: vec![CameraPosePreview { camera_id: "cam-1".into(), ..CameraPosePreview::default() }] };
        let operation = RemodelOperation::SetTrajectory { trajectory: Some(trajectory.clone()) };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.results.trajectory, Some(trajectory));
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetTrajectory { trajectory: scene.results.trajectory.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.results.trajectory, scene.results.trajectory);
    }

    #[test]
    fn set_tracks_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let tracks = vec![MotionTrackSummary { id: "t1".into(), length: 12, class: TrackClass::Moving, mean_speed_m_s: 0.5 }];
        let operation = RemodelOperation::SetTracks { tracks: tracks.clone() };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.results.tracks, tracks);
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetTracks { tracks: scene.results.tracks.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.results.tracks, scene.results.tracks);
    }

    #[test]
    fn set_geo_products_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let geo = GeoProducts { dsm_asset_id: Some("dsm".into()), dtm_asset_id: None, ortho_asset_id: Some("ortho".into()) };
        let operation = RemodelOperation::SetGeoProducts { geo: Some(geo.clone()) };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.results.geo, Some(geo));
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetGeoProducts { geo: scene.results.geo.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
        assert_eq!(reverted.results.geo, scene.results.geo);
    }

    #[test]
    fn set_qc_op_applies_and_reverts() {
        let scene = default_remodel_scene();
        let qc = QcReportSnapshot { reprojection_rms_px: 0.6, warnings: vec!["w".into()], ..QcReportSnapshot::default() };
        let operation = RemodelOperation::SetQc { qc: Some(qc.clone()) };
        let next = apply_remodel_operation(&scene, &operation);
        assert_eq!(next.results.qc, Some(qc));
        let inverse = operation.backwards(&scene);
        assert_eq!(inverse, vec![RemodelOperation::SetQc { qc: scene.results.qc.clone() }]);
        let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
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
                let operation = RemodelOperation::$variant { params: params.clone() };
                let next = apply_remodel_operation(&scene, &operation);
                assert_eq!(next.params.$field, params);
                assert_eq!(operation.diff(&scene).apply(&scene).params.$field, params);
                let inverse = operation.backwards(&scene);
                assert_eq!(inverse, vec![RemodelOperation::$variant { params: scene.params.$field.clone() }]);
                let reverted = inverse.iter().fold(next, |current, operation| apply_remodel_operation(&current, operation));
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
        let op_a = RemodelOperation::SetAsset { key: "frame-1".into(), value: Some(asset_a.clone()) };
        let op_b = RemodelOperation::SetAsset { key: "frame-2".into(), value: Some(asset_b.clone()) };

        let a_then_b = apply_remodel_operation(&apply_remodel_operation(&base, &op_a), &op_b);
        let b_then_a = apply_remodel_operation(&apply_remodel_operation(&base, &op_b), &op_a);

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
        let op_feature = RemodelOperation::SetFeatureParams { params: feature_params.clone() };
        let gcps = vec![GroundControlPoint { id: "gcp-1".into(), name: "Corner".into(), world_position: [1.0, 2.0, 3.0], observations: Vec::new() }];
        let op_gcp = RemodelOperation::SetGcps { gcps: gcps.clone() };

        let feature_then_gcp = apply_remodel_operation(&apply_remodel_operation(&base, &op_feature), &op_gcp);
        let gcp_then_feature = apply_remodel_operation(&apply_remodel_operation(&base, &op_gcp), &op_feature);

        assert_eq!(feature_then_gcp, gcp_then_feature);
        assert_eq!(feature_then_gcp.params.feature, feature_params);
        assert_eq!(feature_then_gcp.gcps, gcps);
    }
    //#endregion Convergence

    //#region 🔖️OpText
    /// 🏗️ Verbatim duplicate of the `rs` crate's own private test-only fixture builder (see that
    /// crate's `populated_scene_fixture` doc comment) — needed here so every `RemodelOperation`
    /// variant below can be exercised against a document that actually populates every field.
    fn populated_scene_fixture() -> RemodelProjection {
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
            mesh: semio_framework_core::mesh_from_kind("box"),
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

    /// ⚡️ One `assert_op_line_round_trip` per `RemodelOperation` variant, per the mechanism contract.
    #[test]
    fn every_operation_variant_roundtrips_through_op_text() {
        let scene = populated_scene_fixture();

        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetStreams { streams: scene.streams.clone() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetAsset { key: "asset-1".into(), value: scene.assets.get("asset-1").cloned() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetAsset { key: "asset-2".into(), value: None });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetCalibration { calibration: scene.calibration.clone() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetGcps { gcps: scene.gcps.clone() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetIngestParams { params: scene.params.ingest.clone() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetFeatureParams { params: scene.params.feature.clone() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetMatchParams { params: scene.params.matching.clone() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetSfmParams { params: scene.params.sfm.clone() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetDenseParams { params: scene.params.dense.clone() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetMeshParams { params: scene.params.mesh.clone() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetMotionParams { params: scene.params.motion.clone() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetGeoParams { params: scene.params.geo.clone() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetJob { job: scene.job.clone() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetSparse { sparse: scene.results.sparse.clone() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetSparse { sparse: None });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetDense { dense: scene.results.dense.clone() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetDense { dense: None });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetMeshResult { mesh: Box::new(scene.results.mesh.clone()) });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetTrajectory { trajectory: scene.results.trajectory.clone() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetTrajectory { trajectory: None });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetTracks { tracks: scene.results.tracks.clone() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetGeoProducts { geo: scene.results.geo.clone() });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetGeoProducts { geo: None });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetQc { qc: scene.results.qc });
        store::test_support::assert_op_line_round_trip(&RemodelOperation::SetQc { qc: None });
    }
    //#endregion 🔖️OpText
}
//#endregion 🧪️Tests
