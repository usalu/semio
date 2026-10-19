//! 🎫️ The ticket domain of the semio repository: the dated id and path scheme, the
//! `🎫️ticket.json` document codec, the open/close/reopen/change lifecycle, the important-document
//! transaction, artifact purging, search, plan/spec source resolution and issue synchronisation.
//!
//! Every filesystem touch goes through [`TicketStore`], every issue mutation through the
//! `IssueTracker` port of `🧩️providers`, every clock read through [`Clock`] and every emission
//! through [`EventSink`], so the whole lifecycle runs in memory with nothing on disk and no `gh`
//! on the machine — see `🧪️tests/🔓️open-close-reopen-lifecycle`.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/🧬️schema/🔣️.json
//! @see 🧰️framework/🛍️products/🦑️repo/AGENTS.md (§5.6, §8.10 — the ticket algebra and id rule)

use semio_framework_repo_model::{Interaction, InteractionFile, Ticket, TicketAgent, TicketManagementData, TicketPlan, TicketStatus};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

//#region 🔁️Reexports

/// 🔁️ The provider shapes a client of this crate needs in order to call it.
///
/// A client may not name `semio-framework-repo-providers` itself — a generated test host links only
/// the crate under test — so everything the ticket API takes or returns is re-exported here
/// explicitly, per the repository's "reexport explicitly if the client needs it" rule.
pub use semio_framework_repo_providers::{IssueTracker, ManagementIssue, ManagementLabel, ManagementMilestone, ManagementProviders, McpClientKind, NullManagementProvider, ProviderError, ProviderResult};

//#endregion 🔁️Reexports

//#region ❌️Errors

/// ⚠️ Every ticket failure, carrying a class for machine consumers and the Go message verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketError {
    pub class: String,
    pub message: String,
}

impl TicketError {
    /// 🆕️ Builds an error of a class.
    pub fn new(class: &str, message: impl Into<String>) -> TicketError {
        TicketError { class: class.to_string(), message: message.into() }
    }

    /// 🚫️ An input the caller may correct.
    pub fn invalid(message: impl Into<String>) -> TicketError {
        TicketError::new("invalid", message)
    }

    /// 🔍️ A ticket, document or node that is not there.
    pub fn not_found(message: impl Into<String>) -> TicketError {
        TicketError::new("not-found", message)
    }

    /// 💥️ A store operation that failed.
    pub fn store(message: impl Into<String>) -> TicketError {
        TicketError::new("store", message)
    }

    /// 🔀️ A state that forbids the requested transition.
    pub fn conflict(message: impl Into<String>) -> TicketError {
        TicketError::new("conflict", message)
    }
}

impl std::fmt::Display for TicketError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for TicketError {}

/// 🎯️ The result of every operation in this crate.
pub type TicketResult<T> = Result<T, TicketError>;

//#endregion ❌️Errors

#[path = "🪪️ids/🦀️.rs"]
mod ids;
pub use ids::*;

#[path = "📄️codec/🦀️.rs"]
mod codec;
pub use codec::*;

            return false;
        }
        let identity = TicketId::new(ticket.year, ticket.month, ticket.day, ticket.slug.clone()).id();
        if !self.filter.is_empty() && ![identity.as_str(), ticket.slug.as_str(), ticket.title.as_str()].iter().any(|candidate| self.matches_filter(candidate)) {
            return false;
        }
        if !self.query.is_empty() {
            let haystack = format!("{identity} {} {} {} {}", ticket.slug, ticket.title, ticket.description, ticket.status.as_str());
            if !haystack.to_lowercase().contains(&self.query.to_lowercase()) {
                return false;
            }
        }
        true
    }

    fn matches_filter(&self, name: &str) -> bool {
        let (target, pattern) = if self.match_case { (name.to_string(), self.filter.clone()) } else { (name.to_lowercase(), self.filter.to_lowercase()) };
        if self.match_whole_word {
            return target.split(|character: char| !(character.is_ascii_alphanumeric() || character == '_')).any(|word| word == pattern);
        }
        target.contains(&pattern)
    }
}

//#endregion 🔎️Search

//#region 🧹️RealPurge

