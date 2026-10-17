//! 🌀️ AKAZE of the remodeling engine features: the nonlinear diffusion scale space, the Hessian blob detector and the M-LDB description. See [`super`].

use super::*;

// #region 🔖️ScaleSpace
const AKAZE_BASE_SIGMA: f32 = 1.6;

pub(super) const AKAZE_FED_TAU_MAX: f32 = 0.25;

const AKAZE_CONTRAST_PERCENTILE: f32 = 0.7;

/// 🌀️ One evolved level of the nonlinear diffusion scale space: the diffused image, its octave/sublevel indices, and its effective smoothing scale `esigma` expressed in ORIGINAL-image pixel units (`esigma = base_sigma * 2^(octave + sublevel / sublevels)`).
#[derive(Clone, Debug, PartialEq)]
pub struct ScaleLevel {
    pub image: ImageGray,
    pub octave: u8,
    pub sublevel: u8,
    pub esigma: f32,
}

/// 🌀️ A full AKAZE nonlinear diffusion scale space: evolution levels in `(octave, sublevel)` order, alongside the base image's dimensions (any level's coordinates convert to base-image pixels by multiplying by `2^octave`).
#[derive(Clone, Debug, PartialEq)]
pub struct ScaleSpace {
    pub levels: Vec<ScaleLevel>,
    pub base_width: u32,
    pub base_height: u32,
}

/// 📊️ Perona-Malik contrast factor `k`: the 70th-percentile gradient magnitude of a `sigma=1` gaussian-presmoothed image — the standard AKAZE contrast-factor estimation procedure that keeps the diffusion's edge-stopping behaviour scaled to the image's own contrast.
/// <https://en.wikipedia.org/wiki/Anisotropic_diffusion>
fn estimate_contrast_factor(img: &ImageGray) -> f32 {
    let smoothed = gaussian_blur(img, 1.0);
    let g = scharr_gradients(&smoothed);
    let mut magnitudes: Vec<f32> = g.gx.iter().zip(g.gy.iter()).map(|(&gx, &gy)| gx.hypot(gy)).collect();
    if magnitudes.is_empty() {
        return 1e-3;
    }
    magnitudes.sort_by(f32::total_cmp);
    let idx = (((magnitudes.len() - 1) as f32) * AKAZE_CONTRAST_PERCENTILE).round() as usize;
    magnitudes[idx.min(magnitudes.len() - 1)].max(1e-6)
}

/// 🌊️ Perona-Malik `g2` conductivity `1 / (1 + (|∇L| / k)^2)` per pixel: decays toward `0` across strong edges (preserving them) and stays near `1` in flat regions (smoothing them freely).
/// <https://en.wikipedia.org/wiki/Anisotropic_diffusion>
fn perona_malik_g2(gradient: &GradientField, k: f32) -> Vec<f32> {
    let k_sq = (k * k).max(1e-12);
    gradient.gx.iter().zip(gradient.gy.iter()).map(|(&gx, &gy)| 1.0 / (1.0 + (gx * gx + gy * gy) / k_sq)).collect()
}

/// ⏱️ Fast Explicit Diffusion step schedule reaching total evolution time `t` in one stable cycle: `n = ceil(sqrt(3t/tau_max + 1/4) - 1/2)` steps with `tau_i = tau_max / (2 cos(pi (2i+1) / (4n+2)))` — the FED scheme AKAZE/KAZE use to take far larger stable steps than plain explicit diffusion.
/// <https://en.wikipedia.org/wiki/Anisotropic_diffusion>
pub(super) fn fed_tau_schedule(t: f32, tau_max: f32) -> Vec<f32> {
    if t <= 0.0 {
        return Vec::new();
    }
    let n = ((3.0 * t / tau_max + 0.25).sqrt() - 0.5).ceil().max(1.0) as usize;
    let n_f = n as f32;
    (0..n).map(|i| tau_max / (2.0 * (std::f32::consts::PI * (2.0 * i as f32 + 1.0) / (4.0 * n_f + 2.0)).cos())).collect()
}

