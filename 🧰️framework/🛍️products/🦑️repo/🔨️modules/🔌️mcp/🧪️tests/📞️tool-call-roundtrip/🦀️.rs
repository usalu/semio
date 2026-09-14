//! 🦀️ Rust subject for the golden call vectors. Dispatches each `2️⃣g2-contract.json` request against a
//! session of the contract server and projects the parsed result, never the raw bytes, so a reference
//! implementation with different whitespace and escaping can project the same thing.

use semio_framework_repo_mcp as mcp;
use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Support

/// 📞️ Dispatches one vector against a fresh contract server and returns the parsed response envelope.
fn dispatch(request: &str, ready: bool) -> Result<Json, String> {
    let server = std::sync::Arc::new(mcp::contract_server(mcp::Limits::default()).map_err(|error| error.to_string())?);
    let session = server.connect("fixture", None).map_err(|error| error.to_string())?;
    if ready {
        let initialize = format!(r#"{{"jsonrpc":"2.0","id":"init","method":"initialize","params":{{"protocolVersion":"{}","capabilities":{{}},"clientInfo":{{"name":"test","version":"1"}}}}}}"#, mcp::PROTOCOL_VERSION);
        session.dispatch(initialize.as_bytes()).map_err(|error| error.to_string())?;
        session.dispatch(br#"{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}"#).map_err(|error| error.to_string())?;
    }
    let response = session.dispatch(request.as_bytes()).map_err(|error| error.to_string())?.ok_or("no response")?;
    parse_json(&response)
}

fn vectors(ctx: &Context) -> Result<Vec<Json>, String> {
    let raw = ctx.fixture_bytes("shared://2️⃣g2-contract.json")?;
    Ok(parse_json(&String::from_utf8_lossy(&raw))?.array("vectors"))
}

//#endregion 🔖️Support

//#region 🔖️Scenarios

fn tool_resource_prompt_roundtrip(ctx: &Context) -> Result<Outcome, String> {
    let mut rows = Vec::new();
    for vector in vectors(ctx)? {
        let name = vector.str("name");
        if !matches!(name.as_str(), "tool" | "resource" | "prompt") {
            continue;
        }
        let envelope = dispatch(&vector.str("request"), matches!(vector.get("ready"), Some(Json::Bool(true))))?;
        rows.push(Json::Object(vec![("name".to_string(), Json::String(name)), ("result".to_string(), envelope.get("result").cloned().unwrap_or(Json::Null))]));
    }
    Ok(Outcome::projection(Json::Object(vec![("vectors".to_string(), Json::Array(rows))])))
}

fn protocol_error_vectors(ctx: &Context) -> Result<Outcome, String> {
    let mut rows = Vec::new();
    for vector in vectors(ctx)? {
        let name = vector.str("name");
        if name != "unknown-method" {
            continue;
        }
        let envelope = dispatch(&vector.str("request"), matches!(vector.get("ready"), Some(Json::Bool(true))))?;
        let code = match envelope.get("error").and_then(|error| error.get("code")) {
            Some(Json::Number(value)) => *value,
            _ => 0.0,
        };
        rows.push(Json::Object(vec![("name".to_string(), Json::String(name)), ("code".to_string(), Json::Number(code))]));
    }
    Ok(Outcome::projection(Json::Object(vec![("vectors".to_string(), Json::Array(rows))])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust").subject("tool-resource-prompt-roundtrip", tool_resource_prompt_roundtrip).subject("protocol-error-vectors", protocol_error_vectors)
}

//#endregion 🔖️Registration
