//! 🧷️ Definition parsing: definition keyword extraction, range detection and orphan definitions.
//!
//! Holds the `🔖️Definitions` region of the crate root; the root re-exports its items.

use super::*;

// #region 🔖️Definitions

/// 🗺️ The canonical kind of a raw definition keyword.
pub fn derive_definition_kind(raw_kind: &str) -> DefinitionKind {
    match table().definition_kind_map.get(&raw_kind.to_lowercase()).map(String::as_str) {
        Some("interface") => DefinitionKind::Interface,
        Some("constant") => DefinitionKind::Constant,
        Some("test") => DefinitionKind::Test,
        _ => DefinitionKind::Implementation,
    }
}

fn extract_definition_keyword(full_match: &str, name: &str) -> String {
    let kw = &table().definition_keywords;
    let lower = full_match.to_lowercase();
    for multi in &kw.multi_word {
        if lower.contains(&multi.to_lowercase()) {
            return multi.clone();
        }
    }
    let lower_name = name.to_lowercase();
    let words: Vec<&str> = lower.split_whitespace().collect();
    let mut prev = String::new();
    for word in &words {
        let clean = word.trim_end_matches(['(', '<', '{', '[']);
        if clean == lower_name {
            break;
        }
        prev = clean.to_string();
    }
    if !prev.is_empty() && kw.keywords.contains(&prev) {
        return prev;
    }
    for word in &words {
        let clean = word.trim_end_matches(['(', '<', '{', '[']);
        if clean == lower_name || kw.modifiers.iter().any(|m| m == clean) {
            continue;
        }
        if kw.keywords.iter().any(|k| k == clean) {
            return clean.to_string();
        }
    }
    kw.fallback.clone()
}

fn refine_definition_kind(raw_kind: &str, line: &str) -> String {
    let r = &table().refinements;
    let lower = raw_kind.to_lowercase();
    if r.promotes.contains(&lower)
        && (pattern_find(&r.arrow_function, line) || pattern_find(&r.function_expression, line) || pattern_find(&r.class_expression, line))
    {
        return r.promoted_to.clone();
    }
    raw_kind.to_string()
}

fn indent_of(line: &str) -> usize {
    line.len() - line.trim_start_matches([' ', '\t']).len()
}

fn parse_definition_ranges_default(lang: &Language, lines: &[&str]) -> Vec<DefinitionRange> {
    let Some(pat) = lang.definition.as_ref() else {
        return Vec::new();
    };
    let indent_scoping = lang.definition_engine == Some(DefinitionEngine::Indent);
    let mut starts: Vec<(String, String, usize)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if let Some((end, caps)) = pattern_match(pat, line) {
            let name = capture(&caps, "name").unwrap_or_default();
            if !name.is_empty() {
                let kind = extract_definition_keyword(&line[..end], &name);
                starts.push((name, kind, i + 1));
            }
        }
    }
    let mut out = Vec::new();
    for i in 0..starts.len() {
        let start = starts[i].2;
        let mut end = start;
        if indent_scoping {
            let start_indent = indent_of(lines[start - 1]);
            #[allow(clippy::needless_range_loop)]
            for line_index in start..lines.len() {
                let line = lines[line_index].trim_end_matches('\r');
                if line.trim().is_empty() {
                    continue;
                }
                if indent_of(line) <= start_indent {
                    end = line_index;
                    break;
                }
                end = line_index + 1;
            }
        } else {
            let mut brace_depth = 0i64;
            let mut saw_open = false;
            let next_def_start = if i + 1 < starts.len() { starts[i + 1].2 - 1 } else { lines.len() };
            let mut line_index = start - 1;
            while line_index < lines.len() {
                if !saw_open && line_index >= next_def_start {
                    break;
                }
                let mut broke = false;
                for ch in lines[line_index].chars() {
                    if ch == '{' {
                        brace_depth += 1;
                        saw_open = true;
                    } else if ch == '}' {
                        if brace_depth > 0 {
                            brace_depth -= 1;
                        }
                        if saw_open && brace_depth == 0 {
                            end = line_index + 1;
                            broke = true;
                            break;
                        }
                    }
                }
                if broke {
                    break;
                }
                if saw_open && brace_depth == 0 && end > start {
                    break;
                }
                line_index += 1;
            }
            if !saw_open && i + 1 < starts.len() {
                end = starts[i + 1].2 - 1;
            }
        }
        let end = end.max(start);
        out.push(DefinitionRange {
            name: starts[i].0.clone(),
            kind: refine_definition_kind(&starts[i].1, lines[start - 1]),
            start: start as i64,
            end: end as i64,
            excerpt: starts[i].0.clone(),
        });
    }
    out
}

