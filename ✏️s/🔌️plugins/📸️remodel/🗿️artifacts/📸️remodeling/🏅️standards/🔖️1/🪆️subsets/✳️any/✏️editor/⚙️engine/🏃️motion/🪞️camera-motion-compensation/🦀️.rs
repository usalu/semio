//! 🪞️ Camera motion compensation of the videogrammetry: the rolling-shutter compensation, the path stabilisation and the deblurring. See [`super`].

use super::*;

// #region 🔖️RollingShutterComp
fn linear_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let mean_x = x.iter().sum::<f64>() / n;
    let mean_y = y.iter().sum::<f64>() / n;
    let mut sxy = 0.0;
    let mut sxx = 0.0;
    for (&xi, &yi) in x.iter().zip(y.iter()) {
        sxy += (xi - mean_x) * (yi - mean_y);
        sxx += (xi - mean_x) * (xi - mean_x);
    }
    if sxx.abs() < 1e-9 {
        return (mean_y, 0.0);
    }
    let slope = sxy / sxx;
    (mean_y - slope * mean_x, slope)
}

/// 🌀️ Simplified 2D-only rolling-shutter velocity estimate: a real 3D SE(3) tangent is not observable from
/// bare 2D tracks (no depth/pose), so this instead fits a per-row-affine 2D displacement field — linear
/// regression of each track's frame-to-frame `(dx, dy)` against its source row — and packs it into the
/// requested `[f64; 6]` slot as `[vx0, vy0, kx, ky, 0.0, 0.0]`: a baseline per-frame velocity `(vx0, vy0)`
/// plus its row-dependent gradient `(kx, ky)` (px/frame per pixel-row), the standard rolling-shutter
/// approximation when only image-plane correspondences are available.
pub fn estimate_rs_velocity(tracks: &[Track2d], image_height: u32) -> [f64; 6] {
    let mut rows = Vec::new();
    let mut dxs = Vec::new();
    let mut dys = Vec::new();
    for track in tracks {
        for w in track.samples.windows(2) {
            let (_, x0, y0) = w[0];
            let (_, x1, y1) = w[1];
            rows.push(f64::from(y0));
            dxs.push(f64::from(x1 - x0));
            dys.push(f64::from(y1 - y0));
        }
    }
    if rows.is_empty() || image_height == 0 {
        return [0.0; 6];
    }
    let (vx0, kx) = linear_regression(&rows, &dxs);
    let (vy0, ky) = linear_regression(&rows, &dys);
    [vx0, vy0, kx, ky, 0.0, 0.0]
}

/// 🗺️ Per-pixel `(map_x, map_y)` rolling-shutter rectification field from [`estimate_rs_velocity`]'s
/// simplified per-row-affine `model`, consumable by `remodeling_image::remap`: shifts each output pixel by
/// the row-dependent gradient `(kx, ky)` relative to the frame's vertical center. `line_delay_s <= 0` (no
/// rolling-shutter skew) short-circuits to the identity map; otherwise, since the fitted `model` is already
/// expressed directly in px-per-row units, `line_delay_s`'s absolute seconds value does not further scale
/// the correction under this simplified model — an explicit, honest limitation of using only 2D track data.
pub fn build_rs_rectify_maps(model: [f64; 6], image_width: u32, image_height: u32, line_delay_s: f64) -> (Vec<f32>, Vec<f32>) {
    let n = image_width as usize * image_height as usize;
    let mut map_x = vec![0.0f32; n];
    let mut map_y = vec![0.0f32; n];
    if line_delay_s <= 0.0 || image_width == 0 || image_height == 0 {
        for row in 0..image_height {
            for col in 0..image_width {
                let idx = (row * image_width + col) as usize;
                map_x[idx] = col as f32;
                map_y[idx] = row as f32;
            }
        }
        return (map_x, map_y);
    }
    let [_, _, kx, ky, ..] = model;
    let ref_row = f64::from(image_height) * 0.5;
    for row in 0..image_height {
        let shift_x = kx * (f64::from(row) - ref_row);
        let shift_y = ky * (f64::from(row) - ref_row);
        for col in 0..image_width {
            let idx = (row * image_width + col) as usize;
            map_x[idx] = (f64::from(col) + shift_x) as f32;
            map_y[idx] = (f64::from(row) + shift_y) as f32;
        }
    }
    (map_x, map_y)
}
// #endregion 🔖️RollingShutterComp

// #region 🔖️Stabilize
const STABILIZE_SG_ORDER: usize = 2;

