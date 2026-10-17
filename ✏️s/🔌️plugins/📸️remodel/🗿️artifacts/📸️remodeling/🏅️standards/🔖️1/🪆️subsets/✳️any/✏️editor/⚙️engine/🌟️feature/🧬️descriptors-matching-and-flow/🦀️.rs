//! 🧬️ Descriptors, matching and flow of the remodeling engine features: the oriented rBRIEF description, the brute-force, epipolar-guided and ZNCC matching and the pyramidal Lucas-Kanade tracking. See [`super`].

use super::*;

// #region 🔖️Describe
const BRIEF_PATCH_RADIUS: u32 = 15;

const BRIEF_SEED: u64 = 0xB817_ED0B_5D3E_C0DE;

const BRIEF_BLUR_SIGMA: f32 = 1.2;

/// 🧬️ 256-bit rotation-aware BRIEF descriptor packed into 4 `u64` words (bit `i` lives at word `i / 64`, position `i % 64`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct Descriptor256(pub [u64; 4]);

impl Descriptor256 {
    /// 📏️ Hamming distance to `other`, as an inherent method for callers that prefer `a.hamming_distance(&b)` over the free [`hamming`] function; both are the same population-count-of-XOR computation, so AKAZE M-LDB descriptors from [`describe_akaze`] (emitted into this same type) match through either.
    pub fn hamming_distance(&self, other: &Descriptor256) -> u32 {
        hamming(self, other)
    }
}

/// 📏️ Hamming distance between two descriptors: population count of the XOR of each of the 4 words, summed.
/// <https://en.wikipedia.org/wiki/Hamming_distance>
pub fn hamming(a: &Descriptor256, b: &Descriptor256) -> u32 {
    a.0.iter().zip(b.0.iter()).map(|(&x, &y)| (x ^ y).count_ones()).sum()
}

type BriefOffsetPair = ((i32, i32), (i32, i32));

fn brief_pattern() -> &'static [BriefOffsetPair; 256] {
    static PATTERN: std::sync::OnceLock<[BriefOffsetPair; 256]> = std::sync::OnceLock::new();
    PATTERN.get_or_init(|| {
        let mut rng = geometry::random::Rng::from_seed(BRIEF_SEED);
        let span = 2 * BRIEF_PATCH_RADIUS as u64 + 1;
        std::array::from_fn(|_| {
            let mut next_offset = || rng.next_range(0, span) as i32 - BRIEF_PATCH_RADIUS as i32;
            ((next_offset(), next_offset()), (next_offset(), next_offset()))
        })
    })
}

/// 🧬️ Oriented rBRIEF description: for each keypoint, extracts a `(2 BRIEF_PATCH_RADIUS + 1)^2` patch steered by the keypoint's angle (rotation folded into the bilinear sampling grid of [`extract_patch`]), gaussian pre-blurs it to damp noise, then sets bit `i` when the blurred intensity at the pattern's first fixed point-pair offset is less than at its second. The 256 offset pairs are generated once from a fixed published seed via `geometry::random`, so the pattern — and hence every descriptor — is identical across builds and runs.
/// <https://en.wikipedia.org/wiki/Oriented_FAST_and_rotated_BRIEF>
pub fn describe_orb(pyramid: &Pyramid, keypoints: &[Keypoint]) -> Vec<Descriptor256> {
    let pattern = brief_pattern();
    let side = 2 * BRIEF_PATCH_RADIUS + 1;
    keypoints
        .iter()
        .map(|kp| {
            let level_idx = (kp.octave as usize).min(pyramid.levels.len().saturating_sub(1));
            let level = &pyramid.levels[level_idx];
            let patch = extract_patch(level, kp.x, kp.y, BRIEF_PATCH_RADIUS, kp.angle);
            let patch_img = ImageGray { width: side, height: side, data: patch.data };
            let blurred = gaussian_blur(&patch_img, BRIEF_BLUR_SIGMA);
            let mut words = [0u64; 4];
            for (bit, &((ax, ay), (bx, by))) in pattern.iter().enumerate() {
                let sample_a = blurred.get((ax + BRIEF_PATCH_RADIUS as i32) as u32, (ay + BRIEF_PATCH_RADIUS as i32) as u32);
                let sample_b = blurred.get((bx + BRIEF_PATCH_RADIUS as i32) as u32, (by + BRIEF_PATCH_RADIUS as i32) as u32);
                if sample_a < sample_b {
                    words[bit / 64] |= 1u64 << (bit % 64);
                }
            }
            Descriptor256(words)
        })
        .collect()
}
// #endregion 🔖️Describe

