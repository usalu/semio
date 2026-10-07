//! 📜️ Block 2D artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::Block2dSnapshot;

/// 📄️ The `hexagonal-cut-concrete-forest-left` example fixture, handcrafted in the `.block2d` DSL —
/// the `NodeKind` half of `s/plugin/puzzle/app/2d/manifest/🔣️.json`.
pub const BLOCK2D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🌲️hexagonal-cut-concrete-forest/🗣️.dsl.semio");
/// 📄️ The `hexagonal-cut-concrete-forest-right` example fixture, handcrafted in the `.block2d` DSL.
pub const BLOCK2D_CONCRETE_FOREST_RIGHT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/➡️hexagonal-cut-concrete-forest-right/🖼️assets/➡️hexagonal-cut-concrete-forest/🗣️.dsl.semio");

/// 📖️ Parses `.block2d` DSL text into a `Block2dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Block2dSnapshot, semio_framework_diagnostic::TextError> {
    <Block2dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Block2dSnapshot` back to `.block2d` DSL text.
pub fn print_dsl(document: &Block2dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Block2dSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{Block2dHandleKind, Block2dHandleTemplate, Block2dPresentation, BLOCK_2D_SCHEMA};
use crate::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta};
use ::semio_framework_schema::ArtifactSchema;

/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for Block2dSnapshot {
    const EXTENSION: &'static str = "block2d";
    fn envelope_id() -> &'static str {
        "block.block2d"
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
use crate::standards::v1::subsets::any::schema::*;
use crate::{Block2dSnapshot};
use ::semio_framework_schema::ArtifactSchema;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::BlockKindIdentity;
use crate::Block2dPresentation;
use crate::Block2dHandleKind;
use crate::Block2dHandleTemplate;
use crate::BlockCompatibilityRule;
use crate::BlockAttribute;
use crate::BlockAuthor;
use crate::BlockCamera2d;
use crate::BlockMeta;

/// 📸️ A fresh, empty `Block2dSnapshot` (all fields at their `Default`).
pub fn empty_block2d_snapshot() -> Block2dSnapshot {
    Block2dSnapshot::default()
}

/// 📄️ The block2d boot document — parsed from the bundled `hexagonal-cut-concrete-forest-left`
/// example fixture (the same DSL text `setActiveExample` loads), falling back to the empty snapshot
/// only when that fixture fails to parse. Shared by `Block2dPlayApp` and `Block2dViewer`.
pub fn default_block2d_snapshot() -> Block2dSnapshot {
    crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(crate::standards::v1::subsets::any::io::text::snapshot::BLOCK2D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).unwrap_or_else(|_| empty_block2d_snapshot())
}
}
pub use snapshot_wire_codec::*;
