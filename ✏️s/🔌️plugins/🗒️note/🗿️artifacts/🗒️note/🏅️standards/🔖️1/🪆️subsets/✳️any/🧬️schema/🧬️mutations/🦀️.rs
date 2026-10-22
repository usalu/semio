//! 🧬️ Note artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! triad leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<NoteSnapshot>`
//! and `impl protocol::SemanticMutation<NoteSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here (the old hand-written `match`-per-variant `diff`/`inverse` are
//! retired along with the 8 bare generic-verb scalar setters, the whole-collection block-list `Vec`
//! setter, the put-synonym asset upsert, and the whole-document-replace escape hatch — see
//! `📓️taxonomy.md`/`📓️derivation-rules.md` in ticket `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`).

use semio_s_artifact_note_note_diff::standards::v1::subsets::any::schema::diff::NoteDiff;
use crate::NoteSnapshot;
use protocol::{ColdMutation, Mutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutations
/// 🧮️ Semantic note document mutation vocabulary, derived from `🧬️schema/📸️snapshot/🦀️.rs`:
/// 9 document-root scalars (`rename-note` for the identity field, `change-*` for the 8 grid/snap/
/// tool settings), 3 id-keyed asset mutations (`create`/`replace-payload`/`delete`), and 21 block
/// mutations over the id-keyed, z-order-meaningful, group-nestable block tree (create/delete(s)/
/// duplicate(s)/reparent/drag/rename/visible/locked/move/resize, plus per-kind content edits for
/// text/math/ink, plus table row/column insert/remove). Whole-document replace has NO replacement
/// here — see `crate::editor::note::support::reset_document_effect`, which goes through
/// `Effect::LoadDocument` outside undo history.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = NoteSnapshot, diff = NoteDiff, schema = "note.note", reference = NoteSnapshot, reference_diff = NoteDiff)]
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
pub use crate::standards::v1::subsets::asset::schema::mutations::create_asset::CreateAsset;
pub use crate::standards::v1::subsets::asset::schema::mutations::delete_asset::DeleteAsset;
pub use crate::standards::v1::subsets::asset::schema::mutations::replace_asset_payload::ReplaceAssetPayload;
pub use crate::standards::v1::subsets::block::schema::mutations::change_block_font_size::ChangeBlockFontSize;
pub use crate::standards::v1::subsets::block::schema::mutations::change_block_locked::ChangeBlockLocked;
pub use crate::standards::v1::subsets::block::schema::mutations::change_block_visible::ChangeBlockVisible;
pub use crate::standards::v1::subsets::block::schema::mutations::create_block::CreateBlock;
pub use crate::standards::v1::subsets::block::schema::mutations::delete_block::DeleteBlock;
pub use crate::standards::v1::subsets::block::schema::mutations::delete_blocks::DeleteBlocks;
pub use crate::standards::v1::subsets::block::schema::mutations::drag_blocks::DragBlocks;
pub use crate::standards::v1::subsets::block::schema::mutations::duplicate_block::DuplicateBlock;
pub use crate::standards::v1::subsets::block::schema::mutations::duplicate_blocks::DuplicateBlocks;
pub use crate::standards::v1::subsets::block::schema::mutations::move_block::MoveBlock;
pub use crate::standards::v1::subsets::block::schema::mutations::move_block_to_container::MoveBlockToContainer;
pub use crate::standards::v1::subsets::block::schema::mutations::rename_block::RenameBlock;
pub use crate::standards::v1::subsets::block::schema::mutations::resize_block::ResizeBlock;
pub use crate::standards::v1::subsets::canvas::schema::mutations::change_grid_opacity::ChangeGridOpacity;
pub use crate::standards::v1::subsets::canvas::schema::mutations::change_grid_spacing::ChangeGridSpacing;
pub use crate::standards::v1::subsets::canvas::schema::mutations::change_grid_subdivisions::ChangeGridSubdivisions;
pub use crate::standards::v1::subsets::canvas::schema::mutations::change_grid_visible::ChangeGridVisible;
pub use crate::standards::v1::subsets::canvas::schema::mutations::change_snap_enabled::ChangeSnapEnabled;
pub use crate::standards::v1::subsets::canvas::schema::mutations::change_snap_grid_spacing::ChangeSnapGridSpacing;
pub use crate::standards::v1::subsets::document::schema::mutations::rename_note::RenameNote;
pub use crate::standards::v1::subsets::ink::schema::mutations::change_block_ink_width::ChangeBlockInkWidth;
pub use crate::standards::v1::subsets::ink::schema::mutations::change_eraser_radius::ChangeEraserRadius;
pub use crate::standards::v1::subsets::ink::schema::mutations::change_pencil_width::ChangePencilWidth;
pub use crate::standards::v1::subsets::ink::schema::mutations::edit_block_ink_stroke::EditBlockInkStroke;
pub use crate::standards::v1::subsets::math::schema::mutations::edit_block_math::EditBlockMath;
pub use crate::standards::v1::subsets::table::schema::mutations::insert_table_column::InsertTableColumn;
pub use crate::standards::v1::subsets::table::schema::mutations::insert_table_row::InsertTableRow;
pub use crate::standards::v1::subsets::table::schema::mutations::remove_table_column::RemoveTableColumn;
pub use crate::standards::v1::subsets::table::schema::mutations::remove_table_row::RemoveTableRow;
pub use crate::standards::v1::subsets::text::schema::mutations::edit_block_text::EditBlockText;
//#endregion 🔖️Reexports

//#region 🔖️Helpers
#[path = "🚪️io/🦀️.rs"]
mod apply_boundary;
pub use apply_boundary::{apply_note_mutation, apply_note_mutation_outcome};

pub fn inverse_note_mutation(snapshot: &NoteSnapshot, mutation: &NoteMutation) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(snapshot)?

    })
}

