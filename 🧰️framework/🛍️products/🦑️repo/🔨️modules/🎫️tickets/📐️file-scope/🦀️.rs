//! 📐️ The version-control port, the diff parser and the ticket file scope.

use super::*;

//#region 📐️FileScope

/// 📈️ The lines a diff added and removed in one file.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffLines {
    pub added: Vec<i64>,
    pub removed: Vec<i64>,
}

/// 🌳️ The git ref of the staging index. Ticket close diffs the index against the working tree only.
pub const GIT_INDEX_REF: &str = ":0";

/// 🔒️ Everything the ticket domain asks a version-control system.
pub trait VersionControl {
    /// 📨️ The unified diff of a path set, produced with zero context lines.
    fn unified_diff(&self, base: &str, head: &str, paths: &[String]) -> TicketResult<String>;
    /// 🏷️ The name-status diff of a path set.
    fn name_status(&self, base: &str, head: &str, paths: &[String]) -> TicketResult<String>;
    /// 🚫️ Whether a path is excluded by the ignore rules.
    fn is_ignored(&self, path: &str) -> bool;
}

/// 🎞️ A recorded version-control transcript, so a ticket case never needs a git checkout.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordedVersionControl {
    #[serde(default)]
    pub unified_diff: String,
    #[serde(default)]
    pub name_status: String,
    #[serde(default)]
    pub ignored: Vec<String>,
}

impl VersionControl for RecordedVersionControl {
    fn unified_diff(&self, _base: &str, _head: &str, _paths: &[String]) -> TicketResult<String> {
        Ok(self.unified_diff.clone())
    }

    fn name_status(&self, _base: &str, _head: &str, _paths: &[String]) -> TicketResult<String> {
        Ok(self.name_status.clone())
    }

    fn is_ignored(&self, path: &str) -> bool {
        self.ignored.iter().any(|candidate| candidate == path)
    }
}

/// 🆚️ Parses a `git diff -U0` hunk stream into the added and removed line numbers per file.
pub fn parse_diff_lines(stdout: &str) -> BTreeMap<String, DiffLines> {
    let mut result: BTreeMap<String, DiffLines> = BTreeMap::new();
    let mut current = String::new();
    for line in stdout.split('\n') {
        if let Some(rest) = line.strip_prefix("diff --git ") {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() >= 2 {
                current = parts[1].strip_prefix("b/").unwrap_or(parts[1]).to_string();
                result.entry(current.clone()).or_default();
            }
        } else if let Some(rest) = line.strip_prefix("+++ b/") {
            current = rest.to_string();
            result.entry(current.clone()).or_default();
        } else if line.starts_with("@@") && !current.is_empty() {
            let Some(hunk) = parse_hunk_header(line) else { continue };
            let entry = result.entry(current.clone()).or_default();
            for offset in 0..hunk.old_count {
                entry.removed.push(hunk.old_start + offset);
            }
            for offset in 0..hunk.new_count {
                entry.added.push(hunk.new_start + offset);
            }
        }
    }
    result
}

struct HunkHeader {
    old_start: i64,
    old_count: i64,
    new_start: i64,
    new_count: i64,
}

fn parse_hunk_header(line: &str) -> Option<HunkHeader> {
    let rest = line.strip_prefix("@@")?.trim_start();
    let mut parts = rest.split_whitespace();
    let old = parts.next()?.strip_prefix('-')?;
    let new = parts.next()?.strip_prefix('+')?;
    let (old_start, old_count) = parse_range(old)?;
    let (new_start, new_count) = parse_range(new)?;
    Some(HunkHeader { old_start, old_count, new_start, new_count })
}

fn parse_range(value: &str) -> Option<(i64, i64)> {
    match value.split_once(',') {
        Some((start, count)) => Some((start.parse().ok()?, count.parse().ok()?)),
        None => Some((value.parse().ok()?, 1)),
    }
}

