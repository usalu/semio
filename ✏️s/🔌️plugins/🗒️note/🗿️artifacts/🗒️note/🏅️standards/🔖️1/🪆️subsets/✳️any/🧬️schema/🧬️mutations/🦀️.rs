//! 🧬️ Note artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! triad leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<NoteSnapshot>`
//! and `impl protocol::SemanticMutation<NoteSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here (the old hand-written `match`-per-variant `diff`/`inverse` are
//! retired along with the 8 bare generic-verb scalar setters, the whole-collection block-list `Vec`
//! setter, the put-synonym asset upsert, and the whole-document-replace escape hatch — see
//! `📓️taxonomy.md`/`📓️derivation-rules.md` in ticket `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`).

use crate::NoteDiff;
use crate::NoteSnapshot;
use protocol::{Mutation, MutationDiff};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutations
/// 🧮️ Semantic note document mutation vocabulary, derived from `🧬️schema/📸️snapshot/🦀️.rs`:
/// 9 document-root scalars (`rename-note` for the identity field, `change-*` for the 8 grid/snap/
/// tool settings), 3 id-keyed asset mutations (`create`/`replace-payload`/`delete`), and 21 block
/// mutations over the id-keyed, z-order-meaningful, group-nestable block tree (create/delete(s)/
/// duplicate(s)/reparent/drag/rename/visible/locked/move/resize, plus per-kind content edits for
/// text/math/ink, plus table row/column insert/remove). Whole-document replace has NO replacement
/// here — see `crate::editor::note::reset_document_effect`, which goes through
/// `Effect::LoadDocument` outside undo history.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
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
pub use crate::standards::v1::subsets::asset::schema::mutations::create_asset::{create_asset, CreateAsset};
pub use crate::standards::v1::subsets::asset::schema::mutations::delete_asset::{delete_asset, DeleteAsset};
pub use crate::standards::v1::subsets::asset::schema::mutations::replace_asset_payload::{replace_asset_payload, ReplaceAssetPayload};
pub use crate::standards::v1::subsets::block::schema::mutations::change_block_font_size::{change_block_font_size, ChangeBlockFontSize};
pub use crate::standards::v1::subsets::block::schema::mutations::change_block_locked::{change_block_locked, ChangeBlockLocked};
pub use crate::standards::v1::subsets::block::schema::mutations::change_block_visible::{change_block_visible, ChangeBlockVisible};
pub use crate::standards::v1::subsets::block::schema::mutations::create_block::{create_block, CreateBlock};
pub use crate::standards::v1::subsets::block::schema::mutations::delete_block::{delete_block, DeleteBlock};
pub use crate::standards::v1::subsets::block::schema::mutations::delete_blocks::{delete_blocks, DeleteBlocks};
pub use crate::standards::v1::subsets::block::schema::mutations::drag_blocks::{drag_blocks, DragBlocks};
pub use crate::standards::v1::subsets::block::schema::mutations::duplicate_block::{duplicate_block, DuplicateBlock};
pub use crate::standards::v1::subsets::block::schema::mutations::duplicate_blocks::{duplicate_blocks, DuplicateBlocks};
pub use crate::standards::v1::subsets::block::schema::mutations::move_block::{move_block, MoveBlock};
pub use crate::standards::v1::subsets::block::schema::mutations::move_block_to_container::{move_block_to_container, MoveBlockToContainer};
pub use crate::standards::v1::subsets::block::schema::mutations::rename_block::{rename_block, RenameBlock};
pub use crate::standards::v1::subsets::block::schema::mutations::resize_block::{resize_block, ResizeBlock};
pub use crate::standards::v1::subsets::canvas::schema::mutations::change_grid_opacity::{change_grid_opacity, ChangeGridOpacity};
pub use crate::standards::v1::subsets::canvas::schema::mutations::change_grid_spacing::{change_grid_spacing, ChangeGridSpacing};
pub use crate::standards::v1::subsets::canvas::schema::mutations::change_grid_subdivisions::{change_grid_subdivisions, ChangeGridSubdivisions};
pub use crate::standards::v1::subsets::canvas::schema::mutations::change_grid_visible::{change_grid_visible, ChangeGridVisible};
pub use crate::standards::v1::subsets::canvas::schema::mutations::change_snap_enabled::{change_snap_enabled, ChangeSnapEnabled};
pub use crate::standards::v1::subsets::canvas::schema::mutations::change_snap_grid_spacing::{change_snap_grid_spacing, ChangeSnapGridSpacing};
pub use crate::standards::v1::subsets::document::schema::mutations::rename_note::{rename_note, RenameNote};
pub use crate::standards::v1::subsets::ink::schema::mutations::change_block_ink_width::{change_block_ink_width, ChangeBlockInkWidth};
pub use crate::standards::v1::subsets::ink::schema::mutations::change_eraser_radius::{change_eraser_radius, ChangeEraserRadius};
pub use crate::standards::v1::subsets::ink::schema::mutations::change_pencil_width::{change_pencil_width, ChangePencilWidth};
pub use crate::standards::v1::subsets::ink::schema::mutations::edit_block_ink_stroke::{edit_block_ink_stroke, EditBlockInkStroke};
pub use crate::standards::v1::subsets::math::schema::mutations::edit_block_math::{edit_block_math, EditBlockMath};
pub use crate::standards::v1::subsets::table::schema::mutations::insert_table_column::{insert_table_column, InsertTableColumn};
pub use crate::standards::v1::subsets::table::schema::mutations::insert_table_row::{insert_table_row, InsertTableRow};
pub use crate::standards::v1::subsets::table::schema::mutations::remove_table_column::{remove_table_column, RemoveTableColumn};
pub use crate::standards::v1::subsets::table::schema::mutations::remove_table_row::{remove_table_row, RemoveTableRow};
pub use crate::standards::v1::subsets::text::schema::mutations::edit_block_text::{edit_block_text, EditBlockText};
//#endregion 🔖️Reexports

