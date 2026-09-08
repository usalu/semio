
use super::*;
use crate::protocol::{InMemoryPromptRegistry, InMemoryResourceRegistry, InMemoryToolRegistry, NullBackend};
use crate::workspace::GatewayBackends;
use std::io::Cursor;

fn fresh_server() -> McpServer {
    McpServer::new(Box::new(InMemoryToolRegistry::new()), Box::new(InMemoryResourceRegistry::new()), Box::new(InMemoryPromptRegistry::new()), Box::new(GatewayBackends::Null(NullBackend)))
}

//#region 🔖️Stdio
#[test]
fn one_request_line_produces_exactly_one_response_line_on_stdout() {
    let input = Cursor::new(b"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"ping\"}\n".to_vec());
    let mut output = Vec::new();
    let mut log = Vec::new();
    let mut transport = StdioTransport::new(input, &mut output, &mut log);
    transport.serve(fresh_server()).unwrap();

    let output_text = String::from_utf8(output).unwrap();
    let lines: Vec<&str> = output_text.lines().collect();
    assert_eq!(lines.len(), 1);
    let response: JsonRpcResponse = serde_json::from_str(lines[0]).unwrap();
    assert!(!response.is_error());
}

#[test]
fn malformed_json_logs_to_the_log_writer_and_never_pollutes_stdout_with_non_json_text() {
    let input = Cursor::new(b"not json at all\n".to_vec());
    let mut output = Vec::new();
    let mut log = Vec::new();
    let mut transport = StdioTransport::new(input, &mut output, &mut log);
    transport.serve(fresh_server()).unwrap();

    let log_text = String::from_utf8(log).unwrap();
    assert!(log_text.contains("malformed JSON-RPC line rejected"), "diagnostic text must land in the log writer");

    let output_text = String::from_utf8(output).unwrap();
    for line in output_text.lines() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(line);
        assert!(parsed.is_ok(), "every stdout line must be valid JSON, got: {line}");
    }
    assert!(!output_text.contains("malformed JSON-RPC line rejected"), "stdout must never carry a diagnostic line");
}

#[test]
fn blank_lines_are_skipped_without_producing_output() {
    let input = Cursor::new(b"\n\n{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"ping\"}\n\n".to_vec());
    let mut output = Vec::new();
    let mut log = Vec::new();
    let mut transport = StdioTransport::new(input, &mut output, &mut log);
    transport.serve(fresh_server()).unwrap();
    let output_text = String::from_utf8(output).unwrap();
    assert_eq!(output_text.lines().count(), 1);
}

#[test]
fn a_notification_line_produces_no_output_line_at_all() {
    let input = Cursor::new(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/cancelled\"}\n".to_vec());
    let mut output = Vec::new();
    let mut log = Vec::new();
    let mut transport = StdioTransport::new(input, &mut output, &mut log);
    transport.serve(fresh_server()).unwrap();
    assert!(output.is_empty());
}

#[test]
fn eof_ends_the_loop_cleanly() {
    let input = Cursor::new(Vec::new());
    let mut output = Vec::new();
    let mut log = Vec::new();
    let mut transport = StdioTransport::new(input, &mut output, &mut log);
    assert!(transport.serve(fresh_server()).is_ok());
    assert!(output.is_empty());
}
//#endregion 🔖️Stdio

//#region 🔖️OriginAndBearerUnit
#[test]
fn loopback_and_null_origins_are_always_allowed() {
    assert!(is_loopback_or_null("null"));
    assert!(is_loopback_or_null("http://127.0.0.1:6300"));
    assert!(is_loopback_or_null("http://localhost:6300"));
    assert!(is_loopback_or_null("http://[::1]:6300"));
    assert!(!is_loopback_or_null("https://evil.example"));
}

#[test]
fn origin_allowed_falls_back_to_the_explicit_allowlist() {
    assert!(origin_allowed(None, &[]));
    assert!(!origin_allowed(Some("https://evil.example"), &[]));
    assert!(origin_allowed(Some("https://trusted.example"), &["https://trusted.example".to_string()]));
}

#[test]
fn constant_time_eq_matches_regular_equality() {
    assert!(constant_time_eq(b"secret", b"secret"));
    assert!(!constant_time_eq(b"secret", b"secre1"));
    assert!(!constant_time_eq(b"secret", b"longer-secret"));
}
//#endregion 🔖️OriginAndBearerUnit

