//! 📝️ Text representation codec surface for `stdio.pdf` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type PdfDiffText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1_7::subsets::base::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{HashMap, HashSet};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfIndirectObject;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::standards::v1_7::subsets::base::schema::snapshot::ObjRef;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfPage;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfStreamFilter;

/// 🧾 One codec for every lane: the derive-owned value encoding. Text is the one-line
/// `value=<dsl value>` record (`📝️text/📖️.grammar.semio`), binary is the `OP_BINARY_FORMAT`
/// byte followed by the container-less pack record body of the same value
/// (`💾️binary/📡️.protocol.semio`). Both are deterministic and decode back to the identical
/// `PdfDiff`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct PdfDiffRecord {
    value: semio_framework_value::DslValue,
}

impl protocol::DiffText for PdfDiff {
fn print_diff(&self) -> String {
    let model = PdfDiffRecord { value: semio_framework_value::ToValue::to_value(self) };
    semio_framework_dsl_record::print(&model.__dsl_to_record(), &PdfDiffRecord::__dsl_spec(), semio_framework_dsl_record::JoinMode::Inline)
}
fn parse_diff(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    let record = semio_framework_dsl_record::parse(text, &PdfDiffRecord::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits { max_bytes: 64 * 1024 * 1024, ..semio_framework_diagnostic::Limits::default() }, mode: semio_framework_dsl_record::SourceMode::Inline })?;
    let model = PdfDiffRecord::__dsl_from_record(&record)?;
    <Self as semio_framework_value::FromValue>::from_value(model.value).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
