//! 📝️ Text representation codec surface for `stdio.step` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_ap214::subsets::base::schema::mutations::*;
use crate::schema::diff::{diff_set_snapshot, StepArgAdded, StepArgModified, StepArgsDiff, StepDiff, StepEntitiesDiff, StepEntityAdded, StepEntityDiff, StepEntityModified};
use crate::standards::v_ap214::subsets::base::io::binary::snapshot::{dec_step_snapshot_bin};
use crate::standards::v_ap214::subsets::base::io::binary::snapshot::{enc_step_snapshot_bin};
use crate::standards::v_ap214::subsets::base::io::text::snapshot::{dec_step_snapshot};
use crate::standards::v_ap214::subsets::base::io::text::snapshot::{enc_step_snapshot};
use crate::standards::v_ap214::subsets::base::io::text::diff::{dec_value};
use crate::standards::v_ap214::subsets::base::io::text::diff::{enc_value};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{dec_value_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{enc_value_bin};
use crate::standards::v_ap214::subsets::base::io::text::diff::{dec_entity};
use crate::standards::v_ap214::subsets::base::io::text::diff::{enc_entity};
use crate::standards::v_ap214::subsets::base::io::text::diff::{parse_u64};
use crate::standards::v_ap214::subsets::base::io::text::diff::{parse_usize};
use crate::standards::v_ap214::subsets::base::io::text::diff::{dec_str};
use crate::standards::v_ap214::subsets::base::io::text::diff::{enc_str};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{dec_entity_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{enc_entity_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{read_str_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{write_str_bin};
use crate::standards::v_ap214::subsets::base::io::text::diff::{dec_file_schema};
use crate::standards::v_ap214::subsets::base::io::text::diff::{enc_file_schema};
use crate::standards::v_ap214::subsets::base::io::text::diff::{dec_file_name};
use crate::standards::v_ap214::subsets::base::io::text::diff::{enc_file_name};
use crate::standards::v_ap214::subsets::base::io::text::diff::{dec_file_description};
use crate::standards::v_ap214::subsets::base::io::text::diff::{enc_file_description};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{dec_file_schema_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{enc_file_schema_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{dec_file_name_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{enc_file_name_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{dec_file_description_bin};
use crate::standards::v_ap214::subsets::base::io::binary::diff::{enc_file_description_bin};
use crate::schema::snapshot::{StepEntity, StepFileDescription, StepFileName, StepFileSchema, StepValue};
use crate::StepSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};

/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `StepMutation` — real `cargo check` confirms 3a
/// on the mutation side too, independently of the diff side: `InsertEntity.entity: StepEntity`
/// fails (`StepEntity: DslField` unsatisfied) and `SetEntityArg`/`InsertEntityArg`'s
/// `value: StepValue` fail directly (`StepValue: DslField` unsatisfied) — `#[derive(dsl::DslOps)]`
/// requires `DslField` on every variant field, transitively; `StepValue`/`StepEntity` are real
/// data-carrying types with no `DslField` impl, same root cause as `SvgMutation`'s `InsertElement`/
/// `SetSnapshot` blockers. Reuses `StepDiff`'s `pub(crate)` grammar primitives (`enc_value`/
/// `enc_entity`/`enc_step_snapshot`/...) rather than duplicating them — same pattern `SvgMutation`
/// uses against `SvgDiff`. Grammar: `keyword arg=value ...` (space-separated), one match arm per
/// variant (no `DslVariants` scaffolding available since nothing here derives it).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_step_mutation(m: &StepMutation) -> String {
    match m {
        StepMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_step_snapshot(snapshot)),
        StepMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(patch),
        StepMutation::SetFileDescription(set_file_description::SetFileDescription { file_description }) => format!("set-file-description file-description={}", enc_file_description(file_description)),
        StepMutation::SetFileName(set_file_name::SetFileName { file_name }) => format!("set-file-name file-name={}", enc_file_name(file_name)),
        StepMutation::SetFileSchema(set_file_schema::SetFileSchema { file_schema }) => format!("set-file-schema file-schema={}", enc_file_schema(file_schema)),
        StepMutation::InsertEntity(insert_entity::InsertEntity { index, entity }) => format!("insert-entity index={index} entity={}", enc_entity(entity)),
        StepMutation::RemoveEntity(remove_entity::RemoveEntity { id }) => format!("remove-entity id={id}"),
        StepMutation::SetEntityName(set_entity_name::SetEntityName { id, name }) => format!("set-entity-name id={id} name={}", enc_str(name)),
        StepMutation::SetEntityArg(set_entity_arg::SetEntityArg { id, arg_index, value }) => format!("set-entity-arg id={id} arg-index={arg_index} value={}", enc_value(value)),
        StepMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id, arg_index, value }) => format!("insert-entity-arg id={id} arg-index={arg_index} value={}", enc_value(value)),
        StepMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id, arg_index }) => format!("remove-entity-arg id={id} arg-index={arg_index}"),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_step_mutation(line: &str) -> Result<StepMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("step mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("step mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { parse_usize(arg(k)?) };
    let u64_arg = |k: &str| -> Result<u64, String> { parse_u64(arg(k)?) };
    match keyword {
        "patch-snapshot" => semio_s_artifact_stdio_contract::editing::snapshot_patch_from_text(line).map(|patch| StepMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch })),
        "set-snapshot" => Ok(StepMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_step_snapshot(arg("snapshot")?)? })),
        "set-file-description" => Ok(StepMutation::SetFileDescription(set_file_description::SetFileDescription { file_description: dec_file_description(arg("file-description")?)? })),
        "set-file-name" => Ok(StepMutation::SetFileName(set_file_name::SetFileName { file_name: dec_file_name(arg("file-name")?)? })),
        "set-file-schema" => Ok(StepMutation::SetFileSchema(set_file_schema::SetFileSchema { file_schema: dec_file_schema(arg("file-schema")?)? })),
        "insert-entity" => Ok(StepMutation::InsertEntity(insert_entity::InsertEntity { index: usize_arg("index")?, entity: dec_entity(arg("entity")?)? })),
        "remove-entity" => Ok(StepMutation::RemoveEntity(remove_entity::RemoveEntity { id: u64_arg("id")? })),
        "set-entity-name" => Ok(StepMutation::SetEntityName(set_entity_name::SetEntityName { id: u64_arg("id")?, name: dec_str(arg("name")?)? })),
        "set-entity-arg" => Ok(StepMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: u64_arg("id")?, arg_index: usize_arg("arg-index")?, value: dec_value(arg("value")?)? })),
        "insert-entity-arg" => Ok(StepMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: u64_arg("id")?, arg_index: usize_arg("arg-index")?, value: dec_value(arg("value")?)? })),
        "remove-entity-arg" => Ok(StepMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: u64_arg("id")?, arg_index: usize_arg("arg-index")? })),
        other => Err(format!("step mutation: unknown keyword {other:?}")),
    }
}

impl OpText for StepMutation {
    fn print_op(&self) -> String {
        print_step_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_step_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
