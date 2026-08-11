//! 🧬️ SemioModelSnapshot — complete per the master plan's `model` row: a flat, id-keyed spatial
//! hierarchy (site/building/storey/space, parent-pointer graph — mirrors ifc/4's
//! `IfcRelAggregates`/`IfcRelContainedInSpatialStructure` shape rather than a recursive tree type)
//! + elements (typed class enum, placement, a BY-ID `GeometryRef` into the sibling `brep`/`mesh`
//! subsets, and named property sets) + relations (typed kind enum, from/to id endpoints). Owned by
//! `model` (w1b-type-ownership.md): `SemioModelElement`, `GeometryRef`, plus this file's own
//! `SpatialNode`/`ModelRelation`/`ElementClass`/`PropertySet`/`Property`/`PsetValue`/
//! `RelationKind`/`SpatialKind`. `model` never inlines brep/mesh geometry data — `GeometryRef`
//! resolves by id into those subsets' own snapshots (spec-mandated cross-reuse, master plan
//! Architecture section); referential integrity of THAT cross-subset link is out of this
//! snapshot's own scope (it is not decodable from `model` alone), but every reference WITHIN this
//! subset's own collections (spatial parent pointers, element→spatial containment, relation
//! endpoints) is checked by the composer's `SemioModelValidator`.

use crate::artifacts::semio::standards::v1::engine::geometry::SemioTransform;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_SEMIOMODEL_DOCUMENT_SCHEMA: &str = "stdio.semio.model";
//#endregion 🔖️Ids

//#region 🔖️Spatial
/// 🏢️ ifc/4 spatial-structure levels this subset targets (`IfcSite`/`IfcBuilding`/
/// `IfcBuildingStorey`/`IfcSpace`) — the master plan's exact "(site/building/storey/space)" list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SpatialKind {
    #[default]
    Site,
    Building,
    Storey,
    Space,
}

/// 🌳️ One spatial-tree node. The tree itself is expressed as a flat id-keyed collection with a
/// `parent_id` pointer (not a recursive struct) — matches how ifc's own spatial containment is a
/// graph of `IfcRelAggregates` edges over flat entities, not a nested Rust type.
/// 🧪️ `Default` is a technical workaround, never a meaningful "empty node" -- a known
/// `serde_derive` limitation (`#[serde(default)]` on the shared `🧰️triples::NamedTripleDiff`'s
/// `added: Vec<T>` field spuriously infers `T: Default`, same root cause bcf's own diff module
/// documents) means every strong-entity type reachable through a `NamedTripleDiff<K,D,T>` needs
/// `Default` purely to satisfy that derive, not because any real code constructs a default one.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpatialNode {
    pub id: String,
    pub kind: SpatialKind,
    pub name: String,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub placement: SemioTransform,
}
//#endregion 🔖️Spatial

//#region 🔖️Element
/// 🧱️ Real, named IFC-style element classes plus an honest `Other{name}` catch-all for a class
/// this subset hasn't named yet — carries the REAL class name rather than silently collapsing it
/// (never a lying black-hole variant).
/// 🧪️ `Default` (first variant, `Wall`) is the same `serde_derive` technical workaround as
/// `SpatialKind`'s -- see `SpatialNode`'s doc comment.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ElementClass {
    #[default]
    Wall,
    Slab,
    Column,
    Beam,
    Door,
    Window,
    Roof,
    Stair,
    Furniture,
    Other { name: String },
}

/// 📐️ Owned by `model`: geometry reference resolved BY ID into a sibling subset's own snapshot
/// (`brep`/`mesh`) — never inline duplication (w1b-type-ownership.md cross-reuse summary). Named
/// variants throughout, never a bare tuple (f6-final-summary.md §4.3).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum GeometryRef {
    #[default]
    None,
    Brep { brep_id: String },
    Mesh { mesh_id: String },
}

/// 🏷️ IFC property-set value — weak value type, whole-value replaced in diffs, never sub-diffed
/// (schema-design.md's strong/weak entity split).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum PsetValue {
    Text { value: String },
    Number { value: f64 },
    Boolean { value: bool },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub key: String,
    pub value: PsetValue,
}

/// 📦️ IFC "Pset_*"-shaped named property bag.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertySet {
    pub name: String,
    #[serde(default)]
    pub properties: Vec<Property>,
}

/// 🏛️ Owned by `model`: one spatial/physical element — the master plan's
/// "elements{class enum, placement, GeometryRef{Brep|Mesh|None}, psets}". `Default` is the same
/// `serde_derive` technical workaround as `SpatialNode`'s (see that struct's doc comment).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioModelElement {
    pub id: String,
    pub class: ElementClass,
    #[serde(default)]
    pub placement: SemioTransform,
    #[serde(default)]
    pub geometry: GeometryRef,
    /// 🗺️ Which `SpatialNode` (by id) contains this element — `None` = not yet placed in the
    /// spatial tree. Checked for dangling references by `SemioModelValidator`.
    #[serde(default)]
    pub spatial_id: Option<String>,
    #[serde(default)]
    pub psets: Vec<PropertySet>,
}
//#endregion 🔖️Element

