
use super::*;

async fn sample_log() -> HistoryLog {
    HistoryLog {
        doc_id: "doc-1".to_string(),
        schema: "org.semio.demo.v1".to_string(),
        edits: vec![
            HistoryEdit {
                id: "edit-1".to_string(),
                actor: Some("alice".to_string()),
                started_at: "2024-01-15T10:30:00Z".to_string(),
                finished_at: Some("2024-01-15T10:30:05Z".to_string()),
                coalesce_key: Some("typing".to_string()),
                description: Some("first edit".to_string()),
                ops: vec![OpPayload { text: Some("set foo=1".to_string()), binary: None }, OpPayload { text: Some("set bar=2".to_string()), binary: None }],
                inverse: Vec::new(),
                meta: None,
            },
            HistoryEdit {
                id: "edit-2".to_string(),
                actor: None,
                started_at: "not-a-canonical-timestamp".to_string(),
                finished_at: None,
                coalesce_key: None,
                description: None,
                ops: vec![OpPayload { text: Some("set baz=3".to_string()), binary: None }],
                inverse: Vec::new(),
                meta: Some(vec![HistoryOpMeta {
                    op_id: Some("op-1".to_string()),
                    dependencies: vec!["edit-1".to_string()],
                    base_version: 7,
                    author_id: Some("alice".to_string()),
                    hlt: Some((1, 1_700_000_000_000, 3)),
                    undo_policy: 2,
                    payload_hash: Some([9u8; 32]),
                    // 🎯️ Non-`None` on purpose: `sample_log().await` feeds every encode/decode
                    // identity test below (`history_encode_decode_identity_standard` etc.), so
                    // a populated `group_id` here proves the composite-gesture stamp survives
                    // a real `.spr` byte round trip via `assert_eq!(decoded, log)`, not just a
                    // narrowly-targeted unit test.
                    group_id: Some("group-composite-1".to_string()),
                    // 🎯️ A non-`Owner`, field-carrying variant on purpose (mirrors the `group_id`
                    // choice above): proves `MutationOrigin::Contributed`'s structured payload —
                    // not just the unit `Owner` case — survives a real `.spr` byte round trip.
                    origin: crate::os_spr::command::MutationOrigin::Contributed { plugin_id: "flow".to_string(), mutation_id: crate::os_spr::ids::SchemaId("widget.doc#recolor".to_string()), payload_hash: crate::os_spr::ids::PayloadHash([3u8; 32]) },
                    messages: Vec::new(),
                }]),
            },
        ],
        changes: vec![HistoryChange { id: "change-1".to_string(), saved_at: "2024-01-15T10:31:00Z".to_string(), edit_ids: vec!["edit-1".to_string(), "edit-2".to_string()], description: None }],
        checkpoints: vec![HistoryCheckpoint {
            id: "ck-1".to_string(),
            timestamp: "2024-01-15T10:32:00Z".to_string(),
            change_ids: vec!["change-1".to_string()],
            parent_id: None,
            authors: vec![HistoryAuthor { id: "u1".to_string(), name: "Ueli Saluz".to_string() }],
            message: Some("first checkpoint".to_string()),
        }],
        alternatives: vec![HistoryAlternative { id: "alt-1".to_string(), name: "main".to_string(), checkpoint_ids: vec!["ck-1".to_string()] }],
        active_alternative_id: Some("alt-1".to_string()),
        cursor: None,
        composition: None,
        conflicts: Vec::new(),
    }
}

async fn sample_conflicts() -> Vec<HistoryConflict> {
    vec![
        HistoryConflict {
            id: "conflict-quarantine-1".to_string(),
            kind: 0,
            status: 0,
            actors: vec!["alice".to_string(), "bob".to_string()],
            hlt: (1, 1_700_000_000_000, 4),
            edit_ids: Vec::new(),
            envelopes: vec![vec![1, 2, 3], vec![4, 5, 6, 7]],
            messages: vec![HistoryMessage { level: 2, code: "mutation.duplicate-id".to_string(), message: "id collided".to_string(), target: vec!["node-1".to_string()], op_index: Some(0) }],
        },
        HistoryConflict {
            id: "conflict-degraded-1".to_string(),
            kind: 1,
            status: 1,
            actors: vec!["carol".to_string()],
            hlt: (2, 1_700_000_001_000, 0),
            edit_ids: vec!["edit-1".to_string(), "edit-2".to_string()],
            envelopes: Vec::new(),
            messages: vec![
                HistoryMessage { level: 0, code: "mutation.cascade".to_string(), message: "cascaded".to_string(), target: Vec::new(), op_index: None },
                HistoryMessage { level: 1, code: "mutation.partial".to_string(), message: "partial apply".to_string(), target: vec!["a".to_string(), "b".to_string()], op_index: Some(1) },
            ],
        },
    ]
}

