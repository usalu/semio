mod tests {
    use super::*;
    use crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sine_series(n: usize, period: f64) -> Vec<f64> {
        (0..n).map(|i| (2.0 * core::f64::consts::PI * i as f64 / period).sin()).collect()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn white_noise_series(n: usize, seed: u64) -> Vec<f64> {
        let mut rng = Xorshift64::new(seed);
        (0..n).map(|_| rng.next_gaussian()).collect()
    }

    #[test]
    fn regularity_config_rejects_m_zero() {
        assert!(matches!(RegularityConfig::new(0, Tolerance::Auto), Err(EntropyError::InvalidConfig { field: "m", .. })));
        assert!(RegularityConfig::new(1, Tolerance::Auto).is_ok());
    }

    #[test]
    fn resolve_tolerance_auto_matches_hand_computation() {
        // 🔐️ [1,2,3,4,5]: mean=3, sample variance=(4+1+0+1+4)/4=2.5, sd=sqrt(2.5).
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        let expected = 0.2 * 2.5_f64.sqrt();
        let r = resolve_tolerance(&x, Tolerance::Auto).unwrap();
        assert!((r - expected).abs() < 1e-12, "r={r} expected={expected}");
    }

    #[test]
    fn resolve_tolerance_rejects_constant_series() {
        let x = [5.0; 10];
        assert!(matches!(resolve_tolerance(&x, Tolerance::Auto), Err(EntropyError::DegenerateInput { .. })));
        assert!(matches!(resolve_tolerance(&x, Tolerance::RelativeToSd(1.0)), Err(EntropyError::DegenerateInput { .. })));
    }

    #[test]
    fn sample_entropy_rejects_very_short_series() {
        let cfg = RegularityConfig::new(2, Tolerance::Auto).unwrap();
        let x = [1.0, 2.0, 3.0];
        assert!(matches!(sample_entropy(&x, cfg, LogBase::Nats), Err(EntropyError::InsufficientData { .. })));
    }

    #[test]
    fn sample_entropy_reports_infinity_when_no_higher_order_matches_exist() {
        // 🔐️ Hand-verified: at m=1 the pairs (0,1),(0,3),(1,3) match within r=1.0, but every one
        // of those pairs diverges by 4-8 at m+1=2, so A=0 while B=3 — SampEn must be +infinity.
        let x = [1.0, 1.0, 5.0, 1.0, 9.0];
        let cfg = RegularityConfig::new(1, Tolerance::Absolute(1.0)).unwrap();
        let est = sample_entropy(&x, cfg, LogBase::Nats).unwrap();
        assert_eq!(est.value, f64::INFINITY);
        assert!(est.warnings.iter().any(|w| matches!(w, Warning::NotConvergedSoft { .. })));
    }

    #[test]
    fn near_constant_signal_under_generous_tolerance_has_near_zero_regularity_entropy() {
        // 🔐️ A constant plus a 1e-10-scale perturbation, compared against a tolerance orders of
        // magnitude larger than the perturbation: virtually every template matches every other
        // template at both m and m+1, so Phi(m) ~= Phi(m+1) and all three measures collapse to
        // ~0 — the "this series is maximally predictable at this resolution" case.
        let mut rng = Xorshift64::new(11);
        let x: Vec<f64> = (0..300).map(|_| 5.0 + 1e-10 * rng.next_f64()).collect();
        let cfg = RegularityConfig::new(2, Tolerance::Absolute(0.5)).unwrap();
        let apen = approximate_entropy(&x, cfg, LogBase::Nats).unwrap();
        let sampen = sample_entropy(&x, cfg, LogBase::Nats).unwrap();
        assert!(apen.value.abs() < 1e-6, "apen={}", apen.value);
        assert!(sampen.value.abs() < 1e-6, "sampen={}", sampen.value);
    }

    #[test]
    fn regular_sine_has_much_lower_regularity_entropy_than_white_noise() {
        // 🔐️ THE canonical ApEn/SampEn/FuzzyEn sanity check: a smooth periodic signal is far more
        // "regular" (predictable from its own past) than i.i.d. noise of the same length, so all
        // three measures must be substantially lower on the sine than on the noise.
        let n = 1000;
        let sine = sine_series(n, 50.0);
        let noise = white_noise_series(n, 42);
        let cfg = RegularityConfig::new(2, Tolerance::Auto).unwrap();

        let apen_sine = approximate_entropy(&sine, cfg, LogBase::Nats).unwrap().value;
        let apen_noise = approximate_entropy(&noise, cfg, LogBase::Nats).unwrap().value;
        assert!(apen_sine < apen_noise - 0.5, "apen_sine={apen_sine} apen_noise={apen_noise}");

        let sampen_sine = sample_entropy(&sine, cfg, LogBase::Nats).unwrap().value;
        let sampen_noise = sample_entropy(&noise, cfg, LogBase::Nats).unwrap().value;
        assert!(sampen_sine < sampen_noise - 0.5, "sampen_sine={sampen_sine} sampen_noise={sampen_noise}");

        let fuzzyen_sine = fuzzy_entropy(&sine, cfg, LogBase::Nats).unwrap().value;
        let fuzzyen_noise = fuzzy_entropy(&noise, cfg, LogBase::Nats).unwrap().value;
        assert!(fuzzyen_sine < fuzzyen_noise - 0.5, "fuzzyen_sine={fuzzyen_sine} fuzzyen_noise={fuzzyen_noise}");
    }

    #[test]
    fn approximate_entropy_small_sample_warning() {
        let cfg = RegularityConfig::new(2, Tolerance::Auto).unwrap();
        let x = sine_series(30, 10.0);
        let est = approximate_entropy(&x, cfg, LogBase::Nats).unwrap();
        assert!(est.warnings.iter().any(|w| matches!(w, Warning::SmallSample { .. })));
    }

    #[test]
    fn base_conversion_is_consistent_across_apen_sampen_fuzzyen() {
        let cfg = RegularityConfig::new(2, Tolerance::Auto).unwrap();
        let x = sine_series(200, 25.0);
        for est in [approximate_entropy(&x, cfg, LogBase::Bits).unwrap(), sample_entropy(&x, cfg, LogBase::Bits).unwrap(), fuzzy_entropy(&x, cfg, LogBase::Bits).unwrap()] {
            let nats = est.nats();
            let back = LogBase::convert(nats, LogBase::Nats, LogBase::Bits);
            assert!((back - est.value).abs() < 1e-9);
        }
    }

    mod quick {
        use super::*;

        #[test]
        fn fuzzy_entropy_finite_and_defined_for_moderate_series() {
            let cfg = RegularityConfig::new(2, Tolerance::Auto).unwrap();
            let x = white_noise_series(500, 7);
            let est = fuzzy_entropy(&x, cfg, LogBase::Nats).unwrap();
            assert!(est.value.is_finite());
        }
    }
}
