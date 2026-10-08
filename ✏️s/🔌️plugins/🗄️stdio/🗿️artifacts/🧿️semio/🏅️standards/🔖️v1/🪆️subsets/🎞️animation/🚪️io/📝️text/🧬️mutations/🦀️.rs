//! 📝️ Text representation codec surface for `stdio.semio.animation.mutations`.

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

/// 🧾️ Each record kind's text-grammar tag, the head `decode_op` re-prefixes onto the argument tail before `parse_op`.
pub(crate) const TEXT_KEYWORDS: [(&str, &str); 11] = [
    ("insert-timeline", "IT"),
    ("remove-timeline", "RT"),
    ("set-timeline-name", "TN"),
    ("insert-channel", "IC"),
    ("remove-channel", "RC"),
    ("set-channel-target", "CT"),
    ("set-channel-interpolation", "CI"),
    ("insert-keyframe", "IK"),
    ("remove-keyframe", "RK"),
    ("set-keyframe-time", "KT"),
    ("set-keyframe-value", "KV"),
];

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::animation::schema::mutations::*;
use crate::standards::v1::subsets::animation::schema::diff::{AnimChannelDiff, AnimKeyframeDiff, AnimTimelineDiff, SemioAnimationDiff};
use crate::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTimeline, AnimValue, SemioAnimationSnapshot};
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use protocol::Mutation;
/// 🔧️ `MutationDiff` in scope for the `#[cfg(test)] mod tests` block below.
#[cfg(test)]
use protocol::MutationDiff;
use protocol::{OpBinary, OpText};

/// 📥️ Decodes this subset's internally tagged (`{"mutation": "<camelCaseVariant>", ...}`) wire value — the shape
/// `🎞️mutate-semio-animation`'s committed specification vectors and doc strings carry — into a real [`SemioAnimationMutation`]. A thin
/// `pack::from_json_str` wrapper over `ToValue`/`FromValue`, so the test adapter reads the committed wire value instead of
/// re-declaring it field by field beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_animation_mutation_json(text: &str) -> Result<SemioAnimationMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(semio_framework_value::ValueError::into_message)
}

