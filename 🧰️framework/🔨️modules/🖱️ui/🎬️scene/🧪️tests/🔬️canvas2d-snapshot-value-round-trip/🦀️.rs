
use super::*;

#[test]
fn canvas2d_snapshot_lease_round_trips() {
    let lease = Canvas2dSnapshotLease { slot: 3, epoch: 7, revision: 42, generation: 9, page_count: 2, byte_count: 1024 };
    let encoded = lease.to_value();
    assert_eq!(Canvas2dSnapshotLease::from_value(encoded), Ok(lease));
}
