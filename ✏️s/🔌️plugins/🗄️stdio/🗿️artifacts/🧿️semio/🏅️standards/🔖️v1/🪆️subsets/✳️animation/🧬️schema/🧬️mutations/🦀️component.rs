//! 🧬️ SemioAnimationMutation — full named-variant vocabulary (gif 89a / docx precedent), replacing
//! the W1b `SetSnapshot`-only scaffold. Every variant's `diff()`/`inverse()` is HAND-WRITTEN
//! (apply-and-capture is banned per `🧬️schema-design.md`'s svg infinite-recursion warning) —
//! `diff()` builds the exact sparse `SemioAnimationDiff` directly via the `diff_*` helpers below,
//! never by diffing a mutated clone against `base`.

use crate::artifacts::semio::standards::v1::engine::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::animation::schema::diff::{
    AnimChannelDiff, AnimKeyframeDiff, AnimTimelineDiff, SemioAnimationDiff, diff_set_snapshot,
};
use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::{
    AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTimeline, AnimValue, SemioAnimationSnapshot,
};
use protocol::Mutation;
#[cfg(test)]
/// 🔧️ `MutationDiff` added — the `#[cfg(test)] mod tests` block below calls `diff.apply(&base)`
/// via method syntax on `SemioAnimationDiff`, which needs `MutationDiff` in scope (unlike this
/// file's own `OpText`/`OpBinary` impls, which use fully-qualified syntax and so don't need the
/// import) (W2b closer fix).
use protocol::{MutationDiff, OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🏷️ Variant ordinals for the real binary `OpBinary` frame below (`tag u8`) — declaration order,
/// 0-12. Must stay in lockstep with `variant_ordinal`/`OP_KEYWORDS`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioAnimationMutation {
    #[default]
    NoMutation,
    /// 📦️ Full-snapshot replace, still sparse under the hood (`diff_set_snapshot` = `between`).
    SetSnapshot { snapshot: SemioAnimationSnapshot },
    InsertTimeline { index: usize, timeline: AnimTimeline },
    RemoveTimeline { index: usize },
    /// 🏷️ `name: None` clears the timeline's display name.
    SetTimelineName { index: usize, name: Option<String> },
    InsertChannel { timeline_index: usize, index: usize, channel: AnimChannel },
    RemoveChannel { timeline_index: usize, index: usize },
    SetChannelTarget { timeline_index: usize, index: usize, target: AnimTarget },
    SetChannelInterpolation { timeline_index: usize, index: usize, interpolation: AnimInterpolation },
    InsertKeyframe { timeline_index: usize, channel_index: usize, index: usize, keyframe: AnimKeyframe },
    RemoveKeyframe { timeline_index: usize, channel_index: usize, index: usize },
    SetKeyframeTime { timeline_index: usize, channel_index: usize, index: usize, t: f64 },
    SetKeyframeValue { timeline_index: usize, channel_index: usize, index: usize, value: AnimValue },
}
//#endregion 🔖️Mutation

//#region 🔖️DiffBuilders
/// 🧱️ Wraps a per-timeline `AnimTimelineDiff` into a full `SemioAnimationDiff` — the innermost
/// layer of the nested-diff tree every non-collection-root mutation ultimately builds on.
fn diff_timeline_field(index: usize, diff: AnimTimelineDiff) -> SemioAnimationDiff {
    SemioAnimationDiff { timelines: Some(IndexedTripleDiff { modified: vec![IndexModified { index, diff }], ..Default::default() }) }
}
fn diff_channel_collection(timeline_index: usize, channels: IndexedTripleDiff<AnimChannelDiff, AnimChannel>) -> SemioAnimationDiff {
    diff_timeline_field(timeline_index, AnimTimelineDiff { name: None, channels: Some(channels) })
}
fn diff_channel_field(timeline_index: usize, index: usize, diff: AnimChannelDiff) -> SemioAnimationDiff {
    diff_channel_collection(timeline_index, IndexedTripleDiff { modified: vec![IndexModified { index, diff }], ..Default::default() })
}
fn diff_keyframe_collection(timeline_index: usize, channel_index: usize, keyframes: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe>) -> SemioAnimationDiff {
    diff_channel_field(timeline_index, channel_index, AnimChannelDiff { target: None, interpolation: None, keyframes: Some(keyframes) })
}
fn diff_keyframe_field(timeline_index: usize, channel_index: usize, index: usize, diff: AnimKeyframeDiff) -> SemioAnimationDiff {
    diff_keyframe_collection(timeline_index, channel_index, IndexedTripleDiff { modified: vec![IndexModified { index, diff }], ..Default::default() })
}
//#endregion 🔖️DiffBuilders

