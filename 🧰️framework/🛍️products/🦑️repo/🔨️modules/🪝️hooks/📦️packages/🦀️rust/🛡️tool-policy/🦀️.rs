//! 🛡️ Tool policy: tool classification, command segmentation and the tool blocking policy.
//!
//! Holds the `🏛️ToolClassification`, `🛡️BlockingPolicy` regions of the crate root; the root re-exports its items.

use super::*;

//#region 🏛️ToolClassification
/// 🏛️ The tool kind of a named tool, across every IDE's tool vocabulary.
pub fn classify_tool(tool_name: &str) -> ToolKind {
    match tool_name {
        "manage_todo_list" | "Task" | "task" | "todo_tool" | "TodoWrite" => ToolKind::Plan,
        "read_file" | "grep_search" | "rg" | "ripgrep" | "file_search" | "semantic_search" | "list_dir" | "list_code_usages" | "get_errors" | "Read" | "fetch_webpage" | "open_simple_browser" | "Grep" | "Glob" | "fs_read" | "code" | "grep" | "glob" | "web_search" | "web_fetch" => ToolKind::CodeSearch,
        "replace_string_in_file" | "create_file" | "multi_replace_string_in_file" | "Edit" | "Write" | "editfile" | "fs_write" => ToolKind::CodeEdit,
        "run_in_terminal" | "get_terminal_output" | "Bash" | "terminal" | "execute_bash" => ToolKind::Terminal,
        "runTests" | "run_tests" => ToolKind::Test,
        "run_task" | "create_and_run_task" => ToolKind::Build,
        _ => ToolKind::Generic,
    }
}

const TEST_COMMAND_NAMES: [&str; 8] = ["pytest", "jest", "mocha", "vitest", "rspec", "phpunit", "ctest", "bats"];

const SEARCH_COMMAND_NAMES: [&str; 68] = [
    "grep", "rg", "ripgrep", "ag", "ack", "ack-grep", "find", "fd", "fdfind", "locate", "mlocate", "ls", "exa", "eza", "tree", "dir", "cat", "bat", "batcat", "less", "more", "head", "tail", "wc", "file", "stat", "du", "which", "whereis", "type", "command", "hash", "diff", "cmp", "comm", "strings", "od", "xxd", "hexdump", "readlink", "realpath", "basename", "dirname", "jq", "yq", "xq", "sort", "uniq", "cut", "tr", "paste", "column", "rev", "fold", "fmt", "nl", "expand", "unexpand", "echo", "printf", "env", "printenv", "set", "export", "pwd", "id", "whoami", "hostname",
];

const SEARCH_COMMAND_NAMES_TAIL: [&str; 15] = ["uname", "date", "uptime", "free", "df", "ps", "top", "htop", "lsof", "netstat", "ss", "test", "python-none", "node-none", "unused"];

const EDIT_COMMAND_NAMES: [&str; 27] = ["rm", "mv", "cp", "install", "mkdir", "rmdir", "touch", "chmod", "chown", "chgrp", "ln", "tee", "patch", "truncate", "dd", "shred", "tar", "zip", "unzip", "gzip", "gunzip", "bzip2", "bunzip2", "xz", "unxz", "zstd", "unused"];

