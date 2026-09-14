//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 🚚️ Repository move: every structural edit the repo CLI and MCP server can make to a checkout —
//! renaming a token in all of its casing variants, moving and deleting files and folders, creating,
//! renaming and deleting sections, extracting a section into its own file and integrating a file
//! into a section — expressed as a pure transformation from a [`Workspace`] to a [`Plan`] of
//! [`Change`]s. Nothing here touches a disk; an [`Executor`] applies a plan, and the in-memory
//! [`Workspace`] is itself one, so every operation is testable without a filesystem.
//! Behaviour twin of `github.com/usalu/semio/repo/move`.

//#endregion 🧲️Header

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use semio_framework_repo_events::{ExtractPayload, FilePayload, FolderPayload, IntegratePayload, SectionPayload};
use semio_framework_repo_identity::normalize_path;
use semio_framework_repo_languages::{format_section_both, format_section_end, format_section_start, pattern_match};

/// 🗣️ The section grammar this module edits. Re-exported explicitly so a client of the move crate
/// never has to reach past it for the types its own signatures speak in.
pub use semio_framework_repo_languages::{language_for_path, parse_sections, parse_sections_with, Language, Section};

//#region 🗄️Workspace

/// 📦️ What a workspace path holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entry {
    /// 📁️ A directory with no content of its own.
    Directory,
    /// 📄️ A text file and its content.
    File(String),
}

/// 🗄️ A checkout held entirely in memory: relative slash-separated paths to entries. Every planner
/// reads one and no planner writes one; a plan applied through [`Executor`] is the only mutation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Workspace {
    entries: BTreeMap<String, Entry>,
}

impl Workspace {
    /// 🆕️ An empty workspace.
    pub fn new() -> Self {
        Self { entries: BTreeMap::new() }
    }

    /// 📄️ Adds a file and every directory leading to it.
    pub fn with_file(mut self, path: &str, content: &str) -> Self {
        self.insert_file(path, content);
        self
    }

    /// 📁️ Adds a directory and every directory leading to it.
    pub fn with_directory(mut self, path: &str) -> Self {
        self.insert_directory(path);
        self
    }

    /// 📄️ Puts a file at `path`, materialising its ancestors.
    pub fn insert_file(&mut self, path: &str, content: &str) {
        let key = clean_relative(path);
        if key.is_empty() {
            return;
        }
        self.insert_ancestors(&key);
        self.entries.insert(key, Entry::File(content.to_string()));
    }

    /// 📁️ Puts a directory at `path`, materialising its ancestors.
    pub fn insert_directory(&mut self, path: &str) {
        let key = clean_relative(path);
        if key.is_empty() {
            return;
        }
        self.insert_ancestors(&key);
        self.entries.entry(key).or_insert(Entry::Directory);
    }

    fn insert_ancestors(&mut self, key: &str) {
        let mut prefix = String::new();
        let segments: Vec<&str> = key.split('/').collect();
        for segment in &segments[..segments.len().saturating_sub(1)] {
            if !prefix.is_empty() {
                prefix.push('/');
            }
            prefix.push_str(segment);
            self.entries.entry(prefix.clone()).or_insert(Entry::Directory);
        }
    }

    /// 🔍️ The entry at `path`.
    pub fn entry(&self, path: &str) -> Option<&Entry> {
        self.entries.get(&clean_relative(path))
    }

    /// 📖️ The content of the file at `path`.
    pub fn file(&self, path: &str) -> Option<&str> {
        match self.entries.get(&clean_relative(path)) {
            Some(Entry::File(content)) => Some(content.as_str()),
            _ => None,
        }
    }

    /// ✅️ Whether anything sits at `path`.
    pub fn exists(&self, path: &str) -> bool {
        self.entries.contains_key(&clean_relative(path))
    }

    /// 📁️ Whether a directory sits at `path`.
    pub fn is_directory(&self, path: &str) -> bool {
        matches!(self.entries.get(&clean_relative(path)), Some(Entry::Directory))
    }

    /// 🗂️ Every path, in sorted order.
    pub fn paths(&self) -> Vec<String> {
        self.entries.keys().cloned().collect()
    }

    /// 🗂️ Every path and entry, in sorted order.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Entry)> {
        self.entries.iter()
    }

    /// 🧾️ Every file path mapped to its content, in sorted order.
    pub fn files(&self) -> BTreeMap<String, String> {
        self.entries
            .iter()
            .filter_map(|(path, entry)| match entry {
                Entry::File(content) => Some((path.clone(), content.clone())),
                Entry::Directory => None,
            })
            .collect()
    }

    /// 🌳️ Every path inside `root`, `root` itself excluded.
    pub fn descendants(&self, root: &str) -> Vec<String> {
        let prefix = format!("{}/", clean_relative(root));
        self.entries.keys().filter(|p| p.starts_with(&prefix)).cloned().collect()
    }
}

/// 🧹️ A path reduced to the slash-separated relative form every planner speaks.
pub fn clean_relative(path: &str) -> String {
    let unified = path.replace('\\', "/");
    let mut segments: Vec<&str> = Vec::new();
    for segment in unified.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                segments.pop();
            }
            other => segments.push(other),
        }
    }
    segments.join("/")
}

//#endregion 🗄️Workspace

//#region 🧾️Plan

/// ✏️ One edit of a plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Change {
    /// 📝️ Replaces or creates the file at `path`.
    Write { path: String, content: String },
    /// 🚚️ Moves `from` to `to`; a directory moves with everything under it.
    Move { from: String, to: String },
    /// 🗑️ Removes `path` and, for a directory, everything under it.
    Delete { path: String },
    /// 📁️ Creates the directory at `path` and every directory leading to it.
    CreateDirectory { path: String },
}

/// 📣️ One line the Go twin writes to its command output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    /// ℹ️ Progress detail.
    Info(String),
    /// ✅️ The closing line of a successful operation.
    Success(String),
}

