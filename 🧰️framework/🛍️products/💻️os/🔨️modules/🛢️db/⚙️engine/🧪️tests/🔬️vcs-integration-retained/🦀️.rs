mod retained_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    struct CountWake(AtomicUsize);

    impl std::task::Wake for CountWake {
        fn wake(self: std::sync::Arc<Self>) {
            self.0.fetch_add(1, Ordering::AcqRel);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn vcs_store_keeps_exact_history_owners_through_changes_checkpoint_and_bounded_close() {
        let graph = VcsVersionGraph::new().await;
        let document = ArtifactId("vcs-owner-catalog".to_string());
        let mut edit_ids = Vec::new();
        for index in 0..9u8 {
            let change =
                ChangeRecord { parent: edit_ids.last().cloned(), content_hash: pack::ContentHash([index.checked_add(1).unwrap(); 32]), author: ActorId("owner".to_string()), message: format!("change-{index}"), timestamp_ms: u64::from(index) + 1 };
            let edit_id = graph.record_change(&document, change).await.expect("every retained history mutation keeps its exact retirement authority");
            assert!(!edit_id.is_empty());
            edit_ids.push(edit_id);
        }
        let checkpoint = graph
            .checkpoint(&document, CheckpointRequest { parent_checkpoint: None, change_ids: edit_ids, message: "checkpoint-after-nine".to_string(), authors: vec![ActorId("owner".to_string())], timestamp_ms: 10 })
            .await
            .expect("checkpoint keeps exact history retirement authority");
        assert!(!checkpoint.is_empty());

        let admission = VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).expect("close lease admission");
        let mut lease = graph.store(&document, &admission).await.expect("retained store lease");
        for _ in 0..4_096 {
            match store::SpaceMember::close_owned_step(lease.store_mut(), 1, VCS_OPERATION_BYTES as usize).expect("bounded VCS store close") {
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= VCS_OPERATION_BYTES as usize);
                }
                store::SnapshotRetirementStep::Blocked => panic!("VCS hash owners have no external close dependency"),
                store::SnapshotRetirementStep::Complete => break,
            }
        }
        assert!(store::SpaceMember::close_owned_terminal_is_empty(lease.store_mut()));
    }

    #[semio_framework_async_macros::async_test]
    async fn vcs_shutdown_error_reinstalls_exact_store_and_retry_reaches_terminal() {
        let graph = VcsVersionGraph::new().await;
        let document = ArtifactId("vcs-shutdown-retry".to_string());
        let change = ChangeRecord { parent: None, content_hash: pack::ContentHash([7; 32]), author: ActorId("owner".to_string()), message: "before-close".to_string(), timestamp_ms: 1 };
        graph.record_change(&document, change).await.expect("seed retained VCS store");
        let exact_cell = {
            let stores = graph.stores.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            std::sync::Arc::as_ptr(stores.get(&document.0).expect("seeded VCS cell"))
        };
        graph.fail_next_shutdown_step();
        assert!(matches!(graph.shutdown_step().await, Err(DbError::Internal(detail)) if detail == "injected retained VCS shutdown fault"));
        {
            let stores = graph.stores.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            assert_eq!(std::sync::Arc::as_ptr(stores.get(&document.0).expect("fault retains VCS cell")), exact_cell);
            assert!(stores.get(&document.0).expect("fault retains VCS cell").state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).store.is_some());
        }
        for _ in 0..4_096 {
            if graph.shutdown_step().await.expect("retry VCS shutdown") == VersionGraphShutdownStep::Complete {
                assert!(graph.stores.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
                return;
            }
        }
        panic!("retained VCS shutdown retry did not reach terminal");
    }

    #[test]
    fn vcs_retained_item_cap_plus_one_and_nested_bytes_plus_one_return_without_mutation() {
        let _guard = TEST_LOCK.lock().unwrap();
        let claims: Vec<VcsOperationAdmission> = (0..VCS_OPERATION_ITEMS).map(|_| VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).unwrap()).collect();
        assert!(VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).is_err());
        assert!(vcs_credit(VCS_OPERATION_ITEMS + 1, [0]).is_err());
        assert!(vcs_credit(1, [VCS_OPERATION_BYTES as usize]).is_err());
        drop(claims);
    }

    fn record_with_author_bytes(bytes: usize) -> ChangeRecord {
        let author = "a".repeat(bytes);
        assert_eq!(author.capacity(), bytes);
        ChangeRecord { parent: None, content_hash: pack::ContentHash([1; 32]), author: ActorId(author), message: String::new(), timestamp_ms: 1 }
    }

    fn checkpoint_with_author_bytes(bytes: usize) -> CheckpointRequest {
        let author = "a".repeat(bytes);
        assert_eq!(author.capacity(), bytes);
        let mut authors = Vec::with_capacity(1);
        authors.push(ActorId(author));
        CheckpointRequest { parent_checkpoint: None, change_ids: Vec::new(), message: String::new(), authors, timestamp_ms: 1 }
    }

    #[test]
    fn vcs_record_derived_owner_credit_cap_plus_one_preserves_exact_input() {
        let document = ArtifactId(String::new());
        let author_bytes = VCS_OPERATION_BYTES as usize - VCS_OPERATION_PAGE_BYTES as usize - size_of::<HashMutation>();
        let accepted = record_with_author_bytes(author_bytes);
        assert_eq!(record_credit(&document, &accepted).unwrap(), (1, VCS_OPERATION_BYTES));
        let rejected = record_with_author_bytes(author_bytes + 1);
        let author_owner = rejected.author.0.as_ptr();
        assert!(record_credit(&document, &rejected).is_err());
        assert_eq!(rejected.author.0.as_ptr(), author_owner);
        assert_eq!(rejected.author.0.len(), author_bytes + 1);
    }

    #[test]
    fn vcs_checkpoint_derived_owner_credit_cap_plus_one_preserves_exact_input() {
        let document = ArtifactId(String::new());
        let fixed = VCS_OPERATION_PAGE_BYTES as usize + size_of::<ActorId>() + size_of::<vcs::Author>();
        let author_bytes = (VCS_OPERATION_BYTES as usize - fixed) / 2;
        let accepted = checkpoint_with_author_bytes(author_bytes);
        assert_eq!(checkpoint_credit(&document, &accepted).unwrap().1, VCS_OPERATION_BYTES);
        let rejected = checkpoint_with_author_bytes(author_bytes + 1);
        let author_owner = rejected.authors[0].0.as_ptr();
        assert!(checkpoint_credit(&document, &rejected).is_err());
        assert_eq!(rejected.authors[0].0.as_ptr(), author_owner);
        assert_eq!(rejected.authors[0].0.len(), author_bytes + 1);
    }

    #[test]
    fn vcs_checkpoint_derived_item_boundary_admits_31_rejects_32_and_preserves_exact_owners() {
        let document = ArtifactId(String::new());
        let request = |count: usize| CheckpointRequest { parent_checkpoint: None, change_ids: Vec::new(), message: String::new(), authors: (0..count).map(|index| ActorId(format!("author-{index}"))).collect(), timestamp_ms: 1 };
        let accepted = request(31);
        let accepted_authors = accepted.authors.as_ptr();
        let accepted_first = accepted.authors[0].0.as_ptr();
        let (items, bytes) = checkpoint_credit(&document, &accepted).unwrap();
        assert_eq!(items, 63);
        let admission = VcsOperationAdmission::try_claim(items, bytes).unwrap();
        assert_eq!(accepted.authors.as_ptr(), accepted_authors);
        assert_eq!(accepted.authors[0].0.as_ptr(), accepted_first);
        drop(admission);

        let rejected = request(32);
        let rejected_authors = rejected.authors.as_ptr();
        let rejected_first = rejected.authors[0].0.as_ptr();
        assert_eq!(1 + rejected.authors.len(), 33, "source-only formula would falsely admit");
        assert_eq!(1 + rejected.authors.len() * 2, 65, "materialized name and id owners exceed the cap");
        assert!(checkpoint_credit(&document, &rejected).is_err());
        assert_eq!(rejected.authors.as_ptr(), rejected_authors);
        assert_eq!(rejected.authors[0].0.as_ptr(), rejected_first);
        assert_eq!(rejected.authors.len(), 32);
    }

    #[test]
    fn vcs_derived_owner_process_aggregate_plus_one_rejects_without_consuming_input() {
        let _guard = TEST_LOCK.lock().unwrap();
        let document = ArtifactId(String::new());
        let author_bytes = VCS_OPERATION_BYTES as usize - VCS_OPERATION_PAGE_BYTES as usize - size_of::<HashMutation>();
        let accepted = record_with_author_bytes(author_bytes);
        let (items, bytes) = record_credit(&document, &accepted).unwrap();
        let claims: Vec<VcsOperationAdmission> = (0..VCS_OPERATION_ITEMS).map(|_| VcsOperationAdmission::try_claim(items, bytes).unwrap()).collect();
        assert_eq!(VCS_ADMISSION.lock().unwrap().bytes, VCS_TOTAL_BYTES);
        let rejected = record_with_author_bytes(author_bytes);
        let author_owner = rejected.author.0.as_ptr();
        assert!(record_credit(&document, &rejected).and_then(|(items, bytes)| VcsOperationAdmission::try_claim(items, bytes)).is_err());
        assert_eq!(rejected.author.0.as_ptr(), author_owner);
        assert_eq!(VCS_ADMISSION.lock().unwrap().bytes, VCS_TOTAL_BYTES);
        drop(claims);

        let checkpoint = checkpoint_with_author_bytes((VCS_OPERATION_BYTES as usize - VCS_OPERATION_PAGE_BYTES as usize - size_of::<ActorId>() - size_of::<vcs::Author>()) / 2);
        let (items, bytes) = checkpoint_credit(&document, &checkpoint).unwrap();
        let claims: Vec<VcsOperationAdmission> = (0..VCS_OPERATION_ITEMS).map(|_| VcsOperationAdmission::try_claim(items, bytes).unwrap()).collect();
        assert_eq!(VCS_ADMISSION.lock().unwrap().bytes, VCS_TOTAL_BYTES);
        let rejected = checkpoint_with_author_bytes((VCS_OPERATION_BYTES as usize - VCS_OPERATION_PAGE_BYTES as usize - size_of::<ActorId>() - size_of::<vcs::Author>()) / 2);
        let author_owner = rejected.authors[0].0.as_ptr();
        assert!(checkpoint_credit(&document, &rejected).and_then(|(items, bytes)| VcsOperationAdmission::try_claim(items, bytes)).is_err());
        assert_eq!(rejected.authors[0].0.as_ptr(), author_owner);
        assert_eq!(VCS_ADMISSION.lock().unwrap().bytes, VCS_TOTAL_BYTES);
        drop(claims);
    }

    #[test]
    fn vcs_retained_pending_wake_is_fifo_one_shot_and_quiet_without_release() {
        let _guard = TEST_LOCK.lock().unwrap();
        let owner = VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).unwrap();
        let second = VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).unwrap();
        let third = VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).unwrap();
        let cell = std::sync::Arc::new(VcsStoreCell::new());
        cell.state.lock().unwrap().busy_generation = Some(owner.generation);
        let second_wake = std::sync::Arc::new(CountWake(AtomicUsize::new(0)));
        let third_wake = std::sync::Arc::new(CountWake(AtomicUsize::new(0)));
        let second_waker = Waker::from(second_wake.clone());
        let third_waker = Waker::from(third_wake.clone());
        let mut second_context = Context::from_waker(&second_waker);
        let mut third_context = Context::from_waker(&third_waker);
        let mut second_acquire = VcsStoreAcquire { cell: cell.clone(), slot: second.slot, generation: second.generation, resolved: false };
        let mut third_acquire = VcsStoreAcquire { cell: cell.clone(), slot: third.slot, generation: third.generation, resolved: false };
        assert!(Pin::new(&mut second_acquire).poll(&mut second_context).is_pending());
        assert!(Pin::new(&mut third_acquire).poll(&mut third_context).is_pending());
        assert_eq!(second_wake.0.load(Ordering::Acquire), 0);
        assert_eq!(third_wake.0.load(Ordering::Acquire), 0);
        cell.release(owner.generation, None);
        assert_eq!(second_wake.0.load(Ordering::Acquire), 1);
        assert_eq!(third_wake.0.load(Ordering::Acquire), 0);
        let late = VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).unwrap();
        let late_wake = std::sync::Arc::new(CountWake(AtomicUsize::new(0)));
        let late_waker = Waker::from(late_wake.clone());
        let mut late_context = Context::from_waker(&late_waker);
        let mut late_acquire = VcsStoreAcquire { cell: cell.clone(), slot: late.slot, generation: late.generation, resolved: false };
        assert!(Pin::new(&mut late_acquire).poll(&mut late_context).is_pending());
        let Poll::Ready(Ok(VcsStoreClaim::Build(permit))) = Pin::new(&mut second_acquire).poll(&mut second_context) else {
            panic!("second retained owner must acquire first");
        };
        drop(permit);
        assert_eq!(third_wake.0.load(Ordering::Acquire), 1);
        assert_eq!(late_wake.0.load(Ordering::Acquire), 0);
    }

    #[test]
    fn vcs_retained_cancel_clears_waiter_and_slot_aba_stays_stale() {
        let _guard = TEST_LOCK.lock().unwrap();
        let owner = VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).unwrap();
        let waiter = VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).unwrap();
        let cell = std::sync::Arc::new(VcsStoreCell::new());
        cell.state.lock().unwrap().busy_generation = Some(owner.generation);
        let waker = Waker::from(std::sync::Arc::new(CountWake(AtomicUsize::new(0))));
        let mut context = Context::from_waker(&waker);
        let mut acquire = VcsStoreAcquire { cell: cell.clone(), slot: waiter.slot, generation: waiter.generation, resolved: false };
        assert!(Pin::new(&mut acquire).poll(&mut context).is_pending());
        drop(acquire);
        assert!(cell.state.lock().unwrap().waiters[waiter.slot].is_none());
        let slot = waiter.slot;
        let generation = waiter.generation;
        drop(waiter);
        let replacement = VcsOperationAdmission::try_claim(1, VCS_OPERATION_PAGE_BYTES).unwrap();
        assert_eq!(replacement.slot, slot);
        assert_ne!(replacement.generation, generation);
        assert!(!VcsOperationAdmission::is_current(slot, generation));
        cell.release(owner.generation, None);
    }

    #[test]
    fn vcs_retained_live_source_has_no_nested_executor_or_guarded_await() {
        let source = include_str!("../../🦀️.rs");
        let vcs = &source[source.find("pub mod vcs_integration").unwrap()..source.find("//#endregion 🔖️VersionGraph").unwrap()];
        let production = &vcs[..vcs.find("#[cfg(test)]\n    mod retained_tests").unwrap()];
        assert!(!production.contains("block_on("));
        assert!(!production.contains("submit_blocking"));
        assert!(!production.contains("ask_blocking"));
        assert!(!production.contains("loop {"));
        assert!(production.contains("VcsStoreAcquire"));
        assert!(production.contains("VcsStoreLease"));
        assert!(production.contains("std::mem::size_of::<HashMutation>()"));
        assert!(production.contains("let derived_author_items = request.authors.len();"));
        assert!(production.contains(".and_then(|value| value.checked_add(derived_author_items))"));
        assert!(production.contains("derived_author_owner_bytes"));
        assert!(production.contains("derived_author_id_bytes"));
        assert!(production.contains("let mutations = Vec::from([operation]);"));
        assert!(!production.contains("change.author.0.clone()"));
        assert!(!production.contains("mutations: vec![operation]"));
    }
}
