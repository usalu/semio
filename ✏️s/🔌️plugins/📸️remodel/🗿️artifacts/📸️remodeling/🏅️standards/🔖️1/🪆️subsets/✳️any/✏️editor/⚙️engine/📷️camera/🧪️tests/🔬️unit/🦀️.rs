use super::*;

fn lcg(state: &mut u64) -> f64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    ((*state >> 11) as f64 / (1_u64 << 53) as f64) * 2.0 - 1.0
}

fn gaussian(state: &mut u64, sigma: f64) -> f64 {
    let u1 = (0.5 * (lcg(state) + 1.0)).max(1e-12);
    let u2 = 0.5 * (lcg(state) + 1.0);
    (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos() * sigma
}

// #region 🔖️DistortionTests
#[test]
fn project_unproject_round_trips_for_none_distortion() {
    let intr = Intrinsics { fx: 600.0, fy: 610.0, cx: 320.0, cy: 240.0, skew: 0.5, distortion: Distortion::None };
    for ix in -3..=3 {
        for iy in -3..=3 {
            let p_cam = [ix as f64 * 0.1, iy as f64 * 0.1, 1.0];
            let px = intr.project(p_cam).expect("in front of camera");
            let ray = intr.unproject_ray(px);
            assert!((ray[0] - p_cam[0]).abs() < 1e-9, "x mismatch at {ix},{iy}: {} vs {}", ray[0], p_cam[0]);
            assert!((ray[1] - p_cam[1]).abs() < 1e-9, "y mismatch at {ix},{iy}: {} vs {}", ray[1], p_cam[1]);
        }
    }
}

#[test]
fn undistort_distort_round_trips_for_brown_conrady_and_fisheye() {
    let models = [Distortion::BrownConrady { k1: -0.15, k2: 0.03, k3: -0.002, p1: 0.001, p2: -0.0015 }, Distortion::FisheyeEquidistant { k1: -0.05, k2: 0.01, k3: -0.002, k4: 0.0005 }];
    for distortion in models {
        let intr = Intrinsics { fx: 700.0, fy: 690.0, cx: 330.0, cy: 250.0, skew: 0.0, distortion };
        for ix in -4..=4 {
            for iy in -4..=4 {
                let p_norm = [ix as f64 * 0.06, iy as f64 * 0.06];
                let distorted = distortion.distort(p_norm);
                let recovered = intr.undistort_point(distorted);
                assert!((recovered[0] - p_norm[0]).abs() < 1e-6, "x mismatch at {ix},{iy}: {} vs {}", recovered[0], p_norm[0]);
                assert!((recovered[1] - p_norm[1]).abs() < 1e-6, "y mismatch at {ix},{iy}: {} vs {}", recovered[1], p_norm[1]);
            }
        }
    }
}

#[test]
fn project_unproject_round_trips_through_moderate_distortion() {
    let intr = Intrinsics { fx: 650.0, fy: 655.0, cx: 315.0, cy: 245.0, skew: 0.0, distortion: Distortion::BrownConrady { k1: -0.1, k2: 0.02, k3: 0.0, p1: 0.0005, p2: -0.0004 } };
    for ix in -3..=3 {
        for iy in -3..=3 {
            let p_cam = [ix as f64 * 0.08, iy as f64 * 0.08, 1.0];
            let px = intr.project(p_cam).expect("in front of camera");
            let ray = intr.unproject_ray(px);
            assert!((ray[0] - p_cam[0]).abs() < 1e-6, "x mismatch at {ix},{iy}");
            assert!((ray[1] - p_cam[1]).abs() < 1e-6, "y mismatch at {ix},{iy}");
        }
    }
}
// #endregion 🔖️DistortionTests

// #region 🔖️ReprojectionTests
#[test]
fn reprojection_problem_residuals_vanish_at_ground_truth_and_lm_recovers_from_perturbation() {
    let intr = Intrinsics { fx: 500.0, fy: 500.0, cx: 250.0, cy: 200.0, skew: 0.0, distortion: Distortion::None };
    let num_cameras = 3;
    let num_points = 8;
    let mut state = 42_u64;
    let true_poses: Vec<Se3> = (0..num_cameras).map(|i| Se3 { r: So3::exp([0.1 * i as f64, -0.05 * i as f64, 0.05 * i as f64]), t: [0.2 * i as f64, -0.1 * i as f64, 0.0] }).collect();
    let true_points: Vec<[f64; 3]> = (0..num_points).map(|_| [lcg(&mut state) * 0.5, lcg(&mut state) * 0.5, 2.0 + lcg(&mut state) * 0.3]).collect();

    let mut observations = Vec::new();
    for (cam_idx, pose) in true_poses.iter().enumerate() {
        for (point_idx, point) in true_points.iter().enumerate() {
            if let Some(px) = reproject(&intr, &CameraPose(*pose), *point) {
                observations.push((cam_idx, point_idx, px));
            }
        }
    }
    let problem = ReprojectionProblem { observations, num_cameras, num_points, intrinsics: intr };

    let mut x_truth = VecD::zeros(problem.parameter_count());
    for (i, pose) in true_poses.iter().enumerate() {
        let log = pose.log();
        for (k, v) in log.into_iter().enumerate() {
            x_truth.set(i * 6 + k, v);
        }
    }
    for (j, point) in true_points.iter().enumerate() {
        let base = num_cameras * 6 + j * 3;
        x_truth.set(base, point[0]);
        x_truth.set(base + 1, point[1]);
        x_truth.set(base + 2, point[2]);
    }
    let mut residuals_at_truth = VecD::zeros(problem.residual_count());
    problem.residuals(&x_truth, &mut residuals_at_truth);
    assert!(residuals_at_truth.norm_inf() < 1e-9, "residuals at ground truth: {}", residuals_at_truth.norm_inf());

    let mut x0 = VecD::zeros(problem.parameter_count());
    for i in 0..x0.len() {
        x0.set(i, x_truth.get(i) + 0.02 * lcg(&mut state));
    }
    let cfg = LmConfig { max_iters: 100, ..LmConfig::default() };
    let result = levenberg_marquardt(&problem, x0, &cfg);
    let mut final_residuals = VecD::zeros(problem.residual_count());
    problem.residuals(&result.x, &mut final_residuals);
    let rmse = (final_residuals.dot(&final_residuals) / final_residuals.len() as f64).sqrt();
    assert!(rmse < 1e-4, "reprojection RMSE after LM: {rmse}");
}
// #endregion 🔖️ReprojectionTests

// #region 🔖️PlanarCalibrationTests
#[test]
fn calibrate_planar_recovers_intrinsics_from_synthetic_checkerboard() {
    let true_intr = Intrinsics { fx: 800.0, fy: 810.0, cx: 322.0, cy: 238.0, skew: 0.0, distortion: Distortion::BrownConrady { k1: -0.12, k2: 0.02, k3: 0.0, p1: 0.0005, p2: -0.0003 } };
    let (nx, ny, square) = (7, 5, 0.03);
    let board_points: Vec<[f64; 2]> = (0..ny).flat_map(|iy| (0..nx).map(move |ix| [(ix as f64 - (nx - 1) as f64 / 2.0) * square, (iy as f64 - (ny - 1) as f64 / 2.0) * square])).collect();

    let mut state = 20260719_u64;
    let num_views = 15;
    let mut views: Vec<Vec<([f64; 2], [f64; 2])>> = Vec::with_capacity(num_views);
    for i in 0..num_views {
        let j1 = 0.2 * lcg(&mut state);
        let j2 = 0.2 * lcg(&mut state);
        let j3 = 0.15 * lcg(&mut state);
        let rot = match i % 3 {
            0 => [0.55 + j1, j2, j3],
            1 => [j1, 0.55 + j2, j3],
            _ => [0.4 + j1, 0.4 + j2, j3],
        };
        let t = [0.25 * lcg(&mut state), 0.25 * lcg(&mut state), 0.7 + 0.15 * lcg(&mut state)];
        let pose = Se3 { r: So3::exp(rot), t };
        let mut view = Vec::with_capacity(board_points.len());
        for &board_xy in &board_points {
            let point_world = [board_xy[0], board_xy[1], 0.0];
            let px = reproject(&true_intr, &CameraPose(pose), point_world).expect("board point in front of camera");
            let noisy = [px[0] + gaussian(&mut state, 0.2), px[1] + gaussian(&mut state, 0.2)];
            view.push((board_xy, noisy));
        }
        views.push(view);
    }

    let result = calibrate_planar(&views, 640, 480).expect("well-posed synthetic calibration");
    let fx_err = (result.intrinsics.fx - true_intr.fx).abs() / true_intr.fx;
    let fy_err = (result.intrinsics.fy - true_intr.fy).abs() / true_intr.fy;
    assert!(fx_err < 0.005, "fx relative error {fx_err}, recovered {}", result.intrinsics.fx);
    assert!(fy_err < 0.005, "fy relative error {fy_err}, recovered {}", result.intrinsics.fy);
    if let Distortion::BrownConrady { k1, .. } = result.intrinsics.distortion {
        assert!((k1 - (-0.12)).abs() < 0.03, "k1 = {k1}");
    } else {
        panic!("expected BrownConrady distortion");
    }
    assert!(result.rms_px < 1.0, "rms_px = {}", result.rms_px);
    assert_eq!(result.poses.len(), num_views);
}

#[test]
fn calibrate_planar_rejects_too_few_views() {
    let view = vec![([0.0, 0.0], [100.0, 100.0]), ([0.1, 0.0], [150.0, 100.0]), ([0.0, 0.1], [100.0, 150.0]), ([0.1, 0.1], [150.0, 150.0])];
    let views = vec![view.clone(), view];
    assert!(matches!(calibrate_planar(&views, 640, 480), Err(CameraError::TooFewViews)));
}
// #endregion 🔖️PlanarCalibrationTests

// #region 🔖️RollingShutterTests
#[test]
fn pose_at_row_matches_pose0_at_first_row_and_grows_monotonically() {
    let model = RollingShutterModel { line_delay_s: 1e-5, readout: ReadoutDirection::TopToBottom };
    let pose0 = CameraPose(Se3::exp([0.1, -0.05, 0.2, 0.05, 0.02, -0.03]));
    let velocity = [0.5, 0.2, -0.1, 0.05, -0.02, 0.03];
    let at_first = pose_at_row(&model, 0, 480, &pose0, velocity);
    assert!((at_first.0.log().iter().zip(pose0.0.log().iter()).map(|(a, b)| (a - b).abs()).fold(0.0_f64, f64::max)) < 1e-9);

    let mut last_translation_norm = 0.0;
    for row in [0_u32, 100, 250, 400, 479] {
        let pose = pose_at_row(&model, row, 480, &pose0, velocity);
        let delta = pose.0.semio_compose_rs(&pose0.0.inverse());
        let norm = vec3d_length(delta.t);
        assert!(norm >= last_translation_norm - 1e-12, "translation magnitude should grow monotonically with row");
        last_translation_norm = norm;
    }
}

#[test]
fn pose_at_row_bottom_to_top_reverses_direction() {
    let model_top = RollingShutterModel { line_delay_s: 1e-5, readout: ReadoutDirection::TopToBottom };
    let model_bottom = RollingShutterModel { line_delay_s: 1e-5, readout: ReadoutDirection::BottomToTop };
    let pose0 = CameraPose(Se3::identity());
    let velocity = [0.3, 0.1, 0.0, 0.02, 0.0, 0.0];
    let top_last = pose_at_row(&model_top, 479, 480, &pose0, velocity);
    let bottom_first = pose_at_row(&model_bottom, 0, 480, &pose0, velocity);
    let diff = top_last.0.log().iter().zip(bottom_first.0.log().iter()).map(|(a, b)| (a - b).abs()).fold(0.0_f64, f64::max);
    assert!(diff < 1e-9, "top-to-bottom last row should match bottom-to-top first row");
}

#[test]
fn rectify_remap_field_is_near_identity_at_zero_velocity() {
    let intr = Intrinsics { fx: 400.0, fy: 400.0, cx: 100.0, cy: 75.0, skew: 0.0, distortion: Distortion::None };
    let model = RollingShutterModel { line_delay_s: 1e-5, readout: ReadoutDirection::TopToBottom };
    let pose0 = CameraPose(Se3::exp([0.1, 0.0, 0.0, 0.0, 0.05, 0.0]));
    let (width, height) = (20, 15);
    let field = rectify_remap_field(&intr, &model, &pose0, [0.0; 6], width, height);
    assert_eq!(field.len(), (width * height) as usize);
    for row in 0..height {
        for col in 0..width {
            let [sx, sy] = field[(row * width + col) as usize];
            assert!((sx - (col as f32 + 0.5)).abs() < 1e-3, "col {col} row {row}: sx = {sx}");
            assert!((sy - (row as f32 + 0.5)).abs() < 1e-3, "col {col} row {row}: sy = {sy}");
        }
    }
}
// #endregion 🔖️RollingShutterTests

// #region 🔖️RigTests
#[test]
fn rig_project_matches_manual_composition() {
    let intr = Intrinsics { fx: 400.0, fy: 400.0, cx: 200.0, cy: 150.0, skew: 0.0, distortion: Distortion::None };
    let camera_from_rig = Se3::exp([0.1, 0.0, 0.0, 0.0, 0.2, 0.0]);
    let rig = CameraRig { cameras: vec![RigExtrinsic { camera_id: "cam7".to_string(), pose_in_rig: camera_from_rig }] };
    let rig_pose = CameraPose(Se3::exp([0.0, 0.3, 0.0, 0.1, 0.0, 0.0]));
    let point_world = [0.3, -0.1, 3.0];
    let expected = reproject(&intr, &CameraPose(camera_from_rig.semio_compose_rs(&rig_pose.0)), point_world);
    let actual = rig_project(&rig, &[intr], "cam7", &rig_pose, point_world);
    assert_eq!(actual, expected);
    assert!(rig_project(&rig, &[intr], "unknown", &rig_pose, point_world).is_none());
}

#[test]
fn refine_rig_recovers_planted_extrinsics_and_instance_poses_from_perturbation() {
    let intr = Intrinsics { fx: 500.0, fy: 505.0, cx: 250.0, cy: 200.0, skew: 0.0, distortion: Distortion::None };
    let intrinsics = [intr, intr, intr];
    let true_cameras = [
        RigExtrinsic { camera_id: "cam0".to_string(), pose_in_rig: Se3::identity() },
        RigExtrinsic { camera_id: "cam1".to_string(), pose_in_rig: Se3::exp([0.3, 0.0, 0.0, 0.0, 0.4, 0.0]) },
        RigExtrinsic { camera_id: "cam2".to_string(), pose_in_rig: Se3::exp([-0.3, 0.0, 0.0, 0.0, -0.4, 0.0]) },
    ];
    let mut state = 555_u64;
    let num_instances = 5;
    let true_rig_poses: Vec<Se3> = (0..num_instances).map(|i| Se3 { r: So3::exp([0.1 * i as f64, -0.05 * i as f64, 0.05 * i as f64]), t: [0.15 * i as f64, -0.1 * i as f64, 0.05 * i as f64] }).collect();
    let true_points: Vec<[f64; 3]> = (0..25).map(|_| [0.6 * lcg(&mut state), 0.6 * lcg(&mut state), 3.0 + 0.4 * lcg(&mut state)]).collect();

    let mut observations = Vec::new();
    for (instance_idx, rig_pose) in true_rig_poses.iter().enumerate() {
        for (camera_idx, cam) in true_cameras.iter().enumerate() {
            let world_to_camera = rig_pose_of_camera(rig_pose, &cam.pose_in_rig);
            for &point_world in &true_points {
                if let Some(pixel) = reproject(&intrinsics[camera_idx], &CameraPose(world_to_camera), point_world) {
                    observations.push(RigObservation { instance_idx, camera_idx, point_world, pixel });
                }
            }
        }
    }

    let initial_cameras = CameraRig {
        cameras: true_cameras
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    // camera 0 is the rig-frame anchor: refine_rig never touches it, so it must start
                    // at its true value (as it would in practice — the rig frame is *defined* by it).
                    return c.clone();
                }
                let mut log = c.pose_in_rig.log();
                for v in log.iter_mut() {
                    *v += 0.01 * lcg(&mut state);
                }
                RigExtrinsic { camera_id: c.camera_id.clone(), pose_in_rig: Se3::exp(log) }
            })
            .collect(),
    };
    let initial_rig_poses: Vec<Se3> = true_rig_poses
        .iter()
        .map(|p| {
            let mut log = p.log();
            for v in log.iter_mut() {
                *v += 0.01 * lcg(&mut state);
            }
            Se3::exp(log)
        })
        .collect();

    let result = refine_rig(&intrinsics, &initial_cameras, &initial_rig_poses, &observations).expect("well-posed synthetic rig refinement");
    assert!(result.rms_px < 1e-3, "rms_px = {}", result.rms_px);
    for (recovered, truth) in result.cameras.iter().zip(true_cameras.iter()) {
        let diff = recovered.pose_in_rig.log().iter().zip(truth.pose_in_rig.log().iter()).map(|(a, b)| (a - b).abs()).fold(0.0_f64, f64::max);
        assert!(diff < 1e-3, "camera {} pose_in_rig diff {diff}", recovered.camera_id);
    }
    for (recovered, truth) in result.rig_poses.iter().zip(true_rig_poses.iter()) {
        let diff = recovered.log().iter().zip(truth.log().iter()).map(|(a, b)| (a - b).abs()).fold(0.0_f64, f64::max);
        assert!(diff < 1e-3, "rig pose diff {diff}");
    }
}
// #endregion 🔖️RigTests

