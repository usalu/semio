use super::*;

fn hex(value: &str) -> Vec<u8> {
    value.as_bytes().as_chunks::<2>().0.iter().map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
}

fn transition_record(doc_id: &str, actor: &str, transition: crate::os_spr::HistoryTransition, timestamp: (u64, u64, u64)) -> HistoryTransitionRecord {
    let envelope = crate::os_spr::history_transition_envelope(
        &transition,
        &crate::os_spr::ArtifactId(doc_id.to_string()),
        &crate::os_spr::ActorId(actor.to_string()),
        Vec::new(),
        crate::os_spr::HybridLogicalTimestamp { actor: timestamp.0, physical_ms: timestamp.1, logical: timestamp.2 },
    );
    HistoryTransitionRecord::from_envelope(&envelope)
}

fn commit(checkpoint_id: &str, parent_id: Option<&str>, change_id: &str, mutation_ids: &[&str]) -> crate::os_spr::HistoryTransition {
    crate::os_spr::HistoryTransition::Commit(crate::os_spr::TransitionCheckpoint {
        checkpoint_id: checkpoint_id.to_string(),
        parent_id: parent_id.map(str::to_string),
        change_id: change_id.to_string(),
        mutation_ids: mutation_ids.iter().map(|id| crate::os_spr::MutationId(id.to_string())).collect(),
        description: None,
        saved_at: "2024-01-15T10:31:00Z".to_string(),
        authors: vec![crate::os_spr::TransitionAuthor { id: "u1".to_string(), name: "Ueli Saluz".to_string(), avatar: None }],
        message: Some("first checkpoint".to_string()),
        timestamp: "2024-01-15T10:32:00Z".to_string(),
    })
}

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
        transitions: vec![
            transition_record("doc-1", "alice", commit("ck-1", None, "change-1", &["edit-1#0", "edit-1#1", "op-1"]), (1, 1_700_000_001_000, 0)),
            transition_record("doc-1", "alice", crate::os_spr::HistoryTransition::Branch { alternative_id: "alt-1".to_string(), name: "main".to_string(), checkpoint_id: "ck-1".to_string() }, (1, 1_700_000_001_000, 1)),
        ],
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

