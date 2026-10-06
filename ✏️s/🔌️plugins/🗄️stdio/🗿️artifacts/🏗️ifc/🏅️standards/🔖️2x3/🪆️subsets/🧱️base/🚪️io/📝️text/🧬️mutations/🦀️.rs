//! 📝️ Text representation codec surface for `stdio.ifc.2x3` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v2x3::subsets::base::schema::mutations::*;
use crate::standards::v2x3::subsets::base::schema::diff::{enc_part21_instance, Ifc2x3Diff};
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

/// 🧪️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: **hand-rolled**
/// `OpText`/`OpBinary` for `Ifc2x3Mutation`, replacing the prior `serde_json::to_string`/`from_str`/
/// `to_vec`/`from_slice` literal-JSON-transfer shortcut — the LAST standard-specific
/// `POLICY_STDIO_JSON_TRANSFER_BAN` violation named anywhere in this program's own census (see
/// `📖️grammar-recipe.md`'s own citation of this exact file/line). `#[derive(dsl::DslOps)]` cannot
/// be used here either: `Part21Value` (reachable via `Part21Instance`/`Part21Header`/
/// `Ifc2x3Snapshot`) is a genuine data-carrying enum with no `DslField` impl, the identical root
/// cause `4`'s own `IfcMutation` doc comment documents for the isomorphic shape. Reuses the diff
/// sibling's `pub(crate)` grammar primitives (`enc_str`/`enc_part21_header`/`enc_part21_instance`/
/// `split_top_level`/...) rather than duplicating them a second time in this file — same
/// intra-artifact-reuse split `4`'s own `🧬️mutations/🦀️.rs` uses. Grammar: `keyword
/// arg=value ...` (space-separated), one match arm per variant.
pub(crate) fn enc_ifc2x3_snapshot_into(s: &Ifc2x3Snapshot, out: &mut String) {
    out.push('[');
    out.push_str(&enc_str(&s.schema));
    out.push(',');
    out.push_str(&enc_part21_header(&s.document.header));
    out.push(',');
    enc_instance_list_into(&s.document.instances, out);
    out.push(',');
    out.push_str(&enc_optional_edm_preamble(&s.edm_preamble));
    out.push(']');
}

pub(crate) fn dec_ifc2x3_snapshot(s: &str) -> Result<Ifc2x3Snapshot, String> {
    let fields = split_top_level(strip_brackets(s)?, ',');
    let [schema, header, instances, edm_preamble] = fields.as_slice() else {
        return Err(format!("ifc2x3 snapshot: expected 4 fields, got {}", fields.len()));
    };
    Ok(Ifc2x3Snapshot { schema: dec_str(schema)?, document: Part21Document { header: dec_part21_header(header)?, instances: dec_instance_list(instances)? }, edm_preamble: dec_optional_edm_preamble(edm_preamble)? })
}

pub(crate) fn print_ifc2x3_mutation(m: &Ifc2x3Mutation) -> String {
    match m {
        Ifc2x3Mutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(patch),
        Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => {
            let mut out = String::with_capacity(snapshot.document.instances.len().saturating_mul(64).saturating_add(22));
            out.push_str("set-snapshot snapshot=");
            enc_ifc2x3_snapshot_into(snapshot, &mut out);
            out
        }
        Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance }) => format!("upsert-instance instance={}", enc_part21_instance(instance)),
        Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id }) => format!("remove-instance id={id}"),
        Ifc2x3Mutation::SetHeader(set_header::SetHeader { header }) => format!("set-header header={}", enc_part21_header(header)),
    }
}

pub(crate) fn parse_ifc2x3_mutation(line: &str) -> Result<Ifc2x3Mutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let (arg_key, arg_val) = rest.split_once('=').ok_or_else(|| format!("ifc2x3 mutation: missing arg for {keyword:?}"))?;
    match (keyword, arg_key) {
        ("patch-snapshot", "patch") => semio_s_artifact_stdio_contract::editing::snapshot_patch_from_text(line).map(|patch| Ifc2x3Mutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch })),
        ("set-snapshot", "snapshot") => Ok(Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Box::new(dec_ifc2x3_snapshot(arg_val)?) })),
        ("upsert-instance", "instance") => Ok(Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance: dec_part21_instance(arg_val)? })),
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
