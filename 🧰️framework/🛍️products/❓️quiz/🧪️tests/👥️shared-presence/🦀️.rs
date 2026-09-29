//! 👥️ Subject adapter of the shared-presence case: the scopes and the presence admission of the `quiz` crate.
//!
//! A state is accepted when it decodes into its typed twin — which refuses unknown members and non-numbers —
//! and `presence_problem`/`cursor_problem` report no problem.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/👥️presence/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use quiz::serde_json::{self, Map, Value};
    use quiz::{cursor_problem, presence_problem, room_scope, roster_scope, thinking_problem, thinking_scope, CursorState, Place, PresenceState, ThinkingState};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://👥️shared-presence/🔣️.json";

    /// 🧫️ One group of the committed vectors.
    fn group(ctx: &Context, name: &str) -> Result<Vec<Value>, String> {
        let document: Value = serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))?;
        document.get(name).and_then(Value::as_array).cloned().ok_or_else(|| format!("{VECTORS} carries no {name} group"))
    }

    /// 🗝️ The projection keyed by vector id.
    fn keyed(vectors: &[Value], answer: impl Fn(&Value) -> Result<Value, String>) -> Result<Outcome, String> {
        let mut projection = Map::new();
        for vector in vectors {
            projection.insert(vector["id"].as_str().unwrap_or_default().to_string(), answer(vector)?);
        }
        Ok(Outcome::projection(parse_json(&Value::Object(projection).to_string())?))
    }

    /// 🗺️ The roster scope and the room scope of every committed catalog and place.
    pub fn room_scopes(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "scopes")?, |vector| {
            let catalog = vector["catalog"].as_str().unwrap_or_default();
            let place: Place = serde_json::from_value(vector["place"].clone()).map_err(|error| format!("{error} in {vector}"))?;
            Ok(serde_json::json!({ "roster": roster_scope(catalog), "room": room_scope(catalog, &place) }))
        })
    }

    /// 👀️ Every committed presence state, accepted or refused.
    pub fn presence_states(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "presence")?, |vector| Ok(Value::Bool(serde_json::from_value::<PresenceState>(vector["state"].clone()).map(|state| presence_problem(&state).is_none()).unwrap_or(false))))
    }

    /// 🎯️ Every committed cursor state, accepted or refused.
    pub fn cursor_states(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "cursors")?, |vector| Ok(Value::Bool(serde_json::from_value::<CursorState>(vector["state"].clone()).map(|state| cursor_problem(&state).is_none()).unwrap_or(false))))
    }

    /// 🗯️ The thinking room of every committed catalog and quiz.
    pub fn thinking_scopes(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "thinkingScopes")?, |vector| Ok(Value::String(thinking_scope(vector["catalog"].as_str().unwrap_or_default(), vector["quiz"].as_str().unwrap_or_default()))))
    }

    /// 🤔️ Every committed thinking state, accepted or refused.
    pub fn thinking_states(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "thinking")?, |vector| Ok(Value::Bool(serde_json::from_value::<ThinkingState>(vector["state"].clone()).map(|state| thinking_problem(&state).is_none()).unwrap_or(false))))
    }
}

/// 🧭️ Subject role only — the oracle is python-jsonschema with the place rules in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("room-scopes", subject::room_scopes).subject("presence-states", subject::presence_states).subject("cursor-states", subject::cursor_states).subject("thinking-scopes", subject::thinking_scopes).subject("thinking-states", subject::thinking_states);
    built
}
