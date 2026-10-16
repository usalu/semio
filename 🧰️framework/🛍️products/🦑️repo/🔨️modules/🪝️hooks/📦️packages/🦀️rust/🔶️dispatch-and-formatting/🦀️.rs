//! 🔶️ Dispatch and formatting: the per-event hook dispatch and the hook result formatting.
//!
//! Holds the `🔶️Dispatch`, `🖨️Formatting` regions of the crate root; the root re-exports its items.

use super::*;

//#region 🔶️Dispatch
fn insert_when_present(result: HookResult, key: &str, value: &str) -> HookResult {
    if value.is_empty() {
        result
    } else {
        result.with(key, Value::String(value.to_string()))
    }
}

fn string_array(values: &[String]) -> Value {
    Value::Array(values.iter().map(|value| Value::String(value.clone())).collect())
}

fn agent_base(context: &HookContext, environment: &dyn HookEnvironment) -> HookResult {
    let input = context.input.as_ref();
    let mut session = extract_session_id(input);
    let second = first_non_empty(&[find_nested_string_value(input, &["second"]), context.second.clone()]);
    let mut parent = resolve_parent_session_id("", input);
    if !context.parent_info.is_empty() {
        parent = resolve_parent_session_id(&context.parent_info, input);
        if context.parent_info == "subagent" {
            let agent_id = find_nested_string_value(input, &["agent_id", "agentId"]);
            if !agent_id.is_empty() {
                let parent_session = first_non_empty(&[extract_parent(input), extract_session_id(input)]);
                session = agent_id;
                parent = normalize_parent_session_id(&parent_session);
            } else if parent.is_empty() {
                parent = session.clone();
            }
        }
    }
    let mut result = HookResult { message_shadowed: true, ..HookResult::new(true, "") };
    result = insert_when_present(result, "checkpoint", &extract_checkpoint_id(input, environment));
    result = insert_when_present(result, "session", &session);
    result = insert_when_present(result, "second", &second);
    result = insert_when_present(result, "client", &context.client);
    result = insert_when_present(result, "llm", &extract_llm(input));
    result = insert_when_present(result, "effort", &extract_effort(input));
    result = insert_when_present(result, "transcript", &extract_transcript(input));
    result = insert_when_present(result, "message", &extract_message_id(input));
    result.with("parent", Value::String(parent))
}

/// 🔤️ The first non-blank value.
pub fn first_non_empty(values: &[String]) -> String {
    values.iter().map(|value| value.trim()).find(|value| !value.is_empty()).unwrap_or_default().to_string()
}

