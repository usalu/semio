//! 🧩️ Planning: the pure translation of a test scope into an ordered invocation plan.
//!
//! Holds the `🗺️Planning` region of the crate root; the root re-exports its items.

use super::*;

//#region 🗺️Planning
/// 🏭️ The program a planned invocation runs.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum Runner {
    /// 🐹️ `go test`.
    Go,
    /// 🦀️ `cargo test`.
    Cargo,
    /// ▶️ `cargo nextest run`.
    CargoNextest,
    /// 🔷️ `dotnet test`.
    Dotnet,
    /// 🟦️ `npx <js runner>`.
    Npx,
    /// 📦️ `npm test`.
    Npm,
    /// 🐍️ `uv run pytest`.
    Uv,
    /// 🧫️ `pytest`.
    Pytest,
    /// 💎️ `rspec`.
    Rspec,
}

impl Runner {
    /// 🏷️ The executable name.
    pub fn program(self) -> &'static str {
        match self {
            Self::Go => "go",
            Self::Cargo | Self::CargoNextest => "cargo",
            Self::Dotnet => "dotnet",
            Self::Npx => "npx",
            Self::Npm => "npm",
            Self::Uv => "uv",
            Self::Pytest => "pytest",
            Self::Rspec => "rspec",
        }
    }
}

/// 🎬️ One process the plan will run, fully determined before anything executes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RunnerInvocation {
    /// 🏃️ Which runner.
    pub runner: Runner,
    /// 📜️ Full argv, program name first.
    pub argv: Vec<String>,
    /// 📁️ Working directory, absolute.
    pub cwd: String,
    /// 🌱️ Environment overlay, sorted by key so a plan is a stable golden.
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    /// 🎯️ The test filter this invocation carries, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}

/// 🗺️ An ordered plan plus the refusals Go reports as errors.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct InvocationPlan {
    /// ⚙️ Invocations in execution order.
    #[serde(default)]
    pub invocations: Vec<RunnerInvocation>,
    /// 🚫️ One message per scope the planner refused, in Go's wording.
    #[serde(default)]
    pub problems: Vec<String>,
}

impl InvocationPlan {
    /// ➕️ Folds another plan in, preserving order.
    pub fn extend(&mut self, other: Self) {
        self.invocations.extend(other.invocations);
        self.problems.extend(other.problems);
    }
}

/// 🗓️ Plans every scope in order — Go's `testCommand` loop over `runTestScope`.
pub fn plan_scopes(snapshot: &FilesystemSnapshot, scopes: &[TestScope]) -> InvocationPlan {
    let mut plan = InvocationPlan::default();
    for scope in scopes {
        plan.extend(plan_scope(snapshot, scope));
    }
    plan
}

/// ☎️ Go `runTestScope`, with the process call replaced by a recorded invocation.
pub fn plan_scope(snapshot: &FilesystemSnapshot, scope: &TestScope) -> InvocationPlan {
    match scope.kind {
        ScopeKind::All => plan_all(snapshot),
        ScopeKind::Technology => plan_technology(snapshot, &scope.bundle_root),
        ScopeKind::Bundle => plan_bundle(snapshot, &scope.bundle_root, &scope.language, "", ""),
        ScopeKind::File => plan_file(snapshot, &scope.file_path, &scope.language, ""),
        ScopeKind::Section => plan_section(snapshot, &scope.file_path, &scope.language, &scope.section),
        ScopeKind::Definition => plan_definition(snapshot, &scope.file_path, &scope.language, &scope.bundle_root, &scope.test_name),
    }
}

/// 🔹️ Go `runAllTests`: every bundle whose language is detectable, bundle table order.
pub fn plan_all(snapshot: &FilesystemSnapshot) -> InvocationPlan {
    let mut plan = InvocationPlan::default();
    for bundle in &snapshot.bundles {
        let language = detect_bundle_language(snapshot, &bundle.root);
        if language.is_empty() {
            continue;
        }
        plan.extend(plan_bundle(snapshot, &bundle.root, &language, "", ""));
    }
    plan
}

/// 🛠️ Go `runTechnologyTests`: bundles whose flattened technology segment matches.
pub fn plan_technology(snapshot: &FilesystemSnapshot, technology_name: &str) -> InvocationPlan {
    let wanted = flat(technology_name);
    let mut plan = InvocationPlan::default();
    for bundle in &snapshot.bundles {
        let technology = bundle.name.split('/').next().unwrap_or("");
        if flat(technology) != wanted {
            continue;
        }
        let language = detect_bundle_language(snapshot, &bundle.root);
        if language.is_empty() {
            continue;
        }
        plan.extend(plan_bundle(snapshot, &bundle.root, &language, "", ""));
    }
    plan
}

