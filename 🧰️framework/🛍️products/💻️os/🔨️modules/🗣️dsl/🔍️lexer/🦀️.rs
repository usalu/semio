//! 🔍 The shared DSL lexer and bare-ident predicate.

use crate::os_dsl::diagnostic::{Limits, TextError, TextSpan};
use crate::os_dsl::token::*;
use crate::os_dsl::trust::Sanitized;

//#region 🔖️Dialect
/// @emoji 🎛️ How a quoted-string region decodes its escapes once the lexer commits to using
/// `quote` as its delimiter — the P2-M1 generalized string/text mechanism serving json's
/// `\uXXXX`, csv's `""`, and step's `''`-doubling uniformly instead of four bespoke fixes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StringEscape {
    /// `\` + any char is copied verbatim into the token text (both chars kept, undecoded) — the
    /// lexer's only behavior before P2-M1, kept as the default so every grammar that doesn't
    /// declare a `string` header directive lexes byte-identically to before this type existed.
    Raw,
    /// Standard backslash decode: `\" \\ \/ \b \f \n \r \t` plus `\uXXXX` (4 hex digits, with
    /// UTF-16 surrogate-pair combination for astral codepoints) — JSON's escape grammar
    /// (RFC 8259 §7). An unrecognized escape is a strict-mode error / kept literal when forgiving.
    Backslash,
    /// The delimiter doubled (`""` / `''`) decodes to one literal delimiter char inside the
    /// string; `\` has no special meaning. CSV's RFC 4180 quoted-field escape / STEP Part 21's
    /// `''`-doubled strings.
    Doubled,
}

/// @emoji 🔤️ One configured quote delimiter + the escape scheme active while scanning it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StringMode {
    pub quote: char,
    pub escape: StringEscape,
}

/// @emoji 💬️ Per-grammar comment syntax — `#`-to-EOL is the shared default; a grammar dialect
/// header can swap/disable the line marker or add a block form. Exists because STEP/IFC's `#` is
/// the entity-reference sigil (`#123=...`), not a comment — it directly collides with the shared
/// lexer's old hardcoded-global `#`-comment rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommentDialect {
    pub line: Option<String>,
    pub block: Option<(String, String)>,
}

impl Default for CommentDialect {
    fn default() -> Self {
        Self { line: Some("#".to_string()), block: None }
    }
}

/// @emoji 🧬️ Full per-grammar lexer configuration. `Default` reproduces the fixed alphabet exactly
/// as it existed before P2-M1 (single `"`-delimited `Raw` string, `#`-to-EOL comment, no block
/// comment) — every grammar that doesn't declare `string`/`comment` header directives is
/// unaffected, byte-for-byte, by this type's existence (the plan's "extension-only" gate).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct LexOptions {
    pub strings: Vec<StringMode>,
    pub comment: CommentDialect,
}

impl LexOptions {
    fn string_mode(&self, c: char) -> Option<StringMode> {
        if self.strings.is_empty() {
            return (c == '"').then_some(StringMode { quote: '"', escape: StringEscape::Raw });
        }
        self.strings.iter().copied().find(|m| m.quote == c)
    }
}

