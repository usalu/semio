//! 🦀️ Rust side of the session logging case. The subject halves are gated behind the `sut` feature
//! the generated host turns on for the subject role only.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_hooks::serde_json::{self, Value};
    use semio_framework_repo_hooks::{dispatch_hook, parse_repo_config, record_session_hook, resolve_log_session_id, validate_hook_event, HookContext, InertEnvironment, InertTestFileResolver, LoggingConfig, MemorySessionStore, SessionMeta};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    const VECTORS: &str = "shared://📓️session-logging/📓️sessions.json";

    fn vectors(ctx: &Context) -> Result<Value, String> {
        let bytes = ctx.fixture_bytes(VECTORS)?;
        serde_json::from_slice(&bytes).map_err(|error| error.to_string())
    }

    fn text(value: &Value, key: &str) -> String {
        value.get(key).and_then(Value::as_str).unwrap_or_default().to_string()
    }

    fn rows(value: &Value, key: &str) -> Vec<Value> {
        value.get(key).and_then(Value::as_array).cloned().unwrap_or_default()
    }

    fn session_by_id(vectors: &Value, id: &str) -> Result<Value, String> {
        rows(vectors, "sessions").into_iter().find(|session| text(session, "id") == id).ok_or_else(|| format!("the fixture has no session {id}"))
    }

    fn as_projection(value: &Value) -> Result<Json, String> {
        parse_json(&value.to_string())
    }

    /// ▶️ Replays one session's invocations into a fresh memory store and answers what was recorded.
    fn replay(session: &Value, logging: &LoggingConfig) -> Result<Option<SessionMeta>, String> {
        let client = text(session, "client");
        let kiro_pid = session.get("kiroParentPid").and_then(Value::as_u64).map(|pid| pid as u32);
        let mut store = MemorySessionStore::new();
        let mut last = None;
        for invocation in rows(session, "invocations") {
            let event = validate_hook_event(&text(&invocation, "event")).map_err(|error| error.message)?;
            let mut context = HookContext::new(event, client.clone()).at_second(text(&invocation, "second")).at_root("/repo").with_tool("", text(&invocation, "toolArgs"));
            if let Some(input) = invocation.get("input") {
                if !input.is_null() {
                    context = context.with_input(input.clone());
                }
            }
            let result = dispatch_hook(&context, &InertEnvironment, &InertTestFileResolver);
            let session_id = resolve_log_session_id(&context, kiro_pid);
            let recorded = record_session_hook(&mut store, &context, &result, &session_id, logging, &InertEnvironment).map_err(|error| error.message)?;
            if recorded.is_some() {
                last = recorded;
            }
        }
        Ok(last)
    }

    fn on(detail: &str, plan: bool) -> LoggingConfig {
        LoggingConfig { session: true, operations: true, plan, detail: detail.to_string() }
    }

    /// ⚙️ Every configuration is parsed and obeyed the same way.
    pub fn the_configuration_decides_whether_anything_is_recorded(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let session = session_by_id(&vectors, "an-agent-session-that-plans-and-is-refused")?;
        let mut projected = Vec::new();
        for config in rows(&vectors, "configs") {
            let logging = parse_repo_config(&text(&config, "document")).logging;
            let recorded = replay(&session, &logging)?;
            projected.push((
                text(&config, "id"),
                Json::Object(vec![
                    ("session".to_string(), Json::Bool(logging.session)),
                    ("operations".to_string(), Json::Bool(logging.operations)),
                    ("plan".to_string(), Json::Bool(logging.plan)),
                    ("detail".to_string(), Json::String(logging.detail.clone())),
                    ("includeResponse".to_string(), Json::Bool(logging.include_response())),
                    ("includeNative".to_string(), Json::Bool(logging.include_native())),
                    ("recordedEntries".to_string(), Json::Number(recorded.as_ref().map(|meta| meta.events.len()).unwrap_or_default() as f64)),
                ]),
            ));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 🔬️ Detail decides how much of each entry survives.
    pub fn detail_decides_how_much_of_each_entry_survives(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let session = session_by_id(&vectors, "an-agent-session-that-plans-and-is-refused")?;
        let refused = rows(&session, "invocations").into_iter().find(|invocation| text(invocation, "event") == "agent.tool.terminal.starting").ok_or("the fixture has no refused invocation")?;
        let single = serde_json::json!({ "client": text(&session, "client"), "invocations": [refused] });
        let mut projected = Vec::new();
        for detail in ["minimal", "standard", "full"] {
            let recorded = replay(&single, &on(detail, true))?.ok_or("session logging was on but nothing was recorded")?;
            let entry = recorded.events.first().ok_or("no entry was recorded")?;
            projected.push((
                detail.to_string(),
                Json::Object(vec![
                    ("carriesNative".to_string(), Json::Bool(entry.native.is_some())),
                    ("carriesResponse".to_string(), Json::Bool(entry.response.is_some())),
                    ("blocked".to_string(), Json::Bool(entry.response.as_ref().and_then(|response| response.blocked).unwrap_or(false))),
                    ("entry".to_string(), as_projection(&serde_json::to_value(entry).map_err(|error| error.to_string())?)?),
                ]),
            ));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 🪪️ A session that does not identify itself is still recorded under a resolvable identity.
    pub fn a_session_is_recorded_under_a_resolvable_identity(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let mut projected = Vec::new();
        for session in rows(&vectors, "sessions") {
            let recorded = replay(&session, &on("standard", true))?;
            projected.push((
                text(&session, "id"),
                match recorded {
                    Some(meta) => Json::Object(vec![
                        ("id".to_string(), Json::String(meta.id.clone())),
                        ("uri".to_string(), Json::String(meta.uri.clone())),
                        ("client".to_string(), Json::String(meta.client.clone())),
                        ("second".to_string(), Json::String(meta.second.clone())),
                        ("transcript".to_string(), Json::String(meta.transcript.clone())),
                        ("contributor".to_string(), Json::String(meta.contributor.clone())),
                        ("entries".to_string(), Json::Number(meta.events.len() as f64)),
                    ]),
                    None => Json::Object(vec![("id".to_string(), Json::String(String::new())), ("uri".to_string(), Json::String(String::new())), ("client".to_string(), Json::String(String::new())), ("second".to_string(), Json::String(String::new())), ("transcript".to_string(), Json::String(String::new())), ("contributor".to_string(), Json::String(String::new())), ("entries".to_string(), Json::Number(0.0))]),
                },
            ));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 🚫️ No version hook ever reaches the session log.
    pub fn a_version_hook_is_never_recorded(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let session = session_by_id(&vectors, "a-version-hook-is-never-recorded")?;
        let recorded = replay(&session, &on("full", true))?;
        Ok(Outcome::projection(Json::Object(vec![("recordedNothing".to_string(), Json::Bool(recorded.is_none()))])))
    }

    /// 🗺️ The recorded plan is folded only while the plan switch is on.
    pub fn the_recorded_plan_follows_the_plan_switch(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let session = session_by_id(&vectors, "an-agent-session-that-plans-and-is-refused")?;
        let with_plan = replay(&session, &on("standard", true))?.ok_or("nothing was recorded with the plan switch on")?;
        let without_plan = replay(&session, &on("standard", false))?.ok_or("nothing was recorded with the plan switch off")?;
        Ok(Outcome::projection(Json::Object(vec![
            ("withPlan".to_string(), as_projection(&serde_json::to_value(&with_plan.plan).map_err(|error| error.to_string())?)?),
            ("withoutPlanIsAbsent".to_string(), Json::Bool(without_plan.plan.is_none())),
        ])))
    }
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let registered = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let registered = registered
        .subject("the-configuration-decides-whether-anything-is-recorded", subject::the_configuration_decides_whether_anything_is_recorded)
        .subject("detail-decides-how-much-of-each-entry-survives", subject::detail_decides_how_much_of_each_entry_survives)
        .subject("a-session-is-recorded-under-a-resolvable-identity", subject::a_session_is_recorded_under_a_resolvable_identity)
        .subject("a-version-hook-is-never-recorded", subject::a_version_hook_is_never_recorded)
        .subject("the-recorded-plan-follows-the-plan-switch", subject::the_recorded_plan_follows_the_plan_switch);
    registered
}
//#endregion 🔖️Registration
