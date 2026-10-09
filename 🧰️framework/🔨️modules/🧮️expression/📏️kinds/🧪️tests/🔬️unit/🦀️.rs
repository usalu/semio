use super::*;
use crate::syntax::parse;

fn params() -> BTreeMap<String, Kind> {
    [("w", Kind::Length), ("h", Kind::Length), ("a", Kind::Angle), ("n", Kind::Number), ("f", Kind::Bool), ("s", Kind::Text), ("area", Kind::Area), ("vol", Kind::Volume)].into_iter().map(|(k, v)| (k.to_string(), v)).collect()
}

fn kind(src: &str) -> Result<Kind, ExprError> {
    infer(&parse(src).unwrap(), &params())
}

fn ok(src: &str, want: Kind) {
    assert_eq!(kind(src), Ok(want), "{src}");
}

fn err(src: &str) -> ExprError {
    kind(src).unwrap_err()
}

#[test]
fn literals_have_their_own_kind() {
    ok("2", Kind::Number);
    ok("2.4 m", Kind::Length);
    ok("30 deg", Kind::Angle);
    ok("45°", Kind::Angle);
    ok("2 m2", Kind::Area);
    ok("2 l", Kind::Volume);
    ok("true", Kind::Bool);
    ok("\"x\"", Kind::Text);
}

#[test]
fn addition_requires_equal_kinds() {
    ok("w + 90 mm", Kind::Length);
    ok("a - 10 deg", Kind::Angle);
    ok("n + 1", Kind::Number);
    ok("min(w, h)", Kind::Length);
    ok("max(area, 1 m2)", Kind::Area);
    let e = err("w + a");
    assert_eq!(e.kind, ErrorKind::MixedKinds { site: "+", left: Kind::Length, right: Kind::Angle });
    assert!(e.path.is_empty());
    assert_eq!(err("w + 1").code(), "mixed-kinds");
}

#[test]
fn multiplication_and_division_follow_dimensions() {
    ok("w * h", Kind::Area);
    ok("w * h * w", Kind::Volume);
    ok("w * area", Kind::Volume);
    ok("area * w", Kind::Volume);
    ok("2 * w", Kind::Length);
    ok("w * 2", Kind::Length);
    ok("a * 2", Kind::Angle);
    ok("w / h", Kind::Number);
    ok("area / w", Kind::Length);
    ok("vol / w", Kind::Area);
    ok("vol / area", Kind::Length);
    ok("w / 2", Kind::Length);
    ok("n / n", Kind::Number);
    assert_eq!(err("area * area").code(), "mixed-kinds");
    assert_eq!(err("w * a").code(), "mixed-kinds");
    assert_eq!(err("2 / w").code(), "mixed-kinds");
    assert_eq!(err("w / a").code(), "mixed-kinds");
}

#[test]
fn powers_need_a_literal_exponent_that_lands_on_a_kind() {
    ok("n ^ n", Kind::Number);
    ok("w ^ 1", Kind::Length);
    ok("w ^ 2", Kind::Area);
    ok("w ^ 3", Kind::Volume);
    ok("area ^ 0.5", Kind::Length);
    ok("area ^ 1", Kind::Area);
    ok("a ^ 1", Kind::Angle);
    assert_eq!(err("w ^ 4").kind, ErrorKind::Exponent { base: Kind::Length, exponent: Some(4.0) });
    assert_eq!(err("w ^ n").kind, ErrorKind::Exponent { base: Kind::Length, exponent: None });
    assert_eq!(err("w ^ -1").kind, ErrorKind::Exponent { base: Kind::Length, exponent: Some(-1.0) });
    assert_eq!(err("w ^ h").code(), "operand-kind");
    assert_eq!(err("f ^ 2").code(), "operand-kind");
}

#[test]
fn unary_operators_check_their_operand() {
    ok("-w", Kind::Length);
    ok("not f", Kind::Bool);
    assert_eq!(err("-f").kind, ErrorKind::OperandKind { site: "-", position: 0, expected: Kind::NUMERIC, found: Kind::Bool });
    assert_eq!(err("not n").kind, ErrorKind::OperandKind { site: "not", position: 0, expected: &[Kind::Bool], found: Kind::Number });
}

