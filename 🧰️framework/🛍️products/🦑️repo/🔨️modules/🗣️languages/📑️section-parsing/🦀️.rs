//! 🔖️ Section parsing: marker, markdown, JSON and code section parsing.
//!
//! Holds the `🔖️Sections` region of the crate root; the root re-exports its items.

use super::*;

// #region 🔖️Sections

/// 🧱️ A zeroed section, since the model crate does not derive Default.
fn empty_section() -> Section {
    Section {
        id: String::new(),
        name: String::new(),
        path: String::new(),
        file_path: String::new(),
        emoji: String::new(),
        start_line: 0,
        end_line: 0,
        start_index: 0,
        end_index: 0,
        children: Vec::new(),
        definitions: Vec::new(),
    }
}

pub(super) fn split_lines(content: &str) -> Vec<&str> {
    content.split('\n').collect()
}

fn parse_marker_sections(lang: &Language, content: &str) -> Vec<Section> {
    let Some(start_pat) = lang.section_start.as_ref() else {
        return Vec::new();
    };
    let lines = split_lines(content);
    let mut arena: Vec<Section> = Vec::new();
    let mut children: Vec<Vec<usize>> = Vec::new();
    let mut roots: Vec<usize> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut char_index = 0usize;
    for (i, line) in lines.iter().enumerate() {
        let line_start = char_index;
        let line_num = (i + 1) as i64;
        if let Some((_, caps)) = pattern_match(start_pat, line) {
            let raw = capture(&caps, "name").unwrap_or_default();
            let raw = raw.trim().to_string();
            let (emoji, rest) = extract_entity_emoji(&raw);
            let mut name = rest.trim().to_string();
            if name.is_empty() {
                name = raw.clone();
            }
            arena.push(Section {
                name,
                emoji,
                start_line: line_num,
                end_line: lines.len() as i64,
                start_index: line_start as i64,
                end_index: content.len() as i64,
                ..empty_section()
            });
            children.push(Vec::new());
            let idx = arena.len() - 1;
            match stack.last() {
                Some(&parent) => children[parent].push(idx),
                None => roots.push(idx),
            }
            stack.push(idx);
        } else if lang.section_end.as_ref().is_some_and(|p| pattern_match(p, line).is_some()) {
            if let Some(idx) = stack.pop() {
                arena[idx].end_line = line_num;
                arena[idx].end_index = (char_index + line.len()) as i64;
            }
        }
        char_index += line.len() + 1;
    }
    fn build(idx: usize, arena: &[Section], children: &[Vec<usize>]) -> Section {
        let mut s = arena[idx].clone();
        s.children = children[idx].iter().map(|c| build(*c, arena, children)).collect();
        s
    }
    roots.iter().map(|r| build(*r, &arena, &children)).collect()
}

/// 📰️ Markdown sections from ATX headings, with front-matter lines added to every line number.
pub fn parse_markdown_sections(content: &str) -> Vec<Section> {
    let lang = language_by_name("markdown").expect("markdown is registered");
    let head_pat = lang.section_start.as_ref().expect("markdown declares a heading pattern");
    let lines = split_lines(content);
    let mut frontmatter_lines = 0i64;
    if let Some(rest) = content.strip_prefix("---") {
        if let Some(end_index) = rest.find("---") {
            frontmatter_lines = content[..end_index + 6].matches('\n').count() as i64;
        }
    }
    let mut arena: Vec<Section> = Vec::new();
    let mut children: Vec<Vec<usize>> = Vec::new();
    let mut roots: Vec<usize> = Vec::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut char_index = 0usize;
    for (i, line) in lines.iter().enumerate() {
        let line_start = char_index;
        if let Some((_, caps)) = pattern_match(head_pat, line) {
            let level = capture(&caps, "level").unwrap_or_default().len();
            let name = capture(&caps, "name").unwrap_or_default().trim().to_string();
            while stack.last().is_some_and(|(l, _)| *l >= level) {
                let (_, idx) = stack.pop().expect("stack is not empty");
                arena[idx].end_line = frontmatter_lines + i as i64;
                arena[idx].end_index = line_start as i64 - 1;
            }
            arena.push(Section {
                name,
                start_line: frontmatter_lines + i as i64 + 1,
                end_line: -1,
                start_index: line_start as i64,
                end_index: -1,
                ..empty_section()
            });
            children.push(Vec::new());
            let idx = arena.len() - 1;
            match stack.last() {
                Some(&(_, parent)) => children[parent].push(idx),
                None => roots.push(idx),
            }
            stack.push((level, idx));
        }
        char_index += line.len() + 1;
    }
    while let Some((_, idx)) = stack.pop() {
        arena[idx].end_line = frontmatter_lines + lines.len() as i64;
        arena[idx].end_index = content.len() as i64;
    }
    fn build(idx: usize, arena: &[Section], children: &[Vec<usize>]) -> Section {
        let mut s = arena[idx].clone();
        s.children = children[idx].iter().map(|c| build(*c, arena, children)).collect();
        s
    }
    roots.iter().map(|r| build(*r, &arena, &children)).collect()
}

