//! 🌐️ Global averaging, loop closure and pose prior tests of the structure from motion: the rotation and translation averaging, the loop detection, the pose-graph optimisation and the similarity and ground-control priors. See [`super`].

use super::*;

// #region 🔖️GlobalTests
#[test]
fn rotation_and_translation_averaging_recover_pose_graph_from_noisy_relative_measurements() {
    let n = 7;
    let mut rng = Rng::from_seed(24_681_357);
    let true_rotations: Vec<So3> = (0..n).map(|i| if i == 0 { So3::identity() } else { So3::exp([0.3 * rng.next_f64() - 0.15, 0.3 * rng.next_f64() - 0.15, 0.3 * rng.next_f64() - 0.15]) }).collect();
    let true_centers: Vec<[f64; 3]> = (0..n).map(|i| if i == 0 { [0.0; 3] } else { [3.0 * (rng.next_f64() - 0.5), 3.0 * (rng.next_f64() - 0.5), 3.0 * (rng.next_f64() - 0.5)] }).collect();
    let mut edges_idx: Vec<(usize, usize)> = (0..n).map(|i| (i, (i + 1) % n)).collect();
    edges_idx.push((0, 3));
    edges_idx.push((1, 5));

    let mut relative_rotations = Vec::new();
    let mut relative_directions = Vec::new();
    for &(i, j) in &edges_idx {
        let noise = So3::exp([0.01 * (rng.next_f64() - 0.5), 0.01 * (rng.next_f64() - 0.5), 0.01 * (rng.next_f64() - 0.5)]);
        let rij = noise.semio_compose_rs(&true_rotations[j].semio_compose_rs(&true_rotations[i].inverse()));
        relative_rotations.push((i, j, rij));
        let dir_world = vec3d_normalize(vec3d_sub(true_centers[j], true_centers[i]));
        let local_dir = true_rotations[i].act(dir_world);
        relative_directions.push((i, j, local_dir));
    }

    let recovered_rotations = rotation_averaging(&relative_rotations);
    assert_eq!(recovered_rotations.len(), n);
    for i in 1..n {
        let err = vec3d_length(recovered_rotations[i].inverse().semio_compose_rs(&true_rotations[i]).log());
        assert!(err < 0.1, "node {i} rotation error {err} rad too high");
    }

    let recovered_centers = translation_averaging(&relative_directions, &recovered_rotations);
    let mut num = 0.0;
    let mut den = 0.0;
    for i in 1..n {
        num += dot3(recovered_centers[i], true_centers[i]);
        den += dot3(recovered_centers[i], recovered_centers[i]);
    }
    assert!(den > 1e-9, "recovered centers should be nontrivial");
    let scale = num / den;
    for i in 1..n {
        let rescaled = scale3(recovered_centers[i], scale);
        let err = vec3d_length(vec3d_sub(rescaled, true_centers[i]));
        let true_norm = vec3d_length(true_centers[i]).max(1e-6);
        assert!(err / true_norm < 0.2, "node {i} translation direction/scale error {} too high", err / true_norm);
    }
}
// #endregion 🔖️GlobalTests

// #region 🔖️LoopClosureTests
#[test]
fn keyframe_index_and_detect_loops_find_a_planted_revisit() {
    let mut rng = Rng::from_seed(112_233);
    let n_frames = 5;
    let n_shared = 20;
    let shared_kps_a: Vec<Keypoint> = (0..n_shared).map(|_| Keypoint { x: (rng.next_f64() * 500.0 + 50.0) as f32, y: (rng.next_f64() * 380.0 + 50.0) as f32, octave: 0, angle: 0.0, response: 1.0 }).collect();
    let h = [[1.0, 0.02, 15.0], [-0.01, 1.0, 8.0], [0.0001, 0.00005, 1.0]];
    let shared_kps_b: Vec<Keypoint> = shared_kps_a
        .iter()
        .map(|k| {
            let (x, y) = (f64::from(k.x), f64::from(k.y));
            let w = h[2][0] * x + h[2][1] * y + h[2][2];
            let px = (h[0][0] * x + h[0][1] * y + h[0][2]) / w;
            let py = (h[1][0] * x + h[1][1] * y + h[1][2]) / w;
            Keypoint { x: px as f32, y: py as f32, octave: 0, angle: 0.0, response: 1.0 }
        })
        .collect();
    let shared_desc: Vec<Descriptor256> = (0..n_shared).map(|i| Descriptor256([0x1234_5678_9ABC_DEF0_u64.wrapping_add(i as u64), 0xDEAD_BEEF_0000_0000, 0x0000_0000_CAFE_BABE, i as u64])).collect();

    let mut all_keypoints: Vec<Vec<Keypoint>> = Vec::new();
    let mut all_descriptors: Vec<Vec<Descriptor256>> = Vec::new();
    for frame in 0..n_frames {
        if frame == 0 {
            all_keypoints.push(shared_kps_a.clone());
            all_descriptors.push(shared_desc.clone());
        } else if frame == n_frames - 1 {
            all_keypoints.push(shared_kps_b.clone());
            all_descriptors.push(shared_desc.clone());
        } else {
            let kps: Vec<Keypoint> = (0..n_shared).map(|_| Keypoint { x: (rng.next_f64() * 600.0) as f32, y: (rng.next_f64() * 400.0) as f32, octave: 0, angle: 0.0, response: 1.0 }).collect();
            let descs: Vec<Descriptor256> = (0..n_shared).map(|_| Descriptor256([rng.next_u64(), rng.next_u64(), rng.next_u64(), rng.next_u64()])).collect();
            all_keypoints.push(kps);
            all_descriptors.push(descs);
        }
    }

    let mut index = KeyframeIndex::new();
    for (frame, descs) in all_descriptors.iter().enumerate().take(n_frames - 1) {
        index.insert(frame, descs);
    }
    let current_frame = n_frames - 1;
    let loops = detect_loops(&index, current_frame, &all_descriptors[current_frame], &all_keypoints[current_frame], &all_keypoints, &all_descriptors);
    assert!(loops.iter().any(|c| c.frame == 0), "expected the planted revisit at frame 0 to be detected, got {:?}", loops.iter().map(|c| c.frame).collect::<Vec<_>>());
}

