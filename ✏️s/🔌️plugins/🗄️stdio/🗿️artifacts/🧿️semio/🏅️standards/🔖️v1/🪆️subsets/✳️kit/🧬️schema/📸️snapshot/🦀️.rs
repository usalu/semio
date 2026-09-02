//! 🧬️ SemioKitSnapshot — semio's own type/design domain: TYPES (with representation references)
//! and DESIGNS (pieces and connections). SECOND COMPOSITE subset (UNIFIED-COMPOSABLE-ARTIFACT-
//! SYSTEM, W2c) — carries both CHILD slots (`objects`/`models`/`properties`, owned) AND a LINK
//! slot (`representations`, independent-lifecycle). Absorbs the duplicated `kit.catalog` artifact
//! kind puzzle/three-block apps currently declare separately (that dissolution — repointing those
//! apps' `AppSchema::artifact_kind()` registrations at this subset — is a later wave's concern,
//! same "later wave" scoping `✳️text`'s report used for its own absorbed `LocalizedText` dissolve).
//!
//! Composes `object`/`model` (owned pieces/example instances) and `value` (shared property set),
//! per `📌️important.md`'s suggested shape. `representations` is the one LINK slot: a TYPE's visual
//! representation is often reused across many kits/catalogs (a shared library item), so it is
//! referenced, never owned — each `ArtifactLink.role` carries the owning `SemioKitType.id` (the
//! join key between the flat link pool and the type that displays it; a type may have zero, one,
//! or many representations sharing its id as `role`).

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioTransform;
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_SEMIOKIT_DOCUMENT_SCHEMA: &str = "stdio.semio.kit";
//#endregion 🔖️Ids

//#region 🔖️Type
/// 🏷️ One TYPE in the kit's catalog — a name/category, its representations living in the sibling
/// `representations` LINK pool (joined by `role == id`, see module doc comment). Id-keyed (no
/// positional meaning — `add-type`/`remove-type`/`rename-type` all address by `id`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, Default)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct SemioKitType {
    pub id: String,
    pub name: String,
    pub category: String,
}
//#endregion 🔖️Type

//#region 🔖️Design
/// 📐️ One PIECE inside a design: an instance of a TYPE (`type_id`, joins `SemioKitType.id`) at a
/// local `transform`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, Default)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct SemioKitPiece {
    pub id: String,
    pub type_id: String,
    pub transform: SemioTransform,
}

/// 🔌️ One CONNECTION between two pieces' named ports.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, Default)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct SemioKitConnection {
    pub id: String,
    pub connecting_piece_id: String,
    pub connecting_port: String,
    pub connected_piece_id: String,
    pub connected_port: String,
}

/// 📋️ One DESIGN — a named arrangement of pieces and their connections. Id-keyed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, Default)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct SemioKitDesign {
    pub id: String,
    pub name: String,
    pub pieces: Vec<SemioKitPiece>,
    pub connections: Vec<SemioKitConnection>,
}
//#endregion 🔖️Design

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.kit")]
pub struct SemioKitSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub types: Vec<SemioKitType>,
    #[state(artifact)]
    #[serde(default)]
    pub designs: Vec<SemioKitDesign>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.object")]
    #[serde(default)]
    pub objects: Vec<store::ArtifactChild<SemioObjectSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.model")]
    #[serde(default)]
    pub models: Vec<store::ArtifactChild<SemioModelSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.value")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<store::ArtifactChild<SemioValueSnapshot>>,
    #[state(artifact)]
    #[link_slot(roles("representation"))]
    #[serde(default)]
    pub representations: Vec<store::ArtifactLink>,
}

