//! 🔎️ Keypoint detectors of the remodeling engine features: the FAST and Harris corner responses, the oriented-FAST and Harris detectors and the resumable bounded detection preparation. See [`super`].

use super::*;

// #region 🔖️Detect
const FAST_CIRCLE_OFFSETS: [(i32, i32); 16] = [(0, -3), (1, -3), (2, -2), (3, -1), (3, 0), (3, 1), (2, 2), (1, 3), (0, 3), (-1, 3), (-2, 2), (-3, 1), (-3, 0), (-3, -1), (-2, -2), (-1, -3)];

const FAST_ARC_LENGTH: usize = 9;

const FAST_BORDER_MARGIN: i64 = 3;

const ORB_HARRIS_K: f32 = 0.04;

const ORB_STRUCTURE_TENSOR_SIGMA: f32 = 1.5;

const ORB_FAST_THRESHOLD: f32 = 0.08;

pub(super) const ORB_GRID_CELL: u32 = 24;

pub(super) const ORB_CENTROID_RADIUS: i32 = 9;

fn fast_longest_arc(signs: &[i8; 16]) -> Option<(usize, usize)> {
    let mut best: Option<(usize, usize)> = None;
    for start in 0..16 {
        let sign = signs[start];
        if sign == 0 {
            continue;
        }
        let mut len = 1;
        while len < 16 && signs[(start + len) % 16] == sign {
            len += 1;
        }
        if best.is_none_or(|(_, best_len)| len > best_len) {
            best = Some((start, len));
        }
    }
    best
}

/// 🟢️ Classic FAST-9 corner response: for every pixel with a 3px border margin, samples the 16-point Bresenham radius-3 circle and looks for a contiguous arc of at least 9 points all brighter than `center + threshold` or all darker than `center - threshold`; the returned score is the summed absolute intensity difference from the center over the qualifying arc.
/// <https://www.edwardrosten.com/work/fast.html>
pub fn fast_corners(img: &ImageGray, threshold: f32) -> Vec<(u32, u32, f32)> {
    let (w, h) = (i64::from(img.width), i64::from(img.height));
    let mut out = Vec::new();
    if w <= 2 * FAST_BORDER_MARGIN || h <= 2 * FAST_BORDER_MARGIN {
        return out;
    }
    for y in FAST_BORDER_MARGIN..(h - FAST_BORDER_MARGIN) {
        for x in FAST_BORDER_MARGIN..(w - FAST_BORDER_MARGIN) {
            let center = img.get(x as u32, y as u32);
            let circle: [f32; 16] = std::array::from_fn(|i| {
                let (dx, dy) = FAST_CIRCLE_OFFSETS[i];
                img.get((x + i64::from(dx)) as u32, (y + i64::from(dy)) as u32)
            });
            let signs: [i8; 16] = std::array::from_fn(|i| {
                if circle[i] > center + threshold {
                    1
                } else if circle[i] < center - threshold {
                    -1
                } else {
                    0
                }
            });
            if let Some((start, len)) = fast_longest_arc(&signs) {
                if len >= FAST_ARC_LENGTH {
                    let score = (0..len).map(|k| (circle[(start + k) % 16] - center).abs()).sum();
                    out.push((x as u32, y as u32, score));
                }
            }
        }
    }
    out
}

/// 🟢️ [`fast_corners`]'s test restricted to rows `[first, last)` (clamped to the border margin),
/// answering only the corner positions.
fn fast_corner_rows(img: &ImageGray, threshold: f32, first: u32, last: u32) -> Vec<(u32, u32)> {
    let (w, h) = (i64::from(img.width), i64::from(img.height));
    let mut out = Vec::new();
    if w <= 2 * FAST_BORDER_MARGIN || h <= 2 * FAST_BORDER_MARGIN {
        return out;
    }
    let first = i64::from(first).max(FAST_BORDER_MARGIN);
    let last = i64::from(last).min(h - FAST_BORDER_MARGIN);
    for y in first..last {
        for x in FAST_BORDER_MARGIN..(w - FAST_BORDER_MARGIN) {
            let center = img.get(x as u32, y as u32);
            let circle: [f32; 16] = std::array::from_fn(|i| {
                let (dx, dy) = FAST_CIRCLE_OFFSETS[i];
                img.get((x + i64::from(dx)) as u32, (y + i64::from(dy)) as u32)
            });
            let signs: [i8; 16] = std::array::from_fn(|i| {
                if circle[i] > center + threshold {
                    1
                } else if circle[i] < center - threshold {
                    -1
                } else {
                    0
                }
            });
            if fast_longest_arc(&signs).is_some_and(|(_, len)| len >= FAST_ARC_LENGTH) {
                out.push((x as u32, y as u32));
            }
        }
    }
    out
}

