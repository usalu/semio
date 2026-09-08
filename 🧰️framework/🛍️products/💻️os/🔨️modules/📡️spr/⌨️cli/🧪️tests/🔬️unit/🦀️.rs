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
    let mut appender = crate::os_spr::HistoryAppender::begin(Vec::new(), "doc-1", "schema-1", &crate::os_spr::WriteOptions::default()).await.unwrap();
    let mut edit_ids = Vec::new();
    for i in 0..edit_count {
        let id = format!("e{i:02}");
        let actor = if i % 2 == 0 { Some("actor-a") } else { Some("actor-b") };
        appender.append_edit(&sample_edit(&id, actor, Some("an edit"), None).await).await.unwrap();
        appender.commit().await.unwrap();
        edit_ids.push(id);
    }
    if with_checkpoint_and_alternative && !edit_ids.is_empty() {
        appender.append_change(&crate::os_spr::HistoryChange { id: "c0".to_string(), saved_at: "2026-07-27T00:01:00Z".to_string(), edit_ids: edit_ids.clone(), description: None }).await.unwrap();
        appender.append_checkpoint(&crate::os_spr::HistoryCheckpoint { id: "cp0".to_string(), timestamp: "2026-07-27T00:02:00Z".to_string(), change_ids: vec!["c0".to_string()], parent_id: None, authors: Vec::new(), message: None }).await.unwrap();
        appender.append_alternative(&crate::os_spr::HistoryAlternative { id: "alt-main".to_string(), name: "main".to_string(), checkpoint_ids: vec!["cp0".to_string()] }).await.unwrap();
        appender.set_active(Some("alt-main")).await.unwrap();
        appender.commit().await.unwrap();
    }
    let bytes = appender.into_sink().await;
    let path = temp_path(name).await;
    std::fs::write(&path, &bytes).unwrap();
    (path, bytes)
}

/// 🧪️ Round-trips a small `HistoryLog` through `HistoryAppender` -> `decompile_ops` to obtain
/// ground-truth `.ops` text without hand-writing the grammar (see the module's design note on
/// why `parse_ops_text`/`print_ops_text` are not directly reachable from this crate).
async fn sample_ops_text() -> String {
    let mut appender = crate::os_spr::HistoryAppender::begin(Vec::new(), "doc-1", "schema-1", &crate::os_spr::WriteOptions::default()).await.unwrap();
    appender.append_edit(&sample_edit("e00", Some("actor-a"), Some("first edit"), None).await).await.unwrap();
    appender.commit().await.unwrap();
    let bytes = appender.into_sink();
    crate::os_spr::decompile_ops(&bytes.await, &crate::os_spr::DecodeOptions::default()).await.unwrap()
}
//#endregion 🔖️Fixtures