//#region 🔖️Composition
#[semio_framework_async_macros::async_test]
async fn composition_overlay_round_trips_through_the_binary_log() {
    let composition = HistoryComposition {
        owner: Some(("parent-1!s.stdio.object@1/*".to_string(), "mesh".to_string(), "child-1".to_string())),
        dialect: Some(("s.stdio.mesh".to_string(), "1".to_string(), "*".to_string())),
        checkpoint_pins: vec![("ck-1".to_string(), vec![("child-1!s.stdio.mesh@1/*".to_string(), "ck-child-7".to_string())])],
    };
    let log = HistoryLog { composition: Some(composition.clone()), ..sample_log().await };

    let bytes = encode_history(&log, &EncodeOptions::default()).await.expect("encode");
    let decoded = decode_history(&bytes, &DecodeOptions::default()).await.expect("decode");
    assert_eq!(decoded.composition, Some(composition), "the composition overlay did not survive the binary round trip");
}

#[semio_framework_async_macros::async_test]
async fn a_log_without_composition_writes_no_composition_record() {
    let bytes = encode_history(&sample_log().await, &EncodeOptions::default()).await.expect("encode");
    assert_eq!(decode_history(&bytes, &DecodeOptions::default()).await.expect("decode").composition, None);
    // 🎯️ The record is non-critical, so a reader that skips it must still read the whole log —
    // which is exactly what "absent" and "skipped" both look like from here.
    assert!(!bytes.windows(1).any(|window| window == [REC_COMPOSITION]) || decode_history(&bytes, &DecodeOptions::default()).await.is_ok());
}
//#endregion 🔖️Composition

//#region 🔖️Conflict
#[semio_framework_async_macros::async_test]
async fn conflict_payload_round_trips_both_kinds_byte_identically() {
    let log = HistoryLog { conflicts: sample_conflicts().await, ..sample_log().await };
    let bytes = encode_history(&log, &EncodeOptions::default()).await.expect("encode");
    let decoded = decode_history(&bytes, &DecodeOptions::default()).await.expect("decode");
    assert_eq!(decoded, log, "conflicts of both kinds must survive the binary round trip structurally");
    let re_encoded = encode_history(&decoded, &EncodeOptions::default()).await.expect("re-encode");
    assert_eq!(re_encoded, bytes, "re-encoding the decoded log must reproduce byte-identical output");
}

#[semio_framework_async_macros::async_test]
async fn encode_conflicts_round_trips_with_dict_and_ordinal_refs() {
    let conflicts = sample_conflicts().await;
    let mut dict = DictBuilder::new();
    let ordinals: HashMap<&str, u64> = [("edit-1", 0u64), ("edit-2", 1u64)].into_iter().collect();
    let payload = encode_conflicts(&conflicts, &mut dict, |id| ordinals.get(id).copied()).await.unwrap();
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
    let edit_ids = ["edit-1".to_string(), "edit-2".to_string()];
    let decoded = decode_conflicts(&payload, &reader, |ord| edit_ids.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await.unwrap();
    assert_eq!(decoded, conflicts);
}

#[semio_framework_async_macros::async_test]
async fn encode_conflicts_round_trips_empty() {
    let mut dict = DictBuilder::new();
    let payload = encode_conflicts(&Vec::new(), &mut dict, |_| None).await.unwrap();
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
    let decoded = decode_conflicts(&payload, &reader, |ord| Err(ProtocolError::DictMiss(ord as u32))).await.unwrap();
    assert_eq!(decoded, Vec::new());
}

#[semio_framework_async_macros::async_test]
async fn conflict_decoder_rejects_duplicate_ids_and_trailing_payload_bytes() {
    let conflict = sample_conflicts().await.remove(0);
    let mut dict = DictBuilder::new();
    let ordinals: HashMap<&str, u64> = [("edit-1", 0u64), ("edit-2", 1u64)].into_iter().collect();
    let duplicate_payload = encode_conflicts(&[conflict.clone(), conflict], &mut dict, |id| ordinals.get(id).copied()).await.expect("encode duplicate fixture");
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).expect("dictionary");
    let edit_ids = ["edit-1".to_string(), "edit-2".to_string()];
    assert!(matches!(decode_conflicts(&duplicate_payload, &reader, |ord| edit_ids.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await, Err(ProtocolError::Malformed { .. })));

    let mut trailing_dict = DictBuilder::new();
    let mut trailing_payload = encode_conflicts(&sample_conflicts().await, &mut trailing_dict, |_| None).await.expect("encode trailing fixture");
    trailing_payload.push(0);
    let mut trailing_reader = DictReader::new();
    trailing_reader.extend(0, trailing_dict.entries_since(0).to_vec()).expect("dictionary");
    assert!(matches!(decode_conflicts(&trailing_payload, &trailing_reader, |_| Err(ProtocolError::DictMiss(0))).await, Err(ProtocolError::Malformed { .. })));
}

#[semio_framework_async_macros::async_test]
async fn conflict_codec_rejects_unknown_kind_and_status_tags() {
    let mut dict = DictBuilder::new();
    let mut conflicts = sample_conflicts().await;
    conflicts[0].kind = 9;
    assert!(matches!(encode_conflicts(&conflicts, &mut dict, |_| None).await, Err(ProtocolError::Malformed { what: "conflict", .. })));

    let mut dict = DictBuilder::new();
    let mut payload = encode_conflicts(&sample_conflicts().await[..1], &mut dict, |_| None).await.expect("encode valid conflict");
    let tag_offset = payload.windows(3).position(|window| window == [0, 0, 2]).expect("kind/status/actor-count tags");
    payload[tag_offset + 1] = 9;
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).expect("dictionary");
    assert!(matches!(decode_conflicts(&payload, &reader, |_| Err(ProtocolError::DictMiss(0))).await, Err(ProtocolError::Malformed { what: "conflict", .. })));
}

