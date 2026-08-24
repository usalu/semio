//! 🗄️ `db_cli` — the `db` binary: `inspect`/`doc`/`wal-inspect`/`snapshot-inspect`/`verify`/
//! `query`/`replay`/`repair`/`compact`/`health`/`conflict-simulate`/`replica-simulate`/`migrate`/
//! `profile` over a `db::storage::FsStorage`-rooted document store. Hand-rolled arg parsing (no
//! external CLI crate, per repo convention), exit codes `0` (success) / `1` (operation failed) /
//! `2` (usage error), never panics on corrupt input. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_cli` row).
//!
//! 🎯️ Design choice (depends on the `db` facade alone, `db_*` paths nowhere): every subcommand
//! below reaches every primitive it needs purely through `db::<submodule>::…` paths (`db::storage`,
//! `db::wal`, `db::snapshot`, `db::conflict`, `db::cluster`, `db::observe`, `db::document`,
//! `db::actor`, `db::core`, …) — the facade's own re-exports, verified complete against `db/rs/
//! lib.rs`. This crate's `Cargo.toml` accordingly depends on nothing but `db` itself plus two
//! siblings that are NOT `db_*` crates: `protocol` (every frozen `Database`/`ArtifactHandle` entry
//! point is typed against `protocol::ArtifactId`/`MutationEnvelope`/…, which the facade exposes
//! without re-exporting a path to) and `pack` (`SnapshotManager::verify`'s `VerificationLevel` is
//! pack's own type, snapshots being pack files). No `db_storage`/`db_wal`/`db_snapshot`/… path
//! dependency of its own — `wal-inspect`/`snapshot-inspect`/`replay`/`repair` still need
//! lower-level access than the actor-mediated `Database` API exposes (there is no `Database` method
//! that lists WAL segments or snapshot generations), reached the same facade-path way.
//!
//! 🎯️ Every subcommand is real, including `migrate` and `profile`: `migrate` appends a genuine
//! `WAL_MIGRATION` record via `db::wal::ArtifactWal` (force-flushed durably); `profile` submits `N`
//! real commands sequentially through `ArtifactHandle::submit` and reports wall-clock throughput —
//! both self-contained (bootstrap their own document/WAL if it doesn't exist yet), needing nothing
//! from `db_testkit` (a separate, non-`db`-facade sibling crate this one deliberately does not
//! depend on, to keep the "facade only" dependency footprint honest). `conflict-simulate` runs the
//! genuine `db::conflict::ConflictDetector` over two hand-built `CommandTouch`es (no storage
//! touched); `replica-simulate` runs the genuine `db::cluster::replicate_document` primitive between
//! two local `FsStorage` roots (no network transport exists yet in this family, which is exactly why
//! this stays a *simulation* rather than a real cluster operation).

use std::collections::HashMap;
use std::path::Path;

use crate as db;
use db::storage::{SnapshotStorage as _, WalStorage as _};

//#region 🔖️Args
/// ✂️ Splits argv-style slices into positionals and `--flag value` / `--flag=value` pairs; a
/// trailing bare `--flag` with nothing after it maps to an empty-string value. Callers with any
/// no-argument boolean flags must strip those out of `args` first via `strip_flag` — this parser
/// always tries to consume the next token as a value, which would otherwise swallow a following
/// positional/flag. Mirrors `protocol_cli::parse_args`'s exact shape (same repo convention).
async fn parse_args(args: &[String]) -> (Vec<String>, HashMap<String, String>) {
    let mut positional = Vec::new();
    let mut flags = HashMap::new();
    let mut index = 0;
    while index < args.len() {
        let arg = &args[index];
        let Some(rest) = arg.strip_prefix("--") else {
            positional.push(arg.clone());
            index += 1;
            continue;
        };
        if let Some((key, value)) = rest.split_once('=') {
            flags.insert(key.to_string(), value.to_string());
            index += 1;
        } else if index + 1 < args.len() {
            flags.insert(rest.to_string(), args[index + 1].clone());
            index += 2;
        } else {
            flags.insert(rest.to_string(), String::new());
            index += 1;
        }
    }
    (positional, flags)
}

/// ✂️ Pulls every occurrence of a no-value boolean `--<name>` flag out of `args` before the
/// generic `parse_args` runs — see `parse_args`'s doc for why that's required.
async fn strip_flag(args: &[String], name: &str) -> (Vec<String>, bool) {
    let flag = format!("--{name}");
    let mut present = false;
    let mut rest = Vec::with_capacity(args.len());
    for arg in args {
        if arg == &flag {
            present = true;
        } else {
            rest.push(arg.clone());
        }
    }
    (rest, present)
}

async fn parse_profile(flags: &HashMap<String, String>) -> Result<db::Profile, String> {
    match flags.get("profile").map(String::as_str) {
        None => Ok(db::Profile::Dev),
        Some("test") => Ok(db::Profile::Test),
        Some("dev") => Ok(db::Profile::Dev),
        Some("prod") => Ok(db::Profile::Prod),
        Some(other) => Err(format!("unknown --profile '{other}' (expected test|dev|prod)")),
    }
}

/// ⚖️ `--policy laissez-faire|normal|vigilant` — the authority-local `protocol::MergePolicy`
/// `db_artifact::ArtifactEngine::submit`'s outcome step judges a batch's worst graded conflict/
/// message level against (contract §C9). Replaces the deleted CRDT-era `--rule-a`/`--rule-b`/
/// `merge:<strategy>` vocabulary (C10).
async fn parse_merge_policy(flags: &HashMap<String, String>) -> Result<protocol::MergePolicy, String> {
    match flags.get("policy").map(String::as_str) {
        None => Ok(protocol::MergePolicy::default()),
        Some("laissez-faire") => Ok(protocol::MergePolicy::LaissezFaire),
        Some("normal") => Ok(protocol::MergePolicy::Normal),
        Some("vigilant") => Ok(protocol::MergePolicy::Vigilant),
        Some(other) => Err(format!("unknown --policy '{other}' (expected laissez-faire|normal|vigilant)")),
    }
}
//#endregion 🔖️Args

