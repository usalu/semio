use super::*;

/// 🧪️ Builds a tiny in-memory `.spr` buffer with `edit_count` edits (ids `"e0".."eN"`), each in
/// its own commit generation so multi-commit paths (`extract_range`, `content_frontier`) get
/// exercised too. Returns the raw bytes.
async fn build_history_bytes(doc_id: &str, schema: &str, edit_count: usize) -> Vec<u8> {
    let mut appender = HistoryAppender::begin(Vec::new(), doc_id, schema, &WriteOptions::default()).await.unwrap();
    for i in 0..edit_count {
        let edit = HistoryEdit {
            id: format!("e{i}"),
            actor: None,
            started_at: format!("2026-07-27T00:00:{i:02}Z"),
            finished_at: None,
            coalesce_key: None,
            description: None,
            ops: vec![OpPayload { text: Some(format!("op-{i}")), binary: None }],
            inverse: Vec::new(),
            meta: None,
        };
        appender.append_edit(&edit).await.unwrap();
        appender.commit().await.unwrap();
    }
    appender.into_sink().await
}

//#region 🔖️Reexports
#[semio_framework_async_macros::async_test]
async fn reexported_types_match_sibling_crate_shapes() {
    let limits = ProtocolLimits::default();
    assert!(limits.max_file_len > 0);
    let hlt = HybridLogicalTimestamp::new(1, 1000);
    assert_eq!(hlt.physical_ms, 1000);
    let _ = ActorId("actor-1".to_string());
    let _ = ArtifactId("doc-1".to_string());
}
//#endregion 🔖️Reexports

//#region 🔖️Compile
#[semio_framework_async_macros::async_test]
async fn compile_ops_decompile_ops_round_trip() {
    let log = HistoryLog {
        doc_id: "doc-1".to_string(),
        schema: "schema-1".to_string(),
        edits: vec![HistoryEdit {
            id: "e0".to_string(),
            actor: Some("actor-1".to_string()),
            started_at: "2026-07-27T00:00:00Z".to_string(),
            finished_at: Some("2026-07-27T00:00:01Z".to_string()),
            coalesce_key: None,
            description: Some("first edit".to_string()),
            ops: vec![OpPayload { text: Some("set foo = 1".to_string()), binary: None }],
            inverse: Vec::new(),
            meta: None,
        }],
        changes: Vec::new(),
        checkpoints: Vec::new(),
        alternatives: Vec::new(),
        active_alternative_id: None,
        // 🎯️ W4: cursor is text-representable (unlike inverse, which is `.spr`-only) —
        // include one here to prove the compile_ops/decompile_ops text-tooling path preserves
        // it byte-for-byte, same as every other structural line.
        cursor: Some(HistoryCursor { applied_edit_ids: vec!["e0".to_string()], redo_edit_ids: Vec::new(), checkpoint_id: None }),
        composition: None,
        conflicts: Vec::new(),
    };
    let ops_text = print_ops_text(&log).unwrap();

    let compiled = compile_ops(&ops_text, &EncodeOptions::default()).await.unwrap();
    let decompiled = decompile_ops(&compiled, &DecodeOptions::default()).await.unwrap();

    assert_eq!(parse_ops_text(&decompiled).unwrap(), parse_ops_text(&ops_text).unwrap());
    assert_eq!(parse_ops_text(&decompiled).unwrap().cursor, log.cursor);
}

#[semio_framework_async_macros::async_test]
async fn compile_ops_rejects_malformed_text() {
    assert!(compile_ops("not a valid ops line", &EncodeOptions::default()).await.is_err());
}
//#endregion 🔖️Compile

//#region 🔖️Sync
#[semio_framework_async_macros::async_test]
async fn extract_range_returns_contiguous_slice_covering_requested_edits() {
    let bytes = build_history_bytes("doc-1", "schema-1", 4).await;

    // Independently compute each edit frame's [offset, offset+frame_len) span for cross-check.
    let mut cursor = FrameCursor::new(&bytes, crate::os_spr::format::HEADER_SIZE as u64).await;
    let mut edit_spans = Vec::new();
    while let Some(frame) = cursor.next_frame().await.unwrap() {
        if frame.kind == crate::os_spr::REC_EDIT {
            edit_spans.push((frame.offset, frame.offset + frame.frame_len().await));
        }
    }
    assert_eq!(edit_spans.len(), 4);

    let slice = extract_range(&bytes, 1..3).await.unwrap();
    assert_eq!(slice.first_edit_ordinal, 1);
    assert_eq!(slice.last_edit_ordinal, 2);
    assert_eq!(slice.count, 2);
    assert_eq!(slice.bytes, &bytes[edit_spans[1].0 as usize..edit_spans[2].1 as usize]);

    // The slice must itself be a structurally valid record stream (re-parseable from offset 0).
    let mut inner = FrameCursor::new(slice.bytes, 0).await;
    let mut edit_kinds_in_slice = 0;
    while let Some(frame) = inner.next_frame().await.unwrap() {
        if frame.kind == crate::os_spr::REC_EDIT {
            edit_kinds_in_slice += 1;
        }
    }
    assert_eq!(edit_kinds_in_slice, 2);
}

#[semio_framework_async_macros::async_test]
async fn extract_range_rejects_empty_and_out_of_bounds_ranges() {
    let bytes = build_history_bytes("doc-1", "schema-1", 2).await;
    assert!(extract_range(&bytes, 1..1).await.is_err());
    assert!(extract_range(&bytes, 0..5).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn verify_slice_accepts_genuine_content_and_rejects_tamper() {
    let bytes = build_history_bytes("doc-1", "schema-1", 3).await;
    let slice = extract_range(&bytes, 0..2).await.unwrap();

    let expected = slice_content_chain(slice.bytes).await.unwrap();
    assert!(verify_slice(slice.bytes, &expected).await.is_ok());

    let mut tampered = slice.bytes.to_vec();
    tampered[0] ^= 0xFF;
    assert!(verify_slice(&tampered, &expected).await.is_err());

    assert!(verify_slice(slice.bytes, &[0u8; 32]).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn content_frontier_reports_head_edit_and_chain_tip() {
    let bytes = build_history_bytes("doc-1", "schema-1", 3).await;
    let frontier = content_frontier(&bytes).await.unwrap();

    assert_eq!(frontier.document_id, "doc-1");
    assert_eq!(frontier.head_edit_ordinal, 2);
    assert_eq!(frontier.head_edit_id, "e2");
    assert_eq!(frontier.last_commit_seq, 3);
    assert!(frontier.alternatives.is_empty());

    // Cross-check chain_hash independently via the last REC_COMMIT frame's payload.
    let mut reverse = ReverseFrameCursor::at_end(&bytes[crate::os_spr::format::HEADER_SIZE..]).await;
    let last_commit = loop {
        let frame = reverse.prev_frame().await.unwrap().unwrap();
        if frame.kind == crate::os_spr::REC_COMMIT {
            break frame;
        }
    };
    let expected = crate::os_spr::format::parse_commit_payload(last_commit.payload().await).unwrap();
    assert_eq!(frontier.chain_hash, expected.chain_hash);
}

#[semio_framework_async_macros::async_test]
async fn content_frontier_on_empty_history_reports_zero_head_and_no_alternatives() {
    let bytes = build_history_bytes("doc-1", "schema-1", 0);
    let frontier = content_frontier(&bytes.await).await.unwrap();
    assert_eq!(frontier.head_edit_ordinal, 0);
    assert_eq!(frontier.head_edit_id, "");
    assert_eq!(frontier.last_commit_seq, 0);
}
//#endregion 🔖️Sync
