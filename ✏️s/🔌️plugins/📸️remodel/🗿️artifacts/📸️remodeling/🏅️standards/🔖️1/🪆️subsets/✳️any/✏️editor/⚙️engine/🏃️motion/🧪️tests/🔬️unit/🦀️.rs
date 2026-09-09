use super::*;
use crate::algebra::{vec3d_cross, vec3d_normalize, vec3d_sub, Mat3d};
use crate::lie::So3;
use remodeling_camera::Distortion;
use remodeling_image::{build_pyramid, scharr_gradients, warp_affine};

// #region 🔖️Fixtures
fn lcg_next(state: &mut u64) -> f64 {
    *state = state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
    (*state >> 11) as f64 / (1u64 << 53) as f64
}

fn grid_texture(size: u32) -> ImageGray {
    let mut img = ImageGray::new(size, size);
    for v in img.data.iter_mut() {
        *v = 0.1;
    }
    let (step, square) = (10u32, 5u32);
    let mut y = 3;
    while y + square <= size {
        let mut x = 3;
        while x + square <= size {
            for dy in 0..square {
                for dx in 0..square {
                    img.set(x + dx, y + dy, 0.9);
                }
            }
            x += step;
        }
        y += step;
    }
    img
}

fn look_at_pose(eye: [f64; 3], target: [f64; 3], up: [f64; 3]) -> Se3 {
    let forward = vec3d_normalize(vec3d_sub(target, eye));
    let right = vec3d_normalize(vec3d_cross(up, forward));
    let true_up = vec3d_cross(forward, right);
    let r_cw = Mat3d::from_axes(right, true_up, forward);
    let r_wc = So3(r_cw).inverse();
    let t = scale3(r_wc.act(eye), -1.0);
    Se3 { r: r_wc, t }
}

fn pinhole(fx: f64, fy: f64, cx: f64, cy: f64) -> Intrinsics {
    Intrinsics { fx, fy, cx, cy, skew: 0.0, distortion: Distortion::None }
}

fn se3_error_norm(a: &Se3, b: &Se3) -> f64 {
    let xi = a.inverse().semio_compose_rs(b).log();
    xi.iter().map(|v| v * v).sum::<f64>().sqrt()
}
// #endregion 🔖️Fixtures

// #region 🔖️Track2dTests
#[test]
fn tracker2d_maintains_tracks_and_redetects_lost_coverage() {
    let base = grid_texture(80);
    let mut pyrs = vec![build_pyramid(&base, 3)];
    for i in 1..6u32 {
        let shifted = warp_affine(&base, &[[1.0, 0.0, (i * 4) as f32], [0.0, 1.0, 0.0]], 80, 80);
        pyrs.push(build_pyramid(&shifted, 3));
    }
    let mut tracker = Tracker2d::new();
    tracker.step(&pyrs[0], &pyrs[0], 0, 4, 20, 16, 2);
    assert!(!tracker.tracks().is_empty(), "seeding step should redetect at least one track");
    for i in 1..pyrs.len() {
        tracker.step(&pyrs[i - 1], &pyrs[i], i as u32, 4, 20, 16, 2);
    }
    let tracks = tracker.tracks();
    assert!(tracks.iter().any(|t| t.samples.len() > 1), "at least one track should survive multiple frames");
    assert!(tracks.iter().any(|t| t.samples[0].0 > 0), "redetection should spawn new tracks at a later frame as content shifts out of view");
}
// #endregion 🔖️Track2dTests