/// 🧹️ Every slug directory under a dated `🎫️tickets` tree, in `YY/MM/DD/SLUG` walk order.
///
/// Twin of `collectTicketFolderRoots` in `📦️packages/🐹️go/🐹️.go`: four levels deep, directories
/// only, dot-prefixed slugs skipped, an unreadable level silently contributing nothing.
pub fn ticket_folder_roots(tickets_dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut roots = Vec::new();
    for year in sorted_directories(tickets_dir) {
        for month in sorted_directories(&year) {
            for day in sorted_directories(&month) {
                for slug in sorted_directories(&day) {
                    if slug.file_name().is_none_or(|name| name.to_string_lossy().starts_with('.')) {
                        continue;
                    }
                    roots.push(slug);
                }
            }
        }
    }
    roots
}

/// 📁️ The child directories of a path, in name order; an unreadable path contributes none.
fn sorted_directories(path: &std::path::Path) -> Vec<std::path::PathBuf> {
    let Ok(entries) = std::fs::read_dir(path) else { return Vec::new() };
    let mut found: Vec<std::path::PathBuf> = entries.flatten().filter(|entry| entry.path().is_dir()).map(|entry| entry.path()).collect();
    found.sort();
    found
}

/// 🧹️ Deletes the oversized artifacts of one real ticket folder, deepest subfolder first.
///
/// Twin of `PurgeOversizedTicketArtifacts` in `📦️packages/🐹️go/🐹️.go`. A missing folder is a
/// no-operation, symlinks are never followed, `🎫️ticket.json` is never removed and a file inside
/// an already-deleted subtree is not visited again.
pub fn purge_oversized_artifacts(ticket_dir: &std::path::Path) -> Result<PurgeReport, String> {
    let mut report = PurgeReport::default();
    let Ok(metadata) = std::fs::symlink_metadata(ticket_dir) else { return Ok(report) };
    if !metadata.is_dir() {
        return Err(format!("ticket path is not a directory: {}", ticket_dir.display()));
    }
    let mut directories: Vec<std::path::PathBuf> = Vec::new();
    let mut files: Vec<(std::path::PathBuf, u64)> = Vec::new();
    collect_real_tree(ticket_dir, &mut directories, &mut files);
    let mut directory_sizes: BTreeMap<std::path::PathBuf, u64> = BTreeMap::new();
    for (path, size) in &files {
        let mut parent = path.parent().map(std::path::Path::to_path_buf);
        while let Some(current) = parent {
            if !current.starts_with(ticket_dir) {
                break;
            }
            *directory_sizes.entry(current.clone()).or_default() += size;
            if current == ticket_dir {
                break;
            }
            parent = current.parent().map(std::path::Path::to_path_buf);
        }
    }
    directories.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
    let mut deleted: Vec<std::path::PathBuf> = Vec::new();
    for directory in directories {
        if deleted.iter().any(|removed| directory.starts_with(removed)) {
            continue;
        }
        if directory_sizes.get(&directory).copied().unwrap_or_default() <= OVERSIZED_FOLDER_BYTES {
            continue;
        }
        std::fs::remove_dir_all(&directory).map_err(|error| format!("delete oversized ticket folder {}: {error}", directory.display()))?;
        report.removed_directories.push(directory.to_string_lossy().replace('\\', "/"));
        deleted.push(directory);
    }
    for (path, size) in files {
        if deleted.iter().any(|removed| path.starts_with(removed)) {
            continue;
        }
        if path.file_name().is_some_and(|name| name == std::ffi::OsStr::new(TICKET_DOCUMENT_NAME)) || size <= OVERSIZED_FILE_BYTES {
            continue;
        }
        std::fs::remove_file(&path).map_err(|error| format!("delete oversized ticket file {}: {error}", path.display()))?;
        report.removed_files.push(path.to_string_lossy().replace('\\', "/"));
    }
    report.removed_directories.sort();
    report.removed_files.sort();
    Ok(report)
}

/// 🚶️ Every directory below `root` and every regular file with its size, symlinks skipped.
fn collect_real_tree(root: &std::path::Path, directories: &mut Vec<std::path::PathBuf>, files: &mut Vec<(std::path::PathBuf, u64)>) {
    let Ok(entries) = std::fs::read_dir(root) else { return };
    let mut children: Vec<std::path::PathBuf> = entries.flatten().map(|entry| entry.path()).collect();
    children.sort();
    for child in children {
        let Ok(metadata) = std::fs::symlink_metadata(&child) else { continue };
        if metadata.file_type().is_symlink() {
            continue;
        }
        if metadata.is_dir() {
            directories.push(child.clone());
            collect_real_tree(&child, directories, files);
        } else {
            files.push((child, metadata.len()));
        }
    }
}

//#endregion 🧹️RealPurge

//#region 🧪️Tests

#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests
