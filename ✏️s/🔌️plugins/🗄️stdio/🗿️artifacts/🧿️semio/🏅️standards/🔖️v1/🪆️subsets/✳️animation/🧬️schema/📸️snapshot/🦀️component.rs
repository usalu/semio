//! 🧬️ SemioAnimationSnapshot — complete per the master plan's animation cell: `timelines` ->
//! `channels{target{node,property}, interpolation, keyframes{t, value}}` — informed by gltf's
//! `Animation`/`Channel`/`Sampler` triad (`asset/animations[]`). Ticket
//! 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W2b: replaces the
//! W1b `AnimTimeline{channels:Vec<AnimChannel{target:String,keyframes}>}` minimal scaffold with the
//! full spec shape (typed `target`/`interpolation`, the 4-variant `AnimValue` union). Named structs
//! throughout — no bare tuples (f6-final-summary.md §4.3), rotation reuses the shared
//! `engine::geometry::SemioQuaternion{x,y,z,w}` instead of a local 4-field redefinition.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioQuaternion};
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA: &str = "s.stdio.semio.animation";
//#endregion 🔖️Ids

//#region 🔖️Target
/// 🎯️ Which property of a node a channel drives — gltf `channel.target.path`, widened with a
/// `Custom` escape hatch for engine/extension-defined paths gltf's own spec leaves open
/// (`KHR_*` animation-pointer style extensions target arbitrary properties by name).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum AnimTargetProperty {
    Translation,
    Rotation,
    Scale,
    Weights,
    Custom { name: String },
}

impl Default for AnimTargetProperty {
    async fn default() -> Self {
        AnimTargetProperty::Translation
    }
}

/// 🎯️ A channel's animated node + which of its properties is driven.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimTarget {
    pub node: String,
    #[serde(default)]
    pub property: AnimTargetProperty,
}
//#endregion 🔖️Target

//#region 🔖️Interpolation
/// 📈️ gltf `sampler.interpolation` — how `keyframes` are resampled between `t` values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AnimInterpolation {
    Linear,
    Step,
    CubicSpline,
}

impl Default for AnimInterpolation {
    async fn default() -> Self {
        AnimInterpolation::Linear
    }
}
//#endregion 🔖️Interpolation

//#region 🔖️Value
/// 🎞️ One keyframe's payload — a tagged union over the shapes a channel's `AnimTargetProperty` can
/// take: `Scalar` for a single animated number (e.g. a custom/extension property), `Vec3` for
/// translation/scale, `Quat` for rotation (reuses the shared named quaternion, never a bare
/// `[f64;4]`), `Weights` for morph-target weight vectors (arity = mesh's own primitive count, not
/// fixed — hence `Vec<f64>`, not a fixed array).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum AnimValue {
    Scalar { value: f64 },
    Vec3 { value: SemioPoint3 },
    Quat { value: SemioQuaternion },
    Weights { values: Vec<f64> },
}

impl Default for AnimValue {
    async fn default() -> Self {
        AnimValue::Scalar { value: 0.0 }
    }
}
//#endregion 🔖️Value

//#region 🔖️Keyframe
/// ⏱️ One sample point on a channel's timeline. Real GIFs/glTF exporters expect `t` non-decreasing
/// across a channel's own `keyframes` (a `SubsetValidator` referential invariant, see the
/// `🎹️composer` module) but this type itself stores whatever was decoded, honestly.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimKeyframe {
    pub t: f64,
    #[serde(default)]
    pub value: AnimValue,
}
//#endregion 🔖️Keyframe

//#region 🔖️Channel
/// 🎚️ One animated property track: gltf `channel` + its `sampler`, flattened into a single owned
/// keyframe list (this snapshot does not separately model gltf's accessor-indirection — the
/// keyframes ARE the resolved sample data).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimChannel {
    pub target: AnimTarget,
    #[serde(default)]
    pub interpolation: AnimInterpolation,
    #[serde(default)]
    pub keyframes: Vec<AnimKeyframe>,
}
//#endregion 🔖️Channel

//#region 🔖️Timeline
/// 🎬️ One gltf `animation` entry — an optional display `name` (gltf's own `animation.name` is
/// optional and not spec-required to be unique, hence `Option<String>` rather than a name key) plus
/// its ordered `channels`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimTimeline {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub channels: Vec<AnimChannel>,
}
//#endregion 🔖️Timeline

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.animation")]
pub struct SemioAnimationSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub timelines: Vec<AnimTimeline>,
}