/// 🧮️ One explicit nonlinear-diffusion step: for every pixel, sums flux across its 4-neighbourhood weighted by the average conductivity between center and neighbour, `L' = L + tau * div(g grad L)`, with clamped (zero-flux) borders.
/// <https://en.wikipedia.org/wiki/Anisotropic_diffusion>
pub(super) fn diffusion_step(l: &ImageGray, g: &[f32], tau: f32) -> ImageGray {
    let (w, h) = (l.width, l.height);
    let mut out = l.clone();
    if w == 0 || h == 0 {
        return out;
    }
    let at = |x: i64, y: i64| -> (f32, f32) {
        let cx = x.clamp(0, i64::from(w) - 1) as u32;
        let cy = y.clamp(0, i64::from(h) - 1) as u32;
        let idx = (cy * w + cx) as usize;
        (l.data[idx], g[idx])
    };
    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) as usize;
            let (lc, gc) = (l.data[idx], g[idx]);
            let (le, ge) = at(i64::from(x) + 1, i64::from(y));
            let (lw, gw) = at(i64::from(x) - 1, i64::from(y));
            let (ls, gs) = at(i64::from(x), i64::from(y) + 1);
            let (ln, gn) = at(i64::from(x), i64::from(y) - 1);
            let flux = 0.5 * (gc + ge) * (le - lc) + 0.5 * (gc + gw) * (lw - lc) + 0.5 * (gc + gs) * (ls - lc) + 0.5 * (gc + gn) * (ln - lc);
            out.data[idx] = lc + tau * flux;
        }
    }
    out
}

/// 🌀️ Builds a full AKAZE nonlinear diffusion scale space over `octaves` octaves of `sublevels` sublevels each: per octave, starts from [`build_pyramid`]'s anti-aliased downsample of `image` (sublevel 0, not yet diffused), estimates a per-octave Perona-Malik contrast factor once via [`estimate_contrast_factor`] on that octave's base image, then for each further sublevel recomputes conductivity from the currently-evolved image's [`scharr_gradients`] and walks a [`fed_tau_schedule`] of explicit [`diffusion_step`]s covering the incremental evolution time `0.5 sigma^2` between consecutive sublevel scales `sigma = base_sigma * 2^(sublevel / sublevels)`.
/// <https://en.wikipedia.org/wiki/Anisotropic_diffusion>
pub fn build_akaze_scale_space(image: &ImageGray, octaves: u8, sublevels: u8) -> ScaleSpace {
    let octaves = octaves.max(1);
    let sublevels = sublevels.max(1);
    let pyramid = build_pyramid(image, usize::from(octaves));
    let mut levels = Vec::new();
    for (o, octave_base) in pyramid.levels.iter().enumerate() {
        if octave_base.width == 0 || octave_base.height == 0 {
            continue;
        }
        let ratio = 2f32.powi(o as i32);
        let k = estimate_contrast_factor(octave_base);
        let mut current = octave_base.clone();
        let mut prev_t = 0.5 * AKAZE_BASE_SIGMA * AKAZE_BASE_SIGMA;
        levels.push(ScaleLevel { image: current.clone(), octave: o as u8, sublevel: 0, esigma: AKAZE_BASE_SIGMA * ratio });
        for s in 1..sublevels {
            let sigma_local = AKAZE_BASE_SIGMA * 2f32.powf(f32::from(s) / f32::from(sublevels));
            let t_now = 0.5 * sigma_local * sigma_local;
            let dt = (t_now - prev_t).max(0.0);
            let gradient = scharr_gradients(&current);
            let conductivity = perona_malik_g2(&gradient, k);
            for tau in fed_tau_schedule(dt, AKAZE_FED_TAU_MAX) {
                current = diffusion_step(&current, &conductivity, tau);
            }
            levels.push(ScaleLevel { image: current.clone(), octave: o as u8, sublevel: s, esigma: sigma_local * ratio });
            prev_t = t_now;
        }
    }
    ScaleSpace { levels, base_width: image.width, base_height: image.height }
}
// #endregion 🔖️ScaleSpace

