
use super::*;

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
    assert_eq!(outline.block_count, 4); // 3 top-level + the Heading nested inside the BlockQuote — walk_block counts every block, not just top-level ones
    assert_eq!(outline.word_count, 6); // Hello World + one two three + Nested
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = MdSnapshot::default();
    assert_eq!(MdOutline::compute(&snapshot), MdOutline::compute(&snapshot));
}