impl Default for SemioAnimationSnapshot {
    async fn default() -> Self {
        Self { schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(), timelines: Default::default() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives
/// 🧪️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION animation wave (following the
/// flow pilot's proven template, `ws-codec-workflow-report.md`, and brep's/drawing's own
/// generalization to data-carrying tagged enums, `ws-codec-brep-report.md`): real hex/bracket-
/// encoded value primitives backing the hand-rolled `ArtifactDsl` below, replacing the old
/// hex-of-`serde_json` passthrough. Duplicated here (not imported from `schema::diff`, which
/// depends ON this module for its own plain types) to keep `snapshot` free of a reverse
/// dependency — same convention brep's own wave established. Field order/tag letters match this
/// subset's own `🔺️diff/🦀️component.rs` `ValueCodecs` region exactly (the pre-existing, already-
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
async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
async fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
async fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
async fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
async fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
async fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
async fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|i| enc(i)).collect::<Vec<_>>().join(","))
}
async fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

/// 🎯️ `t`/`r`/`s`/`w` for the unit variants, `c:<hex>` for `Custom{name}` — the trailing `:`
/// separator (not `c<hex>` glued directly) is REQUIRED: the shared lexer's `is_ident_continue`
/// includes alphanumerics, so a bare `c` immediately followed by hex digits would lex as ONE fused
/// identifier token (`c68656c6c6f`), not two (`c` then a separate hex run) — the grammar's `"c" ":"
/// hex` production could never match a glued token. Same class of authoring pitfall the grammar
/// recipe's own pitfall #2 warns about, just at the LEXER-fusion level rather than `Symbol::Star`
/// backtracking.
async fn enc_property(p: &AnimTargetProperty) -> String {
    match p {
        AnimTargetProperty::Translation => "t".to_string(),
        AnimTargetProperty::Rotation => "r".to_string(),
        AnimTargetProperty::Scale => "s".to_string(),
        AnimTargetProperty::Weights => "w".to_string(),
        AnimTargetProperty::Custom { name } => format!("c:{}", enc_str(name)),
    }
}
async fn dec_property(s: &str) -> Result<AnimTargetProperty, String> {
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
async fn enc_target(t: &AnimTarget) -> String {
    format!("[{},{}]", enc_str(&t.node), enc_property(&t.property))
}
async fn dec_target(s: &str) -> Result<AnimTarget, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [node, prop] = parts.as_slice() else { return Err(format!("target: expected 2 fields, got {}", parts.len())) };
    Ok(AnimTarget { node: dec_str(node)?, property: dec_property(prop)? })
}
async fn enc_interpolation(i: AnimInterpolation) -> char {
    match i {
        AnimInterpolation::Linear => 'l',
        AnimInterpolation::Step => 's',
        AnimInterpolation::CubicSpline => 'c',
    }
}
async fn dec_interpolation(s: &str) -> Result<AnimInterpolation, String> {
    match s {
        "l" => Ok(AnimInterpolation::Linear),
        "s" => Ok(AnimInterpolation::Step),
        "c" => Ok(AnimInterpolation::CubicSpline),
        other => Err(format!("bad interpolation {other:?}")),
    }
}
async fn enc_point3(p: &SemioPoint3) -> String {
    format!("[{},{},{}]", p.x, p.y, p.z)
}
async fn dec_point3(s: &str) -> Result<SemioPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}
async fn enc_quat(q: &SemioQuaternion) -> String {
    format!("[{},{},{},{}]", q.x, q.y, q.z, q.w)
}
async fn dec_quat(s: &str) -> Result<SemioQuaternion, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z, w] = parts.as_slice() else { return Err(format!("quat: expected 4 fields, got {}", parts.len())) };
    Ok(SemioQuaternion { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)?, w: parse_f64(w)? })
}
async fn enc_value(v: &AnimValue) -> String {
    match v {
        AnimValue::Scalar { value } => format!("S:{value}"),
        AnimValue::Vec3 { value } => format!("V:{}", enc_point3(value)),
        AnimValue::Quat { value } => format!("Q:{}", enc_quat(value)),
        AnimValue::Weights { values } => format!("W:{}", enc_list(values, |v: &f64| v.to_string())),
    }
}
async fn dec_value(s: &str) -> Result<AnimValue, String> {
    let (tag, rest) = s.split_once(':').ok_or_else(|| format!("value: bad shape {s:?}"))?;
    match tag {
        "S" => Ok(AnimValue::Scalar { value: parse_f64(rest)? }),
        "V" => Ok(AnimValue::Vec3 { value: dec_point3(rest)? }),
        "Q" => Ok(AnimValue::Quat { value: dec_quat(rest)? }),
        "W" => Ok(AnimValue::Weights { values: dec_list(rest, parse_f64)? }),
        other => Err(format!("value: unknown tag {other:?}")),
    }
}
async fn enc_keyframe(k: &AnimKeyframe) -> String {
    format!("[{},{}]", k.t, enc_value(&k.value))
}
async fn dec_keyframe(s: &str) -> Result<AnimKeyframe, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [t, value] = parts.as_slice() else { return Err(format!("keyframe: expected 2 fields, got {}", parts.len())) };
    Ok(AnimKeyframe { t: parse_f64(t)?, value: dec_value(value)? })
}
async fn enc_channel(c: &AnimChannel) -> String {
    format!("[{},{},{}]", enc_target(&c.target), enc_interpolation(c.interpolation), enc_list(&c.keyframes, enc_keyframe))
}
async fn dec_channel(s: &str) -> Result<AnimChannel, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [target, interp, kfs] = parts.as_slice() else { return Err(format!("channel: expected 3 fields, got {}", parts.len())) };
    Ok(AnimChannel { target: dec_target(target)?, interpolation: dec_interpolation(interp)?, keyframes: dec_list(kfs, dec_keyframe)? })
}
async fn enc_timeline(t: &AnimTimeline) -> String {
    format!("[{},{}]", encode_option(&t.name, |n: &String| enc_str(n)), enc_list(&t.channels, enc_channel))
}
async fn dec_timeline(s: &str) -> Result<AnimTimeline, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, channels] = parts.as_slice() else { return Err(format!("timeline: expected 2 fields, got {}", parts.len())) };
    Ok(AnimTimeline { name: decode_option(name, dec_str)?, channels: dec_list(channels, dec_channel)? })
}