// #region 🔖️Akaze
const MLDB_GRID: u32 = 4;

const MLDB_CELLS: usize = (MLDB_GRID * MLDB_GRID) as usize;

const MLDB_VALUES: usize = MLDB_CELLS * 3;

const MLDB_PATCH_RADIUS: u32 = 8;

const MLDB_SEED: u64 = 0xA11A_2EDE_5C81_B70F;

const AKAZE_RESPONSE_EPS: f32 = 1e-9;

const AKAZE_BORDER_MARGIN: u32 = 2;

/// 🧮️ Scale-normalized Hessian determinant `sigma_local^4 (L_xx L_yy - L_xy^2)` at every pixel of `image`, with second derivatives from two passes of [`scharr_gradients`] (gradient-of-the-gradient) — the standard blob-strength measure driving AKAZE's scale-space extrema search.
/// <https://en.wikipedia.org/wiki/Blob_detection#The_determinant_of_the_Hessian>
fn hessian_determinant_response(image: &ImageGray, sigma_local: f32) -> Vec<f32> {
    let g1 = scharr_gradients(image);
    let gx_img = ImageGray { width: image.width, height: image.height, data: g1.gx };
    let gy_img = ImageGray { width: image.width, height: image.height, data: g1.gy };
    let g2x = scharr_gradients(&gx_img);
    let g2y = scharr_gradients(&gy_img);
    let norm = sigma_local.powi(4).max(1e-12);
    g2x.gx
        .iter()
        .zip(g2y.gy.iter())
        .zip(g2x.gy.iter().zip(g2y.gx.iter()))
        .map(|((&lxx, &lyy), (&lxy_a, &lxy_b))| {
            let lxy = 0.5 * (lxy_a + lxy_b);
            (lxx * lyy - lxy * lxy) * norm
        })
        .collect()
}

/// 📐️ Sub-pixel offset from a local 2D quadratic (Taylor) fit of a response map around integer pixel `(x, y)`: solves the Newton step `H offset = -grad` from central-difference first/second derivatives, clamped to `[-0.5, 0.5]` per axis; a near-singular Hessian yields a zero offset (keep the integer position).
pub(super) fn quadratic_refine_2d(response: &[f32], width: u32, x: u32, y: u32) -> (f32, f32) {
    let at = |dx: i32, dy: i32| response[((y as i32 + dy) as u32 * width + (x as i32 + dx) as u32) as usize];
    let center = at(0, 0);
    let dx = (at(1, 0) - at(-1, 0)) * 0.5;
    let dy = (at(0, 1) - at(0, -1)) * 0.5;
    let dxx = at(1, 0) - 2.0 * center + at(-1, 0);
    let dyy = at(0, 1) - 2.0 * center + at(0, -1);
    let dxy = (at(1, 1) - at(1, -1) - at(-1, 1) + at(-1, -1)) * 0.25;
    let det = dxx * dyy - dxy * dxy;
    if det.abs() < 1e-9 {
        return (0.0, 0.0);
    }
    let ox = -(dyy * dx - dxy * dy) / det;
    let oy = -(dxx * dy - dxy * dx) / det;
    (ox.clamp(-0.5, 0.5), oy.clamp(-0.5, 0.5))
}

pub(super) struct AkazeCandidate {
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) response: f32,
    pub(super) level: u32,
}

