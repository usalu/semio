//! 🧭️ Resolution: command classification and the resolution of a shell command to the test files it would run.
//!
//! Holds the `🔎️Resolution` region of the crate root; the root re-exports its items.

use super::*;

//#region 🔎️Resolution
/// 🧰️ Go `pyTestRunnerBins`.
pub const PYTHON_TEST_RUNNER_BINS: [&str; 4] = ["pytest", "py.test", "nosetests", "nose2"];

/// 🔩️ Go `jsTestRunnerBins`.
pub const JS_TEST_RUNNER_BINS: [&str; 8] = ["jest", "vitest", "mocha", "jasmine", "ava", "tape", "qunit", "karma"];

/// 🏷️ Go `ToolKind`, restricted to the four verdicts `classifyCommandKind` can return.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CommandKind {
    /// 🔬️ A test command.
    Test,
    /// 🔍️ A read-only inspection command.
    CodeSearch,
    /// ✏️ A command that writes files.
    CodeEdit,
    /// 🖥️ Anything else.
    Terminal,
}

/// ✂️ Go `splitCommandSegments`: splits on `;`, `&&`, `|` and `||` outside quotes.
pub fn split_command_segments(command: &str) -> Vec<String> {
    let command = command.trim();
    if command.is_empty() {
        return Vec::new();
    }
    let mut segments: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let bytes: Vec<char> = command.chars().collect();
    let mut index = 0;
    while index < bytes.len() {
        let value = bytes[index];
        if let Some(open) = quote {
            current.push(value);
            if value == open {
                quote = None;
            }
            index += 1;
            continue;
        }
        if value == '\'' || value == '"' {
            quote = Some(value);
            current.push(value);
            index += 1;
            continue;
        }
        if value == ';' {
            flush_segment(&mut segments, &mut current);
            index += 1;
            continue;
        }
        if value == '&' && index + 1 < bytes.len() && bytes[index + 1] == '&' {
            flush_segment(&mut segments, &mut current);
            index += 2;
            continue;
        }
        if value == '|' {
            flush_segment(&mut segments, &mut current);
            index += if index + 1 < bytes.len() && bytes[index + 1] == '|' { 2 } else { 1 };
            continue;
        }
        current.push(value);
        index += 1;
    }
    flush_segment(&mut segments, &mut current);
    segments
}

fn flush_segment(segments: &mut Vec<String>, current: &mut String) {
    let segment = current.trim().to_string();
    if !segment.is_empty() {
        segments.push(segment);
    }
    current.clear();
}

