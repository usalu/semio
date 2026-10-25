//! ✏️ Rename: renaming a token in all of its casing variants.

use super::{apply_rename_casings, clean_relative, fail, sealed_document_paths, sealed_within, simple_lower, Change, Entry, Message, MoveError, Plan, Workspace};
use std::collections::BTreeSet;

//#region 🔤️Rename

const RENAME_SKIPPED_DIRECTORIES: [&str; 2] = [".git", "node_modules"];

/// 🔤️ Rewrites every UPPER, Title and lower variant of `old_token` to `new_token` across the
/// contents and the names of everything under `scope`, deepest path first so a parent rename never
/// invalidates a child's plan. Digest-sealed evidence keeps its bytes, and a rename that would
/// relocate one is refused before anything is planned.
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

    let sealed = sealed_document_paths(workspace)?;
    let mut entries: Vec<(String, bool)> = Vec::with_capacity(files.len() + directories.len());
    entries.extend(files.iter().map(|path| (path.clone(), false)));
    entries.extend(directories.iter().map(|path| (path.clone(), true)));
    entries.sort_by(|left, right| {
        let left_depth = left.0.matches('/').count();
        let right_depth = right.0.matches('/').count();
        right_depth.cmp(&left_depth).then_with(|| right.0.cmp(&left.0))
    });
    let relocated: BTreeSet<String> = entries
        .iter()
        .filter(|(path, _)| {
            let base = path.rsplit('/').next().unwrap_or(path.as_str());
            apply_rename_casings(base, old_token, new_token) != base
        })
        .flat_map(|(path, _)| sealed_within(&sealed, path))
        .collect();
    if !relocated.is_empty() {
        return fail(format!("Rename would relocate digest-sealed evidence: {}", relocated.into_iter().collect::<Vec<_>>().join(", ")));
    }

    let mut plan = Plan::empty();
    let mut files_changed = 0i64;
    let mut files_sealed = 0i64;
    for path in &files {
        let Some(original) = workspace.file(path) else { continue };
        let replaced = apply_rename_casings(original, old_token, new_token);
        if replaced == original {
            continue;
        }
        if sealed.contains(path) {
            files_sealed += 1;
            continue;
        }
        plan.changes.push(Change::Write { path: path.clone(), content: replaced });
        files_changed += 1;
    }

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
        "\n🔤️Renamed {old_token} → {new_token}: {files_changed} files edited, {files_renamed} files renamed, {folders_renamed} folders renamed, {files_sealed} sealed files kept"
    )));
    plan.stats.insert("filesChanged".to_string(), files_changed);
    plan.stats.insert("filesRenamed".to_string(), files_renamed);
    plan.stats.insert("filesSealed".to_string(), files_sealed);
    plan.stats.insert("foldersRenamed".to_string(), folders_renamed);
    Ok(plan)
}

//#endregion 🔤️Rename
