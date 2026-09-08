
use super::*;

fn bounded_shell_decode(bytes: &[u8]) -> Result<ShellToGateway, ShellDecodeFault> {
    let mut decoder = ShellToGatewayDecodeCursor::new(bytes.len());
    let validated = loop {
        match decoder.step(|index| bytes.get(index).copied()) {
            ShellDecodeStep::Pending => {}
            ShellDecodeStep::Complete(frame) => break frame,
            ShellDecodeStep::Fault(fault) => return Err(fault),
        }
    };
    let mut materializer = ShellToGatewayMaterializeCursor::new(validated);
    loop {
        match materializer.step() {
            ShellMaterializeStep::Pending => {}
            ShellMaterializeStep::Complete(frame) => return Ok(frame),
            ShellMaterializeStep::Fault(fault) => return Err(fault),
        }
    }
}

fn retained_broadcast_cursor(handle: &BridgeHandle, ids: &[ShellConnectionId], frame: GatewayToShell) -> BridgeBroadcastCursor {
    let expected = frame.encoded_len().unwrap();
    handle.inner.asynchronous.reserve_broadcast().unwrap();
    let retirement = handle.inner.asynchronous.reserve_retirement().unwrap();
    let connections = handle.inner.connections.lock().unwrap();
    let mut recipients_state = std::array::from_fn(|_| None);
    for (index, id) in ids.iter().copied().enumerate() {
        let outbox = Arc::clone(&connections.get(&id).unwrap().outbox);
        let grant = outbox.claim(expected).unwrap();
        recipients_state[index] = Some(BridgeRecipientState::Claimed { id, outbox, grant });
    }
    BridgeBroadcastCursor {
        frame: Some(frame),
        expected,
        offset: 0,
        encoded: BridgeEncodedFrame::empty(expected, Some((Arc::clone(&handle.inner.asynchronous), retirement))),
        shared: None,
        recipients_state,
        recipients: ids.len(),
        recipient_cursor: 0,
        delivered: 0,
        recipient_closed: 0,
    }
}

fn pending_broadcast(step: BridgeBroadcastStep) -> BridgeBroadcastCursor {
    match step {
        BridgeBroadcastStep::Pending(cursor) => cursor,
        BridgeBroadcastStep::Complete(_) => panic!("broadcast completed before requested fixture boundary"),
    }
}

fn encoded_broadcast(mut cursor: BridgeBroadcastCursor) -> BridgeBroadcastCursor {
    while cursor.offset != cursor.expected || cursor.shared.is_none() {
        cursor = pending_broadcast(cursor.step());
    }
    cursor
}

fn sample_shell_frames() -> Vec<ShellToGateway> {
    vec![
        ShellToGateway::Hello {
            bridge_version: BRIDGE_VERSION,
            shell_kind: ShellKind::React,
            shell_session_id: "shell-1".into(),
            principal_actor: "agent:local".into(),
            flags: BridgeFlags { relay_app_commands: true, shared_backbone: false, elicit: true },
        },
        ShellToGateway::ShellState { revision: 7, state: vec![1, 2, 3, 4] },
        ShellToGateway::ShellStatePatch { revision: 8, base_revision: 7, patch: vec![9, 9] },
        ShellToGateway::Instances { entries: vec![BridgeInstanceRef { plugin_id: "cad".into(), app_id: "viewport".into(), instance_id: "inst-1".into(), artifact_ref: "cad-1".into(), window_ids: vec!["win-1".into(), "win-2".into()] }] },
        ShellToGateway::AppFrames { in_reply_to: 3, instance_id: "inst-1".into(), frames: vec![vec![1], vec![2, 3]] },
        ShellToGateway::ShellCommandResult { in_reply_to: 5, ok: true, fault: None },
        ShellToGateway::ShellCommandResult { in_reply_to: 6, ok: false, fault: Some("instance-busy".into()) },
        ShellToGateway::Approval { approval_id: "appr_1".into(), decision: ApprovalDecision::Once, note: Some("looks fine".into()) },
        ShellToGateway::Approval { approval_id: "appr_2".into(), decision: ApprovalDecision::Deny, note: None },
        ShellToGateway::Ping,
        ShellToGateway::Bye,
    ]
}

