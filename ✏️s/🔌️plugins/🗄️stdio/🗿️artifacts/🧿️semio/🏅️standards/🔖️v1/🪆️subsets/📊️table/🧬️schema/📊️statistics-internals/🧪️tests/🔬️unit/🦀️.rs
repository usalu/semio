
use super::*;

#[test]
fn statistics_error_contract_preserves_transparent_sources_and_conversions() {
    let simple = [
        (StatisticsError::InsufficientData { needed: 3, found: 1 }, "need at least 3 observations, found 1"),
        (StatisticsError::DimensionMismatch { expected: 4, found: 2 }, "dimension mismatch: expected 4, found 2"),
        (StatisticsError::SingularMatrix, "singular matrix"),
        (StatisticsError::NoConvergence { iterations: 7 }, "no convergence after 7 iterations"),
        (StatisticsError::InvalidArgument("rate"), "invalid argument: rate"),
    ];
    for (error, message) in simple {
        assert_eq!(error.to_string(), message);
        assert!(std::error::Error::source(&error).is_none());
    }
    let tabular: StatisticsError = crate::standards::v1::subsets::table::schema::tabular_internals::TabularError::UnknownColumn("x".into()).into();
    assert_eq!(tabular.to_string(), "no column named `x`");
    assert!(std::error::Error::source(&tabular).is_some());
    let probability: StatisticsError = crate::standards::v1::subsets::table::schema::probability_internals::ProbabilityError::NoConvergence { what: "cdf" }.into();
    assert_eq!(probability.to_string(), "no convergence in cdf");
    assert!(std::error::Error::source(&probability).is_some());
}