/// 🔸️ Go `runBundleTests`.
pub fn plan_bundle(snapshot: &FilesystemSnapshot, bundle_root: &str, language: &str, file_filter: &str, test_filter: &str) -> InvocationPlan {
    let cwd = snapshot.absolute(bundle_root);
    let language = if language.is_empty() { detect_bundle_language(snapshot, bundle_root) } else { language.to_string() };
    let filter = optional(test_filter);
    match language.as_str() {
        "go" => {
            let mut args = vec!["test".to_string()];
            if !test_filter.is_empty() {
                args.push("-run".to_string());
                args.push(test_filter.to_string());
            }
            args.push(if file_filter.is_empty() { "./...".to_string() } else { format!("./{file_filter}/...") });
            single(Runner::Go, args, cwd, filter)
        }
        "rust" => {
            let mut args = vec!["test".to_string()];
            if !test_filter.is_empty() {
                args.push(test_filter.to_string());
            }
            single(Runner::Cargo, args, cwd, filter)
        }
        "csharp" => {
            let mut args = vec!["test".to_string()];
            if !test_filter.is_empty() {
                args.push("--filter".to_string());
                args.push(test_filter.to_string());
            }
            single(Runner::Dotnet, args, cwd, filter)
        }
        "typescript" | "javascript" => {
            let (runner, args) = detect_js_test_runner(snapshot, &cwd, test_filter);
            single(runner, args, cwd, filter)
        }
        "python" => {
            let mut args = vec!["run".to_string(), "pytest".to_string()];
            if !file_filter.is_empty() {
                args.push(file_filter.to_string());
            }
            if !test_filter.is_empty() {
                args.push("-k".to_string());
                args.push(test_filter.to_string());
            }
            if snapshot.uv_available {
                single(Runner::Uv, args, cwd, filter)
            } else {
                single(Runner::Pytest, args[2..].to_vec(), cwd, filter)
            }
        }
        _ => InvocationPlan { invocations: Vec::new(), problems: vec![format!("unknown language {language:?} for bundle {bundle_root}")] },
    }
}

/// 🔍️ Go `detectJSTestRunner`: manifest text decides, and vitest is the fallback.
pub fn detect_js_test_runner(snapshot: &FilesystemSnapshot, absolute_root: &str, test_filter: &str) -> (Runner, Vec<String>) {
    let manifest = join_path(absolute_root, "package.json");
    if let Some(contents) = snapshot.read_text(&manifest) {
        if contents.contains("vitest") {
            let mut args = vec!["vitest".to_string(), "run".to_string()];
            if !test_filter.is_empty() {
                args.push("-t".to_string());
                args.push(test_filter.to_string());
            }
            return (Runner::Npx, args);
        }
        if contents.contains("jest") {
            let mut args = vec!["jest".to_string()];
            if !test_filter.is_empty() {
                args.push("-t".to_string());
                args.push(test_filter.to_string());
            }
            return (Runner::Npx, args);
        }
        if contents.contains("\"test\"") {
            return (Runner::Npm, vec!["test".to_string()]);
        }
    }
    let mut args = vec!["vitest".to_string(), "run".to_string()];
    if !test_filter.is_empty() {
        args.push("-t".to_string());
        args.push(test_filter.to_string());
    }
    (Runner::Npx, args)
}

