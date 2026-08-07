//! 📍 Source spans for DSL diagnostics and tokens.

use serde::{Deserialize, Serialize};

//#region 🔖️Span
/// @emoji 📍️ 1-based line/column position with a length, covering a run of source text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextSpan {
    pub line: u32,
    pub column: u32,
    pub length: u32,
}

impl TextSpan {
    pub fn at(line: u32, column: u32) -> Self {
        Self { line, column, length: 0 }
    }

    pub fn with_length(line: u32, column: u32, length: u32) -> Self {
        Self { line, column, length }
    }
}
