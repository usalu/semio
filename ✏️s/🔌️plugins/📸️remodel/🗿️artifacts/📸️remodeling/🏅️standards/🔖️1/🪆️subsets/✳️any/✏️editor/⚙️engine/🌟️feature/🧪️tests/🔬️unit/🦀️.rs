
use super::*;
use remodeling_image::{build_pyramid, warp_affine};

// #region 🔖️Fixtures
fn corner_image(size: u32) -> ImageGray {
    let mut img = ImageGray::new(size, size);
    for y in 0..size {
        for x in 0..size {
            let half = size / 2;
            img.set(x, y, if x < half && y < half { 0.9 } else { 0.1 });
        }
    }
    img
}

fn textured_image(size: u32) -> ImageGray {
    let mut img = ImageGray::new(size, size);
    for v in img.data.iter_mut() {
        *v = 0.1;
    }
    let (step, square) = (8u32, 4u32);
    let mut y = 2;
    while y + square <= size {
        let mut x = 2;
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

fn lcg_next(state: &mut u32) -> f32 {
    *state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    (*state >> 8) as f32 / 16_777_216.0
}

fn lcg_texture(size: u32, seed: u32) -> ImageGray {
    let mut img = ImageGray::new(size, size);
    let mut state = seed;
    for v in img.data.iter_mut() {
        *v = lcg_next(&mut state);
    }
    img
}

fn smooth_texture(size: u32) -> ImageGray {
    let mut img = ImageGray::new(size, size);
    for y in 0..size {
        for x in 0..size {
            let (fx, fy) = (x as f32, y as f32);
            let v = 0.5 + 0.3 * (fx * 0.15).sin() * (fy * 0.12).cos() + 0.15 * (fx * 0.05 + fy * 0.07).sin();
            img.set(x, y, v);
        }
    }
    img
}
// #endregion 🔖️Fixtures

// #region 🔖️DetectTests
#[test]
fn fast_corners_detects_planted_corner_not_flat_region() {
    let img = corner_image(48);
    let corners = fast_corners(&img, 0.2);
    assert!(!corners.is_empty(), "expected at least one detected corner");
    assert!(corners.iter().any(|&(x, y, _)| (x as i32 - 24).abs() <= 3 && (y as i32 - 24).abs() <= 3), "expected a corner near the planted L-corner at (24, 24)");
    assert!(!corners.iter().any(|&(x, y, _)| x <= 6 && y <= 6), "flat interior pixels should not be reported as corners");
    let flat = ImageGray::new(48, 48);
    assert!(fast_corners(&flat, 0.2).is_empty(), "a flat image should have no corners");
}

#[test]
fn harris_response_is_low_on_flat_and_high_on_corner() {
    let mut flat = ImageGray::new(32, 32);
    for v in flat.data.iter_mut() {
        *v = 0.5;
    }
    let corner = corner_image(32);
    let flat_resp = harris_response(&flat, 0.04);
    let corner_resp = harris_response(&corner, 0.04);
    let max_flat = flat_resp.iter().copied().fold(f32::MIN, f32::max);
    let max_corner = corner_resp.iter().copied().fold(f32::MIN, f32::max);
    assert!(max_flat.abs() < 1e-6, "flat image should have ~0 harris response, got {max_flat}");
    assert!(max_corner > 1e-4, "corner response {max_corner} should be clearly positive");
    assert!(max_corner > max_flat * 10.0 + 1e-4, "corner response {max_corner} should exceed flat response {max_flat}");
}

#[test]
fn detect_orb_keypoints_returns_roughly_target_count_and_spread() {
    let img = textured_image(64);
    let pyramid = build_pyramid(&img, 3);
    let target = 40usize;
    let keypoints = detect_orb_keypoints(&pyramid, target);
    assert!(!keypoints.is_empty(), "expected some keypoints on a textured image");
    assert!(keypoints.len() as f64 <= target as f64 * 1.5, "too many keypoints: {}", keypoints.len());
    assert!(keypoints.len() as f64 >= target as f64 * 0.3, "too few keypoints: {}", keypoints.len());
    let min_x = keypoints.iter().map(|k| k.x).fold(f32::MAX, f32::min);
    let max_x = keypoints.iter().map(|k| k.x).fold(f32::MIN, f32::max);
    let min_y = keypoints.iter().map(|k| k.y).fold(f32::MAX, f32::min);
    let max_y = keypoints.iter().map(|k| k.y).fold(f32::MIN, f32::max);
    assert!(max_x - min_x > 20.0, "keypoints should spread across x, got span {}", max_x - min_x);
    assert!(max_y - min_y > 20.0, "keypoints should spread across y, got span {}", max_y - min_y);
}

#[test]
fn detect_harris_keypoints_clusters_near_known_corner() {
    let img = corner_image(48);
    let keypoints = detect_harris_keypoints(&img, 30);
    assert!(!keypoints.is_empty(), "expected some harris keypoints on a corner image");
    assert!(keypoints.iter().any(|kp| (kp.x - 24.0).abs() <= 4.0 && (kp.y - 24.0).abs() <= 4.0), "expected a harris keypoint near the planted L-corner at (24, 24), got {:?}", keypoints.iter().map(|kp| (kp.x, kp.y)).collect::<Vec<_>>());
    let flat = ImageGray::new(48, 48);
    assert!(detect_harris_keypoints(&flat, 30).is_empty(), "a flat image should have no harris keypoints");
}

#[test]
fn detect_orb_keypoints_returns_empty_for_empty_pyramid_or_zero_target() {
    let img = textured_image(32);
    let pyramid = build_pyramid(&img, 2);
    assert!(detect_orb_keypoints(&pyramid, 0).is_empty(), "zero target_count should yield no keypoints");
    let empty_pyramid = Pyramid { levels: Vec::new(), scale: 0.5 };
    assert!(detect_orb_keypoints(&empty_pyramid, 10).is_empty(), "an empty pyramid should yield no keypoints");
}

#[test]
fn detect_harris_keypoints_returns_empty_for_zero_target_or_empty_image() {
    let img = textured_image(16);
    assert!(detect_harris_keypoints(&img, 0).is_empty(), "zero target_count should yield no keypoints");
    assert!(detect_harris_keypoints(&ImageGray::new(0, 0), 10).is_empty(), "a zero-width/height image should yield no keypoints");
    assert!(detect_harris_keypoints(&ImageGray::new(5, 0), 10).is_empty(), "a zero-height image should yield no keypoints");
}

#[test]
fn shi_tomasi_grid_prefers_planted_corner_and_caps_per_cell() {
    let img = corner_image(48);
    let cell = 16u32;
    let results = shi_tomasi_grid(&img, cell, 1);
    assert!(!results.is_empty(), "expected some shi-tomasi candidates");
    assert!(results.len() <= 9, "with 3x3 cells and per_cell=1 at most 9 points should survive, got {}", results.len());
    let mut per_bucket: std::collections::HashMap<(u32, u32), usize> = std::collections::HashMap::new();
    for &(x, y, _) in &results {
        *per_bucket.entry((x / cell, y / cell)).or_default() += 1;
    }
    assert!(per_bucket.values().all(|&n| n <= 1), "each grid cell should keep at most per_cell=1 points");
    assert!(results.iter().any(|&(x, y, score)| (x as i32 - 24).abs() <= 8 && (y as i32 - 24).abs() <= 8 && score > 0.0), "expected a high min-eigenvalue point near the planted corner at (24, 24), got {results:?}");
}

#[test]
fn shi_tomasi_grid_handles_zero_cell_without_panicking() {
    let img = corner_image(8);
    let results = shi_tomasi_grid(&img, 0, 2);
    assert!(!results.is_empty(), "a zero cell size should be clamped to 1 and still return candidates");
    assert!(results.len() <= 64, "should not exceed the pixel count of an 8x8 image, got {}", results.len());
}
// #endregion 🔖️DetectTests

// #region 🔖️DescribeTests
#[test]
fn describe_orb_is_deterministic_and_self_hamming_zero() {
    let img = textured_image(48);
    let pyramid = build_pyramid(&img, 2);
    let kp = Keypoint { x: 24.0, y: 24.0, octave: 0, angle: 0.4, response: 1.0 };
    let d1 = describe_orb(&pyramid, &[kp]);
    let d2 = describe_orb(&pyramid, &[kp]);
    assert_eq!(d1[0], d2[0], "describing the same keypoint twice should give an identical descriptor");
    assert_eq!(hamming(&d1[0], &d1[0]), 0);
}
// #endregion 🔖️DescribeTests

// #region 🔖️MatchTests
#[test]
fn match_brute_recovers_known_translation_correspondences() {
    let size = 72u32;
    let img_a = lcg_texture(size, 123);
    let (tx, ty) = (3.0f32, 2.0f32);
    let m = [[1.0, 0.0, -tx], [0.0, 1.0, -ty]];
    let img_b = warp_affine(&img_a, &m, size, size);
    let pyr_a = build_pyramid(&img_a, 1);
    let pyr_b = build_pyramid(&img_b, 1);
    let kp_a = detect_orb_keypoints(&pyr_a, 80);
    let kp_b = detect_orb_keypoints(&pyr_b, 80);
    let desc_a = describe_orb(&pyr_a, &kp_a);
    let desc_b = describe_orb(&pyr_b, &kp_b);
    let matches = match_brute(&desc_a, &desc_b, 0.85, true);
    assert!(!matches.is_empty(), "expected some matches between the translated pair");
    let margin = 12.0f32;
    let mut checked = 0u32;
    let mut correct = 0u32;
    for mat in &matches {
        let a = kp_a[mat.a as usize];
        let b = kp_b[mat.b as usize];
        if a.x < margin || a.y < margin || a.x > size as f32 - margin || a.y > size as f32 - margin {
            continue;
        }
        checked += 1;
        if (b.x - a.x - tx).abs() < 2.0 && (b.y - a.y - ty).abs() < 2.0 {
            correct += 1;
        }
    }
    assert!(checked > 0, "expected some interior matches to check");
    assert!(f64::from(correct) / f64::from(checked) >= 0.9, "expected at least 90% correct correspondences, got {correct}/{checked}");
}

#[test]
fn match_brute_desc_b_empty_returns_empty() {
    let desc_a = [Descriptor256([0, 0, 0, 0])];
    assert!(match_brute(&desc_a, &[], 0.75, false).is_empty(), "matching against an empty pool should yield no matches");
}

#[test]
fn match_brute_single_candidate_bypasses_ratio_test() {
    let desc_a = [Descriptor256([0, 0, 0, 0]), Descriptor256([u64::MAX, 0, 0, 0])];
    let desc_b = [Descriptor256([0, 0, 0, 0])];
    let matches = match_brute(&desc_a, &desc_b, 0.01, false);
    assert_eq!(matches.len(), 2, "with only one candidate, second_dist is MAX and every query should bypass the ratio test");
    assert!(matches.iter().all(|m| m.b == 0));
}

#[test]
fn match_brute_mutual_filters_non_reciprocal_matches() {
    let a0 = Descriptor256([0, 0, 0, 0]);
    let a1 = Descriptor256([0xF, 0, 0, 0]);
    let b0 = Descriptor256([0, 0, 0, 0]);
    let b1 = Descriptor256([u64::MAX, u64::MAX, u64::MAX, u64::MAX]);
    let desc_a = [a0, a1];
    let desc_b = [b0, b1];
    let unfiltered = match_brute(&desc_a, &desc_b, 0.99, false);
    assert_eq!(unfiltered.len(), 2, "without mutual cross-check both a0 and a1 should match b0");
    let mutual = match_brute(&desc_a, &desc_b, 0.99, true);
    assert_eq!(mutual.len(), 1, "mutual cross-check should keep only the reciprocal a0<->b0 match");
    assert_eq!(mutual[0].a, 0);
    assert_eq!(mutual[0].b, 0);
}

#[test]
fn match_guided_epipolar_recovers_match_along_horizontal_line_and_filters_off_line_candidate() {
    let zero = Descriptor256([0, 0, 0, 0]);
    let kp_a = [Keypoint { x: 10.0, y: 20.0, octave: 0, angle: 0.0, response: 1.0 }];
    let desc_a = [zero];
    let kp_b = [
        Keypoint { x: 12.0, y: 20.0, octave: 0, angle: 0.0, response: 1.0 },
        Keypoint { x: 50.0, y: 20.0, octave: 0, angle: 0.0, response: 1.0 },
        Keypoint { x: 5.0, y: 100.0, octave: 0, angle: 0.0, response: 1.0 },
        Keypoint { x: 20.0, y: 23.9, octave: 0, angle: 0.0, response: 1.0 },
    ];
    let desc_b = [zero, Descriptor256([u64::MAX, 0, 0, 0]), zero, zero];
    // F for a pure horizontal-translation stereo rig: epipolar lines are horizontal (Y = y).
    let f_matrix = [[0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0]];
    let matches = match_guided_epipolar(&kp_a, &desc_a, &kp_b, &desc_b, &f_matrix, 3.0);
    assert_eq!(matches.len(), 1, "expected exactly one match, got {matches:?}");
    assert_eq!(matches[0].a, 0);
    assert_eq!(matches[0].b, 0, "the off-line candidate at index 3 should be excluded despite an identical descriptor");
    assert_eq!(matches[0].distance, 0);
}

#[test]
fn match_guided_epipolar_handles_vertical_epipolar_line() {
    let zero = Descriptor256([0, 0, 0, 0]);
    let kp_a = [Keypoint { x: 15.0, y: 5.0, octave: 0, angle: 0.0, response: 1.0 }];
    let desc_a = [zero];
    let kp_b = [Keypoint { x: 15.0, y: 10.0, octave: 0, angle: 0.0, response: 1.0 }, Keypoint { x: 15.0, y: 80.0, octave: 0, angle: 0.0, response: 1.0 }];
    let desc_b = [zero, Descriptor256([u64::MAX, 0, 0, 0])];
    // F for a pure vertical-translation stereo rig: epipolar lines are vertical (X = x), exercising the y-stepping branch.
    let f_matrix = [[0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [-1.0, 0.0, 0.0]];
    let matches = match_guided_epipolar(&kp_a, &desc_a, &kp_b, &desc_b, &f_matrix, 3.0);
    assert_eq!(matches.len(), 1, "expected exactly one match along the vertical epipolar line, got {matches:?}");
    assert_eq!(matches[0].a, 0);
    assert_eq!(matches[0].b, 0);
}

#[test]
fn match_guided_epipolar_returns_empty_for_empty_kp_b_and_degenerate_matrix() {
    let kp_a = [Keypoint { x: 10.0, y: 20.0, octave: 0, angle: 0.0, response: 1.0 }];
    let desc_a = [Descriptor256([0, 0, 0, 0])];
    let f_matrix = [[0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0]];
    assert!(match_guided_epipolar(&kp_a, &desc_a, &[], &[], &f_matrix, 3.0).is_empty(), "an empty kp_b should yield no matches");
    let kp_b = [Keypoint { x: 12.0, y: 20.0, octave: 0, angle: 0.0, response: 1.0 }];
    let desc_b = [Descriptor256([0, 0, 0, 0])];
    let degenerate_f = [[0.0; 3]; 3];
    assert!(match_guided_epipolar(&kp_a, &desc_a, &kp_b, &desc_b, &degenerate_f, 3.0).is_empty(), "a degenerate (all-zero) F matrix should yield no matches");
}

#[test]
fn match_zncc_fallback_finds_correct_correspondence_within_radius() {
    let size = 64u32;
    let img_a = lcg_texture(size, 55);
    let shift = 3.0f32;
    let m = [[1.0, 0.0, -shift], [0.0, 1.0, -shift]];
    let img_b = warp_affine(&img_a, &m, size, size);
    let kp_a = [Keypoint { x: 30.0, y: 32.0, octave: 0, angle: 0.0, response: 1.0 }];
    let kp_b = [Keypoint { x: 33.0, y: 35.0, octave: 0, angle: 0.0, response: 1.0 }];
    let matches = match_zncc_fallback(&img_a, &kp_a, &img_b, &kp_b, 6.0);
    assert_eq!(matches.len(), 1, "expected the correctly-shifted candidate to match, got {matches:?}");
    assert_eq!(matches[0].a, 0);
    assert_eq!(matches[0].b, 0);
    assert!(matches[0].distance < 100, "expected a high zncc correlation (low distance), got {}", matches[0].distance);
}

#[test]
fn match_zncc_fallback_returns_empty_for_empty_kp_b() {
    let img = lcg_texture(32, 1);
    let kp_a = [Keypoint { x: 16.0, y: 16.0, octave: 0, angle: 0.0, response: 1.0 }];
    assert!(match_zncc_fallback(&img, &kp_a, &img, &[], 5.0).is_empty(), "an empty kp_b should yield no matches");
}

#[test]
fn match_zncc_fallback_no_match_when_correlation_below_threshold() {
    let img_a = lcg_texture(48, 1);
    let img_b = lcg_texture(48, 2);
    let kp_a = [Keypoint { x: 24.0, y: 24.0, octave: 0, angle: 0.0, response: 1.0 }];
    let kp_b = [Keypoint { x: 24.0, y: 24.0, octave: 0, angle: 0.0, response: 1.0 }];
    assert!(match_zncc_fallback(&img_a, &kp_a, &img_b, &kp_b, 5.0).is_empty(), "uncorrelated noise patches should fall below the zncc threshold");
}
// #endregion 🔖️MatchTests

// #region 🔖️FlowTests
#[test]
fn klt_track_recovers_known_shift_and_flags_out_of_bounds() {
    let size = 48u32;
    let img_a = smooth_texture(size);
    let shift = 2.3f32;
    let m = [[1.0, 0.0, -shift], [0.0, 1.0, -shift]];
    let img_b = warp_affine(&img_a, &m, size, size);
    let pyr_a = build_pyramid(&img_a, 3);
    let pyr_b = build_pyramid(&img_b, 3);
    let points = [(12.0, 12.0), (24.0, 20.0), (30.0, 30.0), (46.0, 46.0)];
    let tracked = klt_track(&pyr_a, &pyr_b, &points, 5, 20);
    for (i, tp) in tracked.iter().enumerate().take(3) {
        assert!(tp.valid, "point {i} should track validly");
        assert!((tp.x - (points[i].0 + shift)).abs() < 0.1, "x error too high for point {i}: {}", tp.x);
        assert!((tp.y - (points[i].1 + shift)).abs() < 0.1, "y error too high for point {i}: {}", tp.y);
    }
    assert!(!tracked[3].valid, "a point leaving the image bounds after translation should be invalid");
}

#[test]
fn forward_backward_prune_invalidates_degenerate_track_and_keeps_good_one() {
    let size = 48u32;
    let img_a = smooth_texture(size);
    let shift = 1.5f32;
    let m = [[1.0, 0.0, -shift], [0.0, 1.0, -shift]];
    let img_b = warp_affine(&img_a, &m, size, size);
    let pyr_a = build_pyramid(&img_a, 3);
    let pyr_b = build_pyramid(&img_b, 3);
    let points = [(20.0, 20.0), (5.0, 5.0)];
    let mut tracked = klt_track(&pyr_a, &pyr_b, &points, 5, 20);
    assert!(tracked[0].valid, "well-textured point should track validly first");
    tracked[1] = TrackPoint { x: 5.0, y: 40.0, valid: true, error: 0.0 };
    forward_backward_prune(&pyr_a, &pyr_b, &points, &mut tracked, 5, 20, 0.5);
    assert!(tracked[0].valid, "well-tracked point should remain valid after fb-pruning");
    assert!(!tracked[1].valid, "an implausible/degenerate track should be invalidated by the fb round-trip check");
}

#[test]
fn forward_backward_prune_skips_already_invalid_points() {
    let size = 32u32;
    let img = smooth_texture(size);
    let pyr = build_pyramid(&img, 2);
    let points = [(10.0, 10.0)];
    let original = TrackPoint { x: 999.0, y: 999.0, valid: false, error: 5.0 };
    let mut tracked = [original];
    forward_backward_prune(&pyr, &pyr, &points, &mut tracked, 5, 20, 0.5);
    assert_eq!(tracked[0], original, "an already-invalid track point should be left untouched");
}

#[test]
fn klt_track_returns_invalid_for_empty_pyramid_levels() {
    let empty_pyr = Pyramid { levels: Vec::new(), scale: 0.5 };
    let other_pyr = Pyramid { levels: vec![ImageGray::new(8, 8)], scale: 0.5 };
    let tracked = klt_track(&empty_pyr, &other_pyr, &[(1.0, 1.0)], 3, 5);
    assert_eq!(tracked.len(), 1);
    assert!(!tracked[0].valid, "tracking with zero shared pyramid levels should be invalid");
    assert_eq!(tracked[0].x, 1.0);
    assert_eq!(tracked[0].y, 1.0);
    assert_eq!(tracked[0].error, f32::INFINITY);
}

#[test]
fn klt_track_flags_invalid_on_singular_flat_region() {
    let flat = ImageGray::new(32, 32);
    let pyr = build_pyramid(&flat, 2);
    let tracked = klt_track(&pyr, &pyr, &[(10.0, 10.0)], 5, 10);
    assert!(!tracked[0].valid, "a flat/textureless region has a near-singular structure tensor and should be invalid");
}
// #endregion 🔖️FlowTests

// #region 🔖️AkazeTests
fn blob_image(size: u32) -> ImageGray {
    let mut img = ImageGray::new(size, size);
    for v in img.data.iter_mut() {
        *v = 0.1;
    }
    let (step, square) = (16u32, 8u32);
    let mut y = 4;
    while y + square <= size {
        let mut x = 4;
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
    gaussian_blur(&img, 1.5)
}

struct TestTransform {
    cx: f32,
    cy: f32,
    theta: f32,
    scale: f32,
}

fn rotate_scale_matrix(t: &TestTransform) -> [[f32; 3]; 2] {
    let (sin_t, cos_t) = t.theta.sin_cos();
    let inv_s = 1.0 / t.scale;
    [[cos_t * inv_s, sin_t * inv_s, t.cx - t.cx * cos_t * inv_s - t.cy * sin_t * inv_s], [-sin_t * inv_s, cos_t * inv_s, t.cy + t.cx * sin_t * inv_s - t.cy * cos_t * inv_s]]
}

fn transform_point(t: &TestTransform, x: f32, y: f32) -> (f32, f32) {
    let (sin_t, cos_t) = t.theta.sin_cos();
    let (dx, dy) = (x - t.cx, y - t.cy);
    (t.cx + t.scale * (cos_t * dx - sin_t * dy), t.cy + t.scale * (sin_t * dx + cos_t * dy))
}

fn repeatability_fraction(base: &[Keypoint], transformed: &[Keypoint], t: &TestTransform, size: u32, margin: f32, tolerance: f32) -> f64 {
    let mut checked = 0u32;
    let mut matched = 0u32;
    for kp in base {
        if kp.x < margin || kp.y < margin || kp.x > size as f32 - margin || kp.y > size as f32 - margin {
            continue;
        }
        let (px, py) = transform_point(t, kp.x, kp.y);
        if px < 0.0 || py < 0.0 || px > size as f32 || py > size as f32 {
            continue;
        }
        checked += 1;
        if transformed.iter().any(|c| (c.x - px).hypot(c.y - py) <= tolerance) {
            matched += 1;
        }
    }
    if checked == 0 { 0.0 } else { f64::from(matched) / f64::from(checked) }
}

#[test]
fn akaze_repeatability_is_at_least_orb_repeatability_under_scale_and_rotation() {
    let size = 96u32;
    let img_a = blob_image(size);
    let transform = TestTransform { cx: size as f32 / 2.0, cy: size as f32 / 2.0, theta: 12f32.to_radians(), scale: 1.25 };
    let m = rotate_scale_matrix(&transform);
    let img_b = warp_affine(&img_a, &m, size, size);
    let margin = 10.0f32;
    let tolerance = 2.5f32;

    let pyr_a = build_pyramid(&img_a, 3);
    let pyr_b = build_pyramid(&img_b, 3);
    let orb_a = detect_orb_keypoints(&pyr_a, 80);
    let orb_b = detect_orb_keypoints(&pyr_b, 80);
    let orb_repeatability = repeatability_fraction(&orb_a, &orb_b, &transform, size, margin, tolerance);

    let scale_space_a = build_akaze_scale_space(&img_a, 3, 4);
    let scale_space_b = build_akaze_scale_space(&img_b, 3, 4);
    let akaze_a = detect_akaze_keypoints(&scale_space_a, 80);
    let akaze_b = detect_akaze_keypoints(&scale_space_b, 80);
    let akaze_repeatability = repeatability_fraction(&akaze_a, &akaze_b, &transform, size, margin, tolerance);

    assert!(!orb_a.is_empty() && !akaze_a.is_empty(), "both detectors should find keypoints on the base image");
    assert!(akaze_repeatability > 0.0, "expected some AKAZE keypoints to survive the scale+rotation transform, got {akaze_repeatability:.3}");
    assert!(akaze_repeatability >= orb_repeatability, "expected AKAZE repeatability ({akaze_repeatability:.3}) to be at least ORB's repeatability ({orb_repeatability:.3}) under a 1.25x scale + 12deg rotation");
}

#[test]
fn describe_akaze_is_deterministic_and_self_hamming_zero() {
    let img = textured_image(64);
    let scale_space = build_akaze_scale_space(&img, 3, 4);
    let kp = Keypoint { x: 32.0, y: 32.0, octave: 0, angle: 0.3, response: 1.0 };
    let d1 = describe_akaze(&scale_space, &[kp]);
    let d2 = describe_akaze(&scale_space, &[kp]);
    assert_eq!(d1[0], d2[0], "describing the same AKAZE keypoint twice should give an identical M-LDB descriptor");
    assert_eq!(d1[0].hamming_distance(&d1[0]), 0);
}

#[test]
fn fed_tau_schedule_returns_empty_for_non_positive_time() {
    assert!(fed_tau_schedule(0.0, AKAZE_FED_TAU_MAX).is_empty());
    assert!(fed_tau_schedule(-1.0, AKAZE_FED_TAU_MAX).is_empty());
    assert!(!fed_tau_schedule(1.0, AKAZE_FED_TAU_MAX).is_empty(), "a positive evolution time should produce at least one FED step");
}

#[test]
fn diffusion_step_returns_input_clone_for_zero_sized_image() {
    let empty = ImageGray { width: 0, height: 0, data: Vec::new() };
    let out = diffusion_step(&empty, &[], 0.1);
    assert_eq!(out, empty, "a zero-width/height image should pass through diffusion_step unchanged");
}

#[test]
fn akaze_grid_top_k_returns_empty_for_no_candidates_or_zero_target() {
    assert!(akaze_grid_top_k(Vec::new(), 64, 64, 16, 10).is_empty(), "no candidates should yield no selection");
    let candidates = vec![AkazeCandidate { x: 1.0, y: 1.0, response: 1.0, level: 0 }];
    assert!(akaze_grid_top_k(candidates, 64, 64, 16, 0).is_empty(), "a zero target_count should yield no selection");
}

#[test]
fn quadratic_refine_2d_returns_zero_offset_for_singular_hessian() {
    let response = vec![1.0f32; 25];
    let (ox, oy) = quadratic_refine_2d(&response, 5, 2, 2);
    assert_eq!((ox, oy), (0.0, 0.0), "a perfectly flat response has a singular Hessian and should refine to a zero offset");
}
// #endregion 🔖️AkazeTests
