//! 🧾 `outline` — one named inference: this PDF 1.7 document's own page/text structure.
//! `pageCount` is `pages.len()` verbatim; `wordCount` is a whitespace-split word count summed
//! over every resolved page's `text`; `title` mirrors the document's own `/Info` dictionary
//! `title` field (real, honestly optional — a source PDF may carry no `/Title`).

use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
/// 🧾️ `Pdf` (1.7) document outline.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pdf17Outline {
    pub page_count: u32,
    pub word_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl Pdf17Outline {
    pub async fn compute(snapshot: &PdfSnapshot) -> Self {
        let page_count = snapshot.pages.len() as u32;
        let word_count = snapshot.pages.iter().map(|p| p.text.split_whitespace().count() as u32).sum();
        Self { page_count, word_count, title: snapshot.info.title.clone() }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{PdfInfo, PdfPage};

    #[semio_framework_async_macros::async_test]
    async fn counts_pages_and_words_and_carries_title() {
        let snapshot = PdfSnapshot {
            schema: "stdio.pdf.1.7".into(),
            declared_version: "1.7".into(),
            pages: vec![PdfPage::new(612.0, 792.0), {
                let mut p = PdfPage::new(612.0, 792.0);
                p.text = "hello world".into();
                p
            }],
            info: PdfInfo { title: Some("My Document".into()), ..Default::default() },
            objects: vec![],
            trailer: vec![],
        };
        let outline = Pdf17Outline::compute(&snapshot);
        assert_eq!(outline.page_count, 2);
        assert_eq!(outline.word_count, 2);
        assert_eq!(outline.title, Some("My Document".to_string()));
    }

    #[semio_framework_async_macros::async_test]
    async fn outline_is_deterministic() {
        let snapshot = PdfSnapshot::default();
        assert_eq!(Pdf17Outline::compute(&snapshot), Pdf17Outline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
