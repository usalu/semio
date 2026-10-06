//! 📜️ Note artifact — textual document grammar surface + laws (constitutional: dsl).

use crate::NoteSnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📄️ The `semio` example document, handcrafted in the `.note` DSL.
pub const SEMIO_NOTE_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.note` DSL text into a `NoteSnapshot`.
pub fn parse_dsl(text: &str) -> Result<NoteSnapshot, semio_framework_diagnostic::TextError> {
    <NoteSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `NoteSnapshot` back to `.note` DSL text.
pub fn print_dsl(document: &NoteSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;

//#region 🔖️ExternalBridges
/// 📖️ Parses `.note` DSL text with a plain-`String` error, reachable from OUTSIDE this crate —
/// `store` is a private `extern crate` alias (`🦀️.rs`), so `store::TextError` cannot be named
/// by the exhaustive mutation case's test adapter that has to read the committed
/// `🗣️.dsl.semio` artifact.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn parse_note_dsl(text: &str) -> Result<NoteSnapshot, String> {
    <NoteSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 🖨️ Prints a [`NoteSnapshot`] back to `.note` DSL text under a name an external caller can reach, paired
/// with [`parse_note_dsl`].
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn print_note_dsl(snapshot: &NoteSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🔖️ExternalBridges

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::NoteDiff;
use crate::NoteSnapshot;
use protocol::{Mutation, MutationDiff};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::standards::v1::subsets::asset::schema::mutations::create_asset::{create_asset, CreateAsset};
use crate::standards::v1::subsets::asset::schema::mutations::delete_asset::{delete_asset, DeleteAsset};
use crate::standards::v1::subsets::asset::schema::mutations::replace_asset_payload::{replace_asset_payload, ReplaceAssetPayload};
use crate::standards::v1::subsets::block::schema::mutations::change_block_font_size::{change_block_font_size, ChangeBlockFontSize};
use crate::standards::v1::subsets::block::schema::mutations::change_block_locked::{change_block_locked, ChangeBlockLocked};
use crate::standards::v1::subsets::block::schema::mutations::change_block_visible::{change_block_visible, ChangeBlockVisible};
use crate::standards::v1::subsets::block::schema::mutations::create_block::{create_block, CreateBlock};
use crate::standards::v1::subsets::block::schema::mutations::delete_block::{delete_block, DeleteBlock};
use crate::standards::v1::subsets::block::schema::mutations::delete_blocks::{delete_blocks, DeleteBlocks};
use crate::standards::v1::subsets::block::schema::mutations::drag_blocks::{drag_blocks, DragBlocks};
use crate::standards::v1::subsets::block::schema::mutations::duplicate_block::{duplicate_block, DuplicateBlock};
use crate::standards::v1::subsets::block::schema::mutations::duplicate_blocks::{duplicate_blocks, DuplicateBlocks};
use crate::standards::v1::subsets::block::schema::mutations::move_block::{move_block, MoveBlock};
use crate::standards::v1::subsets::block::schema::mutations::move_block_to_container::{move_block_to_container, MoveBlockToContainer};
use crate::standards::v1::subsets::block::schema::mutations::rename_block::{rename_block, RenameBlock};
use crate::standards::v1::subsets::block::schema::mutations::resize_block::{resize_block, ResizeBlock};
use crate::standards::v1::subsets::canvas::schema::mutations::change_grid_opacity::{change_grid_opacity, ChangeGridOpacity};
use crate::standards::v1::subsets::canvas::schema::mutations::change_grid_spacing::{change_grid_spacing, ChangeGridSpacing};
use crate::standards::v1::subsets::canvas::schema::mutations::change_grid_subdivisions::{change_grid_subdivisions, ChangeGridSubdivisions};
use crate::standards::v1::subsets::canvas::schema::mutations::change_grid_visible::{change_grid_visible, ChangeGridVisible};
use crate::standards::v1::subsets::canvas::schema::mutations::change_snap_enabled::{change_snap_enabled, ChangeSnapEnabled};
use crate::standards::v1::subsets::canvas::schema::mutations::change_snap_grid_spacing::{change_snap_grid_spacing, ChangeSnapGridSpacing};
use crate::standards::v1::subsets::document::schema::mutations::rename_note::{rename_note, RenameNote};
use crate::standards::v1::subsets::ink::schema::mutations::change_block_ink_width::{change_block_ink_width, ChangeBlockInkWidth};
use crate::standards::v1::subsets::ink::schema::mutations::change_eraser_radius::{change_eraser_radius, ChangeEraserRadius};
use crate::standards::v1::subsets::ink::schema::mutations::change_pencil_width::{change_pencil_width, ChangePencilWidth};
use crate::standards::v1::subsets::ink::schema::mutations::edit_block_ink_stroke::{edit_block_ink_stroke, EditBlockInkStroke};
use crate::standards::v1::subsets::math::schema::mutations::edit_block_math::{edit_block_math, EditBlockMath};
use crate::standards::v1::subsets::table::schema::mutations::insert_table_column::{insert_table_column, InsertTableColumn};
use crate::standards::v1::subsets::table::schema::mutations::insert_table_row::{insert_table_row, InsertTableRow};
use crate::standards::v1::subsets::table::schema::mutations::remove_table_column::{remove_table_column, RemoveTableColumn};
use crate::standards::v1::subsets::table::schema::mutations::remove_table_row::{remove_table_row, RemoveTableRow};
use crate::standards::v1::subsets::text::schema::mutations::edit_block_text::{edit_block_text, EditBlockText};

/// 📥️ Decodes a committed `📸️snapshot/{⬅️before,➡️after}/🔣️.json` vector.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_note_snapshot_json(text: &str) -> Result<NoteSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📤️ The snapshot as the same canonical JSON the committed vectors are written in — the
/// projection an external test host compares through.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn encode_note_snapshot_json(snapshot: &NoteSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}
}
pub use mutations_codec::*;


#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{NoteBlockNode, NoteImageAsset, NOTE_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for NoteSnapshot {
    const EXTENSION: &'static str = "note";
    fn envelope_id() -> &'static str {
        "note.note"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::*;
use crate::{NoteTableCell, NoteTextParagraph, NoteTextRun, NOTE_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::NoteBlockNode;
use crate::NoteImageAsset;

/// 📄️ The `semio` example, parsed once from {@link SEMIO_NOTE_EXAMPLE_TEXT} — the source of truth for
/// every "semio" example call site (`setActiveExample`, tests). Falls back to the empty document if the
/// fixture ever fails to parse, matching the old JSON fixture's failure behavior.
pub fn semio_example_snapshot() -> crate::NoteSnapshot {
    <crate::NoteSnapshot as store::ArtifactDsl>::parse_dsl(SEMIO_NOTE_EXAMPLE_TEXT).unwrap_or_else(|_| empty_note_snapshot())
}

/// 📄️ JSON re-serialization of {@link semio_example_snapshot}, for the framework-generic call sites that
/// contractually require JSON text (`PluginApp::render`'s `projection_override_json`, `App::example`'s
/// manifest `document_json`).
pub fn semio_example_json() -> String {
    semio_framework_pack_json::to_json_string(&semio_example_snapshot())
}

pub fn empty_note_snapshot() -> crate::NoteSnapshot {
    crate::NoteSnapshot {
        schema: NOTE_DOCUMENT_SCHEMA.into(),
        id: "empty".into(),
        title: None,
        blocks: Vec::new(),
        grid_visible: Some(true),
        grid_spacing: Some(32.0),
        grid_subdivisions: Some(4.0),
        grid_opacity: Some(0.35),
        snap_enabled: Some(false),
        snap_grid_spacing: Some(8.0),
        pencil_width: Some(3.0),
        eraser_radius: Some(12.0),
        assets: BTreeMap::new(),
        linked_artifact: None,
    }
}
}
pub use snapshot_wire_codec::*;
