//! 💾️ Binary representation codec surface for `stdio.semio.animation.mutations`.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

const OP_BINARY_FORMAT: u8 = 1;

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::animation::io::text::mutations::TEXT_KEYWORDS;
use crate::standards::v1::subsets::animation::schema::mutations::*;
use crate::standards::v1::subsets::animation::schema::diff::{diff_set_snapshot, AnimChannelDiff, AnimKeyframeDiff, AnimTimelineDiff, SemioAnimationDiff};
use crate::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTimeline, AnimValue, SemioAnimationSnapshot};
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use protocol::Mutation;
/// 🔧️ `MutationDiff` added — the `#[cfg(test)] mod tests` block below calls `diff.apply(&base)`
/// via method syntax on `SemioAnimationDiff`, which needs `MutationDiff` in scope (W2b closer fix).
#[cfg(test)]
use protocol::MutationDiff;
use protocol::{OpBinary, OpText};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn wire_tag(m: &SemioAnimationMutation) -> u8 {
    use SemioAnimationMutation::*;
    match m {
        SetSnapshot(_) => TAG_SET_SNAPSHOT,
        PatchSnapshot(_) => TAG_PATCH_SNAPSHOT,
        InsertTimeline(_) => TAG_INSERT_TIMELINE,
        RemoveTimeline(_) => TAG_REMOVE_TIMELINE,
        SetTimelineName(_) => TAG_SET_TIMELINE_NAME,
        InsertChannel(_) => TAG_INSERT_CHANNEL,
        RemoveChannel(_) => TAG_REMOVE_CHANNEL,
        SetChannelTarget(_) => TAG_SET_CHANNEL_TARGET,
        SetChannelInterpolation(_) => TAG_SET_CHANNEL_INTERPOLATION,
        InsertKeyframe(_) => TAG_INSERT_KEYFRAME,
        RemoveKeyframe(_) => TAG_REMOVE_KEYFRAME,
        SetKeyframeTime(_) => TAG_SET_KEYFRAME_TIME,
        SetKeyframeValue(_) => TAG_SET_KEYFRAME_VALUE,
    }
}

/// 🔢️ Real binary op frame (animation wave — off the old whole-`OpText`-line `.into_bytes()` F6
/// text-as-binary shortcut). `format u8` + `tag u8` (its kind's record tag in `💾️binary/📡️.protocol.semio`) as two real fixed
/// fields, then the variant's own `key=value,...` argument text (i.e. `print_op`'s output with its
/// `TAG:` prefix stripped) as one opaque trailing `bytes` chain — reuses the real, tested
/// `print_op`/`parse_op` text codec (one source of truth), same treatment every prior semio wave's
/// `OpBinary` upgrade uses.
impl OpBinary for SemioAnimationMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        if let Self::PatchSnapshot(payload) = self {
            let mut out = vec![1, TAG_PATCH_SNAPSHOT];
            out.extend(protocol::OpBinary::encode_op(&payload.patch)?);
            return Ok(out);
        }
        let printed = <Self as OpText>::print_op(self);
        let args = match printed.split_once(':') {
            Some((_, rest)) => rest,
            None => "",
        };
        let mut out = vec![OP_BINARY_FORMAT, wire_tag(self)];
        out.extend_from_slice(args.as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let malformed = |what: &'static str, detail: String| protocol::ProtocolError::Malformed { what, offset: 0, detail };
        let [format, tag, rest @ ..] = bytes else { return Err(malformed("op header", format!("expected at least 2 bytes, got {}", bytes.len()))) };
        if *format != OP_BINARY_FORMAT {
            return Err(malformed("op format", format!("unsupported op format {format}")));
        }
        if *tag == TAG_PATCH_SNAPSHOT {
            return Ok(Self::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: protocol::OpBinary::decode_op(rest)? }));
        }
        let kind = dsl::protocol_record::kind(WIRE_PROTOCOL, u64::from(*tag)).ok_or_else(|| malformed("op tag", format!("tag {tag} names no record of 📡️.protocol.semio")))?;
        let keyword = TEXT_KEYWORDS.iter().find(|(record, _)| *record == kind).map(|(_, keyword)| *keyword).ok_or_else(|| malformed("op tag", format!("record {kind} has no text keyword")))?;
        let args = std::str::from_utf8(rest).map_err(|e| malformed("op args utf8", e.to_string()))?;
        let line = format!("{keyword}:{args}");
        <Self as OpText>::parse_op(&line).map_err(|e| malformed("op text", e.to_string()))
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `SemioAnimationMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-snapshot");
const TAG_PATCH_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "patch-snapshot");
const TAG_INSERT_TIMELINE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-timeline");
const TAG_REMOVE_TIMELINE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-timeline");
const TAG_SET_TIMELINE_NAME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-timeline-name");
const TAG_INSERT_CHANNEL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-channel");
const TAG_REMOVE_CHANNEL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-channel");
const TAG_SET_CHANNEL_TARGET: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-channel-target");
const TAG_SET_CHANNEL_INTERPOLATION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-channel-interpolation");
const TAG_INSERT_KEYFRAME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-keyframe");
const TAG_REMOVE_KEYFRAME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-keyframe");
const TAG_SET_KEYFRAME_TIME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-keyframe-time");
const TAG_SET_KEYFRAME_VALUE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-keyframe-value");
//#endregion 🏷️WireTags