//#region 🧵️OwnedHttpAuthority
#[test]
fn connection_cap_plus_one_returns_the_exact_owner_without_mutating_fifo() {
    let mut ring = FixedOwnerRing::<u64, HTTP_CONNECTION_CAPACITY>::new();
    for owner in 0..HTTP_CONNECTION_CAPACITY as u64 {
        assert_eq!(ring.try_push(owner), Ok(()));
    }
    assert_eq!(ring.try_push(HTTP_CONNECTION_CAPACITY as u64), Err(HTTP_CONNECTION_CAPACITY as u64));
    for owner in 0..HTTP_CONNECTION_CAPACITY as u64 {
        assert_eq!(ring.pop_front(), Some(owner));
    }
}

#[test]
fn request_and_response_byte_cap_plus_one_return_the_exact_owner() {
    let mut request = FixedByteCredits::new(HTTP_REQUEST_BYTES);
    assert_eq!(request.try_acquire(HTTP_REQUEST_BYTES, "request"), Ok("request"));
    assert_eq!(request.try_acquire(1, "request-plus-one"), Err("request-plus-one"));
    request.release(HTTP_REQUEST_BYTES);
    assert_eq!(request.try_acquire(1, "request-rearmed"), Ok("request-rearmed"));

    let mut response = FixedByteCredits::new(HTTP_RESPONSE_BYTES);
    assert_eq!(response.try_acquire(HTTP_RESPONSE_BYTES, "response"), Ok("response"));
    assert_eq!(response.try_acquire(1, "response-plus-one"), Err("response-plus-one"));
}

#[test]
fn stale_readiness_and_retry_generations_cannot_rearm_an_aba_run() {
    assert!(generation_is_current(7, 7));
    assert!(!generation_is_current(8, 7));
    assert!(!generation_is_current(7, 8));
}

#[test]
fn websocket_handshake_matches_the_rfc6455_accept_vector() {
    assert_eq!(websocket_key_nonce("dGhlIHNhbXBsZSBub25jZQ==").unwrap(), *b"the sample nonce");
    assert_eq!(websocket_accept("dGhlIHNhbXBsZSBub25jZQ=="), "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=");
}

#[test]
fn websocket_key_rejects_duplicate_invalid_alphabet_padding_whitespace_width_and_noncanonical_bits() {
    for invalid in ["", "dGhlIHNhbXBsZSBub25jZQ=", "dGhlIHNhbXBsZSBub25jZQ=A", "dGhlIHNhbXBsZSBub25jZ!==", "dGhlIHNhbXBsZ SBub25jZQ==", "MTIzNDU2Nzg5MDEyMzQ1", "MTIzNDU2Nzg5MDEyMzQ1Njc=", "dGhlIHNhbXBsZSBub25jZR=="] {
        assert!(websocket_key_nonce(invalid).is_none(), "invalid key accepted: {invalid}");
    }
    let head = owned_bridge_head(&["dGhlIHNhbXBsZSBub25jZQ==", "dGhlIHNhbXBsZSBub25jZQ=="]);
    let admission = HttpAdmission::Fixture(Arc::from(b"bridge-token".as_slice()));
    assert!(matches!(dispatch_owned_bridge_handshake(&admission, &head), Err(HttpTerminalReason::Malformed)));
}

#[test]
fn masked_websocket_frame_waits_for_partial_payload_then_yields_one_exact_message() {
    let frame = masked_client_frame(0x2, b"hello", true);
    assert!(decode_client_websocket_frame(&frame[..frame.len() - 1]).unwrap().is_none());
    let decoded = decode_client_websocket_frame(&frame).unwrap().unwrap();
    assert_eq!(decoded.opcode, 0x2);
    let payload: Vec<_> = (0..decoded.payload_len).map(|index| decoded.payload_byte(&frame, index).unwrap()).collect();
    assert_eq!(payload, b"hello");
    assert_eq!(decoded.consumed, frame.len());
}

#[test]
fn websocket_rejects_unmasked_fragmented_and_oversize_frames_before_consumption() {
    let unmasked = encode_server_websocket_frame(0x2, b"hello").unwrap();
    assert!(matches!(decode_client_websocket_frame(&unmasked), Err(HttpTerminalReason::Unsupported)));
    let fragmented = masked_client_frame(0x2, b"hello", false);
    assert!(matches!(decode_client_websocket_frame(&fragmented), Err(HttpTerminalReason::Unsupported)));
    let mut oversize = vec![0x82, 0xff];
    oversize.extend_from_slice(&((WEBSOCKET_FRAME_BYTES as u64) + 1).to_be_bytes());
    assert!(matches!(decode_client_websocket_frame(&oversize), Err(HttpTerminalReason::Capacity)));
}