/// 🧹️ Whether a path is excluded from every ticket file scope regardless of ignore rules.
pub fn is_repo_excluded_path(path: &str) -> bool {
    let normalized = normalize_separators(path.trim()).trim_start_matches("./").to_string();
    if normalized.is_empty() {
        return false;
    }
    let under = |root: &str| normalized == root || normalized.starts_with(&format!("{root}/"));
    if under(".🧬semio") || under("assets/repo") || normalized.contains("/asset/repo/") {
        return true;
    }
    if under("node_modules") || normalized.contains("/node_modules/") {
        return true;
    }
    if under(".git") || normalized.contains("/.git/") {
        return true;
    }
    for segment in ["/dist/", "/build/", "/target/", "/__pycache__/", "/.next/", "/coverage/"] {
        if normalized.contains(segment) {
            return true;
        }
    }
    base_name(&normalized).ends_with(".Designer.cs") || normalized.contains("/codegen/")
}

/// 📝️ Normalises one file identifier a close request carries into a repository-relative path.
///
/// The Go twin additionally resolves a bare file artifact id through the codebase index. That
/// lookup belongs to `🗂️codebase`; here a value with no separator and no extension is passed
/// through unchanged, which is what the Go fallback does when the index holds no match.
pub fn normalize_ticket_file_input(file_path: &str) -> String {
    let normalized = file_path.trim();
    if normalized.is_empty() {
        return String::new();
    }
    if let Some(rest) = normalized.strip_prefix("file://") {
        return normalize_separators(rest).trim_start_matches("./").to_string();
    }
    normalize_separators(normalized).trim_start_matches("./").to_string()
}

/// 💿️ Normalises and de-duplicates the file identifiers of a close request, preserving order.
pub fn normalize_ticket_file_inputs(files: &[String]) -> Vec<String> {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut filtered: Vec<String> = Vec::with_capacity(files.len());
    for file_path in files {
        let normalized = normalize_ticket_file_input(file_path);
        if normalized.is_empty() || !seen.insert(normalized.clone()) {
            continue;
        }
        filtered.push(normalized);
    }
    filtered
}

/// 🗺️ Drops the files that live inside the ticket's own folder — a ticket never contributes itself.
pub fn filter_ticket_workspace_files(folder_path: &str, files: &[String]) -> Vec<String> {
    let relative = normalize_separators(folder_path).trim_start_matches("./").to_string();
    if relative.is_empty() {
        return files.to_vec();
    }
    files
        .iter()
        .filter(|file_path| {
            let normalized = normalize_separators(file_path).trim_start_matches("./").to_string();
            normalized != relative && !normalized.starts_with(&format!("{relative}/"))
        })
        .cloned()
        .collect()
}

/// 🗂️ The file scope a ticket closes with, together with the diff of every file in it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketFileScope {
    pub files: Vec<String>,
    pub excluded: Vec<String>,
    pub diff_lines: BTreeMap<String, DiffLines>,
    pub statuses: Vec<String>,
}

/// 🎫️ Computes the file scope of a ticket close through the version-control port.
pub fn compute_ticket_file_scope<V: VersionControl>(version_control: &V, folder_path: &str, files: &[String]) -> TicketResult<TicketFileScope> {
    let normalized = normalize_ticket_file_inputs(files);
    let workspace = filter_ticket_workspace_files(folder_path, &normalized);
    let mut kept: Vec<String> = Vec::with_capacity(workspace.len());
    let mut excluded: Vec<String> = Vec::new();
    for file_path in workspace {
        if is_repo_excluded_path(&file_path) || version_control.is_ignored(&file_path) {
            excluded.push(file_path);
        } else {
            kept.push(file_path);
        }
    }
    if kept.is_empty() {
        return Err(TicketError::invalid("at least one file is required"));
    }
    let statuses: Vec<String> = version_control.name_status(GIT_INDEX_REF, "", &kept)?.lines().filter(|line| !line.trim().is_empty()).map(str::to_string).collect();
    let diff_lines = parse_diff_lines(&version_control.unified_diff(GIT_INDEX_REF, "", &kept)?);
    Ok(TicketFileScope { files: kept, excluded, diff_lines, statuses })
}

/// 📪️ Whether a ticket may be closed, with the reasons it may not.
pub fn can_close_ticket(ticket: Option<&Ticket>) -> (bool, Vec<String>) {
    match ticket {
        None => (false, vec!["Ticket data is nil".to_string()]),
        Some(_) => (true, Vec::new()),
    }
}

//#endregion 📐️FileScope
