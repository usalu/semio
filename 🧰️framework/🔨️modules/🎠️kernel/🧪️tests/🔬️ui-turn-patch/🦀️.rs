use super::*;

#[test]
fn ui_turn_patch_owner_drop_hands_back_without_waiting_for_arena() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚪️turn-patch-owner/🔣️.json")).unwrap();
    let mut owner = UiTurnPatches::default();
    owner.try_push_ui_patch(patch(1)).unwrap();
    let key = owner.entries[0].retirement.unwrap();
    let (send, receive) = std::sync::mpsc::channel();
    let (waited, worker) = with_ui_turn_patch_retire_arena(|_| {
        let worker = std::thread::spawn(move || {
            drop(owner);
            send.send(()).unwrap();
        });
        (receive.recv_timeout(std::time::Duration::from_millis(100)).is_err(), worker)
    });
    worker.join().unwrap();
    for turn in 0..4096 {
        close_ui_turn_patch_owner_one();
        if with_ui_turn_patch_retire_arena(|arena| !arena.slots[key.slot].reserved) {
            break;
        }
        assert!(turn < 4095);
    }
    assert_eq!(waited, fixture["dropWaitsForArena"].as_bool().unwrap());
}

#[test]
fn ui_turn_patch_owner_normal_close_does_not_wait_for_arena() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚪️turn-patch-owner/🔣️.json")).unwrap();
    let (send, receive) = std::sync::mpsc::channel();
    let (waited, worker) = with_ui_turn_patch_retire_arena(|_| {
        let worker = std::thread::spawn(move || {
            close_ui_turn_patch_owner_one();
            send.send(()).unwrap();
        });
        (receive.recv_timeout(std::time::Duration::from_millis(100)).is_err(), worker)
    });
    worker.join().unwrap();
    assert_eq!(waited, fixture["normalStepWaitsForArena"].as_bool().unwrap());
}

#[test]
fn ui_turn_patch_owner_typed_descendants_preserve_exact_one_byte_grants() {
    use semio_framework_ui_contract as ui;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚪️turn-patch-owner/🔣️.json")).unwrap();
    for grant in fixture["byteGrants"].as_array().unwrap() {
        let surface = fixture["surface"].as_str().unwrap();
        let text = fixture["payload"].as_str().unwrap();
        let mut patch = UiPatch { surface: ui::SurfaceId::try_from(surface).unwrap(), base_revision: ui::UiRevision(0), revision: ui::UiRevision(1), ops: Default::default() };
        patch.ops.try_push(UiPatchOp::SetComponent { id: ui::UiNodeId(7), component: ui::Component::Text(ui::TextProps { value: ui::Label(ui::UiText::try_from_str(text).unwrap()), emphasize: None, data_attributes: None }) }).unwrap();
        let expected = serde_json::to_value(&patch).unwrap();
        let mut owner = UiTurnPatches::default();
        owner.try_push_ui_patch(patch).unwrap();
        assert_eq!(owner.close_step_with_grant(0, 4096).unwrap(), ui::UiValueRetirementStep::default());
        assert_eq!(owner.close_step_with_grant(1, 0).unwrap(), ui::UiValueRetirementStep::default());
        assert_eq!(serde_json::to_value(owner.iter().next().unwrap()).unwrap(), expected);
        let grant = grant.as_u64().unwrap() as usize;
        let mut bytes = 0;
        for turn in 0..65_536 {
            let step = owner.close_step_with_grant(1, grant).unwrap();
            assert!(step.released_items <= 1 && step.released_bytes <= grant);
            bytes += step.released_bytes;
            if step.complete {
                break;
            }
            assert!(turn < 65_535);
        }
        assert_eq!(bytes, surface.len() + text.len());
        assert!(owner.entries[0].contents.terminal_is_empty() && owner.entries[0].retirement.is_none());
    }
}

fn patch(revision: u64) -> UiPatch {
    UiPatch {
        surface: semio_framework_ui_contract::SurfaceId::try_from("turn.surface").expect("bounded surface"),
        base_revision: semio_framework_ui_contract::UiRevision(revision.saturating_sub(1)),
        revision: semio_framework_ui_contract::UiRevision(revision),
        ops: semio_framework_ui_contract::UiPatchOps::default(),
    }
}

