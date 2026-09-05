//! 🦀️ Neutral relay-lifecycle trace adapter. The oracle returns the committed expectation; the
//! subject sends the literal trace to the plugin host's production replay/relay lifecycle seam.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

const TRACE_IDS: [&str; 5] = [
    "replay-first-fault-wins",
    "relay-abandoned-blocked-wake",
    "relay-live-terminal-caller-output",
    "relay-stale-generation-refused",
    "relay-max-plus-one-refused",
];

fn fixture() -> Json {
    parse_json(include_str!("../../🧫️fixtures/♻️relay-lifecycle.json")).expect("relay lifecycle fixture")
}

fn trace(id: &str) -> Result<Json, String> {
    fixture().array("traces").into_iter().find(|trace| trace.str("🪪️id") == id).ok_or_else(|| format!("unknown relay lifecycle trace {id:?}"))
}

fn number(value: &Json, key: &str) -> u64 {
    match value.get(key) {
        Some(Json::Number(number)) => *number as u64,
        _ => 0,
    }
}

fn outcome(value: Json) -> Outcome {
    Outcome::with_raw(value.to_string().into_bytes(), value)
}

fn selected(ctx: &Context) -> Result<(String, Json), String> {
    let id = ctx.doc_json()?.str("id");
    Ok((id.clone(), trace(&id)?))
}

#[cfg(feature = "sut")]
fn subject(ctx: &Context) -> Result<Outcome, String> {
    let (id, trace) = selected(ctx)?;
    let actual = parse_json(&semio_framework_plugin_host::exercise_relay_lifecycle_trace(&trace.to_string())?)?;
    let expected = trace.get("expected").cloned().ok_or_else(|| format!("{id}: missing expected transition"))?;
    if actual != expected {
        return Err(format!("{id}: {} != {}", actual.to_string(), expected.to_string()));
    }
    let accounting = trace.get("accounting").ok_or_else(|| format!("{id}: missing accounting"))?;
    if number(accounting, "seedPagesBefore") != number(accounting, "seedPagesAfter") || number(accounting, "abiBytesBefore") != number(accounting, "abiBytesAfter") {
        return Err(format!("{id}: unbalanced fixture accounting"));
    }
    Ok(outcome(actual).dispatched(&id, 1))
}

/// 🧭️ Registers the Gherkin outline that dispatches each literal trace into the Rust production seam.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    for id in TRACE_IDS {
        built = built.subject(&format!("production-traces-{id}"), subject);
    }
    built
}
