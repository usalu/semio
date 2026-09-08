mod tests {
    use super::*;

    #[test]
    fn haar_periodic_one_level_preserves_energy() {
        // 🔐️ Parseval / energy-preservation: Haar under periodic boundary is exactly orthonormal,
        // so sum(a^2) + sum(d^2) == sum(s^2) to tight tolerance.
        let x = [1.0, 3.0, -2.0, 5.0, 0.5, -1.5, 4.0, 2.0];
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 1, BoundaryMode::Periodic).unwrap();
        let dwt = Dwt::decompose(&x, cfg).unwrap();
        let subband_total: f64 = dwt.subband_energies().iter().sum();
        let signal_total: f64 = x.iter().map(|&v| v * v).sum();
        assert!((subband_total - signal_total).abs() < 1e-9, "subband={subband_total} signal={signal_total}");
    }

    #[test]
    fn haar_periodic_multi_level_preserves_energy() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(11);
        let x: Vec<f64> = (0..64).map(|_| rng.next_f64() * 10.0 - 5.0).collect();
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 4, BoundaryMode::Periodic).unwrap();
        let dwt = Dwt::decompose(&x, cfg).unwrap();
        let subband_total: f64 = dwt.subband_energies().iter().sum();
        let signal_total: f64 = x.iter().map(|&v| v * v).sum();
        assert!((subband_total - signal_total).abs() < 1e-6, "subband={subband_total} signal={signal_total}");
    }

    #[test]
    fn constant_signal_has_zero_haar_detail_energy() {
        let x = [3.0; 8];
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 1, BoundaryMode::Periodic).unwrap();
        let dwt = Dwt::decompose(&x, cfg).unwrap();
        let energies = dwt.subband_energies();
        assert!(energies[0].abs() < 1e-9, "detail energy = {}", energies[0]);
    }

    #[test]
    fn smooth_ramp_has_lower_wavelet_entropy_than_noise() {
        let n = 256;
        let ramp: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(21);
        let noise: Vec<f64> = (0..n).map(|_| rng.next_f64() - 0.5).collect();

        let cfg = WaveletConfig::new(WaveletFamily::Daubechies4, 4, BoundaryMode::Symmetric).unwrap();
        let ramp_est = wavelet_entropy(&ramp, cfg).unwrap();
        let noise_est = wavelet_entropy(&noise, cfg).unwrap();
        assert!(ramp_est.value < noise_est.value, "ramp={} noise={}", ramp_est.value, noise_est.value);
    }

    #[test]
    fn white_noise_has_higher_wavelet_entropy_than_pure_tone() {
        let n = 512;
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(33);
        let noise: Vec<f64> = (0..n).map(|_| rng.next_f64() - 0.5).collect();
        let sine: Vec<f64> = (0..n).map(|i| (2.0 * core::f64::consts::PI * 4.0 * i as f64 / n as f64).sin()).collect();

        let cfg = WaveletConfig::new(WaveletFamily::Daubechies8, 5, BoundaryMode::Symmetric).unwrap();
        let noise_est = wavelet_entropy(&noise, cfg).unwrap();
        let sine_est = wavelet_entropy(&sine, cfg).unwrap();
        assert!(noise_est.value > sine_est.value, "noise={} sine={}", noise_est.value, sine_est.value);
    }

    #[test]
    fn config_new_rejects_zero_levels() {
        assert!(matches!(WaveletConfig::new(WaveletFamily::Haar, 0, BoundaryMode::Zero), Err(EntropyError::InvalidConfig { field: "levels", .. })));
    }

    #[test]
    fn decompose_rejects_input_shorter_than_filter() {
        let x = [1.0, 2.0, 3.0];
        let cfg = WaveletConfig::new(WaveletFamily::Daubechies8, 1, BoundaryMode::Zero).unwrap();
        assert!(matches!(Dwt::decompose(&x, cfg), Err(EntropyError::InsufficientData { needed: 8, actual: 3, .. })));
    }

    #[test]
    fn decompose_stops_early_when_signal_runs_out_of_levels() {
        // 🔐️ 8 samples with Haar (filter_len 2) can only produce 3 dyadic halvings (8->4->2->1)
        // before the approximation would drop below the filter length on a 4th level.
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 10, BoundaryMode::Periodic).unwrap();
        let dwt = Dwt::decompose(&x, cfg).unwrap();
        assert!(dwt.levels_achieved() < 10);
        assert!(dwt.levels_achieved() >= 1);
    }

    #[test]
    fn wavelet_entropy_reports_levels_achieved_diagnostic() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 10, BoundaryMode::Periodic).unwrap();
        let est = wavelet_entropy(&x, cfg).unwrap();
        let levels = est.diagnostics.iter().find(|(k, _)| *k == "levels_achieved").unwrap().1;
        assert!(levels < 10.0);
    }

    #[test]
    fn wavelet_entropy_rejects_all_zero_input() {
        let x = [0.0; 16];
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 2, BoundaryMode::Zero).unwrap();
        assert!(matches!(wavelet_entropy(&x, cfg), Err(EntropyError::DegenerateInput { .. })));
    }

    #[test]
    fn daubechies_filters_are_orthonormal_sum_sqrt2() {
        for family in [WaveletFamily::Haar, WaveletFamily::Daubechies4, WaveletFamily::Daubechies6, WaveletFamily::Daubechies8] {
            let h = family.low_pass();
            let sum: f64 = h.iter().sum();
            // 🔐️ published Daubechies-6 coefficients are truncated to 12 decimal digits, so their
            // sum only approximates sqrt(2) to about 1e-8, not full f64 precision.
            assert!((sum - core::f64::consts::SQRT_2).abs() < 1e-6, "{family:?} sum={sum}");
        }
    }
}
