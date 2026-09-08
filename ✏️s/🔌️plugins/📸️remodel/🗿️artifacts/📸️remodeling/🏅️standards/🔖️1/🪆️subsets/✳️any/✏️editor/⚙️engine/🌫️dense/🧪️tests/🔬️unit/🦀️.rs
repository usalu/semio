
use super::*;

fn lcg_next(state: &mut u64) -> f64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    (*state >> 11) as f64 / (1u64 << 53) as f64
}

fn intrinsics_for(width: u32, height: u32) -> remodeling_camera::Intrinsics {
    remodeling_camera::Intrinsics { fx: 3.0 * f64::from(width), fy: 3.0 * f64::from(width), cx: f64::from(width) / 2.0, cy: f64::from(height) / 2.0, skew: 0.0, distortion: remodeling_camera::Distortion::None }
}

fn translated_pose(tx: f64, ty: f64, tz: f64) -> remodeling_camera::CameraPose {
    remodeling_camera::CameraPose(remodeling_camera::Se3::exp([tx, ty, tz, 0.0, 0.0, 0.0]))
}

/// 🎨️ Non-periodic per-cell hashed noise texture: a checkerboard's strict periodicity creates an
/// aperture-problem ambiguity for stereo matching (a shift by one period looks identical), which
/// biases the recovered depth; hashing a pseudo-random intensity per cell keeps comparable
/// spatial scale (so it stays well-resolved by the pixel footprint) while breaking that
/// periodicity.
fn noise_texture(x: f64, y: f64) -> f32 {
    let cx = (x / 0.05).floor() as i64;
    let cy = (y / 0.05).floor() as i64;
    let h = (cx.wrapping_mul(73_856_093) ^ cy.wrapping_mul(19_349_663)) as u64;
    let h = h ^ (h >> 15);
    let h = h.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    let h = h ^ (h >> 32);
    (h % 1000) as f32 / 1000.0
}

/// 🌍️ `(camera-frame depth, world point)` where the ray through pixel `(px, py)` meets the
/// world plane `z = plane_z`, or `None` when the ray is parallel to the plane or points away
/// from it.
fn plane_camera_depth(intr: &remodeling_camera::Intrinsics, pose: &remodeling_camera::CameraPose, plane_z: f64, px: f64, py: f64) -> Option<(f64, [f64; 3])> {
    let ray = intr.unproject_ray([px, py]);
    let to_world = pose.0.inverse();
    let origin_world = to_world.act([0.0, 0.0, 0.0]);
    let ray_point_world = to_world.act(ray);
    let dir = [ray_point_world[0] - origin_world[0], ray_point_world[1] - origin_world[1], ray_point_world[2] - origin_world[2]];
    if dir[2].abs() < 1e-12 {
        return None;
    }
    let t = (plane_z - origin_world[2]) / dir[2];
    if t <= 0.0 {
        return None;
    }
    let world_point = [origin_world[0] + dir[0] * t, origin_world[1] + dir[1] * t, origin_world[2] + dir[2] * t];
    let cam_point = pose.0.act(world_point);
    Some((cam_point[2], world_point))
}

fn render_plane_image(width: u32, height: u32, intr: &remodeling_camera::Intrinsics, pose: &remodeling_camera::CameraPose, plane_z: f64, texture: impl Fn(f64, f64) -> f32) -> remodeling_image::ImageGray {
    let mut img = remodeling_image::ImageGray::new(width, height);
    for y in 0..height {
        for x in 0..width {
            if let Some((_, world_point)) = plane_camera_depth(intr, pose, plane_z, f64::from(x), f64::from(y)) {
                img.set(x, y, texture(world_point[0], world_point[1]));
            }
        }
    }
    img
}

fn fill_plane_depth_map(width: u32, height: u32, intr: &remodeling_camera::Intrinsics, pose: &remodeling_camera::CameraPose, plane_z: f64) -> DepthMap {
    let mut dm = DepthMap::new(width, height);
    for y in 0..height {
        for x in 0..width {
            if let Some((depth, _)) = plane_camera_depth(intr, pose, plane_z, f64::from(x), f64::from(y)) {
                let idx = depthmap_index(width, x, y);
                dm.depth[idx] = depth as f32;
                dm.normal[idx] = [0.0, 0.0, -1.0];
                dm.confidence[idx] = 1.0;
            }
        }
    }
    dm
}

/// 🎨️ [`noise_texture`] extended to a third coordinate, so it can texture a curved (sphere)
/// surface instead of only a `z = const` plane.
fn noise_texture3(x: f64, y: f64, z: f64) -> f32 {
    let cx = (x / 0.05).floor() as i64;
    let cy = (y / 0.05).floor() as i64;
    let cz = (z / 0.05).floor() as i64;
    let h = (cx.wrapping_mul(73_856_093) ^ cy.wrapping_mul(19_349_663) ^ cz.wrapping_mul(83_492_791)) as u64;
    let h = h ^ (h >> 15);
    let h = h.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    let h = h ^ (h >> 32);
    (h % 1000) as f32 / 1000.0
}