/// 📄️ The real structured text body: two lines — `schema=<hex>`, `timelines=[...]` — matching the
/// grammar's `document = artifact-mark schema-line timelines-line`. Newlines are pure lexer trivia
/// in the shared dialect, so this is genuinely recognizable by `dsl::Recognizer`, not merely
/// readable.
async fn print_animation_snapshot_body(s: &SemioAnimationSnapshot) -> String {
    format!("schema={}\ntimelines=[{}]", enc_str(&s.schema), s.timelines.iter().map(enc_timeline).collect::<Vec<_>>().join(","))
}
async fn parse_animation_snapshot_body(body: &str) -> Result<SemioAnimationSnapshot, String> {
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
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, the same helpers every semio wave's upgraded `ArtifactPack` reuses)
/// backing the real `ArtifactPack` below — replaces the old `serde_json::to_vec`-in-envelope
/// shortcut.
async fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
async fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
async fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
async fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
async fn write_point3(out: &mut Vec<u8>, p: &SemioPoint3) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
    out.extend_from_slice(&p.z.to_le_bytes());
}
async fn read_point3(reader: &mut store::ByteReader<'_>) -> Result<SemioPoint3, String> {
    Ok(SemioPoint3 { x: reader.read_f64_le().map_err(|e| e.to_string())?, y: reader.read_f64_le().map_err(|e| e.to_string())?, z: reader.read_f64_le().map_err(|e| e.to_string())? })
}
async fn write_quat(out: &mut Vec<u8>, q: &SemioQuaternion) {
    out.extend_from_slice(&q.x.to_le_bytes());
    out.extend_from_slice(&q.y.to_le_bytes());
    out.extend_from_slice(&q.z.to_le_bytes());
    out.extend_from_slice(&q.w.to_le_bytes());
}
async fn read_quat(reader: &mut store::ByteReader<'_>) -> Result<SemioQuaternion, String> {
    Ok(SemioQuaternion { x: reader.read_f64_le().map_err(|e| e.to_string())?, y: reader.read_f64_le().map_err(|e| e.to_string())?, z: reader.read_f64_le().map_err(|e| e.to_string())?, w: reader.read_f64_le().map_err(|e| e.to_string())? })
}
async fn write_f64_vec(out: &mut Vec<u8>, v: &[f64]) {
    store::pack_rt::write_varint_u64(out, v.len() as u64);
    for x in v {
        out.extend_from_slice(&x.to_le_bytes());
    }
}
async fn read_f64_vec(reader: &mut store::ByteReader<'_>) -> Result<Vec<f64>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut v = Vec::with_capacity(n as usize);
    for _ in 0..n {
        v.push(reader.read_f64_le().map_err(|e| e.to_string())?);
    }
    Ok(v)
}

