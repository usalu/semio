//! 🔌️ Laws of the duplex socket door (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W15e,
//! audit item 4). Two halves are proven here: the WIRE both isolates must agree on, and the bounded
//! [`SocketLane`] that makes "the peer outran us" a reported number rather than an unbounded queue.
//!
//! ⚖️ Why these run natively: the lane is target-neutral on purpose — the browser door and the
//! native `tokio-tungstenite` transport drive the identical object — so every law below holds on the
//! target the renderer suite actually runs on.

use super::*;

//#region 🔖️Wire
#[test]
fn open_seals_the_url_and_the_exact_ordered_protocol_list() {
    let request = encode_socket_open(7, "wss://hub.example/directory/socket/v1?since=3", &["semio.socket.v1".to_string(), "grantproof".to_string()]).expect("open seals");
    let parsed: serde_json::Value = serde_json::from_str(&request).expect("json");
    assert_eq!(parsed["op"], SOCKET_DOOR_OP);
    assert_eq!(parsed["verb"], "open");
    assert_eq!(parsed["socketId"], 7);
    assert_eq!(parsed["url"], "wss://hub.example/directory/socket/v1?since=3");
    assert_eq!(parsed["protocols"][0], "semio.socket.v1");
    assert_eq!(parsed["protocols"][1], "grantproof");
}

#[test]
fn open_without_a_url_is_refused_at_the_encoder() {
    assert!(encode_socket_open(1, "", &[]).is_err());
}

#[test]
fn a_binary_send_travels_as_standard_base64_and_round_trips() {
    let bytes = vec![0u8, 1, 2, 250, 255];
    let request = encode_socket_send_binary(3, &bytes).expect("send seals");
    let parsed: SocketDoorRequestV1 = serde_json::from_str(&request).expect("json");
    assert_eq!(parsed.verb, "send");
    assert!(parsed.text.is_none());
    let encoded = parsed.binary.expect("binary");
    assert_eq!(semio_framework_io_base64::base64_standard_decode(&encoded).expect("decode"), bytes);
}

#[test]
fn a_frame_past_the_send_ceiling_is_refused_rather_than_handed_to_the_socket() {
    let oversized = vec![0u8; SOCKET_DOOR_SEND_MAX_BYTES + 1];
    assert!(encode_socket_send_binary(1, &oversized).is_err());
    assert!(encode_socket_send_text(1, &"x".repeat(SOCKET_DOOR_SEND_MAX_BYTES + 1)).is_err());
}

#[test]
fn poll_clamps_its_own_page_rather_than_trusting_a_caller() {
    let request = encode_socket_poll(2).expect("poll seals");
    let parsed: SocketDoorRequestV1 = serde_json::from_str(&request).expect("json");
    assert_eq!(parsed.max_messages, Some(SOCKET_DOOR_POLL_MAX_MESSAGES));
    assert_eq!(parsed.max_bytes, Some(SOCKET_DOOR_POLL_MAX_BYTES));
}

#[test]
fn an_answer_decodes_its_state_messages_and_loss_count() {
    let answer = serde_json::json!({
        "state": "open",
        "messages": [{ "text": "{\"kind\":\"heartbeat\"}" }, { "binary": semio_framework_io_base64::base64_standard_encode([9u8, 8, 7]) }],
        "more": true,
        "dropped": 4,
    })
    .to_string();
    let page = decode_socket_answer(&answer).expect("page");
    assert_eq!(page.state, SocketDoorState::Open);
    assert!(page.more);
    assert_eq!(page.dropped, 4);
    assert_eq!(page.messages, vec![SocketMessage::Text("{\"kind\":\"heartbeat\"}".into()), SocketMessage::Binary(vec![9, 8, 7])]);
}

#[test]
fn a_refusal_is_an_error_not_an_empty_page() {
    let answer = serde_json::json!({ "error": "wgpu-host-io.socket: unknown socket 4" }).to_string();
    assert_eq!(decode_socket_answer(&answer).unwrap_err(), "wgpu-host-io.socket: unknown socket 4");
}

#[test]
fn a_stateless_or_unknown_state_answer_is_refused() {
    assert!(decode_socket_answer(&serde_json::json!({ "messages": [] }).to_string()).is_err());
    assert!(decode_socket_answer(&serde_json::json!({ "state": "half-open" }).to_string()).is_err());
}

#[test]
fn one_malformed_frame_is_counted_and_dropped_without_discarding_the_frames_beside_it() {
    let answer = serde_json::json!({
        "state": "open",
        "messages": [{ "text": "kept" }, { "binary": "not base64 at all !!" }, { "text": "also kept" }],
    })
    .to_string();
    let page = decode_socket_answer(&answer).expect("page");
    assert_eq!(page.dropped, 1);
    assert_eq!(page.messages, vec![SocketMessage::Text("kept".into()), SocketMessage::Text("also kept".into())]);
}

#[test]
fn a_close_code_survives_the_answer() {
    let answer = serde_json::json!({ "state": "closed", "closeCode": 4401 }).to_string();
    assert_eq!(decode_socket_answer(&answer).expect("page").close_code, Some(4401));
}
//#endregion 🔖️Wire

