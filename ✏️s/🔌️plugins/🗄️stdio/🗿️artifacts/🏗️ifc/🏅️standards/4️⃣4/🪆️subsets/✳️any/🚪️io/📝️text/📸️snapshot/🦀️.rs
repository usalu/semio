//! 📝️ Text representation codec surface for `stdio.ifc` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type IfcSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use crate::standards::v4::subsets::any::io::sqlite::snapshot::native;
use super::*;
use crate::standards::v4::subsets::any::schema::snapshot::*;
use crate::STDIO_IFC_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_contract::part21::{parse_part21, write_part21, Part21Document, Part21Header, Part21Instance, Part21Value};

impl store::ArtifactDsl for IfcSnapshot {
    const EXTENSION: &'static str = "ifc";
    fn envelope_id() -> &'static str { STDIO_IFC_DOCUMENT_SCHEMA }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> { native::parse_text(text) }
    fn print_dsl(&self) -> String { native::print_text(self) }
}
}
pub use snapshot_codec::*;