impl Message {
    /// 📝️ The line text without its level.
    pub fn text(&self) -> &str {
        match self {
            Message::Info(text) | Message::Success(text) => text.as_str(),
        }
    }
}

/// 📡️ One event the Go twin emits around an operation.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanEvent {
    /// 🏷️ The event kind, such as `file.move.starting`.
    pub kind: String,
    /// 🎁️ The payload, already encoded.
    pub payload: serde_json::Value,
}

impl PlanEvent {
    fn new<T: serde::Serialize>(kind: &str, payload: &T) -> Self {
        Self { kind: kind.to_string(), payload: serde_json::to_value(payload).unwrap_or(serde_json::Value::Null) }
    }
}

/// 🧾️ Everything an operation decided: what to change, what to say, what to emit, what to count.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Plan {
    /// ✏️ The edits, in application order.
    pub changes: Vec<Change>,
    /// 📣️ The command output lines.
    pub messages: Vec<Message>,
    /// 📡️ The events, starting event first.
    pub events: Vec<PlanEvent>,
    /// 🔢️ The numeric result the Go twin returns as tool data.
    pub stats: BTreeMap<String, i64>,
}

impl Plan {
    fn empty() -> Self {
        Self::default()
    }

    /// 📝️ Every message text in order.
    pub fn lines(&self) -> Vec<String> {
        self.messages.iter().map(|m| m.text().to_string()).collect()
    }
}

/// 🚫️ Why an operation refused to produce a plan. The message is the Go twin's, verbatim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveError(pub String);

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for MoveError {}

fn fail<T>(message: impl Into<String>) -> Result<T, MoveError> {
    Err(MoveError(message.into()))
}

//#endregion 🧾️Plan

//#region 🎬️Executor

/// 🎬️ Applies the changes of a plan to some backing store.
pub trait Executor {
    /// ✏️ Applies one change.
    fn apply(&mut self, change: &Change) -> Result<(), MoveError>;
}

/// 🎬️ Applies every change of `plan` in order, stopping at the first failure.
pub fn execute(plan: &Plan, executor: &mut dyn Executor) -> Result<(), MoveError> {
    for change in &plan.changes {
        executor.apply(change)?;
    }
    Ok(())
}

impl Executor for Workspace {
    fn apply(&mut self, change: &Change) -> Result<(), MoveError> {
        match change {
            Change::Write { path, content } => {
                self.insert_file(path, content);
                Ok(())
            }
            Change::CreateDirectory { path } => {
                self.insert_directory(path);
                Ok(())
            }
            Change::Delete { path } => {
                let key = clean_relative(path);
                let prefix = format!("{key}/");
                self.entries.retain(|p, _| p != &key && !p.starts_with(&prefix));
                Ok(())
            }
            Change::Move { from, to } => {
                let source = clean_relative(from);
                let target = clean_relative(to);
                let Some(entry) = self.entries.remove(&source) else {
                    return fail(format!("Source not found: {source}"));
                };
                let prefix = format!("{source}/");
                let moved: Vec<(String, Entry)> = self
                    .entries
                    .keys()
                    .filter(|p| p.starts_with(&prefix))
                    .cloned()
                    .collect::<Vec<String>>()
                    .into_iter()
                    .filter_map(|p| self.entries.remove(&p).map(|e| (format!("{}/{}", target, &p[prefix.len()..]), e)))
                    .collect();
                self.insert_ancestors(&target);
                self.entries.insert(target, entry);
                for (path, entry) in moved {
                    self.entries.insert(path, entry);
                }
                Ok(())
            }
        }
    }
}

/// 💾️ Applies changes to a real directory tree rooted at a path.
#[derive(Debug, Clone)]
pub struct FileSystemExecutor {
    root: PathBuf,
}

impl FileSystemExecutor {
    /// 🆕️ An executor writing under `root`.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn absolute(&self, path: &str) -> PathBuf {
        let mut target = self.root.clone();
        for segment in clean_relative(path).split('/').filter(|s| !s.is_empty()) {
            target.push(segment);
        }
        target
    }

    fn ensure_parent(target: &Path) -> Result<(), MoveError> {
        match target.parent() {
            Some(parent) => fs::create_dir_all(parent).map_err(|error| MoveError(error.to_string())),
            None => Ok(()),
        }
    }
}

impl Executor for FileSystemExecutor {
    fn apply(&mut self, change: &Change) -> Result<(), MoveError> {
        match change {
            Change::Write { path, content } => {
                let target = self.absolute(path);
                Self::ensure_parent(&target)?;
                fs::write(&target, content).map_err(|error| MoveError(error.to_string()))
            }
            Change::CreateDirectory { path } => fs::create_dir_all(self.absolute(path)).map_err(|error| MoveError(error.to_string())),
            Change::Delete { path } => {
                let target = self.absolute(path);
                let result = if target.is_dir() { fs::remove_dir_all(&target) } else { fs::remove_file(&target) };
                result.map_err(|error| MoveError(error.to_string()))
            }
            Change::Move { from, to } => {
                let target = self.absolute(to);
                Self::ensure_parent(&target)?;
                fs::rename(self.absolute(from), &target).map_err(|error| MoveError(error.to_string()))
            }
        }
    }
}

//#endregion 🎬️Executor

//#region 🔤️Casing

/// 🅰️ `value` with its first byte upper-cased and the rest lower-cased, as the Go twin's byte
/// slicing does. Case mapping is the simple one-to-one mapping Go's `strings` package uses, so a
/// rune whose full mapping grows — `ß` — is left alone rather than expanded.
pub fn title_case_token(value: &str) -> String {
    if value.is_empty() {
        return String::new();
    }
    let bytes = value.as_bytes();
    let head = bytes[0].to_ascii_uppercase();
    let tail = match std::str::from_utf8(&bytes[1..]) {
        Ok(rest) => simple_lower(rest),
        Err(_) => String::from_utf8_lossy(&bytes[1..]).into_owned(),
    };
    let mut out = String::with_capacity(value.len());
    out.push(head as char);
    out.push_str(&tail);
    out
}

