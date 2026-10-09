use super::*;
use crate::tree::BinaryOp::*;
use crate::tree::CompareOp::*;

fn n(v: f64) -> Expr {
    Expr::Number(v)
}

fn p(name: &str) -> Expr {
    Expr::param(name)
}

fn b(op: BinaryOp, l: Expr, r: Expr) -> Expr {
    Expr::binary(op, l, r)
}

fn neg(e: Expr) -> Expr {
    Expr::unary(UnaryOp::Negate, e)
}

fn and(l: Expr, r: Expr) -> Expr {
    Expr::And(Box::new(l), Box::new(r))
}

fn or(l: Expr, r: Expr) -> Expr {
    Expr::Or(Box::new(l), Box::new(r))
}

fn err(src: &str) -> ParseError {
    parse(src).unwrap_err()
}

#[test]
fn additive_and_multiplicative_precedence_and_associativity() {
    assert_eq!(parse("1 + 2 * 3").unwrap(), b(Add, n(1.0), b(Multiply, n(2.0), n(3.0))));
    assert_eq!(parse("1 * 2 + 3").unwrap(), b(Add, b(Multiply, n(1.0), n(2.0)), n(3.0)));
    assert_eq!(parse("a - b - c").unwrap(), b(Subtract, b(Subtract, p("a"), p("b")), p("c")));
    assert_eq!(parse("a / b / c").unwrap(), b(Divide, b(Divide, p("a"), p("b")), p("c")));
    assert_eq!(parse("a - b + c").unwrap(), b(Add, b(Subtract, p("a"), p("b")), p("c")));
    assert_eq!(parse("(a + b) * c").unwrap(), b(Multiply, b(Add, p("a"), p("b")), p("c")));
}

#[test]
fn power_is_right_associative_and_binds_tighter_than_unary_minus() {
    assert_eq!(parse("a ^ b ^ c").unwrap(), b(Power, p("a"), b(Power, p("b"), p("c"))));
    assert_eq!(parse("-a ^ b").unwrap(), neg(b(Power, p("a"), p("b"))));
    assert_eq!(parse("(-a) ^ b").unwrap(), b(Power, neg(p("a")), p("b")));
    assert_eq!(parse("a ^ -b").unwrap(), b(Power, p("a"), neg(p("b"))));
    assert_eq!(parse("a ^ -b ^ c").unwrap(), b(Power, p("a"), neg(b(Power, p("b"), p("c")))));
    assert_eq!(parse("2 * a ^ 2").unwrap(), b(Multiply, n(2.0), b(Power, p("a"), n(2.0))));
}

#[test]
fn unary_minus_binds_tighter_than_multiplication() {
    assert_eq!(parse("-a * b").unwrap(), b(Multiply, neg(p("a")), p("b")));
    assert_eq!(parse("a - -b").unwrap(), b(Subtract, p("a"), neg(p("b"))));
    assert_eq!(parse("- - a").unwrap(), neg(neg(p("a"))));
    assert_eq!(parse("−a × b ÷ c").unwrap(), b(Divide, b(Multiply, neg(p("a")), p("b")), p("c")));
}

#[test]
fn logic_precedence() {
    assert_eq!(parse("a or b and c").unwrap(), or(p("a"), and(p("b"), p("c"))));
    assert_eq!(parse("a and b or c").unwrap(), or(and(p("a"), p("b")), p("c")));
    assert_eq!(parse("not a and b").unwrap(), and(Expr::unary(UnaryOp::Not, p("a")), p("b")));
    assert_eq!(parse("not a = b").unwrap(), Expr::unary(UnaryOp::Not, Expr::compare(Equal, p("a"), p("b"))));
    assert_eq!(parse("not not a").unwrap(), Expr::unary(UnaryOp::Not, Expr::unary(UnaryOp::Not, p("a"))));
    assert_eq!(parse("a < b + 1 and c").unwrap(), and(Expr::compare(Less, p("a"), b(Add, p("b"), n(1.0))), p("c")));
    assert_eq!(parse("a and b and c").unwrap(), and(and(p("a"), p("b")), p("c")));
}