/// 🔶️ Builds the event-specific hook result for one context.
///
/// Blocking is applied here and nowhere else: `agent.tool.starting` and
/// `agent.tool.terminal.starting` are the two events that can refuse an invocation, and a refusal is
/// carried as `allowed = false` plus the reason, exactly as every IDE reads it back.
pub fn dispatch_hook(context: &HookContext, environment: &dyn HookEnvironment, tests: &dyn TestFileResolver) -> HookResult {
    let Some(event) = context.hook_event() else {
        return HookResult::new(false, format!("unknown event: {}", context.event));
    };
    let input = context.input.as_ref();
    if hook_event_kind(event) == HookKind::Version {
        let mut result = HookResult::new(true, if event == HookEvent::VersionCheckpointStarting { "checkpoint starting hook passed" } else { "" });
        result = insert_when_present(result, "checkpoint", &extract_checkpoint_id(input, environment));
        if matches!(event, HookEvent::VersionCheckpointStarting | HookEvent::VersionCheckpointEnded) {
            result = insert_when_present(result, "description", &extract_checkpoint_message(input, &context.repo_root, environment));
        }
        return result;
    }
    let base = agent_base(context, environment);
    match event {
        HookEvent::AgentStarted => base,
        HookEvent::AgentEnded => insert_when_present(base, "report", &extract_report(input)),
        HookEvent::AgentPromptSubmitting => insert_when_present(base, "prompt", &find_nested_string_value(input, &["prompt", "text"])),
        HookEvent::AgentCompacting => insert_when_present(base, "chat", &extract_chat(input)),
        HookEvent::AgentToolStarting => {
            let tool_name = first_non_empty(&[context.tool_name.clone(), extract_tool_name(input)]);
            let args = first_non_empty(&[context.tool_args.clone(), extract_terminal_command(input, &context.tool_args), extract_command_from_stdin(input)]);
            let reason = is_tool_blocked(&tool_name, &args);
            let mut result = HookResult { allowed: reason.is_none(), message: reason.unwrap_or_default(), extra: base.extra, message_shadowed: base.message_shadowed };
            result = insert_when_present(result, "name", &tool_name);
            if let Some(tool_input) = extract_tool_input_map(input) {
                result = result.with("input", Value::Object(tool_input.clone()));
            }
            result
        }
        HookEvent::AgentToolEnded => {
            let mut result = insert_when_present(base, "name", &first_non_empty(&[context.tool_name.clone(), extract_tool_name(input)]));
            if let Some(tool_input) = extract_tool_input_map(input) {
                result = result.with("input", Value::Object(tool_input.clone()));
            }
            if let Some(response) = extract_tool_response(input) {
                result = result.with("response", response);
            }
            result
        }
        HookEvent::AgentToolPlanUpdatingStarting | HookEvent::AgentToolPlanUpdatingEnded => {
            let steps = extract_plan_steps(input, &context.tool_args);
            if steps.is_empty() {
                base
            } else {
                base.with("steps", serde_json::to_value(&steps).unwrap_or(Value::Null))
            }
        }
        HookEvent::AgentToolSearchStarting | HookEvent::AgentToolSearchEnded => {
            let reads = extract_search_reads(input, &context.tool_args);
            let mut result = base;
            if !reads.pages.is_empty() {
                result = result.with("pages", string_array(&reads.pages));
            }
            if !reads.ranges.is_empty() {
                result = result.with("ranges", string_array(&reads.ranges));
            }
            result
        }
        HookEvent::AgentToolCodeEditStarting => {
            let edit = extract_code_edit(input, &context.tool_args);
            let mut result = insert_when_present(base, "path", &edit.path);
            result = insert_when_present(result, "old", &edit.old);
            result = insert_when_present(result, "new", &edit.new);
            if edit.all {
                result = result.with("all", Value::Bool(true));
            }
            result
        }
        HookEvent::AgentToolCodeEditEnded => {
            let edit = extract_code_edit(input, &context.tool_args);
            if !edit.path.is_empty() {
                environment.format_file(&edit.path);
            }
            let mut result = insert_when_present(base, "path", &edit.path);
            result = insert_when_present(result, "old", &edit.old);
            insert_when_present(result, "new", &edit.new)
        }
        HookEvent::AgentToolTestStarting => {
            let selection = extract_test_selection(input, &context.tool_args, tests);
            let mut result = base;
            if !selection.labs.is_empty() {
                result = result.with("labs", string_array(&selection.labs));
            }
            if !selection.tests.is_empty() {
                result = result.with("tests", string_array(&selection.tests));
            }
            insert_when_present(result, "timeout", &selection.timeout)
        }
        HookEvent::AgentToolTestEnded => {
            let outcome = extract_test_outcome(input, tests);
            let mut result = base;
            if !outcome.files.is_empty() {
                result = result.with("files", string_array(&outcome.files));
            }
            if !outcome.succeeded.is_empty() {
                result = result.with("succeeded", string_array(&outcome.succeeded));
            }
            if !outcome.failed.is_empty() {
                result = result.with("failed", string_array(&outcome.failed));
            }
            result
        }
        HookEvent::AgentToolBuildStarting => {
            let bundles = extract_build_bundles(input, &context.tool_args);
            let mut result = insert_when_present(base, "name", &first_non_empty(&[context.tool_name.clone(), "build".to_string()]));
            result = result.with("input", serde_json::json!({ "bundles": bundles }));
            result
        }
        HookEvent::AgentToolBuildEnded => {
            let outcome = extract_build_outcome(input);
            let mut result = insert_when_present(base, "name", &first_non_empty(&[context.tool_name.clone(), "build".to_string()]));
            result = result.with("response", serde_json::json!({ "succeeded": outcome.succeeded, "failed": outcome.failed }));
            result
        }
        HookEvent::AgentToolTerminalStarting => {
            let command = extract_terminal_command(input, &context.tool_args);
            let tool = first_non_empty(&[context.tool_name.clone(), "run_in_terminal".to_string()]);
            let reason = is_tool_blocked(&tool, &first_non_empty(&[command.clone(), context.tool_args.clone()]));
            let mut result = HookResult { allowed: reason.is_none(), message: reason.unwrap_or_default(), extra: base.extra, message_shadowed: base.message_shadowed };
            result = insert_when_present(result, "name", &first_non_empty(&[context.tool_name.clone(), extract_tool_name(input)]));
            if let Some(tool_input) = extract_tool_input_map(input) {
                result = result.with("input", Value::Object(tool_input.clone()));
            }
            insert_when_present(result, "command", &command)
        }
        HookEvent::AgentToolTerminalEnded => {
            let outcome = extract_terminal_outcome(input);
            let mut result = insert_when_present(base, "name", &first_non_empty(&[context.tool_name.clone(), extract_tool_name(input)]));
            if let Some(tool_input) = extract_tool_input_map(input) {
                result = result.with("input", Value::Object(tool_input.clone()));
            }
            result = insert_when_present(result, "command", &outcome.command);
            result = result.with("pid", Value::Number(outcome.pid.parse::<i64>().unwrap_or_default().into()));
            if outcome.terminated {
                result = result.with("terminated", Value::Bool(true));
            }
            result = result.with("stdout", Value::String(outcome.stdout));
            result.with("stderr", Value::String(outcome.stderr))
        }
        HookEvent::AgentThinkingStarting | HookEvent::AgentThinkingEnded => {
            let text = find_nested_string_value(input, &["text"]);
            HookResult { allowed: base.allowed, message: text, extra: base.extra, message_shadowed: base.message_shadowed }
        }
        _ => base,
    }
}

