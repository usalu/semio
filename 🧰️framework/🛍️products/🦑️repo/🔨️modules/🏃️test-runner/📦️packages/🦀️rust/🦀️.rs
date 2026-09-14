//! 🏃️ Repo test runner: which tests a scope selects, which processes that becomes, and what the
//! runners answered.
//!
//! The domain is split into three layers that never leak into each other:
//! 1. **Planning is pure.** A [`FilesystemSnapshot`] plus a [`TestScope`] deterministically yields an
//!    ordered [`InvocationPlan`]. No process, no clock, no ambient filesystem — so a plan is a
//!    golden value a fixture can pin.
//! 2. **Execution is a port.** [`ProcessRunner`] is the only way a plan reaches an operating system,
//!    which is what lets [`RecordedProcessRunner`] replay a committed transcript byte for byte.
//! 3. **Parsing is per runner.** Each runner's native report is folded into one [`TestOutcome`], so a
//!    caller never learns which of the six report dialects produced a result.
//!
//! Behavioural reference: the Go `🕸️Test Command` and `🖲️Missing Test Functions` regions of
//! `💻️client/⌨️cli/🧩️component.go` as frozen in this ticket's snapshot. Every planning decision here
//! reproduces that source; the execution and parsing layers are net new (Go streams runner output
//! straight to the terminal and never parses it).

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🪪️IdentityPending
// 🚚️ These four helpers belong to `🪪️identity` (`semio-framework-repo-identity`) and to
// `🗣️languages`. Neither crate exports them yet, so faithful ports of the Go originals live here
// behind `EntityIdentity` / `LanguageTable`; when those crates land, this region collapses to a
// re-export and the default implementations are deleted. `id_to_uri` is the one place where this
// module is knowingly NOT yet behaviour-identical to Go: `DetectEntityKindFromId` needs the entity
// emoji table that `🪪️identity` owns, so the default port answers "" (unknown kind) for every raw
// id, exactly as Go does for an id whose kind it cannot detect. Callers that pass `repo://` URIs —
// the form the CLI and MCP both produce — are unaffected.

/// 🪪️ Identifier vocabulary the scope resolver needs from `🪪️identity`.
pub trait EntityIdentity {
    /// 🔤️ Alphanumerics and non-ASCII runes only, lowercased.
    fn flat(&self, text: &str) -> String;
    /// 🛤️ Reverses percent-encoding of spaces in a URI path segment list.
    fn path_from_uri_path(&self, uri_path: &str) -> String;
    /// 🔗️ `repo://<kind>/<id>` for an id whose entity kind is detectable, else the empty string.
    fn id_to_uri(&self, id: &str) -> String;
}

/// 🗣️ File-extension to language mapping the scope resolver needs from `🗣️languages`.
pub trait LanguageTable {
    /// 🏷️ The language name for a path, or `None` when no plugin claims the extension.
    fn language_of(&self, file_path: &str) -> Option<String>;
}

/// 🪪️ The ported-from-Go default for both ports.
#[derive(Clone, Copy, Debug, Default)]
pub struct PendingIdentity;

impl EntityIdentity for PendingIdentity {
    fn flat(&self, text: &str) -> String {
        flat(text)
    }
    fn path_from_uri_path(&self, uri_path: &str) -> String {
        path_from_uri_path(uri_path)
    }
    fn id_to_uri(&self, _id: &str) -> String {
        String::new()
    }
}

impl LanguageTable for PendingIdentity {
    fn language_of(&self, file_path: &str) -> Option<String> {
        language_of_extension(file_path).map(str::to_string)
    }
}

/// 🔤️ Keeps alphanumerics and every non-ASCII rune, then lowercases — Go `Flat`.
pub fn flat(text: &str) -> String {
    text.chars()
        .filter(|value| value.is_ascii_alphanumeric() || (*value as u32) > 0x7F)
        .collect::<String>()
        .to_lowercase()
}

/// 🛤️ Decodes `%20` per segment — Go `PathFromUriPath`.
pub fn path_from_uri_path(uri_path: &str) -> String {
    uri_path.split('/').map(|segment| segment.replace("%20", " ")).collect::<Vec<_>>().join("/")
}

/// 🗣️ The extension table the planner needs; a subset of `🗣️languages` limited to testable languages.
pub fn language_of_extension(file_path: &str) -> Option<&'static str> {
    let lower = file_path.to_lowercase();
    let extension = lower.rsplit_once('.').map(|(_, tail)| tail)?;
    match extension {
        "go" => Some("go"),
        "rs" => Some("rust"),
        "cs" => Some("csharp"),
        "ts" | "tsx" => Some("typescript"),
        "js" | "jsx" | "mjs" | "cjs" => Some("javascript"),
        "py" => Some("python"),
        "rb" => Some("ruby"),
        _ => None,
    }
}
//#endregion 🪪️IdentityPending

//#region 🧮️Paths
/// 🧭️ Forward-slash normalisation; every path this crate stores or compares is in this form.
pub fn normalize_separators(path: &str) -> String {
    path.replace('\\', "/")
}

/// 📏️ Accepts both POSIX roots and Windows drive roots so one fixture runs on every host.
pub fn is_absolute_path(path: &str) -> bool {
    let path = normalize_separators(path);
    if path.starts_with('/') {
        return true;
    }
    let bytes = path.as_bytes();
    bytes.len() >= 3 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && bytes[2] == b'/'
}

/// 🔗️ Joins and cleans, dropping `.` segments and resolving `..` — Go `filepath.Join`.
pub fn join_path(base: &str, relative: &str) -> String {
    let base = normalize_separators(base);
    let relative = normalize_separators(relative);
    if base.is_empty() {
        return clean_path(&relative);
    }
    if relative.is_empty() {
        return clean_path(&base);
    }
    clean_path(&format!("{}/{}", base.trim_end_matches('/'), relative))
}

/// 🧹️ Collapses separators and `.`/`..` segments while keeping any root prefix.
pub fn clean_path(path: &str) -> String {
    let path = normalize_separators(path);
    let (prefix, rest) = if let Some(tail) = path.strip_prefix('/') {
        ("/".to_string(), tail.to_string())
    } else if is_absolute_path(&path) {
        (path[..3].to_string(), path[3..].to_string())
    } else {
        (String::new(), path.clone())
    };
    let mut segments: Vec<&str> = Vec::new();
    for segment in rest.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                if segments.last().is_some_and(|last| *last != "..") {
                    segments.pop();
                } else if prefix.is_empty() {
                    segments.push("..");
                }
            }
            other => segments.push(other),
        }
    }
    let body = segments.join("/");
    if prefix.is_empty() {
        if body.is_empty() {
            ".".to_string()
        } else {
            body
        }
    } else {
        format!("{prefix}{body}")
    }
}

/// 🏷️ The final segment of a path — Go `filepath.Base`.
pub fn base_name(path: &str) -> String {
    let path = normalize_separators(path);
    let trimmed = path.trim_end_matches('/');
    if trimmed.is_empty() {
        return path;
    }
    trimmed.rsplit('/').next().unwrap_or(trimmed).to_string()
}

