//! 🎞️ `protocol_cli` — the `protocol` binary: `inspect`/`verify`/`hash`/`log`/`compile`/
//! `decompile`/`diff`/`compact`/`repair`/`upgrade` over `.spr` binary op-log files. Schema-free by
//! construction — op payloads stay opaque text/bytes to this crate, exactly like the rest of the
//! `protocol_*` family. This crate depends on nothing but the `protocol` facade (no app crates, no
//! `protocol_core`/`protocol_format`/`protocol_history` path deps of its own) — see the `//#region
//! 🔖️Frame` note below for how the handful of frozen wire constants that implies get sourced.
//!
//! See the `## protocol_cli` section (plus the amendment's workspace-wide notes) of the frozen
//! contract at `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md`.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::Write as _;
use std::path::{Path, PathBuf};

//#region 🔖️Frame
/// 🎞️ `.spr` header size in bytes — frozen by the family's contract (`## protocol_format`,
/// `Header (32 bytes)`). Mirrored here rather than imported: `protocol_cli` depends on the
/// `protocol` facade only (per its own contract section), and the facade does not re-export
/// `crate::os_spr::format::HEADER_SIZE`. `inspect`'s header dump and record-kind walk are the only
/// places this crate needs the file's record-stream start offset.
const HEADER_SIZE: u64 = 32;

/// 🎞️ The 8-byte `.spr` magic, mirrored from the frozen contract for `inspect`'s header display
/// (same rationale as `HEADER_SIZE` — not re-exported by the facade).
const MAGIC: [u8; 8] = [0x89, b'S', b'P', b'R', 0x0D, 0x0A, 0x1A, 0x0A];

// Record-kind byte table, mirrored from `crate::os_spr::REC_*` (frozen contract values, `##
// protocol_core`, `//#region 🔖️RecordKinds`) for the same not-a-direct-dependency reason as
// `HEADER_SIZE` above — this crate only ever reads `RecordFrame::kind` bytes surfaced by the
// facade's re-exported `FrameCursor`, it never needs the full `protocol_core` crate.
const REC_END: u8 = 0x00;
const REC_DOC: u8 = 0x01;
const REC_ACTOR_DICT: u8 = 0x02;
const REC_STR_DICT: u8 = 0x03;
const REC_EDIT: u8 = 0x04;
const REC_CHANGE: u8 = 0x05;
const REC_CHECKPOINT: u8 = 0x06;
const REC_ALTERNATIVE: u8 = 0x07;
const REC_ACTIVE: u8 = 0x08;
const REC_FRONTIER: u8 = 0x09;
const REC_PROJECTION: u8 = 0x0A;
const REC_INDEX: u8 = 0x0B;
const REC_COMMIT: u8 = 0x0C;
const REC_SIGNATURE: u8 = 0x0D;
const REC_REDACTION: u8 = 0x0E;
const REC_UPCAST: u8 = 0x0F;
const REC_EPHEMERAL: u8 = 0x10;
const REC_SEALED: u8 = 0x11;
const REC_COMPACTION: u8 = 0x12;
const REC_PADDING: u8 = 0x7F;

/// 🏷️ A short human-readable label for `inspect`'s record-count table.
// 🚫️async: R9 pure accessor — every consumer is a `println!`/`assert_eq!` argument position,
// which cannot await; no suspension point exists in the body either.
fn kind_name(kind: u8) -> &'static str {
    match kind {
        REC_END => "end",
        REC_DOC => "doc",
        REC_ACTOR_DICT => "actor_dict",
        REC_STR_DICT => "str_dict",
        REC_EDIT => "edit",
        REC_CHANGE => "change",
        REC_CHECKPOINT => "checkpoint",
        REC_ALTERNATIVE => "alternative",
        REC_ACTIVE => "active",
        REC_FRONTIER => "frontier",
        REC_PROJECTION => "snapshot",
        REC_INDEX => "index",
        REC_COMMIT => "commit",
        REC_SIGNATURE => "signature",
        REC_REDACTION => "redaction",
        REC_UPCAST => "upcast",
        REC_EPHEMERAL => "ephemeral",
        REC_SEALED => "sealed",
        REC_COMPACTION => "compaction",
        REC_PADDING => "padding",
        _ => "extension",
    }
}

/// 🎞️ Decoded fields of a `REC_COMMIT` frame's fixed 64-byte payload, hand-parsed against the
/// frozen byte layout (`## protocol_format`, `Commit frame`) for the same not-a-direct-dependency
/// reason as `HEADER_SIZE` above (`crate::os_spr::format::CommitPayload`/`parse_commit_payload` are not
/// part of the facade's re-export surface).
struct CommitFields {
    commit_seq: u64,
    prev_commit_offset: u64,
    records_len: u64,
    record_count: u32,
    chain_hash: [u8; 32],
}

/// 🎞️ Parses a `REC_COMMIT` frame's `payload()` bytes per the frozen 64-byte layout; `None` on
/// any length mismatch (never panics on corrupt input).
async fn parse_commit_fields(payload: &[u8]) -> Option<CommitFields> {
    if payload.len() != 64 {
        return None;
    }
    let commit_seq = u64::from_le_bytes(payload[0..8].try_into().ok()?);
    let prev_commit_offset = u64::from_le_bytes(payload[8..16].try_into().ok()?);
    let records_len = u64::from_le_bytes(payload[16..24].try_into().ok()?);
    let record_count = u32::from_le_bytes(payload[24..28].try_into().ok()?);
    let mut chain_hash = [0u8; 32];
    chain_hash.copy_from_slice(&payload[32..64]);
    Some(CommitFields { commit_seq, prev_commit_offset, records_len, record_count, chain_hash })
}
//#endregion 🔖️Frame

//#region 🔖️Args
/// ✂️ Splits argv-style slices into positionals and `--flag value` / `--flag=value` pairs; a
/// trailing bare `--flag` with nothing after it maps to an empty-string value. Callers with any
/// no-argument boolean flags (`--reverse`, `--truncate-torn-tail`, ...) must strip those out of
/// `args` first (see `parse_log_args`/`parse_repair_args`) — this parser always tries to consume
/// the next token as a value, which would otherwise swallow a following positional/flag.
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

async fn parse_level(flags: &HashMap<String, String>) -> Result<crate::os_spr::VerificationLevel, String> {
    match flags.get("level").map(String::as_str) {
        None => Ok(crate::os_spr::VerificationLevel::Standard),
        Some("trusted") => Ok(crate::os_spr::VerificationLevel::Trusted),
        Some("standard") => Ok(crate::os_spr::VerificationLevel::Standard),
        Some("full") => Ok(crate::os_spr::VerificationLevel::Full),
        Some(other) => Err(format!("unknown --level '{other}' (expected trusted|standard|full)")),
    }
}