/// 🧘️ Savitzky-Golay smoothing of a camera pose sequence in the SE(3) Lie algebra: consecutive relative
/// motions are logged to twists, each of the 6 twist channels is smoothed independently (`deriv = 0`), and
/// the smoothed twists are reintegrated via [`Se3::exp`]/[`Se3::semio_compose_rs`] — so the result stays exactly on
/// the SE(3) manifold instead of naively averaging matrix or quaternion components. `window` is clamped to
/// the largest valid odd value `>= 3` for however many pose-to-pose twists are available.
pub fn smooth_camera_path(poses: &[Se3], window: usize) -> Vec<Se3> {
    let n = poses.len();
    if n < 2 {
        return poses.to_vec();
    }
    let twists: Vec<[f64; 6]> = poses.windows(2).map(|w| w[1].semio_compose_rs(&w[0].inverse()).log()).collect();
    let smoothed_twists = if twists.len() >= 3 {
        let win = odd_window(window, twists.len());
        let order = STABILIZE_SG_ORDER.min(win - 1);
        let channels: Vec<Vec<f64>> = (0..6).map(|c| semio_s_artifact_remodel_remodeling_signal::standards::v1::subsets::any::schema::signal_internals::savitzky_golay(&twists.iter().map(|t| t[c]).collect::<Vec<f64>>(), win, order, 0, 1.0)).collect();
        (0..twists.len()).map(|i| std::array::from_fn(|c| channels[c][i])).collect()
    } else {
        twists
    };
    let mut out = Vec::with_capacity(n);
    out.push(poses[0]);
    for xi in &smoothed_twists {
        let prev = *out.last().expect("out seeded with poses[0]");
        out.push(Se3::exp(*xi).semio_compose_rs(&prev));
    }
    out
}

fn intrinsics_matrix(intr: &Intrinsics) -> [[f64; 3]; 3] {
    [[intr.fx, intr.skew, intr.cx], [0.0, intr.fy, intr.cy], [0.0, 0.0, 1.0]]
}

fn intrinsics_matrix_inverse(intr: &Intrinsics) -> [[f64; 3]; 3] {
    let (fx, fy, cx, cy, skew) = (intr.fx, intr.fy, intr.cx, intr.cy, intr.skew);
    [[1.0 / fx, -skew / (fx * fy), (skew * cy - cx * fy) / (fx * fy)], [0.0, 1.0 / fy, -cy / fy], [0.0, 0.0, 1.0]]
}

/// 🪟️ Per-frame pure-rotation stabilization homography `H = K · ΔR · K⁻¹` warping the original jittery
/// viewpoint into the smoothed one, appropriate for a distant scene where translational parallax is
/// negligible; `ΔR` is the rotation from `original[i]`'s orientation to `smoothed[i]`'s.
pub fn stabilization_warps(original: &[Se3], smoothed: &[Se3], intr: &Intrinsics) -> Vec<[[f64; 3]; 3]> {
    let k = intrinsics_matrix(intr);
    let k_inv = intrinsics_matrix_inverse(intr);
    original
        .iter()
        .zip(smoothed.iter())
        .map(|(orig, smooth)| {
            let r_orig: [[f64; 3]; 3] = std::array::from_fn(|row| std::array::from_fn(|col| orig.r.0.cols[col][row]));
            let r_smooth: [[f64; 3]; 3] = std::array::from_fn(|row| std::array::from_fn(|col| smooth.r.0.cols[col][row]));
            let r_orig_t: [[f64; 3]; 3] = std::array::from_fn(|row| std::array::from_fn(|col| r_orig[col][row]));
            let r_rel = mat3_mul(&r_smooth, &r_orig_t);
            mat3_mul(&mat3_mul(&k, &r_rel), &k_inv)
        })
        .collect()
}
// #endregion 🔖️Stabilize

// #region 🔖️Deblur
/// 🌫️ A small square motion-blur point-spread function, normalized to sum to 1.
#[derive(Clone, Debug, PartialEq)]
pub struct Psf {
    pub kernel: Vec<f32>,
    pub width: u32,
    pub height: u32,
}

const DEBLUR_KERNEL_MARGIN: f32 = 2.0;

const DEBLUR_WIENER_EPS: f32 = 1e-6;

fn splat_bilinear(kernel: &mut [f32], size: u32, x: f32, y: f32, weight: f32) {
    let x0 = x.floor();
    let y0 = y.floor();
    let (fx, fy) = (x - x0, y - y0);
    for (dx, dy, w) in [(0.0, 0.0, (1.0 - fx) * (1.0 - fy)), (1.0, 0.0, fx * (1.0 - fy)), (0.0, 1.0, (1.0 - fx) * fy), (1.0, 1.0, fx * fy)] {
        let px = x0 + dx;
        let py = y0 + dy;
        if px >= 0.0 && py >= 0.0 && (px as u32) < size && (py as u32) < size {
            kernel[(py as u32 * size + px as u32) as usize] += weight * w;
        }
    }
}

