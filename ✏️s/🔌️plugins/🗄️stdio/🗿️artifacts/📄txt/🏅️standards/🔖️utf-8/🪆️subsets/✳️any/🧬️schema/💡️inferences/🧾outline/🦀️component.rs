//! 🧾 `outline` — one named inference: this text document's own structure. `lineCount` is
//! `lines.len()` verbatim; `wordCount` is a whitespace-split word count over every line;
//! `charCount` is the total character count of every line's content (line-ending bytes not
//! included — those live in `line_ending`/`trailing_newline`, not the content itself).

use crate::artifacts::txt::TxtSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
/// 🧾️ `Txt` document outline.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxtOutline {
    pub line_count: u32,
    pub word_count: u32,
    pub char_count: u32,
}

impl TxtOutline {
    pub async fn compute(snapshot: &TxtSnapshot) -> Self {
        let line_count = snapshot.lines.len() as u32;
        let word_count = snapshot.lines.iter().map(|line| line.split_whitespace().count() as u32).sum();
        let char_count = snapshot.lines.iter().map(|line| line.chars().count() as u32).sum();
        Self { line_count, word_count, char_count }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn counts_lines_words_and_chars() {
        let snapshot = TxtSnapshot { schema: "stdio.txt".into(), lines: vec!["hello world".into(), "one two three".into()], trailing_newline: true, line_ending: Default::default() };
        let outline = TxtOutline::compute(&snapshot);
        assert_eq!(outline.line_count, 2);
        assert_eq!(outline.word_count, 5);
        assert_eq!(outline.char_count, "hello world".chars().count() as u32 + "one two three".chars().count() as u32);
    }

    #[semio_framework_async_macros::async_test]
    async fn outline_is_deterministic() {
        let snapshot = TxtSnapshot::default();
        assert_eq!(TxtOutline::compute(&snapshot), TxtOutline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
