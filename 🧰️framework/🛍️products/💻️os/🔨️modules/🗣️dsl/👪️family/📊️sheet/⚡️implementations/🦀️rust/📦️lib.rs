//! @emoji 📊️ `dsl_family_sheet` — the calc-sheet family notation kit, shared by fem2d/3d and the
//! 15 norm-family apps (`en1990`-`en1999`, `din4108`, `din16798`, `din18599`, `iso16757`,
//! `vdi3805`). `dsl_schema::Shape::Expr`/`ExprValue` deliberately "parses/prints the formula, never
//! evaluates it" (its own doc comment says so, naming this exact role: "resolved by the consuming
//! technology"). This crate is that consuming technology's evaluator, plus the self-verifying
//! `name = expr -> value` trace line the architecture plan calls for: `canonicalize_trace`
//! recomputes the expression against a variable environment and rewrites the stored value, so a
//! calc sheet's printed trace can never silently drift from what its own formula actually computes.
//!
//! Depends on `dsl_schema` (for `ExprValue`/`parse_expr_text`/`print_expr`) in addition to
//! `dsl_core` — unlike `dsl_notation`, which deliberately stays `dsl_core`-only because its edge
//! grammar is a `Shape::Wire` successor still pending migration, `Shape::Expr` has no such pending
//! migration to avoid colliding with: reusing it directly is exactly right.

use dsl_core::{lex, Limits, TextError, TextSpan, TokenKind};
use dsl_schema::{parse_expr_text, print_expr, ExprOp, ExprValue};
use std::collections::HashMap;

//#region 🔖️Evaluate
/// @emoji 🚫️ Why an expression failed to evaluate — never a panic, always a diagnosable value.
#[derive(Clone, Debug, PartialEq)]
pub enum EvalError {
    UnknownVariable(String),
    UnknownFunction(String, usize),
    DivisionByZero,
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::UnknownVariable(name) => write!(f, "unknown variable `{name}`"),
            EvalError::UnknownFunction(name, arity) => write!(f, "unknown function `{name}` with {arity} argument(s)"),
            EvalError::DivisionByZero => write!(f, "division by zero"),
        }
    }
}

/// @emoji 🧮️ Evaluates an `ExprValue` against a variable environment. Supports the small,
/// deliberately-closed function set a calc sheet actually needs (`min`, `max`, `abs`, `sqrt`) —
/// not a general call-out mechanism; an unrecognized name/arity is a diagnosed `EvalError`, never
/// a silent 0 or a panic.
pub fn evaluate(expr: &ExprValue, env: &HashMap<String, f64>) -> Result<f64, EvalError> {
    match expr {
        ExprValue::Num(v) => Ok(*v),
        ExprValue::Var(name) => env.get(name).copied().ok_or_else(|| EvalError::UnknownVariable(name.clone())),
        ExprValue::Neg(inner) => Ok(-evaluate(inner, env)?),
        ExprValue::Binary(op, l, r) => {
            let (lv, rv) = (evaluate(l, env)?, evaluate(r, env)?);
            match op {
                ExprOp::Add => Ok(lv + rv),
                ExprOp::Sub => Ok(lv - rv),
                ExprOp::Mul => Ok(lv * rv),
                ExprOp::Div => {
                    if rv == 0.0 {
                        Err(EvalError::DivisionByZero)
                    } else {
                        Ok(lv / rv)
                    }
                }
            }
        }
        ExprValue::Call(name, args) => {
            let values = args.iter().map(|a| evaluate(a, env)).collect::<Result<Vec<_>, _>>()?;
            match (name.as_str(), values.as_slice()) {
                ("min", [a, b]) => Ok(a.min(*b)),
                ("max", [a, b]) => Ok(a.max(*b)),
                ("abs", [a]) => Ok(a.abs()),
                ("sqrt", [a]) => Ok(a.sqrt()),
                _ => Err(EvalError::UnknownFunction(name.clone(), values.len())),
            }
        }
    }
}
//#endregion 🔖️Evaluate