/// 🔺️ Go `runFileTests`.
pub fn plan_file(snapshot: &FilesystemSnapshot, file_path: &str, language: &str, test_filter: &str) -> InvocationPlan {
    let language = if language.is_empty() { language_of_extension(file_path).unwrap_or("").to_string() } else { language.to_string() };
    let absolute_file = snapshot.absolute(file_path);
    let Some(bundle) = snapshot.bundle_by_path(file_path) else {
        return InvocationPlan { invocations: Vec::new(), problems: vec![format!("no bundle found for file {file_path}")] };
    };
    let cwd = join_path(&snapshot.root, &bundle.root);
    let relative_file = relative_path(&cwd, &absolute_file);
    let filter = optional(test_filter);
    match language.as_str() {
        "go" => {
            let mut args = vec!["test".to_string(), "-v".to_string(), "-run".to_string()];
            args.push(if test_filter.is_empty() { ".".to_string() } else { test_filter.to_string() });
            let relative_dir = relative_path(&cwd, &dir_name(&absolute_file));
            args.push(if relative_dir == "." || relative_dir.is_empty() { "./...".to_string() } else { format!("./{relative_dir}/...") });
            single(Runner::Go, args, cwd, filter)
        }
        "python" => {
            let mut args = vec!["run".to_string(), "pytest".to_string(), relative_file];
            if !test_filter.is_empty() {
                args.push("-k".to_string());
                args.push(test_filter.to_string());
            }
            if snapshot.uv_available {
                single(Runner::Uv, args, cwd, filter)
            } else {
                single(Runner::Pytest, args[2..].to_vec(), cwd, filter)
            }
        }
        "typescript" | "javascript" => {
            let (runner, mut args) = detect_js_test_runner(snapshot, &cwd, test_filter);
            args.push(relative_file);
            single(runner, args, cwd, filter)
        }
        "csharp" => {
            let mut args = vec!["test".to_string()];
            if !test_filter.is_empty() {
                args.push("--filter".to_string());
                args.push(test_filter.to_string());
            }
            single(Runner::Dotnet, args, cwd, filter)
        }
        "rust" => {
            let mut args = vec!["test".to_string()];
            if !test_filter.is_empty() {
                args.push(test_filter.to_string());
            }
            single(Runner::Cargo, args, cwd, filter)
        }
        _ => InvocationPlan { invocations: Vec::new(), problems: vec![format!("unsupported language {language:?} for file test")] },
    }
}

/// 🔖️ Go `runSectionTests`: only Go narrows to a section; every other language falls back to the file.
pub fn plan_section(snapshot: &FilesystemSnapshot, file_path: &str, language: &str, section: &str) -> InvocationPlan {
    let language = if language.is_empty() { language_of_extension(file_path).unwrap_or("").to_string() } else { language.to_string() };
    let absolute_file = snapshot.absolute(file_path);
    let Some(bundle) = snapshot.bundle_by_path(file_path) else {
        return InvocationPlan { invocations: Vec::new(), problems: vec![format!("no bundle found for file {file_path}")] };
    };
    let cwd = join_path(&snapshot.root, &bundle.root);
    if language != "go" {
        return plan_file(snapshot, file_path, &language, "");
    }
    let pattern = collect_go_tests_in_section(snapshot.read_text(&absolute_file).unwrap_or(""), section);
    if pattern.is_empty() {
        return InvocationPlan { invocations: Vec::new(), problems: vec![format!("no tests found in section {section:?} of {file_path}")] };
    }
    let args = vec!["test".to_string(), "-v".to_string(), "-run".to_string(), pattern.clone(), "./...".to_string()];
    single(Runner::Go, args, cwd, Some(pattern))
}

/// 🧪️ Go `runDefinitionTest`.
pub fn plan_definition(snapshot: &FilesystemSnapshot, file_path: &str, language: &str, bundle_root: &str, test_name: &str) -> InvocationPlan {
    let language = if language.is_empty() { language_of_extension(file_path).unwrap_or("").to_string() } else { language.to_string() };
    let bundle_root = if bundle_root.is_empty() { snapshot.bundle_by_path(file_path).map(|bundle| bundle.root.clone()).unwrap_or_default() } else { bundle_root.to_string() };
    let cwd = snapshot.absolute(&bundle_root);
    let contents = snapshot.read_text(&join_path(&snapshot.root, file_path)).unwrap_or("");
    let mut original = resolve_test_function_name(contents, test_name);
    if original.is_empty() {
        original = unflatten_test_name(test_name);
    }
    match language.as_str() {
        "go" => single(Runner::Go, vec!["test".to_string(), "-v".to_string(), "-run".to_string(), format!("^{original}$"), "./...".to_string()], cwd, Some(original)),
        "python" => {
            let args = vec!["run".to_string(), "pytest".to_string(), "-k".to_string(), test_name.to_string()];
            if snapshot.uv_available {
                single(Runner::Uv, args, cwd, Some(test_name.to_string()))
            } else {
                single(Runner::Pytest, args[2..].to_vec(), cwd, Some(test_name.to_string()))
            }
        }
        "csharp" => single(Runner::Dotnet, vec!["test".to_string(), "--filter".to_string(), format!("FullyQualifiedName~{original}")], cwd, Some(original)),
        "rust" => single(Runner::Cargo, vec!["test".to_string(), test_name.to_string()], cwd, Some(test_name.to_string())),
        _ => plan_bundle(snapshot, &bundle_root, &language, "", test_name),
    }
}