/// 🗂️ Go `classifyCommandKind`, ported in full because the test-file resolver branches on it.
pub fn classify_command_kind(command: &str) -> CommandKind {
    let trimmed = command.trim();
    if trimmed.is_empty() {
        return CommandKind::Terminal;
    }
    let segments = split_command_segments(trimmed);
    if segments.len() > 1 {
        let mut best = CommandKind::Terminal;
        for segment in &segments {
            let kind = classify_command_kind(segment);
            if kind == CommandKind::Test || kind == CommandKind::CodeEdit {
                return kind;
            }
            if kind == CommandKind::CodeSearch {
                best = CommandKind::CodeSearch;
            }
        }
        if best != CommandKind::Terminal {
            return best;
        }
    }
    let fields: Vec<&str> = trimmed.split_whitespace().collect();
    let Some(first) = fields.first() else { return CommandKind::Terminal };
    let base = base_name(first);
    if base == "test" || base == "[" {
        return CommandKind::CodeSearch;
    }
    if base.starts_with("phpunit") || matches!(base.as_str(), "rspec" | "pytest" | "py.test" | "jest" | "vitest" | "mocha" | "gradle" | "mvn" | "bundle") {
        return CommandKind::Test;
    }
    if trimmed.contains("cargo nextest") {
        return CommandKind::Test;
    }
    for candidate in ["pytest", "jest", "mocha", "vitest", "rspec", "phpunit", "ctest", "bats"] {
        if trimmed.contains(candidate) {
            return CommandKind::Test;
        }
    }
    for prefix in [
        "npm test",
        "npm run test",
        "pnpm test",
        "yarn test",
        "bun test",
        "go test",
        "cargo test",
        "cargo nextest",
        "make test",
        "make check",
        "dotnet test",
        "swift test",
        "dart test",
        "flutter test",
        "mix test",
        "mvn test",
        "mvn verify",
        "gradle test",
        "./gradlew test",
        "gradlew test",
        "cabal test",
        "stack test",
        "lein test",
        "sbt test",
        "bundle exec rspec",
        "tox",
        "rspec ",
    ] {
        if trimmed.starts_with(prefix) {
            return CommandKind::Test;
        }
    }
    if trimmed.contains("pytest") || trimmed.contains("unittest") || trimmed.contains("phpunit") {
        return CommandKind::Test;
    }
    const SEARCH: [&str; 61] = [
        "grep", "rg", "ripgrep", "ag", "ack", "ack-grep", "find", "fd", "fdfind", "locate", "mlocate", "ls", "exa", "eza", "tree", "dir", "cat", "bat", "batcat", "less", "more", "head", "tail",
        "wc", "file", "stat", "du", "which", "whereis", "type", "command", "hash", "diff", "cmp", "comm", "strings", "od", "xxd", "hexdump", "readlink", "realpath", "basename", "dirname", "jq",
        "yq", "xq", "sort", "uniq", "cut", "tr", "paste", "column", "rev", "fold", "fmt", "nl", "expand", "unexpand", "echo", "printf", "env",
    ];
    if SEARCH.contains(&base.as_str()) {
        return CommandKind::CodeSearch;
    }
    const SEARCH_TAIL: [&str; 22] = [
        "printenv", "set", "export", "pwd", "id", "whoami", "hostname", "uname", "date", "uptime", "free", "df", "ps", "top", "htop", "lsof", "netstat", "ss", "test", "sed", "awk", "gawk",
    ];
    if SEARCH_TAIL.contains(&base.as_str()) && !((base == "sed" || base == "awk" || base == "gawk") && trimmed.contains("-i")) {
        return CommandKind::CodeSearch;
    }
    const EDIT: [&str; 26] = [
        "rm", "mv", "cp", "install", "mkdir", "rmdir", "touch", "chmod", "chown", "chgrp", "ln", "tee", "patch", "truncate", "dd", "shred", "tar", "zip", "unzip", "gzip", "gunzip", "bzip2",
        "bunzip2", "xz", "unxz", "zstd",
    ];
    if EDIT.contains(&base.as_str()) {
        return CommandKind::CodeEdit;
    }
    if (base == "sed" || base == "awk" || base == "gawk" || base == "mawk" || base == "nawk") && trimmed.contains("-i") {
        return CommandKind::CodeEdit;
    }
    CommandKind::Terminal
}

/// 🧲️ Go `extractTestSegmentFromCommand`: the test segment and the `cd` that precedes it.
pub fn extract_test_segment_from_command(command: &str) -> (String, String) {
    let command = command.trim();
    if command.is_empty() {
        return (String::new(), String::new());
    }
    let mut segments = split_command_segments(command);
    if segments.is_empty() {
        segments = vec![command.to_string()];
    }
    let has_composite = segments.len() > 1;
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
        let bin = base_name(first);
        if is_test_command_segment(segment, &bin) {
            return (trim_pipeline_tail(segment), cwd);
        }
        if matches!(bin.as_str(), "npx" | "pnpm" | "yarn" | "bun" | "uv") && is_test_command_segment(segment, "") {
            return (trim_pipeline_tail(segment), cwd);
        }
    }
    if has_composite {
        (String::new(), cwd)
    } else {
        (String::new(), String::new())
    }
}

/// 🧪️ Go `isTestCommandSegment`.
pub fn is_test_command_segment(segment: &str, bin: &str) -> bool {
    let trimmed = segment.trim();
    if trimmed.is_empty() {
        return false;
    }
    let fields: Vec<&str> = trimmed.split_whitespace().collect();
    let bin = if bin.is_empty() { fields.first().map(|value| base_name(value)).unwrap_or_default() } else { bin.to_string() };
    if bin.is_empty() {
        return false;
    }
    if classify_command_kind(trimmed) == CommandKind::Test {
        return true;
    }
    if fields.len() < 2 {
        return false;
    }
    if matches!(bin.as_str(), "npx" | "pnpm" | "yarn" | "bun") && classify_command_kind(&fields[1..].join(" ")) == CommandKind::Test {
        return true;
    }
    if bin == "uv" {
        if fields[1] == "run" && fields.len() > 2 && classify_command_kind(&fields[2..].join(" ")) == CommandKind::Test {
            return true;
        }
        if classify_command_kind(&fields[1..].join(" ")) == CommandKind::Test {
            return true;
        }
    }
    false
}