fn sample_gateway_frames() -> Vec<GatewayToShell> {
    vec![
        GatewayToShell::Welcome { bridge_version: BRIDGE_VERSION, connection: "conn_1".into(), principal: "agent:local".into() },
        GatewayToShell::ShellCommand { seq: 1, command: vec![1, 2] },
        GatewayToShell::AppCommand { seq: 2, instance_id: "inst-1".into(), command: vec![3] },
        GatewayToShell::ApprovalRequested { approval_id: "appr_1".into(), summary: "translate selection by (1,0,0)".into() },
        GatewayToShell::ApprovalResolved { approval_id: "appr_1".into(), decision: ApprovalDecision::Session },
        GatewayToShell::AgentPresence { active: true, label: "claude-code".into(), invocation_id: Some("inv-1".into()) },
        GatewayToShell::AgentPresence { active: false, label: "".into(), invocation_id: None },
        GatewayToShell::Pong,
        GatewayToShell::Bye { reason: "shutdown".into() },
    ]
}

//#region 🔖️RoundTrip
#[test]
fn every_shell_to_gateway_variant_round_trips_through_encode_decode() {
    for frame in sample_shell_frames() {
        let bytes = frame.encode();
        let decoded = ShellToGateway::decode(&bytes).unwrap_or_else(|error| panic!("decode failed for {frame:?}: {error}"));
        assert_eq!(decoded, frame);
    }
}

#[test]
fn every_gateway_to_shell_variant_round_trips_through_encode_decode() {
    for frame in sample_gateway_frames() {
        let bytes = frame.encode();
        let decoded = GatewayToShell::decode(&bytes).unwrap_or_else(|error| panic!("decode failed for {frame:?}: {error}"));
        assert_eq!(decoded, frame);
    }
}

#[test]
fn decode_rejects_truncated_buffers() {
    // Tag 0 = Hello, which expects far more bytes than just the tag byte itself.
    let hello_tag_only = vec![0u8];
    assert!(ShellToGateway::decode(&hello_tag_only).is_err());
}

#[test]
fn decode_rejects_trailing_bytes() {
    let mut bytes = ShellToGateway::Ping.encode();
    bytes.push(0xFF);
    let error = ShellToGateway::decode(&bytes).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::InputInvalid);
}

#[test]
fn decode_rejects_an_unknown_tag() {
    let error = ShellToGateway::decode(&[99]).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::InputInvalid);
}

#[test]
fn bounded_shell_decoder_rejects_ffffffff_counts_and_truncated_ranges_before_owner_allocation() {
    let mut instances = vec![3];
    instances.extend_from_slice(&u32::MAX.to_le_bytes());
    assert!(matches!(bounded_shell_decode(&instances), Err(ShellDecodeFault::Capacity)));

    let mut windows = vec![3];
    windows.extend_from_slice(&1u32.to_le_bytes());
    for _ in 0..4 {
        windows.extend_from_slice(&0u32.to_le_bytes());
    }
    windows.extend_from_slice(&u32::MAX.to_le_bytes());
    assert!(matches!(bounded_shell_decode(&windows), Err(ShellDecodeFault::Capacity)));

    let mut frames = vec![4];
    frames.extend_from_slice(&7u64.to_le_bytes());
    frames.extend_from_slice(&0u32.to_le_bytes());
    frames.extend_from_slice(&u32::MAX.to_le_bytes());
    assert!(matches!(bounded_shell_decode(&frames), Err(ShellDecodeFault::Capacity)));

    assert!(matches!(bounded_shell_decode(&[3, 1, 0]), Err(ShellDecodeFault::Malformed)));
    let mut truncated_range = vec![1];
    truncated_range.extend_from_slice(&1u64.to_le_bytes());
    truncated_range.extend_from_slice(&4u32.to_le_bytes());
    truncated_range.extend_from_slice(&[1, 2, 3]);
    assert!(matches!(bounded_shell_decode(&truncated_range), Err(ShellDecodeFault::Malformed)));
}