// #region 🔖️SelfCalibrationTests
fn k_inverse(f: f64, px: f64, py: f64) -> [[f64; 3]; 3] {
    [[1.0 / f, 0.0, -px / f], [0.0, 1.0 / f, -py / f], [0.0, 0.0, 1.0]]
}

fn matd_from_3x3(m: &[[f64; 3]; 3]) -> MatD {
    let mut a = MatD::zeros(3, 3);
    for (r, row) in m.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            a.set(r, c, v);
        }
    }
    a
}

#[test]
fn bougnoux_focal_from_fundamental_recovers_distinct_ground_truth_focals() {
    let f1_true = 850.0;
    let f2_true = 1200.0;
    let pp1 = [310.0, 245.0];
    let pp2 = [300.0, 260.0];
    let r = mat3d_rowmajor(&So3::exp([0.05, -0.12, 0.08]).0);
    let t = [1.0, 0.2, -0.3];
    let e = mat3_mul(&skew3(t), &r);
    let f = mat3_mul(&mat3_mul(&transpose3(&k_inverse(f2_true, pp2[0], pp2[1])), &e), &k_inverse(f1_true, pp1[0], pp1[1]));
    let f_matd = matd_from_3x3(&f);

    let (rf1, rf2) = bougnoux_focal_from_fundamental(&f_matd, pp1, pp2).expect("well-posed synthetic stereo pair");
    assert!((rf1 - f1_true).abs() / f1_true < 0.01, "f1 = {rf1}, expected {f1_true}");
    assert!((rf2 - f2_true).abs() / f2_true < 0.01, "f2 = {rf2}, expected {f2_true}");

    // scale invariance: an arbitrary nonzero rescale of F must give the same answer.
    let mut f_scaled = MatD::zeros(3, 3);
    for r_ in 0..3 {
        for c_ in 0..3 {
            f_scaled.set(r_, c_, f_matd.get(r_, c_) * 17.0);
        }
    }
    let (rf1_scaled, rf2_scaled) = bougnoux_focal_from_fundamental(&f_scaled, pp1, pp2).expect("scaled F still well-posed");
    assert!((rf1_scaled - rf1).abs() < 1e-6, "f1 should be scale-invariant: {rf1_scaled} vs {rf1}");
    assert!((rf2_scaled - rf2).abs() < 1e-6, "f2 should be scale-invariant: {rf2_scaled} vs {rf2}");
}