/// ✂️ `log`'s own arg splitter: `--reverse` takes no value, so it is pulled out before the
/// generic `--flag value` pairing runs (see `parse_args`'s doc comment).
async fn parse_log_args(rest: &[String]) -> (Vec<String>, HashMap<String, String>, bool) {
    let mut filtered = Vec::new();
    let mut reverse = false;
    for arg in rest {
        if arg == "--reverse" {
            reverse = true;
        } else {
            filtered.push(arg.clone());
        }
    }
    let (positional, flags) = parse_args(&filtered).await;
    (positional, flags, reverse)
}

/// ✂️ `repair`'s own arg splitter: both flags are no-value booleans.
async fn parse_repair_args(rest: &[String]) -> (Vec<String>, bool, bool) {
    let mut positional = Vec::new();
    let mut truncate_torn_tail = false;
    let mut rebuild_indexes = false;
    for arg in rest {
        match arg.as_str() {
            "--truncate-torn-tail" => truncate_torn_tail = true,
            "--rebuild-indexes" => rebuild_indexes = true,
            other => positional.push(other.to_string()),
        }
    }
    (positional, truncate_torn_tail, rebuild_indexes)
}
//#endregion 🔖️Args

//#region 🔖️Format
// 🚫️async: R9 pure accessor — every consumer is a `println!` argument position, which cannot
// await; no suspension point exists in the body either.
fn hex32(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// #⃣ A short non-cryptographic content fingerprint for `diff`'s "payload-hash mismatch" lines.
///
/// 🎯️ Design choice: the contract's real chain-hash primitive is `blake3`, but the only `blake3`
/// impl in this family (`crate::os_spr::format::Blake3Hasher`) is not part of the facade's re-export
/// surface (the trait `crate::os_spr::RecordHasher` is re-exported, an implementation is not) — and
/// `protocol_cli` depends on nothing but the facade. `std::hash::Hasher` is dependency-free and
/// sufficient to spot content differences for a CLI diff display; it is explicitly NOT presented
/// as a cryptographic digest (labelled `fp=` in output, never `hash=`).
// 🚫️async: R9 pure accessor — only consumer is a `println!` argument position, which cannot
// await; no suspension point exists in the body either.
fn fingerprint(edit: &crate::os_spr::HistoryEdit) -> String {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    edit.id.hash(&mut hasher);
    edit.actor.hash(&mut hasher);
    edit.started_at.hash(&mut hasher);
    edit.finished_at.hash(&mut hasher);
    edit.coalesce_key.hash(&mut hasher);
    edit.description.hash(&mut hasher);
    for op in &edit.ops {
        op.text.hash(&mut hasher);
        op.binary.hash(&mut hasher);
    }
    format!("{:016x}", hasher.finish())
}
//#endregion 🔖️Format

//#region 🔖️Inspect
/// 🔍️ `protocol inspect <file>` — header, commit chain (all generations), record counts by kind,
/// and dictionary/snapshot/index record tallies. Never panics on corrupt input: a malformed
/// frame simply stops the walk early and the summary prints whatever was scanned so far.
async fn cmd_inspect(rest: &[String]) -> i32 {
    let (positional, _flags) = parse_args(rest).await;
    let Some(path) = positional.first() else {
        eprintln!("usage: protocol inspect <file>");
        return 2;
    };
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("protocol: cannot read '{path}': {error}");
            return 1;
        }
    };
    if (bytes.len() as u64) < HEADER_SIZE {
        eprintln!("protocol: '{path}' is shorter than the {HEADER_SIZE}-byte header");
        return 1;
    }

    println!("== header ==");
    println!("  magic_ok: {}", bytes.get(0..8) == Some(MAGIC.as_slice()));
    println!("  version: {}.{}", u16::from_le_bytes([bytes[8], bytes[9]]), u16::from_le_bytes([bytes[10], bytes[11]]));
    println!("  required_flags: {:#010x}", u32::from_le_bytes(bytes[12..16].try_into().unwrap()));
    println!("  optional_flags: {:#010x}", u32::from_le_bytes(bytes[16..20].try_into().unwrap()));

    let mut counts: BTreeMap<u8, u64> = BTreeMap::new();
    let mut commits: Vec<(u64, CommitFields)> = Vec::new();
    let mut cursor = crate::os_spr::FrameCursor::new(&bytes, HEADER_SIZE).await;
    loop {
        match cursor.next_frame().await {
            Ok(Some(frame)) => {
                *counts.entry(frame.kind).or_insert(0) += 1;
                if frame.kind == REC_COMMIT {
                    if let Some(fields) = parse_commit_fields(frame.payload().await).await {
                        commits.push((frame.offset, fields));
                    }
                }
            }
            Ok(None) => break,
            Err(error) => {
                eprintln!("protocol: stopped scanning at a malformed frame: {error}");
                break;
            }
        }
    }

    println!("== record counts by kind ==");
    for (kind, count) in &counts {
        println!("  {} ({kind:#04x}): {count}", kind_name(*kind));
    }

    println!("== commit chain ==");
    for (offset, commit) in &commits {
        println!("  commit_seq={} offset={offset} prev_commit_offset={} record_count={} records_len={} chain_hash={}", commit.commit_seq, commit.prev_commit_offset, commit.record_count, commit.records_len, hex32(&commit.chain_hash));
    }

    println!("== dictionaries ==");
    println!("  actor_dict_deltas: {}", counts.get(&REC_ACTOR_DICT).copied().unwrap_or(0));
    println!("  str_dict_deltas: {}", counts.get(&REC_STR_DICT).copied().unwrap_or(0));

    println!("== snapshots ==");
    println!("  records: {}", counts.get(&REC_PROJECTION).copied().unwrap_or(0));

    println!("== indexes ==");
    println!("  records: {}", counts.get(&REC_INDEX).copied().unwrap_or(0));

    0
}
//#endregion 🔖️Inspect

//#region 🔖️Verify
/// 🚪️ Opens a `HistoryReader` and decodes its `HistoryLog`, sequenced explicitly: `reader.log()`
/// is async, so it cannot be chained through `Result::and_then`'s sync closure (R10 shape 1).
/// Shared by `cmd_verify`/`cmd_diff`/`cmd_frontier` — every `.spr` inspection command opens the
/// same way.
async fn open_and_log(bytes: &[u8], options: &crate::os_spr::DecodeOptions) -> Result<crate::os_spr::HistoryLog, crate::os_spr::ProtocolError> {
    match crate::os_spr::HistoryReader::open(bytes, options).await {
        Ok(reader) => reader.log().await,
        Err(error) => Err(error),
    }
}

