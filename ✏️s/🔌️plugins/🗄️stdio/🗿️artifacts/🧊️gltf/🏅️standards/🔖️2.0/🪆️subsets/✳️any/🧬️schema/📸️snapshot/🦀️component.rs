//! 🧬️ GltfSnapshot schema — the FULLY TYPED glTF 2.0 JSON document model (ticket
//! ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, F4: kills `document:
//! serde_json::Value` outright, replaced by [`GltfDocument`] — one struct per spec object type,
//! `extras`/`extensions` typed via this module's own [`GltfJson`] value enum, never
//! `serde_json::Value`). Byte/container codecs (base64, accessor decode, `.gltf`/`.glb`
//! parse+serialize) live in `🏅️standards/🔖️2.0/⚙️engine` — this file only owns the persisted shape,
//! its serde wire mapping (every struct derives real `Serialize`/`Deserialize` that round-trips
//! through genuine glTF JSON text via `serde_json::{from_str,to_vec}::<GltfDocument>`, not a
//! bespoke encoding), and the `ArtifactDsl`/`ArtifactPack` envelope glue.

use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::STDIO_GLTF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

//#region 🔖️SourceForm
/// 🧵 Which wire dialect a snapshot was last parsed from -- drives [`serialize_gltf_document`]'s
/// choice of whether a no-`uri` buffer needs re-embedding as a data uri (a `.glb`-sourced buffer
/// serialized back out as plain `.gltf` JSON text has no BIN chunk to lean on).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum GltfSourceForm {
    #[default]
    Json,
    Glb,
}
//#endregion 🔖️SourceForm

//#region 🔖️GltfJson
/// 🌳 A JSON value living inside a glTF `extras`/`extensions` slot -- this artifact's OWN local
/// value enum (structurally similar to `stdio.json`'s `JsonValue`, deliberately NOT imported: the
/// recipe treats "own `JsonValue`-shaped type per artifact" as separate concerns even where the
/// shape coincides). `Object` is a `Vec<(String, GltfJson)>`, never a map, so member insertion
/// order survives decode->encode exactly. `Number` widens to `f64` (glTF extras/extensions are
/// free-form JSON with no `bufferView`-precision requirement, unlike `stdio.json`'s own
/// arbitrary-precision lexeme retention).
#[derive(Clone, Debug, PartialEq)]
pub enum GltfJson {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<GltfJson>),
    Object(Vec<(String, GltfJson)>),
}

impl Default for GltfJson {
    fn default() -> Self {
        GltfJson::Null
    }
}

impl Serialize for GltfJson {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            GltfJson::Null => serializer.serialize_unit(),
            GltfJson::Bool(b) => serializer.serialize_bool(*b),
            GltfJson::Number(n) => serializer.serialize_f64(*n),
            GltfJson::String(s) => serializer.serialize_str(s),
            GltfJson::Array(items) => {
                let mut seq = serializer.serialize_seq(Some(items.len()))?;
                for item in items {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            GltfJson::Object(members) => {
                let mut map = serializer.serialize_map(Some(members.len()))?;
                for (k, v) in members {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}

struct GltfJsonVisitor;
impl<'de> Visitor<'de> for GltfJsonVisitor {
    type Value = GltfJson;
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a JSON value (glTF extras/extensions)")
    }
    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(GltfJson::Null)
    }
    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(GltfJson::Null)
    }
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
        Ok(GltfJson::Bool(v))
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
        Ok(GltfJson::Number(v as f64))
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
        Ok(GltfJson::Number(v as f64))
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
        Ok(GltfJson::Number(v))
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
        Ok(GltfJson::String(v.to_string()))
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
        Ok(GltfJson::String(v))
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut items = Vec::new();
        while let Some(v) = seq.next_element()? {
            items.push(v);
        }
        Ok(GltfJson::Array(items))
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut members = Vec::new();
        while let Some((k, v)) = map.next_entry::<String, GltfJson>()? {
            members.push((k, v));
        }
        Ok(GltfJson::Object(members))
    }
}

impl<'de> Deserialize<'de> for GltfJson {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(GltfJsonVisitor)
    }
}

