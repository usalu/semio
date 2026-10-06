//! 📝️ Text representation codec surface for `stdio.semio.animation.diff`.

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::animation::schema::diff::*;
use crate::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTargetProperty, AnimTimeline, AnimValue, SemioAnimationSnapshot};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion};
use crate::standards::v1::subsets::base::schema::triples::{dec_indexed_triple, enc_indexed_triple, IndexAdded, IndexModified, IndexedTripleDiff};
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::MutationDiff;
use crate::standards::v1::subsets::animation::io::text::snapshot::enc_timeline;
use crate::standards::v1::subsets::animation::io::text::snapshot::dec_timeline;


/// 🎙️ Handcrafted `DiffCodec` grammar: one `timelines{[removed];[modified];[added]}` section
/// (empty diff prints as `""`). Sparse per-field entries inside `modified` use single-letter tags
/// (`N`/`C` for timelines, `G`/`I`/`K` for channels, `T`/`Y` for keyframes); `Option<T>` uses the
/// `[0]`=None / `[1,<T>]`=Some(T) tag shared with the full-item value codecs above. Bytes/strings
/// are lowercase hex — no escaping needed, matching this artifact's own `ArtifactDsl` and the
/// gif 89a precedent. Binary = the text bytes verbatim (same simplification `WriterDiff`'s
/// hand-rolled `DiffCodec` and gif 89a's own `GifDiff` use).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_semio_animation_diff(d: &SemioAnimationDiff) -> String {
    match &d.timelines {
        Some(v) => format!("timelines{{{}}}", enc_indexed_triple(v, enc_timeline_diff, enc_timeline)),
        None => String::new(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_semio_animation_diff(line: &str) -> Result<SemioAnimationDiff, String> {
    if line.is_empty() {
        return Ok(SemioAnimationDiff::default());
    }
    let body = line.strip_prefix("timelines{").and_then(|s| s.strip_suffix('}')).ok_or_else(|| format!("diff: bad shape {line:?}"))?;
    let d = dec_indexed_triple(body, dec_timeline_diff, dec_timeline)?;
    Ok(SemioAnimationDiff { timelines: (!indexed_is_empty(&d)).then_some(d) })
}

impl protocol::DiffText for SemioAnimationDiff {
fn print_diff(&self) -> String {
    print_semio_animation_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_semio_animation_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

/// 🧭️ Full-item ("added"-slot) encoders/decoders for every snapshot-owned value type — reused both
/// by the collection-triple `added` entries and by the sparse per-field diff codecs below.
/// 🎯️ `t`/`r`/`s`/`w` for the unit variants, `c:<hex>` for `Custom{name}` (W2c closer fix: the
/// trailing `:` separator, not `c<hex>` glued directly, is REQUIRED — the shared lexer's
/// `is_ident_continue` includes alphanumerics, so a bare `c` immediately followed by hex digits
/// lexes as ONE fused identifier token, not two; the grammar's `"c" ":" hex` production could
/// never match a glued token otherwise. Matches `📸️snapshot/🦀️.rs`'s own duplicated
/// `enc_property`/`dec_property` field-for-field.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_property(p: &AnimTargetProperty) -> String {
    match p {
        AnimTargetProperty::Translation => "t".to_string(),
        AnimTargetProperty::Rotation => "r".to_string(),
        AnimTargetProperty::Scale => "s".to_string(),
        AnimTargetProperty::Weights => "w".to_string(),
        AnimTargetProperty::Custom { name } => format!("c:{}", enc_str(name)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_property(s: &str) -> Result<AnimTargetProperty, String> {
    match s {
        "t" => Ok(AnimTargetProperty::Translation),
        "r" => Ok(AnimTargetProperty::Rotation),
        "s" => Ok(AnimTargetProperty::Scale),
        "w" => Ok(AnimTargetProperty::Weights),
        other => {
            let rest = other.strip_prefix("c:").ok_or_else(|| format!("bad property {other:?}"))?;
            Ok(AnimTargetProperty::Custom { name: dec_str(rest)? })
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_target(t: &AnimTarget) -> String {
    format!("[{},{}]", enc_str(&t.node), enc_property(&t.property))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_target(s: &str) -> Result<AnimTarget, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [node, prop] = parts.as_slice() else { return Err(format!("target: expected 2 fields, got {}", parts.len())) };
    Ok(AnimTarget { node: dec_str(node)?, property: dec_property(prop)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_interpolation(i: AnimInterpolation) -> char {
    match i {
        AnimInterpolation::Linear => 'l',
        AnimInterpolation::Step => 's',
        AnimInterpolation::CubicSpline => 'c',
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_interpolation(s: &str) -> Result<AnimInterpolation, String> {
    match s {
        "l" => Ok(AnimInterpolation::Linear),
        "s" => Ok(AnimInterpolation::Step),
        "c" => Ok(AnimInterpolation::CubicSpline),
        other => Err(format!("bad interpolation {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_value(v: &AnimValue) -> String {
    match v {
        AnimValue::Scalar { value } => format!("S:{value}"),
        AnimValue::Vec3 { value } => format!("V:[{},{},{}]", value.x, value.y, value.z),
        AnimValue::Quat { value } => format!("Q:[{},{},{},{}]", value.x, value.y, value.z, value.w),
        AnimValue::Weights { values } => format!("W:{}", enc_list(values, |v| v.to_string())),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_value(s: &str) -> Result<AnimValue, String> {
    let (tag, rest) = s.split_once(':').ok_or_else(|| format!("value: bad shape {s:?}"))?;
    match tag {
        "S" => Ok(AnimValue::Scalar { value: parse_f64(rest)? }),
        "V" => {
            let parts = split_top_level(strip_brackets(rest)?, ',');
            let [x, y, z] = parts.as_slice() else { return Err(format!("vec3: expected 3 fields, got {}", parts.len())) };
            Ok(AnimValue::Vec3 { value: SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? } })
        }
        "Q" => {
            let parts = split_top_level(strip_brackets(rest)?, ',');
            let [x, y, z, w] = parts.as_slice() else { return Err(format!("quat: expected 4 fields, got {}", parts.len())) };
            Ok(AnimValue::Quat { value: SemioQuaternion { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)?, w: parse_f64(w)? } })
        }
        "W" => Ok(AnimValue::Weights { values: dec_list(rest, parse_f64)? }),
        other => Err(format!("value: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_keyframe(k: &AnimKeyframe) -> String {
    format!("[{},{}]", k.t, enc_value(&k.value))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_keyframe(s: &str) -> Result<AnimKeyframe, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [t, value] = parts.as_slice() else { return Err(format!("keyframe: expected 2 fields, got {}", parts.len())) };
    Ok(AnimKeyframe { t: parse_f64(t)?, value: dec_value(value)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_channel(c: &AnimChannel) -> String {
    format!("[{},{},{}]", enc_target(&c.target), enc_interpolation(c.interpolation), enc_list(&c.keyframes, enc_keyframe))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_channel(s: &str) -> Result<AnimChannel, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [target, interp, kfs] = parts.as_slice() else { return Err(format!("channel: expected 3 fields, got {}", parts.len())) };
    Ok(AnimChannel { target: dec_target(target)?, interpolation: dec_interpolation(interp)?, keyframes: dec_list(kfs, dec_keyframe)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_timeline(t: &AnimTimeline) -> String {
    format!("[{},{}]", encode_option(&t.name, |n| enc_str(n)), enc_list(&t.channels, enc_channel))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_timeline(s: &str) -> Result<AnimTimeline, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, channels] = parts.as_slice() else { return Err(format!("timeline: expected 2 fields, got {}", parts.len())) };
    Ok(AnimTimeline { name: decode_option(name, dec_str)?, channels: dec_list(channels, dec_channel)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_keyframe_diff(d: &AnimKeyframeDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = d.t {
        parts.push(format!("T:{v}"));
    }
    if let Some(v) = &d.value {
        parts.push(format!("Y:{}", enc_value(v)));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_keyframe_diff(s: &str) -> Result<AnimKeyframeDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = AnimKeyframeDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("keyframe diff: bad entry {entry:?}"))?;
        match tag {
            "T" => d.t = Some(parse_f64(val)?),
            "Y" => d.value = Some(dec_value(val)?),
            other => return Err(format!("keyframe diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_channel_diff(d: &AnimChannelDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &d.target {
        parts.push(format!("G:{}", enc_target(v)));
    }
    if let Some(v) = d.interpolation {
        parts.push(format!("I:{}", enc_interpolation(v)));
    }
    if let Some(v) = &d.keyframes {
        parts.push(format!("K:[{}]", enc_indexed_triple(v, enc_keyframe_diff, enc_keyframe)));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_channel_diff(s: &str) -> Result<AnimChannelDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = AnimChannelDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("channel diff: bad entry {entry:?}"))?;
        match tag {
            "G" => d.target = Some(dec_target(val)?),
            "I" => d.interpolation = Some(dec_interpolation(val)?),
            "K" => d.keyframes = Some(dec_indexed_triple(strip_brackets(val)?, dec_keyframe_diff, dec_keyframe)?),
            other => return Err(format!("channel diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_timeline_diff(d: &AnimTimelineDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &d.name {
        parts.push(format!("N:{}", encode_option(v, |n| enc_str(n))));
    }
    if let Some(v) = &d.channels {
        parts.push(format!("C:[{}]", enc_indexed_triple(v, enc_channel_diff, enc_channel)));
    }
    format!("[{}]", parts.join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_timeline_diff(s: &str) -> Result<AnimTimelineDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = AnimTimelineDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("timeline diff: bad entry {entry:?}"))?;
        match tag {
            "N" => d.name = Some(decode_option(val, dec_str)?),
            "C" => d.channels = Some(dec_indexed_triple(strip_brackets(val)?, dec_channel_diff, dec_channel)?),
            other => return Err(format!("timeline diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}
}
pub use diff_codec::*;
