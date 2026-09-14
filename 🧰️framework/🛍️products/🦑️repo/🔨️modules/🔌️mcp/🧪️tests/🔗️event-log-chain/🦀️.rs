//! 🦀️ Rust subject for the hash-chained event log. Commits the fixture's inputs through the owned log
//! and projects the digests and the JSONL rendering the chain produces.

use semio_framework_repo_mcp as mcp;
use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Support

fn fixture(ctx: &Context) -> Result<Json, String> {
    let raw = ctx.fixture_bytes("shared://🔗️event-chain.json")?;
    parse_json(&String::from_utf8_lossy(&raw))
}

/// 🔗️ Rebuilds the whole chain from the fixture's declared inputs.
fn chain(document: &Json) -> Result<Vec<mcp::Event>, String> {
    let log = mcp::EventLog::new(0, 0);
    let inputs: Vec<mcp::EventInput> = document
        .array("inputs")
        .iter()
        .map(|input| mcp::EventInput {
            kind: input.str("kind"),
            peer: input.str("peer"),
            generation: match input.get("generation") {
                Some(Json::Number(value)) => *value as u64,
                _ => 0,
            },
            request_id: input.str("requestId"),
            payload: input.str("payload"),
        })
        .collect();
    log.commit(&inputs).map_err(|error| error.to_string())?;
    Ok(log.events())
}

//#endregion 🔖️Support

//#region 🔖️Scenarios

fn chain_digests_match_the_golden(ctx: &Context) -> Result<Outcome, String> {
    let document = fixture(ctx)?;
    let events = chain(&document)?;
    let jsonl: String = events.iter().map(|event| format!("{}\n", event.text(&event.hash))).collect();
    Ok(Outcome::projection(Json::Object(vec![
        ("hashes".to_string(), Json::Array(events.iter().map(|event| Json::String(event.hash.clone())).collect())),
        ("jsonl".to_string(), Json::String(jsonl)),
    ])))
}

fn a_tampered_chain_is_refused(ctx: &Context) -> Result<Outcome, String> {
    let document = fixture(ctx)?;
    let jsonl = document.str("jsonl");
    let intact = mcp::replay_events(&jsonl, 0, 0).is_ok();
    let tampered = jsonl.replacen("session.opened", "session.tampered", 1);
    Ok(Outcome::projection(Json::Object(vec![
        ("intactAccepted".to_string(), Json::Bool(intact)),
        ("tamperedRefused".to_string(), Json::Bool(mcp::replay_events(&tampered, 0, 0).is_err())),
    ])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust").subject("chain-digests-match-the-golden", chain_digests_match_the_golden).subject("a-tampered-chain-is-refused", a_tampered_chain_is_refused)
}

//#endregion 🔖️Registration