#[test]
fn pose_graph_optimize_reduces_drift_at_loop_closure_edge() {
    let n = 5;
    let mut rng = Rng::from_seed(998_877);
    let mut true_poses = vec![Se3::identity()];
    for i in 1..n {
        let step = Se3 { r: So3::exp([0.05 * (rng.next_f64() - 0.5), 0.2, 0.02 * (rng.next_f64() - 0.5)]), t: [1.0, 0.05 * (rng.next_f64() - 0.5), 0.02 * (rng.next_f64() - 0.5)] };
        let prev = true_poses[i - 1];
        true_poses.push(step.semio_compose_rs(&prev));
    }

    let drift = Se3 { r: So3::exp([0.0, 0.02, 0.0]), t: [0.05, 0.0, 0.0] };
    let mut initial_poses = vec![Se3::identity()];
    let mut edges = Vec::new();
    for i in 1..n {
        let true_rel = true_poses[i].semio_compose_rs(&true_poses[i - 1].inverse());
        let noisy_rel = drift.semio_compose_rs(&true_rel);
        edges.push((i - 1, i, Sim3 { s: 1.0, r: noisy_rel.r, t: noisy_rel.t }));
        initial_poses.push(noisy_rel.semio_compose_rs(&initial_poses[i - 1]));
    }
    let loop_rel = true_poses[0].semio_compose_rs(&true_poses[n - 1].inverse());
    edges.push((n - 1, 0, Sim3 { s: 1.0, r: loop_rel.r, t: loop_rel.t }));

    let residual_of = |poses: &[Se3]| -> f64 {
        let predicted = poses[0].semio_compose_rs(&poses[n - 1].inverse());
        let err = loop_rel.inverse().semio_compose_rs(&predicted).log();
        err.iter().map(|v| v * v).sum::<f64>().sqrt()
    };
    let before = residual_of(&initial_poses);
    let optimized = pose_graph_optimize(&initial_poses, &edges);
    let after = residual_of(&optimized);
    assert!(after < before * 0.5, "pose_graph_optimize should reduce loop-closure drift: before {before}, after {after}");
}
// #endregion 🔖️LoopClosureTests

// #region 🔖️PriorsTests
#[test]
fn align_to_priors_recovers_planted_similarity() {
    let mut rng = Rng::from_seed(135_791);
    let intr = Intrinsics { fx: 800.0, fy: 800.0, cx: 320.0, cy: 240.0, skew: 0.0, distortion: Distortion::None };
    let mut cameras = Vec::new();
    for i in 0..6 {
        let pose = CameraPose(Se3 { r: So3::exp([0.1 * f64::from(i), 0.05, 0.0]), t: [rng.next_f64() * 2.0, rng.next_f64() * 2.0, rng.next_f64() * 2.0] });
        cameras.push((i as usize, pose));
    }
    let recon = Reconstruction { cameras: cameras.clone(), points: Vec::new(), point_track_ids: Vec::new(), intrinsics: intr };

    let truth = Sim3 { s: 3.5, r: So3::exp([0.2, -0.3, 0.1]), t: [10.0, -5.0, 2.0] };
    let gps_priors: Vec<(usize, [f64; 3])> = cameras.iter().map(|&(f, pose)| (f, truth.act(camera_center(&pose)))).collect();

    let recovered = align_to_priors(&recon, &gps_priors).expect("alignment should succeed for well-posed input");
    assert!((recovered.s - truth.s).abs() < 1e-6, "scale error: got {} want {}", recovered.s, truth.s);
    let rot_err = vec3d_length(recovered.r.inverse().semio_compose_rs(&truth.r).log());
    assert!(rot_err < 1e-6, "rotation error {rot_err}");
    let t_err = vec3d_length(vec3d_sub(recovered.t, truth.t));
    assert!(t_err < 1e-6, "translation error {t_err}");
}

#[test]
fn apply_gcp_prior_residual_matches_scaled_difference() {
    let point = [1.0, 2.0, 3.0];
    let known = [1.1, 1.9, 3.2];
    let (r, jb) = apply_gcp_prior_residual(point, known, 0.5);
    assert!((r.get(0) - (point[0] - known[0]) / 0.5).abs() < 1e-12);
    assert!((r.get(1) - (point[1] - known[1]) / 0.5).abs() < 1e-12);
    assert!((r.get(2) - (point[2] - known[2]) / 0.5).abs() < 1e-12);
    for k in 0..3 {
        assert!((jb.get(k, k) - 2.0).abs() < 1e-12);
    }
}
// #endregion 🔖️PriorsTests