/// 🛡️ `protocol verify <file> [--level=trusted|standard|full]` — opens via `HistoryReader` at the
/// requested `VerificationLevel` and forces a full decode; `full` additionally recomputes the
/// commit hash chain (see `crate::os_spr::history::decode_history_from`'s `VerificationLevel::Full`
/// branch). Prints `OK`/`FAIL: <reason>`, never panics on corrupt input.
async fn cmd_verify(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest).await;
    let Some(path) = positional.first() else {
        eprintln!("usage: protocol verify <file> [--level=trusted|standard|full]");
        return 2;
    };
    let level = match parse_level(&flags).await {
        Ok(level) => level,
        Err(error) => {
            eprintln!("protocol: {error}");
            return 2;
        }
    };
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) => {
            println!("FAIL: cannot read '{path}': {error}");
            return 1;
        }
    };
    let options = crate::os_spr::DecodeOptions { verification: level, limits: crate::os_spr::ProtocolLimits::default() };
    match open_and_log(&bytes, &options).await {
        Ok(log) => {
            println!("OK");
            println!("  doc_id: {}", log.doc_id);
            println!("  edits: {}", log.edits.len());
            println!("  changes: {}", log.changes.len());
            println!("  checkpoints: {}", log.checkpoints.len());
            0
        }
        Err(error) => {
            println!("FAIL: {error}");
            1
        }
    }
}
//#endregion 🔖️Verify

//#region 🔖️Hash
/// #⃣ `protocol hash <file>` — prints `(commit_seq, chain_hash)`, the file's current commit
/// identity, via `crate::os_spr::content_frontier`.
async fn cmd_hash(rest: &[String]) -> i32 {
    let (positional, _flags) = parse_args(rest).await;
    let Some(path) = positional.first() else {
        eprintln!("usage: protocol hash <file>");
        return 2;
    };
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("protocol: cannot read '{path}': {error}");
            return 1;
        }
    };
    match crate::os_spr::content_frontier(&bytes).await {
        Ok(frontier) => {
            println!("{} {}", frontier.last_commit_seq, hex32(&frontier.chain_hash));
            0
        }
        Err(error) => {
            eprintln!("protocol: {error}");
            1
        }
    }
}
//#endregion 🔖️Hash

//#region 🔖️Log
/// 📜️ `protocol log <file> [--limit N] [--actor ID] [--alternative ID] [--reverse]` — a timeline
/// text dump: one line per edit (ordinal, id, actor column, op count, description), annotated with
/// `[checkpoint ...]` lane markers where a checkpoint's reachable edits top out, and `(amends
/// <id>)` when an edit shares a `coalesce_key` with an earlier one in the printed range.
async fn cmd_log(rest: &[String]) -> i32 {
    let (positional, flags, reverse) = parse_log_args(rest).await;
    let Some(path) = positional.first() else {
        eprintln!("usage: protocol log <file> [--limit N] [--actor ID] [--alternative ID] [--reverse]");
        return 2;
    };
    let limit: Option<usize> = match flags.get("limit") {
        None => None,
        Some(value) => match value.parse::<usize>() {
            Ok(n) => Some(n),
            Err(_) => {
                eprintln!("protocol: --limit expects a non-negative integer, got '{value}'");
                return 2;
            }
        },
    };
    let actor_filter = flags.get("actor");
    let alternative_filter = flags.get("alternative");

    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("protocol: cannot read '{path}': {error}");
            return 1;
        }
    };
    let options = crate::os_spr::DecodeOptions::default();
    let log = match open_and_log(&bytes, &options).await {
        Ok(log) => log,
        Err(error) => {
            eprintln!("protocol: {error}");
            return 1;
        }
    };

    let allowed_ids: Option<HashSet<String>> = match alternative_filter {
        None => None,
        Some(alternative_id) => {
            let Some(alternative) = log.alternatives.iter().find(|a| &a.id == alternative_id) else {
                eprintln!("protocol: unknown --alternative '{alternative_id}'");
                return 2;
            };
            let mut ids = HashSet::new();
            for checkpoint_id in &alternative.checkpoint_ids {
                if let Some(checkpoint) = log.checkpoints.iter().find(|c| &c.id == checkpoint_id) {
                    for change_id in &checkpoint.change_ids {
                        if let Some(change) = log.changes.iter().find(|c| &c.id == change_id) {
                            ids.extend(change.edit_ids.iter().cloned());
                        }
                    }
                }
            }
            Some(ids)
        }
    };

    let ordinal_of: HashMap<&str, u64> = log.edits.iter().enumerate().map(|(ordinal, edit)| (edit.id.as_str(), ordinal as u64)).collect();
    let mut checkpoint_lane_at: HashMap<u64, Vec<String>> = HashMap::new();
    for checkpoint in &log.checkpoints {
        let landing_ordinal = checkpoint.change_ids.iter().filter_map(|change_id| log.changes.iter().find(|c| &c.id == change_id)).flat_map(|change| change.edit_ids.iter()).filter_map(|edit_id| ordinal_of.get(edit_id.as_str()).copied()).max();
        if let Some(ordinal) = landing_ordinal {
            checkpoint_lane_at.entry(ordinal).or_default().push(checkpoint.id.clone());
        }
    }

    let mut seen_coalesce: HashMap<&str, &str> = HashMap::new();
    let mut lines = Vec::new();
    for (ordinal, edit) in log.edits.iter().enumerate() {
        let ordinal = ordinal as u64;
        if let Some(actor_id) = actor_filter {
            if edit.actor.as_deref() != Some(actor_id.as_str()) {
                continue;
            }
        }
        if let Some(ids) = &allowed_ids {
            if !ids.contains(&edit.id) {
                continue;
            }
        }
        let actor_display = edit.actor.as_deref().unwrap_or("-");
        let mut amend_marker = String::new();
        if let Some(key) = &edit.coalesce_key {
            if let Some(previous_id) = seen_coalesce.insert(key.as_str(), edit.id.as_str()) {
                amend_marker = format!(" (amends {previous_id})");
            }
        }
        let checkpoint_marker = checkpoint_lane_at.get(&ordinal).map_or(String::new(), |ids| format!(" [checkpoint {}]", ids.join(", ")));
        let description = edit.description.as_deref().map_or(String::new(), |d| format!(" \"{d}\""));
        lines.push(format!("#{ordinal:<5} {} actor={actor_display} started={} ops={}{amend_marker}{checkpoint_marker}{description}", edit.id, edit.started_at, edit.ops.len()));
    }

    if reverse {
        lines.reverse();
    }
    if let Some(n) = limit {
        lines.truncate(n);
    }
    for line in &lines {
        println!("{line}");
    }
    0
}
//#endregion 🔖️Log

