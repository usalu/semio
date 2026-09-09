use super::*;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfInfo, PdfPage};

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
