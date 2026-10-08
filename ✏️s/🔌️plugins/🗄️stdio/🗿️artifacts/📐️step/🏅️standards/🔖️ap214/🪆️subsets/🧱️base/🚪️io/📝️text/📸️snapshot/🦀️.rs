//! 📝️ Text representation codec surface for `stdio.step` (snapshot).
use crate::standards::v_ap214::subsets::base::io::sqlite::snapshot::native;
use crate::standards::v_ap214::subsets::base::io::text::diff::{enc_str,dec_str,enc_file_description,dec_file_description,enc_file_name,dec_file_name,enc_file_schema,dec_file_schema,enc_entity,dec_entity,split_top_level,strip_brackets};

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

}
pub use diff_codec::*;
