//! 🧬️ SemioObjectSnapshot — one *spatial thing*: a placement/transform, its geometry, and its
//! property sets. FIRST COMPOSITE subset (UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, W2c) — carries real
//! `store::ArtifactChild<S>` CHILD slots, unlike every leaf subset authored before it. ⚠️ The name
//! is reused from the old value-tree `object` (renamed to `✳️value` earlier in this ticket); this
//! is a brand-new spatial subset, unrelated in shape.
//!
//! Composes `brep`/`mesh` (geometry, at most one representation of each kind) and `value`
//! (property sets) — all three as OWNED children: the child is its own document with its own
//! history, this snapshot holds only the two-string handle (`child_id`/`target`), never embedded
//! content (per `📌️important.md`'s composition section).

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioTransform;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA: &str = "stdio.semio.object";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
/// 🧊️ One placed spatial thing. `transform` is the object's own placement (world-relative, no
/// parent-chain here — composition of OBJECTS into a scene graph is a later wave's concern, e.g.
/// `kit`'s designs); `brep`/`mesh` are alternative geometry REPRESENTATIONS (a real-world object
/// may carry a precise b-rep AND a tessellated preview mesh at once, hence both, each optional and
/// independently owned); `properties` is one owned `value` tree for arbitrary property-set data
/// (materials, IFC property sets, custom metadata).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.object")]
pub struct SemioObjectSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub transform: SemioTransform,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.brep")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brep: Option<store::ArtifactChild<SemioBrepSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.mesh")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.value")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<store::ArtifactChild<SemioValueSnapshot>>,
}

impl Default for SemioObjectSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA.into(), transform: SemioTransform::identity(), brep: None, mesh: None, properties: None }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived: `brep`/`mesh`/`properties` are `store::ArtifactChild<S>`
/// composed-artifact handles, bridged per-field through `to_dsl_value`/`from_dsl_value`
/// (`🌱️value/🔀️serde`) rather than widening the derive macro — same pattern (and same reasoning)
/// as this subset's own `🧬️schema/🦀️component.rs` (`SemioObjectArtifact`) and the fan-out
/// playbook's `PlaybookArtifact` reference.
impl dsl::ToValue for SemioObjectSnapshot {
    fn to_value(&self) -> dsl::DslValue {
        dsl::DslValue::object([
            ("schema".to_string(), dsl::ToValue::to_value(&self.schema)),
            ("transform".to_string(), dsl::ToValue::to_value(&self.transform)),
            ("brep".to_string(), dsl::to_dsl_value(&self.brep).expect("ArtifactChild serializes")),
            ("mesh".to_string(), dsl::to_dsl_value(&self.mesh).expect("ArtifactChild serializes")),
            ("properties".to_string(), dsl::to_dsl_value(&self.properties).expect("ArtifactChild serializes")),
        ])
    }
}
impl dsl::FromValue for SemioObjectSnapshot {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let entries = dsl::DslValue::into_object(value)?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| dsl::ValueError::new(format!("missing field `{key}`")));
        Ok(Self {
            schema: dsl::FromValue::from_value(field("schema")?)?,
            transform: dsl::FromValue::from_value(field("transform")?)?,
            brep: dsl::from_dsl_value(field("brep")?).map_err(dsl::ValueError::new)?,
            mesh: dsl::from_dsl_value(field("mesh")?).map_err(dsl::ValueError::new)?,
            properties: dsl::from_dsl_value(field("properties")?).map_err(dsl::ValueError::new)?,
        })
    }
}
//#endregion 🔖️ValueCodec

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec — a handle is exactly two strings (`child_id`, the
/// target's `ArtifactRef` flattened via `to_uri()`), never the child's own content (composition
/// rule: "a child handle is two strings; that is all the parent stores").
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
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
pub(crate) fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}

