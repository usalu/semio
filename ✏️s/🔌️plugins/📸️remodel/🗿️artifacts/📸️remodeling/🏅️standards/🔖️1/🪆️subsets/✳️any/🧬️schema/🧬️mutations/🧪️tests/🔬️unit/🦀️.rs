use super::*;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::default_remodeling_scene;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::CameraCalibration;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::CameraPosePreview;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::CameraTrajectory;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::DenseCloud;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::FrameRef;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::GcpObservation;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::GroundControlPoint;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ImageAsset;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MediaKind;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MediaStream;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MeshSource;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::Float32Buffer;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ByteBuffer;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::QcReportSnapshot;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingContentKind;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingMesh;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RigExtrinsic;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::SparseCloud;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::TrackClass;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::VideoCodec;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::VideoSource;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::WatertightReportSnapshot;
use protocol::SemanticMutation;
use semio_os_kernel::os_spr::protocol_laws::assert_mutation_diff_absorb_law;

#[test]
fn retained_parameter_vectors_match_cold_decisions() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../🧫️retained-params/🧪️.json")).unwrap();
    let base = default_remodeling_scene();
    for case in cases.as_array().unwrap() {
        let change = case["change"].as_object().unwrap();
        let value = change.values().next();
        let op = match case["facet"].as_str().unwrap() {
            "ingest" => { let mut p = base.params.ingest.clone(); if let Some(v) = value { p.frame_sample_stride = v.as_u64().unwrap() as u32; } update_ingest_params(p) },
            "feature" => { let mut p = base.params.feature.clone(); if let Some(v) = value { p.target_count = v.as_u64().unwrap() as u32; } update_feature_params(p) },
            "matching" => { let mut p = base.params.matching.clone(); if let Some(v) = value { if change.contains_key("ratio_test") { p.ratio_test = v.as_f64().unwrap() as f32; } else { p.sequential_window = v.as_u64().unwrap() as u32; } } update_match_params(p) },
            "sfm" => { let mut p = base.params.sfm.clone(); if let Some(v) = value { if v.is_string() { p.ransac_threshold_px = f32::NAN; } else { p.ransac_iterations = v.as_u64().unwrap() as u32; } } update_sfm_params(p) },
            "dense" => { let mut p = base.params.dense.clone(); if let Some(v) = value { if v.is_string() { p.confidence_threshold = f32::NAN; } else { p.max_points = v.as_u64().unwrap() as u32; } } update_dense_params(p) },
            "mesh" => { let mut p = base.params.mesh.clone(); if let Some(v) = value { if v.is_string() { p.tsdf_voxel_size_mm = f32::NAN; } else { p.texture_size = v.as_u64().unwrap() as u32; } } update_mesh_params(p) },
            "motion" => { let mut p = base.params.motion.clone(); if let Some(v) = value { if v.is_string() { p.min_track_quality = f32::NAN; } else { p.enabled = v.as_bool().unwrap(); } } update_motion_params(p) },
            "geo" => { let mut p = base.params.geo.clone(); if let Some(v) = value { if v.is_boolean() { p.enabled = v.as_bool().unwrap(); } else { p.gsd_m = v.as_f64().unwrap() as f32; } } update_geo_params(p) },
            other => panic!("unknown facet {other}"),
        };
        let outcome = op.diff(&base);
        let actual = if outcome.messages().iter().any(|m| matches!(m.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)) { "refused" } else if outcome.messages().iter().any(|m| matches!(m.level, semio_framework_diagnostic::Severity::Warning)) { "unchanged" } else { "admitted" };
        assert_eq!(actual, case["expected"].as_str().unwrap(), "{case}");
    }
}

