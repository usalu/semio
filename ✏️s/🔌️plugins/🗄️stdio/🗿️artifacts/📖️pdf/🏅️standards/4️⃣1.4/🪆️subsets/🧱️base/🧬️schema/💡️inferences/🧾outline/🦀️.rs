//! 🧾 `outline` — one named inference: this PDF 1.4 document's own page/text structure.
//! `pageCount` is `pages.len()` verbatim — the real page tree this standard's codec reads, not a
//! constant; `wordCount`/`charCount` are a whitespace-split word count and a character count
//! summed over every page's shown text.

use crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;

//#region 🔖️Outline
/// 🧾️ `Pdf` (1.4) document outline.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
