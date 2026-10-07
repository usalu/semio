//! 💾️ Binary representation codec surface for `stdio.semio.animation.diff`.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::animation::schema::diff::*;
use crate::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTargetProperty, AnimTimeline, AnimValue, SemioAnimationSnapshot};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion};
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{dec_indexed_triple, enc_indexed_triple};
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::MutationDiff;











// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}













































/// 🔢️ Real binary diff frame (animation wave — off the old `print_diff().into_bytes()` F6
/// text-as-binary shortcut). `format u8` + `presence u8` (bit0=`timelines`) as two real fixed
/// header fields; when present, the SAME `enc_indexed_triple`-produced text this facet's own
/// `print_diff` already emits follows as one opaque trailing byte chain (last field in the frame,
/// so no length prefix is needed — matches the recipe's §2.5 "opaque payload LAST" rule). Only one
/// collection exists here (unlike brep's 6/flow's 2), so `presence` only ever uses bit0.
const DIFF_BINARY_FORMAT: u8 = 1;

impl protocol::DiffBinary for SemioAnimationDiff {
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut out = vec![DIFF_BINARY_FORMAT];
    match &self.timelines {
        Some(v) => {
            out.push(1u8);
            out.extend_from_slice(enc_indexed_triple(v, enc_timeline_diff, enc_timeline).as_bytes());
        }
        None => out.push(0u8),
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let malformed = |what: &'static str, detail: String| protocol::ProtocolError::Malformed { what, offset: 0, detail };
    let [format, presence, rest @ ..] = bytes else { return Err(malformed("diff header", format!("expected at least 2 bytes, got {}", bytes.len()))) };
    if *format != DIFF_BINARY_FORMAT {
        return Err(malformed("diff format", format!("unsupported diff format {format}")));
    }
    let timelines = match presence {
        0 => None,
        1 => {
            let text = std::str::from_utf8(rest).map_err(|e| malformed("diff payload utf8", e.to_string()))?;
            Some(dec_indexed_triple(text, dec_timeline_diff, dec_timeline).map_err(|e| malformed("diff payload", e))?)
        }
        other => return Err(malformed("diff presence", format!("unknown presence byte {other}"))),
    };
    Ok(SemioAnimationDiff { timelines })
}
}

use crate::standards::v1::subsets::animation::io::text::diff::{hex_encode, hex_decode, enc_str, dec_str, parse_f64, enc_list, dec_list, enc_property, dec_property, enc_target, dec_target, enc_interpolation, dec_interpolation, enc_value, dec_value, enc_keyframe, dec_keyframe, enc_channel, dec_channel, enc_timeline, dec_timeline, enc_keyframe_diff, dec_keyframe_diff, enc_channel_diff, dec_channel_diff, enc_timeline_diff, dec_timeline_diff};
}
pub use diff_codec::*;
