//! 🧾 `outline` — one named inference: this CommonMark document's own section/word structure.
//! `sectionOutline` is every `Heading` block found anywhere in the tree (including nested inside
//! a `BlockQuote`), in document order, as `(level, text)` — `text` is the heading's flattened
//! inline text (emphasis/strong/code/link text all concatenated, soft/hard breaks become a
//! space); `blockCount` is a real recursive walk counting every `MdBlock` node (list items and
//! block-quote contents included); `wordCount` is a whitespace-split word count over every
//! block's own flattened text (headings, paragraphs, code blocks, list items, block quotes).

use crate::artifacts::md::schema::snapshot::{MdBlock, MdInline};
use crate::artifacts::md::MdSnapshot;

//#region 🔖️Outline
/// 🧾️ One `sectionOutline` entry — a heading's level + flattened text.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct MdHeadingEntry {
    pub level: u8,
    pub text: String,
}

/// 🧾️ `Md` document outline.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct MdOutline {
    pub section_outline: Vec<MdHeadingEntry>,
    pub block_count: u32,
    pub word_count: u32,
}

/// 🔤️ Flattens a run of inlines into plain text — emphasis/strong/link recurse into their own
/// inline children, code spans use their literal, images use their alt text, soft/hard breaks
/// become a single space, raw inline HTML contributes nothing (it isn't prose content).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inline_text(inlines: &[MdInline]) -> String {
    let mut out = String::new();
    for inline in inlines {
        match inline {
            MdInline::Text { text } => out.push_str(text),
            MdInline::Emphasis { inlines } | MdInline::Strong { inlines } => out.push_str(&inline_text(inlines)),
            MdInline::Code { literal } => out.push_str(literal),
            MdInline::Link { text, .. } => out.push_str(&inline_text(text)),
            MdInline::Image { alt, .. } => out.push_str(alt),
            MdInline::SoftBreak | MdInline::HardBreak => out.push(' '),
            MdInline::HtmlInline { .. } => {}
        }
    }
    out
}

/// 🌳️ Recursively walks `block`, appending every `Heading` encountered to `headings`, adding to
/// `block_count`, and appending flattened text to `word_source`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn walk_block(block: &MdBlock, headings: &mut Vec<MdHeadingEntry>, block_count: &mut u32, word_source: &mut String) {
    *block_count += 1;
    match block {
        MdBlock::Heading { level, inlines } => {
            let text = inline_text(inlines);
            word_source.push(' ');
            word_source.push_str(&text);
            headings.push(MdHeadingEntry { level: *level, text });
        }
        MdBlock::Paragraph { inlines } => {
            word_source.push(' ');
            word_source.push_str(&inline_text(inlines));
        }
        MdBlock::List { items, .. } => {
            for item in items {
                for child in item {
                    walk_block(child, headings, block_count, word_source);
                }
            }
        }
        MdBlock::CodeBlock { literal, .. } => {
            word_source.push(' ');
            word_source.push_str(literal);
        }
        MdBlock::BlockQuote { blocks } => {
            for child in blocks {
                walk_block(child, headings, block_count, word_source);
            }
        }
        MdBlock::ThematicBreak | MdBlock::HtmlBlock { .. } => {}
    }
}

impl MdOutline {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compute(snapshot: &MdSnapshot) -> Self {
        let mut section_outline = Vec::new();
        let mut block_count = 0u32;
        let mut word_source = String::new();
        for block in &snapshot.blocks {
            walk_block(block, &mut section_outline, &mut block_count, &mut word_source);
        }
        let word_count = word_source.split_whitespace().count() as u32;
        Self { section_outline, block_count, word_count }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
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
}
//#endregion 🧪️Tests
