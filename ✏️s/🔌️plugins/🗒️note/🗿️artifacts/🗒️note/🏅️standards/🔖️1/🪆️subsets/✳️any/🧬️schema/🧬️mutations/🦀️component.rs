//! 🧬️ Note artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! triad leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<NoteSnapshot>`
//! and `impl protocol::SemanticMutation<NoteSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here (the old hand-written `match`-per-variant `diff`/`inverse` are
//! retired along with the 8 bare generic-verb scalar setters, the whole-collection block-list `Vec`
//! setter, the put-synonym asset upsert, and the whole-document-replace escape hatch — see
//! `📓️taxonomy.md`/`📓️derivation-rules.md` in ticket `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`).

use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use protocol::{Mutation, MutationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 🧮️ Semantic note document mutation vocabulary, derived from `🧬️schema/📸️snapshot/🦀️component.rs`:
/// 9 document-root scalars (`rename-note` for the identity field, `change-*` for the 8 grid/snap/
/// tool settings), 3 id-keyed asset mutations (`create`/`replace-payload`/`delete`), and 21 block
/// mutations over the id-keyed, z-order-meaningful, group-nestable block tree (create/delete(s)/
/// duplicate(s)/reparent/drag/rename/visible/locked/move/resize, plus per-kind content edits for
/// text/math/ink, plus table row/column insert/remove). Whole-document replace has NO replacement
/// here — see `crate::editor::note::reset_document_effect`, which goes through
/// `Effect::LoadDocument` outside undo history.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = NoteSnapshot, diff = NoteDiff, schema = "note.note")]
pub enum NoteMutation {
    RenameNote(RenameNote),
    ChangeGridVisible(ChangeGridVisible),
    ChangeGridSpacing(ChangeGridSpacing),
    ChangeGridSubdivisions(ChangeGridSubdivisions),
    ChangeGridOpacity(ChangeGridOpacity),
    ChangeSnapEnabled(ChangeSnapEnabled),
    ChangeSnapGridSpacing(ChangeSnapGridSpacing),
    ChangePencilWidth(ChangePencilWidth),
    ChangeEraserRadius(ChangeEraserRadius),
    CreateAsset(CreateAsset),
    ReplaceAssetPayload(ReplaceAssetPayload),
    DeleteAsset(DeleteAsset),
    CreateBlock(CreateBlock),
    DeleteBlock(DeleteBlock),
    DeleteBlocks(DeleteBlocks),
    DuplicateBlock(DuplicateBlock),
    DuplicateBlocks(DuplicateBlocks),
    MoveBlockToContainer(MoveBlockToContainer),
    DragBlocks(DragBlocks),
    RenameBlock(RenameBlock),
    ChangeBlockVisible(ChangeBlockVisible),
    ChangeBlockLocked(ChangeBlockLocked),
    MoveBlock(MoveBlock),
    ResizeBlock(ResizeBlock),
    ChangeBlockFontSize(ChangeBlockFontSize),
    EditBlockText(EditBlockText),
    EditBlockMath(EditBlockMath),
    ChangeBlockInkWidth(ChangeBlockInkWidth),
    EditBlockInkStroke(EditBlockInkStroke),
    InsertTableRow(InsertTableRow),
    RemoveTableRow(RemoveTableRow),
    InsertTableColumn(InsertTableColumn),
    RemoveTableColumn(RemoveTableColumn),
}
//#endregion 🔖️Mutations

//#region 🔖️Reexports
pub use super::change_block_font_size::mutation::{change_block_font_size, ChangeBlockFontSize};
pub use super::change_block_ink_width::mutation::{change_block_ink_width, ChangeBlockInkWidth};
pub use super::change_block_locked::mutation::{change_block_locked, ChangeBlockLocked};
pub use super::change_block_visible::mutation::{change_block_visible, ChangeBlockVisible};
pub use super::change_eraser_radius::mutation::{change_eraser_radius, ChangeEraserRadius};
pub use super::change_grid_opacity::mutation::{change_grid_opacity, ChangeGridOpacity};
pub use super::change_grid_spacing::mutation::{change_grid_spacing, ChangeGridSpacing};
pub use super::change_grid_subdivisions::mutation::{change_grid_subdivisions, ChangeGridSubdivisions};
pub use super::change_grid_visible::mutation::{change_grid_visible, ChangeGridVisible};
pub use super::change_pencil_width::mutation::{change_pencil_width, ChangePencilWidth};
pub use super::change_snap_enabled::mutation::{change_snap_enabled, ChangeSnapEnabled};
pub use super::change_snap_grid_spacing::mutation::{change_snap_grid_spacing, ChangeSnapGridSpacing};
pub use super::create_asset::mutation::{create_asset, CreateAsset};
pub use super::create_block::mutation::{create_block, CreateBlock};
pub use super::delete_asset::mutation::{delete_asset, DeleteAsset};
pub use super::delete_block::mutation::{delete_block, DeleteBlock};
pub use super::delete_blocks::mutation::{delete_blocks, DeleteBlocks};
pub use super::drag_blocks::mutation::{drag_blocks, DragBlocks};
pub use super::duplicate_block::mutation::{duplicate_block, DuplicateBlock};
pub use super::duplicate_blocks::mutation::{duplicate_blocks, DuplicateBlocks};
pub use super::edit_block_ink_stroke::mutation::{edit_block_ink_stroke, EditBlockInkStroke};
pub use super::edit_block_math::mutation::{edit_block_math, EditBlockMath};
pub use super::edit_block_text::mutation::{edit_block_text, EditBlockText};
pub use super::insert_table_column::mutation::{insert_table_column, InsertTableColumn};
pub use super::insert_table_row::mutation::{insert_table_row, InsertTableRow};
pub use super::move_block::mutation::{move_block, MoveBlock};
pub use super::move_block_to_container::mutation::{move_block_to_container, MoveBlockToContainer};
pub use super::remove_table_column::mutation::{remove_table_column, RemoveTableColumn};
pub use super::remove_table_row::mutation::{remove_table_row, RemoveTableRow};
pub use super::rename_block::mutation::{rename_block, RenameBlock};
pub use super::rename_note::mutation::{rename_note, RenameNote};
pub use super::replace_asset_payload::mutation::{replace_asset_payload, ReplaceAssetPayload};
pub use super::resize_block::mutation::{resize_block, ResizeBlock};
//#endregion 🔖️Reexports

//#region 🔖️Helpers
/// ▶️ Applies `mutation` via its diff — the sole apply path now (no hand-written match dispatch).
pub async fn apply_note_mutation(snapshot: &NoteSnapshot, mutation: &NoteMutation) -> protocol::MutationApplyResult<NoteSnapshot> {
    let (diff, _messages) = mutation.diff(snapshot).into_parts();
    MutationDiff::apply(&diff, snapshot)
}

pub async fn inverse_note_mutation(snapshot: &NoteSnapshot, mutation: &NoteMutation) -> Vec<NoteMutation> {
    mutation.inverse(snapshot)
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::note::{NoteBlockNode, NoteImageAsset};
    use protocol::testkit::{assert_fatal_never_applies, assert_missing_target_is_error, assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::SemanticMutation;

    async fn sample_snapshot() -> NoteSnapshot {
        let mut snapshot = crate::artifacts::note::schema::empty_note_snapshot();
        snapshot.blocks.push(NoteBlockNode::Text {
            id: "b1".into(),
            name: "Text".into(),
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 40.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            content: crate::artifacts::note::note_text_child_handle_and_cache("b1", &[]),
            font_size: 18.0,
            font_weight: "normal".into(),
            align: "left".into(),
        });
        snapshot.blocks.push(NoteBlockNode::Ink { id: "b2".into(), name: "Ink".into(), x: 0.0, y: 0.0, width: 1.0, height: 1.0, rotation: 0.0, visible: true, locked: false, points: vec![[0.0, 0.0]], stroke_width: 3.0, color: [0.0, 0.0, 0.0, 1.0] });
        snapshot.blocks.push(NoteBlockNode::Table {
            id: "b3".into(),
            name: "Table".into(),
            x: 0.0,
            y: 0.0,
            width: 200.0,
            height: 100.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            columns: vec!["A".into(), "B".into()],
            rows: vec![vec![crate::artifacts::note::NoteTableCell { content: String::new() }, crate::artifacts::note::NoteTableCell { content: String::new() }]],
        });
        snapshot.blocks.push(NoteBlockNode::Math { id: "b4".into(), name: "Math".into(), x: 0.0, y: 0.0, width: 100.0, height: 40.0, rotation: 0.0, visible: true, locked: false, tex: "x".into(), display_mode: true });
        snapshot.assets.insert("asset-1".into(), NoteImageAsset { mime: "image/png".into(), data: "d".into(), width: None, height: None });
        snapshot
    }

    async fn round_trip(snapshot: &NoteSnapshot, mutation: &NoteMutation) -> NoteSnapshot {
        let forward = apply_note_mutation(snapshot, mutation).expect("valid mutation diff");
        let mut restored = forward.clone();
        for back in mutation.inverse(snapshot) {
            restored = apply_note_mutation(&restored, &back).expect("valid inverse mutation diff");
        }
        assert_eq!(&restored, snapshot, "inverse must restore the pre-mutation snapshot for {mutation:?}");
        forward
    }

    #[semio_framework_async_macros::async_test]
    async fn dispatch_registers_semantic_descriptors() {
        register_note_mutation_descriptors();
        for kind in NoteMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(NoteMutation::kinds().len(), 33);
    }

    //#region 🔖️MutationLaws
    #[semio_framework_async_macros::async_test]
    async fn root_scalar_inverse_and_absorb_laws() {
        let base = sample_snapshot();
        for mutation in [
            rename_note(Some("Renamed".into())),
            change_grid_visible(Some(false)),
            change_grid_spacing(Some(16.0)),
            change_grid_subdivisions(Some(8.0)),
            change_grid_opacity(Some(0.6)),
            change_snap_enabled(Some(true)),
            change_snap_grid_spacing(Some(4.0)),
            change_pencil_width(Some(5.0)),
            change_eraser_radius(Some(20.0)),
        ] {
            assert_mutation_inverse_law(&base, &mutation);
        }
        let d1 = change_grid_spacing(Some(10.0)).diff(&base).into_parts().0;
        let mid = MutationDiff::apply(&d1, &base).expect("valid mutation diff");
        let d2 = change_grid_spacing(Some(20.0)).diff(&mid).into_parts().0;
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    async fn asset_inverse_law_create_replace_delete() {
        let base = sample_snapshot();
        let asset = NoteImageAsset { mime: "image/jpeg".into(), data: "e".into(), width: None, height: None };
        assert_mutation_inverse_law(&base, &create_asset("asset-2".into(), asset.clone()));
        assert_mutation_inverse_law(&base, &replace_asset_payload("asset-1".into(), asset.clone()));
        assert_mutation_inverse_law(&base, &delete_asset("asset-1".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn block_lifecycle_inverse_law_create_delete_duplicate() {
        let base = sample_snapshot();
        let new_block = NoteBlockNode::Text {
            id: "b99".into(),
            name: "New".into(),
            x: 5.0,
            y: 6.0,
            width: 80.0,
            height: 30.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            content: crate::artifacts::note::note_text_child_handle_and_cache("b99", &[]),
            font_size: 18.0,
            font_weight: "normal".into(),
            align: "left".into(),
        };
        assert_mutation_inverse_law(&base, &create_block(new_block.clone(), None, None));
        assert_mutation_inverse_law(&base, &delete_block("b1".into()));
        assert_mutation_inverse_law(&base, &delete_blocks(vec!["b1".into(), "b3".into()]));
        let dup = crate::artifacts::note::schema::clone_block(base.blocks.iter().find(|b| crate::artifacts::note::schema::block_id(b) == "b1").unwrap());
        assert_mutation_inverse_law(&base, &duplicate_block("b1".into(), dup));
    }

    #[semio_framework_async_macros::async_test]
    async fn block_reparent_and_drag_inverse_law() {
        let mut base = sample_snapshot();
        base.blocks.push(NoteBlockNode::Group { id: "g1".into(), name: "Group".into(), x: 0.0, y: 0.0, width: 200.0, height: 200.0, rotation: 0.0, visible: true, locked: false, children: Vec::new() });
        assert_mutation_inverse_law(&base, &move_block_to_container("b1".into(), Some("g1".into()), 0));
        assert_mutation_inverse_law(&base, &drag_blocks(vec!["b1".into(), "b2".into()], 5.0, -3.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn block_field_inverse_laws() {
        let base = sample_snapshot();
        assert_mutation_inverse_law(&base, &rename_block("b1".into(), "Renamed".into()));
        assert_mutation_inverse_law(&base, &change_block_visible("b1".into(), false));
        assert_mutation_inverse_law(&base, &change_block_locked("b1".into(), true));
        assert_mutation_inverse_law(&base, &move_block("b1".into(), 42.0, -8.0));
        assert_mutation_inverse_law(&base, &resize_block("b1".into(), 120.0, 60.0));
        assert_mutation_inverse_law(&base, &change_block_font_size("b1".into(), 24.0));
        assert_mutation_inverse_law(&base, &edit_block_text("b1".into(), vec![crate::artifacts::note::NoteTextParagraph { runs: Vec::new() }]));
        assert_mutation_inverse_law(&base, &edit_block_math("b4".into(), "y = mx + b".into()));
        assert_mutation_inverse_law(&base, &change_block_ink_width("b2".into(), 6.0));
        assert_mutation_inverse_law(&base, &edit_block_ink_stroke("b2".into(), vec![[0.0, 0.0], [1.0, 1.0]], 1.0, 2.0, 10.0, 10.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn table_row_column_inverse_laws() {
        let base = sample_snapshot();
        assert_mutation_inverse_law(&base, &insert_table_row("b3".into()));
        assert_mutation_inverse_law(&base, &remove_table_row("b3".into()));
        assert_mutation_inverse_law(&base, &insert_table_column("b3".into()));
        assert_mutation_inverse_law(&base, &remove_table_column("b3".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_delete_block_round_trip_grows_and_shrinks_projection() {
        let base = sample_snapshot();
        let new_block = NoteBlockNode::Text {
            id: "b100".into(),
            name: "New".into(),
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            content: crate::artifacts::note::note_text_child_handle_and_cache("b100", &[]),
            font_size: 18.0,
            font_weight: "normal".into(),
            align: "left".into(),
        };
        let added = round_trip(&base, &create_block(new_block, None, None));
        assert_eq!(added.blocks.len(), base.blocks.len() + 1);
        let removed = round_trip(&added, &delete_block("b100".into()));
        assert_eq!(removed.blocks.len(), base.blocks.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_block_at_a_non_last_index_restores_exact_position_on_undo() {
        let base = sample_snapshot();
        // b1 is index 0 of 4; deleting then undoing must restore it there, not append it at the end.
        round_trip(&base, &delete_block("b1".into()));
    }
    //#endregion 🔖️MutationLaws

    //#region 🔖️OutcomeLaws
    /// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
    /// one `assert_missing_target_is_error`/Fatal check per verb family this facet implements
    /// (create/delete(s)/rename/change/move/resize/drag/duplicate/insert/remove/edit/replace).
    #[semio_framework_async_macros::async_test]
    async fn create_block_duplicate_id_is_fatal() {
        let base = sample_snapshot();
        let existing = NoteBlockNode::Text {
            id: "b1".into(),
            name: "Dup".into(),
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            content: crate::artifacts::note::note_text_child_handle_and_cache("b1", &[]),
            font_size: 18.0,
            font_weight: "normal".into(),
            align: "left".into(),
        };
        let outcome = create_block(existing, None, None).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_block_missing_target_is_error() {
        let base = sample_snapshot();
        assert_missing_target_is_error(&base, &delete_block("ghost".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_blocks_missing_target_is_error() {
        let base = sample_snapshot();
        assert_missing_target_is_error(&base, &delete_blocks(vec!["ghost".into()]));
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_block_missing_target_is_error() {
        let base = sample_snapshot();
        assert_missing_target_is_error(&base, &rename_block("ghost".into(), "x".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn change_block_locked_missing_target_is_error() {
        let base = sample_snapshot();
        assert_missing_target_is_error(&base, &change_block_locked("ghost".into(), true));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_block_missing_target_is_error() {
        let base = sample_snapshot();
        assert_missing_target_is_error(&base, &move_block("ghost".into(), 1.0, 1.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_block_non_finite_is_fatal() {
        let base = sample_snapshot();
        let outcome = move_block("b1".into(), f64::NAN, 0.0).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[semio_framework_async_macros::async_test]
    async fn resize_block_missing_target_is_error() {
        let base = sample_snapshot();
        assert_missing_target_is_error(&base, &resize_block("ghost".into(), 10.0, 10.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn drag_blocks_missing_target_is_error() {
        let base = sample_snapshot();
        assert_missing_target_is_error(&base, &drag_blocks(vec!["ghost".into()], 1.0, 1.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn duplicate_block_missing_source_is_error() {
        let base = sample_snapshot();
        let block = NoteBlockNode::Text {
            id: "b101".into(),
            name: "New".into(),
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            content: crate::artifacts::note::note_text_child_handle_and_cache("b101", &[]),
            font_size: 18.0,
            font_weight: "normal".into(),
            align: "left".into(),
        };
        assert_missing_target_is_error(&base, &duplicate_block("ghost".into(), block));
    }

    #[semio_framework_async_macros::async_test]
    async fn insert_table_row_missing_target_is_error() {
        let base = sample_snapshot();
        assert_missing_target_is_error(&base, &insert_table_row("ghost".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_table_row_missing_target_is_error() {
        let base = sample_snapshot();
        assert_missing_target_is_error(&base, &remove_table_row("ghost".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn edit_block_text_missing_target_is_error() {
        let base = sample_snapshot();
        assert_missing_target_is_error(&base, &edit_block_text("ghost".into(), Vec::new()));
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_asset_payload_missing_target_is_error() {
        let base = sample_snapshot();
        let asset = NoteImageAsset { mime: "image/jpeg".into(), data: "e".into(), width: None, height: None };
        assert_missing_target_is_error(&base, &replace_asset_payload("ghost".into(), asset));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_asset_duplicate_id_is_fatal() {
        let base = sample_snapshot();
        let asset = NoteImageAsset { mime: "image/png".into(), data: "d".into(), width: None, height: None };
        let outcome = create_asset("asset-1".into(), asset).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_asset_missing_target_is_error() {
        let base = sample_snapshot();
        assert_missing_target_is_error(&base, &delete_asset("ghost".into()));
    }
    //#endregion 🔖️OutcomeLaws
}
//#endregion 🧪️Tests