//#region 🔖️BaseAccessors
fn timeline_at(base: &SemioAnimationSnapshot, i: usize) -> Option<&AnimTimeline> {
    base.timelines.get(i)
}
fn channel_at(base: &SemioAnimationSnapshot, ti: usize, ci: usize) -> Option<&AnimChannel> {
    base.timelines.get(ti).and_then(|t| t.channels.get(ci))
}
fn keyframe_at(base: &SemioAnimationSnapshot, ti: usize, ci: usize, ki: usize) -> Option<&AnimKeyframe> {
    base.timelines.get(ti).and_then(|t| t.channels.get(ci)).and_then(|c| c.keyframes.get(ki))
}
//#endregion 🔖️BaseAccessors

impl Mutation<SemioAnimationSnapshot> for SemioAnimationMutation {
    type Diff = SemioAnimationDiff;

    fn diff(&self, base: &SemioAnimationSnapshot) -> Self::Diff {
        use SemioAnimationMutation::*;
        match self {
            NoMutation => SemioAnimationDiff::default(),
            SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            InsertTimeline { index, timeline } => SemioAnimationDiff {
                timelines: Some(IndexedTripleDiff { added: vec![IndexAdded { index: *index, item: timeline.clone() }], ..Default::default() }),
            },
            RemoveTimeline { index } => SemioAnimationDiff {
                timelines: Some(IndexedTripleDiff { removed: vec![*index], ..Default::default() }),
            },
            SetTimelineName { index, name } => diff_timeline_field(*index, AnimTimelineDiff { name: Some(name.clone()), channels: None }),
            InsertChannel { timeline_index, index, channel } => diff_channel_collection(
                *timeline_index,
                IndexedTripleDiff { added: vec![IndexAdded { index: *index, item: channel.clone() }], ..Default::default() },
            ),
            RemoveChannel { timeline_index, index } => diff_channel_collection(*timeline_index, IndexedTripleDiff { removed: vec![*index], ..Default::default() }),
            SetChannelTarget { timeline_index, index, target } => {
                diff_channel_field(*timeline_index, *index, AnimChannelDiff { target: Some(target.clone()), interpolation: None, keyframes: None })
            }
            SetChannelInterpolation { timeline_index, index, interpolation } => {
                diff_channel_field(*timeline_index, *index, AnimChannelDiff { target: None, interpolation: Some(*interpolation), keyframes: None })
            }
            InsertKeyframe { timeline_index, channel_index, index, keyframe } => diff_keyframe_collection(
                *timeline_index,
                *channel_index,
                IndexedTripleDiff { added: vec![IndexAdded { index: *index, item: keyframe.clone() }], ..Default::default() },
            ),
            RemoveKeyframe { timeline_index, channel_index, index } => {
                diff_keyframe_collection(*timeline_index, *channel_index, IndexedTripleDiff { removed: vec![*index], ..Default::default() })
            }
            SetKeyframeTime { timeline_index, channel_index, index, t } => {
                diff_keyframe_field(*timeline_index, *channel_index, *index, AnimKeyframeDiff { t: Some(*t), value: None })
            }
            SetKeyframeValue { timeline_index, channel_index, index, value } => {
                diff_keyframe_field(*timeline_index, *channel_index, *index, AnimKeyframeDiff { t: None, value: Some(value.clone()) })
            }
        }
    }

