//! 🩺️ Verify, query, replay, repair, compact and health: the document verification and the commands that read, replay, repair, compact and report health. See [`super`].

use super::*;

//#region 🔖️Verify
/// 🔬️ The shared per-document check `verify` runs: a full WAL replay (rejects a torn tail) plus,
/// if a snapshot exists, a full-level `SnapshotManager::verify` of its latest generation.
pub(super) async fn verify_document(storage: &db_storage_fs::FsStorage, document: &db_ids::ArtifactId) -> Result<String, db_ids::DbError> {
    let mut records = db_wal_log::replay_committed_document(storage, document, wal_cursor_control()?).await?;
    let mut record_count = 0usize;
    let replay = async {
        loop {
            let mut transaction = match records.next_transaction_step().await? {
                db_wal_log::WalCommittedStep::Transaction(transaction) => transaction,
                db_wal_log::WalCommittedStep::Yield => {
                    semio_framework_async::yield_once().await;
                    continue;
                }
                db_wal_log::WalCommittedStep::Done => break,
            };
            loop {
                match transaction.next_record_step()? {
                    db_wal_log::WalCommittedRecordStep::Record(_) => record_count += 1,
                    db_wal_log::WalCommittedRecordStep::Yield => {
                        semio_framework_async::yield_once().await;
                        continue;
                    }
                    db_wal_log::WalCommittedRecordStep::Done => break,
                }
                while transaction.close_record_step()? {
                    semio_framework_async::yield_once().await;
                }
            }
            transaction.finish()?;
        }
        Ok::<(), db_ids::DbError>(())
    }
    .await;
    let closed = MountedWalReplayCommandClose::committed(records).await;
    replay?;
    closed?;
    let manager = db_snapshot::SnapshotManager::new(storage).await;
    match db_actor::block_on(manager.load_latest(document))? {
        Some((generation, _descriptor)) => {
            db_actor::block_on(manager.verify(document, generation, pack::os_pack::VerificationLevel::Full))?;
            Ok(format!("wal committed records={record_count} snapshot generation={generation} (verified)"))
        }
        None => Ok(format!("wal committed records={record_count} snapshot=none")),
    }
}

/// 🔬️ `db verify <root> [document-id] [--profile ...]` — verifies one document, or every document
/// in the catalog if none is given. Prints `OK <id>: <summary>` / `FAIL <id>: <reason>` per
/// document, never panics on corrupt input; exits `1` iff any document failed.
pub(super) async fn cmd_verify(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest).await;
    let Some(root) = positional.first() else {
        return usage("usage: db verify <root> [document-id] [--profile test|dev|prod]").await;
    };
    let profile = match parse_profile(&flags).await {
        Ok(profile) => profile,
        Err(message) => return usage(&message).await,
    };

    let ids: Vec<String> = match positional.get(1) {
        Some(id) => vec![id.clone()],
        None => {
            let mut database = match open_database(Path::new(root), profile).await {
                Ok(database) => database,
                Err(err) => return fail("open", err).await,
            };
            let ids = database.catalog().await.artifacts.iter().map(|entry| entry.document.0.clone()).collect();
            if let Err(err) = database.shutdown(&db_engine::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await {
                return fail("shutdown", err).await;
            }
            ids
        }
    };
    if ids.is_empty() {
        println!("db verify: no documents in catalog at '{root}'");
        return 0;
    }

    let storage = match open_fs_storage(Path::new(root)) {
        Ok(storage) => storage,
        Err(err) => return fail("open", err).await,
    };
    let mut failures = 0usize;
    for id in &ids {
        let document = db_ids::ArtifactId(id.clone());
        match verify_document(&storage, &document).await {
            Ok(summary) => println!("OK   {id}: {summary}"),
            Err(err) => {
                println!("FAIL {id}: {err}");
                failures += 1;
            }
        }
    }
    if failures > 0 {
        1
    } else {
        0
    }
}
//#endregion 🔖️Verify

