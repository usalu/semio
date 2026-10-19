//! 🚀️ Simulate, migrate, profile and main: conflict and replica simulation, `migrate`, `profile`, the help text and the `main_impl` entry. See [`super`].

use super::*;

//#region 🔖️ConflictSimulate
fn describe_conflict_kind(kind: &db_conflict::ConflictKind) -> String {
    match kind {
        db_conflict::ConflictKind::TouchedRegion(regions) => format!("touched_region({})", regions.iter().map(|region| region.path.as_str()).collect::<Vec<_>>().join(",")),
        db_conflict::ConflictKind::Constraint(description) => format!("constraint({description})"),
    }
}

fn touched_command(command_id: &str, actor: &str, kind: &str, hlc_actor: u64, paths: &str) -> db_conflict::CommandTouch {
    let timestamp = db_actor::block_on(async { protocol::HybridLogicalTimestamp::new(hlc_actor, now_ms().await) });
    let touch = db_conflict::CommandTouch::new(protocol::MutationId(command_id.to_string()), protocol::ActorId(actor.into()), db_conflict::CommandKind::from(kind), timestamp);
    paths.split(',').map(str::trim).filter(|path| !path.is_empty()).fold(touch, |touch, path| touch.touch(db_state::TouchedRegion::write(path)))
}

/// ⚔️ `db conflict-simulate --touch-a p1,p2 --touch-b p2,p3 [--kind-a K] [--kind-b K]` — runs the
/// real `db::conflict::ConflictDetector` over two hand-built `CommandTouch`es (no storage touched at
/// all: a pure, local simulation). Exits `1` iff a conflict was found (so the exit code alone
/// answers "would these conflict"); grading a found conflict into a `protocol::Severity` (and
/// whether a `protocol::MergePolicy` would reject it) is `db_artifact`'s job one layer up, not this
/// detection-only simulation's (see `db_conflict`'s own module doc).
async fn cmd_conflict_simulate(rest: &[String]) -> i32 {
    let (_positional, flags) = parse_args(rest).await;
    let Some(touch_a) = flags.get("touch-a") else {
        return usage("usage: db conflict-simulate --touch-a p1,p2 --touch-b p2,p3 [--kind-a K] [--kind-b K]").await;
    };
    let Some(touch_b) = flags.get("touch-b") else {
        return usage("db conflict-simulate: --touch-b is required").await;
    };
    let kind_a = flags.get("kind-a").map_or("command-a", String::as_str);
    let kind_b = flags.get("kind-b").map_or("command-b", String::as_str);

    let command_a = touched_command("simulated-a", "actor-a", kind_a, 1, touch_a);
    let command_b = touched_command("simulated-b", "actor-b", kind_b, 2, touch_b);
    let records = db_conflict::ConflictDetector::new().detect(&[command_a, command_b]);
    if records.is_empty() {
        println!("no conflict detected");
        return 0;
    }
    for record in &records {
        println!("conflict: {} <-> {} kind={}", record.command_id.0, record.conflicting_with.0, describe_conflict_kind(&record.kind));
    }
    1
}
//#endregion 🔖️ConflictSimulate

//#region 🔖️ReplicaSimulate
/// 📡️ `db replica-simulate <leader-root> <follower-root> <document-id>` — runs the real
/// `db::cluster::replicate_document` primitive between two local `FsStorage` roots: replays both
/// sides' WALs, decides a bootstrap plan, and applies it (tail-append or raw snapshot copy). No
/// network transport exists in this family yet — this is a genuine local simulation of the
/// replication mechanism, not a stub.
async fn cmd_replica_simulate(rest: &[String]) -> i32 {
    let (positional, _flags) = parse_args(rest).await;
    if positional.len() < 3 {
        return usage("usage: db replica-simulate <leader-root> <follower-root> <document-id>").await;
    }
    let leader_root = &positional[0];
    let follower_root = &positional[1];
    let id = &positional[2];

    let leader = match open_fs_storage(Path::new(leader_root)) {
        Ok(storage) => db_storage::DbBackend::Fs(storage),
        Err(err) => return fail("open leader", err).await,
    };
    let follower = match open_fs_storage(Path::new(follower_root)) {
        Ok(storage) => db_storage::DbBackend::Fs(storage),
        Err(err) => return fail("open follower", err).await,
    };
    let document = db_ids::ArtifactId(id.clone());

    match db_actor::block_on(db_cluster::replicate_document(&leader, &follower, document, db_wal_log::GroupCommitPolicy::default(), now_ms().await)) {
        Ok(db_cluster::ReplicationOutcome::UpToDate { frontier }) => {
            println!("== replica-simulate: up to date ==");
            println!("  head_seq: {}", frontier.head_seq);
            0
        }
        Ok(db_cluster::ReplicationOutcome::TailApplied { frontier, count }) => {
            println!("== replica-simulate: tail applied ==");
            println!("  commands: {count}");
            println!("  head_seq: {}", frontier.head_seq);
            0
        }
        Ok(db_cluster::ReplicationOutcome::SnapshotTransferred { generation, pack_hash }) => {
            println!("== replica-simulate: snapshot transferred ==");
            println!("  generation: {generation}");
            println!("  pack_hash: {}", hex32(&pack_hash));
            0
        }
        Err(err) => fail("replicate", close_replication_rejected(err).await).await,
    }
}
//#endregion 🔖️ReplicaSimulate