//#region 🔖️Format
fn hex32(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

async fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64)
}

/// 🧾️ A best-effort human display of a query result's raw value bytes — UTF-8 text verbatim (the
/// common case: `db_artifact`'s path-value convention stores JSON-encoded scalars, which decode as
/// text), or a byte count for anything that doesn't decode, never a panic.
fn describe_value_bytes(bytes: &db::query::QueryBytes) -> String {
    let mut text = String::with_capacity(bytes.len());
    let (mut scalar, mut minimum, mut remaining) = (0_u32, 0_u32, 0_u8);
    for fragment in bytes.fragments() {
        for byte in fragment {
            if remaining == 0 {
                match *byte {
                    0x00..=0x7f => text.push(char::from(*byte)),
                    0xc2..=0xdf => {
                        scalar = u32::from(*byte & 0x1f);
                        minimum = 0x80;
                        remaining = 1;
                    }
                    0xe0..=0xef => {
                        scalar = u32::from(*byte & 0x0f);
                        minimum = 0x800;
                        remaining = 2;
                    }
                    0xf0..=0xf4 => {
                        scalar = u32::from(*byte & 0x07);
                        minimum = 0x1_0000;
                        remaining = 3;
                    }
                    _ => return format!("<{} bytes, non-utf8>", bytes.len()),
                }
            } else if byte & 0xc0 != 0x80 {
                return format!("<{} bytes, non-utf8>", bytes.len());
            } else {
                scalar = (scalar << 6) | u32::from(*byte & 0x3f);
                remaining -= 1;
                if remaining == 0 {
                    if scalar < minimum || scalar > 0x10_ffff || (0xd800..=0xdfff).contains(&scalar) {
                        return format!("<{} bytes, non-utf8>", bytes.len());
                    }
                    text.push(char::from_u32(scalar).expect("validated unicode scalar"));
                }
            }
        }
    }
    if remaining == 0 {
        text
    } else {
        format!("<{} bytes, non-utf8>", bytes.len())
    }
}

async fn fail(context: &str, err: impl std::fmt::Display) -> i32 {
    eprintln!("db: {context}: {err}");
    1
}

async fn usage(message: &str) -> i32 {
    eprintln!("db: {message}");
    2
}
//#endregion 🔖️Format

//#region 🔖️AsyncBridge
/// @emoji 🚀️ Opens `FsStorage` on this binary's process pool, then synchronously waits only
/// at the process entry boundary.
// 🚫️async: E5 executor bridge — `db_cli` is a single-shot, strictly-sequential process (R4
// clause 1: a binary entry point IS its own executor), so every `FsStorage` call in this file
// stays plain sync and crosses the boundary here, once, via `db_actor::block_on`.
fn open_fs_storage(root: &Path) -> Result<db::storage::FsStorage, db::db_ids::DbError> {
    db::actor::block_on(db::storage::FsStorage::open(cli_worker_pool(), root))
}

/// 🧵️ The CLI process's one headless worker pool, shared by every database authority the
/// selected subcommand opens.
fn cli_worker_pool() -> std::sync::Arc<db::semio_framework_async::WorkerPool> {
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    let config = db::semio_framework_async::WorkerPoolConfig::new(db::semio_framework_async::ProcessKind::HeadlessBatch, cores);
    std::sync::Arc::new(db::semio_framework_async::process_worker_pool(config))
}

/// 🗄️ Opens a database and injects the CLI process worker pool before any authority can spawn.
async fn open_database(root: &Path, profile: db::Profile) -> Result<db::Database, db::db_ids::DbError> {
    db::Database::open_at(cli_worker_pool(), root, profile).await
}
//#endregion 🔖️AsyncBridge

//#region 🔖️Health
fn health_state_label(state: &db::observe::HealthState) -> String {
    match state {
        db::observe::HealthState::Healthy => "healthy".to_string(),
        db::observe::HealthState::Degraded(reason) => format!("degraded: {reason}"),
        db::observe::HealthState::Unhealthy(reason) => format!("unhealthy: {reason}"),
    }
}

fn print_health(health: &db::DbHealth) {
    println!("== health ==");
    println!("  overall: {}", health_state_label(&health.report.overall));
    println!("  open_artifacts: {}", health.open_artifacts);
    for (name, state) in &health.report.components {
        println!("  - {name}: {}", health_state_label(state));
    }
}
//#endregion 🔖️Health

//#region 🔖️Inspect
/// 📇️ `db inspect <root> [--profile test|dev|prod]` — opens (zero-touch, creating if absent) the
/// `Database` at `root` and prints its catalog plus a health snapshot.
async fn cmd_inspect(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest).await;
    let Some(root) = positional.first() else {
        return usage("usage: db inspect <root> [--profile test|dev|prod]").await;
    };
    let profile = match parse_profile(&flags).await {
        Ok(profile) => profile,
        Err(message) => return usage(&message).await,
    };
    let database = match open_database(Path::new(root), profile).await {
        Ok(database) => database,
        Err(err) => return fail("open", err).await,
    };

    let catalog = database.catalog().await;
    println!("== catalog: {root} ==");
    println!("  documents: {}", catalog.artifacts.len());
    for entry in &catalog.artifacts {
        println!("  - {} (created_at_ms={})", entry.document.0, entry.created_at_ms);
    }
    print_health(&database.health().await);

    if let Err(err) = database.shutdown(std::time::Duration::from_secs(5)).await {
        return fail("shutdown", err).await;
    }
    0
}
//#endregion 🔖️Inspect