/// 🌉️ Hand-written `ToValue`/`FromValue` — structurally identical to the `Serialize`/`Deserialize`
/// impls above (unit -> `Null`, same scalar/array/object mapping), additive alongside them: the
/// serde pair stays load-bearing for genuine `.gltf`/`.glb` file bytes (`🚪️io/🦀️component.rs`'s
/// `serde_json`-based codec, out of this batch's scope), this pair is what a `Mutation`/
/// `MutationDiff` payload carrying `extras`/`extensions` needs.
impl dsl::ToValue for GltfJson {
    fn to_value(&self) -> dsl::DslValue {
        match self {
            GltfJson::Null => dsl::DslValue::Null,
            GltfJson::Bool(b) => dsl::DslValue::Bool(*b),
            GltfJson::Number(n) => dsl::DslValue::Number(*n),
            GltfJson::String(s) => dsl::DslValue::String(s.clone()),
            GltfJson::Array(items) => dsl::DslValue::Array(items.iter().map(dsl::ToValue::to_value).collect()),
            GltfJson::Object(members) => dsl::DslValue::object(members.iter().map(|(k, v)| (k.clone(), dsl::ToValue::to_value(v)))),
        }
    }
}
impl dsl::FromValue for GltfJson {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        Ok(match value {
            dsl::DslValue::Null => GltfJson::Null,
            dsl::DslValue::Bool(b) => GltfJson::Bool(b),
            dsl::DslValue::Number(n) => GltfJson::Number(n),
            dsl::DslValue::String(s) => GltfJson::String(s),
            dsl::DslValue::Array(items) => GltfJson::Array(items.into_iter().map(dsl::FromValue::from_value).collect::<Result<Vec<_>, _>>()?),
            dsl::DslValue::Object(members) => GltfJson::Object(members.into_iter().map(|(k, v)| Ok((k, dsl::FromValue::from_value(v)?))).collect::<Result<Vec<_>, dsl::ValueError>>()?),
        })
    }
}
//#endregion 🔖️GltfJson

//#region 🔖️OrderedAttrMap
/// 🧩️ `primitive.attributes` (and morph-target maps) are a JSON OBJECT of `semantic -> accessor
/// index`, but `Vec<(String, usize)>` is the right in-memory shape (attribute count is always
/// small and order-preserving matters for stable diffs) -- this hand-rolled serde adapter makes
/// the pair-vec serialize/deserialize AS a JSON object instead of Serde's default array-of-tuples.
mod ordered_attr_map {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn serialize<S: Serializer>(attrs: &[(String, usize)], serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(attrs.len()))?;
        for (k, v) in attrs {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }

