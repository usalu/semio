//! 🦀️ Rust side of the canonical-envelope case. Decodes the golden record, re-encodes it and
//! projects the header, the canonical payload and the checksum the store would persist.

use semio_framework_repo_coordinator as coordinator;
use semio_framework_repo_coordinator::serde_json::{self, Value as SerdeJson};
use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Vectors
fn golden_event(ctx: &Context) -> Result<(coordinator::EventEnvelope, String), String> {
    let bytes = ctx.fixture_bytes("shared://📜️g3-event-log.jsonl")?;
    let line = String::from_utf8(bytes).map_err(|error| error.to_string())?.trim_end_matches('\n').to_string();
    let event = coordinator::decode_event(&line).map_err(|error| error.to_string())?;
    Ok((event, line))
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
//#endregion 🔖️Vectors

//#region 🔖️Scenarios
fn golden_line_is_canonical(ctx: &Context) -> Result<Outcome, String> {
    let (event, line) = golden_event(ctx)?;
    let encoded = coordinator::encode_event(&event).trim_end_matches('\n').to_string();
    let schema_bytes = ctx.fixture_bytes("schema://repo.server.coordinator/G3EventLogContract")?;
    let declared: SerdeJson = serde_json::from_slice(&schema_bytes).map_err(|error| error.to_string())?;
    let declared_of = |key: &str| declared.pointer(&format!("/$defs/G3EventLogContract/properties/{key}/const")).map_or(Json::Null, json_of);
    Ok(Outcome::projection(Json::Object(vec![
        ("line".to_string(), Json::String(encoded.clone())),
        ("reproduced".to_string(), Json::Bool(encoded == line)),
        ("fields".to_string(), declared_of("fields")),
        ("formula".to_string(), declared_of("checksum")),
        ("encoding".to_string(), declared_of("encoding")),
        ("schema".to_string(), declared_of("schema")),
    ])))
}

fn checksum_is_sha256_of_the_preimage(ctx: &Context) -> Result<Outcome, String> {
    let (event, _) = golden_event(ctx)?;
    let payload = coordinator::to_go_json(&event.payload);
    Ok(Outcome::projection(Json::Object(vec![
        ("stream".to_string(), Json::String(event.stream.clone())),
        ("sequence".to_string(), Json::Number(event.sequence as f64)),
        ("id".to_string(), Json::String(event.id.clone())),
        ("generation".to_string(), Json::Number(event.generation as f64)),
        ("type".to_string(), Json::String(event.kind.clone())),
        ("payloadJson".to_string(), Json::String(payload)),
        ("checksum".to_string(), Json::String(coordinator::event_checksum(&event))),
    ])))
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("golden-line-is-canonical", golden_line_is_canonical)
        .subject("checksum-is-sha256-of-the-preimage", checksum_is_sha256_of_the_preimage)
}
//#endregion 🔖️Registration
