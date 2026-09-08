
use super::*;
use semio_s_artifact_stdio_txt::schema::snapshot::LineEnding;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(crate) fn sample_txt() -> TxtSnapshot {
    TxtSnapshot { schema: semio_s_artifact_stdio_txt::STDIO_TXT_DOCUMENT_SCHEMA.into(), lines: vec!["First line.".into(), String::new(), "Third line.".into()], trailing_newline: true, line_ending: LineEnding::Lf }
}

#[semio_framework_async_macros::async_test]
async fn each_line_becomes_a_paragraph_blank_lines_become_empty_paragraphs() {
    let semio = semio_framework_plugin::resolve_ready(SemioDocumentFromTxt::deserialize(&sample_txt())).expect("deserialize");
    assert_eq!(semio.blocks.len(), 3);
    assert!(matches!(&semio.blocks[0], DocBlock::Paragraph { runs, .. } if runs[0].text == "First line."));
    assert!(matches!(&semio.blocks[1], DocBlock::Paragraph { runs, .. } if runs.is_empty()));
    assert!(matches!(&semio.blocks[2], DocBlock::Paragraph { runs, .. } if runs[0].text == "Third line."));
    assert!(semio.styles.is_empty() && semio.images.is_empty());
}