//#region 🔖️Doc
fn print_engine_frontier(frontier: &db::db_engine::Frontier) {
    println!("  head_seq: {}", frontier.head_seq);
    println!("  commit_seq: {}", frontier.commit_seq);
    println!("  epoch: {}", frontier.epoch);
    println!("  chain_hash: {}", hex32(&frontier.chain_hash));
}

/// 📄️ `db doc <root> <document-id> [--profile ...]` — opens `document` through a real
/// `ArtifactHandle` (recovering it from its WAL if not already open) and prints its current
/// frontier plus a tail of its committed history (real: `ArtifactHandle::history` replays the WAL
/// directly per `db_engine`'s own module doc).
async fn cmd_doc(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest).await;
    if positional.len() < 2 {
        return usage("usage: db doc <root> <document-id> [--profile test|dev|prod]").await;
    }
    let root = &positional[0];
    let id = &positional[1];
    let profile = match parse_profile(&flags).await {
        Ok(profile) => profile,
        Err(message) => return usage(&message).await,
    };
    let database = match open_database(Path::new(root), profile).await {
        Ok(database) => database,
        Err(err) => return fail("open", err).await,
    };
    let document_id = protocol::ArtifactId(id.clone());
    let handle = match database.document(&document_id).await {
        Ok(handle) => handle,
        Err(err) => return fail("document", err).await,
    };

    let outcome = match handle.frontier().await {
        Ok(frontier) => {
            println!("== document {id} ==");
            print_engine_frontier(&frontier);
            match handle.history().await {
                Ok(history) => {
                    println!("  history entries: {}", history.entries().len());
                    for entry in history.entries().iter().rev().take(10) {
                        println!("    - operations={} head_seq={}", entry.operation_count, entry.head_seq);
                    }
                }
                Err(err) => println!("  history: unavailable ({err})"),
            }
            0
        }
        Err(err) => fail("frontier", err).await,
    };

    match database.shutdown(std::time::Duration::from_secs(5)).await {
        Ok(()) => outcome,
        Err(err) => fail("shutdown", err).await,
    }
}
//#endregion 🔖️Doc

//#region 🔖️WalInspect
fn wal_cursor_control() -> Result<db::wal::WalCursorControl, db::DbError> {
    db::wal::WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)
}

async fn admit_cli_wal_bytes(source: Vec<u8>, maximum: u64, control: &mut db::wal::WalCursorControl) -> Result<db::wal::WalBytes, db::DbError> {
    match db::wal::WalBytes::try_admit(source, maximum, control).await {
        Ok(bytes) => Ok(bytes),
        Err(mut rejected) => {
            while rejected.close_step()? {
                control.grant()?;
            }
            Err(rejected.into_error())
        }
    }
}

fn wal_record_kind_name(record: &db::wal::WalRecord) -> &'static str {
    match record {
        db::wal::WalRecord::SegmentHeader { .. } => "segment_header",
        db::wal::WalRecord::TxBegin { .. } => "tx_begin",
        db::wal::WalRecord::TxCommit { .. } => "tx_commit",
        db::wal::WalRecord::TxAbort { .. } => "tx_abort",
        db::wal::WalRecord::Command(_) => "command",
        db::wal::WalRecord::Payload(_) => "payload",
        db::wal::WalRecord::Diff(_) => "diff",
        db::wal::WalRecord::Inverse(_) => "inverse",
        db::wal::WalRecord::Event(_) => "event",
        db::wal::WalRecord::Outbox(_) => "outbox",
        db::wal::WalRecord::Frontier(_) => "frontier",
        db::wal::WalRecord::VcsRef(_) => "vcs_ref",
        db::wal::WalRecord::SnapshotPub { .. } => "snapshot_pub",
        db::wal::WalRecord::IndexCkpt { .. } => "index_ckpt",
        db::wal::WalRecord::Lease { .. } => "lease",
        db::wal::WalRecord::Migration(_) => "migration",
    }
}

fn describe_wal_record(record: &db::wal::WalRecord) -> String {
    let kind = wal_record_kind_name(record);
    match record {
        db::wal::WalRecord::SegmentHeader { document, segment_index, prev_chain_hash } => {
            format!("{kind} document={document} segment_index={segment_index} prev_chain_hash={}", prev_chain_hash.map_or_else(|| "-".to_string(), |hash| hex32(&hash)))
        }
        db::wal::WalRecord::TxBegin { tx_id } => format!("{kind} tx_id={tx_id}"),
        db::wal::WalRecord::TxCommit { tx_id, record_count } => format!("{kind} tx_id={tx_id} record_count={record_count}"),
        db::wal::WalRecord::TxAbort { tx_id } => format!("{kind} tx_id={tx_id}"),
        db::wal::WalRecord::Command(bytes) => format!("{kind} bytes={}", bytes.len()),
        db::wal::WalRecord::Payload(db::wal::WalPayloadRef::Inline(bytes)) => format!("{kind} inline bytes={}", bytes.len()),
        db::wal::WalRecord::Payload(db::wal::WalPayloadRef::CasRef(hash)) => format!("{kind} cas_ref hash={}", hex32(&hash.0)),
        db::wal::WalRecord::Diff(bytes) => format!("{kind} bytes={}", bytes.len()),
        db::wal::WalRecord::Inverse(bytes) => format!("{kind} bytes={}", bytes.len()),
        db::wal::WalRecord::Event(bytes) => format!("{kind} bytes={}", bytes.len()),
        db::wal::WalRecord::Outbox(bytes) => format!("{kind} bytes={}", bytes.len()),
        db::wal::WalRecord::Frontier(frontier) => {
            format!("{kind} head_seq={} commit_seq={} epoch={} chain_hash={}", frontier.head_seq, frontier.commit_seq, frontier.epoch, hex32(&frontier.chain_hash))
        }
        db::wal::WalRecord::VcsRef(id) => format!("{kind} id={}", id.as_str()),
        db::wal::WalRecord::SnapshotPub { generation, frontier } => format!("{kind} generation={generation} head_seq={}", frontier.head_seq),
        db::wal::WalRecord::IndexCkpt { run_ids } => format!("{kind} run_ids={}", run_ids.len()),
        db::wal::WalRecord::Lease { resource, holder, fence, expires_at_ms } => format!("{kind} resource={} holder={} fence={fence} expires_at_ms={expires_at_ms}", resource.as_str(), holder.as_str()),
        db::wal::WalRecord::Migration(bytes) => format!("{kind} bytes={}", bytes.len()),
    }
}

