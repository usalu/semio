//! 🧭️ Incremental reconstruction tests of the structure from motion: the bounded step fixtures and ceilings, the full synthetic multi-camera run and the low-parallax initial pair. See [`super`].

use super::*;

// #region 🔖️IncrementalTests
fn bounded_seed_fixture(degenerate: bool) -> (IncrementalSfm, SeedPairPreparation) {
    let intrinsics = Intrinsics { fx: 500.0, fy: 500.0, cx: 256.0, cy: 256.0, skew: 0.0, distortion: Distortion::None };
    let left = CameraPose(Se3::identity());
    let right = CameraPose(Se3 { r: So3::identity(), t: [0.35, 0.0, 0.0] });
    let mut keypoints = vec![Vec::new(), Vec::new()];
    let mut tracks = Vec::new();
    let mut matches = Vec::new();
    for index in 0..64usize {
        let point = [((index % 8) as f64 - 3.5) * 0.12, ((index / 8) as f64 - 3.5) * 0.12, 4.0 + (index % 5) as f64 * 0.08];
        let left_pixel = if degenerate { [256.0; 2] } else { reproject(&intrinsics, &left, point).expect("left projection") };
        let right_pixel = if degenerate { [256.0; 2] } else { reproject(&intrinsics, &right, point).expect("right projection") };
        keypoints[0].push(Keypoint { x: left_pixel[0] as f32, y: left_pixel[1] as f32, octave: 0, angle: 0.0, response: 1.0 });
        keypoints[1].push(Keypoint { x: right_pixel[0] as f32, y: right_pixel[1] as f32, octave: 0, angle: 0.0, response: 1.0 });
        tracks.push(vec![(0, index as u32), (1, index as u32)]);
        matches.push(Match { a: index as u32, b: index as u32, distance: 0 });
    }
    let sfm = IncrementalSfm::new(intrinsics, FeatureTracks { tracks }, keypoints, SfmConfig::default());
    (sfm, SeedPairPreparation::new(0, 1, &matches))
}

fn bounded_registration_fixture(degenerate: bool) -> (IncrementalSfm, RegistrationPreparation) {
    let intrinsics = Intrinsics { fx: 500.0, fy: 500.0, cx: 256.0, cy: 256.0, skew: 0.0, distortion: Distortion::None };
    let registered = CameraPose(Se3::identity());
    let candidate = CameraPose(Se3 { r: So3::exp([0.02, -0.03, 0.01]), t: [0.2, 0.01, -0.02] });
    let mut keypoints = vec![Vec::new(), Vec::new()];
    let mut tracks = Vec::new();
    let mut points = std::collections::BTreeMap::new();
    for index in 0..MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES {
        let point = if degenerate { [0.0, 0.0, 4.0] } else { [((index % 8) as f64 - 3.5) * 0.14, ((index / 8) as f64 - 3.5) * 0.11, 4.0 + (index % 7) as f64 * 0.09] };
        let registered_pixel = reproject(&intrinsics, &registered, point).expect("registered projection");
        let candidate_pixel = reproject(&intrinsics, &candidate, point).expect("candidate projection");
        keypoints[0].push(Keypoint { x: registered_pixel[0] as f32, y: registered_pixel[1] as f32, octave: 0, angle: 0.0, response: 1.0 });
        keypoints[1].push(Keypoint { x: candidate_pixel[0] as f32, y: candidate_pixel[1] as f32, octave: 0, angle: 0.0, response: 1.0 });
        tracks.push(vec![(0, index as u32), (1, index as u32)]);
        points.insert(index, point);
    }
    let mut sfm = IncrementalSfm::new(intrinsics, FeatureTracks { tracks }, keypoints, SfmConfig::default());
    sfm.cameras.push((0, registered));
    sfm.points = points;
    (sfm, RegistrationPreparation::new(1))
}