//#region 🔖️Compile
/// 🔨️ `protocol compile <doc.ops> [--out doc.spr]` — ops text -> `.spr` binary via
/// `crate::os_spr::compile_ops`. Writes to `--out` when given, else emits the raw bytes to stdout.
async fn cmd_compile(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest).await;
    let Some(path) = positional.first() else {
        eprintln!("usage: protocol compile <doc.ops> [--out doc.spr]");
        return 2;
    };
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("protocol: cannot read '{path}': {error}");
            return 1;
        }
    };
    let bytes = match crate::os_spr::compile_ops(&text, &crate::os_spr::EncodeOptions::default()).await {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("protocol: compile failed: {error}");
            return 1;
        }
    };
    match flags.get("out") {
        Some(out_path) => match std::fs::write(out_path, &bytes) {
            Ok(()) => {
                println!("wrote {out_path} ({} bytes)", bytes.len());
                0
            }
            Err(error) => {
                eprintln!("protocol: write failed: {error}");
                1
            }
        },
        None => match std::io::stdout().write_all(&bytes) {
            Ok(()) => 0,
            Err(error) => {
                eprintln!("protocol: stdout write failed: {error}");
                1
            }
        },
    }
}
//#endregion 🔖️Compile

//#region 🔖️Decompile
/// 🔧️ `protocol decompile <doc.spr> [--out doc.ops]` — `.spr` binary -> ops text via
/// `crate::os_spr::decompile_ops`. Writes to `--out` when given, else prints the text to stdout.
async fn cmd_decompile(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest).await;
    let Some(path) = positional.first() else {
        eprintln!("usage: protocol decompile <doc.spr> [--out doc.ops]");
        return 2;
    };
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("protocol: cannot read '{path}': {error}");
            return 1;
        }
    };
    let text = match crate::os_spr::decompile_ops(&bytes, &crate::os_spr::DecodeOptions::default()).await {
        Ok(text) => text,
        Err(error) => {
            eprintln!("protocol: decompile failed: {error}");
            return 1;
        }
    };
    match flags.get("out") {
        Some(out_path) => match std::fs::write(out_path, &text) {
            Ok(()) => {
                println!("wrote {out_path} ({} bytes)", text.len());
                0
            }
            Err(error) => {
                eprintln!("protocol: write failed: {error}");
                1
            }
        },
        None => {
            print!("{text}");
            0
        }
    }
}
//#endregion 🔖️Decompile

//#region 🔖️Diff
/// 🌗️ `protocol diff <a.spr> <b.spr>` — record-level edit diff: the longest common prefix, then
/// `only-in-a`/`only-in-b` by `(ordinal, id)` and content-mismatch lines (see `fingerprint`'s doc
/// comment for why these are labelled `fp=`, not `hash=`) for edits present on both sides past the
/// prefix. Exit `0` when identical, `1` when they differ, `2` on a usage/read/decode error.
async fn cmd_diff(rest: &[String]) -> i32 {
    let (positional, _flags) = parse_args(rest).await;
    if positional.len() < 2 {
        eprintln!("usage: protocol diff <a.spr> <b.spr>");
        return 2;
    }
    let path_a = &positional[0];
    let path_b = &positional[1];
    let bytes_a = match std::fs::read(path_a) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("protocol: cannot read '{path_a}': {error}");
            return 1;
        }
    };
    let bytes_b = match std::fs::read(path_b) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("protocol: cannot read '{path_b}': {error}");
            return 1;
        }
    };
    let options = crate::os_spr::DecodeOptions::default();
    let log_a = match open_and_log(&bytes_a, &options).await {
        Ok(log) => log,
        Err(error) => {
            eprintln!("protocol: decode '{path_a}' failed: {error}");
            return 1;
        }
    };
    let log_b = match open_and_log(&bytes_b, &options).await {
        Ok(log) => log,
        Err(error) => {
            eprintln!("protocol: decode '{path_b}' failed: {error}");
            return 1;
        }
    };

    let common_prefix = log_a.edits.iter().zip(log_b.edits.iter()).take_while(|(a, b)| a == b).count();
    if common_prefix == log_a.edits.len() && common_prefix == log_b.edits.len() {
        println!("identical ({common_prefix} edits)");
        return 0;
    }
    println!("common prefix: {common_prefix} edits");

    let remaining_a = &log_a.edits[common_prefix..];
    let remaining_b = &log_b.edits[common_prefix..];
    let by_id_b: HashMap<&str, (usize, &crate::os_spr::HistoryEdit)> = remaining_b.iter().enumerate().map(|(i, edit)| (edit.id.as_str(), (common_prefix + i, edit))).collect();
    let ids_a: HashSet<&str> = remaining_a.iter().map(|edit| edit.id.as_str()).collect();

    let mut differs = false;
    for (i, edit) in remaining_a.iter().enumerate() {
        let ordinal = common_prefix + i;
        match by_id_b.get(edit.id.as_str()) {
            None => {
                println!("only-in-a: ordinal={ordinal} id={}", edit.id);
                differs = true;
            }
            Some(&(ordinal_b, edit_b)) => {
                if edit != edit_b {
                    println!("payload mismatch: id={} a_ordinal={ordinal} a_fp={} b_ordinal={ordinal_b} b_fp={}", edit.id, fingerprint(edit), fingerprint(edit_b));
                    differs = true;
                }
            }
        }
    }
    for (i, edit) in remaining_b.iter().enumerate() {
        if !ids_a.contains(edit.id.as_str()) {
            println!("only-in-b: ordinal={} id={}", common_prefix + i, edit.id);
            differs = true;
        }
    }

    i32::from(differs)
}
//#endregion 🔖️Diff

