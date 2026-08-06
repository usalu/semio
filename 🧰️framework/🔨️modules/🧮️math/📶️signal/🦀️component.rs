//! 📡️ Signal processing: FFT, windows, spectra, correlation, smoothing and peak analysis for time-series and modal analysis.

use crate::algebra::{MatD, VecD};
use std::f64::consts::PI;

// #region 🔖️Fft
/// 🔢️ Smallest power of two `>= n`; returns `1` for `n == 0`.
pub fn next_pow2(n: usize) -> usize {
    n.max(1).next_power_of_two()
}

/// 🌀️ In-place iterative radix-2 Cooley-Tukey DFT with bit-reversal permutation; asserts equal power-of-two lengths.
pub fn fft(re: &mut [f64], im: &mut [f64]) {
    assert_eq!(re.len(), im.len(), "fft length mismatch");
    let n = re.len();
    assert!(n.is_power_of_two(), "fft requires a power-of-two length");
    if n <= 1 {
        return;
    }
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut len = 2usize;
    while len <= n {
        let half = len / 2;
        for start in (0..n).step_by(len) {
            for k in 0..half {
                let angle = -2.0 * PI * k as f64 / len as f64;
                let (w_re, w_im) = (angle.cos(), angle.sin());
                let even = start + k;
                let odd = start + k + half;
                let t_re = re[odd] * w_re - im[odd] * w_im;
                let t_im = re[odd] * w_im + im[odd] * w_re;
                re[odd] = re[even] - t_re;
                im[odd] = im[even] - t_im;
                re[even] += t_re;
                im[even] += t_im;
            }
        }
        len <<= 1;
    }
}

/// 🔄️ In-place inverse DFT via the conjugate trick with `1/N` scaling; asserts equal power-of-two lengths.
pub fn ifft(re: &mut [f64], im: &mut [f64]) {
    let n = re.len();
    for v in im.iter_mut() {
        *v = -*v;
    }
    fft(re, im);
    let scale = 1.0 / n.max(1) as f64;
    for v in re.iter_mut() {
        *v *= scale;
    }
    for v in im.iter_mut() {
        *v = -*v * scale;
    }
}

/// 🗺️ In-place 2D DFT of a row-major `w * h` grid: row passes then column passes through a scratch column buffer.
pub fn fft2(re: &mut [f64], im: &mut [f64], w: usize, h: usize) {
    assert_eq!(re.len(), w * h, "fft2 size mismatch");
    assert_eq!(im.len(), w * h, "fft2 size mismatch");
    for row in 0..h {
        fft(&mut re[row * w..(row + 1) * w], &mut im[row * w..(row + 1) * w]);
    }
    let mut col_re = vec![0.0; h];
    let mut col_im = vec![0.0; h];
    for col in 0..w {
        for row in 0..h {
            col_re[row] = re[row * w + col];
            col_im[row] = im[row * w + col];
        }
        fft(&mut col_re, &mut col_im);
        for row in 0..h {
            re[row * w + col] = col_re[row];
            im[row * w + col] = col_im[row];
        }
    }
}

/// 🔄️ In-place 2D inverse DFT of a row-major `w * h` grid via the conjugate trick with `1/(w*h)` scaling.
pub fn ifft2(re: &mut [f64], im: &mut [f64], w: usize, h: usize) {
    for v in im.iter_mut() {
        *v = -*v;
    }
    fft2(re, im, w, h);
    let scale = 1.0 / (w * h).max(1) as f64;
    for v in re.iter_mut() {
        *v *= scale;
    }
    for v in im.iter_mut() {
        *v = -*v * scale;
    }
}
// #endregion 🔖️Fft

// #region 🔖️Window
/// 🪟️ Symmetric Hann window of length `n`.
pub fn hann(n: usize) -> Vec<f64> {
    cosine_window(n, &[0.5, -0.5, 0.0])
}

