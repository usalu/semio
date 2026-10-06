//! 📝️ Text representation codec surface for `s.stdio.semio.kit.diff` — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::kit::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitDesign, SemioKitSnapshot, SemioKitType};
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_design_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_design_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_type_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_type_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_link_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_link_list};
use crate::standards::v1::subsets::object::io::text::snapshot::{dec_child_opt};
use crate::standards::v1::subsets::object::io::text::snapshot::{enc_child_opt};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_child_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_child_list};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_kit_diff(d: &SemioKitDiff) -> String {
    let mut fields = Vec::new();
    if let Some(t) = &d.types {
        fields.push(format!("t={}", enc_type_list(&t.values)));
    }
    if let Some(dd) = &d.designs {
        fields.push(format!("d={}", enc_design_list(&dd.values)));
    }
    if let Some(o) = &d.objects {
        fields.push(format!("o={}", enc_child_list(&o.values)));
    }
    if let Some(m) = &d.models {
        fields.push(format!("m={}", enc_child_list(&m.values)));
    }
    if let Some(p) = &d.properties {
        fields.push(format!("p={}", enc_child_opt(p)));
    }
    if let Some(r) = &d.representations {
        fields.push(format!("r={}", enc_link_list(&r.values)));
    }
    fields.join(";")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_kit_diff(line: &str) -> Result<SemioKitDiff, String> {
    let mut d = SemioKitDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for field in line.split(';') {
        let (tag, rest) = field.split_once('=').ok_or_else(|| format!("kit diff: missing '=' in {field:?}"))?;
        match tag {
            "t" => d.types = Some(SemioKitTypeList { values: dec_type_list(rest)? }),
            "d" => d.designs = Some(SemioKitDesignList { values: dec_design_list(rest)? }),
            "o" => d.objects = Some(SemioKitObjectChildList { values: dec_child_list(rest)? }),
            "m" => d.models = Some(SemioKitModelChildList { values: dec_child_list(rest)? }),
            "p" => d.properties = Some(dec_child_opt(rest)?),
            "r" => d.representations = Some(SemioKitLinkList { values: dec_link_list(rest)? }),
            other => return Err(format!("kit diff: unknown field tag {other:?}")),
        }
    }
    Ok(d)
}

impl protocol::DiffText for SemioKitDiff {
fn print_diff(&self) -> String {
    print_kit_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_kit_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
