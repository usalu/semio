//! 📜️ Block 3D artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::Block3dSnapshot;

/// 📄️ The `nakagin-capsule` example fixture, handcrafted in the `.block3d` DSL — the `ObjectKind` half
/// of semio_compose_rs's metabolism-kit `Capsule` type.
pub const BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🏢️nakagin-capsule/🖼️assets/🏢️nakagin-capsule/🗣️.dsl.semio");
/// 📄️ The `hexagonal-cut-concrete-forest-left` example fixture, handcrafted in the `.block3d` DSL.
pub const BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🌲️hexagonal-cut-concrete-forest/🗣️.dsl.semio");

/// 📖️ Parses `.block3d` DSL text into a `Block3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Block3dSnapshot, semio_framework_diagnostic::TextError> {
    <Block3dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Block3dSnapshot` back to `.block3d` DSL text.
pub fn print_dsl(document: &Block3dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

/// 🚀️ The document every `block3d` surface boots with — `hexagonal-cut-concrete-forest-left`, the only
/// built-in example whose every representation `mesh_url` resolves against the delivery catalog
/// (`🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json` and its nested `🌱️metabolism` collection),
/// so the `World3d` surface renders a real mesh instead of an empty scene on first paint. Falls back to
/// the empty document if the embedded fixture ever stops parsing — a boot must never fault on a fixture.
pub fn block3d_boot_snapshot() -> Block3dSnapshot {
    parse_dsl(BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).unwrap_or_default()
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Block3dSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{Block3dVortexKindExtra, Block3dVortexTemplate, BLOCK_3D_SCHEMA};
use crate::{BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use ::semio_framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for Block3dSnapshot {
    const EXTENSION: &'static str = "block3d";
    fn envelope_id() -> &'static str {
        "block.block3d"
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