// #region 🔖️MotTests
#[test]
fn multi_object_tracker_associates_spawns_and_prunes() {
    let mut tracker = MultiObjectTracker::new();
    let det_a = |t: f32| Detection { x: 10.0 + t, y: 10.0, id_hint: None };
    let det_b = |t: f32| Detection { x: 60.0 - t, y: 30.0, id_hint: None };

    let r0 = tracker.update(&[det_a(0.0), det_b(0.0)], 20.0);
    assert_eq!(r0.len(), 2);
    let id_a = r0.iter().find(|&&(_, di)| di == 0).unwrap().0;
    let id_b = r0.iter().find(|&&(_, di)| di == 1).unwrap().0;
    assert_ne!(id_a, id_b);

    for step in 1..8 {
        tracker.predict(1.0);
        let t = step as f32;
        let r = tracker.update(&[det_a(t), det_b(t)], 20.0);
        let matched_a = r.iter().find(|&&(id, _)| id == id_a);
        let matched_b = r.iter().find(|&&(id, _)| id == id_b);
        assert_eq!(matched_a.map(|&(_, di)| di), Some(0), "object A identity must persist through the crossing");
        assert_eq!(matched_b.map(|&(_, di)| di), Some(1), "object B identity must persist through the crossing");
    }

    let r_new = tracker.update(&[det_a(8.0), det_b(8.0), Detection { x: 200.0, y: 200.0, id_hint: None }], 20.0);
    let new_id = r_new.iter().find(|&&(_, di)| di == 2).unwrap().0;
    assert_ne!(new_id, id_a);
    assert_ne!(new_id, id_b);

    for _ in 0..(MOT_MAX_MISSED as usize + 2) {
        tracker.predict(1.0);
        tracker.update(&[], 20.0);
    }
    let r_after_gap = tracker.update(&[Detection { x: 200.0, y: 200.0, id_hint: None }], 5.0);
    let (revived_id, _) = r_after_gap[0];
    assert_ne!(revived_id, new_id, "a track missing for longer than MOT_MAX_MISSED must be pruned, so this detection spawns a fresh id");
}
// #endregion 🔖️MotTests

// #region 🔖️Trajectory3dTests
#[test]
fn triangulate_tracks_recovers_a_slow_moving_point_from_a_single_orbiting_camera() {
    let intr = pinhole(700.0, 700.0, 320.0, 240.0);
    let n_frames = 24u32;
    let p0 = [0.2, 0.1, 0.0];
    let v = [0.01, 0.005, 0.0];

    let mut cams_per_frame = Vec::new();
    let mut frame_timestamps = std::collections::HashMap::new();
    let mut samples = Vec::new();
    for f in 0..n_frames {
        let ts = f64::from(f);
        let angle = f64::from(f) * 0.08;
        let eye = [6.0 * angle.cos(), 6.0 * angle.sin(), 1.2];
        let pose = CameraPose(look_at_pose(eye, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]));
        let point = add3(p0, scale3(v, ts));
        let Some(px) = remodeling_camera::reproject(&intr, &pose, point) else { continue };
        cams_per_frame.push((f, pose, intr));
        frame_timestamps.insert(f, ts);
        samples.push((f, px[0] as f32, px[1] as f32));
    }
    let track = Track2d { id: 0, samples };
    let trajectories = triangulate_tracks(&cams_per_frame, std::slice::from_ref(&track), &frame_timestamps);
    assert_eq!(trajectories.len(), 1);
    let traj = &trajectories[0];
    assert!(traj.samples.len() > 10, "expected most frames to triangulate successfully");
    for &(ts, pos) in &traj.samples {
        let truth = add3(p0, scale3(v, ts));
        assert!(dist3(pos, truth) < 0.3, "triangulated position too far from ground truth: got {pos:?} want {truth:?}");
    }
}
// #endregion 🔖️Trajectory3dTests

// #region 🔖️KinematicsTests
#[test]
fn velocity_and_acceleration_recover_planted_constant_motion() {
    let p0 = [1.0, -2.0, 0.5];
    let v = [0.3, -0.1, 0.2];
    let samples: Vec<(f64, [f64; 3])> = (0..20).map(|i| (f64::from(i), add3(p0, scale3(v, f64::from(i))))).collect();
    let traj = Trajectory3d { samples };
    let vel = velocity(&traj);
    for sample in &vel[4..16] {
        assert!(dist3(*sample, v) < 1e-3, "velocity mismatch: got {sample:?} want {v:?}");
    }

    let a = [0.05, -0.02, 0.01];
    let samples: Vec<(f64, [f64; 3])> = (0..20)
        .map(|i| {
            let t = f64::from(i);
            (t, add3(add3(p0, scale3(v, t)), scale3(a, 0.5 * t * t)))
        })
        .collect();
    let traj = Trajectory3d { samples };
    let acc = acceleration(&traj);
    for sample in &acc[4..16] {
        assert!(dist3(*sample, a) < 1e-2, "acceleration mismatch: got {sample:?} want {a:?}");
    }
}