//#region 🔖️Migrate
/// 🚚️ `db migrate <root> <document-id> <name> [--payload TEXT]` — appends a real `WAL_MIGRATION`
/// record (`name` on its own line, then an optional `--payload` text body) via a real
/// `db::wal::ArtifactWal`, force-flushed durably (`DurabilityClass::Fsync`). `ArtifactWal::open`
/// auto-creates a fresh WAL if `document` has none yet (see its own doc's "Creates a fresh WAL...
/// if `document` has no segments yet" note), so this also works as a bootstrap path for a document
/// that was never `create_document`d through the actor API.
async fn cmd_migrate(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest).await;
    if positional.len() < 3 {
        return usage("usage: db migrate <root> <document-id> <name> [--payload TEXT]").await;
    }
    let root = &positional[0];
    let id = &positional[1];
    let name = &positional[2];
    let payload = flags.get("payload").cloned().unwrap_or_default();

    let storage = match open_fs_storage(Path::new(root)) {
        Ok(storage) => storage,
        Err(err) => return fail("open", err).await,
    };
    let document = db_ids::ArtifactId(id.clone());
    let now = now_ms().await;
    let (mut wal, _report) = match db_actor::block_on(db_wal::ArtifactWal::open(&storage, document, db_wal_log::GroupCommitPolicy::default(), now)) {
        Ok(pair) => pair,
        Err(err) => return fail_wal_open("open wal", err).await,
    };
    let outcome: Result<db_wal::WalAppendReceipt, (&'static str, db_ids::DbError)> = async {
        let bytes = format!("{name}\n{payload}").into_bytes();
        let mut control = wal_cursor_control().map_err(|err| ("wal cursor", err))?;
        let bytes = admit_cli_wal_bytes(bytes, 1024 * 1024, &mut control).await.map_err(|err| ("wal migration admission", err))?;
        let mut records = db_wal_log::WalRecordBatch::new();
        if let Err(record) = records.push(db_wal_log::WalRecord::Migration(bytes)) {
            let record_close = MountedWalRecordCommandClose::new(record).await;
            let batch_close = MountedWalBatchCommandClose::new(records).await;
            match (record_close, batch_close) {
                (Err(err), Err(batch_err)) => {
                    eprintln!("db: wal migration batch close after record close failure: {batch_err}");
                    return Err(("wal migration record close", err));
                }
                (Err(err), _) => return Err(("wal migration record close", err)),
                (_, Err(err)) => return Err(("wal migration batch close", err)),
                (Ok(_), Ok(_)) => {}
            }
            return Err(("wal migration", db_ids::DbError::LimitExceeded("cli wal record batch")));
        }
        let submission = wal.submit(&storage, &[], &records, db_durability::DurabilityClass::Fsync, now).await;
        let batch_close = MountedWalBatchCommandClose::new(records).await;
        let receipt = match (submission, batch_close) {
            (Ok(receipt), Ok(_)) => receipt,
            (Err(err), Err(close_err)) => {
                eprintln!("db: wal migration close after submission failure: {close_err}");
                return Err(("migrate", err));
            }
            (Err(err), _) => return Err(("migrate", err)),
            (_, Err(err)) => return Err(("wal migration close", err)),
        };
        wal.force_flush(&storage).await.map_err(|err| ("flush", err))?;
        Ok(receipt)
    }
    .await;
    let wal_close = wal.close().await;
    match (outcome, wal_close) {
        (Ok(receipt), Ok(())) => {
            println!("== migrate: {id} ==");
            println!("  name: {name}");
            println!("  segment_index: {}", receipt.segment_index);
            println!("  tx_id: {}", receipt.tx_id);
            println!("  committed: {}", receipt.committed);
            0
        }
        (Err((context, err)), Ok(())) => fail(context, err).await,
        (Ok(_), Err(err)) => fail("wal close", err).await,
        (Err((context, err)), Err(close_err)) => {
            eprintln!("db: {context}: {err}");
            fail("wal close after migration failure", close_err).await
        }
    }
}
//#endregion 🔖️Migrate