/// 🔤️ The tool kind a raw shell command implies.
pub fn classify_command_kind(command: &str) -> ToolKind {
    let trimmed = command.trim();
    if trimmed.is_empty() {
        return ToolKind::Terminal;
    }
    let segments = split_command_segments(trimmed);
    if segments.len() > 1 {
        let mut best = ToolKind::Terminal;
        for segment in &segments {
            let kind = classify_command_kind(segment);
            if matches!(kind, ToolKind::Test | ToolKind::Build | ToolKind::CodeEdit | ToolKind::Plan) {
                return kind;
            }
            if kind == ToolKind::CodeSearch {
                best = ToolKind::CodeSearch;
            }
        }
        if best != ToolKind::Terminal {
            return best;
        }
    }
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    let Some(first) = parts.first() else { return ToolKind::Terminal };
    let base = base_name(first);
    if base == "test" || base == "[" {
        return ToolKind::CodeSearch;
    }
    if base.starts_with("phpunit") || matches!(base.as_str(), "rspec" | "pytest" | "py.test" | "jest" | "vitest" | "mocha" | "gradle" | "mvn" | "bundle") {
        return ToolKind::Test;
    }
    if trimmed.contains("cargo nextest") {
        return ToolKind::Test;
    }
    if TEST_COMMAND_NAMES.iter().any(|name| trimmed.contains(name)) {
        return ToolKind::Test;
    }
    const TEST_PREFIXES: [&str; 24] = [
        "npm test", "npm run test", "pnpm test", "yarn test", "bun test", "go test", "cargo test", "cargo nextest", "make test", "make check", "dotnet test", "swift test", "dart test", "flutter test", "mix test", "mvn test", "mvn verify", "gradle test", "./gradlew test", "gradlew test", "cabal test", "stack test", "lein test", "sbt test",
    ];
    if TEST_PREFIXES.iter().any(|prefix| trimmed.starts_with(prefix)) {
        return ToolKind::Test;
    }
    if trimmed.contains("pytest") || trimmed.contains("unittest") || trimmed.contains("phpunit") {
        return ToolKind::Test;
    }
    if trimmed.starts_with("bundle exec rspec") || trimmed.starts_with("tox") || trimmed.starts_with("rspec ") {
        return ToolKind::Test;
    }
    if SEARCH_COMMAND_NAMES.contains(&base.as_str()) || SEARCH_COMMAND_NAMES_TAIL.contains(&base.as_str()) {
        return ToolKind::CodeSearch;
    }
    let stream_editor = base == "sed";
    let awk_family = matches!(base.as_str(), "awk" | "gawk" | "mawk" | "nawk");
    if (stream_editor || awk_family) && !trimmed.contains("-i") {
        return ToolKind::CodeSearch;
    }
    if EDIT_COMMAND_NAMES.contains(&base.as_str()) {
        return ToolKind::CodeEdit;
    }
    if (stream_editor || awk_family) && trimmed.contains("-i") {
        return ToolKind::CodeEdit;
    }
    ToolKind::Terminal
}

/// ✂️ Cuts a command at its first unquoted pipe.
pub fn trim_pipeline_tail(segment: &str) -> String {
    let trimmed = segment.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let mut current = String::new();
    let mut quote = '\0';
    for character in trimmed.chars() {
        if quote != '\0' {
            current.push(character);
            if character == quote {
                quote = '\0';
            }
            continue;
        }
        if character == '\'' || character == '"' {
            quote = character;
            current.push(character);
            continue;
        }
        if character == '|' {
            break;
        }
        current.push(character);
    }
    current.trim().to_string()
}

fn is_test_command_segment(segment: &str, binary: &str) -> bool {
    let trimmed = segment.trim();
    if trimmed.is_empty() {
        return false;
    }
    let fields: Vec<&str> = trimmed.split_whitespace().collect();
    let binary = if binary.is_empty() {
        match fields.first() {
            Some(first) => base_name(first),
            None => return false,
        }
    } else {
        binary.to_string()
    };
    if classify_command_kind(trimmed) == ToolKind::Test {
        return true;
    }
    if fields.len() < 2 {
        return false;
    }
    if matches!(binary.as_str(), "npx" | "pnpm" | "yarn" | "bun") && classify_command_kind(&fields[1..].join(" ")) == ToolKind::Test {
        return true;
    }
    if binary == "uv" {
        if fields[1] == "run" && fields.len() > 2 && classify_command_kind(&fields[2..].join(" ")) == ToolKind::Test {
            return true;
        }
        if classify_command_kind(&fields[1..].join(" ")) == ToolKind::Test {
            return true;
        }
    }
    false
}

/// 🧪️ The test-running segment of a composite command, with the directory a leading `cd` selected.
pub fn extract_test_segment(command: &str) -> (String, String) {
    let command = command.trim();
    if command.is_empty() {
        return (String::new(), String::new());
    }
    let mut segments = split_command_segments(command);
    if segments.is_empty() {
        segments = vec![command.to_string()];
    }
    let composite = segments.len() > 1;
    let mut cwd = String::new();
    for segment in &segments {
        let segment = segment.trim();
        if segment.is_empty() {
            continue;
        }
        if let Some(rest) = segment.strip_prefix("cd ") {
            if let Some(target) = rest.split_whitespace().next() {
                cwd = target.to_string();
            }
            continue;
        }
        if segment.starts_with("export ") || segment.starts_with("set ") {
            continue;
        }
        let Some(first) = segment.split_whitespace().next() else { continue };
        let binary = base_name(first);
        if is_test_command_segment(segment, &binary) {
            return (trim_pipeline_tail(segment), cwd);
        }
        if matches!(binary.as_str(), "npx" | "pnpm" | "yarn" | "bun" | "uv") && is_test_command_segment(segment, "") {
            return (trim_pipeline_tail(segment), cwd);
        }
    }
    if composite {
        return (String::new(), cwd);
    }
    (String::new(), String::new())
}