fn extract_tool_response(input: Option<&Value>) -> Option<Value> {
    let object = decode_hook_input_map(input)?;
    if let Some(response) = object.get("tool_response") {
        return Some(response.clone());
    }
    if let Some(output) = object.get("tool_output") {
        return Some(output.clone());
    }
    let output = find_nested_string_value(input, &["tool_output", "toolOutput", "output", "response"]);
    if output.is_empty() {
        None
    } else {
        Some(Value::String(output))
    }
}

/// 🧪️ What a starting test event selected.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestSelection {
    pub labs: Vec<String>,
    pub tests: Vec<String>,
    pub timeout: String,
}

/// 🧪️ What a finished test event reports.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestOutcome {
    pub files: Vec<String>,
    pub succeeded: Vec<String>,
    pub failed: Vec<String>,
}

fn looks_like_file_path(value: &str) -> bool {
    value.contains('/') || value.contains('\\') || (value.contains('.') && !value.starts_with('.'))
}

fn string_members(source: &Map<String, Value>, keys: &[&str]) -> Vec<String> {
    let mut collected = Vec::new();
    for key in keys {
        match source.get(*key) {
            Some(Value::Array(items)) => {
                for item in items {
                    if let Value::String(item) = item {
                        if !item.is_empty() {
                            collected.push(item.clone());
                        }
                    }
                }
            }
            Some(Value::String(item)) if !item.is_empty() => collected.push(item.clone()),
            _ => {}
        }
    }
    collected
}

/// 🧪️ Reads a starting test event: the files and test selectors it names, and its timeout.
pub fn extract_test_selection(input: Option<&Value>, tool_args: &str, tests: &dyn TestFileResolver) -> TestSelection {
    let source = extract_tool_input_map(input).cloned().or_else(|| parse_tool_args(tool_args)).unwrap_or_default();
    let mut selection = TestSelection::default();
    let mut target_files = string_members(&source, &["files"]);
    for named in string_members(&source, &["testNames", "test_names", "tests"]) {
        if looks_like_file_path(&named) {
            target_files.push(named);
        } else {
            selection.tests.push(named);
        }
    }
    for key in ["timeout", "timeoutMs"] {
        match source.get(key) {
            Some(Value::String(raw)) if !raw.is_empty() => {
                selection.timeout = raw.clone();
                break;
            }
            Some(Value::Number(number)) => {
                selection.timeout = format!("{}", number.as_f64().unwrap_or_default() as i64);
                break;
            }
            _ => {}
        }
    }
    let mut command = extract_command_from_stdin(input);
    if command.is_empty() {
        if let Some(Value::String(raw)) = source.get("command") {
            command = raw.clone();
        }
    }
    if !command.is_empty() && target_files.is_empty() {
        let (_, cwd) = extract_command_and_cwd(input);
        let resolved = tests.test_files(&command, &cwd);
        if !resolved.is_empty() {
            target_files = resolved;
        }
    }
    if !target_files.is_empty() && selection.tests.iter().all(String::is_empty) {
        selection.labs = tests.lab_ids(&target_files);
        if !selection.labs.is_empty() {
            selection.tests.clear();
            return selection;
        }
    }
    selection
}

