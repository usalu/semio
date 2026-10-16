//! 📚️ Agents docs: keeping the agents documents of moved folders consistent.

use super::{Change, Workspace};
use semio_framework_repo_identity::normalize_path;

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
