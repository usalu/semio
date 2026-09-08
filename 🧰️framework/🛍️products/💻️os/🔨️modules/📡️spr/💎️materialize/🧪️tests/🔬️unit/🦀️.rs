
use super::*;
use crate::os_pack::CodecId;
use crate::os_spr::format::{SprWriter, WriteOptions};
use crate::os_spr::history::{HistoryChange, HistoryCheckpoint, HistoryEdit, HistoryLog, OpPayload};
use crate::os_spr::wire::{DictBuilder, REC_EDIT, REQUIRED_HASH_CHAIN};

//#region 🔖️Snapshot
async fn sample_record(anchor: Option<&str>, ordinal: u64, kind: SnapshotBodyKind, body: Option<Vec<u8>>) -> SnapshotRecord {
    let body_hash = match body.as_deref() {
        Some(b) => Blake3Hasher.hash(b),
        None => [0u8; 32],
    };
    SnapshotRecord { anchor_checkpoint_id: anchor.map(str::to_string), edit_ordinal: ordinal, body_kind: kind, body_hash, body }
}

#[semio_framework_async_macros::async_test]
async fn snapshot_round_trips_embedded_with_checkpoint_anchor() {
    let record = sample_record(Some("cp-1"), 7, SnapshotBodyKind::EmbeddedPack, Some(vec![1, 2, 3, 4])).await;
    let bytes = encode_snapshot(&record);
    let decoded = decode_snapshot(&bytes.await).await.unwrap();
    assert_eq!(decoded, record);
}

#[semio_framework_async_macros::async_test]
async fn snapshot_round_trips_embedded_dsl_with_ordinal_only_anchor() {
    let record = sample_record(None, 0, SnapshotBodyKind::EmbeddedDsl, Some(b"(doc)".to_vec())).await;
    let bytes = encode_snapshot(&record);
    let decoded = decode_snapshot(&bytes.await).await.unwrap();
    assert_eq!(decoded, record);
}

#[semio_framework_async_macros::async_test]
async fn snapshot_round_trips_sidecar_without_body() {
    let record = sample_record(Some("cp-9"), 42, SnapshotBodyKind::SidecarPack, None).await;
    let bytes = encode_snapshot(&record);
    let decoded = decode_snapshot(&bytes.await).await.unwrap();
    assert_eq!(decoded.body, None);
    assert_eq!(decoded, record);
}

#[semio_framework_async_macros::async_test]
async fn snapshot_rejects_unknown_format() {
    let mut bytes = encode_snapshot(&sample_record(None, 0, SnapshotBodyKind::EmbeddedPack, Some(vec![9])).await).await;
    bytes[0] = 7;
    assert!(matches!(decode_snapshot(&bytes).await, Err(ProtocolError::Malformed { .. })));
}
//#endregion 🔖️Snapshot

//#region 🔖️Plan
fn sample_edit(id: &str, op_text: &str) -> HistoryEdit {
    HistoryEdit { id: id.to_string(), actor: None, started_at: format!("t-{id}"), finished_at: None, coalesce_key: None, description: None, ops: vec![OpPayload { text: Some(op_text.to_string()), binary: None }], inverse: Vec::new(), meta: None }
}

async fn flush_dict_delta<S: crate::os_pack::PackSink>(writer: &mut SprWriter<S>, dict: &DictBuilder, base: &mut u32) {
    let len = dict.len();
    if len > *base {
        let entries = dict.entries_since(*base);
        let mut payload = ByteWriter::new();
        payload.write_u8(1);
        payload.write_varint_u64(*base as u64);
        payload.write_varint_u64(entries.len() as u64);
        for entry in entries {
            payload.write_varint_u64(entry.len() as u64);
            payload.write_bytes(entry.as_bytes());
        }
        writer.write_record(crate::os_spr::REC_STR_DICT, true, &payload.into_bytes(), CodecId(0)).await.unwrap();
        *base = len;
    }
}