// #region 🔖️Match
const MATCH_EPIPOLAR_RATIO: f32 = 0.75;

const MATCH_ZNCC_THRESHOLD: f32 = 0.6;

const MATCH_ZNCC_PATCH_RADIUS: u32 = 7;

/// 🔗️ A correspondence between descriptor/keypoint index `a` (in the first set) and `b` (in the second set), with the Hamming (or Hamming-equivalent) distance that supported it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct Match {
    pub a: u32,
    pub b: u32,
    pub distance: u32,
}

fn best_and_second_hamming(query: &Descriptor256, pool: &[Descriptor256]) -> (u32, u32, u32) {
    let mut best = (u32::MAX, u32::MAX);
    let mut second = u32::MAX;
    for (j, candidate) in pool.iter().enumerate() {
        let d = hamming(query, candidate);
        if d < best.0 {
            second = best.0;
            best = (d, j as u32);
        } else if d < second {
            second = d;
        }
    }
    (best.0, best.1, second)
}

/// 🤝️ Brute-force Hamming matching from `desc_a` into `desc_b` with Lowe's ratio test (best distance must be less than `ratio` times the second-best), optionally cross-checked so a match only survives when it is also `desc_a`'s best match for its own best-in-`desc_b` partner.
/// <https://en.wikipedia.org/wiki/Nearest_neighbor_search#Nearest_neighbor_algorithms_in_high-dimensional_spaces>
pub fn match_brute(desc_a: &[Descriptor256], desc_b: &[Descriptor256], ratio: f32, mutual: bool) -> Vec<Match> {
    if desc_b.is_empty() {
        return Vec::new();
    }
    let mut matches = Vec::new();
    for (i, da) in desc_a.iter().enumerate() {
        let (best_dist, best_idx, second_dist) = best_and_second_hamming(da, desc_b);
        if second_dist == u32::MAX || (best_dist as f32) < ratio * (second_dist as f32) {
            matches.push(Match { a: i as u32, b: best_idx, distance: best_dist });
        }
    }
    if mutual {
        matches.retain(|m| {
            let (_, back_idx, _) = best_and_second_hamming(&desc_b[m.b as usize], desc_a);
            back_idx == m.a
        });
    }
    matches
}

fn epipolar_line_candidates(grid: &semio_s_artifact_remodel_remodeling_spatial::standards::v1::subsets::any::schema::spatial_internals::Grid2, line: (f64, f64, f64), bounds: (f32, f32, f32, f32), step: f64) -> Vec<u32> {
    let (l0, l1, l2) = line;
    let (min_x, max_x, min_y, max_y) = bounds;
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    let mut collect_at = |p: [f64; 2]| {
        for id in grid.neighbors9(p) {
            if seen.insert(id) {
                out.push(id);
            }
        }
    };
    if l1.abs() >= l0.abs() {
        let mut x = f64::from(min_x);
        while x <= f64::from(max_x) {
            collect_at([x, -(l0 * x + l2) / l1]);
            x += step;
        }
    } else {
        let mut y = f64::from(min_y);
        while y <= f64::from(max_y) {
            collect_at([-(l1 * y + l2) / l0, y]);
            y += step;
        }
    }
    out
}

