//! @emoji 📊️ `dsl_family_sheet` — the calc-sheet family notation kit, shared by fem2d/3d and the
//! 15 norm-family apps (`en1990`-`en1999`, `din4108`, `din16798`, `din18599`, `iso16757`,
//! `vdi3805`). `crate::os_dsl::schema::Shape::Expr`/`ExprValue` deliberately "parses/prints the formula, never
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

use crate::os_dsl::schema::{parse_expr_text, print_expr, ExprOp, ExprValue};
use crate::os_dsl::{lex, Limits, TextError, TextSpan, TokenKind};
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
// 🚫️async: E1 pure recursive evaluator, consumed by `Iterator::map` sync closure in the `Call` arm
// below — and confirmed sync-by-design by every existing call site, including test assertions that
// compare its `Result` directly with no `.await` in sight — see R9
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

async fn find_arrow_after(tokens: &[crate::os_dsl::SpannedToken], after: usize) -> Option<usize> {
    tokens.iter().position(|t| t.kind == TokenKind::Arrow).filter(|&i| i > after)
}

/// @emoji 🔌️ Parses one standalone trace line: `name = expr -> value`.
pub async fn parse_trace_text(text: &str) -> Result<Trace, TextError> {
    let limits = Limits::default();
    let tokens: Vec<_> = lex(text, &limits, false)?.into_iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).collect();

    let name_token = tokens.first().filter(|t| t.kind == TokenKind::Ident).ok_or_else(|| TextError::new("expected a trace name", TextSpan::at(1, 1)))?;
    let name = name_token.text.as_str().to_string();
    let equals_index = 1;
    if tokens.get(equals_index).map(|t| t.kind) != Some(TokenKind::Equals) {
        return Err(TextError::new("expected `=` after the trace name", tokens.get(equals_index).map_or(TextSpan::at(1, 1), |t| t.span)));
    }

    let arrow_index = find_arrow_after(&tokens, equals_index).await.ok_or_else(|| TextError::new("expected `->` closing the trace's expression", TextSpan::at(1, 1)))?;
    let expr_start = tokens[equals_index].byte_range.1 as usize;
    let expr_end = tokens[arrow_index].byte_range.0 as usize;
    let expr = parse_expr_text(text[expr_start..expr_end].trim())?;

    let value_token = tokens.get(arrow_index + 1).filter(|t| matches!(t.kind, TokenKind::Float | TokenKind::Int)).ok_or_else(|| TextError::new("expected a number after `->`", tokens.get(arrow_index + 1).map_or(TextSpan::at(1, 1), |t| t.span)))?;
    let value: f64 = value_token.text.as_str().parse().map_err(|_| TextError::new(format!("not a valid number: {}", value_token.text.as_str()), value_token.span))?;

    if tokens.len() > arrow_index + 2 {
        return Err(TextError::new("unexpected trailing content after trace value", tokens[arrow_index + 2].span));
    }
    Ok(Trace { name, expr, value })
}

/// @emoji 🖨️ Canonical printer — prints `value` exactly as stored (does NOT recompute; that's
/// `canonicalize_trace`'s job, matching this engine's `parse`/`print`/`canonicalize` split).
pub async fn print_trace(trace: &Trace) -> String {
    format!("{} = {} -> {}", trace.name, print_expr(&trace.expr), crate::os_dsl::format_f64(trace.value))
}

/// @emoji ♻️ The self-verifying step: parses `text`, RE-EVALUATES its expression against `env`
/// (ignoring whatever value was written), and reprints with the freshly computed value. A hand-
/// edited or stale trace canonicalizes to the correct one; an unparseable expression or an
/// evaluation error (unknown variable, etc.) surfaces as `Err`, never silently keeps the old value.
pub async fn canonicalize_trace(text: &str, env: &HashMap<String, f64>) -> Result<String, TextError> {
    let trace = parse_trace_text(text).await?;
    let value = evaluate(&trace.expr, env).map_err(|e| TextError::new(e.to_string(), TextSpan::at(1, 1)))?;
    Ok(print_trace(&Trace { value, ..trace }).await)
}
//#endregion 🔖️Trace

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