/// 🪟️ Symmetric Hamming window of length `n`.
pub fn hamming(n: usize) -> Vec<f64> {
    cosine_window(n, &[0.54, -0.46, 0.0])
}

/// 🪟️ Symmetric Blackman window of length `n`.
pub fn blackman(n: usize) -> Vec<f64> {
    cosine_window(n, &[0.42, -0.5, 0.08])
}

fn cosine_window(n: usize, coeffs: &[f64; 3]) -> Vec<f64> {
    if n <= 1 {
        return vec![1.0; n];
    }
    (0..n)
        .map(|i| {
            let phase = 2.0 * PI * i as f64 / (n - 1) as f64;
            coeffs[0] + coeffs[1] * phase.cos() + coeffs[2] * (2.0 * phase).cos()
        })
        .collect()
}
// #endregion 🔖️Window

// #region 🔖️Spectrum
/// 📊️ Welch one-sided power spectral density: Hann-windowed segments with fractional `overlap`, zero-padded to a power of two, averaged periodograms with window-power normalization; bin `k` maps to frequency `k * fs / next_pow2(seg_len)`.
pub fn welch_psd(x: &[f64], seg_len: usize, overlap: f64) -> Vec<f64> {
    let spectra = averaged_segment_spectra(&[x, x], seg_len, overlap);
    let nfft = next_pow2(seg_len);
    let bins = nfft / 2 + 1;
    (0..bins)
        .map(|k| {
            let one_sided = if k == 0 || k == nfft / 2 { 1.0 } else { 2.0 };
            one_sided * spectra[k].0
        })
        .collect()
}

/// 🔀️ Averaged one-sided cross-spectral density `S_ab = conj(A) * B` over Hann-windowed overlapping segments, returned as `(magnitude, phase)`; a delay of `d` samples in `b` shows as phase `-2 * PI * k * d / next_pow2(seg_len)` at bin `k`.
pub fn cross_spectrum(a: &[f64], b: &[f64], seg_len: usize, overlap: f64) -> (Vec<f64>, Vec<f64>) {
    let spectra = averaged_segment_spectra(&[a, b], seg_len, overlap);
    let nfft = next_pow2(seg_len);
    let bins = nfft / 2 + 1;
    let mut magnitude = Vec::with_capacity(bins);
    let mut phase = Vec::with_capacity(bins);
    for s in spectra.iter().take(bins) {
        magnitude.push((s.0 * s.0 + s.1 * s.1).sqrt());
        phase.push(s.1.atan2(s.0));
    }
    (magnitude, phase)
}

fn averaged_segment_spectra(signals: &[&[f64]; 2], seg_len: usize, overlap: f64) -> Vec<(f64, f64)> {
    let (a, b) = (signals[0], signals[1]);
    assert!(!a.is_empty() && !b.is_empty(), "spectrum requires non-empty signals");
    assert!(seg_len >= 2, "spectrum requires seg_len >= 2");
    assert!((0.0..1.0).contains(&overlap), "overlap must be in [0, 1)");
    let n = a.len().min(b.len());
    let seg = seg_len.min(n);
    let hop = ((seg as f64 * (1.0 - overlap)).floor() as usize).max(1);
    let window = hann(seg);
    let window_power: f64 = window.iter().map(|w| w * w).sum();
    let nfft = next_pow2(seg_len);
    let mut acc = vec![(0.0, 0.0); nfft];
    let mut count = 0usize;
    let mut start = 0usize;
    while start + seg <= n {
        let mut a_re = vec![0.0; nfft];
        let mut a_im = vec![0.0; nfft];
        let mut b_re = vec![0.0; nfft];
        let mut b_im = vec![0.0; nfft];
        for (i, w) in window.iter().enumerate() {
            a_re[i] = a[start + i] * w;
            b_re[i] = b[start + i] * w;
        }
        fft(&mut a_re, &mut a_im);
        fft(&mut b_re, &mut b_im);
        for k in 0..nfft {
            acc[k].0 += (a_re[k] * b_re[k] + a_im[k] * b_im[k]) / window_power;
            acc[k].1 += (a_re[k] * b_im[k] - a_im[k] * b_re[k]) / window_power;
        }
        count += 1;
        start += hop;
    }
    let scale = 1.0 / count.max(1) as f64;
    for s in acc.iter_mut() {
        s.0 *= scale;
        s.1 *= scale;
    }
    acc
}
// #endregion 🔖️Spectrum

