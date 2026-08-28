//! 🧾 `outline` — one named inference: this PDF 1.4 document's own page/text structure.
//! `pageCount` is `pages.len()` verbatim — the real page tree this standard's codec reads, not a
//! constant; `wordCount`/`charCount` are a whitespace-split word count and a character count
//! summed over every page's shown text.

use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
/// 🧾️ `Pdf` (1.4) document outline.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfOutline {
    pub page_count: u32,
    pub word_count: u32,
    pub char_count: u32,
}

impl PdfOutline {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compute(snapshot: &PdfSnapshot) -> Self {
        Self {
            page_count: snapshot.pages.len() as u32,
            word_count: snapshot.pages.iter().map(|page| page.text.split_whitespace().count() as u32).sum(),
            char_count: snapshot.pages.iter().map(|page| page.text.chars().count() as u32).sum(),
        }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PageDoc;

    #[semio_framework_async_macros::async_test]
    async fn counts_pages_words_and_chars_across_the_page_tree() {
        let snapshot = PdfSnapshot { schema: "stdio.pdf".into(), pages: vec![PageDoc { width: 612.0, height: 792.0, text: "hello world".into() }, PageDoc { width: 612.0, height: 792.0, text: "and a second page".into() }] };
        let outline = PdfOutline::compute(&snapshot);
        assert_eq!(outline.page_count, 2, "every page of the real page tree is counted, never a constant");
        assert_eq!(outline.word_count, 6);
        assert_eq!(outline.char_count, ("hello world".chars().count() + "and a second page".chars().count()) as u32);
    }

    #[semio_framework_async_macros::async_test]
    async fn outline_is_deterministic() {
        let snapshot = PdfSnapshot::default();
        assert_eq!(PdfOutline::compute(&snapshot), PdfOutline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
