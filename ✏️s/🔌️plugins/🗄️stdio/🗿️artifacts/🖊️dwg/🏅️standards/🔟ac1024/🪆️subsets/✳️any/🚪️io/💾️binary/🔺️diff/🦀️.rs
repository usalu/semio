//! dwg rep for stdio.dwg 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_ac1024::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::schema::snapshot::{DwgApplicationHistory, DwgApplicationInfo, DwgAuxiliaryHeader, DwgClass, DwgDependency, DwgHeaderVariables, DwgIndexedPreview, DwgLogicalDrawing, DwgRevisionHistory, DwgSummaryInfo, DwgTemplate};
use crate::DwgSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyResult, MutationDiff};

semio_framework_os_kernel::diff_binary!(crate::standards::v_ac1024::subsets::any::schema::diff::DwgDiff);
}
pub use diff_codec::*;