// #region 🔖️Correlate
/// 🔗️ Mean-removed, unit-energy normalized cross-correlation for lags `-max_lag..=max_lag`, computed via FFT with zero-padding to `next_pow2(len_a + len_b)`; a delay of `d` samples in `b` peaks at output index `max_lag + d`.
pub fn xcorr_normalized(a: &[f64], b: &[f64], max_lag: usize) -> Vec<f64> {
    assert!(!a.is_empty() && !b.is_empty(), "xcorr requires non-empty signals");
    let mean_a = a.iter().sum::<f64>() / a.len() as f64;
    let mean_b = b.iter().sum::<f64>() / b.len() as f64;
    let nfft = next_pow2(a.len() + b.len());
    let mut a_re = vec![0.0; nfft];
    let mut a_im = vec![0.0; nfft];
    let mut b_re = vec![0.0; nfft];
    let mut b_im = vec![0.0; nfft];
    for (slot, v) in a_re.iter_mut().zip(a.iter()) {
        *slot = v - mean_a;
    }
    for (slot, v) in b_re.iter_mut().zip(b.iter()) {
        *slot = v - mean_b;
    }
    let energy_a: f64 = a_re.iter().map(|v| v * v).sum();
    let energy_b: f64 = b_re.iter().map(|v| v * v).sum();
    let norm = (energy_a * energy_b).sqrt();
    fft(&mut a_re, &mut a_im);
    fft(&mut b_re, &mut b_im);
    let mut c_re = vec![0.0; nfft];
    let mut c_im = vec![0.0; nfft];
    for k in 0..nfft {
        c_re[k] = a_re[k] * b_re[k] + a_im[k] * b_im[k];
        c_im[k] = a_re[k] * b_im[k] - a_im[k] * b_re[k];
    }
    ifft(&mut c_re, &mut c_im);
    let inv_norm = if norm > 0.0 { 1.0 / norm } else { 0.0 };
    (0..=2 * max_lag)
        .map(|i| {
            let lag = i as isize - max_lag as isize;
            let index = lag.rem_euclid(nfft as isize) as usize;
            c_re[index] * inv_norm
        })
        .collect()
}

/// 🎯️ Index of the maximum refined by parabolic 3-point interpolation; `None` when `c` is empty or the peak sits on an edge.
pub fn subsample_peak(c: &[f64]) -> Option<f64> {
    if c.is_empty() {
        return None;
    }
    let mut peak = 0usize;
    for (i, v) in c.iter().enumerate() {
        if *v > c[peak] {
            peak = i;
        }
    }
    if peak == 0 || peak == c.len() - 1 {
        return None;
    }
    let (left, center, right) = (c[peak - 1], c[peak], c[peak + 1]);
    let denom = left - 2.0 * center + right;
    let offset = if denom.abs() > 1e-300 { 0.5 * (left - right) / denom } else { 0.0 };
    Some(peak as f64 + offset)
}
// #endregion 🔖️Correlate

