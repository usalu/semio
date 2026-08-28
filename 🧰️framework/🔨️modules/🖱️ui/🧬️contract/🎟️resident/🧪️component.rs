use super::*;

//#region 🧪️ResidentPermits
#[test]
fn retained_resident_fixed_backing_counts_against_the_same_aggregate() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🗃️fixed/🧪️fixture.json")).unwrap();
    let actual = UiResidentPermit::snapshot().unwrap();
    assert_eq!(actual.bytes > 0, fixture["staticCountsAgainstAggregate"].as_bool().unwrap());
    assert_eq!(actual.bytes, UiResidentPermit::contract_backing_bytes());
    assert_eq!(actual.used_slots, 0);
    assert_eq!(UI_RESIDENT_SLOTS, fixture["slots"].as_u64().unwrap() as usize);
    let runtime = fixture["arithmetic"]["runtime"].as_u64().unwrap() as usize;
    assert!(!UiResidentPermit::try_register_runtime_backing(runtime, 0).unwrap());
    assert_eq!(UiResidentPermit::snapshot().unwrap(), actual);
    assert!(UiResidentPermit::try_register_runtime_backing(runtime, 32768).unwrap());
    assert!(UiResidentPermit::try_register_runtime_backing(runtime, 32768).unwrap());
    assert_eq!(UiResidentPermit::try_register_runtime_backing(runtime + 1, 32768), Err(UiResidentFault::StaticBacking));
    let registered = UiResidentPermit::snapshot().unwrap();
    assert_eq!(registered.bytes, actual.bytes + runtime);
    assert_eq!(registered.used_slots, 0);
    let mut root = reserve(1, 65536);
    let mut output = None;
    assert!(root.split_output_into(&mut output, 32768).unwrap());
    assert_eq!(close(&mut root).returned_bytes, 0);
    assert_eq!(close(output.as_mut().unwrap()).returned_bytes, 65536);
    assert_eq!(UiResidentPermit::snapshot().unwrap(), registered);
    let guard = RESIDENT_LEDGER.lock().unwrap();
    assert_eq!(UiResidentPermit::try_register_runtime_backing(runtime, 32768), Err(UiResidentFault::Contended));
    drop(guard);
    eprintln!("[DEBUG] resident-fixed contract={} runtime={runtime} total={} dynamic-slots=64 final-release-excludes-static=true", actual.bytes, registered.bytes);
}

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧪️fixture.json")).unwrap() }
fn empty_snapshot() -> UiResidentSnapshot { UiResidentSnapshot { bytes: UiResidentPermit::fixed_backing_bytes().unwrap(), ..Default::default() } }
fn reserve(items: usize, bytes: usize) -> UiResidentPermit {
    let mut result = None;
    assert!(UiResidentPermit::try_reserve(UiResidentLimits { items, bytes }, &mut result, 32768).unwrap());
    result.unwrap()
}
fn close(owner: &mut UiResidentPermit) -> UiResidentProgress {
    let progress = owner.close_step(1).unwrap();
    assert!(progress.complete && owner.terminal_is_empty());
    progress
}
fn drain() {
    for _ in 0..UI_RESIDENT_SLOTS * 4 { UiResidentPermit::drain_one().unwrap(); }
}