/// 🧭️ Epipolar-guided matching: for each keypoint in `kp_a`, computes its epipolar line `l = F [x, y, 1]` in image B, walks that line (stepping through a [`crate::standards::v1::subsets::any::schema::spatial_internals::Grid2`] bucketed over `kp_b` for efficiency) to gather candidates within `band_px` of the line by point-to-line distance, then runs the same Lowe's-ratio matching as [`match_brute`] restricted to those candidates.
/// <https://en.wikipedia.org/wiki/Epipolar_geometry>
pub fn match_guided_epipolar(kp_a: &[Keypoint], desc_a: &[Descriptor256], kp_b: &[Keypoint], desc_b: &[Descriptor256], f_matrix: &[[f64; 3]; 3], band_px: f32) -> Vec<Match> {
    if kp_b.is_empty() {
        return Vec::new();
    }
    let cell = f64::from(band_px.max(1.0));
    let mut grid = semio_s_artifact_remodel_remodeling_spatial::standards::v1::subsets::any::schema::spatial_internals::Grid2::new(cell);
    for (j, kp) in kp_b.iter().enumerate() {
        grid.insert([f64::from(kp.x), f64::from(kp.y)], j as u32);
    }
    let bounds = kp_b.iter().fold((f32::MAX, f32::MIN, f32::MAX, f32::MIN), |(lox, hix, loy, hiy), kp| (lox.min(kp.x), hix.max(kp.x), loy.min(kp.y), hiy.max(kp.y)));
    let mut matches = Vec::new();
    for (i, (kpa, da)) in kp_a.iter().zip(desc_a.iter()).enumerate() {
        let x = f64::from(kpa.x);
        let y = f64::from(kpa.y);
        let l0 = f_matrix[0][0] * x + f_matrix[0][1] * y + f_matrix[0][2];
        let l1 = f_matrix[1][0] * x + f_matrix[1][1] * y + f_matrix[1][2];
        let l2 = f_matrix[2][0] * x + f_matrix[2][1] * y + f_matrix[2][2];
        let norm = l0.hypot(l1);
        if norm < 1e-12 {
            continue;
        }
        let candidates = epipolar_line_candidates(&grid, (l0, l1, l2), bounds, cell);
        let mut best = (u32::MAX, u32::MAX);
        let mut second = u32::MAX;
        for j in candidates {
            let kpb = &kp_b[j as usize];
            let dist_line = (l0 * f64::from(kpb.x) + l1 * f64::from(kpb.y) + l2).abs() / norm;
            if dist_line > f64::from(band_px) {
                continue;
            }
            let d = hamming(da, &desc_b[j as usize]);
            if d < best.0 {
                second = best.0;
                best = (d, j);
            } else if d < second {
                second = d;
            }
        }
        if best.1 != u32::MAX && (second == u32::MAX || (best.0 as f32) < MATCH_EPIPOLAR_RATIO * (second as f32)) {
            matches.push(Match { a: i as u32, b: best.1, distance: best.0 });
        }
    }
    matches
}