fn structure_tensor_fields(img: &ImageGray) -> (ImageGray, ImageGray, ImageGray) {
    let g = scharr_gradients(img);
    let gx2 = ImageGray { width: img.width, height: img.height, data: g.gx.iter().map(|&v| v * v).collect() };
    let gy2 = ImageGray { width: img.width, height: img.height, data: g.gy.iter().map(|&v| v * v).collect() };
    let gxy = ImageGray { width: img.width, height: img.height, data: g.gx.iter().zip(g.gy.iter()).map(|(&a, &b)| a * b).collect() };
    (gaussian_blur(&gx2, ORB_STRUCTURE_TENSOR_SIGMA), gaussian_blur(&gy2, ORB_STRUCTURE_TENSOR_SIGMA), gaussian_blur(&gxy, ORB_STRUCTURE_TENSOR_SIGMA))
}

/// 🌄️ Per-pixel Harris corner response `det(M) - k trace(M)^2` from the gaussian-weighted structure tensor `M` of the Scharr gradients.
/// <https://en.wikipedia.org/wiki/Corner_detection#The_Harris_.26_Stephens_.2F_Plessey_.2F_Shi.E2.80.93Tomasi_corner_detection_algorithms>
pub fn harris_response(img: &ImageGray, k: f32) -> Vec<f32> {
    let (sxx, syy, sxy) = structure_tensor_fields(img);
    sxx.data
        .iter()
        .zip(syy.data.iter())
        .zip(sxy.data.iter())
        .map(|((&a, &d), &b)| {
            let det = a * d - b * b;
            let trace = a + d;
            det - k * trace * trace
        })
        .collect()
}

/// 🗺️ Grid-based adaptive non-maximal suppression: partitions the image into `cell x cell` bins and keeps, per bin, the top `per_cell` pixels by Shi-Tomasi minimum eigenvalue `mean(M) - sqrt(diff(M)^2 + off(M)^2)` of the gaussian-weighted structure tensor, for spatially uniform coverage.
/// <https://en.wikipedia.org/wiki/Corner_detection#The_Harris_.26_Stephens_.2F_Plessey_.2F_Shi.E2.80.93Tomasi_corner_detection_algorithms>
pub fn shi_tomasi_grid(img: &ImageGray, cell: u32, per_cell: usize) -> Vec<(u32, u32, f32)> {
    let cell = cell.max(1);
    let (sxx, syy, sxy) = structure_tensor_fields(img);
    let cells_x = img.width.div_ceil(cell).max(1);
    let cells_y = img.height.div_ceil(cell).max(1);
    let mut buckets: Vec<Vec<(u32, u32, f32)>> = vec![Vec::new(); (cells_x * cells_y) as usize];
    for (idx, ((&a, &d), &b)) in sxx.data.iter().zip(syy.data.iter()).zip(sxy.data.iter()).enumerate() {
        let x = idx as u32 % img.width;
        let y = idx as u32 / img.width;
        let mean = (a + d) * 0.5;
        let diff = (a - d) * 0.5;
        let min_eig = mean - (diff * diff + b * b).sqrt();
        let bucket = ((y / cell) * cells_x + (x / cell)) as usize;
        buckets[bucket].push((x, y, min_eig));
    }
    let mut out = Vec::new();
    for bucket in &mut buckets {
        bucket.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        out.extend(bucket.iter().take(per_cell).copied());
    }
    out
}

