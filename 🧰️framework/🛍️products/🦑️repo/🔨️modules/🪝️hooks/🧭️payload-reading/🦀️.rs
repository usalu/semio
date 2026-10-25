//! 🧭️ Payload reading: normalisation of native IDE hook payloads into one neutral shape.
//!
//! Holds the `🧭️PayloadReading` region of the crate root; the root re-exports its items.

use super::*;

//#region 🧭️PayloadReading
/// 🔍️ The payload object, when the input is a JSON object at all.
pub fn decode_hook_input_map(input: Option<&Value>) -> Option<&Map<String, Value>> {
    input.and_then(Value::as_object)
}

/// 🪆️ The first non-blank string reachable under any of `keys`, searched breadth-first at each level
/// then depth-first into every child — the Go `findNestedStringValue` traversal.
pub fn find_nested_string_value(value: Option<&Value>, keys: &[&str]) -> String {
    let Some(value) = value else { return String::new() };
    match value {
        Value::Object(object) => {
            for key in keys {
                if let Some(Value::String(raw)) = object.get(*key) {
                    if !raw.trim().is_empty() {
                        return raw.trim().to_string();
                    }
                }
            }
            for nested in object.values() {
                let found = find_nested_string_value(Some(nested), keys);
                if !found.is_empty() {
                    return found;
                }
            }
            String::new()
        }
        Value::Array(items) => {
            for nested in items {
                let found = find_nested_string_value(Some(nested), keys);
                if !found.is_empty() {
                    return found;
                }
            }
            String::new()
        }
        _ => String::new(),
    }
}

/// 🗺️ The `tool_input` object, at the top level or under `event` or `native.event`.
pub fn extract_tool_input_map(data: Option<&Value>) -> Option<&Map<String, Value>> {
    let object = data?.as_object()?;
    if let Some(tool_input) = object.get("tool_input").and_then(Value::as_object) {
        return Some(tool_input);
    }
    if let Some(event) = object.get("event").and_then(Value::as_object) {
        if let Some(tool_input) = event.get("tool_input").and_then(Value::as_object) {
            return Some(tool_input);
        }
    }
    if let Some(native) = object.get("native").and_then(Value::as_object) {
        if let Some(event) = native.get("event").and_then(Value::as_object) {
            if let Some(tool_input) = event.get("tool_input").and_then(Value::as_object) {
                return Some(tool_input);
            }
        }
    }
    None
}

/// 🔹️ The shell command the payload carries: a bare JSON string, `tool_input.command`, or any
/// nested `command` / `command_line` / `commandLine`.
pub fn extract_command_from_stdin(input: Option<&Value>) -> String {
    let Some(input) = input else { return String::new() };
    if let Value::String(raw) = input {
        return raw.trim().to_string();
    }
    if input.as_object().is_none() {
        return String::new();
    }
    if let Some(tool_input) = extract_tool_input_map(Some(input)) {
        if !tool_input.is_empty() {
            if let Some(Value::String(command)) = tool_input.get("command") {
                return command.trim().to_string();
            }
        }
    }
    find_nested_string_value(Some(input), &["command", "command_line", "commandLine"])
}

/// 📁️ The command and the working directory it runs in; an absent directory stays empty rather than
/// silently becoming the host's cwd, so the projection never depends on where a test ran.
pub fn extract_command_and_cwd(input: Option<&Value>) -> (String, String) {
    let command = extract_command_from_stdin(input);
    let mut cwd = String::new();
    if let Some(tool_input) = extract_tool_input_map(input) {
        if let Some(Value::String(raw)) = tool_input.get("cwd") {
            cwd = raw.trim().to_string();
        }
    }
    if cwd.is_empty() {
        cwd = find_nested_string_value(input, &["cwd", "working_directory", "workingDirectory"]);
    }
    (command, cwd)
}

/// 🔷️ The tool name the payload names.
pub fn extract_tool_name(input: Option<&Value>) -> String {
    find_nested_string_value(input, &["tool", "tool_name", "toolName", "mcp_tool_name", "mcpToolName"])
}

/// 📡️ The native hook event name the payload names.
pub fn extract_hook_event_name(input: Option<&Value>) -> String {
    find_nested_string_value(input, &["hookEventName", "hook_event_name", "hook_event", "hookEvent", "event"])
}

