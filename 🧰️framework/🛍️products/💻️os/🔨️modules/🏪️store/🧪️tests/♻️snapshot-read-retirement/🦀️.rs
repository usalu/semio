//! ♻️ Sparse read retirement visits occupied slots within each bounded grant.
use super::*;

#[test]
fn snapshot_read_retirement_skips_empty_slots_and_wraps_without_starvation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/♻️snapshot-read-retirement/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let registry = Arc::new(SnapshotReadLeaseRegistry::new());
        let issued = row["issued"].as_u64().unwrap() as usize;
        let mut leases: Vec<_> = (0..issued).map(|index| {
            let owner = Arc::new(index as u64);
            let lease = registry.try_issue(owner.clone()).unwrap();
            Some(SnapshotRead::new(owner, lease))
        }).collect();
        registry.state.lock().unwrap().cleanup_cursor = row["cursor"].as_u64().unwrap() as usize;
        for index in row["returned"].as_array().unwrap() { drop(leases[index.as_u64().unwrap() as usize].take()); }
        let expected: Vec<Option<u64>> = serde_json::from_value(row["visits"].clone()).unwrap();
        let actual: Vec<_> = expected.iter().map(|_| registry.try_take_one_returned::<u64>().unwrap().map(|root| *root)).collect();
        drop(leases);
        for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY + issued {
            if !registry.has_returned() { break; }
            drop(registry.try_take_one_returned::<u64>().unwrap());
        }
        assert!(registry.terminal_is_empty(), "{}", row["name"]);
        assert_eq!(actual, expected, "{}", row["name"]);
    }
    eprintln!("[DEBUG] sparse snapshot reads skip empty capacity, wrap, and preserve live-read fairness in three neutral cases");
}
