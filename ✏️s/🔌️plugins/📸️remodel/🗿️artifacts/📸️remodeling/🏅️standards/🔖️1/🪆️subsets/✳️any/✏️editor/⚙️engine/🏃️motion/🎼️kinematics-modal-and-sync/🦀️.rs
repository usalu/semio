//! 🎼️ Kinematics, vibration and synchronisation of the videogrammetry: the strain and kinematic fields, the modal analysis and the camera sync. See [`super`].

use super::*;

// #region 🔖️Kinematics
const KINEMATICS_SG_WINDOW: usize = 5;

const KINEMATICS_SG_ORDER: usize = 2;

pub(super) fn odd_window(desired: usize, max_len: usize) -> usize {
    let cap = desired.min(max_len);
    let odd = if cap.is_multiple_of(2) { cap.saturating_sub(1) } else { cap };
    odd.max(3)
}

fn mean_dt(traj: &Trajectory3d) -> f64 {
    let n = traj.samples.len();
    if n < 2 {
        return 1.0;
    }
    let span = traj.samples[n - 1].0 - traj.samples[0].0;
    if span <= 0.0 {
        1.0
    } else {
        span / (n - 1) as f64
    }
}

/// 📈️ Shared Savitzky-Golay derivative kernel for [`velocity`]/[`acceleration`]: assumes roughly-uniform
/// sampling (`dt` = mean spacing between consecutive timestamps) and applies
/// [`crate::standards::v1::subsets::any::schema::signal_internals::savitzky_golay`] independently to each of the x/y/z channels. Falls back to all
/// zeros when there are too few samples for even the smallest valid odd window (`< 3` points).
fn sg_derivative(traj: &Trajectory3d, deriv: usize) -> Vec<[f64; 3]> {
    let n = traj.samples.len();
    if n < 3 {
        return vec![[0.0; 3]; n];
    }
    let dt = mean_dt(traj);
    let window = odd_window(KINEMATICS_SG_WINDOW, n);
    let order = KINEMATICS_SG_ORDER.min(window - 1);
    let deriv = deriv.min(order).min(2);
    let xs: Vec<f64> = traj.samples.iter().map(|s| s.1[0]).collect();
    let ys: Vec<f64> = traj.samples.iter().map(|s| s.1[1]).collect();
    let zs: Vec<f64> = traj.samples.iter().map(|s| s.1[2]).collect();
    let vx = semio_s_artifact_remodel_remodeling_signal::standards::v1::subsets::any::schema::signal_internals::savitzky_golay(&xs, window, order, deriv, dt);
    let vy = semio_s_artifact_remodel_remodeling_signal::standards::v1::subsets::any::schema::signal_internals::savitzky_golay(&ys, window, order, deriv, dt);
    let vz = semio_s_artifact_remodel_remodeling_signal::standards::v1::subsets::any::schema::signal_internals::savitzky_golay(&zs, window, order, deriv, dt);
    (0..n).map(|i| [vx[i], vy[i], vz[i]]).collect()
}

/// ⚡️ Per-sample velocity of a [`Trajectory3d`] via first-order Savitzky-Golay differentiation, assuming
/// roughly-uniform timestamp spacing (see [`sg_derivative`]).
pub fn velocity(traj: &Trajectory3d) -> Vec<[f64; 3]> {
    sg_derivative(traj, 1)
}

/// 🚀️ Per-sample acceleration of a [`Trajectory3d`] via second-order Savitzky-Golay differentiation,
/// assuming roughly-uniform timestamp spacing (see [`sg_derivative`]).
pub fn acceleration(traj: &Trajectory3d) -> Vec<[f64; 3]> {
    sg_derivative(traj, 2)
}

/// 📏️ Polyline arc length: the sum of Euclidean distances between consecutive samples.
pub fn arc_length(traj: &Trajectory3d) -> f64 {
    traj.samples.windows(2).map(|w| dist3(w[0].1, w[1].1)).sum()
}

/// 🪢️ One neighbourhood edge's Green-Lagrange strain magnitude from [`neighborhood_affine_strain`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StrainPair {
    pub point_a: usize,
    pub point_b: usize,
    pub strain: f64,
}

const STRAIN_MIN_NEIGHBORS: usize = 3;

