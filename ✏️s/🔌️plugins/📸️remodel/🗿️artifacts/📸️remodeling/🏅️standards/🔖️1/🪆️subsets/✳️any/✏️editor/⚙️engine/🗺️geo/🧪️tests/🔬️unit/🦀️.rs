use super::*;
use crate::lie::So3;
use geometry::random::{normal, Rng};

// #region 🔖️GeodesyTests
#[test]
fn ecef_enu_round_trip_sub_millimeter() {
    let cases =
        [(0.0_f64, 0.0_f64, 100.0_f64), (89.9_f64.to_radians(), 45.0_f64.to_radians(), 10.0), (-33.9_f64.to_radians(), 151.2_f64.to_radians(), 50.0), (47.3769_f64.to_radians(), 8.5417_f64.to_radians(), 500.0), (-90.0_f64.to_radians(), 0.0, 0.0)];
    for &(lat, lon, h) in &cases {
        let ecef = geodetic_to_ecef(lat, lon, h);
        let (lat2, lon2, h2) = ecef_to_geodetic(ecef);
        let ecef2 = geodetic_to_ecef(lat2, lon2, h2);
        let err = ((ecef[0] - ecef2[0]).powi(2) + (ecef[1] - ecef2[1]).powi(2) + (ecef[2] - ecef2[2]).powi(2)).sqrt();
        assert!(err < 1e-3, "ecef round trip err {err} at lat={lat} lon={lon}");

        let ref_lat = 47.0_f64.to_radians();
        let ref_lon = 8.0_f64.to_radians();
        let enu = ecef_to_enu(ecef, ref_lat, ref_lon, 0.0);
        let back = enu_to_ecef(enu, ref_lat, ref_lon, 0.0);
        let enu_err = ((ecef[0] - back[0]).powi(2) + (ecef[1] - back[1]).powi(2) + (ecef[2] - back[2]).powi(2)).sqrt();
        assert!(enu_err < 1e-3, "enu round trip err {enu_err} at lat={lat} lon={lon}");
    }
}

#[test]
fn utm_round_trip_sub_millimeter() {
    let cases = [(47.3769_f64, 8.5417_f64), (0.0001_f64, 5.9999_f64), (0.0001_f64, 6.0001_f64), (-33.9_f64, 151.2_f64), (60.0_f64, -1.0_f64), (10.0_f64, 100.0_f64)];
    for &(lat_deg, lon_deg) in &cases {
        let lat = lat_deg.to_radians();
        let lon = lon_deg.to_radians();
        let coord = geodetic_to_utm(lat, lon);
        let (lat2, lon2) = utm_to_geodetic(&coord);
        let back = geodetic_to_ecef(lat2, lon2, 0.0);
        let fwd = geodetic_to_ecef(lat, lon, 0.0);
        let err = ((back[0] - fwd[0]).powi(2) + (back[1] - fwd[1]).powi(2) + (back[2] - fwd[2]).powi(2)).sqrt();
        assert!(err < 1e-3, "utm round trip err {err} at lat={lat_deg} lon={lon_deg}");
    }
}
// #endregion 🔖️GeodesyTests

// #region 🔖️GcpTests
#[test]
fn georeference_recovers_planted_similarity_under_noise() {
    let mut rng = Rng::from_seed(9001);
    let scene = remodeling_sfm::synthetic_scene(9001, 6, 12, false);
    let scene_obs = remodeling_sfm::project_observations(&scene, 0.0, 0.0, 9001);
    let cameras: Vec<(CameraPose, Intrinsics)> = scene.cameras.iter().map(|&(intr, pose)| (pose, intr)).collect();
    let points = scene.points_world;
    let mut by_point: Vec<Vec<(usize, [f64; 2])>> = vec![Vec::new(); points.len()];
    for o in &scene_obs {
        by_point[o.point_index].push((o.camera_index, o.pixel));
    }
    let truth = Sim3 { s: 2.3, r: So3::exp([0.1, -0.2, 0.05]), t: [5.0, -3.0, 1.5] };
    let world_points: Vec<[f64; 3]> = points.iter().map(|&p| truth.act(p)).collect();
    let pos_noise_std = 0.02;
    let noisy_scene_points: Vec<[f64; 3]> = points.iter().map(|&p| [p[0] + normal(&mut rng, 0.0, pos_noise_std), p[1] + normal(&mut rng, 0.0, pos_noise_std), p[2] + normal(&mut rng, 0.0, pos_noise_std)]).collect();
    let gcps: Vec<GroundControlPoint> = (0..points.len()).map(|i| GroundControlPoint { id: format!("gcp{i}"), world_position_enu_or_local: world_points[i], observations: by_point[i].clone() }).collect();

    let (refined, sim3) = refine_gcp_scene_points(&noisy_scene_points, &gcps, &cameras);
    assert!((sim3.s - truth.s).abs() < 0.1, "scale err {} vs truth {}", sim3.s, truth.s);
    let t_err = ((sim3.t[0] - truth.t[0]).powi(2) + (sim3.t[1] - truth.t[1]).powi(2) + (sim3.t[2] - truth.t[2]).powi(2)).sqrt();
    assert!(t_err < 0.1, "translation err {t_err}");

    let rmse = gcp_checkpoint_rmse(&sim3, &refined, &gcps);
    assert!(rmse < 2.0 * pos_noise_std, "checkpoint rmse {rmse} exceeds 2x noise std {}", 2.0 * pos_noise_std);
}
// #endregion 🔖️GcpTests

