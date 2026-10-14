//! 📝️ Text representation codec surface for `stdio.step` (snapshot).
use crate::standards::v_ap214::subsets::base::io::sqlite::snapshot::native;

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
use crate::standards::v_ap214::subsets::base::schema::snapshot::STDIO_STEP_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
/// 🧱 The BrepMesh analyzer types live with the derived view in `engine::brep`, not here — the
/// snapshot only stores the generic graph. Re-exported for pre-existing call sites' convenience.
use crate::standards::v_ap214::subsets::base::io::brep::{BrepFace, BrepMesh, BrepVertex};

impl pack::snapshot::ArtifactDsl for StepSnapshot {
    const EXTENSION: &'static str = "step";
    fn envelope_id() -> &'static str { STDIO_STEP_DOCUMENT_SCHEMA }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> { native::parse_text(text) }
    fn print_dsl(&self) -> String { native::print_text(self) }
}
}

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_ap214::subsets::base::schema::diff::*;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use crate::standards::v_ap214::subsets::base::schema::snapshot::StepComplexType;
use crate::standards::v_ap214::subsets::base::schema::snapshot::StepEntity;
use crate::standards::v_ap214::subsets::base::schema::snapshot::StepFileDescription;
use crate::standards::v_ap214::subsets::base::schema::snapshot::StepFileName;
use crate::standards::v_ap214::subsets::base::schema::snapshot::StepFileSchema;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::standards::v_ap214::subsets::base::schema::snapshot::StepValue;

}