// #region 🔖️DescriptiveTests
#[semio_framework_async_macros::async_test]
async fn mean_and_variance_hand_computed() {
    let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    assert!((mean(&values).unwrap() - 5.0).abs() < 1e-12);
    assert!((variance(&values).unwrap() - 32.0 / 7.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn correlation_of_perfect_line_is_one() {
    let x = [1.0, 2.0, 3.0, 4.0, 5.0];
    let y: Vec<f64> = x.iter().map(|v| 2.0 * v + 1.0).collect();
    assert!((correlation(&x, &y).unwrap() - 1.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn correlation_of_orthogonal_pattern_is_zero() {
    let x = [1.0, -1.0, 1.0, -1.0];
    let y = [1.0, 1.0, -1.0, -1.0];
    assert!(correlation(&x, &y).unwrap().abs() < 1e-9);
}
// #endregion 🔖️DescriptiveTests

// #region 🔖️MatrixTests
#[semio_framework_async_macros::async_test]
async fn correlation_matrix_diagonal_is_one() {
    let x = [1.0, 2.0, 3.0, 4.0];
    let y = [4.0, 3.0, 2.0, 1.0];
    let m = correlation_matrix(&[&x, &y]).unwrap();
    assert!((m.get(0, 0) - 1.0).abs() < 1e-12);
    assert!((m.get(1, 1) - 1.0).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn invert_round_trips_and_detects_singular() {
    let mut a = MatD::zeros(3, 3);
    for (i, v) in [2.0, 0.0, 1.0, 1.0, 3.0, 2.0, 0.0, 1.0, 4.0].into_iter().enumerate() {
        a.set(i / 3, i % 3, v);
    }
    let inv = invert(&a).unwrap();
    let identity = a.matmul(&inv);
    for i in 0..3 {
        for j in 0..3 {
            let expected = if i == j { 1.0 } else { 0.0 };
            assert!((identity.get(i, j) - expected).abs() < 1e-9);
        }
    }
    let singular = MatD::zeros(2, 2);
    assert!(invert(&singular).is_err());
}
// #endregion 🔖️MatrixTests

// #region 🔖️PartialTests
#[semio_framework_async_macros::async_test]
async fn partial_correlation_matches_closed_form() {
    let mut corr = MatD::identity(3);
    corr.set(0, 1, 0.5);
    corr.set(1, 0, 0.5);
    corr.set(0, 2, 0.7);
    corr.set(2, 0, 0.7);
    corr.set(1, 2, 0.7);
    corr.set(2, 1, 0.7);
    let r = partial_correlation(&corr, 0, 1, &[2]).unwrap();
    assert!((r - 0.019_607_843_137_254_9).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn partial_correlation_with_empty_given_equals_plain_correlation() {
    let mut corr = MatD::identity(2);
    corr.set(0, 1, 0.4);
    corr.set(1, 0, 0.4);
    let r = partial_correlation(&corr, 0, 1, &[]).unwrap();
    assert!((r - 0.4).abs() < 1e-9);
}
// #endregion 🔖️PartialTests

// #region 🔖️OlsTests
#[semio_framework_async_macros::async_test]
async fn ols_recovers_exact_line() {
    let xs = [1.0, 2.0, 3.0, 4.0, 5.0];
    let ys: Vec<f64> = xs.iter().map(|x| 3.0 + 2.0 * x).collect();
    let mut design = MatD::zeros(5, 1);
    for (row, &x) in xs.iter().enumerate() {
        design.set(row, 0, x);
    }
    let fit = ols(&design, &ys, true).unwrap();
    assert!((fit.coefficients[0] - 3.0).abs() < 1e-8);
    assert!((fit.coefficients[1] - 2.0).abs() < 1e-8);
    assert!((fit.r_squared - 1.0).abs() < 1e-8);
}
// #endregion 🔖️OlsTests

// #region 🔖️LogisticTests
#[semio_framework_async_macros::async_test]
async fn logistic_symmetric_data_has_near_zero_intercept() {
    // Two labels flipped near the boundary (x=-1 -> y=1, x=1 -> y=0) so the data is not
    // perfectly linearly separable — a perfectly separable fixture has no finite MLE and
    // would make IRLS diverge by construction, which is correct behavior, not a bug.
    let xs = [-3.0, -2.0, -1.0, 1.0, 2.0, 3.0];
    let ys = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0];
    let mut design = MatD::zeros(6, 1);
    for (row, &x) in xs.iter().enumerate() {
        design.set(row, 0, x);
    }
    let fit = logistic(&design, &ys, true).unwrap();
    assert!(fit.coefficients[0].abs() < 0.2, "intercept {} too far from 0", fit.coefficients[0]);
    assert!(fit.coefficients[1] > 0.0, "slope should be positive");
}

#[semio_framework_async_macros::async_test]
async fn logistic_perfect_separation_returns_error_not_panic() {
    let xs = [1.0, 2.0, 3.0, 4.0];
    let ys = [0.0, 0.0, 1.0, 1.0];
    let mut design = MatD::zeros(4, 1);
    for (row, &x) in xs.iter().enumerate() {
        design.set(row, 0, x);
    }
    // perfectly separable data either fails to converge or returns a large-but-finite fit — must not panic
    match logistic(&design, &ys, true) {
        Ok(fit) => assert!(fit.coefficients.iter().all(|c| c.is_finite())),
        Err(StatisticsError::NoConvergence { .. }) => {}
        Err(other) => panic!("unexpected error: {other}"),
    }
}
// #endregion 🔖️LogisticTests

// #region 🔖️HypothesisTests
#[semio_framework_async_macros::async_test]
async fn fisher_z_test_matches_hand_computation() {
    let mut corr = MatD::identity(2);
    corr.set(0, 1, 0.5);
    corr.set(1, 0, 0.5);
    let result = fisher_z_test(&corr, 0, 1, &[], 28).unwrap();
    assert!((result.statistic - 2.746_530_71).abs() < 1e-6);
    assert!((result.p_value - 0.006_021).abs() < 1e-4);
}

#[semio_framework_async_macros::async_test]
async fn chi2_independence_matches_hand_computation() {
    let mut counts = MatD::zeros(2, 2);
    counts.set(0, 0, 10.0);
    counts.set(0, 1, 20.0);
    counts.set(1, 0, 20.0);
    counts.set(1, 1, 10.0);
    let result = chi2_independence(&counts).unwrap();
    assert!((result.statistic - 6.666_666_666_666_667).abs() < 1e-6);
    assert!((result.dof - 1.0).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn g2_ci_test_is_zero_for_margin_product_counts() {
    let x: Vec<u32> = [0u32, 0, 1, 1].repeat(25);
    let y: Vec<u32> = [0u32, 1, 0, 1].repeat(25);
    let result = g2_ci_test(&x, &y, &[], (2, 2, &[])).unwrap();
    assert!(result.statistic.abs() < 1e-9, "G2 {} should be ~0 for independent margin-product counts", result.statistic);
    assert!((result.dof - 1.0).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn welch_t_test_matches_hand_computation() {
    let a = [1.0, 2.0, 3.0, 4.0, 5.0];
    let b = [2.0, 3.0, 4.0, 5.0, 6.0];
    let result = t_test_two_sample(&a, &b, false).unwrap();
    assert!((result.statistic - (-1.0)).abs() < 1e-9);
    assert!((result.dof - 8.0).abs() < 1e-9);
}
// #endregion 🔖️HypothesisTests

// #region 🔖️InformationTests
#[semio_framework_async_macros::async_test]
async fn entropy_of_uniform_four_levels_is_ln_four() {
    let codes: Vec<u32> = [0u32, 1, 2, 3].repeat(100);
    let h = entropy(&codes, 4).unwrap();
    assert!((h - 4.0_f64.ln()).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn mutual_information_of_variable_with_itself_is_its_entropy() {
    let codes: Vec<u32> = [0u32, 0, 1, 1, 2, 2].repeat(50);
    let mi = mutual_information(&codes, &codes, 3, 3).unwrap();
    let h = entropy(&codes, 3).unwrap();
    assert!((mi - h).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn conditional_mutual_information_is_zero_on_markov_chain() {
    // X -> Z -> Y: within each Z stratum, X and Y are independently uniform over {0,1}.
    let z: Vec<u32> = [0u32, 0, 0, 0, 1, 1, 1, 1].repeat(20);
    let x: Vec<u32> = [0u32, 0, 1, 1, 0, 0, 1, 1].repeat(20);
    let y: Vec<u32> = [0u32, 1, 0, 1, 0, 1, 0, 1].repeat(20);
    let z_refs: Vec<&[u32]> = vec![&z];
    let cmi = conditional_mutual_information(&x, &y, &z_refs, (2, 2, &[2])).unwrap();
    assert!(cmi.abs() < 1e-9, "CMI {cmi} should be ~0 on a Markov chain");
}
// #endregion 🔖️InformationTests