impl Default for SemioKitSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOKIT_DOCUMENT_SCHEMA.into(), types: Vec::new(), designs: Vec::new(), objects: Vec::new(), models: Vec::new(), properties: None, representations: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived: `objects`/`models`/`properties` are `store::ArtifactChild<S>`
/// composed-artifact CHILD handles and `representations` is a `Vec<store::ArtifactLink>` LINK
/// slot — both bridged per-field through `to_dsl_value`/`from_dsl_value` (`🌱️value/🔀️serde`)
/// rather than widening the derive macro. Same pattern as `✳️object`'s `SemioObjectSnapshot` and
/// the fan-out playbook's `PlaybookArtifact` reference.
impl dsl::ToValue for SemioKitSnapshot {
    fn to_value(&self) -> dsl::DslValue {
        dsl::DslValue::object([
            ("schema".to_string(), dsl::ToValue::to_value(&self.schema)),
            ("types".to_string(), dsl::ToValue::to_value(&self.types)),
            ("designs".to_string(), dsl::ToValue::to_value(&self.designs)),
            ("objects".to_string(), dsl::to_dsl_value(&self.objects).expect("ArtifactChild serializes")),
            ("models".to_string(), dsl::to_dsl_value(&self.models).expect("ArtifactChild serializes")),
            ("properties".to_string(), dsl::to_dsl_value(&self.properties).expect("ArtifactChild serializes")),
            ("representations".to_string(), dsl::to_dsl_value(&self.representations).expect("ArtifactLink serializes")),
        ])
    }
}
impl dsl::FromValue for SemioKitSnapshot {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let entries = dsl::DslValue::into_object(value)?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| dsl::ValueError::new(format!("missing field `{key}`")));
        Ok(Self {
            schema: dsl::FromValue::from_value(field("schema")?)?,
            types: dsl::FromValue::from_value(field("types")?)?,
            designs: dsl::FromValue::from_value(field("designs")?)?,
            objects: dsl::from_dsl_value(field("objects")?).map_err(dsl::ValueError::new)?,
            models: dsl::from_dsl_value(field("models")?).map_err(dsl::ValueError::new)?,
            properties: dsl::from_dsl_value(field("properties")?).map_err(dsl::ValueError::new)?,
            representations: dsl::from_dsl_value(field("representations")?).map_err(dsl::ValueError::new)?,
        })
    }
}
//#endregion 🔖️ValueCodec

