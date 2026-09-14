//! 🦀️ Rust side of the payload encoding case. Decodes every fixture input into its typed payload and
//! projects the re-encoded JSON, so field order and omit-empty semantics are compared, not described.
//!
//! The fixture is read through the crate's own re-exported `serde_json`: the adapter declares no
//! dependency of its own, exactly as a client of `semio-framework-repo-events` must not have to.

use semio_framework_repo_events as events;
use semio_framework_repo_events::serde_json::{self, Value as SerdeJson};
use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Vectors
struct PayloadCase {
    id: String,
    kind: String,
    input: SerdeJson,
}

fn round_trip(kind: &str, input: &SerdeJson) -> Result<String, String> {
    macro_rules! decode {
        ($($name:literal => $type:ty),* $(,)?) => {
            match kind {
                $($name => {
                    let value: $type = serde_json::from_value(input.clone()).map_err(|error| error.to_string())?;
                    serde_json::to_string(&value).map_err(|error| error.to_string())
                })*
                other => Err(format!("unknown payload type {other}")),
            }
        };
    }
    decode! {
        "TicketPayload" => events::TicketPayload,
        "TicketOpenPayload" => events::TicketOpenPayload,
        "TicketClosePayload" => events::TicketClosePayload,
        "TicketReopenPayload" => events::TicketReopenPayload,
        "TicketChangePayload" => events::TicketChangePayload,
        "GoalPayload" => events::GoalPayload,
        "GoalOpenPayload" => events::GoalOpenPayload,
        "GoalClosePayload" => events::GoalClosePayload,
        "GoalReopenPayload" => events::GoalReopenPayload,
        "GoalChangePayload" => events::GoalChangePayload,
        "ContributorPayload" => events::ContributorPayload,
        "CheckpointPayload" => events::CheckpointPayload,
        "TodoPayload" => events::TodoPayload,
        "TodoCreatePayload" => events::TodoCreatePayload,
        "TodoChangePayload" => events::TodoChangePayload,
        "TodoDeletePayload" => events::TodoDeletePayload,
        "WorkItem" => events::WorkItem,
        "ContributorWork" => events::ContributorWork,
        "DraftPayload" => events::DraftPayload,
        "FilePayload" => events::FilePayload,
        "FolderPayload" => events::FolderPayload,
        "SectionPayload" => events::SectionPayload,
        "IntegratePayload" => events::IntegratePayload,
        "ExtractPayload" => events::ExtractPayload,
    }
}

fn vectors(ctx: &Context) -> Result<Vec<PayloadCase>, String> {
    let bytes = ctx.fixture_bytes("shared://✉️payload-vectors.json")?;
    let parsed: SerdeJson = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    let cases = parsed.get("cases").and_then(SerdeJson::as_array).ok_or("payload vectors have no cases array")?;
    cases
        .iter()
        .map(|case| {
            let id = case.get("id").and_then(SerdeJson::as_str).ok_or("payload case has no id")?.to_string();
            let kind = case.get("type").and_then(SerdeJson::as_str).ok_or("payload case has no type")?.to_string();
            let input = case.get("input").cloned().ok_or("payload case has no input")?;
            Ok(PayloadCase { id, kind, input })
        })
        .collect()
}

fn encode_all(ctx: &Context) -> Result<Vec<(String, String, String)>, String> {
    vectors(ctx)?
        .into_iter()
        .map(|case| {
            round_trip(&case.kind, &case.input)
                .map(|encoded| (case.id.clone(), case.kind.clone(), encoded))
                .map_err(|error| format!("{}: {error}", case.id))
        })
        .collect()
}
//#endregion 🔖️Vectors

//#region 🔖️Scenarios
fn golden_encoding_per_payload(ctx: &Context) -> Result<Outcome, String> {
    let encodings = encode_all(ctx)?;
    let count = encodings.len();
    Ok(Outcome::projection(Json::Object(vec![
        (
            "encodings".to_string(),
            Json::Array(
                encodings
                    .into_iter()
                    .map(|(id, kind, encoded)| {
                        Json::Object(vec![
                            ("id".to_string(), Json::String(id)),
                            ("type".to_string(), Json::String(kind)),
                            ("encoded".to_string(), Json::String(encoded)),
                        ])
                    })
                    .collect(),
            ),
        ),
        ("count".to_string(), Json::Number(count as f64)),
    ])))
}

fn omit_empty_and_explicit_null(ctx: &Context) -> Result<Outcome, String> {
    let encodings = encode_all(ctx)?;
    Ok(Outcome::projection(Json::Object(vec![(
        "optional".to_string(),
        Json::Array(
            encodings
                .into_iter()
                .map(|(id, _, encoded)| {
                    Json::Object(vec![
                        ("id".to_string(), Json::String(id)),
                        ("hasEmptyValue".to_string(), Json::Bool(encoded.contains(":\"\""))),
                        ("hasNullValue".to_string(), Json::Bool(encoded.contains(":null"))),
                    ])
                })
                .collect(),
        ),
    )])))
}

fn envelope_round_trip(_ctx: &Context) -> Result<Outcome, String> {
    let mut payload = serde_json::Map::new();
    payload.insert("id".to_string(), SerdeJson::String("a".to_string()));
    let envelope = events::Event {
        kind: events::TICKET_OPEN_STARTING.to_string(),
        source: "repo-cli".to_string(),
        payload: SerdeJson::Object(payload),
    };
    let encoded = serde_json::to_string(&envelope).map_err(|error| error.to_string())?;
    Ok(Outcome::projection(Json::Object(vec![(
        "envelope".to_string(),
        Json::String(encoded),
    )])))
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("golden-encoding-per-payload", golden_encoding_per_payload)
        .subject("omit-empty-and-explicit-null", omit_empty_and_explicit_null)
        .subject("envelope-round-trip", envelope_round_trip)
}
//#endregion 🔖️Registration