    fn inverse(&self, base: &SemioAnimationSnapshot) -> Vec<Self> {
        use SemioAnimationMutation::*;
        match self {
            NoMutation => vec![NoMutation],
            SetSnapshot { .. } => vec![SetSnapshot { snapshot: base.clone() }],
            InsertTimeline { index, .. } => vec![RemoveTimeline { index: *index }],
            RemoveTimeline { index } => match timeline_at(base, *index) {
                Some(t) => vec![InsertTimeline { index: *index, timeline: t.clone() }],
                None => vec![NoMutation],
            },
            SetTimelineName { index, .. } => vec![SetTimelineName { index: *index, name: timeline_at(base, *index).and_then(|t| t.name.clone()) }],
            InsertChannel { timeline_index, index, .. } => vec![RemoveChannel { timeline_index: *timeline_index, index: *index }],
            RemoveChannel { timeline_index, index } => match channel_at(base, *timeline_index, *index) {
                Some(c) => vec![InsertChannel { timeline_index: *timeline_index, index: *index, channel: c.clone() }],
                None => vec![NoMutation],
            },
            SetChannelTarget { timeline_index, index, .. } => match channel_at(base, *timeline_index, *index) {
                Some(c) => vec![SetChannelTarget { timeline_index: *timeline_index, index: *index, target: c.target.clone() }],
                None => vec![NoMutation],
            },
            SetChannelInterpolation { timeline_index, index, .. } => match channel_at(base, *timeline_index, *index) {
                Some(c) => vec![SetChannelInterpolation { timeline_index: *timeline_index, index: *index, interpolation: c.interpolation }],
                None => vec![NoMutation],
            },
            InsertKeyframe { timeline_index, channel_index, index, .. } => {
                vec![RemoveKeyframe { timeline_index: *timeline_index, channel_index: *channel_index, index: *index }]
            }
            RemoveKeyframe { timeline_index, channel_index, index } => match keyframe_at(base, *timeline_index, *channel_index, *index) {
                Some(k) => vec![InsertKeyframe { timeline_index: *timeline_index, channel_index: *channel_index, index: *index, keyframe: k.clone() }],
                None => vec![NoMutation],
            },
            SetKeyframeTime { timeline_index, channel_index, index, .. } => match keyframe_at(base, *timeline_index, *channel_index, *index) {
                Some(k) => vec![SetKeyframeTime { timeline_index: *timeline_index, channel_index: *channel_index, index: *index, t: k.t }],
                None => vec![NoMutation],
            },
            SetKeyframeValue { timeline_index, channel_index, index, .. } => match keyframe_at(base, *timeline_index, *channel_index, *index) {
                Some(k) => vec![SetKeyframeValue { timeline_index: *timeline_index, channel_index: *channel_index, index: *index, value: k.value.clone() }],
                None => vec![NoMutation],
            },
        }
    }
}

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (mirrors gif's
/// `apply_gif_mutation` convention — used by the builder's `mutate()` and the set-snapshot leaf).
pub fn apply_semio_animation_mutation(snapshot: &mut SemioAnimationSnapshot, mutation: &SemioAnimationMutation) -> SemioAnimationDiff {
    let diff = <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::diff(mutation, snapshot);
    *snapshot = <SemioAnimationDiff as protocol::MutationDiff<SemioAnimationSnapshot>>::apply(&diff, snapshot);
    diff
}

//#region SnapshotLit
/// 🧩️ `SetSnapshot`'s whole-snapshot payload — `[hex(schema),[timeline,...]]`, reusing the diff
/// facet's own `pub(crate)` `enc_timeline`/`dec_timeline`/`enc_str`/`dec_str`/`enc_list`/`dec_list`
/// value codecs (one source of truth, not a third independent copy). W2c closer fix: this REPLACES
/// the old whole-enum `serde_json::to_string`/`from_str` passthrough — a real JSON-transfer-ban
/// violation the brief specifically flagged as a recurring pattern to check for (confirmed present
/// here, unlike the sibling `🔺️diff` facet, which was already fully real pre-wave).
fn enc_animation_snapshot(s: &SemioAnimationSnapshot) -> String {
    use crate::artifacts::semio::standards::v1::subsets::animation::schema::diff::{enc_list, enc_str, enc_timeline};
    format!("[{},{}]", enc_str(&s.schema), enc_list(&s.timelines, enc_timeline))
}
fn dec_animation_snapshot(s: &str) -> Result<SemioAnimationSnapshot, String> {
    use crate::artifacts::semio::standards::v1::engine::triples::{split_top_level, strip_brackets};
    use crate::artifacts::semio::standards::v1::subsets::animation::schema::diff::{dec_list, dec_str, dec_timeline};
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, timelines] = parts.as_slice() else { return Err(format!("snapshot-lit: expected 2 fields, got {}", parts.len())) };
    Ok(SemioAnimationSnapshot { schema: dec_str(schema)?, timelines: dec_list(timelines, dec_timeline)? })
}
//#endregion SnapshotLit