#[test]
fn retained_resident_permit_preserves_existing_capacity_and_paired_final_return() {
    let data = fixture();
    assert_eq!(UI_RESIDENT_SLOTS, data["slots"].as_u64().unwrap() as usize);
    let mut owners = (0..data["smallReservations"].as_u64().unwrap()).map(|_| reserve(1, 65536)).collect::<Vec<_>>();
    assert_eq!(UiResidentPermit::snapshot().unwrap().bytes, empty_snapshot().bytes + 9 * 65536);
    let mut output = None;
    assert!(!owners[0].split_output_into(&mut output, 0).unwrap());
    assert!(owners[0].split_output_into(&mut output, 32768).unwrap());
    assert_eq!(close(&mut owners[0]).returned_bytes, 0);
    assert_eq!(UiResidentPermit::snapshot().unwrap().used_slots, 9);
    assert_eq!(close(output.as_mut().unwrap()).returned_bytes, 65536);
    for owner in &mut owners[1..] { close(owner); }
    assert_eq!(UiResidentPermit::snapshot().unwrap(), empty_snapshot());
    let mut remaining = UI_RESIDENT_AGGREGATE_BYTES - empty_snapshot().bytes;
    let mut full = (0..4).map(|_| { let bytes = remaining.min(UI_RESIDENT_SURFACE_BYTES); remaining -= bytes; reserve(1, bytes) }).collect::<Vec<_>>();
    assert_eq!(remaining, 0);
    assert_eq!(UiResidentPermit::snapshot().unwrap().bytes, UI_RESIDENT_AGGREGATE_BYTES);
    let mut rejected = None;
    assert_eq!(UiResidentPermit::try_reserve(UiResidentLimits { items: 1, bytes: 1 }, &mut rejected, 32768), Err(UiResidentFault::Capacity));
    assert!(rejected.is_none());
    for owner in &mut full { close(owner); }
    let mut slots = (0..64).map(|_| reserve(1, 1)).collect::<Vec<_>>();
    assert_eq!(UiResidentPermit::try_reserve(UiResidentLimits { items: 0, bytes: 0 }, &mut rejected, 32768), Err(UiResidentFault::Capacity));
    let old = slots[0].key.unwrap();
    close(&mut slots[0]);
    let mut reused = reserve(1, 1);
    assert_eq!(reused.key.unwrap().slot, old.slot);
    assert_eq!(reused.key.unwrap().epoch, old.epoch + 1);
    drop(slots.remove(0));
    drain();
    assert_eq!(UiResidentPermit::snapshot().unwrap().used_slots, 64);
    for owner in &mut slots { close(owner); }
    close(&mut reused);
    assert_eq!(UiResidentPermit::snapshot().unwrap(), empty_snapshot());
    eprintln!("[DEBUG] resident-permit small=9 slots=64 aggregate=33554432 paired-return=0,65536 explicit-close-drop-does-not-return-again=true");
}

#[test]
fn retained_resident_permit_contention_keeps_authority_and_deferred_return_does_not_wait() {
    let mut root = reserve(17, 65536);
    let mut output = None;
    assert!(root.split_output_into(&mut output, 32768).unwrap());
    let (entered_tx, entered_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let holder = std::thread::spawn(move || { let _ledger = RESIDENT_LEDGER.lock().unwrap(); entered_tx.send(()).unwrap(); release_rx.recv().unwrap(); });
    entered_rx.recv().unwrap();
    let blocked = root.close_step(1).unwrap();
    let drain_blocked = UiResidentPermit::drain_one().unwrap();
    drop(output);
    let still_owned = !root.terminal_is_empty();
    release_tx.send(()).unwrap(); holder.join().unwrap();
    assert!(!blocked.progressed && !drain_blocked.progressed && still_owned);
    assert_eq!(UiResidentPermit::snapshot().unwrap().bytes, empty_snapshot().bytes + 65536);
    drain();
    assert_eq!(UiResidentPermit::snapshot().unwrap().bytes, empty_snapshot().bytes + 65536);
    assert!(root.try_shrink(UiResidentLimits { items: 3, bytes: 4096 }).unwrap());
    assert_eq!(close(&mut root).returned_bytes, 4096);
    drain();
    assert_eq!(UiResidentPermit::snapshot().unwrap(), empty_snapshot());
}

#[test]
fn retained_resident_permit_cross_worker_returns_cannot_release_reused_epoch() {
    for _ in 0..fixture()["crossWorkerRounds"].as_u64().unwrap() {
        let mut root = reserve(1, 4096);
        let old = root.key.unwrap();
        let mut output = None;
        assert!(root.split_output_into(&mut output, 32768).unwrap());
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(3));
        let first_barrier = barrier.clone();
        let second_barrier = barrier.clone();
        let first = std::thread::spawn(move || { first_barrier.wait(); drop(root); });
        let second = std::thread::spawn(move || { second_barrier.wait(); drop(output); });
        barrier.wait();
        for _ in 0..100000 {
            UiResidentPermit::drain_one().unwrap();
            if UiResidentPermit::snapshot().unwrap().used_slots == 0 { break; }
            std::thread::yield_now();
        }
        let mut next = reserve(1, 2048);
        assert_eq!(next.key.unwrap().slot, old.slot);
        assert_eq!(next.key.unwrap().epoch, old.epoch + 1);
        first.join().unwrap(); second.join().unwrap();
        drain();
        assert_eq!(UiResidentPermit::snapshot().unwrap().bytes, empty_snapshot().bytes + 2048);
        close(&mut next);
    }
    assert_eq!(UiResidentPermit::snapshot().unwrap(), empty_snapshot());
}
//#endregion 🧪️ResidentPermits