/// 🩹️ ZNCC patch-correlation fallback for pairs where binary descriptors fail (cross-sensor or low-texture imagery): for each keypoint in `kp_a`, gathers `kp_b` candidates within `search_radius` via a [`crate::standards::v1::subsets::any::schema::spatial_internals::Grid2`], scores each with [`zncc`] over patches from [`extract_patch`], and keeps the best candidate above a `0.6` correlation floor. The reported `distance` is `round((1 - zncc) * 1000)`, a monotone integer proxy so lower still means a better match.
pub fn match_zncc_fallback(img_a: &ImageGray, kp_a: &[Keypoint], img_b: &ImageGray, kp_b: &[Keypoint], search_radius: f32) -> Vec<Match> {
    if kp_b.is_empty() {
        return Vec::new();
    }
    let cell = f64::from(search_radius.max(1.0));
    let mut grid = semio_s_artifact_remodel_remodeling_spatial::standards::v1::subsets::any::schema::spatial_internals::Grid2::new(cell);
    for (j, kp) in kp_b.iter().enumerate() {
        grid.insert([f64::from(kp.x), f64::from(kp.y)], j as u32);
    }
    let mut matches = Vec::new();
    for (i, kpa) in kp_a.iter().enumerate() {
        let patch_a = extract_patch(img_a, kpa.x, kpa.y, MATCH_ZNCC_PATCH_RADIUS, 0.0);
        let mut best_score = MATCH_ZNCC_THRESHOLD;
        let mut best_idx = u32::MAX;
        for j in grid.neighbors9([f64::from(kpa.x), f64::from(kpa.y)]) {
            let kpb = &kp_b[j as usize];
            if (kpb.x - kpa.x).hypot(kpb.y - kpa.y) > search_radius {
                continue;
            }
            let patch_b = extract_patch(img_b, kpb.x, kpb.y, MATCH_ZNCC_PATCH_RADIUS, 0.0);
            let score = zncc(&patch_a, &patch_b);
            if score > best_score {
                best_score = score;
                best_idx = j;
            }
        }
        if best_idx != u32::MAX {
            matches.push(Match { a: i as u32, b: best_idx, distance: ((1.0 - best_score) * 1000.0).round() as u32 });
        }
    }
    matches
}
// #endregion 🔖️Match

// #region 🔖️Flow
const KLT_SINGULAR_EPSILON: f32 = 1e-6;

const KLT_CONVERGE_EPS: f32 = 1e-4;

/// 🌊️ One point's optical-flow track: subpixel position in the target frame, validity, and the final RMS intensity-residual error over its window.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrackPoint {
    pub x: f32,
    pub y: f32,
    pub valid: bool,
    pub error: f32,
}

fn klt_lucas_kanade_level(level_a: &ImageGray, level_b: &ImageGray, lx: f32, ly: f32, initial_disp: (f32, f32), window_radius: i32, max_iters: usize) -> ((f32, f32), bool, f32) {
    let r = window_radius;
    let mut window = Vec::with_capacity(((2 * r + 1) * (2 * r + 1)) as usize);
    for dy in -r..=r {
        for dx in -r..=r {
            let (fx, fy) = (dx as f32, dy as f32);
            let t = level_a.sample(lx + fx, ly + fy);
            let gx = (level_a.sample(lx + fx + 1.0, ly + fy) - level_a.sample(lx + fx - 1.0, ly + fy)) * 0.5;
            let gy = (level_a.sample(lx + fx, ly + fy + 1.0) - level_a.sample(lx + fx, ly + fy - 1.0)) * 0.5;
            window.push((fx, fy, t, gx, gy));
        }
    }
    let mut sxx = 0.0f32;
    let mut sxy = 0.0f32;
    let mut syy = 0.0f32;
    for &(_, _, _, gx, gy) in &window {
        sxx += gx * gx;
        sxy += gx * gy;
        syy += gy * gy;
    }
    let det = sxx * syy - sxy * sxy;
    if det.abs() < KLT_SINGULAR_EPSILON {
        return (initial_disp, false, f32::INFINITY);
    }
    let mut disp = initial_disp;
    let mut error = f32::INFINITY;
    for _ in 0..max_iters {
        let mut bx = 0.0f32;
        let mut by = 0.0f32;
        let mut sum_sq = 0.0f32;
        for &(fx, fy, t, gx, gy) in &window {
            let sample_x = lx + disp.0 + fx;
            let sample_y = ly + disp.1 + fy;
            if sample_x < 0.0 || sample_y < 0.0 || sample_x > (level_b.width - 1) as f32 || sample_y > (level_b.height - 1) as f32 {
                return (disp, false, f32::INFINITY);
            }
            let residual = level_b.sample(sample_x, sample_y) - t;
            bx += gx * residual;
            by += gy * residual;
            sum_sq += residual * residual;
        }
        let delta_x = (sxy * by - syy * bx) / det;
        let delta_y = (sxy * bx - sxx * by) / det;
        disp.0 += delta_x;
        disp.1 += delta_y;
        error = (sum_sq / window.len() as f32).sqrt();
        if delta_x.hypot(delta_y) < KLT_CONVERGE_EPS {
            break;
        }
    }
    (disp, true, error)
}