/// 🔠️ Go's `strings.ToUpper`: the simple, length-preserving case mapping.
pub fn simple_upper(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            let mut mapped = c.to_uppercase();
            match (mapped.next(), mapped.next()) {
                (Some(single), None) => single,
                _ => c,
            }
        })
        .collect()
}

/// 🔡️ Go's `strings.ToLower`: the simple, length-preserving case mapping.
pub fn simple_lower(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            let mut mapped = c.to_lowercase();
            match (mapped.next(), mapped.next()) {
                (Some(single), None) => single,
                _ => c,
            }
        })
        .collect()
}

/// 🔤️ `content` with the UPPER, Title and lower variants of `old_token` rewritten to the matching
/// variant of `new_token`. A token spelled in kebab, camel, pascal, snake or screaming style is
/// carried through unchanged apart from those three case foldings, exactly as the Go twin does.
pub fn apply_rename_casings(content: &str, old_token: &str, new_token: &str) -> String {
    let old_upper = simple_upper(old_token);
    let new_upper = simple_upper(new_token);
    let old_lower = simple_lower(old_token);
    let new_lower = simple_lower(new_token);
    let old_title = title_case_token(&old_lower);
    let new_title = title_case_token(&new_lower);
    let mut out = content.to_string();
    if old_upper != old_lower {
        out = out.replace(&old_upper, &new_upper);
    }
    if old_title != old_upper && old_title != old_lower {
        out = out.replace(&old_title, &new_title);
    }
    out.replace(&old_lower, &new_lower)
}

//#endregion 🔤️Casing

//#region 🗣️Language Helpers

/// 🔎️ The first section named `name`, searched depth first through the children.
pub fn find_section<'a>(sections: &'a [Section], name: &str) -> Option<&'a Section> {
    for section in sections {
        if section.name == name {
            return Some(section);
        }
        if let Some(found) = find_section(&section.children, name) {
            return Some(found);
        }
    }
    None
}

/// 🔤️ `input` with every repeat dropped, first occurrence kept.
pub fn unique_strings(input: &[String]) -> Vec<String> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut list = Vec::with_capacity(input.len());
    for entry in input {
        if seen.insert(entry.as_str()) {
            list.push(entry.clone());
        }
    }
    list
}

/// 📜️ Whether `line` opens a policy section, and the name it opens.
pub fn policy_section_start_match(lang: &Language, line: &str) -> Option<String> {
    policy_match(lang.policy_section_start.as_ref(), line)
}

/// 🟥️ Whether `line` closes a policy section, and the name it closes.
pub fn policy_section_end_match(lang: &Language, line: &str) -> Option<String> {
    policy_match(lang.policy_section_end.as_ref(), line)
}

fn policy_match(pattern: Option<&semio_framework_repo_languages::Pattern>, line: &str) -> Option<String> {
    let pattern = pattern?;
    let (_, captures) = pattern_match(pattern, line)?;
    let raw = captures.iter().find(|(key, _)| key == "name").map(|(_, value)| value.clone()).unwrap_or_default();
    let (_, rest) = semio_framework_repo_languages::extract_entity_emoji(raw.trim());
    Some(rest.trim().to_string())
}

/// ✂️ `content` split into its header region and the body after it.
pub fn split_header(content: &str, lang: &Language) -> (String, String) {
    for section in parse_sections_with(lang, content) {
        if section.name.eq_ignore_ascii_case("Header") {
            let cut = byte_cut(content, section.end_index);
            return (content[..cut].to_string(), content[cut..].to_string());
        }
    }
    (String::new(), content.to_string())
}

/// 🔀️ `source_header`'s new lines folded into `target_header` above its closing marker.
pub fn merge_headers(target_header: &str, source_header: &str, lang: &Language) -> String {
    if target_header.is_empty() {
        return source_header.to_string();
    }
    if source_header.is_empty() {
        return target_header.to_string();
    }
    let target_lines: Vec<&str> = target_header.split('\n').collect();
    let seen: BTreeSet<&str> = target_lines.iter().map(|line| line.trim()).collect();
    let mut insert_index: Option<usize> = None;
    for (index, line) in target_lines.iter().enumerate() {
        if policy_section_end_match(lang, line).is_some() {
            insert_index = Some(index);
        }
    }
    let Some(insert_index) = insert_index else {
        return target_header.to_string();
    };
    let mut new_lines: Vec<&str> = Vec::new();
    for line in source_header.split('\n') {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if policy_section_start_match(lang, line).is_some() || policy_section_end_match(lang, line).is_some() {
            continue;
        }
        if !seen.contains(trimmed) {
            new_lines.push(line);
        }
    }
    if new_lines.is_empty() {
        return target_header.to_string();
    }
    let mut result: Vec<&str> = Vec::with_capacity(target_lines.len() + new_lines.len());
    result.extend_from_slice(&target_lines[..insert_index]);
    result.extend_from_slice(&new_lines);
    result.extend_from_slice(&target_lines[insert_index..]);
    result.join("\n")
}

/// 📦️ The package declaration of `content` and the body without it.
pub fn extract_package(lang: &Language, content: &str) -> (String, String) {
    if lang.name != "go" {
        return (String::new(), content.to_string());
    }
    let mut package = String::new();
    let mut found = false;
    let mut body: Vec<&str> = Vec::new();
    for line in content.split('\n') {
        if !found && line.trim_start().starts_with("package ") {
            package = line.to_string();
            found = true;
            continue;
        }
        body.push(line);
    }
    (package, body.join("\n"))
}