/// @emoji 🔓️ Decodes one backslash escape unit starting at `chars[j] == '\\'` under
/// [`StringEscape::Backslash`] — JSON's scheme (RFC 8259 §7) incl. `\uXXXX` surrogate-pair
/// combination. Returns the decoded text plus the index just past the consumed escape, or `None`
/// if `chars[j..]` isn't a recognized escape (caller decides raw-fallback vs. strict error).
fn decode_backslash_unit(chars: &[char], j: usize) -> Option<(String, usize)> {
    let next = *chars.get(j + 1)?;
    match next {
        '"' => Some(("\"".to_string(), j + 2)),
        '\'' => Some(("'".to_string(), j + 2)),
        '\\' => Some(("\\".to_string(), j + 2)),
        '/' => Some(("/".to_string(), j + 2)),
        'b' => Some(("\u{8}".to_string(), j + 2)),
        'f' => Some(("\u{c}".to_string(), j + 2)),
        'n' => Some(("\n".to_string(), j + 2)),
        'r' => Some(("\r".to_string(), j + 2)),
        't' => Some(("\t".to_string(), j + 2)),
        'u' => {
            let (hi, after_hi) = read_hex4(chars, j + 2)?;
            if (0xD800..=0xDBFF).contains(&hi) {
                if chars.get(after_hi) == Some(&'\\') && chars.get(after_hi + 1) == Some(&'u') {
                    if let Some((lo, after_lo)) = read_hex4(chars, after_hi + 2) {
                        if (0xDC00..=0xDFFF).contains(&lo) {
                            let scalar = 0x10000u32 + ((hi - 0xD800) << 10) + (lo - 0xDC00);
                            if let Some(ch) = char::from_u32(scalar) {
                                return Some((ch.to_string(), after_lo));
                            }
                        }
                    }
                }
                Some((char::REPLACEMENT_CHARACTER.to_string(), after_hi))
            } else if (0xDC00..=0xDFFF).contains(&hi) {
                Some((char::REPLACEMENT_CHARACTER.to_string(), after_hi))
            } else {
                char::from_u32(hi).map(|ch| (ch.to_string(), after_hi))
            }
        }
        _ => None,
    }
}

fn read_hex4(chars: &[char], start: usize) -> Option<(u32, usize)> {
    if start + 4 > chars.len() {
        return None;
    }
    let text: String = chars[start..start + 4].iter().collect();
    u32::from_str_radix(&text, 16).ok().map(|v| (v, start + 4))
}

/// @emoji 🔎️ True iff `chars[i..]` begins with `needle`'s chars — the multi-char-marker match
/// used by both configurable comment forms (line marker, block open/close).
fn chars_start_with(chars: &[char], i: usize, needle: &str) -> bool {
    let needle_chars: Vec<char> = needle.chars().collect();
    if i + needle_chars.len() > chars.len() {
        return false;
    }
    chars[i..i + needle_chars.len()] == needle_chars[..]
}
//#endregion 🔖️Dialect

//#region 🔖️Lexer
// 🚫️async: E1 pure char classifier consumed by `Option::is_some_and`/`Iterator::filter` sync closures (`:475`, `:614`) — see R9
fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

// 🚫️async: E1 pure char classifier consumed by `Option::is_some_and` sync closure (`:475`) — see R9
fn is_ident_continue(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '-' | '.' | '/')
}

/// @emoji ➡️ Fused edge arrow `-id:Kind>` or `-id-` (not `->` / `--`).
fn lex_fused_edge_arrow(chars: &[char], i: usize) -> Option<(usize, String)> {
    if chars.get(i) != Some(&'-') {
        return None;
    }
    let mut j = i + 1;
    if j >= chars.len() {
        return None;
    }
    if chars[j] == ':' {
        j += 1;
        if j >= chars.len() || !is_ident_start(chars[j]) {
            return None;
        }
        while j < chars.len() && is_ident_continue(chars[j]) && chars[j] != '-' {
            j += 1;
        }
    } else if is_ident_start(chars[j]) {
        while j < chars.len() && is_ident_continue(chars[j]) && chars[j] != '-' {
            j += 1;
        }
        if j < chars.len() && chars[j] == ':' {
            j += 1;
            if j >= chars.len() || !is_ident_start(chars[j]) {
                return None;
            }
            while j < chars.len() && is_ident_continue(chars[j]) && chars[j] != '-' {
                j += 1;
            }
        }
    } else {
        return None;
    }
    if j < chars.len() && chars[j] == '>' {
        j += 1;
        return Some((j, chars[i..j].iter().collect()));
    }
    if j < chars.len() && chars[j] == '-' && chars.get(j + 1) != Some(&'-') {
        j += 1;
        return Some((j, chars[i..j].iter().collect()));
    }
    None
}