fn bucket_top_k(points: &[(u32, u32, f32)], cell: u32, cells_x: u32, per_cell: usize) -> Vec<(u32, u32, f32)> {
    let mut buckets: std::collections::HashMap<u32, Vec<(u32, u32, f32)>> = std::collections::HashMap::new();
    for &(x, y, score) in points {
        let bucket = (y / cell) * cells_x + (x / cell);
        buckets.entry(bucket).or_default().push((x, y, score));
    }
    let mut keys: Vec<u32> = buckets.keys().copied().collect();
    keys.sort_unstable();
    let mut out = Vec::new();
    for key in keys {
        let bucket = buckets.get_mut(&key).expect("key was collected from buckets.keys()");
        bucket.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        out.extend(bucket.iter().take(per_cell).copied());
    }
    out
}

pub(super) fn intensity_centroid_angle(level: &ImageGray, cx: f32, cy: f32, radius: i32) -> f32 {
    let mut m10 = 0.0f32;
    let mut m01 = 0.0f32;
    let radius_sq = (radius * radius) as f32;
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let (fx, fy) = (dx as f32, dy as f32);
            if fx * fx + fy * fy > radius_sq {
                continue;
            }
            let intensity = level.sample(cx + fx, cy + fy);
            m10 += fx * intensity;
            m01 += fy * intensity;
        }
    }
    m01.atan2(m10)
}

/// 🎯️ Oriented-FAST keypoint detector across a pyramid: runs [`fast_corners`] per level, rescoring survivors with [`harris_response`] (dropping edge/flat responses), distributes `target_count` across levels proportional to pixel area, applies a grid-bucketed top-k (à la [`shi_tomasi_grid`]) for spatial spread, and assigns an orientation via the intensity centroid `atan2(sum(y I), sum(x I))` over a circular neighbourhood.
/// <https://en.wikipedia.org/wiki/Oriented_FAST_and_rotated_BRIEF>
pub fn detect_orb_keypoints(pyramid: &Pyramid, target_count: usize) -> Vec<Keypoint> {
    if pyramid.levels.is_empty() || target_count == 0 {
        return Vec::new();
    }
    let areas: Vec<f64> = pyramid.levels.iter().map(|lvl| f64::from(lvl.width) * f64::from(lvl.height)).collect();
    let total_area: f64 = areas.iter().sum();
    let mut keypoints = Vec::new();
    for (octave, level) in pyramid.levels.iter().enumerate() {
        if level.width <= 2 * FAST_BORDER_MARGIN as u32 || level.height <= 2 * FAST_BORDER_MARGIN as u32 || octave > usize::from(u8::MAX) {
            continue;
        }
        let level_target = if total_area > 0.0 { ((areas[octave] / total_area) * target_count as f64).round() as usize } else { 0 };
        if level_target == 0 {
            continue;
        }
        let harris = harris_response(level, ORB_HARRIS_K);
        let candidates: Vec<(u32, u32, f32)> = fast_corners(level, ORB_FAST_THRESHOLD).into_iter().map(|(x, y, _)| (x, y, harris[(y * level.width + x) as usize])).filter(|&(_, _, score)| score > 0.0).collect();
        let cells_x = level.width.div_ceil(ORB_GRID_CELL).max(1);
        let cells_y = level.height.div_ceil(ORB_GRID_CELL).max(1);
        let per_cell = level_target.div_ceil((cells_x * cells_y) as usize).max(1);
        let mut selected = bucket_top_k(&candidates, ORB_GRID_CELL, cells_x, per_cell);
        selected.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        selected.truncate(level_target);
        for (x, y, response) in selected {
            let angle = intensity_centroid_angle(level, x as f32, y as f32, ORB_CENTROID_RADIUS);
            keypoints.push(Keypoint { x: x as f32, y: y as f32, octave: octave as u8, angle, response });
        }
    }
    keypoints
}

