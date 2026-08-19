//! 🧾 `outline` — one named inference: this document's own outline. A note document IS its own
//! outline (a flat/grouped block tree, no separate table of contents), so `sectionOutline` is the
//! block name list (flattened through `Group` nesting, document order), `blockCount` the total
//! flattened block count, and `wordCount` a real sum over every `Text` block's run text.

use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
async fn block_name(block: &NoteBlockNode) -> &str {
    match block {
        NoteBlockNode::Text { name, .. }
        | NoteBlockNode::Image { name, .. }
        | NoteBlockNode::Table { name, .. }
        | NoteBlockNode::Math { name, .. }
        | NoteBlockNode::Ink { name, .. }
        | NoteBlockNode::Group { name, .. } => name,
    }
}

async fn flatten_blocks<'a>(blocks: &'a [NoteBlockNode], out: &mut Vec<&'a NoteBlockNode>) {
    for block in blocks {
        out.push(block);
        if let NoteBlockNode::Group { children, .. } = block {
            flatten_blocks(children, out);
        }
    }
}

async fn block_word_count(block: &NoteBlockNode) -> u32 {
    match block {
        NoteBlockNode::Text { content, .. } => {
            crate::artifacts::note::note_block_text(content).iter().map(|paragraph| paragraph.runs.iter().map(|run| run.text.split_whitespace().count()).sum::<usize>()).sum::<usize>() as u32
        }
        _ => 0,
    }
}

/// 🧾️ `Note` document outline.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteOutline {
    pub section_outline: Vec<String>,
    pub block_count: u32,
    pub word_count: u32,
}

impl NoteOutline {
    pub async fn compute(snapshot: &NoteSnapshot) -> Self {
        let mut flat = Vec::new();
        flatten_blocks(&snapshot.blocks, &mut flat);
        let section_outline = flat.iter().map(|block| block_name(block).to_string()).collect();
        let block_count = flat.len() as u32;
        let word_count: u32 = flat.iter().map(|block| block_word_count(block)).sum();
        Self { section_outline, block_count, word_count }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::note::{NoteTextParagraph, NoteTextRun};

    async fn text_block(id: &str, name: &str, text: &str) -> NoteBlockNode {
        let paragraphs = vec![NoteTextParagraph { runs: vec![NoteTextRun { text: text.into(), bold: None, italic: None, underline: None, link: None }] }];
        NoteBlockNode::Text {
            content: crate::artifacts::note::note_text_child_handle_and_cache(id, &paragraphs),
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

    #[test]
    async fn outline_flattens_group_children_in_document_order() {
        let group = NoteBlockNode::Group { id: "g".into(), name: "Group".into(), x: 0.0, y: 0.0, width: 1.0, height: 1.0, rotation: 0.0, visible: true, locked: false, children: vec![text_block("t1", "Child", "hi")] };
        let snapshot = NoteSnapshot { blocks: vec![group], ..NoteSnapshot::default() };
        let outline = NoteOutline::compute(&snapshot);
        assert_eq!(outline.section_outline, vec!["Group".to_string(), "Child".to_string()]);
        assert_eq!(outline.block_count, 2);
    }

    #[test]
    async fn outline_counts_words_across_text_blocks_only() {
        let snapshot = NoteSnapshot { blocks: vec![text_block("t1", "A", "one two three")], ..NoteSnapshot::default() };
        let outline = NoteOutline::compute(&snapshot);
        assert_eq!(outline.word_count, 3);
    }

    #[test]
    async fn empty_blocks_produce_an_empty_outline() {
        let outline = NoteOutline::compute(&NoteSnapshot::default());
        assert!(outline.section_outline.is_empty());
        assert_eq!(outline.block_count, 0);
        assert_eq!(outline.word_count, 0);
    }

    #[test]
    async fn outline_is_deterministic() {
        let snapshot = NoteSnapshot { blocks: vec![text_block("t1", "A", "hello world")], ..NoteSnapshot::default() };
        assert_eq!(NoteOutline::compute(&snapshot), NoteOutline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
