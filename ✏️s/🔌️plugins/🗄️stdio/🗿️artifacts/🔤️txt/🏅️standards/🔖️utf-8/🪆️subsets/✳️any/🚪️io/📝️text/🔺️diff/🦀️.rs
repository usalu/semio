//! 📝️ Text representation codec surface for `stdio.txt` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type TxtDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_utf_8::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::schema::snapshot::LineEnding;
use crate::TxtSnapshot;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use std::collections::HashSet;


semio_framework_os_kernel::diff_text!(crate::standards::v_utf_8::subsets::any::schema::diff::TxtDiff);
}
pub use diff_codec::*;