    struct AttrVisitor;
    impl<'de> Visitor<'de> for AttrVisitor {
        type Value = Vec<(String, usize)>;
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a JSON object mapping attribute semantic to accessor index")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut out = Vec::new();
            while let Some((k, v)) = map.next_entry::<String, usize>()? {
                out.push((k, v));
            }
            Ok(out)
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<(String, usize)>, D::Error> {
        deserializer.deserialize_map(AttrVisitor)
    }
}

/// 🧩️ `ToValue`/`FromValue` analogs of `ordered_attr_map::{serialize,deserialize}` above — same
/// object-shaped (never array-of-tuples) wire mapping, referenced via `#[value(serialize_with =
/// "ordered_attr_map_to_value", deserialize_with = "ordered_attr_map_from_value")]` on
/// [`GltfPrimitive::attributes`] and by [`GltfMorphTarget`]'s hand-written impls above.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ordered_attr_map_to_value(attrs: &[(String, usize)]) -> dsl::DslValue {
    dsl::DslValue::object(attrs.iter().map(|(k, v)| (k.clone(), dsl::ToValue::to_value(v))))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ordered_attr_map_from_value(value: dsl::DslValue) -> Result<Vec<(String, usize)>, dsl::ValueError> {
    let entries = dsl::DslValue::into_object(value)?;
    entries.into_iter().map(|(k, v)| Ok((k, dsl::FromValue::from_value(v)?))).collect()
}
//#endregion 🔖️OrderedAttrMap

//#region 🔖️SpecDefaults
/// 📐️ glTF 2.0 spec-mandated scalar defaults, applied on read when the JSON key is absent and
/// omitted on write when the in-memory value still equals the default -- a documented (and, for
/// this fixture set, byte-exact) normal form: a field is either genuinely absent or explicitly
/// non-default in every real document this codec has seen.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn default_one_f64() -> f64 {
    1.0
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_one_f64(v: &f64) -> bool {
    *v == 1.0
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn default_zero_u64() -> u64 {
    0
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_zero_u64(v: &u64) -> bool {
    *v == 0
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn default_zero_usize() -> usize {
    0
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_zero_usize(v: &usize) -> bool {
    *v == 0
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn default_wrap() -> u64 {
    10497
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_default_wrap(v: &u64) -> bool {
    *v == 10497
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn default_alpha_cutoff() -> f64 {
    0.5
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_default_alpha_cutoff(v: &f64) -> bool {
    *v == 0.5
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn default_vec3_zero() -> [f64; 3] {
    [0.0, 0.0, 0.0]
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_vec3_zero(v: &[f64; 3]) -> bool {
    *v == [0.0, 0.0, 0.0]
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn default_vec4_one() -> [f64; 4] {
    [1.0, 1.0, 1.0, 1.0]
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_vec4_one(v: &[f64; 4]) -> bool {
    *v == [1.0, 1.0, 1.0, 1.0]
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_false(v: &bool) -> bool {
    !*v
}
//#endregion 🔖️SpecDefaults

//#region 🔖️Asset
/// 📛 `asset` (§3.9) — the one universally mandatory glTF object; `version` is the one mandatory
/// field on it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfAsset {
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minVersion")]
    #[value(default, skip_serializing_if = "Option::is_none", rename = "minVersion")]
    pub min_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}

impl Default for GltfAsset {
    fn default() -> Self {
        Self { version: "2.0".into(), generator: None, copyright: None, min_version: None, extensions: None, extras: None }
    }
}
//#endregion 🔖️Asset

//#region 🔖️Scene
/// 🎬 `scenes[i]` (§5.26).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfScene {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}
//#endregion 🔖️Scene

//#region 🔖️Node
/// 🧍 `nodes[i]` (§5.25). `matrix`/`translation`+`rotation`+`scale` are mutually exclusive per
/// spec but modeled as independently-optional fields rather than an enum -- a real document that
/// (incorrectly) carries both should still round-trip losslessly, and this shape keeps the diff
/// symmetric with every other nullable field instead of needing a `Replace`-only whole-transform
/// diff.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfNode {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub skin: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<[f64; 16]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub translation: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<[f64; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub weights: Vec<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}
//#endregion 🔖️Node

//#region 🔖️Mesh
/// 🎭 One `meshes[i].primitives[j].targets[k]` morph-target attribute map (§5.19.4).
/// 🩹 Hand-written `ToValue`/`FromValue` (not `#[derive(..., transparent)]`): `#[value(transparent)]`
/// forwards straight to the raw `Vec<(String, usize)>` field's OWN `ToValue`/`FromValue` (a
/// 2-element-array-per-entry encoding), bypassing the `ordered_attr_map` object-shaped encoding
/// this type actually needs — same wire shape [`GltfPrimitive::attributes`] uses below.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GltfMorphTarget(#[serde(with = "ordered_attr_map")] pub Vec<(String, usize)>);

impl dsl::ToValue for GltfMorphTarget {
    fn to_value(&self) -> dsl::DslValue {
        ordered_attr_map_to_value(&self.0)
    }
}
impl dsl::FromValue for GltfMorphTarget {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        Ok(GltfMorphTarget(ordered_attr_map_from_value(value)?))
    }
}

/// 🔺 `meshes[i].primitives[j]` (§5.19.4).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfPrimitive {
    #[serde(default, with = "ordered_attr_map")]
    #[value(default, serialize_with = "ordered_attr_map_to_value", deserialize_with = "ordered_attr_map_from_value")]
    pub attributes: Vec<(String, usize)>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub indices: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<GltfMorphTarget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}

/// 🕸️ `meshes[i]` (§5.19).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfMesh {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub primitives: Vec<GltfPrimitive>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub weights: Vec<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}
//#endregion 🔖️Mesh

//#region 🔖️Accessor
/// 🧩️ `accessors[i].sparse.indices` (§5.1.3).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfSparseIndices {
    pub buffer_view: usize,
    #[serde(default = "default_zero_usize", skip_serializing_if = "is_zero_usize")]
    #[value(default = "default_zero_usize", skip_serializing_if = "is_zero_usize")]
    pub byte_offset: usize,
    pub component_type: GltfComponentType,
}

/// 🧩️ `accessors[i].sparse.values` (§5.1.3).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfSparseValues {
    pub buffer_view: usize,
    #[serde(default = "default_zero_usize", skip_serializing_if = "is_zero_usize")]
    #[value(default = "default_zero_usize", skip_serializing_if = "is_zero_usize")]
    pub byte_offset: usize,
}

/// 🧩️ `accessors[i].sparse` (§5.1.3) -- sparse-storage substitution over a (possibly absent, then
/// zero-filled) dense base.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfSparseAccessor {
    pub count: usize,
    pub indices: GltfSparseIndices,
    pub values: GltfSparseValues,
}

/// 🔢️ `accessors[i]` (§5.1).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfAccessor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub buffer_view: Option<usize>,
    #[serde(default = "default_zero_usize", skip_serializing_if = "is_zero_usize")]
    #[value(default = "default_zero_usize", skip_serializing_if = "is_zero_usize")]
    pub byte_offset: usize,
    pub component_type: GltfComponentType,
    #[serde(default, skip_serializing_if = "is_false")]
    #[value(default, skip_serializing_if = "is_false")]
    pub normalized: bool,
    pub count: usize,
    #[serde(rename = "type")]
    #[value(rename = "type")]
    pub kind: GltfAccessorType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<Vec<f64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<Vec<f64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub sparse: Option<GltfSparseAccessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}
//#endregion 🔖️Accessor

//#region 🔖️BufferView
/// 🪟️ `bufferViews[i]` (§5.7).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfBufferView {
    pub buffer: usize,
    #[serde(default = "default_zero_usize", skip_serializing_if = "is_zero_usize")]
    #[value(default = "default_zero_usize", skip_serializing_if = "is_zero_usize")]
    pub byte_offset: usize,
    pub byte_length: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub byte_stride: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}
//#endregion 🔖️BufferView

//#region 🔖️Buffer
/// 📦️ `buffers[i]` (§5.6) -- JSON-level metadata only; the resolved raw bytes live index-aligned
/// in `GltfSnapshot::buffers` (the legitimate bytes-payload exception).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfBuffer {
    pub byte_length: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}
//#endregion 🔖️Buffer

//#region 🔖️Material
/// 🖼️ A texture reference (§5.20 `textureInfo`) shared by `baseColorTexture` /
/// `metallicRoughnessTexture` / `emissiveTexture`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfTextureInfo {
    pub index: usize,
    #[serde(default = "default_zero_u64", skip_serializing_if = "is_zero_u64", rename = "texCoord")]
    #[value(default = "default_zero_u64", skip_serializing_if = "is_zero_u64", rename = "texCoord")]
    pub tex_coord: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}

/// 🖼️ `material.normalTexture` (§5.21) -- adds `scale` (default 1).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfNormalTextureInfo {
    pub index: usize,
    #[serde(default = "default_zero_u64", skip_serializing_if = "is_zero_u64", rename = "texCoord")]
    #[value(default = "default_zero_u64", skip_serializing_if = "is_zero_u64", rename = "texCoord")]
    pub tex_coord: u64,
    #[serde(default = "default_one_f64", skip_serializing_if = "is_one_f64")]
    #[value(default = "default_one_f64", skip_serializing_if = "is_one_f64")]
    pub scale: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}

/// 🖼️ `material.occlusionTexture` (§5.22) -- adds `strength` (default 1).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfOcclusionTextureInfo {
    pub index: usize,
    #[serde(default = "default_zero_u64", skip_serializing_if = "is_zero_u64", rename = "texCoord")]
    #[value(default = "default_zero_u64", skip_serializing_if = "is_zero_u64", rename = "texCoord")]
    pub tex_coord: u64,
    #[serde(default = "default_one_f64", skip_serializing_if = "is_one_f64")]
    #[value(default = "default_one_f64", skip_serializing_if = "is_one_f64")]
    pub strength: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}

/// 🎨️ `material.pbrMetallicRoughness` (§5.23).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfPbrMetallicRoughness {
    #[serde(default = "default_vec4_one", skip_serializing_if = "is_vec4_one")]
    #[value(default = "default_vec4_one", skip_serializing_if = "is_vec4_one")]
    pub base_color_factor: [f64; 4],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub base_color_texture: Option<GltfTextureInfo>,
    #[serde(default = "default_one_f64", skip_serializing_if = "is_one_f64")]
    #[value(default = "default_one_f64", skip_serializing_if = "is_one_f64")]
    pub metallic_factor: f64,
    #[serde(default = "default_one_f64", skip_serializing_if = "is_one_f64")]
    #[value(default = "default_one_f64", skip_serializing_if = "is_one_f64")]
    pub roughness_factor: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub metallic_roughness_texture: Option<GltfTextureInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}

impl Default for GltfPbrMetallicRoughness {
    fn default() -> Self {
        Self { base_color_factor: default_vec4_one(), base_color_texture: None, metallic_factor: 1.0, roughness_factor: 1.0, metallic_roughness_texture: None, extensions: None, extras: None }
    }
}

/// 🔀️ `material.alphaMode` (§5.23.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default, value_derive::ToValue, value_derive::FromValue)]
pub enum GltfAlphaMode {
    #[default]
    #[serde(rename = "OPAQUE")]
    #[value(rename = "OPAQUE")]
    Opaque,
    #[serde(rename = "MASK")]
    #[value(rename = "MASK")]
    Mask,
    #[serde(rename = "BLEND")]
    #[value(rename = "BLEND")]
    Blend,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_opaque(v: &GltfAlphaMode) -> bool {
    matches!(v, GltfAlphaMode::Opaque)
}

/// 🎨️ `materials[i]` (§5.23).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfMaterial {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pbr_metallic_roughness: Option<GltfPbrMetallicRoughness>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub normal_texture: Option<GltfNormalTextureInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub occlusion_texture: Option<GltfOcclusionTextureInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub emissive_texture: Option<GltfTextureInfo>,
    #[serde(default = "default_vec3_zero", skip_serializing_if = "is_vec3_zero")]
    #[value(default = "default_vec3_zero", skip_serializing_if = "is_vec3_zero")]
    pub emissive_factor: [f64; 3],
    #[serde(default, skip_serializing_if = "is_opaque", rename = "alphaMode")]
    #[value(default, skip_serializing_if = "is_opaque", rename = "alphaMode")]
    pub alpha_mode: GltfAlphaMode,
    #[serde(default = "default_alpha_cutoff", skip_serializing_if = "is_default_alpha_cutoff")]
    #[value(default = "default_alpha_cutoff", skip_serializing_if = "is_default_alpha_cutoff")]
    pub alpha_cutoff: f64,
    #[serde(default, skip_serializing_if = "is_false")]
    #[value(default, skip_serializing_if = "is_false")]
    pub double_sided: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}

impl Default for GltfMaterial {
    fn default() -> Self {
        Self {
            name: None,
            pbr_metallic_roughness: None,
            normal_texture: None,
            occlusion_texture: None,
            emissive_texture: None,
            emissive_factor: default_vec3_zero(),
            alpha_mode: GltfAlphaMode::Opaque,
            alpha_cutoff: 0.5,
            double_sided: false,
            extensions: None,
            extras: None,
        }
    }
}
//#endregion 🔖️Material

//#region 🔖️TextureImageSampler
/// 🧵️ `textures[i]` (§5.30).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfTexture {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub sampler: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}

/// 🖼️ `images[i]` (§5.15) -- image bytes are addressed EITHER by `uri` (external/data-uri) OR by
/// `bufferView` (embedded), never both.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfImage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mimeType")]
    #[value(default, skip_serializing_if = "Option::is_none", rename = "mimeType")]
    pub mime_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub buffer_view: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}

