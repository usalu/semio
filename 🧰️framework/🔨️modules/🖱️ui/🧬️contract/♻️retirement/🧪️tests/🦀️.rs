use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧪️fixture.json")).unwrap()
}

fn value_handle(value: &UiValue) -> Option<UiCollectionHandle> {
    match value { UiValue::List(value) => value.handle, UiValue::Map(value) => value.handle, _ => None }
}

pub(super) fn descendants(value: &UiValue) -> Vec<UiCollectionHandle> {
    let mut pending: Vec<_> = value_handle(value).into_iter().collect();
    let mut result = Vec::new();
    with_ui_value_arena(|arena| {
        while let Some(handle) = pending.pop() {
            if result.contains(&handle) { continue; }
            result.push(handle);
            let mut page = arena.collection(handle).unwrap().head;
            while page != UI_VALUE_NONE {
                let value = match arena.pages[page].value.as_ref().unwrap() { UiPageValue::List(value) | UiPageValue::Map(_, value) => value };
                if let Some(nested) = value_handle(value) { pending.push(nested); }
                page = arena.pages[page].next;
            }
        }
    });
    result
}

fn close(owner: &mut UiValueRetirement, grant: usize) -> (usize, usize) {
    let mut items = 0;
    let mut bytes = 0;
    for _ in 0..100_000 {
        let step = owner.close_step(1, grant).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= grant);
        items += step.released_items;
        bytes += step.released_bytes;
        if step.complete { assert!(owner.terminal_is_empty()); return (items, bytes); }
        if !step.progressed { std::thread::yield_now(); }
    }
    panic!("exact value owner did not finish");
}

#[test]
fn instance_lifetime_ui_value_retirement_preserves_every_descendant_and_byte_grant() {
    let fixture = fixture();
    for grant in fixture["grants"].as_array().unwrap() {
        for row in fixture["cases"].as_array().unwrap() {
            let value: UiValue = serde_json::from_value(row["value"].clone()).unwrap();
            let handles = descendants(&value);
            let mut owner = UiValueRetirement::new(value);
            assert_eq!(owner.close_step(0, 4096).unwrap(), UiValueRetirementStep::default());
            assert_eq!(owner.close_step(1, 0).unwrap(), UiValueRetirementStep::default());
            let (items, bytes) = close(&mut owner, grant.as_u64().unwrap() as usize);
            assert_eq!(bytes, row["textBytes"].as_u64().unwrap() as usize, "{}", row["name"]);
            let expected = row["pages"].as_u64().unwrap() + row["collections"].as_u64().unwrap();
            assert_eq!(items as u64, expected.max(1), "{}", row["name"]);
            assert!(with_ui_value_arena(|arena| handles.iter().all(|handle| arena.collection(*handle).is_none())));
        }
    }
}

#[test]
fn instance_lifetime_ui_value_retirement_releases_alias_without_touching_other_roots() {
    let fixture = fixture();
    let value: UiValue = serde_json::from_value(fixture["cases"][0]["value"].clone()).unwrap();
    let mut alias = UiValueRetirement::new(value.credited_clone().unwrap());
    assert_eq!(close(&mut alias, 1), (1, 0));
    assert_eq!(serde_json::to_value(&value).unwrap(), fixture["cases"][0]["value"]);
    let unrelated: UiValue = serde_json::from_value(serde_json::json!(["unrelated"])).unwrap();
    let unrelated_handle = value_handle(&unrelated).unwrap();
    drop(unrelated);
    let mut owner = UiValueRetirement::new(value);
    close(&mut owner, 1);
    assert!(with_ui_value_arena(|arena| arena.collection(unrelated_handle).is_some()));
    while !close_ui_value_page_one() {}
}

#[test]
fn instance_lifetime_ui_value_retirement_epoch_reuse_and_terminal_guard() {
    let mut arena = UiValueArena::default();
    let handbacks = UiArenaHandbacks::<UI_VALUE_ADMISSION_SLOTS, 4>::new();
    let original = arena.reserve_collection(UiCollectionKind::List).unwrap();
    arena.try_push_page(original, UiPageValue::List(UiValue::Text(UiText::try_from_str("first").unwrap()))).unwrap();
    arena.release_exact_handle(original).unwrap();
    while arena.collection(original).is_some() { arena.advance_exact_root(original, 1, &handbacks).unwrap(); }
    let replacement = arena.reserve_collection(UiCollectionKind::List).unwrap();
    assert_eq!(replacement.slot == original.slot && replacement.epoch != original.epoch, fixture()["ownership"]["exactSlotReusedWithNewEpoch"].as_bool().unwrap());
    arena.try_push_page(replacement, UiPageValue::List(UiValue::Text(UiText::try_from_str("replacement").unwrap()))).unwrap();
    assert!(arena.advance_exact_root(original, 4096, &handbacks).unwrap().complete);
    let page = arena.collection(replacement).unwrap().head;
    assert!(matches!(arena.pages[page].value.as_ref(), Some(UiPageValue::List(UiValue::Text(value))) if value.as_str() == "replacement"));
    arena.release_exact_handle(replacement).unwrap();
    while arena.collection(replacement).is_some() { arena.advance_exact_root(replacement, 1, &handbacks).unwrap(); }
    let result = std::panic::catch_unwind(|| drop(UiValueRetirement::new(UiValue::Text(UiText::try_from_str("live").unwrap()))));
    assert_eq!(result.is_err(), fixture()["ownership"]["liveDropRejected"].as_bool().unwrap());
}

