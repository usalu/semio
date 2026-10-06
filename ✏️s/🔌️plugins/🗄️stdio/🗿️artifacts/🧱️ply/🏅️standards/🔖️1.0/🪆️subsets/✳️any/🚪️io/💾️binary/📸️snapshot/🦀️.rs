//! binary rep for stdio.ply 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");



#[path = "📦️pack/🦀️.rs"]
pub(crate) mod native_pack;

#[allow(unused_imports)]
mod diff_codec {
use super::*;
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
pub(crate) fn read_bin_snapshot(r: &mut dsl::ByteReader<'_>) -> Result<PlySnapshot, dsl::PackRefusal> {
    let schema = read_bin_str(r)?;
    let format = read_bin_format(r)?;
    let comments = read_bin_vec(r, read_bin_str)?;
    let elements = read_bin_vec(r, read_bin_element)?;
    Ok(PlySnapshot { schema, format, comments, elements })
}
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
/// 🔣️ `PlySnapshot` real binary — needed by `PlyMutation::SetSnapshot`'s own real binary op frame
/// (`../🧬️mutations/🦀️.rs`, which imports this the same way it already imports the
/// text-codec `enc_snapshot`/`dec_snapshot` primitives from this file).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_snapshot(w: &mut dsl::ByteWriter, s: &PlySnapshot) {
    write_bin_str(w, &s.schema);
    write_bin_format(w, s.format);
    write_bin_vec(w, &s.comments, |w, c: &String| write_bin_str(w, c));
    write_bin_vec(w, &s.elements, write_bin_element);
}
}
pub(crate) use residual_diff_helper::*;
