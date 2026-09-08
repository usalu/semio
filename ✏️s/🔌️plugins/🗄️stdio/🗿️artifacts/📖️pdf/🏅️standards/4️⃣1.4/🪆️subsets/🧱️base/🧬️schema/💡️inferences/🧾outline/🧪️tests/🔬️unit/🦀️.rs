
use super::*;
use crate::standards::v1_4::subsets::base::schema::snapshot::PageDoc;

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
