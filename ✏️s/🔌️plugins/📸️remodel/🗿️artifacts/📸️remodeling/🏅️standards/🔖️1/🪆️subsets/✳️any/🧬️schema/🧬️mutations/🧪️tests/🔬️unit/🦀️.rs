use super::*;
use crate::{
    default_remodeling_scene, CameraCalibration, CameraPosePreview, CameraTrajectory, DenseCloud, FrameRef, GcpObservation, GroundControlPoint, ImageAsset, MediaKind, MediaStream, MeshSource, Float32Buffer, ByteBuffer, QcReportSnapshot,
    RemodelingContentKind, RemodelingMesh, RigExtrinsic, SparseCloud, TrackClass, VideoCodec, VideoSource, WatertightReportSnapshot,
};
use protocol::SemanticMutation;
use semio_framework_os_kernel::os_spr::protocol_laws::assert_mutation_diff_absorb_law;

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
    protocol::apply_diff(outcome.diff(), base).expect("valid mutation diff")
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
    // 🧩️ Seeded through `create-asset` itself so the handle's durable leaves exist — a bare
    // `assets.insert` leaves a handle without content, which no inverse can restore.
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
    // 🧷️ A rig extrinsic may only name a known camera (`create-rig-extrinsic`'s invariant).
    scene.calibration.rig.push(RigExtrinsic { camera_id: "cam-1".into(), ..RigExtrinsic::default() });
    scene.gcps.push(GroundControlPoint { id: "gcp-1".into(), name: "Corner".into(), world_position: [1.0, 2.0, 3.0], observations: vec![GcpObservation { stream_id: "stream-1".into(), frame_index: 0, pixel: [10.0, 20.0] }] });
    scene.params.ingest.min_sharpness = 0.4;
    scene.params.mesh.texture_size = 4096;
    scene.results.sparse = Some(SparseCloud { points: Float32Buffer::from_f32_slice(&[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]), colors: Some(ByteBuffer::from_u8_slice(&[255, 0, 0, 0, 255, 0])) });
    scene.results.dense =
        Some(DenseCloud { positions: Float32Buffer::from_f32_slice(&[0.0, 0.0, 0.0]), colors: Some(ByteBuffer::from_u8_slice(&[0, 0, 255])), confidence: Some(Float32Buffer::from_f32_slice(&[0.9])), classification: Some(ByteBuffer::from_u8_slice(&[2])) });
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
    laws(&base, &create_stream(stream)).await;
    // 🔗️ `gcp-1` observes `stream-1`; the delete refuses (`mutation.target-referenced`) until that
    // observation is removed.
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
    // 🎯️ `remove-stream-frame`'s inverse only round-trips exactly for the LAST frame (see its
    // payload's doc comment) — target index 0, the only frame `populated_scene_fixture` seeds.
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
    // 🔗️ `stream-1`'s only frame addresses `asset-1`; detach it before the delete.
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
    // 🔗️ `stream-1` binds `cam-1` and the rig carries its extrinsic; detach both (the stream first
    // loses its GCP observation, which would otherwise refuse the stream delete).
    let detached = applied(&base, &remove_gcp_observation("gcp-1".into(), 0));
    let detached = applied(&detached, &delete_stream("stream-1".into()));
    let detached = applied(&detached, &delete_rig_extrinsic("cam-1".into()));
    laws(&detached, &delete_camera_calibration("cam-1".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn rig_extrinsic_inverse_law() {
    let base = populated_scene_fixture();
    // 📷️ An extrinsic may only name a known camera: calibrate `cam-99` before rigging it.
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
    // 🎯️ Same last-index constraint as `remove-stream-frame` — target the only seeded observation.
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
    laws(&base, &update_ingest_params(crate::IngestParams { min_sharpness: 0.9, ..base.params.ingest.clone() })).await;
    laws(&base, &update_feature_params(crate::FeatureParams { target_count: 1, ..base.params.feature.clone() })).await;
    laws(&base, &update_match_params(crate::MatchParams { ratio_test: 0.1, ..base.params.matching.clone() })).await;
    laws(&base, &update_sfm_params(crate::SfmParams { ransac_iterations: 1, ..base.params.sfm.clone() })).await;
    laws(&base, &update_dense_params(crate::DenseParams { max_points: 1, ..base.params.dense.clone() })).await;
    laws(&base, &update_mesh_params(crate::MeshParams { texture_size: 1, ..base.params.mesh.clone() })).await;
    laws(&base, &update_motion_params(crate::MotionParams { enabled: true, ..base.params.motion.clone() })).await;
    laws(&base, &update_geo_params(crate::GeoParams { enabled: true, ..base.params.geo.clone() })).await;
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
    let mid = protocol::apply_diff(&d1, &base).expect("valid mutation diff");
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
    // 🧩️ `ImageAsset.data` is base64 — a non-base64 payload is refused (`mutation.invariant`).
    let asset_a = ImageAsset { mime: "image/jpeg".into(), data: base64_codec::base64_standard_encode(b"frame-one"), width: 8, height: 8 };
    let asset_b = ImageAsset { mime: "image/jpeg".into(), data: base64_codec::base64_standard_encode(b"frame-two"), width: 8, height: 8 };
    let op_a = create_asset("frame-a".into(), asset_a.clone());
    let op_b = create_asset("frame-b".into(), asset_b.clone());

    let a = applied(&base, &op_a);
    let b = applied(&base, &op_b);
    let a_then_b = applied(&a, &op_b);
    let b_then_a = applied(&b, &op_a);

    assert_eq!(a_then_b, b_then_a, "concurrent create-asset on disjoint keys must converge regardless of order");
    // 🎯️ The handle is content-addressed (`image_asset_child_handle`), so it is identical for identical
    // `(mime, data)` regardless of who mints it; the durable leaves read back the exact asset.
    assert_eq!(a_then_b.assets.get("frame-a"), Some(&crate::image_asset_child_handle("frame-a", &asset_a)));
    assert_eq!(a_then_b.assets.get("frame-b"), Some(&crate::image_asset_child_handle("frame-b", &asset_b)));
    assert_eq!(crate::remodeling_asset(&a_then_b, "frame-a"), Some(asset_a));
    assert_eq!(crate::remodeling_asset(&a_then_b, "frame-b"), Some(asset_b));
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
