//! 🧭️ Scope and helpers of the codebase: the walk scope and the directory, bundle name, ignore pattern, section, definition and contributor helpers. See [`super`].

use super::*;

//#region 🧭️Scope
/// 🧭️ What subtree of the repository a walk covers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Scope {
    /// 🏠️ The whole repository.
    Repo,
    /// 🏗️ One bundle named by its full bundle name.
    Technology(String),
    /// 📁️ One folder prefix.
    Folder(String),
    /// 📄️ One single file.
    File(String),
}
//#endregion 🧭️Scope

//#region 🔧️Helpers
/// 📂️ The entries of a directory in Go's lexical `os.ReadDir` order.
pub(super) fn read_dir_names(dir: &Path, directories_only: bool) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(dir) else { return Vec::new() };
    let mut names: Vec<String> = entries
        .flatten()
        .filter(|entry| !directories_only || entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect();
    names.sort();
    names
}

/// 📦️ Splits a bundle name into its technology and bundle code.
pub(super) fn split_bundle_name(name: &str) -> (String, String) {
    match name.split_once('/') {
        Some((technology, bundle)) => (technology.to_string(), bundle.to_string()),
        None => (name.to_string(), name.to_string()),
    }
}

/// 🙈️ Reports whether one ignore pattern excludes a path.
pub fn matches_ignore_pattern(path: &str, is_dir: bool, pattern: &str) -> bool {
    let pattern = normalize_path(pattern.trim());
    if pattern.is_empty() {
        return false;
    }
    let path = normalize_path(path.trim());
    if path.is_empty() {
        return false;
    }
    let mut candidates = vec![path.clone()];
    if is_dir && !path.ends_with('/') {
        candidates.push(format!("{path}/"));
    }
    let mut patterns = vec![pattern.clone()];
    if let Some(base) = pattern.strip_suffix("/**") {
        if !base.is_empty() {
            patterns.push(base.to_string());
            patterns.push(format!("{base}/"));
        }
    }
    for candidate_pattern in &patterns {
        if candidate_pattern.is_empty() {
            continue;
        }
        for candidate_path in &candidates {
            if glob_match(candidate_pattern, candidate_path).unwrap_or(false) {
                return true;
            }
        }
    }
    false
}

/// 🔢️ The total number of sections including every nested child.
pub fn count_sections(sections: &[Section]) -> i64 {
    sections.iter().fold(sections.len() as i64, |total, section| total + count_sections(&section.children))
}

/// 🧲️ The file path half of a breach scope.
pub fn extract_file_path(scope: &str) -> String {
    scope.split('#').next().unwrap_or_default().split('§').next().unwrap_or_default().to_string()
}

/// 🔖️ The section path a definition's line range falls inside.
pub fn find_section_for_definition(sections: &[Section], start_line: i64, end_line: i64, prefix: &str) -> String {
    for section in sections {
        if start_line >= section.start_line && end_line <= section.end_line {
            let path = if prefix.is_empty() { section.name.clone() } else { format!("{prefix}#{}", section.name) };
            if !section.children.is_empty() {
                let child = find_section_for_definition(&section.children, start_line, end_line, &path);
                if !child.is_empty() {
                    return child;
                }
            }
            return path;
        }
    }
    prefix.to_string()
}

/// 📖️ Every definition of one file with every addressable field filled: the section that encloses
/// it, its kind emoji and the file it belongs to.
///
/// [`semio_framework_repo_languages::parse_definitions`] answers names, kinds and line ranges only;
/// which section a definition sits in is a property of the whole file, so it is resolved once here
/// and every caller — the `definition list` tool, the GraphQL aggregate, the export snapshot — reads
/// the same record.
pub fn file_definitions(content: &str, file_path: &str) -> Vec<Definition> {
    let parsed = parse_definitions(content, file_path);
    if parsed.is_empty() {
        return Vec::new();
    }
    let sections = parse_sections(content, file_path);
    parsed
        .into_iter()
        .map(|mut definition| {
            definition.file_path = file_path.to_string();
            definition.section_path = find_section_for_definition(&sections, definition.start_line, definition.end_line, "");
            definition.emoji = definition_kind_emoji(definition.kind);
            definition
        })
        .collect()
}

/// 🪨️ The identity one definition of a file is addressed by, built from the file, the section that
/// encloses it, its kind and its name.
pub fn definition_id(file_id: &str, definition: &Definition) -> String {
    build_definition_id(file_id, &semio_framework_repo_languages::normalize_section_path(&definition.section_path), &definition.name, definition.kind)
}

/// 🤝️ Parses a copyright header line into the contributor name and email it carries.
pub fn parse_contributor_identity(line: &str) -> Option<(String, String)> {
    let bytes: Vec<char> = line.chars().collect();
    for start in 0..bytes.len().saturating_sub(3) {
        if !bytes[start..start + 4].iter().all(char::is_ascii_digit) {
            continue;
        }
        if start > 0 && bytes[start - 1].is_ascii_digit() {
            continue;
        }
        let mut cursor = start + 4;
        if bytes.get(cursor).is_some_and(|value| value.is_ascii_digit()) {
            continue;
        }
        if !bytes.get(cursor).is_some_and(|value| value.is_whitespace()) {
            continue;
        }
        while bytes.get(cursor).is_some_and(|value| value.is_whitespace()) {
            cursor += 1;
        }
        let rest: String = bytes[cursor..].iter().collect();
        let Some(open) = rest.find('<') else { continue };
        let Some(close) = rest[open + 1..].find('>') else { continue };
        let email = rest[open + 1..open + 1 + close].trim().to_string();
        if email.is_empty() {
            continue;
        }
        let name = rest[..open].trim().to_string();
        if name.is_empty() {
            continue;
        }
        return Some((name, email));
    }
    None
}
//#endregion 🔧️Helpers