/// 📁️ Everything before the final segment — Go `filepath.Dir`.
pub fn dir_name(path: &str) -> String {
    let cleaned = clean_path(path);
    match cleaned.rfind('/') {
        Some(0) => "/".to_string(),
        Some(index) => cleaned[..index].to_string(),
        None => ".".to_string(),
    }
}

/// ↔️ `target` expressed relative to `base`, or `target` unchanged when it is not below it.
pub fn relative_path(base: &str, target: &str) -> String {
    let base = clean_path(base);
    let target = clean_path(target);
    if base == target {
        return ".".to_string();
    }
    let prefix = format!("{}/", base.trim_end_matches('/'));
    match target.strip_prefix(&prefix) {
        Some(rest) => rest.to_string(),
        None => target,
    }
}
//#endregion 🧮️Paths

//#region 🌲️Snapshot
/// 🌲️ Whether a snapshot entry is a file or a directory.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EntryKind {
    /// 📄️ A regular file.
    File,
    /// 📁️ A directory.
    Directory,
}

/// 📄️ One path in a planning snapshot, with contents only where a planning rule reads them.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SnapshotEntry {
    /// 🛤️ Repository-relative or absolute path, always forward-slashed.
    pub path: String,
    /// 🌲️ File or directory.
    pub kind: EntryKind,
    /// 📝️ Text contents, present only for files a planning rule inspects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
}

/// 📦️ The two bundle fields planning needs, projected from `📐️model`'s richer `Bundle`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SnapshotBundle {
    /// 🏷️ `technology/bundle` name.
    pub name: String,
    /// 🛤️ Repository-relative bundle root.
    pub root: String,
}

impl From<&semio_framework_repo_model::Bundle> for SnapshotBundle {
    fn from(bundle: &semio_framework_repo_model::Bundle) -> Self {
        Self { name: bundle.name.clone(), root: bundle.root.clone() }
    }
}

/// 🌍️ Everything planning is allowed to know about the world.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct FilesystemSnapshot {
    /// 🏠️ Repository root every relative path is joined onto.
    #[serde(default)]
    pub root: String,
    /// 🌲️ Every path the plan may probe.
    #[serde(default)]
    pub entries: Vec<SnapshotEntry>,
    /// 📦️ The bundle table Go reads through `LoadBundles`.
    #[serde(default)]
    pub bundles: Vec<SnapshotBundle>,
    /// 🐍️ Whether `uv` is on `PATH` — Go `uvExists`.
    #[serde(rename = "uvAvailable", default)]
    pub uv_available: bool,
}

impl FilesystemSnapshot {
    /// 🧭️ Absolute form of a repository-relative path.
    pub fn absolute(&self, path: &str) -> String {
        if is_absolute_path(path) {
            clean_path(path)
        } else {
            join_path(&self.root, path)
        }
    }

    /// ✅️ Go `FileExists`: true only for a present regular file.
    pub fn file_exists(&self, path: &str) -> bool {
        let wanted = clean_path(path);
        self.entries.iter().any(|entry| entry.kind == EntryKind::File && clean_path(&entry.path) == wanted)
    }

    /// 📁️ Whether a directory is present.
    pub fn directory_exists(&self, path: &str) -> bool {
        let wanted = clean_path(path);
        self.entries.iter().any(|entry| entry.kind == EntryKind::Directory && clean_path(&entry.path) == wanted)
    }

    /// 📝️ Contents of a recorded file.
    pub fn read_text(&self, path: &str) -> Option<&str> {
        let wanted = clean_path(path);
        self.entries
            .iter()
            .find(|entry| entry.kind == EntryKind::File && clean_path(&entry.path) == wanted)
            .and_then(|entry| entry.contents.as_deref())
    }

    /// 🔎️ Go `filepath.Glob(dir + "/" + pattern)`: non-recursive, direct children only.
    pub fn glob_children(&self, directory: &str, pattern: &str) -> Vec<String> {
        let directory = clean_path(directory);
        let mut matched: Vec<String> = self
            .entries
            .iter()
            .filter(|entry| entry.kind == EntryKind::File)
            .map(|entry| clean_path(&entry.path))
            .filter(|path| dir_name(path) == directory)
            .filter(|path| semio_framework_repo_workspace::glob_match(pattern, &base_name(path)).unwrap_or(false))
            .collect();
        matched.sort();
        matched
    }

    /// 🚶️ Every recorded file at or below `root`, in recorded order — Go `filepath.Walk` order after sorting.
    pub fn walk_files(&self, root: &str, recursive: bool) -> Vec<String> {
        let root = clean_path(root);
        let prefix = format!("{}/", root.trim_end_matches('/'));
        let mut found: Vec<String> = self
            .entries
            .iter()
            .filter(|entry| entry.kind == EntryKind::File)
            .map(|entry| clean_path(&entry.path))
            .filter(|path| *path == root || path.starts_with(&prefix))
            .filter(|path| recursive || dir_name(path) == root)
            .collect();
        found.sort();
        found
    }

    /// 📦️ Go `findBundleByName`: exact name first, then flattened equality.
    pub fn bundle_by_name(&self, name: &str) -> Option<&SnapshotBundle> {
        self.bundles.iter().find(|bundle| bundle.name == name || flat(&bundle.name) == flat(name))
    }

    /// 📦️ Go `GetBundleByPath`: the longest bundle root that prefixes the path.
    pub fn bundle_by_path(&self, path: &str) -> Option<&SnapshotBundle> {
        let path = clean_path(&normalize_separators(path));
        self.bundles
            .iter()
            .filter(|bundle| {
                let root = clean_path(&bundle.root);
                path == root || path.starts_with(&format!("{}/", root.trim_end_matches('/')))
            })
            .max_by_key(|bundle| clean_path(&bundle.root).len())
    }
}

/// 🗣️ Go `detectBundleLanguage`: manifest probing in a fixed, order-sensitive sequence.
pub fn detect_bundle_language(snapshot: &FilesystemSnapshot, bundle_root: &str) -> String {
    let absolute = snapshot.absolute(bundle_root);
    if snapshot.file_exists(&join_path(&absolute, "go.mod")) {
        return "go".to_string();
    }
    if snapshot.file_exists(&join_path(&absolute, "Cargo.toml")) {
        return "rust".to_string();
    }
    if !snapshot.glob_children(&absolute, "*.csproj").is_empty() {
        return "csharp".to_string();
    }
    if !snapshot.glob_children(&absolute, "*.sln").is_empty() {
        return "csharp".to_string();
    }
    if snapshot.file_exists(&join_path(&absolute, "package.json")) {
        return "typescript".to_string();
    }
    if snapshot.file_exists(&join_path(&absolute, "pyproject.toml")) || snapshot.file_exists(&join_path(&absolute, "requirements.txt")) {
        return "python".to_string();
    }
    String::new()
}
//#endregion 🌲️Snapshot

