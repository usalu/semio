//! 📜️ WAL and snapshot inspect: the WAL cursor and record description plus `wal-inspect` and `snapshot-inspect`. See [`super`].

use super::*;

//#region 🔖️WalInspect
pub(super) fn wal_cursor_control() -> Result<db_wal_log::WalCursorControl, db_ids::DbError> {
    db_wal_log::WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)
}

pub(super) async fn admit_cli_wal_bytes(source: Vec<u8>, maximum: u64, control: &mut db_wal_log::WalCursorControl) -> Result<db_wal_log::WalBytes, db_ids::DbError> {
    match db_wal_log::WalBytes::try_admit(source, maximum, control).await {
        Ok(bytes) => Ok(bytes),
        Err(mut rejected) => {
            control.grant()?;
            let _ = rejected.close_step()?;
            Err(rejected.into_error())
        }
    }
}

pub(super) fn wal_record_kind_name(record: &db_wal_log::WalRecord) -> &'static str {
    match record {
        db_wal_log::WalRecord::SegmentHeader { .. } => "segment_header",
        db_wal_log::WalRecord::TxBegin { .. } => "tx_begin",
        db_wal_log::WalRecord::TxCommit { .. } => "tx_commit",
        db_wal_log::WalRecord::TxAbort { .. } => "tx_abort",
        db_wal_log::WalRecord::Command(_) => "command",
        db_wal_log::WalRecord::Payload(_) => "payload",
        db_wal_log::WalRecord::Diff(_) => "diff",
        db_wal_log::WalRecord::Inverse(_) => "inverse",
        db_wal_log::WalRecord::Event(_) => "event",
        db_wal_log::WalRecord::Outbox(_) => "outbox",
        db_wal_log::WalRecord::Frontier(_) => "frontier",
        db_wal_log::WalRecord::SnapshotPub { .. } => "snapshot_pub",
        db_wal_log::WalRecord::IndexCkpt { .. } => "index_ckpt",
        db_wal_log::WalRecord::Lease { .. } => "lease",
        db_wal_log::WalRecord::Migration(_) => "migration",
    }
}

fn describe_wal_record(record: &db_wal_log::WalRecord) -> String {
    let kind = wal_record_kind_name(record);
    match record {
        db_wal_log::WalRecord::SegmentHeader { document, segment_index, prev_chain_hash } => {
            format!("{kind} document={document} segment_index={segment_index} prev_chain_hash={}", prev_chain_hash.map_or_else(|| "-".to_string(), |hash| hex32(&hash)))
        }
        db_wal_log::WalRecord::TxBegin { tx_id } => format!("{kind} tx_id={tx_id}"),
        db_wal_log::WalRecord::TxCommit { tx_id, record_count } => format!("{kind} tx_id={tx_id} record_count={record_count}"),
        db_wal_log::WalRecord::TxAbort { tx_id } => format!("{kind} tx_id={tx_id}"),
        db_wal_log::WalRecord::Command(bytes) => format!("{kind} bytes={}", bytes.len()),
        db_wal_log::WalRecord::Payload(db_wal_log::WalPayloadRef::Inline(bytes)) => format!("{kind} inline bytes={}", bytes.len()),
        db_wal_log::WalRecord::Payload(db_wal_log::WalPayloadRef::CasRef(hash)) => format!("{kind} cas_ref hash={}", hex32(&hash.0)),
        db_wal_log::WalRecord::Diff(bytes) => format!("{kind} bytes={}", bytes.len()),
        db_wal_log::WalRecord::Inverse(bytes) => format!("{kind} bytes={}", bytes.len()),
        db_wal_log::WalRecord::Event(bytes) => format!("{kind} bytes={}", bytes.len()),
        db_wal_log::WalRecord::Outbox(bytes) => format!("{kind} bytes={}", bytes.len()),
        db_wal_log::WalRecord::Frontier(frontier) => {
            format!("{kind} head_seq={} commit_seq={} epoch={} chain_hash={}", frontier.head_seq, frontier.commit_seq, frontier.epoch, hex32(&frontier.chain_hash))
        }
        db_wal_log::WalRecord::SnapshotPub { generation, frontier } => format!("{kind} generation={generation} head_seq={}", frontier.head_seq),
        db_wal_log::WalRecord::IndexCkpt { run_ids } => format!("{kind} run_ids={}", run_ids.len()),
        db_wal_log::WalRecord::Lease { resource, holder, fence, expires_at_ms } => format!("{kind} resource={} holder={} fence={fence} expires_at_ms={expires_at_ms}", resource.as_str(), holder.as_str()),
        db_wal_log::WalRecord::Migration(bytes) => format!("{kind} bytes={}", bytes.len()),
    }
}

