//! 🧪️ Shared typed Flow retirement byte and ownership laws.

use super::*;
use crate::os_store::ErasedSnapshotRetirement;

//#region 🧪️Retirement
#[test]
fn flow_retirement_typed_serde_oracle_and_exact_bytes_survive_worker_transfer() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️retirement.json")).unwrap();
    for maximum in [1, 4096] {
        let value: crate::FlowFixture = serde_json::from_value(fixture["fixture"].clone()).unwrap();
        let oracle: crate::FlowFixture = serde_json::from_value(serde_json::to_value(&value).unwrap()).unwrap();
        assert_eq!(value, oracle);
        let mut oracle_retirement = FlowRetirement::default();
        oracle_retirement.push(FlowOwner::Fixture(oracle));
        while !matches!(oracle_retirement.close_step(1, maximum).unwrap(), crate::os_store::SnapshotRetirementStep::Complete) {}
        let mut retirement = FlowRetirement::default();
        retirement.push(FlowOwner::Fixture(value));
        assert!(matches!(retirement.close_step(0, maximum).unwrap(), crate::os_store::SnapshotRetirementStep::Blocked));
        assert!(matches!(retirement.close_step(1, 0).unwrap(), crate::os_store::SnapshotRetirementStep::Blocked));
        let released = std::thread::spawn(move || {
            let mut released = 0;
            for _ in 0..100_000 {
                match retirement.close_step(1, maximum).unwrap() {
                    crate::os_store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                        assert!(released_items <= 1 && released_bytes <= maximum);
                        released += released_bytes;
                    }
                    crate::os_store::SnapshotRetirementStep::Complete => break,
                    crate::os_store::SnapshotRetirementStep::Blocked => panic!("positive Flow retirement grant blocked"),
                }
            }
            assert!(retirement.terminal_is_empty());
            released
        }).join().unwrap();
        assert_eq!(released, fixture["expected"]["releasedBytes"].as_u64().unwrap() as usize);
    }
}

#[test]
fn flow_retirement_populated_drop_is_guarded_and_unwind_does_not_double_panic() {
    let mut retirement = FlowRetirement::default();
    retirement.push(FlowOwner::Bytes(vec![0; 8192]));
    assert!(std::panic::catch_unwind(|| drop(retirement)).is_err());
    assert!(std::thread::spawn(|| {
        let mut retirement = FlowRetirement::default();
        retirement.push(FlowOwner::Bytes(vec![0; 8192]));
        panic!("primary Flow retirement fault");
    }).join().is_err());
}
//#endregion 🧪️Retirement