/// 🧲️ `samplers[i]` (§5.27) -- `wrapS`/`wrapT` both default to `10497` (`REPEAT`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfSampler {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mag_filter: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min_filter: Option<u64>,
    #[serde(default = "default_wrap", skip_serializing_if = "is_default_wrap")]
    #[value(default = "default_wrap", skip_serializing_if = "is_default_wrap")]
    pub wrap_s: u64,
    #[serde(default = "default_wrap", skip_serializing_if = "is_default_wrap")]
    #[value(default = "default_wrap", skip_serializing_if = "is_default_wrap")]
    pub wrap_t: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}

impl Default for GltfSampler {
    fn default() -> Self {
        Self { mag_filter: None, min_filter: None, wrap_s: 10497, wrap_t: 10497, name: None, extensions: None, extras: None }
    }
}
//#endregion 🔖️TextureImageSampler

//#region 🔖️Skin
/// 🦴️ `skins[i]` (§5.28).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfSkin {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub inverse_bind_matrices: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub skeleton: Option<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub joints: Vec<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}
//#endregion 🔖️Skin

//#region 🔖️Animation
/// 🎞️ `animations[i].channels[j].target.path` (§5.5.2) -- the 4 spec-defined animatable
/// properties.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
pub enum GltfAnimationPath {
    #[serde(rename = "translation")]
    #[value(rename = "translation")]
    Translation,
    #[serde(rename = "rotation")]
    #[value(rename = "rotation")]
    Rotation,
    #[serde(rename = "scale")]
    #[value(rename = "scale")]
    Scale,
    #[serde(rename = "weights")]
    #[value(rename = "weights")]
    Weights,
}