/// 📼️ `db wal-inspect <root> <document-id> [--limit N]` — lists `document`'s raw WAL segments
/// (index + byte length, straight from `WalStorage`, before any decoding) then decodes every
/// `WAL_*` record via `db::wal::replay_document`. A torn/corrupt WAL is reported honestly (with a
/// hint pointing at `db repair`), never panics.
pub(super) async fn cmd_wal_inspect(rest: &[String]) -> i32 {
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
    let document = db_ids::ArtifactId(id.clone());

    let mut segments = match db_actor::block_on(storage.list_segments(&document)) {
        Ok(segments) => segments,
        Err(err) => return fail("list_segments", err).await,
    };
    println!("== wal segments: {} ==", segments.len());
    for index in &segments {
        match db_actor::block_on(storage.segment_len(&document, *index)) {
            Ok(len) => println!("  segment {index}: {len} bytes"),
            Err(err) => println!("  segment {index}: <error reading length: {err}>"),
        }
    }
    let _ = segments.close_step();
    drop(segments);

    let control = match wal_cursor_control() {
        Ok(control) => control,
        Err(err) => return fail("wal cursor", err).await,
    };
    match db_wal_log::replay_document(&storage, &document, control).await {
        Ok(mut records) => {
            println!("== wal records ==");
            let mut count = 0usize;
            let mut shown = 0usize;
            loop {
                match records.next_step().await {
                    Ok(db_wal_log::WalReplayStep::Record(record)) => {
                        if limit.is_none_or(|limit| shown < limit) {
                            println!("  [{count}] {}", describe_wal_record(&record));
                            shown += 1;
                        }
                        count += 1;
                        if let Err(err) = MountedWalRecordCommandClose::new(record).await {
                            let _ = MountedWalReplayCommandClose::new(records).await;
                            return fail("record close", err).await;
                        }
                    }
                    Ok(db_wal_log::WalReplayStep::Yield) => continue,
                    Ok(db_wal_log::WalReplayStep::Done) => break,
                    Err(err) => {
                        let _ = MountedWalReplayCommandClose::new(records).await;
                        return fail("replay", err).await;
                    }
                }
            }
            if let Err(err) = MountedWalReplayCommandClose::new(records).await {
                return fail("replay close", err).await;
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
pub(super) async fn cmd_snapshot_inspect(rest: &[String]) -> i32 {
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
    let document = db_ids::ArtifactId(id.clone());

    let mut generations = match db_actor::block_on(storage.list_generations(&document)) {
        Ok(generations) => generations,
        Err(err) => return fail("list_generations", err).await,
    };
    println!("== snapshot generations: {} ==", generations.len());
    for generation in &generations {
        println!("  - {generation}");
    }
    let latest = generations.last().copied();
    let _ = generations.close_step();
    drop(generations);
    let Some(latest) = latest else {
        return 0;
    };

    let generation = match flags.get("generation").map(|value| value.parse::<u64>()) {
        None => latest,
        Some(Ok(generation)) => generation,
        Some(Err(_)) => return usage("db snapshot-inspect: --generation must be a non-negative integer").await,
    };

    let manager = db_snapshot::SnapshotManager::new(&storage).await;
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = match db_snapshot::SnapshotCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 8_192) {
        Ok(control) => control,
        Err(err) => return fail("snapshot_cursor", err).await,
    };
    let mut cursor = manager.chain_cursor(&document, generation, control);
    let descriptor = match db_actor::block_on(cursor.latest_descriptor()) {
        Ok(descriptor) => descriptor,
        Err(err) => {
            let _ = MountedSnapshotCommandClose::new(cursor).await;
            return fail("snapshot_cursor", err).await;
        }
    };
    println!("== generation {generation} descriptor ==");
    println!("  parent_generation: {}", descriptor.parent_generation.map_or_else(|| "-".to_string(), |generation| generation.to_string()));
    println!("  head_seq: {}", descriptor.head_seq);
    println!("  commit_seq: {}", descriptor.commit_seq);
    println!("  epoch: {}", descriptor.epoch);
    println!("  chain_hash: {}", hex32(&descriptor.chain_hash));
    println!("  protocol_version: {}", descriptor.protocol_version);
    println!("  base_pack_hash: {}", descriptor.base_pack_hash.map_or_else(|| "-".to_string(), |hash| hex32(&hash.0)));
    println!("  roots: {}", descriptor.roots.len());
    println!("  new_pages: {}", descriptor.new_pages.len());
    println!("  created_at_ms: {}", descriptor.created_at_ms);
    if let Err(err) = MountedSnapshotCommandClose::new(cursor).await {
        return fail("snapshot cursor close", err).await;
    }

    if verify {
        match db_actor::block_on(manager.verify(&document, generation, pack::os_pack::VerificationLevel::Full)) {
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
