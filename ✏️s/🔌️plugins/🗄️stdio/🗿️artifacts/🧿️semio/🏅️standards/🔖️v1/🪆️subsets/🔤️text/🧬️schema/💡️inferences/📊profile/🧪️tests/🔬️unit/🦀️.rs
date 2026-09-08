
use super::*;
use crate::standards::v1::subsets::text::schema::snapshot::{STDIO_SEMIOTEXT_DOCUMENT_SCHEMA, SemioTextMark, SemioTextMarkKind, SemioTextRun};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn populated() -> SemioTextSnapshot {
    SemioTextSnapshot {
        schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(),
        runs: vec![
            SemioTextRun { language: "en".into(), content: "Hello, world".into(), marks: vec![] },
            SemioTextRun { language: "en".into(), content: "again".into(), marks: vec![SemioTextMark { kind: SemioTextMarkKind::Bold, href: String::new() }] },
            SemioTextRun { language: "de".into(), content: "semio.tech".into(), marks: vec![SemioTextMark { kind: SemioTextMarkKind::Link, href: "https://semio.tech".into() }] },
            SemioTextRun { language: String::new(), content: "unspecified".into(), marks: vec![] },
        ],
    }
}

#[semio_framework_async_macros::async_test]
async fn censuses_words_chars_marks_and_distinct_languages() {
    let profile = compute_semio_text_profile(&populated());
    assert_eq!(profile.run_count, 4);
    assert_eq!(profile.word_count, 5); // "Hello, world"(2) + "again"(1) + "semio.tech"(1) + "unspecified"(1)
    assert_eq!(profile.mark_count, 2);
    assert_eq!(profile.languages, vec!["de".to_string(), "en".to_string()], "sorted, distinct, unspecified tag excluded");
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = populated();
    assert_eq!(compute_semio_text_profile(&snapshot), compute_semio_text_profile(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_semio_text_profile(&SemioTextSnapshot::default()), SemioTextProfile::default());
}