/// 🧾️ The transcript path the payload names.
pub fn extract_transcript(input: Option<&Value>) -> String {
    find_nested_string_value(input, &["transcript", "transcript_path", "transcriptPath", "log_path", "logPath"])
}

/// 🧠️ The model the payload names.
pub fn extract_llm(input: Option<&Value>) -> String {
    find_nested_string_value(input, &["llm", "model", "model_name", "modelName"])
}

/// 🏋️ The reasoning effort the payload names.
pub fn extract_effort(input: Option<&Value>) -> String {
    find_nested_string_value(input, &["effort", "reasoning_effort", "reasoningEffort"])
}

/// ▪️ The message identifier of the turn.
pub fn extract_message_id(input: Option<&Value>) -> String {
    top_level_string(input, &["messageId", "message_id", "turnId", "turn_id", "requestId", "request_id"])
}

/// ▫️ The message identifier of the parent turn.
pub fn extract_parent_message_id(input: Option<&Value>) -> String {
    top_level_string(input, &["parentMessageId", "parent_message_id", "parentTurnId", "parent_turn_id"])
}

fn top_level_string(input: Option<&Value>, keys: &[&str]) -> String {
    let Some(object) = decode_hook_input_map(input) else { return String::new() };
    for key in keys {
        if let Some(Value::String(raw)) = object.get(*key) {
            if !raw.is_empty() {
                return raw.trim().to_string();
            }
        }
    }
    String::new()
}

/// 🪪️ The session identifier, falling back to the basename of the transcript path.
pub fn extract_session_id(input: Option<&Value>) -> String {
    let session = find_nested_string_value(input, &["session_id", "sessionId", "trajectory_id", "trajectoryId", "conversation_id", "conversationId", "agent_id", "agentId"]);
    if !session.is_empty() {
        return session;
    }
    let transcript = extract_transcript(input);
    if transcript.is_empty() {
        return String::new();
    }
    let base = base_name(transcript.trim());
    match base.rfind('.') {
        Some(index) if index > 0 => base[..index].trim().to_string(),
        _ => base.trim().to_string(),
    }
}

/// 💡️ Normalises a parent hint: the literal `subagent` is a shape, not a session id.
pub fn normalize_parent_session_id(parent: &str) -> String {
    let parent = parent.trim();
    if parent.is_empty() || parent.eq_ignore_ascii_case("subagent") {
        return String::new();
    }
    parent.to_string()
}

fn parent_from_object(object: &Map<String, Value>) -> String {
    for key in ["parent", "parentInfo", "parent_info", "parentSessionId", "parent_session_id", "parentId", "parent_id"] {
        if let Some(Value::String(raw)) = object.get(key) {
            let parent = normalize_parent_session_id(raw);
            if !parent.is_empty() {
                return parent;
            }
        }
    }
    String::new()
}

/// 👪️ The parent session the payload names, at the top level, under `event` or under `native`.
pub fn extract_parent(input: Option<&Value>) -> String {
    let Some(object) = decode_hook_input_map(input) else { return String::new() };
    let parent = parent_from_object(object);
    if !parent.is_empty() {
        return parent;
    }
    for key in ["event", "native"] {
        let Some(nested) = object.get(key).and_then(Value::as_object) else { continue };
        let parent = parent_from_object(nested);
        if !parent.is_empty() {
            return parent;
        }
        if key == "native" {
            if let Some(native_event) = nested.get("event").and_then(Value::as_object) {
                let parent = parent_from_object(native_event);
                if !parent.is_empty() {
                    return parent;
                }
            }
        }
    }
    String::new()
}

/// 🔳️ The parent session: the caller's hint when it names one, otherwise the payload's.
pub fn resolve_parent_session_id(parent_info: &str, input: Option<&Value>) -> String {
    let parent = normalize_parent_session_id(parent_info);
    if !parent.is_empty() {
        return parent;
    }
    extract_parent(input)
}

/// 💬️ The conversation the compaction event is about.
pub fn extract_chat(input: Option<&Value>) -> String {
    let Some(object) = decode_hook_input_map(input) else { return String::new() };
    for key in ["chat", "conversation", "context", "messages"] {
        match object.get(key) {
            Some(Value::String(raw)) => return raw.trim().to_string(),
            Some(Value::Array(items)) => return Value::Array(items.clone()).to_string(),
            _ => {}
        }
    }
    String::new()
}