#[test]
fn every_comparison_spelling() {
    for (src, op) in [("=", Equal), ("==", Equal), ("!=", NotEqual), ("≠", NotEqual), ("<", Less), ("<=", LessEqual), ("≤", LessEqual), (">", Greater), (">=", GreaterEqual), ("≥", GreaterEqual)] {
        assert_eq!(parse(&format!("a {src} b")).unwrap(), Expr::compare(op, p("a"), p("b")), "{src}");
    }
}

#[test]
fn comparisons_do_not_chain() {
    let e = err("a < b < c");
    assert_eq!(e.kind, ParseErrorKind::ChainedComparison);
    assert_eq!((e.span.start, e.span.end), (6, 7));
    assert_eq!(parse("(a < b) = (b < c)").unwrap().children().len(), 2);
}

#[test]
fn units_in_literals() {
    assert_eq!(parse("2.4 m").unwrap(), Expr::Length(2.4, LengthUnit::Metre));
    assert_eq!(parse("2.4m").unwrap(), Expr::Length(2.4, LengthUnit::Metre));
    assert_eq!(parse("90 mm").unwrap(), Expr::Length(90.0, LengthUnit::Millimetre));
    assert_eq!(parse("30 deg").unwrap(), Expr::Angle(30.0, AngleUnit::Degree));
    assert_eq!(parse("45°").unwrap(), Expr::Angle(45.0, AngleUnit::Degree));
    assert_eq!(parse("1.5 rad").unwrap(), Expr::Angle(1.5, AngleUnit::Radian));
    assert_eq!(parse("3 m²").unwrap(), Expr::Area(3.0, AreaUnit::SquareMetre));
    assert_eq!(parse("3 cm2").unwrap(), Expr::Area(3.0, AreaUnit::SquareCentimetre));
    assert_eq!(parse("2 l").unwrap(), Expr::Volume(2.0, VolumeUnit::Litre));
    assert_eq!(parse("2 m3").unwrap(), Expr::Volume(2.0, VolumeUnit::CubicMetre));
    assert_eq!(parse("2 ft + 3 in").unwrap(), b(Add, Expr::Length(2.0, LengthUnit::Foot), Expr::Length(3.0, LengthUnit::Inch)));
    assert_eq!(parse("2 m ^ 2").unwrap(), b(Power, Expr::Length(2.0, LengthUnit::Metre), n(2.0)));
}

#[test]
fn keywords_after_a_number_are_not_units() {
    let e = parse("if c then 1 else 2").unwrap();
    assert_eq!(e, Expr::conditional(p("c"), n(1.0), n(2.0)));
    assert_eq!(parse("1 and 2").unwrap(), and(n(1.0), n(2.0)));
    assert_eq!(parse("1 or 2").unwrap(), or(n(1.0), n(2.0)));
}

#[test]
fn conditionals_nest_and_extend_to_the_right() {
    assert_eq!(parse("if a then b else c + 1").unwrap(), Expr::conditional(p("a"), p("b"), b(Add, p("c"), n(1.0))));
    assert_eq!(parse("if a then b else if c then d else e").unwrap(), Expr::conditional(p("a"), p("b"), Expr::conditional(p("c"), p("d"), p("e"))));
    assert_eq!(parse("if a then if b then c else d else e").unwrap(), Expr::conditional(p("a"), Expr::conditional(p("b"), p("c"), p("d")), p("e")));
    assert_eq!(parse("1 + (if a then 2 else 3)").unwrap(), b(Add, n(1.0), Expr::conditional(p("a"), n(2.0), n(3.0))));
}

#[test]
fn calls_min_and_max() {
    assert_eq!(parse("sqrt(x)").unwrap(), Expr::Call(Function::Sqrt, vec![p("x")]));
    assert_eq!(parse("atan2(y, x + 1)").unwrap(), Expr::Call(Function::Atan2, vec![p("y"), b(Add, p("x"), n(1.0))]));
    assert_eq!(parse("min(a, b)").unwrap(), b(Min, p("a"), p("b")));
    assert_eq!(parse("max(a, b, c)").unwrap(), b(Max, b(Max, p("a"), p("b")), p("c")));
    assert_eq!(parse("min(a, max(b, c))").unwrap(), b(Min, p("a"), b(Max, p("b"), p("c"))));
    assert_eq!(parse("sqrt").unwrap(), p("sqrt"));
    assert_eq!(parse("min * 2").unwrap(), b(Multiply, p("min"), n(2.0)));
}