/// 🔶️ Normalises a native or neutral event name into a neutral [`HookEvent`] plus a parent hint.
///
/// A name that is already a neutral slug wins outright. Otherwise the tool name and the payload's
/// command decide the tool kind, and the calling IDE's own table (from `🧩️providers`) resolves the
/// native name — an unknown name is refused, never silently defaulted.
pub fn resolve_hook_event(name: &str, client: &str, tool_name: &str, input: Option<&Value>) -> HookOutcome<(HookEvent, String)> {
    if let Ok(event) = validate_hook_event(name) {
        return Ok((event, String::new()));
    }
    let mut kind = ToolKind::Generic;
    if !tool_name.trim().is_empty() {
        kind = classify_tool(tool_name);
    }
    let command = extract_command_from_stdin(input);
    if !command.is_empty() {
        let (segment, _) = extract_test_segment(&command);
        let candidate = if segment.trim().is_empty() { command.as_str() } else { segment.as_str() };
        let command_kind = classify_command_kind(candidate);
        if matches!(kind, ToolKind::Generic | ToolKind::Terminal) || matches!(command_kind, ToolKind::Test | ToolKind::Build | ToolKind::CodeSearch | ToolKind::CodeEdit) {
            kind = command_kind;
        }
    }
    if tool_name == "Glob" && input.is_some() {
        kind = ToolKind::Generic;
    }
    match get_editor_provider(client) {
        Some(provider) => provider.resolve_native_event(name, kind).map_err(|error| HookError::new(error.message)),
        None => resolve_claude_compatible_event(name, kind).map_err(|error| HookError::new(error.message)),
    }
}
//#endregion 🏛️ToolClassification

//#region 🛡️BlockingPolicy
/// 🧩️ Tool names that are refused outright, whatever their arguments.
pub const BLOCKED_TOOL_PATTERNS: [&str; 27] = [
    "rm", "rmdir", "mv", "cp", "git add", "git branch", "git checkout", "git cherry-pick", "git clean", "git clone", "git commit", "git config", "git fetch", "git init", "git merge", "git mv", "git pull", "git push", "git rebase", "git remote", "git reset", "git restore", "git revert", "git rm", "git stash", "git switch", "git tag",
];

/// ⌨️ The git subcommands that modify repository state and are therefore refused.
pub const BLOCKED_GIT_VERBS: [&str; 23] = ["add", "branch", "checkout", "cherry-pick", "clone", "commit", "config", "fetch", "init", "merge", "mv", "pull", "push", "rebase", "remote", "reset", "restore", "revert", "rm", "stash", "switch", "tag", "clean"];

const CONCURRENCY_NOTE: &str = "; other developers and agents may be editing the same files concurrently";

/// 🔷️ Splits a command at every unquoted `;`, `&&`, `|` and `||`.
pub fn split_command_segments(command: &str) -> Vec<String> {
    let command = command.trim();
    if command.is_empty() {
        return Vec::new();
    }
    let characters: Vec<char> = command.chars().collect();
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut quote = '\0';
    let mut index = 0;
    while index < characters.len() {
        let character = characters[index];
        if quote != '\0' {
            current.push(character);
            if character == quote {
                quote = '\0';
            }
            index += 1;
            continue;
        }
        if character == '\'' || character == '"' {
            quote = character;
            current.push(character);
            index += 1;
            continue;
        }
        if character == ';' {
            push_segment(&mut segments, &mut current);
            index += 1;
            continue;
        }
        if character == '&' && index + 1 < characters.len() && characters[index + 1] == '&' {
            push_segment(&mut segments, &mut current);
            index += 2;
            continue;
        }
        if character == '|' {
            push_segment(&mut segments, &mut current);
            index += if index + 1 < characters.len() && characters[index + 1] == '|' { 2 } else { 1 };
            continue;
        }
        current.push(character);
        index += 1;
    }
    push_segment(&mut segments, &mut current);
    segments
}

fn push_segment(segments: &mut Vec<String>, current: &mut String) {
    let segment = current.trim().to_string();
    if !segment.is_empty() {
        segments.push(segment);
    }
    current.clear();
}

fn is_word_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}

fn at_word_boundary(characters: &[char], start: usize, end: usize) -> bool {
    let before = start == 0 || !is_word_character(characters[start - 1]);
    let after = end >= characters.len() || !is_word_character(characters[end]);
    before && after
}