#[test]
fn websocket_control_and_close_frames_stay_one_bounded_message() {
    let ping = masked_client_frame(0x9, b"p", true);
    assert_eq!(decode_client_websocket_frame(&ping).unwrap().unwrap().opcode, 0x9);
    let close = masked_client_frame(0x8, &[], true);
    assert_eq!(decode_client_websocket_frame(&close).unwrap().unwrap().opcode, 0x8);
    assert!(encode_server_websocket_frame(0x9, &[0; 126]).is_none());
}

#[test]
fn one_terminal_close_grant_drains_exactly_one_fifo_owner() {
    let mut terminal = FixedOwnerRing::<u64, 3>::new();
    terminal.try_push(11).unwrap();
    terminal.try_push(12).unwrap();
    terminal.try_push(13).unwrap();
    assert_eq!(terminal.pop_front(), Some(11));
    assert_eq!(terminal.len(), 2);
    assert_eq!(terminal.pop_front(), Some(12));
}

#[test]
fn partial_http_read_and_parser_turn_advance_one_page_or_token_per_grant() {
    let (mut state, mut peer) = state_with_connection();
    peer.write_all(b"POST /mcp HTTP/1.1\r\nAuthorization: Bearer test-token\r\n").unwrap();
    assert!(matches!(state.drive_one(1), HttpTurn::MoreWork));
    let connection = state.connections[0].as_ref().unwrap();
    assert!(connection.ingress.len() <= HTTP_IO_PAGE_BYTES);
    assert_eq!(connection.parser.cursor, 0);
    assert!(matches!(state.drive_one(2), HttpTurn::MoreWork));
    let connection = state.connections[0].as_ref().unwrap();
    assert_eq!(connection.parser.method.as_deref(), Some("POST"));
    assert_eq!(connection.parser.header_count, 0, "one parse grant consumes only the request-line token");
}

#[test]
fn request_line_and_header_delimiter_search_faults_at_cap_without_scanning_late_crlf() {
    let mut request_line = vec![b'a'; HTTP_REQUEST_LINE_BYTES + 2];
    request_line[HTTP_REQUEST_LINE_BYTES] = b'\r';
    request_line[HTTP_REQUEST_LINE_BYTES + 1] = b'\n';
    assert_eq!(find_crlf_bounded(&request_line, 0, HTTP_REQUEST_LINE_BYTES), None);
    let (mut state, _peer) = state_with_connection();
    state.connections[0].as_mut().unwrap().ingress = request_line;
    assert!(matches!(state.drive_one(1), HttpTurn::MoreWork));
    assert_eq!(state.terminal.len(), 1);

    let mut header = vec![b'a'; HTTP_HEADER_BYTES + 2];
    header[HTTP_HEADER_BYTES] = b'\r';
    header[HTTP_HEADER_BYTES + 1] = b'\n';
    assert_eq!(find_crlf_bounded(&header, 20, HTTP_HEADER_BYTES), None);
}

#[test]
fn malformed_and_unsupported_open_bridge_binary_frames_retain_exact_raw_ingress() {
    for (payload, reason) in [
        (vec![99], HttpTerminalReason::Malformed),
        (
            crate::bridge::ShellToGateway::Hello { bridge_version: crate::bridge::BRIDGE_VERSION, shell_kind: crate::bridge::ShellKind::React, shell_session_id: "s".into(), principal_actor: "p".into(), flags: crate::bridge::BridgeFlags::NONE }
                .encode(),
            HttpTerminalReason::Unsupported,
        ),
    ] {
        let (mut state, _peer) = state_with_connection();
        let (id, outbox) = state.bridge.register();
        let raw = masked_client_frame(0x2, &payload, true);
        state.request_credits.try_acquire(raw.len(), ()).unwrap();
        let connection = state.connections[0].as_mut().unwrap();
        connection.bridge = BridgeSession { id: Some(id), outbox: Some(outbox), opening: false, inbound: None };
        connection.phase = HttpConnectionPhase::ParseWebSocket;
        connection.ingress = raw.clone();
        for grant in 1..128 {
            assert!(matches!(state.drive_one(grant), HttpTurn::MoreWork));
            if state.terminal.len() != 0 {
                break;
            }
            assert_eq!(state.connections[0].as_ref().unwrap().ingress, raw, "incremental validation must retain the exact raw masked owner");
        }
        let owner = state.terminal.pop_front().expect("protocol fault must retain terminal owner");
        assert_eq!(owner.reason(), reason);
        assert_eq!(owner.ingress(), raw);
        owner.close();
    }
}