/// 🧪️ Reads a finished test event: the files it ran and which cases passed or failed.
pub fn extract_test_outcome(input: Option<&Value>, tests: &dyn TestFileResolver) -> TestOutcome {
    let Some(object) = decode_hook_input_map(input) else { return TestOutcome::default() };
    let mut outcome = TestOutcome::default();
    if let Some(tool_input) = extract_tool_input_map(input) {
        outcome.files = string_members(tool_input, &["files"]);
    }
    if outcome.files.is_empty() {
        let (command, cwd) = extract_command_and_cwd(input);
        if !command.is_empty() {
            outcome.files = tests.test_files(&command, &cwd);
        }
    }
    let tool_output = object.get("tool_output").and_then(Value::as_object).unwrap_or(object);
    outcome.succeeded = string_members(tool_output, &["succeeded", "passed", "passing"]);
    outcome.failed = string_members(tool_output, &["failed", "failing", "errors"]);
    outcome
}

/// 🏗️ The bundles a starting build event names.
pub fn extract_build_bundles(input: Option<&Value>, tool_args: &str) -> Vec<String> {
    let source = extract_tool_input_map(input).cloned().or_else(|| parse_tool_args(tool_args));
    let Some(source) = source else { return Vec::new() };
    string_members(&source, &["bundles", "targets", "technologies", "label"])
}

/// 🏗️ What a finished build event reports.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildOutcome {
    pub succeeded: Vec<String>,
    pub failed: Vec<String>,
}

/// 🏗️ Reads a finished build event.
pub fn extract_build_outcome(input: Option<&Value>) -> BuildOutcome {
    let Some(object) = decode_hook_input_map(input) else { return BuildOutcome::default() };
    let tool_output = object.get("tool_output").and_then(Value::as_object).unwrap_or(object);
    BuildOutcome { succeeded: string_members(tool_output, &["succeeded", "passed", "built"]), failed: string_members(tool_output, &["failed", "errors"]) }
}
//#endregion 🔶️Dispatch

//#region 🖨️Formatting
/// 🖨️ What the repo binary writes after a hook, and with which exit code.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookOutput {
    #[serde(rename = "stdout", default)]
    pub stdout: String,
    #[serde(rename = "stderr", default)]
    pub stderr: String,
    #[serde(rename = "exitCode", default)]
    pub exit_code: i32,
}

/// 🖨️ Renders a hook result the way the calling client reads it.
///
/// Copilot Chat is the only client whose result is wrapped (`hookSpecificOutput`), and it is written
/// to stdout whether or not the invocation was allowed — the permission decision lives inside the
/// record. `json_mode` prints the plain record for every other client. Otherwise a refusal goes to
/// stderr with exit code 2 and an allowed result prints only its message, if it has one.
pub fn render_hook_output(client: &str, event: HookEvent, parent_info: &str, native_event_name: &str, result: &HookResult, json_mode: bool) -> HookOutput {
    if let Some(provider) = get_editor_provider(client) {
        if provider.kind() == "copilot-chat" {
            let hook_event_name = if native_event_name.is_empty() { HookFormatter::native_event_from_hook_event(&provider, event, parent_info) } else { native_event_name.to_string() };
            return HookOutput { stdout: HookFormatter::format_hook_output(&provider, &hook_event_name, result), stderr: String::new(), exit_code: 0 };
        }
    }
    if json_mode {
        return HookOutput { stdout: format_plain_hook_output(result), stderr: String::new(), exit_code: 0 };
    }
    if !result.allowed {
        return HookOutput { stdout: String::new(), stderr: result.message.clone(), exit_code: 2 };
    }
    HookOutput { stdout: result.message.clone(), stderr: String::new(), exit_code: 0 }
}

/// 🔤️ The native event name the client would use for a neutral event, or the payload's own name.
pub fn resolve_native_event_name(client: &str, event: HookEvent, parent_info: &str, input: Option<&Value>) -> String {
    let named = extract_hook_event_name(input);
    if !named.is_empty() {
        return named;
    }
    match get_editor_provider(client) {
        Some(provider) => HookFormatter::native_event_from_hook_event(&provider, event, parent_info),
        None => String::new(),
    }
}

/// 🎆️ The editor provider for a client slug, as the closed provider enum.
pub fn editor_for_client(client: &str) -> Option<EditorProviders> {
    get_editor_provider(client)
}
//#endregion 🖨️Formatting