#[test]
fn bounded_shell_decoder_cap_plus_one_and_every_variant_match_the_canonical_fixture() {
    let mut cap_plus_one = vec![1];
    cap_plus_one.extend_from_slice(&1u64.to_le_bytes());
    cap_plus_one.extend_from_slice(&((BRIDGE_INBOUND_MAX_FIELD_BYTES + 1) as u32).to_le_bytes());
    assert!(matches!(bounded_shell_decode(&cap_plus_one), Err(ShellDecodeFault::Capacity)));
    for frame in sample_shell_frames() {
        assert_eq!(bounded_shell_decode(&frame.encode()), Ok(frame));
    }
}

#[test]
fn bounded_shell_decoder_and_materializer_advance_incrementally() {
    let bytes = ShellToGateway::ShellState { revision: 9, state: vec![7; BRIDGE_INBOUND_PAGE_BYTES + 1] }.encode();
    let mut decoder = ShellToGatewayDecodeCursor::new(bytes.len());
    assert!(matches!(decoder.step(|index| bytes.get(index).copied()), ShellDecodeStep::Pending));
    assert!(matches!(decoder.step(|index| bytes.get(index).copied()), ShellDecodeStep::Pending));
    assert!(matches!(decoder.step(|index| bytes.get(index).copied()), ShellDecodeStep::Pending));
    assert_eq!(decoder.cursor, 9, "one preflight grant may consume only one scalar token");
}

#[test]
fn bridge_outbox_item_cap_plus_one_returns_the_exact_frame_and_rearms_after_one_receive() {
    let handle = BridgeHandle::new();
    let (id, mut outbox) = handle.register();
    for _ in 0..BRIDGE_OUTBOX_MAX_ITEMS {
        assert!(handle.try_send_to(id, GatewayToShell::Pong).is_ok());
    }
    let rejected = GatewayToShell::Bye { reason: "cap-plus-one".into() };
    assert_eq!(handle.try_send_to(id, rejected.clone()), Err(rejected));
    assert_eq!(outbox.try_recv(), Some(GatewayToShell::Pong));
    assert!(handle.try_send_to(id, GatewayToShell::Pong).is_ok());
}

#[test]
fn bridge_outbox_byte_cap_plus_one_returns_the_exact_frame_before_queue_mutation() {
    let outbox = BridgeOutbox::new(7);
    let accepted = GatewayToShell::ShellCommand { seq: 7, command: vec![9; BRIDGE_OUTBOX_MAX_BYTES - 13] };
    assert_eq!(accepted.encoded_len(), Some(BRIDGE_OUTBOX_MAX_BYTES));
    assert_eq!(outbox.try_send(accepted), Ok(()));
    let encoded = outbox.try_recv().unwrap().lease.encoded;
    assert_eq!(encoded.len(), BRIDGE_OUTBOX_MAX_BYTES);
    assert_eq!(encoded.page_count(), BRIDGE_OUTBOX_MAX_PAGES);

    let rejected = GatewayToShell::ShellCommand { seq: 8, command: vec![7; BRIDGE_OUTBOX_MAX_BYTES - 12] };
    let rejected_copy = rejected.clone();
    let encodes_before = outbox.state.lock().unwrap().encode_count;
    assert_eq!(outbox.try_send(rejected), Err(rejected_copy));
    let state = outbox.state.lock().unwrap();
    assert_eq!(state.encode_count, encodes_before, "cap+1 must reject before page allocation/encode");
    assert_eq!(state.len, 0);
    assert_eq!(state.bytes, 0);
    assert!(bridge_wire_field_len(usize::MAX).is_none(), "checked wire-size overflow must fail before admission");
}

