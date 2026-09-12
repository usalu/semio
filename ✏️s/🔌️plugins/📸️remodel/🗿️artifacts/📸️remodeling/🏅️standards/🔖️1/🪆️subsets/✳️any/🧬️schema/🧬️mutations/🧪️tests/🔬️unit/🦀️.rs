use super::*;
use crate::{
    default_remodeling_scene, CameraCalibration, CameraPosePreview, CameraTrajectory, DenseCloud, FrameRef, GcpObservation, GroundControlPoint, ImageAsset, MediaKind, MediaStream, MeshSource, PackedF32, PackedU8, QcReportSnapshot, ReconstructionJob,
    ReconstructionStage, RemodelingMesh, RigExtrinsic, SparseCloud, TrackClass, VideoCodec, VideoSource, WatertightReportSnapshot,
};
use protocol::SemanticMutation;
use semio_framework_os_kernel::os_spr::protocol_laws::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};

//#region 🔖️Fixture
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
    scene.assets.insert("asset-1".into(), crate::store_remodeling_asset("asset-1", &asset_one));
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
        mesh: crate::mint_and_stash_mesh(semio_framework::mesh_from_kind("box")),
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
    scene.results.tracks.push(crate::MotionTrackSummary { id: "track-1".into(), length: 42, class: TrackClass::Moving, mean_speed_m_s: 1.2 });
    scene.results.geo = Some(crate::GeoProducts { dsm_asset_id: Some("asset-dsm".into()), dtm_asset_id: Some("asset-dtm".into()), ortho_asset_id: Some("asset-ortho".into()) });
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
    assert_mutation_inverse_law(&base, &create_stream(stream)).await;
    assert_mutation_inverse_law(&base, &delete_stream("stream-1".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn change_stream_sync_inverse_law() {
    let base = populated_scene_fixture();
    assert_mutation_inverse_law(&base, &change_stream_sync("stream-1".into(), 99.0)).await;
}

#[semio_framework_async_macros::async_test]
async fn add_remove_stream_frame_inverse_law() {
    let base = populated_scene_fixture();
    assert_mutation_inverse_law(&base, &add_stream_frame("stream-1".into(), FrameRef { index: 1, timestamp_ms: 33.0, asset_id: "asset-2".into() }, MediaKind::Video)).await;
    // 🎯️ `remove-stream-frame`'s inverse only round-trips exactly for the LAST frame (see its
    // payload's doc comment) — target index 0, the only frame `populated_scene_fixture` seeds.
    assert_mutation_inverse_law(&base, &remove_stream_frame("stream-1".into(), 0)).await;
}

#[semio_framework_async_macros::async_test]
async fn replace_stream_source_inverse_law() {
    let base = populated_scene_fixture();
    assert_mutation_inverse_law(&base, &replace_stream_source("stream-1".into(), None)).await;
}

#[semio_framework_async_macros::async_test]
async fn create_delete_asset_inverse_law() {
    let base = populated_scene_fixture();
    let asset = ImageAsset { mime: "image/png".into(), data: "zzzz".into(), width: 2, height: 2 };
    assert_mutation_inverse_law(&base, &create_asset("asset-1".into(), asset.clone())).await;
    assert_mutation_inverse_law(&base, &create_asset("asset-2".into(), asset)).await;
    assert_mutation_inverse_law(&base, &delete_asset("asset-1".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn camera_calibration_inverse_law() {
    let base = populated_scene_fixture();
    let camera = CameraCalibration { id: "cam-99".into(), model: "pinhole".into(), ..CameraCalibration::default() };
    assert_mutation_inverse_law(&base, &create_camera_calibration(camera)).await;
    let updated = CameraCalibration { id: "cam-1".into(), fx: 2000.0, ..base.calibration.cameras[0].clone() };
    assert_mutation_inverse_law(&base, &update_camera_calibration(updated)).await;
    assert_mutation_inverse_law(&base, &delete_camera_calibration("cam-1".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn rig_extrinsic_inverse_law() {
    let base = populated_scene_fixture();
    let extrinsic = RigExtrinsic { camera_id: "cam-99".into(), ..RigExtrinsic::default() };
    assert_mutation_inverse_law(&base, &create_rig_extrinsic(extrinsic)).await;
    let updated = RigExtrinsic { translation_m: [1.0, 0.0, 0.0], ..base.calibration.rig[0].clone() };
    assert_mutation_inverse_law(&base, &update_rig_extrinsic(updated)).await;
    assert_mutation_inverse_law(&base, &delete_rig_extrinsic(base.calibration.rig[0].camera_id.clone())).await;
}

#[semio_framework_async_macros::async_test]
async fn gcp_inverse_law() {
    let base = populated_scene_fixture();
    let gcp = GroundControlPoint { id: "gcp-99".into(), name: "New".into(), ..GroundControlPoint::default() };
    assert_mutation_inverse_law(&base, &create_gcp(gcp)).await;
    assert_mutation_inverse_law(&base, &delete_gcp("gcp-1".into())).await;
    assert_mutation_inverse_law(&base, &add_gcp_observation("gcp-1".into(), GcpObservation { stream_id: "stream-1".into(), frame_index: 1, pixel: [1.0, 2.0] })).await;
    // 🎯️ Same last-index constraint as `remove-stream-frame` — target the only seeded observation.
    assert_mutation_inverse_law(&base, &remove_gcp_observation("gcp-1".into(), 0)).await;
}

#[semio_framework_async_macros::async_test]
async fn update_params_inverse_law() {
    let base = populated_scene_fixture();
    assert_mutation_inverse_law(&base, &update_ingest_params(crate::IngestParams { min_sharpness: 0.9, ..base.params.ingest.clone() })).await;
    assert_mutation_inverse_law(&base, &update_feature_params(crate::FeatureParams { target_count: 1, ..base.params.feature.clone() })).await;
    assert_mutation_inverse_law(&base, &update_match_params(crate::MatchParams { ratio_test: 0.1, ..base.params.matching.clone() })).await;
    assert_mutation_inverse_law(&base, &update_sfm_params(crate::SfmParams { ransac_iterations: 1, ..base.params.sfm.clone() })).await;
    assert_mutation_inverse_law(&base, &update_dense_params(crate::DenseParams { max_points: 1, ..base.params.dense.clone() })).await;
    assert_mutation_inverse_law(&base, &update_mesh_params(crate::MeshParams { texture_size: 1, ..base.params.mesh.clone() })).await;
    assert_mutation_inverse_law(&base, &update_motion_params(crate::MotionParams { enabled: true, ..base.params.motion.clone() })).await;
    assert_mutation_inverse_law(&base, &update_geo_params(crate::GeoParams { enabled: true, ..base.params.geo.clone() })).await;
}

#[semio_framework_async_macros::async_test]
async fn replace_job_and_results_inverse_law() {
    let base = populated_scene_fixture();
    assert_mutation_inverse_law(&base, &replace_job(ReconstructionJob { stage: ReconstructionStage::Failed, ..base.job.clone() })).await;
    assert_mutation_inverse_law(&base, &replace_sparse(None)).await;
    assert_mutation_inverse_law(&base, &replace_dense(None)).await;
    assert_mutation_inverse_law(&base, &replace_mesh_result(Box::new(RemodelingMesh::default()))).await;
    assert_mutation_inverse_law(&base, &replace_trajectory(None)).await;
    assert_mutation_inverse_law(&base, &replace_tracks(Vec::new())).await;
    assert_mutation_inverse_law(&base, &replace_geo_products(None)).await;
    assert_mutation_inverse_law(&base, &replace_qc(None)).await;
}

#[semio_framework_async_macros::async_test]
async fn move_step_style_diff_absorb_law() {
    let base = populated_scene_fixture();
    let d1 = change_stream_sync("stream-1".into(), 10.0).diff(&base).into_parts().0;
    let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
    let d2 = change_stream_sync("stream-1".into(), 20.0).diff(&mid).into_parts().0;
    assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn dispatch_registers_semantic_descriptors() {
    register_remodeling_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in RemodelingMutation::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(RemodelingMutation::kinds().len(), 34);
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
    let asset_a = ImageAsset { mime: "image/jpeg".into(), data: "frame-one".into(), width: 8, height: 8 };
    let asset_b = ImageAsset { mime: "image/jpeg".into(), data: "frame-two".into(), width: 8, height: 8 };
    let op_a = create_asset("frame-a".into(), asset_a.clone());
    let op_b = create_asset("frame-b".into(), asset_b.clone());

    let a = apply_remodeling_mutation(&base, &op_a).expect("valid mutation diff");
    let b = apply_remodeling_mutation(&base, &op_b).expect("valid mutation diff");
    let a_then_b = apply_remodeling_mutation(&a, &op_b).expect("valid mutation diff");
    let b_then_a = apply_remodeling_mutation(&b, &op_a).expect("valid mutation diff");

    assert_eq!(a_then_b, b_then_a, "concurrent create-asset on disjoint keys must converge regardless of order");
    // 🎯️ Both assets are `image/jpeg` (unsupported by the real png bridge today, see
    // `semio_image_snapshot_from_image_asset`'s doc comment), so `store_remodeling_asset` falls back
    // to the deterministic raw-bytes handle (`image_asset_child_handle`) — asserting on the HANDLE
    // (content-addressed, so identical for identical `(mime,data)` regardless of who mints it) is
    // the honest convergence check here, not a round-trip through the working-scene cache.
    assert_eq!(a_then_b.assets.get("frame-a"), Some(&crate::image_asset_child_handle("frame-a", &asset_a)));
    assert_eq!(a_then_b.assets.get("frame-b"), Some(&crate::image_asset_child_handle("frame-b", &asset_b)));
}

/// 🔀️ Same convergence contract across two disjoint operation families (feature params tuning vs.
/// adding a GCP) — proves field-granular application converges across the whole vocabulary.
#[semio_framework_async_macros::async_test]
async fn concurrent_edits_across_different_op_families_converge() {
    let base = populated_scene_fixture();
    let op_feature = update_feature_params(crate::FeatureParams { target_count: 9000, ..base.params.feature.clone() });
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
    store::os_store::test_support::assert_op_line_round_trip(&replace_job(scene.job.clone()));
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