#[test]
fn incremental_bridge_decode_cancellation_and_stale_generation_retain_exact_raw_owner() {
    let (mut state, _peer) = state_with_connection();
    let (id, outbox) = state.bridge.register();
    let payload = crate::bridge::ShellToGateway::ShellState { revision: 1, state: vec![4; HTTP_IO_PAGE_BYTES + 1] }.encode();
    let raw = masked_client_frame(0x2, &payload, true);
    state.request_credits.try_acquire(raw.len(), ()).unwrap();
    let connection = state.connections[0].as_mut().unwrap();
    connection.bridge = BridgeSession { id: Some(id), outbox: Some(outbox), opening: false, inbound: None };
    connection.phase = HttpConnectionPhase::ParseWebSocket;
    connection.ingress = raw.clone();
    assert!(matches!(state.drive_one(1), HttpTurn::MoreWork));
    assert!(matches!(state.drive_one(2), HttpTurn::MoreWork));
    let connection = state.connections[0].as_mut().unwrap();
    assert!(connection.bridge.inbound.is_some());
    connection.key.generation = connection.key.generation.wrapping_add(1);
    assert!(matches!(state.drive_one(3), HttpTurn::MoreWork));
    let owner = state.terminal.pop_front().unwrap();
    assert_eq!(owner.reason(), HttpTerminalReason::Interrupted);
    assert_eq!(owner.ingress(), raw);
    owner.close();

    let (mut cancelled, _peer) = state_with_connection();
    let (id, outbox) = cancelled.bridge.register();
    cancelled.request_credits.try_acquire(raw.len(), ()).unwrap();
    let connection = cancelled.connections[0].as_mut().unwrap();
    connection.bridge = BridgeSession { id: Some(id), outbox: Some(outbox), opening: false, inbound: None };
    connection.phase = HttpConnectionPhase::ParseWebSocket;
    connection.ingress = raw.clone();
    assert!(matches!(cancelled.drive_one(1), HttpTurn::MoreWork));
    assert!(matches!(cancelled.drive_one(2), HttpTurn::MoreWork));
    cancelled.mode = HttpTransportMode::Closing;
    assert!(matches!(cancelled.drive_one(3), HttpTurn::MoreWork));
    let owner = cancelled.terminal.pop_front().unwrap();
    assert_eq!(owner.reason(), HttpTerminalReason::Cancelled);
    assert_eq!(owner.ingress(), raw);
    owner.close();
}

#[test]
fn slowloris_deadline_terminalizes_once_and_parks_for_public_retrieval() {
    let (mut state, _peer) = state_with_connection();
    state.connections[0].as_mut().unwrap().header_deadline_ms = 5;
    assert!(matches!(state.drive_one(5), HttpTurn::MoreWork));
    assert_eq!(state.terminal.len(), 1);
    assert!(state.connections[0].is_none());
    let _ = state.drive_one(6);
    assert_eq!(state.terminal.len(), 1);
    state.terminal.pop_front().unwrap().close();
}

#[test]
fn cancellation_and_shutdown_drain_one_connection_authority_per_grant() {
    let (mut state, _peer) = state_with_connection();
    state.mode = HttpTransportMode::Closing;
    state.terminal_policy = HttpTerminalPolicy::Close;
    state.run_generation = 2;
    assert!(matches!(state.drive_one(1), HttpTurn::MoreWork));
    assert_eq!(state.terminal.len(), 1);
    assert!(matches!(state.drive_one(2), HttpTurn::MoreWork));
    assert_eq!(state.terminal.len(), 0);
    assert!(matches!(state.drive_one(3), HttpTurn::Complete(Ok(()))));
}

#[test]
fn terminal_public_fifo_preserves_generation_aba_and_process_close_is_one_owner_per_grant() {
    let (mut state, _peer) = state_with_connection();
    let first = state.connections[0].take().unwrap();
    let mut second = replacement_connection(&state, 0, 2);
    second.ingress.extend_from_slice(b"second");
    state.terminalize(first, HttpTerminalReason::Malformed);
    state.terminalize(second, HttpTerminalReason::Unsupported);
    assert_eq!(state.terminal.len(), 2);
    let first = state.terminal.pop_front().unwrap();
    let second = state.terminal.pop_front().unwrap();
    assert_eq!(first.key(), HttpConnectionKey { slot: 0, generation: 1 });
    assert_eq!(second.key(), HttpConnectionKey { slot: 0, generation: 2 });
    assert_eq!(second.ingress(), b"second");
    first.close();
    second.close();

    let first = replacement_connection(&state, 0, 3);
    let second = replacement_connection(&state, 1, 4);
    state.terminalize(first, HttpTerminalReason::Malformed);
    state.terminalize(second, HttpTerminalReason::Malformed);
    state.terminal_policy = HttpTerminalPolicy::Close;
    assert!(matches!(state.drive_one(1), HttpTurn::MoreWork));
    assert_eq!(state.terminal.len(), 1);
    assert!(matches!(state.drive_one(2), HttpTurn::MoreWork));
    assert_eq!(state.terminal.len(), 0);
}

