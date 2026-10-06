//! 💾️ Binary representation codec surface for `stdio.mp4` (diff).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::isobmff::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Ftyp, Mp4Movie, Mp4Sample, Mp4Snapshot, Mp4Track, Mp4TrackMetadata};
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

semio_framework_os_kernel::diff_binary!(crate::standards::isobmff::subsets::any::schema::diff::Mp4Diff);
}
pub use diff_codec::*;