//#region 🔖️Profile
/// ⏱️ `db profile <root> <document-id> [--commands N] [--durability memory|os|fsync|quorum:N]
/// [--policy laissez-faire|normal|vigilant] [--profile test|dev|prod]` — submits `N` (default 100)
/// trivial single-path `set`-shaped commands sequentially through the real submit pipeline
/// (`ArtifactHandle::submit`, actor-mediated, WAL group-commit and all) and reports wall-clock
/// throughput/latency. Opens `document` if it already exists, else creates it first —
/// self-contained, no separate seeding step required. Deliberately hand-timed with
/// `std::time::Instant` rather than pulling in `db_fault_testing`'s `WorkloadGen`/criterion harness:
/// `db_fault_testing` is a sibling crate, not part of the `db` facade's own re-export surface, and this
/// crate's dependency footprint is the `db` facade alone (see module doc).
async fn cmd_profile(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest).await;
    if positional.len() < 2 {
        return usage("usage: db profile <root> <document-id> [--commands N] [--durability memory|os|fsync|quorum:N] [--policy laissez-faire|normal|vigilant] [--profile test|dev|prod]").await;
    }
    let root = &positional[0];
    let id = &positional[1];
    let commands: u64 = match flags.get("commands") {
        Some(value) => match value.parse() {
            Ok(commands) => commands,
            Err(_) => return usage("db profile: --commands must be a non-negative integer").await,
        },
        None => 100,
    };
    let durability = match flags.get("durability").map(String::as_str) {
        None => db_durability::DurabilityClass::Fsync,
        Some("memory") => db_durability::DurabilityClass::Memory,
        Some("os") => db_durability::DurabilityClass::Os,
        Some("fsync") => db_durability::DurabilityClass::Fsync,
        Some(other) => match other.strip_prefix("quorum:") {
            Some(n) => match n.parse::<u8>() {
                Ok(n) => db_durability::DurabilityClass::Quorum(n),
                Err(_) => return usage(&format!("db profile: bad --durability quorum count '{n}'")).await,
            },
            None => return usage(&format!("db profile: unknown --durability '{other}' (expected memory|os|fsync|quorum:N)")).await,
        },
    };
    let policy = match parse_merge_policy(&flags).await {
        Ok(policy) => policy,
        Err(message) => return usage(&message).await,
    };
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
        Err(rejected) => {
            let error = close_document_open_rejected(rejected).await;
            if !matches!(error, db_ids::DbError::NotFound(_)) {
                return fail("document", error).await;
            }
            match database.create_document(db_engine::ArtifactSpec::new(document_id.clone()).await).await {
                Ok(handle) => handle,
                Err(err) => return fail_document_open("create", err).await,
            }
        }
    };

    let start = std::time::Instant::now();
    for counter in 0..commands {
        let mut forward = serde_json::Map::with_capacity(1);
        forward.insert("cli/profile/counter".to_string(), serde_json::json!(counter));
        let mut backward = serde_json::Map::with_capacity(1);
        backward.insert("cli/profile/counter".to_string(), serde_json::Value::Null);
        let envelope = protocol::MutationEnvelope {
            mutation_id: protocol::MutationId(format!("profile-{}-{counter}", now_ms().await)),
            document_id: document_id.clone(),
            actor: protocol::ActorId("profiler".into()),
            dependencies: Vec::new(),
            observed: None,
            target: Vec::new(),
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db_artifact_contract::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact_contract::encode_pathmap_json(&serde_json::Value::Object(forward)).await.unwrap_or_default() },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId(db_artifact_contract::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact_contract::encode_pathmap_json(&serde_json::Value::Object(backward)).await.unwrap_or_default() },
            timestamp: protocol::HybridLogicalTimestamp::new(0, now_ms().await),
            transaction: None, verb: None, line: None,
        };
        let batch = match db_artifact_contract::CommandBatch::new(vec![envelope]).await {
            Ok(batch) => batch,
            Err(err) => return fail("build batch", err).await,
        };
        match db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions { durability, policy })) {
            Ok(Ok(_receipt)) => {}
            Ok(Err(err)) => return fail(&format!("submit rejected at command {counter}"), err).await,
            Err(err) => return fail(&format!("submit failed at command {counter}"), err).await,
        }
    }
    let elapsed = start.elapsed();
    let elapsed_secs = elapsed.as_secs_f64();
    let per_sec = if elapsed_secs > 0.0 { commands as f64 / elapsed_secs } else { f64::INFINITY };
    let avg_latency_us = if commands > 0 { (elapsed.as_micros() as f64) / (commands as f64) } else { 0.0 };
    println!("== profile: {id} ==");
    println!("  commands: {commands}");
    println!("  elapsed_ms: {:.3}", elapsed_secs * 1000.0);
    println!("  commands_per_sec: {per_sec:.1}");
    println!("  avg_latency_us: {avg_latency_us:.1}");

    drop(handle);
    match database.shutdown(&db_engine::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await {
        Ok(()) => 0,
        Err(err) => fail("shutdown", err).await,
    }
}
//#endregion 🔖️Profile

