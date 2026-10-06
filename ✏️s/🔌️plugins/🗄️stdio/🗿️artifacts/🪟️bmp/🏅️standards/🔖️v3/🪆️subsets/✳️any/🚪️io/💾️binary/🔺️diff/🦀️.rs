//! bmp rep for stdio.bmp 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_v3::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::BmpSnapshot;
use framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationApplyResult, MutationDiff};

semio_framework_os_kernel::diff_binary!(crate::standards::v_v3::subsets::any::schema::diff::BmpDiff);
}
pub use diff_codec::*;
