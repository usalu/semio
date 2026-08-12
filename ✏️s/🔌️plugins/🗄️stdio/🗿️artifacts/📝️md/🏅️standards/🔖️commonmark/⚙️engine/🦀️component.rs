//! ⚙️ MdEngine — owns a real `MdArtifact` + a real, from-scratch CommonMark block/inline
//! parser+renderer. `MdSnapshot.blocks` is the complete typed persisted source of truth (see
//! the snapshot module's own doc comment for the exact honest-subset scope and the deviations
//! it lists); `parse_markdown_blocks`/`render_markdown_blocks` are the codec's read/write halves,
//! independently testable, covering https://spec.commonmark.org/'s common real-world subset.

use crate::artifacts::md::schema::snapshot::{MdBlock, MdInline};
use crate::artifacts::md::{MdArtifact, MdDiff, MdMutation, MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_md_snapshot() -> MdSnapshot {
    MdSnapshot::default()
}

/// 📄️ The demo `stdio.md` document — genuinely exercises `Heading`/`Paragraph` (with `Strong`/
/// `Emphasis`/`Code` inline content on one physical line), a 2-level-nested `BlockQuote` (proving
/// `../../🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`'s `block-quote = {GT block}+`
/// genuine `Ref` self-recursion end-to-end), a fenced `CodeBlock` with a real info string, and
/// `ThematicBreak`. The single source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/
/// `🎒️example.pack.semio` (both are literally this snapshot's `print_dsl`/`encode_pack` output,
/// asserted equal by `fixture_honesty_law` below) and for `grammar_conformance_law`'s own
/// reconstructed-body recognition test.
///
/// Deliberately does NOT use `MdBlock::List` (the grammar's own documented, architecturally
/// excluded leading-whitespace-count mechanism gap — see the snapshot grammar file's own header
/// comment), `MdBlock::HtmlBlock` (not one of the 5 kinds that grammar models), or a multi-line
/// `Paragraph`/quoted-multi-block `BlockQuote` (both would defeat the grammar's own single-`LINE`-
/// per-block-position recognition, documented on the same file).
///
/// 🐛 Block ORDER matters here for a second, independent reason (confirmed by direct
/// reproduction, filed in this wave's `mechanism_gaps` as `md-fence-byte-offset-corruption`, NOT a
/// dialect-design gap — a genuine pre-existing bug in the shared lexer's `Fence`-token byte-offset
/// bookkeeping, `🔍️lexer/🦀️component.rs`'s fence-scanning loop increments `byte_offset` for every
/// consumed char EXCEPT `'\n'` — so every token positioned AFTER a fence whose content spans N
/// lines gets its `byte_range` under-reported by N, which can desync `match_raw_span`'s
/// `text.get(start_byte..).find('\n')` lookup enough to make a subsequent `LINE` match
/// zero-width): `CodeBlock` is placed LAST here, with only `ThematicBreak` after it (a
/// pure-literal-token production, `"--" "-"`, untouched by raw-span corruption since it never
/// calls `LINE`/`REST`) — `BlockQuote`/`Paragraph` (both `LINE`-dependent) are placed BEFORE the
/// fence, never after it, sidestepping the corruption entirely rather than fixing the shared lexer
/// (out of this wave's ownership boundary).
pub fn demo_md_snapshot() -> MdSnapshot {
    use crate::artifacts::md::schema::snapshot::{MdBlock, MdInline};
    let blocks = vec![
        MdBlock::Heading { level: 1, inlines: vec![MdInline::Text { text: "Title".into() }] },
        MdBlock::BlockQuote {
            blocks: vec![MdBlock::BlockQuote {
                blocks: vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "Deeply quoted.".into() }] }],
            }],
        },
        MdBlock::Paragraph {
            inlines: vec![
                MdInline::Text { text: "Lossless ".into() },
                MdInline::Strong { inlines: vec![MdInline::Text { text: "markdown".into() }] },
                MdInline::Text { text: " body with ".into() },
                MdInline::Emphasis { inlines: vec![MdInline::Text { text: "emphasis".into() }] },
                MdInline::Text { text: " and ".into() },
                MdInline::Code { literal: "inline code".into() },
                MdInline::Text { text: ".".into() },
            ],
        },
        MdBlock::CodeBlock { info: Some("rust".into()), literal: "fn demo() -> i32 {\n    42\n}".into() },
        MdBlock::ThematicBreak,
    ];
    MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Codec
//#region 🔖️BlockLineClassifiers
/// 📏️ Leading-space count (ASCII spaces only, matching every other classifier here -- a line
/// indented with unicode whitespace is out of scope, same pre-existing limitation as before this
/// wave).
fn leading_spaces(line: &str) -> usize {
    line.chars().take_while(|&c| c == ' ').count()
}