/// @emoji 🏗️ Hand-assembles a `.spr` stream with 4 edits and one embedded-pack `REC_PROJECTION`
/// taken right after edit ordinal 1 (i.e. covering edits 0 and 1) — the shape `resolve_plan`'s
/// index-free reverse-scan fallback and `materialize_with`'s tail replay are exercised against.
async fn build_stream_with_snapshot(snapshot_body: &[u8]) -> Vec<u8> {
    let write_options = WriteOptions { required_flags: REQUIRED_HASH_CHAIN, optional_flags: 0 };
    let mut writer = SprWriter::begin(Vec::<u8>::new(), &write_options).await.unwrap();
    let mut dict = DictBuilder::new();
    let mut dict_base = 0u32;

    let doc_payload = crate::os_spr::history::encode_doc("doc-1", "schema-1", &mut dict).await;
    flush_dict_delta(&mut writer, &dict, &mut dict_base).await;
    writer.write_record(crate::os_spr::REC_DOC, true, &doc_payload, CodecId(0)).await.unwrap();

    for (i, edit) in [sample_edit("edit-0", "op-0"), sample_edit("edit-1", "op-1")].iter().enumerate() {
        let _ = i;
        let payload = crate::os_spr::history::encode_edit(edit, &mut dict, |_| None).await.unwrap();
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await;
        writer.write_record(REC_EDIT, true, &payload, CodecId(0)).await.unwrap();
    }

    let snapshot = sample_record(None, 1, SnapshotBodyKind::EmbeddedPack, Some(snapshot_body.to_vec())).await;
    writer.write_record(crate::os_spr::REC_PROJECTION, false, &encode_snapshot(&snapshot).await, CodecId(0)).await.unwrap();

    for edit in [sample_edit("edit-2", "op-2"), sample_edit("edit-3", "op-3")] {
        let payload = crate::os_spr::history::encode_edit(&edit, &mut dict, |_| None).await.unwrap();
        flush_dict_delta(&mut writer, &dict, &mut dict_base).await;
        writer.write_record(REC_EDIT, true, &payload, CodecId(0)).await.unwrap();
    }

    writer.commit().await.unwrap();
    writer.into_sink().await
}

type Collected = (Vec<u8>, Vec<String>);

// 🔒️ Must return `Result` to satisfy `materialize_with`'s `apply_edit` closure bound even
// though this particular collector never fails.
#[allow(clippy::unnecessary_wraps)]
async fn collect_ids(p: &mut Collected, edit: &HistoryEdit) -> Result<(), ProtocolError> {
    p.1.push(edit.id.clone());
    Ok(())
}

#[semio_framework_async_macros::async_test]
async fn resolve_plan_picks_snapshot_and_replays_only_the_tail() {
    let body = vec![0xAA, 0xBB, 0xCC];
    let bytes = build_stream_with_snapshot(&body).await;

    let plan = resolve_plan(&bytes, &[], MaterializeTarget::LatestOnActive, &ProtocolLimits::default()).await.unwrap();
    assert_eq!(plan.base.applied_edits, 2);
    assert_eq!(plan.target_edit_ordinal, None);
    assert_eq!(plan.skipped_corrupt, 0);
    match plan.base.bytes {
        BaseBytes::Borrowed(b) => assert_eq!(b, body.as_slice()),
        BaseBytes::Sidecar { .. } => panic!("expected an embedded base"),
    }

    let (result, report) = materialize_with::<Collected, ProtocolError>(plan, &bytes, |b| Ok((b.to_vec(), Vec::new())), collect_ids).await.unwrap();
    assert_eq!(result.0, body);
    assert_eq!(result.1, vec!["edit-2".to_string(), "edit-3".to_string()]);
    assert!(!report.genesis_replay);
    assert_eq!(report.snapshot_used, Some((None, 2)));
    assert_eq!(report.edits_replayed, 2);
    assert_eq!(report.snapshots_skipped_corrupt, 0);
    assert!(report.bytes_read > 0);
}

#[semio_framework_async_macros::async_test]
async fn resolve_plan_falls_back_to_initial_pack_when_target_precedes_every_snapshot() {
    let bytes = build_stream_with_snapshot(&[0xAA, 0xBB, 0xCC]).await;
    let initial_pack = b"INIT";

    let plan = resolve_plan(&bytes, initial_pack, MaterializeTarget::AtEditOrdinal(0), &ProtocolLimits::default()).await.unwrap();
    assert_eq!(plan.base.applied_edits, 0);
    match plan.base.bytes {
        BaseBytes::Borrowed(b) => assert_eq!(b, initial_pack),
        BaseBytes::Sidecar { .. } => panic!("expected the initial pack"),
    }

    let (result, report) = materialize_with::<Collected, ProtocolError>(plan, &bytes, |b| Ok((b.to_vec(), Vec::new())), collect_ids).await.unwrap();
    assert_eq!(result.0, initial_pack);
    assert_eq!(result.1, vec!["edit-0".to_string()]);
    assert!(report.genesis_replay);
    assert_eq!(report.snapshot_used, None);
    assert_eq!(report.edits_replayed, 1);
}

#[semio_framework_async_macros::async_test]
async fn resolve_plan_at_edit_ordinal_beyond_snapshot_replays_full_tail() {
    let body = vec![0xAA, 0xBB, 0xCC];
    let bytes = build_stream_with_snapshot(&body).await;

    let plan = resolve_plan(&bytes, &[], MaterializeTarget::AtEditOrdinal(3), &ProtocolLimits::default()).await.unwrap();
    let (result, report) = materialize_with::<Collected, ProtocolError>(plan, &bytes, |b| Ok((b.to_vec(), Vec::new())), collect_ids).await.unwrap();
    assert_eq!(result.1, vec!["edit-2".to_string(), "edit-3".to_string()]);
    assert_eq!(report.edits_replayed, 2);
}