/// 🧾️ The page carries every publication up to its DERIVED capacity, in order, and the one past the
/// capacity comes back as the exact owner its caller handed in — the count is no longer `1`, so this
/// law also pins the derivation and the contiguous extent the page may not exceed.
#[test]
fn ui_turn_patches_max_plus_one_returns_the_exact_patch_owner() {
    assert_eq!(UI_TURN_PATCHES_MAXIMUM, semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / UI_TURN_PATCH_OWNER_BYTES);
    assert!(UI_TURN_PATCHES_MAXIMUM > 1 && size_of::<UiTurnPatches>() <= semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES);
    let mut patches = UiTurnPatches::default();
    for revision in 1..=UI_TURN_PATCHES_MAXIMUM {
        patches.try_push_ui_patch(patch(revision as u64)).expect("maximum owner");
    }
    let rejected = patches.try_push_ui_patch(patch(9_001)).expect_err("maximum plus one");
    assert_eq!(rejected.revision, semio_framework_ui_contract::UiRevision(9_001));
    assert_eq!(patches.len(), UI_TURN_PATCHES_MAXIMUM);
    assert_eq!(patches.iter().map(|patch| patch.revision.0).collect::<Vec<_>>(), (1..=UI_TURN_PATCHES_MAXIMUM as u64).collect::<Vec<_>>());
    drop(patches);
    while close_ui_turn_patch_owner_one() {}
}

#[test]
fn refused_turn_patch_transfer_restores_the_exact_retirement_owner() {
    let mut patches = UiTurnPatches::default();
    patches.try_push_ui_patch(patch(1)).expect("one patch");
    assert!(matches!(patches.try_transfer_one(Err::<(), UiPatch>), UiTurnPatchTransfer::Refused));
    assert_eq!(patches.len(), 1);
    assert_eq!(patches.iter().next().map(|patch| patch.revision), Some(semio_framework_ui_contract::UiRevision(1)));
    drop(patches);
    while !close_ui_turn_patch_owner_one() {}
}

#[test]
fn ui_turn_patches_fixed_serde_visitor_rejects_plus_one() {
    let encoded = serde_json::to_vec(&(1..=UI_TURN_PATCHES_MAXIMUM as u64 + 1).map(patch).collect::<Vec<_>>()).expect("bounded fixture encoding");
    let error = serde_json::from_slice::<UiTurnPatches>(&encoded).expect_err("visitor maximum plus one");
    assert!(error.to_string().contains("turn patch page capacity exceeded"));
}

#[test]
fn ui_turn_patches_close_retires_one_op_or_patch_owner_per_step() {
    let mut owner = patch(1);
    owner.ops.try_push(UiPatchOp::SetRoot { id: semio_framework_ui_contract::UiNodeId(7) }).expect("one op");
    let mut patches = UiTurnPatches::default();
    patches.try_push_ui_patch(owner).expect("one patch");
    for turn in 0..4096 {
        let step = patches.close_step_with_grant(1, 4096).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= 4096);
        if step.complete {
            break;
        }
        assert!(turn < 4095);
    }
    assert!(patches.entries.iter().all(|entry| entry.contents.terminal_is_empty() && entry.retirement.is_none()));
}

#[test]
fn ui_turn_patch_retirement_max_plus_one_refuses_before_owner_transfer() {
    let mut arena = UiTurnPatchRetireArena::default();
    let mut keys = [None; UI_TURN_PATCH_RETIRE_SLOTS];
    for key in &mut keys {
        *key = arena.reserve();
        assert!(key.is_some());
    }
    assert!(arena.reserve().is_none());
    for key in keys.into_iter().flatten() {
        assert!(arena.release_empty(key));
    }
}

#[test]
fn ui_turn_patch_retirement_rejects_stale_epoch_release_and_closes_one_owner_per_step() {
    let mut arena = UiTurnPatchRetireArena::default();
    let key = arena.reserve().expect("fixed retirement slot");
    let stale = UiTurnPatchRetireKey { epoch: key.epoch.checked_add(1).expect("fixture epoch"), ..key };
    assert!(!arena.release_empty(stale));
    let mut owner = patch(1);
    owner.ops.try_push(UiPatchOp::SetRoot { id: semio_framework_ui_contract::UiNodeId(7) }).expect("one op");
    let mut contents = UiTurnPatchContents::default();
    *contents.pending.source_mut().unwrap() = Some(owner);
    arena.handback(key, contents).unwrap();
    assert!(arena.close_one(1, 4096));
    assert!(arena.slots[key.slot].reserved);
    for turn in 0..4096 {
        arena.close_one(1, 4096);
        if !arena.slots[key.slot].reserved {
            break;
        }
        assert!(turn < 4095);
    }
    assert!(!arena.slots[key.slot].reserved);
}