/// 🚂️ Go `trimPipelineTail`: everything up to the first unquoted `|`.
pub fn trim_pipeline_tail(segment: &str) -> String {
    let trimmed = segment.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let mut current = String::new();
    let mut quote: Option<char> = None;
    for value in trimmed.chars() {
        if let Some(open) = quote {
            current.push(value);
            if value == open {
                quote = None;
            }
            continue;
        }
        if value == '\'' || value == '"' {
            quote = Some(value);
            current.push(value);
            continue;
        }
        if value == '|' {
            break;
        }
        current.push(value);
    }
    current.trim().to_string()
}

/// 🔎️ Go `resolveTestFilesFromCommand`: the test files a shell command would exercise.
pub fn resolve_test_files_from_command(snapshot: &FilesystemSnapshot, command: &str, cwd: &str) -> Vec<String> {
    let command = command.trim();
    if command.is_empty() {
        return Vec::new();
    }
    let mut cwd = if cwd.is_empty() { snapshot.root.clone() } else { cwd.to_string() };
    let (segment, extracted_cwd) = extract_test_segment_from_command(command);
    let command = if segment.is_empty() { command.to_string() } else { segment };
    if !extracted_cwd.is_empty() {
        cwd = if is_absolute_path(&extracted_cwd) { clean_path(&extracted_cwd) } else { join_path(&cwd, &extracted_cwd) };
    }
    let parts: Vec<String> = command.split_whitespace().map(str::to_string).collect();
    let Some(first) = parts.first() else { return Vec::new() };
    let bin = base_name(first);
    match bin.as_str() {
        "go" => resolve_go_test_files(snapshot, &parts, &cwd),
        "cargo" => resolve_cargo_test_files(snapshot, &cwd),
        "dotnet" => resolve_dotnet_test_files(snapshot, &cwd),
        "python" | "python3" => resolve_python_test_files(snapshot, &cwd),
        "pytest" | "py.test" => resolve_pytest_files(snapshot, &[], &cwd),
        "uv" => {
            if parts.len() > 2 && parts[1] == "run" {
                let runner = base_name(&parts[2]);
                if runner == "pytest" || PYTHON_TEST_RUNNER_BINS.contains(&runner.as_str()) {
                    return resolve_pytest_files(snapshot, &[], &cwd);
                }
            }
            Vec::new()
        }
        "uvx" => {
            if parts.len() > 1 && PYTHON_TEST_RUNNER_BINS.contains(&base_name(&parts[1]).as_str()) {
                return resolve_pytest_files(snapshot, &[], &cwd);
            }
            Vec::new()
        }
        "rspec" => resolve_rspec_files(snapshot, &cwd),
        "npm" | "pnpm" | "yarn" | "jest" | "vitest" | "mocha" | "jasmine" | "ava" => find_js_test_files(snapshot, &cwd),
        "npx" | "bunx" | "pnpx" => {
            if parts.len() > 1 && JS_TEST_RUNNER_BINS.contains(&base_name(&parts[1]).as_str()) {
                return find_js_test_files(snapshot, &cwd);
            }
            Vec::new()
        }
        "bun" => {
            if parts.len() > 1 && parts[1] == "test" {
                return find_js_test_files(snapshot, &cwd);
            }
            Vec::new()
        }
        "bundle" => {
            if parts.len() > 2 && parts[1] == "exec" && base_name(&parts[2]) == "rspec" {
                return resolve_rspec_files(snapshot, &cwd);
            }
            Vec::new()
        }
        _ => Vec::new(),
    }
}

/// 🐹️ Go `resolveGoTestFiles`: package targets after `go test`, `/...` meaning recursive.
pub fn resolve_go_test_files(snapshot: &FilesystemSnapshot, args: &[String], cwd: &str) -> Vec<String> {
    let targets: Vec<&String> = args.iter().skip(2).filter(|arg| !arg.trim().is_empty() && !arg.starts_with('-')).collect();
    let mut files: Vec<String> = Vec::new();
    let push = |found: Vec<String>, files: &mut Vec<String>| {
        for path in found {
            if path.ends_with("_test.go") && !files.contains(&path) {
                files.push(path);
            }
        }
    };
    if targets.is_empty() {
        push(snapshot.walk_files(cwd, true), &mut files);
        return files;
    }
    for target in targets {
        let recursive = target.ends_with("/...");
        let stripped = target.strip_suffix("/...").unwrap_or(target);
        let target_path = if stripped == "." || stripped == "./" || stripped.is_empty() {
            cwd.to_string()
        } else if is_absolute_path(stripped) {
            clean_path(stripped)
        } else {
            join_path(cwd, stripped)
        };
        if snapshot.directory_exists(&target_path) {
            push(snapshot.walk_files(&target_path, recursive), &mut files);
        } else if snapshot.file_exists(&target_path) && target_path.ends_with("_test.go") && !files.contains(&target_path) {
            files.push(target_path);
        }
    }
    files
}