fn bounded_triangulation_fixture(degenerate: bool) -> IncrementalSfm {
    let intrinsics = Intrinsics { fx: 500.0, fy: 500.0, cx: 256.0, cy: 256.0, skew: 0.0, distortion: Distortion::None };
    let point = [0.1, -0.1, 4.0];
    let mut keypoints = Vec::new();
    let mut track = Vec::new();
    let mut cameras = Vec::new();
    for frame in 0..MAX_INTERACTIVE_TRACK_OBSERVATIONS {
        let pose = if degenerate { CameraPose(Se3::identity()) } else { CameraPose(Se3 { r: So3::identity(), t: [frame as f64 * 0.04, 0.0, 0.0] }) };
        let pixel = reproject(&intrinsics, &pose, point).expect("track projection");
        keypoints.push(vec![Keypoint { x: pixel[0] as f32, y: pixel[1] as f32, octave: 0, angle: 0.0, response: 1.0 }]);
        track.push((frame, 0));
        cameras.push((frame, pose));
    }
    let mut sfm = IncrementalSfm::new(intrinsics, FeatureTracks { tracks: vec![track] }, keypoints, SfmConfig { min_track_length: 2, ..SfmConfig::default() });
    sfm.cameras = cameras;
    sfm
}

#[test]
fn maximum_seed_pair_and_degenerate_solve_steps_stay_below_hard_ceiling() {
    let (mut sfm, mut preparation) = bounded_seed_fixture(false);
    assert_eq!(preparation.matches.len(), MAX_INTERACTIVE_SEED_CORRESPONDENCES);
    let mut steps = 0usize;
    loop {
        let started = std::time::Instant::now();
        let complete = sfm.advance_seed_pair(&mut preparation, 1).expect("maximum admitted seed pair");
        step_ceiling::admit_step(started.elapsed(), format_args!("maximum 64-correspondence/32-hypothesis seed worker step {steps} exceeded 8 ms"));
        steps += 1;
        if complete {
            break;
        }
        assert!(steps < 1_000, "bounded seed pair failed to terminate");
    }

    let (mut malformed, mut malformed_preparation) = bounded_seed_fixture(true);
    while malformed_preparation.phase == SeedPairPhase::Collect {
        let started = std::time::Instant::now();
        let result = malformed.advance_seed_pair(&mut malformed_preparation, 1);
        step_ceiling::admit_step(started.elapsed(), format_args!("degenerate seed collection/solve worker step exceeded 8 ms"));
        if result.is_err() {
            return;
        }
    }
    let started = std::time::Instant::now();
    assert!(malformed.advance_seed_pair(&mut malformed_preparation, 1).is_err(), "coincident correspondences must be rejected");
    step_ceiling::admit_step(started.elapsed(), format_args!("degenerate maximum seed solve exceeded 8 ms"));
}

#[test]
fn maximum_registration_and_malformed_pnp_steps_stay_below_hard_ceiling_in_each_build_profile() {
    for degenerate in [false, true] {
        let (mut sfm, mut preparation) = bounded_registration_fixture(degenerate);
        let mut steps = 0usize;
        loop {
            let started = std::time::Instant::now();
            let result = sfm.advance_registration(&mut preparation, 1);
            step_ceiling::admit_step(started.elapsed(), format_args!("64-correspondence one-hypothesis registration worker step {steps} exceeded 8 ms"));
            steps += 1;
            match result {
                Ok(true) if !degenerate => break,
                Err(SfmError::PnpFailed) if degenerate => break,
                Ok(false) => {}
                outcome => panic!("unexpected bounded registration outcome: {outcome:?}"),
            }
            assert!(steps < MAX_INTERACTIVE_TRACKS + MAX_INTERACTIVE_REGISTRATION_ATTEMPTS as usize + 4, "bounded registration failed to terminate");
        }
        assert_eq!(preparation.world_points.len(), MAX_INTERACTIVE_REGISTRATION_CORRESPONDENCES);
    }
}

#[test]
fn maximum_track_triangulation_and_degenerate_geometry_steps_stay_below_hard_ceiling_in_each_build_profile() {
    for degenerate in [false, true] {
        let mut sfm = bounded_triangulation_fixture(degenerate);
        let mut preparation = BundlePreparation { point_track_ids: Vec::new(), cursor: 0, phase: BundlePhase::Retriangulate, tolerance: 3.0 };
        let started = std::time::Instant::now();
        assert!(!sfm.advance_bundle(&mut preparation, 1));
        step_ceiling::admit_step(started.elapsed(), format_args!("8-observation triangulation worker step exceeded 8 ms"));
        assert_eq!(sfm.points.contains_key(&0), !degenerate);
    }
}