#[test]
fn ui_turn_patch_transport_round_trip_is_single_claim_and_preserves_populated_owner() {
    let session = 70_001;
    let mut patch = patch(3);
    patch.ops.try_push(UiPatchOp::SetRoot { id: semio_framework_ui_contract::UiNodeId(9) }).expect("one populated op");
    let mut owner = UiTurnPatches::default();
    owner.try_push_ui_patch(patch).expect("one patch");
    let mut producer = UiTurnPatchTransportProducer::try_new(session, owner).expect("fixed transport admission");
    assert_eq!(producer.drive_one(session, false, true), UiTurnPatchTransportStep::MoreWork);
    assert_eq!((producer.patch, producer.operation), (0, 0));
    assert_eq!(producer.drive_one(session, false, false), UiTurnPatchTransportStep::MoreWork);
    assert_eq!(producer.drive_one(session, false, false), UiTurnPatchTransportStep::MoreWork);
    assert_eq!(producer.drive_one(session, false, false), UiTurnPatchTransportStep::Ready);
    let token = producer.take_ready().expect("valid publication authority").expect("complete token");
    let lease = UiTurnPatchTransportLease::try_from_token(&token, session).expect("first exact claim");
    assert!(UiTurnPatchTransportLease::try_from_token(&token, session).is_err());
    let mut owner = match lease.take_owner() {
        Ok(owner) => owner,
        Err(_) => panic!("exact transport owner"),
    };
    let patch = owner.iter().next().expect("one patch");
    assert_eq!(patch.revision, semio_framework_ui_contract::UiRevision(3));
    assert_eq!(patch.ops.len(), 1);
    while !owner.close_step() {}
}

/// 🪪️ A transport session is the granted actor's own id, and `ActorId(0)` is a real actor — the first
/// app of the first plugin a host activates (plugin ordinal, kind, ordinal and generation all zero).
/// Refusing session 0 faulted every patch-carrying turn of that actor ("fixed turn patch transport
/// admission refused the exact owner"), so a native shell's first mounted guest could paint nothing
/// (ticket 26/09/23 slice WG8, measured on block2d).
#[test]
fn ui_turn_patch_transport_admits_the_zero_actor_as_its_session() {
    let mut owner = UiTurnPatches::default();
    owner.try_push_ui_patch(patch(4)).expect("one patch");
    let mut producer = UiTurnPatchTransportProducer::try_new(0, owner).expect("actor 0 owns a transport session");
    while producer.drive_one(0, false, false) != UiTurnPatchTransportStep::Ready {}
    let token = producer.take_ready().expect("valid publication authority").expect("complete token");
    assert!(UiTurnPatchTransportLease::try_from_token(&token, 1).is_err(), "another actor never claims actor 0's patches");
    let lease = UiTurnPatchTransportLease::try_from_token(&token, 0).expect("actor 0 claims its own patches");
    let mut owner = match lease.take_owner() {
        Ok(owner) => owner,
        Err(_) => panic!("exact transport owner"),
    };
    assert_eq!(owner.iter().next().expect("one patch").revision, semio_framework_ui_contract::UiRevision(4));
    while !owner.close_step() {}
}

#[test]
fn ui_turn_patch_transport_producer_drop_hands_back_without_waiting_for_arena() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚪️turn-patch-owner/🔣️.json")).unwrap();
    let mut owner = UiTurnPatches::default();
    owner.try_push_ui_patch(patch(7)).unwrap();
    let producer = UiTurnPatchTransportProducer::try_new(700_011, owner).unwrap();
    let key = producer.key;
    let (send, receive) = std::sync::mpsc::channel();
    let (waited, worker) = with_ui_turn_patch_transport_arena(|_| {
        let worker = std::thread::spawn(move || {
            drop(producer);
            send.send(()).unwrap();
        });
        (receive.recv_timeout(std::time::Duration::from_millis(100)).is_err(), worker)
    });
    worker.join().unwrap();
    for turn in 0..4096 {
        close_ui_turn_patch_transport_one().unwrap();
        if with_ui_turn_patch_transport_arena(|arena| arena.slot_mut(key).is_none()) {
            break;
        }
        assert!(turn < 4095);
    }
    assert!(with_ui_turn_patch_transport_arena(|arena| arena.slot_mut(key).is_none()));
    assert_eq!(waited, fixture["transportProducerDropWaitsForArena"].as_bool().unwrap());
}