#[test]
fn instance_lifetime_ui_value_retirement_nested_shared_alias_keeps_external_payload() {
    let fixture = fixture();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "shared-child").unwrap();
    let child: UiValue = serde_json::from_value(row["value"].clone()).unwrap();
    let external = child.credited_clone().unwrap();
    let handle = value_handle(&external).unwrap();
    let mut builder = UiListBuilder::try_new().unwrap();
    builder.push(child).unwrap();
    let mut parent = UiValueRetirement::new(UiValue::List(builder.finish()));
    assert_eq!(close(&mut parent, 1), (3, 0));
    assert_eq!(with_ui_value_arena(|arena| arena.collection(handle).is_some_and(|value| value.aliases == 1 && !value.retiring)), fixture["ownership"]["nestedSharedAliasRetainsExternalPayload"].as_bool().unwrap());
    assert_eq!(serde_json::to_value(&external).unwrap(), row["value"]);
    let oracle = &fixture["queuedOracleReader"];
    assert_eq!(with_ui_value_arena(|arena| arena.collection(handle).unwrap().aliases), oracle["aliasesBefore"].as_u64().unwrap());
    assert!(UI_VALUE_HANDBACKS.has_slot_pending(handle.slot));
    let mut owner = UiValueRetirement::new(external);
    let reader_release = owner.close_step(1, 1).unwrap();
    assert!(!reader_release.complete && reader_release.progressed);
    assert_eq!(reader_release.released_items, oracle["readerReleasedItems"].as_u64().unwrap() as usize);
    assert_eq!(reader_release.released_bytes, oracle["readerReleasedBytes"].as_u64().unwrap() as usize);
    assert_eq!(with_ui_value_arena(|arena| arena.collection(handle).unwrap().aliases), oracle["aliasesAfter"].as_u64().unwrap());
    assert!(!UI_VALUE_HANDBACKS.has_slot_pending(handle.slot));
    assert_eq!(close(&mut owner, 1), (oracle["descendantReleasedItems"].as_u64().unwrap() as usize, oracle["descendantReleasedBytes"].as_u64().unwrap() as usize));
}

#[test]
fn instance_lifetime_ui_value_retirement_caught_drop_keeps_exact_arena_owner() {
    for started in [false, true] {
        let value: UiValue = serde_json::from_value(serde_json::json!([{ "child": ["retained"] }])).unwrap();
        let root = value_handle(&value).unwrap();
        let descendants = descendants(&value);
        let mut owner = UiValueRetirement::new(value);
        if started { assert!(owner.close_step(1, 1).unwrap().progressed); }
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(owner)));
        assert!(result.is_err());
        assert_eq!(with_ui_value_arena(|arena| arena.collection(root).is_some()) && UI_VALUE_HANDBACKS.has_slot_pending(root.slot), fixture()["ownership"]["caughtLiveDropKeepsArenaOwner"].as_bool().unwrap());
        while !close_ui_value_page_one() {}
        assert!(with_ui_value_arena(|arena| descendants.iter().all(|handle| arena.collection(*handle).is_none())));
    }
}

#[test]
fn instance_lifetime_ui_value_retirement_global_drain_cannot_consume_claimed_root() {
    let value: UiValue = serde_json::from_value(serde_json::json!(["exact"])).unwrap();
    let handle = value_handle(&value).unwrap();
    let mut owner = UiValueRetirement::new(value);
    assert!(owner.close_step(1, 1).unwrap().progressed);
    while !close_ui_value_page_one() {}
    let consumed = with_ui_value_arena(|arena| {
        let collection = arena.collection(handle).unwrap();
        assert!(collection.retirement_claimed && !collection.retirement_queued);
        !matches!(arena.pages[collection.head].value.as_ref(), Some(UiPageValue::List(UiValue::Text(value))) if value.as_str() == "exact")
    });
    assert_eq!(consumed, fixture()["ownership"]["globalDrainConsumesClaimedRoot"].as_bool().unwrap());
    assert_eq!(close(&mut owner, 1), (2, 5));
}

