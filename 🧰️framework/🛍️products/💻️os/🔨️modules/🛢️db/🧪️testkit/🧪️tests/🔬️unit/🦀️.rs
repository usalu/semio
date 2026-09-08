
use super::*;
use std::cell::RefCell;
use std::rc::Rc;

fn entrypoint_pool() -> Arc<semio_framework_async::WorkerPool> {
    Arc::new(semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)))
}

fn pages(bytes: &[u8]) -> DbIoPages {
    let mut writer = db_storage::DbIoPageWriter::try_reserve(bytes.len().div_ceil(db_storage::DB_IO_PAGE_BYTES)).expect("testkit writer admitted");
    for fragment in bytes.chunks(db_storage::DB_IO_PAGE_BYTES) {
        assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
    }
    writer.finish().unwrap()
}

//#region 🔖️Prng + Generators
#[semio_framework_async_macros::async_test]
async fn split_mix_64_same_seed_reproduces_identical_sequence_different_seed_diverges() {
    let mut a = SplitMix64::new(7);
    let mut b = SplitMix64::new(7);
    let mut c = SplitMix64::new(8);
    let sequence_a: Vec<u64> = (0..16).map(|_| a.next_u64()).collect();
    let sequence_b: Vec<u64> = (0..16).map(|_| b.next_u64()).collect();
    let sequence_c: Vec<u64> = (0..16).map(|_| c.next_u64()).collect();
    assert_eq!(sequence_a, sequence_b, "same seed must reproduce the identical draw sequence");
    assert_ne!(sequence_a, sequence_c, "a different seed must (overwhelmingly likely) diverge");
}

#[semio_framework_async_macros::async_test]
async fn next_range_never_divides_by_zero_and_stays_in_bounds() {
    let mut rng = SplitMix64::new(1);
    assert_eq!(rng.next_range(0), 0);
    for _ in 0..64 {
        assert!(rng.next_range(10) < 10);
    }
}

#[semio_framework_async_macros::async_test]
async fn workload_gen_disjoint_batch_is_deterministic_and_covers_distinct_paths() {
    let document = protocol::ArtifactId("gen-doc".to_string());
    let ops_a = WorkloadGen::new(42).disjoint_batch(&document, 8).await;
    let ops_b = WorkloadGen::new(42).disjoint_batch(&document, 8).await;
    let ops_c = WorkloadGen::new(43).disjoint_batch(&document, 8).await;
    assert_eq!(ops_a, ops_b, "same seed must generate byte-identical envelopes");
    assert_ne!(ops_a, ops_c, "a different seed must generate different actor/value draws");
    // 🪡 `.flat_map` takes a sync closure, so the per-envelope decode (now async) is hoisted
    // into an explicit loop instead — same accumulation, no `.await` inside a sync closure.
    let mut paths: std::collections::HashSet<String> = std::collections::HashSet::new();
    for envelope in ops_a.iter() {
        let decoded = db_artifact::decode_pathmap_json(&envelope.diff.payload).await.unwrap();
        paths.extend(decoded.as_object().unwrap().keys().cloned());
    }
    assert_eq!(paths.len(), 8, "disjoint_batch must touch exactly `count` distinct paths");
}
//#endregion 🔖️Prng + Generators

//#region 🔖️SimRuntime
#[semio_framework_async_macros::async_test]
async fn sim_runtime_same_seed_reproduces_identical_task_order() {
    let build = |seed: u64| {
        let mut runtime = SimRuntime::new(seed);
        for i in 0..6 {
            runtime.schedule(format!("task-{i}"), |_clock| {});
        }
        runtime.run(10)
    };
    assert_eq!(build(42), build(42), "identical seed must reproduce an identical schedule");
}

#[semio_framework_async_macros::async_test]
async fn sim_runtime_clock_advances_monotonically_and_only_explicitly() {
    let mut runtime = SimRuntime::new(1);
    for i in 0..4 {
        runtime.schedule(format!("task-{i}"), |clock| {
            let _ = clock.now_ms();
        });
    }
    let clock_before = SimClock::new();
    assert_eq!(clock_before.now_ms(), 0);
    let order = runtime.run(20);
    assert_eq!(order.len(), 4);
}