/// 🧩️ Solves the best-fit local linear map `F` (`d1_k ≈ F d0_k`) for a neighbourhood's offset vectors via
/// independent per-row least squares ([`solve_llsq`]) — this crate depends on `crate::algebra`
/// (already required transitively via `remodeling_camera`/`remodeling_sfm`), so a hand-rolled 3x3 normal-equation
/// solve would just duplicate its QR-based `solve_llsq`.
fn fit_affine_3x3(d0: &[[f64; 3]], d1: &[[f64; 3]]) -> Option<[[f64; 3]; 3]> {
    let mut a = MatD::zeros(d0.len(), 3);
    for (row, p) in d0.iter().enumerate() {
        a.set(row, 0, p[0]);
        a.set(row, 1, p[1]);
        a.set(row, 2, p[2]);
    }
    let mut f = [[0.0; 3]; 3];
    for (r, row) in f.iter_mut().enumerate() {
        let b = VecD::from_vec(d1.iter().map(|p| p[r]).collect());
        let x = solve_llsq(&a, &b).ok()?;
        *row = [x.get(0), x.get(1), x.get(2)];
    }
    Some(f)
}

fn green_lagrange_frobenius(f: &[[f64; 3]; 3]) -> f64 {
    let mut sum_sq = 0.0;
    for i in 0..3 {
        for j in 0..3 {
            let ftf_ij: f64 = (0..3).map(|k| f[k][i] * f[k][j]).sum();
            let e = 0.5 * (ftf_ij - if i == j { 1.0 } else { 0.0 });
            sum_sq += e * e;
        }
    }
    sum_sq.sqrt()
}

/// 🧵️ Local (neighbourhood-level) affine deformation-gradient strain: for every point `i` in `cloud_t0`
/// with at least [`STRAIN_MIN_NEIGHBORS`] index-corresponding neighbours within `radius` (matched to
/// `cloud_t1` by shared index — the same point cloud at two instants), fits the best-fit `3x3` linear map
/// `F` ([`fit_affine_3x3`]) taking each neighbour's `t0`-relative offset to its `t1`-relative offset, then
/// reports the Green-Lagrange strain tensor's Frobenius norm `‖(FᵀF - I) / 2‖_F` (zero for a pure rotation,
/// since `FᵀF = I` there) once per `(point_a: i, point_b: neighbour)` edge that supported the fit.
/// <https://en.wikipedia.org/wiki/Finite_strain_theory>
pub fn neighborhood_affine_strain(cloud_t0: &[[f64; 3]], cloud_t1: &[[f64; 3]], radius: f64) -> Vec<StrainPair> {
    if cloud_t0.len() != cloud_t1.len() || cloud_t0.is_empty() {
        return Vec::new();
    }
    let tree = KdTree::<3>::build(cloud_t0);
    let mut out = Vec::new();
    for i in 0..cloud_t0.len() {
        let neighbor_idxs: Vec<usize> = tree.radius(&cloud_t0[i], radius).into_iter().filter_map(|(id, _)| (id as usize != i).then_some(id as usize)).collect();
        if neighbor_idxs.len() < STRAIN_MIN_NEIGHBORS {
            continue;
        }
        let d0: Vec<[f64; 3]> = neighbor_idxs.iter().map(|&j| sub3(cloud_t0[j], cloud_t0[i])).collect();
        let d1: Vec<[f64; 3]> = neighbor_idxs.iter().map(|&j| sub3(cloud_t1[j], cloud_t1[i])).collect();
        let Some(f) = fit_affine_3x3(&d0, &d1) else { continue };
        let strain = green_lagrange_frobenius(&f);
        for &j in &neighbor_idxs {
            out.push(StrainPair { point_a: i, point_b: j, strain });
        }
    }
    out
}
// #endregion 🔖️Kinematics

// #region 🔖️Modal
/// 🎼️ Extracted vibration modes from [`modal_analysis`]: parallel arrays indexed by mode, plus mode shapes
/// laid out `mode_shapes[mode][track] = cross-spectral magnitude between that track and the reference
/// track at the mode's frequency, signed by `cos(phase)` so in-phase vs. anti-phase motion (the coarse part
/// of the phase information a real-valued mode shape can carry) survives the fold into a single scalar.
#[derive(Clone, Debug, PartialEq)]
pub struct ModalResult {
    pub frequencies_hz: Vec<f64>,
    pub damping_ratios: Vec<f64>,
    pub mode_shapes: Vec<Vec<f64>>,
}

