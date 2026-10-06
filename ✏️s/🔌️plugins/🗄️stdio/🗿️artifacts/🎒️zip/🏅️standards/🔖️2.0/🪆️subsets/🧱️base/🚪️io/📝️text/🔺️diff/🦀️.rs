//! 📝️ Text representation codec surface for `stdio.zip` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type ZipDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v2_0::subsets::base::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::ZipSnapshot;
use crate::schema::snapshot::ZipEntryMetadata;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{HashMap, HashSet};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::ZipEntry;

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct ZipDiffRecord {
    value: semio_framework_value::DslValue,
}

impl protocol::DiffText for ZipDiff {
fn print_diff(&self) -> String {
    let model = ZipDiffRecord { value: semio_framework_value::ToValue::to_value(self) };
    semio_framework_dsl_record::print(&model.__dsl_to_record(), &ZipDiffRecord::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document)
}
fn parse_diff(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    let record = semio_framework_dsl_record::parse(text, &ZipDiffRecord::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits { max_bytes: 64 * 1024 * 1024, ..semio_framework_diagnostic::Limits::default() }, mode: semio_framework_dsl_record::SourceMode::Document })?;
    let model = ZipDiffRecord::__dsl_from_record(&record)?;
    <Self as semio_framework_value::FromValue>::from_value(model.value).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