//#region OpCodecs
/// 🎙️ Handcrafted `OpText`/`OpBinary` — one `TAG:payload` line per variant, reusing the diff
/// module's `pub(crate)` value codecs (`enc_timeline`/`enc_channel`/`enc_keyframe`/`enc_target`/
/// `enc_value`/`enc_interpolation`/hex-string helpers) instead of re-deriving a second parallel
/// grammar. `SetSnapshot` reuses the `enc_animation_snapshot`/`dec_animation_snapshot` whole-
/// snapshot codec above (W2c closer fix — was `serde_json`, see that region's doc comment).
impl protocol::OpText for SemioAnimationMutation {
    fn print_op(&self) -> String {
        use crate::artifacts::semio::standards::v1::subsets::animation::schema::diff::{
            enc_channel, enc_interpolation, enc_keyframe, enc_str, enc_target, enc_timeline, enc_value,
        };
        use SemioAnimationMutation::*;
        match self {
            NoMutation => "N".to_string(),
            SetSnapshot { snapshot } => format!("S:{}", enc_animation_snapshot(snapshot)),
            InsertTimeline { index, timeline } => format!("IT:{index},{}", enc_timeline(timeline)),
            RemoveTimeline { index } => format!("RT:{index}"),
            SetTimelineName { index, name } => format!("TN:{index},{}", match name { None => "[0]".to_string(), Some(n) => format!("[1,{}]", enc_str(n)) }),
            InsertChannel { timeline_index, index, channel } => format!("IC:{timeline_index},{index},{}", enc_channel(channel)),
            RemoveChannel { timeline_index, index } => format!("RC:{timeline_index},{index}"),
            SetChannelTarget { timeline_index, index, target } => format!("CT:{timeline_index},{index},{}", enc_target(target)),
            SetChannelInterpolation { timeline_index, index, interpolation } => format!("CI:{timeline_index},{index},{}", enc_interpolation(*interpolation)),
            InsertKeyframe { timeline_index, channel_index, index, keyframe } => format!("IK:{timeline_index},{channel_index},{index},{}", enc_keyframe(keyframe)),
            RemoveKeyframe { timeline_index, channel_index, index } => format!("RK:{timeline_index},{channel_index},{index}"),
            SetKeyframeTime { timeline_index, channel_index, index, t } => format!("KT:{timeline_index},{channel_index},{index},{t}"),
            SetKeyframeValue { timeline_index, channel_index, index, value } => format!("KV:{timeline_index},{channel_index},{index},{}", enc_value(value)),
        }
    }

    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        use crate::artifacts::semio::standards::v1::engine::triples::{split_top_level, strip_brackets};
        use crate::artifacts::semio::standards::v1::subsets::animation::schema::diff::{
            dec_channel, dec_interpolation, dec_keyframe, dec_str, dec_target, dec_timeline, dec_value,
        };
        use SemioAnimationMutation::*;
        let fail = |e: String| store::TextError::new(e, dsl::TextSpan::at(1, 1));
        let parse_usize = |s: &str| s.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string());
        let parse_f64 = |s: &str| s.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string());

        if line == "N" {
            return Ok(NoMutation);
        }
        let (tag, rest) = line.split_once(':').ok_or_else(|| fail(format!("op: bad shape {line:?}")))?;
        (|| -> Result<Self, String> {
            match tag {
                "S" => Ok(SetSnapshot { snapshot: dec_animation_snapshot(rest)? }),
                "IT" => {
                    let (index, timeline) = rest.split_once(',').ok_or_else(|| "IT: missing comma".to_string())?;
                    Ok(InsertTimeline { index: parse_usize(index)?, timeline: dec_timeline(timeline)? })
                }
                "RT" => Ok(RemoveTimeline { index: parse_usize(rest)? }),
                "TN" => {
                    let (index, name) = rest.split_once(',').ok_or_else(|| "TN: missing comma".to_string())?;
                    let parts = split_top_level(strip_brackets(name)?, ',');
                    let name = match parts.as_slice() {
                        ["0"] => None,
                        [tag, value] if *tag == "1" => Some(dec_str(value)?),
                        other => return Err(format!("TN: bad option shape {other:?}")),
                    };
                    Ok(SetTimelineName { index: parse_usize(index)?, name })
                }
                "IC" => {
                    let parts = split_top_level(rest, ',');
                    let [ti, index, rest_channel @ ..] = parts.as_slice() else { return Err("IC: expected 3+ fields".to_string()) };
                    let channel = rest_channel.join(",");
                    Ok(InsertChannel { timeline_index: parse_usize(ti)?, index: parse_usize(index)?, channel: dec_channel(&channel)? })
                }
                "RC" => {
                    let (ti, index) = rest.split_once(',').ok_or_else(|| "RC: missing comma".to_string())?;
                    Ok(RemoveChannel { timeline_index: parse_usize(ti)?, index: parse_usize(index)? })
                }
                "CT" => {
                    let parts = split_top_level(rest, ',');
                    let [ti, index, rest_target @ ..] = parts.as_slice() else { return Err("CT: expected 3+ fields".to_string()) };
                    Ok(SetChannelTarget { timeline_index: parse_usize(ti)?, index: parse_usize(index)?, target: dec_target(&rest_target.join(","))? })
                }
                "CI" => {
                    let parts: Vec<&str> = rest.splitn(3, ',').collect();
                    let [ti, index, interp] = parts.as_slice() else { return Err("CI: expected 3 fields".to_string()) };
                    Ok(SetChannelInterpolation { timeline_index: parse_usize(ti)?, index: parse_usize(index)?, interpolation: dec_interpolation(interp)? })
                }
                "IK" => {
                    let parts = split_top_level(rest, ',');
                    let [ti, ci, index, rest_kf @ ..] = parts.as_slice() else { return Err("IK: expected 4+ fields".to_string()) };
                    Ok(InsertKeyframe {
                        timeline_index: parse_usize(ti)?,
                        channel_index: parse_usize(ci)?,
                        index: parse_usize(index)?,
                        keyframe: dec_keyframe(&rest_kf.join(","))?,
                    })
                }
                "RK" => {
                    let parts: Vec<&str> = rest.splitn(3, ',').collect();
                    let [ti, ci, index] = parts.as_slice() else { return Err("RK: expected 3 fields".to_string()) };
                    Ok(RemoveKeyframe { timeline_index: parse_usize(ti)?, channel_index: parse_usize(ci)?, index: parse_usize(index)? })
                }
                "KT" => {
                    let parts: Vec<&str> = rest.splitn(4, ',').collect();
                    let [ti, ci, index, t] = parts.as_slice() else { return Err("KT: expected 4 fields".to_string()) };
                    Ok(SetKeyframeTime { timeline_index: parse_usize(ti)?, channel_index: parse_usize(ci)?, index: parse_usize(index)?, t: parse_f64(t)? })
                }
                "KV" => {
                    let parts = split_top_level(rest, ',');
                    let [ti, ci, index, rest_value @ ..] = parts.as_slice() else { return Err("KV: expected 4+ fields".to_string()) };
                    Ok(SetKeyframeValue {
                        timeline_index: parse_usize(ti)?,
                        channel_index: parse_usize(ci)?,
                        index: parse_usize(index)?,
                        value: dec_value(&rest_value.join(","))?,
                    })
                }
                other => Err(format!("op: unknown tag {other:?}")),
            }
        })()
        .map_err(fail)
    }
}