#[semio_framework_async_macros::async_test]
async fn resolve_plan_at_checkpoint_falls_back_to_full_decode_without_an_index() {
    let mut log = HistoryLog { doc_id: "doc-2".to_string(), schema: "schema-2".to_string(), ..Default::default() };
    log.edits.push(sample_edit("edit-0", "op-0"));
    log.edits.push(sample_edit("edit-1", "op-1"));
    log.changes.push(HistoryChange { id: "change-1".to_string(), saved_at: "t-change-1".to_string(), edit_ids: vec!["edit-0".to_string(), "edit-1".to_string()], description: None });
    log.checkpoints.push(HistoryCheckpoint { id: "cp-1".to_string(), timestamp: "t-cp-1".to_string(), change_ids: vec!["change-1".to_string()], parent_id: None, authors: Vec::new(), message: None });

    let bytes = crate::os_spr::history::encode_history(&log, &crate::os_spr::history::EncodeOptions::default()).await.unwrap();
    let initial_pack = b"BASE";

    let plan = resolve_plan(&bytes, initial_pack, MaterializeTarget::AtCheckpoint("cp-1".to_string()), &ProtocolLimits::default()).await.unwrap();
    assert_eq!(plan.target_edit_ordinal, Some(1));
    assert_eq!(plan.base.applied_edits, 0); // no REC_PROJECTION in this stream at all

    let (result, _report) = materialize_with::<Collected, ProtocolError>(plan, &bytes, |b| Ok((b.to_vec(), Vec::new())), collect_ids).await.unwrap();
    assert_eq!(result.0, initial_pack);
    assert_eq!(result.1, vec!["edit-0".to_string(), "edit-1".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn resolve_plan_skips_a_corrupt_snapshot_and_falls_back_to_initial_pack() {
    let write_options = WriteOptions { required_flags: REQUIRED_HASH_CHAIN, optional_flags: 0 };
    let mut writer = SprWriter::begin(Vec::<u8>::new(), &write_options).await.unwrap();
    let mut dict = DictBuilder::new();
    let mut dict_base = 0u32;

    let doc_payload = crate::os_spr::history::encode_doc("doc-3", "schema-3", &mut dict).await;
    flush_dict_delta(&mut writer, &dict, &mut dict_base).await;
    writer.write_record(crate::os_spr::REC_DOC, true, &doc_payload, CodecId(0)).await.unwrap();

    // A snapshot whose stored body_hash does not match its body — must be treated as corrupt.
    let mut bad_record = sample_record(None, 0, SnapshotBodyKind::EmbeddedPack, Some(vec![1, 2, 3])).await;
    bad_record.body_hash = [0xFFu8; 32];
    writer.write_record(crate::os_spr::REC_PROJECTION, false, &encode_snapshot(&bad_record).await, CodecId(0)).await.unwrap();

    let payload = crate::os_spr::history::encode_edit(&sample_edit("edit-0", "op-0"), &mut dict, |_| None).await.unwrap();
    flush_dict_delta(&mut writer, &dict, &mut dict_base).await;
    writer.write_record(REC_EDIT, true, &payload, CodecId(0)).await.unwrap();
    writer.commit().await.unwrap();
    let bytes = writer.into_sink().await;

    let plan = resolve_plan(&bytes, b"INIT", MaterializeTarget::LatestOnActive, &ProtocolLimits::default()).await.unwrap();
    assert_eq!(plan.base.applied_edits, 0);
    assert_eq!(plan.skipped_corrupt, 1);
    match plan.base.bytes {
        BaseBytes::Borrowed(b) => assert_eq!(b, b"INIT"),
        BaseBytes::Sidecar { .. } => panic!("expected the initial pack fallback"),
    }
}
//#endregion 🔖️Plan

//#region 🔖️Policy
#[semio_framework_async_macros::async_test]
async fn checkpoint_policy_default_matches_documented_values() {
    let policy = CheckpointPolicy::default();
    assert_eq!(policy.every_edits, 512);
    assert_eq!(policy.every_bytes, 4 * 1024 * 1024);
    assert!(policy.on_checkpoint_commit);
    assert_eq!(policy.embed_below, 1024 * 1024);
}

#[semio_framework_async_macros::async_test]
async fn checkpoint_policy_triggers_on_any_threshold() {
    let policy = CheckpointPolicy::default();
    assert!(policy.should_checkpoint(512, 0, false).await);
    assert!(policy.should_checkpoint(0, 4 * 1024 * 1024, false).await);
    assert!(policy.should_checkpoint(0, 0, true).await);
    assert!(!policy.should_checkpoint(1, 1, false).await);
    assert!(policy.should_embed(1024).await);
    assert!(!policy.should_embed(2 * 1024 * 1024).await);
}
//#endregion 🔖️Policy
