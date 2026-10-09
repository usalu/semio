use super::*;
use crate::syntax::parse;

fn env() -> BTreeMap<String, Value> {
    [
        ("w", Value::Length(2.4)),
        ("h", Value::Length(0.9)),
        ("a", Value::Angle(std::f64::consts::FRAC_PI_2)),
        ("n", Value::Number(3.0)),
        ("zero", Value::Number(0.0)),
        ("f", Value::Bool(true)),
        ("s", Value::Text("oak".into())),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}

fn run(src: &str) -> Result<Value, ExprError> {
    evaluate(&parse(src).unwrap(), &env())
}

fn close(src: &str, want: Value) {
    let got = run(src).unwrap();
    assert_eq!(got.kind(), want.kind(), "{src}");
    let (g, w) = (got.magnitude().unwrap(), want.magnitude().unwrap());
    assert!((g - w).abs() <= 1e-12 * w.abs().max(1.0), "{src}: {g} vs {w}");
}

#[test]
fn literals_are_converted_to_si() {
    assert_eq!(run("2.4 m"), Ok(Value::Length(2.4)));
    assert_eq!(run("90 mm"), Ok(Value::Length(0.09)));
    assert_eq!(run("250 cm"), Ok(Value::Length(2.5)));
    assert_eq!(run("1 km"), Ok(Value::Length(1000.0)));
    assert_eq!(run("180 deg"), Ok(Value::Angle(std::f64::consts::PI)));
    assert_eq!(run("180°"), Ok(Value::Angle(std::f64::consts::PI)));
    assert_eq!(run("2 rad"), Ok(Value::Angle(2.0)));
    assert_eq!(run("10000 cm2"), Ok(Value::Area(1.0)));
    assert_eq!(run("1000 l"), Ok(Value::Volume(1.0)));
    assert_eq!(run("\"oak\""), Ok(Value::Text("oak".into())));
    assert_eq!(run("true"), Ok(Value::Bool(true)));
}

#[test]
fn arithmetic_mixes_units_through_si() {
    close("w + 90 mm", Value::Length(2.49));
    close("w - 40 cm", Value::Length(2.0));
    close("w * h", Value::Area(2.16));
    close("w * h * 2 m", Value::Volume(4.32));
    close("w / h", Value::Number(2.4 / 0.9));
    close("w * h / w", Value::Length(0.9));
    close("2 * w", Value::Length(4.8));
    close("-w", Value::Length(-2.4));
    close("w ^ 2", Value::Area(5.76));
    close("w ^ 3", Value::Volume(13.824));
    close("4 m2 ^ 0.5", Value::Length(2.0));
    close("2 ^ 10", Value::Number(1024.0));
    close("2 ^ -1", Value::Number(0.5));
    close("2 ^ 3 ^ 2", Value::Number(512.0));
    close("-2 ^ 2", Value::Number(-4.0));
    close("min(w, h)", Value::Length(0.9));
    close("max(w, h, 3 m)", Value::Length(3.0));
}

#[test]
fn precedence_is_the_usual_one() {
    close("1 + 2 * 3", Value::Number(7.0));
    close("(1 + 2) * 3", Value::Number(9.0));
    close("10 - 4 - 3", Value::Number(3.0));
    close("100 / 10 / 5", Value::Number(2.0));
    close("2 * 3 ^ 2", Value::Number(18.0));
    close("-3 * -2", Value::Number(6.0));
    close("1 - -1", Value::Number(2.0));
}

#[test]
fn functions_compute() {
    close("sqrt(16)", Value::Number(4.0));
    close("sqrt(4 m2)", Value::Length(2.0));
    close("abs(-2 m)", Value::Length(2.0));
    close("round(2.5)", Value::Number(3.0));
    close("round(-2.5)", Value::Number(-3.0));
    close("round(2.4)", Value::Number(2.0));
    close("floor(-2.5)", Value::Number(-3.0));
    close("ceil(2.1)", Value::Number(3.0));
    close("sin(a)", Value::Number(1.0));
    close("cos(60 deg)", Value::Number(0.5));
    close("tan(45 deg)", Value::Number(1.0));
    close("atan2(1 m, 1 m)", Value::Angle(std::f64::consts::FRAC_PI_4));
    close("atan2(1, -1)", Value::Angle(3.0 * std::f64::consts::FRAC_PI_4));
    close("round(w / 100 mm) * 100 mm", Value::Length(2.4));
}

#[test]
fn comparisons_are_tolerant_and_consistent() {
    assert_eq!(run("0.1 + 0.2 = 0.3"), Ok(Value::Bool(true)));
    assert_eq!(run("0.1 + 0.2 != 0.3"), Ok(Value::Bool(false)));
    assert_eq!(run("0.1 + 0.2 <= 0.3"), Ok(Value::Bool(true)));
    assert_eq!(run("0.1 + 0.2 >= 0.3"), Ok(Value::Bool(true)));
    assert_eq!(run("0.1 + 0.2 < 0.3"), Ok(Value::Bool(false)));
    assert_eq!(run("0.1 + 0.2 > 0.3"), Ok(Value::Bool(false)));
    assert_eq!(run("1 < 2"), Ok(Value::Bool(true)));
    assert_eq!(run("2 m >= 200 cm"), Ok(Value::Bool(true)));
    assert_eq!(run("1 m = 1.0000001 m"), Ok(Value::Bool(false)));
    assert_eq!(run("s = \"oak\""), Ok(Value::Bool(true)));
    assert_eq!(run("s != \"oak\""), Ok(Value::Bool(false)));
    assert_eq!(run("f = false"), Ok(Value::Bool(false)));
    assert!(approx_eq(1e9, 1e9 + 0.5));
    assert!(!approx_eq(1.0, 1.0 + 1e-8));
}

#[test]
fn logic_is_lazy_but_still_kind_checked() {
    assert_eq!(run("zero != 0 and 1 / zero > 2"), Ok(Value::Bool(false)));
    assert_eq!(run("zero = 0 or 1 / zero > 2"), Ok(Value::Bool(true)));
    assert_eq!(run("if zero = 0 then 1 else 1 / zero"), Ok(Value::Number(1.0)));
    assert_eq!(run("if zero != 0 then 1 / zero else 7"), Ok(Value::Number(7.0)));
    assert_eq!(run("not f"), Ok(Value::Bool(false)));
    assert_eq!(run("f and not f or f"), Ok(Value::Bool(true)));
    assert_eq!(run("if f then w else h"), Ok(Value::Length(2.4)));
    assert_eq!(run("zero = 0 or w + a").unwrap_err().code(), "mixed-kinds");
}

#[test]
fn runtime_failures_are_values_with_paths() {
    assert_eq!(run("1 / zero"), Err(ExprError::at(&[], ErrorKind::DivisionByZero)));
    assert_eq!(run("2 + 1 / (n - 3)").unwrap_err().path, vec![1]);
    assert_eq!(run("0 ^ -1").unwrap_err().code(), "division-by-zero");
    assert_eq!(run("(-8) ^ 0.5").unwrap_err().kind, ErrorKind::Domain { site: "^" });
    assert_eq!(run("sqrt(-1)").unwrap_err().kind, ErrorKind::Domain { site: "sqrt" });
    assert_eq!(run("10 ^ 400").unwrap_err().kind, ErrorKind::Overflow);
    assert_eq!(run("1e308 * 10").unwrap_err().kind, ErrorKind::Overflow);
    assert_eq!(run("missing + 1").unwrap_err().kind, ErrorKind::UnknownParam { name: "missing".into() });
}

#[test]
fn kind_errors_win_over_runtime_errors() {
    assert_eq!(run("(1 / zero) + w").unwrap_err().code(), "mixed-kinds");
}

#[test]
fn evaluation_is_a_pure_function_of_the_inputs() {
    let e = parse("w * h + sqrt(w ^ 2) / n - round(h * 10)").unwrap();
    let first = evaluate(&e, &env());
    for _ in 0..5 {
        assert_eq!(evaluate(&e, &env()), first);
    }
}

#[test]
fn nested_deep_expressions_evaluate() {
    let mut e = Expr::Number(1.0);
    for _ in 0..400 {
        e = Expr::binary(BinaryOp::Add, e, Expr::Number(1.0));
    }
    assert_eq!(evaluate(&e, &BTreeMap::new()), Ok(Value::Number(401.0)));
}