/// 🌐️ `(camera-frame depth, world point, world outward normal)` for the nearest intersection of
/// the ray through pixel `(px, py)` with the sphere of `radius` centered at `center` (world
/// frame), or `None` when the ray misses the sphere or only hits it behind the camera.
fn sphere_camera_depth(intr: &remodeling_camera::Intrinsics, pose: &remodeling_camera::CameraPose, center: [f64; 3], radius: f64, px: f64, py: f64) -> Option<(f64, [f64; 3], [f64; 3])> {
    let ray = intr.unproject_ray([px, py]);
    let to_world = pose.0.inverse();
    let origin_world = to_world.act([0.0, 0.0, 0.0]);
    let ray_point_world = to_world.act(ray);
    let dir = [ray_point_world[0] - origin_world[0], ray_point_world[1] - origin_world[1], ray_point_world[2] - origin_world[2]];
    let dir_len = (dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2]).sqrt();
    if dir_len < 1e-12 {
        return None;
    }
    let dir_n = [dir[0] / dir_len, dir[1] / dir_len, dir[2] / dir_len];
    let oc = [origin_world[0] - center[0], origin_world[1] - center[1], origin_world[2] - center[2]];
    let b = 2.0 * (oc[0] * dir_n[0] + oc[1] * dir_n[1] + oc[2] * dir_n[2]);
    let c = oc[0] * oc[0] + oc[1] * oc[1] + oc[2] * oc[2] - radius * radius;
    let disc = b * b - 4.0 * c;
    if disc < 0.0 {
        return None;
    }
    let sqrt_disc = disc.sqrt();
    let t0 = (-b - sqrt_disc) / 2.0;
    let t1 = (-b + sqrt_disc) / 2.0;
    let t = if t0 > 1e-6 {
        t0
    } else if t1 > 1e-6 {
        t1
    } else {
        return None;
    };
    let world_point = [origin_world[0] + dir_n[0] * t, origin_world[1] + dir_n[1] * t, origin_world[2] + dir_n[2] * t];
    let cam_point = pose.0.act(world_point);
    if cam_point[2] <= 0.0 {
        return None;
    }
    let normal_world = [(world_point[0] - center[0]) / radius, (world_point[1] - center[1]) / radius, (world_point[2] - center[2]) / radius];
    Some((cam_point[2], world_point, normal_world))
}

fn render_sphere_image(width: u32, height: u32, intr: &remodeling_camera::Intrinsics, pose: &remodeling_camera::CameraPose, center: [f64; 3], radius: f64) -> remodeling_image::ImageGray {
    let mut img = remodeling_image::ImageGray::new(width, height);
    for y in 0..height {
        for x in 0..width {
            if let Some((_, world_point, _)) = sphere_camera_depth(intr, pose, center, radius, f64::from(x), f64::from(y)) {
                img.set(x, y, noise_texture3(world_point[0], world_point[1], world_point[2]));
            }
        }
    }
    img
}

/// 🌐️ Analytic depth map of the sphere, camera-frame normals derived from the world outward
/// normal via the linearity of the rigid transform (`act(p + n) - act(p) == R n` exactly, since
/// the translation term cancels — avoids needing a separate rotation-only API).
fn fill_sphere_depth_map(width: u32, height: u32, intr: &remodeling_camera::Intrinsics, pose: &remodeling_camera::CameraPose, center: [f64; 3], radius: f64) -> DepthMap {
    let mut dm = DepthMap::new(width, height);
    for y in 0..height {
        for x in 0..width {
            if let Some((depth, world_point, normal_world)) = sphere_camera_depth(intr, pose, center, radius, f64::from(x), f64::from(y)) {
                let idx = depthmap_index(width, x, y);
                dm.depth[idx] = depth as f32;
                let p_cam = pose.0.act(world_point);
                let n_cam = pose.0.act([world_point[0] + normal_world[0], world_point[1] + normal_world[1], world_point[2] + normal_world[2]]);
                let ncam_dir = [n_cam[0] - p_cam[0], n_cam[1] - p_cam[1], n_cam[2] - p_cam[2]];
                dm.normal[idx] = [ncam_dir[0] as f32, ncam_dir[1] as f32, ncam_dir[2] as f32];
                dm.confidence[idx] = 1.0;
            }
        }
    }
    dm
}

// #region 🔖️PatchMatchTests
#[test]
fn patchmatch_mvs_recovers_known_plane_depth() {
    let (width, height) = (48u32, 48u32);
    let intr = intrinsics_for(width, height);
    let true_depth = 5.0f64;
    let ref_pose = remodeling_camera::CameraPose(remodeling_camera::Se3::identity());
    let src_pose1 = translated_pose(-0.6, 0.0, 0.0);
    let src_pose2 = translated_pose(0.5, -0.35, 0.0);
    let ref_img = render_plane_image(width, height, &intr, &ref_pose, true_depth, noise_texture);
    let src_img1 = render_plane_image(width, height, &intr, &src_pose1, true_depth, noise_texture);
    let src_img2 = render_plane_image(width, height, &intr, &src_pose2, true_depth, noise_texture);
    let src_views = vec![(src_img1, src_pose1, intr), (src_img2, src_pose2, intr)];
    let cfg = PatchMatchConfig { window_radius: 6, iterations: 8, depth_min: 2.0, depth_max: 10.0, seed: 7, best_k: 2 };
    let dm = patchmatch_mvs(&ref_img, &(ref_pose, intr), &src_views, &cfg);

    let mut abs_errors = Vec::new();
    for y in 4..(height - 4) {
        for x in 4..(width - 4) {
            if let Some(d) = dm.get(x, y) {
                abs_errors.push((f64::from(d) - true_depth).abs());
            }
        }
    }
    assert!(abs_errors.len() > 200, "expected most interior pixels valid, got {}", abs_errors.len());
    abs_errors.sort_by(f64::total_cmp);
    let median_err = abs_errors[abs_errors.len() / 2];
    let range = f64::from(cfg.depth_max - cfg.depth_min);
    assert!(median_err < 0.01 * range, "median depth error {median_err} vs 1% of range {range} ({}..{})", cfg.depth_min, cfg.depth_max);
}