pub(super) fn akaze_grid_top_k(candidates: Vec<AkazeCandidate>, image_w: u32, image_h: u32, cell: u32, target_count: usize) -> Vec<AkazeCandidate> {
    if candidates.is_empty() || target_count == 0 {
        return Vec::new();
    }
    let cells_x = image_w.div_ceil(cell).max(1);
    let cells_y = image_h.div_ceil(cell).max(1);
    let per_cell = target_count.div_ceil((cells_x * cells_y) as usize).max(1);
    let mut buckets: std::collections::HashMap<u32, Vec<AkazeCandidate>> = std::collections::HashMap::new();
    for c in candidates {
        let bx = ((c.x as u32) / cell).min(cells_x - 1);
        let by = ((c.y as u32) / cell).min(cells_y - 1);
        buckets.entry(by * cells_x + bx).or_default().push(c);
    }
    let mut keys: Vec<u32> = buckets.keys().copied().collect();
    keys.sort_unstable();
    let mut out = Vec::new();
    for key in keys {
        let mut bucket = buckets.remove(&key).expect("key was collected from buckets.keys()");
        bucket.sort_by(|a, b| b.response.partial_cmp(&a.response).unwrap_or(std::cmp::Ordering::Equal));
        bucket.truncate(per_cell);
        out.extend(bucket);
    }
    out.sort_by(|a, b| b.response.partial_cmp(&a.response).unwrap_or(std::cmp::Ordering::Equal));
    out.truncate(target_count);
    out
}

/// 🎯️ Hessian-determinant blob detector over an AKAZE [`ScaleSpace`]: scores every level (every octave × every sublevel) with [`hessian_determinant_response`], keeps pixels that are a strict spatial local maximum in their level's own `3x3` neighbourhood and positive, sub-pixel refines the `(x, y)` position via [`quadratic_refine_2d`], rescales to base-image coordinates, and assigns an intensity-centroid orientation from the owning level's diffused image. Candidates from every level are then pooled and passed through the same grid-bucketed top-k spatial spread as [`detect_orb_keypoints`] (which, since a genuine blob typically survives as a local max across several neighbouring sublevels, also acts as the cross-scale duplicate suppression). Each returned [`Keypoint`]'s `octave` field is the flat index into `scale_space.levels` it was detected at (not a true octave number), so [`describe_akaze`] can look the owning [`ScaleLevel`] back up directly.
/// <https://en.wikipedia.org/wiki/Blob_detection>
pub fn detect_akaze_keypoints(scale_space: &ScaleSpace, target_count: usize) -> Vec<Keypoint> {
    if scale_space.levels.is_empty() || target_count == 0 {
        return Vec::new();
    }
    let mut candidates = Vec::new();
    for (li, level) in scale_space.levels.iter().enumerate() {
        let ratio = 2f32.powi(level.octave as i32);
        let (w, h) = (level.image.width, level.image.height);
        if w <= 2 * AKAZE_BORDER_MARGIN + 2 || h <= 2 * AKAZE_BORDER_MARGIN + 2 {
            continue;
        }
        let response = hessian_determinant_response(&level.image, level.esigma / ratio);
        for y in (AKAZE_BORDER_MARGIN + 1)..(h - AKAZE_BORDER_MARGIN - 1) {
            for x in (AKAZE_BORDER_MARGIN + 1)..(w - AKAZE_BORDER_MARGIN - 1) {
                let idx = (y * w + x) as usize;
                let value = response[idx];
                if value <= AKAZE_RESPONSE_EPS {
                    continue;
                }
                let mut is_max = true;
                'window: for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nidx = ((y as i32 + dy) as u32 * w + (x as i32 + dx) as u32) as usize;
                        if response[nidx] > value {
                            is_max = false;
                            break 'window;
                        }
                    }
                }
                if !is_max {
                    continue;
                }
                let (ox, oy) = quadratic_refine_2d(&response, w, x, y);
                let (lx, ly) = (x as f32 + ox, y as f32 + oy);
                candidates.push(AkazeCandidate { x: lx * ratio, y: ly * ratio, response: value, level: li as u32 });
            }
        }
    }
    let selected = akaze_grid_top_k(candidates, scale_space.base_width, scale_space.base_height, ORB_GRID_CELL, target_count);
    selected
        .into_iter()
        .map(|c| {
            let level = &scale_space.levels[c.level as usize];
            let ratio = 2f32.powi(level.octave as i32);
            let angle = intensity_centroid_angle(&level.image, c.x / ratio, c.y / ratio, ORB_CENTROID_RADIUS);
            Keypoint { x: c.x, y: c.y, octave: c.level.min(u32::from(u8::MAX)) as u8, angle, response: c.response }
        })
        .collect()
}