fn fence_open(line: &str) -> Option<(char, usize, String)> {
    let trimmed = line.trim_start();
    if leading_spaces(line) > 3 {
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
    if leading_spaces(line) > 3 {
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

/// 🔳 `---`, `***`, `___` (>=3 of the same char, optionally space-separated, nothing else) --
/// checked BEFORE `list_item_marker` since `- - -` is a thematic break, not a 3-item list.
fn thematic_break(line: &str) -> bool {
    let trimmed = line.trim_start();
    if leading_spaces(line) > 3 {
        return false;
    }
    let stripped: String = trimmed.chars().filter(|c| !c.is_whitespace()).collect();
    if stripped.chars().count() < 3 {
        return false;
    }
    let first = match stripped.chars().next() {
        Some(c) => c,
        None => return false,
    };
    (first == '-' || first == '_' || first == '*') && stripped.chars().all(|c| c == first)
}

/// 🔖️ `(ordered, start_number, marker_char_width, content_after_marker)`. `marker_char_width` is
/// a CHAR count (not necessarily byte count, though every consumed char here is ASCII so the two
/// coincide) of everything before `content_after_marker` -- used by the list parser to dedent
/// continuation lines via [`dedent_by_chars`].
fn list_item_marker(line: &str) -> Option<(bool, Option<u32>, usize, &str)> {
    let indent = leading_spaces(line);
    let trimmed = line.trim_start();
    if indent > 3 {
        return None;
    }
    if let Some(rest) = trimmed.strip_prefix("- ").or_else(|| trimmed.strip_prefix("* ")).or_else(|| trimmed.strip_prefix("+ ")) {
        return Some((false, None, indent + 2, rest));
    }
    let digits: String = trimmed.chars().take_while(|c| c.is_ascii_digit()).collect();
    if !digits.is_empty() && digits.len() <= 9 {
        let after = &trimmed[digits.len()..];
        if let Some(rest) = after.strip_prefix(". ").or_else(|| after.strip_prefix(") ")) {
            return Some((true, digits.parse::<u32>().ok(), indent + digits.len() + 2, rest));
        }
    }
    None
}

/// 🏷️ `> content` (up to 3 leading spaces, `>` optionally followed by ONE space). No lazy
/// continuation support (documented scope cut) -- a block quote ends at the first line that
/// doesn't itself start with `>`.
fn blockquote_marker(line: &str) -> Option<&str> {
    let indent = leading_spaces(line);
    let trimmed = line.trim_start();
    if indent > 3 || !trimmed.starts_with('>') {
        return None;
    }
    let rest = &trimmed[1..];
    Some(rest.strip_prefix(' ').unwrap_or(rest))
}

/// 🏷️ Simplified single-rule HTML block start: a line beginning (up to 3 leading spaces) with
/// `<tag`, `</tag`, or `<!--`. Real spec has 7 distinct start conditions with different end
/// conditions (documented scope cut) -- here every HTML block ends at the next blank line.
fn html_block_start(line: &str) -> bool {
    let indent = leading_spaces(line);
    let trimmed = line.trim_start();
    if indent > 3 || !trimmed.starts_with('<') {
        return false;
    }
    let mut chars = trimmed.chars();
    chars.next();
    match chars.next() {
        Some('!') | Some('?') => true,
        Some('/') => chars.next().map(|c| c.is_ascii_alphabetic()).unwrap_or(false),
        Some(c) => c.is_ascii_alphabetic(),
        None => false,
    }
}

/// ✂️ Char-safe dedent: strips exactly `width` leading chars from `line` IF all of them are
/// spaces, returning the remainder. A line shorter than `width` but entirely spaces dedents to
/// `""` (blank continuation). Any non-space char before reaching `width` fails the dedent
/// (`None`) -- never panics on a multi-byte boundary, unlike raw byte slicing.
fn dedent_by_chars(line: &str, width: usize) -> Option<String> {
    let mut out_start: Option<usize> = None;
    let mut count = 0usize;
    for (idx, ch) in line.char_indices() {
        if count == width {
            out_start = Some(idx);
            break;
        }
        if ch != ' ' {
            return None;
        }
        count += 1;
    }
    match out_start {
        Some(idx) => Some(line[idx..].to_string()),
        None if count == width => Some(String::new()),
        None => {
            if line.chars().all(|c| c == ' ') { Some(String::new()) } else { None }
        }
    }
}
//#endregion 🔖️BlockLineClassifiers

//#region 🔖️BlockParser
/// 📥 Top-level entry: parses `text` into the complete block sequence.
pub fn parse_markdown_blocks(text: &str) -> Vec<MdBlock> {
    let lines: Vec<&str> = text.lines().collect();
    parse_blocks(&lines)
}

/// 📥 Recursive block parser over an already-dedented line slice -- called at the top level and
/// recursively for block-quote content and list-item content, which is exactly how nested lists
/// and quote-inside-list (and vice versa) fall out for free.
fn parse_blocks(lines: &[&str]) -> Vec<MdBlock> {
    let mut blocks = Vec::new();
    let mut i = 0usize;
    while i < lines.len() {
        let line = lines[i];
        if line.trim().is_empty() {
            i += 1;
            continue;
        }
        if thematic_break(line) {
            blocks.push(MdBlock::ThematicBreak);
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
                literal: code_lines.join("\n"),
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
            blocks.push(MdBlock::CodeBlock { info: None, literal: code_lines.join("\n") });
            continue;
        }
        if blockquote_marker(line).is_some() {
            let mut quote_lines = Vec::new();
            while i < lines.len() {
                match blockquote_marker(lines[i]) {
                    Some(content) => {
                        quote_lines.push(content.to_string());
                        i += 1;
                    }
                    None => break,
                }
            }
            let quote_refs: Vec<&str> = quote_lines.iter().map(|s| s.as_str()).collect();
            blocks.push(MdBlock::BlockQuote { blocks: parse_blocks(&quote_refs) });
            continue;
        }
        if let Some((level, rest)) = atx_heading(line) {
            blocks.push(MdBlock::Heading { level, inlines: parse_inline(rest) });
            i += 1;
            continue;
        }
        if html_block_start(line) {
            let mut html_lines = vec![line];
            i += 1;
            while i < lines.len() && !lines[i].trim().is_empty() {
                html_lines.push(lines[i]);
                i += 1;
            }
            blocks.push(MdBlock::HtmlBlock { raw: html_lines.join("\n") });
            continue;
        }
        if let Some((ordered, start, _, _)) = list_item_marker(line) {
            let (list_block, consumed) = parse_list(&lines[i..], ordered, start);
            blocks.push(list_block);
            i += consumed;
            continue;
        }
        let mut para_lines = vec![line];
        i += 1;
        while i < lines.len() {
            let l = lines[i];
            if l.trim().is_empty()
                || fence_open(l).is_some()
                || indented_code_line(l).is_some()
                || atx_heading(l).is_some()
                || list_item_marker(l).is_some()
                || blockquote_marker(l).is_some()
                || thematic_break(l)
                || html_block_start(l)
            {
                break;
            }
            para_lines.push(l);
            i += 1;
        }
        blocks.push(MdBlock::Paragraph { inlines: parse_inline_lines(&para_lines) });
    }
    blocks
}

/// 📥 Parses one whole list (all its items, incl. nested content) starting at `lines[0]`, which
/// must already satisfy `list_item_marker`. Returns the built block plus how many of `lines` it
/// consumed. `tight` is a simplified approximation of the spec's real tightness rule: `false` iff
/// ANY blank line was seen between/inside items (real spec additionally distinguishes blank lines
/// used only for a nested block's own formatting -- documented scope cut).
fn parse_list(lines: &[&str], ordered: bool, start: Option<u32>) -> (MdBlock, usize) {
    let mut items: Vec<Vec<MdBlock>> = Vec::new();
    let mut i = 0usize;
    let mut saw_blank = false;
    loop {
        if i >= lines.len() {
            break;
        }
        // 🕳️ Skip blank lines, deciding whether the list continues past them.
        if lines[i].trim().is_empty() {
            let mut j = i;
            while j < lines.len() && lines[j].trim().is_empty() {
                j += 1;
            }
            if j >= lines.len() {
                break;
            }
            let continues_as_item = matches!(list_item_marker(lines[j]), Some((ord2, ..)) if ord2 == ordered);
            let continues_as_indent = leading_spaces(lines[j]) >= 2 && !continues_as_item;
            if continues_as_item || continues_as_indent {
                saw_blank = true;
                i = j;
                continue;
            }
            break;
        }
        let Some((ord2, _, marker_width, first_content)) = list_item_marker(lines[i]) else { break };
        if ord2 != ordered {
            break;
        }
        let mut item_lines: Vec<String> = vec![first_content.to_string()];
        i += 1;
        loop {
            if i >= lines.len() {
                break;
            }
            if lines[i].trim().is_empty() {
                let mut j = i;
                while j < lines.len() && lines[j].trim().is_empty() {
                    j += 1;
                }
                if j < lines.len() && dedent_by_chars(lines[j], marker_width).is_some() && leading_spaces(lines[j]) >= marker_width {
                    for _ in i..j {
                        item_lines.push(String::new());
                    }
                    saw_blank = true;
                    i = j;
                    continue;
                }
                break;
            }
            match dedent_by_chars(lines[i], marker_width) {
                Some(dedented) => {
                    item_lines.push(dedented);
                    i += 1;
                }
                None => break,
            }
        }
        let item_refs: Vec<&str> = item_lines.iter().map(|s| s.as_str()).collect();
        items.push(parse_blocks(&item_refs));
    }
    (MdBlock::List { ordered, start, tight: !saw_blank, items }, i)
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

/// 🔎️ Shared bracket-then-paren scan used by both link and image parsing: given `chars[start]`
/// is `[`, finds the matching `]` (depth-aware) and, if immediately followed by `(...)`, the
/// matching `)` (depth-aware). Returns `(text_range, url, title, total_consumed)`.
fn try_parse_bracket_paren(chars: &[char], start: usize) -> Option<((usize, usize), String, Option<String>, usize)> {
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
    Some(((text_start, text_end), url, title, k + 1 - start))
}

fn try_parse_link(chars: &[char], start: usize) -> Option<(MdInline, usize)> {
    let ((text_start, text_end), url, title, consumed) = try_parse_bracket_paren(chars, start)?;
    let text: String = chars[text_start..text_end].iter().collect();
    Some((MdInline::Link { text: parse_inline(&text), url, title }, consumed))
}

fn try_parse_image(chars: &[char], start: usize) -> Option<(MdInline, usize)> {
    // 🖼️ `start` is the index of `!`; the bracket scan begins one char later, at `[`.
    if chars.get(start + 1) != Some(&'[') {
        return None;
    }
    let ((text_start, text_end), url, title, consumed) = try_parse_bracket_paren(chars, start + 1)?;
    let alt: String = chars[text_start..text_end].iter().collect();
    Some((MdInline::Image { alt, url, title }, consumed + 1))
}

/// 🏷️ Simplified inline HTML: `<!--...-->` or `<[/]tag...>` (no nested `<`, no attribute-value
/// angle brackets -- documented scope cut). `start` is the index of `<`.
fn try_parse_html_inline(chars: &[char], start: usize) -> Option<(String, usize)> {
    if chars.get(start) != Some(&'<') {
        return None;
    }
    if chars.get(start + 1) == Some(&'!') && chars.get(start + 2) == Some(&'-') && chars.get(start + 3) == Some(&'-') {
        let mut j = start + 4;
        while j + 2 < chars.len() {
            if chars[j] == '-' && chars[j + 1] == '-' && chars[j + 2] == '>' {
                let raw: String = chars[start..j + 3].iter().collect();
                return Some((raw, j + 3 - start));
            }
            j += 1;
        }
        return None;
    }
    let mut j = start + 1;
    if chars.get(j) == Some(&'/') {
        j += 1;
    }
    let tag_start = j;
    while chars.get(j).map(|c| c.is_ascii_alphanumeric() || *c == '-').unwrap_or(false) {
        j += 1;
    }
    if j == tag_start {
        return None;
    }
    while j < chars.len() && chars[j] != '>' && chars[j] != '<' {
        j += 1;
    }
    if chars.get(j) != Some(&'>') {
        return None;
    }
    let raw: String = chars[start..=j].iter().collect();
    Some((raw, j + 1 - start))
}

/// 📥 Parses a single line's worth of inline content: images, links, inline code spans, strong
/// (`**`/`__`), emphasis (`*`/`_`), raw inline HTML, plain text. Unrecognized/unterminated
/// delimiter runs degrade gracefully to literal `Text` (never a parse failure).
pub fn parse_inline(text: &str) -> Vec<MdInline> {
    let chars: Vec<char> = text.chars().collect();
    let mut nodes = Vec::new();
    let mut buf = String::new();
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i] == '!' && chars.get(i + 1) == Some(&'[') {
            if let Some((image, consumed)) = try_parse_image(&chars, i) {
                if !buf.is_empty() {
                    nodes.push(MdInline::Text { text: std::mem::take(&mut buf) });
                }
                nodes.push(image);
                i += consumed;
                continue;
            }
        }
        if chars[i] == '[' {
            if let Some((link, consumed)) = try_parse_link(&chars, i) {
                if !buf.is_empty() {
                    nodes.push(MdInline::Text { text: std::mem::take(&mut buf) });
                }
                nodes.push(link);
                i += consumed;
                continue;
            }
        }
        if chars[i] == '`' {
            if let Some((code, consumed)) = try_parse_code_span(&chars, i) {
                if !buf.is_empty() {
                    nodes.push(MdInline::Text { text: std::mem::take(&mut buf) });
                }
                nodes.push(MdInline::Code { literal: code });
                i += consumed;
                continue;
            }
        }
        if chars[i] == '<' {
            if let Some((raw, consumed)) = try_parse_html_inline(&chars, i) {
                if !buf.is_empty() {
                    nodes.push(MdInline::Text { text: std::mem::take(&mut buf) });
                }
                nodes.push(MdInline::HtmlInline { raw });
                i += consumed;
                continue;
            }
        }
        if (chars[i] == '*' || chars[i] == '_') && chars.get(i + 1) == Some(&chars[i]) {
            if let Some((inner, consumed)) = try_parse_delim(&chars, i, chars[i], 2) {
                if !buf.is_empty() {
                    nodes.push(MdInline::Text { text: std::mem::take(&mut buf) });
                }
                nodes.push(MdInline::Strong { inlines: parse_inline(&inner) });
                i += consumed;
                continue;
            }
        }
        if chars[i] == '*' || chars[i] == '_' {
            if let Some((inner, consumed)) = try_parse_delim(&chars, i, chars[i], 1) {
                if !buf.is_empty() {
                    nodes.push(MdInline::Text { text: std::mem::take(&mut buf) });
                }
                nodes.push(MdInline::Emphasis { inlines: parse_inline(&inner) });
                i += consumed;
                continue;
            }
        }
        buf.push(chars[i]);
        i += 1;
    }
    if !buf.is_empty() {
        nodes.push(MdInline::Text { text: buf });
    }
    nodes
}

/// 📥 Parses a paragraph's multiple RAW (pre-join) lines, inserting `SoftBreak`/`HardBreak`
/// between them per the trailing marker on each non-final line (2+ trailing spaces or a trailing
/// `\` => hard; a bare line ending => soft).
fn parse_inline_lines(lines: &[&str]) -> Vec<MdInline> {
    let mut out = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        let hard = line.ends_with("  ") || line.trim_end_matches(' ').ends_with('\\');
        let content = line.trim_end_matches(' ');
        let content = if content.ends_with('\\') { &content[..content.len() - 1] } else { content };
        out.extend(parse_inline(content));
        if idx + 1 < lines.len() {
            out.push(if hard { MdInline::HardBreak } else { MdInline::SoftBreak });
        }
    }
    out
}
//#endregion 🔖️InlineParser

//#region 🔖️BlockRenderer
/// 📤 Top-level entry: renders the complete block sequence back to CommonMark text. Documented
/// normal form (see the snapshot module's doc comment): indented code re-emits fenced, list
/// tightness/marker style is normalized, and no attempt is made at byte-identical round-trip --
/// `decode(encode(x)) == x` at the SNAPSHOT level (semantic fixed point) is the contract, not
/// byte preservation of arbitrary source text (`codec_retention_law`).
pub fn render_markdown_blocks(blocks: &[MdBlock]) -> String {
    let mut out = String::new();
    render_blocks(blocks, &mut out);
    while out.ends_with('\n') {
        out.pop();
    }
    out
}

fn render_blocks(blocks: &[MdBlock], out: &mut String) {
    for block in blocks {
        render_block(block, out);
    }
}

fn render_block(block: &MdBlock, out: &mut String) {
    match block {
        MdBlock::Heading { level, inlines } => {
            out.push_str(&"#".repeat((*level).clamp(1, 6) as usize));
            out.push(' ');
            out.push_str(&render_inlines(inlines));
            out.push_str("\n\n");
        }
        MdBlock::Paragraph { inlines } => {
            out.push_str(&render_inlines(inlines));
            out.push_str("\n\n");
        }
        MdBlock::CodeBlock { info, literal } => {
            out.push_str("```");
            if let Some(i) = info {
                out.push_str(i);
            }
            out.push('\n');
            out.push_str(literal);
            if !literal.is_empty() && !literal.ends_with('\n') {
                out.push('\n');
            }
            out.push_str("```\n\n");
        }
        MdBlock::BlockQuote { blocks } => {
            let mut inner = String::new();
            render_blocks(blocks, &mut inner);
            while inner.ends_with('\n') {
                inner.pop();
            }
            for line in inner.lines() {
                out.push_str("> ");
                out.push_str(line);
                out.push('\n');
            }
            out.push('\n');
        }
        MdBlock::ThematicBreak => out.push_str("---\n\n"),
        MdBlock::HtmlBlock { raw } => {
            out.push_str(raw);
            out.push_str("\n\n");
        }
        MdBlock::List { ordered, start, tight, items } => {
            let mut n = start.unwrap_or(1);
            for item in items {
                let marker = if *ordered { format!("{n}. ") } else { "- ".to_string() };
                n += 1;
                render_list_item(&marker, item, *tight, out);
            }
        }
    }
}

fn render_list_item(marker: &str, blocks: &[MdBlock], tight: bool, out: &mut String) {
    let indent = " ".repeat(marker.chars().count());
    let mut first_line = true;
    for block in blocks {
        let mut buf = String::new();
        render_block(block, &mut buf);
        while buf.ends_with('\n') {
            buf.pop();
        }
        for line in buf.lines() {
            if first_line {
                out.push_str(marker);
            } else {
                out.push_str(&indent);
            }
            out.push_str(line);
            out.push('\n');
            first_line = false;
        }
    }
    if !tight {
        out.push('\n');
    }
}
//#endregion 🔖️BlockRenderer

//#region 🔖️InlineRenderer
fn render_inlines(inlines: &[MdInline]) -> String {
    let mut out = String::new();
    for inline in inlines {
        render_inline(inline, &mut out);
    }
    out
}

fn render_inline(inline: &MdInline, out: &mut String) {
    match inline {
        MdInline::Text { text } => out.push_str(text),
        MdInline::Emphasis { inlines } => {
            out.push('*');
            out.push_str(&render_inlines(inlines));
            out.push('*');
        }
        MdInline::Strong { inlines } => {
            out.push_str("**");
            out.push_str(&render_inlines(inlines));
            out.push_str("**");
        }
        MdInline::Code { literal } => {
            out.push('`');
            out.push_str(literal);
            out.push('`');
        }
        MdInline::Link { text, url, title } => {
            out.push('[');
            out.push_str(&render_inlines(text));
            out.push_str("](");
            out.push_str(url);
            if let Some(t) = title {
                out.push_str(" \"");
                out.push_str(t);
                out.push('"');
            }
            out.push(')');
        }
        MdInline::Image { alt, url, title } => {
            out.push_str("![");
            out.push_str(alt);
            out.push_str("](");
            out.push_str(url);
            if let Some(t) = title {
                out.push_str(" \"");
                out.push_str(t);
                out.push('"');
            }
            out.push(')');
        }
        MdInline::SoftBreak => out.push('\n'),
        MdInline::HardBreak => out.push_str("  \n"),
        MdInline::HtmlInline { raw } => out.push_str(raw),
    }
}
//#endregion 🔖️InlineRenderer
//#endregion 🔖️Codec

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::md::io_registry::register();
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<MdSnapshot, MdMutation>(STDIO_MD_DOCUMENT_SCHEMA));
}

