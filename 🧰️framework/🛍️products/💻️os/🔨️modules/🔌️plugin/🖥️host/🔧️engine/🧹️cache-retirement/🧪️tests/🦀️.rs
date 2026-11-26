//! 🧪️ Contract: the compiled cache retires by the declared age limit and size cap, least recently used first; the shared fixture is also judged by the TypeScript oracle.

use super::cache_retirement::*;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

const DAY_MS: u64 = 86_400_000;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()
}

fn fixture_policy(value: &serde_json::Value) -> CacheRetirementPolicy {
    let seconds = |key: &str| value["policy"][key].as_u64().unwrap() * 1000;
    CacheRetirementPolicy { maximum_bytes: value["policy"]["maximumBytes"].as_u64().unwrap(), maximum_age_ms: seconds("maximumAgeSeconds"), minimum_idle_ms: seconds("minimumIdleSeconds"), scratch_grace_ms: seconds("scratchGraceSeconds"), ..CacheRetirementPolicy::shipped() }
}

#[test]
fn the_shipped_policy_is_the_declared_json_read_by_an_independent_parser() {
    let text = include_str!("../⚙️configuration/🔣️.json");
    let value: serde_json::Value = serde_json::from_str(text).unwrap();
    let policy = CacheRetirementPolicy::shipped();
    assert_eq!(policy.maximum_bytes, value["maximumBytes"].as_u64().unwrap());
    assert_eq!(policy.maximum_age_ms, value["maximumAgeSeconds"].as_u64().unwrap() * 1000);
    assert_eq!(policy.minimum_idle_ms, value["minimumIdleSeconds"].as_u64().unwrap() * 1000);
    assert_eq!(policy.scratch_grace_ms, value["scratchGraceSeconds"].as_u64().unwrap() * 1000);
    assert_eq!(policy.minimum_interval_ms, value["minimumIntervalSeconds"].as_u64().unwrap() * 1000);
    assert_eq!(policy.touch_interval_ms, value["touchIntervalSeconds"].as_u64().unwrap() * 1000);
    assert_eq!(policy.maximum_entries as u64, value["maximumEntries"].as_u64().unwrap());
    assert_eq!(policy.stamp_file, value["stampFile"].as_str().unwrap());
    assert_eq!(policy.maximum_bytes, 8 * 1024 * 1024 * 1024);
    assert_eq!(policy.maximum_age_ms, 14 * DAY_MS);
    assert!(CacheRetirementPolicy::parse("{\"version\": 1}").is_err());
}

#[test]
fn the_shared_fixture_cases_select_exactly_the_declared_entries() {
    let value = fixture();
    let policy = fixture_policy(&value);
    let now_ms = 1_000_000_000u64;
    for case in value["cases"].as_array().unwrap() {
        let entries: Vec<CacheEntry> = case["entries"].as_array().unwrap().iter().map(|row| CacheEntry { path: PathBuf::from(row["name"].as_str().unwrap()), bytes: row["bytes"].as_u64().unwrap(), used_ms: now_ms - row["idleSeconds"].as_u64().unwrap() * 1000, scratch: row["scratch"].as_bool().unwrap_or(false) }).collect();
        let mut names: Vec<String> = select_retired(&entries, now_ms, &policy).into_iter().map(|index| entries[index].path.to_string_lossy().into_owned()).collect();
        names.sort();
        let mut expected: Vec<String> = case["retired"].as_array().unwrap().iter().map(|name| name.as_str().unwrap().to_string()).collect();
        expected.sort();
        assert_eq!(names, expected, "case {}", case["name"]);
    }
}

fn aged(path: &std::path::Path, bytes: usize, idle_ms: u64) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, vec![7u8; bytes]).unwrap();
    let file = fs::OpenOptions::new().write(true).open(path).unwrap();
    file.set_modified(SystemTime::now() - Duration::from_millis(idle_ms)).unwrap();
}

#[test]
fn a_real_cache_root_loses_the_expired_and_the_oldest_over_the_cap_and_keeps_in_use_files_and_other_files() {
    let root = std::env::temp_dir().join(format!("semio-cache-retirement-{}-{}", std::process::id(), now_epoch_ms()));
    let policy = CacheRetirementPolicy { maximum_bytes: 250, maximum_age_ms: 10 * DAY_MS, minimum_idle_ms: 60_000, scratch_grace_ms: 3_600_000, minimum_interval_ms: 3_600_000, ..CacheRetirementPolicy::shipped() };
    aged(&root.join("aa/expired.cwasm"), 10, 11 * DAY_MS);
    aged(&root.join("aa/oldest.cwasm"), 100, 5 * DAY_MS);
    aged(&root.join("bb/newer.cwasm"), 100, DAY_MS);
    aged(&root.join("bb/open.cwasm"), 100, 1_000);
    aged(&root.join("bb/writer.cwasm.4242.scratch"), 500, 2 * 3_600_000);
    aged(&root.join("bb/live-writer.cwasm.4243.scratch"), 500, 1_000);
    fs::write(root.join("cold-compile.lock"), b"").unwrap();
    let first = retire_compiled_cache_if_due(&root, now_epoch_ms(), &policy).expect("the first run is due");
    assert_eq!((first.removed_files, first.removed_bytes), (3, 10 + 100 + 500));
    assert!(!root.join("aa/expired.cwasm").exists() && !root.join("aa/oldest.cwasm").exists() && !root.join("bb/writer.cwasm.4242.scratch").exists());
    assert!(root.join("bb/newer.cwasm").exists() && root.join("bb/open.cwasm").exists() && root.join("bb/live-writer.cwasm.4243.scratch").exists() && root.join("cold-compile.lock").exists());
    assert!(retire_compiled_cache_if_due(&root, now_epoch_ms(), &policy).is_none(), "a second run within the interval is not due");
    assert!(retire_compiled_cache_if_due(&root, now_epoch_ms() + 2 * 3_600_000, &policy).is_some());
    fs::remove_dir_all(&root).unwrap();
}

#[test]
fn a_cache_hit_refreshes_the_recency_only_after_the_touch_interval() {
    let root = std::env::temp_dir().join(format!("semio-cache-touch-{}-{}", std::process::id(), now_epoch_ms()));
    let policy = CacheRetirementPolicy { touch_interval_ms: 600_000, ..CacheRetirementPolicy::shipped() };
    let stale = root.join("aa/stale.cwasm");
    let fresh = root.join("aa/fresh.cwasm");
    aged(&stale, 1, DAY_MS);
    aged(&fresh, 1, 1_000);
    let before = fs::metadata(&fresh).unwrap().modified().unwrap();
    touch_compiled_cache_entry(&stale, now_epoch_ms(), &policy);
    touch_compiled_cache_entry(&fresh, now_epoch_ms(), &policy);
    assert!(SystemTime::now().duration_since(fs::metadata(&stale).unwrap().modified().unwrap()).unwrap() < Duration::from_secs(60));
    assert_eq!(fs::metadata(&fresh).unwrap().modified().unwrap(), before);
    fs::remove_dir_all(&root).unwrap();
}

#[test]
fn a_missing_root_is_not_created_and_not_an_error() {
    let root = std::env::temp_dir().join(format!("semio-cache-absent-{}-{}", std::process::id(), now_epoch_ms()));
    assert!(retire_compiled_cache_if_due(&root, now_epoch_ms(), &CacheRetirementPolicy::shipped()).is_none());
    assert!(!root.exists());
}