//#region 🔖️Trace
/// @emoji 📈️ One self-verifying calc-sheet line: `name = expr -> value`. `value` is whatever was
/// last printed — `canonicalize_trace` is what re-derives it from `expr`/`env` and keeps it honest;
/// parsing alone doesn't check it (a stale/hand-edited trace parses fine as data — it's
/// `canonicalize_trace`'s job to catch drift, exactly like every other canonicalizer in this
/// engine catches non-canonical-but-parseable input).
#[derive(Clone, Debug, PartialEq)]
pub struct Trace {
    pub name: String,
    pub expr: ExprValue,
    pub value: f64,
}

fn find_arrow_after(tokens: &[dsl_core::SpannedToken], after: usize) -> Option<usize> {
    tokens.iter().position(|t| t.kind == TokenKind::Arrow).filter(|&i| i > after)
}

/// @emoji 🔌️ Parses one standalone trace line: `name = expr -> value`.
pub fn parse_trace_text(text: &str) -> Result<Trace, TextError> {
    let limits = Limits::default();
    let tokens: Vec<_> = lex(text, &limits, false)?.into_iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).collect();

    let name_token = tokens.first().filter(|t| t.kind == TokenKind::Ident).ok_or_else(|| TextError::new("expected a trace name", TextSpan::at(1, 1)))?;
    let name = name_token.text.as_str().to_string();
    let equals_index = 1;
    if tokens.get(equals_index).map(|t| t.kind) != Some(TokenKind::Equals) {
        return Err(TextError::new("expected `=` after the trace name", tokens.get(equals_index).map(|t| t.span).unwrap_or(TextSpan::at(1, 1))));
    }

    let arrow_index = find_arrow_after(&tokens, equals_index).ok_or_else(|| TextError::new("expected `->` closing the trace's expression", TextSpan::at(1, 1)))?;
    let expr_start = tokens[equals_index].byte_range.1 as usize;
    let expr_end = tokens[arrow_index].byte_range.0 as usize;
    let expr = parse_expr_text(text[expr_start..expr_end].trim())?;

    let value_token = tokens.get(arrow_index + 1).filter(|t| matches!(t.kind, TokenKind::Float | TokenKind::Int)).ok_or_else(|| {
        TextError::new("expected a number after `->`", tokens.get(arrow_index + 1).map(|t| t.span).unwrap_or(TextSpan::at(1, 1)))
    })?;
    let value: f64 = value_token.text.as_str().parse().map_err(|_| TextError::new(format!("not a valid number: {}", value_token.text.as_str()), value_token.span))?;

    if tokens.len() > arrow_index + 2 {
        return Err(TextError::new("unexpected trailing content after trace value", tokens[arrow_index + 2].span));
    }
    Ok(Trace { name, expr, value })
}

/// @emoji 🖨️ Canonical printer — prints `value` exactly as stored (does NOT recompute; that's
/// `canonicalize_trace`'s job, matching this engine's `parse`/`print`/`canonicalize` split).
pub fn print_trace(trace: &Trace) -> String {
    format!("{} = {} -> {}", trace.name, print_expr(&trace.expr), dsl_core::format_f64(trace.value))
}

