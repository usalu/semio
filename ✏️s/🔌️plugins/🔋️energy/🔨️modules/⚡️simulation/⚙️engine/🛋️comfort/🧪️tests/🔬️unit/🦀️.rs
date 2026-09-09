use super::*;

fn neutral_input() -> ComfortInput {
    ComfortInput { air_temp_c: 22.0, mean_radiant_temp_c: 22.0, air_speed_m_s: 0.1, relative_humidity: 0.5, metabolic_rate_met: 1.0, clothing_insulation_clo: 0.5, external_work_met: 0.0 }
}

#[test]
fn operative_equals_air_when_equal_mrt() {
    let top = operative_temp_c(22.0, 22.0, 0.1);
    assert!((top - 22.0).abs() < 0.5);
}

#[test]
fn pmv_near_zero_at_neutral() {
    let p = pmv(&neutral_input());
    assert!(p.abs() < 1.5, "pmv={p}");
}

#[test]
fn ppd_minimum_at_zero_pmv() {
    let p = ppd(0.0);
    assert!((p - 5.0).abs() < 1.0);
}

#[test]
fn pmv_increases_when_too_warm() {
    let mut hot = neutral_input();
    hot.air_temp_c = 30.0;
    hot.mean_radiant_temp_c = 30.0;
    assert!(pmv(&hot) > pmv(&neutral_input()));
}

#[test]
fn adaptive_range_centers_on_outdoor() {
    let (lo, hi) = adaptive_comfort_range_c(AdaptiveStandard::Ashrae55, 20.0, 1);
    let center = 0.5 * (lo + hi);
    assert!((center - (0.31 * 20.0 + 17.8)).abs() < 0.5);
}

#[test]
fn mrt_from_surfaces() {
    let temps_k = [293.15, 303.15];
    let vfs = [0.5, 0.5];
    let mrt = mean_radiant_temp_c(&temps_k, &vfs);
    assert!(mrt > 20.0 && mrt < 30.0);
}
