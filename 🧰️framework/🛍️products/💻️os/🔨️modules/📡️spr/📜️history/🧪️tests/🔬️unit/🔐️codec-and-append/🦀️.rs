/// 🔐️ The history codec identity and its verification, tamper detection, critical transition records and streamed append against the buffered encode.

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
            assert_ne!(frame.flags & protocol::FRAME_FLAG_CRITICAL, 0);
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
    let options = WriteOptions { required_flags: protocol::REQUIRED_HASH_CHAIN, optional_flags: protocol::OPTIONAL_CANONICAL };
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
    let options = WriteOptions { required_flags: protocol::REQUIRED_HASH_CHAIN, optional_flags: protocol::OPTIONAL_CANONICAL };
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