//#endregion 🔖️Helpers

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every [`NoteMutation`] variant, in declaration order — the vocabulary the
/// `note-1-any` mutation catalog (`../../🔮️oracles/🔣️.json`) declares and the
/// exhaustive `mutate-*` case measures itself against (9 document-root scalars, 3 asset-pool kinds and 21 block-tree kinds). The framework never
/// parses Rust, so `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest
/// against both the enum and the committed catalog.
pub const KINDS: &[&str] = &[
    "rename-note",
    "change-grid-visible",
    "change-grid-spacing",
    "change-grid-subdivisions",
    "change-grid-opacity",
    "change-snap-enabled",
    "change-snap-grid-spacing",
    "change-pencil-width",
    "change-eraser-radius",
    "create-asset",
    "replace-asset-payload",
    "delete-asset",
    "create-block",
    "delete-block",
    "delete-blocks",
    "duplicate-block",
    "duplicate-blocks",
    "move-block-to-container",
    "drag-blocks",
    "rename-block",
    "change-block-visible",
    "change-block-locked",
    "move-block",
    "resize-block",
    "change-block-font-size",
    "edit-block-text",
    "edit-block-math",
    "change-block-ink-width",
    "edit-block-ink-stroke",
    "insert-table-row",
    "remove-table-row",
    "insert-table-column",
    "remove-table-column",
];

/// ↩️ `mutation`'s own inverse against `base`, as the step LIST `protocol::Mutation::inverse`
/// returns. Reachable from outside this crate, which `protocol::Mutation` itself is not — the
/// `protocol` extern-crate alias is private to `🦀️.rs`.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn inverse_note_mutation_steps(mutation: &NoteMutation, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok({
    <NoteMutation as ColdMutation<NoteSnapshot>>::inverse(mutation, base)?

    })
}






//#endregion 🔖️Kinds

//#region 🧪️KindsCatalog

//#endregion 🧪️KindsCatalog

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️ConcreteBuilders
/// 🏗️ Builder of the `change_block_font_size` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_block_font_size(id: String, new_font_size: f64) -> NoteMutation {
    crate::standards::v1::subsets::block::schema::mutations::change_block_font_size::change_block_font_size::<NoteMutation>(id, new_font_size)
}

/// 🏗️ Builder of the `change_block_ink_width` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_block_ink_width(id: String, new_stroke_width: f64) -> NoteMutation {
    crate::standards::v1::subsets::ink::schema::mutations::change_block_ink_width::change_block_ink_width::<NoteMutation>(id, new_stroke_width)
}

/// 🏗️ Builder of the `change_block_locked` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_block_locked(id: String, new_locked: bool) -> NoteMutation {
    crate::standards::v1::subsets::block::schema::mutations::change_block_locked::change_block_locked::<NoteMutation>(id, new_locked)
}

