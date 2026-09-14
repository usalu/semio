//! 🦀️ Rust side of the HTTP route case. Starts the coordinator on an ephemeral port, replays the
//! committed request fixture over a hand-rolled HTTP/1.1 client and projects status and body.

use semio_framework_repo_coordinator as coordinator;
use semio_framework_repo_coordinator::serde_json::{self, Map, Value as SerdeJson};
use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;

//#region 🔖️Vectors
struct RouteFixture {
    requests: Vec<SerdeJson>,
    volatile: Vec<String>,
}

fn route_fixture(ctx: &Context) -> Result<RouteFixture, String> {
    let bytes = ctx.fixture_bytes("local://🌐️requests.json")?;
    let parsed: SerdeJson = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    Ok(RouteFixture {
        requests: parsed.get("requests").and_then(SerdeJson::as_array).ok_or("request fixture has no requests array")?.clone(),
        volatile: parsed
            .get("volatileKeys")
            .and_then(SerdeJson::as_array)
            .map(|keys| keys.iter().filter_map(SerdeJson::as_str).map(str::to_string).collect())
            .unwrap_or_default(),
    })
}

/// 🚀️ Brings up one coordinator with an empty log in a directory of this scenario's own.
fn start_coordinator(ctx: &Context, name: &str) -> Result<coordinator::Service, String> {
    let dir = ctx.work_dir.join(name);
    if dir.exists() {
        fs::remove_dir_all(&dir).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    coordinator::start(coordinator::Config {
        address: "127.0.0.1:0".to_string(),
        database_path: dir.join("coordinator.events").display().to_string(),
        repo_root: dir.display().to_string(),
        ..coordinator::Config::default()
    })
    .map_err(|error| error.to_string())
}

fn issue(address: &str, request: &SerdeJson) -> Result<(u16, String), String> {
    let method = request.get("method").and_then(SerdeJson::as_str).unwrap_or("GET");
    let path = request.get("path").and_then(SerdeJson::as_str).unwrap_or("/");
    let body = request.get("body").map(coordinator::to_go_json).unwrap_or_default();
    let mut head = format!("{method} {path} HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n");
    if !body.is_empty() {
        head.push_str(&format!("Content-Type: application/json\r\nContent-Length: {}\r\n", body.len()));
    }
    if let Some(headers) = request.get("header").and_then(SerdeJson::as_object) {
        for (name, value) in headers {
            head.push_str(&format!("{name}: {}\r\n", value.as_str().unwrap_or_default()));
        }
    }
    head.push_str("\r\n");
    let mut stream = TcpStream::connect(address).map_err(|error| error.to_string())?;
    stream.write_all(head.as_bytes()).map_err(|error| error.to_string())?;
    if !body.is_empty() {
        stream.write_all(body.as_bytes()).map_err(|error| error.to_string())?;
    }
    stream.flush().map_err(|error| error.to_string())?;
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).map_err(|error| error.to_string())?;
    let text = String::from_utf8_lossy(&raw).to_string();
    let (head, payload) = text.split_once("\r\n\r\n").ok_or("response has no header terminator")?;
    let status = head.split(' ').nth(1).and_then(|code| code.parse::<u16>().ok()).ok_or("response has no status code")?;
    Ok((status, payload.to_string()))
}

fn normalize_body(raw: &str, volatile: &[String]) -> Json {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Json::String(String::new());
    }
    match serde_json::from_str::<SerdeJson>(trimmed) {
        Err(_) => Json::String(trimmed.to_string()),
        Ok(value) => json_of(&scrub(&value, volatile)),
    }
}

fn scrub(value: &SerdeJson, volatile: &[String]) -> SerdeJson {
    match value {
        SerdeJson::Object(entries) => {
            let mut out = Map::new();
            for (key, item) in entries {
                if !item.is_null() && volatile.iter().any(|name| name == key) {
                    out.insert(key.clone(), SerdeJson::String("<volatile>".to_string()));
                    continue;
                }
                out.insert(key.clone(), scrub(item, volatile));
            }
            SerdeJson::Object(out)
        }
        SerdeJson::Array(items) => SerdeJson::Array(items.iter().map(|item| scrub(item, volatile)).collect()),
        other => other.clone(),
    }
}

fn json_of(value: &SerdeJson) -> Json {
    match value {
        SerdeJson::Null => Json::Null,
        SerdeJson::Bool(flag) => Json::Bool(*flag),
        SerdeJson::Number(number) => Json::Number(number.as_f64().unwrap_or_default()),
        SerdeJson::String(text) => Json::String(text.clone()),
        SerdeJson::Array(items) => Json::Array(items.iter().map(json_of).collect()),
        SerdeJson::Object(entries) => Json::Object(entries.iter().map(|(key, item)| (key.clone(), json_of(item))).collect()),
    }
}

fn replay(ctx: &Context, name: &str, refusals_only: bool) -> Result<Vec<Json>, String> {
    let fixture = route_fixture(ctx)?;
    let service = start_coordinator(ctx, name)?;
    let address = service.address.clone();
    let mut answers = Vec::new();
    for request in &fixture.requests {
        let (status, body) = issue(&address, request)?;
        if refusals_only && status < 400 {
            continue;
        }
        answers.push(Json::Object(vec![
            ("name".to_string(), Json::String(request.get("name").and_then(SerdeJson::as_str).unwrap_or_default().to_string())),
            ("status".to_string(), Json::Number(f64::from(status))),
            ("body".to_string(), normalize_body(&body, &fixture.volatile)),
        ]));
    }
    service.stop();
    Ok(answers)
}
//#endregion 🔖️Vectors

//#region 🔖️Scenarios
fn every_route_answers_the_same_status_and_body(ctx: &Context) -> Result<Outcome, String> {
    Ok(Outcome::projection(Json::Object(vec![("answers".to_string(), Json::Array(replay(ctx, "route-contract", false)?))])))
}

fn a_method_mismatch_and_a_missing_field_are_refused(ctx: &Context) -> Result<Outcome, String> {
    Ok(Outcome::projection(Json::Object(vec![("refusals".to_string(), Json::Array(replay(ctx, "route-refusals", true)?))])))
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("every-route-answers-the-same-status-and-body", every_route_answers_the_same_status_and_body)
        .subject("a-method-mismatch-and-a-missing-field-are-refused", a_method_mismatch_and_a_missing_field_are_refused)
}
//#endregion 🔖️Registration