/// 🌄️ Standalone Harris-only keypoint detector — the third `detector: harris` option promised by the plugin UI, independent of FAST: scores every pixel with [`harris_response`], keeps the positive-response ones, applies the same grid-bucketed top-k spatial spread as [`detect_orb_keypoints`], and assigns orientation via the intensity centroid, so the resulting [`Keypoint`]s (all at `octave: 0`, single-level) feed straight into [`describe_orb`]/[`match_brute`] unchanged.
/// <https://en.wikipedia.org/wiki/Harris_Corner_Detector>
pub fn detect_harris_keypoints(image: &ImageGray, target_count: usize) -> Vec<Keypoint> {
    if target_count == 0 || image.width == 0 || image.height == 0 {
        return Vec::new();
    }
    let harris = harris_response(image, ORB_HARRIS_K);
    let candidates: Vec<(u32, u32, f32)> = harris.iter().enumerate().map(|(idx, &score)| (idx as u32 % image.width, idx as u32 / image.width, score)).filter(|&(_, _, score)| score > 0.0).collect();
    let cells_x = image.width.div_ceil(ORB_GRID_CELL).max(1);
    let cells_y = image.height.div_ceil(ORB_GRID_CELL).max(1);
    let per_cell = target_count.div_ceil((cells_x * cells_y) as usize).max(1);
    let mut selected = bucket_top_k(&candidates, ORB_GRID_CELL, cells_x, per_cell);
    selected.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    selected.truncate(target_count);
    selected
        .into_iter()
        .map(|(x, y, response)| {
            let angle = intensity_centroid_angle(image, x as f32, y as f32, ORB_CENTROID_RADIUS);
            Keypoint { x: x as f32, y: y as f32, octave: 0, angle, response }
        })
        .collect()
}

/// 🧭️ Which corner candidates a bounded detection scores.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum BoundedDetector {
    /// FAST-9 corners rescored by Harris across the pyramid, oriented by the intensity centroid
    /// (the [`detect_orb_keypoints`] policy).
    Orb,
    /// Every positive Harris response on the base level (the [`detect_harris_keypoints`] policy).
    Harris,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum BoundedDetectionPhase {
    /// Building pyramid level `level + 1` from level `level`, `row` rows of it done.
    Level { level: usize, row: u32 },
    /// Scharr gradient products of `level`, `row` rows done.
    Gradients { level: usize, row: u32 },
    /// Horizontal pass of the structure-tensor blur, `row` rows done.
    BlurRows { level: usize, row: u32 },
    /// Vertical pass of the blur and the Harris score, `row` rows done.
    BlurColumns { level: usize, row: u32 },
    /// Testing the corners of `level`, `row` rows of it done (positions and scores kept per row).
    Corners { level: usize, row: u32 },
    /// Suppressing non-maxima among the kept corners of `level`, `row` rows of it done.
    Suppress { level: usize, row: u32 },
    /// Spreading and ranking the candidates of each level.
    Select,
    /// Assigning intensity-centroid orientations, `cursor` keypoints done.
    Orient { cursor: usize },
    Done,
}

/// 🧭️ Resumable ORB/Harris keypoint detection for a fuel-bounded worker: the same pyramid, FAST +
/// Harris candidate scoring, grid-bucketed spread and intensity-centroid orientation as
/// [`detect_orb_keypoints`] / [`detect_harris_keypoints`], computed one row band at a time. Every
/// band is handed to the batch filters with a halo of rows wide enough for the blur, Scharr and FAST
/// footprints, so the band's interior rows are bit-identical to a whole-image pass — the pyramid and
/// the candidate set never depend on the band size — while one call touches only
/// `rows × width` pixels plus that fixed halo.
#[derive(semio_framework_value::RetireOwned)]
pub struct BoundedDetectionPreparation {
    detector: BoundedDetector,
    /// 🌄️ The active level's streamed Harris response.
    harris: Option<HarrisStream>,
    /// 🟢️ The active level's corners by row: `(x, score)`, from the FAST pass, for the suppression.
    corner_rows: Vec<Vec<(u32, f32)>>,
    /// 🧭️ Leave every keypoint at angle `0` (an upright descriptor) instead of steering it by the
    /// intensity centroid. Steering wins on the orbit fixture at every step size measured (5°–36°,
    /// `diagnose_orb_matching_vs_orbit_step`), so it is the default; upright is the option for
    /// captures known to carry no roll.
    upright: bool,
    pub(super) target_count: usize,
    pub(super) levels: Vec<ImageGray>,
    level_count: usize,
    pub(super) candidates: Vec<Vec<(u32, u32, f32)>>,
    pub(super) keypoints: Vec<Keypoint>,
    phase: BoundedDetectionPhase,
}

/// 🌫️ Rows the blur of one pyramid level reaches past a band (`sigma = 1` → kernel radius 3).
const LEVEL_BLUR_HALO_ROWS: u32 = 3;