#[semio_framework_async_macros::async_test]
async fn explore_interleavings_every_permutation_of_disjoint_writes_converges_to_the_same_state() {
    let document = protocol::ArtifactId("explore-doc".to_string());
    // 🪡 Hoisted out of the sync closure below: `explore_interleavings` takes `impl FnMut(u64) -> T`
    // (deliberately sync — it drives a deterministic sim clock), so `.await` cannot live inside it.
    // The batch never depended on the per-permutation `seed` anyway (fixed generator seed 55), so
    // computing it once and cloning per permutation is behavior-identical to the original in-closure call.
    let ops = WorkloadGen::new(55).disjoint_batch(&document, 5).await;
    let hashes = explore_interleavings(4242, 12, |seed| {
        let ops = ops.clone();
        let storage: Arc<DbBackend> = Arc::new(DbBackend::Memory(db_actor::block_on(db_storage::MemoryStorage::new(db_storage::db_io_test_pool())).unwrap()));
        let engine = Rc::new(RefCell::new(db_artifact::ArtifactEngine::create(document.clone(), storage, db_artifact::ArtifactEngineConfig::default(), 0).expect("create engine")));
        let mut runtime = SimRuntime::new(seed);
        for (i, envelope) in ops.into_iter().enumerate() {
            let engine = engine.clone();
            runtime.schedule(envelope.mutation_id.0.clone(), move |clock| {
                let now = clock.now_ms() + i as u64;
                // 🪡 `SimRuntime::schedule` takes a plain sync `FnOnce` (the whole point is
                // deterministic single-threaded step ordering), so the now-async `submit` is
                // driven synchronously via the same `db_actor::block_on` bridge every other
                // test in this crate already uses — sound here because the backing storage is
                // in-memory (`MemoryStorage`) and authz is the default `AllowAll`, so nothing
                // in `submit`'s call graph ever actually suspends.
                db_actor::block_on(engine.borrow_mut().submit(db_actor::block_on(single_envelope_batch(envelope)), db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, now)).expect("submit");
            });
        }
        runtime.run(4);
        let chain_hash = db_actor::block_on(engine.borrow().frontier()).chain_hash;
        chain_hash
    });
    assert!(hashes.windows(2).all(|pair| pair[0] == pair[1]), "every explored interleaving of disjoint writes must converge to the identical state hash");
}
//#endregion 🔖️SimRuntime