fn parse_definition_ranges_ruby(lang: &Language, lines: &[&str]) -> Vec<DefinitionRange> {
    let Some(pat) = lang.definition.as_ref() else {
        return Vec::new();
    };
    let end_pat = lang.aux_patterns.get("endLine").expect("ruby declares endLine");
    let block_pat = lang.aux_patterns.get("blockStart").expect("ruby declares blockStart");
    let mut def_stack: Vec<(String, i64, i64)> = Vec::new();
    let mut out: Vec<DefinitionRange> = Vec::new();
    let mut depth = 0i64;
    for (i, line) in lines.iter().enumerate() {
        let line_num = (i + 1) as i64;
        let trimmed = line.trim();
        match pattern_match(pat, line) {
            Some((_, caps)) => {
                let name = capture(&caps, "name").unwrap_or_default();
                if !name.is_empty() {
                    def_stack.push((name, line_num, depth));
                    depth += 1;
                }
            }
            None => {
                if pattern_find(block_pat, line) && !line.contains(" do ") {
                    depth += 1;
                }
            }
        }
        if pattern_match(end_pat, trimmed).is_some() {
            if depth > 0 {
                depth -= 1;
            }
            while def_stack.last().is_some_and(|(_, _, d)| *d == depth) {
                let (name, start, _) = def_stack.pop().expect("stack is not empty");
                out.push(DefinitionRange { name: name.clone(), kind: String::new(), start, end: line_num, excerpt: name });
            }
        }
    }
    for (name, start, _) in def_stack {
        out.push(DefinitionRange { name: name.clone(), kind: String::new(), start, end: lines.len() as i64, excerpt: name });
    }
    out.sort_by_key(|d| d.start);
    out
}

/// ▶️ The raw definition ranges a language finds in `lines`.
pub fn parse_definition_ranges(lang: &Language, lines: &[&str]) -> Vec<DefinitionRange> {
    match lang.definition_engine {
        Some(DefinitionEngine::RubyEnd) => parse_definition_ranges_ruby(lang, lines),
        _ => parse_definition_ranges_default(lang, lines),
    }
}

/// 👻️ The extra single-line definitions a language contributes outside its definition pattern.
pub fn extra_orphan_definitions(lang: &Language, lines: &[&str]) -> Vec<DefinitionRange> {
    let mut defs = Vec::new();
    match lang.orphan_definitions {
        OrphanDefinitions::None => {}
        OrphanDefinitions::GoPackageAndImports => {
            for (i, line) in lines.iter().enumerate() {
                let trimmed = line.trim();
                if trimmed.starts_with("package ") {
                    defs.push(DefinitionRange {
                        name: format!("package-{}", i + 1),
                        kind: String::new(),
                        start: i as i64 + 1,
                        end: i as i64 + 1,
                        excerpt: trimmed.to_string(),
                    });
                    break;
                }
            }
            let mut i = 0usize;
            while i < lines.len() {
                let trimmed = lines[i].trim();
                if trimmed.starts_with("import ") {
                    let start = i + 1;
                    let mut end = start;
                    if trimmed.starts_with("import (") {
                        #[allow(clippy::needless_range_loop)]
                        for j in i + 1..lines.len() {
                            if lines[j].trim() == ")" {
                                end = j + 1;
                                i = j;
                                break;
                            }
                        }
                    }
                    defs.push(DefinitionRange {
                        name: format!("import-{start}"),
                        kind: String::new(),
                        start: start as i64,
                        end: end as i64,
                        excerpt: lines[start - 1].trim().to_string(),
                    });
                }
                i += 1;
            }
        }
        OrphanDefinitions::RustModDeclarations => {
            for (i, line) in lines.iter().enumerate() {
                let trimmed = line.trim();
                let rest = trimmed.strip_prefix("pub ").unwrap_or(trimmed);
                if let Some(tail) = rest.strip_prefix("mod ") {
                    let name = tail.trim_end_matches(';').trim();
                    if !name.is_empty() && tail.trim_end().ends_with(';') && ident_len(name, 0) == name.len() {
                        defs.push(DefinitionRange {
                            name: format!("mod-{}-{}", name, i + 1),
                            kind: String::new(),
                            start: i as i64 + 1,
                            end: i as i64 + 1,
                            excerpt: trimmed.to_string(),
                        });
                    }
                }
            }
        }
        OrphanDefinitions::RubyModules => {
            for (i, line) in lines.iter().enumerate() {
                let after = line.trim_start();
                if let Some(tail) = after.strip_prefix("module ") {
                    let ends = scoped_ends(tail, 0, "::");
                    if let Some(&end) = ends.first() {
                        defs.push(DefinitionRange {
                            name: format!("module-{}-{}", &tail[..end], i + 1),
                            kind: String::new(),
                            start: i as i64 + 1,
                            end: i as i64 + 1,
                            excerpt: line.trim().to_string(),
                        });
                    }
                }
            }
        }
    }
    defs
}

/// 📖️ The definitions of `content` under the language claiming `file_path`.
pub fn parse_definitions(content: &str, file_path: &str) -> Vec<Definition> {
    let Some(lang) = language_for_path(file_path) else {
        return Vec::new();
    };
    if !lang.supports_definitions() {
        return Vec::new();
    }
    let lines = split_lines(content);
    parse_definition_ranges(lang, &lines)
        .into_iter()
        .map(|r| Definition {
            id: String::new(),
            name: r.name,
            kind: derive_definition_kind(&r.kind),
            file_path: file_path.to_string(),
            section_path: String::new(),
            emoji: String::new(),
            start_line: r.start,
            end_line: r.end,
            start_index: 0,
            end_index: 0,
        })
        .collect()
}

// #endregion 🔖️Definitions