/// 🪒️ Go `StripLeadingGrapheme`: drops one leading non-ASCII grapheme cluster — the emoji plus the
/// variation selectors, skin-tone modifiers and zero-width-joined runes that belong to it — so that
/// `🚀Gamma` and `🚀️Gamma` both name the section `Gamma`.
pub fn strip_leading_grapheme(value: &str) -> String {
    let runes: Vec<char> = value.chars().collect();
    let mut index = 0;
    while index < runes.len() && (runes[index] as u32) <= 0x7F {
        index += 1;
    }
    if index >= runes.len() {
        return value.to_string();
    }
    index += 1;
    while index < runes.len() {
        let rune = runes[index] as u32;
        let joined = rune == 0x200D;
        let modifier = (0xFE00..=0xFE0F).contains(&rune) || (0x1F3FB..=0x1F3FF).contains(&rune) || (0x0300..=0x036F).contains(&rune) || rune == 0x20E3;
        if !joined && !modifier {
            break;
        }
        index += 1;
        if joined && index < runes.len() {
            index += 1;
        }
    }
    runes[index..].iter().collect::<String>().trim().to_string()
}

/// 📑️ Go `collectGoTestsInSection`: region-marker scanning that yields an anchored `-run` pattern.
pub fn collect_go_tests_in_section(content: &str, section_name: &str) -> String {
    let flat_section = flat(section_name);
    let mut in_section = false;
    let mut names: Vec<String> = Vec::new();
    for line in content.split('\n') {
        let trimmed = line.trim();
        if trimmed.starts_with("//") {
            if let Some(index) = trimmed.find("#region") {
                let region = strip_leading_grapheme(trimmed[index + "#region".len()..].trim());
                if flat(&region) == flat_section {
                    in_section = true;
                    continue;
                } else if in_section {
                    in_section = false;
                }
            } else if trimmed.contains("#endregion") && in_section {
                break;
            }
        }
        if in_section {
            if let Some(name) = go_test_function_name(line) {
                names.push(name);
            }
        }
    }
    if names.is_empty() {
        return String::new();
    }
    format!("^({})$", names.join("|"))
}

/// 🧬️ Go `resolveTestFunctionName`: the declared name whose flat form matches.
pub fn resolve_test_function_name(content: &str, flat_name: &str) -> String {
    for name in declared_test_function_names(content) {
        if flat(&name) == flat_name {
            return name;
        }
    }
    String::new()
}

/// 🧱️ Go `unflattenTestName`: re-cases a flat name from the known prefixes.
pub fn unflatten_test_name(value: &str) -> String {
    for prefix in ["testbenchmark", "testfuzz", "benchmark", "fuzz", "test"] {
        if let Some(rest) = value.strip_prefix(prefix) {
            if !rest.is_empty() {
                return format!("{}{}{}{}", prefix[..1].to_uppercase(), &prefix[1..], rest[..1].to_uppercase(), &rest[1..]);
            }
        }
    }
    if value.is_empty() {
        return String::new();
    }
    format!("{}{}", value[..1].to_uppercase(), &value[1..])
}

/// 🔤️ `^func (Test\w+)\(` on one line, hand-rolled.
fn go_test_function_name(line: &str) -> Option<String> {
    let rest = line.strip_prefix("func ")?;
    let rest = rest.strip_prefix("Test")?;
    let name: String = rest.chars().take_while(|value| value.is_ascii_alphanumeric() || *value == '_').collect();
    if rest[name.len()..].starts_with('(') {
        Some(format!("Test{name}"))
    } else {
        None
    }
}

/// 🔎️ `func ((?:Test|Benchmark|Fuzz)\w+)\(` anywhere in the text, hand-rolled.
fn declared_test_function_names(content: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = content;
    while let Some(index) = rest.find("func ") {
        let tail = &rest[index + "func ".len()..];
        for prefix in ["Test", "Benchmark", "Fuzz"] {
            if let Some(after) = tail.strip_prefix(prefix) {
                let name: String = after.chars().take_while(|value| value.is_ascii_alphanumeric() || *value == '_').collect();
                if after[name.len()..].starts_with('(') {
                    found.push(format!("{prefix}{name}"));
                }
                break;
            }
        }
        rest = tail;
    }
    found
}

fn optional(value: &str) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn single(runner: Runner, args: Vec<String>, cwd: String, filter: Option<String>) -> InvocationPlan {
    let mut argv = vec![runner.program().to_string()];
    argv.extend(args);
    InvocationPlan { invocations: vec![RunnerInvocation { runner, argv, cwd, env: BTreeMap::new(), filter }], problems: Vec::new() }
}
//#endregion 🗺️Planning
