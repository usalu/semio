//! 🔧️ Autofix: the repairs applied to one source and the statutes they fixed.

use super::{policy_section_start_match, SourceFile, Statute};
use semio_framework_repo_languages as languages;
use serde::{Deserialize, Serialize};

//#region 🩹️Autofix

/// 📄️ The outcome of applying every autofix transformation to one file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FixedFile {
    pub path: String,
    pub content: String,
    pub fixed: Vec<Statute>,
}

/// 🩹️ Applies every autofixable transformation to one source and reports the statutes it repaired.
pub fn autofix(source: &SourceFile) -> FixedFile {
    let mut content = source.content.clone();
    let mut fixed = Vec::new();
    if let Some(next) = fix_newline_after_region(&source.path, &content) {
        if next != content {
            content = next;
            fixed.push(Statute::from("code/section/wrong-format/newline-after-region"));
        }
    }
    FixedFile { path: source.path.clone(), content, fixed }
}

fn fix_newline_after_region(path: &str, content: &str) -> Option<String> {
    let language = languages::language_for_path(path)?;
    if !language.supports_sections() {
        return None;
    }
    let lines: Vec<&str> = content.split('\n').collect();
    let mut drop = vec![false; lines.len()];
    for (index, raw) in lines.iter().enumerate() {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        if policy_section_start_match(language, line).is_none() {
            continue;
        }
        let mut next = index + 1;
        while next < lines.len() && lines[next].trim().is_empty() {
            drop[next] = true;
            next += 1;
        }
    }
    let kept: Vec<&str> = lines.iter().enumerate().filter(|(index, _)| !drop[*index]).map(|(_, line)| *line).collect();
    Some(kept.join("\n"))
}

//#endregion 🩹️Autofix
