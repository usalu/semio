//! 🦀️ Rust side of the hook result formatting case. The subject halves are gated behind the `sut`
//! feature the generated host turns on for the subject role only.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_hooks::serde_json::{self, Value};
    use semio_framework_repo_hooks::{dispatch_hook, micro_commit_argv, render_hook_output, resolve_native_event_name, run_micro_commit, validate_hook_event, HookContext, InertEnvironment, InertTestFileResolver, ProcessRunner, RecordedProcessRunner};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    const VECTORS: &str = "shared://🖨️hook-result-formatting/🖨️invocations.json";
    const BARE_CLIENT: &str = "claude-code";
    const WRAPPING_CLIENT: &str = "copilot-chat";

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

    fn context_of(vector: &Value, client: &str) -> Result<HookContext, String> {
        let event = validate_hook_event(&text(vector, "event")).map_err(|error| error.message)?;
        let mut context = HookContext::new(event, client).at_second("2026-09-06T12:00:00Z").at_root("/repo").with_tool(text(vector, "toolName"), text(vector, "toolArgs")).with_parent(text(vector, "parentInfo"));
        if let Some(input) = vector.get("input") {
            if !input.is_null() {
                context = context.with_input(input.clone());
            }
        }
        Ok(context)
    }

    /// 🖨️ Copilot Chat is wrapped, the other seven are bare.
    pub fn every_client_receives_the_shape_it_reads(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let clients: Vec<String> = rows(&vectors, "clients").iter().map(|client| client.as_str().unwrap_or_default().to_string()).collect();
        let mut projected = Vec::new();
        for vector in rows(&vectors, "invocations") {
            let mut per_client = Vec::new();
            for client in &clients {
                let context = context_of(&vector, client)?;
                let event = context.hook_event().ok_or("the fixture names an unknown event")?;
                let result = dispatch_hook(&context, &InertEnvironment, &InertTestFileResolver);
                let native = resolve_native_event_name(client, event, &context.parent_info, context.input.as_ref());
                let output = render_hook_output(client, event, &context.parent_info, &native, &result, false);
                let wrapped = parse_json(&output.stdout).map(|record| record.get("hookSpecificOutput").is_some()).unwrap_or(false);
                per_client.push((client.clone(), Json::Object(vec![("wrapped".to_string(), Json::Bool(wrapped)), ("exit".to_string(), Json::Number(f64::from(output.exit_code))), ("stdoutIsEmpty".to_string(), Json::Bool(output.stdout.is_empty())), ("stderrIsEmpty".to_string(), Json::Bool(output.stderr.is_empty()))])));
            }
            projected.push((text(&vector, "id"), Json::Object(per_client)));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 🚫️ A refusal denies inside the wrapped record and exits 2 outside it.
    pub fn a_refusal_is_unmistakable_in_both_shapes(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let mut projected = Vec::new();
        for vector in rows(&vectors, "invocations") {
            let expected_allowed = matches!(vector.get("expectedAllowed"), Some(Value::Bool(true)));
            let expected_exit = vector.get("expectedExit").and_then(Value::as_i64).unwrap_or_default();
            let wrapping_context = context_of(&vector, WRAPPING_CLIENT)?;
            let event = wrapping_context.hook_event().ok_or("the fixture names an unknown event")?;
            let wrapping_result = dispatch_hook(&wrapping_context, &InertEnvironment, &InertTestFileResolver);
            let native = resolve_native_event_name(WRAPPING_CLIENT, event, &wrapping_context.parent_info, wrapping_context.input.as_ref());
            let wrapped_output = render_hook_output(WRAPPING_CLIENT, event, &wrapping_context.parent_info, &native, &wrapping_result, false);
            let wrapped_record = parse_json(&wrapped_output.stdout)?;
            let decision = wrapped_record.get("hookSpecificOutput").map(|specific| specific.str("permissionDecision")).unwrap_or_default();
            let bare_context = context_of(&vector, BARE_CLIENT)?;
            let bare_result = dispatch_hook(&bare_context, &InertEnvironment, &InertTestFileResolver);
            let bare_output = render_hook_output(BARE_CLIENT, event, &bare_context.parent_info, "", &bare_result, false);
            projected.push((
                text(&vector, "id"),
                Json::Object(vec![
                    ("allowedMatchesSpecification".to_string(), Json::Bool(bare_result.allowed == expected_allowed)),
                    ("bareExitMatchesSpecification".to_string(), Json::Bool(i64::from(bare_output.exit_code) == expected_exit)),
                    ("wrappedExitIsAlwaysZero".to_string(), Json::Bool(wrapped_output.exit_code == 0)),
                    ("permissionDecision".to_string(), Json::String(decision)),
                    ("refusalReachesStderrOnlyWhenBare".to_string(), Json::Bool(bare_output.stderr.is_empty() == bare_result.allowed && wrapped_output.stderr.is_empty())),
                ]),
            ));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 🧾️ JSON mode always prints a parseable record and never exits 2.
    pub fn json_mode_always_prints_the_bare_record_to_stdout(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let mut projected = Vec::new();
        for vector in rows(&vectors, "invocations") {
            let context = context_of(&vector, BARE_CLIENT)?;
            let event = context.hook_event().ok_or("the fixture names an unknown event")?;
            let result = dispatch_hook(&context, &InertEnvironment, &InertTestFileResolver);
            let output = render_hook_output(BARE_CLIENT, event, &context.parent_info, "", &result, true);
            let record = parse_json(&output.stdout)?;
            projected.push((
                text(&vector, "id"),
                Json::Object(vec![
                    ("exit".to_string(), Json::Number(f64::from(output.exit_code))),
                    ("stderrIsEmpty".to_string(), Json::Bool(output.stderr.is_empty())),
                    ("recordIsAnObject".to_string(), Json::Bool(matches!(record, Json::Object(_)))),
                    ("allowed".to_string(), Json::Bool(matches!(record.get("allowed"), Some(Json::Bool(true))))),
                    ("carriesNoWrapper".to_string(), Json::Bool(record.get("hookSpecificOutput").is_none())),
                ]),
            ));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 📋️ Each event contributes its own members and no others.
    pub fn the_record_carries_the_event_specific_members(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let mut projected = Vec::new();
        for vector in rows(&vectors, "invocations") {
            let context = context_of(&vector, BARE_CLIENT)?;
            let result = dispatch_hook(&context, &InertEnvironment, &InertTestFileResolver);
            let mut members: Vec<String> = result.payload().keys().cloned().collect();
            members.sort();
            projected.push((text(&vector, "id"), Json::Array(members.into_iter().map(Json::String).collect())));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 🔖️ Every micro-commit subcommand is delegated through the same argv.
    pub fn micro_commit_is_delegated_through_one_argv(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let mut projected = Vec::new();
        for vector in rows(&vectors, "microCommit") {
            let args: Vec<String> = rows(&vector, "args").iter().map(|argument| argument.as_str().unwrap_or_default().to_string()).collect();
            let argv = micro_commit_argv("/bin/bun", &args);
            let transcript = serde_json::json!({ "exchanges": [{ "argv": argv, "stdout": "micro-commit done\n", "stderr": "", "status": 0 }] });
            let runner = RecordedProcessRunner::from_json(&transcript.to_string()).map_err(|error| error.message)?;
            let outcome = run_micro_commit(&runner, "/repo", "/bin/bun", &args);
            projected.push((
                text(&vector, "id"),
                Json::Object(vec![
                    ("argv".to_string(), Json::Array(argv.iter().cloned().map(Json::String).collect())),
                    ("issued".to_string(), Json::Array(runner.issued().into_iter().map(|issued| Json::String(issued.join(" "))).collect())),
                    ("stdout".to_string(), Json::String(outcome.stdout)),
                    ("status".to_string(), Json::Number(f64::from(outcome.status))),
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
        .subject("every-client-receives-the-shape-it-reads", subject::every_client_receives_the_shape_it_reads)
        .subject("a-refusal-is-unmistakable-in-both-shapes", subject::a_refusal_is_unmistakable_in_both_shapes)
        .subject("json-mode-always-prints-the-bare-record-to-stdout", subject::json_mode_always_prints_the_bare_record_to_stdout)
        .subject("the-record-carries-the-event-specific-members", subject::the_record_carries_the_event_specific_members)
        .subject("micro-commit-is-delegated-through-one-argv", subject::micro_commit_is_delegated_through_one_argv);
    registered
}
//#endregion 🔖️Registration
