/// ⚔️ The conflict records and the op-meta messages: both kinds byte-identically, dictionary and ordinal references, rejected tags, severity shapes and message code interning.

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
        origin: protocol::mutation::MutationOrigin::Owner,
        transaction: None,
        messages: vec![
            HistoryMessage { level: 0, code: "mutation.cascade".to_string(), message: "cascaded".to_string(), target: Vec::new(), op_index: None },
            HistoryMessage { level: 1, code: "mutation.clamped".to_string(), message: "clamped".to_string(), target: vec!["a".to_string()], op_index: Some(0) },
            HistoryMessage { level: 2, code: "mutation.target-missing".to_string(), message: "missing".to_string(), target: vec!["a".to_string(), "b".to_string()], op_index: Some(3) },
            HistoryMessage { level: 3, code: "mutation.invariant".to_string(), message: "broken".to_string(), target: vec!["x".to_string(), "y".to_string(), "z".to_string()], op_index: None },
        ],
    };
    let edit = HistoryEdit { line: None,
        id: "edit-m".to_string(),
        actor: None,
        started_at: "2024-01-01T00:00:00Z".to_string(),
        finished_at: None, verb: None,
        ops: vec![OpPayload { text: Some("noop".to_string()), binary: None }],
        inverse: Vec::new(),
        meta: Some(vec![meta.clone()]), lane: None,
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
    let meta = HistoryOpMeta { op_id: None, dependencies: Vec::new(), base_version: 0, author_id: None, hlt: None, undo_policy: 0, payload_hash: None, group_id: None, origin: protocol::mutation::MutationOrigin::Owner, messages: Vec::new(), transaction: None };
    let mut dict = DictBuilder::new();
    let mut out = ByteWriter::new();
    write_op_meta(&mut out, &meta, &mut dict, &|_: &str| None).await.unwrap();
    let payload = out.into_bytes();
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
    let origin = protocol::MutationOrigin::Contributed { plugin_id: "s.stdio.mesh".to_string(), mutation_id: protocol::SchemaId("mesh/v1".to_string()), payload_hash: protocol::PayloadHash(core::array::from_fn(|index| index as u8)) };
    let oracle = ContributedOriginOracle { kind: "contributed".to_string(), plugin_id: "s.stdio.mesh".to_string(), mutation_id: "mesh/v1".to_string(), payload_hash: core::array::from_fn(|index| index as u8) };
    let via_serde = serde_json::to_string(&oracle).expect("serde_json encodes independent origin oracle");
    let via_pack = semio_framework_pack_json::to_json_string(&origin);
    assert_eq!(via_serde, via_pack, "canonical origin bytes must match the independent oracle");
    assert!(via_pack.contains("\"payloadHash\":[0,1,2,"), "got {via_pack} — payload_hash bytes must stay bare integers, never x.0");
    let round_trip_serde: ContributedOriginOracle = serde_json::from_str(&via_pack).expect("serde_json decodes first-party origin bytes");
    let round_trip_pack: protocol::MutationOrigin = semio_framework_pack_json::from_json_str(&via_serde, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("pack::json decodes independent origin bytes");
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
    assert!(bytes_per_message < 10, "expected sub-10-byte-per-message average from interning, got {bytes_per_message}");
}
//#endregion 🔖️Message