//#region 🔭️Scope
/// 🔭️ How wide a test run reaches — Go `testScopeKind`.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ScopeKind {
    /// 🌍️ Every bundle with a detectable language.
    #[default]
    All,
    /// 🧰️ Every bundle of one technology.
    Technology,
    /// ⌨️ One bundle.
    Bundle,
    /// 🥼️ One file.
    File,
    /// 🔖️ One section of one file.
    Section,
    /// 🧪️ One test function.
    Definition,
}

/// 🧪️ A resolved scope — Go `testScope`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct TestScope {
    /// 🔭️ Granularity.
    pub kind: ScopeKindField,
    /// 📦️ Bundle root, or the technology name for a technology scope.
    #[serde(default)]
    pub bundle_root: String,
    /// 🥼️ File path.
    #[serde(default)]
    pub file_path: String,
    /// 🔖️ Section name.
    #[serde(default)]
    pub section: String,
    /// 🧪️ Flat test name.
    #[serde(default)]
    pub test_name: String,
    /// 🗣️ Language.
    #[serde(default)]
    pub language: String,
}

/// 🔭️ `ScopeKind` with `All` as the serde default, so a fixture may omit it.
pub type ScopeKindField = ScopeKind;

impl TestScope {
    /// 🌍️ The scope an empty or unrecognised selector resolves to.
    pub fn all() -> Self {
        Self { kind: ScopeKind::All, ..Self::default() }
    }
}

/// 🔗️ Go `resolveTestScopes`: no selector means one `all` scope.
pub fn resolve_test_scopes(ids: &[String], snapshot: &FilesystemSnapshot, identity: &dyn EntityIdentity, languages: &dyn LanguageTable) -> Vec<TestScope> {
    if ids.is_empty() {
        return vec![TestScope::all()];
    }
    ids.iter().map(|raw| resolve_test_scope(raw, snapshot, identity, languages)).collect()
}

/// 🔷️ Go `resolveTestScope`: variation-selector stripping, `repo://` parsing, `p/` and `f/` routes.
pub fn resolve_test_scope(raw: &str, snapshot: &FilesystemSnapshot, identity: &dyn EntityIdentity, languages: &dyn LanguageTable) -> TestScope {
    let normalized = raw.replace(['\u{FE0E}', '\u{FE0F}'], "").trim().to_string();
    let uri = if normalized.starts_with("repo://") { normalized } else { identity.id_to_uri(&normalized) };
    if uri.is_empty() {
        return TestScope::all();
    }
    let path = uri.strip_prefix("repo://").unwrap_or(&uri).to_string();

    if let Some(rest) = path.strip_prefix("p/") {
        let parts = split_n(rest, '/', 3);
        if parts.len() == 2 {
            return TestScope { kind: ScopeKind::Technology, bundle_root: identity.path_from_uri_path(&parts[1]), ..TestScope::default() };
        }
        if parts.len() >= 3 {
            if let Some(bundle_rest) = parts[2].strip_prefix("b/") {
                let bundle_parts = split_n(bundle_rest, '/', 3);
                if bundle_parts.len() >= 2 {
                    let technology = identity.path_from_uri_path(&parts[1]);
                    let bundle = identity.path_from_uri_path(&bundle_parts[1]);
                    let bundle_name = format!("{}/{}", technology.trim_start_matches('@'), bundle);
                    let bundle_root = snapshot.bundle_by_name(&bundle_name).map_or(bundle_name, |found| found.root.clone());
                    if bundle_parts.len() == 2 {
                        let language = detect_bundle_language(snapshot, &bundle_root);
                        return TestScope { kind: ScopeKind::Bundle, bundle_root, language, ..TestScope::default() };
                    }
                    return resolve_test_scope_from_bundle_sub_path(&bundle_root, &bundle_parts[2], snapshot, identity, languages);
                }
            }
        }
    }

    if let Some(rest) = path.strip_prefix("f/") {
        let parts = split_n(rest, '/', 2);
        let file_path = identity.path_from_uri_path(&parts[0]);
        let language = languages.language_of(&file_path).unwrap_or_default();
        if parts.len() == 1 {
            return TestScope { kind: ScopeKind::File, file_path, language, ..TestScope::default() };
        }
        return resolve_test_scope_from_file_sub_path(&file_path, &language, &parts[1], identity);
    }

    TestScope::all()
}

/// 📦️ Go `resolveTestScopeFromBundleSubPath`: `f`/`s`/`d`/`fd` markers narrow the scope in order.
pub fn resolve_test_scope_from_bundle_sub_path(
    bundle_root: &str,
    sub_path: &str,
    snapshot: &FilesystemSnapshot,
    identity: &dyn EntityIdentity,
    languages: &dyn LanguageTable,
) -> TestScope {
    let mut language = detect_bundle_language(snapshot, bundle_root);
    let parts: Vec<&str> = sub_path.split('/').collect();
    let mut file_path = String::new();
    let mut section = String::new();
    let mut test_name = String::new();
    let mut kind = ScopeKind::Bundle;
    let mut index = 0;
    while index < parts.len() {
        if parts[index] == "f" && index + 1 < parts.len() {
            file_path = join_path(bundle_root, &identity.path_from_uri_path(parts[index + 1]));
            kind = ScopeKind::File;
            index += 1;
        } else if parts[index] == "s" && index + 1 < parts.len() {
            section = identity.path_from_uri_path(parts[index + 1]);
            kind = ScopeKind::Section;
            index += 1;
        } else if parts[index] == "d" && index + 2 < parts.len() {
            test_name = identity.path_from_uri_path(parts[index + 2]);
            kind = ScopeKind::Definition;
            index += 2;
        } else if parts[index] == "fd" && index + 2 < parts.len() {
            index += 2;
        }
        index += 1;
    }
    if let Some(found) = languages.language_of(&file_path) {
        language = found;
    }
    TestScope { kind, bundle_root: bundle_root.to_string(), file_path, section, test_name, language }
}

/// 📄️ Go `resolveTestScopeFromFileSubPath`.
pub fn resolve_test_scope_from_file_sub_path(file_path: &str, language: &str, sub_path: &str, identity: &dyn EntityIdentity) -> TestScope {
    let parts: Vec<&str> = sub_path.split('/').collect();
    let mut section = String::new();
    let mut test_name = String::new();
    let mut kind = ScopeKind::File;
    let mut index = 0;
    while index < parts.len() {
        if parts[index] == "s" && index + 1 < parts.len() {
            section = identity.path_from_uri_path(parts[index + 1]);
            kind = ScopeKind::Section;
            index += 1;
        } else if parts[index] == "d" && index + 2 < parts.len() {
            test_name = identity.path_from_uri_path(parts[index + 2]);
            kind = ScopeKind::Definition;
            index += 2;
        }
        index += 1;
    }
    TestScope { kind, bundle_root: String::new(), file_path: file_path.to_string(), section, test_name, language: language.to_string() }
}

/// ✂️ Go `strings.SplitN`: at most `n` pieces, the last one keeping every remaining separator.
fn split_n(value: &str, separator: char, n: usize) -> Vec<String> {
    value.splitn(n, separator).map(str::to_string).collect()
}
//#endregion 🔭️Scope