// #region 🔖️Smooth
/// 📉️ Savitzky-Golay filter with mirror-padded edges: least-squares polynomial of `order` on an odd `window`, returning the `deriv`-th derivative (0..=2) scaled by `deriv! / dt^deriv`; coefficients come from the Vandermonde normal equations solved once via `MatD::lu_solve`.
pub fn savitzky_golay(x: &[f64], window: usize, order: usize, deriv: usize, dt: f64) -> Vec<f64> {
    assert!(window % 2 == 1 && window >= 3, "savitzky_golay requires an odd window >= 3");
    assert!(order < window, "savitzky_golay requires order < window");
    assert!(deriv <= order && deriv <= 2, "savitzky_golay requires deriv <= min(order, 2)");
    assert!(dt > 0.0, "savitzky_golay requires dt > 0");
    if x.is_empty() {
        return Vec::new();
    }
    let half = window / 2;
    let terms = order + 1;
    let mut design = MatD::zeros(window, terms);
    for i in 0..window {
        let t = i as f64 - half as f64;
        let mut power = 1.0;
        for j in 0..terms {
            design.set(i, j, power);
            power *= t;
        }
    }
    let normal = design.transpose().matmul(&design);
    let mut unit = VecD::zeros(terms);
    unit.set(deriv, 1.0);
    let solution = normal.lu_solve(&unit).expect("savitzky_golay normal equations are non-singular");
    let factorial: f64 = (1..=deriv).map(|k| k as f64).product::<f64>().max(1.0);
    let scale = factorial / dt.powi(deriv as i32);
    let weights: Vec<f64> = (0..window)
        .map(|i| {
            let mut sum = 0.0;
            for j in 0..terms {
                sum += solution.get(j) * design.get(i, j);
            }
            sum * scale
        })
        .collect();
    convolve_mirrored(x, &weights, half)
}

/// 🌫️ Gaussian smoothing with a normalized kernel of radius `3 * sigma` and mirror-padded edges; returns the input unchanged for `sigma <= 0`.
pub fn gaussian_smooth_1d(x: &[f64], sigma: f64) -> Vec<f64> {
    if sigma <= 0.0 || x.is_empty() {
        return x.to_vec();
    }
    let radius = (3.0 * sigma).ceil() as usize;
    let mut kernel: Vec<f64> = (0..=2 * radius)
        .map(|i| {
            let t = i as f64 - radius as f64;
            (-t * t / (2.0 * sigma * sigma)).exp()
        })
        .collect();
    let total: f64 = kernel.iter().sum();
    for w in kernel.iter_mut() {
        *w /= total;
    }
    convolve_mirrored(x, &kernel, radius)
}

/// 📏️ Centered moving average over `window` samples with mirror-padded edges.
pub fn moving_average(x: &[f64], window: usize) -> Vec<f64> {
    assert!(window >= 1, "moving_average requires window >= 1");
    if x.is_empty() {
        return Vec::new();
    }
    let kernel = vec![1.0 / window as f64; window];
    convolve_mirrored(x, &kernel, window / 2)
}

fn convolve_mirrored(x: &[f64], kernel: &[f64], center: usize) -> Vec<f64> {
    let n = x.len();
    (0..n)
        .map(|t| {
            let mut sum = 0.0;
            for (i, w) in kernel.iter().enumerate() {
                let offset = t as isize + i as isize - center as isize;
                sum += w * x[mirror_index(offset, n)];
            }
            sum
        })
        .collect()
}

fn mirror_index(offset: isize, n: usize) -> usize {
    if n == 1 {
        return 0;
    }
    let period = 2 * (n as isize - 1);
    let mut m = offset.rem_euclid(period);
    if m >= n as isize {
        m = period - m;
    }
    m as usize
}
// #endregion 🔖️Smooth

// #region 🔖️Peaks
/// ⛰️ A detected local maximum with its topographic prominence.
#[derive(Clone, Debug, PartialEq)]
pub struct PeakInfo {
    pub index: usize,
    pub value: f64,
    pub prominence: f64,
}