#[test]
fn patchmatch_mvs_recovers_known_sphere_depth() {
    let (width, height) = (48u32, 48u32);
    let intr = intrinsics_for(width, height);
    let center = [0.0, 0.0, 5.0];
    let radius = 1.0;
    let ref_pose = remodeling_camera::CameraPose(remodeling_camera::Se3::identity());
    let src_pose1 = translated_pose(-0.4, 0.0, 0.0);
    let src_pose2 = translated_pose(0.3, -0.25, 0.0);
    let ref_img = render_sphere_image(width, height, &intr, &ref_pose, center, radius);
    let src_img1 = render_sphere_image(width, height, &intr, &src_pose1, center, radius);
    let src_img2 = render_sphere_image(width, height, &intr, &src_pose2, center, radius);
    let src_views = vec![(src_img1, src_pose1, intr), (src_img2, src_pose2, intr)];
    let depth_min = (center[2] - radius - 0.5) as f32;
    let depth_max = (center[2] + radius + 0.5) as f32;
    let cfg = PatchMatchConfig { window_radius: 6, iterations: 8, depth_min, depth_max, seed: 11, best_k: 2 };
    let dm = patchmatch_mvs(&ref_img, &(ref_pose, intr), &src_views, &cfg);

    let mut abs_errors = Vec::new();
    for y in 4..(height - 4) {
        for x in 4..(width - 4) {
            if let Some(d) = dm.get(x, y) {
                if let Some((true_depth, _, _)) = sphere_camera_depth(&intr, &ref_pose, center, radius, f64::from(x), f64::from(y)) {
                    abs_errors.push((f64::from(d) - true_depth).abs());
                }
            }
        }
    }
    assert!(abs_errors.len() > 200, "expected most interior pixels valid, got {}", abs_errors.len());
    abs_errors.sort_by(f64::total_cmp);
    let median_err = abs_errors[abs_errors.len() / 2];
    let range = f64::from(depth_max - depth_min);
    assert!(median_err < 0.01 * range, "median depth error {median_err} vs 1% of range {range}");
}

#[test]
fn maximum_patchmatch_allocation_and_one_pixel_step_stay_below_hard_ceiling_in_each_build_profile() {
    let (width, height) = (512u32, 512u32);
    let pixels = width as usize * height as usize;
    let mut data = vec![0.0; pixels];
    for (index, value) in data.iter_mut().enumerate() {
        *value = f32::from(((index * 131) ^ (index / width as usize * 197)) as u8) / 255.0;
    }
    let reference = remodeling_image::ImageGray { width, height, data };
    let intrinsics = intrinsics_for(width, height);
    let pose = remodeling_camera::CameraPose(remodeling_camera::Se3::identity());
    let sources = (0..8).map(|index| (reference.clone(), translated_pose(index as f64 * 0.01, 0.0, 0.0), intrinsics)).collect::<Vec<_>>();
    let started = std::time::Instant::now();
    let mut preparation = PatchMatchPreparation::new(width, height);
    assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum PatchMatch fixed-envelope buffer reservation exceeded 8 ms");
    assert_eq!(preparation.depths.capacity(), pixels);
    preparation.phase = PatchMatchPhase::Initialize;
    preparation.depths.push(0.0);
    preparation.normals.push([0.0, 0.0, -1.0]);
    preparation.costs.push(-1.0);
    preparation.output.depth.push(0.0);
    preparation.output.normal.push([0.0; 3]);
    preparation.output.confidence.push(0.0);
    let config = PatchMatchConfig { window_radius: 4, iterations: 1, depth_min: 0.1, depth_max: 100.0, seed: u64::MAX, best_k: 4 };
    let started = std::time::Instant::now();
    assert!(!preparation.advance(&reference, &(pose, intrinsics), &sources, &config, 1));
    assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum-source/radius PatchMatch pixel step exceeded 8 ms");
}
// #endregion 🔖️PatchMatchTests

// #region 🔖️PlaneSweepTests
#[test]
fn plane_sweep_depth_recovers_known_plane_depth() {
    let (width, height) = (32u32, 32u32);
    let intr = intrinsics_for(width, height);
    let true_depth = 5.0f64;
    let ref_pose = remodeling_camera::CameraPose(remodeling_camera::Se3::identity());
    let src_pose1 = translated_pose(-0.6, 0.0, 0.0);
    let src_pose2 = translated_pose(0.5, -0.35, 0.0);
    let ref_img = render_plane_image(width, height, &intr, &ref_pose, true_depth, noise_texture);
    let src_img1 = render_plane_image(width, height, &intr, &src_pose1, true_depth, noise_texture);
    let src_img2 = render_plane_image(width, height, &intr, &src_pose2, true_depth, noise_texture);
    let src_views = vec![(src_img1, src_pose1, intr), (src_img2, src_pose2, intr)];
    let dm = plane_sweep_depth(&ref_img, &(ref_pose, intr), &src_views, 2.0, 10.0, 96);

    let mut depths = Vec::new();
    for y in 4..(height - 4) {
        for x in 4..(width - 4) {
            if let Some(d) = dm.get(x, y) {
                depths.push(d);
            }
        }
    }
    assert!(depths.len() > 200, "expected most interior pixels valid, got {}", depths.len());
    depths.sort_by(f32::total_cmp);
    let median = f64::from(depths[depths.len() / 2]);
    let rel_err = (median - true_depth).abs() / true_depth;
    assert!(rel_err < 0.02, "median depth {median} vs true {true_depth}, rel_err {rel_err}");
}
// #endregion 🔖️PlaneSweepTests

