//! 🧭️ Scopes and headers: scope identifiers, line-based parsing and the file header format.
//!
//! Holds the `🔖️Scopes`, `🔖️Headers` regions of the crate root; the root re-exports its items.

use super::*;

// #region 🔖️Scopes

/// 📍️ One addressable scope inside a file.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ScopeEntry {
    pub kind: String,
    pub id: String,
    #[serde(rename = "filePath")]
    pub file_path: String,
    #[serde(rename = "sectionPath", skip_serializing_if = "String::is_empty")]
    pub section_path: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub definition: String,
    #[serde(rename = "startLine")]
    pub start_line: i64,
    #[serde(rename = "endLine")]
    pub end_line: i64,
}

/// 📑️ A section found by the scope builder: flat, dot-joined path, no emoji split.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedSection {
    pub name: String,
    pub path: String,
    pub start_line: i64,
    pub end_line: i64,
}

/// 🔭️ The deterministic identifier of a scope.
pub fn build_scope_id(kind: &str, file_path: &str, section_path: &str, definition: &str) -> String {
    match kind {
        "file" => format!("file:{file_path}"),
        "section" => format!("section:{file_path}#{section_path}"),
        _ if !section_path.is_empty() => format!("def:{file_path}#{section_path}::{definition}"),
        _ => format!("def:{file_path}#{definition}"),
    }
}

/// 💬️ A region marker on `line`: its name and whether it closes a region.
pub fn parse_region_marker(line: &str) -> Option<(String, bool)> {
    let m = &table().scope_region_marker;
    let mut trimmed = line.trim().to_string();
    for prefix in &m.strip_prefixes {
        if let Some(rest) = trimmed.strip_prefix(prefix.as_str()) {
            trimmed = rest.to_string();
        }
    }
    for suffix in &m.strip_suffixes {
        if let Some(rest) = trimmed.strip_suffix(suffix.as_str()) {
            trimmed = rest.to_string();
        }
    }
    let trimmed = trimmed.trim();
    for (keyword, is_end) in [(&m.start_keyword, false), (&m.end_keyword, true)] {
        if let Some(rest) = trimmed.strip_prefix(keyword.as_str()) {
            let (emoji, name) = extract_entity_emoji(rest.trim());
            if !emoji.is_empty() {
                return Some((name, is_end));
            }
        }
    }
    None
}

/// 🔬️ A markdown heading on `line`: its level and title.
pub fn parse_markdown_heading(line: &str) -> Option<(usize, String)> {
    let trimmed = line.trim();
    if !trimmed.starts_with('#') {
        return None;
    }
    let level = trimmed.chars().take_while(|c| *c == '#').count();
    if level == 0 || level > 6 {
        return None;
    }
    let name = trimmed[level..].trim();
    if name.is_empty() {
        return None;
    }
    Some((level, name.to_string()))
}

/// 🌳️ Sections of `lines` for the scope builder: region markers plus markdown headings.
pub fn parse_sections_from_lines(lines: &[&str], ext: &str) -> Vec<ParsedSection> {
    struct Frame {
        name: String,
        start_line: i64,
        level: usize,
        path: String,
    }
    let mut sections = Vec::new();
    let mut stack: Vec<Frame> = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let line_number = (index + 1) as i64;
        if let Some((name, is_end)) = parse_region_marker(line) {
            if is_end {
                if let Some(frame) = stack.pop() {
                    sections.push(ParsedSection { name: frame.name, path: frame.path, start_line: frame.start_line, end_line: line_number - 1 });
                }
            } else {
                let path = match stack.last() {
                    Some(top) => format!("{}.{}", top.path, name),
                    None => name.clone(),
                };
                stack.push(Frame { name, start_line: line_number, level: 0, path });
            }
            continue;
        }
        if ext == ".md" || ext == ".mdx" {
            if let Some((level, title)) = parse_markdown_heading(line) {
                while stack.last().is_some_and(|f| f.level >= level) {
                    let frame = stack.pop().expect("stack is not empty");
                    sections.push(ParsedSection { name: frame.name, path: frame.path, start_line: frame.start_line, end_line: line_number - 1 });
                }
                let path = match stack.last() {
                    Some(top) => format!("{}.{}", top.path, title),
                    None => title.clone(),
                };
                stack.push(Frame { name: title, start_line: line_number, level, path });
            }
        }
    }
    for frame in stack {
        sections.push(ParsedSection { name: frame.name, path: frame.path, start_line: frame.start_line, end_line: lines.len() as i64 });
    }
    sections
}

