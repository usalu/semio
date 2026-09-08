use super::*;

fn receipt(row: &serde_json::Value) -> ActorUiPatchReceipt {
    ActorUiPatchReceipt {
        lifetime: semio_framework::kernel::ActorInstanceLifetime { activation_generation: row["activation"].as_u64().unwrap(), instance_id: row["instance"].as_u64().unwrap() as u32, guest_lifetime: row["guest"].as_u64().unwrap() },
        patch_sequence: row["sequence"].as_u64().unwrap(),
    }
}

fn patch(row: &serde_json::Value) -> UiPatch {
    UiPatch { surface: ui_contract::SurfaceId::try_from(row["surface"].as_str().unwrap()).unwrap(), base_revision: ui_contract::UiRevision(0), revision: ui_contract::UiRevision(row["revision"].as_u64().unwrap()), ops: Default::default() }
}

fn take(pending: &mut PendingPatchAuthority) -> UiPatch {
    assert!(pending.take_one(65536).unwrap().is_none());
    pending.take_one(65536).unwrap().unwrap()
}

fn close(pending: &mut PendingPatchAuthority) {
    for _ in 0..1024 {
        if pending.close_instance_step(7, 1, 4096).unwrap().complete {
            return;
        }
    }
    panic!("bounded exact pending patch close");
}

#[test]
fn reactor_issued_patch_ack_and_rejection_match_neutral_exact_tuple() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixture/🩹️receipt.json")).unwrap();
    for rejection in [false, true] {
        for row in fixture["cases"].as_array().unwrap() {
            let mut pending = PendingPatchAuthority::new();
            pending.push_external(patch(&fixture["issued"])).unwrap();
            let patch = take(&mut pending);
            let issued = receipt(&fixture["issued"]);
            pending.stage_emission(issued, &patch).unwrap();
            if row["committed"].as_bool().unwrap() {
                pending.commit_emission();
            }
            if !row["pending"].as_bool().unwrap() {
                assert!(pending.apply_issued_ack(issued, &patch.surface.0, patch.revision.0, 65536, |_| panic!("external patch has no reconciler")).unwrap());
            }
            if !row["live"].as_bool().unwrap() {
                let key = super::super::instance_lifetime::NativeCloseKey::fixture(7, 1);
                pending.reserve_close_instance(key).unwrap();
                pending.activate_close_instance(key).unwrap();
            }
            let ack = receipt(&row["ack"]);
            let surface = row["ack"]["surface"].as_str().unwrap();
            let revision = row["ack"]["revision"].as_u64().unwrap();
            let accepted = if rejection {
                pending.apply_issued_rejection(ack, surface, revision, |_| panic!("external patch has no reconciler"))
            } else {
                pending.apply_issued_ack(ack, surface, revision, 65536, |_| panic!("external patch has no reconciler")).unwrap()
            };
            assert_eq!(accepted, row["accepted"].as_bool().unwrap(), "{}", row["id"]);
            if !row["committed"].as_bool().unwrap() {
                pending.hand_back_turn(patch).unwrap();
            }
            close(&mut pending);
        }
    }
    eprintln!("[DEBUG] exact issued patch ACK/rejection neutral cases=20");
}

#[test]
fn reactor_issued_parallel_patch_slots_and_duplicate_ack_remain_independent() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixture/🩹️receipt.json")).unwrap();
    let mut pending = PendingPatchAuthority::new();
    let first = receipt(&fixture["issued"]);
    let second = ActorUiPatchReceipt { patch_sequence: first.patch_sequence + 1, ..first };
    for receipt in [first, second] {
        pending.push_external(patch(&fixture["issued"])).unwrap();
        let patch = take(&mut pending);
        pending.stage_emission(receipt, &patch).unwrap();
        pending.commit_emission();
    }
    assert_eq!(pending.slots.iter().flatten().filter(|slot| slot.issued.is_some()).count(), 2);
    assert!(pending.apply_issued_ack(second, "7:window", 2, 65536, |_| unreachable!()).unwrap());
    assert!(!pending.apply_issued_ack(second, "7:window", 2, 65536, |_| unreachable!()).unwrap());
    assert_eq!(pending.slots.iter().flatten().filter(|slot| slot.issued.is_some()).count(), 1);
    assert!(pending.apply_issued_rejection(first, "7:window", 2, |_| unreachable!()));
    close(&mut pending);
    eprintln!("[DEBUG] parallel issued patch slots preserve independent exact receipts");
}

#[test]
fn reactor_uncommitted_patch_handback_preserves_exact_slot_and_retry() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixture/🩹️receipt.json")).unwrap();
    let mut pending = PendingPatchAuthority::new();
    pending.push_external(patch(&fixture["issued"])).unwrap();
    let patch = take(&mut pending);
    let sequence = pending.turn_handback_sequence;
    let old = receipt(&fixture["issued"]);
    pending.stage_emission(old, &patch).unwrap();
    assert!(!pending.apply_issued_ack(old, "7:window", 2, 65536, |_| unreachable!()).unwrap());
    pending.hand_back_turn(patch).unwrap();
    assert_eq!(pending.turn_handback_sequence, sequence);
    let patch = pending.take_one(65536).unwrap().unwrap();
    let current = ActorUiPatchReceipt { patch_sequence: old.patch_sequence + 1, ..old };
    pending.stage_emission(current, &patch).unwrap();
    pending.commit_emission();
    assert!(!pending.apply_issued_ack(old, "7:window", 2, 65536, |_| unreachable!()).unwrap());
    assert!(pending.apply_issued_ack(current, "7:window", 2, 65536, |_| unreachable!()).unwrap());
    close(&mut pending);
    eprintln!("[DEBUG] unpublished exact patch survives failed output and retry without ghost ACK");
}

#[test]
fn reactor_acknowledged_patch_slots_retire_without_instance_close() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixture/🩹️receipt.json")).unwrap();
    let mut pending = PendingPatchAuthority::new();
    for rejection in [false, true] {
        for sequence in 1..=fixture["retirementCycles"].as_u64().unwrap() {
            let receipt = ActorUiPatchReceipt { patch_sequence: sequence, ..receipt(&fixture["issued"]) };
            pending.push_external(patch(&fixture["issued"])).unwrap();
            let patch = take(&mut pending);
            pending.stage_emission(receipt, &patch).unwrap();
            pending.commit_emission();
            if rejection {
                assert!(pending.apply_issued_rejection(receipt, "7:window", 2, |_| unreachable!()));
            } else {
                assert!(pending.apply_issued_ack(receipt, "7:window", 2, 65536, |_| unreachable!()).unwrap());
            }
            for turn in 0..1024 {
                if pending.close_step().unwrap() {
                    break;
                }
                assert!(turn < 1023);
            }
            assert!(pending.slots.iter().all(Option::is_none));
            assert!(pending.closing_instances.iter().all(Option::is_none));
            assert!(pending.has_capacity());
            assert!(!pending.has_unpublished());
        }
    }
    eprintln!("[DEBUG] acknowledged patch slots recycle beyond full capacity without instance close");
}
