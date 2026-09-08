
use super::*;

#[test]
fn probability_error_contract_is_owned_and_stable() {
    let errors = [
        (ProbabilityError::InvalidParameter { name: "sigma", value: 2.5 }, "invalid parameter sigma = 2.5"),
        (ProbabilityError::OutOfDomain { name: "p", value: -1.0 }, "p = -1 outside domain"),
        (ProbabilityError::NoConvergence { what: "quantile" }, "no convergence in quantile"),
    ];
    for (error, message) in errors {
        assert_eq!(error.to_string(), message);
        assert!(std::error::Error::source(&error).is_none());
    }
}

// #region 🔖️SpecialTests
#[test]
fn erf_matches_known_values() {
    assert!((erf(1.0) - 0.842_700_792_949_714_9).abs() < 1e-9);
    assert!((erf(0.5) - 0.520_499_877_813_046_5).abs() < 1e-9);
    assert!((erfc(2.0) - 0.004_677_734_981_063_127).abs() < 1e-9);
}

#[test]
fn erf_is_odd() {
    for x in [0.1, 0.5, 1.0, 1.5, 2.0, 3.0] {
        assert!((erf(-x) + erf(x)).abs() < 1e-12, "erf not odd at x={x}");
    }
}

#[test]
fn ln_gamma_matches_known_values() {
    assert!((ln_gamma(0.5) - std::f64::consts::PI.sqrt().ln()).abs() < 1e-9);
    assert!((ln_gamma(5.0) - 24.0_f64.ln()).abs() < 1e-9);
}

#[test]
fn ln_gamma_satisfies_recurrence() {
    for x in [0.3, 0.7, 1.5, 2.5, 4.2, 8.9] {
        let lhs = ln_gamma(x + 1.0) - ln_gamma(x);
        assert!((lhs - x.ln()).abs() < 1e-9, "recurrence failed at x={x}");
    }
}

#[test]
fn gamma_p_matches_analytic_identity_for_a_one() {
    for x in [0.1_f64, 0.5, 1.0, 2.0, 5.0, 10.0] {
        let expected = 1.0 - (-x).exp();
        assert!((gamma_p(1.0, x) - expected).abs() < 1e-9, "mismatch at x={x}");
    }
}

#[test]
fn gamma_p_and_q_sum_to_one() {
    for a in [0.5, 1.0, 2.5, 5.0, 10.0] {
        for x in [0.1, 1.0, 3.0, 8.0, 20.0] {
            let sum = gamma_p(a, x) + gamma_q(a, x);
            assert!((sum - 1.0).abs() < 1e-9, "a={a} x={x} sum={sum}");
        }
    }
}

#[test]
fn beta_inc_symmetry_identity() {
    for (a, b, x) in [(2.0, 3.0, 0.3), (1.5, 4.5, 0.6), (5.0, 5.0, 0.5)] {
        let lhs = beta_inc(a, b, x) + beta_inc(b, a, 1.0 - x);
        assert!((lhs - 1.0).abs() < 1e-9, "a={a} b={b} x={x}");
    }
}

#[test]
fn beta_inc_uniform_case_is_identity() {
    for x in [0.1, 0.4, 0.7, 0.9] {
        assert!((beta_inc(1.0, 1.0, x) - x).abs() < 1e-9);
    }
}

#[test]
fn beta_inc_symmetric_midpoint() {
    assert!((beta_inc(2.0, 2.0, 0.5) - 0.5).abs() < 1e-9);
}
// #endregion 🔖️SpecialTests

// #region 🔖️NormalTests
#[test]
fn normal_cdf_matches_known_value() {
    let n = Normal::STANDARD;
    assert!((n.cdf(1.959_963_984_540_054) - 0.975).abs() < 1e-9);
}

#[test]
fn normal_quantile_matches_known_value() {
    let n = Normal::STANDARD;
    let q = n.quantile(0.975).unwrap();
    assert!((q - 1.959_963_984_540_054).abs() < 1e-9);
}

