//! 🧭️ The details-pane vocabulary of the md editor: a snapshot pointer walks the block tree (`/blocks/<i>`, then `blocks/<j>` of a quote or
//! `items/<k>/<j>` of a list per level) to the block it addresses and raises that block's kind: a block insert and remove at its container,
//! a paragraph or heading `inlines` edit as `set-inlines`, and every other block edit as `replace-block`. The table itself is empty because the
//! kinds are addressed by a recursive path, not by a fixed pointer shape.

use crate::standards::v_commonmark::subsets::any::schema::mutations::{insert_block, remove_block, replace_block, set_inlines, MdMutation, MdPathStep};
use crate::standards::v_commonmark::subsets::any::schema::snapshot::{MdBlock, MdSnapshot};
use semio_framework_plugin::Fault;
use semio_framework_value::{FromValue, ToValue};
use semio_s_artifact_stdio_contract::editing::{edited_subtree, EditRules, SnapshotEditEvent};

/// 📚 No fixed pointer shape: see [`resolve`].
pub const EDIT_RULES: EditRules = EditRules { entities: &[], inserts: &[], removes: &[] };

fn fail(message: impl Into<String>) -> Fault {
    Fault::from(message.into())
}

fn segments_of(pointer: &str) -> Result<Vec<String>, Fault> {
    if pointer.is_empty() {
        return Ok(Vec::new());
    }
    pointer.strip_prefix('/').map(|rest| rest.split('/').map(|raw| raw.replace("~1", "/").replace("~0", "~")).collect()).ok_or_else(|| fail(format!("'{pointer}' is not a pointer")))
}

fn index_of(segment: &str, length: usize, insertion: bool) -> Result<usize, Fault> {
    let index = if insertion && segment == "-" { length } else { segment.parse::<usize>().map_err(|error| fail(error.to_string()))? };
    (index < length || (insertion && index == length)).then_some(index).ok_or_else(|| fail(format!("index {index} is outside 0..{length}")))
}

fn walk(blocks: &[MdBlock], pointer: &str, segments: &[String], path: &[MdPathStep], event: &SnapshotEditEvent) -> Result<Vec<MdMutation>, Fault> {
    let Some((first, rest)) = segments.split_first() else { return Err(fail("no kind edits the block list itself")) };
    let at = path.to_vec();
    let inserting = matches!(event, SnapshotEditEvent::InsertValue { .. });
    let index = index_of(first, blocks.len(), inserting && rest.is_empty())?;
    if rest.is_empty() {
        match event {
            SnapshotEditEvent::InsertValue { value, .. } => {
                let block = MdBlock::from_value(value.clone()).map_err(|error| fail(error.to_string()))?;
                return Ok(vec![MdMutation::InsertBlock(insert_block::InsertBlock { path: at, index, block })]);
            }
            SnapshotEditEvent::RemoveValue { .. } => return Ok(vec![MdMutation::RemoveBlock(remove_block::RemoveBlock { path: at, index })]),
            _ => {}
        }
    }
    let block_pointer = format!("{pointer}/{index}");
    let block = &blocks[index];
    let nested = |step: MdPathStep| path.iter().cloned().chain(std::iter::once(step)).collect::<Vec<_>>();
    match (block, rest) {
        (MdBlock::BlockQuote { blocks: inner }, [key, tail @ ..]) if key == "blocks" && !tail.is_empty() => walk(inner, &format!("{block_pointer}/blocks"), tail, &nested(MdPathStep::BlockQuote { index }), event),
        (MdBlock::List { items, .. }, [key, item, tail @ ..]) if key == "items" && !tail.is_empty() => {
            let item_index = index_of(item, items.len(), false)?;
            walk(&items[item_index], &format!("{block_pointer}/items/{item_index}"), tail, &nested(MdPathStep::ListItem { index, item: item_index }), event)
        }
        _ => {
            let edited = MdBlock::from_value(edited_subtree(&block.to_value(), &block_pointer, event).map_err(|error| fail(error.to_string()))?).map_err(|error| fail(error.to_string()))?;
            if &edited == block {
                return Ok(Vec::new());
            }
            let inlines_only = matches!(rest.first().map(String::as_str), Some("inlines"));
            Ok(vec![match (&edited, inlines_only) {
                (MdBlock::Paragraph { inlines }, true) | (MdBlock::Heading { inlines, .. }, true) => MdMutation::SetInlines(set_inlines::SetInlines { path: at, index, inlines: inlines.clone() }),
                _ => MdMutation::ReplaceBlock(replace_block::ReplaceBlock { path: at, index, block: edited }),
            }])
        }
    }
}

/// 🎯 The concrete kinds an edit of the markdown document denotes, or `None` when the pointer is outside `/blocks`.
pub fn resolve(snapshot: &MdSnapshot, event: &SnapshotEditEvent) -> Result<Option<Vec<MdMutation>>, Fault> {
    let path = match event {
        SnapshotEditEvent::SetValue { path, .. } | SnapshotEditEvent::InsertValue { path, .. } | SnapshotEditEvent::RemoveValue { path } | SnapshotEditEvent::MoveValue { path, .. } | SnapshotEditEvent::RenameKey { path, .. } => path,
        SnapshotEditEvent::ReplaceSource { .. } => return Ok(None),
    };
    let segments = segments_of(path)?;
    match segments.split_first() {
        Some((blocks, tail)) if blocks == "blocks" && !tail.is_empty() => walk(&snapshot.blocks, "/blocks", tail, &[], event).map(Some),
        _ => Ok(None),
    }
}