//#region 🔖️Query
/// 🔎️ `db query <root> <document-id> <path> [more-paths...] [--profile ...]` — resolves one or
/// more paths against `document`'s live (`Consistency::Canonical`) state through a real
/// `ArtifactHandle::query`.
pub(super) async fn cmd_query(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest).await;
    if positional.len() < 3 {
        return usage("usage: db query <root> <document-id> <path> [more-paths...] [--profile test|dev|prod]").await;
    }
    let root = &positional[0];
    let id = &positional[1];
    let paths: Vec<String> = positional[2..].to_vec();
    let profile = match parse_profile(&flags).await {
        Ok(profile) => profile,
        Err(message) => return usage(&message).await,
    };
    let mut database = match open_database(Path::new(root), profile).await {
        Ok(database) => database,
        Err(err) => return fail("open", err).await,
    };
    let document_id = protocol::ArtifactId(id.clone());
    let handle = match database.document(&document_id).await {
        Ok(handle) => handle,
        Err(err) => return fail_document_open("document", err).await,
    };

    let query = match paths.len() {
        1 => db_engine_base::Query::Get { path: paths[0].clone() },
        _ => db_engine_base::Query::GetMany { paths },
    };
    let outcome = match handle.query(query, db_engine_base::Consistency::Canonical).await {
        Ok(mut stream) => {
            for entry in stream.iter() {
                match entry.value() {
                    Some(bytes) => println!("{} = {}", entry.path(), describe_value_bytes(bytes)),
                    None => println!("{} = <unset>", entry.path()),
                }
            }
            match (|| -> Result<(), db_ids::DbError> {
                let _ = stream.close_step()?;
                Ok(())
            })() {
                Ok(()) => 0,
                Err(error) => fail("query close", error).await,
            }
        }
        Err(err) => fail("query", err).await,
    };

    drop(handle);
    match database.shutdown(&db_engine::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await {
        Ok(()) => outcome,
        Err(err) => fail("shutdown", err).await,
    }
}
//#endregion 🔖️Query

//#region 🔖️Replay
/// 🔁️ `db replay <root> <document-id>` — a raw, actor-bypassing `db::wal::replay_document` pass:
/// record-kind counts plus the frontier reconstructed from the last `WAL_FRONTIER` record. Distinct
/// from `doc`, which goes through a live `ArtifactAuthority` — this is the lower-level diagnostic
/// twin, useful precisely when the actor path itself is in question.
pub(super) async fn cmd_replay(rest: &[String]) -> i32 {
    let (positional, _flags) = parse_args(rest).await;
    if positional.len() < 2 {
        return usage("usage: db replay <root> <document-id>").await;
    }
    let root = &positional[0];
    let id = &positional[1];
    let storage = match open_fs_storage(Path::new(root)) {
        Ok(storage) => storage,
        Err(err) => return fail("open", err).await,
    };
    let document = db_ids::ArtifactId(id.clone());
    let control = match wal_cursor_control() {
        Ok(control) => control,
        Err(err) => return fail("wal cursor", err).await,
    };
    let mut records = match db_wal_log::replay_document(&storage, &document, control).await {
        Ok(records) => records,
        Err(err) => return fail("replay", err).await,
    };

    let mut kind_counts: std::collections::BTreeMap<&'static str, usize> = std::collections::BTreeMap::new();
    let mut last_frontier: Option<db_durability::Frontier> = None;
    let mut record_count = 0usize;
    loop {
        let record = match records.next_step().await {
            Ok(db_wal_log::WalReplayStep::Record(record)) => record,
            Ok(db_wal_log::WalReplayStep::Yield) => continue,
            Ok(db_wal_log::WalReplayStep::Done) => break,
            Err(err) => {
                let _ = MountedWalReplayCommandClose::new(records).await;
                return fail("replay", err).await;
            }
        };
        record_count += 1;
        *kind_counts.entry(wal_record_kind_name(&record)).or_insert(0) += 1;
        if let db_wal_log::WalRecord::Frontier(frontier) = &record {
            last_frontier = Some(frontier.clone());
        }
        if let Err(err) = MountedWalRecordCommandClose::new(record).await {
            let _ = MountedWalReplayCommandClose::new(records).await;
            return fail("record close", err).await;
        }
    }
    if let Err(err) = MountedWalReplayCommandClose::new(records).await {
        return fail("replay close", err).await;
    }

    println!("== replay: {id} ==");
    println!("  records: {record_count}");
    for (kind, count) in &kind_counts {
        println!("  - {kind}: {count}");
    }
    match last_frontier {
        Some(frontier) => {
            println!("  reconstructed frontier:");
            println!("    head_seq: {}", frontier.head_seq);
            println!("    commit_seq: {}", frontier.commit_seq);
            println!("    epoch: {}", frontier.epoch);
            println!("    chain_hash: {}", hex32(&frontier.chain_hash));
        }
        None => println!("  reconstructed frontier: none (no committed transaction found)"),
    }
    0
}
//#endregion 🔖️Replay