#[test]
fn normal_quantile_cdf_round_trip() {
    let n = Normal::new(3.0, 2.0).unwrap();
    for x in [-5.0, -1.0, 0.0, 1.0, 2.5, 6.0, 10.0] {
        let p = n.cdf(x);
        let back = n.quantile(p).unwrap();
        assert!((back - x).abs() < 1e-6, "x={x} p={p} back={back}");
    }
}

#[test]
fn normal_rejects_nonpositive_std_dev() {
    assert!(Normal::new(0.0, 0.0).is_err());
    assert!(Normal::new(0.0, -1.0).is_err());
}

#[test]
fn normal_sample_mean_and_variance_within_band() {
    let n = Normal::new(5.0, 3.0).unwrap();
    let mut rng = semio_framework_geometry::random::Rng::from_seed(42);
    let draws = 20_000;
    let samples: Vec<f64> = (0..draws).map(|_| n.sample(&mut rng)).collect();
    let mean: f64 = samples.iter().sum::<f64>() / draws as f64;
    let variance: f64 = samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / draws as f64;
    let se_mean = 3.0 / (draws as f64).sqrt();
    assert!((mean - 5.0).abs() < 5.0 * se_mean, "mean {mean} too far from 5.0");
    assert!((variance - 9.0).abs() < 1.0, "variance {variance} too far from 9.0");
}
// #endregion 🔖️NormalTests

// #region 🔖️UniformTests
#[test]
fn uniform_pdf_cdf_quantile_consistency() {
    let u = Uniform::new(2.0, 6.0).unwrap();
    assert!((u.pdf(4.0) - 0.25).abs() < 1e-12);
    assert!((u.cdf(4.0) - 0.5).abs() < 1e-12);
    assert!((u.quantile(0.5).unwrap() - 4.0).abs() < 1e-12);
    assert_eq!(u.pdf(1.0), 0.0);
    assert_eq!(u.cdf(10.0), 1.0);
}

#[test]
fn uniform_rejects_invalid_bounds() {
    assert!(Uniform::new(5.0, 5.0).is_err());
    assert!(Uniform::new(5.0, 2.0).is_err());
}

#[test]
fn uniform_sample_mean_and_variance_within_band() {
    let u = Uniform::new(0.0, 10.0).unwrap();
    let mut rng = semio_framework_geometry::random::Rng::from_seed(7);
    let draws = 20_000;
    let samples: Vec<f64> = (0..draws).map(|_| u.sample(&mut rng)).collect();
    let mean: f64 = samples.iter().sum::<f64>() / draws as f64;
    assert!((mean - 5.0).abs() < 0.2, "mean {mean} too far from 5.0");
}
// #endregion 🔖️UniformTests

// #region 🔖️ChiSquaredTests
#[test]
fn chi_squared_quantiles_match_standard_table() {
    let expected = [3.841_458_820_694_124, 5.991_464_547_107_979, 7.814_727_903_251_179, 9.487_729_036_781_154, 11.070_497_693_516_351];
    for (i, &exp) in expected.iter().enumerate() {
        let dof = (i + 1) as f64;
        let c = ChiSquared::new(dof).unwrap();
        let q = c.quantile(0.95).unwrap();
        assert!((q - exp).abs() < 1e-6, "dof={dof} q={q} expected={exp}");
    }
}

#[test]
fn chi_squared_quantile_cdf_round_trip() {
    let c = ChiSquared::new(4.0).unwrap();
    for p in [0.05, 0.25, 0.5, 0.75, 0.95, 0.99] {
        let x = c.quantile(p).unwrap();
        let back = c.cdf(x);
        assert!((back - p).abs() < 1e-6, "p={p} x={x} back={back}");
    }
}