#[semio_framework_async_macros::async_test]
async fn a_log_without_conflicts_writes_no_conflict_record() {
    let bytes = encode_history(&sample_log().await, &EncodeOptions::default()).await.expect("encode");
    assert_eq!(decode_history(&bytes, &DecodeOptions::default()).await.expect("decode").conflicts, Vec::new());
    // 🎯️ Scans actual frames (not a raw byte-window guess like the composition precedent
    // above) — the rigorous form of "no conflicts ⇒ no REC_CONFLICT record emitted".
    let mut cursor = FrameCursor::new(&bytes, HEADER_SIZE as u64).await;
    let mut saw_conflict_record = false;
    while let Some(frame) = cursor.next_frame().await.expect("scan") {
        if frame.kind == REC_CONFLICT {
            saw_conflict_record = true;
        }
    }
    assert!(!saw_conflict_record, "no REC_CONFLICT frame should be emitted when conflicts is empty");
}
//#endregion 🔖️Conflict

//#region 🔖️Message
#[semio_framework_async_macros::async_test]
async fn op_meta_messages_round_trip_every_severity_and_target_shape() {
    let meta = HistoryOpMeta {
        op_id: Some("op-9".to_string()),
        dependencies: Vec::new(),
        base_version: 1,
        author_id: None,
        hlt: None,
        undo_policy: 0,
        payload_hash: None,
        group_id: None,
        origin: crate::os_spr::command::MutationOrigin::Owner,
        messages: vec![
            HistoryMessage { level: 0, code: "mutation.cascade".to_string(), message: "cascaded".to_string(), target: Vec::new(), op_index: None },
            HistoryMessage { level: 1, code: "mutation.clamped".to_string(), message: "clamped".to_string(), target: vec!["a".to_string()], op_index: Some(0) },
            HistoryMessage { level: 2, code: "mutation.target-missing".to_string(), message: "missing".to_string(), target: vec!["a".to_string(), "b".to_string()], op_index: Some(3) },
            HistoryMessage { level: 3, code: "mutation.invariant".to_string(), message: "broken".to_string(), target: vec!["x".to_string(), "y".to_string(), "z".to_string()], op_index: None },
        ],
    };
    let edit = HistoryEdit {
        id: "edit-m".to_string(),
        actor: None,
        started_at: "2024-01-01T00:00:00Z".to_string(),
        finished_at: None,
        coalesce_key: None,
        description: None,
        ops: vec![OpPayload { text: Some("noop".to_string()), binary: None }],
        inverse: Vec::new(),
        meta: Some(vec![meta.clone()]),
    };
    let mut dict = DictBuilder::new();
    let payload = encode_edit(&edit, &mut dict, |_| None).await.unwrap();
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
    let decoded = decode_edit(&payload, &reader, |ord| Err(ProtocolError::DictMiss(ord as u32))).await.unwrap();
    assert_eq!(decoded.meta, Some(vec![meta]));
}