#[test]
fn neighborhood_affine_strain_distinguishes_rotation_from_stretch() {
    let mut state = 42u64;
    let cloud_t0: Vec<[f64; 3]> = (0..40).map(|_| [lcg_next(&mut state) * 4.0 - 2.0, lcg_next(&mut state) * 4.0 - 2.0, lcg_next(&mut state) * 4.0 - 2.0]).collect();

    let angle = 0.3f64;
    let (c, s) = (angle.cos(), angle.sin());
    let rotated: Vec<[f64; 3]> = cloud_t0.iter().map(|&p| [c * p[0] - s * p[1], s * p[0] + c * p[1], p[2]]).collect();
    let rot_pairs = neighborhood_affine_strain(&cloud_t0, &rotated, 2.5);
    assert!(!rot_pairs.is_empty(), "expected at least one neighbourhood with enough points");
    for pair in &rot_pairs {
        assert!(pair.strain < 1e-6, "pure rotation should report near-zero strain, got {}", pair.strain);
    }

    let (sx, sy, sz) = (1.3, 1.0, 0.9);
    let stretched: Vec<[f64; 3]> = cloud_t0.iter().map(|&p| [p[0] * sx, p[1] * sy, p[2] * sz]).collect();
    let expected = ((0.5f64 * (sx * sx - 1.0)).powi(2) + (0.5f64 * (sy * sy - 1.0)).powi(2) + (0.5f64 * (sz * sz - 1.0)).powi(2)).sqrt();
    let stretch_pairs = neighborhood_affine_strain(&cloud_t0, &stretched, 2.5);
    assert!(!stretch_pairs.is_empty());
    for pair in &stretch_pairs {
        assert!((pair.strain - expected).abs() < 1e-6, "got {} want {}", pair.strain, expected);
    }
}
// #endregion 🔖️KinematicsTests

// #region 🔖️ModalTests
#[test]
fn modal_analysis_recovers_a_planted_frequency() {
    let fps = 50.0;
    let n = 2048usize;
    let target_hz = 4.0;
    let mut tracks = Vec::new();
    for track_idx in 0..3 {
        let phase = f64::from(track_idx) * 0.4;
        let samples: Vec<(u32, f32, f32)> = (0..n)
            .map(|i| {
                let t = i as f64 / fps;
                let y = 5.0 * (2.0 * std::f64::consts::PI * target_hz * t + phase).sin();
                (i as u32, 0.0, y as f32)
            })
            .collect();
        tracks.push(Track2d { id: track_idx as u32, samples });
    }
    let result = modal_analysis(&tracks, fps, 0, 2);
    assert!(!result.frequencies_hz.is_empty(), "expected at least one detected mode");
    let closest = result.frequencies_hz.iter().min_by(|a, b| (**a - target_hz).abs().total_cmp(&(**b - target_hz).abs())).unwrap();
    assert!((closest - target_hz).abs() < 0.2, "closest detected frequency {closest} too far from planted {target_hz}");
    assert_eq!(result.mode_shapes[0].len(), tracks.len());
}
// #endregion 🔖️ModalTests

// #region 🔖️SyncTests
#[test]
fn estimate_offset_and_refine_subframe_recover_a_fractional_shift() {
    let f = |t: f64| (2.0 * std::f64::consts::PI * t / 37.0).sin() + 0.5 * (2.0 * std::f64::consts::PI * t / 11.0).sin();
    let n = 220usize;
    let signal_a: Vec<f64> = (0..n).map(|i| f(i as f64)).collect();
    let true_offset = 3.6;
    let signal_b: Vec<f64> = (0..n).map(|i| f(i as f64 - true_offset)).collect();

    let coarse = estimate_offset(&signal_a, &signal_b, 30.0);
    assert!(coarse.confidence > 0.5, "expected a strong correlation peak, got confidence {}", coarse.confidence);
    let refined = refine_subframe(&signal_a, &signal_b, coarse.offset_frames);
    assert!((refined - true_offset).abs() < 0.1, "refined offset {refined} too far from planted {true_offset}");
}
// #endregion 🔖️SyncTests