/// 🔣️ JSON sections: one section per object key, spanning its key through its value.
///
/// The Go original keeps `*Section` pointers into slices it keeps appending to, so every key but the
/// last in an object is written through a stale backing array and comes back with `endLine = -1`.
/// This implementation gives every key its real range; the defect is recorded for the Go split.
pub fn parse_json_sections(content: &str) -> Vec<Section> {
    struct Frame {
        kind: u8,
        section: Option<usize>,
        path: String,
        expect_key: bool,
        location: Option<usize>,
    }
    let bytes = content.as_bytes();
    let mut arena: Vec<Section> = Vec::new();
    let mut children: Vec<Vec<usize>> = Vec::new();
    let mut roots: Vec<usize> = Vec::new();
    let mut stack: Vec<Frame> = Vec::new();
    let mut line = 1i64;
    let mut in_string = false;
    let mut escape = false;
    let mut string_start = 0usize;
    let mut buf = String::new();
    let mut pending_key: Option<(String, usize, i64)> = None;
    let mut awaiting: Option<usize> = None;
    let mut awaiting_value_start: Option<usize> = None;
    let mut i = 0usize;
    while i < bytes.len() {
        let ch = bytes[i];
        if ch == b'\n' {
            line += 1;
        }
        if in_string {
            if escape {
                escape = false;
                buf.push(ch as char);
            }
            if ch == b'\\' {
                escape = true;
                buf.push(ch as char);
                i += 1;
                continue;
            }
            if ch == b'"' {
                in_string = false;
                let value = std::mem::take(&mut buf);
                let expecting = stack.last().is_some_and(|f| f.kind == b'{' && f.expect_key);
                if expecting && awaiting.is_none() {
                    pending_key = Some((value, string_start, line));
                    if let Some(f) = stack.last_mut() {
                        f.expect_key = false;
                    }
                } else if let Some(idx) = awaiting {
                    if awaiting_value_start == Some(string_start) {
                        arena[idx].end_line = line;
                        arena[idx].end_index = i as i64 + 1;
                        awaiting = None;
                        awaiting_value_start = None;
                    }
                }
                i += 1;
                continue;
            }
            buf.push_str(&content[i..i + 1]);
            i += 1;
            continue;
        }
        if ch == b'"' {
            if awaiting.is_some() {
                awaiting_value_start = Some(i);
            }
            in_string = true;
            string_start = i;
            i += 1;
            continue;
        }
        if ch == b':' && pending_key.is_some() && stack.last().is_some_and(|f| f.kind == b'{') {
            let (key, key_start, key_line) = pending_key.take().expect("pending key present");
            let top = stack.last().expect("object frame present");
            let path = if top.path.is_empty() { key.clone() } else { format!("{}/{}", top.path, key) };
            arena.push(Section {
                name: key,
                path: path.clone(),
                start_line: key_line,
                end_line: -1,
                start_index: key_start as i64,
                end_index: -1,
                ..empty_section()
            });
            children.push(Vec::new());
            let idx = arena.len() - 1;
            match top.section {
                Some(parent) => children[parent].push(idx),
                None => roots.push(idx),
            }
            awaiting = Some(idx);
            awaiting_value_start = None;
            i += 1;
            continue;
        }
        if let Some(idx) = awaiting {
            if ch == b'{' || ch == b'[' {
                let path = arena[idx].path.clone();
                stack.push(Frame { kind: ch, section: Some(idx), path, expect_key: ch == b'{', location: Some(idx) });
                awaiting = None;
                awaiting_value_start = None;
                i += 1;
                continue;
            }
            if ch == b'-' || ch.is_ascii_digit() || ch == b't' || ch == b'f' || ch == b'n' {
                let mut end = i;
                while end < bytes.len() {
                    let c = bytes[end];
                    if c == b'\n' {
                        line += 1;
                    }
                    if matches!(c, b',' | b'}' | b']' | b' ' | b'\t' | b'\r' | b'\n') {
                        break;
                    }
                    end += 1;
                }
                arena[idx].end_line = line;
                arena[idx].end_index = end as i64;
                awaiting = None;
                awaiting_value_start = None;
                i = end;
                continue;
            }
        }
        if ch == b'{' || ch == b'[' {
            stack.push(Frame { kind: ch, section: None, path: String::new(), expect_key: ch == b'{', location: None });
            i += 1;
            continue;
        }
        if ch == b'}' || ch == b']' {
            if let Some(top) = stack.pop() {
                if let Some(idx) = top.location {
                    arena[idx].end_line = line;
                    arena[idx].end_index = i as i64 + 1;
                }
                if let Some(f) = stack.last_mut() {
                    if f.kind == b'{' {
                        f.expect_key = true;
                    }
                }
            }
            i += 1;
            continue;
        }
        if ch == b',' {
            if let Some(f) = stack.last_mut() {
                if f.kind == b'{' {
                    f.expect_key = true;
                }
            }
            i += 1;
            continue;
        }
        i += 1;
    }
    for s in arena.iter_mut() {
        if s.end_index == -1 {
            s.end_line = line;
            s.end_index = content.len() as i64;
        }
    }
    fn build(idx: usize, arena: &[Section], children: &[Vec<usize>]) -> Section {
        let mut s = arena[idx].clone();
        s.children = children[idx].iter().map(|c| build(*c, arena, children)).collect();
        s
    }
    roots.iter().map(|r| build(*r, &arena, &children)).collect()
}