/// 📼️ `db wal-inspect <root> <document-id> [--limit N]` — lists `document`'s raw WAL segments
/// (index + byte length, straight from `WalStorage`, before any decoding) then decodes every
/// `WAL_*` record via `db::wal::replay_document`. A torn/corrupt WAL is reported honestly (with a
/// hint pointing at `db repair`), never panics.
async fn cmd_wal_inspect(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest).await;
    if positional.len() < 2 {
        return usage("usage: db wal-inspect <root> <document-id> [--limit N]").await;
    }
    let root = &positional[0];
    let id = &positional[1];
    let limit = match flags.get("limit").map(|value| value.parse::<usize>()) {
        None => None,
        Some(Ok(limit)) => Some(limit),
        Some(Err(_)) => return usage("db wal-inspect: --limit must be a non-negative integer").await,
    };

    let storage = match open_fs_storage(Path::new(root)) {
        Ok(storage) => storage,
        Err(err) => return fail("open", err).await,
    };
    let document = db::db_ids::ArtifactId(id.clone());

    let mut segments = match db::actor::block_on(storage.list_segments(&document)) {
        Ok(segments) => segments,
        Err(err) => return fail("list_segments", err).await,
    };
    println!("== wal segments: {} ==", segments.len());
    for index in &segments {
        match db::actor::block_on(storage.segment_len(&document, *index)) {
            Ok(len) => println!("  segment {index}: {len} bytes"),
            Err(err) => println!("  segment {index}: <error reading length: {err}>"),
        }
    }
    while segments.close_step() {}

    let control = match wal_cursor_control() {
        Ok(control) => control,
        Err(err) => return fail("wal cursor", err).await,
    };
    match db::wal::replay_document(&storage, &document, control).await {
        Ok(mut records) => {
            println!("== wal records ==");
            let mut count = 0usize;
            let mut shown = 0usize;
            loop {
                match records.next().await {
                    Ok(Some(mut record)) => {
                        if limit.is_none_or(|limit| shown < limit) {
                            println!("  [{count}] {}", describe_wal_record(&record));
                            shown += 1;
                        }
                        count += 1;
                        loop {
                            match record.close_step() {
                                Ok(true) => {}
                                Ok(false) => break,
                                Err(err) => return fail("record close", err).await,
                            }
                        }
                    }
                    Ok(None) => break,
                    Err(err) => return fail("replay", err).await,
                }
            }
            loop {
                match records.close_step().await {
                    Ok(true) => {}
                    Ok(false) => break,
                    Err(err) => return fail("replay close", err).await,
                }
            }
            if shown < count {
                println!("  ... {} more (pass --limit to see more)", count - shown);
            }
            0
        }
        Err(err) => {
            println!("== wal records: unavailable ==");
            println!("  {err}");
            println!("  (hint: run `db repair {root} {id}` to discard a torn tail)");
            1
        }
    }
}
//#endregion 🔖️WalInspect

//#region 🔖️SnapshotInspect
/// 📸️ `db snapshot-inspect <root> <document-id> [--generation N] [--verify]` — lists every stored
/// generation, then decodes one (the latest, or `--generation N`) via `db::snapshot::open_latest`
/// over `SnapshotManager`'s retained lineage cursor, printing its descriptor.
/// `--verify` additionally runs `SnapshotManager::verify` at `VerificationLevel::Full`.
async fn cmd_snapshot_inspect(rest: &[String]) -> i32 {
    let (rest, verify) = strip_flag(rest, "verify").await;
    let (positional, flags) = parse_args(&rest).await;
    if positional.len() < 2 {
        return usage("usage: db snapshot-inspect <root> <document-id> [--generation N] [--verify]").await;
    }
    let root = &positional[0];
    let id = &positional[1];

    let storage = match open_fs_storage(Path::new(root)) {
        Ok(storage) => storage,
        Err(err) => return fail("open", err).await,
    };
    let document = db::db_ids::ArtifactId(id.clone());

    let mut generations = match db::actor::block_on(storage.list_generations(&document)) {
        Ok(generations) => generations,
        Err(err) => return fail("list_generations", err).await,
    };
    println!("== snapshot generations: {} ==", generations.len());
    for generation in &generations {
        println!("  - {generation}");
    }
    let latest = generations.last().copied();
    while generations.close_step() {}
    let Some(latest) = latest else {
        return 0;
    };

    let generation = match flags.get("generation").map(|value| value.parse::<u64>()) {
        None => latest,
        Some(Ok(generation)) => generation,
        Some(Err(_)) => return usage("db snapshot-inspect: --generation must be a non-negative integer").await,
    };

    let manager = db::snapshot::SnapshotManager::new(&storage).await;
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = match db::snapshot::SnapshotCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 8_192) {
        Ok(control) => control,
        Err(err) => return fail("snapshot_cursor", err).await,
    };
    let mut cursor = manager.chain_cursor(&document, generation, control);
    let descriptor = match db::actor::block_on(cursor.latest_descriptor()) {
        Ok(descriptor) => descriptor,
        Err(err) => return fail("snapshot_cursor", err).await,
    };
    println!("== generation {generation} descriptor ==");
    println!("  parent_generation: {}", descriptor.parent_generation.map_or_else(|| "-".to_string(), |generation| generation.to_string()));
    println!("  head_seq: {}", descriptor.head_seq);
    println!("  commit_seq: {}", descriptor.commit_seq);
    println!("  epoch: {}", descriptor.epoch);
    println!("  chain_hash: {}", hex32(&descriptor.chain_hash));
    println!("  protocol_version: {}", descriptor.protocol_version);
    println!("  vcs_head: {}", descriptor.vcs_head.as_deref().unwrap_or("-"));
    println!("  base_pack_hash: {}", descriptor.base_pack_hash.map_or_else(|| "-".to_string(), |hash| hex32(&hash.0)));
    println!("  roots: {}", descriptor.roots.len());
    println!("  new_pages: {}", descriptor.new_pages.len());
    println!("  created_at_ms: {}", descriptor.created_at_ms);
    loop {
        match cursor.close_step() {
            Ok(true) => {}
            Ok(false) => break,
            Err(err) => return fail("snapshot cursor close", err).await,
        }
    }

    if verify {
        match db::actor::block_on(manager.verify(&document, generation, pack::os_pack::VerificationLevel::Full)) {
            Ok(()) => println!("== verify: OK =="),
            Err(err) => {
                println!("== verify: FAIL ==");
                println!("  {err}");
                return 1;
            }
        }
    }
    0
}
//#endregion 🔖️SnapshotInspect