// #region 🔖️DepthFilterTests
#[test]
fn left_right_check_invalidates_planted_inconsistency() {
    let (width, height) = (20u32, 20u32);
    let intr = intrinsics_for(width, height);
    let ref_pose = remodeling_camera::CameraPose(remodeling_camera::Se3::identity());
    let other_pose = translated_pose(-0.1, 0.0, 0.0);
    let true_depth = 5.0f32;

    let mut ref_dm = DepthMap::new(width, height);
    let mut other_dm = DepthMap::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let idx = depthmap_index(width, x, y);
            ref_dm.depth[idx] = true_depth;
            ref_dm.confidence[idx] = 1.0;
            other_dm.depth[idx] = true_depth;
            other_dm.confidence[idx] = 1.0;
        }
    }
    let bad_idx = depthmap_index(width, 10, 10);
    ref_dm.depth[bad_idx] = 9.0;

    let filtered = left_right_check(&ref_dm, &other_dm, &(ref_pose, intr), &(other_pose, intr), 0.05);
    assert!(filtered.get(10, 10).is_none());
    assert!(filtered.get(9, 9).is_some());
    assert!((filtered.get(9, 9).unwrap() - true_depth).abs() < 1e-4);
}

#[test]
fn speckle_filter_removes_small_isolated_components() {
    let (width, height) = (10u32, 10u32);
    let mut dm = DepthMap::new(width, height);
    for y in 0..height {
        for x in 0..8u32 {
            let idx = depthmap_index(width, x, y);
            dm.depth[idx] = 5.0;
            dm.confidence[idx] = 1.0;
        }
    }
    for y in 0..2u32 {
        let idx = depthmap_index(width, 9, y);
        dm.depth[idx] = 5.0;
        dm.confidence[idx] = 1.0;
    }
    let filtered = speckle_filter(&dm, 5, 0.1);
    assert!(filtered.get(9, 0).is_none());
    assert!(filtered.get(9, 1).is_none());
    assert!(filtered.get(0, 0).is_some());
    assert!(filtered.get(7, 5).is_some());
}

#[test]
fn median_fill_fills_missing_pixels_from_valid_neighbors() {
    let (width, height) = (5u32, 5u32);
    let mut dm = DepthMap::new(width, height);
    for v in dm.depth.iter_mut() {
        *v = 2.0;
    }
    let hole = depthmap_index(width, 2, 2);
    dm.depth[hole] = 0.0;
    let filled = median_fill(&dm, 1);
    assert!((filled.get(2, 2).unwrap() - 2.0).abs() < 1e-6);

    let empty = DepthMap::new(3, 3);
    let filled_empty = median_fill(&empty, 1);
    assert!(filled_empty.get(1, 1).is_none());
}

#[test]
fn margin_confidence_rewards_well_textured_high_margin() {
    let (width, height) = (24u32, 24u32);
    let intr = intrinsics_for(width, height);
    let true_depth = 5.0;
    let ref_pose = remodeling_camera::CameraPose(remodeling_camera::Se3::identity());
    let src_pose = translated_pose(-0.5, 0.0, 0.0);
    let ref_img = render_plane_image(width, height, &intr, &ref_pose, true_depth, noise_texture);
    let src_img = render_plane_image(width, height, &intr, &src_pose, true_depth, noise_texture);
    let src_views = vec![(src_img, src_pose, intr)];
    let dm = fill_plane_depth_map(width, height, &intr, &ref_pose, true_depth);

    // A small depth perturbation induces only a sub-pixel reprojection shift at this
    // depth/baseline (too little to move ZNCC), so the margin probe needs a step large enough
    // to move the warp by several pixels.
    let scored = margin_confidence(&dm, &ref_img, &(ref_pose, intr), &src_views, 3, 3.0);
    let mut confidences = Vec::new();
    for y in 4..(height - 4) {
        for x in 4..(width - 4) {
            if scored.get(x, y).is_some() {
                confidences.push(scored.confidence[depthmap_index(width, x, y)]);
            }
        }
    }
    assert!(!confidences.is_empty());
    let mean: f32 = confidences.iter().sum::<f32>() / confidences.len() as f32;
    assert!(mean > 0.15, "expected a clear margin for the correct depth on textured content, got mean {mean}");

    // A blank (untextured) image gives ZNCC == 0 everywhere (degenerate variance, per
    // `remodeling_image::zncc`), so there is no margin between the accepted depth and its
    // depth-perturbed competitors.
    let blank_ref = remodeling_image::ImageGray::new(width, height);
    let blank_src = remodeling_image::ImageGray::new(width, height);
    let blank_views = vec![(blank_src, src_pose, intr)];
    let blank_scored = margin_confidence(&dm, &blank_ref, &(ref_pose, intr), &blank_views, 3, 3.0);
    for y in 4..(height - 4) {
        for x in 4..(width - 4) {
            if blank_scored.get(x, y).is_some() {
                let c = blank_scored.confidence[depthmap_index(width, x, y)];
                assert!(c < 1e-6, "expected zero margin on blank content, got {c}");
            }
        }
    }
}
// #endregion 🔖️DepthFilterTests