/// 🏷️ `AnimTargetProperty` variant tags — 0=Translation, 1=Rotation, 2=Scale, 3=Weights, 4=Custom.
async fn write_property(out: &mut Vec<u8>, p: &AnimTargetProperty) {
    match p {
        AnimTargetProperty::Translation => out.push(0),
        AnimTargetProperty::Rotation => out.push(1),
        AnimTargetProperty::Scale => out.push(2),
        AnimTargetProperty::Weights => out.push(3),
        AnimTargetProperty::Custom { name } => {
            out.push(4);
            write_str_lp(out, name);
        }
    }
}
async fn read_property(reader: &mut store::ByteReader<'_>) -> Result<AnimTargetProperty, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(AnimTargetProperty::Translation),
        1 => Ok(AnimTargetProperty::Rotation),
        2 => Ok(AnimTargetProperty::Scale),
        3 => Ok(AnimTargetProperty::Weights),
        4 => Ok(AnimTargetProperty::Custom { name: read_str_lp(reader)? }),
        other => Err(format!("property: unknown binary tag {other}")),
    }
}
/// 🏷️ `AnimInterpolation` variant tags — 0=Linear, 1=Step, 2=CubicSpline.
async fn write_interpolation(out: &mut Vec<u8>, i: AnimInterpolation) {
    out.push(match i {
        AnimInterpolation::Linear => 0,
        AnimInterpolation::Step => 1,
        AnimInterpolation::CubicSpline => 2,
    });
}
async fn read_interpolation(reader: &mut store::ByteReader<'_>) -> Result<AnimInterpolation, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(AnimInterpolation::Linear),
        1 => Ok(AnimInterpolation::Step),
        2 => Ok(AnimInterpolation::CubicSpline),
        other => Err(format!("interpolation: unknown binary tag {other}")),
    }
}
/// 🏷️ `AnimValue` variant tags — 0=Scalar, 1=Vec3, 2=Quat, 3=Weights.
async fn write_value(out: &mut Vec<u8>, v: &AnimValue) {
    match v {
        AnimValue::Scalar { value } => {
            out.push(0);
            out.extend_from_slice(&value.to_le_bytes());
        }
        AnimValue::Vec3 { value } => {
            out.push(1);
            write_point3(out, value);
        }
        AnimValue::Quat { value } => {
            out.push(2);
            write_quat(out, value);
        }
        AnimValue::Weights { values } => {
            out.push(3);
            write_f64_vec(out, values);
        }
    }
}
async fn read_value(reader: &mut store::ByteReader<'_>) -> Result<AnimValue, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(AnimValue::Scalar { value: reader.read_f64_le().map_err(|e| e.to_string())? }),
        1 => Ok(AnimValue::Vec3 { value: read_point3(reader)? }),
        2 => Ok(AnimValue::Quat { value: read_quat(reader)? }),
        3 => Ok(AnimValue::Weights { values: read_f64_vec(reader)? }),
        other => Err(format!("value: unknown binary tag {other}")),
    }
}
async fn write_target(out: &mut Vec<u8>, t: &AnimTarget) {
    write_str_lp(out, &t.node);
    write_property(out, &t.property);
}
async fn read_target(reader: &mut store::ByteReader<'_>) -> Result<AnimTarget, String> {
    Ok(AnimTarget { node: read_str_lp(reader)?, property: read_property(reader)? })
}
async fn write_keyframe(out: &mut Vec<u8>, k: &AnimKeyframe) {
    out.extend_from_slice(&k.t.to_le_bytes());
    write_value(out, &k.value);
}
async fn read_keyframe(reader: &mut store::ByteReader<'_>) -> Result<AnimKeyframe, String> {
    Ok(AnimKeyframe { t: reader.read_f64_le().map_err(|e| e.to_string())?, value: read_value(reader)? })
}
async fn write_channel(out: &mut Vec<u8>, c: &AnimChannel) {
    write_target(out, &c.target);
    write_interpolation(out, c.interpolation);
    store::pack_rt::write_varint_u64(out, c.keyframes.len() as u64);
    for k in &c.keyframes {
        write_keyframe(out, k);
    }
}
async fn read_channel(reader: &mut store::ByteReader<'_>) -> Result<AnimChannel, String> {
    let target = read_target(reader)?;
    let interpolation = read_interpolation(reader)?;
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut keyframes = Vec::with_capacity(n as usize);
    for _ in 0..n {
        keyframes.push(read_keyframe(reader)?);
    }
    Ok(AnimChannel { target, interpolation, keyframes })
}
async fn write_timeline(out: &mut Vec<u8>, t: &AnimTimeline) {
    match &t.name {
        Some(n) => {
            out.push(1);
            write_str_lp(out, n);
        }
        None => out.push(0),
    }
    store::pack_rt::write_varint_u64(out, t.channels.len() as u64);
    for c in &t.channels {
        write_channel(out, c);
    }
}
async fn read_timeline(reader: &mut store::ByteReader<'_>) -> Result<AnimTimeline, String> {
    let has_name = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let name = if has_name { Some(read_str_lp(reader)?) } else { None };
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut channels = Vec::with_capacity(n as usize);
    for _ in 0..n {
        channels.push(read_channel(reader)?);
    }
    Ok(AnimTimeline { name, channels })
}

