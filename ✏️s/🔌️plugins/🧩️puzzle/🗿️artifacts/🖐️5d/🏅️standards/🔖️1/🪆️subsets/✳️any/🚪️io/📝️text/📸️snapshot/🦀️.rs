//! 🗣️ Puzzle 5d artifact — the textual `.puzzle5d` document grammar surface and its laws, plus the
//! two handcrafted example fixtures the play app's example picker loads.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::Puzzle5dSnapshot;

/// 📄️ The `concrete-forest` example fixture, handcrafted in the `.puzzle5d` DSL.
pub const PUZZLE5D_CONCRETE_FOREST_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🌲️concrete-forest/🖼️assets/🌲️forest/🗣️.dsl.semio");
/// 📄️ The `nakagin-capsule-tower` example fixture, handcrafted in the `.puzzle5d` DSL.
pub const PUZZLE5D_NAKAGIN_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🏢️tower/🗣️.dsl.semio");
pub const PUZZLE5D_CAPSULE_DREAM_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🌙️capsule-dream/🖼️assets/🌙️dream/🗣️.dsl.semio");

/// 📖️ Parses `.puzzle5d` DSL text into a `Puzzle5dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Puzzle5dSnapshot, semio_framework_diagnostic::TextError> {
    <Puzzle5dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Puzzle5dSnapshot` back to `.puzzle5d` DSL text.
pub fn print_dsl(document: &Puzzle5dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Puzzle5dSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{Puzzle5dFastener, Puzzle5dKindCompatibility, Puzzle5dMeta, Puzzle5dPart, Puzzle5dTargetVolume, PUZZLE_5D_SCHEMA};
use ::semio_framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::Puzzle5dKindCatalogsExtra;

/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for Puzzle5dSnapshot {
    const EXTENSION: &'static str = "puzzle5d";
    fn envelope_id() -> &'static str {
        "puzzle.puzzle5d"
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
