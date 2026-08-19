//! 🧩️ CommonMark (md) import — a real, from-scratch block/inline parser covering
//! https://spec.commonmark.org/'s common real-world subset. `MdSnapshot.blocks` is the complete
//! typed persisted source of truth (see the snapshot module's own doc comment for the exact
//! honest-subset scope and the deviations it lists); `parse_markdown_blocks` is the codec's read
//! half, independently testable.

use crate::artifacts::md::schema::snapshot::{MdBlock, MdInline};

//#region 🔖️BlockLineClassifiers
/// 📏️ Leading-space count (ASCII spaces only, matching every other classifier here -- a line
/// indented with unicode whitespace is out of scope, same pre-existing limitation as before this
/// wave).
async fn leading_spaces(line: &str) -> usize {
    line.chars().take_while(|&c| c == ' ').count()
}

async fn fence_open(line: &str) -> Option<(char, usize, String)> {
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

async fn atx_heading(line: &str) -> Option<(u8, &str)> {
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

async fn indented_code_line(line: &str) -> Option<&str> {
    line.strip_prefix("    ").or_else(|| line.strip_prefix('\t'))
}

/// 🔳 `---`, `***`, `___` (>=3 of the same char, optionally space-separated, nothing else) --
/// checked BEFORE `list_item_marker` since `- - -` is a thematic break, not a 3-item list.
async fn thematic_break(line: &str) -> bool {
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
async fn list_item_marker(line: &str) -> Option<(bool, Option<u32>, usize, &str)> {
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
async fn blockquote_marker(line: &str) -> Option<&str> {
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
async fn html_block_start(line: &str) -> bool {
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
async fn dedent_by_chars(line: &str, width: usize) -> Option<String> {
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
            if line.chars().all(|c| c == ' ') {
                Some(String::new())
            } else {
                None
            }
        }
    }
}
//#endregion 🔖️BlockLineClassifiers

//#region 🔖️BlockParser
/// 📥 Top-level entry: parses `text` into the complete block sequence.
pub async fn parse_markdown_blocks(text: &str) -> Vec<MdBlock> {
    let lines: Vec<&str> = text.lines().collect();
    parse_blocks(&lines)
}

/// 📥 Recursive block parser over an already-dedented line slice -- called at the top level and
/// recursively for block-quote content and list-item content, which is exactly how nested lists
/// and quote-inside-list (and vice versa) fall out for free.
async fn parse_blocks(lines: &[&str]) -> Vec<MdBlock> {
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
            blocks.push(MdBlock::CodeBlock { info: if info.is_empty() { None } else { Some(info) }, literal: code_lines.join("\n") });
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
            if l.trim().is_empty() || fence_open(l).is_some() || indented_code_line(l).is_some() || atx_heading(l).is_some() || list_item_marker(l).is_some() || blockquote_marker(l).is_some() || thematic_break(l) || html_block_start(l) {
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
async fn parse_list(lines: &[&str], ordered: bool, start: Option<u32>) -> (MdBlock, usize) {
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
async fn try_parse_delim(chars: &[char], start: usize, delim: char, count: usize) -> Option<(String, usize)> {
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

async fn try_parse_code_span(chars: &[char], start: usize) -> Option<(String, usize)> {
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

async fn split_url_title(inside: &str) -> (String, Option<String>) {
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
async fn try_parse_bracket_paren(chars: &[char], start: usize) -> Option<((usize, usize), String, Option<String>, usize)> {
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

async fn try_parse_link(chars: &[char], start: usize) -> Option<(MdInline, usize)> {
    let ((text_start, text_end), url, title, consumed) = try_parse_bracket_paren(chars, start)?;
    let text: String = chars[text_start..text_end].iter().collect();
    Some((MdInline::Link { text: parse_inline(&text), url, title }, consumed))
}

async fn try_parse_image(chars: &[char], start: usize) -> Option<(MdInline, usize)> {
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
async fn try_parse_html_inline(chars: &[char], start: usize) -> Option<(String, usize)> {
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
pub async fn parse_inline(text: &str) -> Vec<MdInline> {
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
async fn parse_inline_lines(lines: &[&str]) -> Vec<MdInline> {
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
