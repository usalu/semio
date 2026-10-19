use crate::{schema::snapshot::MdBlock, schema::snapshot::MdInline};
use crate::MdSnapshot;
use crate::standards::v_commonmark::subsets::any::schema::inferences::outline::*;

#[semio_framework_async_macros::async_test]
async fn collects_headings_and_counts_words_and_blocks() {
    let snapshot = MdSnapshot {
        schema: "stdio.md".into(),
        blocks: vec![
            MdBlock::Heading { level: 1, inlines: vec![MdInline::Text { text: "Hello World".into() }] },
            MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "one two three".into() }] },
            MdBlock::BlockQuote { blocks: vec![MdBlock::Heading { level: 2, inlines: vec![MdInline::Text { text: "Nested".into() }] }] },
        ],
    };
    let outline = MdOutline::compute(&snapshot);
    assert_eq!(outline.section_outline, vec![MdHeadingEntry { level: 1, text: "Hello World".into() }, MdHeadingEntry { level: 2, text: "Nested".into() }]);
    assert_eq!(outline.block_count, 4);
    assert_eq!(outline.word_count, 6);
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = MdSnapshot::default();
    assert_eq!(MdOutline::compute(&snapshot), MdOutline::compute(&snapshot));
}
