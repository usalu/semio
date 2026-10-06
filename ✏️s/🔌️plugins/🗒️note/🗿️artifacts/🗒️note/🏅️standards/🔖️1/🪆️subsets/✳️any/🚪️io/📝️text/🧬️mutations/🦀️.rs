//! 🔧 note — OpText/OpBinary for `NoteMutation`.
use crate::schema::mutations::{apply_note_mutation, NoteMutation};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for NoteMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl protocol::OpBinary for NoteMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

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

/// 📥️ Decodes the internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) projection the
/// committed `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️.json` vectors carry.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_note_mutation_json(text: &str) -> Result<NoteMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