/// 💨️ Simple linear motion-blur kernel: a line segment of length `speed * exposure_fraction` (clamped to at
/// least 1px) along the velocity direction, rasterized into a small square kernel via bilinear splatting
/// and normalized to sum to 1.
pub fn estimate_motion_psf(track_velocity_px_per_frame: (f32, f32), exposure_fraction: f32) -> Psf {
    let (vx, vy) = track_velocity_px_per_frame;
    let speed = vx.hypot(vy);
    let length = (speed * exposure_fraction.clamp(0.0, 1.0)).max(1.0);
    let size = (((length + DEBLUR_KERNEL_MARGIN * 2.0).ceil() as u32).max(3)) | 1;
    let mut kernel = vec![0.0f32; (size * size) as usize];
    let center = size as f32 / 2.0;
    let (dir_x, dir_y) = if speed > 1e-6 { (vx / speed, vy / speed) } else { (1.0, 0.0) };
    let steps = ((length.ceil() as usize) * 4).max(2);
    for s in 0..=steps {
        let t = (s as f32 / steps as f32 - 0.5) * length;
        splat_bilinear(&mut kernel, size, center + dir_x * t, center + dir_y * t, 1.0);
    }
    let sum: f32 = kernel.iter().sum();
    if sum > 0.0 {
        for v in &mut kernel {
            *v /= sum;
        }
    }
    Psf { kernel, width: size, height: size }
}

/// 🔍️ Frequency-domain Wiener deconvolution against a known [`Psf`]: `G = conj(H) / (|H|² + 1/snr)` applied
/// to the DFT of the (zero-padded, power-of-two) image via [`crate::standards::v1::subsets::any::schema::signal_internals::fft2`]/[`crate::standards::v1::subsets::any::schema::signal_internals::ifft2`],
/// with the PSF's center wrapped to the origin for circular convolution, then cropped back to the input size.
/// <https://en.wikipedia.org/wiki/Wiener_deconvolution>
pub fn wiener_deconvolve(img: &ImageGray, psf: &Psf, snr: f32) -> ImageGray {
    if img.width == 0 || img.height == 0 || psf.width == 0 || psf.height == 0 {
        return img.clone();
    }
    let pad_w = semio_s_artifact_remodel_remodeling_signal::standards::v1::subsets::any::schema::signal_internals::next_pow2((img.width + psf.width) as usize);
    let pad_h = semio_s_artifact_remodel_remodeling_signal::standards::v1::subsets::any::schema::signal_internals::next_pow2((img.height + psf.height) as usize);
    let mut img_re = vec![0.0f64; pad_w * pad_h];
    let mut img_im = vec![0.0f64; pad_w * pad_h];
    for y in 0..img.height {
        for x in 0..img.width {
            img_re[y as usize * pad_w + x as usize] = f64::from(img.get(x, y));
        }
    }
    let mut psf_re = vec![0.0f64; pad_w * pad_h];
    let mut psf_im = vec![0.0f64; pad_w * pad_h];
    let cx = i64::from(psf.width / 2);
    let cy = i64::from(psf.height / 2);
    for y in 0..psf.height {
        for x in 0..psf.width {
            let value = psf.kernel[(y * psf.width + x) as usize];
            let ox = (i64::from(x) - cx).rem_euclid(pad_w as i64) as usize;
            let oy = (i64::from(y) - cy).rem_euclid(pad_h as i64) as usize;
            psf_re[oy * pad_w + ox] += f64::from(value);
        }
    }
    semio_s_artifact_remodel_remodeling_signal::standards::v1::subsets::any::schema::signal_internals::fft2(&mut img_re, &mut img_im, pad_w, pad_h);
    semio_s_artifact_remodel_remodeling_signal::standards::v1::subsets::any::schema::signal_internals::fft2(&mut psf_re, &mut psf_im, pad_w, pad_h);
    let inv_snr = 1.0f64 / f64::from(snr.max(DEBLUR_WIENER_EPS));
    let mut out_re = vec![0.0f64; pad_w * pad_h];
    let mut out_im = vec![0.0f64; pad_w * pad_h];
    for i in 0..pad_w * pad_h {
        let (hr, hi) = (psf_re[i], psf_im[i]);
        let denom = hr * hr + hi * hi + inv_snr;
        let (gr, gi) = if denom > 1e-300 { (hr / denom, -hi / denom) } else { (0.0, 0.0) };
        out_re[i] = gr * img_re[i] - gi * img_im[i];
        out_im[i] = gr * img_im[i] + gi * img_re[i];
    }
    semio_s_artifact_remodel_remodeling_signal::standards::v1::subsets::any::schema::signal_internals::ifft2(&mut out_re, &mut out_im, pad_w, pad_h);
    let mut result = ImageGray::new(img.width, img.height);
    for y in 0..img.height {
        for x in 0..img.width {
            result.set(x, y, out_re[y as usize * pad_w + x as usize].clamp(0.0, 1.0) as f32);
        }
    }
    result
}
// #endregion 🔖️Deblur
