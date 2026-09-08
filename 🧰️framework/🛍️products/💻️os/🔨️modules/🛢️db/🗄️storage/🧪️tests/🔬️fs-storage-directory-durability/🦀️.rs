mod directory_durability_tests {
    use super::*;

    fn lifecycle<T>(storage: &FsStorage, action: impl FnOnce(&mut FsLifecycleState) -> T) -> T {
        let mut registry = super::super::lock(super::super::db_io_backend_registry());
        let slot = usize::from(super::super::db_io_backend_parts(storage.control).0);
        let executor = registry.slots[slot].executor.as_mut().unwrap().as_any_mut().downcast_mut::<FsDbIoExecutor>().unwrap();
        let result = action(&mut executor.lifecycle.state.lock().unwrap());
        result
    }

    fn phase_name(phase: FsLifecyclePhase) -> &'static str {
        match phase {
            FsLifecyclePhase::DirectoryCreated => "directory-created",
            FsLifecyclePhase::DirectoryParentSynced => "directory-parent-synced",
            FsLifecyclePhase::SegmentCreated => "segment-created",
            FsLifecyclePhase::SegmentFileSynced => "segment-file-synced",
            FsLifecyclePhase::SegmentParentSynced => "segment-parent-synced",
            FsLifecyclePhase::MarkerCreated => "marker-created",
            FsLifecyclePhase::MarkerFileSynced => "marker-file-synced",
            FsLifecyclePhase::MarkerParentSynced => "marker-parent-synced",
            FsLifecyclePhase::SegmentDeleted => "segment-deleted",
            FsLifecyclePhase::SegmentDeleteParentSynced => "segment-delete-parent-synced",
            FsLifecyclePhase::MarkerDeleted => "marker-deleted",
            FsLifecyclePhase::MarkerDeleteParentSynced => "marker-delete-parent-synced",
            FsLifecyclePhase::ReplacementRenamed => "replacement-renamed",
            FsLifecyclePhase::ReplacementParentSynced => "replacement-parent-synced",
            FsLifecyclePhase::WalFileSynced => "wal-file-synced",
            FsLifecyclePhase::WalParentSynced => "wal-parent-synced",
        }
    }

    fn take_trace(storage: &FsStorage) -> Vec<&'static str> {
        lifecycle(storage, |state| {
            let trace = state.trace[..state.count].iter().flatten().copied().filter(|phase| !matches!(phase, FsLifecyclePhase::DirectoryCreated | FsLifecyclePhase::DirectoryParentSynced)).map(phase_name).collect();
            *state = FsLifecycleState::default();
            trace
        })
    }

    async fn storage(name: &str) -> (FsStorage, PathBuf, serde_json::Value) {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
        let path = base.join(format!("directory-durability-{name}-{}-{}", std::process::id(), NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed))).join("nested/root");
        let storage = FsStorage::open(super::super::db_io_test_pool(), &path).await.unwrap();
        let fixture = serde_json::from_str(include_str!("../../🧫️fixtures/📁️directory-durability/🔣️.json")).unwrap();
        (storage, path, fixture)
    }

    #[semio_framework_async_macros::async_test]
    async fn fs_wal_directory_barriers_match_neutral_order_and_duplicate_create_is_atomic() {
        let (storage, root, fixture) = storage("ordered").await;
        lifecycle(&storage, |state| assert!(state.trace[..state.count].iter().filter(|phase| **phase == Some(FsLifecyclePhase::DirectoryCreated)).count() >= 3));
        take_trace(&storage);
        let document: ArtifactId = "durable-names".into();
        let writer = storage.acquire_writer(&document).await.unwrap();
        storage.create_segment(&writer, 0).await.unwrap();
        assert_eq!(serde_json::json!(take_trace(&storage)), fixture["create"]);
        let bytes = super::super::db_io_copy_pages(b"original bytes").unwrap().await.unwrap();
        storage.append(&writer, 0, bytes).await.unwrap();
        storage.sync(&writer, 0, DurabilityClass::Fsync).await.unwrap();
        assert!(matches!(storage.create_segment(&writer, 0).await, Err(DbError::AlreadyExists(_))));
        assert_eq!(std::fs::read(root.join("wal/durable-names/segment-00000000000000000000.bin")).unwrap() == b"original bytes", fixture["duplicatePreservesBytes"].as_bool().unwrap());
        take_trace(&storage);
        storage.seal(&writer, 0).await.unwrap();
        assert_eq!(serde_json::json!(take_trace(&storage)), fixture["seal"]);
        storage.delete_segment(&writer, 0).await.unwrap();
        assert_eq!(serde_json::json!(take_trace(&storage)), fixture["delete"]);
        writer.release().await.unwrap();
        storage.close().await.unwrap();
        eprintln!("[DEBUG] mounted filesystem WAL created nested names, fsynced file and parent in neutral order, rejected duplicate creation without truncation, and retired segment before marker");
    }

    #[semio_framework_async_macros::async_test]
    async fn fs_wal_directory_faults_retain_seal_and_delete_order_until_explicit_retry() {
        let (storage, root, fixture) = storage("faulted").await;
        let document: ArtifactId = "faulted-names".into();
        let writer = storage.acquire_writer(&document).await.unwrap();
        storage.create_segment(&writer, 0).await.unwrap();
        lifecycle(&storage, |state| {
            *state = FsLifecycleState::default();
            state.fail = Some(FsLifecyclePhase::MarkerCreated);
        });
        assert!(matches!(storage.seal(&writer, 0).await, Err(DbError::Io(_))));
        assert_eq!(storage.segment_state(&document, 0).await.unwrap(), WalSegmentState::Sealed);
        storage.seal(&writer, 0).await.unwrap();
        assert_eq!(take_trace(&storage), ["marker-created", "marker-file-synced", "marker-parent-synced"]);
        lifecycle(&storage, |state| state.fail = Some(FsLifecyclePhase::SegmentDeleteParentSynced));
        assert!(matches!(storage.delete_segment(&writer, 0).await, Err(DbError::Io(_))));
        let marker = root.join("wal/faulted-names/segment-00000000000000000000.sealed");
        assert!(marker.is_file());
        assert!(matches!(storage.segment_state(&document, 0).await, Err(DbError::NotFound(_))));
        assert_eq!(fixture["deleteFaultState"], "not-found-with-retained-marker");
        storage.delete_segment(&writer, 0).await.unwrap();
        assert!(!marker.exists());
        writer.release().await.unwrap();
        storage.close().await.unwrap();
        eprintln!("[DEBUG] seal fault retained a sealed marker through retry; deletion fault after parent fsync retained only the marker and never resurrected an active segment");
    }

    #[semio_framework_async_macros::async_test]
    async fn fs_replacement_reports_failure_until_renamed_parent_is_synced() {
        let (storage, _, fixture) = storage("replace").await;
        let document: ArtifactId = "snapshot-names".into();
        lifecycle(&storage, |state| {
            *state = FsLifecycleState::default();
            state.fail = Some(FsLifecyclePhase::ReplacementRenamed);
        });
        let pages = super::super::db_io_copy_pages(b"uncertain snapshot").unwrap().await.unwrap();
        assert!(matches!(storage.write_generation(&document, 0, pages).await, Err(DbError::Io(_))));
        assert_eq!(take_trace(&storage), ["replacement-renamed"]);
        let pages = super::super::db_io_copy_pages(b"confirmed snapshot").unwrap().await.unwrap();
        storage.write_generation(&document, 0, pages).await.unwrap();
        assert_eq!(serde_json::json!(take_trace(&storage)), fixture["replace"]);
        let mut pages = storage.read_generation(&document, 0).await.unwrap();
        assert_eq!(pages, b"confirmed snapshot");
        while pages.close_step().unwrap().is_some() {
            semio_framework_async::yield_once().await;
        }
        storage.close().await.unwrap();
        eprintln!("[DEBUG] mounted snapshot replacement did not acknowledge an injected post-rename failure; explicit replacement completed only after parent fsync");
    }

    #[semio_framework_async_macros::async_test]
    async fn fs_wal_reopen_repairs_unacknowledged_segment_namespace_before_header_ack() {
        for (phase, missing) in [(FsLifecyclePhase::SegmentCreated, false), (FsLifecyclePhase::SegmentFileSynced, false), (FsLifecyclePhase::SegmentCreated, true)] {
            let (storage, root, fixture) = storage("recovery").await;
            let document: ArtifactId = "recovery-names".into();
            let writer = storage.acquire_writer(&document).await.unwrap();
            lifecycle(&storage, |state| {
                *state = FsLifecycleState::default();
                state.fail = Some(phase);
            });
            assert!(matches!(storage.create_segment(&writer, 0).await, Err(DbError::Io(_))));
            writer.release().await.unwrap();
            storage.close().await.unwrap();
            if missing {
                std::fs::remove_file(root.join("wal/recovery-names/segment-00000000000000000000.bin")).unwrap();
            }
            let reopened = FsStorage::open(super::super::db_io_test_pool(), &root).await.unwrap();
            take_trace(&reopened);
            let (mut wal, _) = crate::db_wal::ArtifactWal::open(&reopened, document, crate::db_wal::GroupCommitPolicy::default(), 0).await.unwrap();
            let trace = take_trace(&reopened);
            let recovery: Vec<_> = trace.iter().copied().filter(|step| step.starts_with("wal-")).collect();
            assert_eq!(serde_json::json!(recovery), fixture["recoverySync"]);
            if missing {
                assert_eq!(serde_json::json!(&trace[..3]), fixture["create"]);
            }
            wal.close().await.unwrap();
            reopened.close().await.unwrap();
        }
        eprintln!("[DEBUG] independent filesystem reopen repaired the parent name after both pre-barrier create failures and durably recreated a missing unacknowledged segment before header acknowledgment");
    }
}