/// 🧲️ The import lines of `content` and the body without them.
pub fn extract_imports(lang: &Language, content: &str) -> (Vec<String>, String) {
    match lang.name.as_str() {
        "typescript" => extract_typescript_imports(content),
        "go" => extract_go_imports(content),
        "python" => extract_prefixed_imports(content, &["import ", "from "], false),
        "csharp" => extract_prefixed_imports(content, &["using "], true),
        _ => (Vec::new(), content.to_string()),
    }
}

fn extract_typescript_imports(content: &str) -> (Vec<String>, String) {
    let lines: Vec<&str> = content.split('\n').collect();
    let mut imports: Vec<String> = Vec::new();
    let mut body: Vec<&str> = Vec::new();
    let mut i = 0usize;
    while i < lines.len() {
        let line = lines[i];
        if line.trim().starts_with("import ") {
            let mut current = line.to_string();
            if !line.contains(';') {
                let mut j = i + 1;
                while j < lines.len() {
                    current.push('\n');
                    current.push_str(lines[j]);
                    if lines[j].contains(';') {
                        i = j;
                        break;
                    }
                    j += 1;
                }
            }
            imports.push(current);
        } else {
            body.push(line);
        }
        i += 1;
    }
    (imports, body.join("\n"))
}

fn extract_go_imports(content: &str) -> (Vec<String>, String) {
    let mut imports: Vec<String> = Vec::new();
    let mut body: Vec<&str> = Vec::new();
    let mut in_block = false;
    for line in content.split('\n') {
        let trimmed = line.trim();
        if in_block {
            if trimmed == ")" {
                in_block = false;
            } else if !trimmed.is_empty() {
                imports.push(trimmed.trim_matches(',').to_string());
            }
            continue;
        }
        if trimmed.starts_with("import (") {
            in_block = true;
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("import ") {
            imports.push(rest.to_string());
            continue;
        }
        body.push(line);
    }
    (imports, body.join("\n"))
}

fn extract_prefixed_imports(content: &str, prefixes: &[&str], require_semicolon: bool) -> (Vec<String>, String) {
    let mut imports: Vec<String> = Vec::new();
    let mut body: Vec<&str> = Vec::new();
    for line in content.split('\n') {
        let trimmed = line.trim();
        let matched = prefixes.iter().any(|prefix| trimmed.starts_with(prefix)) && (!require_semicolon || trimmed.ends_with(';'));
        if matched {
            imports.push(line.to_string());
        } else {
            body.push(line);
        }
    }
    (imports, body.join("\n"))
}

/// 📥️ `imports` written back the way `lang` spells an import block.
pub fn format_imports(lang: &Language, imports: &[String]) -> String {
    if imports.is_empty() {
        return String::new();
    }
    let unique = unique_strings(imports);
    match lang.name.as_str() {
        "typescript" => unique.join("\n"),
        "go" => {
            let mut sorted = unique;
            sorted.sort();
            let mut block = String::from("import (\n");
            for entry in &sorted {
                block.push('\t');
                block.push_str(entry);
                block.push('\n');
            }
            block.push(')');
            block
        }
        "python" | "csharp" => {
            let mut sorted = unique;
            sorted.sort();
            sorted.join("\n")
        }
        _ => String::new(),
    }
}

fn byte_cut(content: &str, index: i64) -> usize {
    let mut cut = index.max(0) as usize;
    if cut > content.len() {
        cut = content.len();
    }
    while cut < content.len() && !content.is_char_boundary(cut) {
        cut += 1;
    }
    cut
}

fn ensure_trailing_newline(value: &mut String) {
    if !value.is_empty() && !value.ends_with('\n') {
        value.push('\n');
    }
}

//#endregion 🗣️Language Helpers

//#region 🔤️Rename

const RENAME_SKIPPED_DIRECTORIES: [&str; 2] = [".git", "node_modules"];

/// 🔤️ Rewrites every UPPER, Title and lower variant of `old_token` to `new_token` across the
/// contents and the names of everything under `scope`, deepest path first so a parent rename never
/// invalidates a child's plan.
///
/// The scope root is part of the rename: naming a scope narrows *where* occurrences are rewritten,
/// it does not exempt the scope itself, so a scope directory whose own name carries the token is
/// renamed like any other directory — and one that does not is left alone, like any other.
pub fn plan_rename(workspace: &Workspace, old_token: &str, new_token: &str, scope: &str) -> Result<Plan, MoveError> {
    if old_token.is_empty() || new_token.is_empty() {
        return fail("Old and new token must be non-empty");
    }
    if simple_lower(old_token) == simple_lower(new_token) {
        return fail("Old and new token are identical");
    }
    let scope = clean_relative(scope);
    if !scope.is_empty() && !workspace.is_directory(&scope) {
        return fail(format!("Scope is not a directory: {scope}"));
    }
    let mut files: Vec<String> = Vec::new();
    let mut directories: Vec<String> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();
    for (path, entry) in workspace.iter() {
        if !scope.is_empty() && path != &scope && !path.starts_with(&format!("{scope}/")) {
            continue;
        }
        if skipped.iter().any(|root| path.starts_with(&format!("{root}/"))) {
            continue;
        }
        let base = path.rsplit('/').next().unwrap_or(path.as_str());
        if matches!(entry, Entry::Directory) && RENAME_SKIPPED_DIRECTORIES.contains(&base) {
            skipped.push(path.clone());
            continue;
        }
        match entry {
            Entry::Directory => directories.push(path.clone()),
            Entry::File(_) => files.push(path.clone()),
        }
    }

    let mut plan = Plan::empty();
    let mut files_changed = 0i64;
    for path in &files {
        let Some(original) = workspace.file(path) else { continue };
        let replaced = apply_rename_casings(original, old_token, new_token);
        if replaced == original {
            continue;
        }
        plan.changes.push(Change::Write { path: path.clone(), content: replaced });
        files_changed += 1;
    }

    let mut entries: Vec<(String, bool)> = Vec::with_capacity(files.len() + directories.len());
    entries.extend(files.iter().map(|path| (path.clone(), false)));
    entries.extend(directories.iter().map(|path| (path.clone(), true)));
    entries.sort_by(|left, right| {
        let left_depth = left.0.matches('/').count();
        let right_depth = right.0.matches('/').count();
        right_depth.cmp(&left_depth).then_with(|| right.0.cmp(&left.0))
    });

    let mut occupied: BTreeSet<String> = workspace.paths().into_iter().collect();
    let mut files_renamed = 0i64;
    let mut folders_renamed = 0i64;
    for (path, is_directory) in entries {
        let base = path.rsplit('/').next().unwrap_or(path.as_str()).to_string();
        let new_base = apply_rename_casings(&base, old_token, new_token);
        if new_base == base {
            continue;
        }
        let parent = match path.rfind('/') {
            Some(index) => path[..index].to_string(),
            None => String::new(),
        };
        let new_path = if parent.is_empty() { new_base } else { format!("{parent}/{new_base}") };
        if occupied.contains(&new_path) {
            return fail(format!("Rename target already exists: {new_path}"));
        }
        occupied.remove(&path);
        occupied.insert(new_path.clone());
        plan.changes.push(Change::Move { from: path, to: new_path });
        if is_directory {
            folders_renamed += 1;
        } else {
            files_renamed += 1;
        }
    }

    plan.messages.push(Message::Success(format!(
        "\n🔤️Renamed {old_token} → {new_token}: {files_changed} files edited, {files_renamed} files renamed, {folders_renamed} folders renamed"
    )));
    plan.stats.insert("filesChanged".to_string(), files_changed);
    plan.stats.insert("filesRenamed".to_string(), files_renamed);
    plan.stats.insert("foldersRenamed".to_string(), folders_renamed);
    Ok(plan)
}

//#endregion 🔤️Rename

//#region 📁️Folders

/// 📁️ Creates an empty folder.
pub fn plan_folder_create(workspace: &Workspace, path: &str) -> Result<Plan, MoveError> {
    if workspace.exists(path) {
        return fail(format!("Folder already exists: {path}"));
    }
    let mut plan = Plan::empty();
    let payload = FolderPayload { path: path.to_string(), ..FolderPayload::default() };
    plan.events.push(PlanEvent::new("folder.create.starting", &payload));
    plan.changes.push(Change::CreateDirectory { path: clean_relative(path) });
    plan.events.push(PlanEvent::new("folder.create.ended", &payload));
    plan.messages.push(Message::Success(format!("\n📁️Created folder: {path}")));
    Ok(plan)
}

/// 🚚️ Moves a folder and rewrites the `AGENTS.md` headings that name it.
pub fn plan_folder_move(workspace: &Workspace, source: &str, target: &str) -> Result<Plan, MoveError> {
    if !workspace.exists(source) {
        return fail(format!("Source folder not found: {source}"));
    }
    if workspace.exists(target) {
        return fail(format!("Target folder already exists: {target}"));
    }
    let mut plan = Plan::empty();
    let payload = FolderPayload { path: target.to_string(), from: source.to_string(), ..FolderPayload::default() };
    plan.events.push(PlanEvent::new("folder.move.starting", &payload));
    plan.changes.push(Change::Move { from: clean_relative(source), to: clean_relative(target) });
    if let Some(change) = plan_agents_docs_path(workspace, source, target) {
        plan.changes.push(change);
    }
    plan.events.push(PlanEvent::new("folder.move.ended", &payload));
    plan.messages.push(Message::Success(format!("\n📁️Moved folder: {source} → {target}")));
    Ok(plan)
}

/// 🗑️ Deletes a folder and everything under it.
pub fn plan_folder_delete(workspace: &Workspace, path: &str) -> Result<Plan, MoveError> {
    if !workspace.exists(path) {
        return fail(format!("Folder not found: {path}"));
    }
    let mut plan = Plan::empty();
    let payload = FolderPayload { path: path.to_string(), ..FolderPayload::default() };
    plan.events.push(PlanEvent::new("folder.delete.starting", &payload));
    plan.changes.push(Change::Delete { path: clean_relative(path) });
    plan.events.push(PlanEvent::new("folder.delete.ended", &payload));
    plan.messages.push(Message::Success(format!("\n🗑️ Deleted folder: {path}")));
    Ok(plan)
}

//#endregion 📁️Folders

//#region 📄️Files

/// 📄️ Creates a file holding `header`, the caller's rendered file header.
pub fn plan_file_create(workspace: &Workspace, path: &str, header: &str) -> Result<Plan, MoveError> {
    if workspace.exists(path) {
        return fail(format!("File already exists: {path}"));
    }
    let mut plan = Plan::empty();
    let payload = FilePayload { path: path.to_string(), ..FilePayload::default() };
    plan.events.push(PlanEvent::new("file.create.starting", &payload));
    plan.changes.push(Change::Write { path: clean_relative(path), content: header.to_string() });
    plan.events.push(PlanEvent::new("file.create.ended", &payload));
    plan.messages.push(Message::Success(format!("\n📄️Created file: {path}")));
    Ok(plan)
}

/// 🚚️ Moves a file and rewrites the `AGENTS.md` headings that name it.
pub fn plan_file_move(workspace: &Workspace, source: &str, target: &str) -> Result<Plan, MoveError> {
    if !workspace.exists(source) {
        return fail(format!("Source file not found: {source}"));
    }
    if workspace.exists(target) {
        return fail(format!("Target file already exists: {target}"));
    }
    let mut plan = Plan::empty();
    let payload = FilePayload { path: target.to_string(), from: source.to_string(), ..FilePayload::default() };
    plan.events.push(PlanEvent::new("file.move.starting", &payload));
    plan.changes.push(Change::Move { from: clean_relative(source), to: clean_relative(target) });
    if let Some(change) = plan_agents_docs_path(workspace, source, target) {
        plan.changes.push(change);
    }
    plan.events.push(PlanEvent::new("file.move.ended", &payload));
    plan.messages.push(Message::Success(format!("\n📄️Moved file: {source} → {target}")));
    Ok(plan)
}

/// 🗑️ Deletes a file.
pub fn plan_file_delete(workspace: &Workspace, path: &str) -> Result<Plan, MoveError> {
    if !workspace.exists(path) {
        return fail(format!("File not found: {path}"));
    }
    let mut plan = Plan::empty();
    let payload = FilePayload { path: path.to_string(), ..FilePayload::default() };
    plan.events.push(PlanEvent::new("file.delete.starting", &payload));
    plan.changes.push(Change::Delete { path: clean_relative(path) });
    plan.events.push(PlanEvent::new("file.delete.ended", &payload));
    plan.messages.push(Message::Success(format!("\n🗑️ Deleted file: {path}")));
    Ok(plan)
}

//#endregion 📄️Files

//#region 📚️Agents Docs

const AGENTS_DOCS_PATH: &str = "AGENTS.md";

/// 🎛️ The `AGENTS.md` rewrite a file or folder move implies, or nothing when no heading names it.
pub fn plan_agents_docs_path(workspace: &Workspace, old_path: &str, new_path: &str) -> Option<Change> {
    let content = workspace.file(AGENTS_DOCS_PATH)?;
    let old_normalized = normalize_path(old_path);
    let new_normalized = normalize_path(new_path);
    if old_normalized == new_normalized {
        return None;
    }
    let mut changed = false;
    let lines: Vec<String> = content
        .split('\n')
        .map(|line| {
            if !line.starts_with("## ") && !line.starts_with("### ") {
                return line.to_string();
            }
            match line.find(&old_normalized) {
                Some(index) => {
                    changed = true;
                    format!("{}{}{}", &line[..index], new_normalized, &line[index + old_normalized.len()..])
                }
                None => line.to_string(),
            }
        })
        .collect();
    if !changed {
        return None;
    }
    Some(Change::Write { path: AGENTS_DOCS_PATH.to_string(), content: lines.join("\n") })
}

/// 🧹️ The `AGENTS.md` rewrite that drops the `## ` block naming `file_path`.
pub fn plan_agents_docs_removal(workspace: &Workspace, file_path: &str) -> Option<Change> {
    let content = workspace.file(AGENTS_DOCS_PATH)?;
    let normalized = normalize_path(file_path);
    let mut kept: Vec<&str> = Vec::new();
    let mut skip = false;
    for line in content.split('\n') {
        if line.starts_with('#') {
            skip = false;
            if let Some(header) = line.strip_prefix("## ") {
                let clean: String = header.chars().filter(|c| *c != '\u{FE0E}' && *c != '\u{FE0F}').collect();
                let runes: Vec<char> = clean.chars().collect();
                if runes.len() > 1 {
                    let path_part: String = runes[1..].iter().collect();
                    if path_part.trim_end_matches('/') == normalized {
                        skip = true;
                        continue;
                    }
                }
            }
        }
        if !skip {
            kept.push(line);
        }
    }
    let new_content = kept.join("\n");
    if new_content == content {
        return None;
    }
    Some(Change::Write { path: AGENTS_DOCS_PATH.to_string(), content: new_content })
}

//#endregion 📚️Agents Docs

//#region 📑️Sections

fn last_segment(section_path: &str) -> String {
    section_path.rsplit('#').next().unwrap_or(section_path).to_string()
}

fn require_file<'a>(workspace: &'a Workspace, file_path: &str) -> Result<&'a str, MoveError> {
    match workspace.file(file_path) {
        Some(content) => Ok(content),
        None => fail(format!("File not found: {file_path}")),
    }
}