/// 🌄️ Rows the corner suppression (radius 3) reaches past a band.
const CORNER_HALO_ROWS: u32 = 3;

/// 🌄️ The Harris response of one level computed in row bands: the Scharr gradient products, their
/// separable `sigma = 1.5` blur as two streamed passes, and the score. Each pass reads only what
/// earlier passes finished, so a band costs its own rows and nothing is recomputed for a halo —
/// the halo-band version of this (whole filters re-run on 12 extra rows per band) could not keep
/// an adversarial 512×512 frame's band inside a worker step.
#[derive(semio_framework_value::RetireOwned)]
struct HarrisStream {
    pub(super) width: u32,
    pub(super) height: u32,
    /// Scharr products `gx²`, `gy²`, `gx·gy`, then (in place) their horizontally blurred versions.
    xx: Vec<f32>,
    yy: Vec<f32>,
    xy: Vec<f32>,
    /// Horizontally blurred products, source of the vertical pass.
    hxx: Vec<f32>,
    hyy: Vec<f32>,
    hxy: Vec<f32>,
    /// Per-pixel Harris response.
    pub(super) score: Vec<f32>,
    kernel: Vec<f32>,
}

fn mirror(index: i64, n: i64) -> i64 {
    let mut index = index;
    loop {
        if index < 0 {
            index = -1 - index;
        } else if index >= n {
            index = 2 * n - 1 - index;
        } else {
            return index;
        }
    }
}

impl HarrisStream {
    pub(super) fn new(width: u32, height: u32) -> Self {
        let pixels = width as usize * height as usize;
        let radius = (3.0 * ORB_STRUCTURE_TENSOR_SIGMA).ceil() as i64;
        let mut kernel: Vec<f32> = (-radius..=radius).map(|k| ((k * k) as f32 * (-0.5 / (ORB_STRUCTURE_TENSOR_SIGMA * ORB_STRUCTURE_TENSOR_SIGMA))).exp()).collect();
        let total: f32 = kernel.iter().sum();
        for weight in &mut kernel {
            *weight /= total;
        }
        Self { width, height, xx: vec![0.0; pixels], yy: vec![0.0; pixels], xy: vec![0.0; pixels], hxx: vec![0.0; pixels], hyy: vec![0.0; pixels], hxy: vec![0.0; pixels], score: vec![0.0; pixels], kernel }
    }

    /// Scharr gradient products of rows `[first, last)`.
    fn gradients(&mut self, image: &ImageGray, first: u32, last: u32) {
        let (w, h) = (i64::from(self.width), i64::from(self.height));
        for y in i64::from(first)..i64::from(last) {
            for x in 0..w {
                let p = |dx: i64, dy: i64| image.data[(mirror(y + dy, h) * w + mirror(x + dx, w)) as usize];
                let gx = (3.0 * (p(1, -1) - p(-1, -1)) + 10.0 * (p(1, 0) - p(-1, 0)) + 3.0 * (p(1, 1) - p(-1, 1))) / 32.0;
                let gy = (3.0 * (p(-1, 1) - p(-1, -1)) + 10.0 * (p(0, 1) - p(0, -1)) + 3.0 * (p(1, 1) - p(1, -1))) / 32.0;
                let index = (y * w + x) as usize;
                self.xx[index] = gx * gx;
                self.yy[index] = gy * gy;
                self.xy[index] = gx * gy;
            }
        }
    }

    /// Horizontal blur of rows `[first, last)`.
    fn blur_rows(&mut self, first: u32, last: u32) {
        let w = i64::from(self.width);
        let radius = (self.kernel.len() / 2) as i64;
        for y in i64::from(first)..i64::from(last) {
            for x in 0..w {
                let (mut sxx, mut syy, mut sxy) = (0.0f32, 0.0f32, 0.0f32);
                for (k, weight) in self.kernel.iter().enumerate() {
                    let source = (y * w + mirror(x + k as i64 - radius, w)) as usize;
                    sxx += weight * self.xx[source];
                    syy += weight * self.yy[source];
                    sxy += weight * self.xy[source];
                }
                let index = (y * w + x) as usize;
                self.hxx[index] = sxx;
                self.hyy[index] = syy;
                self.hxy[index] = sxy;
            }
        }
    }

