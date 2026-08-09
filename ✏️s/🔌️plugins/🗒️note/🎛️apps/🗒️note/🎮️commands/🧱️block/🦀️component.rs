//! 🧱️ Note play app commands — block create/move/delete/duplicate/patch. Document-mutating.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::engine::{block_id, block_id_from_tree_row_id, clone_block, create_block_by_kind, find_block, insert_after, insert_block, offset_block_tree, patch_block_field, remove_block_from_tree};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Helpers
/// 🧬️ Clones each of `ids` (present in `document`), offsets the clone by `(24, 24)`, and selects the
/// clones — the shared body of `DuplicateBlock`/`DuplicateSelection`.
fn duplicate_blocks(document: &NoteSnapshot, ids: &[String]) -> Emit<NoteMutation, NoteConfigMutation> {
    let mut blocks = document.blocks.clone();
    let mut new_ids = Vec::new();
    for source_id in ids {
        if let Some(block) = find_block(&blocks, source_id).cloned() {
            let mut cloned = clone_block(&block);
            offset_block_tree(&mut cloned, 24.0, 24.0);
            new_ids.push(block_id(&cloned).to_string());
            if !insert_after(&mut blocks, source_id, cloned.clone()) {
                blocks.push(cloned);
            }
        }
    }
    if new_ids.is_empty() {
        return Emit::default();
    }
    Emit { document_mutations: vec![NoteMutation::SetBlocks { blocks }], config_mutations: vec![NoteConfigMutation::SetSelection { block_ids: new_ids }], ..Default::default() }
}

/// 🩹️ `PatchBlocks`'s typed field/value pair, reconstructed into the `serde_json::Value` shape
/// `note_engine::patch_block_field` expects — mirrors `shooting_ui`'s `shot_patch_for_field`/
/// `asset_patch_for_field` string-value convention, extended with the numeric/bool fields note's
/// inspector patches that shooting's string-only fields never needed.
fn note_patch_json_value(field: &str, value: &str) -> Value {
    match field {
        "visible" | "locked" => Value::Bool(value.parse::<bool>().unwrap_or(false)),
        "x" | "y" | "width" | "height" | "textSize" | "inkWidth" => value.parse::<f64>().ok().and_then(serde_json::Number::from_f64).map_or(Value::Null, Value::Number),
        _ => Value::String(value.to_string()),
    }
}
//#endregion 🔖️Helpers

//#region 🔖️AddBlock
pub mod add_block {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-block")]
    pub struct AddBlock {
        pub kind: String,
        pub x: f64,
        pub y: f64,
    }

    pub fn handle(payload: &AddBlock, doc: &DocumentView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        let block = create_block_by_kind(&payload.kind, payload.x, payload.y);
        let new_id = block_id(&block).to_string();
        let mut blocks = doc.snapshot.blocks.clone();
        blocks.push(block);
        Ok(Emit { document_mutations: vec![NoteMutation::SetBlocks { blocks }], config_mutations: vec![NoteConfigMutation::SetSelection { block_ids: vec![new_id] }], ..Default::default() })
    }
}
//#endregion 🔖️AddBlock

//#region 🔖️MoveBlock
pub mod move_block {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "move-block")]
    pub struct MoveBlock {
        pub block_id: String,
        pub target_row_id: String,
        pub drop_position: String,
    }

    pub fn handle(payload: &MoveBlock, doc: &DocumentView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        let document = doc.snapshot;
        let Some(block) = find_block(&document.blocks, &payload.block_id).cloned() else {
            return Ok(Emit::default());
        };
        let target_id = block_id_from_tree_row_id(&payload.target_row_id);
        let parent_id = target_id.as_ref().and_then(|id| find_block(&document.blocks, id).and_then(|entry| if matches!(entry, NoteBlockNode::Group { .. }) { Some(id.clone()) } else { None }));
        let index = if payload.drop_position == "before" {
            0
        } else if let Some(ref parent) = parent_id {
            find_block(&document.blocks, parent)
                .and_then(|entry| match entry {
                    NoteBlockNode::Group { children, .. } => Some(children.len()),
                    _ => None,
                })
                .unwrap_or(0)
        } else {
            document.blocks.len()
        };
        let mut blocks = document.blocks.clone();
        remove_block_from_tree(&mut blocks, &payload.block_id);
        insert_block(&mut blocks, parent_id.as_deref(), index, block);
        Ok(Emit::mutations(vec![NoteMutation::SetBlocks { blocks }]))
    }
}
//#endregion 🔖️MoveBlock

//#region 🔖️DeleteBlock
pub mod delete_block {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "delete-block")]
    pub struct DeleteBlock {
        pub block_id: String,
    }

    pub fn handle(payload: &DeleteBlock, doc: &DocumentView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        let mut blocks = doc.snapshot.blocks.clone();
        remove_block_from_tree(&mut blocks, &payload.block_id);
        let selection: Vec<String> = cfg.snapshot.selected_block_ids.iter().filter(|id| **id != payload.block_id).cloned().collect();
        Ok(Emit { document_mutations: vec![NoteMutation::SetBlocks { blocks }], config_mutations: vec![NoteConfigMutation::SetSelection { block_ids: selection }], ..Default::default() })
    }
}
//#endregion 🔖️DeleteBlock