// #region 🔖️FusionTests
#[test]
fn fuse_depth_maps_recovers_plane_points() {
    let (width, height) = (16u32, 16u32);
    let intr = intrinsics_for(width, height);
    let plane_z = 5.0;
    let poses = [remodeling_camera::CameraPose(remodeling_camera::Se3::identity()), translated_pose(-0.3, 0.0, 0.0), translated_pose(0.15, -0.2, 0.0)];
    let depth_maps: Vec<DepthMap> = poses.iter().map(|pose| fill_plane_depth_map(width, height, &intr, pose, plane_z)).collect();
    let views: Vec<(remodeling_camera::CameraPose, remodeling_camera::Intrinsics)> = poses.iter().map(|&p| (p, intr)).collect();
    let cfg = FusionConfig { max_relative_depth_diff: 0.02, max_normal_angle_deg: 20.0, min_consistent_views: 2 };
    let cloud = fuse_depth_maps(&views, &depth_maps, &cfg);
    assert!(!cloud.is_empty());
    for p in &cloud.positions {
        assert!((p[2] - plane_z).abs() < 0.05, "point {p:?} not on plane");
    }
}

#[test]
fn maximum_fusion_reservation_and_comparison_step_stay_below_hard_ceiling_in_each_build_profile() {
    let started = std::time::Instant::now();
    let mut preparation = FusionPreparation::new(usize::MAX);
    assert!(started.elapsed() < std::time::Duration::from_millis(8), "bounded fused-cloud reservation exceeded 8 ms");
    assert_eq!(preparation.output.positions.capacity(), MAX_INTERACTIVE_FUSED_POINTS);

    let intrinsics = intrinsics_for(1, 1);
    let views = (0..12).map(|index| (translated_pose(index as f64 * 0.001, 0.0, 0.0), intrinsics)).collect::<Vec<_>>();
    let depth_maps = (0..12).map(|_| DepthMap { width: 1, height: 1, depth: vec![4.0], normal: vec![[0.0, 0.0, -1.0]], confidence: vec![1.0] }).collect::<Vec<_>>();
    let started = std::time::Instant::now();
    let _ = preparation.advance(&views, &depth_maps, &FusionConfig::default(), 256);
    assert!(started.elapsed() < std::time::Duration::from_millis(8), "12-view/256-comparison fusion worker step exceeded 8 ms");

    preparation.output.positions.resize(MAX_INTERACTIVE_FUSED_POINTS, [0.0; 3]);
    preparation.output.normals.resize(MAX_INTERACTIVE_FUSED_POINTS, [0.0; 3]);
    preparation.output.confidence.resize(MAX_INTERACTIVE_FUSED_POINTS, 0.0);
    preparation.agreeing_count = 12;
    preparation.finish_pixel(2);
    assert_eq!(preparation.output.positions.len(), MAX_INTERACTIVE_FUSED_POINTS, "fusion must not grow past the interactive output envelope");
}
// #endregion 🔖️FusionTests

// #region 🔖️TsdfTests
#[test]
fn tsdf_integrate_zero_crossing_near_true_surface() {
    let (width, height) = (24u32, 24u32);
    let intr = intrinsics_for(width, height);
    let pose = remodeling_camera::CameraPose(remodeling_camera::Se3::identity());
    let plane_z = 3.0;
    let dm = fill_plane_depth_map(width, height, &intr, &pose, plane_z);
    let mut vol = TsdfVolume::new(0.05, 0.2);
    vol.integrate(&dm, &(pose, intr), false);

    let ray = intr.unproject_ray([f64::from(width) / 2.0, f64::from(height) / 2.0]);
    let mut prev_sign: Option<f64> = None;
    let mut crossing_z: Option<f64> = None;
    let mut z = plane_z - 0.15;
    while z <= plane_z + 0.15 {
        let p = [ray[0] * z, ray[1] * z, ray[2] * z];
        if let Some(sdf) = vol.sample_tsdf(p) {
            let sign = sdf.signum();
            if let Some(ps) = prev_sign {
                if ps != sign && sign != 0.0 && crossing_z.is_none() {
                    crossing_z = Some(z);
                }
            }
            prev_sign = Some(sign);
        }
        z += 0.02;
    }
    let cz = crossing_z.expect("expected a zero crossing near the surface");
    assert!((cz - plane_z).abs() < 0.1, "crossing at {cz} vs true {plane_z}");
}