#[test]
fn instance_lifetime_ui_value_retirement_unwind_guard_is_not_waived() {
    if std::env::var_os("SEMIO_UI_VALUE_RETIREMENT_UNWIND_CHILD").is_some() {
        let _owner = UiValueRetirement::new(UiValue::Text(UiText::try_from_str("live during unwind").unwrap()));
        panic!("intentional retained value unwind");
    }
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("instance_lifetime_ui_value_retirement_unwind_guard_is_not_waived").arg("--nocapture")
        .env("SEMIO_UI_VALUE_RETIREMENT_UNWIND_CHILD", "1").env("RUST_BACKTRACE", "0").output().unwrap();
    assert_eq!(!output.status.success(), fixture()["ownership"]["unwindGuardFatal"].as_bool().unwrap());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("UiValueRetirement requires exact terminal closure"));
    assert!(error.contains("panic in a destructor during cleanup") || error.contains("thread caused non-unwinding panic"));
}

#[test]
fn instance_lifetime_ui_value_retirement_contention_preserves_owner_without_waiting() {
    for started in [false, true] {
        let value: UiValue = serde_json::from_value(serde_json::json!(["contended"])).unwrap();
        let mut owner = UiValueRetirement::new(value);
        if started { assert!(owner.close_step(1, 1).unwrap().progressed); }
        let (ready_tx, ready_rx) = std::sync::mpsc::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        let holder = std::thread::spawn(move || {
            let _guard = UI_VALUE_ARENA.lock().unwrap();
            ready_tx.send(()).unwrap();
            let _ = release_rx.recv_timeout(std::time::Duration::from_millis(200));
        });
        ready_rx.recv().unwrap();
        let blocked = owner.close_step(1, 1).unwrap();
        let _ = release_tx.send(());
        holder.join().unwrap();
        close(&mut owner, 1);
        assert_eq!(blocked.progressed, fixture()["ownership"]["arenaContentionAdvancesOwner"].as_bool().unwrap());
        assert_eq!((blocked.released_items, blocked.released_bytes), (0, 0));
    }
}

fn drop_waits_for_value_arena(mode: u8) -> bool {
    let value: UiValue = serde_json::from_value(serde_json::json!([{"nested":["owned"]}])).unwrap();
    let handles = descendants(&value);
    let root = value_handle(&value).unwrap();
    let action: Box<dyn FnOnce()> = if mode == 0 { Box::new(move || drop(value)) } else {
        let mut owner = UiValueRetirement::new(value);
        if mode == 2 { owner.close_step(1, 1).unwrap(); }
        Box::new(move || drop(owner))
    };
    let (ready_tx, ready_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let released = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let holder_released = released.clone();
    let holder = std::thread::spawn(move || {
        let _guard = UI_VALUE_ARENA.lock().unwrap();
        ready_tx.send(()).unwrap();
        let _ = release_rx.recv_timeout(std::time::Duration::from_millis(200));
        holder_released.store(true, std::sync::atomic::Ordering::Release);
    });
    ready_rx.recv().unwrap();
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(action));
    let waited = released.load(std::sync::atomic::Ordering::Acquire);
    let _ = release_tx.send(());
    holder.join().unwrap();
    let retained = with_ui_value_arena(|arena| arena.collection(root).is_some());
    while !close_ui_value_page_one() {}
    assert_eq!(outcome.is_err(), mode != 0);
    assert!(retained);
    assert!(with_ui_value_arena(|arena| handles.iter().all(|handle| arena.collection(*handle).is_none())));
    waited
}

#[test]
fn instance_lifetime_ui_value_drop_hands_back_without_waiting_for_arena() {
    assert_eq!(drop_waits_for_value_arena(0), fixture()["ownership"]["dropWaitsForArena"].as_bool().unwrap());
}

#[test]
fn instance_lifetime_ui_value_unstarted_guard_hands_back_without_waiting_for_arena() {
    assert_eq!(drop_waits_for_value_arena(1), fixture()["ownership"]["dropWaitsForArena"].as_bool().unwrap());
}

#[test]
fn instance_lifetime_ui_value_claimed_guard_hands_back_without_waiting_for_arena() {
    assert_eq!(drop_waits_for_value_arena(2), fixture()["ownership"]["dropWaitsForArena"].as_bool().unwrap());
}