//#region 🗺️Planning
/// 🏭️ The program a planned invocation runs.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum Runner {
    /// 🐹️ `go test`.
    Go,
    /// 🦀️ `cargo test`.
    Cargo,
    /// 🦀️ `cargo nextest run`.
    CargoNextest,
    /// 🔷️ `dotnet test`.
    Dotnet,
    /// 🟦️ `npx <js runner>`.
    Npx,
    /// 🟦️ `npm test`.
    Npm,
    /// 🐍️ `uv run pytest`.
    Uv,
    /// 🐍️ `pytest`.
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
    /// 🏭️ Which runner.
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
    /// 🎬️ Invocations in execution order.
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

/// 🗺️ Plans every scope in order — Go's `testCommand` loop over `runTestScope`.
pub fn plan_scopes(snapshot: &FilesystemSnapshot, scopes: &[TestScope]) -> InvocationPlan {
    let mut plan = InvocationPlan::default();
    for scope in scopes {
        plan.extend(plan_scope(snapshot, scope));
    }
    plan
}

/// 🗺️ Go `runTestScope`, with the process call replaced by a recorded invocation.
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

/// 🟦️ Go `detectJSTestRunner`: manifest text decides, and vitest is the fallback.
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

/// 🎯️ Go `resolveTestFunctionName`: the declared name whose flat form matches.
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

/// 🐹️ `^func (Test\w+)\(` on one line, hand-rolled.
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

//#region ⚙️Execution
// 🚚️ `ProcessRequest` / `ProcessOutput` / `ProcessRunner` are the shape `🧩️providers` will own for
// every process this repository shells out to (git, gh, devcontainer, the test runners). They are
// declared here because `🧩️providers` does not exist yet; consolidating means deleting this block
// and importing the identical trait, with no call-site change.

/// 🎬️ One process to run.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProcessRequest {
    /// 🏷️ Executable name.
    pub program: String,
    /// 📜️ Arguments after the program name.
    pub args: Vec<String>,
    /// 📁️ Working directory.
    pub cwd: String,
    /// 🌱️ Environment overlay.
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

/// 📤️ What a process answered.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct ProcessOutput {
    /// 🔢️ Exit status.
    pub status: i32,
    /// 📝️ Standard output.
    #[serde(default)]
    pub stdout: String,
    /// 📝️ Standard error.
    #[serde(default)]
    pub stderr: String,
}

/// ⚙️ The only door from a plan to an operating system.
pub trait ProcessRunner {
    /// ▶️ Runs one process to completion.
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, String>;
}

/// 🎞️ One recorded `(argv, cwd) → output` pair.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RecordedTranscript {
    /// 📜️ The argv this transcript answers, program name first.
    pub argv: Vec<String>,
    /// 📁️ The working directory it answers for; empty matches any.
    #[serde(default)]
    pub cwd: String,
    /// 📤️ The recorded answer.
    pub output: ProcessOutput,
}

/// 🎞️ A `ProcessRunner` that replays committed transcripts and never touches the machine.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct RecordedProcessRunner {
    /// 🎞️ Transcripts, matched in order.
    #[serde(default)]
    pub transcripts: Vec<RecordedTranscript>,
}

impl RecordedProcessRunner {
    /// 🆕️ Builds a replaying runner.
    pub fn new(transcripts: Vec<RecordedTranscript>) -> Self {
        Self { transcripts }
    }
}

impl ProcessRunner for RecordedProcessRunner {
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, String> {
        let mut argv = vec![request.program.clone()];
        argv.extend(request.args.clone());
        self.transcripts
            .iter()
            .find(|transcript| transcript.argv == argv && (transcript.cwd.is_empty() || clean_path(&transcript.cwd) == clean_path(&request.cwd)))
            .map(|transcript| transcript.output.clone())
            .ok_or_else(|| format!("no recorded transcript for {}", argv.join(" ")))
    }
}

/// 🛑️ A cooperative cancellation flag shared with whoever drives the run.
#[derive(Clone, Debug, Default)]
pub struct CancellationToken {
    flag: Arc<AtomicBool>,
}

impl CancellationToken {
    /// 🆕️ A token that is not cancelled.
    pub fn new() -> Self {
        Self::default()
    }
    /// 🛑️ Requests cancellation; every later invocation is skipped.
    pub fn cancel(&self) {
        self.flag.store(true, Ordering::SeqCst);
    }
    /// ❓️ Whether cancellation was requested.
    pub fn is_cancelled(&self) -> bool {
        self.flag.load(Ordering::SeqCst)
    }
}

/// 📣️ What the caller is told while a plan runs.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "event", rename_all = "kebab-case")]
pub enum ProgressEvent {
    /// 🚦️ The plan is about to start.
    PlanStarted {
        /// 🔢️ Invocations in the plan.
        total: usize,
    },
    /// ▶️ One invocation is starting.
    InvocationStarted {
        /// 🔢️ Zero-based position in the plan.
        index: usize,
        /// 📜️ Its argv.
        argv: Vec<String>,
    },
    /// ⏹️ One invocation finished.
    InvocationFinished {
        /// 🔢️ Zero-based position in the plan.
        index: usize,
        /// 🚦️ How it ended.
        status: RunStatus,
        /// 🔢️ Failing tests it reported.
        failed: usize,
    },
    /// 🛑️ Cancellation was observed before the invocation at `index` started.
    Cancelled {
        /// 🔢️ Invocations that had already completed.
        completed: usize,
    },
    /// 🏁️ The plan is done.
    PlanFinished {
        /// 🔢️ Invocations that completed.
        completed: usize,
    },
}

/// 📊️ What a whole plan produced.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExecutionReport {
    /// 📊️ One outcome per completed invocation, in plan order.
    #[serde(default)]
    pub outcomes: Vec<TestOutcome>,
    /// 🛑️ Whether the run stopped early because cancellation was requested.
    #[serde(default)]
    pub cancelled: bool,
    /// 🔢️ Invocations that completed.
    #[serde(default)]
    pub completed: usize,
    /// 🔢️ Invocations the plan contained.
    #[serde(default)]
    pub total: usize,
    /// 🚫️ Planning refusals plus any runner that could not be started.
    #[serde(default)]
    pub problems: Vec<String>,
}

/// ▶️ Runs a plan through the port, reporting progress and honouring cancellation between invocations.
pub fn execute_plan(plan: &InvocationPlan, runner: &dyn ProcessRunner, cancellation: &CancellationToken, progress: &mut dyn FnMut(ProgressEvent)) -> ExecutionReport {
    let total = plan.invocations.len();
    let mut report = ExecutionReport { total, problems: plan.problems.clone(), ..ExecutionReport::default() };
    progress(ProgressEvent::PlanStarted { total });
    for (index, invocation) in plan.invocations.iter().enumerate() {
        if cancellation.is_cancelled() {
            report.cancelled = true;
            progress(ProgressEvent::Cancelled { completed: report.completed });
            return report;
        }
        progress(ProgressEvent::InvocationStarted { index, argv: invocation.argv.clone() });
        let request = ProcessRequest {
            program: invocation.argv.first().cloned().unwrap_or_default(),
            args: invocation.argv.iter().skip(1).cloned().collect(),
            cwd: invocation.cwd.clone(),
            env: invocation.env.clone(),
        };
        let outcome = match runner.run(&request) {
            Ok(output) => parse_outcome(invocation.runner, &output),
            Err(message) => {
                report.problems.push(message);
                TestOutcome { runner: invocation.runner, status: RunStatus::NotRun, ..TestOutcome::default() }
            }
        };
        progress(ProgressEvent::InvocationFinished { index, status: outcome.status, failed: outcome.totals.failed });
        report.outcomes.push(outcome);
        report.completed += 1;
    }
    progress(ProgressEvent::PlanFinished { completed: report.completed });
    report
}
//#endregion ⚙️Execution