#[test]
fn run_all_reconstructs_synthetic_multi_camera_scene() {
    let n_cams = 6;
    let scene = synthetic_scene(2026, n_cams, 50, false);
    let scene_obs = project_observations(&scene, 0.3, 0.0, 2026);
    let intr = scene.cameras[0].0;

    let mut by_point: Vec<Vec<(usize, [f64; 2])>> = vec![Vec::new(); scene.points_world.len()];
    for o in &scene_obs {
        by_point[o.point_index].push((o.camera_index, o.pixel));
    }
    let mut keypoints_per_frame: Vec<Vec<Keypoint>> = vec![Vec::new(); n_cams];
    let mut tracks: Vec<Vec<(usize, u32)>> = Vec::new();
    for cam_obs in &by_point {
        let mut track = Vec::new();
        for &(c, px) in cam_obs {
            let kp_idx = keypoints_per_frame[c].len() as u32;
            keypoints_per_frame[c].push(Keypoint { x: px[0] as f32, y: px[1] as f32, octave: 0, angle: 0.0, response: 1.0 });
            track.push((c, kp_idx));
        }
        if track.len() >= 2 {
            tracks.push(track);
        }
    }
    let feature_tracks = FeatureTracks { tracks };

    let frame_order: Vec<usize> = (0..n_cams).collect();
    let matches01: Vec<Match> = feature_tracks
        .tracks
        .iter()
        .filter_map(|track| {
            let a = track.iter().find(|&&(f, _)| f == 0)?;
            let b = track.iter().find(|&&(f, _)| f == 1)?;
            Some(Match { a: a.1, b: b.1, distance: 0 })
        })
        .collect();
    assert!(matches01.len() >= 8, "expected enough shared tracks between frames 0 and 1, got {}", matches01.len());
    let pairwise_matches = vec![(0usize, 1usize, matches01)];

    let cfg = SfmConfig { ransac_threshold_px: 2.5, min_track_length: 2, ba_max_iterations: 30, robust_loss: RobustLoss::Huber(2.0), min_triangulation_angle_rad: 1.0_f64.to_radians(), min_visible_points_to_keep_camera: 6 };
    let mut sfm = IncrementalSfm::new(intr, feature_tracks.clone(), keypoints_per_frame.clone(), cfg);
    let recon = sfm.run_all(&frame_order, &pairwise_matches).expect("run_all should reconstruct the synthetic scene");

    assert!(recon.cameras.len() + 1 >= n_cams, "expected almost all cameras registered, got {}", recon.cameras.len());
    assert!(recon.points.len() >= 10, "expected a reasonable number of triangulated points, got {}", recon.points.len());

    let true_centers: Vec<[f64; 3]> = recon.cameras.iter().map(|&(f, _)| camera_center(&scene.cameras[f].1)).collect();
    let recovered_centers: Vec<[f64; 3]> = recon.cameras.iter().map(|&(_, pose)| camera_center(&pose)).collect();
    let sim = umeyama(&recovered_centers, &true_centers, true).expect("recovered cameras should not be degenerate");

    let mut max_center_err = 0.0_f64;
    for (rec, truth) in recovered_centers.iter().zip(true_centers.iter()) {
        let aligned = sim.act(*rec);
        max_center_err = max_center_err.max(vec3d_length(vec3d_sub(aligned, *truth)));
    }
    assert!(max_center_err < 1.0, "aligned camera center error {max_center_err} too high (orbit radius 5)");

    let mut reproj_errs = Vec::new();
    for (i, &tid) in recon.point_track_ids.iter().enumerate() {
        let point = recon.points[i];
        for &(f, kp) in &feature_tracks.tracks[tid] {
            let Some(&(_, pose)) = recon.cameras.iter().find(|&(cf, _)| *cf == f) else { continue };
            let k = keypoints_per_frame[f][kp as usize];
            let obs = [f64::from(k.x), f64::from(k.y)];
            if let Some(pred) = reproject(&intr, &pose, point) {
                reproj_errs.push(((pred[0] - obs[0]).powi(2) + (pred[1] - obs[1]).powi(2)).sqrt());
            }
        }
    }
    assert!(!reproj_errs.is_empty(), "expected some reprojection measurements");
    let mean_reproj: f64 = reproj_errs.iter().sum::<f64>() / reproj_errs.len() as f64;
    assert!(mean_reproj < 2.0, "mean reprojection error {mean_reproj} too high");
}