    /// Vertical blur and Harris score of rows `[first, last)` (reads the finished horizontal pass).
    fn blur_columns(&mut self, first: u32, last: u32) {
        let (w, h) = (i64::from(self.width), i64::from(self.height));
        let radius = (self.kernel.len() / 2) as i64;
        for y in i64::from(first)..i64::from(last) {
            for x in 0..w {
                let (mut sxx, mut syy, mut sxy) = (0.0f32, 0.0f32, 0.0f32);
                for (k, weight) in self.kernel.iter().enumerate() {
                    let source = (mirror(y + k as i64 - radius, h) * w + x) as usize;
                    sxx += weight * self.hxx[source];
                    syy += weight * self.hyy[source];
                    sxy += weight * self.hxy[source];
                }
                let det = sxx * syy - sxy * sxy;
                let trace = sxx + syy;
                self.score[(y * w + x) as usize] = det - ORB_HARRIS_K * trace * trace;
            }
        }
    }
}

/// 🧭️ Keypoints one orientation call handles per fuel unit.
const ORIENTATIONS_PER_UNIT: usize = 8;

/// 🎯️ Radius (pixels, per level) inside which only the strongest candidate survives. FAST answers a
/// cluster of adjacent pixels around one corner; without suppression the same corner is two or
/// three keypoints, a pair matches one of them and the next pair another, and the track builder
/// sees two tracks where one point was observed — on the orbit fixture most tracks spanned a single
/// registered frame and every registration after the fifth starved.
const CANDIDATE_SUPPRESSION_RADIUS: u32 = 3;

impl BoundedDetectionPreparation {
    /// 🧭️ Detection over `levels` pyramid levels of `base`, aiming at `target_count` keypoints.
    pub fn new(base: ImageGray, levels: usize, target_count: usize, detector: BoundedDetector, upright: bool) -> Self {
        let level_count = match detector {
            BoundedDetector::Orb => levels.max(1),
            BoundedDetector::Harris => 1,
        };
        let phase = if level_count > 1 && base.width > 1 && base.height > 1 { BoundedDetectionPhase::Level { level: 0, row: 0 } } else { BoundedDetectionPhase::Gradients { level: 0, row: 0 } };
        Self { detector, harris: None, corner_rows: Vec::new(), upright, target_count, levels: vec![base], level_count, candidates: Vec::new(), keypoints: Vec::new(), phase }
    }

    /// 📏️ The rows `[first, last)` of `image` with up to `halo` rows on either side, and the offset
    /// of `first` inside the copy.
    fn band(image: &ImageGray, first: u32, last: u32, halo: u32) -> (ImageGray, u32) {
        let start = first.saturating_sub(halo);
        let end = last.saturating_add(halo).min(image.height);
        let width = image.width as usize;
        let data = image.data[start as usize * width..end as usize * width].to_vec();
        (ImageGray { width: image.width, height: end - start, data }, first - start)
    }