/// 🪪️ `[<hex child_id>,<hex target-uri>]` — the two-string handle, real and complete.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_child<S>(c: &store::ArtifactChild<S>) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let parts = crate::artifacts::semio::standards::v1::subsets::any::schema::triples::split_top_level(crate::artifacts::semio::standards::v1::subsets::any::schema::triples::strip_brackets(s)?, ',');
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_child_opt<S>(c: &Option<store::ArtifactChild<S>>) -> String {
    match c {
        Some(c) => enc_child(c),
        None => "[]".to_string(),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_child_opt<S>(s: &str) -> Result<Option<store::ArtifactChild<S>>, String> {
    if s == "[]" {
        return Ok(None);
    }
    Ok(Some(dec_child(s)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_transform(t: &SemioTransform) -> String {
    format!("[{},{},{},{},{},{},{},{},{},{}]", t.translation.x, t.translation.y, t.translation.z, t.rotation.x, t.rotation.y, t.rotation.z, t.rotation.w, t.scale.x, t.scale.y, t.scale.z,)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_transform(s: &str) -> Result<SemioTransform, String> {
    let parts = crate::artifacts::semio::standards::v1::subsets::any::schema::triples::split_top_level(crate::artifacts::semio::standards::v1::subsets::any::schema::triples::strip_brackets(s)?, ',');
    let [tx, ty, tz, rx, ry, rz, rw, sx, sy, sz] = parts.as_slice() else {
        return Err(format!("transform: expected 10 fields, got {}", parts.len()));
    };
    let f = |s: &str| s.trim().parse::<f64>().map_err(|e| e.to_string());
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioQuaternion};
    Ok(SemioTransform { translation: SemioPoint3 { x: f(tx)?, y: f(ty)?, z: f(tz)? }, rotation: SemioQuaternion { x: f(rx)?, y: f(ry)?, z: f(rz)?, w: f(rw)? }, scale: SemioPoint3 { x: f(sx)?, y: f(sy)?, z: f(sz)? } })
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️TextPrimitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_object_snapshot_body(s: &SemioObjectSnapshot) -> String {
    format!("schema={}\ntransform={}\nbrep={}\nmesh={}\nproperties={}", enc_str(&s.schema), enc_transform(&s.transform), enc_child_opt(&s.brep), enc_child_opt(&s.mesh), enc_child_opt(&s.properties),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_object_snapshot_body(body: &str) -> Result<SemioObjectSnapshot, String> {
    let mut schema = None;
    let mut transform = None;
    let mut brep = None;
    let mut mesh = None;
    let mut properties = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("transform=") {
            transform = Some(dec_transform(rest)?);
        } else if let Some(rest) = line.strip_prefix("brep=") {
            brep = dec_child_opt(rest)?;
        } else if let Some(rest) = line.strip_prefix("mesh=") {
            mesh = dec_child_opt(rest)?;
        } else if let Some(rest) = line.strip_prefix("properties=") {
            properties = dec_child_opt(rest)?;
        } else {
            return Err(format!("semio object snapshot: unknown line {line:?}"));
        }
    }
    Ok(SemioObjectSnapshot { schema: schema.ok_or_else(|| "semio object snapshot: missing schema line".to_string())?, transform: transform.ok_or_else(|| "semio object snapshot: missing transform line".to_string())?, brep, mesh, properties })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) {
    write_str_lp(out, &r.to_uri());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_child<S>(out: &mut Vec<u8>, c: &store::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_child_opt<S>(out: &mut Vec<u8>, c: &Option<store::ArtifactChild<S>>) {
    match c {
        Some(c) => {
            out.push(1);
            write_child(out, c);
        }
        None => out.push(0),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_child_opt<S>(reader: &mut store::ByteReader<'_>) -> Result<Option<store::ArtifactChild<S>>, String> {
    let presence = reader.read_u8().map_err(|e| e.to_string())?;
    if presence == 0 {
        Ok(None)
    } else {
        Ok(Some(read_child(reader)?))
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_transform(out: &mut Vec<u8>, t: &SemioTransform) {
    for v in [t.translation.x, t.translation.y, t.translation.z, t.rotation.x, t.rotation.y, t.rotation.z, t.rotation.w, t.scale.x, t.scale.y, t.scale.z] {
        out.extend_from_slice(&v.to_le_bytes());
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_transform(reader: &mut store::ByteReader<'_>) -> Result<SemioTransform, String> {
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioQuaternion};
    let mut next = || -> Result<f64, String> { Ok(f64::from_le_bytes(reader.read_bytes(8).map_err(|e| e.to_string())?.try_into().map_err(|_| "transform: short read".to_string())?)) };
    Ok(SemioTransform { translation: SemioPoint3 { x: next()?, y: next()?, z: next()? }, rotation: SemioQuaternion { x: next()?, y: next()?, z: next()?, w: next()? }, scale: SemioPoint3 { x: next()?, y: next()?, z: next()? } })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_object_snapshot_binary(s: &SemioObjectSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &s.schema);
    write_transform(&mut out, &s.transform);
    write_child_opt(&mut out, &s.brep);
    write_child_opt(&mut out, &s.mesh);
    write_child_opt(&mut out, &s.properties);
    out
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_object_snapshot_binary(bytes: &[u8]) -> Result<SemioObjectSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let transform = read_transform(&mut reader)?;
    let brep = read_child_opt(&mut reader)?;
    let mesh = read_child_opt(&mut reader)?;
    let properties = read_child_opt(&mut reader)?;
    Ok(SemioObjectSnapshot { schema, transform, brep, mesh, properties })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for SemioObjectSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_object_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_object_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioObjectSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_object_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_object_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge
/// 📤️ This subset's own `#[value(rename_all = "camelCase")]`-shaped structural JSON projection of
/// `s.stdio.semio.object` — the shape `mutate-semio-object` compares under `ordered-json-v1`,
/// derived from the snapshot type's own hand-written `ToValue` impl above (§ValueCodec) rather
/// than hand-written a second time in the adapter, where it could drift away from the type it
/// claims to project. This is the bridge that makes the CHILD slots reachable at all — `ToValue`
/// bridges each `brep`/`mesh`/`properties` field through `to_dsl_value` (real ArtifactChild data:
/// `child_id` + `target` URI, never embedded content).
/// A thin `pack::to_json_string` wrapper (first-party, over `ToValue`/`DslValue`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_object_snapshot_json(snapshot: &SemioObjectSnapshot) -> String {
    pack::to_json_string(snapshot)
}

/// 📥️ The `pack::from_json_str` inverse of [`encode_semio_object_snapshot_json`] — decodes the
/// committed `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️component.json`
/// specification vectors into real [`SemioObjectSnapshot`] values, so `mutate-semio-object`'s
/// adapter reads the committed fixture instead of re-declaring it as a Rust literal beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_object_snapshot_json(text: &str) -> Result<SemioObjectSnapshot, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}
//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Wire
/// 📝️ Parses `s.stdio.semio.object` DSL text into a [`SemioObjectSnapshot`] — a named pass-through of this snapshot's own
/// `store::ArtifactDsl` impl above, whose trait and error type are both unnameable outside this
/// crate, so `mutate-semio-object`'s `identity-round-trip` scenario reaches the real committed
/// artifact (`../../📚️examples/📦️crate/🖼️assets/🗣️example.dsl.semio`) through this instead.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_object_dsl(text: &str) -> Result<SemioObjectSnapshot, String> {
    <SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📝️ Renders a [`SemioObjectSnapshot`] back as `s.stdio.semio.object` DSL text — the inverse of
/// [`parse_semio_object_dsl`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_object_dsl(snapshot: &SemioObjectSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Encodes a [`SemioObjectSnapshot`] as a semio pack envelope — the binary twin of the DSL text, produced by a
/// SEPARATE codec, which is what makes the two committed encodings of one document able to
/// contradict each other.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_object_pack(snapshot: &SemioObjectSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}

/// 📦️ Decodes a semio pack envelope into a [`SemioObjectSnapshot`] — the inverse of
/// [`encode_semio_object_pack`], reading `../../📚️examples/📦️crate/🖼️assets/🎒️example.pack.semio`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_object_pack(bytes: &[u8]) -> Result<SemioObjectSnapshot, String> {
    <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
//#endregion 🔖️Wire

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.object` — a non-identity transform plus all three child handles
/// populated (real child_id/target pairs, never embedded content). Single source of truth for
/// `📚️examples/📦️crate/🖼️assets/…` and this facet's own conformance-law tests.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_object_snapshot() -> SemioObjectSnapshot {
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioQuaternion};
    let dialect = |subset: &str| store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
    SemioObjectSnapshot {
        schema: STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA.into(),
        transform: SemioTransform { translation: SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 }, rotation: SemioQuaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } },
        brep: Some(store::ArtifactChild::new("brep-01".into(), store::os_io::ArtifactRef { artifact_id: "crate-brep".into(), dialect: dialect("brep") })),
        mesh: Some(store::ArtifactChild::new("mesh-01".into(), store::os_io::ArtifactRef { artifact_id: "crate-mesh".into(), dialect: dialect("mesh") })),
        properties: Some(store::ArtifactChild::new("props-01".into(), store::os_io::ArtifactRef { artifact_id: "crate-props".into(), dialect: dialect("value") })),
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn json_pack_round_trips() {
        let snap = SemioObjectSnapshot::default();
        let bytes = <SemioObjectSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_text_round_trips() {
        let snap = SemioObjectSnapshot::default();
        let text = <SemioObjectSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    /// 🧪️ codec_retention_law on a fully-populated snapshot (all 3 child handles present, non-
    /// identity transform), not just the default.
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let snap = demo_object_snapshot();
        let bytes = <SemioObjectSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
        let text = <SemioObjectSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back_text = <SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back_text);
    }

    /// 🧪️ A parent snapshot NEVER embeds child content — only the handle's two strings. Proven by
    /// asserting the printed DSL contains the child's `child_id`/target URI but never a byte
    /// sequence that could only come from parsing the CHILD's own snapshot type.
    #[semio_framework_async_macros::async_test]
    async fn parent_snapshot_stores_only_child_handles_never_content() {
        let snap = demo_object_snapshot();
        let text = <SemioObjectSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        assert!(text.contains(&enc_str("brep-01")), "hex-encoded child_id must be present");
        assert!(!text.to_lowercase().contains("vertices") && !text.to_lowercase().contains("faces"), "must never embed brep/mesh field names — only the handle");
    }
}
//#endregion 🔖️Tests