/// 🌱️ `init_pair` on a low-parallax, near-planar seed pair (same depth-shallow, small-baseline
/// geometry as `five_point_recovers_pose_on_planar_scene_where_eight_point_struggles` — the classic
/// 8-point degeneracy, and representative of consecutive video frames rather than a wide-baseline
/// photoset): confirms `init_pair` -> [`estimate_init_pair_essential`] actually routes through the
/// five-point solver end to end (not just at the primitive level) and comes out with a usable pose and
/// a fully triangulated seed point cloud.
#[test]
fn init_pair_recovers_pose_and_triangulates_on_low_parallax_pair() {
    let intr = Intrinsics { fx: 800.0, fy: 800.0, cx: 320.0, cy: 240.0, skew: 0.0, distortion: Distortion::None };
    let pose_a = CameraPose(Se3::identity());
    let pose_b = CameraPose(Se3 { r: So3::exp([0.02, 0.15, -0.02]), t: [0.6, 0.02, 0.05] });
    let mut rng = Rng::from_seed(3033);
    let mut keypoints_a = Vec::new();
    let mut keypoints_b = Vec::new();
    let mut track_list: Vec<Vec<(usize, u32)>> = Vec::new();
    while track_list.len() < 200 {
        let p = [(rng.next_f64() - 0.5) * 2.0, (rng.next_f64() - 0.5) * 2.0, 5.0 + (rng.next_f64() - 0.5) * 0.5];
        let (Some(a), Some(b)) = (reproject(&intr, &pose_a, p), reproject(&intr, &pose_b, p)) else { continue };
        let a_noised = [a[0] + normal(&mut rng, 0.0, 0.05), a[1] + normal(&mut rng, 0.0, 0.05)];
        let b_noised = [b[0] + normal(&mut rng, 0.0, 0.05), b[1] + normal(&mut rng, 0.0, 0.05)];
        let kp_idx = keypoints_a.len() as u32;
        keypoints_a.push(Keypoint { x: a_noised[0] as f32, y: a_noised[1] as f32, octave: 0, angle: 0.0, response: 1.0 });
        keypoints_b.push(Keypoint { x: b_noised[0] as f32, y: b_noised[1] as f32, octave: 0, angle: 0.0, response: 1.0 });
        track_list.push(vec![(0usize, kp_idx), (1usize, kp_idx)]);
    }
    let matches: Vec<Match> = (0..track_list.len() as u32).map(|i| Match { a: i, b: i, distance: 0 }).collect();
    let tracks = FeatureTracks { tracks: track_list };
    let cfg = SfmConfig { ransac_threshold_px: 2.0, min_track_length: 2, ..SfmConfig::default() };
    let mut sfm = IncrementalSfm::new(intr, tracks, vec![keypoints_a, keypoints_b], cfg);
    sfm.init_pair(0, 1, &matches).expect("init_pair should succeed on a low-parallax pair once routed through the five-point solver");

    let recon = sfm.reconstruction();
    assert_eq!(recon.cameras.len(), 2, "init_pair should register exactly the two seed cameras");
    assert!(recon.points.len() > 100, "expected most tracks to triangulate, got {}", recon.points.len());

    let recovered_rel = relative_pose(&recon.cameras[0].1, &recon.cameras[1].1);
    let true_rel = relative_pose(&pose_a, &pose_b);
    let rot_err = rotation_error_deg(&recovered_rel.r, &true_rel.r);
    assert!(rot_err < 1.0, "init_pair rotation error {rot_err} deg exceeds 1 deg on the low-parallax pair");
}
// #endregion 🔖️IncrementalTests