#[test]
fn retained_parameter_message_vectors_match_cold_diagnostics() {
    fn float(value: &serde_json::Value) -> f32 {
        match value.as_str() { Some("NaN") => f32::NAN, Some("inf") => f32::INFINITY, Some("-inf") => f32::NEG_INFINITY, Some(other) => panic!("unknown float {other}"), None => value.as_f64().unwrap() as f32 }
    }
    let cases: serde_json::Value = serde_json::from_str(include_str!("../🧫️retained-params/💬️.json")).unwrap();
    let base = default_remodeling_scene();
    for case in cases.as_array().unwrap() {
        let op = match case["facet"].as_str().unwrap() {
            "ingest" => { let mut p = base.params.ingest.clone(); p.min_sharpness = float(&case["a"]); p.max_frames = case["b"].as_u64().unwrap() as u32; p.frame_sample_stride = case["c"].as_u64().unwrap() as u32; update_ingest_params(p) },
            "feature" => { let mut p = base.params.feature.clone(); p.target_count = case["a"].as_u64().unwrap() as u32; p.edge_threshold = float(&case["b"]); update_feature_params(p) },
            "matching" => { let mut p = base.params.matching.clone(); p.ratio_test = float(&case["a"]); update_match_params(p) },
            other => panic!("unknown facet {other}"),
        };
        let outcome = op.diff(&base);
        assert_eq!(outcome.messages().len(), 1, "{case}");
        assert_eq!(outcome.messages()[0].message, case["message"].as_str().unwrap(), "{case}");
    }
}

/// ⚖️ Both inverse laws at once: sequential replay restores the base, and the inverse rows' diffs sum to the negative diff.
async fn laws<P, Op>(base: &P, mutation: &Op)
where
    P: Clone + PartialEq + std::fmt::Debug,
    Op: protocol::Mutation<P>,
{
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(base, mutation).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(mutation, base).await;
}


//#region 🔖️Fixture
/// ✅️ Applies `mutation` through its own diff and REFUSES an Error/Fatal outcome — a scenario step
/// that the ownership laws reject must fail the test loudly, never land as a silent no-op.
fn applied(base: &RemodelingSnapshot, mutation: &RemodelingMutation) -> RemodelingSnapshot {
    let outcome = mutation.diff(base);
    let rejected: Vec<_> = outcome.messages().iter().filter(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)).map(|message| format!("{message:?}")).collect();
    assert!(rejected.is_empty(), "scenario step {mutation:?} was rejected: {rejected:?}");
    protocol::apply_diff_cold(outcome.diff(), base).expect("valid mutation diff")
}