/// @emoji 🔬️ Grammar-independent lexer for the fixed token alphabet shared by every DSL grammar
/// declared on this engine. `forgiving = true` never fails (malformed regions become `Error`
/// tokens instead), which is what editor/completion mode needs; `forgiving = false` is strict
/// parse mode and returns the first lexical error. The fixed-alphabet default entry point — every
/// pre-P2-M1 caller keeps this exact 3-arg signature; equivalent to
/// `lex_with(text, limits, forgiving, &LexOptions::default())`.
pub fn lex(text: &str, limits: &Limits, forgiving: bool) -> Result<Vec<SpannedToken>, TextError> {
    lex_with(text, limits, forgiving, &LexOptions::default())
}

/// @emoji 🎛️ P2-M1: same lexer, parameterized by a per-grammar [`LexOptions`] (string quote+escape
/// modes, comment syntax). `lex` is the fixed-default entry point every pre-M1 caller still uses.
pub fn lex_with(text: &str, limits: &Limits, forgiving: bool, opts: &LexOptions) -> Result<Vec<SpannedToken>, TextError> {
    limits.check_bytes(text.len())?;
    let chars: Vec<char> = text.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0usize;
    let mut line = 1u32;
    let mut column = 1u32;
    let mut byte_offset = 0u32;
    let mut next_id = 0u32;

    macro_rules! push {
        ($kind:expr, $start_line:expr, $start_col:expr, $start_byte:expr, $text:expr) => {{
            let text_str: String = $text;
            let len = text_str.chars().count() as u32;
            tokens.push(SpannedToken { id: TokenId(next_id), kind: $kind, text: Symbol::intern(&text_str), span: TextSpan::with_length($start_line, $start_col, len), byte_range: ($start_byte, byte_offset) });
            next_id += 1;
        }};
    }

    while i < chars.len() {
        limits.check_tokens(tokens.len(), TextSpan::at(line, column))?;
        let c = chars[i];
        let start_line = line;
        let start_col = column;
        let start_byte = byte_offset;

        if c == '\n' {
            i += 1;
            byte_offset += c.len_utf8() as u32;
            push!(TokenKind::Newline, start_line, start_col, start_byte, "\n".to_string());
            line += 1;
            column = 1;
            continue;
        }
        if c.is_whitespace() {
            let mut j = i;
            let mut buf = String::new();
            while j < chars.len() && chars[j].is_whitespace() && chars[j] != '\n' {
                buf.push(chars[j]);
                byte_offset += chars[j].len_utf8() as u32;
                column += 1;
                j += 1;
            }
            i = j;
            push!(TokenKind::Whitespace, start_line, start_col, start_byte, buf);
            continue;
        }
        // Line comment: marker is a configurable string (default "#"), `None` disables it
        // entirely — STEP/IFC declare `comment line none` because `#` is their entity-ref sigil.
        if let Some(marker) = opts.comment.line.as_deref() {
            if chars_start_with(&chars, i, marker) {
                let mut j = i;
                let mut buf = String::new();
                while j < chars.len() && chars[j] != '\n' {
                    buf.push(chars[j]);
                    byte_offset += chars[j].len_utf8() as u32;
                    column += 1;
                    j += 1;
                }
                i = j;
                push!(TokenKind::Comment, start_line, start_col, start_byte, buf);
                continue;
            }
        }
        // Block comment: configurable `(open, close)` pair, disabled by default. May span lines —
        // line/column tracked through the body like the Fence token below. An unterminated block
        // comment falls through unconsumed in forgiving mode (its open-marker chars — e.g. `/*`'s
        // `/` and `*` — are each already ordinary single-char tokens) instead of looping forever.
        if let Some((open, close)) = &opts.comment.block {
            if chars_start_with(&chars, i, open) {
                let open_len = open.chars().count();
                let mut k = i + open_len;
                let mut found_end: Option<usize> = None;
                while k < chars.len() {
                    if chars_start_with(&chars, k, close) {
                        found_end = Some(k + close.chars().count());
                        break;
                    }
                    k += 1;
                }
                if let Some(end) = found_end {
                    let buf: String = chars[i..end].iter().collect();
                    for idx in i..end {
                        if chars[idx] == '\n' {
                            line += 1;
                            column = 1;
                        } else {
                            byte_offset += chars[idx].len_utf8() as u32;
                            column += 1;
                        }
                    }
                    i = end;
                    push!(TokenKind::Comment, start_line, start_col, start_byte, buf);
                    continue;
                }
                if !forgiving {
                    return Err(TextError::new("unterminated block comment (no closing marker)", TextSpan::at(start_line, start_col)));
                }
                // forgiving: fall through — the open marker's own chars lex as ordinary tokens.
            }
        }
        if let Some(mode) = opts.string_mode(c) {
            let quote = mode.quote;
            let mut j = i + 1;
            let mut buf = String::new();
            let mut closed = false;
            byte_offset += c.len_utf8() as u32;
            column += 1;
            match mode.escape {
                StringEscape::Raw => {
                    while j < chars.len() {
                        let cj = chars[j];
                        if cj == '\\' && j + 1 < chars.len() {
                            buf.push(cj);
                            buf.push(chars[j + 1]);
                            byte_offset += cj.len_utf8() as u32 + chars[j + 1].len_utf8() as u32;
                            column += 2;
                            j += 2;
                            continue;
                        }
                        if cj == quote {
                            byte_offset += cj.len_utf8() as u32;
                            column += 1;
                            j += 1;
                            closed = true;
                            break;
                        }
                        if cj == '\n' {
                            if forgiving {
                                break;
                            }
                            return Err(TextError::new("unterminated string literal (newline before closing quote)", TextSpan::at(start_line, start_col)));
                        }
                        buf.push(cj);
                        byte_offset += cj.len_utf8() as u32;
                        column += 1;
                        j += 1;
                    }
                }
                StringEscape::Backslash => {
                    while j < chars.len() {
                        let cj = chars[j];
                        if cj == quote {
                            byte_offset += cj.len_utf8() as u32;
                            column += 1;
                            j += 1;
                            closed = true;
                            break;
                        }
                        if cj == '\n' {
                            if forgiving {
                                break;
                            }
                            return Err(TextError::new("unterminated string literal (newline before closing quote)", TextSpan::at(start_line, start_col)));
                        }
                        if cj == '\\' {
                            match decode_backslash_unit(&chars, j) {
                                Some((decoded, next_j)) => {
                                    buf.push_str(&decoded);
                                    for k in j..next_j {
                                        byte_offset += chars[k].len_utf8() as u32;
                                        column += 1;
                                    }
                                    j = next_j;
                                }
                                None if forgiving || j + 1 >= chars.len() => {
                                    buf.push('\\');
                                    byte_offset += cj.len_utf8() as u32;
                                    column += 1;
                                    j += 1;
                                }
                                None => {
                                    return Err(TextError::new("unknown backslash escape in string literal", TextSpan::at(start_line, start_col)));
                                }
                            }
                            continue;
                        }
                        buf.push(cj);
                        byte_offset += cj.len_utf8() as u32;
                        column += 1;
                        j += 1;
                    }
                }
                StringEscape::Doubled => {
                    while j < chars.len() {
                        let cj = chars[j];
                        if cj == quote {
                            if chars.get(j + 1) == Some(&quote) {
                                buf.push(quote);
                                byte_offset += quote.len_utf8() as u32 * 2;
                                column += 2;
                                j += 2;
                                continue;
                            }
                            byte_offset += cj.len_utf8() as u32;
                            column += 1;
                            j += 1;
                            closed = true;
                            break;
                        }
                        if cj == '\n' {
                            if forgiving {
                                break;
                            }
                            return Err(TextError::new("unterminated string literal (newline before closing quote)", TextSpan::at(start_line, start_col)));
                        }
                        buf.push(cj);
                        byte_offset += cj.len_utf8() as u32;
                        column += 1;
                        j += 1;
                    }
                }
            }
            i = j;
            if !closed && !forgiving {
                return Err(TextError::new("unterminated string literal", TextSpan::at(start_line, start_col)));
            }
            push!(if closed { TokenKind::Text } else { TokenKind::Error }, start_line, start_col, start_byte, buf);
            continue;
        }
        // Fenced block: ```lang\ncontent\n``` — the ONE place this lexer scans raw multi-line
        // content instead of token-by-token (`Shape::Embed`'s only consumer). The lang tag is
        // everything up to the first newline (may be empty); content is everything up to a line
        // that is EXACTLY three backticks (no leading/trailing whitespace on that line — a fence
        // can never be indented, matching every other "structural" line in this grammar). Encoded
        // as one token, text = "lang\u{0}content" (NUL can't occur in valid authored text, so it's
        // a safe separator without adding a field to `SpannedToken`) — `dsl_schema`'s `Shape::Embed`
        // splits on it, the same trick `Shape::Dim` uses for its glued `x`-separated components.
        if c == '`' && chars.get(i + 1) == Some(&'`') && chars.get(i + 2) == Some(&'`') {
            let mut j = i + 3;
            while j < chars.len() && chars[j] != '\n' {
                j += 1;
            }
            let lang: String = chars[i + 3..j].iter().collect();
            let mut closing: Option<(usize, usize)> = None;
            if j < chars.len() {
                let content_start = j + 1;
                let mut k = content_start;
                loop {
                    let line_start = k;
                    let mut line_end = k;
                    while line_end < chars.len() && chars[line_end] != '\n' {
                        line_end += 1;
                    }
                    if chars[line_start..line_end].iter().collect::<String>() == "```" {
                        let content_end = if line_start > content_start { line_start - 1 } else { line_start };
                        let resume = if line_end < chars.len() { line_end + 1 } else { line_end };
                        closing = Some((content_end, resume));
                        break;
                    }
                    if line_end >= chars.len() {
                        break;
                    }
                    k = line_end + 1;
                }
            }
            if let Some((content_end, resume)) = closing {
                let content_start = j + 1;
                let content: String = chars[content_start..content_end.max(content_start)].iter().collect();
                for idx in i..resume {
                    if chars[idx] == '\n' {
                        line += 1;
                        column = 1;
                    } else {
                        byte_offset += chars[idx].len_utf8() as u32;
                        column += 1;
                    }
                }
                i = resume;
                push!(TokenKind::Fence, start_line, start_col, start_byte, format!("{lang}\u{0}{content}"));
                continue;
            }
            if !forgiving {
                return Err(TextError::new("unterminated fenced block (no closing '```' line)", TextSpan::at(start_line, start_col)));
            }
            // forgiving mode: fall through to "unknown character" for the opening backtick, then
            // the lexer naturally re-tries from the next character on its next loop iteration.
        }
        // `-inf` is its own special float literal (the negative-infinity half of the "nan/inf/-inf"
        // ident convention `format_f64`/`parse_f64` round-trip) — unlike ordinary numbers, `-` isn't
        // followed by a digit here, and `-` isn't a valid ident-start character either, so without
        // this it falls through every branch below to "unknown character".
        if c == '-' && i + 4 <= chars.len() && chars[i + 1] == 'i' && chars[i + 2] == 'n' && chars[i + 3] == 'f' && !chars.get(i + 4).is_some_and(|next| is_ident_continue(*next)) {
            i += 4;
            byte_offset += 4;
            column += 4;
            push!(TokenKind::Float, start_line, start_col, start_byte, "-inf".to_string());
            continue;
        }
        if c.is_ascii_digit() || (c == '-' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit()) {
            let mut j = i;
            let mut buf = String::new();
            let mut is_float = false;
            if chars[j] == '-' {
                buf.push('-');
                byte_offset += 1;
                column += 1;
                j += 1;
            }
            while j < chars.len() && chars[j].is_ascii_digit() {
                buf.push(chars[j]);
                byte_offset += 1;
                column += 1;
                j += 1;
            }
            // Trailing-dot floats (P2-M1): `0.` / `10.` — STEP Part 21's `read_number` accepts a
            // dot with no following digit. The one case that must NOT be swallowed here is a `.`
            // immediately followed by another `.` (the Range literal `0..10`) — anything else
            // after the dot (digit, letter, whitespace, EOF) commits to a trailing-dot float.
            if j < chars.len() && chars[j] == '.' && chars.get(j + 1) != Some(&'.') {
                is_float = true;
                buf.push('.');
                byte_offset += 1;
                column += 1;
                j += 1;
                while j < chars.len() && chars[j].is_ascii_digit() {
                    buf.push(chars[j]);
                    byte_offset += 1;
                    column += 1;
                    j += 1;
                }
            }
            if j < chars.len() && (chars[j] == 'e' || chars[j] == 'E') {
                let mut k = j + 1;
                if k < chars.len() && (chars[k] == '+' || chars[k] == '-') {
                    k += 1;
                }
                if k < chars.len() && chars[k].is_ascii_digit() {
                    is_float = true;
                    while j < k {
                        buf.push(chars[j]);
                        byte_offset += 1;
                        column += 1;
                        j += 1;
                    }
                    while j < chars.len() && chars[j].is_ascii_digit() {
                        buf.push(chars[j]);
                        byte_offset += 1;
                        column += 1;
                        j += 1;
                    }
                }
            }
            i = j;
            push!(if is_float { TokenKind::Float } else { TokenKind::Int }, start_line, start_col, start_byte, buf);
            continue;
        }
        if is_ident_start(c) {
            let mut j = i;
            let mut buf = String::new();
            while j < chars.len() && is_ident_continue(chars[j]) {
                // A '-' that starts an `->`/`--` operator terminates the ident here instead of
                // being swallowed into it, so kebab-case idents ("hexagonal-mushroom-column")
                // and the Arrow/DashArrow operators coexist without ambiguity.
                if chars[j] == '-' && j + 1 < chars.len() && matches!(chars[j + 1], '>' | '-') {
                    break;
                }
                buf.push(chars[j]);
                byte_offset += chars[j].len_utf8() as u32;
                column += 1;
                j += 1;
            }
            i = j;
            // A lone `_` is the placeholder sigil (positional "absent" marker), never an ident —
            // `_foo`/`foo_bar` still lex as ordinary idents since the buffer differs from "_".
            let kind = if buf == "_" { TokenKind::Placeholder } else { TokenKind::Ident };
            push!(kind, start_line, start_col, start_byte, buf);
            continue;
        }
        if c == '<' && i + 1 < chars.len() && chars[i + 1] == '-' {
            i += 2;
            byte_offset += 2;
            column += 2;
            push!(TokenKind::BackArrow, start_line, start_col, start_byte, "<-".to_string());
            continue;
        }
        if c == '-' && i + 1 < chars.len() && chars[i + 1] == '>' {
            i += 2;
            byte_offset += 2;
            column += 2;
            push!(TokenKind::Arrow, start_line, start_col, start_byte, "->".to_string());
            continue;
        }
        if c == '-' && i + 1 < chars.len() && chars[i + 1] == '-' {
            i += 2;
            byte_offset += 2;
            column += 2;
            push!(TokenKind::DashArrow, start_line, start_col, start_byte, "--".to_string());
            continue;
        }
        if c == '-' {
            if let Some((end_j, fused_text)) = lex_fused_edge_arrow(&chars, i) {
                let _len = end_j - i;
                for k in i..end_j {
                    byte_offset += chars[k].len_utf8() as u32;
                    column += 1;
                }
                i = end_j;
                push!(TokenKind::EdgeArrow, start_line, start_col, start_byte, fused_text);
                continue;
            }
            i += 1;
            byte_offset += 1;
            column += 1;
            push!(TokenKind::Minus, start_line, start_col, start_byte, "-".to_string());
            continue;
        }
        // `..` (Range literal `lo..hi`) must be checked before the single-char table below since
        // a lone `.` isn't `is_ident_start` and would otherwise fall through to "unknown character".
        if c == '.' && i + 1 < chars.len() && chars[i + 1] == '.' {
            i += 2;
            byte_offset += 2;
            column += 2;
            push!(TokenKind::DotDot, start_line, start_col, start_byte, "..".to_string());
            continue;
        }
        // Leading-dot enum literal (P2-M1): STEP Part 21's `.T.` / `.UNSPECIFIED.` — a dot, an
        // ident-shaped run, a closing dot, captured as one `DotEnum` token (text keeps both dots).
        // Checked after `..` (so a bare `..` still wins) and only commits when the closing dot is
        // actually found; otherwise falls through untouched, leaving today's behavior (a lone `.`
        // becomes "unknown character") unchanged for every format that never used this shape.
        if c == '.' && chars.get(i + 1).is_some_and(|next| is_ident_start(*next)) {
            let mut j = i + 1;
            // Deliberately narrower than `is_ident_continue` (which allows '.'/'-'/'/' so ordinary
            // idents like "a..b" or "hexagonal-mushroom-column" stay one token) — a dot-enum body
            // must stop AT its own closing dot rather than swallowing it as if it continued.
            while j < chars.len() && (chars[j].is_alphanumeric() || chars[j] == '_') {
                j += 1;
            }
            if chars.get(j) == Some(&'.') {
                let end = j + 1;
                let text: String = chars[i..end].iter().collect();
                for k in i..end {
                    byte_offset += chars[k].len_utf8() as u32;
                    column += 1;
                }
                i = end;
                push!(TokenKind::DotEnum, start_line, start_col, start_byte, text);
                continue;
            }
        }
        let single = match c {
            '=' => Some(TokenKind::Equals),
            ',' => Some(TokenKind::Comma),
            ':' => Some(TokenKind::Colon),
            '@' => Some(TokenKind::At),
            '^' => Some(TokenKind::Caret),
            '+' => Some(TokenKind::Plus),
            '*' => Some(TokenKind::Star),
            '/' => Some(TokenKind::Slash),
            '{' => Some(TokenKind::LBrace),
            '}' => Some(TokenKind::RBrace),
            '[' => Some(TokenKind::LBracket),
            ']' => Some(TokenKind::RBracket),
            '(' => Some(TokenKind::LParen),
            ')' => Some(TokenKind::RParen),
            // Promoted single-char tokens (P2-M1 item 3). `<` only reaches here when it wasn't the
            // start of `<-` (checked earlier and `continue`s on match), so no collision with
            // BackArrow; `>` is only ever swallowed by `lex_fused_edge_arrow` from a leading `-`,
            // never checked standalone before this table, so no collision there either.
            '<' => Some(TokenKind::Lt),
            '>' => Some(TokenKind::Gt),
            '&' => Some(TokenKind::Amp),
            '$' => Some(TokenKind::Dollar),
            ';' => Some(TokenKind::Semicolon),
            _ => None,
        };
        if let Some(kind) = single {
            i += 1;
            byte_offset += c.len_utf8() as u32;
            column += 1;
            push!(kind, start_line, start_col, start_byte, c.to_string());
            continue;
        }
        // Unknown character.
        i += 1;
        byte_offset += c.len_utf8() as u32;
        column += 1;
        if !forgiving {
            return Err(TextError::new(format!("unexpected character '{c}'"), TextSpan::at(start_line, start_col)));
        }
        push!(TokenKind::Error, start_line, start_col, start_byte, c.to_string());
    }
    let eof_line = line;
    let eof_col = column;
    tokens.push(SpannedToken { id: TokenId(next_id), kind: TokenKind::Eof, text: Symbol::intern(""), span: TextSpan::at(eof_line, eof_col), byte_range: (byte_offset, byte_offset) });
    // Strict-mode success is exactly the invariant `Sanitized` documents — brand it here so the
    // type isn't just a paper promise, then unwrap since callers still want plain tokens.
    if forgiving {
        Ok(tokens)
    } else {
        // `Sanitized::new_trusted`/`.into_inner` (`🗣️dsl/🎖️trust/🦀️.rs`) were reverted to
        // sync per R9 (trivial wrapper types, 0 await/0 io) — direct sync calls now.
        Ok(Sanitized::new_trusted(tokens).into_inner())
    }
}