//#region 📊️Parsing
/// 🚦️ How one test ended.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum TestStatus {
    /// ✅️ Passed.
    #[default]
    Passed,
    /// ❌️ Failed.
    Failed,
    /// ⏭️ Skipped, ignored or pending.
    Skipped,
}

/// 🚦️ How one invocation ended.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RunStatus {
    /// ✅️ Every test passed.
    Passed,
    /// ❌️ At least one test failed, or the runner exited non-zero.
    Failed,
    /// 🛑️ Cancelled before it produced a verdict.
    Cancelled,
    /// ⚪️ Never started.
    NotRun,
}

/// 🧪️ One test, normalised across every runner dialect.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestCaseOutcome {
    /// 🏷️ Test name as the runner reported it.
    pub name: String,
    /// 📦️ Package, file or suite the runner attributed it to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub suite: String,
    /// 🚦️ Verdict.
    pub status: TestStatus,
    /// ⏱️ Duration in milliseconds where the runner reports one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<f64>,
    /// 💬️ Failure text where the runner reports one.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

/// 🔢️ Counts, always recomputed from the cases so a transcript's own summary can never drift.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TestTotals {
    /// ✅️ Passing tests.
    pub passed: usize,
    /// ❌️ Failing tests.
    pub failed: usize,
    /// ⏭️ Skipped tests.
    pub skipped: usize,
    /// 🔢️ All reported tests.
    pub total: usize,
}

/// 📊️ One invocation's result in the one model every runner folds into.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TestOutcome {
    /// 🏭️ Which runner produced it.
    pub runner: Runner,
    /// 🚦️ Overall verdict.
    pub status: RunStatus,
    /// 🧪️ Every reported test, in report order.
    #[serde(default)]
    pub tests: Vec<TestCaseOutcome>,
    /// 🔢️ Counts.
    pub totals: TestTotals,
    /// 🔢️ Runner exit status where one was observed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_status: Option<i32>,
}

impl Default for TestOutcome {
    fn default() -> Self {
        Self { runner: Runner::Go, status: RunStatus::NotRun, tests: Vec::new(), totals: TestTotals::default(), exit_status: None }
    }
}

impl TestOutcome {
    fn finish(runner: Runner, tests: Vec<TestCaseOutcome>, output: &ProcessOutput) -> Self {
        let mut totals = TestTotals { total: tests.len(), ..TestTotals::default() };
        for case in &tests {
            match case.status {
                TestStatus::Passed => totals.passed += 1,
                TestStatus::Failed => totals.failed += 1,
                TestStatus::Skipped => totals.skipped += 1,
            }
        }
        let status = if totals.failed > 0 || output.status != 0 { RunStatus::Failed } else { RunStatus::Passed };
        Self { runner, status, tests, totals, exit_status: Some(output.status) }
    }
}

/// 📊️ Dispatches a runner's native report to its parser.
pub fn parse_outcome(runner: Runner, output: &ProcessOutput) -> TestOutcome {
    match runner {
        Runner::Go => parse_go_test_json(output),
        Runner::Cargo => parse_cargo_test(output),
        Runner::CargoNextest => parse_cargo_nextest(output),
        Runner::Dotnet => parse_dotnet_test(output),
        Runner::Npx | Runner::Npm => parse_vitest_json(output),
        Runner::Uv | Runner::Pytest => parse_pytest(output),
        Runner::Rspec => parse_rspec_json(output),
    }
}

/// 🐹️ `go test -json`: one JSON object per line, verdicts on `pass`/`fail`/`skip` with a `Test`.
pub fn parse_go_test_json(output: &ProcessOutput) -> TestOutcome {
    let mut messages: BTreeMap<String, String> = BTreeMap::new();
    let mut tests: Vec<TestCaseOutcome> = Vec::new();
    for line in output.stdout.lines() {
        let Ok(Value::Object(event)) = serde_json::from_str::<Value>(line.trim()) else { continue };
        let action = event.get("Action").and_then(Value::as_str).unwrap_or("");
        let package = event.get("Package").and_then(Value::as_str).unwrap_or("").to_string();
        let Some(name) = event.get("Test").and_then(Value::as_str) else { continue };
        let key = format!("{package}::{name}");
        if action == "output" {
            let text = event.get("Output").and_then(Value::as_str).unwrap_or("");
            messages.entry(key).or_default().push_str(text);
            continue;
        }
        let status = match action {
            "pass" => TestStatus::Passed,
            "fail" => TestStatus::Failed,
            "skip" => TestStatus::Skipped,
            _ => continue,
        };
        let duration_ms = event.get("Elapsed").and_then(Value::as_f64).map(|seconds| seconds * 1000.0);
        let message = if status == TestStatus::Failed { messages.get(&key).cloned().unwrap_or_default().trim_end().to_string() } else { String::new() };
        tests.push(TestCaseOutcome { name: name.to_string(), suite: package, status, duration_ms, message });
    }
    TestOutcome::finish(Runner::Go, tests, output)
}

/// 🟦️ Vitest's JSON reporter (the shape Jest's reporter also emits).
pub fn parse_vitest_json(output: &ProcessOutput) -> TestOutcome {
    let mut tests: Vec<TestCaseOutcome> = Vec::new();
    if let Ok(Value::Object(report)) = serde_json::from_str::<Value>(json_body(&output.stdout)) {
        for file in report.get("testResults").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default() {
            let suite = file.get("name").and_then(Value::as_str).unwrap_or("").to_string();
            for assertion in file.get("assertionResults").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default() {
                let name = assertion
                    .get("fullName")
                    .and_then(Value::as_str)
                    .or_else(|| assertion.get("title").and_then(Value::as_str))
                    .unwrap_or("")
                    .to_string();
                let status = match assertion.get("status").and_then(Value::as_str).unwrap_or("") {
                    "passed" => TestStatus::Passed,
                    "failed" => TestStatus::Failed,
                    _ => TestStatus::Skipped,
                };
                let message = assertion
                    .get("failureMessages")
                    .and_then(Value::as_array)
                    .map(|values| values.iter().filter_map(Value::as_str).collect::<Vec<_>>().join("\n"))
                    .unwrap_or_default();
                tests.push(TestCaseOutcome { name, suite: suite.clone(), status, duration_ms: assertion.get("duration").and_then(Value::as_f64), message });
            }
        }
    }
    TestOutcome::finish(Runner::Npx, tests, output)
}