/// 🏗️ Shared fixture — a scene that exercises every optional/collection field at least once
/// (verbatim duplicate of the `rs`/`📝️text` crates' own private test-only builder — see that
/// crate's `populated_scene_fixture` doc comment).
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
    let mut scene = applied(&scene, &create_asset("asset-1".into(), asset_one));
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
    scene.calibration.rig.push(RigExtrinsic { camera_id: "cam-1".into(), ..RigExtrinsic::default() });
    scene.gcps.push(GroundControlPoint { id: "gcp-1".into(), name: "Corner".into(), world_position: [1.0, 2.0, 3.0], observations: vec![GcpObservation { stream_id: "stream-1".into(), frame_index: 0, pixel: [10.0, 20.0] }] });
    scene.params.ingest.min_sharpness = 0.4;
    scene.params.mesh.texture_size = 4096;
    scene.results.sparse = Some(SparseCloud { points: Float32Buffer::from_f32_slice(&[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]), colors: Some(ByteBuffer::from_u8_slice(&[255, 0, 0, 0, 255, 0])) });
    scene.results.dense =
        Some(DenseCloud { positions: Float32Buffer::from_f32_slice(&[0.0, 0.0, 0.0]), colors: Some(ByteBuffer::from_u8_slice(&[0, 0, 255])), confidence: Some(Float32Buffer::from_f32_slice(&[0.9])), classification: Some(ByteBuffer::from_u8_slice(&[2])) });
    scene.results.mesh = RemodelingMesh {
        mesh: semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::mint_and_stash_mesh(semio_framework::mesh_from_kind("box")),
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
    scene.results.tracks.push(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MotionTrackSummary { id: "track-1".into(), length: 42, class: TrackClass::Moving, mean_speed_m_s: 1.2 });
    scene.results.geo = Some(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::GeoProducts { dsm_asset_id: Some("asset-dsm".into()), dtm_asset_id: Some("asset-dtm".into()), ortho_asset_id: Some("asset-ortho".into()) });
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
//#endregion 🔖️Fixture

//#region 🔖️MutationLaws
#[semio_framework_async_macros::async_test]
async fn create_delete_stream_inverse_law() {
    let base = populated_scene_fixture();
    let stream = MediaStream { id: "stream-99".into(), name: "extra".into(), ..MediaStream::default() };
    laws(&base, &create_stream(stream)).await;
    let detached = applied(&base, &remove_gcp_observation("gcp-1".into(), 0));
    laws(&detached, &delete_stream("stream-1".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn change_stream_sync_inverse_law() {
    let base = populated_scene_fixture();
    laws(&base, &change_stream_sync("stream-1".into(), 99.0)).await;
}

#[semio_framework_async_macros::async_test]
async fn add_remove_stream_frame_inverse_law() {
    let base = populated_scene_fixture();
    laws(&base, &add_stream_frame("stream-1".into(), FrameRef { index: 1, timestamp_ms: 33.0, asset_id: "asset-2".into() }, MediaKind::Video)).await;
    laws(&base, &remove_stream_frame("stream-1".into(), 0)).await;
}

#[semio_framework_async_macros::async_test]
async fn replace_stream_source_inverse_law() {
    let base = populated_scene_fixture();
    laws(&base, &replace_stream_source("stream-1".into(), None)).await;
}

#[semio_framework_async_macros::async_test]
async fn create_delete_asset_inverse_law() {
    let base = populated_scene_fixture();
    let asset = ImageAsset { mime: "image/png".into(), data: "zzzz".into(), width: 2, height: 2 };
    laws(&base, &create_asset("asset-1".into(), asset.clone())).await;
    laws(&base, &create_asset("asset-2".into(), asset)).await;
    let detached = applied(&base, &remove_stream_frame("stream-1".into(), 0));
    laws(&detached, &delete_asset("asset-1".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn camera_calibration_inverse_law() {
    let base = populated_scene_fixture();
    let camera = CameraCalibration { id: "cam-99".into(), model: "pinhole".into(), ..CameraCalibration::default() };
    laws(&base, &create_camera_calibration(camera)).await;
    let updated = CameraCalibration { id: "cam-1".into(), fx: 2000.0, ..base.calibration.cameras[0].clone() };
    laws(&base, &update_camera_calibration(updated)).await;
    let detached = applied(&base, &remove_gcp_observation("gcp-1".into(), 0));
    let detached = applied(&detached, &delete_stream("stream-1".into()));
    let detached = applied(&detached, &delete_rig_extrinsic("cam-1".into()));
    laws(&detached, &delete_camera_calibration("cam-1".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn rig_extrinsic_inverse_law() {
    let base = populated_scene_fixture();
    let calibrated = applied(&base, &create_camera_calibration(CameraCalibration { id: "cam-99".into(), model: "pinhole".into(), ..CameraCalibration::default() }));
    let extrinsic = RigExtrinsic { camera_id: "cam-99".into(), ..RigExtrinsic::default() };
    laws(&calibrated, &create_rig_extrinsic(extrinsic)).await;
    let updated = RigExtrinsic { translation_m: [1.0, 0.0, 0.0], ..base.calibration.rig[0].clone() };
    laws(&base, &update_rig_extrinsic(updated)).await;
    laws(&base, &delete_rig_extrinsic(base.calibration.rig[0].camera_id.clone())).await;
}

#[semio_framework_async_macros::async_test]
async fn gcp_inverse_law() {
    let base = populated_scene_fixture();
    let gcp = GroundControlPoint { id: "gcp-99".into(), name: "New".into(), ..GroundControlPoint::default() };
    laws(&base, &create_gcp(gcp)).await;
    laws(&base, &delete_gcp("gcp-1".into())).await;
    laws(&base, &add_gcp_observation("gcp-1".into(), GcpObservation { stream_id: "stream-1".into(), frame_index: 1, pixel: [1.0, 2.0] })).await;
    laws(&base, &remove_gcp_observation("gcp-1".into(), 0)).await;
}

/// 📍️ Every collection is key-ordered, so deleting or removing a MIDDLE member is undone at its original position.
#[semio_framework_async_macros::async_test]
async fn middle_member_inverses_restore_the_original_position() {
    let mut base = populated_scene_fixture();
    for id in ["stream-0", "stream-5"] {
        base = applied(&base, &create_stream(MediaStream { id: id.into(), name: id.into(), ..MediaStream::default() }));
    }
    for id in ["gcp-0", "gcp-5"] {
        base = applied(&base, &create_gcp(GroundControlPoint { id: id.into(), name: id.into(), ..GroundControlPoint::default() }));
    }
    for id in ["cam-0", "cam-5"] {
        base = applied(&base, &create_camera_calibration(CameraCalibration { id: id.into(), model: "pinhole".into(), ..CameraCalibration::default() }));
        base = applied(&base, &create_rig_extrinsic(RigExtrinsic { camera_id: id.into(), ..RigExtrinsic::default() }));
    }
    for index in 1..4 {
        base = applied(&base, &add_stream_frame("stream-5".into(), FrameRef { index, timestamp_ms: index as f64 * 33.0, asset_id: format!("asset-{index}") }, MediaStream::default().kind));
    }
    for frame in 1..4 {
        base = applied(&base, &add_gcp_observation("gcp-5".into(), GcpObservation { stream_id: "stream-5".into(), frame_index: frame, pixel: [1.0, 2.0] }));
    }
    assert!(base.streams.len() >= 3 && base.gcps.len() >= 3 && base.calibration.cameras.len() >= 3);
    let detached = applied(&base, &remove_gcp_observation("gcp-1".into(), 0));
    laws(&detached, &delete_stream("stream-1".into())).await;
    laws(&base, &delete_gcp("gcp-1".into())).await;
    laws(&base, &delete_rig_extrinsic("cam-1".into())).await;
    laws(&base, &remove_stream_frame("stream-5".into(), 1)).await;
    laws(&base, &remove_gcp_observation("gcp-5".into(), 1)).await;
}

#[semio_framework_async_macros::async_test]
async fn update_params_inverse_law() {
    let base = populated_scene_fixture();
    laws(&base, &update_ingest_params(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::IngestParams { min_sharpness: 0.9, ..base.params.ingest.clone() })).await;
    laws(&base, &update_feature_params(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::FeatureParams { target_count: 1, ..base.params.feature.clone() })).await;
    laws(&base, &update_match_params(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MatchParams { ratio_test: 0.1, ..base.params.matching.clone() })).await;
    laws(&base, &update_sfm_params(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::SfmParams { ransac_iterations: 1, ..base.params.sfm.clone() })).await;
    laws(&base, &update_dense_params(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::DenseParams { max_points: 1, ..base.params.dense.clone() })).await;
    laws(&base, &update_mesh_params(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MeshParams { texture_size: 1, ..base.params.mesh.clone() })).await;
    laws(&base, &update_motion_params(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MotionParams { enabled: true, ..base.params.motion.clone() })).await;
    laws(&base, &update_geo_params(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::GeoParams { enabled: true, ..base.params.geo.clone() })).await;
}

#[semio_framework_async_macros::async_test]
async fn results_and_content_inverse_law() {
    let base = populated_scene_fixture();
    let leaf = ByteBuffer(vec![0u8; 12]);
    let append = AppendContent { content_id: "remodeling-asset-unit".into(), kind: RemodelingContentKind::Sparse, mime: None, width: 0, height: 0, first: 0, chunks: vec![leaf.clone()] };
    laws(&base, &append_content(append.clone())).await;
    let published = apply_remodeling_mutation(&base, &append_content(append)).expect("append applies");
    laws(&published, &append_content(AppendContent { content_id: "remodeling-asset-unit".into(), kind: RemodelingContentKind::Sparse, mime: None, width: 0, height: 0, first: 1, chunks: vec![leaf] })).await;
    laws(&published, &remove_content("remodeling-asset-unit".into(), 0)).await;
    laws(&published, &commit_reconstruction(CommitReconstruction { sparse: Some(SparseCloud { points: Float32Buffer::Content { content_id: "remodeling-asset-unit".into(), chunk_count: 1 }, colors: None }), trajectory: None, mesh: None, geo: None, qc: None, assets: Vec::new() })).await;
    laws(&base, &replace_sparse(None)).await;
    laws(&base, &replace_dense(None)).await;
    laws(&base, &replace_mesh_result(Box::new(RemodelingMesh::default()))).await;
    laws(&base, &replace_trajectory(None)).await;
    laws(&base, &replace_tracks(Vec::new())).await;
    laws(&base, &replace_geo_products(None)).await;
    laws(&base, &replace_qc(None)).await;
}

#[semio_framework_async_macros::async_test]
async fn move_step_style_diff_absorb_law() {
    let base = populated_scene_fixture();
    let d1 = change_stream_sync("stream-1".into(), 10.0).diff(&base).into_parts().0;
    let mid = protocol::apply_diff_cold(&d1, &base).expect("valid mutation diff");
    let d2 = change_stream_sync("stream-1".into(), 20.0).diff(&mid).into_parts().0;
    assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn dispatch_registers_semantic_descriptors() {
    register_remodeling_mutation_descriptors(::semio_framework_schema_state::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in RemodelingMutation::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(RemodelingMutation::kinds().len(), 36);
}
//#endregion 🔖️MutationLaws

//#region 🔖️Convergence
/// 🔀️ The CRDT convergence contract: two collaborators concurrently importing different assets
/// (`create-asset` on disjoint keys) must converge to an identical scene regardless of application
/// order — the reason `create-asset` clones+inserts into `base.assets` rather than any whole-map
/// replace that could drop a concurrent key.
#[semio_framework_async_macros::async_test]
async fn concurrent_create_asset_ops_converge_regardless_of_order() {
    let base = populated_scene_fixture();
    let asset_a = ImageAsset { mime: "image/jpeg".into(), data: base64_codec::base64_standard_encode(b"frame-one"), width: 8, height: 8 };
    let asset_b = ImageAsset { mime: "image/jpeg".into(), data: base64_codec::base64_standard_encode(b"frame-two"), width: 8, height: 8 };
    let op_a = create_asset("frame-a".into(), asset_a.clone());
    let op_b = create_asset("frame-b".into(), asset_b.clone());

    let a = applied(&base, &op_a);
    let b = applied(&base, &op_b);
    let a_then_b = applied(&a, &op_b);
    let b_then_a = applied(&b, &op_a);

    assert_eq!(a_then_b, b_then_a, "concurrent create-asset on disjoint keys must converge regardless of order");
    assert_eq!(a_then_b.assets.get("frame-a"), Some(&semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::image_asset_child_handle("frame-a", &asset_a)));
    assert_eq!(a_then_b.assets.get("frame-b"), Some(&semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::image_asset_child_handle("frame-b", &asset_b)));
    assert_eq!(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::remodeling_asset(&a_then_b, "frame-a"), Some(asset_a));
    assert_eq!(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::remodeling_asset(&a_then_b, "frame-b"), Some(asset_b));
}

/// 🔀️ Same convergence contract across two disjoint operation families (feature params tuning vs.
/// adding a GCP) — proves field-granular application converges across the whole vocabulary.
#[semio_framework_async_macros::async_test]
async fn concurrent_edits_across_different_op_families_converge() {
    let base = populated_scene_fixture();
    let op_feature = update_feature_params(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::FeatureParams { target_count: 9000, ..base.params.feature.clone() });
    let gcp = GroundControlPoint { id: "gcp-99".into(), name: "New".into(), ..GroundControlPoint::default() };
    let op_gcp = create_gcp(gcp.clone());

    let feature = apply_remodeling_mutation(&base, &op_feature).expect("valid mutation diff");
    let gcp = apply_remodeling_mutation(&base, &op_gcp).expect("valid mutation diff");
    let feature_then_gcp = apply_remodeling_mutation(&feature, &op_gcp).expect("valid mutation diff");
    let gcp_then_feature = apply_remodeling_mutation(&gcp, &op_feature).expect("valid mutation diff");

    assert_eq!(feature_then_gcp, gcp_then_feature);
    assert_eq!(feature_then_gcp.params.feature.target_count, 9000);
    assert!(feature_then_gcp.gcps.iter().any(|entry| entry.id == "gcp-99"));
}
//#endregion 🔖️Convergence

//#region 🔖️OpText
/// ⚡️ One `assert_op_line_round_trip` per `RemodelingMutation` variant, per the mechanism contract.
#[semio_framework_async_macros::async_test]
async fn every_mutation_variant_roundtrips_through_op_text() {
    let scene = populated_scene_fixture();

    store::os_store::test_support::assert_op_line_round_trip(&create_stream(scene.streams[0].clone()));
    store::os_store::test_support::assert_op_line_round_trip(&delete_stream("stream-1".into()));
    store::os_store::test_support::assert_op_line_round_trip(&change_stream_sync("stream-1".into(), 42.0));
    store::os_store::test_support::assert_op_line_round_trip(&add_stream_frame("stream-1".into(), FrameRef { index: 1, timestamp_ms: 33.0, asset_id: "asset-2".into() }, MediaKind::Video));
    store::os_store::test_support::assert_op_line_round_trip(&remove_stream_frame("stream-1".into(), 0));
    store::os_store::test_support::assert_op_line_round_trip(&replace_stream_source("stream-1".into(), scene.streams[0].source.clone()));
    store::os_store::test_support::assert_op_line_round_trip(&replace_stream_source("stream-1".into(), None));
    store::os_store::test_support::assert_op_line_round_trip(&create_asset("asset-1".into(), ImageAsset { mime: "image/jpeg".into(), data: "abcd".into(), width: 4, height: 4 }));
    store::os_store::test_support::assert_op_line_round_trip(&delete_asset("asset-2".into()));
    store::os_store::test_support::assert_op_line_round_trip(&create_camera_calibration(scene.calibration.cameras[0].clone()));
    store::os_store::test_support::assert_op_line_round_trip(&update_camera_calibration(scene.calibration.cameras[0].clone()));
    store::os_store::test_support::assert_op_line_round_trip(&delete_camera_calibration("cam-1".into()));
    store::os_store::test_support::assert_op_line_round_trip(&create_rig_extrinsic(scene.calibration.rig[0].clone()));
    store::os_store::test_support::assert_op_line_round_trip(&update_rig_extrinsic(scene.calibration.rig[0].clone()));
    store::os_store::test_support::assert_op_line_round_trip(&delete_rig_extrinsic(scene.calibration.rig[0].camera_id.clone()));
    store::os_store::test_support::assert_op_line_round_trip(&create_gcp(scene.gcps[0].clone()));
    store::os_store::test_support::assert_op_line_round_trip(&delete_gcp("gcp-1".into()));
    store::os_store::test_support::assert_op_line_round_trip(&add_gcp_observation("gcp-1".into(), scene.gcps[0].observations[0].clone()));
    store::os_store::test_support::assert_op_line_round_trip(&remove_gcp_observation("gcp-1".into(), 0));
    store::os_store::test_support::assert_op_line_round_trip(&update_ingest_params(scene.params.ingest.clone()));
    store::os_store::test_support::assert_op_line_round_trip(&update_feature_params(scene.params.feature.clone()));
    store::os_store::test_support::assert_op_line_round_trip(&update_match_params(scene.params.matching.clone()));
    store::os_store::test_support::assert_op_line_round_trip(&update_sfm_params(scene.params.sfm.clone()));
    store::os_store::test_support::assert_op_line_round_trip(&update_dense_params(scene.params.dense.clone()));
    store::os_store::test_support::assert_op_line_round_trip(&update_mesh_params(scene.params.mesh.clone()));
    store::os_store::test_support::assert_op_line_round_trip(&update_motion_params(scene.params.motion.clone()));
    store::os_store::test_support::assert_op_line_round_trip(&update_geo_params(scene.params.geo.clone()));
    store::os_store::test_support::assert_op_line_round_trip(&append_content(AppendContent { content_id: "remodeling-asset-unit".into(), kind: RemodelingContentKind::Image, mime: Some("image/png".into()), width: 2, height: 3, first: 4, chunks: vec![ByteBuffer(vec![0, 1, 2])] }));
    store::os_store::test_support::assert_op_line_round_trip(&remove_content("remodeling-asset-unit".into(), 2));
    store::os_store::test_support::assert_op_line_round_trip(&commit_reconstruction(CommitReconstruction { sparse: scene.results.sparse.clone(), trajectory: scene.results.trajectory.clone(), mesh: Some(Box::new(scene.results.mesh.clone())), geo: scene.results.geo.clone(), qc: scene.results.qc.clone(), assets: vec![ReconstructionAssetCommit { id: "dsm".into(), content_id: Some("remodeling-asset-unit".into()) }, ReconstructionAssetCommit { id: "dtm".into(), content_id: None }] }));
    store::os_store::test_support::assert_op_line_round_trip(&replace_sparse(scene.results.sparse.clone()));
    store::os_store::test_support::assert_op_line_round_trip(&replace_sparse(None));
    store::os_store::test_support::assert_op_line_round_trip(&replace_dense(scene.results.dense.clone()));
    store::os_store::test_support::assert_op_line_round_trip(&replace_dense(None));
    store::os_store::test_support::assert_op_line_round_trip(&replace_mesh_result(Box::new(scene.results.mesh.clone())));
    store::os_store::test_support::assert_op_line_round_trip(&replace_trajectory(scene.results.trajectory.clone()));
    store::os_store::test_support::assert_op_line_round_trip(&replace_trajectory(None));
    store::os_store::test_support::assert_op_line_round_trip(&replace_tracks(scene.results.tracks.clone()));
    store::os_store::test_support::assert_op_line_round_trip(&replace_geo_products(scene.results.geo.clone()));
    store::os_store::test_support::assert_op_line_round_trip(&replace_geo_products(None));
    store::os_store::test_support::assert_op_line_round_trip(&replace_qc(scene.results.qc));
    store::os_store::test_support::assert_op_line_round_trip(&replace_qc(None));
}
//#endregion 🔖️OpText

#[test]fn neutral_result_cases_match_cold_decisions_and_exact_inverse(){use protocol::ColdMutation;let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️retained-results/🔣️.json")).unwrap();let mut checked=0;for case in fixture["cases"].as_array().unwrap(){let field=case["field"].as_str().unwrap();if field=="mesh"{continue}let mut base=default_remodeling_scene();let original=case["original"].clone();let replacement=case["replacement"].clone();let op:RemodelingMutation=match field{"sparse"=>{base.results.sparse=serde_json::from_value(original).unwrap();replace_sparse(serde_json::from_value(replacement).unwrap())},"dense"=>{base.results.dense=serde_json::from_value(original).unwrap();replace_dense(serde_json::from_value(replacement).unwrap())},"trajectory"=>{base.results.trajectory=serde_json::from_value(original).unwrap();replace_trajectory(serde_json::from_value(replacement).unwrap())},"geo"=>{base.results.geo=serde_json::from_value(original).unwrap();replace_geo_products(serde_json::from_value(replacement).unwrap())},"qc"=>{base.results.qc=serde_json::from_value(original).unwrap();replace_qc(serde_json::from_value(replacement).unwrap())},"tracks"=>{base.results.tracks=serde_json::from_value(original).unwrap();replace_tracks(serde_json::from_value(replacement).unwrap())},_=>panic!("unknown result fixture")};let outcome=ColdMutation::diff(&op,&base);let actual=if outcome.messages().iter().any(|m|matches!(m.level,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal)){"missing"}else if outcome.messages().iter().any(|m|matches!(m.level,semio_framework_diagnostic::Severity::Warning)){"no-op"}else{"changed"};assert_eq!(actual,case["status"].as_str().unwrap(),"{case}");if actual!="changed"{assert_eq!(outcome.messages().len(),1);assert_eq!(outcome.messages()[0].message,case["message"].as_str().unwrap());assert_eq!(outcome.messages()[0].code.0,case["code"].as_str().unwrap());}if actual=="missing"{assert_eq!(outcome.messages()[0].target,vec![base.id.clone()]);}let inverse=ColdMutation::inverse(&op,&base).unwrap();assert_eq!(inverse.len(),1);let inverse_value=match &inverse[0]{RemodelingMutation::ReplaceSparse(v)=>serde_json::to_value(&v.sparse).unwrap(),RemodelingMutation::ReplaceDense(v)=>serde_json::to_value(&v.dense).unwrap(),RemodelingMutation::ReplaceTrajectory(v)=>serde_json::to_value(&v.trajectory).unwrap(),RemodelingMutation::ReplaceGeoProducts(v)=>serde_json::to_value(&v.geo).unwrap(),RemodelingMutation::ReplaceQc(v)=>serde_json::to_value(&v.qc).unwrap(),RemodelingMutation::ReplaceTracks(v)=>serde_json::to_value(&v.tracks).unwrap(),_=>panic!("wrong inverse variant")};assert_eq!(inverse_value,case["inverse"],"{case}");if actual=="changed"{let next=protocol::apply_diff_cold(outcome.diff(),&base).unwrap();let back=ColdMutation::diff(&inverse[0],&next);let restored=protocol::apply_diff_cold(back.diff(),&next).unwrap();assert_eq!(restored,base,"{case}");}checked+=1;}assert_eq!(checked,24);}

#[test]fn neutral_mesh_cases_use_actual_handles_and_durable_content(){use protocol::ColdMutation;use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::{placeholder_remodeling_mesh_handle,remodeling_mesh_content_handle,remodeling_content_is_complete,RemodelingDurableArtifact};fn projection(mesh:&RemodelingMesh)->serde_json::Value{serde_json::json!({"childId":&mesh.mesh.child_id,"handle":&mesh.mesh.target.artifact_id,"source":match mesh.source{MeshSource::Placeholder=>"placeholder",MeshSource::Reconstructed=>"reconstructed",MeshSource::Imported=>"imported"},"textureAssetId":&mesh.texture_asset_id})}let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️retained-results/🔣️.json")).unwrap();let mut checked=0;for case in fixture["cases"].as_array().unwrap().iter().filter(|c|c["field"]=="mesh"){let mut base=default_remodeling_scene();let content=case["durable"].as_str().unwrap();let mut old=RemodelingMesh::default();old.mesh=if content=="none"{placeholder_remodeling_mesh_handle()}else{remodeling_mesh_content_handle("mesh-content-1",1)};if content=="complete"{base.durable_artifacts.insert("mesh-content-1".into(),RemodelingDurableArtifact{kind:RemodelingContentKind::Mesh.wire().to_owned(),chunks:vec![ByteBuffer(vec![0])],..Default::default()});assert!(remodeling_content_is_complete(&base.durable_artifacts,"mesh-content-1",RemodelingContentKind::Mesh,1));}assert_eq!(projection(&old),case["original"]);base.results.mesh=old.clone();let mut replacement=old;replacement.source=if case["replacement"]["source"]=="imported"{MeshSource::Imported}else{MeshSource::Placeholder};assert_eq!(projection(&replacement),case["replacement"]);let op:RemodelingMutation=replace_mesh_result(Box::new(replacement));let outcome=ColdMutation::diff(&op,&base);let actual=if outcome.messages().iter().any(|m|matches!(m.level,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal)){"mismatch"}else if outcome.messages().iter().any(|m|matches!(m.level,semio_framework_diagnostic::Severity::Warning)){"no-op"}else{"changed"};assert_eq!(actual,case["status"].as_str().unwrap(),"{case}");if actual!="changed"{assert_eq!(outcome.messages().len(),1);assert_eq!(outcome.messages()[0].message,case["message"].as_str().unwrap());assert_eq!(outcome.messages()[0].code.0,case["code"].as_str().unwrap());}if actual=="mismatch"{assert_eq!(outcome.messages()[0].target,vec![case["target"].as_str().unwrap().to_owned()]);}let inverse=ColdMutation::inverse(&op,&base).unwrap();assert_eq!(inverse.len(),1);let inverse_value=match &inverse[0]{RemodelingMutation::ReplaceMeshResult(value)=>projection(&value.mesh),_=>panic!("wrong mesh inverse variant")};assert_eq!(inverse_value,case["inverse"]);if actual=="changed"{let next=protocol::apply_diff_cold(outcome.diff(),&base).unwrap();let back=ColdMutation::diff(&inverse[0],&next);let restored=protocol::apply_diff_cold(back.diff(),&next).unwrap();assert_eq!(restored,base);}checked+=1;}assert_eq!(checked,4);}

#[test]fn neutral_sparse_nan_is_not_equal_to_itself(){use protocol::ColdMutation;let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️retained-results/🔣️.json")).unwrap();let case=&f["ieeeCases"][0];assert_eq!(case["sample"],"NaN");let cloud=SparseCloud{points:Float32Buffer::Inline{values:vec![f32::NAN]},colors:None};assert!(!(cloud==cloud));let mut base=default_remodeling_scene();base.results.sparse=Some(cloud.clone());let op:RemodelingMutation=replace_sparse(Some(cloud));let outcome=ColdMutation::diff(&op,&base);assert!(outcome.messages().is_empty());assert!(outcome.diff().results.as_ref().and_then(|results|results.sparse.as_ref()).is_some());assert_eq!(case["status"],"changed");}