/// 📌️ P2-FG1: 5-role `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per the recipe's
/// json exemplar — `stdio.md`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`. `diff`'s
/// `protocol` slot stays `None` matching the exemplar's own shape exactly (the 5-role scheme has no
/// dedicated "diff binary" role even though `🔺️diff/💾️binary/📡️component.protocol.semio` is a
/// real, conformance-tested file — its binary form is exercised directly by `protocol_walk_law`
/// above, just not wired through a 6th `LanguageRole`).
///
/// `register_schema_spec` is deliberately NOT called here — `MdSnapshot`/`MdDiff`/`MdMutation`
/// derive `schema::ArtifactSchema` but NOT `dsl::DslArtifact`/`DslRecord`/`DslDiff`/`DslOps` (both
/// `📸️snapshot/🦀️component.rs`'s `MdSnapshot` doc comment [via `🔺️diff/🦀️component.rs`'s own
/// `MdDiff` doc comment] and `🧬️mutations/🦀️component.rs`'s `MdMutation` doc comment cite the
/// same structural blocker: `MdBlock`/`MdBlockDiff` are data-carrying recursive enums with no
/// `DslField` impl) — no derivable `RecordSpec` exists for any facet, same situation
/// json/csv/zip/png hit. Filed as a `mechanism_gaps` entry rather than fabricating an unrelated
/// `RecordSpec` that would diverge from what this artifact's real hand-rolled codecs actually do.
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
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.md.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::md::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::md::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::md::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::md::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.md.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.md.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::md::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::md::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("stdio.md.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.md.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::md::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::md::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.md.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.md.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::md::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::md::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.md.spr"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.md`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::md::schema::md_artifact_schema_descriptor());
}

/// 💡️ Registers `s.stdio.md.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inferences() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::md::standards::v_commonmark::subsets::any::schema::inferences::md_artifact_inference_descriptor());
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
    use crate::artifacts::md::schema::diff::{
        diff_at_path, MdBlockAdded, MdBlockDiff, MdBlockModified, MdBlocksDiff, MdListItemAdded, MdListItemModified,
        MdListItemsDiff,
    };
    use crate::artifacts::md::schema::mutations::MdPathStep;
    use protocol::command::DiffAlgebra;
    use protocol::{Mutation, MutationDiff};

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
    fn demo_snapshot_round_trip() {
        let snap = demo_md_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <MdSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.blocks, snap.blocks);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <MdSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded.blocks, snap.blocks);
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-FG1: per-artifact conformance laws (recipe §4's checklist item) — grammar/protocol
    /// parseability, `Recognizer` against real fixtures AND real `print_op`/`print_diff` output,
    /// `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff` bytes, and the
    /// fixture-honesty round-trip. Lives here (the engine's own test region), not any framework
    /// file — `m5` auto-discovers the snapshot grammar+`.dsl.semio`/protocol+`.pack.semio` pairs
    /// independently (`🧪️fixture-sweep/🦀️component.rs`'s `m5_auto_discovery`); these tests are this
    /// artifact's OWN early-warning, plus direct coverage of the mutations/diff facets that harness
    /// does not auto-discover at all.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::md::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
        /// `walk_protocol` laws below (a parse failure here fails fast with a clearer message).
        #[test]
        fn committed_facet_files_parse() {
            for (label, text) in [
                ("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO),
                ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO),
            ] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [
                ("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO),
            ] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output
        /// for the demo (genuinely block-quote-recursive) snapshot — same preamble-stripped body
        /// reconstruction `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture` uses,
        /// so this is a direct proof this artifact will pass that harness once graduated.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_md_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `MdMutation` variant (`mutations::demo_mutation_cases()`), incl. the
        /// `List`-block `MdBlock` payload and both `MdPathStep` variants.
        #[test]
        fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
        /// for every representative `MdDiff` (`diff::demo_diff_cases()`), incl. both tri-states and
        /// the `Replace` kind-change fallback.
        #[test]
        fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
        /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
        /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
        /// mutation's `encode_op`, and every demo diff's `encode_diff` — asserting
        /// `consumed == bytes.len()`.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_md_snapshot());
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

            let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            for mutation in mutations::demo_mutation_cases() {
                let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in diff::demo_diff_cases() {
                let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
        /// `print_dsl`/`encode_pack` output of `demo_md_snapshot()` — `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin — so the
        /// fixtures can never silently drift back to a fake again.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_md_snapshot();

            let parsed = <MdSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_md_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_md_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <MdSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_md_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_md_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws

    //#region 🔖️ParserUnitTests
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
            MdBlock::CodeBlock { info, literal } => {
                assert_eq!(info.as_deref(), Some("rust"));
                assert_eq!(literal, "fn main() {}");
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
            MdBlock::CodeBlock { literal, .. } => assert_eq!(literal, "let x = 1;\nlet y = 2;"),
            other => panic!("expected indented code block, got {other:?}"),
        }
    }

    #[test]
    fn thematic_break_variants() {
        for text in ["---\n", "***\n", "___\n", "- - -\n"] {
            let blocks = parse_markdown_blocks(text);
            assert_eq!(blocks.len(), 1, "input {text:?}");
            assert!(matches!(&blocks[0], MdBlock::ThematicBreak), "input {text:?}");
        }
    }

    #[test]
    fn block_quote_recursive() {
        let text = "> # Quoted heading\n> a paragraph\n";
        let blocks = parse_markdown_blocks(text);
        assert_eq!(blocks.len(), 1);
        match &blocks[0] {
            MdBlock::BlockQuote { blocks } => {
                assert_eq!(blocks.len(), 2);
                assert!(matches!(&blocks[0], MdBlock::Heading { level: 1, .. }));
                assert!(matches!(&blocks[1], MdBlock::Paragraph { .. }));
            }
            other => panic!("expected block quote, got {other:?}"),
        }
    }

    #[test]
    fn html_block_raw_retention() {
        let text = "<div class=\"note\">\nplain content\n</div>\n";
        let blocks = parse_markdown_blocks(text);
        assert_eq!(blocks.len(), 1);
        match &blocks[0] {
            MdBlock::HtmlBlock { raw } => assert!(raw.contains("<div") && raw.contains("</div>")),
            other => panic!("expected html block, got {other:?}"),
        }
    }

    #[test]
    fn unordered_and_ordered_lists() {
        let unordered = parse_markdown_blocks("- one\n- two\n- three\n");
        assert_eq!(unordered.len(), 1);
        match &unordered[0] {
            MdBlock::List { ordered, items, tight, .. } => {
                assert!(!ordered);
                assert!(tight);
                assert_eq!(items.len(), 3);
            }
            other => panic!("expected list, got {other:?}"),
        }

        let ordered = parse_markdown_blocks("1. first\n2. second\n");
        match &ordered[0] {
            MdBlock::List { ordered, start, items, .. } => {
                assert!(*ordered);
                assert_eq!(*start, Some(1));
                assert_eq!(items.len(), 2);
            }
            other => panic!("expected list, got {other:?}"),
        }
    }

    #[test]
    fn nested_list_and_loose_list() {
        let nested = parse_markdown_blocks("- outer\n  - inner a\n  - inner b\n- outer two\n");
        match &nested[0] {
            MdBlock::List { items, .. } => {
                assert_eq!(items.len(), 2);
                let has_nested = items[0].iter().any(|b| matches!(b, MdBlock::List { .. }));
                assert!(has_nested, "expected a nested list inside the first item, got {:?}", items[0]);
            }
            other => panic!("expected list, got {other:?}"),
        }

        let loose = parse_markdown_blocks("- one\n\n- two\n");
        match &loose[0] {
            MdBlock::List { tight, .. } => assert!(!tight, "blank line between items must make the list loose"),
            other => panic!("expected list, got {other:?}"),
        }
    }

    #[test]
    fn emphasis_strong_links_images_and_html_in_inline() {
        let inline = parse_inline(
            "plain **strong** and *em* and [a link](https://example.com \"title\") and ![alt](img.png) and <br/>",
        );
        assert!(inline.iter().any(|n| matches!(n, MdInline::Strong { inlines } if inlines == &vec![MdInline::Text { text: "strong".into() }])));
        assert!(inline.iter().any(|n| matches!(n, MdInline::Emphasis { inlines } if inlines == &vec![MdInline::Text { text: "em".into() }])));
        let link = inline.iter().find_map(|n| match n {
            MdInline::Link { text, url, title } => Some((text.clone(), url.clone(), title.clone())),
            _ => None,
        }).expect("link present");
        assert_eq!(link.0, vec![MdInline::Text { text: "a link".into() }]);
        assert_eq!(link.1, "https://example.com");
        assert_eq!(link.2.as_deref(), Some("title"));
        let image = inline.iter().find_map(|n| match n {
            MdInline::Image { alt, url, .. } => Some((alt.clone(), url.clone())),
            _ => None,
        }).expect("image present");
        assert_eq!(image, ("alt".into(), "img.png".into()));
        assert!(inline.iter().any(|n| matches!(n, MdInline::HtmlInline { raw } if raw == "<br/>")));
    }

    #[test]
    fn inline_code_span_is_not_emphasis() {
        let inline = parse_inline("use `*not emphasis*` here");
        assert!(inline.iter().any(|n| matches!(n, MdInline::Code { literal } if literal == "*not emphasis*")));
    }

    #[test]
    fn soft_and_hard_breaks_in_paragraph() {
        let blocks = parse_markdown_blocks("line one  \nline two\\\nline three\nline four\n");
        match &blocks[0] {
            MdBlock::Paragraph { inlines } => {
                let hard_count = inlines.iter().filter(|n| matches!(n, MdInline::HardBreak)).count();
                let soft_count = inlines.iter().filter(|n| matches!(n, MdInline::SoftBreak)).count();
                assert_eq!(hard_count, 2, "trailing-2-spaces and trailing-backslash both hard-break: {inlines:?}");
                assert_eq!(soft_count, 1, "bare line ending soft-breaks: {inlines:?}");
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn unrecognized_delimiter_degrades_to_plain_text() {
        // 🕳️ Exactly ONE `*` in the whole input -- genuinely unpairable (not to be confused with
        // an input containing a valid pair elsewhere, which correctly parses as emphasis).
        let inline = parse_inline("plain text with one lonely *delimiter and no partner at all");
        assert!(inline.iter().all(|n| matches!(n, MdInline::Text { .. })), "unmatched * must degrade to Text, got {inline:?}");
    }
    //#endregion 🔖️ParserUnitTests

    //#region 🔖️Fixtures
    fn sample_snapshot() -> MdSnapshot {
        MdSnapshot {
            schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
            blocks: vec![
                MdBlock::Heading { level: 1, inlines: vec![MdInline::Text { text: "Title".into() }] },
                MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "hello".into() }] },
            ],
        }
    }

    /// 🌱 `sweep_a`/`sweep_b`: differ in EVERY mutable field. Top-level `blocks` DIFFER IN LENGTH
    /// (a: 3, b: 2) -- required per the recipe's own documented structural limitation (naive
    /// positional `between`, `0..min(len)`, only one tail can be non-empty per call): the shared
    /// prefix (index 0 = `List`, index 1 = `CodeBlock`) is modified in every one of ITS fields
    /// (the `List`'s `items` sub-triple additionally shows removed+modified via the SAME
    /// different-length-prefix trick one level deeper: 2 items vs 1, index 0 shared+modified,
    /// index 1 dropped), while `a`'s trailing index-2 `Paragraph` is the top-level `removed` tail
    /// in `between(a, b)` and the top-level `added` tail in `between(b, a)` --
    /// `between_roundtrip_law`/`field_sweep` both check both directions, matching xml's F1
    /// precedent.
    fn sweep_a() -> MdSnapshot {
        MdSnapshot {
            schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
            blocks: vec![
                MdBlock::List {
                    ordered: false,
                    start: None,
                    tight: true,
                    items: vec![
                        vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "item keep+modify".into() }] }],
                        vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "item drop".into() }] }],
                    ],
                },
                MdBlock::CodeBlock { info: Some("rust".into()), literal: "fn a() {}".into() },
                MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "to remove".into() }] },
            ],
        }
    }

    fn sweep_b() -> MdSnapshot {
        MdSnapshot {
            schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
            blocks: vec![
                MdBlock::List {
                    ordered: true,
                    start: Some(3),
                    tight: false,
                    items: vec![vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "item keep+modify CHANGED".into() }] }]],
                },
                MdBlock::CodeBlock { info: None, literal: "fn b() {}".into() },
            ],
        }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️MutationDiffLaw
    fn sample_mutations() -> Vec<MdMutation> {
        vec![
            MdMutation::NoMutation,
            MdMutation::SetSnapshot { snapshot: sweep_b() },
            MdMutation::InsertBlock { path: vec![], index: 1, block: MdBlock::ThematicBreak },
            MdMutation::RemoveBlock { path: vec![], index: 0 },
            MdMutation::ReplaceBlock { path: vec![], index: 1, block: MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "replaced".into() }] } },
            MdMutation::SetInlines { path: vec![], index: 1, inlines: vec![MdInline::Text { text: "new inlines".into() }] },
        ]
    }

    #[test]
    fn mutation_diff_law() {
        for mutation in sample_mutations() {
            let base = sample_snapshot();
            let diff_direct = Mutation::diff(&mutation, &base);
            let applied_via_diff = MutationDiff::apply(&diff_direct, &base);

            let mut via_apply = base.clone();
            let diff_from_apply = crate::artifacts::md::schema::mutations::apply_md_mutation(&mut via_apply, &mutation);

            assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
            assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[test]
    fn inverse_law() {
        for mutation in sample_mutations() {
            let base = sample_snapshot();

            let mut round_tripped = base.clone();
            crate::artifacts::md::schema::mutations::apply_md_mutation(&mut round_tripped, &mutation);
            for inverse_mutation in <MdMutation as Mutation<MdSnapshot>>::inverse(&mutation, &base) {
                crate::artifacts::md::schema::mutations::apply_md_mutation(&mut round_tripped, &inverse_mutation);
            }
            assert_eq!(round_tripped, base, "inverse_law (mutation-level) failed for {mutation:?}");

            let diff = Mutation::diff(&mutation, &base);
            let next = MutationDiff::apply(&diff, &base);
            let inverse_diff = DiffAlgebra::inverse(&diff, &base);
            let restored = MutationDiff::apply(&inverse_diff, &next);
            assert_eq!(restored, base, "inverse_law (diff-level) failed for {mutation:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    fn two_para_root(a: &str, b: &str) -> MdSnapshot {
        MdSnapshot {
            schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
            blocks: vec![
                MdBlock::Paragraph { inlines: vec![MdInline::Text { text: a.into() }] },
                MdBlock::Paragraph { inlines: vec![MdInline::Text { text: b.into() }] },
            ],
        }
    }

    fn assert_absorb_matches_sequential(base: &MdSnapshot, d1: &MdDiff, d2: &MdDiff) -> MdDiff {
        let sequential = MutationDiff::apply(d2, &MutationDiff::apply(d1, base));
        let mut absorbed = d1.clone();
        MutationDiff::absorb(&mut absorbed, d2.clone());
        assert_eq!(MutationDiff::apply(&absorbed, base), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
        absorbed
    }

    fn root_blocks_diff(diff: &MdDiff) -> &MdBlocksDiff {
        diff.blocks.as_ref().expect("blocks diff present")
    }

    #[test]
    fn absorb_law() {
        // Canonical: Insert(2)+Remove(0) -> {removed:[0], added:[(1,f)]}.
        {
            let base = two_para_root("a", "b");
            let d1 = Mutation::diff(&MdMutation::InsertBlock { path: vec![], index: 2, block: MdBlock::ThematicBreak }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&MdMutation::RemoveBlock { path: vec![], index: 0 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = root_blocks_diff(&absorbed);
            assert_eq!(triple.removed, vec![0]);
            assert_eq!(triple.added.len(), 1);
            assert_eq!(triple.added[0].index, 1);
            assert!(matches!(triple.added[0].item, MdBlock::ThematicBreak));
        }

        // Canonical: Insert(2,f)+Insert(2,g) -> both survive.
        {
            let base = two_para_root("a", "b");
            let d1 = Mutation::diff(&MdMutation::InsertBlock { path: vec![], index: 2, block: MdBlock::ThematicBreak }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(
                &MdMutation::InsertBlock { path: vec![], index: 2, block: MdBlock::HtmlBlock { raw: "<hr/>".into() } },
                &mid,
            );
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = root_blocks_diff(&absorbed);
            assert_eq!(triple.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
        }

        // Canonical: Insert(1,f)+SetField(1,v) -> patch into the added payload.
        {
            let base = two_para_root("a", "b");
            let d1 = Mutation::diff(
                &MdMutation::InsertBlock { path: vec![], index: 1, block: MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "f".into() }] } },
                &base,
            );
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(
                &MdMutation::SetInlines { path: vec![], index: 1, inlines: vec![MdInline::Text { text: "v".into() }] },
                &mid,
            );
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = root_blocks_diff(&absorbed);
            assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
            assert_eq!(triple.added.len(), 1);
            match &triple.added[0].item {
                MdBlock::Paragraph { inlines } => assert_eq!(inlines, &vec![MdInline::Text { text: "v".into() }]),
                other => panic!("expected paragraph, got {other:?}"),
            }
        }

        // Canonical: Modify+Remove -> the modify is annihilated by the later remove.
        {
            let base = two_para_root("a", "b");
            let d1 = Mutation::diff(&MdMutation::SetInlines { path: vec![], index: 1, inlines: vec![MdInline::Text { text: "v".into() }] }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&MdMutation::RemoveBlock { path: vec![], index: 1 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = root_blocks_diff(&absorbed);
            assert!(triple.modified.is_empty(), "modify of a since-removed item must not survive absorb");
            assert_eq!(triple.removed, vec![1]);
        }

        // Associativity over a triple.
        {
            let base = two_para_root("a", "b");
            let d1 = Mutation::diff(&MdMutation::InsertBlock { path: vec![], index: 2, block: MdBlock::ThematicBreak }, &base);
            let mid1 = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&MdMutation::InsertBlock { path: vec![], index: 2, block: MdBlock::HtmlBlock { raw: "<hr/>".into() } }, &mid1);
            let mid2 = MutationDiff::apply(&d2, &mid1);
            let d3 = Mutation::diff(&MdMutation::RemoveBlock { path: vec![], index: 0 }, &mid2);
            let sequential = MutationDiff::apply(&d3, &mid2);

            let mut left = d1.clone();
            MutationDiff::absorb(&mut left, d2.clone());
            MutationDiff::absorb(&mut left, d3.clone());

            let mut d2_then_d3 = d2.clone();
            MutationDiff::absorb(&mut d2_then_d3, d3.clone());
            let mut right = d1.clone();
            MutationDiff::absorb(&mut right, d2_then_d3);

            assert_eq!(MutationDiff::apply(&left, &base), sequential, "absorb associativity (left) failed");
            assert_eq!(MutationDiff::apply(&right, &base), sequential, "absorb associativity (right) failed");
        }

        // Nested (BlockQuote) canonical: Insert-inside-quote + Remove-before-it-inside-quote.
        {
            let base = MdSnapshot {
                schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
                blocks: vec![MdBlock::BlockQuote {
                    blocks: vec![
                        MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "qa".into() }] },
                        MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "qb".into() }] },
                    ],
                }],
            };
            let path = [MdPathStep::BlockQuote { index: 0 }];
            let d1 = Mutation::diff(&MdMutation::InsertBlock { path: path.to_vec(), index: 2, block: MdBlock::ThematicBreak }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&MdMutation::RemoveBlock { path: path.to_vec(), index: 0 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let MdBlockDiff::BlockQuote { blocks: Some(inner) } = &absorbed.blocks.as_ref().unwrap().modified[0].diff else {
                panic!("expected nested block-quote diff");
            };
            assert_eq!(inner.removed, vec![0]);
            assert_eq!(inner.added.len(), 1);
        }
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[test]
    fn between_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(MutationDiff::apply(&<MdDiff as DiffAlgebra<MdSnapshot>>::between(&a, &b), &a), b);
        assert_eq!(MutationDiff::apply(&<MdDiff as DiffAlgebra<MdSnapshot>>::between(&b, &a), &b), a);

        let sample = sample_snapshot();
        assert_eq!(MutationDiff::apply(&<MdDiff as DiffAlgebra<MdSnapshot>>::between(&sample, &sample), &sample), sample);

        // Real fixture (the demo's `example.md`) diffed against a mutated variant.
        let fixture_text = include_str!("../../../📚️examples/🎬️demo/🖼️assets/example.md");
        let fixture_blocks = parse_markdown_blocks(fixture_text);
        let fixture = MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks: fixture_blocks };
        let mut mutated = fixture.clone();
        crate::artifacts::md::schema::mutations::apply_md_mutation(
            &mut mutated,
            &MdMutation::InsertBlock { path: vec![], index: 0, block: MdBlock::ThematicBreak },
        );
        assert_ne!(fixture, mutated);
        assert_eq!(MutationDiff::apply(&<MdDiff as DiffAlgebra<MdSnapshot>>::between(&fixture, &mutated), &fixture), mutated);
        assert_eq!(MutationDiff::apply(&<MdDiff as DiffAlgebra<MdSnapshot>>::between(&mutated, &fixture), &mutated), fixture);
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️CodecRetentionLaw
    #[test]
    fn codec_retention_law() {
        // Documented normal form (see `render_markdown_blocks`'s doc comment): semantic fixed
        // point at the SNAPSHOT level, not byte-identical text. Fixture is written to already be
        // a fixed point of this codec's own parse/render pair (avoids incidental normalizations --
        // e.g. indented-vs-fenced code -- that would make a byte-diff assertion meaningless here).
        let fixture_text = include_str!("../../../📚️examples/🎬️demo/🖼️assets/example.md");
        let blocks = parse_markdown_blocks(fixture_text);
        let re_encoded_text = render_markdown_blocks(&blocks);
        let re_parsed = parse_markdown_blocks(&re_encoded_text);
        assert_eq!(re_parsed, blocks, "decode(encode(x)) must equal x at the snapshot level");

        let snap = MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks };
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <MdSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
    //#endregion 🔖️CodecRetentionLaw

    //#region 🔖️FieldSweep
    /// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable field (see the
    /// fixtures' doc comment for exactly how each collection flavor is exercised).
    #[test]
    fn field_sweep_covers_every_mutable_field() {
        let a = sweep_a();
        let b = sweep_b();

        let diff_ab = <MdDiff as DiffAlgebra<MdSnapshot>>::between(&a, &b);
        assert_eq!(MutationDiff::apply(&diff_ab, &a), b);
        let diff_ba = <MdDiff as DiffAlgebra<MdSnapshot>>::between(&b, &a);
        assert_eq!(MutationDiff::apply(&diff_ba, &b), a);
        assert!(<MdDiff as DiffAlgebra<MdSnapshot>>::between(&a, &a).is_empty());

        // Direction a->b: top-level `removed` (a's trailing paragraph, beyond b's length) +
        // `modified` (the shared-prefix `List` in every one of its own fields, AND the shared
        // `CodeBlock`) are exercised.
        let blocks_ab = diff_ab.blocks.as_ref().expect("blocks diff present (a->b)");
        assert!(!blocks_ab.removed.is_empty(), "top-level: removed not exercised (a->b)");
        assert_eq!(blocks_ab.modified.len(), 2, "expected the List AND the CodeBlock entries modified");
        let list_entry = blocks_ab.modified.iter().find(|m| matches!(m.diff, MdBlockDiff::List { .. }))
            .expect("a List-shaped modified entry must be present");
        let MdBlockDiff::List { ordered, start, tight, items } = &list_entry.diff else { unreachable!() };
        assert!(ordered.is_some(), "List.ordered not exercised");
        assert_eq!(*start, Some(Some(3)), "List.start tri-state (None -> Some(3)) not exercised");
        assert!(tight.is_some(), "List.tight not exercised");
        let items_diff: &MdListItemsDiff = items.as_ref().expect("List.items diff present");
        assert!(!items_diff.removed.is_empty(), "List.items: removed not exercised");
        assert_eq!(items_diff.modified.len(), 1, "expected exactly one modified item");
        assert!(!items_diff.modified[0].diff.modified.is_empty(), "modified item's own content not exercised");
        let code_entry = blocks_ab.modified.iter().find(|m| matches!(m.diff, MdBlockDiff::CodeBlock { .. }))
            .expect("a CodeBlock-shaped modified entry must be present");
        let MdBlockDiff::CodeBlock { info, literal } = &code_entry.diff else { unreachable!() };
        assert_eq!(*info, Some(None), "CodeBlock.info tri-state (Some -> None) not exercised");
        assert!(literal.is_some(), "CodeBlock.literal not exercised");

        // Direction b->a: top-level `added` (a's trailing paragraph reappearing) is exercised.
        let blocks_ba = diff_ba.blocks.as_ref().expect("blocks diff present (b->a)");
        assert!(!blocks_ba.added.is_empty(), "top-level: added not exercised (b->a)");

        // Sanity: nested list-item content diff and top-level block-kind Replace both exist as
        // reachable shapes (exercised directly, not just via sweep, since the naive between()
        // can't surface every shape from one pair -- same rationale as xml's F1 precedent).
        let leaf = diff_at_path(
            &[],
            0,
            crate::artifacts::md::schema::diff::MdBlocksLeafDiff::Modified(MdBlockDiff::Replace { block: MdBlock::ThematicBreak }),
        );
        assert!(leaf.blocks.is_some());
        let nested = MdListItemsDiff {
            removed: vec![0],
            modified: vec![MdListItemModified { index: 1, diff: MdBlocksDiff { removed: vec![], modified: vec![], added: vec![MdBlockAdded { index: 0, item: MdBlock::ThematicBreak }] } }],
            added: vec![MdListItemAdded { index: 2, item: vec![MdBlock::ThematicBreak] }],
        };
        assert!(!nested.removed.is_empty() && !nested.modified.is_empty() && !nested.added.is_empty());
        let _ = MdBlockModified { index: 0, diff: MdBlockDiff::ThematicBreak };
    }
    //#endregion 🔖️FieldSweep
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, composer_entry_of};
    use crate::artifacts::md::standards::v_commonmark::subsets::any::schema::MdComposer as MdRawAnyComposer;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<MdRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
