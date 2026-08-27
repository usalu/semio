//! 🧪️ Exact monotonic clock authority laws shared with the host-clock schema.

use super::*;

//#region 🪪️Installation
#[test]
fn microsecond_clock_installation_is_exact_and_repeatable() {
    fn browser() -> Option<u64> { Some(500) }
    fn foreign() -> Option<u64> { Some(900) }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧵️job/⏱️budget/🧪️clock.json")).unwrap();
    for law in fixture["installation"].as_array().unwrap() {
        let authority = OnceLock::new();
        let clock = |name: &str| if name == "browser" { browser as fn() -> Option<u64> } else { foreign as fn() -> Option<u64> };
        if let Some(current) = law["current"].as_str() { assert!(authority.set(clock(current)).is_ok()); }
        let requested = clock(law["requested"].as_str().unwrap());
        assert_eq!(install_exact_clock(&authority, requested).is_ok(), law["accepted"].as_bool().unwrap());
        assert!(std::ptr::fn_addr_eq(*authority.get().unwrap(), clock(law["retained"].as_str().unwrap())));
        eprintln!("[DEBUG] monotonic clock installation current={} requested={} accepted={} retained={}", law["current"], law["requested"], law["accepted"], law["retained"]);
    }
}
//#endregion 🪪️Installation

//#region 🐕️StrictBoundary
#[test]
fn microsecond_watchdog_boundary_is_strictly_below_eight_ms() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧵️job/⏱️budget/🧪️clock.json")).unwrap();
    for law in fixture["watchdog"].as_array().unwrap() {
        let elapsed = law["elapsedMicroseconds"].as_str().unwrap().parse().unwrap();
        assert_eq!(interactive_step_contract_violated(elapsed), law["violated"].as_bool().unwrap());
        eprintln!("[DEBUG] exact watchdog boundary elapsed_us={elapsed} violated={}", law["violated"]);
    }
}
//#endregion 🐕️StrictBoundary