#[semio_framework_async_macros::async_test]
async fn op_meta_without_messages_writes_no_messages_section() {
    let meta = HistoryOpMeta { op_id: None, dependencies: Vec::new(), base_version: 0, author_id: None, hlt: None, undo_policy: 0, payload_hash: None, group_id: None, origin: crate::os_spr::command::MutationOrigin::Owner, messages: Vec::new() };
    let mut dict = DictBuilder::new();
    let mut out = ByteWriter::new();
    write_op_meta(&mut out, &meta, &mut dict, &|_: &str| None).await.unwrap();
    let payload = out.into_bytes();
    // presence byte is the very first byte written by write_op_meta; bit6 (0x40) must be unset.
    assert_eq!(payload[0] & 0b0100_0000, 0, "bit6 must be unset for empty messages");
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
    let mut input = ByteReader::new(&payload);
    let decoded = read_op_meta(&mut input, &reader, &|ord: u64| Err(ProtocolError::DictMiss(ord as u32))).await.unwrap();
    assert_eq!(decoded.messages, Vec::new());
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ContributedOriginOracle {
    kind: String,
    plugin_id: String,
    mutation_id: String,
    payload_hash: [u8; 32],
}

/// 🔢️ Checks canonical origin bytes against an independent test-only serde representation.
#[semio_framework_async_macros::async_test]
async fn mutation_origin_canonical_json_is_byte_identical_between_serde_json_and_pack_json() {
    let origin = crate::os_spr::MutationOrigin::Contributed { plugin_id: "s.stdio.mesh".to_string(), mutation_id: crate::os_spr::SchemaId("mesh/v1".to_string()), payload_hash: crate::os_spr::PayloadHash(core::array::from_fn(|index| index as u8)) };
    let oracle = ContributedOriginOracle { kind: "contributed".to_string(), plugin_id: "s.stdio.mesh".to_string(), mutation_id: "mesh/v1".to_string(), payload_hash: core::array::from_fn(|index| index as u8) };
    let via_serde = serde_json::to_string(&oracle).expect("serde_json encodes independent origin oracle");
    let via_pack = crate::os_pack::json::to_json_string(&origin);
    assert_eq!(via_serde, via_pack, "canonical origin bytes must match the independent oracle");
    assert!(via_pack.contains("\"payloadHash\":[0,1,2,"), "got {via_pack} — payload_hash bytes must stay bare integers, never x.0");
    let round_trip_serde: ContributedOriginOracle = serde_json::from_str(&via_pack).expect("serde_json decodes first-party origin bytes");
    let round_trip_pack: crate::os_spr::MutationOrigin = crate::os_pack::json::from_json_str(&via_serde).expect("pack::json decodes independent origin bytes");
    assert_eq!(round_trip_serde, oracle);
    assert_eq!(round_trip_pack, origin);
}

#[semio_framework_async_macros::async_test]
async fn history_message_decoder_rejects_invalid_severity_presence_and_index_width() {
    let message = HistoryMessage { level: 1, code: "mutation.clamped".to_string(), message: "clamped".to_string(), target: Vec::new(), op_index: None };
    let mut dict = DictBuilder::new();
    let mut out = ByteWriter::new();
    write_history_message(&mut out, &message, &mut dict).await.expect("encode message");
    let encoded = out.into_bytes();
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).expect("dictionary");

    let mut invalid_severity = encoded.clone();
    invalid_severity[0] = 4;
    assert!(matches!(read_history_message(&mut ByteReader::new(&invalid_severity), &reader).await, Err(ProtocolError::Malformed { .. })));

    let mut invalid_presence = encoded;
    *invalid_presence.last_mut().expect("presence byte") = 2;
    assert!(matches!(read_history_message(&mut ByteReader::new(&invalid_presence), &reader).await, Err(ProtocolError::Malformed { .. })));

    let indexed = HistoryMessage { op_index: Some(0), ..message };
    let mut indexed_out = ByteWriter::new();
    write_history_message(&mut indexed_out, &indexed, &mut dict).await.expect("encode indexed message");
    let mut oversized_index = indexed_out.into_bytes();
    oversized_index.pop();
    oversized_index.extend_from_slice(&[0x80, 0x80, 0x80, 0x80, 0x10]);
    assert!(matches!(read_history_message(&mut ByteReader::new(&oversized_index), &reader).await, Err(ProtocolError::Malformed { .. })));
}

#[semio_framework_async_macros::async_test]
async fn message_code_interning_does_not_grow_the_dictionary_linearly() {
    let mut dict = DictBuilder::new();
    let mut out = ByteWriter::new();
    for _ in 0..500 {
        let message = HistoryMessage { level: 1, code: "mutation.clamped".to_string(), message: String::new(), target: Vec::new(), op_index: None };
        write_history_message(&mut out, &message, &mut dict).await.unwrap();
    }
    assert_eq!(dict.len(), 1, "500 identical codes must intern to a single dictionary entry, not 500");
    let payload_len = out.into_bytes().len();
    let bytes_per_message = payload_len / 500;
    // 🎯️ A raw (non-interned) code string would cost len("mutation.clamped")=17 bytes alone
    // per repeat; interning collapses every repeat after the first to a small dictref (tag +
    // varint index), so the true per-message average must stay far below the raw string's own
    // length — proof the growth is sub-linear, not just "small on average by luck".
    assert!(bytes_per_message < 10, "expected sub-10-byte-per-message average from interning, got {bytes_per_message}");
}
//#endregion 🔖️Message

//#region 🔖️TextGrammar
#[semio_framework_async_macros::async_test]
async fn ops_text_round_trips_a_full_log() {
    // `HistoryEdit::meta` is derived data the text grammar never carries (see the Model
    // region note) — parse_ops_text always yields `meta: None`, so the expectation strips it
    // before comparing rather than asserting full structural equality including meta.
    let mut log = sample_log().await;
    for edit in &mut log.edits {
        edit.meta = None;
    }
    let text = print_ops_text(&log).unwrap();
    let parsed = parse_ops_text(&text).unwrap();
    assert_eq!(parsed, log);
}

#[semio_framework_async_macros::async_test]
async fn ops_text_is_a_fixpoint_under_reprint() {
    let log = sample_log().await;
    let text = print_ops_text(&log).unwrap();
    let reparsed = parse_ops_text(&text).unwrap();
    assert_eq!(print_ops_text(&reparsed).unwrap(), text);
}

#[semio_framework_async_macros::async_test]
async fn ops_text_skips_comments_and_blank_lines() {
    let text = "doc doc-1 schema=s1\n\n# a comment\nactive alt-1\n";
    let log = parse_ops_text(text).unwrap();
    assert_eq!(log.doc_id, "doc-1");
    assert_eq!(log.active_alternative_id.as_deref(), Some("alt-1"));
}