//#region 🔖️Cli
async fn print_help() {
    eprintln!("usage: db <command> [args...]");
    eprintln!();
    eprintln!("commands:");
    eprintln!("  inspect <root> [--profile test|dev|prod]");
    eprintln!("  doc <root> <document-id> [--profile ...]");
    eprintln!("  wal-inspect <root> <document-id> [--limit N]");
    eprintln!("  snapshot-inspect <root> <document-id> [--generation N] [--verify]");
    eprintln!("  verify <root> [document-id] [--profile ...]");
    eprintln!("  query <root> <document-id> <path> [more-paths...] [--profile ...]");
    eprintln!("  replay <root> <document-id>");
    eprintln!("  repair <root> <document-id>");
    eprintln!("  compact <root> <document-id> [--holder H] [--consolidate] [--profile ...]");
    eprintln!("  health <root> [--profile ...]");
    eprintln!("  conflict-simulate --touch-a p1,p2 --touch-b p2,p3 [--kind-a K] [--kind-b K]");
    eprintln!("  replica-simulate <leader-root> <follower-root> <document-id>");
    eprintln!("  migrate <root> <document-id> <name> [--payload TEXT]");
    eprintln!("  profile <root> <document-id> [--commands N] [--durability memory|os|fsync|quorum:N] [--policy laissez-faire|normal|vigilant] [--profile ...]");
}

pub async fn main_impl(args: &[String]) -> i32 {
    let Some((command, rest)) = args.split_first() else {
        print_help().await;
        return 2;
    };
    match command.as_str() {
        "inspect" => cmd_inspect(rest).await,
        "doc" => cmd_doc(rest).await,
        "wal-inspect" => cmd_wal_inspect(rest).await,
        "snapshot-inspect" => cmd_snapshot_inspect(rest).await,
        "verify" => cmd_verify(rest).await,
        "query" => cmd_query(rest).await,
        "replay" => cmd_replay(rest).await,
        "repair" => cmd_repair(rest).await,
        "compact" => cmd_compact(rest).await,
        "health" => cmd_health(rest).await,
        "conflict-simulate" => cmd_conflict_simulate(rest).await,
        "replica-simulate" => cmd_replica_simulate(rest).await,
        "migrate" => cmd_migrate(rest).await,
        "profile" => cmd_profile(rest).await,
        "help" | "--help" | "-h" => {
            print_help().await;
            0
        }
        other => {
            eprintln!("db: unknown subcommand '{other}'\n");
            print_help().await;
            2
        }
    }
}
//#endregion 🔖️Cli