#[test]
fn bridge_outbox_page_boundary_matches_the_canonical_encoder() {
    for reason_bytes in [BRIDGE_OUTBOX_PAGE_BYTES - 5, BRIDGE_OUTBOX_PAGE_BYTES - 4] {
        let frame = GatewayToShell::Bye { reason: "x".repeat(reason_bytes) };
        let expected = frame.encode();
        let outbox = BridgeOutbox::new(9);
        outbox.try_send(frame).unwrap();
        let encoded = outbox.try_recv().unwrap().lease.encoded;
        let mut actual = vec![0; encoded.len()];
        assert_eq!(encoded.copy_into(0, &mut actual), actual.len());
        assert_eq!(actual, expected);
        assert_eq!(encoded.page_count(), if reason_bytes == BRIDGE_OUTBOX_PAGE_BYTES - 5 { 1 } else { 2 });
    }
}

#[test]
fn bridge_outbox_terminal_close_rejects_the_exact_late_frame() {
    let handle = BridgeHandle::new();
    let (id, mut outbox) = handle.register();
    handle.unregister(id);
    let rejected = GatewayToShell::Bye { reason: "late-after-close".into() };
    assert_eq!(handle.try_send_to(id, rejected.clone()), Err(rejected));
    assert!(outbox.try_recv().is_none());
}

#[test]
fn broadcast_partial_saturation_rolls_back_every_claim_and_returns_the_exact_uncloned_message() {
    let handle = BridgeHandle::new();
    let (first, _first_receiver) = handle.register();
    let (_second, mut second_receiver) = handle.register();
    for _ in 0..BRIDGE_OUTBOX_MAX_ITEMS {
        assert!(handle.try_send_to(first, GatewayToShell::Pong).is_ok());
    }
    let original = GatewayToShell::Bye { reason: "partial-saturation".into() };
    assert_eq!(handle.broadcast(original.clone()), Err(original));
    assert!(second_receiver.try_recv().is_none());
    assert_eq!(handle.inner.asynchronous.state.lock().unwrap().broadcast_len, 0);
}

#[test]
fn broadcast_many_recipient_and_oversize_preflight_reject_before_encode() {
    let handle = BridgeHandle::new();
    for _ in 0..=BRIDGE_BROADCAST_MAX_RECIPIENTS {
        handle.register();
    }
    let many = GatewayToShell::Pong;
    assert_eq!(handle.broadcast(many.clone()), Err(many));

    let other = BridgeHandle::new();
    other.register();
    let oversized = GatewayToShell::ShellCommand { seq: 1, command: vec![0; BRIDGE_OUTBOX_MAX_BYTES] };
    assert_eq!(other.broadcast(oversized.clone()), Err(oversized));
    assert_eq!(other.inner.asynchronous.state.lock().unwrap().broadcast_len, 0);
}

#[test]
fn shared_broadcast_leases_are_generation_keyed_and_close_rejects_aba_publish() {
    let first = Arc::new(BridgeOutbox::new(11));
    let second = Arc::new(BridgeOutbox::new(12));
    let frame = GatewayToShell::Bye { reason: "shared".into() };
    let bytes = frame.encoded_len().unwrap();
    let first_grant = first.claim(bytes).unwrap();
    let second_grant = second.claim(bytes).unwrap();
    let encoded = Arc::new(BridgeEncodedFrame::encode(&frame, bytes));
    assert!(first.publish(first_grant, Arc::clone(&encoded)).is_ok());
    assert!(second.publish(second_grant, Arc::clone(&encoded)).is_ok());
    let first_lease = first.try_recv().unwrap().lease;
    let second_lease = second.try_recv().unwrap().lease;
    assert!(Arc::ptr_eq(&first_lease.encoded, &second_lease.encoded));

    let stale = first.claim(bytes).unwrap();
    first.close();
    assert!(first.publish(stale, Arc::clone(&encoded)).is_err());
    assert!(first.try_recv().is_none(), "closed generation must not yield an ABA lease");
}