/// 🌿️ The first blocked `git <verb>` invocation in arbitrary inline code, matching the Go
/// `\bgit\s+(verb)\b` pattern case-insensitively and returning the matched text.
fn find_inline_git_verb(code: &str) -> Option<String> {
    let characters: Vec<char> = code.chars().collect();
    for start in 0..characters.len() {
        if !(characters[start].eq_ignore_ascii_case(&'g') && start + 3 <= characters.len()) {
            continue;
        }
        let word: String = characters[start..(start + 3).min(characters.len())].iter().collect();
        if !word.eq_ignore_ascii_case("git") {
            continue;
        }
        if start > 0 && is_word_character(characters[start - 1]) {
            continue;
        }
        let mut cursor = start + 3;
        let space_start = cursor;
        while cursor < characters.len() && characters[cursor].is_whitespace() {
            cursor += 1;
        }
        if cursor == space_start {
            continue;
        }
        for verb in BLOCKED_GIT_VERBS {
            let verb_characters: Vec<char> = verb.chars().collect();
            let end = cursor + verb_characters.len();
            if end > characters.len() {
                continue;
            }
            let candidate: String = characters[cursor..end].iter().collect();
            if !candidate.eq_ignore_ascii_case(verb) {
                continue;
            }
            if end < characters.len() && is_word_character(characters[end]) {
                continue;
            }
            return Some(characters[start..end].iter().collect());
        }
    }
    None
}

/// 📚️ Whether inline code carries a list-form git invocation such as `['git', 'stash']`.
fn contains_list_form_git(code: &str) -> bool {
    let characters: Vec<char> = code.chars().collect();
    let is_quote = |index: usize| index < characters.len() && (characters[index] == '\'' || characters[index] == '"');
    let skip_spaces = |mut index: usize| {
        while index < characters.len() && characters[index].is_whitespace() {
            index += 1;
        }
        index
    };
    for start in 0..characters.len() {
        if !is_quote(start) {
            continue;
        }
        let mut cursor = skip_spaces(start + 1);
        if cursor + 3 > characters.len() {
            continue;
        }
        let word: String = characters[cursor..cursor + 3].iter().collect();
        if !word.eq_ignore_ascii_case("git") {
            continue;
        }
        cursor = skip_spaces(cursor + 3);
        if !is_quote(cursor) {
            continue;
        }
        cursor = skip_spaces(cursor + 1);
        if cursor >= characters.len() || characters[cursor] != ',' {
            continue;
        }
        cursor = skip_spaces(cursor + 1);
        if !is_quote(cursor) {
            continue;
        }
        cursor = skip_spaces(cursor + 1);
        for verb in BLOCKED_GIT_VERBS {
            let verb_characters: Vec<char> = verb.chars().collect();
            let end = cursor + verb_characters.len();
            if end > characters.len() {
                continue;
            }
            let candidate: String = characters[cursor..end].iter().collect();
            if !candidate.eq_ignore_ascii_case(verb) {
                continue;
            }
            let closing = skip_spaces(end);
            if is_quote(closing) {
                return true;
            }
        }
    }
    false
}

/// 🌿️ Scans arbitrary inline code (`python -c`, `node -e`, …) for a blocked git invocation.
pub fn contains_blocked_git_in_code(code: &str) -> Option<String> {
    if let Some(matched) = find_inline_git_verb(code) {
        return Some(format!("blocked: {}", matched.trim().to_lowercase()));
    }
    if contains_list_form_git(code) {
        return Some("blocked: git (list form) in inline code".to_string());
    }
    None
}

/// 🎯️ Whether a lowercased command kills a process selected by `lsof -t -i:PORT`, which in a
/// container can match PID 1 and terminate the whole devcontainer.
fn matches_kill_lsof_port(lower: &str) -> bool {
    let characters: Vec<char> = lower.chars().collect();
    let find_word = |needle: &str, from: usize| -> Option<usize> {
        let needle_characters: Vec<char> = needle.chars().collect();
        (from..characters.len()).find(|&start| {
            let end = start + needle_characters.len();
            end <= characters.len() && characters[start..end] == needle_characters[..] && at_word_boundary(&characters, start, end)
        })
    };
    let find_literal = |needle: &str, from: usize| -> Option<usize> {
        let needle_characters: Vec<char> = needle.chars().collect();
        (from..characters.len()).find(|&start| {
            let end = start + needle_characters.len();
            end <= characters.len() && characters[start..end] == needle_characters[..]
        })
    };
    let Some(kill) = find_word("kill", 0) else { return false };
    let Some(open) = find_literal("$(", kill + 4) else { return false };
    let mut cursor = open + 2;
    while cursor < characters.len() && characters[cursor].is_whitespace() {
        cursor += 1;
    }
    let Some(lsof) = find_word("lsof", cursor) else { return false };
    if lsof != cursor {
        return false;
    }
    let Some(dash_t) = find_literal("-t", lsof + 4) else { return false };
    let Some(dash_i) = find_literal("-i", dash_t + 2) else { return false };
    let mut cursor = dash_i + 2;
    while cursor < characters.len() && characters[cursor].is_whitespace() {
        cursor += 1;
    }
    if cursor >= characters.len() || characters[cursor] != ':' {
        return false;
    }
    cursor += 1;
    while cursor < characters.len() && characters[cursor].is_whitespace() {
        cursor += 1;
    }
    let digits_start = cursor;
    while cursor < characters.len() && characters[cursor].is_ascii_digit() {
        cursor += 1;
    }
    if cursor == digits_start {
        return false;
    }
    find_literal(")", cursor).is_some()
}