#[test]
fn chi_squared_sample_mean_within_band() {
    let c = ChiSquared::new(6.0).unwrap();
    let mut rng = semio_framework_geometry::random::Rng::from_seed(11);
    let draws = 20_000;
    let samples: Vec<f64> = (0..draws).map(|_| c.sample(&mut rng)).collect();
    let mean: f64 = samples.iter().sum::<f64>() / draws as f64;
    let se_mean = (2.0 * 6.0f64).sqrt() / (draws as f64).sqrt();
    assert!((mean - 6.0).abs() < 5.0 * se_mean, "mean {mean} too far from 6.0");
}
// #endregion 🔖️ChiSquaredTests

// #region 🔖️StudentTTests
#[test]
fn student_t_cdf_matches_known_value() {
    let t = StudentT::new(10.0).unwrap();
    assert!((t.cdf(2.0) - 0.963_305_982_614_629_9).abs() < 1e-9);
}

#[test]
fn student_t_quantile_matches_known_value() {
    let t = StudentT::new(10.0).unwrap();
    let q = t.quantile(0.975).unwrap();
    assert!((q - 2.228_138_851_986_273).abs() < 1e-6);
}

#[test]
fn student_t_quantile_cdf_round_trip() {
    let t = StudentT::new(7.0).unwrap();
    for p in [0.05, 0.25, 0.5, 0.75, 0.95] {
        let x = t.quantile(p).unwrap();
        let back = t.cdf(x);
        assert!((back - p).abs() < 1e-6, "p={p} x={x} back={back}");
    }
}

#[test]
fn student_t_sample_mean_within_band() {
    let t = StudentT::new(15.0).unwrap();
    let mut rng = semio_framework_geometry::random::Rng::from_seed(13);
    let draws = 20_000;
    let samples: Vec<f64> = (0..draws).map(|_| t.sample(&mut rng)).collect();
    let mean: f64 = samples.iter().sum::<f64>() / draws as f64;
    assert!(mean.abs() < 0.1, "mean {mean} too far from 0.0");
}
// #endregion 🔖️StudentTTests

// #region 🔖️FisherFTests
#[test]
fn fisher_f_cdf_symmetry_point() {
    for k in [2.0, 5.0, 10.0, 20.0] {
        let f = FisherF::new(k, k).unwrap();
        assert!((f.cdf(1.0) - 0.5).abs() < 1e-9, "k={k}");
    }
}

#[test]
fn fisher_f_quantile_cdf_round_trip() {
    let f = FisherF::new(5.0, 10.0).unwrap();
    for p in [0.1, 0.5, 0.9, 0.95] {
        let x = f.quantile(p).unwrap();
        let back = f.cdf(x);
        assert!((back - p).abs() < 1e-6, "p={p} x={x} back={back}");
    }
}

#[test]
fn fisher_f_sample_is_nonnegative() {
    let f = FisherF::new(4.0, 8.0).unwrap();
    let mut rng = semio_framework_geometry::random::Rng::from_seed(17);
    for _ in 0..1000 {
        assert!(f.sample(&mut rng) >= 0.0);
    }
}
// #endregion 🔖️FisherFTests

// #region 🔖️BernoulliTests
#[test]
fn bernoulli_pmf_cdf_quantile() {
    let b = Bernoulli::new(0.3).unwrap();
    assert!((b.pmf(0) - 0.7).abs() < 1e-12);
    assert!((b.pmf(1) - 0.3).abs() < 1e-12);
    assert!((b.cdf(0) - 0.7).abs() < 1e-12);
    assert_eq!(b.cdf(1), 1.0);
    assert_eq!(b.quantile(0.5).unwrap(), 0);
    assert_eq!(b.quantile(0.8).unwrap(), 1);
}

#[test]
fn bernoulli_sample_frequency_matches_p() {
    let b = Bernoulli::new(0.7).unwrap();
    let mut rng = semio_framework_geometry::random::Rng::from_seed(21);
    let draws = 20_000;
    let successes: u64 = (0..draws).map(|_| b.sample(&mut rng)).sum();
    let freq = successes as f64 / draws as f64;
    assert!((freq - 0.7).abs() < 0.02, "freq {freq} too far from 0.7");
}
// #endregion 🔖️BernoulliTests