const MODAL_WELCH_SEG_LEN: usize = 512;

const MODAL_WELCH_OVERLAP: f64 = 0.5;

const MODAL_PROMINENCE_FRACTION: f64 = 0.05;

fn half_power_bandwidth_damping(psd: &[f64], peak_idx: usize, bin_hz: f64, freq_hz: f64) -> f64 {
    if freq_hz <= 0.0 {
        return 0.0;
    }
    let half_power = psd[peak_idx] * 0.5;
    let mut lo = peak_idx;
    while lo > 0 && psd[lo] > half_power {
        lo -= 1;
    }
    let mut hi = peak_idx;
    while hi + 1 < psd.len() && psd[hi] > half_power {
        hi += 1;
    }
    let bandwidth_hz = (hi - lo) as f64 * bin_hz;
    (bandwidth_hz / (2.0 * freq_hz)).max(0.0)
}

/// 🎵️ Per-track Welch-PSD modal analysis: extracts each track's y-displacement signal, computes
/// `reference_track`'s power spectral density ([`crate::standards::v1::subsets::any::schema::signal_internals::welch_psd`]), picks the strongest
/// `max_modes` peaks ([`crate::standards::v1::subsets::any::schema::signal_internals::find_peaks`]) above a PSD-relative prominence floor, estimates
/// each mode's half-power-bandwidth damping ratio, and derives every track's mode shape from cross-spectral
/// analysis ([`crate::standards::v1::subsets::any::schema::signal_internals::cross_spectrum`]) against the reference track at each modal frequency.
/// <https://en.wikipedia.org/wiki/Q_factor#Bandwidth_definition>
pub fn modal_analysis(tracks: &[Track2d], fps: f64, reference_track: usize, max_modes: usize) -> ModalResult {
    if tracks.is_empty() || fps <= 0.0 || max_modes == 0 || reference_track >= tracks.len() {
        return ModalResult { frequencies_hz: Vec::new(), damping_ratios: Vec::new(), mode_shapes: Vec::new() };
    }
    let signals: Vec<Vec<f64>> = tracks.iter().map(|t| t.samples.iter().map(|&(_, _, y)| f64::from(y)).collect()).collect();
    if signals[reference_track].len() < 4 {
        return ModalResult { frequencies_hz: Vec::new(), damping_ratios: Vec::new(), mode_shapes: Vec::new() };
    }
    let seg_len = MODAL_WELCH_SEG_LEN.min(signals[reference_track].len());
    let psd = semio_s_artifact_remodel_remodeling_signal::standards::v1::subsets::any::schema::signal_internals::welch_psd(&signals[reference_track], seg_len, MODAL_WELCH_OVERLAP);
    let nfft = semio_s_artifact_remodel_remodeling_signal::standards::v1::subsets::any::schema::signal_internals::next_pow2(seg_len);
    let bin_hz = fps / nfft as f64;
    let max_psd = psd.iter().copied().fold(0.0f64, f64::max);

    let mut peaks = semio_s_artifact_remodel_remodeling_signal::standards::v1::subsets::any::schema::signal_internals::find_peaks(&psd, max_psd * MODAL_PROMINENCE_FRACTION);
    peaks.sort_by(|a, b| b.value.total_cmp(&a.value));
    peaks.truncate(max_modes);
    peaks.sort_by_key(|p| p.index);

    let cross: Vec<(Vec<f64>, Vec<f64>)> = signals.iter().map(|sig| semio_s_artifact_remodel_remodeling_signal::standards::v1::subsets::any::schema::signal_internals::cross_spectrum(&signals[reference_track], sig, seg_len, MODAL_WELCH_OVERLAP)).collect();

    let mut frequencies_hz = Vec::with_capacity(peaks.len());
    let mut damping_ratios = Vec::with_capacity(peaks.len());
    let mut mode_shapes = Vec::with_capacity(peaks.len());
    for peak in &peaks {
        let freq = peak.index as f64 * bin_hz;
        frequencies_hz.push(freq);
        damping_ratios.push(half_power_bandwidth_damping(&psd, peak.index, bin_hz, freq));
        let shape: Vec<f64> = cross
            .iter()
            .map(|(mag, phase)| {
                let m = mag.get(peak.index).copied().unwrap_or(0.0);
                let p = phase.get(peak.index).copied().unwrap_or(0.0);
                m * p.cos()
            })
            .collect();
        mode_shapes.push(shape);
    }
    ModalResult { frequencies_hz, damping_ratios, mode_shapes }
}
// #endregion 🔖️Modal