/// 🎙️ Handcrafted `OpText`/`OpBinary` — one `TAG:payload` line per variant, reusing the diff
/// module's `pub(crate)` value codecs (`enc_timeline`/`enc_channel`/`enc_keyframe`/`enc_target`/
/// `enc_value`/`enc_interpolation`/hex-string helpers) instead of re-deriving a second parallel
/// grammar.
impl OpText for SemioAnimationMutation {
    fn print_op(&self) -> String {
        use crate::standards::v1::subsets::animation::io::text::snapshot::{enc_timeline};
        use crate::standards::v1::subsets::animation::io::text::snapshot::{enc_keyframe};
        use crate::standards::v1::subsets::animation::io::text::snapshot::{enc_value};
        use crate::standards::v1::subsets::animation::io::text::snapshot::{enc_interpolation};
        use crate::standards::v1::subsets::animation::io::text::snapshot::{enc_target};
        use crate::standards::v1::subsets::animation::io::text::snapshot::{enc_channel};
        use crate::standards::v1::subsets::animation::io::text::snapshot::{enc_str};
        use SemioAnimationMutation::*;
        match self {
            InsertTimeline(insert_timeline::InsertTimeline { index, timeline }) => format!("IT:{index},{}", enc_timeline(timeline)),
            RemoveTimeline(remove_timeline::RemoveTimeline { index }) => format!("RT:{index}"),
            SetTimelineName(set_timeline_name::SetTimelineName { index, name }) => format!(
                "TN:{index},{}",
                match name {
                    None => "[0]".to_string(),
                    Some(n) => format!("[1,{}]", enc_str(n)),
                }
            ),
            InsertChannel(insert_channel::InsertChannel { timeline_index, index, channel }) => format!("IC:{timeline_index},{index},{}", enc_channel(channel)),
            RemoveChannel(remove_channel::RemoveChannel { timeline_index, index }) => format!("RC:{timeline_index},{index}"),
            SetChannelTarget(set_channel_target::SetChannelTarget { timeline_index, index, target }) => format!("CT:{timeline_index},{index},{}", enc_target(target)),
            SetChannelInterpolation(set_channel_interpolation::SetChannelInterpolation { timeline_index, index, interpolation }) => format!("CI:{timeline_index},{index},{}", enc_interpolation(*interpolation)),
            InsertKeyframe(insert_keyframe::InsertKeyframe { timeline_index, channel_index, index, keyframe }) => format!("IK:{timeline_index},{channel_index},{index},{}", enc_keyframe(keyframe)),
            RemoveKeyframe(remove_keyframe::RemoveKeyframe { timeline_index, channel_index, index }) => format!("RK:{timeline_index},{channel_index},{index}"),
            SetKeyframeTime(set_keyframe_time::SetKeyframeTime { timeline_index, channel_index, index, t }) => format!("KT:{timeline_index},{channel_index},{index},{t}"),
            SetKeyframeValue(set_keyframe_value::SetKeyframeValue { timeline_index, channel_index, index, value }) => format!("KV:{timeline_index},{channel_index},{index},{}", enc_value(value)),
        }
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        use crate::standards::v1::subsets::animation::io::text::snapshot::{dec_timeline};
        use crate::standards::v1::subsets::animation::io::text::snapshot::{dec_keyframe};
        use crate::standards::v1::subsets::animation::io::text::snapshot::{dec_value};
        use crate::standards::v1::subsets::animation::io::text::snapshot::{dec_interpolation};
        use crate::standards::v1::subsets::animation::io::text::snapshot::{dec_target};
        use crate::standards::v1::subsets::animation::io::text::snapshot::{dec_channel};
        use crate::standards::v1::subsets::animation::io::text::snapshot::{dec_str};
        use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
        use SemioAnimationMutation::*;
        let fail = |e: String| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1));
        let parse_usize = |s: &str| s.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string());
        let parse_f64 = |s: &str| s.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string());

        let (tag, rest) = line.split_once(':').ok_or_else(|| fail(format!("op: bad shape {line:?}")))?;
        (|| -> Result<Self, String> {
            match tag {
                "IT" => {
                    let (index, timeline) = rest.split_once(',').ok_or_else(|| "IT: missing comma".to_string())?;
                    Ok(InsertTimeline(insert_timeline::InsertTimeline { index: parse_usize(index)?, timeline: dec_timeline(timeline)? }))
                }
                "RT" => Ok(RemoveTimeline(remove_timeline::RemoveTimeline { index: parse_usize(rest)? })),
                "TN" => {
                    let (index, name) = rest.split_once(',').ok_or_else(|| "TN: missing comma".to_string())?;
                    let parts = split_top_level(strip_brackets(name)?, ',');
                    let name = match parts.as_slice() {
                        ["0"] => None,
                        [tag, value] if *tag == "1" => Some(dec_str(value)?),
                        other => return Err(format!("TN: bad option shape {other:?}")),
                    };
                    Ok(SetTimelineName(set_timeline_name::SetTimelineName { index: parse_usize(index)?, name }))
                }
                "IC" => {
                    let parts = split_top_level(rest, ',');
                    let [ti, index, rest_channel @ ..] = parts.as_slice() else { return Err("IC: expected 3+ fields".to_string()) };
                    let channel = rest_channel.join(",");
                    Ok(InsertChannel(insert_channel::InsertChannel { timeline_index: parse_usize(ti)?, index: parse_usize(index)?, channel: dec_channel(&channel)? }))
                }
                "RC" => {
                    let (ti, index) = rest.split_once(',').ok_or_else(|| "RC: missing comma".to_string())?;
                    Ok(RemoveChannel(remove_channel::RemoveChannel { timeline_index: parse_usize(ti)?, index: parse_usize(index)? }))
                }
                "CT" => {
                    let parts = split_top_level(rest, ',');
                    let [ti, index, rest_target @ ..] = parts.as_slice() else { return Err("CT: expected 3+ fields".to_string()) };
                    Ok(SetChannelTarget(set_channel_target::SetChannelTarget { timeline_index: parse_usize(ti)?, index: parse_usize(index)?, target: dec_target(&rest_target.join(","))? }))
                }
                "CI" => {
                    let parts: Vec<&str> = rest.splitn(3, ',').collect();
                    let [ti, index, interp] = parts.as_slice() else { return Err("CI: expected 3 fields".to_string()) };
                    Ok(SetChannelInterpolation(set_channel_interpolation::SetChannelInterpolation { timeline_index: parse_usize(ti)?, index: parse_usize(index)?, interpolation: dec_interpolation(interp)? }))
                }
                "IK" => {
                    let parts = split_top_level(rest, ',');
                    let [ti, ci, index, rest_kf @ ..] = parts.as_slice() else { return Err("IK: expected 4+ fields".to_string()) };
                    Ok(InsertKeyframe(insert_keyframe::InsertKeyframe { timeline_index: parse_usize(ti)?, channel_index: parse_usize(ci)?, index: parse_usize(index)?, keyframe: dec_keyframe(&rest_kf.join(","))? }))
                }
                "RK" => {
                    let parts: Vec<&str> = rest.splitn(3, ',').collect();
                    let [ti, ci, index] = parts.as_slice() else { return Err("RK: expected 3 fields".to_string()) };
                    Ok(RemoveKeyframe(remove_keyframe::RemoveKeyframe { timeline_index: parse_usize(ti)?, channel_index: parse_usize(ci)?, index: parse_usize(index)? }))
                }
                "KT" => {
                    let parts: Vec<&str> = rest.splitn(4, ',').collect();
                    let [ti, ci, index, t] = parts.as_slice() else { return Err("KT: expected 4 fields".to_string()) };
                    Ok(SetKeyframeTime(set_keyframe_time::SetKeyframeTime { timeline_index: parse_usize(ti)?, channel_index: parse_usize(ci)?, index: parse_usize(index)?, t: parse_f64(t)? }))
                }
                "KV" => {
                    let parts = split_top_level(rest, ',');
                    let [ti, ci, index, rest_value @ ..] = parts.as_slice() else { return Err("KV: expected 4+ fields".to_string()) };
                    Ok(SetKeyframeValue(set_keyframe_value::SetKeyframeValue { timeline_index: parse_usize(ti)?, channel_index: parse_usize(ci)?, index: parse_usize(index)?, value: dec_value(&rest_value.join(","))? }))
                }
                other => Err(format!("op: unknown tag {other:?}")),
            }
        })()
        .map_err(fail)
    }
}
}
pub use mutations_codec::*;
