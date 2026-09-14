//! 🦀️ Rust subject for the advertised capability surface. Lists tools, resources and prompts through
//! the owned session so the projection is exactly what a client sees on the wire.

use semio_framework_repo_mcp as mcp;
use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Support

/// 📋️ Runs one listing method against a ready session of the given profile and returns the result member.
fn list(profile: mcp::Profile, method: &str, field: &str) -> Result<Json, String> {
    let server = std::sync::Arc::new(mcp::repository_server(mcp::RecordingRepository::new(), profile, mcp::Limits::default()).map_err(|error| error.to_string())?);
    let session = server.connect("listing", None).map_err(|error| error.to_string())?;
    let initialize = format!(r#"{{"jsonrpc":"2.0","id":0,"method":"initialize","params":{{"protocolVersion":"{}","capabilities":{{}},"clientInfo":{{"name":"listing","version":"1"}}}}}}"#, mcp::PROTOCOL_VERSION);
    session.dispatch(initialize.as_bytes()).map_err(|error| error.to_string())?;
    session.dispatch(br#"{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}"#).map_err(|error| error.to_string())?;
    let request = format!(r#"{{"jsonrpc":"2.0","id":1,"method":"{method}","params":{{}}}}"#);
    let response = session.dispatch(request.as_bytes()).map_err(|error| error.to_string())?.ok_or("no response")?;
    let envelope = parse_json(&response)?;
    Ok(Json::Array(envelope.get("result").map(|result| result.array(field)).unwrap_or_default()))
}

fn entries(items: &Json, key: &str) -> Json {
    match items {
        Json::Array(values) => Json::Array(
            values
                .iter()
                .map(|item| Json::Object(vec![("id".to_string(), Json::String(item.str(key))), ("description".to_string(), Json::String(item.str("description")))]))
                .collect(),
        ),
        _ => Json::Array(Vec::new()),
    }
}

fn profile_of(slug: &str) -> Result<mcp::Profile, String> {
    mcp::Profile::parse(slug).map_err(|error| error.to_string())
}

//#endregion 🔖️Support

//#region 🔖️Scenarios

fn generic_profile_surface(_ctx: &Context) -> Result<Outcome, String> {
    Ok(Outcome::projection(Json::Object(vec![
        ("tools".to_string(), entries(&list(mcp::Profile::Generic, "tools/list", "tools")?, "name")),
        ("resources".to_string(), entries(&list(mcp::Profile::Generic, "resources/list", "resources")?, "uri")),
        ("prompts".to_string(), entries(&list(mcp::Profile::Generic, "prompts/list", "prompts")?, "name")),
    ])))
}

fn ide_profile_surface(ctx: &Context) -> Result<Outcome, String> {
    let raw = ctx.fixture_bytes("shared://📋️surface.json")?;
    let surface = parse_json(&String::from_utf8_lossy(&raw))?;
    let ticket_tools: Vec<String> = surface.array("ticketTools").iter().map(|value| if let Json::String(name) = value { name.clone() } else { String::new() }).collect();
    let mut rows = Vec::new();
    for profile in surface.array("profiles") {
        let slug = profile.str("slug");
        let tools = list(profile_of(&slug)?, "tools/list", "tools")?;
        let mut arguments = Vec::new();
        if let Json::Array(items) = &tools {
            for item in items {
                let name = item.str("name");
                if !ticket_tools.contains(&name) {
                    continue;
                }
                let properties = item.get("inputSchema").and_then(|schema| schema.get("properties")).cloned().unwrap_or(Json::Null);
                let extra = if properties.get("plan_id").is_some() {
                    "plan_id"
                } else if properties.get("spec_id").is_some() {
                    "spec_id"
                } else {
                    ""
                };
                arguments.push(Json::Object(vec![("tool".to_string(), Json::String(name)), ("extraArgument".to_string(), Json::String(extra.to_string()))]));
            }
        }
        rows.push(Json::Object(vec![("slug".to_string(), Json::String(slug)), ("ticketArguments".to_string(), Json::Array(arguments))]));
    }
    Ok(Outcome::projection(Json::Object(vec![("profiles".to_string(), Json::Array(rows))])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust").subject("generic-profile-surface", generic_profile_surface).subject("ide-profile-surface", ide_profile_surface)
}

//#endregion 🔖️Registration
