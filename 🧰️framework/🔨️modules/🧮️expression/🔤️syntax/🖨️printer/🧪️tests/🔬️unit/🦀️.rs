use super::*;
use crate::syntax::parse;

fn canon(src: &str) -> String {
    print(&parse(src).unwrap())
}

#[test]
fn canonical_form_normalises_spelling() {
    assert_eq!(canon("2.4m+90mm"), "2.4 m + 90 mm");
    assert_eq!(canon("45°"), "45 deg");
    assert_eq!(canon("3 m²"), "3 m2");
    assert_eq!(canon("a × b ÷ c − d"), "a * b / c - d");
    assert_eq!(canon("a == b and b ≠ c"), "a = b and b != c");
    assert_eq!(canon("a ≤ b and c ≥ d"), "a <= b and c >= d");
    assert_eq!(canon("1.50"), "1.5");
    assert_eq!(canon("2.0"), "2");
    assert_eq!(canon("TRUE_VALUE"), "TRUE_VALUE");
}

#[test]
fn redundant_parentheses_are_dropped_and_needed_ones_kept() {
    assert_eq!(canon("((1 + 2)) * 3"), "(1 + 2) * 3");
    assert_eq!(canon("(1 * 2) + 3"), "1 * 2 + 3");
    assert_eq!(canon("1 - (2 - 3)"), "1 - (2 - 3)");
    assert_eq!(canon("(1 - 2) - 3"), "1 - 2 - 3");
    assert_eq!(canon("(a ^ b) ^ c"), "(a ^ b) ^ c");
    assert_eq!(canon("a ^ (b ^ c)"), "a ^ b ^ c");
    assert_eq!(canon("(-a) ^ b"), "(-a) ^ b");
    assert_eq!(canon("-(a ^ b)"), "-a ^ b");
    assert_eq!(canon("a ^ -b"), "a ^ (-b)");
    assert_eq!(canon("-(a * b)"), "-(a * b)");
    assert_eq!(canon("-(-a)"), "-(-a)");
    assert_eq!(canon("(-a) * b"), "-a * b");
    assert_eq!(canon("a - -b"), "a - -b");
}

#[test]
fn logic_parentheses() {
    assert_eq!(canon("(a or b) and c"), "(a or b) and c");
    assert_eq!(canon("a or (b and c)"), "a or b and c");
    assert_eq!(canon("a and (b and c)"), "a and (b and c)");
    assert_eq!(canon("(not a) = b"), "(not a) = b");
    assert_eq!(canon("not (a = b)"), "not a = b");
    assert_eq!(canon("not (a and b)"), "not (a and b)");
    assert_eq!(canon("(a < b) = (c < d)"), "(a < b) = (c < d)");
    assert_eq!(canon("(not a) and b"), "not a and b");
}

#[test]
fn conditionals_are_wrapped_only_as_operands() {
    assert_eq!(canon("if a then b else c"), "if a then b else c");
    assert_eq!(canon("(if a then b else c) + 1"), "(if a then b else c) + 1");
    assert_eq!(canon("1 + (if a then b else c)"), "1 + (if a then b else c)");
    assert_eq!(canon("if a then b else if c then d else e"), "if a then b else if c then d else e");
    assert_eq!(canon("sqrt(if a then 4 else 9)"), "sqrt(if a then 4 else 9)");
    assert_eq!(canon("min(if a then 1 else 2, 3)"), "min(if a then 1 else 2, 3)");
}

#[test]
fn calls_and_min_max() {
    assert_eq!(canon("atan2( y ,x )"), "atan2(y, x)");
    assert_eq!(canon("min(a,b,c)"), "min(min(a, b), c)");
    assert_eq!(canon("max(a, min(b, c))"), "max(a, min(b, c))");
}

#[test]
fn names_are_quoted_only_when_needed() {
    assert_eq!(canon("`width`"), "width");
    assert_eq!(canon("`frame width`"), "`frame width`");
    assert_eq!(canon("`2x`"), "`2x`");
    assert_eq!(canon("`if`"), "`if`");
    assert_eq!(canon("`true` and `not`"), "`true` and `not`");
    assert_eq!(canon("`a\\`b`"), "`a\\`b`");
    assert_eq!(canon("`Höhe_1`"), "Höhe_1");
    assert_eq!(canon("`m` * 2"), "m * 2");
}

#[test]
fn text_is_escaped() {
    assert_eq!(print(&Expr::Text("a\"b\\c\nd\te".into())), "\"a\\\"b\\\\c\\nd\\te\"");
    assert_eq!(canon("\"x\" = \"y\""), "\"x\" = \"y\"");
}

#[test]
fn extreme_numbers_use_exponents_and_round_trip() {
    for v in [0.0, 1.0, 0.5, 1e-7, 1.5e-9, 1e15, 1e16, 1.2345678901234567e20, 1e300, 0.000001, 123456789.125, f64::MIN_POSITIVE, 0.1 + 0.2] {
        let text = print(&Expr::Number(v));
        assert_eq!(parse(&text).unwrap(), Expr::Number(v), "{text}");
    }
    assert_eq!(print(&Expr::Number(1e16)), "1e16");
    assert_eq!(print(&Expr::Number(1e-7)), "1e-7");
    assert_eq!(print(&Expr::Number(0.000001)), "0.000001");
}

#[test]
fn spans_locate_every_node() {
    let e = parse("(w + 90 mm) * 2").unwrap();
    let (text, spans) = print_with_spans(&e);
    assert_eq!(text, "(w + 90 mm) * 2");
    let by_path: std::collections::BTreeMap<Vec<usize>, Span> = spans.iter().cloned().collect();
    let slice = |path: &[usize]| {
        let s = by_path[path];
        text.chars().skip(s.start).take(s.end - s.start).collect::<String>()
    };
    assert_eq!(slice(&[]), "(w + 90 mm) * 2");
    assert_eq!(slice(&[0]), "w + 90 mm");
    assert_eq!(slice(&[0, 0]), "w");
    assert_eq!(slice(&[0, 1]), "90 mm");
    assert_eq!(slice(&[1]), "2");
    assert_eq!(spans.len(), e.size());
    assert!(spans.windows(2).all(|w| w[0].0 < w[1].0));
}

#[test]
fn spans_count_characters_for_non_ascii_names() {
    let e = parse("Höhe + 1").unwrap();
    let (_, spans) = print_with_spans(&e);
    assert_eq!(spans.iter().find(|(p, _)| p == &vec![1]).unwrap().1, Span { start: 7, end: 8 });
}

#[test]
fn spans_of_calls_conditionals_and_unary() {
    let e = parse("if a then -b else sqrt(c)").unwrap();
    let (text, spans) = print_with_spans(&e);
    let by_path: std::collections::BTreeMap<Vec<usize>, Span> = spans.into_iter().collect();
    let slice = |path: &[usize]| {
        let s = by_path[path];
        text.chars().skip(s.start).take(s.end - s.start).collect::<String>()
    };
    assert_eq!(slice(&[1]), "-b");
    assert_eq!(slice(&[1, 0]), "b");
    assert_eq!(slice(&[2]), "sqrt(c)");
    assert_eq!(slice(&[2, 0]), "c");
}