fn mldb_pattern() -> &'static [(u8, u8); 256] {
    static PATTERN: std::sync::OnceLock<[(u8, u8); 256]> = std::sync::OnceLock::new();
    PATTERN.get_or_init(|| {
        let mut rng = geometry::random::Rng::from_seed(MLDB_SEED);
        std::array::from_fn(|_| {
            let a = rng.next_range(0, MLDB_VALUES as u64) as u8;
            let mut b = rng.next_range(0, MLDB_VALUES as u64) as u8;
            while b == a {
                b = rng.next_range(0, MLDB_VALUES as u64) as u8;
            }
            (a, b)
        })
    })
}

/// 🧬️ M-LDB (Modified Local Difference Binary) description: for each keypoint, extracts a `(2 MLDB_PATCH_RADIUS + 1)^2` patch from its owning [`ScaleLevel`] (looked up via `Keypoint::octave` as a flat scale-space index, per [`detect_akaze_keypoints`]), steered by the keypoint's angle via [`extract_patch`], splits it into a `4x4` grid of sub-cells and averages 3 channels per cell — mean intensity and the two mean [`scharr_gradients`] components — into 48 scalar values, then sets bit `i` from a fixed published-seed pattern of 256 value-index pairs (generated once via `geometry::random`, so identical across builds and runs) whenever the pattern's first value is less than its second. Emits into the same [`Descriptor256`] the rBRIEF path produces, so [`match_brute`]/[`hamming`] work unchanged on AKAZE descriptors.
/// <https://en.wikipedia.org/wiki/AKAZE>
pub fn describe_akaze(scale_space: &ScaleSpace, keypoints: &[Keypoint]) -> Vec<Descriptor256> {
    let pattern = mldb_pattern();
    let side = 2 * MLDB_PATCH_RADIUS + 1;
    let cell = (side / MLDB_GRID).max(1);
    keypoints
        .iter()
        .map(|kp| {
            let level_idx = (kp.octave as usize).min(scale_space.levels.len().saturating_sub(1));
            let level = &scale_space.levels[level_idx];
            let ratio = 2f32.powi(level.octave as i32);
            let (lx, ly) = (kp.x / ratio, kp.y / ratio);
            let patch = extract_patch(&level.image, lx, ly, MLDB_PATCH_RADIUS, kp.angle);
            let patch_img = ImageGray { width: side, height: side, data: patch.data };
            let grad = scharr_gradients(&patch_img);
            let mut values = [0f32; MLDB_VALUES];
            for gy in 0..MLDB_GRID {
                for gx in 0..MLDB_GRID {
                    let x0 = gx * cell;
                    let x1 = if gx + 1 == MLDB_GRID { side } else { (gx + 1) * cell };
                    let y0 = gy * cell;
                    let y1 = if gy + 1 == MLDB_GRID { side } else { (gy + 1) * cell };
                    let (mut sum_i, mut sum_dx, mut sum_dy, mut n) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
                    for y in y0..y1 {
                        for x in x0..x1 {
                            let idx = (y * side + x) as usize;
                            sum_i += patch_img.data[idx];
                            sum_dx += grad.gx[idx];
                            sum_dy += grad.gy[idx];
                            n += 1.0;
                        }
                    }
                    let cell_idx = (gy * MLDB_GRID + gx) as usize;
                    if n > 0.0 {
                        values[cell_idx] = sum_i / n;
                        values[MLDB_CELLS + cell_idx] = sum_dx / n;
                        values[2 * MLDB_CELLS + cell_idx] = sum_dy / n;
                    }
                }
            }
            let mut words = [0u64; 4];
            for (bit, &(a, b)) in pattern.iter().enumerate() {
                if values[a as usize] < values[b as usize] {
                    words[bit / 64] |= 1u64 << (bit % 64);
                }
            }
            Descriptor256(words)
        })
        .collect()
}
// #endregion 🔖️Akaze
