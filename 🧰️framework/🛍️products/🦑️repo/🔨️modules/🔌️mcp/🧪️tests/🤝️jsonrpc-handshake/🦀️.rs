//! 🦀️ Rust subject for the MCP initialize handshake. Drives the owned session in process and projects
//! only what a client can observe, so the SDK oracle can project the very same shape.

use semio_framework_repo_mcp as mcp;
use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Support

/// 🤝️ Initializes a generic-profile session with the fixture's client members and parses the reply.
fn handshake(ctx: &Context) -> Result<(Json, String), String> {
    let raw = ctx.fixture_bytes("shared://🤝️initialize-lenient.json")?;
    let fixture = parse_json(&String::from_utf8_lossy(&raw))?;
    let requested = fixture.str("requestedProtocolVersion");
    let client_info = fixture.get("clientInfo").cloned().unwrap_or(Json::Null).to_string();
    let capabilities = fixture.get("capabilities").cloned().unwrap_or(Json::Null).to_string();
    let server = std::sync::Arc::new(mcp::repository_server(mcp::RecordingRepository::new(), mcp::Profile::Generic, mcp::Limits::default()).map_err(|error| error.to_string())?);
    let session = server.connect("conformance", None).map_err(|error| error.to_string())?;
    let request = format!(r#"{{"jsonrpc":"2.0","id":1,"method":"initialize","params":{{"protocolVersion":"{requested}","capabilities":{capabilities},"clientInfo":{client_info}}}}}"#);
    let response = session.dispatch(request.as_bytes()).map_err(|error| error.to_string())?.ok_or("no response")?;
    Ok((parse_json(&response)?, requested))
}

/// 🔁️ Delivers the initialized notification while the session is still `Connected` — the exact window
/// a pipelining client opens — then initializes and pings, and reports whether the ping was answered.
fn pipelined_handshake(ctx: &Context) -> Result<Json, String> {
    let raw = ctx.fixture_bytes("shared://🤝️initialize-lenient.json")?;
    let fixture = parse_json(&String::from_utf8_lossy(&raw))?;
    let requested = fixture.str("requestedProtocolVersion");
    let client_info = fixture.get("clientInfo").cloned().unwrap_or(Json::Null).to_string();
    let capabilities = fixture.get("capabilities").cloned().unwrap_or(Json::Null).to_string();
    let server = std::sync::Arc::new(mcp::repository_server(mcp::RecordingRepository::new(), mcp::Profile::Generic, mcp::Limits::default()).map_err(|error| error.to_string())?);
    let session = server.connect("conformance", None).map_err(|error| error.to_string())?;
    session.dispatch(br#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#).map_err(|error| error.to_string())?;
    let initialize = format!(r#"{{"jsonrpc":"2.0","id":1,"method":"initialize","params":{{"protocolVersion":"{requested}","capabilities":{capabilities},"clientInfo":{client_info}}}}}"#);
    let initialized = parse_json(&session.dispatch(initialize.as_bytes()).map_err(|error| error.to_string())?.ok_or("no response")?)?;
    let ping = parse_json(&session.dispatch(br#"{"jsonrpc":"2.0","id":2,"method":"ping"}"#).map_err(|error| error.to_string())?.ok_or("no response")?)?;
    Ok(Json::Object(vec![
        ("initializeAccepted".to_string(), Json::Bool(initialized.get("error").is_none())),
        ("pingAnswered".to_string(), Json::Bool(ping.get("error").is_none())),
    ]))
}

/// 🧺️ A writer the served peer can keep while the adapter still reads what was written to it.
#[derive(Clone)]
struct Capture(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);

impl std::io::Write for Capture {
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap_or_else(|poison| poison.into_inner()).extend_from_slice(data);
        Ok(data.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// 🚰️ Serves one burst whose reader ends the moment the last request was delivered — the shape a
/// client that writes everything and closes its input produces — and returns every reply written.
fn burst_then_eof(ctx: &Context) -> Result<Vec<Json>, String> {
    let raw = ctx.fixture_bytes("shared://🤝️initialize-lenient.json")?;
    let fixture = parse_json(&String::from_utf8_lossy(&raw))?;
    let requested = fixture.str("requestedProtocolVersion");
    let client_info = fixture.get("clientInfo").cloned().unwrap_or(Json::Null).to_string();
    let capabilities = fixture.get("capabilities").cloned().unwrap_or(Json::Null).to_string();
    let burst = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{{\"protocolVersion\":\"{requested}\",\"capabilities\":{capabilities},\"clientInfo\":{client_info}}}}}\n{{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\",\"params\":{{}}}}\n{{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/list\",\"params\":{{}}}}\n"
    );
    let server = std::sync::Arc::new(mcp::repository_server(mcp::RecordingRepository::new(), mcp::Profile::Generic, mcp::Limits::default()).map_err(|error| error.to_string())?);
    let capture = Capture(std::sync::Arc::new(std::sync::Mutex::new(Vec::new())));
    let sink = capture.clone();
    match mcp::serve(&server, "burst", std::io::Cursor::new(burst.into_bytes()), sink) {
        Ok(()) | Err(mcp::Error::PeerDropped) => {}
        Err(error) => return Err(error.to_string()),
    }
    let written = capture.0.lock().unwrap_or_else(|poison| poison.into_inner()).clone();
    String::from_utf8_lossy(&written).lines().filter(|line| !line.trim().is_empty()).map(parse_json).collect()
}

/// 🧾️ Reduces the replies to the ids answered, whether the request that followed the initialized
/// notification carries a result, and how many replies refused an uninitialized session.
fn pipelined_burst_projection(replies: &[Json]) -> Json {
    let mut answered: Vec<String> = replies.iter().map(|reply| reply.get("id").map(Json::to_string).unwrap_or_default()).collect();
    answered.sort();
    let refused = replies
        .iter()
        .filter(|reply| matches!(reply.get("error").and_then(|error| error.get("code")), Some(Json::Number(code)) if *code == -32002.0))
        .count();
    let served = replies.iter().any(|reply| reply.get("id").map(Json::to_string).unwrap_or_default() == "2" && reply.get("result").is_some());
    Json::Object(vec![
        ("answered".to_string(), Json::Array(answered.into_iter().map(Json::String).collect())),
        ("followingRequestServed".to_string(), Json::Bool(served)),
        ("notInitializedRefusals".to_string(), Json::Number(refused as f64)),
    ])
}

/// 🧾️ Reduces the replies to the request ids answered and the closed-session refusals among them.
fn burst_projection(replies: &[Json]) -> Json {
    let mut answered: Vec<String> = replies.iter().map(|reply| reply.get("id").map(Json::to_string).unwrap_or_default()).collect();
    answered.sort();
    let refused = replies
        .iter()
        .filter(|reply| matches!(reply.get("error").and_then(|error| error.get("code")), Some(Json::Number(code)) if *code == -32004.0))
        .count();
    Json::Object(vec![
        ("answered".to_string(), Json::Array(answered.into_iter().map(Json::String).collect())),
        ("sessionClosedRefusals".to_string(), Json::Number(refused as f64)),
    ])
}

//#endregion 🔖️Support

//#region 🔖️Scenarios

fn initialize_accepts_unknown_members(ctx: &Context) -> Result<Outcome, String> {
    let (envelope, _) = handshake(ctx)?;
    let result = envelope.get("result").cloned().unwrap_or(Json::Null);
    let mut capabilities: Vec<String> = match result.get("capabilities") {
        Some(Json::Object(members)) => members.iter().map(|(name, _)| name.clone()).collect(),
        _ => Vec::new(),
    };
    capabilities.sort();
    Ok(Outcome::projection(Json::Object(vec![
        ("accepted".to_string(), Json::Bool(envelope.get("error").is_none())),
        ("serverVersion".to_string(), Json::String(result.get("serverInfo").map(|info| info.str("version")).unwrap_or_default())),
        ("capabilities".to_string(), Json::Array(capabilities.into_iter().map(Json::String).collect())),
    ])))
}

fn initialize_and_initialized_pipelined(ctx: &Context) -> Result<Outcome, String> {
    Ok(Outcome::projection(pipelined_handshake(ctx)?))
}

fn initialize_echoes_a_supported_version(ctx: &Context) -> Result<Outcome, String> {
    let (envelope, requested) = handshake(ctx)?;
    let negotiated = envelope.get("result").map(|result| result.str("protocolVersion")).unwrap_or_default();
    Ok(Outcome::projection(Json::Object(vec![("protocolVersionEchoed".to_string(), Json::Bool(negotiated == requested))])))
}

fn pipelined_burst_serves_requests_after_initialized(ctx: &Context) -> Result<Outcome, String> {
    Ok(Outcome::projection(pipelined_burst_projection(&burst_then_eof(ctx)?)))
}

fn eof_after_burst_completes_queued_requests(ctx: &Context) -> Result<Outcome, String> {
    Ok(Outcome::projection(burst_projection(&burst_then_eof(ctx)?)))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("initialize-accepts-unknown-members", initialize_accepts_unknown_members)
        .subject("initialize-and-initialized-pipelined", initialize_and_initialized_pipelined)
        .subject("initialize-echoes-a-supported-version", initialize_echoes_a_supported_version)
        .subject("pipelined-burst-serves-requests-after-initialized", pipelined_burst_serves_requests_after_initialized)
        .subject("eof-after-burst-completes-queued-requests", eof_after_burst_completes_queued_requests)
}

//#endregion 🔖️Registration