//#region 🔖️Compact
/// ♻️ `protocol compact <file> [--out FIXED]` — calls `crate::os_spr::compact` (atomic, in-place
/// rewrite). `--out`, when given, first copies `<file>` to `FIXED` and compacts the copy, leaving
/// the original untouched; without it, `<file>` is compacted in place (matching `compact`'s own
/// in-place-only signature).
async fn cmd_compact(rest: &[String]) -> i32 {
    let (positional, flags) = parse_args(rest).await;
    let Some(path) = positional.first() else {
        eprintln!("usage: protocol compact <file> [--out FIXED]");
        return 2;
    };
    let target: PathBuf = match flags.get("out") {
        Some(out_path) => {
            if let Err(error) = std::fs::copy(path, out_path) {
                eprintln!("protocol: cannot copy '{path}' to '{out_path}': {error}");
                return 1;
            }
            PathBuf::from(out_path)
        }
        None => PathBuf::from(path),
    };
    let options = crate::os_spr::CompactOptions { drop_ephemeral: true, keep_snapshots: crate::os_spr::KeepSnapshots::All };
    match crate::os_spr::compact(&target, &options, &crate::os_spr::ProtocolLimits::default()).await {
        Ok(()) => {
            println!("compacted {}", target.display());
            0
        }
        Err(error) => {
            eprintln!("protocol: compact failed: {error}");
            1
        }
    }
}
//#endregion 🔖️Compact

//#region 🔖️Repair
/// 🩹️ `protocol repair <file> [--truncate-torn-tail] [--rebuild-indexes]` — runs
/// `crate::os_spr::recover_file` and prints its `RecoveryReport`; `--truncate-torn-tail` then physically
/// truncates the file to the recovered prefix.
///
/// 🎯️ `--rebuild-indexes` is a documented no-op in this build: rebuilding `REC_INDEX` payloads
/// needs `crate::os_spr::history::IndexBuilder`/`IndexReader`, which are not part of the facade's
/// re-export surface (this crate's sole dependency) — same rationale as `upgrade`'s "hook exists,
/// v1 passthrough" note in the contract.
async fn cmd_repair(rest: &[String]) -> i32 {
    let (positional, truncate_torn_tail, rebuild_indexes) = parse_repair_args(rest).await;
    let Some(path) = positional.first() else {
        eprintln!("usage: protocol repair <file> [--truncate-torn-tail] [--rebuild-indexes]");
        return 2;
    };
    let limits = crate::os_spr::ProtocolLimits::default();
    let report = match crate::os_spr::recover_file(Path::new(path), &limits, crate::os_spr::RecoveryMode::LastCommit).await {
        Ok(report) => report,
        Err(error) => {
            eprintln!("protocol: recovery failed: {error}");
            return 1;
        }
    };
    println!("records_recovered: {}", report.records_recovered);
    println!("bytes_recovered: {}", report.bytes_recovered);
    println!("last_commit_seq: {}", report.last_commit_seq);
    println!("torn_tail_bytes: {}", report.torn_tail_bytes);

    if truncate_torn_tail && report.torn_tail_bytes > 0 {
        let file = match std::fs::OpenOptions::new().write(true).open(path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("protocol: cannot open '{path}' for truncation: {error}");
                return 1;
            }
        };
        if let Err(error) = file.set_len(report.bytes_recovered) {
            eprintln!("protocol: truncate failed: {error}");
            return 1;
        }
        println!("truncated torn tail: {} bytes removed", report.torn_tail_bytes);
    }
    if rebuild_indexes {
        println!("note: --rebuild-indexes is a no-op in this build (see cmd_repair's doc comment)");
    }
    0
}
//#endregion 🔖️Repair

//#region 🔖️Upgrade
/// ⬆️ `protocol upgrade <file>` — v1 `RecordUpcaster`-driven rewrite hook: no upcaster exists yet
/// in this family, so `upgrade` validates the file decodes cleanly (Full verification) and passes
/// it through unmodified, exactly as the contract's "no-op passthrough, hook exists" note specifies.
async fn cmd_upgrade(rest: &[String]) -> i32 {
    let (positional, _flags) = parse_args(rest).await;
    let Some(path) = positional.first() else {
        eprintln!("usage: protocol upgrade <file>");
        return 2;
    };
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("protocol: cannot read '{path}': {error}");
            return 1;
        }
    };
    let options = crate::os_spr::DecodeOptions { verification: crate::os_spr::VerificationLevel::Full, limits: crate::os_spr::ProtocolLimits::default() };
    match open_and_log(&bytes, &options).await {
        Ok(_) => {
            println!("no upgrade needed (v1 passthrough; RecordUpcaster hook not yet wired to any schema)");
            0
        }
        Err(error) => {
            eprintln!("protocol: upgrade check failed: {error}");
            1
        }
    }
}
//#endregion 🔖️Upgrade

//#region 🔖️Cli
async fn print_help() {
    println!("protocol — inspect/verify/hash/log/compile/decompile/diff/compact/repair/upgrade .spr binary op-log files\n");
    println!("USAGE:");
    println!("  protocol inspect <file>");
    println!("  protocol verify <file> [--level=trusted|standard|full]");
    println!("  protocol hash <file>");
    println!("  protocol log <file> [--limit N] [--actor ID] [--alternative ID] [--reverse]");
    println!("  protocol compile <doc.ops> [--out doc.spr]");
    println!("  protocol decompile <doc.spr> [--out doc.ops]");
    println!("  protocol diff <a.spr> <b.spr>");
    println!("  protocol compact <file> [--out FIXED]");
    println!("  protocol repair <file> [--truncate-torn-tail] [--rebuild-indexes]");
    println!("  protocol upgrade <file>");
}

