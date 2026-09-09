use super::*;

#[test]
fn present_value_positive() {
    let pv = present_value(1000.0, 0.05, 10);
    assert!(pv > 0.0 && pv < 10_000.0);
}

#[test]
fn lcca_computes_payback() {
    let params = LccaParameters { study_period_years: 20, discount_rate: 0.03, inflation_rate: 0.02, initial_cost: 10_000.0, annual_maintenance: 500.0, replacement_cost: 0.0, replacement_interval_years: 0 };
    let lcca = compute_lcca(2000.0, &params);
    assert!((lcca.simple_payback_years - 5.0).abs() < 1e-6);
}
