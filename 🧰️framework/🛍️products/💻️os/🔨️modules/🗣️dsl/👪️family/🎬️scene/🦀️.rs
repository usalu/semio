//! 🎬️ `dsl_family_scene` — scene/layout family kit: layer stacks and shared edge notation.



use semio_framework_dsl::lex;
use semio_framework_diagnostic::Limits;
use semio_framework_diagnostic::TextError;
use semio_framework_value::ValueRefusalKind;
use semio_framework_dsl::TokenKind;

/// 📐️ Parses `id@x y [z]` layer placement literals.
pub async fn parse_layer_anchor_text(text: &str) -> Result<(String, f64, f64, Option<f64>), TextError> {
    let limits = Limits::default();
    let tokens: Vec<_> = lex(text, &limits, false)?.into_iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).collect();
    let id = tokens.first().ok_or_else(|| TextError::new(ValueRefusalKind::InvalidValue, "expected layer id", semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    if id.kind != TokenKind::Ident {
        return Err(TextError::new(ValueRefusalKind::InvalidValue, "expected layer id", id.span));
    }
    if tokens.get(1).map(|t| t.kind) != Some(TokenKind::At) {
        return Err(TextError::new(ValueRefusalKind::InvalidValue, "expected `@` after layer id", id.span));
    }
    let x = tokens.get(2).ok_or_else(|| TextError::new(ValueRefusalKind::InvalidValue, "expected x", id.span))?;
    let y = tokens.get(3).ok_or_else(|| TextError::new(ValueRefusalKind::InvalidValue, "expected y", id.span))?;
    if !matches!(x.kind, TokenKind::Float | TokenKind::Int) || !matches!(y.kind, TokenKind::Float | TokenKind::Int) {
        return Err(TextError::new(ValueRefusalKind::InvalidValue, "expected numeric x y", y.span));
    }
    let xf: f64 = x.text.as_str().parse().map_err(|_| TextError::new(ValueRefusalKind::InvalidValue, "bad x", x.span))?;
    let yf: f64 = y.text.as_str().parse().map_err(|_| TextError::new(ValueRefusalKind::InvalidValue, "bad y", y.span))?;
    let z = tokens.get(4).and_then(|t| if matches!(t.kind, TokenKind::Float | TokenKind::Int) { t.text.as_str().parse().ok() } else { None });
    Ok((id.text.as_str().to_string(), xf, yf, z))
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