    /// ⏱️ Advances by at most `row_budget` rows of the active level (or its band-equivalent in the
    /// selection and orientation phases); `true` once the keypoints are ready.
    pub fn advance(&mut self, row_budget: usize) -> bool {
        let rows = row_budget.max(1) as u32;
        match self.phase {
            BoundedDetectionPhase::Level { level, row } => {
                let previous = &self.levels[level];
                let next_height = previous.height.div_ceil(2);
                let next_width = previous.width.div_ceil(2);
                if self.levels.len() == level + 1 {
                    self.levels.push(ImageGray { width: next_width, height: next_height, data: Vec::with_capacity(next_width as usize * next_height as usize) });
                }
                let end = (row + rows).min(next_height);
                let previous = &self.levels[level];
                let (band, offset) = Self::band(previous, 2 * row, (2 * end).min(previous.height), LEVEL_BLUR_HALO_ROWS);
                let blurred = gaussian_blur(&band, 1.0);
                let width = blurred.width as usize;
                let interior_rows = (2 * end).min(previous.height) - 2 * row;
                let interior = ImageGray { width: blurred.width, height: interior_rows, data: blurred.data[offset as usize * width..(offset + interior_rows) as usize * width].to_vec() };
                let halved = remodeling_image::downsample_half(&interior);
                debug_assert_eq!(halved.height, end - row);
                self.levels[level + 1].data.extend_from_slice(&halved.data);
                if end == next_height {
                    let built = level + 1;
                    let last = &self.levels[built];
                    self.phase = if built + 1 < self.level_count && last.width > 1 && last.height > 1 { BoundedDetectionPhase::Level { level: built, row: 0 } } else { BoundedDetectionPhase::Gradients { level: 0, row: 0 } };
                } else {
                    self.phase = BoundedDetectionPhase::Level { level, row: end };
                }
            }
            BoundedDetectionPhase::Gradients { level, row } => {
                let image = &self.levels[level];
                let stream = self.harris.get_or_insert_with(|| HarrisStream::new(image.width, image.height));
                let end = (row + rows).min(image.height);
                stream.gradients(image, row, end);
                self.phase = if end == image.height { BoundedDetectionPhase::BlurRows { level, row: 0 } } else { BoundedDetectionPhase::Gradients { level, row: end } };
            }
            BoundedDetectionPhase::BlurRows { level, row } => {
                let height = self.levels[level].height;
                let stream = self.harris.as_mut().expect("harris stream");
                let end = (row + rows).min(height);
                stream.blur_rows(row, end);
                self.phase = if end == height { BoundedDetectionPhase::BlurColumns { level, row: 0 } } else { BoundedDetectionPhase::BlurRows { level, row: end } };
            }
            BoundedDetectionPhase::BlurColumns { level, row } => {
                let height = self.levels[level].height;
                let stream = self.harris.as_mut().expect("harris stream");
                let end = (row + rows).min(height);
                stream.blur_columns(row, end);
                self.phase = if end == height { BoundedDetectionPhase::Corners { level, row: 0 } } else { BoundedDetectionPhase::BlurColumns { level, row: end } };
            }
            BoundedDetectionPhase::Corners { level, row } => {
                let image = &self.levels[level];
                let stream = self.harris.as_ref().expect("harris stream");
                if self.corner_rows.len() != image.height as usize {
                    self.corner_rows = vec![Vec::new(); image.height as usize];
                }
                let end = (row + rows).min(image.height);
                if image.width > 2 * FAST_BORDER_MARGIN as u32 && image.height > 2 * FAST_BORDER_MARGIN as u32 {
                    let positions: Vec<(u32, u32)> = match self.detector {
                        BoundedDetector::Orb => fast_corner_rows(image, ORB_FAST_THRESHOLD, row, end),
                        BoundedDetector::Harris => (row..end).flat_map(|y| (0..image.width).map(move |x| (x, y))).collect(),
                    };
                    for (x, y) in positions {
                        let score = stream.score[(y * image.width + x) as usize];
                        if score > 0.0 {
                            self.corner_rows[y as usize].push((x, score));
                        }
                    }
                }
                self.phase = if end == image.height { BoundedDetectionPhase::Suppress { level, row: 0 } } else { BoundedDetectionPhase::Corners { level, row: end } };
            }
            BoundedDetectionPhase::Suppress { level, row } => {
                if self.candidates.len() == level {
                    self.candidates.push(Vec::new());
                }
                let height = self.levels[level].height;
                let end = (row + rows).min(height);
                // The band's corners plus a suppression-radius halo, so an interior corner is
                // judged against its neighbours across the band edge too.
                let (halo_first, halo_last) = (row.saturating_sub(CORNER_HALO_ROWS), end.saturating_add(CORNER_HALO_ROWS).min(height));
                // Dense score grid of those rows: a corner survives when no corner within the
                // suppression radius outscores it (ties go to the earlier row, then column), a
                // fixed 7×7 look per corner however dense the corners are.
                let width = self.levels[level].width as usize;
                let grid_rows = (halo_last - halo_first) as usize;
                let mut grid = vec![0.0f32; grid_rows * width];
                for y in halo_first..halo_last {
                    for &(x, score) in &self.corner_rows[y as usize] {
                        grid[(y - halo_first) as usize * width + x as usize] = score;
                    }
                }
                let radius = CANDIDATE_SUPPRESSION_RADIUS as i64;
                let mut found: Vec<(u32, u32, f32)> = Vec::new();
                for y in row..end {
                    for &(x, score) in &self.corner_rows[y as usize] {
                        let mut suppressed = false;
                        'window: for dy in -radius..=radius {
                            let gy = i64::from(y - halo_first) + dy;
                            if gy < 0 || gy >= grid_rows as i64 {
                                continue;
                            }
                            for dx in -radius..=radius {
                                let gx = i64::from(x) + dx;
                                if gx < 0 || gx >= width as i64 || (dx == 0 && dy == 0) {
                                    continue;
                                }
                                let other = grid[gy as usize * width + gx as usize];
                                if other > score || (other == score && (dy < 0 || (dy == 0 && dx < 0))) {
                                    suppressed = true;
                                    break 'window;
                                }
                            }
                        }
                        if !suppressed {
                            found.push((x, y, score));
                        }
                    }
                }
                // No per-band cap: the suppression already bounds a level's candidates to one per
                // `(radius + 1)²` pixels whatever the image (a cap sized from the band count once
                // cut dense rows of real corners at one-row bands and changed the keypoints with
                // the step size), and the spread below is linear in that count.
                self.candidates[level].extend(found);
                if end == height {
                    self.harris = None;
                    self.corner_rows = Vec::new();
                    self.phase = if level + 1 < self.levels.len() && self.detector == BoundedDetector::Orb { BoundedDetectionPhase::Gradients { level: level + 1, row: 0 } } else { BoundedDetectionPhase::Select };
                } else {
                    self.phase = BoundedDetectionPhase::Suppress { level, row: end };
                }
            }
            BoundedDetectionPhase::Select => {
                let areas: Vec<f64> = self.candidates.iter().enumerate().map(|(level, _)| f64::from(self.levels[level].width) * f64::from(self.levels[level].height)).collect();
                let total_area: f64 = areas.iter().sum();
                for (level, candidates) in self.candidates.iter().enumerate() {
                    let image = &self.levels[level];
                    let level_target = if self.detector == BoundedDetector::Harris {
                        self.target_count
                    } else if total_area > 0.0 {
                        ((areas[level] / total_area) * self.target_count as f64).round() as usize
                    } else {
                        0
                    };
                    if level_target == 0 || candidates.is_empty() {
                        continue;
                    }
                    let cells_x = image.width.div_ceil(ORB_GRID_CELL).max(1);
                    let cells_y = image.height.div_ceil(ORB_GRID_CELL).max(1);
                    let per_cell = level_target.div_ceil((cells_x * cells_y) as usize).max(1);
                    let mut selected = bucket_top_k(candidates, ORB_GRID_CELL, cells_x, per_cell);
                    selected.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
                    selected.truncate(level_target);
                    self.keypoints.extend(selected.into_iter().map(|(x, y, response)| Keypoint { x: x as f32, y: y as f32, octave: level as u8, angle: 0.0, response }));
                }
                self.candidates = Vec::new();
                self.phase = if self.upright { BoundedDetectionPhase::Done } else { BoundedDetectionPhase::Orient { cursor: 0 } };
            }
            BoundedDetectionPhase::Orient { cursor } => {
                let end = cursor.saturating_add(row_budget.max(1) * ORIENTATIONS_PER_UNIT).min(self.keypoints.len());
                for keypoint in &mut self.keypoints[cursor..end] {
                    let level = &self.levels[usize::from(keypoint.octave)];
                    keypoint.angle = intensity_centroid_angle(level, keypoint.x, keypoint.y, ORB_CENTROID_RADIUS);
                }
                self.phase = if end == self.keypoints.len() { BoundedDetectionPhase::Done } else { BoundedDetectionPhase::Orient { cursor: end } };
            }
            BoundedDetectionPhase::Done => {}
        }
        self.phase == BoundedDetectionPhase::Done
    }

    /// 🧭️ The phase the next call works on, for the worker-law tests' diagnostics.
    pub fn phase_label(&self) -> String {
        format!("{:?}", self.phase)
    }

    /// 🧭️ The pyramid the keypoints live in and the keypoints, once [`Self::advance`] answered `true`.
    pub fn finish(self) -> (Pyramid, Vec<Keypoint>) {
        (Pyramid { levels: self.levels, scale: 0.5 }, self.keypoints)
    }
}
// #endregion 🔖️Detect
