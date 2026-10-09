use semio_framework_expression::{evaluate, evaluate_all_declared, infer, parse, print, AngleUnit, AreaUnit, BinaryOp, CompareOp, Expr, ExprError, Function, Kind, LengthUnit, UnaryOp, Value, VolumeUnit};
use serde_json::Value as Json;
use std::collections::BTreeMap;

fn fixture(text: &str) -> Json {
    serde_json::from_str(text).unwrap()
}

fn syntax() -> Json {
    fixture(include_str!("../../🧫️fixtures/🔤️syntax/🔣️.json"))
}

fn kinds() -> Json {
    fixture(include_str!("../../🧫️fixtures/📏️kinds/🔣️.json"))
}

fn evaluation() -> Json {
    fixture(include_str!("../../🧫️fixtures/🧮️evaluation/🔣️.json"))
}

fn parameters() -> Json {
    fixture(include_str!("../../🧫️fixtures/🕸️parameters/🔣️.json"))
}

fn text(v: &Json) -> &str {
    v.as_str().unwrap()
}

fn boxed(v: &Json, key: &str) -> Box<Expr> {
    Box::new(decode(&v[key]))
}

fn decode(v: &Json) -> Expr {
    let number = || v["value"].as_f64().unwrap();
    let unit = || text(&v["unit"]);
    match text(&v["op"]) {
        "number" => Expr::Number(number()),
        "length" => Expr::Length(number(), LengthUnit::from_symbol(unit()).unwrap()),
        "angle" => Expr::Angle(number(), AngleUnit::from_symbol(unit()).unwrap()),
        "area" => Expr::Area(number(), AreaUnit::from_symbol(unit()).unwrap()),
        "volume" => Expr::Volume(number(), VolumeUnit::from_symbol(unit()).unwrap()),
        "bool" => Expr::Bool(v["value"].as_bool().unwrap()),
        "text" => Expr::Text(text(&v["value"]).to_string()),
        "param" => Expr::Param(text(&v["name"]).to_string()),
        "unary" => Expr::Unary(if text(&v["operator"]) == "negate" { UnaryOp::Negate } else { UnaryOp::Not }, boxed(v, "operand")),
        "binary" => Expr::Binary(
            match text(&v["operator"]) {
                "add" => BinaryOp::Add,
                "subtract" => BinaryOp::Subtract,
                "multiply" => BinaryOp::Multiply,
                "divide" => BinaryOp::Divide,
                "power" => BinaryOp::Power,
                "min" => BinaryOp::Min,
                other => {
                    assert_eq!(other, "max");
                    BinaryOp::Max
                }
            },
            boxed(v, "left"),
            boxed(v, "right"),
        ),
        "compare" => Expr::Compare(
            match text(&v["operator"]) {
                "equal" => CompareOp::Equal,
                "not-equal" => CompareOp::NotEqual,
                "less" => CompareOp::Less,
                "less-equal" => CompareOp::LessEqual,
                "greater" => CompareOp::Greater,
                other => {
                    assert_eq!(other, "greater-equal");
                    CompareOp::GreaterEqual
                }
            },
            boxed(v, "left"),
            boxed(v, "right"),
        ),
        "and" => Expr::And(boxed(v, "left"), boxed(v, "right")),
        "or" => Expr::Or(boxed(v, "left"), boxed(v, "right")),
        "if" => Expr::If(boxed(v, "condition"), boxed(v, "then"), boxed(v, "otherwise")),
        "call" => Expr::Call(Function::from_name(text(&v["function"])).unwrap(), v["arguments"].as_array().unwrap().iter().map(decode).collect()),
        other => panic!("unknown op {other}"),
    }
}

fn value_of(v: &Json) -> Value {
    let kind = Kind::from_name(text(&v["kind"])).unwrap();
    match kind {
        Kind::Bool => Value::Bool(v["value"].as_bool().unwrap()),
        Kind::Text => Value::Text(text(&v["value"]).to_string()),
        _ => Value::of_kind(kind, v["value"].as_f64().unwrap()).unwrap(),
    }
}

fn env_of(v: &Json) -> BTreeMap<String, Value> {
    v.as_object().unwrap().iter().map(|(k, row)| (k.clone(), value_of(row))).collect()
}

fn same_value(got: &Value, want: &Value) -> bool {
    match (got.magnitude(), want.magnitude()) {
        (Some(g), Some(w)) => got.kind() == want.kind() && (g - w).abs() <= 1e-12 * w.abs().max(1.0),
        _ => got == want,
    }
}

fn path_of(v: &Json) -> Vec<usize> {
    v.as_array().unwrap().iter().map(|i| i.as_u64().unwrap() as usize).collect()
}

fn failure_matches(got: &ExprError, want: &Json) -> bool {
    got.code() == text(&want["code"]) && (want.get("path").is_none() || got.path == path_of(&want["path"]))
}

#[test]
fn syntax_cases_parse_print_and_round_trip() {
    let doc = syntax();
    let cases = doc["cases"].as_array().unwrap();
    assert!(cases.len() >= 300);
    for case in cases {
        let (name, source) = (text(&case["name"]), text(&case["source"]));
        let tree = decode(&case["ast"]);
        assert_eq!(parse(source).as_ref(), Ok(&tree), "{name}: parse {source}");
        assert_eq!(print(&tree), text(&case["canonical"]), "{name}: print");
        assert_eq!(parse(text(&case["canonical"])).as_ref(), Ok(&tree), "{name}: canonical re-parse");
    }
}