/// 🏔️ Strict local maxima with prominence measured to the lowest saddle toward higher terrain on each side (signal edge when no higher terrain exists), filtered by `min_prominence`.
pub fn find_peaks(x: &[f64], min_prominence: f64) -> Vec<PeakInfo> {
    let mut peaks = Vec::new();
    if x.len() < 3 {
        return peaks;
    }
    for i in 1..x.len() - 1 {
        if !(x[i] > x[i - 1] && x[i] > x[i + 1]) {
            continue;
        }
        let mut left_min = x[i];
        let mut j = i;
        while j > 0 {
            j -= 1;
            if x[j] > x[i] {
                break;
            }
            left_min = left_min.min(x[j]);
        }
        let mut right_min = x[i];
        let mut k = i;
        while k < x.len() - 1 {
            k += 1;
            if x[k] > x[i] {
                break;
            }
            right_min = right_min.min(x[k]);
        }
        let prominence = x[i] - left_min.max(right_min);
        if prominence >= min_prominence {
            peaks.push(PeakInfo { index: i, value: x[i], prominence });
        }
    }
    peaks
}
// #endregion 🔖️Peaks

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn seeded_noise(n: usize, seed: u64) -> Vec<f64> {
        let mut state = seed;
        (0..n)
            .map(|_| {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                (state >> 11) as f64 / (1u64 << 53) as f64 - 0.5
            })
            .collect()
    }

    #[test]
    fn fft_ifft_round_trips() {
        let n = 256;
        let original_re = seeded_noise(n, 7);
        let original_im = seeded_noise(n, 13);
        let mut re = original_re.clone();
        let mut im = original_im.clone();
        fft(&mut re, &mut im);
        ifft(&mut re, &mut im);
        for i in 0..n {
            assert!((re[i] - original_re[i]).abs() < 1e-9);
            assert!((im[i] - original_im[i]).abs() < 1e-9);
        }
    }

    #[test]
    fn fft_satisfies_parseval() {
        let n = 256;
        let time_re = seeded_noise(n, 21);
        let time_im = seeded_noise(n, 42);
        let time_energy: f64 = time_re.iter().zip(time_im.iter()).map(|(r, i)| r * r + i * i).sum();
        let mut re = time_re;
        let mut im = time_im;
        fft(&mut re, &mut im);
        let freq_energy: f64 = re.iter().zip(im.iter()).map(|(r, i)| (r * r + i * i) / n as f64).sum();
        assert!((time_energy - freq_energy).abs() < 1e-9);
    }

    #[test]
    fn fft2_ifft2_round_trips() {
        let (w, h) = (8, 4);
        let original = seeded_noise(w * h, 3);
        let mut re = original.clone();
        let mut im = vec![0.0; w * h];
        fft2(&mut re, &mut im, w, h);
        ifft2(&mut re, &mut im, w, h);
        for i in 0..w * h {
            assert!((re[i] - original[i]).abs() < 1e-9);
            assert!(im[i].abs() < 1e-9);
        }
    }

    #[test]
    fn windows_have_expected_shape() {
        for window in [hann(64), hamming(64), blackman(64)] {
            assert_eq!(window.len(), 64);
            let peak = window.iter().fold(0.0f64, |acc, v| acc.max(*v));
            assert!((peak - 1.0).abs() < 0.01);
            assert!(window[0] < 0.1);
            assert!((window[0] - window[63]).abs() < 1e-12);
        }
    }

    #[test]
    fn welch_psd_finds_planted_sinusoid() {
        let fs = 100.0;
        let x: Vec<f64> = (0..1000).map(|t| (2.0 * PI * 7.0 * t as f64 / fs).sin()).collect();
        let psd = welch_psd(&x, 256, 0.5);
        assert_eq!(psd.len(), 129);
        let mut peak_bin = 0;
        for (k, v) in psd.iter().enumerate() {
            if *v > psd[peak_bin] {
                peak_bin = k;
            }
        }
        let peak_freq = peak_bin as f64 * fs / 256.0;
        assert!((peak_freq - 7.0).abs() < 0.5);
    }

    #[test]
    fn cross_spectrum_phase_encodes_lag() {
        let cycles = 16.0;
        let delay = 3.0;
        let a: Vec<f64> = (0..600).map(|t| (2.0 * PI * cycles * t as f64 / 256.0).sin()).collect();
        let b: Vec<f64> = (0..600).map(|t| (2.0 * PI * cycles * (t as f64 - delay) / 256.0).sin()).collect();
        let (magnitude, phase) = cross_spectrum(&a, &b, 256, 0.5);
        let mut peak_bin = 0;
        for (k, v) in magnitude.iter().enumerate() {
            if *v > magnitude[peak_bin] {
                peak_bin = k;
            }
        }
        assert_eq!(peak_bin, 16);
        let expected = -2.0 * PI * cycles * delay / 256.0;
        assert!((phase[peak_bin] - expected).abs() < 1e-2);
    }

    #[test]
    fn xcorr_peaks_at_known_shift() {
        let base = gaussian_smooth_1d(&seeded_noise(200, 99), 2.0);
        let mut shifted = vec![0.0; 200];
        shifted[5..200].copy_from_slice(&base[0..195]);
        let c = xcorr_normalized(&base, &shifted, 10);
        assert_eq!(c.len(), 21);
        let mut peak = 0;
        for (i, v) in c.iter().enumerate() {
            if *v > c[peak] {
                peak = i;
            }
        }
        assert_eq!(peak, 15);
        assert!(c[peak] > 0.95 && c[peak] <= 1.0 + 1e-9);
    }

    #[test]
    fn subsample_peak_recovers_fractional_lag() {
        let pulse = |center: f64| -> Vec<f64> { (0..128).map(|t| (-((t as f64 - center) * (t as f64 - center)) / (2.0 * 16.0)).exp()).collect() };
        let a = pulse(50.0);
        let b = pulse(53.37);
        let c = xcorr_normalized(&a, &b, 10);
        let refined = subsample_peak(&c).expect("interior peak");
        assert!((refined - (10.0 + 3.37)).abs() < 0.05);
        assert_eq!(subsample_peak(&[]), None);
        assert_eq!(subsample_peak(&[3.0, 2.0, 1.0]), None);
    }

    #[test]
    fn savitzky_golay_differentiates_cubic() {
        let dt = 0.1;
        let x: Vec<f64> = (0..50).map(|i| (i as f64 * dt).powi(3)).collect();
        let d1 = savitzky_golay(&x, 7, 3, 1, dt);
        for (i, &value) in d1.iter().enumerate().take(47).skip(3) {
            let expected = 3.0 * (i as f64 * dt).powi(2);
            assert!((value - expected).abs() < 1e-6);
        }
    }

    #[test]
    fn savitzky_golay_preserves_low_order_polynomials() {
        let x: Vec<f64> = (0..40).map(|i| 2.0 + 0.5 * i as f64 + 0.03 * (i * i) as f64).collect();
        let smoothed = savitzky_golay(&x, 5, 2, 0, 1.0);
        for i in 2..38 {
            assert!((smoothed[i] - x[i]).abs() < 1e-9);
        }
    }

    #[test]
    fn smoothing_preserves_constant_signals() {
        let x = vec![4.2; 30];
        for out in [gaussian_smooth_1d(&x, 1.5), moving_average(&x, 5), savitzky_golay(&x, 7, 2, 0, 1.0)] {
            for v in out {
                assert!((v - 4.2).abs() < 1e-9);
            }
        }
    }

    #[test]
    fn find_peaks_ranks_two_bumps_by_prominence() {
        let x: Vec<f64> = (0..100)
            .map(|t| {
                let bump = |center: f64, height: f64| height * (-((t as f64 - center) * (t as f64 - center)) / 18.0).exp();
                bump(30.0, 1.0) + bump(70.0, 0.6)
            })
            .collect();
        let peaks = find_peaks(&x, 0.1);
        assert_eq!(peaks.len(), 2);
        assert_eq!(peaks[0].index, 30);
        assert_eq!(peaks[1].index, 70);
        assert!(peaks[0].prominence > peaks[1].prominence);
        assert!((peaks[0].prominence - 1.0).abs() < 0.01);
        assert!((peaks[1].prominence - 0.6).abs() < 0.01);
    }
}
// #endregion 🔖️Tests