//#region 🔖️Repair
/// 🩹️ `db repair <root> <document-id>` — the real repair primitive: `db::wal::ArtifactWal::open`
/// already discards a torn active-segment tail and rewrites it from the trusted prefix (see its own
/// doc's "forced by `protocol::SprWriter`'s API" design-choice note) — this subcommand simply drives
/// that recovery path and reports what it found. Idempotent on an already-clean WAL.
pub(super) async fn cmd_repair(rest: &[String]) -> i32 {
    let (positional, _flags) = parse_args(rest).await;
    if positional.len() < 2 {
        return usage("usage: db repair <root> <document-id>").await;
    }
    let root = &positional[0];
    let id = &positional[1];
    let storage = match open_fs_storage(Path::new(root)) {
        Ok(storage) => storage,
        Err(err) => return fail("open", err).await,
    };
    let document = db_ids::ArtifactId(id.clone());
    match db_actor::block_on(db_wal::ArtifactWal::open(&storage, document, db_wal_log::GroupCommitPolicy::default(), now_ms().await)) {
        Ok((mut wal, report)) => {
            if let Err(err) = wal.close().await {
                return fail("repair close", err).await;
            }
            println!("== repair: {id} ==");
            println!("  segments_seen: {}", report.segments_seen);
            println!("  records_replayed: {}", report.records_replayed);
            println!("  torn_tail_bytes: {}", report.torn_tail_bytes);
            if report.torn_tail_bytes > 0 {
                println!("  discarded a torn tail — the active segment has been rewritten");
            } else {
                println!("  no torn tail found — wal was already clean");
            }
            0
        }
        Err(err) => fail_wal_open("repair", err).await,
    }
}
//#endregion 🔖️Repair

//#region 🔖️Compact
/// 🧹️ `db compact <root> <document-id> [--holder H] [--consolidate] [--profile ...]` — drives a
/// real, fenced `db_compact::Compactor` pass via `Database::compact_document`.
pub(super) async fn cmd_compact(rest: &[String]) -> i32 {
    let (rest, consolidate) = strip_flag(rest, "consolidate").await;
    let (positional, flags) = parse_args(&rest).await;
    if positional.len() < 2 {
        return usage("usage: db compact <root> <document-id> [--holder H] [--consolidate] [--profile test|dev|prod]").await;
    }
    let root = &positional[0];
    let id = &positional[1];
    let holder = flags.get("holder").cloned().unwrap_or_else(|| "db-cli".to_string());
    let profile = match parse_profile(&flags).await {
        Ok(profile) => profile,
        Err(message) => return usage(&message).await,
    };
    let mut database = match open_database(Path::new(root), profile).await {
        Ok(database) => database,
        Err(err) => return fail("open", err).await,
    };
    let document_id = protocol::ArtifactId(id.clone());

    let outcome = match database.compact_document(&document_id, &holder, consolidate).await {
        Ok(report) => {
            println!("== compact: {id} ==");
            println!("  wal_segments_deleted: {}", report.wal_segments_deleted);
            println!("  payloads_deleted: {}", report.payloads_deleted);
            println!("  index_reports: {}", report.index_reports.len());
            println!("  snapshot_consolidated_generation: {}", report.snapshot_consolidated_generation.map_or_else(|| "-".to_string(), |generation| generation.to_string()));
            println!("  snapshot_generations_pruned: {}", report.snapshot_generations_pruned);
            0
        }
        Err(err) => fail_document_open("compact", err).await,
    };

    match database.shutdown(&db_engine::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await {
        Ok(()) => outcome,
        Err(err) => fail("shutdown", err).await,
    }
}
//#endregion 🔖️Compact

//#region 🔖️HealthCmd
/// 🩺️ `db health <root> [--profile ...]` — a real `Database::health()` snapshot. Exits `1` iff the
/// overall status is `Unhealthy`.
pub(super) async fn cmd_health(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest).await;
    let Some(root) = positional.first() else {
        return usage("usage: db health <root> [--profile test|dev|prod]").await;
    };
    let profile = match parse_profile(&flags).await {
        Ok(profile) => profile,
        Err(message) => return usage(&message).await,
    };
    let mut database = match open_database(Path::new(root), profile).await {
        Ok(database) => database,
        Err(err) => return fail("open", err).await,
    };
    let health = database.health().await;
    print_health(&health);
    let exit = if matches!(health.report.overall, db_observe::HealthState::Unhealthy(_)) { 1 } else { 0 };

    match database.shutdown(&db_engine::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await {
        Ok(()) => exit,
        Err(err) => fail("shutdown", err).await,
    }
}
//#endregion 🔖️HealthCmd