// #region 🔖️RollingShutterCompTests
#[test]
fn rolling_shutter_velocity_and_rectify_maps_are_sane() {
    let mut tracks = Vec::new();
    for (idx, &row) in [10.0f32, 30.0, 50.0, 70.0].iter().enumerate() {
        let base_vx = 2.0f32;
        let k = 0.05f32;
        let dx = base_vx + k * row;
        tracks.push(Track2d { id: idx as u32, samples: vec![(0, 100.0, row), (1, 100.0 + dx, row)] });
    }
    let model = estimate_rs_velocity(&tracks, 100);
    assert!((model[2] - 0.05).abs() < 0.01, "expected kx close to planted 0.05, got {}", model[2]);
    assert!(model[3].abs() < 0.01, "expected ky close to 0");

    let (map_x, _map_y) = build_rs_rectify_maps(model, 20, 100, 1e-5);
    assert_eq!(map_x.len(), 20 * 100);
    let idx = 90usize * 20 + 5;
    assert!((map_x[idx] - 5.0).abs() > 0.5, "expected a nonzero rectification shift far from the reference row");

    let mut flat_tracks = Vec::new();
    for (idx, &row) in [10.0f32, 30.0, 50.0, 70.0].iter().enumerate() {
        flat_tracks.push(Track2d { id: idx as u32, samples: vec![(0, 100.0, row), (1, 103.0, row)] });
    }
    let flat_model = estimate_rs_velocity(&flat_tracks, 100);
    let (flat_map_x, flat_map_y) = build_rs_rectify_maps(flat_model, 20, 100, 1e-5);
    for row in 0..100usize {
        for col in 0..20usize {
            let i = row * 20 + col;
            assert!((flat_map_x[i] - col as f32).abs() < 0.05, "pure translation should yield near-zero x rectification");
            assert!((flat_map_y[i] - row as f32).abs() < 0.05, "pure translation should yield near-zero y rectification");
        }
    }
}
// #endregion 🔖️RollingShutterCompTests

// #region 🔖️StabilizeTests
#[test]
fn smooth_camera_path_reduces_planted_jitter() {
    let true_xi = [0.02, 0.0, 0.0, 0.0, 0.0, 0.03];
    let n = 81;
    let mut true_poses = Vec::with_capacity(n);
    true_poses.push(Se3::identity());
    for i in 1..n {
        true_poses.push(Se3::exp(true_xi).semio_compose_rs(&true_poses[i - 1]));
    }
    let mut state = 7u64;
    let jittered: Vec<Se3> = true_poses
        .iter()
        .map(|p| {
            let jitter: [f64; 6] = std::array::from_fn(|_| (lcg_next(&mut state) - 0.5) * 0.01);
            Se3::exp(jitter).semio_compose_rs(p)
        })
        .collect();
    let smoothed = smooth_camera_path(&jittered, 15);

    let orig_err: f64 = jittered.iter().zip(true_poses.iter()).map(|(a, b)| se3_error_norm(a, b)).sum();
    let smooth_err: f64 = smoothed.iter().zip(true_poses.iter()).map(|(a, b)| se3_error_norm(a, b)).sum();
    assert!(smooth_err < orig_err * 0.85, "smoothing should reduce total pose error: orig {orig_err} smoothed {smooth_err}");
}

#[test]
fn stabilization_warps_are_near_identity_for_matching_poses() {
    let intr = pinhole(500.0, 500.0, 160.0, 120.0);
    let pose = Se3::exp([0.0, 0.0, 0.0, 0.1, 0.0, 0.0]);
    let warps = stabilization_warps(&[pose], &[pose], &intr);
    let h = warps[0];
    for (r, row) in h.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            let expect = if r == c { 1.0 } else { 0.0 };
            assert!((v - expect).abs() < 1e-6, "identity delta rotation should yield an identity homography, got {h:?}");
        }
    }
}
// #endregion 🔖️StabilizeTests

// #region 🔖️DeblurTests
#[test]
fn wiener_deconvolve_measurably_sharpens_a_blurred_image() {
    let size = 48u32;
    let mut sharp = ImageGray::new(size, size);
    let mut state = 99u64;
    for y in 0..size {
        for x in 0..size {
            let block = (x / 6 + y / 6) % 2;
            let noise = (lcg_next(&mut state) - 0.5) * 0.02;
            sharp.set(x, y, (block as f32 * 0.8 + 0.1 + noise as f32).clamp(0.0, 1.0));
        }
    }
    let psf = estimate_motion_psf((6.0, 0.0), 1.0);
    let mut blurred = ImageGray::new(size, size);
    let half = i64::from(psf.width / 2);
    for y in 0..size {
        for x in 0..size {
            let mut acc = 0.0f32;
            for ky in 0..psf.height {
                for kx in 0..psf.width {
                    let sx = i64::from(x) + i64::from(kx) - half;
                    let sy = i64::from(y) + i64::from(ky) - half;
                    if sx >= 0 && sy >= 0 && (sx as u32) < size && (sy as u32) < size {
                        acc += sharp.get(sx as u32, sy as u32) * psf.kernel[(ky * psf.width + kx) as usize];
                    }
                }
            }
            blurred.set(x, y, acc);
        }
    }
    let gradient_energy = |img: &ImageGray| -> f32 {
        let g = scharr_gradients(img);
        g.gx.iter().zip(g.gy.iter()).map(|(&gx, &gy)| gx * gx + gy * gy).sum()
    };
    let energy_sharp = gradient_energy(&sharp);
    let energy_blurred = gradient_energy(&blurred);
    assert!(energy_blurred < energy_sharp, "blur should reduce gradient energy");

    let deconvolved = wiener_deconvolve(&blurred, &psf, 200.0);
    let energy_deconv = gradient_energy(&deconvolved);
    assert!(energy_deconv > energy_blurred, "deconvolution should measurably sharpen the blurred image: blurred {energy_blurred} deconv {energy_deconv}");
}
// #endregion 🔖️DeblurTests

