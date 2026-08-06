//! @emoji 🌍️ `dsl_family_geo` — geo/crs family kit for spatial artifacts.

pub use crate::os_dsl::notation::{print_edge, EdgeLabel, EdgeLink, EdgeNode, EdgeValue};

use crate::os_dsl::core::{lex, Limits, TextError, TokenKind};

/// @emoji 📍 Parses `lon lat [alt]` tuples.
pub fn parse_point_text(text: &str) -> Result<(f64, f64, Option<f64>), TextError> {
    let limits = Limits::default();
    let nums: Vec<f64> = lex(text, &limits, false)?
        .into_iter()
        .filter(|t| matches!(t.kind, TokenKind::Float | TokenKind::Int))
        .map(|t| t.text.as_str().parse().map_err(|_| TextError::new("bad number", t.span.clone())))
        .collect::<Result<_, _>>()?;
    if nums.len() < 2 {
        return Err(TextError::new("expected at least lon lat", crate::os_dsl::core::TextSpan::at(1, 1)));
    }
    Ok((nums[0], nums[1], nums.get(2).copied()))
}
