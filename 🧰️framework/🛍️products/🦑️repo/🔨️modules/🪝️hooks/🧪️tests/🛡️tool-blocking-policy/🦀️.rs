//! 🦀️ Rust side of the tool blocking policy case. The subject halves are gated behind the `sut`
//! feature the generated host turns on for the subject role only.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_hooks::{contains_blocked_git_in_code, is_tool_blocked, split_command_segments};
    use semio_framework_repo_hooks::serde_json::{self, Value};
    use semio_repo_test_host::{Context, Json, Outcome};

    const VECTORS: &str = "shared://🛡️tool-blocking-policy/🛡️invocations.json";

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

    fn strings(value: &Value, key: &str) -> Vec<String> {
        rows(value, key).iter().map(|item| item.as_str().unwrap_or_default().to_string()).collect()
    }

    fn verdict(reason: Option<String>) -> Json {
        match reason {
            Some(reason) => Json::Object(vec![("blocked".to_string(), Json::Bool(true)), ("reason".to_string(), Json::String(reason))]),
            None => Json::Object(vec![("blocked".to_string(), Json::Bool(false)), ("reason".to_string(), Json::String(String::new()))]),
        }
    }

    /// 🛡️ Every invocation gets the same verdict and the same reason.
    pub fn every_invocation_is_judged_the_same_way(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let mut projected = Vec::new();
        for invocation in rows(&vectors, "invocations") {
            projected.push((text(&invocation, "id"), verdict(is_tool_blocked(&text(&invocation, "tool"), &text(&invocation, "args")))));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 📜️ Every verdict matches the pinned specification.
    pub fn the_verdict_matches_the_pinned_specification(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let mut projected = Vec::new();
        for invocation in rows(&vectors, "invocations") {
            let blocked = is_tool_blocked(&text(&invocation, "tool"), &text(&invocation, "args")).is_some();
            let pinned = matches!(invocation.get("blocked"), Some(Value::Bool(true)));
            projected.push((text(&invocation, "id"), Json::Bool(blocked == pinned)));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// ✂️ A composite command splits into the same segments.
    pub fn a_command_splits_into_the_same_segments(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let mut projected = Vec::new();
        for command in strings(&vectors, "segments") {
            projected.push((command.clone(), Json::Array(split_command_segments(&command).into_iter().map(Json::String).collect())));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 🌿️ Inline code is scanned the same way.
    pub fn inline_code_is_scanned_the_same_way(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let mut projected = Vec::new();
        for code in strings(&vectors, "inlineCode") {
            projected.push((code.clone(), verdict(contains_blocked_git_in_code(&code))));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// ♻️ Extending a refused command with an allowed segment never makes it allowed.
    pub fn refusing_is_idempotent_and_order_free(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let mut projected = Vec::new();
        for invocation in rows(&vectors, "invocations") {
            let tool = text(&invocation, "tool");
            let args = text(&invocation, "args");
            if is_tool_blocked(&tool, &args).is_none() || args.is_empty() {
                continue;
            }
            let appended = is_tool_blocked(&tool, &format!("{args} && echo done"));
            let prepended = is_tool_blocked(&tool, &format!("echo start && {args}"));
            projected.push((
                text(&invocation, "id"),
                Json::Object(vec![
                    ("appendedStillRefused".to_string(), Json::Bool(appended.is_some())),
                    ("prependedStillRefused".to_string(), Json::Bool(prepended.is_some())),
                    ("appendedReason".to_string(), Json::String(appended.unwrap_or_default())),
                    ("prependedReason".to_string(), Json::String(prepended.unwrap_or_default())),
                ]),
            ));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let registered = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let registered = registered
        .subject("every-invocation-is-judged-the-same-way", subject::every_invocation_is_judged_the_same_way)
        .subject("the-verdict-matches-the-pinned-specification", subject::the_verdict_matches_the_pinned_specification)
        .subject("a-command-splits-into-the-same-segments", subject::a_command_splits_into_the_same_segments)
        .subject("inline-code-is-scanned-the-same-way", subject::inline_code_is_scanned_the_same_way)
        .subject("refusing-is-idempotent-and-order-free", subject::refusing_is_idempotent_and_order_free);
    registered
}
//#endregion 🔖️Registration
