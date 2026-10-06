//! 📝️ Text representation codec surface for `stdio.semio.animation.snapshot`.

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::animation::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion};
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;

/// 🧪️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION animation wave (following the
/// flow pilot's proven template, `ws-codec-workflow-report.md`, and brep's/drawing's own
/// generalization to data-carrying tagged enums, `ws-codec-brep-report.md`): real hex/bracket-
/// encoded value primitives backing the hand-rolled `ArtifactDsl` below, replacing the old
/// hex-of-`serde_json` passthrough. Duplicated here (not imported from `schema::diff`, which
/// depends ON this module for its own plain types) to keep `snapshot` free of a reverse
/// dependency — same convention brep's own wave established. Field order/tag letters match this
/// subset's own `🔺️diff/🦀️.rs` `ValueCodecs` region exactly (the pre-existing, already-
/// real hand-rolled text convention this wave generalizes, not invents).
///
/// 🧩️ The `#[derive(dsl::DslArtifact)]` path remains blocked here for the SAME reason brep's own
/// wave hit: `AnimTargetProperty`/`AnimValue` are data-carrying TAGGED ENUMS whose variants hold
/// DIFFERENT field sets (`Translation`/`Rotation`/`Scale`/`Weights`/`Custom{name}`,
/// `Scalar{value:f64}`/`Vec3{value:SemioPoint3}`/`Quat{value:SemioQuaternion}`/
/// `Weights{values:Vec<f64>}`) — even though their own scalar/record payload fields
/// (`SemioPoint3`/`SemioQuaternion`) are `dsl::DslRecord`-derivable, no `DslEnum`-over-
/// heterogeneous-payload-shape mechanism is proven to emit a matching TEXT production set
/// (`semio-tagged-enum-heterogeneous-variants-no-dslenum-text-path`, brep's own gap, re-hit here).
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
    native::parse(s)
}

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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

/// 🎯️ `t`/`r`/`s`/`w` for the unit variants, `c:<hex>` for `Custom{name}` — the trailing `:`
/// separator (not `c<hex>` glued directly) is REQUIRED: the shared lexer's `is_ident_continue`
/// includes alphanumerics, so a bare `c` immediately followed by hex digits would lex as ONE fused
/// identifier token (`c68656c6c6f`), not two (`c` then a separate hex run) — the grammar's `"c" ":"
/// hex` production could never match a glued token. Same class of authoring pitfall the grammar
/// recipe's own pitfall #2 warns about, just at the LEXER-fusion level rather than `Symbol::Star`
/// backtracking.
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
pub(crate) fn enc_point3(p: &SemioPoint3) -> String {
    format!("[{},{},{}]", native::NativeF64(p.x), native::NativeF64(p.y), native::NativeF64(p.z))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point3(s: &str) -> Result<SemioPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_quat(q: &SemioQuaternion) -> String {
    format!("[{},{},{},{}]", native::NativeF64(q.x), native::NativeF64(q.y), native::NativeF64(q.z), native::NativeF64(q.w))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_quat(s: &str) -> Result<SemioQuaternion, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z, w] = parts.as_slice() else { return Err(format!("quat: expected 4 fields, got {}", parts.len())) };
    Ok(SemioQuaternion { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)?, w: parse_f64(w)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_value(v: &AnimValue) -> String {
    match v {
        AnimValue::Scalar { value } => format!("S:{}",native::NativeF64(*value)),
        AnimValue::Vec3 { value } => format!("V:{}", enc_point3(value)),
        AnimValue::Quat { value } => format!("Q:{}", enc_quat(value)),
        AnimValue::Weights { values } => format!("W:{}", enc_list(values, |v: &f64| native::NativeF64(*v).to_string())),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_value(s: &str) -> Result<AnimValue, String> {
    let (tag, rest) = s.split_once(':').ok_or_else(|| format!("value: bad shape {s:?}"))?;
    match tag {
        "S" => Ok(AnimValue::Scalar { value: parse_f64(rest)? }),
        "V" => Ok(AnimValue::Vec3 { value: dec_point3(rest)? }),
        "Q" => Ok(AnimValue::Quat { value: dec_quat(rest)? }),
        "W" => Ok(AnimValue::Weights { values: dec_list(rest, parse_f64)? }),
        other => Err(format!("value: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_keyframe(k: &AnimKeyframe) -> String {
    format!("[{},{}]", native::NativeF64(k.t), enc_value(&k.value))
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
    format!("[{},{}]", encode_option(&t.name, |n: &String| enc_str(n)), enc_list(&t.channels, enc_channel))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_timeline(s: &str) -> Result<AnimTimeline, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, channels] = parts.as_slice() else { return Err(format!("timeline: expected 2 fields, got {}", parts.len())) };
    Ok(AnimTimeline { name: decode_option(name, dec_str)?, channels: dec_list(channels, dec_channel)? })
}

/// 📄️ The real structured text body: two lines — `schema=<hex>`, `timelines=[...]` — matching the
/// grammar's `document = artifact-mark schema-line timelines-line`. Newlines are pure lexer trivia
/// in the shared dialect, so this is genuinely recognizable by `dsl::Recognizer`, not merely
/// readable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_animation_snapshot_body(s: &SemioAnimationSnapshot) -> String {
    format!("schema={}\ntimelines=[{}]", enc_str(&s.schema), s.timelines.iter().map(enc_timeline).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_animation_snapshot_body(body: &str) -> Result<SemioAnimationSnapshot, String> {
    let mut schema = None;
    let mut timelines = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("timelines=") {
            timelines = split_top_level(strip_brackets(rest)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_timeline).collect::<Result<Vec<_>, String>>()?;
        } else {
            return Err(format!("animation snapshot: unknown line {line:?}"));
        }
    }
    let schema = schema.ok_or_else(|| "animation snapshot: missing schema line".to_string())?;
    Ok(SemioAnimationSnapshot { schema, timelines })
}

/// 🎁 Real structured text/binary codecs (animation wave — off the old hex-dump-of-`serde_json`
/// shortcut, following the flow pilot's proven template). Wrapped in the repo-wide
/// `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioAnimationSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_animation_snapshot_body(body).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_animation_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📝️ Free-function face of [`SemioAnimationSnapshot`]'s own `store::ArtifactDsl` text codec. `ArtifactDsl` is
/// declared by the os-kernel, which is an INTERNAL dependency of this plugin (aliased `store` in
/// `🦀️.rs`) and is therefore not nameable by a consumer that links only this crate — a
/// generated test host being the concrete case. The codec itself is the subset's, so its entry
/// point belongs here rather than behind a trait the caller cannot import.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_animation_dsl(text: &str) -> Result<SemioAnimationSnapshot, String> {
    <SemioAnimationSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| e.to_string())
}

/// 📝️ Free-function face of [`SemioAnimationSnapshot`]'s own `store::ArtifactDsl` printer — see
/// [`parse_semio_animation_dsl`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_animation_dsl(snapshot: &SemioAnimationSnapshot) -> String {
    <SemioAnimationSnapshot as store::ArtifactDsl>::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;