#[test]
fn maximum_tsdf_sample_and_malformed_parameter_steps_stay_below_hard_ceiling_in_each_build_profile() {
    let intrinsics = intrinsics_for(1, 1);
    let pose = remodeling_camera::CameraPose(remodeling_camera::Se3::identity());
    let depth = DepthMap { width: 1, height: 1, depth: vec![4.0], normal: vec![[0.0, 0.0, -1.0]], confidence: vec![1.0] };
    let mut volume = TsdfVolume::new(0.001, 1.0);
    let mut preparation = TsdfIntegrationPreparation::new();
    let started = std::time::Instant::now();
    assert!(!preparation.advance(&mut volume, &depth, &(pose, intrinsics), true, 256));
    assert!(started.elapsed() < std::time::Duration::from_millis(8), "256-sample TSDF allocation/integration worker step exceeded 8 ms");

    let mut malformed = TsdfVolume::new(0.0, f64::NAN);
    let mut preparation = TsdfIntegrationPreparation::new();
    let started = std::time::Instant::now();
    assert!(preparation.advance(&mut malformed, &depth, &(pose, intrinsics), true, 256));
    assert!(started.elapsed() < std::time::Duration::from_millis(8), "malformed TSDF parameter rejection exceeded 8 ms");
}

#[test]
fn tsdf_sphere_multi_view_zero_crossing_within_one_voxel() {
    let (width, height) = (48u32, 48u32);
    let intr = intrinsics_for(width, height);
    let center = [0.0, 0.0, 5.0];
    let radius = 1.0;
    let poses = [translated_pose(0.0, 0.0, 0.0), translated_pose(-0.3, 0.0, 0.0), translated_pose(0.3, 0.0, 0.0), translated_pose(0.0, -0.3, 0.0), translated_pose(0.0, 0.3, 0.0)];
    let voxel_size = 0.05;
    let mut vol = TsdfVolume::new(voxel_size, 0.2);
    for pose in &poses {
        let dm = fill_sphere_depth_map(width, height, &intr, pose, center, radius);
        vol.integrate(&dm, &(*pose, intr), true);
    }

    let mut prev_sign: Option<f64> = None;
    let mut crossing_r: Option<f64> = None;
    let mut r = radius - 0.2;
    while r <= radius + 0.2 {
        let p = [center[0], center[1], center[2] - r];
        if let Some(sdf) = vol.sample_tsdf(p) {
            let sign = sdf.signum();
            if let Some(ps) = prev_sign {
                if ps != sign && sign != 0.0 && crossing_r.is_none() {
                    crossing_r = Some(r);
                }
            }
            prev_sign = Some(sign);
        }
        r += 0.01;
    }
    let cr = crossing_r.expect("expected a zero crossing near the sphere surface");
    assert!((cr - radius).abs() < voxel_size, "crossing radius {cr} vs true {radius}, voxel {voxel_size}");
}

#[test]
fn tsdf_sample_agrees_across_block_boundaries_regardless_of_integration_order() {
    // Deliberately low-resolution/low-truncation: a running weighted average is exactly
    // order-independent only until `TSDF_MAX_WEIGHT` clamps a voxel's accumulated weight, at
    // which point which particular samples got "locked in" is a genuine (not merely
    // floating-point-noise) function of arrival order — an inherent property of any
    // weight-capped Curless-Levoy fusion, not a bug. Keeping per-voxel contributions well under
    // the cap isolates the property this test actually checks: `sample`'s pure hash-lookup
    // addressing agrees bit-for-bit across block boundaries regardless of integration order.
    let (width, height) = (5u32, 5u32);
    let intr = intrinsics_for(width, height);
    let pose = remodeling_camera::CameraPose(remodeling_camera::Se3::identity());
    let pose2 = translated_pose(-0.05, 0.02, 0.0);
    let pose3 = translated_pose(0.04, -0.03, 0.0);
    // Plane depth chosen so its zero-crossing lands right at a TSDF block boundary (block dim
    // 8, voxel size 0.1 -> boundary at world z = 0.8), the exact seam `sample()` must resolve
    // identically from either side of.
    let voxel_size = 0.1;
    let truncation = 0.1;
    let plane_z = 0.8;
    let dm1 = fill_plane_depth_map(width, height, &intr, &pose, plane_z);
    let dm2 = fill_plane_depth_map(width, height, &intr, &pose2, plane_z);
    let dm3 = fill_plane_depth_map(width, height, &intr, &pose3, plane_z);

    let mut vol_forward = TsdfVolume::new(voxel_size, truncation);
    vol_forward.integrate(&dm1, &(pose, intr), false);
    vol_forward.integrate(&dm2, &(pose2, intr), false);
    vol_forward.integrate(&dm3, &(pose3, intr), false);

    let mut vol_reverse = TsdfVolume::new(voxel_size, truncation);
    vol_reverse.integrate(&dm3, &(pose3, intr), false);
    vol_reverse.integrate(&dm2, &(pose2, intr), false);
    vol_reverse.integrate(&dm1, &(pose, intr), false);

    let mut compared = 0;
    for ix in -3..3 {
        for iy in -3..3 {
            for iz in 3..13 {
                let a = vol_forward.sample(ix, iy, iz);
                let b = vol_reverse.sample(ix, iy, iz);
                match (a, b) {
                    (Some((sdf_a, w_a)), Some((sdf_b, w_b))) => {
                        // Tolerance is well above pure floating-point summation-order noise but
                        // still far tighter than weight-cap-induced order dependence would need
                        // (ruled out above by construction), so it stays a meaningful check.
                        assert!((sdf_a - sdf_b).abs() < 1e-3, "sdf mismatch at ({ix},{iy},{iz}): {sdf_a} vs {sdf_b}");
                        assert!((w_a - w_b).abs() < 1e-3, "weight mismatch at ({ix},{iy},{iz}): {w_a} vs {w_b}");
                        compared += 1;
                    }
                    (None, None) => {}
                    _ => panic!("observed-state mismatch at ({ix},{iy},{iz}): {a:?} vs {b:?}"),
                }
            }
        }
    }
    assert!(compared > 5, "expected several overlapping observed voxels, got {compared}");
}
// #endregion 🔖️TsdfTests

