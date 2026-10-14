//! 📝️ Text representation codec surface for `stdio.gltf` (diff).

use crate::standards::v2_0::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
#[cfg(test)]
use crate::standards::v2_0::subsets::any::schema::snapshot::GltfDocument;
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use protocol::MutationDiff;
use crate::standards::v2_0::subsets::any::io::text::diff::wire::{parse_gltf_diff, print_gltf_diff};

#[macro_use]
#[path = "🧵️wire/🦀️.rs"]
pub mod wire;

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
impl protocol::DiffText for GltfDiff {
fn print_diff(&self) -> String {
    print_gltf_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_gltf_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
