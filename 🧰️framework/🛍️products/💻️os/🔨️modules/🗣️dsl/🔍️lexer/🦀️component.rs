//! 🔍 The shared DSL lexer and bare-ident predicate.

use crate::os_dsl::diagnostic::{Limits, TextError, TextSpan};
use crate::os_dsl::token::*;

//#region 🔖️Lexer
fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

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
        while j < chars.len() && is_ident_continue(chars[j]) {
            j += 1;
        }
    } else if is_ident_start(chars[j]) {
        while j < chars.len() && is_ident_continue(chars[j]) {
            j += 1;
        }
        if j < chars.len() && chars[j] == ':' {
            j += 1;
            if j >= chars.len() || !is_ident_start(chars[j]) {
                return None;
            }
            while j < chars.len() && is_ident_continue(chars[j]) {
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
/// parse mode and returns the first lexical error.
pub fn lex(text: &str, limits: &Limits, forgiving: bool) -> Result<Vec<SpannedToken>, TextError> {
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
        if c == '#' {
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
        if c == '"' {
            let mut j = i + 1;
            let mut buf = String::new();
            let mut closed = false;
            byte_offset += c.len_utf8() as u32;
            column += 1;
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
                if cj == '"' {
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
            if j < chars.len() && chars[j] == '.' && j + 1 < chars.len() && chars[j + 1].is_ascii_digit() {
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
                let len = end_j - i;
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
                TokenKind::Placeholder => TokenClass::Ident,
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
                | TokenKind::Slash => TokenClass::Operator,
                TokenKind::Fence => TokenClass::String,
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
mod tests {
    use super::*;
    use crate::os_dsl::diagnostic::*;
    use crate::os_dsl::token::*;
    use crate::os_dsl::span::TextSpan;

    #[test]
    fn escape_round_trips_every_control_case() {
        let cases = ["plain text", "with \"quotes\" and \\backslash\\", "line1\nline2\ttabbed\r\n", "unicode: 🔖️ café naïve", "\u{0007}bell and \u{001b}escape"];
        for case in cases {
            let escaped = escape_text(case);
            assert!(!escaped.contains('\n'), "escaped text must not contain a raw newline: {escaped:?}");
            let restored = unescape_text(&escaped, false).expect("unescape");
            assert_eq!(restored, case, "round trip failed for {case:?}");
        }
    }

    #[test]
    fn unescape_forgiving_mode_keeps_unknown_escapes_literal() {
        assert_eq!(unescape_text("\\q", true).unwrap(), "\\q");
        assert!(unescape_text("\\q", false).is_err());
    }

    #[test]
    fn float_format_round_trips_including_specials() {
        for value in [0.0_f64, -0.0, 1.5, -42.125, 1e300, 1e-300, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let printed = format_f64(value);
            let parsed = parse_f64(&printed).expect("parse");
            if value.is_nan() {
                assert!(parsed.is_nan());
            } else {
                assert_eq!(parsed, value, "float round trip failed for {value} -> {printed}");
            }
        }
    }

    #[test]
    fn lexer_tokenizes_a_representative_record_line() {
        let tokens = lex(r#"camera x=1.5 y=-2 zoom=1 label="a \"b\" c""#, &Limits::default(), false).expect("lex");
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

    #[test]
    fn lexer_spans_are_real_not_placeholder() {
        let tokens = lex("a\nb c", &Limits::default(), false).expect("lex");
        let b = tokens.iter().find(|t| t.text.as_str().as_ref() == "b").expect("b token");
        assert_eq!(b.span.line, 2);
        assert_eq!(b.span.column, 1);
        let c = tokens.iter().find(|t| t.text.as_str().as_ref() == "c").expect("c token");
        assert_eq!(c.span.line, 2);
        assert_eq!(c.span.column, 3);
    }

    #[test]
    fn lexer_wire_literal_alphabet_tokenizes() {
        let tokens = lex("a:Kind@out->b:Kind2@in", &Limits::default(), false).expect("lex");
        let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind).filter(|k| !k.is_trivia() && *k != TokenKind::Eof).collect();
        assert_eq!(kinds, vec![TokenKind::Ident, TokenKind::Colon, TokenKind::Ident, TokenKind::At, TokenKind::Ident, TokenKind::Arrow, TokenKind::Ident, TokenKind::Colon, TokenKind::Ident, TokenKind::At, TokenKind::Ident,]);
    }

    #[test]
    fn lexer_kebab_case_ident_and_arrow_coexist() {
        let tokens = lex("hexagonal-mushroom-column->target", &Limits::default(), false).expect("lex");
        let significant: Vec<(TokenKind, String)> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
        assert_eq!(significant, vec![(TokenKind::Ident, "hexagonal-mushroom-column".to_string()), (TokenKind::Arrow, "->".to_string()), (TokenKind::Ident, "target".to_string()),]);
    }

    #[test]
    fn lexer_recognizes_negative_infinity_as_one_float_token() {
        let tokens = lex("x=-inf y=-influence z=5", &Limits::default(), true).expect("lex");
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
        assert_eq!(parse_f64("-inf").unwrap(), f64::NEG_INFINITY);
        assert_eq!(format_f64(f64::NEG_INFINITY), "-inf");
    }

    #[test]
    fn lexer_strict_mode_errors_on_unterminated_string_with_real_span() {
        let error = lex("key=\"unterminated", &Limits::default(), false).unwrap_err();
        assert_eq!(error.span.line, 1);
        assert_eq!(error.span.column, 5);
    }

    #[test]
    fn lexer_forgiving_mode_never_fails_on_malformed_input() {
        let result = lex("key=\"unterminated\n$$$", &Limits::default(), true);
        assert!(result.is_ok(), "forgiving lexer must not error");
    }

    #[test]
    fn limits_reject_oversized_input_with_a_diagnostic_not_a_panic() {
        let tiny = Limits { max_bytes: 4, ..Limits::default() };
        let error = lex("way too long", &tiny, false).unwrap_err();
        assert!(error.message.contains("max_bytes"));
    }

    #[test]
    fn symbol_interning_is_stable_and_deduplicates() {
        let a = Symbol::intern("hello");
        let b = Symbol::intern("hello");
        let c = Symbol::intern("world");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(a.as_str().as_ref(), "hello");
    }

    #[test]
    fn token_classes_distinguish_keywords_from_idents() {
        let tokens = lex("camera x=1", &Limits::default(), false).expect("lex");
        let classes = token_classes(&tokens, &["camera"]);
        assert_eq!(classes[0].0, TokenClass::Keyword);
        assert_eq!(classes[1].0, TokenClass::Ident);
    }

    #[test]
    fn diagnostic_lowers_to_text_error_with_expected_description() {
        let diagnostic = Diagnostic::error("DSL0001", TextSpan::at(2, 3), "unexpected token").with_expected(ExpectedSet { tokens: vec![], keywords: vec!["camera".into(), "layer".into()], keys: vec![] });
        let error = diagnostic.into_text_error();
        assert_eq!(error.span, TextSpan::at(2, 3));
        assert_eq!(error.expected.as_deref(), Some("camera|layer"));
    }

    #[test]
    fn lexer_back_arrow_tokenizes_distinctly_from_dash_and_arrow() {
        let tokens = lex("a<-b a->b a--b a<-hexagonal-column", &Limits::default(), false).expect("lex");
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

    #[test]
    fn lexer_lone_underscore_is_placeholder_but_underscore_words_are_ident() {
        let tokens = lex("_ _foo foo_bar _", &Limits::default(), false).expect("lex");
        let significant: Vec<(TokenKind, String)> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
        assert_eq!(significant, vec![(TokenKind::Placeholder, "_".to_string()), (TokenKind::Ident, "_foo".to_string()), (TokenKind::Ident, "foo_bar".to_string()), (TokenKind::Placeholder, "_".to_string()),]);
    }

    #[test]
    fn is_bare_ident_accepts_normal_idents_and_rejects_reserved_and_number_shaped() {
        assert!(is_bare_ident("alpha"));
        assert!(is_bare_ident("hexagonal-mushroom-column"));
        assert!(is_bare_ident("airtightness_n50"));
        assert!(!is_bare_ident("_"));
        assert!(!is_bare_ident("true"));
        assert!(!is_bare_ident("false"));
        assert!(!is_bare_ident("null"));
        assert!(!is_bare_ident("nan"));
        assert!(!is_bare_ident("inf"));
        assert!(!is_bare_ident("3"));
        assert!(!is_bare_ident("1.5"));
        assert!(!is_bare_ident("-inf"));
        assert!(!is_bare_ident("-2"));
        assert!(!is_bare_ident("two words"));
        assert!(!is_bare_ident(""));
        assert!(!is_bare_ident("\"quoted\""));
    }

    #[test]
    fn lexer_caret_and_dotdot_tokenize_distinctly_from_neighbors() {
        let tokens = lex("^0,1,0 (0..10,0.5) 1.5..3 a..b", &Limits::default(), false).expect("lex");
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

    #[test]
    fn lexer_fence_captures_lang_and_multiline_content() {
        let source = "text=```jack\nMATCH (a) RETURN a\nWHERE a.x > 1\n```\nafter=1";
        let tokens = lex(source, &Limits::default(), false).expect("lex");
        let significant: Vec<&SpannedToken> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).collect();
        let fence = significant.iter().find(|t| t.kind == TokenKind::Fence).expect("a Fence token");
        let raw = fence.text.as_str();
        let (lang, content) = raw.split_once('\u{0}').expect("NUL separator");
        assert_eq!(lang, "jack");
        assert_eq!(content, "MATCH (a) RETURN a\nWHERE a.x > 1");
        // lexing must resume normally right after the closing fence line.
        assert!(significant.iter().any(|t| t.kind == TokenKind::Ident && t.text.as_str().as_ref() == "after"));
    }

    #[test]
    fn lexer_fence_with_no_lang_tag_and_empty_content() {
        let tokens = lex("body=```\n```", &Limits::default(), false).expect("lex");
        let fence = tokens.iter().find(|t| t.kind == TokenKind::Fence).expect("a Fence token");
        let raw = fence.text.as_str();
        let (lang, content) = raw.split_once('\u{0}').expect("NUL separator");
        assert_eq!(lang, "");
        assert_eq!(content, "");
    }

    #[test]
    fn lexer_unterminated_fence_is_a_strict_error_and_forgiving_error_token() {
        let strict = lex("body=```jack\nMATCH (a) RETURN a", &Limits::default(), false);
        assert!(strict.is_err(), "unterminated fence must be a strict-mode error");
        let forgiving = lex("body=```jack\nMATCH (a) RETURN a", &Limits::default(), true);
        assert!(forgiving.is_ok(), "forgiving mode must never fail on malformed input");
    }

    #[test]
    fn unit_lookup_finds_known_symbols_and_rejects_unknown_ones() {
        assert_eq!(unit_by_symbol("GPa").unwrap().symbol, "GPa");
        assert_eq!(unit_by_symbol("deg").unwrap().dimension, DIM_ANGLE);
        assert!(unit_by_symbol("frobnicate").is_none());
    }

    #[test]
    fn unit_conversion_scales_within_a_dimension_and_rejects_across_dimensions() {
        let gpa = unit_by_symbol("GPa").unwrap();
        let mpa = unit_by_symbol("MPa").unwrap();
        assert_eq!(convert(210.0, gpa, mpa), Some(210_000.0));
        let deg = unit_by_symbol("deg").unwrap();
        let rad = unit_by_symbol("rad").unwrap();
        let converted = convert(180.0, deg, rad).unwrap();
        assert!((converted - std::f64::consts::PI).abs() < 1e-9);
        let kg = unit_by_symbol("kg").unwrap();
        assert_eq!(convert(1.0, gpa, kg), None, "pressure must not convert into mass");
    }

    #[test]
    fn unit_conversion_round_trips_back_to_the_original_value() {
        let kn = unit_by_symbol("kN").unwrap();
        let n = unit_by_symbol("N").unwrap();
        let forward = convert(1.5, kn, n).unwrap();
        let back = convert(forward, n, kn).unwrap();
        assert!((back - 1.5).abs() < 1e-9);
    }

    #[test]
    fn unit_conversion_same_unit_short_circuits_bit_exactly() {
        let deg = unit_by_symbol("deg").unwrap();
        // 30.0 degrees previously round-tripped as 29.999999999999996 due to (30.0 * (PI/180)) / (PI/180).
        assert_eq!(convert(30.0, deg, deg), Some(30.0));
    }

    #[test]
    fn ten_thousand_iteration_generative_escape_round_trip() {
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
            let restored = unescape_text(&escaped, false).unwrap_or_else(|e| panic!("seed-reproducible failure for {s:?}: {e}"));
            assert_eq!(restored, s, "generative round trip failed for {s:?}");
        }
    }
}
}
