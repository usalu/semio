//! 🧬️ SemioModelSnapshot — complete per the master plan's `model` row: a flat, id-keyed spatial
//! hierarchy (site/building/storey/space, parent-pointer graph — mirrors ifc/4's
//! `IfcRelAggregates`/`IfcRelContainedInSpatialStructure` shape rather than a recursive tree type)
//! + elements (typed class enum, placement, a BY-ID `GeometryRef` into the sibling `brep`/`mesh`
//!   subsets, and named property sets) + relations (typed kind enum, from/to id endpoints). Owned by
//!   `model` (w1b-type-ownership.md): `SemioModelElement`, `GeometryRef`, plus this file's own
//!   `SpatialNode`/`ModelRelation`/`ElementClass`/`PropertySet`/`Property`/`PsetValue`/
//!   `RelationKind`/`SpatialKind`. `model` never inlines brep/mesh geometry data — `GeometryRef`
//!   resolves by id into those subsets' own snapshots (spec-mandated cross-reuse, master plan
//!   Architecture section); referential integrity of THAT cross-subset link is out of this
//!   snapshot's own scope (it is not decodable from `model` alone), but every reference WITHIN this
//!   subset's own collections (spatial parent pointers, element→spatial containment, relation
//!   endpoints) is checked by the composer's `SemioModelValidator`.

use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion, SemioTransform};


use framework_schema::ArtifactSchema;

//#region 🔖️Ids
pub const STDIO_SEMIOMODEL_DOCUMENT_SCHEMA: &str = "stdio.semio.model";
//#endregion 🔖️Ids

//#region 🔖️Spatial
/// 🏢️ ifc/4 spatial-structure levels this subset targets (`IfcSite`/`IfcBuilding`/
/// `IfcBuildingStorey`/`IfcSpace`) — the master plan's exact "(site/building/storey/space)" list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
/// `serde_derive` limitation (`#[value(default)]` on the shared `🧰️triples::NamedTripleDiff`'s
/// `added: Vec<T>` field spuriously infers `T: Default`, same root cause bcf's own diff module
/// documents) means every strong-entity type reachable through a `NamedTripleDiff<K,D,T>` needs
/// `Default` purely to satisfy that derive, not because any real code constructs a default one.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SpatialNode {
    pub id: String,
    pub kind: SpatialKind,
    pub name: String,
    #[value(default)]
    pub parent_id: Option<String>,
    #[value(default)]
    pub placement: SemioTransform,
}
//#endregion 🔖️Spatial

//#region 🔖️Element
/// 🧱️ Real, named IFC-style element classes plus an honest `Other{name}` catch-all for a class
/// this subset hasn't named yet — carries the REAL class name rather than silently collapsing it
/// (never a lying black-hole variant).
/// 🧪️ `Default` (first variant, `Wall`) is the same `serde_derive` technical workaround as
/// `SpatialKind`'s -- see `SpatialNode`'s doc comment.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned)]
#[value(tag = "kind", rename_all = "camelCase")]
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
    Other {
        name: String,
    },
}

/// 📐️ Owned by `model`: geometry reference resolved BY ID into a sibling subset's own snapshot
/// (`brep`/`mesh`) — never inline duplication (w1b-type-ownership.md cross-reuse summary). Named
/// variants throughout, never a bare tuple (f6-final-summary.md §4.3).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum GeometryRef {
    #[default]
    None,
    Brep {
        brep_id: String,
    },
    Mesh {
        mesh_id: String,
    },
}

/// 🏷️ IFC property-set value — weak value type, whole-value replaced in diffs, never sub-diffed
/// (schema-design.md's strong/weak entity split).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum PsetValue {
    Text { value: String },
    Number { value: f64 },
    Boolean { value: bool },
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Property {
    pub key: String,
    pub value: PsetValue,
}

/// 📦️ IFC "Pset_*"-shaped named property bag.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PropertySet {
    pub name: String,
    #[value(default)]
    pub properties: Vec<Property>,
}

/// 🏛️ Owned by `model`: one spatial/physical element — the master plan's
/// "elements{class enum, placement, GeometryRef{Brep|Mesh|None}, psets}". `Default` is the same
/// `serde_derive` technical workaround as `SpatialNode`'s (see that struct's doc comment).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioModelElement {
    pub id: String,
    pub class: ElementClass,
    #[value(default)]
    pub placement: SemioTransform,
    #[value(default)]
    pub geometry: GeometryRef,
    /// 🗺️ Which `SpatialNode` (by id) contains this element — `None` = not yet placed in the
    /// spatial tree. Checked for dangling references by `SemioModelValidator`.
    #[value(default)]
    pub spatial_id: Option<String>,
    #[value(default)]
    pub psets: Vec<PropertySet>,
}
//#endregion 🔖️Element

//#region 🔖️Relation
/// 🔗️ IFC-style relationship kinds between two ids (elements and/or spatial nodes) plus an honest
/// `Other{label}` catch-all, same rationale as `ElementClass::Other`.
/// 🧪️ `Default` (first variant, `Aggregates`) is the same `serde_derive` technical workaround as
/// `SpatialKind`'s -- see `SpatialNode`'s doc comment.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum RelationKind {
    #[default]
    Aggregates,
    ContainedIn,
    ConnectsTo,
    FillsVoid,
    VoidsElement,
    Other {
        label: String,
    },
}

/// 🔗️ Owned by `model`: the master plan's "relations{kind enum, from, to}" — `id` is this
/// subset's own synthesized edge key (needed to diff relations as a keyed collection via the
/// shared `🧰️triples` engine; the master plan's 3-field description is the payload this key
/// wraps, not a rejection of having one). `Default` is the same `serde_derive` technical
/// workaround as `SpatialNode`'s.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ModelRelation {
    pub id: String,
    pub kind: RelationKind,
    pub from: String,
    pub to: String,
}
//#endregion 🔖️Relation

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.model")]
pub struct SemioModelSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub spatial: Vec<SpatialNode>,
    #[state(artifact)]
    #[value(default)]
    pub elements: Vec<SemioModelElement>,
    #[state(artifact)]
    #[value(default)]
    pub relations: Vec<ModelRelation>,
}

impl Default for SemioModelSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOMODEL_DOCUMENT_SCHEMA.into(), spatial: Vec::new(), elements: Vec::new(), relations: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives


















































//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives










































//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️ReachableCodecs




//#endregion 🔖️ReachableCodecs

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.model` document — a fully-populated snapshot exercising every
/// collection/leaf shape at least once. Single source of truth for
/// `📚️examples/🏢️building/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` and for the
/// conformance-law tests in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_semio_model_snapshot() -> SemioModelSnapshot {
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
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests







