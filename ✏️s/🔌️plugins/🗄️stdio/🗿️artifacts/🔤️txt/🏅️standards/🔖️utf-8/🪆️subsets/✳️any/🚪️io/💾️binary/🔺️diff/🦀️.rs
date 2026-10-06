//! binary rep for stdio.txt 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_utf_8::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::schema::snapshot::LineEnding;
use crate::TxtSnapshot;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use std::collections::HashSet;


semio_framework_os_kernel::diff_binary!(crate::standards::v_utf_8::subsets::any::schema::diff::TxtDiff);
}
pub use diff_codec::*;
