//! 🛍️ The app-static catalogue is fetched once per APP INSTANCE — on every outcome.
//!
//! `ShellState::refresh_app_catalogue` runs from `refresh_ui`, which runs on every settled command.
//! Its cache key used to be recorded only after a successful reassembly, so a catalogue that failed
//! was re-fetched by every later refresh. On the live wgpu target the SECOND fetch of
//! `framework.section.catalogue` never returned from its own `submit_turn`: `boot_shell` never
//! returned, the canvas never bound a pointer listener, and no input of any kind reached the shell
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-input-hit-runtime-2026-09-13.md`).
//!
//! These lanes drive the production predicate `claim_app_catalogue_fetch` over
//! `🧫️fixtures/🛍️app-catalogue-attempt/🔣️.json`, and additionally re-derive what the PRE-FIX rule
//! (record only successes) produced, so the fixture is held to actually discriminating the two.

use super::claim_app_catalogue_fetch;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/🛍️app-catalogue-attempt/🔣️.json")).expect("app catalogue attempt fixture parses")
}

fn indices(case: &Value, key: &str) -> Vec<usize> {
    case[key].as_array().expect("index array").iter().map(|value| value.as_u64().expect("index") as usize).collect()
}

/// 🛍️ The PRE-FIX rule, re-derived here so the fixture's `baselineFetchIndices` is checked rather
/// than asserted: the instance was recorded only when that fetch's payload reassembled.
fn baseline_claim(recorded: &mut Option<u32>, instance_id: u32, outcome: Option<&str>) -> bool {
    if *recorded == Some(instance_id) {
        return false;
    }
    if outcome == Some("succeeded") {
        *recorded = Some(instance_id);
    }
    true
}

#[test]
fn every_fixture_case_fetches_the_catalogue_once_per_app_instance_on_every_outcome() {
    let fixture = fixture();
    let cases = fixture["cases"].as_array().expect("cases");
    assert!(!cases.is_empty(), "fixture declares no cases");
    for case in cases {
        let name = case["name"].as_str().expect("case name");
        let refreshes = case["refreshes"].as_array().expect("refreshes");
        let mut recorded: Option<u32> = None;
        let mut fetched = Vec::new();
        for (index, refresh) in refreshes.iter().enumerate() {
            let instance_id = refresh["instanceId"].as_u64().expect("instanceId") as u32;
            if claim_app_catalogue_fetch(&mut recorded, instance_id) {
                fetched.push(index);
            }
        }
        println!("[DEBUG] app-catalogue-attempt {name}: fetches {fetched:?} recorded={recorded:?}");
        assert_eq!(fetched, indices(case, "expectedFetchIndices"), "{name}: which refreshes fetched");
        assert_eq!(recorded, Some(case["recordedAfter"].as_u64().expect("recordedAfter") as u32), "{name}: recorded instance after the run");
    }
}

#[test]
fn the_pre_fix_rule_refetches_a_failed_catalogue_on_every_refresh() {
    let fixture = fixture();
    let mut discriminating = 0usize;
    for case in fixture["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("case name");
        let refreshes = case["refreshes"].as_array().expect("refreshes");
        let mut recorded: Option<u32> = None;
        let mut fetched = Vec::new();
        for (index, refresh) in refreshes.iter().enumerate() {
            let instance_id = refresh["instanceId"].as_u64().expect("instanceId") as u32;
            if baseline_claim(&mut recorded, instance_id, refresh["outcome"].as_str()) {
                fetched.push(index);
            }
        }
        let baseline = indices(case, "baselineFetchIndices");
        assert_eq!(fetched, baseline, "{name}: the fixture's recorded pre-fix behaviour");
        let expected = indices(case, "expectedFetchIndices");
        let discriminates = case["discriminates"].as_bool().expect("discriminates");
        assert_eq!(baseline != expected, discriminates, "{name}: whether this case tells the two rules apart");
        if discriminates {
            discriminating += 1;
            println!("[DEBUG] app-catalogue-attempt {name}: pre-fix {baseline:?} vs rule {expected:?}");
        }
    }
    assert!(discriminating >= 3, "the fixture must carry cases that fail on the pre-fix rule, got {discriminating}");
}