#[test]
fn broadcast_close_before_first_publish_delivers_survivors_in_stable_admitted_order() {
    let handle = BridgeHandle::new();
    let (first, mut first_receiver) = handle.register();
    let (second, mut second_receiver) = handle.register();
    let original = GatewayToShell::Bye { reason: "x".repeat(BRIDGE_OUTBOX_PAGE_BYTES + 1) };
    let mut cursor = encoded_broadcast(retained_broadcast_cursor(&handle, &[first, second], original));
    handle.unregister(first);
    cursor = pending_broadcast(cursor.step());
    assert!(matches!(cursor.recipients_state[0].as_ref(), Some(BridgeRecipientState::RecipientClosed { id, .. }) if *id == first));
    cursor = pending_broadcast(cursor.step());
    assert!(matches!(cursor.recipients_state[1].as_ref(), Some(BridgeRecipientState::Published { id }) if *id == second));
    assert!(matches!(cursor.step(), BridgeBroadcastStep::Complete(BridgeBroadcastCompletion::Delivered { delivered: 1, recipient_closed: 1 })));
    assert!(first_receiver.try_recv().is_none());
    assert!(second_receiver.try_recv().is_some());
}

#[test]
fn broadcast_close_mid_recipient_list_reports_partial_counts_and_fifo_delivery() {
    let handle = BridgeHandle::new();
    let (first, mut first_receiver) = handle.register();
    let (second, mut second_receiver) = handle.register();
    let (third, mut third_receiver) = handle.register();
    let mut cursor = encoded_broadcast(retained_broadcast_cursor(&handle, &[first, second, third], GatewayToShell::Pong));
    cursor = pending_broadcast(cursor.step());
    handle.unregister(second);
    cursor = pending_broadcast(cursor.step());
    cursor = pending_broadcast(cursor.step());
    assert!(matches!(cursor.recipients_state[0].as_ref(), Some(BridgeRecipientState::Published { id }) if *id == first));
    assert!(matches!(cursor.recipients_state[1].as_ref(), Some(BridgeRecipientState::RecipientClosed { id, .. }) if *id == second));
    assert!(matches!(cursor.recipients_state[2].as_ref(), Some(BridgeRecipientState::Published { id }) if *id == third));
    assert!(matches!(cursor.step(), BridgeBroadcastStep::Complete(BridgeBroadcastCompletion::Delivered { delivered: 2, recipient_closed: 1 })));
    assert_eq!(first_receiver.try_recv(), Some(GatewayToShell::Pong));
    assert!(second_receiver.try_recv().is_none());
    assert_eq!(third_receiver.try_recv(), Some(GatewayToShell::Pong));
}

#[test]
fn broadcast_all_close_returns_the_exact_original_completion_after_every_claim() {
    let handle = BridgeHandle::new();
    let (first, _first_receiver) = handle.register();
    let (second, _second_receiver) = handle.register();
    let original = GatewayToShell::Bye { reason: "all-closed".into() };
    let mut cursor = encoded_broadcast(retained_broadcast_cursor(&handle, &[first, second], original.clone()));
    handle.unregister(first);
    handle.unregister(second);
    cursor = pending_broadcast(cursor.step());
    cursor = pending_broadcast(cursor.step());
    assert_eq!(
        match cursor.step() {
            BridgeBroadcastStep::Complete(completion) => completion,
            BridgeBroadcastStep::Pending(_) => panic!("all-close completion remained pending"),
        },
        BridgeBroadcastCompletion::Undelivered { frame: original, recipient_closed: 2 }
    );
}