#[test]
fn syntax_errors_report_their_code_and_span() {
    let doc = syntax();
    let errors = doc["errors"].as_array().unwrap();
    assert!(errors.len() >= 25);
    for case in errors {
        let (name, source) = (text(&case["name"]), text(&case["source"]));
        let error = parse(source).expect_err(name);
        assert_eq!(error.code(), text(&case["code"]), "{name}");
        assert_eq!((error.span.start as u64, error.span.end as u64), (case["start"].as_u64().unwrap(), case["end"].as_u64().unwrap()), "{name}");
    }
}

#[test]
fn kind_cases_infer_the_same_kind_or_first_error() {
    let doc = kinds();
    let cases = doc["cases"].as_array().unwrap();
    assert!(cases.len() >= 300);
    let mut errors = 0;
    for case in cases {
        let name = text(&case["name"]);
        let params: BTreeMap<String, Kind> = case["params"].as_object().unwrap().iter().map(|(k, v)| (k.clone(), Kind::from_name(text(v)).unwrap())).collect();
        let got = infer(&parse(text(&case["source"])).unwrap(), &params);
        let want = &case["expected"];
        match (&got, want.get("error")) {
            (Ok(kind), None) => assert_eq!(kind.name(), text(&want["kind"]), "{name}"),
            (Err(e), Some(expected)) => {
                errors += 1;
                assert!(failure_matches(e, expected), "{name}: {e:?} vs {expected}");
            }
            _ => panic!("{name}: {got:?} vs {want}"),
        }
    }
    assert!(errors > 50);
}

#[test]
fn evaluation_cases_produce_the_same_value_or_error() {
    let doc = evaluation();
    let cases = doc["cases"].as_array().unwrap();
    assert!(cases.len() >= 400);
    let (mut values, mut errors) = (0, 0);
    for case in cases {
        let name = text(&case["name"]);
        let got = evaluate(&parse(text(&case["source"])).unwrap(), &env_of(&case["env"]));
        let want = &case["expected"];
        match (&got, want.get("error")) {
            (Ok(value), None) => {
                values += 1;
                assert!(same_value(value, &value_of(want)), "{name}: {value:?} vs {want}");
            }
            (Err(e), Some(expected)) => {
                errors += 1;
                assert!(failure_matches(e, expected), "{name}: {e:?} vs {expected}");
            }
            _ => panic!("{name}: {got:?} vs {want}"),
        }
    }
    assert!(values > 250 && errors > 60, "{values} values, {errors} errors");
}

#[test]
fn parameter_cases_resolve_in_the_same_order_with_the_same_errors() {
    let doc = parameters();
    let cases = doc["cases"].as_array().unwrap();
    assert!(cases.len() >= 50);
    let formulas = |v: &Json| -> BTreeMap<String, Expr> { v.as_object().unwrap().iter().map(|(k, row)| (k.clone(), parse(text(&row["source"])).unwrap())).collect() };
    for case in cases {
        let name = text(&case["name"]);
        let declared: BTreeMap<String, Kind> = case["declared"].as_object().unwrap().iter().map(|(k, v)| (k.clone(), Kind::from_name(text(v)).unwrap())).collect();
        let got = evaluate_all_declared(&formulas(&case["params"]), &formulas(&case["overrides"]), &declared);
        let want = &case["expected"];
        let order: Vec<&str> = want["order"].as_array().unwrap().iter().map(text).collect();
        assert_eq!(got.order.iter().map(String::as_str).collect::<Vec<_>>(), order, "{name}: order");
        let want_values = want["values"].as_object().unwrap();
        assert_eq!(got.values.keys().collect::<Vec<_>>(), want_values.keys().collect::<Vec<_>>(), "{name}: resolved names");
        for (key, value) in &got.values {
            assert!(same_value(value, &value_of(&want_values[key])), "{name}: {key}");
        }
        let want_errors = want["errors"].as_object().unwrap();
        assert_eq!(got.errors.keys().collect::<Vec<_>>(), want_errors.keys().collect::<Vec<_>>(), "{name}: failing names");
        for (key, error) in &got.errors {
            let expected = &want_errors[key];
            assert!(failure_matches(error, expected), "{name}: {key}: {error:?} vs {expected}");
            if let Some(members) = expected.get("members") {
                let want_members: Vec<&str> = members.as_array().unwrap().iter().map(text).collect();
                assert_eq!(error.kind, semio_framework_expression::ErrorKind::Cycle { members: want_members.iter().map(|s| s.to_string()).collect() }, "{name}: {key}");
            }
            if let Some(dependency) = expected.get("name") {
                assert_eq!(error.kind, semio_framework_expression::ErrorKind::FailedDependency { name: text(dependency).to_string() }, "{name}: {key}");
            }
            if let (Some(declared), Some(found)) = (expected.get("declared"), expected.get("found")) {
                let (d, f) = (Kind::from_name(text(declared)).unwrap(), Kind::from_name(text(found)).unwrap());
                assert_eq!(error.kind, semio_framework_expression::ErrorKind::DeclaredKind { declared: d, found: f }, "{name}: {key}");
            }
        }
    }
}
