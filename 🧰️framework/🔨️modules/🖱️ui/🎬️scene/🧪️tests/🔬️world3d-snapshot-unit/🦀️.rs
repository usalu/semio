
use super::*;

#[test]
fn world3d_snapshot_lease_round_trips() {
    let lease = World3dSnapshotLease { slot: 1, epoch: 2, revision: 3, generation: 4, page_count: 5, item_count: 6, byte_count: 7 };
    let encoded = lease.to_value();
    assert_eq!(World3dSnapshotLease::from_value(encoded), Ok(lease));
}

fn page(kind: World3dSnapshotPageKind, value: &str) -> World3dSnapshotPage {
    let mut page = World3dSnapshotPage::new(kind);
    let id = page.push_string(value).unwrap();
    page.push_item(World3dSnapshotItem { strings: [Some(id), None, None, None], ..Default::default() }).unwrap();
    page.seal().unwrap();
    page
}

#[test]
fn fixed_page_snapshot_validates_aba_iteration_and_one_page_close() {
    let descriptor = World3dSnapshotDescriptor { revision: 7, generation: 9, page_count: 2, item_count: 2, byte_count: 7, draw_count: 1, draw_instance_count: 2, draw_byte_count: 7 };
    let token = world3d_snapshot_begin(descriptor).unwrap();
    world3d_snapshot_admit_page(token, page(World3dSnapshotPageKind::Camera, "cam")).unwrap();
    world3d_snapshot_admit_page(token, page(World3dSnapshotPageKind::Instance, "item")).unwrap();
    let lease = world3d_snapshot_seal(token).unwrap();
    assert_eq!(world3d_snapshot_with_page(lease, 1, |page| page.string(page.item(0).unwrap().strings[0].unwrap()).unwrap().to_owned()).unwrap(), "item");
    let stale = World3dSnapshotLease { epoch: lease.epoch.wrapping_add(1), ..lease };
    assert_eq!(world3d_snapshot_with_page(stale, 0, |_| ()), Err(World3dSnapshotFault::Stale));
    world3d_snapshot_begin_close(lease).unwrap();
    assert!(!world3d_snapshot_close_step(lease).unwrap());
    assert!(!world3d_snapshot_close_step(lease).unwrap());
    assert!(world3d_snapshot_close_step(lease).unwrap());
    assert!(world3d_snapshot_terminal_is_empty(lease));
}

#[test]
fn descriptor_and_page_capacity_plus_one_fail_before_publication() {
    let descriptor = World3dSnapshotDescriptor { revision: 1, generation: 1, page_count: WORLD3D_SNAPSHOT_PAGE_CAPACITY as u16 + 1, item_count: 0, byte_count: 0, draw_count: 0, draw_instance_count: 0, draw_byte_count: 0 };
    assert_eq!(world3d_snapshot_begin(descriptor), Err(World3dSnapshotFault::Capacity));
    let mut full = World3dSnapshotPage::new(World3dSnapshotPageKind::Instance);
    for _ in 0..WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY {
        full.push_item(World3dSnapshotItem::default()).unwrap();
    }
    assert_eq!(full.push_item(World3dSnapshotItem::default()), Err(World3dSnapshotFault::ItemCredits));
}

#[test]
fn interrupted_writer_aborts_one_admitted_page_per_step() {
    let descriptor = World3dSnapshotDescriptor { revision: 3, generation: 5, page_count: 2, item_count: 2, byte_count: 2, draw_count: 0, draw_instance_count: 0, draw_byte_count: 0 };
    let token = world3d_snapshot_begin(descriptor).unwrap();
    world3d_snapshot_admit_page(token, page(World3dSnapshotPageKind::Status, "a")).unwrap();
    world3d_snapshot_admit_page(token, page(World3dSnapshotPageKind::Engagement, "b")).unwrap();
    world3d_snapshot_abort_write(token).unwrap();
    assert!(!world3d_snapshot_abort_write_step(token).unwrap());
    assert!(!world3d_snapshot_abort_write_step(token).unwrap());
    assert!(world3d_snapshot_abort_write_step(token).unwrap());
    assert!(world3d_snapshot_write_terminal_is_empty(token));
}

#[test]
fn draw_permit_is_reserved_before_publication_and_orphan_close_is_one_page() {
    let before = {
        let store = WORLD3D_SNAPSHOTS.lock().unwrap();
        (store.reserved_draws, store.reserved_draw_instances, store.reserved_draw_bytes)
    };
    let descriptor = World3dSnapshotDescriptor { revision: 17, generation: 19, page_count: 1, item_count: 1, byte_count: 1, draw_count: 1, draw_instance_count: 2, draw_byte_count: 7 };
    let token = world3d_snapshot_begin(descriptor).unwrap();
    {
        let store = WORLD3D_SNAPSHOTS.lock().unwrap();
        assert_eq!((store.reserved_draws, store.reserved_draw_instances, store.reserved_draw_bytes), (before.0 + 1, before.1 + 2, before.2 + 7));
    }
    world3d_snapshot_admit_page(token, page(World3dSnapshotPageKind::Mesh, "x")).unwrap();
    let lease = world3d_snapshot_seal(token).unwrap();
    let permit = world3d_snapshot_claim_draw_permit(lease, 1, 2, 7).unwrap();
    assert_eq!((permit.draw_count, permit.instance_count, permit.byte_count), (1, 2, 7));
    assert_eq!(world3d_snapshot_claim_draw_permit(lease, 1, 2, 7), Err(World3dSnapshotFault::Stale));
    world3d_snapshot_recover_lease(lease).unwrap();
    assert_eq!(world3d_snapshot_recovery_close_step(0), Some((0, 0)));
    assert_eq!(world3d_snapshot_recovery_close_step(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY), Some((1, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY)));
    assert_eq!(world3d_snapshot_recovery_close_step(WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY), Some((1, 0)));
    let store = WORLD3D_SNAPSHOTS.lock().unwrap();
    assert_eq!((store.reserved_draws, store.reserved_draw_instances, store.reserved_draw_bytes), before);
}
