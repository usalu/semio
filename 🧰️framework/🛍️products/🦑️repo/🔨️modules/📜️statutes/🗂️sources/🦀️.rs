//! 🗂️ Sources: the analysed files, their parsed sections and the definition-head helpers.

use super::parse_ignore_directives;
use semio_framework_repo_languages as languages;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🗂️Sources

/// 📄️ One analysed file: its repository-relative path and its full text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFile {
    pub path: String,
    pub content: String,
}

/// 📑️ The analysed files with their parsed sections and ignore directives.
pub struct SourceSet {
    files: Vec<SourceFile>,
    pub(super) directives: BTreeMap<String, BTreeMap<i64, Vec<String>>>,
}

impl SourceSet {
    /// 🆕️ Parses the ignore directives of every file once.
    pub fn new(files: Vec<SourceFile>) -> Self {
        let directives = files.iter().map(|f| (f.path.clone(), parse_ignore_directives(&f.content))).collect();
        Self { files, directives }
    }

    /// 🗃️ The analysed files.
    pub fn files(&self) -> &[SourceFile] {
        &self.files
    }

    /// 🙈️ The ignore directives of one file.
    pub fn directives(&self, path: &str) -> Option<&BTreeMap<i64, Vec<String>>> {
        self.directives.get(path)
    }
}

pub(super) fn is_test_or_benchmark_file(file: &str) -> bool {
    let normalized = file.replace('\\', "/").to_lowercase();
    let name = normalized.rsplit('/').next().unwrap_or(&normalized).to_string();
    let extension = match name.rfind('.') {
        Some(index) if index > 0 => name[index..].to_string(),
        _ => String::new(),
    };
    let stem = name.strip_suffix(&extension).unwrap_or(&name).to_string();
    let lab_suffixes = [".test", ".spec", ".tests", ".requirements", "_test", "_tests", "_spec", "_benchmark", ".benchmark", ".stories", ".story"];
    if lab_suffixes.iter().any(|s| stem.ends_with(s)) || name.starts_with("test_") || name.starts_with("test.") || name == "conftest.py" {
        return true;
    }
    if normalized.contains("/tests/") || normalized.contains(".tests/") || normalized.contains("/test/") || normalized.contains(".test/") || normalized.contains("/benchmark/") || normalized.contains(".benchmark/") {
        return true;
    }
    name.ends_with("_test.go")
        || name.ends_with(".test.ts")
        || name.ends_with(".test.tsx")
        || name.ends_with(".test.js")
        || name.ends_with(".test.jsx")
        || name.ends_with(".spec.ts")
        || name.ends_with(".spec.tsx")
        || name.ends_with(".spec.js")
        || name.ends_with(".spec.jsx")
        || name.starts_with("test_")
        || name.contains("benchmark")
}

pub(super) fn policy_section_start_match(lang: &languages::Language, line: &str) -> Option<String> {
    let pattern = lang.policy_section_start.as_ref()?;
    let (_, captures) = languages::pattern_match(pattern, line)?;
    Some(capture_name(&captures))
}

pub(super) fn policy_section_end_match(lang: &languages::Language, line: &str) -> Option<String> {
    let pattern = lang.policy_section_end.as_ref()?;
    let (_, captures) = languages::pattern_match(pattern, line)?;
    Some(capture_name(&captures))
}

fn capture_name(captures: &languages::Captures) -> String {
    let raw = captures.iter().find(|(name, _)| name == "name").map(|(_, value)| value.trim().to_string()).unwrap_or_default();
    let (_, rest) = languages::extract_entity_emoji(&raw);
    rest.trim().to_string()
}

/// 🧿 The words a TypeScript definition head may carry before `function` or `class`.
const DEFINITION_MODIFIER_WORDS: [&str; 4] = ["async ", "abstract ", "declare ", "default "];

/// ✂️ Strips leading modifier words as whole space-separated prefixes, never as a character cutset.
fn strip_definition_modifier_words(line: &str) -> &str {
    let mut rest = line;
    loop {
        let Some(stripped) = DEFINITION_MODIFIER_WORDS.iter().find_map(|word| rest.strip_prefix(word)) else { return rest };
        rest = stripped;
    }
}

pub(super) fn requires_definition_requirements(line: &str, lang_name: &str) -> bool {
    let trimmed = line.trim();
    match lang_name {
        "typescript" => {
            let no_export = trimmed.strip_prefix("export ").unwrap_or(trimmed);
            let no_modifier = strip_definition_modifier_words(no_export);
            no_modifier.starts_with("function ") || no_modifier.starts_with("class ")
        }
        "go" => trimmed.starts_with("func "),
        "python" => trimmed.starts_with("def ") || trimmed.starts_with("class ") || trimmed.starts_with("async def "),
        "csharp" => !trimmed.contains(" interface ") && !trimmed.contains(" enum "),
        "rust" => {
            let mut no_prefix = trimmed.strip_prefix("pub ").unwrap_or(trimmed);
            if no_prefix.starts_with('(') {
                if let Some(index) = no_prefix.find(") ") {
                    no_prefix = no_prefix[index + 2..].trim_start();
                }
            }
            no_prefix.starts_with("fn ") || no_prefix.starts_with("struct ") || no_prefix.starts_with("impl ") || no_prefix.starts_with("trait ")
        }
        _ => true,
    }
}

//#endregion 🗂️Sources