#[test]
fn ui_turn_patch_transport_lease_drop_hands_back_without_waiting_for_arena() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚪️turn-patch-owner/🔣️.json")).unwrap();
    let mut owner = UiTurnPatches::default();
    owner.try_push_ui_patch(patch(8)).unwrap();
    let mut producer = UiTurnPatchTransportProducer::try_new(700_012, owner).unwrap();
    while producer.drive_one(700_012, false, false) != UiTurnPatchTransportStep::Ready {}
    let lease = UiTurnPatchTransportLease::try_from_token(&producer.take_ready().unwrap().unwrap(), 700_012).unwrap();
    let key = lease.key;
    let (send, receive) = std::sync::mpsc::channel();
    let (waited, worker) = with_ui_turn_patch_transport_arena(|_| {
        let worker = std::thread::spawn(move || {
            drop(lease);
            send.send(()).unwrap();
        });
        (receive.recv_timeout(std::time::Duration::from_millis(100)).is_err(), worker)
    });
    worker.join().unwrap();
    for turn in 0..4096 {
        close_ui_turn_patch_transport_one().unwrap();
        if with_ui_turn_patch_transport_arena(|arena| arena.slot_mut(key).is_none()) {
            break;
        }
        assert!(turn < 4095);
    }
    assert!(with_ui_turn_patch_transport_arena(|arena| arena.slot_mut(key).is_none()));
    assert_eq!(waited, fixture["transportLeaseDropWaitsForArena"].as_bool().unwrap());
}

#[test]
fn ui_turn_patch_transport_normal_close_does_not_wait_for_arena() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚪️turn-patch-owner/🔣️.json")).unwrap();
    let (send, receive) = std::sync::mpsc::channel();
    let (waited, worker) = with_ui_turn_patch_transport_arena(|_| {
        let worker = std::thread::spawn(move || {
            close_ui_turn_patch_transport_one().unwrap();
            send.send(()).unwrap();
        });
        (receive.recv_timeout(std::time::Duration::from_millis(100)).is_err(), worker)
    });
    worker.join().unwrap();
    assert_eq!(waited, fixture["transportNormalStepWaitsForArena"].as_bool().unwrap());
}

#[test]
fn ui_turn_patch_transport_session_close_waits_for_exact_external_handback() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚪️turn-patch-owner/🔣️.json")).unwrap();
    let mut producer = UiTurnPatchTransportProducer::try_new(700_013, UiTurnPatches::default()).unwrap();
    let key = producer.key;
    assert!(matches!(close_ui_turn_patch_transport_session_one(700_013).unwrap(), UiTurnPatchTransportProgress::Pending { .. }));
    assert_eq!(close_ui_turn_patch_transport_one().unwrap() == UiTurnPatchTransportProgress::Blocked, fixture["transportBlocksExternalReuse"].as_bool().unwrap());
    assert!(with_ui_turn_patch_transport_arena(|arena| arena.slot_mut(key).is_some_and(|slot| slot.external)));
    assert_eq!(producer.drive_one(700_013, false, false), UiTurnPatchTransportStep::Cancelled);
    while matches!(close_ui_turn_patch_transport_one().unwrap(), UiTurnPatchTransportProgress::Pending { .. }) {}
    let replacement = UiTurnPatchTransportProducer::try_new(700_014, UiTurnPatches::default()).unwrap();
    assert_eq!(replacement.key.slot, key.slot);
    assert_ne!(replacement.key.epoch, key.epoch);
    drop(producer);
    assert_eq!(close_ui_turn_patch_transport_one().unwrap(), UiTurnPatchTransportProgress::Idle);
    assert!(with_ui_turn_patch_transport_arena(|arena| arena.slot_mut(replacement.key).is_some_and(|slot| slot.external)));
    drop(replacement);
    while matches!(close_ui_turn_patch_transport_one().unwrap(), UiTurnPatchTransportProgress::Pending { .. }) {}
}

#[test]
fn ui_turn_patch_transport_poison_retains_exact_owner_until_explicit_test_recovery() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚪️turn-patch-owner/🔣️.json")).unwrap();
    let mut producer = UiTurnPatchTransportProducer::try_new(700_015, UiTurnPatches::default()).unwrap();
    assert_eq!(producer.drive_one(700_015, false, false), UiTurnPatchTransportStep::Ready);
    let key = producer.key;
    let panic = std::panic::catch_unwind(|| with_ui_turn_patch_transport_arena(|_| panic!("controlled transport mutex poison")));
    let drive_fault = matches!(producer.drive_one(700_015, false, false), UiTurnPatchTransportStep::Fault(_));
    let close_fault = close_ui_turn_patch_transport_one().is_err();
    let publish_fault = producer.take_ready().is_err();
    drop(producer);
    let retained = UI_TURN_PATCH_TRANSPORT_HANDBACKS[key.slot].ready.load(std::sync::atomic::Ordering::Acquire);
    UI_TURN_PATCH_TRANSPORT_ARENA.clear_poison();
    while matches!(close_ui_turn_patch_transport_one().unwrap(), UiTurnPatchTransportProgress::Pending { .. }) {}
    assert!(panic.is_err() && retained);
    assert_eq!(drive_fault && close_fault && publish_fault, fixture["transportPoisonIsFault"].as_bool().unwrap());
    assert!(with_ui_turn_patch_transport_arena(|arena| arena.slot_mut(key).is_none()));
}

