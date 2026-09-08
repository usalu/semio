
use super::*;

fn round_trips(text: &str) -> MathNode {
    let parsed = parse_formula(text).unwrap_or_else(|e| panic!("parse of {text:?} failed: {e}"));
    let printed = print(&parsed);
    let reparsed = parse_formula(&printed).unwrap_or_else(|e| panic!("reparse of canonical {printed:?} (from {text:?}) failed: {e}"));
    assert_eq!(reparsed, parsed, "round trip mismatch for {text:?} -> {printed:?}");
    let canonical_twice = canonicalize(&printed).expect("canonicalize");
    assert_eq!(canonical_twice, printed, "canonicalize is not idempotent for {printed:?}");
    parsed
}

#[test]
fn parses_bare_symbol_and_number() {
    assert_eq!(round_trips("x"), MathNode::Symbol("x".to_string()));
    assert_eq!(round_trips("42"), MathNode::Number("42".to_string()));
    assert_eq!(round_trips("3.14"), MathNode::Number("3.14".to_string()));
    assert_eq!(round_trips("alpha"), MathNode::Symbol("alpha".to_string()));
    assert_eq!(round_trips("α"), MathNode::Symbol("α".to_string()));
}

#[test]
fn parses_superscript_and_subscript() {
    assert_eq!(round_trips("x^2"), MathNode::Sup(Box::new(MathNode::Symbol("x".to_string())), Box::new(MathNode::Number("2".to_string()))));
    assert_eq!(round_trips("x_1"), MathNode::Sub(Box::new(MathNode::Symbol("x".to_string())), Box::new(MathNode::Number("1".to_string()))));
    let combined = round_trips("x_i^2");
    assert_eq!(combined, MathNode::Sup(Box::new(MathNode::Sub(Box::new(MathNode::Symbol("x".to_string())), Box::new(MathNode::Symbol("i".to_string())))), Box::new(MathNode::Number("2".to_string()))));
}

#[test]
fn subscript_does_not_glue_into_the_preceding_ident() {
    // Regression guard for the exact bug this crate's pre-scan lexer exists to avoid:
    // `os_dsl::lex` alone would swallow `_1` into one `Ident("x_1")`.
    let node = parse_formula("x_1").expect("parse");
    assert!(matches!(node, MathNode::Sub(..)), "expected Sub(x, 1), got {node:?}");
}

#[test]
fn parses_braced_group_exponent() {
    let node = round_trips("x^{n+1}");
    match node {
        MathNode::Sup(base, exponent) => {
            assert_eq!(*base, MathNode::Symbol("x".to_string()));
            assert!(matches!(*exponent, MathNode::Group(_)));
        }
        other => panic!("expected Sup, got {other:?}"),
    }
}

#[test]
fn parses_fraction_and_root_calls() {
    let frac = round_trips("frac(a, b)");
    assert_eq!(frac, MathNode::Call("frac".to_string(), vec![vec![MathNode::Symbol("a".to_string()), MathNode::Symbol("b".to_string())]]));
    let root = round_trips("root(3, x)");
    assert_eq!(root, MathNode::Call("root".to_string(), vec![vec![MathNode::Number("3".to_string()), MathNode::Symbol("x".to_string())]]));
    round_trips("sqrt(x)");
    round_trips("hat(x)");
}

#[test]
fn parses_matrix_rows_and_cells() {
    let mat = round_trips("mat(1, 2; 3, 4)");
    assert_eq!(mat, MathNode::Call("mat".to_string(), vec![vec![MathNode::Number("1".to_string()), MathNode::Number("2".to_string())], vec![MathNode::Number("3".to_string()), MathNode::Number("4".to_string())],]));
}

#[test]
fn parses_cases_with_text_and_relation_cells() {
    let node = round_trips("cases(x, \"if\" x > 0; 0, \"else\")");
    match node {
        MathNode::Call(name, rows) => {
            assert_eq!(name, "cases");
            assert_eq!(rows.len(), 2);
            assert_eq!(rows[0][0], MathNode::Symbol("x".to_string()));
            // `"if" x > 0` parses as `BinOp(Gt, Sequence(["if", x]), 0)` — the juxtaposed text
            // and symbol form the relation's left-hand side, not the whole cell.
            match &rows[0][1] {
                MathNode::BinOp(BinOp::Gt, lhs, rhs) => {
                    assert!(matches!(**lhs, MathNode::Sequence(_)), "expected \"if\" x to be a Sequence, got {lhs:?}");
                    assert_eq!(**rhs, MathNode::Number("0".to_string()));
                }
                other => panic!("expected BinOp(Gt, ..), got {other:?}"),
            }
            assert_eq!(rows[1], vec![MathNode::Number("0".to_string()), MathNode::Text("else".to_string())]);
        }
        other => panic!("expected Call, got {other:?}"),
    }
}