/// 🏗️ Builder of the `change_block_visible` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_block_visible(id: String, new_visible: bool) -> NoteMutation {
    crate::standards::v1::subsets::block::schema::mutations::change_block_visible::change_block_visible::<NoteMutation>(id, new_visible)
}

/// 🏗️ Builder of the `change_eraser_radius` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_eraser_radius(new_radius: Option<f64>) -> NoteMutation {
    crate::standards::v1::subsets::ink::schema::mutations::change_eraser_radius::change_eraser_radius::<NoteMutation>(new_radius)
}

/// 🏗️ Builder of the `change_grid_opacity` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_grid_opacity(new_opacity: Option<f64>) -> NoteMutation {
    crate::standards::v1::subsets::canvas::schema::mutations::change_grid_opacity::change_grid_opacity::<NoteMutation>(new_opacity)
}

/// 🏗️ Builder of the `change_grid_spacing` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_grid_spacing(new_spacing: Option<f64>) -> NoteMutation {
    crate::standards::v1::subsets::canvas::schema::mutations::change_grid_spacing::change_grid_spacing::<NoteMutation>(new_spacing)
}

/// 🏗️ Builder of the `change_grid_subdivisions` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_grid_subdivisions(new_subdivisions: Option<f64>) -> NoteMutation {
    crate::standards::v1::subsets::canvas::schema::mutations::change_grid_subdivisions::change_grid_subdivisions::<NoteMutation>(new_subdivisions)
}

/// 🏗️ Builder of the `change_grid_visible` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_grid_visible(new_visible: Option<bool>) -> NoteMutation {
    crate::standards::v1::subsets::canvas::schema::mutations::change_grid_visible::change_grid_visible::<NoteMutation>(new_visible)
}

/// 🏗️ Builder of the `change_pencil_width` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_pencil_width(new_width: Option<f64>) -> NoteMutation {
    crate::standards::v1::subsets::ink::schema::mutations::change_pencil_width::change_pencil_width::<NoteMutation>(new_width)
}

/// 🏗️ Builder of the `change_snap_enabled` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_snap_enabled(new_enabled: Option<bool>) -> NoteMutation {
    crate::standards::v1::subsets::canvas::schema::mutations::change_snap_enabled::change_snap_enabled::<NoteMutation>(new_enabled)
}

/// 🏗️ Builder of the `change_snap_grid_spacing` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_snap_grid_spacing(new_spacing: Option<f64>) -> NoteMutation {
    crate::standards::v1::subsets::canvas::schema::mutations::change_snap_grid_spacing::change_snap_grid_spacing::<NoteMutation>(new_spacing)
}

/// 🏗️ Builder of the `create_asset` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn create_asset(key: String, asset: crate::NoteImageAsset) -> NoteMutation {
    crate::standards::v1::subsets::asset::schema::mutations::create_asset::create_asset::<NoteMutation>(key, asset)
}

/// 🏗️ Builder of the `create_block` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn create_block(block: crate::NoteBlockNode, parent_id: Option<String>, index: Option<usize>) -> NoteMutation {
    crate::standards::v1::subsets::block::schema::mutations::create_block::create_block::<NoteMutation>(block, parent_id, index)
}

/// 🏗️ Builder of the `delete_asset` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn delete_asset(key: String) -> NoteMutation {
    crate::standards::v1::subsets::asset::schema::mutations::delete_asset::delete_asset::<NoteMutation>(key)
}

/// 🏗️ Builder of the `delete_block` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn delete_block(id: String) -> NoteMutation {
    crate::standards::v1::subsets::block::schema::mutations::delete_block::delete_block::<NoteMutation>(id)
}

/// 🏗️ Builder of the `delete_blocks` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn delete_blocks(ids: Vec<String>) -> NoteMutation {
    crate::standards::v1::subsets::block::schema::mutations::delete_blocks::delete_blocks::<NoteMutation>(ids)
}

/// 🏗️ Builder of the `drag_blocks` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn drag_blocks(ids: Vec<String>, dx: f64, dy: f64) -> NoteMutation {
    crate::standards::v1::subsets::block::schema::mutations::drag_blocks::drag_blocks::<NoteMutation>(ids, dx, dy)
}