#[test]
fn names_booleans_and_text() {
    assert_eq!(parse("true and false").unwrap(), and(Expr::Bool(true), Expr::Bool(false)));
    assert_eq!(parse("`frame width` * 2").unwrap(), b(Multiply, p("frame width"), n(2.0)));
    assert_eq!(parse("Höhe").unwrap(), p("Höhe"));
    assert_eq!(parse("\"a b\"").unwrap(), Expr::Text("a b".into()));
    assert_eq!(parse("`if`").unwrap(), p("if"));
}

#[test]
fn whitespace_is_insignificant() {
    assert_eq!(parse("  1+2\n*\t3 ").unwrap(), parse("1 + 2 * 3").unwrap());
}

#[test]
fn unexpected_end_is_reported_at_the_end() {
    let e = err("1 +");
    assert_eq!(e.kind, ParseErrorKind::UnexpectedEnd { expected: "an expression" });
    assert_eq!((e.span.start, e.span.end), (3, 3));
    assert_eq!(err("").kind, ParseErrorKind::UnexpectedEnd { expected: "an expression" });
    assert_eq!(err("(1 + 2").kind, ParseErrorKind::UnexpectedEnd { expected: "`)`" });
    assert_eq!(err("if a then b").kind, ParseErrorKind::UnexpectedEnd { expected: "`else`" });
}

#[test]
fn unexpected_tokens_name_what_was_found() {
    let e = err("1 + * 2");
    assert_eq!(e.kind, ParseErrorKind::UnexpectedToken { found: "*".into(), expected: "an expression" });
    assert_eq!((e.span.start, e.span.end), (4, 5));
    assert_eq!(err("1 2").kind, ParseErrorKind::UnexpectedToken { found: "2".into(), expected: "an operator or the end of the expression" });
    assert_eq!(err("1 + 2)").span.start, 5);
    assert_eq!(err("and").kind, ParseErrorKind::UnexpectedToken { found: "and".into(), expected: "an expression" });
    assert_eq!(err("if a b").kind, ParseErrorKind::UnexpectedToken { found: "b".into(), expected: "`then`" });
    assert_eq!(err("sqrt(1 2)").kind, ParseErrorKind::UnexpectedToken { found: "2".into(), expected: "`,` or `)`" });
}

#[test]
fn unknown_units_and_functions() {
    let e = err("2 foo");
    assert_eq!(e.kind, ParseErrorKind::UnknownUnit { name: "foo".into() });
    assert_eq!((e.span.start, e.span.end), (2, 5));
    let e = err("1 + frob(2)");
    assert_eq!(e.kind, ParseErrorKind::UnknownFunction { name: "frob".into() });
    assert_eq!((e.span.start, e.span.end), (4, 8));
}

#[test]
fn argument_counts_are_checked_with_the_whole_call_span() {
    let e = err("sqrt(1, 2)");
    assert_eq!(e.kind, ParseErrorKind::WrongArgumentCount { function: "sqrt".into(), expected: 1, exact: true, found: 2 });
    assert_eq!((e.span.start, e.span.end), (0, 10));
    assert_eq!(err("atan2(1)").code(), "argument-count");
    assert_eq!(err("sin()").kind, ParseErrorKind::WrongArgumentCount { function: "sin".into(), expected: 1, exact: true, found: 0 });
    assert_eq!(err("min(1)").kind, ParseErrorKind::WrongArgumentCount { function: "min".into(), expected: 2, exact: false, found: 1 });
}

#[test]
fn render_underlines_the_span() {
    let e = err("1 + * 2");
    assert_eq!(e.render("1 + * 2"), "found `*` where an expression was expected\n1 + * 2\n    ^");
    let e = err("sqrt(1, 2)");
    assert!(e.render("sqrt(1, 2)").ends_with("sqrt(1, 2)\n^^^^^^^^^^"));
    let e = err("a +\nb *");
    assert_eq!(e.render("a +\nb *").lines().collect::<Vec<_>>()[1], "b *");
}

#[test]
fn deep_nesting_parses() {
    let src = format!("{}1{}", "(".repeat(300), ")".repeat(300));
    assert_eq!(parse(&src).unwrap(), n(1.0));
}