/// 🐍️ pytest's verbose terminal report: `<nodeid> <VERDICT>` lines plus its failure sections.
pub fn parse_pytest(output: &ProcessOutput) -> TestOutcome {
    let mut tests: Vec<TestCaseOutcome> = Vec::new();
    for line in output.stdout.lines() {
        let trimmed = line.trim_end();
        let Some((node, tail)) = trimmed.split_once(' ') else { continue };
        if !node.contains("::") {
            continue;
        }
        let verdict = tail.split_whitespace().next().unwrap_or("");
        let status = match verdict {
            "PASSED" | "XPASS" => TestStatus::Passed,
            "FAILED" | "ERROR" => TestStatus::Failed,
            "SKIPPED" | "XFAIL" => TestStatus::Skipped,
            _ => continue,
        };
        let (suite, name) = node.split_once("::").unwrap_or(("", node));
        tests.push(TestCaseOutcome { name: name.to_string(), suite: suite.to_string(), status, duration_ms: None, message: String::new() });
    }
    TestOutcome::finish(Runner::Pytest, tests, output)
}

/// 🦀️ libtest's text report: `test <path> ... ok|FAILED|ignored`, with `---- <path> stdout ----` bodies.
pub fn parse_cargo_test(output: &ProcessOutput) -> TestOutcome {
    let mut tests: Vec<TestCaseOutcome> = Vec::new();
    for line in output.stdout.lines() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix("test ") else { continue };
        let Some((name, verdict)) = rest.split_once(" ... ") else { continue };
        let status = match verdict.trim() {
            "ok" => TestStatus::Passed,
            "FAILED" => TestStatus::Failed,
            value if value.starts_with("ignored") => TestStatus::Skipped,
            _ => continue,
        };
        let message = if status == TestStatus::Failed { cargo_failure_body(&output.stdout, name.trim()) } else { String::new() };
        tests.push(TestCaseOutcome { name: name.trim().to_string(), suite: String::new(), status, duration_ms: None, message });
    }
    TestOutcome::finish(Runner::Cargo, tests, output)
}

/// 🦀️ cargo-nextest's report: `        PASS [   0.012s] <crate> <test>`.
pub fn parse_cargo_nextest(output: &ProcessOutput) -> TestOutcome {
    let mut tests: Vec<TestCaseOutcome> = Vec::new();
    for line in output.stdout.lines().chain(output.stderr.lines()) {
        let trimmed = line.trim();
        let (verdict, rest) = match trimmed.split_once(' ') {
            Some(parts) => parts,
            None => continue,
        };
        let status = match verdict {
            "PASS" => TestStatus::Passed,
            "FAIL" | "TRY" => TestStatus::Failed,
            "SKIP" => TestStatus::Skipped,
            _ => continue,
        };
        let rest = rest.trim();
        let Some(close) = rest.find(']') else { continue };
        let duration_ms = rest[..close].trim_start_matches('[').trim().trim_end_matches('s').trim().parse::<f64>().ok().map(|seconds| seconds * 1000.0);
        let identifier = rest[close + 1..].trim();
        let (suite, name) = identifier.split_once(' ').unwrap_or(("", identifier));
        tests.push(TestCaseOutcome { name: name.trim().to_string(), suite: suite.trim().to_string(), status, duration_ms, message: String::new() });
    }
    TestOutcome::finish(Runner::CargoNextest, tests, output)
}

/// 🔷️ `dotnet test`'s console logger: `  Passed <name> [1 ms]`.
pub fn parse_dotnet_test(output: &ProcessOutput) -> TestOutcome {
    let mut tests: Vec<TestCaseOutcome> = Vec::new();
    for line in output.stdout.lines() {
        let trimmed = line.trim();
        let (verdict, rest) = match trimmed.split_once(' ') {
            Some(parts) => parts,
            None => continue,
        };
        let status = match verdict {
            "Passed" => TestStatus::Passed,
            "Failed" => TestStatus::Failed,
            "Skipped" => TestStatus::Skipped,
            _ => continue,
        };
        let rest = rest.trim();
        if rest.starts_with('!') || rest.starts_with('-') || rest.is_empty() {
            continue;
        }
        let (name, duration_ms) = match rest.rsplit_once(" [") {
            Some((name, tail)) => (name.trim().to_string(), parse_dotnet_duration(tail.trim_end_matches(']'))),
            None => (rest.to_string(), None),
        };
        tests.push(TestCaseOutcome { name, suite: String::new(), status, duration_ms, message: String::new() });
    }
    TestOutcome::finish(Runner::Dotnet, tests, output)
}

/// 💎️ RSpec's `--format json` document.
pub fn parse_rspec_json(output: &ProcessOutput) -> TestOutcome {
    let mut tests: Vec<TestCaseOutcome> = Vec::new();
    if let Ok(Value::Object(report)) = serde_json::from_str::<Value>(json_body(&output.stdout)) {
        for example in report.get("examples").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default() {
            let name = example
                .get("full_description")
                .and_then(Value::as_str)
                .or_else(|| example.get("description").and_then(Value::as_str))
                .unwrap_or("")
                .to_string();
            let status = match example.get("status").and_then(Value::as_str).unwrap_or("") {
                "passed" => TestStatus::Passed,
                "failed" => TestStatus::Failed,
                _ => TestStatus::Skipped,
            };
            let message = example.get("exception").and_then(|value| value.get("message")).and_then(Value::as_str).unwrap_or("").to_string();
            tests.push(TestCaseOutcome {
                name,
                suite: example.get("file_path").and_then(Value::as_str).unwrap_or("").to_string(),
                status,
                duration_ms: example.get("run_time").and_then(Value::as_f64).map(|seconds| seconds * 1000.0),
                message,
            });
        }
    }
    TestOutcome::finish(Runner::Rspec, tests, output)
}

/// 🧹️ The JSON document inside output that may carry leading runner chatter.
fn json_body(text: &str) -> &str {
    let start = text.find('{').unwrap_or(0);
    let end = text.rfind('}').map_or(text.len(), |index| index + 1);
    if start < end {
        &text[start..end]
    } else {
        text
    }
}

fn parse_dotnet_duration(value: &str) -> Option<f64> {
    let value = value.trim();
    let (number, unit) = value.split_once(' ')?;
    let number: f64 = number.parse().ok()?;
    match unit {
        "ms" => Some(number),
        "s" => Some(number * 1000.0),
        "us" => Some(number / 1000.0),
        _ => None,
    }
}

fn cargo_failure_body(stdout: &str, name: &str) -> String {
    let header = format!("---- {name} stdout ----");
    let Some(index) = stdout.find(&header) else { return String::new() };
    let rest = &stdout[index + header.len()..];
    let end = rest.find("\n----").unwrap_or(rest.len());
    rest[..end].trim().to_string()
}
//#endregion 📊️Parsing

//#region 🔎️Resolution
/// 🧰️ Go `pyTestRunnerBins`.
pub const PYTHON_TEST_RUNNER_BINS: [&str; 4] = ["pytest", "py.test", "nosetests", "nose2"];

