use super::*;

/// 🧪️ A zone at rest has zero third-order storage: the history part equals the new-temperature
/// coefficient times the settled temperature.
#[test]
fn settled_history_stores_nothing() {
    let state = ZoneAirState::new(22.0, 0.008);
    let (coefficient, history) = state.storage_terms(100.0);
    assert!((coefficient * 22.0 - history).abs() < 1e-9);
}

/// 🧪️ Committing shifts the history so the backward difference sees the three latest steps.
#[test]
fn commit_shifts_the_history() {
    let mut state = ZoneAirState::new(20.0, 0.008);
    state.commit(21.0, 0.009);
    state.commit(22.0, 0.010);
    assert_eq!(state.temp_history_c, [22.0, 21.0, 20.0]);
    assert_eq!(state.humidity_history, [0.010, 0.009, 0.008]);
    let (coefficient, history) = state.storage_terms(6.0);
    assert!((coefficient - 11.0).abs() < 1e-12);
    assert!((history - 6.0 * (66.0 - 31.5 + 20.0 / 3.0)).abs() < 1e-9);
}

/// 🧪️ The capacity rate is `ρ·V·c_p / Δt` of moist air.
#[test]
fn capacity_rate_is_moist_air_heat_capacity_per_step() {
    let state = ZoneAirState::new(20.0, 0.0);
    let rate = state.capacity_rate_w_k(129.6, 101_325.0, 600.0);
    assert!((rate - 1.2041 * 129.6 * 1004.84 / 600.0).abs() / rate < 2e-3, "rate was {rate} W/K");
}

/// 🧪️ With outdoor air flowing and no latent gain the humidity ratio relaxes to the outdoor one;
/// a latent gain holds it above.
#[test]
fn humidity_relaxes_to_outdoor_air_and_rises_with_latent_gain() {
    let mut dry = ZoneAirState::new(22.0, 0.012);
    let mut humid = ZoneAirState::new(22.0, 0.012);
    for _ in 0..2000 {
        let w = dry.next_humidity_ratio(100.0, 101_325.0, 600.0, 0.05, 0.004, 0.0);
        dry.commit(22.0, w);
        let w = humid.next_humidity_ratio(100.0, 101_325.0, 600.0, 0.05, 0.004, 500.0);
        humid.commit(22.0, w);
    }
    assert!((dry.humidity_ratio - 0.004).abs() < 1e-9);
    assert!(humid.humidity_ratio > 0.004 + 1e-4);
}