// #region 🔖️Sync
/// ⏱️ Coarse-to-fine camera-sync result: fractional-frame offset of `motion_signal_b` relative to
/// `motion_signal_a`, and the peak normalized-correlation confidence backing it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SyncResult {
    pub offset_frames: f64,
    pub confidence: f64,
}

/// 🔗️ Coarse sync-offset estimate between two per-frame motion-activity signals (e.g. mean optical-flow
/// magnitude or track-position-change per frame): normalized cross-correlation
/// ([`crate::standards::v1::subsets::any::schema::signal_internals::xcorr_normalized`]) locates the lag, sub-sample-refined
/// ([`crate::standards::v1::subsets::any::schema::signal_internals::subsample_peak`]) to fractional-frame precision. `fps` is accepted for interface
/// symmetry with time-domain callers (and to mirror [`refine_subframe`]'s frame-domain contract) — the
/// correlation itself already operates in frame-index units, so it does not change the computation.
pub fn estimate_offset(motion_signal_a: &[f64], motion_signal_b: &[f64], fps: f64) -> SyncResult {
    let _ = fps;
    if motion_signal_a.is_empty() || motion_signal_b.is_empty() {
        return SyncResult { offset_frames: 0.0, confidence: 0.0 };
    }
    let max_lag = (motion_signal_a.len().min(motion_signal_b.len()) / 4).max(4);
    let xc = semio_s_artifact_remodel_remodeling_signal::standards::v1::subsets::any::schema::signal_internals::xcorr_normalized(motion_signal_a, motion_signal_b, max_lag);
    let confidence = xc.iter().copied().fold(f64::MIN, f64::max).max(0.0);
    let Some(peak_idx_f) = semio_s_artifact_remodel_remodeling_signal::standards::v1::subsets::any::schema::signal_internals::subsample_peak(&xc) else {
        return SyncResult { offset_frames: 0.0, confidence };
    };
    SyncResult { offset_frames: peak_idx_f - max_lag as f64, confidence }
}

fn correlation_at_offset(a: &[f64], b: &[f64], offset: f64) -> f64 {
    let mut sum_ab = 0.0;
    let mut sum_a2 = 0.0;
    let mut sum_b2 = 0.0;
    let mut count = 0usize;
    for (i, &av) in a.iter().enumerate() {
        let bi = i as f64 + offset;
        if bi < 0.0 || bi > (b.len() - 1) as f64 {
            continue;
        }
        let lo = bi.floor() as usize;
        let hi = (lo + 1).min(b.len() - 1);
        let frac = bi - lo as f64;
        let bv = b[lo] * (1.0 - frac) + b[hi] * frac;
        sum_ab += av * bv;
        sum_a2 += av * av;
        sum_b2 += bv * bv;
        count += 1;
    }
    if count == 0 || sum_a2 <= 0.0 || sum_b2 <= 0.0 {
        return -1.0;
    }
    sum_ab / (sum_a2.sqrt() * sum_b2.sqrt())
}

/// 🔬️ Sub-frame polish of a coarse sync offset: a local golden-section search
/// ([`crate::standards::v1::subsets::any::schema::optimize_internals::golden_section`]) over `[coarse_offset - 1, coarse_offset + 1]` maximizing the
/// linearly-interpolated normalized correlation between `motion_signal_a` and a continuously-shifted
/// `motion_signal_b`.
pub fn refine_subframe(motion_signal_a: &[f64], motion_signal_b: &[f64], coarse_offset: f64) -> f64 {
    if motion_signal_a.is_empty() || motion_signal_b.is_empty() {
        return coarse_offset;
    }
    let (best_offset, _best_value) = semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::golden_section(|o| -correlation_at_offset(motion_signal_a, motion_signal_b, o), coarse_offset - 1.0, coarse_offset + 1.0, 1e-4);
    best_offset
}
// #endregion 🔖️Sync
