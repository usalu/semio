use super::*;

async fn env(pairs: &[(&str, f64)]) -> HashMap<String, f64> {
    pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
}

#[semio_framework_async_macros::async_test]
async fn evaluates_a_load_combination_formula() {
    let expr = parse_expr_text("1.35*G + 1.5*Q").expect("parse_expr_text");
    let value = evaluate(&expr, &env(&[("G", 100.0), ("Q", 50.0)]).await).expect("evaluate");
    assert!((value - 210.0).abs() < 1e-9, "got {value}");
}

#[semio_framework_async_macros::async_test]
async fn evaluates_min_max_abs_sqrt() {
    assert_eq!(evaluate(&parse_expr_text("min(3, 5)").unwrap(), &env(&[]).await), Ok(3.0));
    assert_eq!(evaluate(&parse_expr_text("max(3, 5)").unwrap(), &env(&[]).await), Ok(5.0));
    assert_eq!(evaluate(&parse_expr_text("abs(0-4)").unwrap(), &env(&[]).await), Ok(4.0));
    assert_eq!(evaluate(&parse_expr_text("sqrt(9)").unwrap(), &env(&[]).await), Ok(3.0));
}

#[semio_framework_async_macros::async_test]
async fn unknown_variable_and_function_are_diagnosed_not_panicked() {
    assert_eq!(evaluate(&parse_expr_text("z").unwrap(), &env(&[]).await), Err(EvalError::UnknownVariable("z".to_string())));
    assert_eq!(evaluate(&parse_expr_text("frobnicate(1)").unwrap(), &env(&[]).await), Err(EvalError::UnknownFunction("frobnicate".to_string(), 1)));
}

#[semio_framework_async_macros::async_test]
async fn division_by_zero_is_diagnosed() {
    assert_eq!(evaluate(&parse_expr_text("1/0").unwrap(), &env(&[]).await), Err(EvalError::DivisionByZero));
}

#[semio_framework_async_macros::async_test]
async fn parses_and_prints_a_trace_line() {
    // `crate::os_dsl::schema::print_expr`'s canonical form spaces every binary operator (`1.35 * G`, not
    // `1.35*G`) — parse accepts either spacing; only the printed/canonical form is fixed.
    let trace = parse_trace_text("uls = 1.35*G + 1.5*Q -> 210").await.expect("parse_trace_text");
    assert_eq!(trace.name, "uls");
    assert_eq!(trace.value, 210.0);
    assert_eq!(print_trace(&trace).await, "uls = 1.35 * G + 1.5 * Q -> 210");
}

#[semio_framework_async_macros::async_test]
async fn canonicalize_trace_recomputes_a_stale_value() {
    let stale = "uls = 1.35*G + 1.5*Q -> 999";
    let canonical = canonicalize_trace(stale, &env(&[("G", 100.0), ("Q", 50.0)]).await).await.expect("canonicalize_trace");
    assert_eq!(canonical, "uls = 1.35 * G + 1.5 * Q -> 210");
}

#[semio_framework_async_macros::async_test]
async fn canonicalize_trace_is_idempotent_once_correct() {
    let correct = "uls = 1.35 * G + 1.5 * Q -> 210";
    let canonical = canonicalize_trace(correct, &env(&[("G", 100.0), ("Q", 50.0)]).await).await.expect("canonicalize_trace");
    assert_eq!(canonical, correct);
}

#[semio_framework_async_macros::async_test]
async fn canonicalize_trace_surfaces_an_unknown_variable_as_an_error() {
    let err = canonicalize_trace("uls = 1.35*G -> 135", &env(&[]).await).await.unwrap_err();
    assert!(err.message.contains("unknown variable"), "unexpected message: {}", err.message);
}

/// @emoji 📖️ The fragment's `.grammar` file must at least parse under `dsl_grammar`'s parser.
#[semio_framework_async_macros::async_test]
async fn grammar_file_is_syntactically_valid() {
    let source = include_str!("../../📖️.grammar.semio");
    let grammar = crate::os_dsl::grammar::parse_grammar(source).expect("family-sheet.grammar must parse");
    assert_eq!(grammar.id, "family-sheet");
    assert!(grammar.productions.len() > 10, "family-sheet should cover qty, assign, expr, and eng-record");
}

#[semio_framework_async_macros::async_test]
async fn round_trip_matrix() {
    let sources = vec!["uls = 1.35 * G + 1.5 * Q -> 210", "check = N-Ed / N-c-Rd -> 0.28", "simple = 5 -> 5"];
    for source in sources {
        let trace = parse_trace_text(source).await.unwrap_or_else(|e| panic!("parse of {source:?} failed: {e:?}"));
        let printed = print_trace(&trace);
        assert_eq!(printed.await, source, "canonical print should match already-canonical input for {source:?}");
    }
}