/// 🏷️ Appends an empty section skeleton to a file.
pub fn plan_section_create(workspace: &Workspace, file_path: &str, section_path: &str) -> Result<Plan, MoveError> {
    let content = require_file(workspace, file_path)?;
    let section_name = last_segment(section_path);
    let Some(lang) = language_for_path(file_path) else {
        return fail("Unsupported file type");
    };
    if !lang.supports_sections() {
        return fail("Unsupported file type");
    }
    let skeleton = format_section_both(lang, &section_name);
    if skeleton.is_empty() {
        return fail("Cannot create section for this file type");
    }
    let mut plan = Plan::empty();
    let payload = SectionPayload { file: file_path.to_string(), name: section_path.to_string(), ..SectionPayload::default() };
    plan.events.push(PlanEvent::new("section.create.starting", &payload));
    plan.changes.push(Change::Write { path: clean_relative(file_path), content: format!("{content}{skeleton}") });
    plan.events.push(PlanEvent::new("section.create.ended", &payload));
    plan.messages.push(Message::Success(format!("\n🏷️Created section \"{section_name}\" in {file_path}")));
    Ok(plan)
}

/// 🏷️ Renames a section by rewriting its opening and closing markers in place.
pub fn plan_section_move(workspace: &Workspace, file_path: &str, old_path: &str, new_path: &str) -> Result<Plan, MoveError> {
    let original = require_file(workspace, file_path)?;
    let old_name = last_segment(old_path);
    let new_name = last_segment(new_path);
    let mut content = original.to_string();
    if let Some(lang) = language_for_path(file_path) {
        if lang.supports_sections() {
            let old_start = format_section_start(lang, &old_name);
            let new_start = format_section_start(lang, &new_name);
            if !old_start.is_empty() && !new_start.is_empty() {
                content = content.replace(&old_start, &new_start);
            }
            let old_end = format_section_end(lang, &old_name);
            let new_end = format_section_end(lang, &new_name);
            if !old_end.is_empty() && !new_end.is_empty() {
                content = content.replace(&old_end, &new_end);
            }
            if lang.name == "markdown" {
                content = content.replace(&format!("# {old_name}"), &format!("# {new_name}"));
            }
        }
    }
    let mut plan = Plan::empty();
    let payload = SectionPayload {
        file: file_path.to_string(),
        name: new_path.to_string(),
        old_name: old_path.to_string(),
        ..SectionPayload::default()
    };
    plan.events.push(PlanEvent::new("section.move.starting", &payload));
    plan.changes.push(Change::Write { path: clean_relative(file_path), content });
    plan.events.push(PlanEvent::new("section.move.ended", &payload));
    plan.messages.push(Message::Success(format!("\n🏷️Renamed section \"{old_name}\" to \"{new_name}\" in {file_path}")));
    Ok(plan)
}

