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
    async fn string_mode(&self, c: char) -> Option<StringMode> {
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
async fn decode_backslash_unit(chars: &[char], j: usize) -> Option<(String, usize)> {
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
            let (hi, after_hi) = read_hex4(chars, j + 2).await?;
            if (0xD800..=0xDBFF).contains(&hi) {
                if chars.get(after_hi) == Some(&'\\') && chars.get(after_hi + 1) == Some(&'u') {
                    if let Some((lo, after_lo)) = read_hex4(chars, after_hi + 2).await {
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

async fn read_hex4(chars: &[char], start: usize) -> Option<(u32, usize)> {
    if start + 4 > chars.len() {
        return None;
    }
    let text: String = chars[start..start + 4].iter().collect();
    u32::from_str_radix(&text, 16).ok().map(|v| (v, start + 4))
}

/// @emoji 🔎️ True iff `chars[i..]` begins with `needle`'s chars — the multi-char-marker match
/// used by both configurable comment forms (line marker, block open/close).
async fn chars_start_with(chars: &[char], i: usize, needle: &str) -> bool {
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
async fn lex_fused_edge_arrow(chars: &[char], i: usize) -> Option<(usize, String)> {
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
pub async fn lex(text: &str, limits: &Limits, forgiving: bool) -> Result<Vec<SpannedToken>, TextError> {
    lex_with(text, limits, forgiving, &LexOptions::default()).await
}

/// @emoji 🎛️ P2-M1: same lexer, parameterized by a per-grammar [`LexOptions`] (string quote+escape
/// modes, comment syntax). `lex` is the fixed-default entry point every pre-M1 caller still uses.
pub async fn lex_with(text: &str, limits: &Limits, forgiving: bool, opts: &LexOptions) -> Result<Vec<SpannedToken>, TextError> {
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
            if chars_start_with(&chars, i, marker).await {
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
            if chars_start_with(&chars, i, open).await {
                let open_len = open.chars().count();
                let mut k = i + open_len;
                let mut found_end: Option<usize> = None;
                while k < chars.len() {
                    if chars_start_with(&chars, k, close).await {
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
        if let Some(mode) = opts.string_mode(c).await {
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
                            match decode_backslash_unit(&chars, j).await {
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
            if let Some((end_j, fused_text)) = lex_fused_edge_arrow(&chars, i).await {
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
        Ok(Sanitized::new_trusted(tokens).await.into_inner().await)
    }
}

/// @emoji 🎨️ Maps lexed tokens to editor highlighting classes. `keywords` is the live set of
/// idents that are structural keywords in the current grammar context (schema-declared).
pub async fn token_classes(tokens: &[SpannedToken], keywords: &[&str]) -> Vec<(TokenClass, TextSpan)> {
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
pub async fn is_bare_ident(s: &str) -> bool {
    if matches!(s, "_" | "true" | "false" | "null" | "nan" | "inf") {
        return false;
    }
    match lex(s, &Limits::default(), false).await {
        Ok(tokens) => {
            let significant: Vec<&SpannedToken> = tokens.iter().filter(|t| t.kind != TokenKind::Eof).collect();
            matches!(significant.as_slice(), [only] if only.kind == TokenKind::Ident && only.text.as_str().as_ref() == s)
        }
        Err(_) => false,
    }
}
//#endregion 🔖️Lexer

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_dsl::diagnostic::*;
    use crate::os_dsl::token::*;
    use crate::os_dsl::span::TextSpan;

    #[semio_framework_async_macros::async_test]
    async fn escape_round_trips_every_control_case() {
        let cases = ["plain text", "with \"quotes\" and \\backslash\\", "line1\nline2\ttabbed\r\n", "unicode: 🔖️ café naïve", "\u{0007}bell and \u{001b}escape"];
        for case in cases {
            let escaped = escape_text(case);
            assert!(!escaped.contains('\n'), "escaped text must not contain a raw newline: {escaped:?}");
            let restored = unescape_text(&escaped, false).await.expect("unescape");
            assert_eq!(restored, case, "round trip failed for {case:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn unescape_forgiving_mode_keeps_unknown_escapes_literal() {
        assert_eq!(unescape_text("\\q", true).await.unwrap(), "\\q");
        assert!(unescape_text("\\q", false).await.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn float_format_round_trips_including_specials() {
        for value in [0.0_f64, -0.0, 1.5, -42.125, 1e300, 1e-300, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let printed = format_f64(value);
            let parsed = parse_f64(&printed).await.expect("parse");
            if value.is_nan() {
                assert!(parsed.is_nan());
            } else {
                assert_eq!(parsed, value, "float round trip failed for {value} -> {printed}");
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn lexer_tokenizes_a_representative_record_line() {
        let tokens = lex(r#"camera x=1.5 y=-2 zoom=1 label="a \"b\" c""#, &Limits::default(), false).await.expect("lex");
        let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind).filter(|k| !k.is_trivia()).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Ident, // camera
                TokenKind::Ident, // x
                TokenKind::Equals,
                TokenKind::Float, // 1.5
                TokenKind::Ident, // y
                TokenKind::Equals,
                TokenKind::Int,   // -2
                TokenKind::Ident, // zoom
                TokenKind::Equals,
                TokenKind::Int,   // 1
                TokenKind::Ident, // label
                TokenKind::Equals,
                TokenKind::Text,
                TokenKind::Eof,
            ]
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn lexer_spans_are_real_not_placeholder() {
        let tokens = lex("a\nb c", &Limits::default(), false).await.expect("lex");
        let b = tokens.iter().find(|t| t.text.as_str().as_ref() == "b").expect("b token");
        assert_eq!(b.span.line, 2);
        assert_eq!(b.span.column, 1);
        let c = tokens.iter().find(|t| t.text.as_str().as_ref() == "c").expect("c token");
        assert_eq!(c.span.line, 2);
        assert_eq!(c.span.column, 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn lexer_wire_literal_alphabet_tokenizes() {
        let tokens = lex("a:Kind@out->b:Kind2@in", &Limits::default(), false).await.expect("lex");
        let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind).filter(|k| !k.is_trivia() && *k != TokenKind::Eof).collect();
        assert_eq!(kinds, vec![TokenKind::Ident, TokenKind::Colon, TokenKind::Ident, TokenKind::At, TokenKind::Ident, TokenKind::Arrow, TokenKind::Ident, TokenKind::Colon, TokenKind::Ident, TokenKind::At, TokenKind::Ident,]);
    }

    #[semio_framework_async_macros::async_test]
    async fn lexer_kebab_case_ident_and_arrow_coexist() {
        let tokens = lex("hexagonal-mushroom-column->target", &Limits::default(), false).await.expect("lex");
        let significant: Vec<(TokenKind, String)> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
        assert_eq!(significant, vec![(TokenKind::Ident, "hexagonal-mushroom-column".to_string()), (TokenKind::Arrow, "->".to_string()), (TokenKind::Ident, "target".to_string()),]);
    }

    #[semio_framework_async_macros::async_test]
    async fn lexer_recognizes_negative_infinity_as_one_float_token() {
        let tokens = lex("x=-inf y=-influence z=5", &Limits::default(), true).await.expect("lex");
        let significant: Vec<(TokenKind, String)> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
        assert_eq!(
            significant,
            vec![
                (TokenKind::Ident, "x".to_string()),
                (TokenKind::Equals, "=".to_string()),
                (TokenKind::Float, "-inf".to_string()),
                (TokenKind::Ident, "y".to_string()),
                (TokenKind::Equals, "=".to_string()),
                // "-influence" must NOT be split into a "-inf" float token plus a stray "luence"
                // ident — the lookahead requires the char right after "-inf" to not itself
                // continue an identifier, so the leading '-' falls through to the Minus operator
                // token instead (added for Shape::Expr; previously this fell all the way through
                // to "unknown character"/Error before Minus existed), then "influence" lexes as
                // its own ident, unaffected either way.
                (TokenKind::Minus, "-".to_string()),
                (TokenKind::Ident, "influence".to_string()),
                (TokenKind::Ident, "z".to_string()),
                (TokenKind::Equals, "=".to_string()),
                (TokenKind::Int, "5".to_string()),
            ]
        );
        assert_eq!(parse_f64("-inf").await.unwrap(), f64::NEG_INFINITY);
        assert_eq!(format_f64(f64::NEG_INFINITY), "-inf");
    }

    #[semio_framework_async_macros::async_test]
    async fn lexer_strict_mode_errors_on_unterminated_string_with_real_span() {
        let error = lex("key=\"unterminated", &Limits::default(), false).await.unwrap_err();
        assert_eq!(error.span.line, 1);
        assert_eq!(error.span.column, 5);
    }

    #[semio_framework_async_macros::async_test]
    async fn lexer_forgiving_mode_never_fails_on_malformed_input() {
        let limits = Limits::default();
        let result = lex("key=\"unterminated\n$$$", &limits, true);
        assert!(result.await.is_ok(), "forgiving lexer must not error");
    }

    #[semio_framework_async_macros::async_test]
    async fn limits_reject_oversized_input_with_a_diagnostic_not_a_panic() {
        let tiny = Limits { max_bytes: 4, ..Limits::default() };
        let error = lex("way too long", &tiny, false).await.unwrap_err();
        assert!(error.message.contains("max_bytes"));
    }

    #[semio_framework_async_macros::async_test]
    async fn symbol_interning_is_stable_and_deduplicates() {
        let a = Symbol::intern("hello");
        let b = Symbol::intern("hello");
        let c = Symbol::intern("world");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(a.as_str().as_ref(), "hello");
    }

    #[semio_framework_async_macros::async_test]
    async fn token_classes_distinguish_keywords_from_idents() {
        let tokens = lex("camera x=1", &Limits::default(), false).await.expect("lex");
        let classes = token_classes(&tokens, &["camera"]).await;
        assert_eq!(classes[0].0, TokenClass::Keyword);
        assert_eq!(classes[1].0, TokenClass::Ident);
    }

    #[semio_framework_async_macros::async_test]
    async fn diagnostic_lowers_to_text_error_with_expected_description() {
        let diagnostic = Diagnostic::error("DSL0001", TextSpan::at(2, 3), "unexpected token").with_expected(ExpectedSet { tokens: vec![], keywords: vec!["camera".into(), "layer".into()], keys: vec![] });
        let error = diagnostic.into_text_error();
        assert_eq!(error.span, TextSpan::at(2, 3));
        assert_eq!(error.expected.as_deref(), Some("camera|layer"));
    }

    #[semio_framework_async_macros::async_test]
    async fn lexer_back_arrow_tokenizes_distinctly_from_dash_and_arrow() {
        let tokens = lex("a<-b a->b a--b a<-hexagonal-column", &Limits::default(), false).await.expect("lex");
        let significant: Vec<(TokenKind, String)> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
        assert_eq!(
            significant,
            vec![
                (TokenKind::Ident, "a".to_string()),
                (TokenKind::BackArrow, "<-".to_string()),
                (TokenKind::Ident, "b".to_string()),
                (TokenKind::Ident, "a".to_string()),
                (TokenKind::Arrow, "->".to_string()),
                (TokenKind::Ident, "b".to_string()),
                (TokenKind::Ident, "a".to_string()),
                (TokenKind::DashArrow, "--".to_string()),
                (TokenKind::Ident, "b".to_string()),
                (TokenKind::Ident, "a".to_string()),
                (TokenKind::BackArrow, "<-".to_string()),
                // `<` isn't ident-continue, so the '<' of "<-" can never be swallowed into the
                // preceding kebab ident, and the following ident lexes untouched.
                (TokenKind::Ident, "hexagonal-column".to_string()),
            ]
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn lexer_lone_underscore_is_placeholder_but_underscore_words_are_ident() {
        let tokens = lex("_ _foo foo_bar _", &Limits::default(), false).await.expect("lex");
        let significant: Vec<(TokenKind, String)> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
        assert_eq!(significant, vec![(TokenKind::Placeholder, "_".to_string()), (TokenKind::Ident, "_foo".to_string()), (TokenKind::Ident, "foo_bar".to_string()), (TokenKind::Placeholder, "_".to_string()),]);
    }

    #[semio_framework_async_macros::async_test]
    async fn is_bare_ident_accepts_normal_idents_and_rejects_reserved_and_number_shaped() {
        assert!(is_bare_ident("alpha").await);
        assert!(is_bare_ident("hexagonal-mushroom-column").await);
        assert!(is_bare_ident("airtightness_n50").await);
        assert!(!is_bare_ident("_").await);
        assert!(!is_bare_ident("true").await);
        assert!(!is_bare_ident("false").await);
        assert!(!is_bare_ident("null").await);
        assert!(!is_bare_ident("nan").await);
        assert!(!is_bare_ident("inf").await);
        assert!(!is_bare_ident("3").await);
        assert!(!is_bare_ident("1.5").await);
        assert!(!is_bare_ident("-inf").await);
        assert!(!is_bare_ident("-2").await);
        assert!(!is_bare_ident("two words").await);
        assert!(!is_bare_ident("").await);
        assert!(!is_bare_ident("\"quoted\"").await);
    }

    #[semio_framework_async_macros::async_test]
    async fn lexer_caret_and_dotdot_tokenize_distinctly_from_neighbors() {
        let tokens = lex("^0,1,0 (0..10,0.5) 1.5..3 a..b", &Limits::default(), false).await.expect("lex");
        let significant: Vec<(TokenKind, String)> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
        assert_eq!(
            significant,
            vec![
                (TokenKind::Caret, "^".to_string()),
                (TokenKind::Int, "0".to_string()),
                (TokenKind::Comma, ",".to_string()),
                (TokenKind::Int, "1".to_string()),
                (TokenKind::Comma, ",".to_string()),
                (TokenKind::Int, "0".to_string()),
                (TokenKind::LParen, "(".to_string()),
                (TokenKind::Int, "0".to_string()),
                (TokenKind::DotDot, "..".to_string()),
                (TokenKind::Int, "10".to_string()),
                (TokenKind::Comma, ",".to_string()),
                (TokenKind::Float, "0.5".to_string()),
                (TokenKind::RParen, ")".to_string()),
                (TokenKind::Float, "1.5".to_string()),
                (TokenKind::DotDot, "..".to_string()),
                (TokenKind::Int, "3".to_string()),
                // A dot INSIDE an already-started ident never splits into DotDot — "a..b" stays
                // one ident, exactly like kebab idents protect "-" from the Arrow/DashArrow checks.
                (TokenKind::Ident, "a..b".to_string()),
            ]
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn lexer_fence_captures_lang_and_multiline_content() {
        let source = "text=```jack\nMATCH (a) RETURN a\nWHERE a.x > 1\n```\nafter=1";
        let tokens = lex(source, &Limits::default(), false).await.expect("lex");
        let significant: Vec<&SpannedToken> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).collect();
        let fence = significant.iter().find(|t| t.kind == TokenKind::Fence).expect("a Fence token");
        let raw = fence.text.as_str();
        let (lang, content) = raw.split_once('\u{0}').expect("NUL separator");
        assert_eq!(lang, "jack");
        assert_eq!(content, "MATCH (a) RETURN a\nWHERE a.x > 1");
        // lexing must resume normally right after the closing fence line.
        assert!(significant.iter().any(|t| t.kind == TokenKind::Ident && t.text.as_str().as_ref() == "after"));
    }

    #[semio_framework_async_macros::async_test]
    async fn lexer_fence_with_no_lang_tag_and_empty_content() {
        let tokens = lex("body=```\n```", &Limits::default(), false).await.expect("lex");
        let fence = tokens.iter().find(|t| t.kind == TokenKind::Fence).expect("a Fence token");
        let raw = fence.text.as_str();
        let (lang, content) = raw.split_once('\u{0}').expect("NUL separator");
        assert_eq!(lang, "");
        assert_eq!(content, "");
    }

    #[semio_framework_async_macros::async_test]
    async fn lexer_unterminated_fence_is_a_strict_error_and_forgiving_error_token() {
        let limits = Limits::default();
        let strict = lex("body=```jack\nMATCH (a) RETURN a", &limits, false);
        assert!(strict.await.is_err(), "unterminated fence must be a strict-mode error");
        let forgiving = lex("body=```jack\nMATCH (a) RETURN a", &limits, true);
        assert!(forgiving.await.is_ok(), "forgiving mode must never fail on malformed input");
    }

    #[semio_framework_async_macros::async_test]
    async fn unit_lookup_finds_known_symbols_and_rejects_unknown_ones() {
        assert_eq!(unit_by_symbol("GPa").await.unwrap().symbol, "GPa");
        assert_eq!(unit_by_symbol("deg").await.unwrap().dimension, DIM_ANGLE);
        assert!(unit_by_symbol("frobnicate").await.is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn unit_conversion_scales_within_a_dimension_and_rejects_across_dimensions() {
        let gpa = unit_by_symbol("GPa").await.unwrap();
        let mpa = unit_by_symbol("MPa").await.unwrap();
        assert_eq!(convert(210.0, gpa, mpa).await, Some(210_000.0));
        let deg = unit_by_symbol("deg").await.unwrap();
        let rad = unit_by_symbol("rad").await.unwrap();
        let converted = convert(180.0, deg, rad).await.unwrap();
        assert!((converted - std::f64::consts::PI).abs() < 1e-9);
        let kg = unit_by_symbol("kg").await.unwrap();
        assert_eq!(convert(1.0, gpa, kg).await, None, "pressure must not convert into mass");
    }

    #[semio_framework_async_macros::async_test]
    async fn unit_conversion_round_trips_back_to_the_original_value() {
        let kn = unit_by_symbol("kN").await.unwrap();
        let n = unit_by_symbol("N").await.unwrap();
        let forward = convert(1.5, kn, n).await.unwrap();
        let back = convert(forward, n, kn).await.unwrap();
        assert!((back - 1.5).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn unit_conversion_same_unit_short_circuits_bit_exactly() {
        let deg = unit_by_symbol("deg").await.unwrap();
        // 30.0 degrees previously round-tripped as 29.999999999999996 due to (30.0 * (PI/180)) / (PI/180).
        assert_eq!(convert(30.0, deg, deg).await, Some(30.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn ten_thousand_iteration_generative_escape_round_trip() {
        // Hand-rolled xorshift — no proptest/quickcheck dependency in this workspace.
        let mut state: u64 = 0x9E3779B97F4A7C15;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let alphabet: Vec<char> = "abc\"\\\n\t\r🔖️café".chars().collect();
        for _ in 0..10_000 {
            let len = (next() % 12) as usize;
            let s: String = (0..len).map(|_| alphabet[(next() as usize) % alphabet.len()]).collect();
            let escaped = escape_text(&s);
            assert!(!escaped.contains('\n'));
            let restored = unescape_text(&escaped, false).await.unwrap_or_else(|e| panic!("seed-reproducible failure for {s:?}: {e}"));
            assert_eq!(restored, s, "generative round trip failed for {s:?}");
        }
    }

    //#region 🔖️P2M1Dialect
    // P2-M1 item 1: generalized string/text token — configurable quote+escape modes.
    #[semio_framework_async_macros::async_test]
    async fn default_lex_options_is_byte_identical_to_pre_m1_raw_double_quote_behavior() {
        let opts = LexOptions::default();
        let tokens = lex_with(r#"label="a \"b\" c""#, &Limits::default(), false, &opts).await.expect("lex_with default");
        let text = tokens.iter().find(|t| t.kind == TokenKind::Text).expect("Text token");
        // Raw mode: backslash pairs stay undecoded in the token text, exactly like `lex` always did.
        assert_eq!(text.text.as_str().as_ref(), r#"a \"b\" c"#);
    }

    #[semio_framework_async_macros::async_test]
    async fn json_style_backslash_mode_decodes_standard_escapes_and_u_xxxx_surrogate_pairs() {
        let opts = LexOptions { strings: vec![StringMode { quote: '"', escape: StringEscape::Backslash }], comment: CommentDialect::default() };
        // Raw Rust string so `\n`/`\t`/`\uD83D`/`\uDE00` reach the lexer as literal backslash
        // sequences (not pre-decoded by Rust itself) — `😀` is U+1F600's UTF-16
        // surrogate pair, the exact shape RFC 8259 §7 requires for astral codepoints.
        let tokens = lex_with(r#""line1\nline2\ttabA\uD83D\uDE00""#, &Limits::default(), false, &opts).await.expect("lex_with json backslash");
        let text = tokens.iter().find(|t| t.kind == TokenKind::Text).expect("Text token");
        assert_eq!(text.text.as_str().as_ref(), "line1\nline2\ttabA\u{1F600}");
    }

    #[semio_framework_async_macros::async_test]
    async fn csv_style_doubled_quote_mode_decodes_doubled_delimiter_and_ignores_backslash() {
        let opts = LexOptions { strings: vec![StringMode { quote: '"', escape: StringEscape::Doubled }], comment: CommentDialect::default() };
        let tokens = lex_with(r#""a""b",backslash="\not-an-escape""#, &Limits::default(), false, &opts).await.expect("lex_with csv doubled");
        let texts: Vec<String> = tokens.iter().filter(|t| t.kind == TokenKind::Text).map(|t| t.text.as_str().to_string()).collect();
        assert_eq!(texts[0], "a\"b", "doubled `\"\"` decodes to one literal quote");
        assert_eq!(texts[1], r#"\not-an-escape"#, "backslash has no special meaning under Doubled");
    }

    #[semio_framework_async_macros::async_test]
    async fn single_quote_strings_work_alongside_double_quote_xml_style() {
        let opts = LexOptions { strings: vec![StringMode { quote: '"', escape: StringEscape::Raw }, StringMode { quote: '\'', escape: StringEscape::Raw }], comment: CommentDialect::default() };
        let tokens = lex_with(r#"a="1" b='2'"#, &Limits::default(), false, &opts).await.expect("lex_with xml quotes");
        let texts: Vec<(TokenKind, String)> = tokens.iter().filter(|t| t.kind == TokenKind::Text).map(|t| (t.kind, t.text.as_str().to_string())).collect();
        assert_eq!(texts, vec![(TokenKind::Text, "1".to_string()), (TokenKind::Text, "2".to_string())]);
    }

    #[semio_framework_async_macros::async_test]
    async fn step_style_single_quote_doubled_mode_decodes_doubled_apostrophe() {
        let opts = LexOptions { strings: vec![StringMode { quote: '\'', escape: StringEscape::Doubled }], comment: CommentDialect::default() };
        let tokens = lex_with(r#"'it''s a beam'"#, &Limits::default(), false, &opts).await.expect("lex_with step doubled");
        let text = tokens.iter().find(|t| t.kind == TokenKind::Text).expect("Text token");
        assert_eq!(text.text.as_str().as_ref(), "it's a beam");
    }

    // P2-M1 item 3: promoted single-char tokens `< > & $ ;`, non-colliding with arrow forms.
    #[semio_framework_async_macros::async_test]
    async fn promoted_tokens_lex_standalone_without_breaking_arrow_forms() {
        let tokens = lex("<tag a=\"1\" & $VAR ; b<-c d->e f--g", &Limits::default(), false).await.expect("lex");
        let significant: Vec<(TokenKind, String)> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
        assert_eq!(
            significant,
            vec![
                (TokenKind::Lt, "<".to_string()),
                (TokenKind::Ident, "tag".to_string()),
                (TokenKind::Ident, "a".to_string()),
                (TokenKind::Equals, "=".to_string()),
                (TokenKind::Text, "1".to_string()),
                (TokenKind::Amp, "&".to_string()),
                (TokenKind::Dollar, "$".to_string()),
                (TokenKind::Ident, "VAR".to_string()),
                (TokenKind::Semicolon, ";".to_string()),
                (TokenKind::Ident, "b".to_string()),
                (TokenKind::BackArrow, "<-".to_string()),
                (TokenKind::Ident, "c".to_string()),
                (TokenKind::Ident, "d".to_string()),
                (TokenKind::Arrow, "->".to_string()),
                (TokenKind::Ident, "e".to_string()),
                (TokenKind::Ident, "f".to_string()),
                (TokenKind::DashArrow, "--".to_string()),
                (TokenKind::Ident, "g".to_string()),
            ]
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn bare_gt_lexes_standalone_outside_fused_edge_arrow_context() {
        let tokens = lex("a > b", &Limits::default(), false).await.expect("lex");
        let significant: Vec<TokenKind> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| t.kind).collect();
        assert_eq!(significant, vec![TokenKind::Ident, TokenKind::Gt, TokenKind::Ident]);
    }

    // P2-M1 item 4: per-grammar comment dialect (custom line marker, disabled, block comment).
    #[semio_framework_async_macros::async_test]
    async fn comment_line_marker_is_configurable_and_disableable() {
        let slash_slash = LexOptions { strings: vec![], comment: CommentDialect { line: Some("//".to_string()), block: None } };
        let tokens = lex_with("a // not a hash comment\n# still data now b", &Limits::default(), true, &slash_slash).await.expect("lex_with //");
        let significant: Vec<(TokenKind, String)> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
        // "//..." is a comment (dropped as trivia, stops at the real newline); "#" is no longer
        // special under this dialect and falls through to "unknown character" (Error, forgiving),
        // then "still"/"data"/"now"/"b" lex as ordinary idents on the next line.
        assert_eq!(significant[0], (TokenKind::Ident, "a".to_string()));
        assert!(significant.iter().any(|(k, t)| *k == TokenKind::Error && t == "#"), "'#' must no longer be swallowed as a comment marker");
        assert!(significant.iter().any(|(_, t)| t == "still"), "text after the real newline must still lex, comment stopped at EOL");

        let none = LexOptions { strings: vec![], comment: CommentDialect { line: None, block: None } };
        let entity_like = lex_with("#123=WALL;", &Limits::default(), true, &none).await.expect("lex_with comment none");
        let kinds: Vec<(TokenKind, String)> = entity_like.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
        // With line comments off, '#' is no longer swallowed into a Comment token — the entity
        // number "123" lexes as a real Int and ";" as a real Semicolon, not eaten by a comment.
        assert!(kinds.iter().any(|(k, t)| *k == TokenKind::Int && t == "123"), "the entity number 123 must lex as a real Int, not be eaten by a comment");
        assert!(kinds.iter().any(|(k, _)| *k == TokenKind::Semicolon), "';' must lex as a real Semicolon token");
    }

    #[semio_framework_async_macros::async_test]
    async fn block_comment_step_style_spans_lines_and_does_not_consume_entity_hash() {
        let step_opts = LexOptions { strings: vec![StringMode { quote: '\'', escape: StringEscape::Doubled }], comment: CommentDialect { line: None, block: Some(("/*".to_string(), "*/".to_string())) } };
        let source = "#10=IFCWALL('a''b')\n/* a block\ncomment spanning lines */\n#20=IFCSLAB('c');";
        let tokens = lex_with(source, &Limits::default(), true, &step_opts).await.expect("lex_with step block comment");
        let significant: Vec<(TokenKind, String)> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
        // `#` isn't a comment marker here (comment.line = None) — both entity lines' `#NN` sigils
        // lex as Dollar-less Error/Int pairs, i.e. the numbers 10/20 are real Int tokens, not eaten.
        assert!(significant.iter().any(|(k, t)| *k == TokenKind::Int && t == "10"));
        assert!(significant.iter().any(|(k, t)| *k == TokenKind::Int && t == "20"));
        // The doubled-quote string decodes "a''b" -> "a'b".
        assert!(significant.iter().any(|(k, t)| *k == TokenKind::Text && t == "a'b"));
        // The block comment's own content (the words "block"/"comment"/"spanning"/"lines") must
        // NOT appear as separate tokens — it was consumed whole as one Comment (trivia).
        assert!(!significant.iter().any(|(_, t)| t == "spanning"));
    }

    // P2-M1 item 5: trailing-dot floats + leading-dot enum literals.
    #[semio_framework_async_macros::async_test]
    async fn trailing_dot_floats_lex_while_range_dotdot_still_wins() {
        let tokens = lex("0. 10. 3.5 0..10 10..", &Limits::default(), false).await.expect("lex");
        let significant: Vec<(TokenKind, String)> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
        assert_eq!(
            significant,
            vec![
                (TokenKind::Float, "0.".to_string()),
                (TokenKind::Float, "10.".to_string()),
                (TokenKind::Float, "3.5".to_string()),
                (TokenKind::Int, "0".to_string()),
                (TokenKind::DotDot, "..".to_string()),
                (TokenKind::Int, "10".to_string()),
                (TokenKind::Int, "10".to_string()),
                (TokenKind::DotDot, "..".to_string()),
            ]
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn leading_dot_enum_literals_lex_as_dotenum_step_style() {
        let tokens = lex(".T. .F. .UNSPECIFIED. plain", &Limits::default(), false).await.expect("lex");
        let significant: Vec<(TokenKind, String)> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
        assert_eq!(
            significant,
            vec![
                (TokenKind::DotEnum, ".T.".to_string()),
                (TokenKind::DotEnum, ".F.".to_string()),
                (TokenKind::DotEnum, ".UNSPECIFIED.".to_string()),
                (TokenKind::Ident, "plain".to_string()),
            ]
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn lone_leading_dot_without_closing_dot_is_unaffected_by_dotenum() {
        // ".foo" (no closing dot) must NOT become DotEnum — falls through exactly like before
        // this feature existed: an "unknown character" '.' (forgiving -> Error) then an Ident.
        let tokens = lex(".foo", &Limits::default(), true).await.expect("lex forgiving");
        let significant: Vec<TokenKind> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| t.kind).collect();
        assert_eq!(significant, vec![TokenKind::Error, TokenKind::Ident]);
    }
    //#endregion 🔖️P2M1Dialect
}
