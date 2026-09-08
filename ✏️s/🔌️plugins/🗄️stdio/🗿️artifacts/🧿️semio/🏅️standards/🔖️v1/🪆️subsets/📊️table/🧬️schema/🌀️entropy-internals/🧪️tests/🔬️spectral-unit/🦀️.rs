mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sine(n: usize, freq: f64, sample_rate: f64) -> Vec<f64> {
        (0..n).map(|i| (2.0 * core::f64::consts::PI * freq * i as f64 / sample_rate).sin()).collect()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn white_noise(n: usize, seed: u64) -> Vec<f64> {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(seed);
        (0..n).map(|_| rng.next_f64() - 0.5).collect()
    }

    #[test]
    fn pure_sine_has_low_normalized_spectral_entropy() {
        let x = sine(4096, 50.0, 1000.0);
        let cfg = SpectralConfig { window: WindowKind::Hann, normalize: true, ..Default::default() };
        let est = spectral_entropy(&x, cfg).unwrap();
        assert!(est.value < 0.3, "got {}", est.value);
        assert!(est.n_effective >= 4.0);
    }

    #[test]
    fn white_noise_has_high_normalized_spectral_entropy() {
        let x = white_noise(4096, 7);
        let cfg = SpectralConfig { window: WindowKind::Hann, normalize: true, ..Default::default() };
        let est = spectral_entropy(&x, cfg).unwrap();
        assert!(est.value > 0.85, "got {}", est.value);
    }

    #[test]
    fn band_restriction_changes_entropy_and_validates_range() {
        let x = sine(4096, 50.0, 1000.0);
        let full = spectral_entropy(&x, SpectralConfig { normalize: true, ..Default::default() }).unwrap();
        let in_band = spectral_entropy(&x, SpectralConfig { normalize: true, band: Some((0.02, 0.08)), ..Default::default() }).unwrap();
        let out_of_band = spectral_entropy(&x, SpectralConfig { normalize: true, band: Some((0.3, 0.5)), ..Default::default() }).unwrap();
        assert!(in_band.value != full.value);
        assert!(out_of_band.value > in_band.value, "in_band={} out_of_band={}", in_band.value, out_of_band.value);

        assert!(matches!(spectral_entropy(&x, SpectralConfig { band: Some((0.4, 0.1)), ..Default::default() }), Err(EntropyError::InvalidConfig { field: "band", .. })));
        assert!(matches!(spectral_entropy(&x, SpectralConfig { band: Some((0.6, 0.7)), ..Default::default() }), Err(EntropyError::InvalidConfig { field: "band", .. })));
    }

    #[test]
    fn rejects_segment_len_larger_than_input() {
        let x = vec![0.0, 1.0, 2.0, 3.0];
        let cfg = SpectralConfig { segment_len: 100, ..Default::default() };
        assert!(matches!(spectral_entropy(&x, cfg), Err(EntropyError::InvalidConfig { field: "segment_len", .. })));
    }

    #[test]
    fn config_new_rejects_overlap_at_or_above_one() {
        assert!(matches!(SpectralConfig::new(WindowKind::Hann, 128, 1.0), Err(EntropyError::InvalidConfig { field: "overlap", .. })));
        assert!(matches!(SpectralConfig::new(WindowKind::Hann, 128, 1.5), Err(EntropyError::InvalidConfig { field: "overlap", .. })));
        assert!(SpectralConfig::new(WindowKind::Hann, 128, 0.75).is_ok());
    }
}