//#region 🔖️Lane
#[test]
fn a_fresh_lane_is_connecting_and_neither_open_nor_closed() {
    let lane = SocketLane::new();
    assert_eq!(lane.state(), SocketDoorState::Connecting);
    assert!(!lane.is_open());
    assert!(!lane.is_closed());
    assert_eq!(lane.dropped(), 0);
}

#[test]
fn a_closed_lane_never_reopens_from_a_late_answer() {
    let mut lane = SocketLane::new();
    lane.note_closed(Some(1006));
    lane.note_open();
    assert!(lane.is_closed());
    assert_eq!(lane.close_code(), Some(1006));
}

#[test]
fn closing_discards_the_outbound_queue_and_keeps_what_really_arrived() {
    let mut lane = SocketLane::new();
    lane.note_open();
    assert!(lane.push_outbound(SocketMessage::Text("ping".into())));
    lane.push_inbound(SocketMessage::Text("pong".into()));
    lane.note_closed(None);
    assert_eq!(lane.outbound_len(), 0);
    assert_eq!(lane.inbound_len(), 1);
}

#[test]
fn a_closed_lane_refuses_a_send_rather_than_parking_a_frame_nothing_will_write() {
    let mut lane = SocketLane::new();
    lane.note_closed(None);
    assert!(!lane.push_outbound(SocketMessage::Text("lost".into())));
}

#[test]
fn an_oversized_outbound_frame_is_refused_counted_and_named() {
    let mut lane = SocketLane::new();
    lane.note_open();
    assert!(!lane.push_outbound(SocketMessage::Binary(vec![0u8; SOCKET_DOOR_SEND_MAX_BYTES + 1])));
    assert_eq!(lane.dropped(), 1);
    assert!(lane.last_error().is_some_and(|error| error.contains("socket lane refused")));
}

#[test]
fn a_peer_that_outruns_the_poll_loses_the_oldest_frames_and_the_loss_is_counted() {
    let mut lane = SocketLane::new();
    lane.note_open();
    for index in 0..(SOCKET_LANE_MAX_MESSAGES + 3) {
        lane.push_inbound(SocketMessage::Text(format!("frame-{index}")));
    }
    assert_eq!(lane.inbound_len(), SOCKET_LANE_MAX_MESSAGES);
    assert_eq!(lane.dropped(), 3);
    assert_eq!(lane.pop_inbound(), Some(SocketMessage::Text("frame-3".into())));
}

#[test]
fn an_inbound_page_stops_at_the_message_ceiling() {
    let mut lane = SocketLane::new();
    for index in 0..(SOCKET_DOOR_POLL_MAX_MESSAGES + 5) {
        lane.push_inbound(SocketMessage::Text(format!("{index}")));
    }
    assert_eq!(lane.take_inbound_page().len(), SOCKET_DOOR_POLL_MAX_MESSAGES);
    assert_eq!(lane.inbound_len(), 5);
}

#[test]
fn an_inbound_page_stops_at_the_byte_ceiling_but_never_returns_nothing() {
    let mut lane = SocketLane::new();
    let big = "x".repeat(SOCKET_DOOR_POLL_MAX_BYTES - 1);
    lane.push_inbound(SocketMessage::Text(big));
    lane.push_inbound(SocketMessage::Text("x".repeat(64)));
    let page = lane.take_inbound_page();
    assert_eq!(page.len(), 1, "the byte ceiling cuts the page after the first frame");
    // 🧾️ A single frame LARGER than the whole page ceiling still travels: refusing it would stall
    // the lane forever on a frame the peer is entitled to send.
    let mut lane = SocketLane::new();
    lane.push_inbound(SocketMessage::Text("y".repeat(SOCKET_DOOR_POLL_MAX_BYTES * 2)));
    assert_eq!(lane.take_inbound_page().len(), 1);
}

#[test]
fn applying_a_door_page_folds_its_state_frames_and_loss_into_the_lane() {
    let mut lane = SocketLane::new();
    lane.apply_page(SocketDoorPage { state: SocketDoorState::Open, messages: vec![SocketMessage::Text("a".into())], more: true, dropped: 2, close_code: None });
    assert!(lane.is_open());
    assert_eq!(lane.inbound_len(), 1);
    assert_eq!(lane.dropped(), 2);
    lane.apply_page(SocketDoorPage { state: SocketDoorState::Closed, close_code: Some(1001), ..SocketDoorPage::default() });
    assert!(lane.is_closed());
    assert_eq!(lane.close_code(), Some(1001));
}

#[test]
fn a_redial_keeps_nothing_from_the_dead_socket() {
    let mut lane = SocketLane::new();
    lane.note_open();
    lane.push_inbound(SocketMessage::Text("stale".into()));
    lane.push_outbound(SocketMessage::Text("half written".into()));
    lane.note_closed(Some(1006));
    lane.reset_for_redial();
    assert_eq!(lane.state(), SocketDoorState::Connecting);
    assert_eq!(lane.inbound_len(), 0);
    assert_eq!(lane.outbound_len(), 0);
    assert_eq!(lane.close_code(), None);
}

#[test]
fn every_state_spelling_round_trips_between_the_two_isolates() {
    for state in [SocketDoorState::Connecting, SocketDoorState::Open, SocketDoorState::Closed] {
        assert_eq!(SocketDoorState::from_wire(state.wire()), Some(state));
    }
}
//#endregion 🔖️Lane