/// 🏗️ Builder of the `duplicate_block` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn duplicate_block(source_id: String, block: crate::NoteBlockNode) -> NoteMutation {
    crate::standards::v1::subsets::block::schema::mutations::duplicate_block::duplicate_block::<NoteMutation>(source_id, block)
}

/// 🏗️ Builder of the `duplicate_blocks` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn duplicate_blocks(source_ids: Vec<String>, blocks: Vec<crate::NoteBlockNode>) -> NoteMutation {
    crate::standards::v1::subsets::block::schema::mutations::duplicate_blocks::duplicate_blocks::<NoteMutation>(source_ids, blocks)
}

/// 🏗️ Builder of the `edit_block_ink_stroke` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn edit_block_ink_stroke(id: String, new_points: Vec<[f64; 2]>, new_x: f64, new_y: f64, new_width: f64, new_height: f64) -> NoteMutation {
    crate::standards::v1::subsets::ink::schema::mutations::edit_block_ink_stroke::edit_block_ink_stroke::<NoteMutation>(id, new_points, new_x, new_y, new_width, new_height)
}

/// 🏗️ Builder of the `edit_block_math` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn edit_block_math(id: String, new_tex: String) -> NoteMutation {
    crate::standards::v1::subsets::math::schema::mutations::edit_block_math::edit_block_math::<NoteMutation>(id, new_tex)
}

/// 🏗️ Builder of the `edit_block_text` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn edit_block_text(id: String, new_paragraphs: Vec<crate::NoteTextParagraph>) -> NoteMutation {
    crate::standards::v1::subsets::text::schema::mutations::edit_block_text::edit_block_text::<NoteMutation>(id, new_paragraphs)
}

/// 🏗️ Builder of the `insert_table_column` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn insert_table_column(id: String) -> NoteMutation {
    crate::standards::v1::subsets::table::schema::mutations::insert_table_column::insert_table_column::<NoteMutation>(id)
}

/// 🏗️ Builder of the `insert_table_row` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn insert_table_row(id: String) -> NoteMutation {
    crate::standards::v1::subsets::table::schema::mutations::insert_table_row::insert_table_row::<NoteMutation>(id)
}

/// 🏗️ Builder of the `move_block` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn move_block(id: String, new_x: f64, new_y: f64) -> NoteMutation {
    crate::standards::v1::subsets::block::schema::mutations::move_block::move_block::<NoteMutation>(id, new_x, new_y)
}

/// 🏗️ Builder of the `move_block_to_container` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn move_block_to_container(id: String, new_parent_id: Option<String>, index: usize) -> NoteMutation {
    crate::standards::v1::subsets::block::schema::mutations::move_block_to_container::move_block_to_container::<NoteMutation>(id, new_parent_id, index)
}

/// 🏗️ Builder of the `remove_table_column` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn remove_table_column(id: String) -> NoteMutation {
    crate::standards::v1::subsets::table::schema::mutations::remove_table_column::remove_table_column::<NoteMutation>(id)
}

/// 🏗️ Builder of the `remove_table_row` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn remove_table_row(id: String) -> NoteMutation {
    crate::standards::v1::subsets::table::schema::mutations::remove_table_row::remove_table_row::<NoteMutation>(id)
}

/// 🏗️ Builder of the `rename_block` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn rename_block(id: String, new_name: String) -> NoteMutation {
    crate::standards::v1::subsets::block::schema::mutations::rename_block::rename_block::<NoteMutation>(id, new_name)
}

/// 🏗️ Builder of the `rename_note` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn rename_note(new_title: Option<String>) -> NoteMutation {
    crate::standards::v1::subsets::document::schema::mutations::rename_note::rename_note::<NoteMutation>(new_title)
}

/// 🏗️ Builder of the `replace_asset_payload` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn replace_asset_payload(key: String, new_asset: crate::NoteImageAsset) -> NoteMutation {
    crate::standards::v1::subsets::asset::schema::mutations::replace_asset_payload::replace_asset_payload::<NoteMutation>(key, new_asset)
}

/// 🏗️ Builder of the `resize_block` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn resize_block(id: String, new_width: f64, new_height: f64) -> NoteMutation {
    crate::standards::v1::subsets::block::schema::mutations::resize_block::resize_block::<NoteMutation>(id, new_width, new_height)
}
//#endregion 🔖️ConcreteBuilders