/// 🗑️ Removes every line of a section, markers included.
pub fn plan_section_delete(workspace: &Workspace, file_path: &str, section_path: &str) -> Result<Plan, MoveError> {
    let content = require_file(workspace, file_path)?;
    let section_name = last_segment(section_path);
    let Some(lang) = language_for_path(file_path) else {
        return fail(format!("Section not found: {section_name}"));
    };
    let sections = parse_sections_with(lang, content);
    let Some(section) = find_section(&sections, &section_name) else {
        return fail(format!("Section not found: {section_name}"));
    };
    let kept: Vec<&str> = content
        .split('\n')
        .enumerate()
        .filter(|(index, _)| {
            let line_number = (*index as i64) + 1;
            line_number < section.start_line || line_number > section.end_line
        })
        .map(|(_, line)| line)
        .collect();
    let mut plan = Plan::empty();
    let payload = SectionPayload { file: file_path.to_string(), name: section_path.to_string(), ..SectionPayload::default() };
    plan.events.push(PlanEvent::new("section.delete.starting", &payload));
    plan.changes.push(Change::Write { path: clean_relative(file_path), content: kept.join("\n") });
    plan.events.push(PlanEvent::new("section.delete.ended", &payload));
    plan.messages.push(Message::Success(format!("\n🗑️ Deleted section \"{section_name}\" from {file_path}")));
    Ok(plan)
}

