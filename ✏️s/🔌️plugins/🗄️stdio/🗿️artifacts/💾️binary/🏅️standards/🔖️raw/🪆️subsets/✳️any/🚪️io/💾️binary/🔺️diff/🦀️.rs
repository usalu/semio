//! binary rep for stdio.binary 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_raw::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::BinarySnapshot;
use protocol::MutationDiff;
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;


semio_framework_os_kernel::diff_binary!(crate::standards::v_raw::subsets::any::schema::diff::BinaryDiff);
}
pub use diff_codec::*;