//#region 🔖️Verify
/// 🔬️ The shared per-document check `verify` runs: a full WAL replay (rejects a torn tail) plus,
/// if a snapshot exists, a full-level `SnapshotManager::verify` of its latest generation.
async fn verify_document(storage: &db::storage::FsStorage, document: &db::db_ids::ArtifactId) -> Result<String, db::DbError> {
    let mut records = db::wal::replay_document(storage, document, wal_cursor_control()?).await?;
    let mut record_count = 0usize;
    while let Some(mut record) = records.next().await? {
        record_count += 1;
        while record.close_step()? {}
    }
    while records.close_step().await? {}
    let manager = db::snapshot::SnapshotManager::new(storage).await;
    match db::actor::block_on(manager.load_latest(document))? {
        Some((generation, _descriptor)) => {
            db::actor::block_on(manager.verify(document, generation, pack::os_pack::VerificationLevel::Full))?;
            Ok(format!("wal records={record_count} snapshot generation={generation} (verified)"))
        }
        None => Ok(format!("wal records={record_count} snapshot=none")),
    }
}

/// 🔬️ `db verify <root> [document-id] [--profile ...]` — verifies one document, or every document
/// in the catalog if none is given. Prints `OK <id>: <summary>` / `FAIL <id>: <reason>` per
/// document, never panics on corrupt input; exits `1` iff any document failed.
async fn cmd_verify(rest: &[String]) -> i32 {
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
            let database = match open_database(Path::new(root), profile).await {
                Ok(database) => database,
                Err(err) => return fail("open", err).await,
            };
            let ids = database.catalog().await.artifacts.iter().map(|entry| entry.document.0.clone()).collect();
            if let Err(err) = database.shutdown(std::time::Duration::from_secs(5)).await {
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
        let document = db::db_ids::ArtifactId(id.clone());
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
async fn cmd_query(rest: &[String]) -> i32 {
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
    let database = match open_database(Path::new(root), profile).await {
        Ok(database) => database,
        Err(err) => return fail("open", err).await,
    };
    let document_id = protocol::ArtifactId(id.clone());
    let handle = match database.document(&document_id).await {
        Ok(handle) => handle,
        Err(err) => return fail("document", err).await,
    };

    let query = match paths.len() {
        1 => db::Query::Get { path: paths[0].clone() },
        _ => db::Query::GetMany { paths },
    };
    let outcome = match handle.query(query, db::Consistency::Canonical).await {
        Ok(mut stream) => {
            for entry in stream.iter() {
                match entry.value() {
                    Some(bytes) => println!("{} = {}", entry.path(), describe_value_bytes(bytes)),
                    None => println!("{} = <unset>", entry.path()),
                }
            }
            match (|| -> Result<(), db::DbError> {
                while stream.close_step()? {}
                Ok(())
            })() {
                Ok(()) => 0,
                Err(error) => fail("query close", error).await,
            }
        }
        Err(err) => fail("query", err).await,
    };

    match database.shutdown(std::time::Duration::from_secs(5)).await {
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
async fn cmd_replay(rest: &[String]) -> i32 {
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
    let document = db::db_ids::ArtifactId(id.clone());
    let control = match wal_cursor_control() {
        Ok(control) => control,
        Err(err) => return fail("wal cursor", err).await,
    };
    let mut records = match db::wal::replay_document(&storage, &document, control).await {
        Ok(records) => records,
        Err(err) => return fail("replay", err).await,
    };

    let mut kind_counts: std::collections::BTreeMap<&'static str, usize> = std::collections::BTreeMap::new();
    let mut last_frontier: Option<db::db_durability::Frontier> = None;
    let mut record_count = 0usize;
    loop {
        let mut record = match records.next().await {
            Ok(Some(record)) => record,
            Ok(None) => break,
            Err(err) => return fail("replay", err).await,
        };
        record_count += 1;
        *kind_counts.entry(wal_record_kind_name(&record)).or_insert(0) += 1;
        if let db::wal::WalRecord::Frontier(frontier) = &record {
            last_frontier = Some(frontier.clone());
        }
        loop {
            match record.close_step() {
                Ok(true) => {}
                Ok(false) => break,
                Err(err) => return fail("record close", err).await,
            }
        }
    }
    loop {
        match records.close_step().await {
            Ok(true) => {}
            Ok(false) => break,
            Err(err) => return fail("replay close", err).await,
        }
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
async fn cmd_repair(rest: &[String]) -> i32 {
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
    let document = db::db_ids::ArtifactId(id.clone());
    match db::actor::block_on(db::wal::ArtifactWal::open(&storage, document, db::wal::GroupCommitPolicy::default(), now_ms().await)) {
        Ok((_wal, report)) => {
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
        Err(err) => fail("repair", err).await,
    }
}
//#endregion 🔖️Repair

//#region 🔖️Compact
/// 🧹️ `db compact <root> <document-id> [--holder H] [--consolidate] [--profile ...]` — drives a
/// real, fenced `db_compact::Compactor` pass via `Database::compact_document`.
async fn cmd_compact(rest: &[String]) -> i32 {
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
    let database = match open_database(Path::new(root), profile).await {
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
        Err(err) => fail("compact", err).await,
    };

    match database.shutdown(std::time::Duration::from_secs(5)).await {
        Ok(()) => outcome,
        Err(err) => fail("shutdown", err).await,
    }
}
//#endregion 🔖️Compact

//#region 🔖️HealthCmd
/// 🩺️ `db health <root> [--profile ...]` — a real `Database::health()` snapshot. Exits `1` iff the
/// overall status is `Unhealthy`.
async fn cmd_health(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest).await;
    let Some(root) = positional.first() else {
        return usage("usage: db health <root> [--profile test|dev|prod]").await;
    };
    let profile = match parse_profile(&flags).await {
        Ok(profile) => profile,
        Err(message) => return usage(&message).await,
    };
    let database = match open_database(Path::new(root), profile).await {
        Ok(database) => database,
        Err(err) => return fail("open", err).await,
    };
    let health = database.health().await;
    print_health(&health);
    let exit = if matches!(health.report.overall, db::observe::HealthState::Unhealthy(_)) { 1 } else { 0 };

    match database.shutdown(std::time::Duration::from_secs(5)).await {
        Ok(()) => exit,
        Err(err) => fail("shutdown", err).await,
    }
}
//#endregion 🔖️HealthCmd

//#region 🔖️ConflictSimulate
fn describe_conflict_kind(kind: &db::conflict::ConflictKind) -> String {
    match kind {
        db::conflict::ConflictKind::TouchedRegion(regions) => format!("touched_region({})", regions.iter().map(|region| region.path.as_str()).collect::<Vec<_>>().join(",")),
        db::conflict::ConflictKind::Constraint(description) => format!("constraint({description})"),
    }
}

fn touched_command(command_id: &str, actor: &str, kind: &str, hlc_actor: u64, paths: &str) -> db::conflict::CommandTouch {
    let timestamp = db::actor::block_on(async { protocol::HybridLogicalTimestamp::new(hlc_actor, now_ms().await) });
    let touch = db::conflict::CommandTouch::new(protocol::MutationId(command_id.to_string()), protocol::ActorId(actor.to_string()), db::conflict::CommandKind::from(kind), timestamp);
    paths.split(',').map(str::trim).filter(|path| !path.is_empty()).fold(touch, |touch, path| touch.touch(db::state::TouchedRegion::write(path)))
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
    let records = db::conflict::ConflictDetector::new().detect(&[command_a, command_b]);
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
        Ok(storage) => db::storage::DbBackend::Fs(storage),
        Err(err) => return fail("open leader", err).await,
    };
    let follower = match open_fs_storage(Path::new(follower_root)) {
        Ok(storage) => db::storage::DbBackend::Fs(storage),
        Err(err) => return fail("open follower", err).await,
    };
    let document = db::db_ids::ArtifactId(id.clone());

    match db::actor::block_on(db::cluster::replicate_document(&leader, &follower, document, db::wal::GroupCommitPolicy::default(), now_ms().await)) {
        Ok(db::cluster::ReplicationOutcome::UpToDate { frontier }) => {
            println!("== replica-simulate: up to date ==");
            println!("  head_seq: {}", frontier.head_seq);
            0
        }
        Ok(db::cluster::ReplicationOutcome::TailApplied { frontier, count }) => {
            println!("== replica-simulate: tail applied ==");
            println!("  commands: {count}");
            println!("  head_seq: {}", frontier.head_seq);
            0
        }
        Ok(db::cluster::ReplicationOutcome::SnapshotTransferred { generation, pack_hash }) => {
            println!("== replica-simulate: snapshot transferred ==");
            println!("  generation: {generation}");
            println!("  pack_hash: {}", hex32(&pack_hash));
            0
        }
        Err(err) => fail("replicate", err).await,
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
    let document = db::db_ids::ArtifactId(id.clone());
    let now = now_ms().await;
    let (mut wal, _report) = match db::actor::block_on(db::wal::ArtifactWal::open(&storage, document, db::wal::GroupCommitPolicy::default(), now)) {
        Ok(pair) => pair,
        Err(err) => return fail("open wal", err).await,
    };
    let bytes = format!("{name}\n{payload}").into_bytes();
    let mut control = match wal_cursor_control() {
        Ok(control) => control,
        Err(err) => return fail("wal cursor", err).await,
    };
    let bytes = match admit_cli_wal_bytes(bytes, 1024 * 1024, &mut control).await {
        Ok(bytes) => bytes,
        Err(err) => return fail("wal migration admission", err).await,
    };
    let mut records = db::wal::WalRecordBatch::new();
    if records.push(db::wal::WalRecord::Migration(bytes)).is_err() {
        return fail("wal migration", db::DbError::LimitExceeded("cli wal record batch")).await;
    }
    match wal.submit(&storage, &records, db::DurabilityClass::Fsync, now).await {
        Ok(receipt) => {
            loop {
                match records.close_step() {
                    Ok(true) => {}
                    Ok(false) => break,
                    Err(err) => return fail("wal migration close", err).await,
                }
            }
            if let Err(err) = db::actor::block_on(wal.force_flush(&storage)) {
                return fail("flush", err).await;
            }
            println!("== migrate: {id} ==");
            println!("  name: {name}");
            println!("  segment_index: {}", receipt.segment_index);
            println!("  tx_id: {}", receipt.tx_id);
            println!("  committed: {}", receipt.committed);
            0
        }
        Err(err) => fail("migrate", err).await,
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
/// `std::time::Instant` rather than pulling in `db_testkit`'s `WorkloadGen`/criterion harness:
/// `db_testkit` is a sibling crate, not part of the `db` facade's own re-export surface, and this
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
        None => db::DurabilityClass::Fsync,
        Some("memory") => db::DurabilityClass::Memory,
        Some("os") => db::DurabilityClass::Os,
        Some("fsync") => db::DurabilityClass::Fsync,
        Some(other) => match other.strip_prefix("quorum:") {
            Some(n) => match n.parse::<u8>() {
                Ok(n) => db::DurabilityClass::Quorum(n),
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
    let database = match open_database(Path::new(root), profile).await {
        Ok(database) => database,
        Err(err) => return fail("open", err).await,
    };
    let document_id = protocol::ArtifactId(id.clone());
    let handle = match database.document(&document_id).await {
        Ok(handle) => handle,
        Err(_) => match database.create_document(db::ArtifactSpec::new(document_id.clone()).await).await {
            Ok(handle) => handle,
            Err(err) => return fail("create", err).await,
        },
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
            actor: protocol::ActorId("profiler".to_string()),
            dependencies: Vec::new(),
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: db::document::encode_pathmap_json(&serde_json::Value::Object(forward)).await.unwrap_or_default() },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: db::document::encode_pathmap_json(&serde_json::Value::Object(backward)).await.unwrap_or_default() },
            timestamp: protocol::HybridLogicalTimestamp::new(0, now_ms().await),
        };
        let batch = match db::document::CommandBatch::new(vec![envelope]).await {
            Ok(batch) => batch,
            Err(err) => return fail("build batch", err).await,
        };
        match db::actor::block_on(handle.submit(batch, db::document::SubmitOptions { durability, policy })) {
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

    match database.shutdown(std::time::Duration::from_secs(5)).await {
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧸️Fixtures
    async fn tempdir(name: &str) -> std::path::PathBuf {
        let mut dir = std::env::temp_dir();
        dir.push(format!("db_cli-test-{name}-{}-{}", std::process::id(), now_ms().await));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    async fn test_envelope(id: &str, document: &protocol::ArtifactId) -> protocol::MutationEnvelope {
        protocol::MutationEnvelope {
            mutation_id: protocol::MutationId(id.to_string()),
            document_id: document.clone(),
            actor: protocol::ActorId("tester".to_string()),
            dependencies: Vec::new(),
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: db::document::encode_pathmap_json(&serde_json::json!({"greeting": "hello"})).await.unwrap() },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: db::document::encode_pathmap_json(&serde_json::json!({"greeting": null})).await.unwrap() },
            timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        }
    }

    /// 🌱️ Seeds `doc-1` at `root` with one committed, `Fsync`-durable transaction through the real
    /// `Database::create_document`/`ArtifactHandle::submit` round trip, then cleanly shuts down.
    async fn seed_document(root: &Path) {
        let database = open_database(root, db::Profile::Test).await.unwrap();
        let document = protocol::ArtifactId("doc-1".to_string());
        let handle = database.create_document(db::ArtifactSpec::new(document.clone()).await).await.unwrap();
        let batch = db::document::CommandBatch::new(vec![test_envelope("op-1", &document).await]).await.unwrap();
        db::actor::block_on(handle.submit(batch, db::document::SubmitOptions { durability: db::DurabilityClass::Fsync, ..Default::default() })).unwrap().unwrap();
        database.shutdown(std::time::Duration::from_secs(1)).await.unwrap();
    }
    //#endregion 🧸️Fixtures

    //#region 🔖️Inspect
    #[semio_framework_async_macros::async_test]
    async fn cli_inspect_reports_an_empty_catalog_and_healthy_status_on_a_fresh_root() {
        let root = tempdir("inspect-fresh").await;
        assert_eq!(main_impl(&[String::from("inspect"), root.to_string_lossy().to_string()]).await, 0);
    }
    //#endregion 🔖️Inspect

    //#region 🔖️FullCycle
    #[semio_framework_async_macros::async_test]
    async fn cli_full_cycle_succeeds_for_a_seeded_document() {
        let root = tempdir("full-cycle").await;
        seed_document(&root).await;
        let root_str = root.to_string_lossy().to_string();

        assert_eq!(main_impl(&[String::from("doc"), root_str.clone(), String::from("doc-1")]).await, 0);
        assert_eq!(main_impl(&[String::from("query"), root_str.clone(), String::from("doc-1"), String::from("greeting")]).await, 0);
        assert_eq!(main_impl(&[String::from("wal-inspect"), root_str.clone(), String::from("doc-1")]).await, 0);
        assert_eq!(main_impl(&[String::from("replay"), root_str.clone(), String::from("doc-1")]).await, 0);
        assert_eq!(main_impl(&[String::from("verify"), root_str.clone(), String::from("doc-1")]).await, 0);
        assert_eq!(main_impl(&[String::from("verify"), root_str.clone()]).await, 0);
        assert_eq!(main_impl(&[String::from("repair"), root_str.clone(), String::from("doc-1")]).await, 0);
        assert_eq!(main_impl(&[String::from("compact"), root_str.clone(), String::from("doc-1"), String::from("--consolidate")]).await, 0);
        assert_eq!(main_impl(&[String::from("health"), root_str.clone()]).await, 0);
        assert_eq!(main_impl(&[String::from("snapshot-inspect"), root_str, String::from("doc-1")]).await, 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn cli_doc_and_query_err_cleanly_on_an_unknown_document() {
        let root = tempdir("unknown-doc").await;
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("doc"), root_str.clone(), String::from("never-created")]).await, 1);
        assert_eq!(main_impl(&[String::from("query"), root_str, String::from("never-created"), String::from("x")]).await, 1);
    }
    //#endregion 🔖️FullCycle

    //#region 🔖️Verify
    #[semio_framework_async_macros::async_test]
    async fn cli_verify_fails_on_a_torn_wal_tail_and_repair_fixes_it() {
        let root = tempdir("torn-tail").await;
        seed_document(&root).await;

        let wal_dir = root.join("wal").join("doc-1");
        let segment_path = std::fs::read_dir(&wal_dir).unwrap().filter_map(|entry| entry.ok()).map(|entry| entry.path()).find(|path| path.extension().is_some_and(|ext| ext == "bin")).expect("expected at least one wal segment file");
        let mut bytes = std::fs::read(&segment_path).unwrap();
        assert!(bytes.len() > 16, "segment must be large enough to truncate meaningfully");
        bytes.truncate(bytes.len() - 5);
        std::fs::write(&segment_path, &bytes).unwrap();

        let root_str = root.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("verify"), root_str.clone(), String::from("doc-1")]).await, 1);
        assert_eq!(main_impl(&[String::from("wal-inspect"), root_str.clone(), String::from("doc-1")]).await, 1);
        assert_eq!(main_impl(&[String::from("repair"), root_str.clone(), String::from("doc-1")]).await, 0);
        assert_eq!(main_impl(&[String::from("verify"), root_str, String::from("doc-1")]).await, 0);
    }
    //#endregion 🔖️Verify

    //#region 🔖️ConflictSimulate
    #[semio_framework_async_macros::async_test]
    async fn cli_conflict_simulate_detects_overlapping_writes_and_ignores_disjoint_ones() {
        assert_eq!(main_impl(&[String::from("conflict-simulate"), String::from("--touch-a"), String::from("a/name"), String::from("--touch-b"), String::from("a/name")]).await, 1);
        assert_eq!(main_impl(&[String::from("conflict-simulate"), String::from("--touch-a"), String::from("a/name"), String::from("--touch-b"), String::from("b/name")]).await, 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn cli_conflict_simulate_requires_both_touch_flags() {
        assert_eq!(main_impl(&[String::from("conflict-simulate")]).await, 2);
        assert_eq!(main_impl(&[String::from("conflict-simulate"), String::from("--touch-a"), String::from("a")]).await, 2);
    }
    //#endregion 🔖️ConflictSimulate

    //#region 🔖️ReplicaSimulate
    #[semio_framework_async_macros::async_test]
    async fn cli_replica_simulate_copies_missing_commands_to_a_fresh_follower() {
        let leader_root = tempdir("replica-leader").await;
        let follower_root = tempdir("replica-follower").await;
        seed_document(&leader_root).await;

        let leader_str = leader_root.to_string_lossy().to_string();
        let follower_str = follower_root.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("replica-simulate"), leader_str, follower_str.clone(), String::from("doc-1")]).await, 0);
        assert_eq!(main_impl(&[String::from("verify"), follower_str, String::from("doc-1")]).await, 0);
    }
    //#endregion 🔖️ReplicaSimulate

    //#region 🔖️Migrate
    #[semio_framework_async_macros::async_test]
    async fn cli_migrate_appends_a_migration_record_visible_to_wal_inspect() {
        let root = tempdir("migrate").await;
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("migrate"), root_str.clone(), String::from("doc-1"), String::from("rename-field"), String::from("--payload"), String::from("old->new")]).await, 0);
        assert_eq!(main_impl(&[String::from("wal-inspect"), root_str, String::from("doc-1")]).await, 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn cli_migrate_reports_a_usage_error_with_too_few_args() {
        assert_eq!(main_impl(&[String::from("migrate"), String::from("root-only")]).await, 2);
    }
    //#endregion 🔖️Migrate

    //#region 🔖️Profile
    #[semio_framework_async_macros::async_test]
    async fn cli_profile_reports_throughput_for_n_commands_on_a_fresh_document() {
        let root = tempdir("profile").await;
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("profile"), root_str.clone(), String::from("doc-1"), String::from("--commands"), String::from("5"), String::from("--durability"), String::from("memory")]).await, 0);
        // 🎯️ The 5 profiled commands are real, durable commits — verified via the query subcommand
        // rather than parsing this test's own stdout (`println!` isn't easily captured in-process).
        assert_eq!(main_impl(&[String::from("query"), root_str, String::from("doc-1"), String::from("cli/profile/counter")]).await, 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn cli_profile_rejects_a_bad_durability_flag() {
        let root = tempdir("profile-bad-durability").await;
        let root_str = root.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("profile"), root_str, String::from("doc-1"), String::from("--durability"), String::from("bogus")]).await, 2);
    }
    //#endregion 🔖️Profile

    //#region 🔖️Cli
    #[semio_framework_async_macros::async_test]
    async fn cli_help_and_unknown_subcommand() {
        assert_eq!(main_impl(&[]).await, 2);
        assert_eq!(main_impl(&[String::from("help")]).await, 0);
        assert_eq!(main_impl(&[String::from("--help")]).await, 0);
        assert_eq!(main_impl(&[String::from("bogus-subcommand")]).await, 2);
    }
    //#endregion 🔖️Cli
}
//#endregion 🧪️Tests