#[test]
fn aggregate_self_calibration_recovers_median_and_rejects_outliers() {
    let candidates = [(800.0, 1000.0), (820.0, 990.0), (5000.0, 40.0), (810.0, 1010.0), (790.0, 1005.0)];
    let (f1, f2) = aggregate_self_calibration(&candidates);
    assert!((f1 - 810.0).abs() < 1e-9, "f1 median = {f1}");
    assert!((f2 - 1000.0).abs() < 1e-9, "f2 median = {f2}");
    assert_eq!(aggregate_self_calibration(&[]), (0.0, 0.0));
}

#[test]
fn bougnoux_focal_from_fundamental_rejects_wrong_shaped_matrix() {
    let mut f_wrong = MatD::zeros(2, 3);
    f_wrong.set(0, 0, 1.0);
    assert!(bougnoux_focal_from_fundamental(&f_wrong, [0.0, 0.0], [0.0, 0.0]).is_none());
}
// #endregion 🔖️SelfCalibrationTests

// #region 🔖️ReprojectionJacobianTests
fn perturb_intrinsics(intr: &Intrinsics, k: usize, delta: f64) -> Intrinsics {
    let mut out = *intr;
    match k {
        0 => out.fx += delta,
        1 => out.fy += delta,
        2 => out.cx += delta,
        3 => out.cy += delta,
        4 => out.skew += delta,
        _ => {
            let di = k - 5;
            out.distortion = match out.distortion {
                Distortion::None => Distortion::None,
                Distortion::BrownConrady { mut k1, mut k2, mut k3, mut p1, mut p2 } => {
                    match di {
                        0 => k1 += delta,
                        1 => k2 += delta,
                        2 => k3 += delta,
                        3 => p1 += delta,
                        4 => p2 += delta,
                        _ => {}
                    }
                    Distortion::BrownConrady { k1, k2, k3, p1, p2 }
                }
                Distortion::FisheyeEquidistant { mut k1, mut k2, mut k3, mut k4 } => {
                    match di {
                        0 => k1 += delta,
                        1 => k2 += delta,
                        2 => k3 += delta,
                        3 => k4 += delta,
                        _ => {}
                    }
                    Distortion::FisheyeEquidistant { k1, k2, k3, k4 }
                }
            };
        }
    }
    out
}