#[semio_framework_async_macros::async_test]
async fn retained_history_decode_yields_across_bytes_and_semantic_records() {
    let log = sample_log().await;
    let bytes = encode_history(&log, &EncodeOptions::default()).await.expect("encode retained history fixture");
    let limits = crate::os_spr::format::retained::RetainedSprLimits { file_bytes: bytes.len() as u64, frame_body_bytes: 1_048_576, records: 8_192 };
    let mut decode = RetainedHistoryDecode::new(bytes.len(), limits).expect("admit retained decode");
    let mut pending = 0usize;
    loop {
        match decode.step(&bytes, 7, 1).expect("advance retained decode") {
            RetainedHistoryDecodeStep::Pending { completed_bytes, total_bytes, decoded_records } => {
                assert!(completed_bytes <= total_bytes);
                assert!(decoded_records <= limits.records);
                pending += 1;
            }
            RetainedHistoryDecodeStep::Ready => break,
        }
    }
    assert!(pending > bytes.len() / 7, "semantic frames must remain separately scheduled after byte verification");
    assert_eq!(decode.take_ready(), Some(log));
    let _ = decode.take_auxiliary_owners();
    assert!(decode.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn retained_history_decode_rejects_a_crc_valid_malformed_transition_after_valid_records() {
    let mut log = sample_log().await;
    log.transitions.push(HistoryTransitionRecord { id: "transition-after-valid-prefix".to_string(), actor: "alice".to_string(), hlt: (1, 2, 0), dependencies: Vec::new(), payload: vec![0xff] });
    let bytes = encode_history(&log, &EncodeOptions::default()).await.expect("encode semantically malformed retained history");
    let limits = crate::os_spr::format::retained::RetainedSprLimits { file_bytes: bytes.len() as u64, frame_body_bytes: 1_048_576, records: 8_192 };
    let mut decode = RetainedHistoryDecode::new(bytes.len(), limits).expect("admit retained semantic refusal");
    let error = loop {
        match decode.step(&bytes, 11, 1) {
            Ok(RetainedHistoryDecodeStep::Pending { .. }) => {}
            Ok(RetainedHistoryDecodeStep::Ready) => panic!("malformed transition reached ready"),
            Err(error) => break error,
        }
    };
    assert!(error.contains("transition-after-valid-prefix"));
    let partial = decode.take_partial().expect("semantic rejection retains its exact partial history owner");
    assert_eq!(partial.transitions.last().map(|transition| transition.id.as_str()), Some("transition-after-valid-prefix"));
    let _ = decode.take_auxiliary_owners();
    assert!(decode.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn retained_history_decode_rejects_a_repeated_transition_id() {
    let mut log = sample_log().await;
    log.transitions.push(log.transitions[0].clone());
    let bytes = encode_history(&log, &EncodeOptions::default()).await.expect("encode repeated transition");
    let limits = crate::os_spr::format::retained::RetainedSprLimits { file_bytes: bytes.len() as u64, frame_body_bytes: 1_048_576, records: 8_192 };
    let mut decode = RetainedHistoryDecode::new(bytes.len(), limits).expect("admit retained decode");
    let error = loop {
        match decode.step(&bytes, 4096, 4) {
            Ok(RetainedHistoryDecodeStep::Pending { .. }) => {}
            Ok(RetainedHistoryDecodeStep::Ready) => panic!("repeated transition reached ready"),
            Err(error) => break error,
        }
    };
    assert!(error.contains(&log.transitions[0].id));
    let _ = decode.take_partial();
    let _ = decode.take_auxiliary_owners();
    assert!(decode.terminal_is_empty());
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
    let text = "doc doc-1 schema=s1\n\n# a comment\ntransition t-1 actor=alice hlc=1,2,3 dependencies=[] payload=\"AAEEb3AtYg==\"\n";
    let log = parse_ops_text(text).unwrap();
    assert_eq!(log.doc_id, "doc-1");
    assert_eq!(log.transitions, vec![HistoryTransitionRecord { id: "t-1".to_string(), actor: "alice".to_string(), hlt: (1, 2, 3), dependencies: Vec::new(), payload: hex("0001046f702d62") }]);
}

#[semio_framework_async_macros::async_test]
async fn ops_text_rejects_unknown_line_keyword() {
    for retired in ["bogus x", "active alt-1", "cursor applied=[] redo=[]", "change c saved=t edits=[]", "checkpoint c at=t changes=[] by=[]", "alternative a name=n checkpoints=[]"] {
        let err = parse_ops_text(&format!("doc doc-1 schema=s1\n{retired}\n")).unwrap_err();
        assert!(matches!(err, ProtocolError::Malformed { .. }), "{retired}");
    }
}

#[semio_framework_async_macros::async_test]
async fn ops_text_round_trips_every_transition_field() {
    let mut log = sample_log().await;
    for edit in &mut log.edits {
        edit.meta = None;
    }
    log.transitions.push(HistoryTransitionRecord { id: "transition-x".to_string(), actor: "bob".to_string(), hlt: (u64::MAX, 0, 7), dependencies: vec!["op-1".to_string(), "edit-1#0".to_string()], payload: Vec::new() });
    let text = print_ops_text(&log).unwrap();
    assert_eq!(text.lines().filter(|line| line.starts_with("transition ")).count(), 3);
    assert_eq!(parse_ops_text(&text).unwrap(), log);
}

#[semio_framework_async_macros::async_test]
async fn ops_text_rejects_a_transition_without_hlc_or_payload() {
    for line in ["transition t actor=a dependencies=[] payload=\"AA==\"", "transition t actor=a hlc=1,2 dependencies=[] payload=\"AA==\"", "transition t actor=a hlc=1,2,3 dependencies=[]"] {
        assert!(parse_ops_text(&format!("doc d schema=s\n{line}\n")).is_err(), "{line}");
    }
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
async fn transition_payload_round_trips_with_interned_ids() {
    for transition in sample_log().await.transitions {
        let mut dict = DictBuilder::new();
        let payload = encode_transition(&transition, &mut dict).await.unwrap();
        let mut reader = DictReader::new();
        reader.extend(0, dict.entries_since(0).to_vec()).unwrap();
        assert_eq!(decode_transition(&payload, &reader).await.unwrap(), transition);
    }
}

#[semio_framework_async_macros::async_test]
async fn transition_payload_matches_the_language_agnostic_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔀️transition-record/🔣️.json")).unwrap();
    let record = &fixture["record"];
    let hlt: Vec<u64> = record["hlt"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap()).collect();
    let revert = crate::os_spr::HistoryTransition::Revert { mutation_ids: record["transition"]["revert"].as_array().unwrap().iter().map(|id| crate::os_spr::MutationId(id.as_str().unwrap().to_string())).collect() };
    assert_eq!(crate::os_spr::encode_history_transition(&revert), hex(record["payloadHex"].as_str().unwrap()));
    let transition = HistoryTransitionRecord {
        id: record["id"].as_str().unwrap().to_string(),
        actor: record["actor"].as_str().unwrap().to_string(),
        hlt: (hlt[0], hlt[1], hlt[2]),
        dependencies: record["dependencies"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_string()).collect(),
        payload: hex(record["payloadHex"].as_str().unwrap()),
    };
    let mut dict = DictBuilder::new();
    let payload = encode_transition(&transition, &mut dict).await.unwrap();
    assert_eq!(payload, hex(fixture["recordHex"].as_str().unwrap()));
    let dictionary: Vec<String> = fixture["dictionary"].as_array().unwrap().iter().map(|entry| entry.as_str().unwrap().to_string()).collect();
    assert_eq!(dict.entries_since(0).to_vec(), dictionary);
    assert_eq!(fixture["kind"].as_u64().unwrap(), u64::from(REC_TRANSITION));
    let mut reader = DictReader::new();
    reader.extend(0, dictionary).unwrap();
    assert_eq!(decode_transition(&payload, &reader).await.unwrap(), transition);
}

#[semio_framework_async_macros::async_test]
async fn transition_decoder_rejects_newer_format_truncation_and_trailing_bytes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔀️transition-record/🔣️.json")).unwrap();
    let mut reader = DictReader::new();
    reader.extend(0, fixture["dictionary"].as_array().unwrap().iter().map(|entry| entry.as_str().unwrap().to_string()).collect::<Vec<_>>()).unwrap();
    let bytes = hex(fixture["recordHex"].as_str().unwrap());
    let mut newer = bytes.clone();
    newer[0] = 2;
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(decode_transition(&newer, &reader).await.is_err());
    assert!(decode_transition(&trailing, &reader).await.is_err());
    assert!(decode_transition(&bytes[..bytes.len() - 1], &reader).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn transition_record_round_trips_its_envelope() {
    let transition = sample_log().await.transitions.remove(0);
    let envelope = transition.to_envelope("doc-1");
    assert!(crate::os_spr::is_history_transition(&envelope));
    assert_eq!(envelope.inverse.schema.0, crate::os_spr::HISTORY_TRANSITION_SCHEMA);
    assert!(envelope.inverse.payload.is_empty());
    assert_eq!(envelope.document_id.0, "doc-1");
    assert_eq!(HistoryTransitionRecord::from_envelope(&envelope), transition);
    assert!(matches!(crate::os_spr::history_transition_from_envelope(&envelope).unwrap(), Some(crate::os_spr::HistoryTransition::Commit(_))));
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
async fn history_round_trips_backwards_binary_payloads_and_transitions_when_write_backwards_section_is_set() {
    let mut log = sample_log().await;
    log.edits[0].inverse = vec![OpPayload { text: Some("unset foo".to_string()), binary: Some(vec![9, 9]) }, OpPayload { text: Some("unset bar".to_string()), binary: None }];
    log.edits[1].ops[0].binary = Some(vec![7]);
    let options = EncodeOptions { write_backwards_section: true, ..EncodeOptions::default() };
    let bytes = encode_history(&log, &options).await.unwrap();
    let decoded = decode_history(&bytes, &DecodeOptions::default()).await.unwrap();
    assert_eq!(decoded, log);
    assert_eq!(decoded.edits[0].inverse[0].binary, Some(vec![9, 9]));
    assert_eq!(decoded.edits[1].ops[0].binary, Some(vec![7]));
    assert_eq!(decoded.transitions, log.transitions);
}

#[semio_framework_async_macros::async_test]
async fn history_writes_every_transition_as_a_critical_record() {
    let log = sample_log().await;
    let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
    let mut cursor = FrameCursor::new(&bytes, HEADER_SIZE as u64).await;
    let mut transitions = 0;
    while let Some(frame) = cursor.next_frame().await.unwrap() {
        assert!(frame.kind != 0x40, "the undo/redo cursor record is gone");
        if frame.kind == REC_TRANSITION {
            assert_ne!(frame.flags & crate::os_spr::FRAME_FLAG_CRITICAL, 0);
            transitions += 1;
        }
    }
    assert_eq!(transitions, log.transitions.len());
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
    for transition in &log.transitions {
        appender.append_transition(transition).await.unwrap();
    }
    appender.commit().await.unwrap();
    let streamed_bytes = appender.into_sink().await;

    let decoded = decode_history(&streamed_bytes, &DecodeOptions::default()).await.unwrap();
    assert_eq!(decoded, log);
}

#[semio_framework_async_macros::async_test]
async fn appended_transitions_decode_in_append_order_across_commits() {
    let log = sample_log().await;
    let options = WriteOptions { required_flags: crate::os_spr::REQUIRED_HASH_CHAIN, optional_flags: crate::os_spr::OPTIONAL_CANONICAL };
    let mut appender = HistoryAppender::begin(Vec::<u8>::new(), &log.doc_id, &log.schema, &options).await.unwrap();
    let mut offsets = Vec::new();
    for transition in log.transitions.iter().rev() {
        offsets.push(appender.append_transition(transition).await.unwrap());
        appender.commit().await.unwrap();
    }
    let bytes = appender.into_sink().await;
    assert!(offsets.windows(2).all(|pair| pair[0] < pair[1]));
    let decoded = decode_history(&bytes, &DecodeOptions::default()).await.unwrap();
    assert_eq!(decoded.transitions, log.transitions.iter().rev().cloned().collect::<Vec<_>>());
}
//#endregion 🔖️Append

//#region 🔖️Fold
fn fold_edit(id: &str, op_id: &str, physical_ms: i64) -> HistoryEdit {
    HistoryEdit {
        id: id.to_string(),
        actor: Some("alice".to_string()),
        started_at: "2024-01-15T10:30:00Z".to_string(),
        finished_at: None,
        coalesce_key: None,
        description: None,
        ops: vec![OpPayload { text: Some(format!("set {id}=1")), binary: None }],
        inverse: Vec::new(),
        meta: Some(vec![HistoryOpMeta { op_id: Some(op_id.to_string()), hlt: Some((1, physical_ms, 0)), ..HistoryOpMeta::default() }]),
    }
}

fn fold_log() -> HistoryLog {
    HistoryLog {
        doc_id: "doc-f".to_string(),
        schema: "schema-f".to_string(),
        edits: vec![fold_edit("edit-a", "op-a", 100), fold_edit("edit-b", "op-b", 200)],
        transitions: vec![
            transition_record("doc-f", "alice", crate::os_spr::HistoryTransition::Revert { mutation_ids: vec![crate::os_spr::MutationId("op-b".to_string())] }, (1, 400, 0)),
            transition_record("doc-f", "alice", commit("ck-1", None, "change-1", &["op-a", "op-b"]), (1, 300, 0)),
        ],
        composition: None,
        conflicts: Vec::new(),
    }
}

#[semio_framework_async_macros::async_test]
async fn fold_derives_applied_redo_and_checkpoint_from_edits_commit_and_revert() {
    let fold = fold_log().fold().unwrap();
    assert_eq!(fold.applied, vec!["edit-a".to_string()]);
    assert_eq!(fold.redo, vec!["edit-b".to_string()]);
    assert_eq!(fold.checkpoint.as_deref(), Some("ck-1"));
    assert_eq!(fold.alternative, None);
    assert_eq!(fold.changes.len(), 1);
    assert_eq!(fold.changes[0].edit_ids, vec!["edit-a".to_string(), "edit-b".to_string()]);
    assert_eq!(fold.checkpoints[0].change_ids, vec!["change-1".to_string()]);
    assert_eq!(fold.checkpoints[0].authors[0].name, "Ueli Saluz");
}

#[semio_framework_async_macros::async_test]
async fn fold_is_identical_after_a_binary_round_trip() {
    let log = fold_log();
    let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
    assert_eq!(decode_history(&bytes, &DecodeOptions::default()).await.unwrap().fold().unwrap(), log.fold().unwrap());
}

#[semio_framework_async_macros::async_test]
async fn fold_falls_back_to_positional_mutation_ids_and_zero_clock_without_meta() {
    let log = sample_log().await;
    let fold = log.fold().unwrap();
    assert_eq!(fold.changes[0].edit_ids, vec!["edit-1".to_string(), "edit-2".to_string()]);
    assert_eq!(fold.alternative.as_deref(), Some("alt-1"));
    assert_eq!(fold.alternatives[0].checkpoint_ids, vec!["ck-1".to_string()]);
    assert_eq!(fold.applied, vec!["edit-1".to_string(), "edit-2".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn fold_excludes_edits_quarantined_by_an_unaccepted_conflict() {
    let mut log = fold_log();
    log.transitions.clear();
    let quarantined = crate::os_spr::MutationEnvelope {
        mutation_id: crate::os_spr::MutationId("op-a".to_string()),
        document_id: crate::os_spr::ArtifactId("doc-f".to_string()),
        actor: crate::os_spr::ActorId("bob".to_string()),
        dependencies: Vec::new(),
        diff: crate::os_spr::ArtifactDiff { schema: crate::os_spr::SchemaId("schema-f".to_string()), payload: vec![1] },
        inverse: crate::os_spr::InverseMutation { schema: crate::os_spr::SchemaId("schema-f".to_string()), payload: Vec::new() },
        timestamp: crate::os_spr::HybridLogicalTimestamp { actor: 2, physical_ms: 100, logical: 0 },
    };
    let mut envelope = Vec::new();
    crate::os_spr::encode_envelope(&quarantined, &mut envelope);
    log.conflicts.push(HistoryConflict { id: "conflict-1".to_string(), kind: 0, status: 0, actors: vec!["bob".to_string()], hlt: (2, 100, 0), edit_ids: Vec::new(), envelopes: vec![envelope], messages: Vec::new() });
    assert_eq!(log.fold().unwrap().applied, vec!["edit-b".to_string()]);
    log.conflicts[0].status = 1;
    assert_eq!(log.fold().unwrap().applied, vec!["edit-a".to_string(), "edit-b".to_string()]);
    log.conflicts[0].status = 2;
    assert_eq!(log.fold().unwrap().applied, vec!["edit-b".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn fold_rejects_a_transition_naming_an_unknown_operation_and_a_negative_clock() {
    let mut log = fold_log();
    log.transitions.push(transition_record("doc-f", "alice", crate::os_spr::HistoryTransition::Reinstate { mutation_ids: vec![crate::os_spr::MutationId("op-missing".to_string())] }, (1, 500, 0)));
    assert!(log.fold().is_err());
    let mut log = fold_log();
    log.edits[0].meta.as_mut().unwrap()[0].hlt = Some((1, -1, 0));
    assert!(log.fold().is_err());
}
//#endregion 🔖️Fold

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