// #region 🔖️NonRigidTests
#[test]
fn build_and_fit_deformation_graph_recovers_a_planted_bend_for_held_out_points() {
    let radius = 3.0;
    let mut seed_points = Vec::new();
    for j in 0..4 {
        for i in 0..4 {
            seed_points.push([f64::from(i) * 2.0 - 3.0, f64::from(j) * 2.0 - 3.0, 0.0]);
        }
    }
    let graph = build_deformation_graph(&seed_points, radius);

    let mut src_points = Vec::new();
    for j in 0..9 {
        for i in 0..9 {
            src_points.push([f64::from(i) - 4.0, f64::from(j) - 4.0, 0.0]);
        }
    }
    let bend_radius = 8.0;
    let bend = |p: [f64; 3]| -> [f64; 3] {
        let theta = p[0] / bend_radius;
        [bend_radius * theta.sin(), p[1], bend_radius * (1.0 - theta.cos())]
    };

    let mut dst_correspondences = Vec::new();
    let mut held_out = Vec::new();
    for (idx, &p) in src_points.iter().enumerate() {
        if idx % 5 == 0 {
            held_out.push(idx);
        } else {
            dst_correspondences.push((idx, bend(p)));
        }
    }

    let fitted = fit_deformation(&graph, &src_points, &dst_correspondences, radius);
    let mut max_err = 0.0f64;
    for &idx in &held_out {
        let predicted = deform_point(&fitted, src_points[idx], radius);
        let truth = bend(src_points[idx]);
        max_err = max_err.max(dist3(predicted, truth));
    }
    assert!(max_err < 0.6, "held-out bend recovery error too large: {max_err}");
}
// #endregion 🔖️NonRigidTests

// #region 🔖️Pose6dTests
#[test]
fn track_rigid_body_recovers_a_known_rotating_translating_cube_with_partial_visibility() {
    let intr = pinhole(700.0, 700.0, 320.0, 240.0);
    let model_points: Vec<[f64; 3]> = (0..8).map(|i| [if i & 1 == 0 { -0.3 } else { 0.3 }, if i & 2 == 0 { -0.3 } else { 0.3 }, if i & 4 == 0 { -0.3 } else { 0.3 }]).collect();

    let n_frames = 16u32;
    let mut per_frame_obs = Vec::new();
    let mut truth = Vec::new();
    for f in 0..n_frames {
        let t = f64::from(f);
        let angle = t * 0.1;
        let rot = So3::exp([0.0, angle, 0.0]);
        let translation = [0.0, 0.0, 5.0 + 0.05 * t];
        let pose = Se3 { r: rot, t: translation };
        let hidden = f as usize % 8;
        let obs: Vec<Option<[f64; 2]>> = model_points
            .iter()
            .enumerate()
            .map(|(i, &mp)| {
                if i == hidden {
                    return None;
                }
                let cam_pt = pose.act(mp);
                intr.project(cam_pt)
            })
            .collect();
        per_frame_obs.push((f, obs));
        truth.push((f, pose));
    }

    let recovered = track_rigid_body(&model_points, &per_frame_obs, &intr);
    assert!(recovered.len() > n_frames as usize / 2, "expected most frames to recover a pose");
    for &(frame, pose) in &recovered {
        let (_, true_pose) = truth.iter().find(|&&(f, _)| f == frame).unwrap();
        let err = se3_error_norm(&pose, true_pose);
        assert!(err < 0.1, "frame {frame}: pose error {err} too large");
    }
}
// #endregion 🔖️Pose6dTests
