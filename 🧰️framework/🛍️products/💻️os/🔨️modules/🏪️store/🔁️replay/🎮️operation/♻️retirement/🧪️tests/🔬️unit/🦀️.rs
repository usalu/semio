//! 🎟️ Neutral terminal scaffold demand is admitted before the exact one-turn release.

use super::*;

struct ByteFactory;

impl ArtifactOwnedValueRetirementFactory<u8> for ByteFactory {
    fn retire_owned(&self, owner: u8) -> Box<dyn ErasedSnapshotRetirement> { semio_framework_value::retirement::owned_retirement(owner) }
}

#[test]
fn cooperative_replay_capacity_release_obeys_the_neutral_law() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔣️.json")).unwrap();
    let law = &law["capacityRelease"];
    let mut owners = Vec::<u8>::new();
    owners.try_reserve_exact(law["emptySlots"].as_u64().unwrap() as usize).unwrap();
    let capacity = owners.capacity();
    let mut retirement = super::super::super::ArtifactStoreOperationRowsRetirement::new(owners, Arc::new(ByteFactory));
    let body = law["bodyBytes"].as_u64().unwrap() as usize;
    assert_eq!(retirement.next_close_byte_demand(), capacity);
    assert_eq!(retirement.close_step(0, capacity).unwrap(), SnapshotRetirementStep::Blocked);
    assert!(!retirement.terminal_is_empty());
    assert_eq!(retirement.close_step(1, body).unwrap() != SnapshotRetirementStep::Blocked, law["ordinaryGrantAccepted"].as_bool().unwrap());
    assert_eq!(retirement.next_close_byte_demand(), capacity);
    assert_eq!(retirement.close_step(1, capacity).unwrap(), SnapshotRetirementStep::Pending { released_items: 1, released_bytes: capacity });
    assert_eq!(retirement.terminal_is_empty(), law["exactGrantAccepted"].as_bool().unwrap());
    assert_eq!(retirement.close_step(1, body).unwrap(), SnapshotRetirementStep::Complete);
    eprintln!("[DEBUG] replay terminal scaffold demand={capacity} body={body} release={capacity} terminal-empty=true");
}


#[test]
fn cooperative_replay_paged_inverse_preserves_semantic_array_and_releases_each_page() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔣️.json")).unwrap();
    let law = &law["pagedInverse"];
    let count = law["rows"].as_u64().unwrap() as usize;
    let maximum_bytes = law["maximumTurnBytes"].as_u64().unwrap() as usize;
    let expected: Vec<u8> = (0..count).map(|index| (index % 256) as u8).collect();
    let owners = semio_framework_value::list::PagedList::<u8, {usize::MAX}>::try_from_iter(expected.iter().copied()).unwrap();
    assert_eq!(serde_json::to_value(&owners).unwrap(), serde_json::to_value(&expected).unwrap());
    let mut retirement = ReplayMutationsRetirement { owners: ManuallyDrop::new(Some(owners)), active: None, factory: Arc::new(ByteFactory) };
    let mut backing_releases = 0;
    for turn in 0..100000 {
        assert!(retirement.next_close_byte_demand() <= maximum_bytes);
        match retirement.close_step(1, maximum_bytes).unwrap() {
            SnapshotRetirementStep::Complete => break,
            SnapshotRetirementStep::Blocked => panic!("native paged inverse exact grant blocked"),
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1 && released_bytes <= maximum_bytes);
                backing_releases += usize::from(released_bytes > 0);
            }
        }
        assert!(turn < 99999);
    }
    assert!(retirement.terminal_is_empty());
    assert!(backing_releases >= law["minimumBackingReleases"].as_u64().unwrap() as usize);
    eprintln!("[DEBUG] native paged inverse rows={count} boundedBackingReleases={backing_releases} terminal-empty=true");
}
