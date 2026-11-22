use super::*;

#[test]
fn worst_case_terminal_windows_never_exceed_their_budget() {
    let len = usize::MAX;
    let camera_end = bounded_terminal_end(17, len, 64);
    let point_end = bounded_terminal_end(23, len, 256);
    let quality_end = bounded_terminal_end(29, len, 256);
    assert_eq!(camera_end - 17, 64);
    assert_eq!(point_end - 23, 256);
    assert_eq!(quality_end - 29, 256);
}


#[test]
fn mixed_calibrations_reach_dense_texture_and_terminal_outputs() {
    use semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::{Reconstruction, synthetic_scene};

    let mut fixture = synthetic_scene(42, 2, 1, false);
    let mut engine = ReconstructionEngine::new(&EngineParams::default());
    let capture_indices = [(0, 3), (1, 3)];
    let dimensions = [(8, 6), (7, 5)];
    let mut cameras = Vec::new();
    let mut intrinsics = std::collections::BTreeMap::new();
    for (frame, ((calibration, pose), ((stream, index), (width, height)))) in fixture.cameras.iter_mut().zip(capture_indices.into_iter().zip(dimensions)).enumerate() {
        *calibration = remodeling_camera::Intrinsics { fx: 6.0 + frame as f64, fy: 8.0, cx: 3.0, cy: 2.0, skew: frame as f64, distortion: remodeling_camera::Distortion::None };
        let capture = FrameCapture { stream, intrinsics: Some(*calibration) };
        let image = remodeling_image::ImageRgba8 { width, height, data: vec![0; (width * height * 4) as usize] };
        assert_eq!(engine.push_frame_with_sharpness(index, image, frame as f64, 1.0, capture), FrameAcceptance::Accepted);
        cameras.push((frame, *pose));
        intrinsics.insert(frame, *calibration);
        let pixel = remodeling_camera::reproject(calibration, pose, fixture.points_world[0]).expect("visible point");
        engine.observations.push((frame, 0, pixel));
    }
    engine.start().expect("working frames");
    engine.reconstruction = Some(Reconstruction { cameras, points: fixture.points_world, point_track_ids: vec![0], intrinsics });
    engine.dense_camera_indices = vec![0, 1];

    assert!(engine.step_dense_stereo());
    assert!(engine.step_dense_stereo());
    let sources = &engine.dense_preparation.as_ref().expect("dense preparation").source_grays;
    assert_eq!((sources[0].0.width, sources[0].0.height, sources[0].2), (7, 5, fixture.cameras[1].0));
    assert!(engine.step_fusing_volume());
    assert!(engine.step_fusing_volume());
    assert_eq!(engine.fusion_views, fixture.cameras.iter().map(|&(calibration, pose)| (pose, calibration)).collect::<Vec<_>>());

    engine.tsdf = Some(remodeling_dense::TsdfVolume::new(0.05, 0.15));
    for _ in 0..3 {
        assert!(!engine.step_begin_meshing());
    }
    let views = &engine.meshing_preparation.as_ref().expect("texture preparation").views;
    assert_eq!(views.iter().map(|view| ((view.image.width, view.image.height), view.intrinsics)).collect::<Vec<_>>(),
        dimensions.into_iter().zip(fixture.cameras.iter().map(|&(calibration, _)| calibration)).collect::<Vec<_>>());

    let sparse = engine.terminal_sparse_chunk(0, 0, 2, 1);
    assert_eq!(sparse.camera_poses.iter().map(|&(entity, _)| entity).collect::<Vec<_>>(), [3, (1 << 32) | 3]);
    let quality = engine.terminal_quality_chunk(0, 2);
    assert_eq!(quality.observation_count, 2);
    assert_eq!(quality.squared_error_sum, 0.0);
}