/// 📦️ The report an agent-ended payload carries, at the top level or under `event`/`input`/`native`.
pub fn extract_report(input: Option<&Value>) -> String {
    fn report_of(node: Option<&Map<String, Value>>) -> String {
        let Some(node) = node else { return String::new() };
        if let Some(Value::String(report)) = node.get("report") {
            if !report.trim().is_empty() {
                return report.trim().to_string();
            }
        }
        if let Some(tool_info) = node.get("tool_info").and_then(Value::as_object) {
            if let Some(Value::String(response)) = tool_info.get("response") {
                if !response.trim().is_empty() {
                    return response.trim().to_string();
                }
            }
        }
        String::new()
    }
    let Some(object) = decode_hook_input_map(input) else { return String::new() };
    let report = report_of(Some(object));
    if !report.is_empty() {
        return report;
    }
    for key in ["event", "input"] {
        let report = report_of(object.get(key).and_then(Value::as_object));
        if !report.is_empty() {
            return report;
        }
    }
    if let Some(native) = object.get("native").and_then(Value::as_object) {
        let report = report_of(Some(native));
        if !report.is_empty() {
            return report;
        }
        let report = report_of(native.get("event").and_then(Value::as_object));
        if !report.is_empty() {
            return report;
        }
    }
    String::new()
}

/// 🔖️ The checkpoint identifier the payload names, falling back to the environment.
pub fn extract_checkpoint_id(input: Option<&Value>, environment: &dyn HookEnvironment) -> String {
    let checkpoint = top_level_string(input, &["sha", "checkpoint_sha", "checkpointSha", "hash"]);
    if !checkpoint.is_empty() {
        return checkpoint;
    }
    environment.current_checkpoint()
}

/// ⛏️ The checkpoint message the payload names, falling back to the environment.
pub fn extract_checkpoint_message(input: Option<&Value>, repo_root: &str, environment: &dyn HookEnvironment) -> String {
    let message = top_level_string(input, &["message", "commit_message", "commitMessage"]);
    if !message.is_empty() {
        return message;
    }
    environment.checkpoint_message(repo_root)
}

/// 📝️ What a code edit event carries: the path, the replaced text, the replacement, and whether the
/// edit applies to every occurrence.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeEdit {
    pub path: String,
    pub old: String,
    pub new: String,
    pub all: bool,
}

/// ✏️ Reads a code edit out of the payload, or out of the raw tool argument string.
pub fn extract_code_edit(input: Option<&Value>, tool_args: &str) -> CodeEdit {
    let fallback = parse_tool_args(tool_args);
    let source = extract_tool_input_map(input).cloned().or(fallback);
    let Some(source) = source else { return CodeEdit::default() };
    let mut edit = CodeEdit::default();
    for key in ["filePath", "path", "file_path", "file"] {
        if let Some(Value::String(raw)) = source.get(key) {
            if !raw.is_empty() {
                edit.path = raw.clone();
                break;
            }
        }
    }
    for key in ["oldString", "old", "old_string", "search"] {
        if let Some(Value::String(raw)) = source.get(key) {
            edit.old = raw.clone();
            break;
        }
    }
    for key in ["newString", "new", "new_string", "replace"] {
        if let Some(Value::String(raw)) = source.get(key) {
            edit.new = raw.clone();
            break;
        }
    }
    if let Some(Value::Bool(all)) = source.get("all") {
        edit.all = *all;
    } else if let Some(Value::Bool(all)) = source.get("replaceAll") {
        edit.all = *all;
    }
    edit
}

/// 🟫️ The terminal command: the payload's, or the raw tool argument string's `command` member.
pub fn extract_terminal_command(input: Option<&Value>, tool_args: &str) -> String {
    let command = extract_command_from_stdin(input);
    if !command.is_empty() {
        return command;
    }
    match parse_tool_args(tool_args).and_then(|args| args.get("command").cloned()) {
        Some(Value::String(command)) => command,
        _ => String::new(),
    }
}

