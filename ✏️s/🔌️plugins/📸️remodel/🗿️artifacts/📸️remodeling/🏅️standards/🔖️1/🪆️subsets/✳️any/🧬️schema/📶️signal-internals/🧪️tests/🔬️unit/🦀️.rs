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

#[semio_framework_async_macros::async_test]
async fn fft_ifft_round_trips() {
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

#[semio_framework_async_macros::async_test]
async fn fft_satisfies_parseval() {
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

#[semio_framework_async_macros::async_test]
async fn fft2_ifft2_round_trips() {
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

#[semio_framework_async_macros::async_test]
async fn windows_have_expected_shape() {
    for window in [hann(64), hamming(64), blackman(64)] {
        assert_eq!(window.len(), 64);
        let peak = window.iter().fold(0.0f64, |acc, v| acc.max(*v));
        assert!((peak - 1.0).abs() < 0.01);
        assert!(window[0] < 0.1);
        assert!((window[0] - window[63]).abs() < 1e-12);
    }
}

#[semio_framework_async_macros::async_test]
async fn welch_psd_finds_planted_sinusoid() {
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

#[semio_framework_async_macros::async_test]
async fn cross_spectrum_phase_encodes_lag() {
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

#[semio_framework_async_macros::async_test]
async fn xcorr_peaks_at_known_shift() {
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

#[semio_framework_async_macros::async_test]
async fn subsample_peak_recovers_fractional_lag() {
    let pulse = |center: f64| -> Vec<f64> { (0..128).map(|t| (-((t as f64 - center) * (t as f64 - center)) / (2.0 * 16.0)).exp()).collect() };
    let a = pulse(50.0);
    let b = pulse(53.37);
    let c = xcorr_normalized(&a, &b, 10);
    let refined = subsample_peak(&c).expect("interior peak");
    assert!((refined - (10.0 + 3.37)).abs() < 0.05);
    assert_eq!(subsample_peak(&[]), None);
    assert_eq!(subsample_peak(&[3.0, 2.0, 1.0]), None);
}

#[semio_framework_async_macros::async_test]
async fn savitzky_golay_differentiates_cubic() {
    let dt = 0.1;
    let x: Vec<f64> = (0..50).map(|i| (i as f64 * dt).powi(3)).collect();
    let d1 = savitzky_golay(&x, 7, 3, 1, dt);
    for (i, &value) in d1.iter().enumerate().take(47).skip(3) {
        let expected = 3.0 * (i as f64 * dt).powi(2);
        assert!((value - expected).abs() < 1e-6);
    }
}

#[semio_framework_async_macros::async_test]
async fn savitzky_golay_preserves_low_order_polynomials() {
    let x: Vec<f64> = (0..40).map(|i| 2.0 + 0.5 * i as f64 + 0.03 * (i * i) as f64).collect();
    let smoothed = savitzky_golay(&x, 5, 2, 0, 1.0);
    for i in 2..38 {
        assert!((smoothed[i] - x[i]).abs() < 1e-9);
    }
}

#[semio_framework_async_macros::async_test]
async fn smoothing_preserves_constant_signals() {
    let x = vec![4.2; 30];
    for out in [gaussian_smooth_1d(&x, 1.5), moving_average(&x, 5), savitzky_golay(&x, 7, 2, 0, 1.0)] {
        for v in out {
            assert!((v - 4.2).abs() < 1e-9);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn find_peaks_ranks_two_bumps_by_prominence() {
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