#[test]
fn partial_response_write_never_exceeds_one_io_page_per_grant() {
    let (mut state, mut peer) = state_with_connection();
    peer.set_nonblocking(true).unwrap();
    let connection = state.connections[0].as_mut().unwrap();
    connection.egress = vec![7; HTTP_IO_PAGE_BYTES * 2];
    connection.phase = HttpConnectionPhase::Write(HttpAfterWrite::Close);
    state.response_credits.try_acquire(connection.egress.len(), ()).unwrap();
    assert!(matches!(state.drive_one(1), HttpTurn::MoreWork));
    let written = state.connections[0].as_ref().map_or(HTTP_IO_PAGE_BYTES, |connection| connection.written);
    assert!(written <= HTTP_IO_PAGE_BYTES);
    let mut page = vec![0; HTTP_IO_PAGE_BYTES];
    let _ = peer.read(&mut page);
}

fn state_with_connection() -> (HttpTransportState, TcpStream) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let peer = TcpStream::connect(address).unwrap();
    let (stream, remote) = listener.accept().unwrap();
    stream.set_nonblocking(true).unwrap();
    listener.set_nonblocking(true).unwrap();
    let options = HttpTransportOptions::fixture("test-token").bind_addr(address);
    let events = Arc::new(Mutex::new(EventLog::default()));
    let bridge = crate::bridge::BridgeHandle::new();
    let mut state = HttpTransportState::new(listener, fresh_server(), &options, events, bridge);
    state.io_cursor = 1;
    state.connections[0] = Some(HttpConnection {
        key: HttpConnectionKey { slot: 0, generation: 1 },
        peer: remote,
        stream,
        ingress: Vec::with_capacity(HTTP_REQUEST_BYTES),
        egress: Vec::with_capacity(HTTP_RESPONSE_BYTES),
        bridge_egress: None,
        written: 0,
        parser: ParsedHttpHead::new(),
        phase: HttpConnectionPhase::ReadHttp,
        bridge: BridgeSession { id: None, outbox: None, opening: false, inbound: None },
        last_progress_ms: 0,
        header_deadline_ms: HTTP_SLOWLORIS_MS,
    });
    (state, peer)
}

fn replacement_connection(state: &HttpTransportState, slot: u16, generation: u64) -> HttpConnection {
    let address = state.listener.as_ref().unwrap().local_addr().unwrap();
    let _peer = TcpStream::connect(address).unwrap();
    let (stream, remote) = state.listener.as_ref().unwrap().accept().unwrap();
    stream.set_nonblocking(true).unwrap();
    HttpConnection {
        key: HttpConnectionKey { slot, generation },
        peer: remote,
        stream,
        ingress: Vec::with_capacity(HTTP_REQUEST_BYTES),
        egress: Vec::with_capacity(HTTP_RESPONSE_BYTES),
        bridge_egress: None,
        written: 0,
        parser: ParsedHttpHead::new(),
        phase: HttpConnectionPhase::ReadHttp,
        bridge: BridgeSession { id: None, outbox: None, opening: false, inbound: None },
        last_progress_ms: 0,
        header_deadline_ms: HTTP_SLOWLORIS_MS,
    }
}

fn owned_bridge_head(keys: &[&str]) -> ParsedHttpHead {
    let mut head = ParsedHttpHead::new();
    head.method = Some("GET".into());
    head.path = Some("/bridge".into());
    for (name, value) in
        [("upgrade", "websocket"), ("connection", "Upgrade"), ("sec-websocket-version", "13"), ("sec-websocket-protocol", "semio.mcp.bridge.v1, bridge-token")].into_iter().chain(keys.iter().copied().map(|value| ("sec-websocket-key", value)))
    {
        head.headers[head.header_count] = Some(OwnedHeader { name: name.into(), value: value.into() });
        head.header_count += 1;
    }
    head
}

pub(super) fn masked_client_frame(opcode: u8, payload: &[u8], fin: bool) -> Vec<u8> {
    assert!(payload.len() <= 125);
    let mask = [1u8, 2, 3, 4];
    let mut frame = vec![(if fin { 0x80 } else { 0 }) | opcode, 0x80 | payload.len() as u8];
    frame.extend_from_slice(&mask);
    frame.extend(payload.iter().enumerate().map(|(index, byte)| byte ^ mask[index % 4]));
    frame
}
//#endregion 🧵️OwnedHttpAuthority