/// 🦀️ Go `resolveCargoTestFiles`: `.rs` files carrying a test attribute.
pub fn resolve_cargo_test_files(snapshot: &FilesystemSnapshot, cwd: &str) -> Vec<String> {
    snapshot
        .walk_files(cwd, true)
        .into_iter()
        .filter(|path| path.ends_with(".rs"))
        .filter(|path| snapshot.read_text(path).is_some_and(|text| text.contains("#[test") || text.contains("#[cfg(test)")))
        .collect()
}

/// 🔷️ Go `resolveDotnetTestFiles`: `.cs` files carrying a test attribute.
pub fn resolve_dotnet_test_files(snapshot: &FilesystemSnapshot, cwd: &str) -> Vec<String> {
    snapshot
        .walk_files(cwd, true)
        .into_iter()
        .filter(|path| path.ends_with(".cs"))
        .filter(|path| snapshot.read_text(path).is_some_and(|text| text.contains("[Test") || text.contains("[Fact") || text.contains("[TestMethod")))
        .collect()
}

/// 🐍️ Go `resolvePythonTestFiles`: `test_*.py` and `*_test.py`.
pub fn resolve_python_test_files(snapshot: &FilesystemSnapshot, cwd: &str) -> Vec<String> {
    snapshot
        .walk_files(cwd, true)
        .into_iter()
        .filter(|path| path.ends_with(".py"))
        .filter(|path| {
            let base = base_name(path);
            base.starts_with("test_") || base.contains("_test.")
        })
        .collect()
}

/// 🗃️ Go `resolvePytestFiles`: pytest naming plus `conftest.py`, honouring positional targets.
pub fn resolve_pytest_files(snapshot: &FilesystemSnapshot, args: &[String], cwd: &str) -> Vec<String> {
    let is_pytest_file = |path: &str| {
        let base = base_name(path);
        base.starts_with("test_") || base.contains("_test.") || base.eq_ignore_ascii_case("conftest.py")
    };
    let mut targets: Vec<String> = Vec::new();
    let mut index = 0;
    while index < args.len() {
        let arg = args[index].trim();
        if arg.is_empty() {
            index += 1;
            continue;
        }
        if arg.starts_with('-') {
            if matches!(arg, "-k" | "-m" | "--maxfail" | "-c") && index + 1 < args.len() {
                index += 1;
            }
            index += 1;
            continue;
        }
        targets.push(arg.to_string());
        index += 1;
    }
    let mut files: Vec<String> = Vec::new();
    let scan = |root: &str, files: &mut Vec<String>| {
        for path in snapshot.walk_files(root, true) {
            if path.ends_with(".py") && is_pytest_file(&path) && !files.contains(&path) {
                files.push(path);
            }
        }
    };
    if targets.is_empty() {
        scan(cwd, &mut files);
        return files;
    }
    for target in targets {
        let target_path = if is_absolute_path(&target) { clean_path(&target) } else { join_path(cwd, &target) };
        if snapshot.directory_exists(&target_path) {
            scan(&target_path, &mut files);
        } else if snapshot.file_exists(&target_path) && target_path.ends_with(".py") && is_pytest_file(&target_path) && !files.contains(&target_path) {
            files.push(target_path);
        }
    }
    files
}

/// 💎️ Go `resolveRspecFiles`: `_spec.rb` below the working directory.
pub fn resolve_rspec_files(snapshot: &FilesystemSnapshot, cwd: &str) -> Vec<String> {
    snapshot.walk_files(cwd, true).into_iter().filter(|path| path.ends_with("_spec.rb")).collect()
}

/// 🟦️ Go `findJSTestFiles`: `test.*`, `*.test.{js,ts}`, `*.spec.{js,ts}`.
pub fn find_js_test_files(snapshot: &FilesystemSnapshot, cwd: &str) -> Vec<String> {
    snapshot
        .walk_files(cwd, true)
        .into_iter()
        .filter(|path| path.ends_with(".js") || path.ends_with(".ts") || path.ends_with(".jsx") || path.ends_with(".tsx"))
        .filter(|path| {
            let base = base_name(path);
            base.starts_with("test.") || base.ends_with(".test.js") || base.ends_with(".test.ts") || base.ends_with(".spec.js") || base.ends_with(".spec.ts")
        })
        .collect()
}
//#endregion 🔎️Resolution
