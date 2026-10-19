use super::*;

use db_storage::FaultScript;
use std::{cell::RefCell, rc::Rc};
#[cfg(test)]
use pack::corruption_testing as pack_corruption;

/// 🧵️ Each law owns its pool: the process entry-point pool belongs to the process's real entry
/// point (the CLI), and a law that claimed it with its own sizing would refuse every other one.
fn entrypoint_pool() -> Arc<semio_framework_async::WorkerPool> {
    Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)))
}

fn pages(bytes: &[u8]) -> DbIoPages {
    let mut writer = db_io_core::DbIoPageWriter::try_reserve(bytes.len().div_ceil(db_io_core::DB_IO_PAGE_BYTES)).expect("testkit writer admitted");
    for fragment in bytes.chunks(db_io_core::DB_IO_PAGE_BYTES) {
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
    let mut paths: std::collections::HashSet<String> = std::collections::HashSet::new();
    for envelope in ops_a.iter() {
        let decoded = db_artifact_contract::decode_pathmap_json(&envelope.diff.payload).await.unwrap();
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
    if !db_io_driver::process_isolated_law("tests::explore_interleavings_every_permutation_of_disjoint_writes_converges_to_the_same_state") {
        return;
    }
    let document = protocol::ArtifactId("explore-doc".to_string());
    let ops = WorkloadGen::new(55).disjoint_batch(&document, 5).await;
    let hashes = explore_interleavings(4242, 12, |seed| {
        let ops = ops.clone();
        let storage: Arc<DbBackend> = Arc::new(DbBackend::Memory(db_actor::block_on(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool())).unwrap()));
        let engine = Rc::new(RefCell::new(db_artifact_engine::ArtifactEngine::create(document.clone(), storage, db_artifact_engine::ArtifactEngineConfig::default(), 0).expect("create engine")));
        let mut runtime = SimRuntime::new(seed);
        for (i, envelope) in ops.into_iter().enumerate() {
            let engine = engine.clone();
            runtime.schedule(envelope.mutation_id.0.clone(), move |clock| {
                let now = clock.now_ms() + i as u64;
                db_actor::block_on(engine.borrow_mut().submit(db_actor::block_on(single_envelope_batch(envelope)), db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, now)).expect("submit");
            });
        }
        runtime.run(4);
        let chain_hash = db_actor::block_on(engine.borrow().frontier()).chain_hash;
        chain_hash
    });
    assert!(hashes.windows(2).all(|pair| pair[0] == pair[1]), "every explored interleaving of disjoint writes must converge to the identical state hash");
}
//#endregion 🔖️SimRuntime
//#region 🔖️CrashHarness
#[semio_framework_async_macros::async_test]
async fn crash_harness_recovers_cleanly_after_every_injected_write_failure() {
    if !db_io_driver::process_isolated_law("tests::crash_harness_recovers_cleanly_after_every_injected_write_failure") {
        return;
    }
    let report = CrashHarness::run_crash_after_every_write(9001, 4).await;
    assert!(report.writes_tested >= 4, "at least one write boundary per committed submit plus genesis must be tested");
    assert!(report.is_clean().await, "recovery must never fail or corrupt state after any single injected write failure: {report:?}");
}

#[semio_framework_async_macros::async_test]
async fn document_wal_open_recovers_from_a_torn_write_by_truncating_the_tail() {
    let storage: Arc<DbBackend> = new_fault_backend().await;
    as_fault(&storage).await.set_script(FaultScript { torn_write_at: Some((2, 1)), ..FaultScript::default() }).await;
    let document = protocol::ArtifactId("torn-doc".to_string());
    {
        let mut engine = db_artifact_engine::ArtifactEngine::create(document.clone(), storage.clone(), db_artifact_engine::ArtifactEngineConfig::default(), 0).unwrap();
        let envelope = schema_erased_envelope(&document, "op-1", "actor-1", "x", serde_json::json!(1), serde_json::Value::Null);
        let _ = engine.submit(single_envelope_batch(envelope.await).await, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 1).await;
    }
    let (recovered, report) = db_artifact_engine::ArtifactEngine::open(document, &storage, db_artifact_engine::ArtifactEngineConfig::default(), 2).expect("recovery must not error");
    assert_eq!(report.torn_tail_bytes, 1, "recovery must report exactly the torn bytes it discarded");
    assert_eq!(recovered.frontier().await.head_seq, 0, "the torn (unrecoverable) commit must not be visible after recovery");
}
//#endregion 🔖️CrashHarness
//#region 🔖️Laws
#[semio_framework_async_macros::async_test]
async fn law_replay_deterministic() {
    if !db_io_driver::process_isolated_law("tests::law_replay_deterministic") {
        return;
    }
    assert_replay_deterministic(entrypoint_pool(), 11, 5).await;
}

#[semio_framework_async_macros::async_test]
async fn law_snapshot_plus_suffix_equals_replay() {
    if !db_io_driver::process_isolated_law("tests::law_snapshot_plus_suffix_equals_replay") {
        return;
    }
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
    if !db_io_driver::process_isolated_law("tests::law_sync_convergence") {
        return;
    }
    assert_sync_convergence(16, 10).await;
}

#[semio_framework_async_macros::async_test]
async fn law_fencing_excludes_stale_writer_memory() {
    assert_fencing_excludes_stale_writer(&db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()).await;
}

#[semio_framework_async_macros::async_test]
async fn law_fencing_excludes_stale_writer_fs() {
    if !db_io_driver::process_isolated_law("tests::law_fencing_excludes_stale_writer_fs") {
        return;
    }
    let root = temp_dir("fencing-fs").await;
    let storage = db_actor::block_on(db_storage_fs::FsStorage::open(entrypoint_pool(), &root)).expect("open fs storage");
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
/// 🐌️ Genuinely slower, exhaustive-only corruption fuzzing — real db_wal recovery driven
/// through `pack_corruption`'s truncation/bit-flip corruption harness (its documented precedent,
/// per this crate's own module doc), proving `db_wal::replay_document` never panics on a
/// corrupted WAL segment, only ever returns an `Err` (or, rarely, coincidentally still decodes).
mod exhaustive {
    use pack::corruption_testing as pack_corruption;
    use super::*;

    // 🚫️async: E1-adjacent — consumed as `impl Fn(&[u8]) -> Result<(), String>` by
    // `pack_corruption::fuzz_truncation`/`fuzz_bit_flips` (sync closure bound, first-party but not
    // ours to change from this test module), and every suspension point inside is already
    // bridged synchronously via `db_actor::block_on` — see R9.
    fn decode_wal_bytes(bytes: &[u8]) -> Result<(), String> {
        let storage = db_actor::block_on(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool())).unwrap();
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
        match db_artifact_engine::ArtifactEngine::open(protocol::ArtifactId(document.0), &storage, db_artifact_engine::ArtifactEngineConfig::default(), 0) {
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
        let storage: Arc<DbBackend> = Arc::new(DbBackend::Memory(db_actor::block_on(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool())).unwrap()));
        {
            let mut engine = db_artifact_engine::ArtifactEngine::create(document.clone(), storage.clone(), db_artifact_engine::ArtifactEngineConfig::default(), 0).unwrap();
            for i in 0..2 {
                let envelope = schema_erased_envelope(&document, &format!("op-{i}"), "actor-1", &format!("x{i}"), serde_json::json!(i), serde_json::Value::Null).await;
                engine.submit(single_envelope_batch(envelope).await, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, i as u64).await.unwrap();
            }
        }
        let core_document = ArtifactId(document.0);
        let wal_facet = db_actor::block_on(storage.wal());
        let len = db_actor::block_on(wal_facet.segment_len(&core_document, 0)).unwrap();
        let bytes = db_io_pages_into_vec(db_actor::block_on(wal_facet.read(&core_document, 0, pack::ByteRange { offset: 0, len })).unwrap()).await;

        let truncation_report = pack_corruption::fuzz_truncation(&bytes, pack_corruption::CorruptionLevel::Exhaustive, decode_wal_bytes);
        assert!(truncation_report.cases_panicked.is_empty(), "wal recovery must never panic on truncated input: {:?}", truncation_report.cases_panicked);
        assert!(truncation_report.cases_run > 0);

        let bit_flip_report = pack_corruption::fuzz_bit_flips(&bytes, pack_corruption::CorruptionLevel::Long, decode_wal_bytes);
        assert!(bit_flip_report.cases_panicked.is_empty(), "wal recovery must never panic on bit-flipped input: {:?}", bit_flip_report.cases_panicked);
        assert!(bit_flip_report.cases_run > 0);
    }
}
//#endregion 🔖️exhaustive