/// 🎯️ `animations[i].channels[j].target` (§5.5.2).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfAnimationChannelTarget {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<usize>,
    pub path: GltfAnimationPath,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}

/// 🔗️ `animations[i].channels[j]` (§5.5.1).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfAnimationChannel {
    pub sampler: usize,
    pub target: GltfAnimationChannelTarget,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}

/// 📈️ `animations[i].samplers[j].interpolation` (§5.5.3), default `LINEAR`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default, value_derive::ToValue, value_derive::FromValue)]
pub enum GltfInterpolation {
    #[default]
    #[serde(rename = "LINEAR")]
    #[value(rename = "LINEAR")]
    Linear,
    #[serde(rename = "STEP")]
    #[value(rename = "STEP")]
    Step,
    #[serde(rename = "CUBICSPLINE")]
    #[value(rename = "CUBICSPLINE")]
    CubicSpline,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_linear(v: &GltfInterpolation) -> bool {
    matches!(v, GltfInterpolation::Linear)
}

/// 📈️ `animations[i].samplers[j]` (§5.5.3).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfAnimationSampler {
    pub input: usize,
    #[serde(default, skip_serializing_if = "is_linear")]
    #[value(default, skip_serializing_if = "is_linear")]
    pub interpolation: GltfInterpolation,
    pub output: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}