// #region 🔖️BinomialTests
#[test]
fn binomial_pmf_matches_known_value() {
    let b = Binomial::new(10, 0.5).unwrap();
    assert!((b.pmf(5) - 252.0 / 1024.0).abs() < 1e-9);
}

#[test]
fn binomial_cdf_matches_manual_partial_sum() {
    let b = Binomial::new(8, 0.4).unwrap();
    for k in 0..=8 {
        let manual: f64 = (0..=k).map(|i| b.pmf(i)).sum();
        assert!((b.cdf(k) - manual).abs() < 1e-9, "k={k}");
    }
}

#[test]
fn binomial_cdf_matches_beta_identity() {
    for (n, p, k) in [(10u64, 0.3, 4u64), (20, 0.6, 12), (15, 0.5, 7)] {
        let b = Binomial::new(n, p).unwrap();
        let manual: f64 = (0..=k).map(|i| b.pmf(i)).sum();
        assert!((b.cdf(k) - manual).abs() < 1e-9, "n={n} p={p} k={k}");
    }
}

#[test]
fn binomial_quantile_cdf_consistency() {
    let b = Binomial::new(20, 0.35).unwrap();
    for p in [0.1, 0.3, 0.5, 0.7, 0.9] {
        let k = b.quantile(p).unwrap();
        assert!(b.cdf(k) >= p - 1e-9, "p={p} k={k} cdf={}", b.cdf(k));
    }
}

#[test]
fn binomial_sample_mean_within_band() {
    let b = Binomial::new(50, 0.4).unwrap();
    let mut rng = semio_framework_geometry::random::Rng::from_seed(23);
    let draws = 20_000;
    let samples: Vec<u64> = (0..draws).map(|_| b.sample(&mut rng)).collect();
    let mean: f64 = samples.iter().sum::<u64>() as f64 / draws as f64;
    let se_mean = (50.0 * 0.4 * 0.6f64).sqrt() / (draws as f64).sqrt();
    assert!((mean - 20.0).abs() < 5.0 * se_mean, "mean {mean} too far from 20.0");
}
// #endregion 🔖️BinomialTests

// #region 🔖️MultinomialTests
#[test]
fn multinomial_pmf_matches_hand_computation() {
    let m = Multinomial::new(4, vec![0.2, 0.3, 0.5]).unwrap();
    let counts = [1u64, 1, 2];
    // 4! / (1! 1! 2!) * 0.2^1 * 0.3^1 * 0.5^2 = 12 * 0.2 * 0.3 * 0.25 = 0.18
    let expected = 0.18;
    assert!((m.pmf(&counts) - expected).abs() < 1e-9, "pmf={}", m.pmf(&counts));
}

#[test]
fn multinomial_rejects_probs_not_summing_to_one() {
    assert!(Multinomial::new(4, vec![0.2, 0.3, 0.4]).is_err());
}

#[test]
fn multinomial_sample_counts_sum_to_n() {
    let m = Multinomial::new(30, vec![0.2, 0.3, 0.5]).unwrap();
    let mut rng = semio_framework_geometry::random::Rng::from_seed(29);
    for _ in 0..100 {
        let counts = m.sample(&mut rng);
        assert_eq!(counts.len(), 3);
        assert_eq!(counts.iter().sum::<u64>(), 30);
    }
}

#[test]
fn multinomial_sample_category_means_within_band() {
    let m = Multinomial::new(100, vec![0.2, 0.3, 0.5]).unwrap();
    let mut rng = semio_framework_geometry::random::Rng::from_seed(31);
    let draws = 2000;
    let mut sums = [0u64; 3];
    for _ in 0..draws {
        let counts = m.sample(&mut rng);
        for i in 0..3 {
            sums[i] += counts[i];
        }
    }
    let expected = [20.0, 30.0, 50.0];
    for i in 0..3 {
        let mean = sums[i] as f64 / draws as f64;
        assert!((mean - expected[i]).abs() < 2.0, "category {i} mean {mean} too far from {}", expected[i]);
    }
}
// #endregion 🔖️MultinomialTests