//#region 🔖️Helpers
/// ▶️ Applies `mutation` via its diff — the sole apply path now (no hand-written match dispatch).
pub fn apply_note_mutation(snapshot: &NoteSnapshot, mutation: &NoteMutation) -> protocol::MutationApplyResult<NoteSnapshot> {
    let (diff, _messages) = mutation.diff(snapshot).into_parts();
    MutationDiff::apply(&diff, snapshot)
}

pub fn inverse_note_mutation(snapshot: &NoteSnapshot, mutation: &NoteMutation) -> Vec<NoteMutation> {
    mutation.inverse(snapshot)
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

/// 🧮️ Applies `mutation` to `base` and hands back the whole `protocol::MutationOutcome`, the
/// diagnostics included — the shape an external conformance host needs, since a committed
/// `🎯️outcome` vector declares a status AND its diagnostic codes, and the plain apply wrapper
/// beside this one answers `Result<_, _>` and drops the messages.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn apply_note_mutation_outcome(snapshot: &mut NoteSnapshot, mutation: &NoteMutation) -> protocol::MutationOutcome<NoteDiff> {
    let outcome = <NoteMutation as Mutation<NoteSnapshot>>::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ `mutation`'s own inverse against `base`, as the step LIST `protocol::Mutation::inverse`
/// returns. Reachable from outside this crate, which `protocol::Mutation` itself is not — the
/// `protocol` extern-crate alias is private to `🦀️.rs`.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn inverse_note_mutation_steps(mutation: &NoteMutation, base: &NoteSnapshot) -> Vec<NoteMutation> {
    <NoteMutation as Mutation<NoteSnapshot>>::inverse(mutation, base)
}

/// 📥️ Decodes the internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) projection the
/// committed `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️.json` vectors carry.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_note_mutation_json(text: &str) -> Result<NoteMutation, String> {
    dsl::os_pack::from_json_str(text).map_err(|error| error.to_string())
}

/// 📥️ Decodes a committed `📸️snapshot/{⬅️before,➡️after}/🔣️.json` vector.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_note_snapshot_json(text: &str) -> Result<NoteSnapshot, String> {
    dsl::os_pack::from_json_str(text).map_err(|error| error.to_string())
}

/// 📤️ The snapshot as the same canonical JSON the committed vectors are written in — the
/// projection an external test host compares through.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn encode_note_snapshot_json(snapshot: &NoteSnapshot) -> String {
    dsl::os_pack::to_json_string(snapshot)
}
//#endregion 🔖️Kinds

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️KindsCatalog

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