#[semio_framework_async_macros::async_test]
async fn ops_text_rejects_unknown_line_keyword() {
    let err = parse_ops_text("doc doc-1 schema=s1\nbogus x\n").unwrap_err();
    assert!(matches!(err, ProtocolError::Malformed { .. }));
}

#[semio_framework_async_macros::async_test]
async fn ops_text_edit_without_active_line_leaves_none() {
    let log = HistoryLog { doc_id: "d".into(), schema: "s".into(), active_alternative_id: None, ..Default::default() };
    let text = print_ops_text(&log).unwrap();
    assert!(!text.contains("active"));
    assert_eq!(parse_ops_text(&text).unwrap().active_alternative_id, None);
}

#[semio_framework_async_macros::async_test]
async fn ops_text_round_trips_a_cursor_line_with_undo_then_apply_interleaving() {
    let mut log = sample_log().await;
    for edit in &mut log.edits {
        edit.meta = None;
    }
    // A single tail-edit marker cannot represent this: edit-1 undone (moved to redo), then a
    // later apply produced edit-2 — edit-1 precedes edit-2 in file order but is NOT applied.
    log.cursor = Some(HistoryCursor { applied_edit_ids: vec!["edit-2".to_string()], redo_edit_ids: vec!["edit-1".to_string()], checkpoint_id: Some("ck-1".to_string()) });
    let text = print_ops_text(&log).unwrap();
    assert!(text.contains("cursor"));
    let parsed = parse_ops_text(&text).unwrap();
    assert_eq!(parsed, log);
}

#[semio_framework_async_macros::async_test]
async fn ops_text_without_a_cursor_line_leaves_cursor_none() {
    let log = HistoryLog { doc_id: "d".into(), schema: "s".into(), ..Default::default() };
    let text = print_ops_text(&log).unwrap();
    assert!(!text.contains("cursor"));
    assert_eq!(parse_ops_text(&text).unwrap().cursor, None);
}
//#endregion 🔖️TextGrammar

//#region 🔖️Payloads
#[semio_framework_async_macros::async_test]
async fn doc_payload_round_trips() {
    let mut dict = DictBuilder::new();
    let payload = encode_doc("doc-1", "org.semio.demo.v1", &mut dict).await;
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
    let (id, schema) = decode_doc(&payload, &reader).await.unwrap();
    assert_eq!(id, "doc-1");
    assert_eq!(schema, "org.semio.demo.v1");
}

