//! 📝️ Text representation codec surface for `stdio.dwg` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type DwgDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_ac1024::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::schema::snapshot::{DwgApplicationHistory, DwgApplicationInfo, DwgAuxiliaryHeader, DwgClass, DwgDependency, DwgHeaderVariables, DwgIndexedPreview, DwgLogicalDrawing, DwgRevisionHistory, DwgSummaryInfo, DwgTemplate};
use crate::DwgSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyResult, MutationDiff};

semio_framework_os_kernel::diff_text!(crate::standards::v_ac1024::subsets::any::schema::diff::DwgDiff);
}
pub use diff_codec::*;
