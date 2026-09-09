use super::*;
use crate::{NoteTextParagraph, NoteTextRun};

fn text_block(id: &str, name: &str, text: &str) -> NoteBlockNode {
    let paragraphs = vec![NoteTextParagraph { runs: vec![NoteTextRun { text: text.into(), bold: None, italic: None, underline: None, link: None }] }];
    NoteBlockNode::Text {
        content: crate::note_text_child_record(id, &paragraphs),
        id: id.into(),
        name: name.into(),
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
        rotation: 0.0,
        visible: true,
        locked: false,
        font_size: 18.0,
        font_weight: "normal".into(),
        align: "left".into(),
    }
}

#[semio_framework_async_macros::async_test]
async fn outline_flattens_group_children_in_document_order() {
    let group = NoteBlockNode::Group { id: "g".into(), name: "Group".into(), x: 0.0, y: 0.0, width: 1.0, height: 1.0, rotation: 0.0, visible: true, locked: false, children: vec![text_block("t1", "Child", "hi")] };
    let snapshot = NoteSnapshot { blocks: vec![group], ..NoteSnapshot::default() };
    let outline = NoteOutline::compute(&snapshot);
    assert_eq!(outline.section_outline, vec!["Group".to_string(), "Child".to_string()]);
    assert_eq!(outline.block_count, 2);
}

#[semio_framework_async_macros::async_test]
async fn outline_counts_words_across_text_blocks_only() {
    let snapshot = NoteSnapshot { blocks: vec![text_block("t1", "A", "one two three")], ..NoteSnapshot::default() };
    let outline = NoteOutline::compute(&snapshot);
    assert_eq!(outline.word_count, 3);
}

#[semio_framework_async_macros::async_test]
async fn empty_blocks_produce_an_empty_outline() {
    let outline = NoteOutline::compute(&NoteSnapshot::default());
    assert!(outline.section_outline.is_empty());
    assert_eq!(outline.block_count, 0);
    assert_eq!(outline.word_count, 0);
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = NoteSnapshot { blocks: vec![text_block("t1", "A", "hello world")], ..NoteSnapshot::default() };
    assert_eq!(NoteOutline::compute(&snapshot), NoteOutline::compute(&snapshot));
}
