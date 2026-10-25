//! 🗣️ Language helpers: section markers and patterns per source language.

use super::{parse_sections_with, Language, Section};
use semio_framework_repo_languages::pattern_match;
use std::collections::BTreeSet;

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

pub(super) fn ensure_trailing_newline(value: &mut String) {
    if !value.is_empty() && !value.ends_with('\n') {
        value.push('\n');
    }
}

//#endregion 🗣️Language Helpers