/// 🏷️ `SemioAnimationMutation` variant ordinals — declaration order, 0-12 (matches
/// `parse_op`'s own keyword match). Used by the real `OpBinary` frame below.
const OP_KEYWORDS: [&str; 13] = ["N", "S", "IT", "RT", "TN", "IC", "RC", "CT", "CI", "IK", "RK", "KT", "KV"];
fn variant_ordinal(m: &SemioAnimationMutation) -> u8 {
    use SemioAnimationMutation::*;
    match m {
        NoMutation => 0,
        SetSnapshot { .. } => 1,
        InsertTimeline { .. } => 2,
        RemoveTimeline { .. } => 3,
        SetTimelineName { .. } => 4,
        InsertChannel { .. } => 5,
        RemoveChannel { .. } => 6,
        SetChannelTarget { .. } => 7,
        SetChannelInterpolation { .. } => 8,
        InsertKeyframe { .. } => 9,
        RemoveKeyframe { .. } => 10,
        SetKeyframeTime { .. } => 11,
        SetKeyframeValue { .. } => 12,
    }
}
const OP_BINARY_FORMAT: u8 = 1;

/// 🔢️ Real binary op frame (animation wave — off the old whole-`OpText`-line `.into_bytes()` F6
/// text-as-binary shortcut). `format u8` + `tag u8` (the variant ordinal above) as two real fixed
/// fields, then the variant's own `key=value,...` argument text (i.e. `print_op`'s output with its
/// `TAG:` prefix stripped) as one opaque trailing `bytes` chain — reuses the real, tested
/// `print_op`/`parse_op` text codec (one source of truth), same treatment every prior semio wave's
/// `OpBinary` upgrade uses.
impl protocol::OpBinary for SemioAnimationMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let printed = <Self as protocol::OpText>::print_op(self);
        let args = match printed.split_once(':') {
            Some((_, rest)) => rest,
            None => "",
        };
        let mut out = vec![OP_BINARY_FORMAT, variant_ordinal(self)];
        out.extend_from_slice(args.as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let malformed = |what: &'static str, detail: String| protocol::ProtocolError::Malformed { what, offset: 0, detail };
        let [format, tag, rest @ ..] = bytes else { return Err(malformed("op header", format!("expected at least 2 bytes, got {}", bytes.len()))) };
        if *format != OP_BINARY_FORMAT {
            return Err(malformed("op format", format!("unsupported op format {format}")));
        }
        let keyword = OP_KEYWORDS.get(*tag as usize).ok_or_else(|| malformed("op tag", format!("unknown op tag {tag}")))?;
        let args = std::str::from_utf8(rest).map_err(|e| malformed("op args utf8", e.to_string()))?;
        let line = if *keyword == "N" { "N".to_string() } else { format!("{keyword}:{args}") };
        <Self as protocol::OpText>::parse_op(&line).map_err(|e| malformed("op text", e.to_string()))
    }
}
//#endregion OpCodecs