/// 🧩️ Single-line definitions of `lines` for the scope builder.
pub fn parse_definitions_from_lines(lines: &[&str], patterns: &[Pattern]) -> Vec<(String, i64)> {
    let mut defs = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        for pattern in patterns {
            if let Some((_, caps)) = pattern_match(pattern, line) {
                if let Some(name) = capture(&caps, "name") {
                    defs.push((name, (index + 1) as i64));
                    break;
                }
            }
        }
    }
    defs
}

/// 🏗️ Every addressable scope of a file: the file itself, its sections and its definitions.
pub fn build_scopes_for_file(path: &str, content: &str) -> Vec<ScopeEntry> {
    let lines = split_lines(content);
    let ext = path_extension(path).to_lowercase();
    let mut entries = vec![ScopeEntry {
        kind: "file".to_string(),
        id: build_scope_id("file", path, "", ""),
        file_path: path.to_string(),
        section_path: String::new(),
        definition: String::new(),
        start_line: 1,
        end_line: lines.len() as i64,
    }];
    let sections = parse_sections_from_lines(&lines, &ext);
    for s in &sections {
        entries.push(ScopeEntry {
            kind: "section".to_string(),
            id: build_scope_id("section", path, &s.path, ""),
            file_path: path.to_string(),
            section_path: s.path.clone(),
            definition: String::new(),
            start_line: s.start_line,
            end_line: s.end_line,
        });
    }
    let mut section_by_line: BTreeMap<i64, String> = BTreeMap::new();
    for s in &sections {
        for line in s.start_line..=s.end_line {
            section_by_line.insert(line, s.path.clone());
        }
    }
    let empty = Vec::new();
    let patterns = table().scope_definition_patterns.get(&ext).unwrap_or(&empty);
    for (name, start_line) in parse_definitions_from_lines(&lines, patterns) {
        let sp = section_by_line.get(&start_line).cloned().unwrap_or_default();
        entries.push(ScopeEntry {
            kind: "definition".to_string(),
            id: build_scope_id("definition", path, &sp, &name),
            file_path: path.to_string(),
            section_path: sp,
            definition: name,
            start_line,
            end_line: start_line,
        });
    }
    entries
}

// #endregion 🔖️Scopes

// #region 🔖️Headers

/// 🧾️ The decomposed fields of a file header region.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize)]
pub struct Header {
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[serde(rename = "fileUri")]
    pub file_uri: String,
    pub summary: String,
    pub contributors: String,
    pub license: String,
    pub requirements: String,
}

/// 🐍️ The Rust module identifier a section name is wrapped in.
pub fn section_name_to_mod_name(name: &str) -> String {
    let runes: Vec<char> = name.chars().collect();
    let mut buf = String::new();
    for (i, r) in runes.iter().enumerate() {
        if r.is_ascii_uppercase() {
            if i > 0 {
                let prev = runes[i - 1];
                if prev.is_ascii_lowercase() || prev.is_ascii_digit() {
                    buf.push('_');
                }
            }
            buf.push(r.to_ascii_lowercase());
        } else if r.is_ascii_lowercase() || r.is_ascii_digit() {
            buf.push(*r);
        } else if !buf.is_empty() && !buf.ends_with('_') {
            buf.push('_');
        }
    }
    let result = buf.trim_matches('_').to_string();
    if result.is_empty() {
        "section".to_string()
    } else {
        result
    }
}

fn apply_format(lang: &Language, format: &str, name: &str) -> String {
    if format.is_empty() {
        return String::new();
    }
    let with_name = format.replace("{name}", name);
    match lang.section_naming {
        SectionNaming::ModName => with_name.replace("{modName}", &section_name_to_mod_name(name)),
        SectionNaming::Verbatim => with_name,
    }
}

