
use super::*;
use std::sync::Arc;
use store::{SnapshotRetirementFactory, SnapshotRetirementStep};

#[test]
fn sourcing_presence_retirement_preserves_shared_readers_and_exact_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️retirement.json")).unwrap();
    let maximum_bytes = fixture["maximumBytes"].as_u64().unwrap() as usize;
    for value in fixture["snapshots"].as_array().unwrap() {
        let snapshot: SourcingCurationPresence = serde_json::from_value(value.clone()).unwrap();
        let packed = SourcingCurationPresence::decode_pack(&snapshot.encode_pack()).unwrap();
        assert_eq!(serde_json::to_value(packed).unwrap(), *value);
        let root = Arc::new(snapshot);
        let weak = Arc::downgrade(&root);
        let reader = root.clone();
        let mut first = SourcingPresenceRetirementFactory.retire(root);
        assert_eq!(first.close_step(0, maximum_bytes).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(first.close_step(1, maximum_bytes - 1).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(first.close_step(1, maximum_bytes).unwrap(), SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        assert_eq!(first.close_step(1, maximum_bytes).unwrap(), SnapshotRetirementStep::Complete);
        assert!(first.terminal_is_empty());
        assert_eq!(serde_json::to_value(reader.as_ref()).unwrap(), *value);
        let mut final_owner = SourcingPresenceRetirementFactory.retire(reader);
        assert_eq!(final_owner.close_step(1, maximum_bytes).unwrap(), SnapshotRetirementStep::Pending { released_items: 1, released_bytes: maximum_bytes });
        assert_eq!(final_owner.close_step(1, maximum_bytes).unwrap(), SnapshotRetirementStep::Complete);
        assert!(final_owner.terminal_is_empty());
        assert!(weak.upgrade().is_none());
    }
}
