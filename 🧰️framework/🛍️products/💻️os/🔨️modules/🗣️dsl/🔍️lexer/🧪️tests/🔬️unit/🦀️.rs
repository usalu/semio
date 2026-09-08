use super::*;
use crate::os_dsl::diagnostic::*;
use crate::os_dsl::span::TextSpan;
use crate::os_dsl::token::*;

#[semio_framework_async_macros::async_test]
async fn escape_round_trips_every_control_case() {
    let cases = ["plain text", "with \"quotes\" and \\backslash\\", "line1\nline2\ttabbed\r\n", "unicode: 🔖️ café naïve", "\u{0007}bell and \u{001b}escape"];
    for case in cases {
        let escaped = escape_text(case);
        assert!(!escaped.contains('\n'), "escaped text must not contain a raw newline: {escaped:?}");
        let restored = unescape_text(&escaped, false).expect("unescape");
        assert_eq!(restored, case, "round trip failed for {case:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn unescape_forgiving_mode_keeps_unknown_escapes_literal() {
    assert_eq!(unescape_text("\\q", true).unwrap(), "\\q");
    assert!(unescape_text("\\q", false).is_err());
}

#[semio_framework_async_macros::async_test]
async fn float_format_round_trips_including_specials() {
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

#[semio_framework_async_macros::async_test]
async fn lexer_tokenizes_a_representative_record_line() {
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

#[semio_framework_async_macros::async_test]
async fn lexer_spans_are_real_not_placeholder() {
    let tokens = lex("a\nb c", &Limits::default(), false).expect("lex");
    let b = tokens.iter().find(|t| t.text.as_str().as_ref() == "b").expect("b token");
    assert_eq!(b.span.line, 2);
    assert_eq!(b.span.column, 1);
    let c = tokens.iter().find(|t| t.text.as_str().as_ref() == "c").expect("c token");
    assert_eq!(c.span.line, 2);
    assert_eq!(c.span.column, 3);
}

#[semio_framework_async_macros::async_test]
async fn lexer_wire_literal_alphabet_tokenizes() {
    let tokens = lex("a:Kind@out->b:Kind2@in", &Limits::default(), false).expect("lex");
    let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind).filter(|k| !k.is_trivia() && *k != TokenKind::Eof).collect();
    assert_eq!(kinds, vec![TokenKind::Ident, TokenKind::Colon, TokenKind::Ident, TokenKind::At, TokenKind::Ident, TokenKind::Arrow, TokenKind::Ident, TokenKind::Colon, TokenKind::Ident, TokenKind::At, TokenKind::Ident,]);
}

#[semio_framework_async_macros::async_test]
async fn lexer_kebab_case_ident_and_arrow_coexist() {
    let tokens = lex("hexagonal-mushroom-column->target", &Limits::default(), false).expect("lex");
    let significant: Vec<(TokenKind, String)> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
    assert_eq!(significant, vec![(TokenKind::Ident, "hexagonal-mushroom-column".to_string()), (TokenKind::Arrow, "->".to_string()), (TokenKind::Ident, "target".to_string()),]);
}

#[semio_framework_async_macros::async_test]
async fn lexer_recognizes_negative_infinity_as_one_float_token() {
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

#[semio_framework_async_macros::async_test]
async fn lexer_strict_mode_errors_on_unterminated_string_with_real_span() {
    let error = lex("key=\"unterminated", &Limits::default(), false).unwrap_err();
    assert_eq!(error.span.line, 1);
    assert_eq!(error.span.column, 5);
}

#[semio_framework_async_macros::async_test]
async fn lexer_forgiving_mode_never_fails_on_malformed_input() {
    let limits = Limits::default();
    let result = lex("key=\"unterminated\n$$$", &limits, true);
    assert!(result.is_ok(), "forgiving lexer must not error");
}

#[semio_framework_async_macros::async_test]
async fn limits_reject_oversized_input_with_a_diagnostic_not_a_panic() {
    let tiny = Limits { max_bytes: 4, ..Limits::default() };
    let error = lex("way too long", &tiny, false).unwrap_err();
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
    let tokens = lex("camera x=1", &Limits::default(), false).expect("lex");
    let classes = token_classes(&tokens, &["camera"]);
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

#[semio_framework_async_macros::async_test]
async fn lexer_lone_underscore_is_placeholder_but_underscore_words_are_ident() {
    let tokens = lex("_ _foo foo_bar _", &Limits::default(), false).expect("lex");
    let significant: Vec<(TokenKind, String)> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
    assert_eq!(significant, vec![(TokenKind::Placeholder, "_".to_string()), (TokenKind::Ident, "_foo".to_string()), (TokenKind::Ident, "foo_bar".to_string()), (TokenKind::Placeholder, "_".to_string()),]);
}

#[semio_framework_async_macros::async_test]
async fn is_bare_ident_accepts_normal_idents_and_rejects_reserved_and_number_shaped() {
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

#[semio_framework_async_macros::async_test]
async fn lexer_caret_and_dotdot_tokenize_distinctly_from_neighbors() {
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

#[semio_framework_async_macros::async_test]
async fn lexer_fence_captures_lang_and_multiline_content() {
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

#[semio_framework_async_macros::async_test]
async fn lexer_fence_with_no_lang_tag_and_empty_content() {
    let tokens = lex("body=```\n```", &Limits::default(), false).expect("lex");
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
    assert!(strict.is_err(), "unterminated fence must be a strict-mode error");
    let forgiving = lex("body=```jack\nMATCH (a) RETURN a", &limits, true);
    assert!(forgiving.is_ok(), "forgiving mode must never fail on malformed input");
}

#[semio_framework_async_macros::async_test]
async fn unit_lookup_finds_known_symbols_and_rejects_unknown_ones() {
    assert_eq!(unit_by_symbol("GPa").unwrap().symbol, "GPa");
    assert_eq!(unit_by_symbol("deg").unwrap().dimension, DIM_ANGLE);
    assert!(unit_by_symbol("frobnicate").is_none());
}

#[semio_framework_async_macros::async_test]
async fn unit_conversion_scales_within_a_dimension_and_rejects_across_dimensions() {
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

#[semio_framework_async_macros::async_test]
async fn unit_conversion_round_trips_back_to_the_original_value() {
    let kn = unit_by_symbol("kN").unwrap();
    let n = unit_by_symbol("N").unwrap();
    let forward = convert(1.5, kn, n).unwrap();
    let back = convert(forward, n, kn).unwrap();
    assert!((back - 1.5).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn unit_conversion_same_unit_short_circuits_bit_exactly() {
    let deg = unit_by_symbol("deg").unwrap();
    // 30.0 degrees previously round-tripped as 29.999999999999996 due to (30.0 * (PI/180)) / (PI/180).
    assert_eq!(convert(30.0, deg, deg), Some(30.0));
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
        let restored = unescape_text(&escaped, false).unwrap_or_else(|e| panic!("seed-reproducible failure for {s:?}: {e}"));
        assert_eq!(restored, s, "generative round trip failed for {s:?}");
    }
}

//#region 🔖️P2M1Dialect
// P2-M1 item 1: generalized string/text token — configurable quote+escape modes.
#[semio_framework_async_macros::async_test]
async fn default_lex_options_is_byte_identical_to_pre_m1_raw_double_quote_behavior() {
    let opts = LexOptions::default();
    let tokens = lex_with(r#"label="a \"b\" c""#, &Limits::default(), false, &opts).expect("lex_with default");
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
    let tokens = lex_with(r#""line1\nline2\ttabA\uD83D\uDE00""#, &Limits::default(), false, &opts).expect("lex_with json backslash");
    let text = tokens.iter().find(|t| t.kind == TokenKind::Text).expect("Text token");
    assert_eq!(text.text.as_str().as_ref(), "line1\nline2\ttabA\u{1F600}");
}

#[semio_framework_async_macros::async_test]
async fn csv_style_doubled_quote_mode_decodes_doubled_delimiter_and_ignores_backslash() {
    let opts = LexOptions { strings: vec![StringMode { quote: '"', escape: StringEscape::Doubled }], comment: CommentDialect::default() };
    let tokens = lex_with(r#""a""b",backslash="\not-an-escape""#, &Limits::default(), false, &opts).expect("lex_with csv doubled");
    let texts: Vec<String> = tokens.iter().filter(|t| t.kind == TokenKind::Text).map(|t| t.text.as_str().to_string()).collect();
    assert_eq!(texts[0], "a\"b", "doubled `\"\"` decodes to one literal quote");
    assert_eq!(texts[1], r#"\not-an-escape"#, "backslash has no special meaning under Doubled");
}

#[semio_framework_async_macros::async_test]
async fn single_quote_strings_work_alongside_double_quote_xml_style() {
    let opts = LexOptions { strings: vec![StringMode { quote: '"', escape: StringEscape::Raw }, StringMode { quote: '\'', escape: StringEscape::Raw }], comment: CommentDialect::default() };
    let tokens = lex_with(r#"a="1" b='2'"#, &Limits::default(), false, &opts).expect("lex_with xml quotes");
    let texts: Vec<(TokenKind, String)> = tokens.iter().filter(|t| t.kind == TokenKind::Text).map(|t| (t.kind, t.text.as_str().to_string())).collect();
    assert_eq!(texts, vec![(TokenKind::Text, "1".to_string()), (TokenKind::Text, "2".to_string())]);
}

#[semio_framework_async_macros::async_test]
async fn step_style_single_quote_doubled_mode_decodes_doubled_apostrophe() {
    let opts = LexOptions { strings: vec![StringMode { quote: '\'', escape: StringEscape::Doubled }], comment: CommentDialect::default() };
    let tokens = lex_with(r#"'it''s a beam'"#, &Limits::default(), false, &opts).expect("lex_with step doubled");
    let text = tokens.iter().find(|t| t.kind == TokenKind::Text).expect("Text token");
    assert_eq!(text.text.as_str().as_ref(), "it's a beam");
}

// P2-M1 item 3: promoted single-char tokens `< > & $ ;`, non-colliding with arrow forms.
#[semio_framework_async_macros::async_test]
async fn promoted_tokens_lex_standalone_without_breaking_arrow_forms() {
    let tokens = lex("<tag a=\"1\" & $VAR ; b<-c d->e f--g", &Limits::default(), false).expect("lex");
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
    let tokens = lex("a > b", &Limits::default(), false).expect("lex");
    let significant: Vec<TokenKind> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| t.kind).collect();
    assert_eq!(significant, vec![TokenKind::Ident, TokenKind::Gt, TokenKind::Ident]);
}

// P2-M1 item 4: per-grammar comment dialect (custom line marker, disabled, block comment).
#[semio_framework_async_macros::async_test]
async fn comment_line_marker_is_configurable_and_disableable() {
    let slash_slash = LexOptions { strings: vec![], comment: CommentDialect { line: Some("//".to_string()), block: None } };
    let tokens = lex_with("a // not a hash comment\n# still data now b", &Limits::default(), true, &slash_slash).expect("lex_with //");
    let significant: Vec<(TokenKind, String)> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
    // "//..." is a comment (dropped as trivia, stops at the real newline); "#" is no longer
    // special under this dialect and falls through to "unknown character" (Error, forgiving),
    // then "still"/"data"/"now"/"b" lex as ordinary idents on the next line.
    assert_eq!(significant[0], (TokenKind::Ident, "a".to_string()));
    assert!(significant.iter().any(|(k, t)| *k == TokenKind::Error && t == "#"), "'#' must no longer be swallowed as a comment marker");
    assert!(significant.iter().any(|(_, t)| t == "still"), "text after the real newline must still lex, comment stopped at EOL");

    let none = LexOptions { strings: vec![], comment: CommentDialect { line: None, block: None } };
    let entity_like = lex_with("#123=WALL;", &Limits::default(), true, &none).expect("lex_with comment none");
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
    let tokens = lex_with(source, &Limits::default(), true, &step_opts).expect("lex_with step block comment");
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
    let tokens = lex("0. 10. 3.5 0..10 10..", &Limits::default(), false).expect("lex");
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
    let tokens = lex(".T. .F. .UNSPECIFIED. plain", &Limits::default(), false).expect("lex");
    let significant: Vec<(TokenKind, String)> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| (t.kind, t.text.as_str().to_string())).collect();
    assert_eq!(significant, vec![(TokenKind::DotEnum, ".T.".to_string()), (TokenKind::DotEnum, ".F.".to_string()), (TokenKind::DotEnum, ".UNSPECIFIED.".to_string()), (TokenKind::Ident, "plain".to_string()),]);
}

#[semio_framework_async_macros::async_test]
async fn lone_leading_dot_without_closing_dot_is_unaffected_by_dotenum() {
    // ".foo" (no closing dot) must NOT become DotEnum — falls through exactly like before
    // this feature existed: an "unknown character" '.' (forgiving -> Error) then an Ident.
    let tokens = lex(".foo", &Limits::default(), true).expect("lex forgiving");
    let significant: Vec<TokenKind> = tokens.iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).map(|t| t.kind).collect();
    assert_eq!(significant, vec![TokenKind::Error, TokenKind::Ident]);
}
//#endregion 🔖️P2M1Dialect
