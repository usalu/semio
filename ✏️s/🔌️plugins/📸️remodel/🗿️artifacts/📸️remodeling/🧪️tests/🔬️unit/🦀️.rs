
use super::*;

/// 🏗️ Shared fixture for both the JSON and the `.remodeling` DSL round-trip tests: a scene that
/// exercises every optional/collection field at least once, so `assert_dsl_round_trip` (and the
/// pre-existing `populated_scene_roundtrips_through_json`) actually walk the full document shape
/// instead of just `default_remodeling_scene()`'s mostly-empty surface. Duplicated verbatim into every
/// taxonomy node that needs it (`🗣️dsl`, `🔧️op`, `🎒️pack`) since it is a private test-only builder.
fn populated_scene_fixture() -> RemodelingSnapshot {
    let mut scene = default_remodeling_scene();
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
    let asset_one = ImageAsset { mime: "image/jpeg".into(), data: "abcd".into(), width: 4, height: 4 };
    scene.assets.insert("asset-1".into(), store_remodeling_asset("asset-1", &asset_one));
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
    scene.results.mesh = RemodelingMesh {
        mesh: mint_and_stash_mesh(semio_framework::mesh_from_kind("box")),
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

#[semio_framework_async_macros::async_test]
async fn default_scene_has_placeholder_mesh() {
    let scene = default_remodeling_scene();
    assert_eq!(scene.results.mesh.source, MeshSource::Placeholder);
    let mesh = remodeling_mesh_workspace(&scene.results.mesh.mesh).expect("working-scene cache warm right after default_remodeling_scene()");
    assert!(!mesh.positions.is_empty());
    assert!(!mesh.indices.is_empty());
    assert_eq!(scene.results.mesh.watertight, None);
    assert!(scene.streams.is_empty());
    assert!(scene.assets.is_empty());
    assert!(scene.gcps.is_empty());
    assert_eq!(scene.job, ReconstructionJob::default());
    assert_eq!(scene.results.sparse, None);
    assert_eq!(scene.results.dense, None);
    assert_eq!(scene.results.trajectory, None);
    assert!(scene.results.tracks.is_empty());
    assert_eq!(scene.results.geo, None);
    assert_eq!(scene.results.qc, None);
}

#[semio_framework_async_macros::async_test]
async fn scene_roundtrips_through_json() {
    let scene = default_remodeling_scene();
    let json = pack::to_json_string(&scene);
    let parsed: RemodelingSnapshot = pack::from_json_str(&json).expect("deserialize");
    assert_eq!(parsed, scene);
}

#[semio_framework_async_macros::async_test]
async fn populated_scene_roundtrips_through_json() {
    let scene = populated_scene_fixture();
    let json = pack::to_json_string(&scene);
    let parsed: RemodelingSnapshot = pack::from_json_str(&json).expect("deserialize");
    assert_eq!(parsed, scene);
}

#[semio_framework_async_macros::async_test]
async fn packed_f32_roundtrips_exactly() {
    let values = vec![1.5_f32, -2.25, 3.0, f32::MIN_POSITIVE, -0.0];
    let packed = PackedF32::from_f32_slice(&values);
    let value = serde_json::to_value(&packed).expect("serialize");
    assert!(value.is_string(), "PackedF32 must serialize as a base64 string, got {value:?}");
    let parsed: PackedF32 = serde_json::from_value(value).expect("deserialize");
    assert_eq!(parsed, packed);
    assert_eq!(parsed.to_f32_vec(), values);

    let empty = PackedF32::default();
    assert!(empty.is_empty());
    assert_eq!(empty.to_f32_vec(), Vec::<f32>::new());
}

#[semio_framework_async_macros::async_test]
async fn packed_u8_roundtrips_exactly() {
    let values = vec![0_u8, 128, 255, 64];
    let packed = PackedU8::from_u8_slice(&values);
    let value = serde_json::to_value(&packed).expect("serialize");
    assert!(value.is_string(), "PackedU8 must serialize as a base64 string, got {value:?}");
    let parsed: PackedU8 = serde_json::from_value(value).expect("deserialize");
    assert_eq!(parsed, packed);
    assert_eq!(parsed.to_u8_vec(), values);

    let empty = PackedU8::default();
    assert!(empty.is_empty());
    assert_eq!(empty.to_u8_vec(), Vec::<u8>::new());
}

#[semio_framework_async_macros::async_test]
async fn reconstruction_stage_serde_is_stable() {
    let cases: [(ReconstructionStage, &str); 18] = [
        (ReconstructionStage::Idle, "\"idle\""),
        (ReconstructionStage::Ingesting, "\"ingesting\""),
        (ReconstructionStage::Calibrating, "\"calibrating\""),
        (ReconstructionStage::ExtractingFeatures, "\"extracting-features\""),
        (ReconstructionStage::MatchingFeatures, "\"matching-features\""),
        (ReconstructionStage::EstimatingPoses, "\"estimating-poses\""),
        (ReconstructionStage::BundleAdjusting, "\"bundle-adjusting\""),
        (ReconstructionStage::Georeferencing, "\"georeferencing\""),
        (ReconstructionStage::DenseStereo, "\"dense-stereo\""),
        (ReconstructionStage::FusingVolume, "\"fusing-volume\""),
        (ReconstructionStage::ExtractingSurface, "\"extracting-surface\""),
        (ReconstructionStage::CleaningMesh, "\"cleaning-mesh\""),
        (ReconstructionStage::Texturing, "\"texturing\""),
        (ReconstructionStage::TrackingMotion, "\"tracking-motion\""),
        (ReconstructionStage::DerivingGeoProducts, "\"deriving-geo-products\""),
        (ReconstructionStage::ReportingQc, "\"reporting-qc\""),
        (ReconstructionStage::Done, "\"done\""),
        (ReconstructionStage::Failed, "\"failed\""),
    ];
    for (stage, expected) in cases {
        assert_eq!(serde_json::to_string(&stage).expect("serialize"), expected);
    }
}
