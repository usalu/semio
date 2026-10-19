/// 📜️ The `.ops` text grammar as the shared log and the doc, edit and transition payloads with their language-agnostic fixture.

//#region 🔖️TextGrammar
#[semio_framework_async_macros::async_test]
async fn ops_text_round_trips_a_full_log() {
    // region note) — parse_ops_text always yields `meta: None`, so the expectation strips it
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
    assert_eq!(log.transitions, vec![HistoryTransitionRecord { id: "t-1".to_string(), actor: "alice".into(), hlt: (1, 2, 3), dependencies: Vec::new(), observed: None, payload: hex("0001046f702d62") }]);
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
    log.transitions.push(HistoryTransitionRecord { id: "transition-x".to_string(), actor: "bob".into(), hlt: (u64::MAX, 0, 7), dependencies: vec!["op-1".to_string(), "edit-1#0".to_string()], observed: None, payload: Vec::new() });
    let text = print_ops_text(&log).unwrap();
    assert_eq!(text.lines().filter(|line| line.starts_with("transition ")).count(), 3);
    assert_eq!(parse_ops_text(&text).unwrap(), log);
}

/// 👁 LAW: the `.ops` text is the shared log — a replica viewing an alternative or an explicit checkpoint prints exactly the
/// text the canonical trunk tip prints, and its text parses back to the trunk tip; the head stays `.spr`-local.
#[semio_framework_async_macros::async_test]
async fn ops_text_is_the_shared_log_without_the_viewer_head() {
    let mut log = sample_log().await;
    for edit in &mut log.edits {
        edit.meta = None;
    }
    let shared = print_ops_text(&log).unwrap();
    for (line, checkpoint) in [(Some("alt-1"), None), (None, Some("ck-1")), (Some("alt-1"), Some("ck-1"))] {
        let viewed = HistoryLog { viewer_line: line.map(str::to_string), viewer_checkpoint: checkpoint.map(str::to_string), ..log.clone() };
        assert_eq!(print_ops_text(&viewed).unwrap(), shared, "the viewer head never reaches the text");
        assert_eq!(parse_ops_text(&shared).unwrap(), log, "the text parses back to the trunk tip");
    }
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
    let edit = HistoryEdit { line: None, id: "edit-x".to_string(), actor: None, started_at: "2024-01-01T00:00:00Z".to_string(), finished_at: None, verb: None, ops: Vec::new(), inverse: Vec::new(), meta: None, lane: None };
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
    let revert = protocol::HistoryTransition::Revert { mutation_ids: record["transition"]["revert"].as_array().unwrap().iter().map(|id| protocol::MutationId(id.as_str().unwrap().to_string())).collect() };
    assert_eq!(protocol::encode_history_transition(&revert), hex(record["payloadHex"].as_str().unwrap()));
    let transition = HistoryTransitionRecord {
        id: record["id"].as_str().unwrap().to_string(),
        actor: record["actor"].as_str().unwrap().into(),
        hlt: (hlt[0], hlt[1], hlt[2]),
        dependencies: record["dependencies"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_string()).collect(),
        observed: None,
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
    assert_eq!(envelope.actor.0.as_ptr(),transition.actor.as_ptr());
    assert!(protocol::is_history_transition(&envelope));
    assert_eq!(envelope.inverse.schema.0, protocol::HISTORY_TRANSITION_SCHEMA);
    assert!(envelope.inverse.payload.is_empty());
    assert_eq!(envelope.document_id.0, "doc-1");
    assert_eq!(HistoryTransitionRecord::from_envelope(&envelope), transition);
    assert_eq!(HistoryTransitionRecord::from_envelope(&envelope).actor.as_ptr(),transition.actor.as_ptr());
    assert!(matches!(protocol::history_transition_from_envelope(&envelope).unwrap(), Some(protocol::HistoryTransition::Commit(_))));
}

#[semio_framework_async_macros::async_test]
async fn edit_payload_round_trips_a_backwards_section_mixing_text_and_binary_payloads() {
    let edit = HistoryEdit { line: None,
        id: "edit-y".to_string(),
        actor: Some("bob".into()),
        started_at: "2024-02-01T00:00:00Z".to_string(),
        finished_at: Some("2024-02-01T00:00:01Z".to_string()), verb: None,
        ops: vec![OpPayload { text: Some("set n=1".to_string()), binary: Some(vec![1, 2, 3]) }, OpPayload { text: Some("set n=2".to_string()), binary: None }],
        inverse: vec![OpPayload { text: Some("set n=0".to_string()), binary: Some(vec![0]) }, OpPayload { text: Some("set n=1".to_string()), binary: None }],
        meta: None, lane: None,
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
    let edit = HistoryEdit { line: None,
        id: "edit-z".to_string(),
        actor: None,
        started_at: "2024-02-01T00:00:00Z".to_string(),
        finished_at: None, verb: None,
        ops: vec![OpPayload { text: Some("noop".to_string()), binary: None }],
        inverse: Vec::new(),
        meta: None, lane: None,
    };
    let mut dict = DictBuilder::new();
    let payload = encode_edit(&edit, &mut dict, |_| None).await.unwrap();
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