/// 🧰️ Go `jsTestRunnerBins`.
pub const JS_TEST_RUNNER_BINS: [&str; 8] = ["jest", "vitest", "mocha", "jasmine", "ava", "tape", "qunit", "karma"];

/// 🏷️ Go `ToolKind`, restricted to the four verdicts `classifyCommandKind` can return.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CommandKind {
    /// 🧪️ A test command.
    Test,
    /// 🔎️ A read-only inspection command.
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

/// 🏷️ Go `classifyCommandKind`, ported in full because the test-file resolver branches on it.
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

/// ✂️ Go `trimPipelineTail`: everything up to the first unquoted `|`.
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

/// 🐍️ Go `resolvePytestFiles`: pytest naming plus `conftest.py`, honouring positional targets.
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

//#region 🧫️Vectors
// 🧫️ The fixture envelopes of this module's `🧪️tests`, described by `🧬️schema/🔣️.json`. They live in
// the crate rather than in an adapter so both implementations read one declared shape, and so a
// caller that wants to replay a recorded run has the same door the tests use.

/// 🧭️ One runner-detection vector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct DetectionVector {
    /// 🏷️ Scenario-local id.
    pub id: String,
    /// 📦️ The bundle root to probe.
    #[serde(rename = "bundleRoot", default)]
    pub bundle_root: String,
    /// 🎯️ Filter handed to the JavaScript runner detection.
    #[serde(rename = "testFilter", default)]
    pub test_filter: String,
    /// 🗣️ The language detection must answer.
    #[serde(rename = "expectedLanguage", default)]
    pub expected_language: String,
    /// 📜️ The JavaScript runner argv detection must answer, where the vector declares one.
    #[serde(rename = "expectedArgv", default, skip_serializing_if = "Option::is_none")]
    pub expected_argv: Option<Vec<String>>,
}

/// 🧭️ The runner-detection fixture.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct DetectionVectors {
    /// 🌍️ The world the vectors probe.
    #[serde(default)]
    pub snapshot: FilesystemSnapshot,
    /// 🧭️ The vectors.
    #[serde(default)]
    pub vectors: Vec<DetectionVector>,
}

/// 🗺️ One invocation-planning vector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct PlanningVector {
    /// 🏷️ Scenario-local id.
    pub id: String,
    /// 🔭️ The scope to plan.
    pub scope: TestScope,
    /// 🗺️ The plan the scope must produce.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<InvocationPlan>,
}

/// 🗺️ The invocation-planning fixture.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct PlanningVectors {
    /// 🌍️ The world the scopes are planned against.
    #[serde(default)]
    pub snapshot: FilesystemSnapshot,
    /// 🗺️ The vectors.
    #[serde(default)]
    pub vectors: Vec<PlanningVector>,
}

/// 🧬️ The scope-identifier fixture.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectorVectors {
    /// 🧬️ Raw selectors as a caller would type them.
    #[serde(default)]
    pub selectors: Vec<String>,
}

/// 📊️ One recorded runner report.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TranscriptVector {
    /// 🏷️ Scenario-local id.
    pub id: String,
    /// 🏭️ The runner that produced it.
    pub runner: Runner,
    /// 📤️ The recorded report.
    pub output: ProcessOutput,
}

/// 📊️ The result-parsing fixture.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct TranscriptVectors {
    /// 📊️ The vectors.
    #[serde(default)]
    pub vectors: Vec<TranscriptVector>,
}

/// 🛑️ The cancellation fixture.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct CancellationVectors {
    /// 🗺️ The plan to run.
    #[serde(default)]
    pub plan: InvocationPlan,
    /// 🎞️ The transcripts that answer it.
    #[serde(default)]
    pub transcripts: Vec<RecordedTranscript>,
    /// 🛑️ Cancel once this many invocations have completed.
    #[serde(rename = "cancelAfter", default)]
    pub cancel_after: usize,
}

/// 🧭️ Reads a runner-detection fixture.
pub fn parse_detection_vectors(bytes: &[u8]) -> Result<DetectionVectors, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}

/// 🗺️ Reads an invocation-planning fixture.
pub fn parse_planning_vectors(bytes: &[u8]) -> Result<PlanningVectors, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}

/// 🧬️ Reads a scope-identifier fixture.
pub fn parse_selector_vectors(bytes: &[u8]) -> Result<SelectorVectors, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}

/// 📊️ Reads a result-parsing fixture.
pub fn parse_transcript_vectors(bytes: &[u8]) -> Result<TranscriptVectors, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}

/// 🛑️ Reads a cancellation fixture.
pub fn parse_cancellation_vectors(bytes: &[u8]) -> Result<CancellationVectors, String> {
    serde_json::from_slice(bytes).map_err(|error| error.to_string())
}

/// 🗺️ Renders a plan as JSON text — concrete on purpose, so no serialization type crosses the API.
pub fn plan_to_json_text(plan: &InvocationPlan) -> String {
    serde_json::to_string(plan).unwrap_or_else(|error| format!("{{\"error\":{:?}}}", error.to_string()))
}

/// 🔭️ Renders a scope as JSON text.
pub fn scope_to_json_text(scope: &TestScope) -> String {
    serde_json::to_string(scope).unwrap_or_else(|error| format!("{{\"error\":{:?}}}", error.to_string()))
}

/// 📊️ Renders one outcome as JSON text.
pub fn outcome_to_json_text(outcome: &TestOutcome) -> String {
    serde_json::to_string(outcome).unwrap_or_else(|error| format!("{{\"error\":{:?}}}", error.to_string()))
}

/// 📊️ Renders a whole execution report as JSON text.
pub fn report_to_json_text(report: &ExecutionReport) -> String {
    serde_json::to_string(report).unwrap_or_else(|error| format!("{{\"error\":{:?}}}", error.to_string()))
}

/// 📣️ Renders a progress event stream as JSON text.
pub fn progress_to_json_text(events: &[ProgressEvent]) -> String {
    serde_json::to_string(events).unwrap_or_else(|error| format!("{{\"error\":{:?}}}", error.to_string()))
}
//#endregion 🧫️Vectors

//#region 💽️RealWorld
// 💽️ The only block in this crate that touches the machine: the walk that turns a real repository
// into a `FilesystemSnapshot`, the `ProcessRunner` that actually spawns, and the streaming spawn
// the CLI's `test` verb uses so a child's output reaches the terminal unbuffered.

/// 📖️ The manifest files planning probes for contents, by base name.
const SNAPSHOT_READ_NAMES: &[&str] = &["package.json"];

/// 📏️ A file larger than this is recorded as a path without contents.
const SNAPSHOT_READ_LIMIT: u64 = 1 << 20;