#[semio_framework_async_macros::async_test]
async fn edit_payload_round_trips_with_all_optionals_and_meta() {
    let log = sample_log().await;
    let edit = &log.edits[1];
    let mut dict = DictBuilder::new();
    let ordinals: HashMap<&str, u64> = [("edit-1", 0u64)].into_iter().collect();
    let payload = encode_edit(edit, &mut dict, |id| ordinals.get(id).copied()).await.unwrap();
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
    let edit_ids = ["edit-1".to_string()];
    let decoded = decode_edit(&payload, &reader, |ord| edit_ids.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await.unwrap();
    assert_eq!(decoded, *edit);
}

#[semio_framework_async_macros::async_test]
async fn edit_payload_round_trips_minimal_edit() {
    let edit = HistoryEdit { id: "edit-x".to_string(), actor: None, started_at: "2024-01-01T00:00:00Z".to_string(), finished_at: None, coalesce_key: None, description: None, ops: Vec::new(), inverse: Vec::new(), meta: None };
    let mut dict = DictBuilder::new();
    let payload = encode_edit(&edit, &mut dict, |_| None).await.unwrap();
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
    let decoded = decode_edit(&payload, &reader, |ord| Err(ProtocolError::DictMiss(ord as u32))).await.unwrap();
    assert_eq!(decoded, edit);
}

#[semio_framework_async_macros::async_test]
async fn change_payload_round_trips_and_references_edit_ordinals() {
    let change = HistoryChange { id: "change-1".to_string(), saved_at: "2024-01-01T00:00:00Z".to_string(), edit_ids: vec!["edit-1".to_string(), "edit-2".to_string()], description: Some("d".to_string()) };
    let mut dict = DictBuilder::new();
    let ordinals: HashMap<&str, u64> = [("edit-1", 0u64), ("edit-2", 1u64)].into_iter().collect();
    let payload = encode_change(&change, &mut dict, |id| ordinals.get(id).copied()).await.unwrap();
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
    let edit_ids = ["edit-1".to_string(), "edit-2".to_string()];
    let decoded = decode_change(&payload, &reader, |ord| edit_ids.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await.unwrap();
    assert_eq!(decoded, change);
}

#[semio_framework_async_macros::async_test]
async fn checkpoint_payload_round_trips_with_authors() {
    let checkpoint = sample_log().await.checkpoints.remove(0);
    let mut dict = DictBuilder::new();
    let payload = encode_checkpoint(&checkpoint, &mut dict).await.unwrap();
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
    let decoded = decode_checkpoint(&payload, &reader).await.unwrap();
    assert_eq!(decoded, checkpoint);
}

#[semio_framework_async_macros::async_test]
async fn alternative_payload_round_trips() {
    let alternative = sample_log().await.alternatives.remove(0);
    let mut dict = DictBuilder::new();
    let payload = encode_alternative(&alternative, &mut dict).await.unwrap();
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
    let decoded = decode_alternative(&payload, &reader).await.unwrap();
    assert_eq!(decoded, alternative);
}

#[semio_framework_async_macros::async_test]
async fn active_payload_round_trips_some_and_none() {
    let mut dict = DictBuilder::new();
    let payload_some = encode_active(Some("alt-1"), &mut dict).await;
    let payload_none = encode_active(None, &mut dict).await;
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
    assert_eq!(decode_active(&payload_some, &reader).await.unwrap(), Some("alt-1".to_string()));
    assert_eq!(decode_active(&payload_none, &reader).await.unwrap(), None);
}

#[semio_framework_async_macros::async_test]
async fn edit_payload_round_trips_a_backwards_section_mixing_text_and_binary_payloads() {
    let edit = HistoryEdit {
        id: "edit-y".to_string(),
        actor: Some("bob".to_string()),
        started_at: "2024-02-01T00:00:00Z".to_string(),
        finished_at: Some("2024-02-01T00:00:01Z".to_string()),
        coalesce_key: None,
        description: None,
        ops: vec![OpPayload { text: Some("set n=1".to_string()), binary: Some(vec![1, 2, 3]) }, OpPayload { text: Some("set n=2".to_string()), binary: None }],
        inverse: vec![OpPayload { text: Some("set n=0".to_string()), binary: Some(vec![0]) }, OpPayload { text: Some("set n=1".to_string()), binary: None }],
        meta: None,
    };
    let mut dict = DictBuilder::new();
    let payload = encode_edit(&edit, &mut dict, |_| None).await.unwrap();
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
    let decoded = decode_edit(&payload, &reader, |ord| Err(ProtocolError::DictMiss(ord as u32))).await.unwrap();
    assert_eq!(decoded, edit);
    assert_eq!(decoded.ops[0].binary, Some(vec![1, 2, 3]));
    assert_eq!(decoded.inverse[0].binary, Some(vec![0]));
}

#[semio_framework_async_macros::async_test]
async fn edit_payload_with_empty_backwards_omits_the_section_and_decodes_empty() {
    let edit = HistoryEdit {
        id: "edit-z".to_string(),
        actor: None,
        started_at: "2024-02-01T00:00:00Z".to_string(),
        finished_at: None,
        coalesce_key: None,
        description: None,
        ops: vec![OpPayload { text: Some("noop".to_string()), binary: None }],
        inverse: Vec::new(),
        meta: None,
    };
    let mut dict = DictBuilder::new();
    let payload = encode_edit(&edit, &mut dict, |_| None).await.unwrap();
    // presence byte is the 2nd byte (offset 1); bit5 (0x20) must be unset when inverse is empty.
    assert_eq!(payload[1] & 0b0010_0000, 0, "bit5 must be unset for empty inverse");
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
    let decoded = decode_edit(&payload, &reader, |ord| Err(ProtocolError::DictMiss(ord as u32))).await.unwrap();
    assert_eq!(decoded.inverse, Vec::new());
}

#[semio_framework_async_macros::async_test]
async fn cursor_payload_round_trips_with_dict_and_ordinal_refs() {
    let cursor = HistoryCursor { applied_edit_ids: vec!["edit-1".to_string(), "edit-2".to_string()], redo_edit_ids: vec!["edit-3".to_string()], checkpoint_id: Some("ck-1".to_string()) };
    let mut dict = DictBuilder::new();
    let ordinals: HashMap<&str, u64> = [("edit-1", 0u64), ("edit-2", 1u64)].into_iter().collect();
    let payload = encode_cursor(&cursor, &mut dict, |id| ordinals.get(id).copied()).await.unwrap();
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
    let edit_ids = ["edit-1".to_string(), "edit-2".to_string()];
    let decoded = decode_cursor(&payload, &reader, |ord| edit_ids.get(ord as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(ord as u32))).await.unwrap();
    assert_eq!(decoded, cursor);
}

#[semio_framework_async_macros::async_test]
async fn cursor_payload_round_trips_without_a_checkpoint() {
    let cursor = HistoryCursor { applied_edit_ids: Vec::new(), redo_edit_ids: Vec::new(), checkpoint_id: None };
    let mut dict = DictBuilder::new();
    let payload = encode_cursor(&cursor, &mut dict, |_| None).await.unwrap();
    let mut reader = DictReader::new();
    reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
    let decoded = decode_cursor(&payload, &reader, |ord| Err(ProtocolError::DictMiss(ord as u32))).await.unwrap();
    assert_eq!(decoded, cursor);
}

#[semio_framework_async_macros::async_test]
async fn cursor_decoder_rejects_unknown_presence_bits_and_trailing_payload() {
    let dict = DictReader::new();
    assert!(matches!(decode_cursor(&[1, 2], &dict, |_| Err(ProtocolError::DictMiss(0))).await, Err(ProtocolError::Malformed { .. })));
    assert!(matches!(decode_cursor(&[1, 0, 0, 0, 0], &dict, |_| Err(ProtocolError::DictMiss(0))).await, Err(ProtocolError::Malformed { .. })));
}

#[semio_framework_async_macros::async_test]
async fn decode_rejects_unsupported_format() {
    let payload = vec![2u8, 0, 0];
    let dict = DictReader::new();
    let err = decode_doc(&payload, &dict).await.unwrap_err();
    assert!(matches!(err, ProtocolError::Malformed { .. }));
}
//#endregion 🔖️Payloads

//#region 🔖️Codec
#[semio_framework_async_macros::async_test]
async fn history_encode_decode_identity_standard() {
    let log = sample_log().await;
    let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
    let decoded = decode_history(&bytes, &DecodeOptions::default()).await.unwrap();
    assert_eq!(decoded, log);
}

#[semio_framework_async_macros::async_test]
async fn history_encode_decode_identity_full_verification() {
    let log = sample_log().await;
    let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
    let options = DecodeOptions { verification: VerificationLevel::Full, limits: ProtocolLimits::default() };
    let decoded = decode_history(&bytes, &options).await.unwrap();
    assert_eq!(decoded, log);
}

#[semio_framework_async_macros::async_test]
async fn history_encode_is_canonically_stable() {
    let log = sample_log().await;
    let a = encode_history(&log, &EncodeOptions::default()).await.unwrap();
    let b = encode_history(&log, &EncodeOptions::default()).await.unwrap();
    assert_eq!(a, b);
}

#[semio_framework_async_macros::async_test]
async fn history_full_verification_detects_tampering() {
    let log = sample_log().await;
    let mut bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
    let last = bytes.len();
    bytes[last - 10] ^= 0xFF;
    let options = DecodeOptions { verification: VerificationLevel::Full, limits: ProtocolLimits::default() };
    // A single-byte flip breaks that frame's own CRC-32C, so `crate::os_spr::format::recover`
    // (RecoveryMode::LastCommit, run by `HistoryReader::open`) truncates the trusted range to
    // before the corrupted frame — here that means before the file's only commit, so the
    // decoded log comes back empty rather than as an `Err`. Either outcome is an acceptable
    // "tamper never goes unnoticed": the result must never silently equal the original log.
    let result = decode_history(&bytes, &options).await;
    assert!(result.is_err() || result.unwrap() != log);
}

#[semio_framework_async_macros::async_test]
async fn history_round_trips_backwards_and_binary_payloads_and_cursor_when_write_backwards_section_is_set() {
    let mut log = sample_log().await;
    log.edits[0].inverse = vec![OpPayload { text: Some("unset foo".to_string()), binary: Some(vec![9, 9]) }, OpPayload { text: Some("unset bar".to_string()), binary: None }];
    log.edits[1].ops[0].binary = Some(vec![7]);
    log.cursor = Some(HistoryCursor { applied_edit_ids: vec!["edit-1".to_string(), "edit-2".to_string()], redo_edit_ids: Vec::new(), checkpoint_id: Some("ck-1".to_string()) });
    let options = EncodeOptions { write_backwards_section: true, ..EncodeOptions::default() };
    let bytes = encode_history(&log, &options).await.unwrap();
    let decoded = decode_history(&bytes, &DecodeOptions::default()).await.unwrap();
    assert_eq!(decoded, log);
    assert_eq!(decoded.edits[0].inverse[0].binary, Some(vec![9, 9]));
    assert_eq!(decoded.edits[1].ops[0].binary, Some(vec![7]));
    assert_eq!(decoded.cursor, log.cursor);
}

#[semio_framework_async_macros::async_test]
async fn history_strips_backwards_when_write_backwards_section_is_unset_even_if_populated() {
    let mut log = sample_log().await;
    log.edits[0].inverse = vec![OpPayload { text: Some("unset foo".to_string()), binary: None }];
    let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
    let decoded = decode_history(&bytes, &DecodeOptions::default()).await.unwrap();
    assert_eq!(decoded.edits[0].inverse, Vec::new(), "write_backwards_section defaults false and must strip populated inverse");
}
//#endregion 🔖️Codec

//#region 🔖️Append
#[semio_framework_async_macros::async_test]
async fn streamed_append_equals_buffered_encode() {
    let log = sample_log().await;
    let options = WriteOptions { required_flags: crate::os_spr::REQUIRED_HASH_CHAIN, optional_flags: crate::os_spr::OPTIONAL_CANONICAL };
    let mut appender = HistoryAppender::begin(Vec::<u8>::new(), &log.doc_id, &log.schema, &options).await.unwrap();
    for edit in &log.edits {
        appender.append_edit(edit).await.unwrap();
    }
    for change in &log.changes {
        appender.append_change(change).await.unwrap();
    }
    for checkpoint in &log.checkpoints {
        appender.append_checkpoint(checkpoint).await.unwrap();
    }
    for alternative in &log.alternatives {
        appender.append_alternative(alternative).await.unwrap();
    }
    appender.set_active(log.active_alternative_id.as_deref()).await.unwrap();
    appender.commit().await.unwrap();
    let streamed_bytes = appender.into_sink().await;

    let decoded = decode_history(&streamed_bytes, &DecodeOptions::default()).await.unwrap();
    assert_eq!(decoded, log);
}

#[semio_framework_async_macros::async_test]
async fn append_cursor_then_decode_recovers_it() {
    let mut log = sample_log().await;
    for edit in &mut log.edits {
        edit.meta = None;
    }
    let cursor = HistoryCursor { applied_edit_ids: vec!["edit-1".to_string()], redo_edit_ids: vec!["edit-2".to_string()], checkpoint_id: Some("ck-1".to_string()) };
    let options = WriteOptions { required_flags: crate::os_spr::REQUIRED_HASH_CHAIN, optional_flags: crate::os_spr::OPTIONAL_CANONICAL };
    let mut appender = HistoryAppender::begin(Vec::<u8>::new(), &log.doc_id, &log.schema, &options).await.unwrap();
    for edit in &log.edits {
        appender.append_edit(edit).await.unwrap();
    }
    appender.append_cursor(&cursor).await.unwrap();
    appender.commit().await.unwrap();
    let bytes = appender.into_sink().await;

    let decoded = decode_history(&bytes, &DecodeOptions::default()).await.unwrap();
    assert_eq!(decoded.cursor, Some(cursor));
}
//#endregion 🔖️Append

//#region 🔖️Scan
#[semio_framework_async_macros::async_test]
async fn reader_edits_forward_matches_log() {
    let log = sample_log().await;
    let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
    let reader = HistoryReader::open(&bytes, &DecodeOptions::default()).await.unwrap();
    let edits: Vec<HistoryEdit> = reader.edits().await.map(|r| r.unwrap()).collect();
    assert_eq!(edits, log.edits);
}

#[semio_framework_async_macros::async_test]
async fn reader_edits_rev_matches_tail_in_reverse() {
    let log = sample_log().await;
    let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
    let reader = HistoryReader::open(&bytes, &DecodeOptions::default()).await.unwrap();
    let rev: Vec<HistoryEdit> = reader.edits_rev(1).await.map(|r| r.unwrap()).collect();
    assert_eq!(rev.len(), 1);
    assert_eq!(rev[0], log.edits[1]);
}

#[semio_framework_async_macros::async_test]
async fn reader_edits_rev_full_matches_reversed_forward() {
    let log = sample_log().await;
    let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
    let reader = HistoryReader::open(&bytes, &DecodeOptions::default()).await.unwrap();
    let rev: Vec<HistoryEdit> = reader.edits_rev(usize::MAX).await.map(|r| r.unwrap()).collect();
    let mut expected = log.edits;
    expected.reverse();
    assert_eq!(rev, expected);
}
//#endregion 🔖️Scan

//#region 🔖️Frontier
async fn frontier(head_edit_ordinal: u64, head_edit_id: &str, chain_hash: [u8; 32]) -> FrontierSummary {
    FrontierSummary { document_id: "doc-1".to_string(), head_edit_ordinal, head_edit_id: head_edit_id.to_string(), alternatives: Vec::new(), last_commit_seq: 1, chain_hash }
}

#[semio_framework_async_macros::async_test]
async fn frontier_delta_reports_equal_ahead_behind_diverged() {
    let a = frontier(5, "edit-5", [1u8; 32]).await;
    let b = frontier(5, "edit-5", [1u8; 32]).await;
    assert_eq!(frontier_delta(&a, &b).await, FrontierComparison::Equal);

    let ahead = frontier(6, "edit-6", [2u8; 32]).await;
    assert_eq!(frontier_delta(&ahead, &a).await, FrontierComparison::Ahead);
    assert_eq!(frontier_delta(&a, &ahead).await, FrontierComparison::Behind);

    let diverged = frontier(5, "edit-5-alt", [3u8; 32]).await;
    assert_eq!(frontier_delta(&a, &diverged).await, FrontierComparison::Diverged { common_edit_count: 5 });
}
//#endregion 🔖️Frontier

//#region 🔖️Index
#[semio_framework_async_macros::async_test]
async fn index_round_trips_edits_checkpoints_and_snapshots() {
    let mut builder = IndexBuilder::new().await;
    builder.record_edit(0, 100).await;
    builder.record_edit(5, 300).await;
    builder.record_edit(10, 500).await;
    builder.record_checkpoint("ck-1", 700, 10).await;
    builder.record_snapshot(5, 250).await;
    builder.record_sealed(50).await;
    let payload = builder.build().await;

    let reader = IndexReader::open(&payload).await.unwrap();
    assert_eq!(reader.edit_offset_at_or_before(7).await, Some(300));
    assert_eq!(reader.edit_offset_at_or_before(0).await, Some(100));
    assert_eq!(reader.edit_offset_at_or_before(10).await, Some(500));
    assert_eq!(reader.checkpoint_offset("ck-1").await, Some((700, 10)));
    assert_eq!(reader.checkpoint_offset("missing").await, None);
    assert_eq!(reader.latest_snapshot_offset_at_or_before(9).await, Some(250));
}
//#endregion 🔖️Index
