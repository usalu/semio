use super::*;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixture/🔣️.json")).unwrap() }

#[test]
fn instance_lifetime_ui_handback_alias_counter_preserves_full_u64_domain() {
    let fixture = fixture();
    let counter = &fixture["aliasCounter"];
    let owners = UiArenaHandbacks::<8, 1>::new();
    let pending: &AtomicU64 = &owners.releases[3];
    let before: u64 = counter["before"].as_str().unwrap().parse().unwrap();
    pending.store(before, Ordering::Release);
    owners.record(3, UiArenaHandback::ReleaseAlias);
    assert_eq!(pending.load(Ordering::Acquire).to_string(), counter["afterReturn"]);
    assert_eq!(owners.take_one(3), Some(UiArenaHandback::ReleaseAlias));
    assert_eq!(pending.load(Ordering::Acquire).to_string(), counter["afterConsume"]);
    pending.store(u64::MAX - 1, Ordering::Release);
    owners.record(3, UiArenaHandback::ReleaseAlias);
    assert_eq!(pending.load(Ordering::Acquire).to_string(), counter["maximum"]);
    assert_eq!(owners.take_one(3), Some(UiArenaHandback::ReleaseAlias));
    assert_eq!(pending.load(Ordering::Acquire), u64::MAX - 1);
}

#[test]
fn instance_lifetime_ui_handback_word_boundaries_preserve_fair_exact_obligations() {
    let fixture = fixture();
    let owners = UiArenaHandbacks::<256, 4>::new();
    for row in fixture["obligations"].as_array().unwrap() {
        owners.record(row["slot"].as_u64().unwrap() as usize, if row["kind"] == "release" { UiArenaHandback::ReleaseAlias } else { UiArenaHandback::ReturnClaim });
    }
    let mut start = fixture["start"].as_u64().unwrap() as usize;
    let mut order = Vec::new();
    while let Some(slot) = owners.next_slot(start) {
        assert!(owners.take_one(slot).is_some());
        order.push(slot);
        start = (slot + 1) % 256;
    }
    assert_eq!(serde_json::to_value(order).unwrap(), fixture["expectedOrder"]);
    assert!(!owners.has_pending());
    owners.record(3, UiArenaHandback::ReleaseAlias);
    let rejected = owners.take_one(3).unwrap();
    owners.record(3, rejected);
    assert_eq!(owners.take_one(3).is_some(), fixture["rejectedObligationRetained"].as_bool().unwrap());
}

#[test]
fn instance_lifetime_ui_handback_delayed_ready_bit_cannot_consume_reused_slot() {
    let owners = UiArenaHandbacks::<8, 1>::new();
    owners.releases[2].fetch_add(1, Ordering::Release);
    assert_eq!(owners.take_one(2), Some(UiArenaHandback::ReleaseAlias));
    owners.ready[0].fetch_or(1 << 2, Ordering::Release);
    assert_eq!(owners.next_slot(0), Some(2));
    assert_eq!(owners.take_one(2).is_some(), fixture()["emptyReadyBitConsumesNewOwner"].as_bool().unwrap());
    assert!(!owners.has_pending());
    owners.record(2, UiArenaHandback::ReleaseAlias);
    assert_eq!(owners.take_one(2), Some(UiArenaHandback::ReleaseAlias));
    assert!(!owners.has_pending());
}

#[test]
fn instance_lifetime_ui_handback_racing_producers_preserve_every_admitted_alias() {
    let fixture = fixture();
    let producers = fixture["producers"].as_u64().unwrap() as usize;
    let returns = fixture["returnsPerProducer"].as_u64().unwrap() as usize;
    let owners = std::sync::Arc::new(UiArenaHandbacks::<256, 4>::new());
    let mut threads = Vec::new();
    for _ in 0..producers {
        let owners = owners.clone();
        threads.push(std::thread::spawn(move || { for _ in 0..returns { owners.record(255, UiArenaHandback::ReleaseAlias); } }));
    }
    let mut consumed = 0;
    while threads.iter().any(|thread| !thread.is_finished()) {
        if let Some(slot) = owners.next_slot(0) { consumed += usize::from(owners.take_one(slot).is_some()); }
        std::thread::yield_now();
    }
    for thread in threads { thread.join().unwrap(); }
    while let Some(slot) = owners.next_slot(0) { consumed += usize::from(owners.take_one(slot).is_some()); }
    assert_eq!(consumed, producers * returns);
    assert!(!owners.has_slot_pending(255));
}
