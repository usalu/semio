//! 📜️ Block 5D artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::Block5dSnapshot;

/// 📄️ The `hexagonal-cut-concrete-forest-left` example fixture, handcrafted in the `.block5d` DSL —
/// the `PartKind` slice of `s/plugin/puzzle/app/5d/example/🧩️concrete-forest.puzzle5d`.
pub const BLOCK5D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🌲️hexagonal-cut-concrete-forest/🗣️.dsl.semio");
/// 📄️ The `nakagin-capsule` example fixture, handcrafted in the `.block5d` DSL.
pub const BLOCK5D_NAKAGIN_CAPSULE_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🏢️nakagin-capsule/🖼️assets/🏢️nakagin-capsule/🗣️.dsl.semio");

/// 📖️ Parses `.block5d` DSL text into a `Block5dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Block5dSnapshot, semio_framework_diagnostic::TextError> {
    <Block5dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Block5dSnapshot` back to `.block5d` DSL text.
pub fn print_dsl(document: &Block5dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Block5dSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d, BLOCK_5D_SCHEMA};
use crate::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use ::semio_framework_schema::ArtifactSchema;

/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for Block5dSnapshot {
    const EXTENSION: &'static str = "block5d";
    fn envelope_id() -> &'static str {
        "block.block5d"
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
use crate::{Block5dSnapshot};
use ::semio_framework_schema::ArtifactSchema;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::BlockKindIdentity;
use crate::Block5dPart2d;
use crate::Block5dPart3d;
use crate::BlockRepresentation;
use crate::Block5dGripKind;
use crate::Block5dGripTemplate;
use crate::BlockCompatibilityRule;
use crate::BlockAttribute;
use crate::BlockAuthor;
use crate::BlockCamera2d;
use crate::BlockCamera3d;
use crate::BlockMeta;

/// 📸️ A fresh, empty `Block5dSnapshot` (all fields at their `Default`).
pub fn empty_block5d_snapshot() -> Block5dSnapshot {
    Block5dSnapshot::default()
}

/// 📄️ The block5d boot document — parsed from the bundled `hexagonal-cut-concrete-forest-left`
/// example fixture (the same DSL text `setActiveExample` loads), falling back to the empty snapshot
/// only when that fixture fails to parse. Shared by `Block5dPlayApp` and `Block5dViewer`.
///
/// 🪪️ The sibling `nakagin-capsule` fixture carries a mesh too, but booting on it would strand
/// `part_kind.id` at `"Capsule J"` for the rest of the session: `Block5dMutation` deliberately has no
/// id verb (see `🧬️mutations/🦀️.rs`'s `KINDS`), so `setActiveExample`'s whole-document load can never
/// carry identity across examples. Booting on the same fixture the id assertions name keeps the boot
/// document and its own identity consistent.
pub fn default_block5d_snapshot() -> Block5dSnapshot {
    crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::BLOCK5D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).unwrap_or_else(|_| empty_block5d_snapshot())
}
}
pub use snapshot_wire_codec::*;