//#region 🔖️CodecPrimitives
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

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_child<S>(c: &store::ArtifactChild<S>) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_child_list<S>(list: &[store::ArtifactChild<S>]) -> String {
    format!("[{}]", list.iter().map(enc_child).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_child_list<S>(s: &str) -> Result<Vec<store::ArtifactChild<S>>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_child).collect()
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

/// 📌️ `LinkPin`: `h` (Head) | `c,<hex id>` (Checkpoint) | `s,<hex hash>,<size>,<hex media_type>` (Snapshot).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_pin(p: &store::LinkPin) -> String {
    match p {
        store::LinkPin::Head => "[h]".to_string(),
        store::LinkPin::Checkpoint { id } => format!("[c,{}]", enc_str(id)),
        store::LinkPin::Snapshot { blob } => format!("[s,{},{},{}]", enc_str(&blob.hash), blob.size, enc_str(&blob.media_type)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_pin(s: &str) -> Result<store::LinkPin, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    match parts.as_slice() {
        [tag] if *tag == "h" => Ok(store::LinkPin::Head),
        [tag, id] if *tag == "c" => Ok(store::LinkPin::Checkpoint { id: dec_str(id)? }),
        [tag, hash, size, media_type] if *tag == "s" => Ok(store::LinkPin::Snapshot { blob: store::BlobRef { hash: dec_str(hash)?, size: size.trim().parse().map_err(|e: std::num::ParseIntError| e.to_string())?, media_type: dec_str(media_type)? } }),
        _ => Err(format!("link pin: unrecognized {s:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_link(l: &store::ArtifactLink) -> String {
    format!("[{},{},{}]", enc_ref(&l.target), enc_pin(&l.pin), enc_str(&l.role))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_link(s: &str) -> Result<store::ArtifactLink, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [target, pin, role] = parts.as_slice() else { return Err(format!("link: expected 3 fields, got {}", parts.len())) };
    Ok(store::ArtifactLink { target: dec_ref(target)?, pin: dec_pin(pin)?, role: dec_str(role)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_link_list(list: &[store::ArtifactLink]) -> String {
    format!("[{}]", list.iter().map(enc_link).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_link_list(s: &str) -> Result<Vec<store::ArtifactLink>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_link).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_transform(t: &SemioTransform) -> String {
    format!("[{},{},{},{},{},{},{},{},{},{}]", t.translation.x, t.translation.y, t.translation.z, t.rotation.x, t.rotation.y, t.rotation.z, t.rotation.w, t.scale.x, t.scale.y, t.scale.z,)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_transform(s: &str) -> Result<SemioTransform, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [tx, ty, tz, rx, ry, rz, rw, sx, sy, sz] = parts.as_slice() else {
        return Err(format!("transform: expected 10 fields, got {}", parts.len()));
    };
    let f = |s: &str| s.trim().parse::<f64>().map_err(|e| e.to_string());
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioQuaternion};
    Ok(SemioTransform { translation: SemioPoint3 { x: f(tx)?, y: f(ty)?, z: f(tz)? }, rotation: SemioQuaternion { x: f(rx)?, y: f(ry)?, z: f(rz)?, w: f(rw)? }, scale: SemioPoint3 { x: f(sx)?, y: f(sy)?, z: f(sz)? } })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_type(t: &SemioKitType) -> String {
    format!("[{},{},{}]", enc_str(&t.id), enc_str(&t.name), enc_str(&t.category))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_type(s: &str) -> Result<SemioKitType, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, name, category] = parts.as_slice() else { return Err(format!("type: expected 3 fields, got {}", parts.len())) };
    Ok(SemioKitType { id: dec_str(id)?, name: dec_str(name)?, category: dec_str(category)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_type_list(list: &[SemioKitType]) -> String {
    format!("[{}]", list.iter().map(enc_type).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_type_list(s: &str) -> Result<Vec<SemioKitType>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_type).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_piece(p: &SemioKitPiece) -> String {
    format!("[{},{},{}]", enc_str(&p.id), enc_str(&p.type_id), enc_transform(&p.transform))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_piece(s: &str) -> Result<SemioKitPiece, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, type_id, transform] = parts.as_slice() else { return Err(format!("piece: expected 3 fields, got {}", parts.len())) };
    Ok(SemioKitPiece { id: dec_str(id)?, type_id: dec_str(type_id)?, transform: dec_transform(transform)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_connection(c: &SemioKitConnection) -> String {
    format!("[{},{},{},{},{}]", enc_str(&c.id), enc_str(&c.connecting_piece_id), enc_str(&c.connecting_port), enc_str(&c.connected_piece_id), enc_str(&c.connected_port))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_connection(s: &str) -> Result<SemioKitConnection, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, cp_id, cp_port, cd_id, cd_port] = parts.as_slice() else { return Err(format!("connection: expected 5 fields, got {}", parts.len())) };
    Ok(SemioKitConnection { id: dec_str(id)?, connecting_piece_id: dec_str(cp_id)?, connecting_port: dec_str(cp_port)?, connected_piece_id: dec_str(cd_id)?, connected_port: dec_str(cd_port)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_design(d: &SemioKitDesign) -> String {
    let pieces = d.pieces.iter().map(enc_piece).collect::<Vec<_>>().join(",");
    let connections = d.connections.iter().map(enc_connection).collect::<Vec<_>>().join(",");
    format!("[{},{},[{}],[{}]]", enc_str(&d.id), enc_str(&d.name), pieces, connections)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_design(s: &str) -> Result<SemioKitDesign, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, name, pieces, connections] = parts.as_slice() else { return Err(format!("design: expected 4 fields, got {}", parts.len())) };
    let pieces = split_top_level(strip_brackets(pieces)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_piece).collect::<Result<Vec<_>, String>>()?;
    let connections = split_top_level(strip_brackets(connections)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_connection).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioKitDesign { id: dec_str(id)?, name: dec_str(name)?, pieces, connections })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_design_list(list: &[SemioKitDesign]) -> String {
    format!("[{}]", list.iter().map(enc_design).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_design_list(s: &str) -> Result<Vec<SemioKitDesign>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_design).collect()
}
//#endregion 🔖️CodecPrimitives

//#region 🔖️TextPrimitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_kit_snapshot_body(s: &SemioKitSnapshot) -> String {
    format!(
        "schema={}\ntypes={}\ndesigns={}\nobjects={}\nmodels={}\nproperties={}\nrepresentations={}",
        enc_str(&s.schema),
        enc_type_list(&s.types),
        enc_design_list(&s.designs),
        enc_child_list(&s.objects),
        enc_child_list(&s.models),
        enc_child_opt(&s.properties),
        enc_link_list(&s.representations),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_kit_snapshot_body(body: &str) -> Result<SemioKitSnapshot, String> {
    let mut schema = None;
    let mut types = Vec::new();
    let mut designs = Vec::new();
    let mut objects = Vec::new();
    let mut models = Vec::new();
    let mut properties = None;
    let mut representations = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("types=") {
            types = dec_type_list(rest)?;
        } else if let Some(rest) = line.strip_prefix("designs=") {
            designs = dec_design_list(rest)?;
        } else if let Some(rest) = line.strip_prefix("objects=") {
            objects = dec_child_list(rest)?;
        } else if let Some(rest) = line.strip_prefix("models=") {
            models = dec_child_list(rest)?;
        } else if let Some(rest) = line.strip_prefix("properties=") {
            properties = dec_child_opt(rest)?;
        } else if let Some(rest) = line.strip_prefix("representations=") {
            representations = dec_link_list(rest)?;
        } else {
            return Err(format!("semio kit snapshot: unknown line {line:?}"));
        }
    }
    Ok(SemioKitSnapshot { schema: schema.ok_or_else(|| "semio kit snapshot: missing schema line".to_string())?, types, designs, objects, models, properties, representations })
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
pub(crate) fn write_child_list<S>(out: &mut Vec<u8>, list: &[store::ArtifactChild<S>]) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for c in list {
        write_child(out, c);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_child_list<S>(reader: &mut store::ByteReader<'_>) -> Result<Vec<store::ArtifactChild<S>>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| read_child(reader)).collect()
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
pub(crate) fn write_pin(out: &mut Vec<u8>, p: &store::LinkPin) {
    match p {
        store::LinkPin::Head => out.push(0),
        store::LinkPin::Checkpoint { id } => {
            out.push(1);
            write_str_lp(out, id);
        }
        store::LinkPin::Snapshot { blob } => {
            out.push(2);
            write_str_lp(out, &blob.hash);
            store::pack_rt::write_varint_u64(out, blob.size);
            write_str_lp(out, &blob.media_type);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_pin(reader: &mut store::ByteReader<'_>) -> Result<store::LinkPin, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(store::LinkPin::Head),
        1 => Ok(store::LinkPin::Checkpoint { id: read_str_lp(reader)? }),
        2 => {
            let hash = read_str_lp(reader)?;
            let size = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let media_type = read_str_lp(reader)?;
            Ok(store::LinkPin::Snapshot { blob: store::BlobRef { hash, size, media_type } })
        }
        other => Err(format!("unsupported link pin tag {other}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_link(out: &mut Vec<u8>, l: &store::ArtifactLink) {
    write_ref(out, &l.target);
    write_pin(out, &l.pin);
    write_str_lp(out, &l.role);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_link(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactLink, String> {
    let target = read_ref(reader)?;
    let pin = read_pin(reader)?;
    let role = read_str_lp(reader)?;
    Ok(store::ArtifactLink { target, pin, role })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_link_list(out: &mut Vec<u8>, list: &[store::ArtifactLink]) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for l in list {
        write_link(out, l);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_link_list(reader: &mut store::ByteReader<'_>) -> Result<Vec<store::ArtifactLink>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| read_link(reader)).collect()
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
pub(crate) fn write_type(out: &mut Vec<u8>, t: &SemioKitType) {
    write_str_lp(out, &t.id);
    write_str_lp(out, &t.name);
    write_str_lp(out, &t.category);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_type(reader: &mut store::ByteReader<'_>) -> Result<SemioKitType, String> {
    Ok(SemioKitType { id: read_str_lp(reader)?, name: read_str_lp(reader)?, category: read_str_lp(reader)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_type_list(out: &mut Vec<u8>, list: &[SemioKitType]) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for t in list {
        write_type(out, t);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_type_list(reader: &mut store::ByteReader<'_>) -> Result<Vec<SemioKitType>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| read_type(reader)).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_piece(out: &mut Vec<u8>, p: &SemioKitPiece) {
    write_str_lp(out, &p.id);
    write_str_lp(out, &p.type_id);
    write_transform(out, &p.transform);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_piece(reader: &mut store::ByteReader<'_>) -> Result<SemioKitPiece, String> {
    Ok(SemioKitPiece { id: read_str_lp(reader)?, type_id: read_str_lp(reader)?, transform: read_transform(reader)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_connection(out: &mut Vec<u8>, c: &SemioKitConnection) {
    write_str_lp(out, &c.id);
    write_str_lp(out, &c.connecting_piece_id);
    write_str_lp(out, &c.connecting_port);
    write_str_lp(out, &c.connected_piece_id);
    write_str_lp(out, &c.connected_port);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_connection(reader: &mut store::ByteReader<'_>) -> Result<SemioKitConnection, String> {
    Ok(SemioKitConnection { id: read_str_lp(reader)?, connecting_piece_id: read_str_lp(reader)?, connecting_port: read_str_lp(reader)?, connected_piece_id: read_str_lp(reader)?, connected_port: read_str_lp(reader)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_design(out: &mut Vec<u8>, d: &SemioKitDesign) {
    write_str_lp(out, &d.id);
    write_str_lp(out, &d.name);
    store::pack_rt::write_varint_u64(out, d.pieces.len() as u64);
    for p in &d.pieces {
        write_piece(out, p);
    }
    store::pack_rt::write_varint_u64(out, d.connections.len() as u64);
    for c in &d.connections {
        write_connection(out, c);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_design(reader: &mut store::ByteReader<'_>) -> Result<SemioKitDesign, String> {
    let id = read_str_lp(reader)?;
    let name = read_str_lp(reader)?;
    let piece_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let pieces = (0..piece_count).map(|_| read_piece(reader)).collect::<Result<Vec<_>, String>>()?;
    let connection_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let connections = (0..connection_count).map(|_| read_connection(reader)).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioKitDesign { id, name, pieces, connections })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_design_list(out: &mut Vec<u8>, list: &[SemioKitDesign]) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for d in list {
        write_design(out, d);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_design_list(reader: &mut store::ByteReader<'_>) -> Result<Vec<SemioKitDesign>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| read_design(reader)).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_kit_snapshot_binary(s: &SemioKitSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &s.schema);
    write_type_list(&mut out, &s.types);
    write_design_list(&mut out, &s.designs);
    write_child_list(&mut out, &s.objects);
    write_child_list(&mut out, &s.models);
    write_child_opt(&mut out, &s.properties);
    write_link_list(&mut out, &s.representations);
    out
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_kit_snapshot_binary(bytes: &[u8]) -> Result<SemioKitSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let types = read_type_list(&mut reader)?;
    let designs = read_design_list(&mut reader)?;
    let objects = read_child_list(&mut reader)?;
    let models = read_child_list(&mut reader)?;
    let properties = read_child_opt(&mut reader)?;
    let representations = read_link_list(&mut reader)?;
    Ok(SemioKitSnapshot { schema, types, designs, objects, models, properties, representations })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for SemioKitSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOKIT_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_kit_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_kit_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioKitSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_kit_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_kit_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️JsonBridge
/// 📥️ Decodes this subset's own `#[value(rename_all = "camelCase")]`-shaped JSON projection — the
/// exact shape the committed `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/
/// 🔣️.json` specification-vector fixtures carry — into a real `SemioKitSnapshot`, using
/// the snapshot's own hand-written `ToValue`/`FromValue` (§ValueCodec above). A thin
/// `pack::from_json_str` wrapper (first-party, over `ToValue`/`DslValue`) so external Rust callers
/// that cannot name this crate's private `store` extern-crate item (e.g. `mutate-semio-kit`'s test
/// adapter — see its own doc comment for why) can still decode a snapshot from committed fixture
/// text without hand-transcribing one field at a time, which is both laborious and a place for the
/// transcription to silently drift away from the fixture it claims to mirror.
pub fn decode_kit_snapshot_json(text: &str) -> Result<SemioKitSnapshot, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

/// 📤️ The `pack::to_json_string` inverse of `decode_kit_snapshot_json` — same rationale.
pub fn encode_kit_snapshot_json(snapshot: &SemioKitSnapshot) -> String {
    pack::to_json_string(snapshot)
}
//#endregion 🔖️JsonBridge

//#region 🔖️Wire
/// 📝️ Parses `s.stdio.semio.kit` DSL text into a [`SemioKitSnapshot`] — a named pass-through of this snapshot's own
/// `store::ArtifactDsl` impl above, whose trait and error type are both unnameable outside this
/// crate, so `mutate-semio-kit`'s `identity-round-trip` scenario reaches the real committed
/// artifact (`../../📚️examples/🪑️furniture/🖼️assets/🗣️.dsl.semio`) through this instead.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_kit_dsl(text: &str) -> Result<SemioKitSnapshot, String> {
    <SemioKitSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📝️ Renders a [`SemioKitSnapshot`] back as `s.stdio.semio.kit` DSL text — the inverse of
/// [`parse_semio_kit_dsl`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_kit_dsl(snapshot: &SemioKitSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Encodes a [`SemioKitSnapshot`] as a semio pack envelope — the binary twin of the DSL text, produced by a
/// SEPARATE codec, which is what makes the two committed encodings of one document able to
/// contradict each other.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_kit_pack(snapshot: &SemioKitSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}

/// 📦️ Decodes a semio pack envelope into a [`SemioKitSnapshot`] — the inverse of
/// [`encode_semio_kit_pack`], reading `../../📚️examples/🪑️furniture/🖼️assets/🎒️example.pack.semio`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_kit_pack(bytes: &[u8]) -> Result<SemioKitSnapshot, String> {
    <SemioKitSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
//#endregion 🔖️Wire

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.kit` — one type ("chair") with one representation link, one design
/// ("living-room") with two pieces and one connection, one owned object child, one owned model
/// child, and a properties child. Exercises every field shape at least once.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_kit_snapshot() -> SemioKitSnapshot {
    let dialect = |subset: &str| store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
    SemioKitSnapshot {
        schema: STDIO_SEMIOKIT_DOCUMENT_SCHEMA.into(),
        types: vec![SemioKitType { id: "chair".into(), name: "Chair".into(), category: "furniture".into() }],
        designs: vec![SemioKitDesign {
            id: "living-room".into(),
            name: "Living Room".into(),
            pieces: vec![SemioKitPiece { id: "piece-1".into(), type_id: "chair".into(), transform: SemioTransform::identity() }, SemioKitPiece { id: "piece-2".into(), type_id: "chair".into(), transform: SemioTransform::identity() }],
            connections: vec![SemioKitConnection { id: "conn-1".into(), connecting_piece_id: "piece-1".into(), connecting_port: "left".into(), connected_piece_id: "piece-2".into(), connected_port: "right".into() }],
        }],
        objects: vec![store::ArtifactChild::new("obj-01".into(), store::os_io::ArtifactRef { artifact_id: "chair-instance".into(), dialect: dialect("object") })],
        models: vec![store::ArtifactChild::new("model-01".into(), store::os_io::ArtifactRef { artifact_id: "chair-bim".into(), dialect: dialect("model") })],
        properties: Some(store::ArtifactChild::new("props-01".into(), store::os_io::ArtifactRef { artifact_id: "kit-props".into(), dialect: dialect("value") })),
        representations: vec![store::ArtifactLink { target: store::os_io::ArtifactRef { artifact_id: "chair-repr".into(), dialect: dialect("mesh") }, pin: store::LinkPin::Head, role: "chair".into() }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn json_pack_round_trips() {
        let snap = SemioKitSnapshot::default();
        let bytes = <SemioKitSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioKitSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_text_round_trips() {
        let snap = SemioKitSnapshot::default();
        let text = <SemioKitSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioKitSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let snap = demo_kit_snapshot();
        let bytes = <SemioKitSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioKitSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
        let text = <SemioKitSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back_text = <SemioKitSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back_text);
    }

    /// 🧪️ A parent snapshot NEVER embeds owned-child content — only handles. Links are references
    /// by design (never owned), so this proves both composition primitives stay handle-only.
    #[semio_framework_async_macros::async_test]
    async fn parent_snapshot_stores_only_handles_never_child_content() {
        let snap = demo_kit_snapshot();
        let text = <SemioKitSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        assert!(text.contains(&enc_str("obj-01")));
        assert!(!text.to_lowercase().contains("primitives") && !text.to_lowercase().contains("elements"), "must never embed object/model field names — only the handle");
    }
}
//#endregion 🔖️Tests