const SHELL_INTERPRETERS: [&str; 8] = ["bash", "sh", "zsh", "fish", "ksh", "csh", "tcsh", "dash"];
const SCRIPT_INTERPRETERS: [&str; 13] = ["python", "python3", "python2", "node", "nodejs", "perl", "ruby", "php", "lua", "tclsh", "groovy", "scala", "unused"];
const ALLOWED_SEGMENT_PREFIXES: [&str; 10] = ["grep ", "rg ", "ripgrep ", "echo ", "printf ", "ls ", "pwd", "cat ", "sed ", "awk "];

/// 🐙️ The reason one command segment is refused, or `None` when it is allowed.
pub fn is_command_segment_blocked(segment: &str) -> Option<String> {
    let segment = segment.trim();
    if segment.is_empty() {
        return None;
    }
    let lower = segment.to_lowercase();
    if matches_kill_lsof_port(&lower) {
        return Some("blocked: kill $(lsof -t -i:PORT); this can match PID 1 in containers and terminate the devcontainer, stopping all running work".to_string());
    }
    if ALLOWED_SEGMENT_PREFIXES.iter().any(|prefix| lower.starts_with(prefix)) {
        return None;
    }
    let mut tokens: Vec<String> = lower.split_whitespace().map(str::to_string).collect();
    while let Some(first) = tokens.first().cloned() {
        if first.contains('=') || matches!(first.as_str(), "env" | "command" | "sudo") {
            tokens.remove(0);
            continue;
        }
        if SHELL_INTERPRETERS.contains(&first.as_str()) || first == "xargs" {
            let joined = tokens.join(" ");
            return match joined.find("git ") {
                Some(index) => is_command_segment_blocked(&joined[index..]),
                None => None,
            };
        }
        if SCRIPT_INTERPRETERS.contains(&first.as_str()) {
            return contains_blocked_git_in_code(&tokens[1..].join(" "));
        }
        break;
    }
    let first = tokens.first_mut()?;
    if first.ends_with("/git") {
        *first = "git".to_string();
    }
    if tokens[0] != "git" {
        return None;
    }
    let mut verb_index = 1;
    while verb_index < tokens.len() && tokens[verb_index].starts_with('-') {
        verb_index += 1;
        if verb_index < tokens.len() && !tokens[verb_index].starts_with('-') && (tokens[verb_index - 1] == "-c" || tokens[verb_index - 1] == "-C") {
            verb_index += 1;
        }
    }
    if verb_index >= tokens.len() {
        return None;
    }
    let verb = tokens[verb_index].trim_matches(|character| character == '"' || character == '\'');
    if !BLOCKED_GIT_VERBS.contains(&verb) {
        return None;
    }
    if verb == "clean" && !lower.contains("-fd") && !lower.contains("-df") {
        return None;
    }
    Some(format!("blocked: git {verb}"))
}

/// ✔️ Whether an invocation is refused, and why. The arguments are examined segment by segment
/// first, so a blocked git call hidden behind `&&`, a pipe, a shell or an inline script is caught
/// before the tool name is even considered.
pub fn is_tool_blocked(tool: &str, args: &str) -> Option<String> {
    for segment in split_command_segments(args) {
        if let Some(reason) = is_command_segment_blocked(&segment) {
            return Some(format!("{reason}{CONCURRENCY_NOTE}"));
        }
    }
    let tool = tool.trim().to_lowercase();
    if BLOCKED_TOOL_PATTERNS.contains(&tool.as_str()) {
        return Some(format!("blocked: {tool}{CONCURRENCY_NOTE}"));
    }
    None
}
//#endregion 🛡️BlockingPolicy