//#endregion 📑️Sections

//#region 📥️Integrate

/// 🧬️ Folds a whole source file into a named section of a target file, merging the two headers and
/// the two import lists and keeping the target's package declaration.
pub fn plan_integrate(
    workspace: &Workspace,
    source_path: &str,
    target_section_name: &str,
    target_file_path: &str,
    target_parent_section_name: &str,
) -> Result<Plan, MoveError> {
    let Some(source_content) = workspace.file(source_path) else {
        return fail(format!("Source file not found: {source_path}"));
    };
    let Some(target_content) = workspace.file(target_file_path) else {
        return fail(format!("Target file not found: {target_file_path}"));
    };
    let Some(lang) = language_for_path(target_file_path) else {
        return fail("Target file type does not support sections");
    };
    if !lang.supports_sections() {
        return fail("Target file type does not support sections");
    }

    let mut plan = Plan::empty();
    let payload = IntegratePayload {
        source: source_path.to_string(),
        target_file: target_file_path.to_string(),
        target_section: target_section_name.to_string(),
        ..IntegratePayload::default()
    };
    plan.events.push(PlanEvent::new("integrate.starting", &payload));
    plan.messages.push(Message::Info("Splitting headers...".to_string()));

    let (source_header, source_body) = split_header(source_content, lang);
    let (target_header, target_body) = split_header(target_content, lang);
    plan.messages.push(Message::Info(format!("Source Header len: {}, Source Body len: {}", source_header.len(), source_body.len())));

    let (_, source_body_no_package) = extract_package(lang, &source_body);
    let (target_package, target_body_no_package) = extract_package(lang, &target_body);
    let (source_imports, source_code) = extract_imports(lang, &source_body_no_package);
    let (target_imports, target_code) = extract_imports(lang, &target_body_no_package);

    let merged_header = merge_headers(&target_header, &source_header, lang);
    let mut merged_imports = target_imports;
    merged_imports.extend(source_imports);
    let merged_imports = unique_strings(&merged_imports);

    let start_marker = format_section_start(lang, target_section_name);
    let end_marker = format_section_end(lang, target_section_name);
    let mut source_code = source_code;
    ensure_trailing_newline(&mut source_code);
    let section_content = format!("\n{start_marker}\n{source_code}{end_marker}\n");

    let updated_body = if target_parent_section_name.is_empty() {
        let mut body = target_code;
        ensure_trailing_newline(&mut body);
        body.push_str(&section_content);
        body
    } else {
        let sections = parse_sections_with(lang, &target_code);
        let Some(parent) = find_section(&sections, target_parent_section_name) else {
            return fail(format!("Parent section not found: {target_parent_section_name}"));
        };
        if parent.end_line == -1 {
            return fail(format!("Parent section {target_parent_section_name} is not properly closed"));
        }
        let lines: Vec<&str> = target_code.split('\n').collect();
        let cut = (parent.end_line - 1).max(0) as usize;
        let cut = cut.min(lines.len());
        let inserted: Vec<&str> = section_content.trim_matches('\n').split('\n').collect();
        let mut new_lines: Vec<&str> = Vec::with_capacity(lines.len() + inserted.len());
        new_lines.extend_from_slice(&lines[..cut]);
        new_lines.extend_from_slice(&inserted);
        new_lines.extend_from_slice(&lines[cut..]);
        new_lines.join("\n")
    };

    let mut final_content = merged_header;
    if !final_content.is_empty() {
        ensure_trailing_newline(&mut final_content);
        final_content.push('\n');
    }
    if !target_package.is_empty() {
        final_content.push_str(&target_package);
        final_content.push_str("\n\n");
    }
    let formatted_imports = format_imports(lang, &merged_imports);
    if !formatted_imports.is_empty() {
        final_content.push_str(&formatted_imports);
        final_content.push_str("\n\n");
    }
    final_content.push_str(&updated_body);

    plan.changes.push(Change::Write { path: clean_relative(target_file_path), content: final_content });
    plan.events.push(PlanEvent::new("integrate.ended", &payload));
    plan.messages.push(Message::Success(format!("\n🧩️Integrated {source_path} into {target_section_name} section of {target_file_path}")));
    Ok(plan)
}