//#region 🔖️DeleteSelection
pub mod delete_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "delete-selection")]
    pub struct DeleteSelection {}

    pub fn handle(_payload: &DeleteSelection, doc: &DocumentView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        let config = cfg.snapshot;
        if config.selected_block_ids.is_empty() {
            return Ok(Emit::default());
        }
        let mut blocks = doc.snapshot.blocks.clone();
        for id in &config.selected_block_ids {
            remove_block_from_tree(&mut blocks, id);
        }
        Ok(Emit { document_mutations: vec![NoteMutation::SetBlocks { blocks }], config_mutations: vec![NoteConfigMutation::SetSelection { block_ids: Vec::new() }], ..Default::default() })
    }
}
//#endregion 🔖️DeleteSelection

//#region 🔖️DuplicateBlock
pub mod duplicate_block {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "duplicate-block")]
    pub struct DuplicateBlock {
        pub block_id: String,
    }

    pub fn handle(payload: &DuplicateBlock, doc: &DocumentView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        Ok(duplicate_blocks(doc.snapshot, std::slice::from_ref(&payload.block_id)))
    }
}
//#endregion 🔖️DuplicateBlock

//#region 🔖️DuplicateSelection
pub mod duplicate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "duplicate-selection")]
    pub struct DuplicateSelection {}

    pub fn handle(_payload: &DuplicateSelection, doc: &DocumentView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        Ok(duplicate_blocks(doc.snapshot, &cfg.snapshot.selected_block_ids))
    }
}
//#endregion 🔖️DuplicateSelection

//#region 🔖️PatchBlocks
pub mod patch_blocks {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-blocks")]
    pub struct PatchBlocks {
        pub block_ids: Vec<String>,
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchBlocks, doc: &DocumentView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        if payload.block_ids.is_empty() || payload.field.is_empty() {
            return Ok(Emit::default());
        }
        let json_value = note_patch_json_value(&payload.field, &payload.value);
        let mut next = doc.snapshot.clone();
        for id in &payload.block_ids {
            next = patch_block_field(&next, id, &payload.field, &json_value);
        }
        Ok(Emit::mutations(vec![NoteMutation::SetBlocks { blocks: next.blocks }]))
    }
}
//#endregion 🔖️PatchBlocks

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::note::testkit::{dispatch, note_app};
    use crate::apps::note::NoteCommand;

    #[test]
    fn add_block_action_emits_one_op_and_grows_projection() {
        let mut app = note_app();
        let result = dispatch(&mut app, NoteCommand::AddBlock(add_block::AddBlock { kind: "text".into(), x: 80.0, y: 80.0 }));
        assert_eq!(result.mutations.len(), 1);
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(projection.blocks.len(), 1);
        assert_eq!(crate::artifacts::note::engine::block_kind(&projection.blocks[0]), "text");
    }

    #[test]
    fn add_block_then_undo_round_trip() {
        use semio_framework_plugin::testkit;
        let mut app = note_app();
        testkit::assert_undo_redo_round_trip(&mut app, NoteCommand::AddBlock(add_block::AddBlock { kind: "text".into(), x: 0.0, y: 0.0 }), |app| app.snapshot().expect("snapshot").blocks.len(), 0, 1);
    }

    #[test]
    fn patch_blocks_table_row_and_column_ops_clamp_at_one() {
        let mut app = note_app();
        dispatch(&mut app, NoteCommand::AddBlock(add_block::AddBlock { kind: "table".into(), x: 0.0, y: 0.0 }));
        let table_id = block_id(&app.snapshot().expect("snapshot").blocks[0]).to_string();

        for (field, expected_rows, expected_columns) in [("tableAddRow", 3, 3), ("tableAddColumn", 3, 4), ("tableRemoveRow", 2, 4), ("tableRemoveRow", 1, 4), ("tableRemoveRow", 1, 4), ("tableRemoveColumn", 1, 3)] {
            dispatch(&mut app, NoteCommand::PatchBlocks(patch_blocks::PatchBlocks { block_ids: vec![table_id.clone()], field: field.into(), value: String::new() }));
            let projection = app.snapshot().expect("snapshot");
            let block = find_block(&projection.blocks, &table_id).unwrap();
            if let NoteBlockNode::Table { rows, columns, .. } = block {
                assert_eq!(rows.len(), expected_rows, "field {field}");
                assert_eq!(columns.len(), expected_columns, "field {field}");
            } else {
                panic!("expected table block");
            }
        }
    }

    #[test]
    fn duplicate_selection_clones_with_offset_and_selects_clones() {
        let mut app = note_app();
        dispatch(&mut app, NoteCommand::AddBlock(add_block::AddBlock { kind: "text".into(), x: 10.0, y: 10.0 }));
        let source_id = block_id(&app.snapshot().expect("snapshot").blocks[0]).to_string();

        let result = dispatch(&mut app, NoteCommand::DuplicateSelection(duplicate_selection::DuplicateSelection {}));
        assert_eq!(result.mutations.len(), 1);
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(projection.blocks.len(), 2);
        let clone = projection.blocks.iter().find(|block| block_id(block) != source_id).expect("clone block");
        let (x, y, ..) = crate::artifacts::note::engine::block_bounds(clone);
        assert_eq!((x, y), (34.0, 34.0));
    }
}
//#endregion 🧪️Tests