fn klt_track_single(pyr_a: &Pyramid, pyr_b: &Pyramid, x0: f32, y0: f32, window_radius: i32, max_iters: usize) -> TrackPoint {
    let n_levels = pyr_a.levels.len().min(pyr_b.levels.len());
    if n_levels == 0 {
        return TrackPoint { x: x0, y: y0, valid: false, error: f32::INFINITY };
    }
    let mut disp = (0.0f32, 0.0f32);
    let mut ok = true;
    let mut error = f32::INFINITY;
    for level in (0..n_levels).rev() {
        let scale = pyr_a.scale.powi(level as i32);
        let level_a = &pyr_a.levels[level];
        let level_b = &pyr_b.levels[level];
        let (level_disp, level_ok, level_error) = klt_lucas_kanade_level(level_a, level_b, x0 * scale, y0 * scale, disp, window_radius, max_iters);
        disp = level_disp;
        ok = level_ok;
        error = level_error;
        if level > 0 {
            disp = (disp.0 / pyr_a.scale, disp.1 / pyr_a.scale);
        }
    }
    let (fx, fy) = (x0 + disp.0, y0 + disp.1);
    let finest = &pyr_a.levels[0];
    let in_bounds = fx >= 0.0 && fy >= 0.0 && fx < finest.width as f32 && fy < finest.height as f32;
    TrackPoint { x: fx, y: fy, valid: ok && in_bounds, error }
}

/// 🌊️ Pyramidal Lucas-Kanade tracking of `points` (given in `pyr_a`'s level-0 coordinates) from `pyr_a` into `pyr_b`: coarse-to-fine over shared pyramid levels, each level running Gauss-Newton on a 2-DoF translation using the template's gradient structure tensor (an inverse-compositional simplification, since a pure translation warp has a constant Jacobian) and the image-difference residual over a `(2 window_radius + 1)^2` window. A point is marked invalid if its structure tensor is near-singular, a sample step leaves the image bounds, or the final position lands outside the level-0 image.
/// <https://en.wikipedia.org/wiki/Lucas%E2%80%93Kanade_method>
pub fn klt_track(pyr_a: &Pyramid, pyr_b: &Pyramid, points: &[(f32, f32)], window_radius: i32, max_iters: usize) -> Vec<TrackPoint> {
    points.iter().map(|&(x0, y0)| klt_track_single(pyr_a, pyr_b, x0, y0, window_radius, max_iters)).collect()
}

/// ↩️ Forward-backward consistency pruning: re-tracks each already-valid `tracked` point from `pyr_b` back into `pyr_a` and invalidates it (in place) if the backward track itself fails or its round-trip distance to the original `points` entry exceeds `max_fb_error`.
/// <https://en.wikipedia.org/wiki/Lucas%E2%80%93Kanade_method>
pub fn forward_backward_prune(pyr_a: &Pyramid, pyr_b: &Pyramid, points: &[(f32, f32)], tracked: &mut [TrackPoint], window_radius: i32, max_iters: usize, max_fb_error: f32) {
    for (tp, &(ox, oy)) in tracked.iter_mut().zip(points.iter()) {
        if !tp.valid {
            continue;
        }
        let back = klt_track_single(pyr_b, pyr_a, tp.x, tp.y, window_radius, max_iters);
        let fb_error = (back.x - ox).hypot(back.y - oy);
        if !back.valid || fb_error > max_fb_error {
            tp.valid = false;
        }
    }
}
// #endregion 🔖️Flow
