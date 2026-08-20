//! 🧾 `outline` — one named inference: this single-page PDF 1.4 document's own text structure.
//! `pageCount` is always `1` (this subset's `PageDoc` models exactly one page — not fabricated,
//! it's the honest shape of the snapshot itself); `wordCount`/`charCount` are a whitespace-split
//! word count and character count over `page.text`.

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
        Self { page_count: 1, word_count: snapshot.page.text.split_whitespace().count() as u32, char_count: snapshot.page.text.chars().count() as u32 }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PageDoc;

    #[semio_framework_async_macros::async_test]
    async fn counts_words_and_chars_in_page_text() {
        let snapshot = PdfSnapshot { schema: "stdio.pdf".into(), page: PageDoc { width: 612.0, height: 792.0, text: "hello world".into() } };
        let outline = PdfOutline::compute(&snapshot);
        assert_eq!(outline.page_count, 1);
        assert_eq!(outline.word_count, 2);
        assert_eq!(outline.char_count, "hello world".chars().count() as u32);
    }

    #[semio_framework_async_macros::async_test]
    async fn outline_is_deterministic() {
        let snapshot = PdfSnapshot::default();
        assert_eq!(PdfOutline::compute(&snapshot), PdfOutline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
