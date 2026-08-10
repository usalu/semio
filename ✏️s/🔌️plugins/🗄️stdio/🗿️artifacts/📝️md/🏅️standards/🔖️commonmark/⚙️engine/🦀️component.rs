//! ⚙️ MdEngine — owns a real `MdArtifact` + a real CommonMark-shaped block/inline parser.
//!
//! `MdSnapshot.body` remains the lossless persisted source of truth (round-tripping
//! arbitrary CommonMark as a typed AST losslessly is out of this format's S-M scope —
//! see the D2 ground rule "unmodeled bytes kept verbatim where cheaper than full
//! modeling"); `parse_markdown_blocks` is a real, independently testable read view on
//! top of it, covering https://spec.commonmark.org/'s common real-world subset.

use crate::artifacts::md::schema::snapshot::{MdBlock, MdInline};
use crate::artifacts::md::{MdArtifact, MdDiff, MdMutation, MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_md_snapshot() -> MdSnapshot {
    MdSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Codec
//#region 🔖️BlockLineClassifiers
fn fence_open(line: &str) -> Option<(char, usize, String)> {
    let trimmed = line.trim_start();
    if line.len() - trimmed.len() > 3 {
        return None;
    }
    let ch = trimmed.chars().next()?;
    if ch != '`' && ch != '~' {
        return None;
    }
    let len = trimmed.chars().take_while(|&c| c == ch).count();
    if len < 3 {
        return None;
    }
    Some((ch, len, trimmed[len..].trim().to_string()))
}

fn atx_heading(line: &str) -> Option<(u8, &str)> {
    let trimmed = line.trim_start();
    if line.len() - trimmed.len() > 3 {
        return None;
    }
    let hashes = trimmed.chars().take_while(|&c| c == '#').count();
    if hashes == 0 || hashes > 6 {
        return None;
    }
    let rest = &trimmed[hashes..];
    if !rest.is_empty() && !rest.starts_with(' ') && !rest.starts_with('\t') {
        return None;
    }
    let content = rest.trim();
    let stripped = content.trim_end_matches('#');
    let content = if stripped != content && (stripped.is_empty() || stripped.ends_with(' ')) { stripped.trim_end() } else { content };
    Some((hashes as u8, content))
}

fn indented_code_line(line: &str) -> Option<&str> {
    line.strip_prefix("    ").or_else(|| line.strip_prefix('\t'))
}

fn list_item_marker(line: &str) -> Option<(bool, Option<u64>, &str)> {
    let trimmed = line.trim_start();
    if line.len() - trimmed.len() > 3 {
        return None;
    }
    if let Some(rest) = trimmed.strip_prefix("- ").or_else(|| trimmed.strip_prefix("* ")).or_else(|| trimmed.strip_prefix("+ ")) {
        return Some((false, None, rest));
    }
    let digits: String = trimmed.chars().take_while(|c| c.is_ascii_digit()).collect();
    if !digits.is_empty() {
        let after = &trimmed[digits.len()..];
        if let Some(rest) = after.strip_prefix(". ").or_else(|| after.strip_prefix(") ")) {
            return Some((true, digits.parse::<u64>().ok(), rest));
        }
    }
    None
}
//#endregion 🔖️BlockLineClassifiers

//#region 🔖️BlockParser
/// 📥 Parses a real (common-subset) CommonMark block structure: ATX headings, fenced
/// code blocks (info string preserved), indented code blocks, ordered/unordered lists
/// (single-line items — no nested blocks, a documented scope cut), and paragraphs.
pub fn parse_markdown_blocks(text: &str) -> Vec<MdBlock> {
    let lines: Vec<&str> = text.lines().collect();
    let mut blocks = Vec::new();
    let mut i = 0usize;
    while i < lines.len() {
        let line = lines[i];
        if line.trim().is_empty() {
            i += 1;
            continue;
        }
        if let Some((fence_char, fence_len, info)) = fence_open(line) {
            let mut code_lines = Vec::new();
            i += 1;
            while i < lines.len() {
                if let Some((c2, len2, _)) = fence_open(lines[i]) {
                    if c2 == fence_char && len2 >= fence_len && lines[i].trim().chars().all(|c| c == fence_char) {
                        i += 1;
                        break;
                    }
                }
                code_lines.push(lines[i]);
                i += 1;
            }
            blocks.push(MdBlock::CodeBlock {
                info: if info.is_empty() { None } else { Some(info) },
                code: code_lines.join("\n"),
                fenced: true,
            });
            continue;
        }
        if let Some(rest) = indented_code_line(line) {
            let mut code_lines = vec![rest.to_string()];
            i += 1;
            while i < lines.len() {
                match indented_code_line(lines[i]) {
                    Some(r) => {
                        code_lines.push(r.to_string());
                        i += 1;
                    }
                    None => break,
                }
            }
            blocks.push(MdBlock::CodeBlock { info: None, code: code_lines.join("\n"), fenced: false });
            continue;
        }
        if let Some((level, rest)) = atx_heading(line) {
            blocks.push(MdBlock::Heading { level, inline: parse_inline(rest) });
            i += 1;
            continue;
        }
        if let Some((ordered, start, content)) = list_item_marker(line) {
            let mut items = vec![parse_inline(content)];
            i += 1;
            while i < lines.len() {
                if let Some((ord2, _, content2)) = list_item_marker(lines[i]) {
                    if ord2 == ordered {
                        items.push(parse_inline(content2));
                        i += 1;
                        continue;
                    }
                }
                break;
            }
            blocks.push(MdBlock::List { ordered, start, items });
            continue;
        }
        let mut para_lines = vec![line];
        i += 1;
        while i < lines.len() {
            let l = lines[i];
            if l.trim().is_empty() || fence_open(l).is_some() || indented_code_line(l).is_some() || atx_heading(l).is_some() || list_item_marker(l).is_some() {
                break;
            }
            para_lines.push(l);
            i += 1;
        }
        blocks.push(MdBlock::Paragraph { inline: parse_inline(&para_lines.join(" ")) });
    }
    blocks
}
//#endregion 🔖️BlockParser

//#region 🔖️InlineParser
fn try_parse_delim(chars: &[char], start: usize, delim: char, count: usize) -> Option<(String, usize)> {
    for k in 0..count {
        if chars.get(start + k) != Some(&delim) {
            return None;
        }
    }
    let content_start = start + count;
    let mut j = content_start;
    while j + count <= chars.len() {
        if (0..count).all(|k| chars.get(j + k) == Some(&delim)) {
            if j == content_start {
                return None;
            }
            let inner: String = chars[content_start..j].iter().collect();
            return Some((inner, j + count - start));
        }
        j += 1;
    }
    None
}

fn try_parse_code_span(chars: &[char], start: usize) -> Option<(String, usize)> {
    let mut n = 0usize;
    while chars.get(start + n) == Some(&'`') {
        n += 1;
    }
    let content_start = start + n;
    let mut j = content_start;
    while j < chars.len() {
        if chars[j] == '`' {
            let mut k = 0usize;
            while chars.get(j + k) == Some(&'`') {
                k += 1;
            }
            if k == n {
                let inner: String = chars[content_start..j].iter().collect();
                return Some((inner.trim().to_string(), j + k - start));
            }
            j += k;
        } else {
            j += 1;
        }
    }
    None
}

fn split_url_title(inside: &str) -> (String, Option<String>) {
    let trimmed = inside.trim();
    if let Some(q) = trimmed.find('"') {
        let (url_part, rest) = trimmed.split_at(q);
        let title = rest.trim_matches('"').trim().to_string();
        (url_part.trim().to_string(), if title.is_empty() { None } else { Some(title) })
    } else {
        (trimmed.to_string(), None)
    }
}

fn try_parse_link(chars: &[char], start: usize) -> Option<(MdInline, usize)> {
    let mut j = start + 1;
    let text_start = j;
    let mut depth = 1i32;
    while j < chars.len() {
        match chars[j] {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }
        j += 1;
    }
    if j >= chars.len() || chars[j] != ']' {
        return None;
    }
    let text_end = j;
    if chars.get(j + 1) != Some(&'(') {
        return None;
    }
    let url_start = j + 2;
    let mut k = url_start;
    let mut paren_depth = 1i32;
    while k < chars.len() {
        match chars[k] {
            '(' => paren_depth += 1,
            ')' => {
                paren_depth -= 1;
                if paren_depth == 0 {
                    break;
                }
            }
            _ => {}
        }
        k += 1;
    }
    if k >= chars.len() || chars[k] != ')' {
        return None;
    }
    let inside: String = chars[url_start..k].iter().collect();
    let (url, title) = split_url_title(&inside);
    let text: String = chars[text_start..text_end].iter().collect();
    Some((MdInline::Link { text: parse_inline(&text), url, title }, k + 1 - start))
}

/// 📥 Parses a real (common-subset) inline run: links, inline code spans, strong
/// (`**`/`__`), emphasis (`*`/`_`), plain text.
pub fn parse_inline(text: &str) -> Vec<MdInline> {
    let chars: Vec<char> = text.chars().collect();
    let mut nodes = Vec::new();
    let mut buf = String::new();
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i] == '[' {
            if let Some((link, consumed)) = try_parse_link(&chars, i) {
                if !buf.is_empty() {
                    nodes.push(MdInline::Text(std::mem::take(&mut buf)));
                }
                nodes.push(link);
                i += consumed;
                continue;
            }
        }
        if chars[i] == '`' {
            if let Some((code, consumed)) = try_parse_code_span(&chars, i) {
                if !buf.is_empty() {
                    nodes.push(MdInline::Text(std::mem::take(&mut buf)));
                }
                nodes.push(MdInline::Code(code));
                i += consumed;
                continue;
            }
        }
        if (chars[i] == '*' || chars[i] == '_') && chars.get(i + 1) == Some(&chars[i]) {
            if let Some((inner, consumed)) = try_parse_delim(&chars, i, chars[i], 2) {
                if !buf.is_empty() {
                    nodes.push(MdInline::Text(std::mem::take(&mut buf)));
                }
                nodes.push(MdInline::Strong(parse_inline(&inner)));
                i += consumed;
                continue;
            }
        }
        if chars[i] == '*' || chars[i] == '_' {
            if let Some((inner, consumed)) = try_parse_delim(&chars, i, chars[i], 1) {
                if !buf.is_empty() {
                    nodes.push(MdInline::Text(std::mem::take(&mut buf)));
                }
                nodes.push(MdInline::Emphasis(parse_inline(&inner)));
                i += consumed;
                continue;
            }
        }
        buf.push(chars[i]);
        i += 1;
    }
    if !buf.is_empty() {
        nodes.push(MdInline::Text(buf));
    }
    nodes
}
//#endregion 🔖️InlineParser
//#endregion 🔖️Codec

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::md::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<MdSnapshot, MdMutation>(STDIO_MD_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.md",
        extension: Some("md"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::md::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::md::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::md::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::md::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.md"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.md`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::md::schema::md_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.md` artifact engine.
pub struct MdEngine {
    artifact_state: MdArtifact,
    snapshot_state: MdSnapshot,
}

impl MdEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: MdSnapshot) -> Self {
        let artifact_state = MdArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_md_snapshot();
        assert_eq!(snapshot.schema, STDIO_MD_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_md_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <MdSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <MdSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    #[test]
    fn headings_all_levels() {
        let blocks = parse_markdown_blocks("# H1\n## H2\n###### H6\n");
        assert_eq!(blocks.len(), 3);
        assert!(matches!(&blocks[0], MdBlock::Heading { level: 1, .. }));
        assert!(matches!(&blocks[1], MdBlock::Heading { level: 2, .. }));
        assert!(matches!(&blocks[2], MdBlock::Heading { level: 6, .. }));
    }

    #[test]
    fn paragraph_and_fenced_code_block_with_info_string() {
        let text = "A paragraph of text.\n\n```rust\nfn main() {}\n```\n";
        let blocks = parse_markdown_blocks(text);
        assert_eq!(blocks.len(), 2);
        assert!(matches!(&blocks[0], MdBlock::Paragraph { .. }));
        match &blocks[1] {
            MdBlock::CodeBlock { info, code, fenced } => {
                assert_eq!(info.as_deref(), Some("rust"));
                assert_eq!(code, "fn main() {}");
                assert!(*fenced);
            }
            other => panic!("expected fenced code block, got {other:?}"),
        }
    }

    #[test]
    fn indented_code_block() {
        let text = "    let x = 1;\n    let y = 2;\n";
        let blocks = parse_markdown_blocks(text);
        assert_eq!(blocks.len(), 1);
        match &blocks[0] {
            MdBlock::CodeBlock { fenced, code, .. } => {
                assert!(!fenced);
                assert_eq!(code, "let x = 1;\nlet y = 2;");
            }
            other => panic!("expected indented code block, got {other:?}"),
        }
    }

    #[test]
    fn unordered_and_ordered_lists() {
        let unordered = parse_markdown_blocks("- one\n- two\n- three\n");
        assert_eq!(unordered.len(), 1);
        match &unordered[0] {
            MdBlock::List { ordered, items, .. } => {
                assert!(!ordered);
                assert_eq!(items.len(), 3);
            }
            other => panic!("expected list, got {other:?}"),
        }

        let ordered = parse_markdown_blocks("1. first\n2. second\n");
        match &ordered[0] {
            MdBlock::List { ordered, start, items } => {
                assert!(*ordered);
                assert_eq!(*start, Some(1));
                assert_eq!(items.len(), 2);
            }
            other => panic!("expected ordered list, got {other:?}"),
        }
    }

    #[test]
    fn emphasis_strong_and_links_in_inline() {
        let inline = parse_inline("plain **strong** and *em* and [a link](https://example.com \"title\")");
        assert!(inline.iter().any(|n| matches!(n, MdInline::Strong(inner) if inner == &vec![MdInline::Text("strong".into())])));
        assert!(inline.iter().any(|n| matches!(n, MdInline::Emphasis(inner) if inner == &vec![MdInline::Text("em".into())])));
        let link = inline.iter().find_map(|n| match n {
            MdInline::Link { text, url, title } => Some((text.clone(), url.clone(), title.clone())),
            _ => None,
        }).expect("link present");
        assert_eq!(link.0, vec![MdInline::Text("a link".into())]);
        assert_eq!(link.1, "https://example.com");
        assert_eq!(link.2.as_deref(), Some("title"));
    }

    #[test]
    fn inline_code_span_is_not_emphasis() {
        let inline = parse_inline("use `*not emphasis*` here");
        assert!(inline.iter().any(|n| matches!(n, MdInline::Code(c) if c == "*not emphasis*")));
    }
}
//#endregion 🧪️Tests