/// 🎬️ `animations[i]` (§5.5).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfAnimation {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub channels: Vec<GltfAnimationChannel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub samplers: Vec<GltfAnimationSampler>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}
//#endregion 🔖️Animation

//#region 🔖️Camera
/// 📷️ `cameras[i].orthographic` (§5.10.1).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfOrthographic {
    pub xmag: f64,
    pub ymag: f64,
    pub zfar: f64,
    pub znear: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}

/// 📷️ `cameras[i].perspective` (§5.10.2).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfPerspective {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "aspectRatio")]
    #[value(default, skip_serializing_if = "Option::is_none", rename = "aspectRatio")]
    pub aspect_ratio: Option<f64>,
    pub yfov: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub zfar: Option<f64>,
    pub znear: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}

/// 🔀️ A camera is EITHER `perspective` OR `orthographic` (§5.10) -- modeled as a tagged union on
/// the sibling `type` string field.
#[derive(Clone, Debug, PartialEq)]
pub enum GltfCameraProjection {
    Perspective(GltfPerspective),
    Orthographic(GltfOrthographic),
}

impl Serialize for GltfCameraProjection {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(tag = "type", rename_all = "lowercase")]
        enum Wire<'a> {
            Perspective { perspective: &'a GltfPerspective },
            Orthographic { orthographic: &'a GltfOrthographic },
        }
        match self {
            Self::Perspective(perspective) => Wire::Perspective { perspective }.serialize(serializer),
            Self::Orthographic(orthographic) => Wire::Orthographic { orthographic }.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for GltfCameraProjection {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(tag = "type", rename_all = "lowercase")]
        enum Wire {
            Perspective { perspective: GltfPerspective },
            Orthographic { orthographic: GltfOrthographic },
        }
        Ok(match Wire::deserialize(deserializer)? {
            Wire::Perspective { perspective } => Self::Perspective(perspective),
            Wire::Orthographic { orthographic } => Self::Orthographic(orthographic),
        })
    }
}

/// 📷️ `cameras[i]` (§5.10).
#[derive(Clone, Debug, PartialEq)]
pub struct GltfCamera {
    pub projection: GltfCameraProjection,
    pub name: Option<String>,
    pub extensions: Option<GltfJson>,
    pub extras: Option<GltfJson>,
}

impl Serialize for GltfCamera {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Wire<'a> {
            #[serde(rename = "type")]
            kind: &'static str,
            #[serde(skip_serializing_if = "Option::is_none")]
            perspective: Option<&'a GltfPerspective>,
            #[serde(skip_serializing_if = "Option::is_none")]
            orthographic: Option<&'a GltfOrthographic>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<&'a String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            extensions: Option<&'a GltfJson>,
            #[serde(skip_serializing_if = "Option::is_none")]
            extras: Option<&'a GltfJson>,
        }
        let (kind, perspective, orthographic) = match &self.projection {
            GltfCameraProjection::Perspective(p) => ("perspective", Some(p), None),
            GltfCameraProjection::Orthographic(o) => ("orthographic", None, Some(o)),
        };
        Wire { kind, perspective, orthographic, name: self.name.as_ref(), extensions: self.extensions.as_ref(), extras: self.extras.as_ref() }.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for GltfCamera {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Wire {
            #[serde(rename = "type")]
            kind: String,
            #[serde(default)]
            perspective: Option<GltfPerspective>,
            #[serde(default)]
            orthographic: Option<GltfOrthographic>,
            #[serde(default)]
            name: Option<String>,
            #[serde(default)]
            extensions: Option<GltfJson>,
            #[serde(default)]
            extras: Option<GltfJson>,
        }
        let wire = Wire::deserialize(deserializer)?;
        let projection = match wire.kind.as_str() {
            "perspective" => GltfCameraProjection::Perspective(wire.perspective.ok_or_else(|| serde::de::Error::missing_field("perspective"))?),
            "orthographic" => GltfCameraProjection::Orthographic(wire.orthographic.ok_or_else(|| serde::de::Error::missing_field("orthographic"))?),
            other => return Err(serde::de::Error::custom(format!("camera.type must be 'perspective' or 'orthographic', got {other:?}"))),
        };
        Ok(GltfCamera { projection, name: wire.name, extensions: wire.extensions, extras: wire.extras })
    }
}