#[test]
fn broadcast_reopen_same_slot_aba_cannot_consume_the_stale_recipient_claim() {
    let old = Arc::new(BridgeOutbox::new(41));
    let frame = GatewayToShell::Pong;
    let bytes = frame.encoded_len().unwrap();
    let stale = old.claim(bytes).unwrap();
    old.close();
    let reopened = Arc::new(BridgeOutbox::new(42));
    let encoded = Arc::new(BridgeEncodedFrame::encode(&frame, bytes));
    let rejected = old.publish(stale, Arc::clone(&encoded)).unwrap_err();
    assert_eq!(rejected.grant.generation, 41);
    assert_eq!(reopened.state.lock().unwrap().len, 0);
    assert_eq!(reopened.state.lock().unwrap().reserved_items, 0);
}

#[test]
fn broadcast_shutdown_cancel_poison_closes_each_remaining_claim_one_grant_then_reports_partial_delivery() {
    let handle = BridgeHandle::new();
    let (first, mut first_receiver) = handle.register();
    let (second, _second_receiver) = handle.register();
    let (third, _third_receiver) = handle.register();
    let cursor = encoded_broadcast(retained_broadcast_cursor(&handle, &[first, second, third], GatewayToShell::Pong));
    let cursor = pending_broadcast(cursor.step());
    {
        let mut state = handle.inner.asynchronous.state.lock().unwrap();
        state.broadcast_reserved -= 1;
        state.broadcasts[0] = Some(cursor);
        state.broadcast_len = 1;
    }
    handle.cancel_broadcasts();
    assert!(handle.close_one_terminal_broadcast_claim());
    assert!(handle.close_one_terminal_broadcast_claim());
    assert!(handle.close_one_terminal_broadcast_claim());
    assert_eq!(handle.take_broadcast_completion(), Some(BridgeBroadcastCompletion::Delivered { delivered: 1, recipient_closed: 2 }));
    assert_eq!(first_receiver.try_recv(), Some(GatewayToShell::Pong));
}

#[test]
fn last_shared_lease_transfers_pages_to_one_page_terminal_retirement_grants() {
    let handle = BridgeHandle::new();
    let retirement = handle.inner.asynchronous.reserve_retirement().unwrap();
    handle.inner.asynchronous.terminal.store(true, Ordering::Release);
    let mut encoded = BridgeEncodedFrame::empty(BRIDGE_OUTBOX_PAGE_BYTES + 1, Some((Arc::clone(&handle.inner.asynchronous), retirement)));
    encoded.pages[0] = Some(Box::new([1; BRIDGE_OUTBOX_PAGE_BYTES]));
    encoded.pages[1] = Some(Box::new([2; BRIDGE_OUTBOX_PAGE_BYTES]));
    drop(encoded);
    assert_eq!(handle.inner.asynchronous.state.lock().unwrap().retirement_len, 1);
    assert!(handle.close_one_terminal_retired_page());
    assert!(handle.close_one_terminal_retired_page());
    assert_eq!(handle.inner.asynchronous.state.lock().unwrap().retirement_len, 0);
}