//#region 🔖️Relation
/// 🔗️ IFC-style relationship kinds between two ids (elements and/or spatial nodes) plus an honest
/// `Other{label}` catch-all, same rationale as `ElementClass::Other`.
/// 🧪️ `Default` (first variant, `Aggregates`) is the same `serde_derive` technical workaround as
/// `SpatialKind`'s -- see `SpatialNode`'s doc comment.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RelationKind {
    #[default]
    Aggregates,
    ContainedIn,
    ConnectsTo,
    FillsVoid,
    VoidsElement,
    Other { label: String },
}

/// 🔗️ Owned by `model`: the master plan's "relations{kind enum, from, to}" — `id` is this
/// subset's own synthesized edge key (needed to diff relations as a keyed collection via the
/// shared `🧰️triples` engine; the master plan's 3-field description is the payload this key
/// wraps, not a rejection of having one). `Default` is the same `serde_derive` technical
/// workaround as `SpatialNode`'s.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelRelation {
    pub id: String,
    pub kind: RelationKind,
    pub from: String,
    pub to: String,
}
//#endregion 🔖️Relation

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.model")]
pub struct SemioModelSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub spatial: Vec<SpatialNode>,
    #[state(persistent)]
    #[serde(default)]
    pub elements: Vec<SemioModelElement>,
    #[state(persistent)]
    #[serde(default)]
    pub relations: Vec<ModelRelation>,
}

impl Default for SemioModelSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_SEMIOMODEL_DOCUMENT_SCHEMA.into(),
            spatial: Vec::new(),
            elements: Vec::new(),
            relations: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁️ JSON-pack round trip wrapped in the shared `store::semio_format` envelope (real, genuinely
/// working — not a per-format on-disk binary codec, since this subset's snapshot is a NEUTRAL
/// semio type). Preserved verbatim from the W1b scaffold — envelope shape is unaffected by the
/// snapshot's own field growth.
impl store::ArtifactDsl for SemioModelSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIOMODEL_DOCUMENT_SCHEMA }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        serde_json::from_slice(&bytes).map_err(|e| store::TextError::new(format!("json decode: {e}"), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = serde_json::to_vec(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioModelSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = serde_json::to_vec(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::engine::geometry::{SemioPoint3, SemioQuaternion};

    /// 🏗️ A fully-populated snapshot exercising every collection and every field — the fixture
    /// `codec_retention_law` below round-trips through both codecs.
    fn rich_snapshot() -> SemioModelSnapshot {
        SemioModelSnapshot {
            schema: STDIO_SEMIOMODEL_DOCUMENT_SCHEMA.into(),
            spatial: vec![
                SpatialNode { id: "site-1".into(), kind: SpatialKind::Site, name: "Site One".into(), parent_id: None, placement: SemioTransform::identity() },
                SpatialNode {
                    id: "storey-1".into(),
                    kind: SpatialKind::Storey,
                    name: "Ground Floor".into(),
                    parent_id: Some("site-1".into()),
                    placement: SemioTransform { translation: SemioPoint3 { x: 0.0, y: 0.0, z: 3.0 }, rotation: SemioQuaternion::default(), scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } },
                },
            ],
            elements: vec![SemioModelElement {
                id: "wall-1".into(),
                class: ElementClass::Wall,
                placement: SemioTransform::identity(),
                geometry: GeometryRef::Brep { brep_id: "brep-1".into() },
                spatial_id: Some("storey-1".into()),
                psets: vec![PropertySet {
                    name: "Pset_WallCommon".into(),
                    properties: vec![
                        Property { key: "IsExternal".into(), value: PsetValue::Boolean { value: true } },
                        Property { key: "FireRating".into(), value: PsetValue::Text { value: "REI60".into() } },
                        Property { key: "ThermalTransmittance".into(), value: PsetValue::Number { value: 0.24 } },
                    ],
                }],
            }],
            relations: vec![ModelRelation { id: "rel-1".into(), kind: RelationKind::ContainedIn, from: "wall-1".into(), to: "storey-1".into() }],
        }
    }

    #[test]
    fn json_pack_round_trips() {
        let snap = SemioModelSnapshot::default();
        let bytes = <SemioModelSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioModelSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = SemioModelSnapshot::default();
        let text = <SemioModelSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    /// 🧪️ codec_retention_law: decode(encode(x)) == x for a fully-populated snapshot (every
    /// collection non-empty, every field set), through both the pack (binary) and DSL (text) codecs.
    #[test]
    fn codec_retention_law() {
        let snap = rich_snapshot();
        let packed = <SemioModelSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let unpacked = <SemioModelSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode pack");
        assert_eq!(snap, unpacked);

        let text = <SemioModelSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let reparsed = <SemioModelSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse dsl");
        assert_eq!(snap, reparsed);
    }
}
//#endregion 🔖️Tests
