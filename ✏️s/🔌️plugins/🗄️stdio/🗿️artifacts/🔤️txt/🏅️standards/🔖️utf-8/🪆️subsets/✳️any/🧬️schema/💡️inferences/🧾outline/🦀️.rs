//! 🧾 `outline` — one named inference: this text document's own structure. `lineCount` is
//! `lines.len()` verbatim; `wordCount` is a whitespace-split word count over every line;
//! `charCount` is the total character count of every line's content (line-ending bytes not
//! included — those live in `line_ending`/`trailing_newline`, not the content itself).

use crate::TxtSnapshot;

//#region 🔖️Outline
/// 🧾️ `Txt` document outline.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TxtOutline {
    pub line_count: u32,
    pub word_count: u32,
    pub char_count: u32,
}

impl TxtOutline {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compute(snapshot: &TxtSnapshot) -> Self {
        let line_count = snapshot.lines.len() as u32;
        let word_count = snapshot.lines.iter().map(|line| line.split_whitespace().count() as u32).sum();
        let char_count = snapshot.lines.iter().map(|line| line.chars().count() as u32).sum();
        Self { line_count, word_count, char_count }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
