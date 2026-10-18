// 📚️ History wake and terminal close laws: the history admission retirement, the public wake and the history completion and terminal close laws.

//#region 🔖️Compact + Sync
//#endregion 🔖️Compact + Sync
//#region 🔖️Retained submit authority
fn retire_history_admission(mut admission: ArtifactHistoryAdmission) {
    let mut cursor = admission.begin_reservation_close().expect("history test admission retained its reservation");
    for _ in 0..50_000 {
        if cursor.terminal_is_empty() {
            break;
        }
        assert!(cursor.close_step());
    }
    assert!(cursor.terminal_is_empty());
}

struct ControlledHistoryPublicWake {
    pub state: std::sync::Weak<ArtifactHistoryState>,
    wakes: std::sync::atomic::AtomicUsize,
    lock_released: std::sync::atomic::AtomicBool,
}

impl std::task::Wake for ControlledHistoryPublicWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(state) = self.state.upgrade() {
            self.lock_released.store(state.waker.try_lock().is_ok(), std::sync::atomic::Ordering::Release);
        }
        self.wakes.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_completion_interleavings_preserve_result_and_wake() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_completion_interleavings_preserve_result_and_wake") {
        return;
    }
    let _history_capacity = db_artifact_history::history_capacity_test_lock();
    use std::sync::atomic::Ordering;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📜️history-completion/🔣️.json")).unwrap();
    let root = tempdir("history-publication").await;
    std::fs::create_dir_all(&root).unwrap();
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId(fixture["document"].as_str().unwrap().into());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    for operation in fixture["operations"].as_array().unwrap() {
        let id = operation.as_str().unwrap();
        let batch = db_artifact_contract::CommandBatch::new(vec![envelope(id, &[], "history-fixture", &document, &[("value", serde_json::json!(id))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..db_artifact_contract::SubmitOptions::default() })).unwrap().unwrap();
    }
    let mut observations = Vec::new();
    for row in fixture["publication"].as_array().unwrap() {
        let mut probe = HistoryFuture::prepare(&handle);
        let state = probe.state.clone();
        let terminal = probe.terminal_handle();
        let generation = probe.generation();
        let admission_slot = state.admission.lock().unwrap().as_ref().unwrap().slot;
        let (captured, capture) = std::sync::mpsc::sync_channel(1);
        *state.controlled_completion_hook.lock().unwrap() = Some(Box::new(move |result, progress| {
            assert!(captured.send((result, progress)).is_ok(), "physical history result capture must remain live");
        }));
        let cancelled = row["outcome"] == "cancelled";
        if cancelled {
            probe.cancel();
        } else {
            state.schedule();
        }
        let (result, progress) = capture.recv_timeout(std::time::Duration::from_secs(5)).expect("actual history producer completion deadline");
        let entry_pointer = result.as_ref().ok().map(|view| view.entries().as_ptr() as usize);
        if let Ok(view) = result.as_ref() {
            assert_eq!(view.entries().len(), fixture["operations"].as_array().unwrap().len());
            assert_eq!(view.admission.as_ref().unwrap().generation, generation);
            assert!(Arc::ptr_eq(view.terminal_state.as_ref().unwrap(), &state));
        }
        assert!(ARTIFACT_HISTORY_ADMISSION.lock().unwrap().slots[admission_slot].occupied);
        let publish_state = state.clone();
        let publish = move || publish_state.complete(result, progress);
        let mut deferred = None;
        match row["publication"].as_str().unwrap() {
            "before-poll" => publish(),
            "before-registration" => *state.controlled_publication_before_waker_hook.lock().unwrap() = Some(Box::new(publish)),
            "after-pending" => deferred = Some(publish),
            _ => unreachable!(),
        }
        let wake = Arc::new(ControlledHistoryPublicWake { state: Arc::downgrade(&state), wakes: std::sync::atomic::AtomicUsize::new(0), lock_released: std::sync::atomic::AtomicBool::new(true) });
        let waker = std::task::Waker::from(wake.clone());
        let mut context = std::task::Context::from_waker(&waker);
        let first = std::pin::Pin::new(&mut probe).poll(&mut context);
        let first_ready = first.is_ready();
        if let Some(publish) = deferred {
            publish();
        }
        let wake_count = wake.wakes.load(Ordering::Acquire);
        let wake_lock_released = wake.lock_released.load(Ordering::Acquire);
        let outcome = match first {
            std::task::Poll::Ready(result) => result,
            std::task::Poll::Pending => match std::pin::Pin::new(&mut probe).poll(&mut context) {
                std::task::Poll::Ready(result) => result,
                _ => panic!("physically published history result must remain consumable"),
            },
        };
        let waiter_empty = state.waker.lock().unwrap().is_none();
        let exact_result = match outcome {
            Ok(mut view) => {
                assert!(!cancelled);
                let exact = Some(view.entries().as_ptr() as usize) == entry_pointer
                    && view.admission.as_ref().is_some_and(|owner| owner.slot == admission_slot && owner.generation == generation)
                    && view.terminal_state.as_ref().is_some_and(|owner| Arc::ptr_eq(owner, &state))
                    && fixture["operations"].as_array().unwrap().iter().enumerate().all(|(index, operation)| view.operation_id_eq(index, 0, operation.as_str().unwrap()));
                for _ in 0..50_000 {
                    if view.terminal_is_empty() {
                        break;
                    }
                    assert!(view.close_step(), "actual replay result must make bounded retirement progress");
                }
                assert!(view.terminal_is_empty(), "history fixture must close every actual replay owner");
                exact
            }
            Err(error) => cancelled && error == DbError::Closed,
        };
        drop(probe);
        for _ in 0..50_000 {
            if terminal.terminal_is_empty() {
                break;
            }
            if !terminal.close_step() {
                std::thread::yield_now();
            }
        }
        let admission_released = !ARTIFACT_HISTORY_ADMISSION.lock().unwrap().slots[admission_slot].occupied;
        let registry_empty = handle.history_terminal(generation).is_none();
        assert!(terminal.terminal_is_empty(), "history fixture cleanup must finish its exact terminal state");
        eprintln!(
            "history-completion: {} first-ready={first_ready} wakes={wake_count} waiter-empty={waiter_empty} wake-lock-released={wake_lock_released} exact-result={exact_result} admission-released={admission_released} registry-empty={registry_empty}",
            row["name"]
        );
        observations.push((row.clone(), first_ready, wake_count, waiter_empty, exact_result, admission_released, registry_empty, wake_lock_released));
    }
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    for (row, first_ready, wakes, waiter_empty, exact_result, admission_released, registry_empty, wake_lock_released) in observations {
        assert_eq!(first_ready, row["firstReady"].as_bool().unwrap(), "history completion in registration gap must be observed in the same poll: {}", row["name"]);
        assert_eq!(wakes as u64, row["wakes"].as_u64().unwrap(), "{}", row["name"]);
        assert_eq!(waiter_empty, row["waiterEmpty"].as_bool().unwrap(), "{}", row["name"]);
        assert_eq!(wake_lock_released, row["wakeLockReleased"].as_bool().unwrap(), "wake callback must not execute under the waiter mutex: {}", row["name"]);
        assert_eq!(exact_result, row["exactResult"].as_bool().unwrap(), "{}", row["name"]);
        assert_eq!(admission_released, row["admissionReleased"].as_bool().unwrap(), "{}", row["name"]);
        assert_eq!(registry_empty, row["registryEmpty"].as_bool().unwrap(), "{}", row["name"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_public_terminal_close_releases_admission_only_after_roots_are_empty() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_public_terminal_close_releases_admission_only_after_roots_are_empty") {
        return;
    }
    let _history_capacity = db_artifact_history::history_capacity_test_lock();
    let root = tempdir("history-public-terminal-release").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("history-public-terminal-release".to_string());
    let handle = database.create_document(ArtifactSpec::new(document).await).await.unwrap();
    let mut peers = Vec::new();
    for _ in 1..ARTIFACT_HISTORY_OPERATION_SLOTS {
        peers.push(ArtifactHistoryAdmission::try_claim().unwrap());
    }
    let history = handle.history();
    let generation = history.generation();
    let terminal = history.terminal_handle();
    history.cancel();
    drop(history);
    assert!(matches!(ArtifactHistoryAdmission::try_claim(), Err(ArtifactHistoryAdmissionError::Rejected(DbError::Unavailable(_)))));
    assert!(!terminal.terminal_is_empty());
    let mut released = false;
    for _ in 0..100_000 {
        if terminal.terminal_is_empty() {
            break;
        }
        let roots_were_empty = terminal.state.terminal_roots_are_empty();
        let admission_was_retained = terminal.state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        let progressed = terminal.close_step();
        let admission_is_released = terminal.state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none();
        if admission_was_retained && admission_is_released {
            assert!(roots_were_empty, "the admission release must occupy a grant after every terminal root was already empty");
            assert!(progressed);
            assert!(terminal.state.finished.load(std::sync::atomic::Ordering::Acquire));
            released = true;
        }
        if !progressed {
            std::thread::yield_now();
        }
    }
    assert!(released);
    assert!(terminal.terminal_is_empty());
    assert!(handle.history_terminal(generation).is_none(), "terminal completion must unregister the released generation");
    let replacement = ArtifactHistoryAdmission::try_claim().unwrap();
    retire_history_admission(replacement);
    for peer in peers {
        retire_history_admission(peer);
    }
}

//#endregion 🔖️Retained submit authority