#[test]
fn comparisons_are_bool_and_ordering_is_numeric_only() {
    ok("w = h", Kind::Bool);
    ok("w < 2 m", Kind::Bool);
    ok("f = true", Kind::Bool);
    ok("s != \"a\"", Kind::Bool);
    assert_eq!(err("w < a").code(), "mixed-kinds");
    assert_eq!(err("w = 2").code(), "mixed-kinds");
    assert_eq!(err("f < true").kind, ErrorKind::OperandKind { site: "<", position: 0, expected: Kind::NUMERIC, found: Kind::Bool });
    assert_eq!(err("w < s").kind, ErrorKind::OperandKind { site: "<", position: 1, expected: Kind::NUMERIC, found: Kind::Text });
}

#[test]
fn logic_and_conditionals() {
    ok("f and w > h", Kind::Bool);
    ok("f or not f", Kind::Bool);
    ok("if f then w else h", Kind::Length);
    ok("if w > h then 1 else 2", Kind::Number);
    assert_eq!(err("n and f").kind, ErrorKind::OperandKind { site: "and", position: 0, expected: &[Kind::Bool], found: Kind::Number });
    assert_eq!(err("f or n").kind, ErrorKind::OperandKind { site: "or", position: 1, expected: &[Kind::Bool], found: Kind::Number });
    assert_eq!(err("if n then 1 else 2").kind, ErrorKind::OperandKind { site: "if", position: 0, expected: &[Kind::Bool], found: Kind::Number });
    assert_eq!(err("if f then w else a").kind, ErrorKind::Branches { then: Kind::Length, otherwise: Kind::Angle });
}

#[test]
fn functions_map_kinds() {
    ok("sqrt(n)", Kind::Number);
    ok("sqrt(area)", Kind::Length);
    ok("abs(w)", Kind::Length);
    ok("abs(-a)", Kind::Angle);
    ok("round(w / 10 mm)", Kind::Number);
    ok("floor(n)", Kind::Number);
    ok("ceil(w / h)", Kind::Number);
    ok("sin(a)", Kind::Number);
    ok("cos(30 deg)", Kind::Number);
    ok("tan(a / 2)", Kind::Number);
    ok("atan2(w, h)", Kind::Angle);
    ok("atan2(n, 2)", Kind::Angle);
    assert_eq!(err("sqrt(w)").kind, ErrorKind::OperandKind { site: "sqrt", position: 0, expected: &[Kind::Number, Kind::Area], found: Kind::Length });
    assert_eq!(err("sin(n)").kind, ErrorKind::OperandKind { site: "sin", position: 0, expected: &[Kind::Angle], found: Kind::Number });
    assert_eq!(err("round(w)").code(), "operand-kind");
    assert_eq!(err("abs(f)").code(), "operand-kind");
    assert_eq!(err("atan2(w, a)").code(), "mixed-kinds");
    assert_eq!(err("atan2(f, a)").kind, ErrorKind::OperandKind { site: "atan2", position: 0, expected: Kind::NUMERIC, found: Kind::Bool });
}

#[test]
fn programmatic_arity_errors_are_reported() {
    let e = Expr::Call(Function::Sqrt, vec![Expr::Number(1.0), Expr::Number(2.0)]);
    assert_eq!(infer(&e, &params()).unwrap_err().kind, ErrorKind::Arity { site: "sqrt", expected: 1, found: 2 });
}

#[test]
fn unknown_parameters_are_reported_at_their_node() {
    let e = err("w + (missing * 2)");
    assert_eq!(e.kind, ErrorKind::UnknownParam { name: "missing".into() });
    assert_eq!(e.path, vec![1, 0]);
}

#[test]
fn errors_are_found_in_post_order_and_point_at_the_innermost_node() {
    let e = err("(w + a) * (h + n)");
    assert_eq!(e.path, vec![0]);
    let e = err("if f then (w + a) else (h + n)");
    assert_eq!(e.path, vec![1]);
    let e = err("sqrt(w + a)");
    assert_eq!(e.path, vec![0]);
}

#[test]
fn kind_names_round_trip() {
    for k in Kind::ALL {
        assert_eq!(Kind::from_name(k.name()), Some(*k));
    }
    assert_eq!(Kind::from_name("string"), None);
    assert_eq!(Kind::NUMERIC.len(), 5);
}

#[test]
fn division_by_a_literal_zero_is_a_value_problem_not_a_kind_problem() {
    ok("1 / 0", Kind::Number);
    ok("w / 0", Kind::Length);
}
