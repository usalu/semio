//! 🧪️ cursors tests (🔬️unit): compiled against the `testkit` surface of the world crates.

use semio_framework_os_infinite_world_model::{WORLD_INTERACTION_MARQUEE_POINT_CAPACITY, WorldInteractionObjectToken};
use semio_framework_os_infinite_world_components::WORLD_COMPONENT_TARGET_BYTE_CAPACITY;
use semio_framework_os_infinite_world_cursors::{WORLD_COMPONENT_MARQUEE_CAPACITY, WORLD_MARQUEE_RESULT_PAGE_CAPACITY, WORLD_MARQUEE_RESULT_PAGE_COUNT, WorldComponentTarget, WorldMarqueeGesture, WorldMarqueeResult, WorldMarqueeResultPages};

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.
fn read_static(path_from_manifest: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path_from_manifest);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("test input {}: {error}", path.display())).into_boxed_str())
}

/// 📖️ Reads several test inputs at run time and joins them into the one text the unsplit sources formed.
fn read_joined(paths: &[&str]) -> &'static str {
    let mut joined = String::new();
    for path in paths {
        joined.push_str(read_static(path));
    }
    Box::leak(joined.into_boxed_str())
}

#[test]
fn world_marquee_gesture_has_fixed_points_generation_and_cursorized_close() {
    let mut gesture = WorldMarqueeGesture::new(7, 11, [0.0, 0.0]);
    for index in 1..WORLD_INTERACTION_MARQUEE_POINT_CAPACITY {
        assert!(gesture.push([index as f32, 0.0]));
    }
    assert!(!gesture.push([999.0, 0.0]));
    assert!(!gesture.is_click([10.0, 0.0]));
    let mut turns = 0;
    while !gesture.close_step() {
        turns += 1;
        assert!(turns <= WORLD_INTERACTION_MARQUEE_POINT_CAPACITY);
    }
    assert_eq!(turns, WORLD_INTERACTION_MARQUEE_POINT_CAPACITY);
    assert_eq!(gesture.len, 0);
    assert!(gesture.points.iter().all(Option::is_none));
}

#[test]
fn world_marquee_result_pages_admit_exact_capacity_and_retire_one_target_per_grant() {
    let token = WorldInteractionObjectToken { slot: 0, generation: 1, revision: 1 };
    let mut pages = WorldMarqueeResultPages::default();
    for _ in 0..WORLD_MARQUEE_RESULT_PAGE_CAPACITY * WORLD_MARQUEE_RESULT_PAGE_COUNT {
        assert!(pages.push(WorldMarqueeResult::Object(token), 1));
    }
    assert_eq!(pages.page_len as usize, WORLD_MARQUEE_RESULT_PAGE_COUNT);
    assert!(!pages.push(WorldMarqueeResult::Object(token), 1));
    let mut turns = 0;
    while !pages.close_step() {
        turns += 1;
        assert!(turns <= WORLD_MARQUEE_RESULT_PAGE_CAPACITY * WORLD_MARQUEE_RESULT_PAGE_COUNT + WORLD_MARQUEE_RESULT_PAGE_COUNT);
    }
    assert_eq!(turns, WORLD_MARQUEE_RESULT_PAGE_CAPACITY * WORLD_MARQUEE_RESULT_PAGE_COUNT + WORLD_MARQUEE_RESULT_PAGE_COUNT);
    assert_eq!(pages.page_len, 0);
}

#[test]
fn world_component_marquee_capacity_plus_one_fails_closed_and_retires() {
    let token = WorldInteractionObjectToken { slot: 0, generation: 1, revision: 1 };
    let mut results = WorldMarqueeResultPages::default();
    for id in 0..WORLD_COMPONENT_MARQUEE_CAPACITY as u32 {
        assert!(results.push(WorldMarqueeResult::Component { object: token, id }, 0));
    }
    assert!(!results.push(WorldMarqueeResult::Component { object: token, id: WORLD_COMPONENT_MARQUEE_CAPACITY as u32 }, 0));
    let mut turns = 0;
    while !results.close_step() {
        turns += 1;
        assert!(turns <= WORLD_COMPONENT_MARQUEE_CAPACITY + 1);
    }
    assert_eq!(results.page_len, 0);
}

/// 🎯️ Equal mesh groups on different original objects remain distinct selection targets.
#[test]
fn world_component_contract_preserves_original_marquee_objects() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🎯️component-contract/🔣️.json")).unwrap();
    let mut results = WorldMarqueeResultPages::default();
    let mut oracle = std::collections::BTreeSet::new();
    for row in fixture["marquee"].as_array().unwrap() {
        let object = WorldInteractionObjectToken { slot: row["slot"].as_u64().unwrap() as u16, generation: 1, revision: 1 };
        let id = row["group"].as_u64().unwrap() as u32;
        oracle.insert(serde_json::to_string(row).unwrap());
        assert!(results.push(WorldMarqueeResult::Component { object, id }, 0));
        assert!(results.push(WorldMarqueeResult::Component { object, id }, 0));
    }
    assert_eq!(usize::from(results.lens[0]), oracle.len());
    for turn in 0..16 { if results.close_step() { break; } assert!(turn < 15); }
    assert_eq!(results.page_len, 0);
    println!("[DEBUG] componentMarquee originalObjectGroups={} exactDuplicateDedup=true terminalEmpty=true", oracle.len());
}

/// 📏️ The original target formatter owns every byte of the widest canonical analytic target.
#[test]
fn world_component_target_capacity_covers_exact_maximum_width() {
    use std::fmt::Write;
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🎯️component-target-capacity/🔣️.json")).unwrap();
    let expected = format!("{}.{}.{}~{}~{}~{}",fixture["renderId"].as_str().unwrap(),fixture["kind"].as_str().unwrap(),fixture["group"].as_u64().unwrap(),fixture["handle"].as_str().unwrap(),fixture["label"].as_str().unwrap(),fixture["revision"].as_str().unwrap());
    assert_eq!(expected.len() as u64,fixture["expectedBytes"].as_u64().unwrap());
    let mut target = WorldComponentTarget { bytes: [0; WORLD_COMPONENT_TARGET_BYTE_CAPACITY], len: 0 };
    target.write_str(&expected).expect("original formatter owns maximum canonical target");
    assert_eq!(target.as_str(),expected);
    assert!(target.write_str("x").is_err(),"byte beyond declared maximum is refused");
    println!("[DEBUG] componentTargetCapacity exactBytes={} independentSerdeSource=true",expected.len());
}
