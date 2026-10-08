//! 📝️ Text representation codec surface for `stdio.ifc.2x3` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v2x3::subsets::base::schema::mutations::*;
use crate::standards::v2x3::subsets::base::schema::diff::Ifc2x3Diff;
use crate::standards::v2x3::subsets::base::io::text::diff::enc_part21_instance;
use crate::standards::v2x3::subsets::base::io::text::diff::{dec_part21_instance};
use crate::standards::v2x3::subsets::base::io::binary::diff::{dec_part21_instance_bin};
use crate::standards::v2x3::subsets::base::io::binary::diff::{enc_part21_instance_bin};
use crate::standards::v2x3::subsets::base::io::text::diff::{dec_instance_list};
use crate::standards::v2x3::subsets::base::io::text::diff::{enc_instance_list_into};
use crate::standards::v2x3::subsets::base::io::text::diff::{dec_part21_header};
use crate::standards::v2x3::subsets::base::io::text::diff::{enc_part21_header};
use crate::standards::v2x3::subsets::base::io::text::diff::{strip_brackets};
use crate::standards::v2x3::subsets::base::io::text::diff::{split_top_level};
use crate::standards::v2x3::subsets::base::io::text::diff::{dec_optional_edm_preamble};
use crate::standards::v2x3::subsets::base::io::text::diff::{enc_optional_edm_preamble};
use crate::standards::v2x3::subsets::base::io::text::diff::{dec_str};
use crate::standards::v2x3::subsets::base::io::text::diff::{enc_str};
use crate::standards::v2x3::subsets::base::io::binary::diff::{dec_part21_header_bin};
use crate::standards::v2x3::subsets::base::io::binary::diff::{enc_part21_header_bin};
use crate::standards::v2x3::subsets::base::io::binary::diff::{dec_edm_preamble_bin};
use crate::standards::v2x3::subsets::base::io::binary::diff::{enc_edm_preamble_bin};
use crate::standards::v2x3::subsets::base::io::binary::diff::{read_str_bin};
use crate::standards::v2x3::subsets::base::io::binary::diff::{write_str_bin};
use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use protocol::os_spr::command::DiffAlgebra;
use protocol::Mutation;
#[cfg(test)]
use semio_s_artifact_stdio_contract::part21::Part21Value;
use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Header, Part21Instance};

pub(crate) fn print_ifc2x3_mutation(m: &Ifc2x3Mutation) -> String {
    match m {
        Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance, index }) => format!("upsert-instance instance={}{}", enc_part21_instance(instance), index.map_or_else(String::new, |index| format!(" index={index}"))),
        Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id }) => format!("remove-instance id={id}"),
        Ifc2x3Mutation::SetHeader(set_header::SetHeader { header }) => format!("set-header header={}", enc_part21_header(header)),
    }
}

pub(crate) fn parse_ifc2x3_mutation(line: &str) -> Result<Ifc2x3Mutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let (arg_key, arg_val) = rest.split_once('=').ok_or_else(|| format!("ifc2x3 mutation: missing arg for {keyword:?}"))?;
    match (keyword, arg_key) {
        ("upsert-instance", "instance") => {
            let (instance, index) = match arg_val.split_once(" index=") {
                Some((instance, index)) => (instance, Some(index.parse().map_err(|e: std::num::ParseIntError| e.to_string())?)),
                None => (arg_val, None),
            };
            Ok(Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance: dec_part21_instance(instance)?, index }))
        }
        ("remove-instance", "id") => Ok(Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id: arg_val.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        ("set-header", "header") => Ok(Ifc2x3Mutation::SetHeader(set_header::SetHeader { header: dec_part21_header(arg_val)? })),
        (other, _) => Err(format!("ifc2x3 mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for Ifc2x3Mutation {
    fn print_op(&self) -> String {
        print_ifc2x3_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_ifc2x3_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