// #region 🔖️CloudOpsTests
#[test]
fn estimate_normals_recovers_plane_normal() {
    let mut state = 123u64;
    let mut positions = Vec::new();
    for _ in 0..300 {
        let x = (lcg_next(&mut state) - 0.5) * 10.0;
        let y = (lcg_next(&mut state) - 0.5) * 10.0;
        positions.push([x, y, 0.0]);
    }
    let mut cloud = PointCloud::from_positions(positions);
    estimate_normals(&mut cloud, 12, [0.0, 0.0, 10.0]);
    let normals = cloud.normals.clone();
    assert!(!normals.is_empty(), "normals set");
    let mut ok = 0;
    let mut total = 0;
    for (i, n) in normals.iter().enumerate() {
        let p = cloud.positions[i];
        if p[0].abs() > 3.5 || p[1].abs() > 3.5 {
            continue;
        }
        total += 1;
        if n[2] > 0.99 {
            ok += 1;
        }
    }
    assert!(total > 50, "expected enough interior points, got {total}");
    assert!(ok as f64 / total as f64 > 0.9, "expected most interior normals to face +z, got {ok}/{total}");
}

#[test]
fn voxel_downsample_averages_grid_cells_correctly() {
    let positions = vec![[0.1, 0.1, 0.1], [0.4, 0.4, 0.4], [1.6, 1.6, 1.6], [1.9, 1.9, 1.9]];
    let cloud = PointCloud::from_positions(positions);
    let down = voxel_downsample(&cloud, 1.0);
    assert!(down.len() <= 4);
    assert_eq!(down.len(), 2);
    let mut sorted = down.positions;
    sorted.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
    assert!((sorted[0][0] - 0.25).abs() < 1e-9);
    assert!((sorted[1][0] - 1.75).abs() < 1e-9);
}

#[test]
fn statistical_outlier_removal_keeps_cluster_and_drops_far_outliers() {
    let mut state = 55u64;
    let mut positions = Vec::new();
    for _ in 0..200 {
        positions.push([(lcg_next(&mut state) - 0.5) * 2.0, (lcg_next(&mut state) - 0.5) * 2.0, (lcg_next(&mut state) - 0.5) * 2.0]);
    }
    let cluster_size = positions.len();
    positions.push([100.0, 100.0, 100.0]);
    positions.push([-120.0, 50.0, 10.0]);
    let cloud = PointCloud::from_positions(positions);
    let filtered = statistical_outlier_removal(&cloud, 8, 2.0);
    assert!(filtered.len() >= cluster_size - 5);
    for p in &filtered.positions {
        assert!(p[0].abs() < 50.0 && p[1].abs() < 50.0);
    }
}

#[test]
fn radius_outlier_removal_drops_isolated_points() {
    let mut state = 91u64;
    let mut positions = Vec::new();
    for _ in 0..1000 {
        positions.push([(lcg_next(&mut state) - 0.5) * 2.0, (lcg_next(&mut state) - 0.5) * 2.0, (lcg_next(&mut state) - 0.5) * 2.0]);
    }
    let cluster_size = positions.len();
    positions.push([50.0, 50.0, 50.0]);
    let cloud = PointCloud::from_positions(positions);
    let filtered = radius_outlier_removal(&cloud, 0.5, 3);
    assert!(filtered.positions.iter().all(|p| p[0].abs() < 10.0));
    assert!(filtered.len() >= cluster_size - 20, "kept {} of {cluster_size} cluster points", filtered.len());
}
// #endregion 🔖️CloudOpsTests

// #region 🔖️ClassifyTests
#[test]
fn classify_ground_pmf_achieves_high_ground_recall_with_buildings_and_vegetation() {
    let mut positions = Vec::new();
    for iy in 0..20 {
        for ix in 0..20 {
            positions.push([f64::from(ix) * 0.5, f64::from(iy) * 0.5, 0.0]);
        }
    }
    let ground_count = positions.len();
    // Elevated "building" block: a small planar cluster well above the ground.
    for iy in 0..4 {
        for ix in 0..4 {
            positions.push([2.0 + f64::from(ix) * 0.3, 5.0 + f64::from(iy) * 0.3, 3.0]);
        }
    }
    // Scattered "vegetation": irregular heights over a patch that overlaps the ground grid's
    // own `[0, 9.5]` extent (so PMF's opening window has nearby ground samples to erode from —
    // a cluster placed entirely outside the ground's covered area is fundamentally unreachable
    // for *any* ground filter, not just this one), well above the PMF's max
    // (window <= max_iterations = 4) `max_slope * window * cell = 0.3 * 4 * 0.5 = 0.6` opening
    // threshold so it is reliably excluded from ground.
    let mut state = 7u64;
    for _ in 0..40 {
        let x = 6.0 + lcg_next(&mut state) * 3.0;
        let y = 6.0 + lcg_next(&mut state) * 3.0;
        let z = 1.0 + lcg_next(&mut state) * 2.0;
        positions.push([x, y, z]);
    }
    let cloud = PointCloud::from_positions(positions);
    let labels = classify_ground_pmf(&cloud, 0.5, 0.3, 4);
    let ground_labeled = labels[..ground_count].iter().filter(|l| matches!(l, PointClass::Ground)).count();
    let recall = f64::from(ground_labeled as u32) / ground_count as f64;
    assert!(recall >= 0.95, "ground recall {recall} ({ground_labeled}/{ground_count})");
    let non_ground_kept = labels[ground_count..].iter().filter(|l| matches!(l, PointClass::Ground)).count();
    let non_ground_total = labels.len() - ground_count;
    assert!(f64::from(non_ground_kept as u32) / non_ground_total as f64 <= 0.1, "too many non-ground points kept as ground: {non_ground_kept}/{non_ground_total}");
}