/// 📑️ Sections of `content` as read by the language registered under `language_name`.
pub fn parse_code_sections(content: &str, language_name: &str) -> Vec<Section> {
    let Some(lang) = language_by_name(language_name) else {
        return Vec::new();
    };
    if !lang.supports_sections() {
        return Vec::new();
    }
    parse_sections_with(lang, content)
}

/// 📩️ Sections of `content` as read by the language claiming `file_path`.
pub fn parse_sections(content: &str, file_path: &str) -> Vec<Section> {
    match language_for_path(file_path) {
        Some(lang) => parse_sections_with(lang, content),
        None => Vec::new(),
    }
}

/// 🧭️ Sections of `content` under an explicit language.
pub fn parse_sections_with(lang: &Language, content: &str) -> Vec<Section> {
    match lang.section_engine {
        SectionEngine::Markers => parse_marker_sections(lang, content),
        SectionEngine::MarkdownHeadings => parse_markdown_sections(content),
        SectionEngine::JsonKeys => parse_json_sections(content),
        SectionEngine::None => Vec::new(),
    }
}

/// 🛤️ Splits a section path on `#` and `/`, dropping empty segments.
pub fn normalize_section_path(section_path: &str) -> Vec<String> {
    section_path
        .replace('#', "/")
        .split('/')
        .filter(|p| !p.is_empty())
        .map(str::to_string)
        .collect()
}

/// 🎯️ Attaches every definition to the deepest section whose line range contains it.
pub fn hydrate_sections_with_definitions(sections: &[Section], definitions: &[Definition]) -> Vec<Section> {
    sections
        .iter()
        .map(|section| {
            let mut out = section.clone();
            let subset: Vec<Definition> = definitions
                .iter()
                .filter(|d| d.start_line >= out.start_line && d.end_line <= out.end_line)
                .cloned()
                .collect();
            out.children = hydrate_sections_with_definitions(&out.children, &subset);
            out.definitions = subset
                .iter()
                .filter(|d| !out.children.iter().any(|c| d.start_line >= c.start_line && d.end_line <= c.end_line))
                .cloned()
                .collect();
            out
        })
        .collect()
}

// #endregion 🔖️Sections