impl FilesystemSnapshot {
    /// 🌍️ Walks a real repository into the snapshot planning reads.
    ///
    /// Every non-ignored directory and file below `root` is recorded as a repository-relative,
    /// forward-slashed path. Contents are recorded only for the manifests a planning rule inspects
    /// ([`SNAPSHOT_READ_NAMES`]), so the walk stays a directory traversal rather than a full read;
    /// [`FilesystemSnapshot::hydrate`] loads any further file a narrower scope needs.
    pub fn from_root(root: &Path, ignore: Option<&semio_framework_repo_workspace::GitIgnore>) -> Self {
        let mut snapshot = FilesystemSnapshot { root: normalize_separators(&root.to_string_lossy()), uv_available: program_on_path("uv"), ..FilesystemSnapshot::default() };
        walk_into(root, root, ignore, &mut snapshot.entries);
        snapshot.entries.sort_by(|left, right| left.path.cmp(&right.path));
        snapshot
    }

    /// 📖️ Loads the contents of one already-recorded file from disk, so a scope that inspects a
    /// source file (a section or a definition run) sees the same text the reference read.
    pub fn hydrate(&mut self, path: &str) {
        let absolute = self.absolute(path);
        let Ok(contents) = std::fs::read_to_string(absolute.replace('/', std::path::MAIN_SEPARATOR_STR)) else { return };
        let wanted = clean_path(&absolute);
        match self.entries.iter_mut().find(|entry| entry.kind == EntryKind::File && clean_path(&entry.path) == wanted) {
            Some(entry) => entry.contents = Some(contents),
            None => self.entries.push(SnapshotEntry { path: wanted, kind: EntryKind::File, contents: Some(contents) }),
        }
    }

    /// 📦️ Replaces the bundle table planning resolves names and roots against.
    pub fn with_bundles(mut self, bundles: Vec<SnapshotBundle>) -> Self {
        self.bundles = bundles;
        self
    }

    /// 📦️ The snapshot planning actually reads: the bundle table plus one directory listing per
    /// bundle root.
    ///
    /// Every planning rule that touches the world probes a bundle root's own manifests
    /// (`go.mod`, `Cargo.toml`, `*.csproj`, `*.sln`, `package.json`, `pyproject.toml`,
    /// `requirements.txt`) or one named source file, which [`FilesystemSnapshot::hydrate`] loads on
    /// demand. Nothing walks the tree, so this is what the CLI's `test` verb builds — the reference
    /// stats those same manifests and never walks either.
    pub fn for_bundles(root: &Path, bundles: Vec<SnapshotBundle>) -> Self {
        let mut snapshot = FilesystemSnapshot { root: normalize_separators(&root.to_string_lossy()), bundles, uv_available: program_on_path("uv"), entries: Vec::new() };
        let roots: Vec<String> = snapshot.bundles.iter().map(|bundle| snapshot.absolute(&bundle.root)).collect();
        for absolute in roots {
            snapshot.entries.push(SnapshotEntry { path: absolute.clone(), kind: EntryKind::Directory, contents: None });
            let Ok(children) = std::fs::read_dir(absolute.replace('/', std::path::MAIN_SEPARATOR_STR)) else { continue };
            for child in children.flatten() {
                let Ok(kind) = child.file_type() else { continue };
                let path = normalize_separators(&child.path().to_string_lossy());
                if kind.is_dir() {
                    snapshot.entries.push(SnapshotEntry { path, kind: EntryKind::Directory, contents: None });
                    continue;
                }
                let contents = if SNAPSHOT_READ_NAMES.contains(&base_name(&path).as_str()) { std::fs::read_to_string(child.path()).ok() } else { None };
                snapshot.entries.push(SnapshotEntry { path, kind: EntryKind::File, contents });
            }
        }
        snapshot.entries.sort_by(|left, right| left.path.cmp(&right.path));
        snapshot.entries.dedup_by(|left, right| left.path == right.path);
        snapshot
    }
}

/// 🚶️ Records one directory level, descending into every directory the ignore rules keep.
fn walk_into(root: &Path, directory: &Path, ignore: Option<&semio_framework_repo_workspace::GitIgnore>, entries: &mut Vec<SnapshotEntry>) {
    let Ok(children) = std::fs::read_dir(directory) else { return };
    for child in children.flatten() {
        let path = child.path();
        let Ok(relative) = path.strip_prefix(root) else { continue };
        let relative = normalize_separators(&relative.to_string_lossy());
        if relative == ".git" || relative.ends_with("/.git") {
            continue;
        }
        if ignore.is_some_and(|rules| rules.matches_path(&relative)) {
            continue;
        }
        let absolute = normalize_separators(&path.to_string_lossy());
        let Ok(kind) = child.file_type() else { continue };
        if kind.is_symlink() {
            continue;
        }
        if kind.is_dir() {
            entries.push(SnapshotEntry { path: absolute, kind: EntryKind::Directory, contents: None });
            walk_into(root, &path, ignore, entries);
            continue;
        }
        let contents = if SNAPSHOT_READ_NAMES.contains(&base_name(&absolute).as_str()) && child.metadata().is_ok_and(|meta| meta.len() <= SNAPSHOT_READ_LIMIT) {
            std::fs::read_to_string(&path).ok()
        } else {
            None
        };
        entries.push(SnapshotEntry { path: absolute, kind: EntryKind::File, contents });
    }
}

/// 🔎️ Whether an executable of that name is reachable through `PATH` — Go `exec.LookPath`.
pub fn program_on_path(program: &str) -> bool {
    semio_framework_repo_workspace::program_exists(program)
}



/// 💽️ The [`ProcessRunner`] that actually spawns, capturing both streams.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemProcessRunner;

impl ProcessRunner for SystemProcessRunner {
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, String> {
        let mut command = semio_framework_repo_workspace::spawn_command(&request.program);
        command.args(&request.args);
        if !request.cwd.is_empty() {
            command.current_dir(request.cwd.replace('/', std::path::MAIN_SEPARATOR_STR));
        }
        for (name, value) in &request.env {
            command.env(name, value);
        }
        let output = command.output().map_err(|error| format!("{}: {error}", request.program))?;
        Ok(ProcessOutput {
            status: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}

/// ▶️ Spawns one planned invocation with the parent's streams, answering its exit code.
///
/// This is what the reference's `runExternalCommand` does: the child writes straight to the
/// terminal, so a long test run shows progress instead of arriving in one block at the end.
pub fn run_invocation_inherited(invocation: &RunnerInvocation) -> Result<i32, String> {
    let program = invocation.argv.first().cloned().unwrap_or_default();
    let mut command = semio_framework_repo_workspace::spawn_command(&program);
    command.args(invocation.argv.iter().skip(1));
    if !invocation.cwd.is_empty() {
        command.current_dir(invocation.cwd.replace('/', std::path::MAIN_SEPARATOR_STR));
    }
    for (name, value) in &invocation.env {
        command.env(name, value);
    }
    let status = command.status().map_err(|error| format!("{program}: {error}"))?;
    Ok(status.code().unwrap_or(-1))
}

/// 🏷️ The `Running: <argv> (in <dir>)` line the reference prints before every invocation.
pub fn running_line(invocation: &RunnerInvocation) -> String {
    format!("Running: {} (in {})\n", invocation.argv.join(" "), invocation.cwd.replace('/', std::path::MAIN_SEPARATOR_STR))
}
//#endregion 💽️RealWorld