#[test]
fn terminal_broadcast_close_returns_one_exact_original_and_cancels_recipient_credit() {
    let handle = BridgeHandle::new();
    let (id, _receiver) = handle.register();
    let original = GatewayToShell::Bye { reason: "terminal-broadcast".into() };
    let bytes = original.encoded_len().unwrap();
    handle.inner.asynchronous.reserve_broadcast().unwrap();
    let retirement = handle.inner.asynchronous.reserve_retirement().unwrap();
    let outbox = Arc::clone(&handle.inner.connections.lock().unwrap().get(&id).unwrap().outbox);
    let grant = outbox.claim(bytes).unwrap();
    let mut recipients_state: [Option<BridgeRecipientState>; BRIDGE_BROADCAST_MAX_RECIPIENTS] = std::array::from_fn(|_| None);
    recipients_state[0] = Some(BridgeRecipientState::Claimed { id, outbox: Arc::clone(&outbox), grant });
    let cursor = BridgeBroadcastCursor {
        frame: Some(original.clone()),
        expected: bytes,
        offset: 0,
        encoded: BridgeEncodedFrame::empty(bytes, Some((Arc::clone(&handle.inner.asynchronous), retirement))),
        shared: None,
        recipients_state,
        recipients: 1,
        recipient_cursor: 0,
        delivered: 0,
        recipient_closed: 0,
    };
    {
        let mut state = handle.inner.asynchronous.state.lock().unwrap();
        state.broadcast_reserved -= 1;
        state.broadcasts[0] = Some(cursor);
        state.broadcast_len = 1;
    }
    handle.inner.asynchronous.terminal.store(true, Ordering::Release);
    assert!(handle.close_one_terminal_broadcast_claim());
    assert!(handle.close_one_terminal_broadcast_claim());
    assert_eq!(handle.take_broadcast_completion(), Some(BridgeBroadcastCompletion::Undelivered { frame: original, recipient_closed: 1 }));
    let state = outbox.state.lock().unwrap();
    assert_eq!(state.reserved_items, 0);
    assert_eq!(state.reserved_bytes, 0);
    drop(state);
    assert!(handle.close_one_terminal_retired_page());
}
//#endregion 🔖️RoundTrip

//#region 🔖️FixtureParity
/// 🧬️ The anti-drift mechanism: `🧫️fixtures/📨️frames.json` holds `{direction, variant, frame, hex}`
/// rows. For each row this test (a) deserializes `frame` via serde into the real enum, (b) hand-
/// encodes it and compares to `hex`, and (c) hex-decodes `hex` and compares the result back to the
/// serde-deserialized frame — proving the fixture's `frame` JSON and `hex` bytes agree with THIS
/// codec. The TS twin runs the mirror-image check against the SAME file.
#[test]
fn every_fixture_round_trips_through_the_rust_codec() {
    let raw = include_str!("../../🧫️fixtures/📨️frames.json");
    let rows: Vec<serde_json::Value> = serde_json::from_str(raw).expect("🧫️fixtures/📨️frames.json must parse");
    assert!(!rows.is_empty(), "fixture file must not be empty");
    let mut shell_to_gateway_count = 0;
    let mut gateway_to_shell_count = 0;
    for row in &rows {
        let direction = row["direction"].as_str().expect("row.direction");
        let hex = row["hex"].as_str().expect("row.hex");
        let frame_json = row["frame"].clone();
        let expected_bytes = decode_hex(hex);
        match direction {
            "shell_to_gateway" => {
                shell_to_gateway_count += 1;
                let frame: ShellToGateway = serde_json::from_value(frame_json).expect("frame must deserialize as ShellToGateway");
                assert_eq!(encode_hex(&frame.encode()), hex, "encode mismatch for {frame:?}");
                let decoded = ShellToGateway::decode(&expected_bytes).unwrap();
                assert_eq!(decoded, frame, "decode mismatch for hex {hex}");
            }
            "gateway_to_shell" => {
                gateway_to_shell_count += 1;
                let frame: GatewayToShell = serde_json::from_value(frame_json).expect("frame must deserialize as GatewayToShell");
                assert_eq!(encode_hex(&frame.encode()), hex, "encode mismatch for {frame:?}");
                let decoded = GatewayToShell::decode(&expected_bytes).unwrap();
                assert_eq!(decoded, frame, "decode mismatch for hex {hex}");
            }
            other => panic!("unknown fixture direction: {other}"),
        }
    }
    assert_eq!(shell_to_gateway_count, 11, "fixtures must cover every ShellToGateway variant instance");
    assert_eq!(gateway_to_shell_count, 9, "fixtures must cover every GatewayToShell variant instance");
}

fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn decode_hex(hex: &str) -> Vec<u8> {
    (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("valid hex")).collect()
}
//#endregion 🔖️FixtureParity
