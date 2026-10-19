
use super::*;

#[test]
fn canvas2d_snapshot_lease_round_trips() {
    let lease = Canvas2dSnapshotLease { slot: 3, epoch: 7, revision: 42, generation: 9, page_count: 2, byte_count: 1024 };
    let encoded = lease.to_value();
    assert_eq!(Canvas2dSnapshotLease::from_value(encoded), Ok(lease));
}

#[test]
fn canvas2d_page_policy_matches_schema_and_retires_exactly_one_page() {
    let schema: serde_json::Value = serde_json::from_str(include_str!("../../🖼️canvas2d-snapshot/🧬️schema/🔣️.json")).unwrap();
    assert_eq!(CANVAS2D_SNAPSHOT_PAGE_CAPACITY, schema["properties"]["pageCapacity"]["const"].as_u64().unwrap() as usize);
    assert_eq!(CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY, schema["properties"]["pageBytes"]["const"].as_u64().unwrap() as usize);
    let descriptor = Canvas2dSnapshotDescriptor { revision: 791, generation: 791, page_count: CANVAS2D_SNAPSHOT_PAGE_CAPACITY as u8, byte_count: (CANVAS2D_SNAPSHOT_PAGE_CAPACITY * CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY) as u32 };
    assert_eq!(canvas2d_snapshot_begin(Canvas2dSnapshotDescriptor { page_count: descriptor.page_count + 1, ..descriptor }), Err(Canvas2dSnapshotFault::Capacity));
    let token = canvas2d_snapshot_begin(descriptor).unwrap();
    for index in 0..CANVAS2D_SNAPSHOT_PAGE_CAPACITY {
        let mut page = Canvas2dSnapshotPage::new();
        page.push(&[index as u8; CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY]).unwrap();
        page.seal().unwrap();
        canvas2d_snapshot_admit_page(token, page).unwrap();
    }
    let lease = canvas2d_snapshot_seal(token).unwrap();
    for index in 0..CANVAS2D_SNAPSHOT_PAGE_CAPACITY {
        assert!(canvas2d_snapshot_with_page(lease, index as u8, |page| page.bytes().iter().all(|byte| *byte == index as u8)).unwrap());
    }
    canvas2d_snapshot_begin_close(lease).unwrap();
    for _ in 0..CANVAS2D_SNAPSHOT_PAGE_CAPACITY { assert!(!canvas2d_snapshot_close_step(lease).unwrap()); assert!(!canvas2d_snapshot_terminal_is_empty(lease)); }
    assert!(canvas2d_snapshot_close_step(lease).unwrap());
    assert!(canvas2d_snapshot_terminal_is_empty(lease));
}
