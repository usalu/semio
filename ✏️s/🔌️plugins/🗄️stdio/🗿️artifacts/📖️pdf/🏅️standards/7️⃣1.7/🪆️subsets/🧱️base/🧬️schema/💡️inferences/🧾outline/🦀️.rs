//! 🧾 `outline` — one named inference: this PDF 1.7 document's own page/text structure.
//! `pageCount` is `pages.len()` verbatim; `wordCount` is a whitespace-split word count summed
//! over every resolved page's `text`; `title` mirrors the document's own `/Info` dictionary
//! `title` field (real, honestly optional — a source PDF may carry no `/Title`).

use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;

//#region 🔖️Outline
/// 🧾️ `Pdf` (1.7) document outline.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Pdf17Outline {
    pub page_count: u32,
    pub word_count: u32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl Pdf17Outline {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compute(snapshot: &PdfSnapshot) -> Self {
        let page_count = snapshot.pages.len() as u32;
        let word_count = snapshot.pages.iter().map(|p| p.text.split_whitespace().count() as u32).sum();
        Self { page_count, word_count, title: snapshot.info.title.clone() }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