#[test]
fn classify_building_vegetation_splits_planar_from_scattered() {
    // A flat planar patch (roof-like) vs a scattered noisy patch (canopy-like), both elevated
    // above the ground and pre-labeled Unclassified as classify_ground_pmf would leave them.
    // Interior-grid points are asserted on (not the boundary ring): a boundary point's k-NN
    // neighborhood is one-sided even on a perfectly flat patch, which can skew its two in-plane
    // eigenvalues apart and understate planarity — the same edge effect the other geometric
    // tests in this module dodge by checking only interior points.
    let mut positions = Vec::new();
    let mut interior = Vec::new();
    for iy in 0..10 {
        for ix in 0..10 {
            positions.push([f64::from(ix) * 0.3, f64::from(iy) * 0.3, 3.0]);
            interior.push(ix > 0 && ix < 9 && iy > 0 && iy < 9);
        }
    }
    let planar_count = positions.len();
    let mut state = 42u64;
    for _ in 0..150 {
        let x = 5.0 + lcg_next(&mut state) * 3.0;
        let y = 5.0 + lcg_next(&mut state) * 3.0;
        let z = 3.0 + lcg_next(&mut state) * 2.5;
        positions.push([x, y, z]);
    }
    let cloud = PointCloud::from_positions(positions);
    let mut labels = vec![PointClass::Unclassified; cloud.len()];
    classify_building_vegetation(&cloud, &mut labels, 10, 0.6);

    let interior_total = interior.iter().filter(|&&f| f).count();
    let planar_building = (0..planar_count).filter(|&i| interior[i] && matches!(labels[i], PointClass::Building)).count();
    assert!(f64::from(planar_building as u32) / interior_total as f64 > 0.85, "expected most of the interior planar patch classified Building, got {planar_building}/{interior_total}");

    let scattered_count = labels.len() - planar_count;
    let scattered_vegetation = labels[planar_count..].iter().filter(|l| matches!(l, PointClass::Vegetation)).count();
    assert!(f64::from(scattered_vegetation as u32) / scattered_count as f64 > 0.6, "expected most of the scattered patch classified Vegetation, got {scattered_vegetation}/{scattered_count}");
}

#[test]
fn region_grow_planes_groups_planar_patch() {
    let mut positions = Vec::new();
    for iy in 0..10 {
        for ix in 0..10 {
            positions.push([f64::from(ix) * 0.3, f64::from(iy) * 0.3, 0.0]);
        }
    }
    let mut cloud = PointCloud::from_positions(positions);
    estimate_normals(&mut cloud, 8, [0.0, 0.0, 10.0]);
    let segments = region_grow_planes(&cloud, 10.0, 20);
    assert!(!segments.is_empty());
    let biggest = &segments[0];
    assert!(biggest.point_indices.len() >= 50, "biggest segment only has {} points", biggest.point_indices.len());
    assert!(biggest.normal[2].abs() > 0.95, "unexpected normal {:?}", biggest.normal);
}
// #endregion 🔖️ClassifyTests

// #region 🔖️ChangeTests
#[test]
fn cloud_distance_and_m3c2_measure_planted_offset() {
    let mut positions_a = Vec::new();
    for iy in 0..15 {
        for ix in 0..15 {
            positions_a.push([f64::from(ix) * 0.2, f64::from(iy) * 0.2, 0.0]);
        }
    }
    let mut cloud_a = PointCloud::from_positions(positions_a.clone());
    estimate_normals(&mut cloud_a, 10, [0.0, 0.0, 10.0]);
    let offset = 0.2;
    let positions_b: Vec<[f64; 3]> = positions_a.iter().map(|p| [p[0], p[1], p[2] + offset]).collect();
    let cloud_b = PointCloud::from_positions(positions_b);

    let dists = cloud_to_cloud_distance(&cloud_a, &cloud_b);
    for d in &dists {
        assert!((d - offset).abs() < 0.05, "distance {d} vs offset {offset}");
    }

    let m3c2 = m3c2_distance(&cloud_a, &cloud_b, 1.0, 0.3);
    let mut checked = 0;
    for (i, d) in m3c2.iter().enumerate() {
        let p = cloud_a.positions[i];
        if p[0] < 0.7 || p[0] > 2.1 || p[1] < 0.7 || p[1] > 2.1 {
            continue;
        }
        if let Some(v) = d {
            assert!((v - offset).abs() < 0.05, "m3c2 {v} vs {offset}");
            checked += 1;
        }
    }
    assert!(checked > 20, "expected enough interior m3c2 results, got {checked}");
}
// #endregion 🔖️ChangeTests