/// 🔤️ The opening marker of a named section.
pub fn format_section_start(lang: &Language, name: &str) -> String {
    apply_format(lang, &lang.section_start_format, name)
}

/// ⏹️ The closing marker of a named section.
pub fn format_section_end(lang: &Language, name: &str) -> String {
    apply_format(lang, &lang.section_end_format, name)
}

/// ⬛️ The empty section skeleton of a named section.
pub fn format_section_both(lang: &Language, name: &str) -> String {
    apply_format(lang, &lang.section_both_format, name)
}

/// ⬜️ The header region of a file, or the empty string for a language without headers.
pub fn format_header(lang: &Language, header: &Header) -> String {
    if !lang.supports_headers {
        return String::new();
    }
    let cp = &lang.comment_prefix;
    let mut b = String::new();
    b.push_str(&format_section_start(lang, "Header"));
    b.push('\n');
    b.push_str(&format!("{} [{}]({})\n", cp, header.file_id, header.file_uri));
    for line in header.contributors.split('\n') {
        if !line.trim().is_empty() {
            b.push_str(&format!("{cp} {line}\n"));
        }
    }
    b.push('\n');
    for line in header.license.split('\n') {
        if line.is_empty() {
            b.push_str(&format!("{cp}\n"));
        } else {
            b.push_str(&format!("{cp} {line}\n"));
        }
    }
    b.push('\n');
    for block in [&header.summary, &header.requirements] {
        if block.is_empty() {
            continue;
        }
        for line in block.split('\n') {
            if line.is_empty() {
                b.push_str(&format!("{cp}\n"));
            } else {
                b.push_str(&format!("{cp} {line}\n"));
            }
        }
        b.push('\n');
    }
    b.push_str(&format_section_end(lang, "Header"));
    b.push('\n');
    b
}

/// 📖️ Reads a header region back into its fields.
///
/// A formatted header carries no marker separating the summary block from the requirements block, so
/// the first optional block is read as the summary and the second as the requirements. The property
/// that holds for every input is idempotence: formatting a parsed header reproduces the text exactly.
pub fn parse_header(lang: &Language, content: &str) -> Option<Header> {
    if !lang.supports_headers {
        return None;
    }
    let cp = &lang.comment_prefix;
    let start = format_section_start(lang, "Header");
    let end = format_section_end(lang, "Header");
    let lines: Vec<&str> = content.split('\n').collect();
    let start_at = lines.iter().position(|l| l.trim_end() == start.trim_end())?;
    let end_at = lines.iter().skip(start_at + 1).position(|l| l.trim_end() == end.trim_end())? + start_at + 1;
    let body = &lines[start_at + 1..end_at];
    let mut blocks: Vec<Vec<String>> = Vec::new();
    let mut current: Vec<String> = Vec::new();
    for raw in body {
        if raw.trim().is_empty() {
            if !current.is_empty() {
                blocks.push(std::mem::take(&mut current));
            }
            continue;
        }
        let stripped = raw.strip_prefix(cp.as_str())?;
        current.push(stripped.strip_prefix(' ').unwrap_or(stripped).to_string());
    }
    if !current.is_empty() {
        blocks.push(current);
    }
    let identity = blocks.first()?.first()?.clone();
    let open = identity.find('[')?;
    let close = identity.find("](")?;
    let uri_end = identity.rfind(')')?;
    let mut header = Header {
        file_id: identity[open + 1..close].to_string(),
        file_uri: identity[close + 2..uri_end].to_string(),
        contributors: blocks[0][1..].join("\n"),
        ..Header::default()
    };
    if let Some(license) = blocks.get(1) {
        header.license = license.join("\n");
    }
    if let Some(summary) = blocks.get(2) {
        header.summary = summary.join("\n");
    }
    if let Some(requirements) = blocks.get(3) {
        header.requirements = requirements.join("\n");
    }
    Some(header)
}

// #endregion 🔖️Headers