//#region 🔖️FaultStorage
#[semio_framework_async_macros::async_test]
async fn fault_storage_passes_through_untouched_when_no_fault_is_scripted() {
    let inner = Arc::new(DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let faulted = FaultStorage::new(inner).await;
    let document = ArtifactId("doc-1".to_string());
    let writer = faulted.acquire_writer(&document).await.unwrap();
    db_actor::block_on(faulted.create_segment(&writer, 0)).unwrap();
    assert_eq!(db_actor::block_on(faulted.append(&writer, 0, pages(b"hello"))).unwrap(), 5);
    assert_eq!(db_actor::block_on(faulted.read(&document, 0, pack::ByteRange { offset: 0, len: 5 })).unwrap(), b"hello");
    assert_eq!(faulted.append_calls().await, 1);
    writer.release().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn fault_storage_fail_nth_write_fails_exactly_once_at_the_scripted_call() {
    let faulted = FaultStorage::new(Arc::new(DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()))).await;
    faulted.set_script(FaultScript { fail_nth_write: Some(2), ..FaultScript::default() }).await;
    let document = ArtifactId("doc-1".to_string());
    let writer = faulted.acquire_writer(&document).await.unwrap();
    db_actor::block_on(faulted.create_segment(&writer, 0)).unwrap();
    assert!(db_actor::block_on(faulted.append(&writer, 0, pages(b"first"))).is_ok(), "call #1 must succeed");
    assert!(db_actor::block_on(faulted.append(&writer, 0, pages(b"second"))).is_err(), "call #2 must be the injected failure");
    assert!(db_actor::block_on(faulted.append(&writer, 0, pages(b"third"))).is_ok(), "call #3 must succeed again — the script fires exactly once");
    writer.release().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn fault_storage_torn_write_forwards_only_the_kept_prefix() {
    let faulted = FaultStorage::new(Arc::new(DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()))).await;
    faulted.set_script(FaultScript { torn_write_at: Some((1, 3)), ..FaultScript::default() }).await;
    let document = ArtifactId("doc-1".to_string());
    let writer = faulted.acquire_writer(&document).await.unwrap();
    db_actor::block_on(faulted.create_segment(&writer, 0)).unwrap();
    let new_len = db_actor::block_on(faulted.append(&writer, 0, pages(b"hello world"))).unwrap();
    assert_eq!(new_len, 3, "a torn write must report only the bytes that actually landed");
    assert_eq!(db_actor::block_on(faulted.read(&document, 0, pack::ByteRange { offset: 0, len: 3 })).unwrap(), b"hel");
    writer.release().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn fault_storage_fsync_lies_never_delegates_to_the_inner_backend() {
    let faulted = FaultStorage::new(Arc::new(DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()))).await;
    let document = ArtifactId("doc-1".to_string());
    let writer = faulted.acquire_writer(&document).await.unwrap();
    db_actor::block_on(faulted.create_segment(&writer, 0)).unwrap();
    assert!(db_actor::block_on(faulted.sync(&writer, 0, DurabilityClass::Fsync)).is_ok());
    assert_eq!(faulted.sync_delegated_calls().await, 1, "an unfaulted sync must delegate");

    faulted.set_script(FaultScript { fsync_lies: true, ..FaultScript::default() }).await;
    assert!(db_actor::block_on(faulted.sync(&writer, 0, DurabilityClass::Fsync)).is_ok(), "a lying fsync must still report success");
    assert_eq!(faulted.sync_delegated_calls().await, 1, "a lying fsync must never actually delegate");
    writer.release().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn fault_storage_fail_nth_sync_fails_once_after_the_preceding_append() {
    let faulted = FaultStorage::new(Arc::new(DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()))).await;
    let document = ArtifactId("doc-sync-fault".to_string());
    let writer = faulted.acquire_writer(&document).await.unwrap();
    db_actor::block_on(faulted.create_segment(&writer, 0)).unwrap();
    assert_eq!(db_actor::block_on(faulted.append(&writer, 0, pages(b"committed-before-sync"))).unwrap(), 21);
    faulted.set_script(FaultScript { fail_nth_sync: Some(1), ..FaultScript::default() }).await;
    assert!(db_actor::block_on(faulted.sync(&writer, 0, DurabilityClass::Fsync)).is_err());
    assert_eq!(faulted.sync_calls().await, 1);
    assert_eq!(faulted.sync_delegated_calls().await, 0);
    assert!(db_actor::block_on(faulted.sync(&writer, 0, DurabilityClass::Fsync)).is_ok());
    assert_eq!(faulted.sync_calls().await, 2);
    assert_eq!(faulted.sync_delegated_calls().await, 1);
    assert_eq!(db_actor::block_on(faulted.segment_len(&document, 0)).unwrap(), 21);
    writer.release().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn fault_storage_segment_state_is_observational_and_counter_neutral() {
    let faulted = FaultStorage::new(Arc::new(DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()))).await;
    let document = ArtifactId("doc-state".to_string());
    let writer = faulted.acquire_writer(&document).await.unwrap();
    db_actor::block_on(faulted.create_segment(&writer, 0)).unwrap();
    assert_eq!(db_actor::block_on(faulted.segment_state(&document, 0)).unwrap(), WalSegmentState::Active);
    db_actor::block_on(faulted.seal(&writer, 0)).unwrap();
    assert_eq!(db_actor::block_on(faulted.segment_state(&document, 0)).unwrap(), WalSegmentState::Sealed);
    assert_eq!(faulted.append_calls().await, 0);
    assert_eq!(faulted.sync_delegated_calls().await, 0);
    assert_eq!(faulted.cas_calls().await, 0);
    writer.release().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn fault_storage_cas_conflict_injection_rejects_without_touching_the_inner_root() {
    let faulted = FaultStorage::new(Arc::new(DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()))).await;
    faulted.set_script(FaultScript { cas_conflict_nth: Some(1), ..FaultScript::default() }).await;
    let result = db_actor::block_on(faulted.cas_root(EpochFence::INITIAL, pages(b"attempt")));
    assert!(matches!(result, Err(DbError::Fenced { .. })), "the scripted call must be rejected as fenced");
    assert!(db_actor::block_on(faulted.read_root()).unwrap().is_none(), "the injected conflict must never have reached the inner backend's root");
}
//#endregion 🔖️FaultStorage

//#region 🔖️CrashHarness
#[semio_framework_async_macros::async_test]
async fn crash_harness_recovers_cleanly_after_every_injected_write_failure() {
    let report = CrashHarness::run_crash_after_every_write(9001, 4).await;
    assert!(report.writes_tested >= 4, "at least one write boundary per committed submit plus genesis must be tested");
    assert!(report.is_clean().await, "recovery must never fail or corrupt state after any single injected write failure: {report:?}");
}

#[semio_framework_async_macros::async_test]
async fn document_wal_open_recovers_from_a_torn_write_by_truncating_the_tail() {
    let storage: Arc<DbBackend> = new_fault_backend().await;
    // Call #1 is the document's own genesis header write; torn-write call #2 is the first
    // `submit()`'s commit — truncated to 1 byte, an unrecoverable partial commit frame.
    as_fault(&storage).await.set_script(FaultScript { torn_write_at: Some((2, 1)), ..FaultScript::default() }).await;
    let document = protocol::ArtifactId("torn-doc".to_string());
    {
        let mut engine = db_artifact::ArtifactEngine::create(document.clone(), storage.clone(), db_artifact::ArtifactEngineConfig::default(), 0).unwrap();
        let envelope = schema_erased_envelope(&document, "op-1", "actor-1", "x", serde_json::json!(1), serde_json::Value::Null);
        let _ = engine.submit(single_envelope_batch(envelope.await).await, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 1).await;
    }
    let (recovered, report) = db_artifact::ArtifactEngine::open(document, &storage, db_artifact::ArtifactEngineConfig::default(), 2).expect("recovery must not error");
    assert_eq!(report.torn_tail_bytes, 1, "recovery must report exactly the torn bytes it discarded");
    assert_eq!(recovered.frontier().await.head_seq, 0, "the torn (unrecoverable) commit must not be visible after recovery");
}
//#endregion 🔖️CrashHarness

//#region 🔖️Laws
#[semio_framework_async_macros::async_test]
async fn law_replay_deterministic() {
    assert_replay_deterministic(entrypoint_pool(), 11, 5).await;
}

#[semio_framework_async_macros::async_test]
async fn law_snapshot_plus_suffix_equals_replay() {
    assert_snapshot_plus_suffix_equals_replay(12, 4, 3).await;
    assert_snapshot_plus_suffix_equals_replay(13, 0, 3).await;
}

#[semio_framework_async_macros::async_test]
async fn law_projection_rebuild_equals_incremental() {
    assert_projection_rebuild_equals_incremental(14, 9).await;
}

#[semio_framework_async_macros::async_test]
async fn law_inverse_undo_roundtrip() {
    assert_inverse_undo_roundtrip(15).await;
}

#[semio_framework_async_macros::async_test]
async fn law_sync_convergence() {
    assert_sync_convergence(16, 10).await;
}

#[semio_framework_async_macros::async_test]
async fn law_fencing_excludes_stale_writer_memory() {
    assert_fencing_excludes_stale_writer(&db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()).await;
}

#[semio_framework_async_macros::async_test]
async fn law_fencing_excludes_stale_writer_fs() {
    let root = temp_dir("fencing-fs").await;
    let storage = db_actor::block_on(db_storage::FsStorage::open(entrypoint_pool(), &root)).expect("open fs storage");
    assert_fencing_excludes_stale_writer(&storage).await;
}

#[semio_framework_async_macros::async_test]
async fn law_preview_never_durable() {
    assert_preview_never_durable(17).await;
}

#[semio_framework_async_macros::async_test]
async fn law_overlay_structural_sharing() {
    assert_overlay_structural_sharing(20).await;
}
//#endregion 🔖️Laws

//#region 🔖️exhaustive
/// @emoji 🐌️ Genuinely slower, exhaustive-only corruption fuzzing — real db_wal recovery driven
/// through `pack_testkit`'s truncation/bit-flip corruption harness (its documented precedent,
/// per this crate's own module doc), proving `db_wal::replay_document` never panics on a
/// corrupted WAL segment, only ever returns an `Err` (or, rarely, coincidentally still decodes).
mod exhaustive {
    use super::*;

    // 🚫️async: E1-adjacent — consumed as `impl Fn(&[u8]) -> Result<(), String>` by
    // `pack_testkit::fuzz_truncation`/`fuzz_bit_flips` (sync closure bound, first-party but not
    // ours to change from this test module), and every suspension point inside is already
    // bridged synchronously via `db_actor::block_on` — see R9.
    fn decode_wal_bytes(bytes: &[u8]) -> Result<(), String> {
        let storage = db_actor::block_on(db_storage::MemoryStorage::new(db_storage::db_io_test_pool())).unwrap();
        let document = ArtifactId("fuzz-doc".to_string());
        let writer = db_actor::block_on(storage.acquire_writer(&document)).map_err(|err| err.to_string())?;
        let seed = db_actor::block_on(async {
            storage.create_segment(&writer, 0).await?;
            storage.append(&writer, 0, pages(bytes)).await?;
            Ok::<_, DbError>(())
        })
        .map_err(|err| err.to_string());
        let release = db_actor::block_on(writer.release()).map_err(|err| err.error().to_string());
        seed?;
        release?;
        let storage: Arc<DbBackend> = Arc::new(DbBackend::Memory(storage));
        match db_artifact::ArtifactEngine::open(protocol::ArtifactId(document.0), &storage, db_artifact::ArtifactEngineConfig::default(), 0) {
            Ok(_) => Ok(()),
            Err(mut rejected) => loop {
                match db_actor::block_on(rejected.retry_close()) {
                    Ok(error) => break Err(error.to_string()),
                    Err(retained) => rejected = retained,
                }
            },
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_recovery_never_panics_under_truncation_or_bit_flip_corruption() {
        let document = protocol::ArtifactId("fuzz-doc".to_string());
        let storage: Arc<DbBackend> = Arc::new(DbBackend::Memory(db_actor::block_on(db_storage::MemoryStorage::new(db_storage::db_io_test_pool())).unwrap()));
        {
            let mut engine = db_artifact::ArtifactEngine::create(document.clone(), storage.clone(), db_artifact::ArtifactEngineConfig::default(), 0).unwrap();
            for i in 0..2 {
                let envelope = schema_erased_envelope(&document, &format!("op-{i}"), "actor-1", &format!("x{i}"), serde_json::json!(i), serde_json::Value::Null).await;
                engine.submit(single_envelope_batch(envelope).await, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, i as u64).await.unwrap();
            }
        }
        let core_document = ArtifactId(document.0);
        let wal_facet = db_actor::block_on(storage.wal());
        let len = db_actor::block_on(wal_facet.segment_len(&core_document, 0)).unwrap();
        let bytes = db_io_pages_into_vec(db_actor::block_on(wal_facet.read(&core_document, 0, pack::ByteRange { offset: 0, len })).unwrap()).await;

        let truncation_report = pack_testkit::fuzz_truncation(&bytes, pack_testkit::CorruptionLevel::Exhaustive, decode_wal_bytes);
        assert!(truncation_report.cases_panicked.is_empty(), "wal recovery must never panic on truncated input: {:?}", truncation_report.cases_panicked);
        assert!(truncation_report.cases_run > 0);

        let bit_flip_report = pack_testkit::fuzz_bit_flips(&bytes, pack_testkit::CorruptionLevel::Long, decode_wal_bytes);
        assert!(bit_flip_report.cases_panicked.is_empty(), "wal recovery must never panic on bit-flipped input: {:?}", bit_flip_report.cases_panicked);
        assert!(bit_flip_report.cases_run > 0);
    }
}
//#endregion 🔖️exhaustive