async fn encode_animation_snapshot_binary(s: &SemioAnimationSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.timelines.len() as u64);
    for t in &s.timelines {
        write_timeline(&mut out, t);
    }
    out
}
async fn decode_animation_snapshot_binary(bytes: &[u8]) -> Result<SemioAnimationSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut timelines = Vec::with_capacity(n as usize);
    for _ in 0..n {
        timelines.push(read_timeline(&mut reader)?);
    }
    Ok(SemioAnimationSnapshot { schema, timelines })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 Real structured text/binary codecs (animation wave — off the old hex-dump-of-`serde_json`
/// shortcut, following the flow pilot's proven template). Wrapped in the repo-wide
/// `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioAnimationSnapshot {
    const EXTENSION: &'static str = "semio";
    async fn envelope_id() -> &'static str {
        STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_animation_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    async fn print_dsl(&self) -> String {
        let body = print_animation_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioAnimationSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_animation_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_animation_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.animation` document — one timeline exercising every `AnimValue`
/// variant (`Scalar`/`Vec3`/`Quat`/`Weights`) and every `AnimTargetProperty` kind (incl. `Custom`).
/// Single source of truth for `📚️examples/🚶️walk/🖼️assets/🗣️example.dsl.semio`/
/// `🎒️example.pack.semio` and for the conformance-law tests in `🎹️composer/🦀️component.rs`.
#[cfg(test)]
pub(crate) async fn demo_animation_snapshot() -> SemioAnimationSnapshot {
    SemioAnimationSnapshot {
        schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(),
        timelines: vec![AnimTimeline {
            name: Some("walk".into()),
            channels: vec![
                AnimChannel {
                    target: AnimTarget { node: "hip".into(), property: AnimTargetProperty::Translation },
                    interpolation: AnimInterpolation::Linear,
                    keyframes: vec![AnimKeyframe { t: 0.0, value: AnimValue::Vec3 { value: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 } } }, AnimKeyframe { t: 1.0, value: AnimValue::Vec3 { value: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } } }],
                },
                AnimChannel {
                    target: AnimTarget { node: "spine".into(), property: AnimTargetProperty::Rotation },
                    interpolation: AnimInterpolation::CubicSpline,
                    keyframes: vec![AnimKeyframe { t: 0.5, value: AnimValue::Quat { value: SemioQuaternion::default() } }],
                },
                AnimChannel {
                    target: AnimTarget { node: "face".into(), property: AnimTargetProperty::Weights },
                    interpolation: AnimInterpolation::Step,
                    keyframes: vec![AnimKeyframe { t: 0.0, value: AnimValue::Weights { values: vec![0.0, 1.0, 0.5] } }],
                },
                AnimChannel {
                    target: AnimTarget { node: "rig".into(), property: AnimTargetProperty::Custom { name: "opacity".into() } },
                    interpolation: AnimInterpolation::Linear,
                    keyframes: vec![AnimKeyframe { t: 0.0, value: AnimValue::Scalar { value: 1.0 } }],
                },
            ],
        }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ codec_retention_law: decode(encode(x)) == x through both the pack (binary) and dsl
    /// (text) envelopes, on a snapshot exercising every `AnimValue` variant and both `AnimTarget`
    /// property kinds (incl. `Custom`).
    #[test]
    async fn codec_retention_law() {
        let snap = demo_animation_snapshot();
        let bytes = <SemioAnimationSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioAnimationSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);

        let text = <SemioAnimationSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back_text = <SemioAnimationSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back_text);
    }

    #[test]
    async fn default_snapshot_round_trips() {
        let snap = SemioAnimationSnapshot::default();
        let bytes = <SemioAnimationSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioAnimationSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }
}
//#endregion 🔖️Tests