/// 💠️ What a finished terminal invocation reports.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalOutcome {
    pub command: String,
    pub pid: String,
    pub terminated: bool,
    pub stdout: String,
    pub stderr: String,
}

/// 🖥️ Reads a finished terminal invocation out of the payload.
pub fn extract_terminal_outcome(input: Option<&Value>) -> TerminalOutcome {
    let Some(object) = decode_hook_input_map(input) else { return TerminalOutcome::default() };
    let mut outcome = TerminalOutcome::default();
    let tool_input = object.get("tool_input").and_then(Value::as_object).unwrap_or(object);
    for key in ["command", "command_line"] {
        if let Some(Value::String(raw)) = tool_input.get(key) {
            if !raw.is_empty() {
                outcome.command = raw.clone();
                break;
            }
        }
    }
    if let Some(tool_info) = object.get("tool_info").and_then(Value::as_object) {
        if let Some(Value::String(raw)) = tool_info.get("command_line") {
            if !raw.is_empty() {
                outcome.command = raw.clone();
            }
        }
    }
    for key in ["pid", "process_id", "execution_id", "id"] {
        match object.get(key) {
            Some(Value::String(raw)) if !raw.is_empty() => {
                outcome.pid = raw.clone();
                break;
            }
            Some(Value::Number(number)) => {
                outcome.pid = format!("{}", number.as_f64().unwrap_or_default() as i64);
                break;
            }
            _ => {}
        }
    }
    for key in ["terminated", "has_terminated", "exited", "stopped"] {
        if let Some(Value::Bool(flag)) = object.get(key) {
            outcome.terminated = *flag;
            break;
        }
    }
    for key in ["stdout", "output", "tool_output"] {
        if let Some(Value::String(raw)) = object.get(key) {
            outcome.stdout = raw.clone();
            break;
        }
    }
    for key in ["stderr", "error_output"] {
        if let Some(Value::String(raw)) = object.get(key) {
            outcome.stderr = raw.clone();
            break;
        }
    }
    outcome
}

/// 🔎️ Web pages and file line ranges a search event touched.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchReads {
    pub pages: Vec<String>,
    pub ranges: Vec<String>,
}

/// 🔭️ Reads the search coverage out of the payload, or out of the raw tool argument string.
pub fn extract_search_reads(input: Option<&Value>, tool_args: &str) -> SearchReads {
    let Some(source) = plan_source(input, tool_args) else { return SearchReads::default() };
    let mut reads = SearchReads::default();
    if let Some(Value::String(url)) = source.get("url") {
        if !url.is_empty() {
            reads.pages.push(url.clone());
        }
    }
    if let Some(Value::Array(pages)) = source.get("pages") {
        for page in pages {
            if let Value::String(page) = page {
                if page.starts_with("http://") || page.starts_with("https://") {
                    reads.pages.push(page.clone());
                }
            }
        }
    }
    let file_path = match (source.get("file_path"), source.get("filePath")) {
        (Some(Value::String(path)), _) => path.clone(),
        (_, Some(Value::String(path))) => path.clone(),
        _ => String::new(),
    };
    if file_path.is_empty() {
        return reads;
    }
    let number = |key: &str| source.get(key).and_then(Value::as_f64).unwrap_or_default() as i64;
    let start_line = number("startLine");
    let mut end_line = number("endLine");
    let limit = number("limit");
    let offset = if source.contains_key("offset") { number("offset") } else { 1 };
    if start_line > 0 {
        if end_line <= 0 {
            end_line = start_line;
        }
        if start_line == end_line {
            reads.ranges.push(format!("{file_path}#L{start_line}"));
        } else {
            reads.ranges.push(format!("{file_path}#L{start_line}-L{end_line}"));
        }
    } else if limit > 0 {
        let start = if offset > 0 { offset } else { 1 };
        reads.ranges.push(format!("{file_path}#L{start}-L{}", start + limit - 1));
    } else {
        reads.ranges.push(file_path);
    }
    reads
}

pub(super) fn parse_tool_args(tool_args: &str) -> Option<Map<String, Value>> {
    if tool_args.is_empty() {
        return None;
    }
    serde_json::from_str::<Value>(tool_args).ok().and_then(|value| value.as_object().cloned())
}
//#endregion 🧭️PayloadReading