//#region 🔖️Args
#[semio_framework_async_macros::async_test]
async fn parse_args_splits_flags_and_positionals() {
    let args = vec![String::from("a.spr"), String::from("--level=full"), String::from("--actor"), String::from("actor-1"), String::from("b.spr")];
    let (positional, flags) = parse_args(&args).await;
    assert_eq!(positional, vec!["a.spr".to_string(), "b.spr".to_string()]);
    assert_eq!(flags.get("level"), Some(&"full".to_string()));
    assert_eq!(flags.get("actor"), Some(&"actor-1".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn parse_log_args_extracts_reverse_without_disturbing_value_flags() {
    let args = vec![String::from("file.spr"), String::from("--reverse"), String::from("--limit"), String::from("3")];
    let (positional, flags, reverse) = parse_log_args(&args).await;
    assert_eq!(positional, vec!["file.spr".to_string()]);
    assert!(reverse);
    assert_eq!(flags.get("limit"), Some(&"3".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn parse_repair_args_extracts_both_boolean_flags() {
    let args = vec![String::from("file.spr"), String::from("--truncate-torn-tail"), String::from("--rebuild-indexes")];
    let (positional, truncate, rebuild) = parse_repair_args(&args).await;
    assert_eq!(positional, vec!["file.spr".to_string()]);
    assert!(truncate);
    assert!(rebuild);
}

#[semio_framework_async_macros::async_test]
async fn parse_level_accepts_known_values_and_rejects_unknown() {
    let mut flags = HashMap::new();
    assert!(matches!(parse_level(&flags).await, Ok(crate::os_spr::VerificationLevel::Standard)));
    flags.insert("level".to_string(), "full".to_string());
    assert!(matches!(parse_level(&flags).await, Ok(crate::os_spr::VerificationLevel::Full)));
    flags.insert("level".to_string(), "bogus".to_string());
    assert!(parse_level(&flags).await.is_err());
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
    assert_eq!(main_impl(&[String::from("inspect"), path.to_string_lossy().to_string()]).await, 0);
    std::fs::remove_file(&path).ok();
}

#[semio_framework_async_macros::async_test]
async fn cli_inspect_reports_error_on_missing_file() {
    let missing = temp_path("missing.spr").await.to_string_lossy().to_string();
    assert_eq!(main_impl(&[String::from("inspect"), missing]).await, 1);
}
//#endregion 🔖️Inspect

//#region 🔖️Verify
#[semio_framework_async_macros::async_test]
async fn cli_verify_ok_at_every_level_on_a_clean_file() {
    let (path, _bytes) = build_history_file("verify_ok", 4, false).await;
    let path_str = path.to_string_lossy().to_string();
    for level in ["trusted", "standard", "full"] {
        assert_eq!(main_impl(&[String::from("verify"), path_str.clone(), format!("--level={level}")]).await, 0, "level {level}");
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
        assert_ne!(main_impl(&[String::from("verify"), path_str.clone(), format!("--level={level}")]).await, 0, "level {level}");
    }
    std::fs::remove_file(&path).ok();
}

#[semio_framework_async_macros::async_test]
async fn cli_verify_rejects_unknown_level() {
    let (path, _bytes) = build_history_file("verify_bad_level", 1, false).await;
    let path_str = path.to_string_lossy().to_string();
    assert_eq!(main_impl(&[String::from("verify"), path_str, String::from("--level=bogus")]).await, 2);
    std::fs::remove_file(&path).ok();
}
//#endregion 🔖️Verify

//#region 🔖️Hash
#[semio_framework_async_macros::async_test]
async fn cli_hash_prints_commit_seq_and_chain_hash() {
    let (path, _bytes) = build_history_file("hash", 2, false).await;
    assert_eq!(main_impl(&[String::from("hash"), path.to_string_lossy().to_string()]).await, 0);
    std::fs::remove_file(&path).ok();
}
//#endregion 🔖️Hash

//#region 🔖️Log
#[semio_framework_async_macros::async_test]
async fn cli_log_filters_by_actor_and_alternative_and_respects_limit_reverse() {
    let (path, _bytes) = build_history_file("log", 4, true).await;
    let path_str = path.to_string_lossy().to_string();
    assert_eq!(main_impl(&[String::from("log"), path_str.clone()]).await, 0);
    assert_eq!(main_impl(&[String::from("log"), path_str.clone(), String::from("--actor"), String::from("actor-a")]).await, 0);
    assert_eq!(main_impl(&[String::from("log"), path_str.clone(), String::from("--alternative"), String::from("alt-main")]).await, 0);
    assert_eq!(main_impl(&[String::from("log"), path_str.clone(), String::from("--limit"), String::from("1")]).await, 0);
    assert_eq!(main_impl(&[String::from("log"), path_str.clone(), String::from("--reverse")]).await, 0);
    assert_eq!(main_impl(&[String::from("log"), path_str, String::from("--alternative"), String::from("bogus")]).await, 2);
    std::fs::remove_file(&path).ok();
}
//#endregion 🔖️Log

//#region 🔖️Compile
#[semio_framework_async_macros::async_test]
async fn cli_compile_and_decompile_round_trip_via_files() {
    let ops_text = sample_ops_text().await;
    let ops_path = temp_path("roundtrip.ops").await;
    std::fs::write(&ops_path, &ops_text).unwrap();
    let ops_path_str = ops_path.to_string_lossy().to_string();

    let spr_path = temp_path("roundtrip.spr").await;
    let spr_path_str = spr_path.to_string_lossy().to_string();
    assert_eq!(main_impl(&[String::from("compile"), ops_path_str, String::from("--out"), spr_path_str.clone()]).await, 0);
    assert!(spr_path.exists());
    assert_eq!(main_impl(&[String::from("verify"), spr_path_str.clone()]).await, 0);

    let decompiled_path = temp_path("roundtrip.decompiled.ops").await;
    let decompiled_path_str = decompiled_path.to_string_lossy().to_string();
    assert_eq!(main_impl(&[String::from("decompile"), spr_path_str, String::from("--out"), decompiled_path_str]).await, 0);
    let decompiled_text = std::fs::read_to_string(&decompiled_path).unwrap();
    assert_eq!(decompiled_text, ops_text);

    std::fs::remove_file(&ops_path).ok();
    std::fs::remove_file(&spr_path).ok();
    std::fs::remove_file(&decompiled_path).ok();
}

#[semio_framework_async_macros::async_test]
async fn cli_compile_rejects_malformed_ops_text() {
    let ops_path = temp_path("bad.ops").await;
    std::fs::write(&ops_path, "not a valid ops line\n").unwrap();
    assert_eq!(main_impl(&[String::from("compile"), ops_path.to_string_lossy().to_string()]).await, 1);
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
    assert_eq!(main_impl(&[String::from("diff"), path_a_str.clone(), path_a_str.clone()]).await, 0);
    assert_eq!(main_impl(&[String::from("diff"), path_a_str, path_b_str]).await, 1);
    std::fs::remove_file(&path_a).ok();
    std::fs::remove_file(&path_b).ok();
}

#[semio_framework_async_macros::async_test]
async fn cli_diff_reports_only_in_a_when_b_is_a_shorter_prefix() {
    let mut appender_a = crate::os_spr::HistoryAppender::begin(Vec::new(), "doc-1", "schema-1", &crate::os_spr::WriteOptions::default()).await.unwrap();
    appender_a.append_edit(&sample_edit("e00", Some("actor-a"), None, None).await).await.unwrap();
    appender_a.append_edit(&sample_edit("e01", Some("actor-a"), None, None).await).await.unwrap();
    appender_a.commit().await.unwrap();
    let bytes_a = appender_a.into_sink();

    let mut appender_b = crate::os_spr::HistoryAppender::begin(Vec::new(), "doc-1", "schema-1", &crate::os_spr::WriteOptions::default()).await.unwrap();
    appender_b.append_edit(&sample_edit("e00", Some("actor-a"), None, None).await).await.unwrap();
    appender_b.commit().await.unwrap();
    let bytes_b = appender_b.into_sink();

    let path_a = temp_path("diff_prefix_a.spr").await;
    let path_b = temp_path("diff_prefix_b.spr").await;
    std::fs::write(&path_a, &bytes_a.await).unwrap();
    std::fs::write(&path_b, &bytes_b.await).unwrap();

    assert_eq!(main_impl(&[String::from("diff"), path_a.to_string_lossy().to_string(), path_b.to_string_lossy().to_string()]).await, 1);

    std::fs::remove_file(&path_a).ok();
    std::fs::remove_file(&path_b).ok();
}
//#endregion 🔖️Diff

//#region 🔖️Compact
#[semio_framework_async_macros::async_test]
async fn cli_compact_in_place_and_via_out_both_leave_a_verifiable_file() {
    let (path, _bytes) = build_history_file("compact", 3, false).await;
    let path_str = path.to_string_lossy().to_string();
    assert_eq!(main_impl(&[String::from("compact"), path_str.clone()]).await, 0);
    assert_eq!(main_impl(&[String::from("verify"), path_str.clone()]).await, 0);

    let out_path = temp_path("compact_out.spr").await;
    let out_path_str = out_path.to_string_lossy().to_string();
    assert_eq!(main_impl(&[String::from("compact"), path_str, String::from("--out"), out_path_str.clone()]).await, 0);
    assert!(out_path.exists());
    assert_eq!(main_impl(&[String::from("verify"), out_path_str]).await, 0);

    std::fs::remove_file(&path).ok();
    std::fs::remove_file(&out_path).ok();
}
//#endregion 🔖️Compact

//#region 🔖️Repair
#[semio_framework_async_macros::async_test]
async fn cli_repair_reports_clean_file_and_truncates_a_torn_tail() {
    let (path, bytes) = build_history_file("repair", 2, false).await;
    let path_str = path.to_string_lossy().to_string();
    assert_eq!(main_impl(&[String::from("repair"), path_str.clone()]).await, 0);

    let mut torn = bytes;
    let commit_frame_len = 75u64;
    torn.truncate(torn.len() - commit_frame_len as usize + 3);
    std::fs::write(&path, &torn).unwrap();
    assert_eq!(main_impl(&[String::from("repair"), path_str.clone(), String::from("--truncate-torn-tail"), String::from("--rebuild-indexes")]).await, 0);
    let repaired = std::fs::read(&path).unwrap();
    assert!(repaired.len() < torn.len());
    assert_eq!(main_impl(&[String::from("verify"), path_str]).await, 0);

    std::fs::remove_file(&path).ok();
}

#[semio_framework_async_macros::async_test]
async fn cli_repair_reports_error_on_missing_file() {
    let missing = temp_path("missing_repair.spr").await.to_string_lossy().to_string();
    assert_eq!(main_impl(&[String::from("repair"), missing]).await, 1);
}
//#endregion 🔖️Repair

//#region 🔖️Upgrade
#[semio_framework_async_macros::async_test]
async fn cli_upgrade_passes_through_a_valid_file() {
    let (path, _bytes) = build_history_file("upgrade", 1, false).await;
    assert_eq!(main_impl(&[String::from("upgrade"), path.to_string_lossy().to_string()]).await, 0);
    std::fs::remove_file(&path).ok();
}

#[semio_framework_async_macros::async_test]
async fn cli_upgrade_fails_on_corrupt_file() {
    let (path, mut bytes) = build_history_file("upgrade_corrupt", 1, false).await;
    let mid = bytes.len() / 2;
    bytes[mid] ^= 0xFF;
    std::fs::write(&path, &bytes).unwrap();
    assert_ne!(main_impl(&[String::from("upgrade"), path.to_string_lossy().to_string()]).await, 0);
    std::fs::remove_file(&path).ok();
}
//#endregion 🔖️Upgrade

//#region 🔖️Cli
#[semio_framework_async_macros::async_test]
async fn cli_help_and_unknown_subcommand() {
    assert_eq!(main_impl(&[]).await, 2);
    assert_eq!(main_impl(&[String::from("help")]).await, 0);
    assert_eq!(main_impl(&[String::from("--help")]).await, 0);
    assert_eq!(main_impl(&[String::from("bogus-subcommand")]).await, 2);
}
//#endregion 🔖️Cli