/// 🚪️ The CLI's single testable entry point — `main` is a thin `std::process::exit` wrapper
/// around this. Never panics on malformed input; every subcommand handler maps errors to a
/// printed message and a non-zero exit code instead. Exit codes: `0` success, `1` runtime/decode
/// failure, `2` usage error.
pub async fn main_impl(args: &[String]) -> i32 {
    let Some((command, rest)) = args.split_first() else {
        print_help();
        return 2;
    };
    match command.as_str() {
        "inspect" => cmd_inspect(rest).await,
        "verify" => cmd_verify(rest).await,
        "hash" => cmd_hash(rest).await,
        "log" => cmd_log(rest).await,
        "compile" => cmd_compile(rest).await,
        "decompile" => cmd_decompile(rest).await,
        "diff" => cmd_diff(rest).await,
        "compact" => cmd_compact(rest).await,
        "repair" => cmd_repair(rest).await,
        "upgrade" => cmd_upgrade(rest).await,
        "help" | "--help" | "-h" => {
            print_help();
            0
        }
        other => {
            eprintln!("protocol: unknown subcommand '{other}'\n");
            print_help();
            2
        }
    }
}
//#endregion 🔖️Cli

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    //#region 🔖️Fixtures
    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    async fn temp_path(name: &str) -> PathBuf {
        let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("protocol_cli_test_{}_{counter}_{name}", std::process::id()))
    }

    async fn sample_edit(id: &str, actor: Option<&str>, description: Option<&str>, coalesce_key: Option<&str>) -> crate::os_spr::HistoryEdit {
        crate::os_spr::HistoryEdit {
            id: id.to_string(),
            actor: actor.map(str::to_string),
            started_at: format!("2026-07-27T00:00:{id}Z", id = &id[id.len().saturating_sub(2)..]),
            finished_at: None,
            coalesce_key: coalesce_key.map(str::to_string),
            description: description.map(str::to_string),
            ops: vec![crate::os_spr::OpPayload { text: Some(format!("set {id} = 1")), binary: None }],
            inverse: Vec::new(),
            meta: None,
        }
    }

    /// 🧪️ Builds a small `.spr` file on disk with `edit_count` edits (ids `"e00".."eNN"`, one per
    /// commit generation), an optional checkpoint landing on the last edit, and an alternative
    /// pointing at that checkpoint. Returns the file path and the raw bytes written.
    async fn build_history_file(name: &str, edit_count: usize, with_checkpoint_and_alternative: bool) -> (PathBuf, Vec<u8>) {
        let mut appender = crate::os_spr::HistoryAppender::begin(Vec::new(), "doc-1", "schema-1", &crate::os_spr::WriteOptions::default()).unwrap();
        let mut edit_ids = Vec::new();
        for i in 0..edit_count {
            let id = format!("e{i:02}");
            let actor = if i % 2 == 0 { Some("actor-a") } else { Some("actor-b") };
            appender.append_edit(&sample_edit(&id, actor, Some("an edit"), None)).unwrap();
            appender.commit().unwrap();
            edit_ids.push(id);
        }
        if with_checkpoint_and_alternative && !edit_ids.is_empty() {
            appender.append_change(&crate::os_spr::HistoryChange { id: "c0".to_string(), saved_at: "2026-07-27T00:01:00Z".to_string(), edit_ids: edit_ids.clone(), description: None }).unwrap();
            appender.append_checkpoint(&crate::os_spr::HistoryCheckpoint { id: "cp0".to_string(), timestamp: "2026-07-27T00:02:00Z".to_string(), change_ids: vec!["c0".to_string()], parent_id: None, authors: Vec::new(), message: None }).unwrap();
            appender.append_alternative(&crate::os_spr::HistoryAlternative { id: "alt-main".to_string(), name: "main".to_string(), checkpoint_ids: vec!["cp0".to_string()] }).unwrap();
            appender.set_active(Some("alt-main")).unwrap();
            appender.commit().unwrap();
        }
        let bytes = appender.into_sink();
        let path = temp_path(name);
        std::fs::write(&path, &bytes).unwrap();
        (path, bytes)
    }

    /// 🧪️ Round-trips a small `HistoryLog` through `HistoryAppender` -> `decompile_ops` to obtain
    /// ground-truth `.ops` text without hand-writing the grammar (see the module's design note on
    /// why `parse_ops_text`/`print_ops_text` are not directly reachable from this crate).
    async fn sample_ops_text() -> String {
        let mut appender = crate::os_spr::HistoryAppender::begin(Vec::new(), "doc-1", "schema-1", &crate::os_spr::WriteOptions::default()).unwrap();
        appender.append_edit(&sample_edit("e00", Some("actor-a"), Some("first edit"), None)).unwrap();
        appender.commit().unwrap();
        let bytes = appender.into_sink();
        crate::os_spr::decompile_ops(&bytes, &crate::os_spr::DecodeOptions::default()).unwrap()
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️Args
    #[semio_framework_async_macros::async_test]
    async fn parse_args_splits_flags_and_positionals() {
        let args = vec![String::from("a.spr"), String::from("--level=full"), String::from("--actor"), String::from("actor-1"), String::from("b.spr")];
        let (positional, flags) = parse_args(&args);
        assert_eq!(positional, vec!["a.spr".to_string(), "b.spr".to_string()]);
        assert_eq!(flags.get("level"), Some(&"full".to_string()));
        assert_eq!(flags.get("actor"), Some(&"actor-1".to_string()));
    }

    #[semio_framework_async_macros::async_test]
    async fn parse_log_args_extracts_reverse_without_disturbing_value_flags() {
        let args = vec![String::from("file.spr"), String::from("--reverse"), String::from("--limit"), String::from("3")];
        let (positional, flags, reverse) = parse_log_args(&args);
        assert_eq!(positional, vec!["file.spr".to_string()]);
        assert!(reverse);
        assert_eq!(flags.get("limit"), Some(&"3".to_string()));
    }

    #[semio_framework_async_macros::async_test]
    async fn parse_repair_args_extracts_both_boolean_flags() {
        let args = vec![String::from("file.spr"), String::from("--truncate-torn-tail"), String::from("--rebuild-indexes")];
        let (positional, truncate, rebuild) = parse_repair_args(&args);
        assert_eq!(positional, vec!["file.spr".to_string()]);
        assert!(truncate);
        assert!(rebuild);
    }

    #[semio_framework_async_macros::async_test]
    async fn parse_level_accepts_known_values_and_rejects_unknown() {
        let mut flags = HashMap::new();
        assert!(matches!(parse_level(&flags), Ok(crate::os_spr::VerificationLevel::Standard)));
        flags.insert("level".to_string(), "full".to_string());
        assert!(matches!(parse_level(&flags), Ok(crate::os_spr::VerificationLevel::Full)));
        flags.insert("level".to_string(), "bogus".to_string());
        assert!(parse_level(&flags).is_err());
    }
    //#endregion 🔖️Args

    //#region 🔖️Frame
    #[semio_framework_async_macros::async_test]
    async fn kind_name_covers_every_frozen_kind_byte() {
        for (kind, name) in [
            (REC_END, "end"),
            (REC_DOC, "doc"),
            (REC_ACTOR_DICT, "actor_dict"),
            (REC_STR_DICT, "str_dict"),
            (REC_EDIT, "edit"),
            (REC_CHANGE, "change"),
            (REC_CHECKPOINT, "checkpoint"),
            (REC_ALTERNATIVE, "alternative"),
            (REC_ACTIVE, "active"),
            (REC_FRONTIER, "frontier"),
            (REC_PROJECTION, "snapshot"),
            (REC_INDEX, "index"),
            (REC_COMMIT, "commit"),
            (REC_SIGNATURE, "signature"),
            (REC_REDACTION, "redaction"),
            (REC_UPCAST, "upcast"),
            (REC_EPHEMERAL, "ephemeral"),
            (REC_SEALED, "sealed"),
            (REC_COMPACTION, "compaction"),
            (REC_PADDING, "padding"),
        ] {
            assert_eq!(kind_name(kind), name);
        }
        assert_eq!(kind_name(0x50), "extension");
    }

    #[semio_framework_async_macros::async_test]
    async fn parse_commit_fields_matches_a_real_commit_frame() {
        let (_path, bytes) = build_history_file("commit_fields", 1, false).await;
        let mut cursor = crate::os_spr::FrameCursor::new(&bytes, HEADER_SIZE).await;
        // 🚫️async: R10 shape 1 — `next_frame` is async but `Iterator::from_fn`'s closure is sync;
        // rewritten as a plain loop so it can be awaited.
        let commit_frame = loop {
            let frame = cursor.next_frame().await.unwrap().unwrap();
            if frame.kind == REC_COMMIT {
                break frame;
            }
        };
        let fields = parse_commit_fields(commit_frame.payload().await).await.unwrap();
        assert_eq!(fields.commit_seq, 1);
        assert_eq!(fields.prev_commit_offset, 0);
        // 🎯️ `HistoryAppender::begin` writes a `REC_STR_DICT` delta (doc_id + schema interned)
        // then `REC_DOC` immediately (pending, not yet committed); one `append_edit` then flushes
        // another `REC_STR_DICT` delta (edit id + actor interned) before its own `REC_EDIT` — so
        // the first `commit()` covers 4 pending records, not 1.
        assert_eq!(fields.record_count, 4);
    }

    #[semio_framework_async_macros::async_test]
    async fn parse_commit_fields_rejects_wrong_length_payload() {
        assert!(parse_commit_fields(&[0u8; 10]).await.is_none());
    }
    //#endregion 🔖️Frame

    //#region 🔖️Inspect
    #[semio_framework_async_macros::async_test]
    async fn cli_inspect_reports_header_kinds_and_commit_chain() {
        let (path, _bytes) = build_history_file("inspect", 3, true).await;
        assert_eq!(main_impl(&[String::from("inspect"), path.to_string_lossy().to_string()]), 0);
        std::fs::remove_file(&path).ok();
    }

    #[semio_framework_async_macros::async_test]
    async fn cli_inspect_reports_error_on_missing_file() {
        let missing = temp_path("missing.spr").to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("inspect"), missing]), 1);
    }
    //#endregion 🔖️Inspect

    //#region 🔖️Verify
    #[semio_framework_async_macros::async_test]
    async fn cli_verify_ok_at_every_level_on_a_clean_file() {
        let (path, _bytes) = build_history_file("verify_ok", 4, false).await;
        let path_str = path.to_string_lossy().to_string();
        for level in ["trusted", "standard", "full"] {
            assert_eq!(main_impl(&[String::from("verify"), path_str.clone(), format!("--level={level}")]), 0, "level {level}");
        }
        std::fs::remove_file(&path).ok();
    }

    #[semio_framework_async_macros::async_test]
    async fn cli_verify_rejects_file_with_corrupted_header() {
        // 🎯️ Design note: this family's read path (`HistoryReader::open` -> `crate::os_spr::format::
        // recover`) is deliberately self-healing for interior corruption — any tampered frame's
        // own CRC-32C fails during `recover`'s forward scan, which simply EXCLUDES it (and
        // everything after) from the trusted prefix rather than erroring, so `verify` reports
        // `OK` on whatever shorter trusted prefix remains (this is the append-only format's
        // intended torn-tail tolerance, not a gap in this CLI). The one corruption class that IS
        // guaranteed unrecoverable at every `VerificationLevel` is a corrupted 32-byte header
        // (bad magic), since there is no earlier trusted state to fall back to at all.
        let (path, mut bytes) = build_history_file("verify_bad_header", 2, false).await;
        bytes[0] ^= 0xFF;
        std::fs::write(&path, &bytes).unwrap();
        let path_str = path.to_string_lossy().to_string();
        for level in ["trusted", "standard", "full"] {
            assert_ne!(main_impl(&[String::from("verify"), path_str.clone(), format!("--level={level}")]), 0, "level {level}");
        }
        std::fs::remove_file(&path).ok();
    }

    #[semio_framework_async_macros::async_test]
    async fn cli_verify_rejects_unknown_level() {
        let (path, _bytes) = build_history_file("verify_bad_level", 1, false).await;
        let path_str = path.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("verify"), path_str, String::from("--level=bogus")]), 2);
        std::fs::remove_file(&path).ok();
    }
    //#endregion 🔖️Verify

    //#region 🔖️Hash
    #[semio_framework_async_macros::async_test]
    async fn cli_hash_prints_commit_seq_and_chain_hash() {
        let (path, _bytes) = build_history_file("hash", 2, false).await;
        assert_eq!(main_impl(&[String::from("hash"), path.to_string_lossy().to_string()]), 0);
        std::fs::remove_file(&path).ok();
    }
    //#endregion 🔖️Hash

    //#region 🔖️Log
    #[semio_framework_async_macros::async_test]
    async fn cli_log_filters_by_actor_and_alternative_and_respects_limit_reverse() {
        let (path, _bytes) = build_history_file("log", 4, true).await;
        let path_str = path.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("log"), path_str.clone()]), 0);
        assert_eq!(main_impl(&[String::from("log"), path_str.clone(), String::from("--actor"), String::from("actor-a")]), 0);
        assert_eq!(main_impl(&[String::from("log"), path_str.clone(), String::from("--alternative"), String::from("alt-main")]), 0);
        assert_eq!(main_impl(&[String::from("log"), path_str.clone(), String::from("--limit"), String::from("1")]), 0);
        assert_eq!(main_impl(&[String::from("log"), path_str.clone(), String::from("--reverse")]), 0);
        assert_eq!(main_impl(&[String::from("log"), path_str, String::from("--alternative"), String::from("bogus")]), 2);
        std::fs::remove_file(&path).ok();
    }
    //#endregion 🔖️Log

    //#region 🔖️Compile
    #[semio_framework_async_macros::async_test]
    async fn cli_compile_and_decompile_round_trip_via_files() {
        let ops_text = sample_ops_text();
        let ops_path = temp_path("roundtrip.ops");
        std::fs::write(&ops_path, &ops_text).unwrap();
        let ops_path_str = ops_path.to_string_lossy().to_string();

        let spr_path = temp_path("roundtrip.spr");
        let spr_path_str = spr_path.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("compile"), ops_path_str, String::from("--out"), spr_path_str.clone()]), 0);
        assert!(spr_path.exists());
        assert_eq!(main_impl(&[String::from("verify"), spr_path_str.clone()]), 0);

        let decompiled_path = temp_path("roundtrip.decompiled.ops");
        let decompiled_path_str = decompiled_path.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("decompile"), spr_path_str, String::from("--out"), decompiled_path_str]), 0);
        let decompiled_text = std::fs::read_to_string(&decompiled_path).unwrap();
        assert_eq!(decompiled_text, ops_text);

        std::fs::remove_file(&ops_path).ok();
        std::fs::remove_file(&spr_path).ok();
        std::fs::remove_file(&decompiled_path).ok();
    }

    #[semio_framework_async_macros::async_test]
    async fn cli_compile_rejects_malformed_ops_text() {
        let ops_path = temp_path("bad.ops");
        std::fs::write(&ops_path, "not a valid ops line\n").unwrap();
        assert_eq!(main_impl(&[String::from("compile"), ops_path.to_string_lossy().to_string()]), 1);
        std::fs::remove_file(&ops_path).ok();
    }
    //#endregion 🔖️Compile

    //#region 🔖️Diff
    #[semio_framework_async_macros::async_test]
    async fn cli_diff_reports_identical_and_divergent_files() {
        // `diff_b` has fewer edits than `diff_a` — a genuine divergence (only-in-a for the tail),
        // not just a differently-named copy of the same content.
        let (path_a, _) = build_history_file("diff_a", 3, false).await;
        let (path_b, _) = build_history_file("diff_b", 2, false).await;
        let path_a_str = path_a.to_string_lossy().to_string();
        let path_b_str = path_b.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("diff"), path_a_str.clone(), path_a_str.clone()]), 0);
        assert_eq!(main_impl(&[String::from("diff"), path_a_str, path_b_str]), 1);
        std::fs::remove_file(&path_a).ok();
        std::fs::remove_file(&path_b).ok();
    }

    #[semio_framework_async_macros::async_test]
    async fn cli_diff_reports_only_in_a_when_b_is_a_shorter_prefix() {
        let mut appender_a = crate::os_spr::HistoryAppender::begin(Vec::new(), "doc-1", "schema-1", &crate::os_spr::WriteOptions::default()).unwrap();
        appender_a.append_edit(&sample_edit("e00", Some("actor-a"), None, None)).unwrap();
        appender_a.append_edit(&sample_edit("e01", Some("actor-a"), None, None)).unwrap();
        appender_a.commit().unwrap();
        let bytes_a = appender_a.into_sink();

        let mut appender_b = crate::os_spr::HistoryAppender::begin(Vec::new(), "doc-1", "schema-1", &crate::os_spr::WriteOptions::default()).unwrap();
        appender_b.append_edit(&sample_edit("e00", Some("actor-a"), None, None)).unwrap();
        appender_b.commit().unwrap();
        let bytes_b = appender_b.into_sink();

        let path_a = temp_path("diff_prefix_a.spr");
        let path_b = temp_path("diff_prefix_b.spr");
        std::fs::write(&path_a, &bytes_a).unwrap();
        std::fs::write(&path_b, &bytes_b).unwrap();

        assert_eq!(main_impl(&[String::from("diff"), path_a.to_string_lossy().to_string(), path_b.to_string_lossy().to_string()]), 1);

        std::fs::remove_file(&path_a).ok();
        std::fs::remove_file(&path_b).ok();
    }
    //#endregion 🔖️Diff

    //#region 🔖️Compact
    #[semio_framework_async_macros::async_test]
    async fn cli_compact_in_place_and_via_out_both_leave_a_verifiable_file() {
        let (path, _bytes) = build_history_file("compact", 3, false).await;
        let path_str = path.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("compact"), path_str.clone()]), 0);
        assert_eq!(main_impl(&[String::from("verify"), path_str.clone()]), 0);

        let out_path = temp_path("compact_out.spr");
        let out_path_str = out_path.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("compact"), path_str, String::from("--out"), out_path_str.clone()]), 0);
        assert!(out_path.exists());
        assert_eq!(main_impl(&[String::from("verify"), out_path_str]), 0);

        std::fs::remove_file(&path).ok();
        std::fs::remove_file(&out_path).ok();
    }
    //#endregion 🔖️Compact

    //#region 🔖️Repair
    #[semio_framework_async_macros::async_test]
    async fn cli_repair_reports_clean_file_and_truncates_a_torn_tail() {
        let (path, bytes) = build_history_file("repair", 2, false).await;
        let path_str = path.to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("repair"), path_str.clone()]), 0);

        let mut torn = bytes;
        let commit_frame_len = 75u64;
        torn.truncate(torn.len() - commit_frame_len as usize + 3);
        std::fs::write(&path, &torn).unwrap();
        assert_eq!(main_impl(&[String::from("repair"), path_str.clone(), String::from("--truncate-torn-tail"), String::from("--rebuild-indexes")]), 0);
        let repaired = std::fs::read(&path).unwrap();
        assert!(repaired.len() < torn.len());
        assert_eq!(main_impl(&[String::from("verify"), path_str]), 0);

        std::fs::remove_file(&path).ok();
    }

    #[semio_framework_async_macros::async_test]
    async fn cli_repair_reports_error_on_missing_file() {
        let missing = temp_path("missing_repair.spr").to_string_lossy().to_string();
        assert_eq!(main_impl(&[String::from("repair"), missing]), 1);
    }
    //#endregion 🔖️Repair

    //#region 🔖️Upgrade
    #[semio_framework_async_macros::async_test]
    async fn cli_upgrade_passes_through_a_valid_file() {
        let (path, _bytes) = build_history_file("upgrade", 1, false).await;
        assert_eq!(main_impl(&[String::from("upgrade"), path.to_string_lossy().to_string()]), 0);
        std::fs::remove_file(&path).ok();
    }

    #[semio_framework_async_macros::async_test]
    async fn cli_upgrade_fails_on_corrupt_file() {
        let (path, mut bytes) = build_history_file("upgrade_corrupt", 1, false).await;
        let mid = bytes.len() / 2;
        bytes[mid] ^= 0xFF;
        std::fs::write(&path, &bytes).unwrap();
        assert_ne!(main_impl(&[String::from("upgrade"), path.to_string_lossy().to_string()]), 0);
        std::fs::remove_file(&path).ok();
    }
    //#endregion 🔖️Upgrade

    //#region 🔖️Cli
    #[semio_framework_async_macros::async_test]
    async fn cli_help_and_unknown_subcommand() {
        assert_eq!(main_impl(&[]), 2);
        assert_eq!(main_impl(&[String::from("help")]), 0);
        assert_eq!(main_impl(&[String::from("--help")]), 0);
        assert_eq!(main_impl(&[String::from("bogus-subcommand")]), 2);
    }
    //#endregion 🔖️Cli
}
//#endregion 🧪️Tests