/// 🧱️ Module-scope (not `mod tests`-local) fixture + demo mutation cases — so the `🎹️composer`
/// conformance-law tests can reuse them, same promotion pattern every prior semio wave's report
/// documents (a private item of a child `mod tests` isn't visible to a sibling module).
#[cfg(test)]
fn fixture() -> SemioAnimationSnapshot {
    SemioAnimationSnapshot {
        timelines: vec![AnimTimeline {
            name: Some("walk".into()),
            channels: vec![AnimChannel {
                target: AnimTarget { node: "hip".into(), property: crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::AnimTargetProperty::Translation },
                interpolation: AnimInterpolation::Linear,
                keyframes: vec![AnimKeyframe { t: 0.0, value: AnimValue::Scalar { value: 1.0 } }],
            }],
        }],
        ..SemioAnimationSnapshot::default()
    }
}

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<SemioAnimationMutation> {
    let base = fixture();
    use SemioAnimationMutation::*;
    vec![
        NoMutation,
        SetSnapshot { snapshot: base.clone() },
        InsertTimeline { index: 1, timeline: AnimTimeline { name: Some("wave".into()), channels: vec![] } },
        RemoveTimeline { index: 0 },
        SetTimelineName { index: 0, name: None },
        InsertChannel { timeline_index: 0, index: 1, channel: base.timelines[0].channels[0].clone() },
        RemoveChannel { timeline_index: 0, index: 0 },
        SetChannelTarget { timeline_index: 0, index: 0, target: AnimTarget { node: "spine".into(), property: crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::AnimTargetProperty::Rotation } },
        SetChannelInterpolation { timeline_index: 0, index: 0, interpolation: AnimInterpolation::Step },
        InsertKeyframe { timeline_index: 0, channel_index: 0, index: 1, keyframe: AnimKeyframe { t: 2.0, value: AnimValue::Scalar { value: 5.0 } } },
        RemoveKeyframe { timeline_index: 0, channel_index: 0, index: 0 },
        SetKeyframeTime { timeline_index: 0, channel_index: 0, index: 0, t: 3.5 },
        SetKeyframeValue { timeline_index: 0, channel_index: 0, index: 0, value: AnimValue::Weights { values: vec![0.1, 0.9] } },
    ]
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ mutation_diff_law: `m.diff(base).apply(base) == { apply_x_mutation(&mut s, m); s }`.
    #[test]
    fn mutation_diff_law_covers_every_variant() {
        let base = fixture();
        for m in demo_mutation_cases() {
            let diff = <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::diff(&m, &base);
            let via_diff = diff.apply(&base);

            let mut applied = base.clone();
            let returned_diff = apply_semio_animation_mutation(&mut applied, &m);

            assert_eq!(via_diff, applied, "diff().apply(base) must match apply_semio_animation_mutation's result for {m:?}");
            assert_eq!(returned_diff, diff, "apply_semio_animation_mutation must return the same diff as Mutation::diff for {m:?}");
        }
    }

    /// 🧪️ inverse_law: every variant's inverse restores `base` when applied after the mutation.
    #[test]
    fn inverse_law_covers_every_variant() {
        let base = fixture();
        for m in demo_mutation_cases() {
            let mut mutated = base.clone();
            let _ = apply_semio_animation_mutation(&mut mutated, &m);
            let inv = <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::inverse(&m, &base);
            assert_eq!(inv.len(), 1, "every variant's inverse is exactly 1 mutation for {m:?}");
            let mut restored = mutated.clone();
            let _ = apply_semio_animation_mutation(&mut restored, &inv[0]);
            assert_eq!(restored, base, "inverse must restore base for {m:?}");
        }
    }

    /// 🧪️ op_text_binary_roundtrip_law: handcrafted `OpText`/`OpBinary` round trip for every
    /// variant.
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = fixture();
        for m in demo_mutation_cases() {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioAnimationMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = SemioAnimationMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
}
//#endregion 🔖️Tests