/// 🌉️ Hand-written `ToValue`/`FromValue` for the tagged-union `type`+sibling-key wire shape —
/// mirrors the two hand-rolled `Serialize`/`Deserialize` impls above exactly (`{"type":
/// "perspective", "perspective": {...}}`), which neither `#[value(tag = "…")]` (fixed content
/// key, not one named after the tag value) nor `#[value(tag = "…", content = "…")]` (same
/// mismatch) can express generically.
impl dsl::ToValue for GltfCameraProjection {
    fn to_value(&self) -> dsl::DslValue {
        match self {
            Self::Perspective(perspective) => dsl::DslValue::object([("type".to_string(), dsl::DslValue::String("perspective".to_string())), ("perspective".to_string(), dsl::ToValue::to_value(perspective))]),
            Self::Orthographic(orthographic) => dsl::DslValue::object([("type".to_string(), dsl::DslValue::String("orthographic".to_string())), ("orthographic".to_string(), dsl::ToValue::to_value(orthographic))]),
        }
    }
}
impl dsl::FromValue for GltfCameraProjection {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let entries = dsl::DslValue::into_object(value)?;
        let kind = entries.iter().find(|(k, _)| k == "type").map(|(_, v)| v.clone()).ok_or_else(|| dsl::ValueError::new("missing field `type`"))?;
        let kind = match kind {
            dsl::DslValue::String(s) => s,
            other => return Err(dsl::ValueError::new(format!("expected a string, found {other:?}"))),
        };
        match kind.as_str() {
            "perspective" => {
                let payload = entries.iter().find(|(k, _)| k == "perspective").map(|(_, v)| v.clone()).ok_or_else(|| dsl::ValueError::new("missing field `perspective`"))?;
                Ok(Self::Perspective(dsl::FromValue::from_value(payload)?))
            }
            "orthographic" => {
                let payload = entries.iter().find(|(k, _)| k == "orthographic").map(|(_, v)| v.clone()).ok_or_else(|| dsl::ValueError::new("missing field `orthographic`"))?;
                Ok(Self::Orthographic(dsl::FromValue::from_value(payload)?))
            }
            other => Err(dsl::ValueError::new(format!("camera.type must be 'perspective' or 'orthographic', got {other:?}"))),
        }
    }
}

/// 🌉️ Hand-written `ToValue`/`FromValue` for `GltfCamera` — flattens `projection`'s own
/// `type`+sibling-key entries alongside `name`/`extensions`/`extras`, mirroring the hand-rolled
/// `Serialize`/`Deserialize` impls above.
impl dsl::ToValue for GltfCamera {
    fn to_value(&self) -> dsl::DslValue {
        let mut entries = match dsl::ToValue::to_value(&self.projection) {
            dsl::DslValue::Object(entries) => entries,
            other => vec![("projection".to_string(), other)],
        };
        if let Some(name) = &self.name {
            entries.push(("name".to_string(), dsl::ToValue::to_value(name)));
        }
        if let Some(extensions) = &self.extensions {
            entries.push(("extensions".to_string(), dsl::ToValue::to_value(extensions)));
        }
        if let Some(extras) = &self.extras {
            entries.push(("extras".to_string(), dsl::ToValue::to_value(extras)));
        }
        dsl::DslValue::Object(entries)
    }
}
impl dsl::FromValue for GltfCamera {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let entries = dsl::DslValue::into_object(value)?;
        let projection = dsl::FromValue::from_value(dsl::DslValue::Object(entries.clone()))?;
        let name = match entries.iter().find(|(k, _)| k == "name") {
            Some((_, v)) => dsl::FromValue::from_value(v.clone())?,
            None => None,
        };
        let extensions = match entries.iter().find(|(k, _)| k == "extensions") {
            Some((_, v)) => dsl::FromValue::from_value(v.clone())?,
            None => None,
        };
        let extras = match entries.iter().find(|(k, _)| k == "extras") {
            Some((_, v)) => dsl::FromValue::from_value(v.clone())?,
            None => None,
        };
        Ok(GltfCamera { projection, name, extensions, extras })
    }
}
//#endregion 🔖️Camera

