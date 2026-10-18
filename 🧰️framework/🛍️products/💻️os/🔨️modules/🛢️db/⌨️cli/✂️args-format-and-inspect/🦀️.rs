//! ✂️ Args, format and inspect: argv parsing, value and error formatting, the storage and worker bootstrap, health labels, `inspect` and `doc`. See [`super`].

use super::*;

//#region 🔖️Args
/// ✂️ Splits argv-style slices into positionals and `--flag value` / `--flag=value` pairs; a
/// trailing bare `--flag` with nothing after it maps to an empty-string value. Callers with any
/// no-argument boolean flags must strip those out of `args` first via `strip_flag` — this parser
/// always tries to consume the next token as a value, which would otherwise swallow a following
/// positional/flag. Mirrors `protocol_cli::parse_args`'s exact shape (same repo convention).
pub(super) async fn parse_args(args: &[String]) -> (Vec<String>, HashMap<String, String>) {
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
pub(super) async fn strip_flag(args: &[String], name: &str) -> (Vec<String>, bool) {
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

pub(super) async fn parse_profile(flags: &HashMap<String, String>) -> Result<db_policy::Profile, String> {
    match flags.get("profile").map(String::as_str) {
        None => Ok(db_policy::Profile::Dev),
        Some("test") => Ok(db_policy::Profile::Test),
        Some("dev") => Ok(db_policy::Profile::Dev),
        Some("prod") => Ok(db_policy::Profile::Prod),
        Some(other) => Err(format!("unknown --profile '{other}' (expected test|dev|prod)")),
    }
}

/// ⚖️ `--policy laissez-faire|normal|vigilant` — the authority-local `protocol::MergePolicy`
/// `db_artifact::ArtifactEngine::submit`'s outcome step judges a batch's worst graded conflict/
/// message level against (contract §C9). Replaces the deleted CRDT-era `--rule-a`/`--rule-b`/
/// `merge:<strategy>` vocabulary (C10).
pub(super) async fn parse_merge_policy(flags: &HashMap<String, String>) -> Result<protocol::MergePolicy, String> {
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
pub(super) fn hex32(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub(super) async fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64)
}

/// 🧾️ A best-effort human display of a query result's raw value bytes — UTF-8 text verbatim (the
/// common case: `db_artifact`'s path-value convention stores JSON-encoded scalars, which decode as
/// text), or a byte count for anything that doesn't decode, never a panic.
pub(super) fn describe_value_bytes(bytes: &db_query::QueryBytes) -> String {
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

pub(super) async fn fail(context: &str, err: impl std::fmt::Display) -> i32 {
    eprintln!("db: {context}: {err}");
    1
}

async fn close_wal_open_rejected(mut rejected: db_wal::ArtifactWalOpenRejected) -> db_ids::DbError {
    loop {
        match rejected.retry_close().await {
            Ok(cause) => return cause,
            Err(retained) => rejected = retained,
        }
    }
}

pub(super) async fn close_document_open_rejected(mut rejected: db_engine::DatabaseDocumentOpenRejected) -> db_ids::DbError {
    loop {
        match rejected.retry_close().await {
            Ok(cause) => return cause,
            Err(retained) => rejected = retained,
        }
    }
}

pub(super) async fn fail_wal_open(context: &str, rejected: db_wal::ArtifactWalOpenRejected) -> i32 {
    fail(context, close_wal_open_rejected(rejected).await).await
}

pub(super) async fn fail_document_open(context: &str, rejected: db_engine::DatabaseDocumentOpenRejected) -> i32 {
    fail(context, close_document_open_rejected(rejected).await).await
}

pub(super) async fn close_replication_rejected(mut rejected: db_cluster::ReplicationRejected) -> db_ids::DbError {
    loop {
        match rejected.retry_close().await {
            Ok(cause) => return cause,
            Err(retained) => rejected = retained,
        }
    }
}

pub(super) async fn usage(message: &str) -> i32 {
    eprintln!("db: {message}");
    2
}
//#endregion 🔖️Format

//#region 🔖️AsyncBridge
/// 🚀️ Opens `FsStorage` on this binary's process pool, then synchronously waits only
/// at the process entry boundary.
// 🚫️async: E5 executor bridge — `db_cli` is a single-shot, strictly-sequential process (R4
// clause 1: a binary entry point IS its own executor), so every `FsStorage` call in this file
// stays plain sync and crosses the boundary here, once, via `db_actor::block_on`.
pub(super) fn open_fs_storage(root: &Path) -> Result<db_storage_fs::FsStorage, db_ids::DbError> {
    match db_actor::block_on(db_storage_fs::FsStorage::open(cli_worker_pool(), root)) {
        Ok(storage) => Ok(storage),
        Err(mut rejected) => loop {
            match db_actor::block_on(rejected.retry_close()) {
                Ok(cause) => return Err(cause),
                Err(retained) => rejected = retained,
            }
        },
    }
}

/// 🧵️ The CLI process's one headless worker pool, shared by every database authority the
/// selected subcommand opens.
fn cli_worker_pool() -> std::sync::Arc<semio_framework_async::WorkerPool> {
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    let config = semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, cores);
    std::sync::Arc::new(semio_framework_async::process_worker_pool(config))
}

/// 🗄️ Opens a database and injects the CLI process worker pool before any authority can spawn.
pub(super) async fn open_database(root: &Path, profile: db_policy::Profile) -> Result<db_engine::Database, db_ids::DbError> {
    match db_engine::Database::open_at(cli_worker_pool(), root, profile).await {
        Ok(database) => Ok(database),
        Err(mut rejected) => loop {
            match rejected.retry_close().await {
                Ok(cause) => return Err(cause),
                Err(retained) => rejected = retained,
            }
        },
    }
}
//#endregion 🔖️AsyncBridge

//#region 🔖️Health
fn health_state_label(state: &db_observe::HealthState) -> String {
    match state {
        db_observe::HealthState::Healthy => "healthy".to_string(),
        db_observe::HealthState::Degraded(reason) => format!("degraded: {reason}"),
        db_observe::HealthState::Unhealthy(reason) => format!("unhealthy: {reason}"),
    }
}

pub(super) fn print_health(health: &db_engine::DbHealth) {
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
pub(super) async fn cmd_inspect(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest).await;
    let Some(root) = positional.first() else {
        return usage("usage: db inspect <root> [--profile test|dev|prod]").await;
    };
    let profile = match parse_profile(&flags).await {
        Ok(profile) => profile,
        Err(message) => return usage(&message).await,
    };
    let mut database = match open_database(Path::new(root), profile).await {
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

    if let Err(err) = database.shutdown(&db_engine::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await {
        return fail("shutdown", err).await;
    }
    0
}
//#endregion 🔖️Inspect

//#region 🔖️Doc
fn print_engine_frontier(frontier: &db_engine_base::Frontier) {
    println!("  head_seq: {}", frontier.head_seq);
    println!("  commit_seq: {}", frontier.commit_seq);
    println!("  epoch: {}", frontier.epoch);
    println!("  chain_hash: {}", hex32(&frontier.chain_hash));
}

/// 📄️ `db doc <root> <document-id> [--profile ...]` — opens `document` through a real
/// `ArtifactHandle` (recovering it from its WAL if not already open) and prints its current
/// frontier plus a tail of its committed history (real: `ArtifactHandle::history` replays the WAL
/// directly per `db_engine`'s own module doc).
pub(super) async fn cmd_doc(rest: &[String]) -> i32 {
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
    let mut database = match open_database(Path::new(root), profile).await {
        Ok(database) => database,
        Err(err) => return fail("open", err).await,
    };
    let document_id = protocol::ArtifactId(id.clone());
    let handle = match database.document(&document_id).await {
        Ok(handle) => handle,
        Err(err) => return fail_document_open("document", err).await,
    };

    let outcome = match handle.frontier().await {
        Ok(frontier) => {
            println!("== document {id} ==");
            print_engine_frontier(&frontier);
            match handle.history().await {
                Ok(mut history) => {
                    println!("  history entries: {}", history.entries().len());
                    for entry in history.entries().iter().rev().take(10) {
                        println!("    - operations={} head_seq={}", entry.operation_count, entry.head_seq);
                    }
                    while history.close_step() {}
                }
                Err(err) => println!("  history: unavailable ({err})"),
            }
            0
        }
        Err(err) => fail("frontier", err).await,
    };

    drop(handle);
    match database.shutdown(&db_engine::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await {
        Ok(()) => outcome,
        Err(err) => fail("shutdown", err).await,
    }
}
//#endregion 🔖️Doc