/// @emoji 🎨️ Maps lexed tokens to editor highlighting classes. `keywords` is the live set of
/// idents that are structural keywords in the current grammar context (schema-declared).
pub fn token_classes(tokens: &[SpannedToken], keywords: &[&str]) -> Vec<(TokenClass, TextSpan)> {
    tokens
        .iter()
        .filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof)
        .map(|t| {
            let class = match t.kind {
                TokenKind::Ident => {
                    let text = t.text.as_str();
                    if keywords.contains(&text.as_ref()) {
                        TokenClass::Keyword
                    } else {
                        TokenClass::Ident
                    }
                }
                TokenKind::Int | TokenKind::Float => TokenClass::Number,
                TokenKind::Text => TokenClass::String,
                TokenKind::Placeholder | TokenKind::DotEnum => TokenClass::Ident,
                TokenKind::Equals
                | TokenKind::Arrow
                | TokenKind::DashArrow
                | TokenKind::BackArrow
                | TokenKind::EdgeArrow
                | TokenKind::At
                | TokenKind::Colon
                | TokenKind::Caret
                | TokenKind::DotDot
                | TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Star
                | TokenKind::Slash
                | TokenKind::Lt
                | TokenKind::Gt
                | TokenKind::Amp
                | TokenKind::Dollar => TokenClass::Operator,
                TokenKind::Fence => TokenClass::String,
                TokenKind::Semicolon => TokenClass::Punctuation,
                TokenKind::Comma | TokenKind::LBrace | TokenKind::RBrace | TokenKind::LBracket | TokenKind::RBracket | TokenKind::LParen | TokenKind::RParen => TokenClass::Punctuation,
                TokenKind::Comment => TokenClass::Comment,
                TokenKind::Error => TokenClass::Error,
                TokenKind::Whitespace | TokenKind::Newline | TokenKind::Eof => unreachable!("filtered above"),
            };
            (class, t.span)
        })
        .collect()
}
/// @emoji 🪪️ True iff `s` lexes (strict) as exactly one `Ident` token whose text equals `s` —
/// i.e. `s` is safe to print bare (unquoted) wherever `Shape::Text` is expected. Excludes the
/// reserved literal idents (`_`/`true`/`false`/`null`/`nan`/`inf`) and anything number-shaped
/// (those lex as `Int`/`Float`/`Placeholder`, not `Ident`, so they're already excluded by
/// construction — the reserved-word list catches the ones that would otherwise lex as `Ident`).
/// Implemented defensively by actually calling the lexer rather than hand-rolling a second
/// notion of "identifier-shaped" that could drift from the real grammar.
pub fn is_bare_ident(s: &str) -> bool {
    if matches!(s, "_" | "true" | "false" | "null" | "nan" | "inf") {
        return false;
    }
    match lex(s, &Limits::default(), false) {
        Ok(tokens) => {
            let significant: Vec<&SpannedToken> = tokens.iter().filter(|t| t.kind != TokenKind::Eof).collect();
            matches!(significant.as_slice(), [only] if only.kind == TokenKind::Ident && only.text.as_str().as_ref() == s)
        }
        Err(_) => false,
    }
}
//#endregion 🔖️Lexer

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
