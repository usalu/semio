//! 📝️ Text representation codec surface for `s.stdio.semio.object.diff` — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::object::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use crate::standards::v1::subsets::object::io::text::snapshot::{dec_child_opt};
use crate::standards::v1::subsets::object::io::text::snapshot::{enc_child_opt};
use crate::standards::v1::subsets::model::io::text::diff::{dec_transform};
use crate::standards::v1::subsets::model::io::text::diff::{enc_transform};

/// 🧾️ `<hex-flag><line>` per field, `\n`-joined, empty string = no-op diff — real, not decorative.
/// `t=`/`b=`/`m=`/`p=` prefixes; a field absent from the diff simply has no line.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_object_diff(d: &SemioObjectDiff) -> String {
    let mut lines = Vec::new();
    if let Some(t) = &d.transform {
        lines.push(format!("t={}", enc_transform(t)));
    }
    if let Some(b) = &d.brep {
        lines.push(format!("b={}", enc_child_opt(b)));
    }
    if let Some(m) = &d.mesh {
        lines.push(format!("m={}", enc_child_opt(m)));
    }
    if let Some(p) = &d.properties {
        lines.push(format!("p={}", enc_child_opt(p)));
    }
    lines.join(";")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_object_diff(line: &str) -> Result<SemioObjectDiff, String> {
    let mut d = SemioObjectDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for field in line.split(';') {
        let (tag, rest) = field.split_once('=').ok_or_else(|| format!("object diff: missing '=' in {field:?}"))?;
        match tag {
            "t" => d.transform = Some(dec_transform(rest)?),
            "b" => d.brep = Some(dec_child_opt(rest)?),
            "m" => d.mesh = Some(dec_child_opt(rest)?),
            "p" => d.properties = Some(dec_child_opt(rest)?),
            other => return Err(format!("object diff: unknown field tag {other:?}")),
        }
    }
    Ok(d)
}

impl protocol::DiffText for SemioObjectDiff {
fn print_diff(&self) -> String {
    print_object_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_object_diff(line).and_then(|diff| { diff.validate()?; Ok(diff) }).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