// #region 🔖️DsmTests
fn bump(x: f64, y: f64) -> f64 {
    10.0 + 5.0 * (-(x * x + y * y) / 8.0).exp()
}

#[test]
fn dsm_dtm_match_known_terrain_and_idw_fills_holes() {
    let mut rng = Rng::from_seed(77);
    let mut positions = Vec::new();
    let mut labels = Vec::new();
    for _ in 0..4000 {
        let x = (rng.next_f64() - 0.5) * 20.0;
        let y = (rng.next_f64() - 0.5) * 20.0;
        let ground_z = bump(x, y);
        positions.push([x, y, ground_z]);
        labels.push(PointClass::Ground);
        if rng.next_bool(0.3) {
            positions.push([x, y, ground_z + 2.0 + rng.next_f64()]);
            labels.push(PointClass::Vegetation);
        }
    }
    let cloud = PointCloud { positions, classification: labels, ..PointCloud::default() };
    let cell = 1.0;
    let origin = [-10.0, -10.0];
    let dtm = build_dtm(&cloud, cell, origin, 20, 20);
    let mut checked = 0;
    for y in 2..18 {
        for x in 2..18 {
            if let Some(z) = dtm.get(x, y) {
                let [wx, wy] = dtm.cell_center(x, y);
                let expected = bump(wx, wy);
                assert!((f64::from(z) - expected).abs() < 1.5, "dtm cell ({x},{y}) = {z} vs expected {expected}");
                checked += 1;
            }
        }
    }
    assert!(checked > 100, "expected many populated dtm cells, got {checked}");

    let mut holey = dtm;
    let hole_idx = holey.index(10, 10);
    let true_val = holey.values[hole_idx];
    holey.valid[hole_idx] = false;
    let filled = idw_fill(&holey, 5.0, 2.0);
    let filled_val = filled.get(10, 10).expect("idw should fill the deliberately emptied cell");
    assert!((f64::from(filled_val) - f64::from(true_val)).abs() < 1.5, "idw fill {filled_val} vs true {true_val}");
}
// #endregion 🔖️DsmTests

// #region 🔖️OrthoTests
#[test]
fn orthomosaic_blends_smoothly_across_camera_overlap() {
    let dsm = Raster { width: 40, height: 4, cell_size: 0.5, origin: [0.0, 0.0], values: vec![0.0; 160], valid: vec![true; 160] };
    let intr = Intrinsics { fx: 200.0, fy: 200.0, cx: 100.0, cy: 100.0, skew: 0.0, distortion: remodeling_camera::Distortion::None };
    // `p_cam = p_world + t` (identity rotation): a raster point at world `(wx, wy, 0)` needs
    // `t` chosen so `p_cam.z = t.z > 0` (in front of the camera) and `(wx, wy) + (t.x, t.y) = (0,
    // 0)` when the camera is centered over `(cx_world, cy_world)` — i.e. `t = (-cx_world,
    // -cy_world, height)`. Camera A centers over world x=5, camera B over world x=15, giving a
    // ~10m-wide overlap band in the middle of the 20m-wide raster.
    let pose_a = CameraPose(crate::lie::Se3 { r: So3::identity(), t: [-5.0, -1.0, 20.0] });
    let pose_b = CameraPose(crate::lie::Se3 { r: So3::identity(), t: [-15.0, -1.0, 20.0] });
    let mut img_a = ImageRgba8::new(200, 200);
    let mut img_b = ImageRgba8::new(200, 200);
    for px in img_a.data.as_chunks_mut::<4>().0.iter_mut() {
        *px = [220, 20, 20, 255];
    }
    for px in img_b.data.as_chunks_mut::<4>().0.iter_mut() {
        *px = [20, 20, 220, 255];
    }
    let cameras = vec![(pose_a, intr), (pose_b, intr)];
    let images = vec![img_a, img_b];
    let ortho = build_orthomosaic(&dsm, &cameras, &images);

    let mut max_step = 0_i32;
    let mut prev_r: Option<i32> = None;
    let mut saw_mid_tone = false;
    for x in 0..dsm.width {
        let idx = (2 * ortho.width + x) as usize * 4;
        let r = i32::from(ortho.data[idx]);
        if r > 60 && r < 190 {
            saw_mid_tone = true;
        }
        if let Some(pr) = prev_r {
            max_step = max_step.max((r - pr).abs());
        }
        prev_r = Some(r);
    }
    assert!(saw_mid_tone, "expected an intermediate blended tone across the overlap band");
    assert!(max_step < 150, "single-step color jump {max_step} too large for a feathered blend");
}
// #endregion 🔖️OrthoTests

