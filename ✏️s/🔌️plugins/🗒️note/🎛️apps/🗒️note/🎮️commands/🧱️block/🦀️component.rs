//! 🧱️ Note play app commands — block create/move/delete/duplicate/patch. Document-mutating.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::engine::{block_bounds, block_id, block_id_from_tree_row_id, clone_block, create_block_by_kind, find_block, offset_block_tree};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::schema::mutations::{
    change_block_font_size, change_block_ink_width, change_block_locked, change_block_visible, delete_block as delete_block_mutation, delete_blocks as delete_blocks_mutation,
    duplicate_block as duplicate_block_mutation, duplicate_blocks as duplicate_blocks_mutation, edit_block_math, edit_block_text, insert_table_column, insert_table_row,
    move_block as move_block_mutation, move_block_to_container, remove_table_column, remove_table_row, rename_block, resize_block,
};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot, NoteTextParagraph, NoteTextRun};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Helpers
/// 🧬️ Clones each of `ids` (present in `document`), offsets the clone by `(24, 24)`, and selects the
/// clones — the shared body of `DuplicateBlock`/`DuplicateSelection`. Placement (right after each
/// source, same parent) is computed by `duplicate-block(s)`'s own diff from `base`, so this only
/// builds the finished clone VALUES (deterministic ids/offsets), never touches the tree itself.
fn duplicate_blocks(document: &NoteSnapshot, ids: &[String]) -> Emit<NoteMutation, NoteConfigMutation> {
    let mut source_ids = Vec::new();
    let mut blocks = Vec::new();
    let mut new_ids = Vec::new();
    for source_id in ids {
        if let Some(block) = find_block(&document.blocks, source_id) {
            let mut cloned = clone_block(block);
            offset_block_tree(&mut cloned, 24.0, 24.0);
            new_ids.push(block_id(&cloned).to_string());
            source_ids.push(source_id.clone());
            blocks.push(cloned);
        }
    }
    if blocks.is_empty() {
        return Emit::default();
    }
    let mutation = if blocks.len() == 1 { duplicate_block_mutation(source_ids.remove(0), blocks.remove(0)) } else { duplicate_blocks_mutation(source_ids, blocks) };
    Emit { artifact_mutations: vec![mutation], config_mutations: vec![NoteConfigMutation::SetSelection { block_ids: new_ids }], ..Default::default() }
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

    pub fn handle(payload: &AddBlock, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        let block = create_block_by_kind(&payload.kind, payload.x, payload.y);
        let new_id = block_id(&block).to_string();
        Ok(Emit { artifact_mutations: vec![crate::artifacts::note::schema::mutations::create_block(block, None, None)], config_mutations: vec![NoteConfigMutation::SetSelection { block_ids: vec![new_id] }], ..Default::default() })
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

    /// 🚚 Reparents `block_id` into `target_row_id`'s container at the drop-appropriate index —
    /// dispatches `move-block-to-container` (hierarchy move), never a whole-`blocks` vec swap.
    pub fn handle(payload: &MoveBlock, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        let document = doc.snapshot;
        if find_block(&document.blocks, &payload.block_id).is_none() {
            return Ok(Emit::default());
        }
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
        Ok(Emit::mutations(vec![move_block_to_container(payload.block_id.clone(), parent_id, index)]))
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

    pub fn handle(payload: &DeleteBlock, _doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        let selection: Vec<String> = cfg.snapshot.selected_block_ids.iter().filter(|id| **id != payload.block_id).cloned().collect();
        Ok(Emit { artifact_mutations: vec![delete_block_mutation(payload.block_id.clone())], config_mutations: vec![NoteConfigMutation::SetSelection { block_ids: selection }], ..Default::default() })
    }
}
//#endregion 🔖️DeleteBlock

//#region 🔖️DeleteSelection
pub mod delete_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "delete-selection")]
    pub struct DeleteSelection {}

    pub fn handle(_payload: &DeleteSelection, _doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        let config = cfg.snapshot;
        if config.selected_block_ids.is_empty() {
            return Ok(Emit::default());
        }
        Ok(Emit { artifact_mutations: vec![delete_blocks_mutation(config.selected_block_ids.clone())], config_mutations: vec![NoteConfigMutation::SetSelection { block_ids: Vec::new() }], ..Default::default() })
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

    pub fn handle(payload: &DuplicateBlock, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
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

    pub fn handle(_payload: &DuplicateSelection, doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
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

    /// 🩹️ Routes the inspector's typed field/value pair to the one narrow semantic mutation that
    /// owns it — one mutation per (id, field) pair, batched into a single `Emit` so a multi-select
    /// patch is still one undo step. Replaces the old `note_engine::patch_block_field`
    /// whole-document-clone + whole-collection re-dump.
    pub fn handle(payload: &PatchBlocks, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        if payload.block_ids.is_empty() || payload.field.is_empty() {
            return Ok(Emit::default());
        }
        let document = doc.snapshot;
        let mut mutations = Vec::new();
        for id in &payload.block_ids {
            let Some(block) = find_block(&document.blocks, id) else { continue };
            let (x, y, width, height) = block_bounds(block);
            let mutation = match payload.field.as_str() {
                "name" => Some(rename_block(id.clone(), payload.value.clone())),
                "visible" => Some(change_block_visible(id.clone(), payload.value.parse::<bool>().unwrap_or(false))),
                "locked" => Some(change_block_locked(id.clone(), payload.value.parse::<bool>().unwrap_or(false))),
                "x" => Some(move_block_mutation(id.clone(), payload.value.parse::<f64>().unwrap_or(0.0), y)),
                "y" => Some(move_block_mutation(id.clone(), x, payload.value.parse::<f64>().unwrap_or(0.0))),
                "width" => Some(resize_block(id.clone(), payload.value.parse::<f64>().unwrap_or(0.0), height)),
                "height" => Some(resize_block(id.clone(), width, payload.value.parse::<f64>().unwrap_or(0.0))),
                "textContent" => Some(edit_block_text(id.clone(), vec![NoteTextParagraph { runs: vec![NoteTextRun { text: payload.value.clone(), bold: None, italic: None, underline: None, link: None }] }])),
                "textSize" => Some(change_block_font_size(id.clone(), payload.value.parse::<f64>().unwrap_or(18.0))),
                "mathTex" => Some(edit_block_math(id.clone(), payload.value.clone())),
                "inkWidth" => Some(change_block_ink_width(id.clone(), payload.value.parse::<f64>().unwrap_or(3.0))),
                "tableAddRow" => Some(insert_table_row(id.clone())),
                "tableRemoveRow" => Some(remove_table_row(id.clone())),
                "tableAddColumn" => Some(insert_table_column(id.clone())),
                "tableRemoveColumn" => Some(remove_table_column(id.clone())),
                _ => None,
            };
            if let Some(mutation) = mutation {
                mutations.push(mutation);
            }
        }
        if mutations.is_empty() {
            return Ok(Emit::default());
        }
        Ok(Emit::mutations(mutations))
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
