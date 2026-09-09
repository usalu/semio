//! 🧾 `outline` — one named inference: this document's own outline. A note document IS its own
//! outline (a flat/grouped block tree, no separate table of contents), so `sectionOutline` is the
//! block name list (flattened through `Group` nesting, document order), `blockCount` the total
//! flattened block count, and `wordCount` a real sum over every `Text` block's run text.

use crate::{NoteBlockNode, NoteSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
fn block_name(block: &NoteBlockNode) -> &str {
    match block {
        NoteBlockNode::Text { name, .. } | NoteBlockNode::Image { name, .. } | NoteBlockNode::Table { name, .. } | NoteBlockNode::Math { name, .. } | NoteBlockNode::Ink { name, .. } | NoteBlockNode::Group { name, .. } => name,
    }
}

fn flatten_blocks<'a>(blocks: &'a [NoteBlockNode], out: &mut Vec<&'a NoteBlockNode>) {
    for block in blocks {
        out.push(block);
        if let NoteBlockNode::Group { children, .. } = block {
            flatten_blocks(children, out);
        }
    }
}

fn block_word_count(block: &NoteBlockNode) -> u32 {
    match block {
        NoteBlockNode::Text { content, .. } => crate::note_block_text(content).iter().map(|paragraph| paragraph.runs.iter().map(|run| run.text.split_whitespace().count()).sum::<usize>()).sum::<usize>() as u32,
        _ => 0,
    }
}

/// 🧾️ `Note` document outline.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct NoteOutline {
    pub section_outline: Vec<String>,
    pub block_count: u32,
    pub word_count: u32,
}

impl NoteOutline {
    pub fn compute(snapshot: &NoteSnapshot) -> Self {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
