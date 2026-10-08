//! 💾️ Binary representation codec surface for `stdio.semio` (diff) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::base::schema::diff::*;

/// 🏷️ Binary tag ordinal for [`SemioDiff`] — `0` = `NoChange`, `1..=18` = the 18 wrapped subset
/// kinds (same enum declaration order as [`crate::standards::v1::subsets::base::schema::snapshot::subset_ordinal`],
/// offset by one to make room for `NoChange`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_tag(d: &SemioDiff) -> u8 {
    match d {
        SemioDiff::NoChange => 0,
        SemioDiff::Rejected(_) => 20,
        SemioDiff::Brep(_) => 1,
        SemioDiff::Mesh(_) => 2,
        SemioDiff::Model(_) => 3,
        SemioDiff::Value(_) => 4,
        SemioDiff::Document(_) => 5,
        SemioDiff::Cad(_) => 6,
        SemioDiff::Drawing(_) => 7,
        SemioDiff::Image(_) => 8,
        SemioDiff::Video(_) => 9,
        SemioDiff::Audio(_) => 10,
        SemioDiff::Animation(_) => 11,
        SemioDiff::Presentation(_) => 12,
        SemioDiff::Flow(_) => 13,
        SemioDiff::Text(_) => 14,
        SemioDiff::Table(_) => 15,
        SemioDiff::Graph(_) => 16,
        SemioDiff::Object(_) => 17,
        SemioDiff::Kit(_) => 18,
    }
}
use crate::standards::v1::subsets::animation::schema::{diff::SemioAnimationDiff, snapshot::SemioAnimationSnapshot};
use crate::standards::v1::subsets::audio::schema::{diff::SemioAudioDiff, snapshot::SemioAudioSnapshot};
use crate::standards::v1::subsets::base::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};
use crate::standards::v1::subsets::brep::schema::{diff::SemioBrepDiff, snapshot::SemioBrepSnapshot};
use crate::standards::v1::subsets::cad::schema::{diff::SemioCadDiff, snapshot::SemioCadSnapshot};
use crate::standards::v1::subsets::document::schema::{diff::SemioDocumentDiff, snapshot::SemioDocumentSnapshot};
use crate::standards::v1::subsets::drawing::schema::{diff::SemioDrawingDiff, snapshot::SemioDrawingSnapshot};
use crate::standards::v1::subsets::flow::schema::{diff::SemioFlowDiff, snapshot::SemioFlowSnapshot};
use crate::standards::v1::subsets::graph::schema::{diff::SemioGraphDiff, snapshot::SemioGraphSnapshot};
use crate::standards::v1::subsets::image::schema::{diff::SemioImageDiff, snapshot::SemioImageSnapshot};
use crate::standards::v1::subsets::kit::schema::{diff::SemioKitDiff, snapshot::SemioKitSnapshot};
use crate::standards::v1::subsets::mesh::schema::{diff::SemioMeshDiff, snapshot::SemioMeshSnapshot};
use crate::standards::v1::subsets::model::schema::{diff::SemioModelDiff, snapshot::SemioModelSnapshot};
use crate::standards::v1::subsets::object::schema::{diff::SemioObjectDiff, snapshot::SemioObjectSnapshot};
use crate::standards::v1::subsets::presentation::schema::{diff::SemioPresentationDiff, snapshot::SemioPresentationSnapshot};
use crate::standards::v1::subsets::table::schema::{diff::SemioTableDiff, snapshot::SemioTableSnapshot};
use crate::standards::v1::subsets::text::schema::{diff::SemioTextDiff, snapshot::SemioTextSnapshot};
use crate::standards::v1::subsets::value::schema::{diff::SemioValueTreeDiff, snapshot::SemioValueSnapshot};
use crate::standards::v1::subsets::video::schema::{diff::SemioVideoDiff, snapshot::SemioVideoSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::MutationApplyError;
use protocol::MutationDiff;





impl protocol::DiffBinary for SemioDiff {
/// ⚡️ Real delegating binary: `format u8` + `tag u8` ([`diff_tag`]) as two genuine,
/// individually protocol-walkable fixed header fields, then ONE opaque trailing payload —
/// for the 13 same-kind variants, that payload is exactly the wrapped subset's OWN real
/// `DiffBinary::encode_diff()` bytes (genuine reuse, never re-derived here); `NoChange` carries no payload at all.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    let mut out = vec![DIFF_BINARY_FORMAT, diff_tag(self)];
    let payload: Vec<u8> = match self {
        SemioDiff::NoChange => Vec::new(),
        SemioDiff::Rejected(error) => enc_rejection(error).into_bytes(),
        SemioDiff::Brep(d) => d.encode_diff()?,
        SemioDiff::Mesh(d) => d.encode_diff()?,
        SemioDiff::Model(d) => d.encode_diff()?,
        SemioDiff::Value(d) => d.encode_diff()?,
        SemioDiff::Document(d) => d.encode_diff()?,
        SemioDiff::Cad(d) => d.encode_diff()?,
        SemioDiff::Drawing(d) => d.encode_diff()?,
        SemioDiff::Image(d) => d.encode_diff()?,
        SemioDiff::Video(d) => d.encode_diff()?,
        SemioDiff::Audio(d) => d.encode_diff()?,
        SemioDiff::Animation(d) => d.encode_diff()?,
        SemioDiff::Presentation(d) => d.encode_diff()?,
        SemioDiff::Flow(d) => d.encode_diff()?,
        SemioDiff::Text(d) => d.encode_diff()?,
        SemioDiff::Table(d) => d.encode_diff()?,
        SemioDiff::Graph(d) => d.encode_diff()?,
        SemioDiff::Object(d) => d.encode_diff()?,
        SemioDiff::Kit(d) => d.encode_diff()?,
    };
    out.extend_from_slice(&payload);
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    if bytes.len() < 2 {
        return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated".to_string() });
    }
    let format = bytes[0];
    if format != DIFF_BINARY_FORMAT {
        return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported format {format}") });
    }
    let tag = bytes[1];
    let payload = &bytes[2..];
    Ok(match tag {
        0 => SemioDiff::NoChange,
        1 => SemioDiff::Brep(SemioBrepDiff::decode_diff(payload)?),
        2 => SemioDiff::Mesh(SemioMeshDiff::decode_diff(payload)?),
        3 => SemioDiff::Model(SemioModelDiff::decode_diff(payload)?),
        4 => SemioDiff::Value(SemioValueTreeDiff::decode_diff(payload)?),
        5 => SemioDiff::Document(SemioDocumentDiff::decode_diff(payload)?),
        6 => SemioDiff::Cad(SemioCadDiff::decode_diff(payload)?),
        7 => SemioDiff::Drawing(SemioDrawingDiff::decode_diff(payload)?),
        8 => SemioDiff::Image(SemioImageDiff::decode_diff(payload)?),
        9 => SemioDiff::Video(SemioVideoDiff::decode_diff(payload)?),
        10 => SemioDiff::Audio(SemioAudioDiff::decode_diff(payload)?),
        11 => SemioDiff::Animation(SemioAnimationDiff::decode_diff(payload)?),
        12 => SemioDiff::Presentation(SemioPresentationDiff::decode_diff(payload)?),
        13 => SemioDiff::Flow(SemioFlowDiff::decode_diff(payload)?),
        14 => SemioDiff::Text(SemioTextDiff::decode_diff(payload)?),
        15 => SemioDiff::Table(SemioTableDiff::decode_diff(payload)?),
        16 => SemioDiff::Graph(SemioGraphDiff::decode_diff(payload)?),
        17 => SemioDiff::Object(SemioObjectDiff::decode_diff(payload)?),
        18 => SemioDiff::Kit(SemioKitDiff::decode_diff(payload)?),
        20 => SemioDiff::Rejected(
            dec_rejection(std::str::from_utf8(payload).map_err(|error| protocol::ProtocolError::Malformed { what: "rejected diff", offset: 2, detail: error.to_string() })?).map_err(|error| protocol::ProtocolError::Malformed {
                what: "rejected diff",
                offset: 2,
                detail: error,
            })?,
        ),
        other => return Err(protocol::ProtocolError::Malformed { what: "diff tag", offset: 1, detail: format!("unknown tag {other}") }),
    })
}
}

use crate::standards::v1::subsets::base::io::text::diff::{enc_rejection, dec_rejection};
}
pub use diff_codec::*;