//#endregion 📥️Integrate

//#region 🧲️Extract

/// 🧲️ Lifts one section out of a file into a new file that carries the source's header, package
/// declaration and imports, and removes the section from the source.
pub fn plan_extract(workspace: &Workspace, source_file_path: &str, source_section_name: &str, target_file_path: &str) -> Result<Plan, MoveError> {
    let Some(source_content) = workspace.file(source_file_path) else {
        return fail(format!("Source file not found: {source_file_path}"));
    };
    let Some(lang) = language_for_path(source_file_path) else {
        return fail("Source file type does not support sections");
    };
    if !lang.supports_sections() {
        return fail("Source file type does not support sections");
    }
    let sections = parse_sections_with(lang, source_content);
    let Some(section) = find_section(&sections, source_section_name) else {
        return fail(format!("Section not found: {source_section_name}"));
    };
    let lines: Vec<&str> = source_content.split('\n').collect();
    let total = lines.len() as i64;
    if section.start_line > total || section.end_line > total {
        return fail("Section range invalid");
    }

    let extracted_body = if section.end_line > section.start_line {
        let from = section.start_line.max(0) as usize;
        let to = (section.end_line - 1).max(0) as usize;
        lines[from.min(lines.len())..to.min(lines.len())].join("\n")
    } else {
        String::new()
    };

    let (header, source_body) = split_header(source_content, lang);
    let (package_declaration, source_body_no_package) = extract_package(lang, &source_body);
    let (imports, _) = extract_imports(lang, &source_body_no_package);

    let mut target_content = String::new();
    if !header.is_empty() {
        target_content.push_str(&header);
        target_content.push_str("\n\n");
    }
    if !package_declaration.is_empty() {
        target_content.push_str(&package_declaration);
        target_content.push_str("\n\n");
    }
    if !imports.is_empty() {
        let formatted = format_imports(lang, &imports);
        if !formatted.is_empty() {
            target_content.push_str(&formatted);
            target_content.push_str("\n\n");
        }
    }
    target_content.push_str(&extracted_body);
    ensure_trailing_newline(&mut target_content);
    if target_content.is_empty() {
        target_content.push('\n');
    }

    let mut remaining: Vec<&str> = Vec::with_capacity(lines.len());
    if section.start_line > 0 {
        remaining.extend_from_slice(&lines[..((section.start_line - 1).max(0) as usize).min(lines.len())]);
    }
    if section.end_line < total {
        remaining.extend_from_slice(&lines[(section.end_line.max(0) as usize).min(lines.len())..]);
    }

    let mut plan = Plan::empty();
    let payload = ExtractPayload {
        source_file: source_file_path.to_string(),
        source_section: source_section_name.to_string(),
        target_file: target_file_path.to_string(),
        ..ExtractPayload::default()
    };
    plan.events.push(PlanEvent::new("extract.starting", &payload));
    plan.changes.push(Change::Write { path: clean_relative(target_file_path), content: target_content });
    plan.changes.push(Change::Write { path: clean_relative(source_file_path), content: remaining.join("\n") });
    plan.events.push(PlanEvent::new("extract.ended", &payload));
    plan.messages.push(Message::Success(format!("\n🧩️Extracted {source_section_name} from {source_file_path} to {target_file_path}")));
    Ok(plan)
}

//#endregion 🧲️Extract

//#region 🧪️Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casings_cover_upper_title_and_lower() {
        assert_eq!(apply_rename_casings("MODEL Model model", "model", "representation"), "REPRESENTATION Representation representation");
    }

    #[test]
    fn rename_moves_deepest_paths_first() {
        let workspace = Workspace::new().with_file("model/model.ts", "const model = 1;\n").with_directory("model");
        let plan = plan_rename(&workspace, "model", "shape", "").expect("plan");
        let moves: Vec<&Change> = plan.changes.iter().filter(|c| matches!(c, Change::Move { .. })).collect();
        assert_eq!(moves.first(), Some(&&Change::Move { from: "model/model.ts".to_string(), to: "model/shape.ts".to_string() }));
        assert_eq!(moves.last(), Some(&&Change::Move { from: "model".to_string(), to: "shape".to_string() }));
    }

    #[test]
    fn workspace_applies_a_directory_move() {
        let mut workspace = Workspace::new().with_file("a/b/c.txt", "x");
        workspace.apply(&Change::Move { from: "a".to_string(), to: "z".to_string() }).expect("move");
        assert_eq!(workspace.file("z/b/c.txt"), Some("x"));
        assert!(!workspace.exists("a/b/c.txt"));
    }

    #[test]
    fn section_move_rewrites_both_markers() {
        let content = "// #region 🔖️Alpha\nconst a = 1;\n// #endregion 🔖️Alpha\n";
        let workspace = Workspace::new().with_file("x.ts", content);
        let plan = plan_section_move(&workspace, "x.ts", "Alpha", "Beta").expect("plan");
        match &plan.changes[0] {
            Change::Write { content, .. } => assert_eq!(content, "// #region 🔖️Beta\nconst a = 1;\n// #endregion 🔖️Beta\n"),
            other => panic!("unexpected change {other:?}"),
        }
    }
}

//#endregion 🧪️Tests
