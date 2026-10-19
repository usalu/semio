/// 🗝️ Registry bounds, reaper and publication bounds: the registry maximum plus one generation, the mounted test session, the detached reaper rounds and its exact output and the publication maximum plus one.

#[test]
fn guest_cold_relay_registry_max_plus_one_generation_and_zero_pump_are_exact() {
    let registry = GuestRelayMountedRegistry::new(test_relay_wake_authority());
    let mut generations = Vec::new();
    for _ in 0..GUEST_RELAY_MOUNTED_SLOTS {
        let (index, generation) = registry.reserve().expect("exact mounted relay capacity");
        let output_identity = {
            let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let GuestRelayMountedSlot::Reserved { output, .. } = &*slot else { panic!("reserved output owner") };
            output.storage.as_ptr()
        };
        generations.push((index, generation, output_identity));
    }
    assert!(registry.reserve().is_none());
    assert!(generations.windows(2).all(|pair| pair[0].1 < pair[1].1));
    assert!(!registry.has_detached());
    for (index, generation, output_identity) in generations {
        let mut slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let GuestRelayMountedSlot::Reserved { generation: actual, output } = &*slot else { panic!("reserved output remains mounted") };
        assert_eq!(*actual, generation);
        assert_eq!(output.storage.as_ptr(), output_identity);
        *slot = GuestRelayMountedSlot::Empty;
    }
    let exhausted = GuestRelayMountedRegistry::new(test_relay_wake_authority());
    exhausted.next_generation.store(u64::MAX, std::sync::atomic::Ordering::Release);
    let (index, generation) = exhausted.reserve().expect("last generation remains admissible");
    assert_eq!(generation, u64::MAX);
    *exhausted.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Empty;
    assert!(exhausted.reserve().is_none());
}

fn mounted_test_session(generation: u64, lifecycle: GuestRelayMountedLifecycle, output: &[u8]) -> GuestRelayMountedSession {
    let mut mounted_output = GuestRelayMountedOutput::new();
    mounted_output.write_page(output).expect("mounted test output");
    GuestRelayMountedSession {
        generation,
        wake:GuestRelayWakeCustody::new(),
        owner: GuestRelayMountedOwner::Empty,
        checked_out: None,
        lifecycle_probe_checked_out: None,
        outcome_stage: GuestRelayOutcomeStage::Idle,
        outcome_kind: GuestRelayOutcomeKind::Continues,
        outcome_page: 0,
        output: mounted_output,
        terminal: Some(GuestRelayMountedTerminal::Complete),
        lifecycle,
    }
}

#[test]
fn detached_reaper_reclaims_one_slot_per_opportunity_round_robin_and_refuses_stale_generation() {
    let fixture = relay_lifecycle_fixture();
    assert_eq!(fixture["schemaVersion"], 1);
    assert_eq!(fixture["capacities"]["mountedRelaySlots"].as_u64(), Some(GUEST_RELAY_MOUNTED_SLOTS as u64));
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone(),test_relay_wake_authority()));
    let mut generations = Vec::new();
    for _ in 0..GUEST_RELAY_MOUNTED_SLOTS {
        let (index, generation) = registry.reserve().expect("full detached fixture capacity");
        registry.mount(index, generation, GuestRelayMountedOwner::Empty);
        let mut slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let GuestRelayMountedSlot::Mounted(session) = &mut *slot else { panic!("mounted fixture") };
        session.lifecycle = GuestRelayMountedLifecycle::DetachedForReap;
        generations.push((index, generation));
    }
    for released in 1..=GUEST_RELAY_MOUNTED_SLOTS {
        assert!(matches!(registry.reap_one(std::task::Waker::noop()), GuestRelayMountedReap::Progress));
        let empty = registry.slots.iter().filter(|slot| matches!(&*slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty)).count();
        assert_eq!(empty, released, "one reaper opportunity releases one rotating slot");
    }
    assert!(matches!(registry.reap_one(std::task::Waker::noop()), GuestRelayMountedReap::Idle));
    let (index, generation) = registry.reserve().expect("reclaimed capacity is reusable");
    registry.mount(index, generation, GuestRelayMountedOwner::Empty);
    registry.detach(index, generations[0].1);
    let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert!(matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.generation == generation && session.lifecycle == GuestRelayMountedLifecycle::Running));
    drop(slot);
    *registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Empty;
    pool.shutdown().expect("worker shutdown");
}

#[test]
fn detached_reaper_never_steals_a_live_draining_callers_exact_output() {
    let fixture = relay_lifecycle_fixture();
    let expected = fixture["traces"].as_array().expect("fixture traces").iter().find(|trace| trace["🪪️id"] == "relay-live-terminal-caller-output").and_then(|trace| trace["expected"]["callerOutput"].as_str()).expect("caller output trace");
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone(),test_relay_wake_authority()));
    let (detached_index, detached_generation) = registry.reserve().expect("detached slot");
    *registry.slots[detached_index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Mounted(mounted_test_session(detached_generation, GuestRelayMountedLifecycle::DetachedForReap, &[]));
    let (live_index, live_generation) = registry.reserve().expect("live slot");
    *registry.slots[live_index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Mounted(mounted_test_session(live_generation, GuestRelayMountedLifecycle::DrainingForCaller, expected.as_bytes()));

    assert!(matches!(registry.reap_one(std::task::Waker::noop()), GuestRelayMountedReap::Progress));
    assert!(matches!(&*registry.slots[detached_index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty));
    assert!(matches!(&*registry.slots[live_index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DrainingForCaller));
    assert!(matches!(registry.pump(live_index, live_generation, std::task::Waker::noop()), std::task::Poll::Ready(Ok(bytes)) if bytes == expected.as_bytes()));
    pool.shutdown().expect("worker shutdown");
}

#[test]
fn guest_cold_relay_publication_max_plus_one_selects_retained_fault_without_losing_source() {
    let mut maximum = GuestRelayPublication::new(GuestRelayPublicationKind::Commit, vec![7; semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES]);
    assert!(!maximum.oversized);
    assert_eq!(maximum.source.len(), semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES);
    let mut plus_one = GuestRelayPublication::new(GuestRelayPublicationKind::Commit, vec![9; semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES + 1]);
    assert!(plus_one.oversized);
    assert_eq!(plus_one.source.len(), semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES + 1);
    assert!(matches!(plus_one.kind, GuestRelayPublicationKind::Fault));
    maximum.begin_close();
    plus_one.begin_close();
    while !maximum.terminal_is_empty() {
        let _ = maximum.close_step(test_relay_drive_policy());
    }
    while !plus_one.terminal_is_empty() {
        let _ = plus_one.close_step(test_relay_drive_policy());
    }
}
