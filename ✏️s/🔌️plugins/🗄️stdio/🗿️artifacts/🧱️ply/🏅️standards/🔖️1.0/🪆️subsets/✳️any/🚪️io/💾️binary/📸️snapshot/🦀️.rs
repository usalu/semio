//! binary rep for stdio.ply 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");



#[path = "📦️pack/🦀️.rs"]
pub(crate) mod native_pack;

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1_0::subsets::any::io::binary::diff::{read_bin_str,read_bin_format,read_bin_vec,read_bin_element};
use crate::standards::v1_0::subsets::any::schema::diff::*;
use crate::schema::snapshot::{PlyElement, PlyFormat, PlyProperty, PlyRow, PlyScalarType};
use crate::PlySnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, BTreeSet, HashSet};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::PlyValue;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
}
pub use diff_codec::*;

#[allow(unused_imports)]
mod residual_diff_helper {
use crate::standards::v1_0::subsets::any::schema::diff::*;
use crate::standards::v1_0::subsets::any::io::binary::diff::write_bin_element;
use crate::standards::v1_0::subsets::any::io::binary::diff::write_bin_format;
use crate::standards::v1_0::subsets::any::io::binary::diff::write_bin_vec;
use crate::standards::v1_0::subsets::any::io::binary::diff::write_bin_str;
use crate::schema::snapshot::{PlyElement, PlyFormat, PlyProperty, PlyRow, PlyScalarType};
use crate::PlySnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, BTreeSet, HashSet};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::PlyValue;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
}
pub(crate) use residual_diff_helper::*;
