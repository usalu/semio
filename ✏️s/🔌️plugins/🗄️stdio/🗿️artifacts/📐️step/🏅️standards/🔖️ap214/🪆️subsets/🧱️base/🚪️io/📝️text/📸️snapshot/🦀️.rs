//! 📝️ Text representation codec surface for `stdio.step` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type StepSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_ap214::subsets::base::schema::snapshot::*;
use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
use crate::STDIO_STEP_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
/// 🧱 The BrepMesh analyzer types live with the derived view in `engine::brep`, not here — the
/// snapshot only stores the generic graph. Re-exported for pre-existing call sites' convenience.
use crate::engine::brep::{BrepFace, BrepMesh, BrepVertex};

impl store::ArtifactDsl for StepSnapshot {
    const EXTENSION: &'static str = "step";
    fn envelope_id() -> &'static str { STDIO_STEP_DOCUMENT_SCHEMA }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> { native::parse_text(text) }
    fn print_dsl(&self) -> String { native::print_text(self) }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_ap214::subsets::base::schema::diff::*;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::StepSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use crate::schema::snapshot::StepComplexType;
use crate::schema::snapshot::StepEntity;
use crate::schema::snapshot::StepFileDescription;
use crate::schema::snapshot::StepFileName;
use crate::schema::snapshot::StepFileSchema;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::StepValue;

/// 📸️ Full `StepSnapshot` codec — needed by `SetSnapshot`'s `OpText`/`OpBinary` (mutations file
/// imports this `pub(crate)`), never by `StepDiff` itself (no `snapshot: Option<StepSnapshot>`
/// full-replace slot exists on the diff).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_step_snapshot(s: &StepSnapshot) -> String {
    format!("[{},{},{},{},[{}]]", enc_str(&s.schema), enc_file_description(&s.header.file_description), enc_file_name(&s.header.file_name), enc_file_schema(&s.header.file_schema), s.entities.iter().map(enc_entity).collect::<Vec<_>>().join(","),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_step_snapshot(s: &str) -> Result<StepSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, file_description, file_name, file_schema, entities] = parts.as_slice() else {
        return Err(format!("step snapshot: expected 5 fields, got {}", parts.len()));
    };
    Ok(StepSnapshot {
        schema: dec_str(schema)?,
        header: crate::schema::snapshot::StepHeader { file_description: dec_file_description(file_description)?, file_name: dec_file_name(file_name)?, file_schema: dec_file_schema(file_schema)? },
        entities: split_top_level(strip_brackets(entities)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_entity).collect::<Result<Vec<_>, String>>()?,
    })
}
}
pub use diff_codec::*;
