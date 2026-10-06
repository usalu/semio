//! 📝️ Text representation codec surface for `stdio.png` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type PngDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1_2::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::PngSnapshot;
use framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationApplyResult, MutationDiff};

semio_framework_os_kernel::diff_text!(crate::standards::v1_2::subsets::any::schema::diff::PngDiff);
}
pub use diff_codec::*;