/// @emoji ♻️ The self-verifying step: parses `text`, RE-EVALUATES its expression against `env`
/// (ignoring whatever value was written), and reprints with the freshly computed value. A hand-
/// edited or stale trace canonicalizes to the correct one; an unparseable expression or an
/// evaluation error (unknown variable, etc.) surfaces as `Err`, never silently keeps the old value.
pub fn canonicalize_trace(text: &str, env: &HashMap<String, f64>) -> Result<String, TextError> {
    let trace = parse_trace_text(text)?;
    let value = evaluate(&trace.expr, env).map_err(|e| TextError::new(e.to_string(), TextSpan::at(1, 1)))?;
    Ok(print_trace(&Trace { value, ..trace }))
}
//#endregion 🔖️Trace

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn env(pairs: &[(&str, f64)]) -> HashMap<String, f64> {
        pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    #[test]
    fn evaluates_a_load_combination_formula() {
        let expr = parse_expr_text("1.35*G + 1.5*Q").expect("parse_expr_text");
        let value = evaluate(&expr, &env(&[("G", 100.0), ("Q", 50.0)])).expect("evaluate");
        assert!((value - 210.0).abs() < 1e-9, "got {value}");
    }

    #[test]
    fn evaluates_min_max_abs_sqrt() {
        assert_eq!(evaluate(&parse_expr_text("min(3, 5)").unwrap(), &env(&[])), Ok(3.0));
        assert_eq!(evaluate(&parse_expr_text("max(3, 5)").unwrap(), &env(&[])), Ok(5.0));
        assert_eq!(evaluate(&parse_expr_text("abs(0-4)").unwrap(), &env(&[])), Ok(4.0));
        assert_eq!(evaluate(&parse_expr_text("sqrt(9)").unwrap(), &env(&[])), Ok(3.0));
    }

    #[test]
    fn unknown_variable_and_function_are_diagnosed_not_panicked() {
        assert_eq!(evaluate(&parse_expr_text("z").unwrap(), &env(&[])), Err(EvalError::UnknownVariable("z".to_string())));
        assert_eq!(evaluate(&parse_expr_text("frobnicate(1)").unwrap(), &env(&[])), Err(EvalError::UnknownFunction("frobnicate".to_string(), 1)));
    }

    #[test]
    fn division_by_zero_is_diagnosed() {
        assert_eq!(evaluate(&parse_expr_text("1/0").unwrap(), &env(&[])), Err(EvalError::DivisionByZero));
    }

    #[test]
    fn parses_and_prints_a_trace_line() {
        // `dsl_schema::print_expr`'s canonical form spaces every binary operator (`1.35 * G`, not
        // `1.35*G`) — parse accepts either spacing; only the printed/canonical form is fixed.
        let trace = parse_trace_text("uls = 1.35*G + 1.5*Q -> 210").expect("parse_trace_text");
        assert_eq!(trace.name, "uls");
        assert_eq!(trace.value, 210.0);
        assert_eq!(print_trace(&trace), "uls = 1.35 * G + 1.5 * Q -> 210");
    }

    #[test]
    fn canonicalize_trace_recomputes_a_stale_value() {
        let stale = "uls = 1.35*G + 1.5*Q -> 999";
        let canonical = canonicalize_trace(stale, &env(&[("G", 100.0), ("Q", 50.0)])).expect("canonicalize_trace");
        assert_eq!(canonical, "uls = 1.35 * G + 1.5 * Q -> 210");
    }

    #[test]
    fn canonicalize_trace_is_idempotent_once_correct() {
        let correct = "uls = 1.35 * G + 1.5 * Q -> 210";
        let canonical = canonicalize_trace(correct, &env(&[("G", 100.0), ("Q", 50.0)])).expect("canonicalize_trace");
        assert_eq!(canonical, correct);
    }

    #[test]
    fn canonicalize_trace_surfaces_an_unknown_variable_as_an_error() {
        let err = canonicalize_trace("uls = 1.35*G -> 135", &env(&[])).unwrap_err();
        assert!(err.message.contains("unknown variable"), "unexpected message: {}", err.message);
    }

    /// @emoji 📖️ The fragment's `.grammar` file must at least parse under `dsl_grammar`'s parser.
    #[test]
    fn grammar_file_is_syntactically_valid() {
        let source = include_str!("../../📖️family-sheet.grammar.semio");
        let grammar = dsl_grammar::parse_grammar(source).expect("family-sheet.grammar must parse");
        assert_eq!(grammar.id, "family-sheet");
    }

    #[test]
    fn round_trip_matrix() {
        let sources = vec!["uls = 1.35 * G + 1.5 * Q -> 210", "check = N-Ed / N-c-Rd -> 0.28", "simple = 5 -> 5"];
        for source in sources {
            let trace = parse_trace_text(source).unwrap_or_else(|e| panic!("parse of {source:?} failed: {e:?}"));
            let printed = print_trace(&trace);
            assert_eq!(printed, source, "canonical print should match already-canonical input for {source:?}");
        }
    }
}
//#endregion 🔖️Tests
