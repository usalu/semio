//! 📝️ Text representation codec surface for `stdio.ifc.2x3` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Ifc2x3SnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use crate::standards::v2x3::subsets::base::io::sqlite::snapshot::native;
use super::*;
use crate::standards::v2x3::subsets::base::schema::snapshot::*;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_contract::part21::Part21Document;

impl store::ArtifactDsl for Ifc2x3Snapshot {
    const EXTENSION: &'static str = "ifc";
    fn envelope_id() -> &'static str { STDIO_IFC2X3_DOCUMENT_SCHEMA }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> { native::parse_text(text) }
    fn print_dsl(&self) -> String { native::print_text(self) }
}
}
pub use snapshot_codec::*;