#[test]
fn reprojection_jacobians_match_central_difference_at_random_configurations() {
    let mut state = 909_u64;
    let models = [Distortion::None, Distortion::BrownConrady { k1: -0.12, k2: 0.02, k3: 0.001, p1: 0.0008, p2: -0.0005 }, Distortion::FisheyeEquidistant { k1: -0.04, k2: 0.008, k3: -0.001, k4: 0.0002 }];
    let eps = 1e-6;
    for distortion in models {
        for _ in 0..5 {
            let intr = Intrinsics { fx: 600.0 + 20.0 * lcg(&mut state), fy: 605.0 + 20.0 * lcg(&mut state), cx: 320.0 + 5.0 * lcg(&mut state), cy: 240.0 + 5.0 * lcg(&mut state), skew: 0.3 * lcg(&mut state), distortion };
            let pose = CameraPose(Se3::exp([0.3 * lcg(&mut state), 0.2 * lcg(&mut state), 0.1 * lcg(&mut state), 0.4 * lcg(&mut state), -0.3 * lcg(&mut state), 0.2 * lcg(&mut state)]));
            let point = [0.3 * lcg(&mut state), 0.2 * lcg(&mut state), 2.0 + 0.3 * lcg(&mut state)];
            let obs = reproject(&intr, &pose, point).expect("synthetic point stays in front of camera");

            let (j_pose, j_point, j_intr) = reprojection_jacobians(&intr, &pose, point, obs);

            for k in 0..6 {
                let mut dxi = [0.0; 6];
                dxi[k] = eps;
                let pose_plus = CameraPose(Se3::exp(dxi).semio_compose_rs(&pose.0));
                dxi[k] = -eps;
                let pose_minus = CameraPose(Se3::exp(dxi).semio_compose_rs(&pose.0));
                let r_plus = reprojection_residual(&intr, &pose_plus, point, obs);
                let r_minus = reprojection_residual(&intr, &pose_minus, point, obs);
                for row in 0..2 {
                    let numeric = (r_plus[row] - r_minus[row]) / (2.0 * eps);
                    assert!((numeric - j_pose.get(row, k)).abs() < 1e-4, "pose col {k} row {row}: numeric {numeric} vs analytic {}", j_pose.get(row, k));
                }
            }

            for k in 0..3 {
                let mut p_plus = point;
                p_plus[k] += eps;
                let mut p_minus = point;
                p_minus[k] -= eps;
                let r_plus = reprojection_residual(&intr, &pose, p_plus, obs);
                let r_minus = reprojection_residual(&intr, &pose, p_minus, obs);
                for row in 0..2 {
                    let numeric = (r_plus[row] - r_minus[row]) / (2.0 * eps);
                    assert!((numeric - j_point.get(row, k)).abs() < 1e-4, "point col {k} row {row}: numeric {numeric} vs analytic {}", j_point.get(row, k));
                }
            }

            let k_count = 5 + distortion.param_count();
            for k in 0..k_count {
                let intr_plus = perturb_intrinsics(&intr, k, eps);
                let intr_minus = perturb_intrinsics(&intr, k, -eps);
                let r_plus = reprojection_residual(&intr_plus, &pose, point, obs);
                let r_minus = reprojection_residual(&intr_minus, &pose, point, obs);
                for row in 0..2 {
                    let numeric = (r_plus[row] - r_minus[row]) / (2.0 * eps);
                    assert!((numeric - j_intr.get(row, k)).abs() < 1e-4, "intrinsics col {k} row {row}: numeric {numeric} vs analytic {}", j_intr.get(row, k));
                }
            }
        }
    }
}
// #endregion 🔖️ReprojectionJacobianTests