#[test]
fn ui_turn_patch_transport_handback_reports_exact_typed_descendant_bytes() {
    use semio_framework_ui_contract as ui;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚪️turn-patch-owner/🔣️.json")).unwrap();
    let surface = fixture["surface"].as_str().unwrap();
    let text = fixture["payload"].as_str().unwrap();
    let mut patch = UiPatch { surface: ui::SurfaceId::try_from(surface).unwrap(), base_revision: ui::UiRevision(0), revision: ui::UiRevision(1), ops: Default::default() };
    patch.ops.try_push(UiPatchOp::SetComponent { id: ui::UiNodeId(7), component: ui::Component::Text(ui::TextProps { value: ui::Label(ui::UiText::try_from_str(text).unwrap()), emphasize: None, data_attributes: None }) }).unwrap();
    let mut owner = UiTurnPatches::default();
    owner.try_push_ui_patch(patch).unwrap();
    let mut producer = UiTurnPatchTransportProducer::try_new(700_016, owner).unwrap();
    while producer.drive_one(700_016, false, false) != UiTurnPatchTransportStep::Ready {}
    let token = producer.take_ready().unwrap().unwrap();
    let lease = UiTurnPatchTransportLease::try_from_token(&token, 700_016).unwrap();
    let key = lease.key;
    drop(lease);
    let mut bytes = 0;
    for turn in 0..65_536 {
        match close_ui_turn_patch_transport_one().unwrap() {
            UiTurnPatchTransportProgress::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1 && released_bytes <= 4096);
                bytes += released_bytes;
            }
            UiTurnPatchTransportProgress::Blocked => {}
            UiTurnPatchTransportProgress::Idle => break,
        }
        assert!(turn < 65_535);
    }
    assert_eq!(bytes, surface.len() + text.len());
    assert!(with_ui_turn_patch_transport_arena(|arena| arena.slot_mut(key).is_none()));
    assert!(UiTurnPatchTransportLease::try_from_token(&token, 700_016).is_err());
}

#[test]
fn ui_turn_patch_transport_rejects_truncated_stale_and_cancelled_tokens() {
    assert!(UiTurnPatchTransportLease::try_from_token(&[0; UI_TURN_PATCH_TRANSPORT_TOKEN_BYTES - 1], 81).is_err());
    let mut producer = UiTurnPatchTransportProducer::try_new(81, UiTurnPatches::default()).expect("fixed transport admission");
    assert_eq!(producer.drive_one(82, false, false), UiTurnPatchTransportStep::Stale);
    assert_eq!(producer.drive_one(81, true, false), UiTurnPatchTransportStep::Cancelled);
    drop(producer);
    while matches!(close_ui_turn_patch_transport_one().unwrap(), UiTurnPatchTransportProgress::Pending { .. }) {}
}

#[test]
fn ui_turn_patch_transport_max_plus_one_returns_exact_owner_and_session_close_is_incremental() {
    let mut arena = UiTurnPatchTransportArena::default();
    for session in 1..=UI_TURN_PATCH_TRANSPORT_SLOTS {
        let key = arena.reserve(u64::try_from(session).expect("bounded session"), UiTurnPatches::default()).expect("fixed slot");
        arena.slot_mut(key).unwrap().external = false;
    }
    let rejected = arena.reserve(90_001, UiTurnPatches::default()).expect_err("maximum plus one");
    assert!(rejected.is_empty());
    assert!(arena.request_session_close(1));
    assert!(matches!(arena.close_one(1, 4096).unwrap(), UiTurnPatchTransportProgress::Pending { released_items: 1, .. }));
    assert_eq!(arena.slots.iter().filter(|slot| slot.state != UiTurnPatchTransportState::Vacant).count(), UI_TURN_PATCH_TRANSPORT_SLOTS - 1);
}
