use super::*;

#[test]
fn resilience_counts_extremes() {
    let temps = vec![35.0, 5.0, 22.0];
    let r = compute_resilience(&temps, 20.0, 26.0, true);
    assert_eq!(r.hours_above_heat_index_32c, 1);
    assert_eq!(r.hours_below_10c, 1);
}