//#region 🔖️Document
/// 🌍 The full glTF 2.0 JSON document (§5), fully typed -- one field per spec top-level array/
/// object, `extras`/`extensions` typed via [`GltfJson`], never `serde_json::Value`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GltfDocument {
    pub asset: GltfAsset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scene: Option<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub scenes: Vec<GltfScene>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<GltfNode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub meshes: Vec<GltfMesh>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub accessors: Vec<GltfAccessor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub buffer_views: Vec<GltfBufferView>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub buffers: Vec<GltfBuffer>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub materials: Vec<GltfMaterial>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub textures: Vec<GltfTexture>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<GltfImage>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub samplers: Vec<GltfSampler>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub skins: Vec<GltfSkin>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub animations: Vec<GltfAnimation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub cameras: Vec<GltfCamera>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "extensionsUsed")]
    #[value(default, skip_serializing_if = "Vec::is_empty", rename = "extensionsUsed")]
    pub extensions_used: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "extensionsRequired")]
    #[value(default, skip_serializing_if = "Vec::is_empty", rename = "extensionsRequired")]
    pub extensions_required: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<GltfJson>,
}

impl Default for GltfDocument {
    fn default() -> Self {
        Self {
            asset: GltfAsset::default(),
            scene: None,
            scenes: Vec::new(),
            nodes: Vec::new(),
            meshes: Vec::new(),
            accessors: Vec::new(),
            buffer_views: Vec::new(),
            buffers: Vec::new(),
            materials: Vec::new(),
            textures: Vec::new(),
            images: Vec::new(),
            samplers: Vec::new(),
            skins: Vec::new(),
            animations: Vec::new(),
            cameras: Vec::new(),
            extensions_used: Vec::new(),
            extensions_required: Vec::new(),
            extensions: None,
            extras: None,
        }
    }
}
//#endregion 🔖️Document

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.gltf` snapshot: the fully typed [`GltfDocument`] plus `buffers`: the
/// resolved raw bytes for each `document.buffers[i]` (index-aligned), since a `.glb`-sourced
/// buffer may have no `uri` at all and its bytes must live somewhere other than the JSON.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gltf")]
pub struct GltfSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    #[value(default)]
    pub document: GltfDocument,
    #[state(artifact)]
    #[serde(default)]
    #[value(default)]
    pub buffers: Vec<Vec<u8>>,
    #[state(artifact)]
    #[serde(default)]
    #[value(default)]
    pub source_form: GltfSourceForm,
}

impl Default for GltfSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document: GltfDocument::default(), buffers: Vec::new(), source_form: GltfSourceForm::Json }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for GltfSnapshot {
    const EXTENSION: &'static str = "gltf";
    fn envelope_id() -> &'static str {
        "stdio.gltf"
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        crate::artifacts::gltf::engine::parse_gltf_document(body.trim().as_bytes()).map_err(|e| store::TextError::new(format!("gltf json: {e}"), dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body_bytes = crate::artifacts::gltf::engine::serialize_gltf_document(self);
        let body = String::from_utf8(body_bytes).unwrap_or_else(|_| "{}".into());
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for GltfSnapshot {
    /// 📦️ P2-FG3: routes through the REAL `.glb` binary container (`encode_glb`), not the prior
    /// F6-era `serialize_gltf_document` JSON-as-"binary" shortcut — glTF's own genuine binary
    /// serialization, matching `../💾️binary/📡️component.protocol.semio`'s real chunk-container
    /// framing exactly (this is what that protocol file's `walk_protocol` is built to walk; per
    /// the recipe's own instruction, the protocol description must match what `encode_pack`
    /// actually produces). A raw `.glb` file byte-for-byte (unwrapped) still decodes directly via
    /// `crate::artifacts::gltf::engine::decode_glb` (🧐️analyzer's own fast path) — this impl only
    /// adds the SEMIO envelope around the SAME real container, it does not invent a second shape.
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::gltf::engine::encode_glb(self).map_err(store::PackError::Schema)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        crate::artifacts::gltf::engine::decode_glb(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