#[test]
fn parses_emoji_shortcode() {
    assert_eq!(round_trips(":rocket:"), MathNode::Emoji("rocket".to_string()));
}

#[test]
fn parses_stretchy_parens_and_brackets_distinct_from_calls() {
    let paren = round_trips("(x + y)");
    assert!(matches!(paren, MathNode::Paren('(', _)));
    let bracket = round_trips("[x + y]");
    assert!(matches!(bracket, MathNode::Paren('[', _)));
    // Same characters, different meaning when directly preceded by an Ident: a call.
    let call = round_trips("frac(x, y)");
    assert!(matches!(call, MathNode::Call(..)));
}

#[test]
fn parses_relational_and_arrow_operators() {
    assert_eq!(round_trips("x = y"), MathNode::BinOp(BinOp::Eq, Box::new(MathNode::Symbol("x".to_string())), Box::new(MathNode::Symbol("y".to_string()))));
    assert!(matches!(parse_formula("x != y").expect("parse"), MathNode::BinOp(BinOp::Ne, ..)));
    assert!(matches!(parse_formula("x <= y").expect("parse"), MathNode::BinOp(BinOp::Le, ..)));
    assert!(matches!(parse_formula("x >= y").expect("parse"), MathNode::BinOp(BinOp::Ge, ..)));
    assert!(matches!(parse_formula("x < y").expect("parse"), MathNode::BinOp(BinOp::Lt, ..)));
    assert!(matches!(parse_formula("x > y").expect("parse"), MathNode::BinOp(BinOp::Gt, ..)));
    let lim = round_trips("lim_{x -> 0}");
    assert!(matches!(lim, MathNode::Sub(..)));
}

#[test]
fn parses_sum_with_limits() {
    let node = round_trips("sum_{i=1}^{n} x_i");
    assert!(matches!(node, MathNode::Sequence(_)), "expected a Sequence joining the sum and x_i, got {node:?}");
}

#[test]
fn distinguishes_explicit_star_from_bare_juxtaposition() {
    let implicit = parse_formula("2x").expect("parse");
    match implicit {
        MathNode::Sequence(items) => assert!(!items[1].dot, "bare juxtaposition must not set dot"),
        other => panic!("expected Sequence, got {other:?}"),
    }
    let explicit = parse_formula("2 * x").expect("parse");
    match explicit {
        MathNode::Sequence(items) => assert!(items[1].dot, "explicit `*` must set dot"),
        other => panic!("expected Sequence, got {other:?}"),
    }
    round_trips("2x");
    round_trips("2 * x");
}

#[test]
fn accents_and_stretchy_delimiters_round_trip() {
    round_trips("hat(x)");
    round_trips("bar(x)");
    round_trips("vec(x)");
    round_trips("dot(x)");
    round_trips("ddot(x)");
    round_trips("tilde(x)");
    round_trips("abs(x)");
    round_trips("norm(x)");
    round_trips("brace(x)");
}

#[test]
fn bare_bang_is_a_lex_error_not_a_silent_accept() {
    let err = parse_formula("x ! y").expect_err("bare `!` must not lex");
    assert!(err.message.contains("!="), "unexpected message: {}", err.message);
}

#[test]
fn unclosed_paren_is_an_error() {
    assert!(parse_formula("(x + y").is_err());
    assert!(parse_formula("frac(a, b").is_err());
}

#[test]
fn trailing_garbage_is_an_error() {
    assert!(parse_formula("x y )").is_err());
}

/// @emoji 🪞️ This crate's own normative `.grammar` file parses under `dsl_grammar`'s parser and
/// round-trips — the self-conformance proof for the *format* of the spec (not a recognizer
/// check against real math text, which `os_dsl::grammar::Recognizer` cannot do here since it
/// matches `os_dsl::lex` tokens directly and has no visibility into this crate's own
/// pre-scanned `_ ; < > !` extras — a real, documented gap, not a silent approximation).
#[test]
fn math_grammar_parses_under_dsl_grammar() {
    let source = include_str!("../../📖️.grammar.semio");
    let parsed = crate::os_dsl::grammar::parse_grammar(source).expect("📖️.grammar.semio must parse under dsl_grammar's own parser");
    assert_eq!(parsed.id, "math");
    assert_eq!(parsed.start, "formula");
    let printed = crate::os_dsl::grammar::print_grammar(&parsed);
    let reparsed = crate::os_dsl::grammar::parse_grammar(&printed).expect("canonical print of 📖️.grammar.semio must reparse");
    assert_eq!(reparsed, parsed);
}