// #region 🔖️ContoursTests
#[test]
fn circular_feature_extracts_closed_contour_with_expected_perimeter() {
    let n = 61_u32;
    let cell = 0.1;
    let origin = [-3.0, -3.0];
    let mut raster = Raster::new(n, n, cell, origin);
    let radius = 2.0_f64;
    for y in 0..n {
        for x in 0..n {
            let [wx, wy] = raster.cell_center(x, y);
            let z = 10.0 - (wx * wx + wy * wy).sqrt();
            raster.set(x, y, z as f32);
        }
    }
    let level = 10.0 - radius as f32;
    let contours = extract_contours(&raster, &[level]);
    let (_, polylines) = &contours[0];
    assert!(!polylines.is_empty(), "expected at least one contour polyline");
    let closed: Vec<&Vec<[f64; 2]>> = polylines.iter().filter(|p| p.len() > 3 && dist(p[0], p[p.len() - 1]) < 2.0 * cell).collect();
    assert!(!closed.is_empty(), "expected a closed loop among the extracted polylines");
    let loop_line = closed.iter().max_by_key(|p| p.len()).unwrap();
    let perimeter: f64 = loop_line.windows(2).map(|w| dist(w[0], w[1])).sum();
    let expected_perimeter = 2.0 * std::f64::consts::PI * radius;
    assert!((perimeter - expected_perimeter).abs() / expected_perimeter < 0.05, "perimeter {perimeter} vs expected {expected_perimeter}");
}

fn dist(a: [f64; 2], b: [f64; 2]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
}
// #endregion 🔖️ContoursTests

// #region 🔖️VolumeTests
#[test]
fn cut_fill_matches_planted_block_volume() {
    let n = 20_u32;
    let cell = 1.0;
    let mut before = Raster::new(n, n, cell, [0.0, 0.0]);
    let mut after = Raster::new(n, n, cell, [0.0, 0.0]);
    for y in 0..n {
        for x in 0..n {
            before.set(x, y, 0.0);
            let raised = (5..15).contains(&x) && (5..15).contains(&y);
            after.set(x, y, if raised { 2.0 } else { 0.0 });
        }
    }
    let report = cut_fill_volume(&before, &after);
    let expected = 10.0 * 10.0 * 2.0 * cell * cell;
    assert!((report.fill_m3 - expected).abs() / expected < 0.005, "fill {} vs expected {}", report.fill_m3, expected);
    assert!(report.cut_m3.abs() < 1e-9, "expected zero cut, got {}", report.cut_m3);
    assert!((report.net_m3 - expected).abs() / expected < 0.005);

    let vs_plane = cut_fill_volume_vs_plane(&after, 0.0);
    assert!((vs_plane.fill_m3 - expected).abs() / expected < 0.005);
}
// #endregion 🔖️VolumeTests

// #region 🔖️QualityTests
#[test]
fn quality_report_populates_sane_fields_from_real_sfm_output() {
    let scene = remodeling_sfm::synthetic_scene(4242, 5, 40, false);
    let scene_obs = remodeling_sfm::project_observations(&scene, 0.3, 0.0, 4242);
    let points = scene.points_world;
    let recon_cameras: Vec<(usize, CameraPose)> = scene.cameras.iter().enumerate().map(|(i, &(_, pose))| (i, pose)).collect();
    let intrinsics = scene.cameras[0].0;
    let recon = Reconstruction { cameras: recon_cameras, points: points.clone(), point_track_ids: (0..points.len()).collect(), intrinsics };

    let observations: Vec<(usize, usize, [f64; 2])> = scene_obs.iter().map(|o| (o.camera_index, o.point_index, o.pixel)).collect();
    assert!(observations.len() > points.len(), "expected multi-view observations in the fixture");

    let report = build_quality_report(&recon, &observations, None, None, None, None);
    assert!(report.reprojection_rms_px.is_finite() && report.reprojection_rms_px < 5.0, "rms {}", report.reprojection_rms_px);
    assert_eq!(report.per_camera_rms_px.len(), scene.cameras.len());
    assert!(report.per_camera_rms_px.iter().all(|r| r.is_finite()));
    assert!(report.track_stats.track_count > 0);
    assert!(report.track_stats.mean_track_length >= 1.0);
    assert!(!report.per_camera_covariance.is_empty(), "expected non-empty camera covariance diagonals");
    assert_eq!(report.per_point_sigma.len(), points.len());
    let finite_sigmas = report.per_point_sigma.iter().filter(|s| s.is_finite()).count();
    assert!(finite_sigmas > 0, "expected at least some well-observed points with finite sigma");
}
// #endregion 🔖️QualityTests
