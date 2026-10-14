//! 🧬️ Puzzle3d artifact schema — every field of the artifact with its state class.

use crate::{Puzzle3dSnapshot};
use ::semio_framework_schema::ArtifactSchema;
//#region 🔖️Artifact
/// 🧬️ puzzle3d document artifact state.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle3d")]
pub struct Puzzle3dArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub domain: String,
    #[state(artifact)]
    pub meta: Puzzle3dMeta,
    #[state(artifact)]
    pub objects: Vec<Puzzle3dObject>,
    #[state(artifact)]
    pub attractions: Vec<Puzzle3dAttraction>,
    #[state(artifact)]
    pub target_volumes: Vec<Puzzle3dTargetVolume>,
    #[state(artifact)]
    pub references: Vec<Puzzle3dReference>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Puzzle3dArtifact {
    fn default() -> Self {
        Self::from_snapshot(Puzzle3dSnapshot::default())
    }
}

impl Puzzle3dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Puzzle3dSnapshot {
        Puzzle3dSnapshot {
            schema: self.schema.clone(),
            domain: self.domain.clone(),
            meta: self.meta.clone(),
            objects: self.objects.clone(),
            attractions: self.attractions.clone(),
            target_volumes: self.target_volumes.clone(),
            references: self.references.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: Puzzle3dSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            domain: snapshot.domain,
            meta: snapshot.meta,
            objects: snapshot.objects,
            attractions: snapshot.attractions,
            target_volumes: snapshot.target_volumes,
            references: snapshot.references,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Puzzle3dSnapshot) {
        self.schema = snapshot.schema;
        self.domain = snapshot.domain;
        self.meta = snapshot.meta;
        self.objects = snapshot.objects;
        self.attractions = snapshot.attractions;
        self.target_volumes = snapshot.target_volumes;
        self.references = snapshot.references;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.puzzle.puzzle3d` — twenty handcrafted schema leaves.
pub fn puzzle3d_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.puzzle.puzzle3d",
        artifact: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️PrecomputeModel
// ⚙️➡️🧬️ Rehomed from the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
// the pure data shapes the interactive brush/fill precompute session (now `crate::editor::puzzle3d::precompute`)
// exchanges with its host — the kind catalogs, the host rules/weights, the `EngineSceneSnapshot`/`SceneConfig` wire
// projection `Puzzle3dEngineCommand::SetScene` carries, and the brush/fill readouts. An artifact is a schema
// plus an io system, never an engine — the actual stateful session lives app-side; this is its data.
pub(crate) type Quat = [f64; 4];
pub(crate) type Vec3 = [f64; 3];

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BrushHostRules {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) reject_capital_on_tambour: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) reject_last_single_storey_on_mid_tambour: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) door_tambour_requires_door_capsule: bool,
    #[cfg_attr(test, serde(default = "default_door_capsule_min_abs_x"))]
    #[value(default = "default_door_capsule_min_abs_x")]
    pub(crate) door_capsule_min_abs_x: f64,
    #[cfg_attr(test, serde(default = "default_door_capsule_max_abs_y"))]
    #[value(default = "default_door_capsule_max_abs_y")]
    pub(crate) door_capsule_max_abs_y: f64,
}

fn default_door_capsule_min_abs_x() -> f64 {
    0.9
}

fn default_door_capsule_max_abs_y() -> f64 {
    1.6
}

impl Default for BrushHostRules {
    fn default() -> Self {
        Self {
            reject_capital_on_tambour: true,
            reject_last_single_storey_on_mid_tambour: true,
            door_tambour_requires_door_capsule: true,
            door_capsule_min_abs_x: default_door_capsule_min_abs_x(),
            door_capsule_max_abs_y: default_door_capsule_max_abs_y(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BrushKindWeights {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) object_weights: std::collections::BTreeMap<String, f64>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) vortex_weights: std::collections::BTreeMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct KindCompatEntry {
    pub(crate) source: String,
    pub(crate) target: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) bidirectional: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) important: bool,
    pub(crate) specificity: Option<String>,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectKindVortexTemplate {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) label: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) description: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) icon: String,
    #[cfg_attr(test, serde(rename = "vortexKind", default))]
    #[value(rename = "vortexKind", default)]
    pub(crate) vortex_kind: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) point: Vec3,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) direction: Option<Vec3>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) t: Option<f64>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) mandatory: Option<bool>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) radius: Option<f64>,
}

impl Default for ObjectKindVortexTemplate {
    fn default() -> Self {
        Self { id: String::new(), name: String::new(), label: String::new(), description: String::new(), icon: String::new(), vortex_kind: None, point: [0.0, 0.0, 0.0], direction: None, t: None, mandatory: None, radius: None }
    }
}

#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectKindRepresentation {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) url: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) mime: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) tags: Vec<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) lod: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) description: String,
}

#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectKind {
    pub(crate) id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) representations: Vec<ObjectKindRepresentation>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) scale: Option<semio_framework_value::DslValue>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) vortices: Vec<ObjectKindVortexTemplate>,
}

#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct VortexKindCatalog {
    pub(crate) id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) code: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) label: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) order: Option<i32>,
    #[cfg_attr(test, serde(default, rename = "compatibleWith"))]
    #[value(default, rename = "compatibleWith")]
    pub(crate) compatible_with: Vec<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) description: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) icon: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) color: String,
    #[cfg_attr(test, serde(rename = "defaultCableKind", default))]
    #[value(rename = "defaultCableKind", default)]
    pub(crate) default_cable_kind: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct CableKindCatalog {
    pub(crate) id: String,
    #[cfg_attr(test, serde(rename = "defaultAttractionKind", default))]
    #[value(rename = "defaultAttractionKind", default)]
    pub(crate) default_attraction_kind: Option<String>,
}

/// 🗂️ The compile-time-catalog side of a scene: object/vortex/cable kind rows, reachable through
/// `apply_brush_placement_to_snapshot`'s public signature.
#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct KindCatalogBundle {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) objects: Vec<ObjectKind>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) vortices: Vec<VortexKindCatalog>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) cables: Vec<CableKindCatalog>,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct VortexProps {
    pub id: String,
    #[cfg_attr(test, serde(rename = "vortexKind", default))]
    #[value(rename = "vortexKind", default)]
    pub vortex_kind: Option<String>,
    pub position: Vec3,
    pub direction: Option<Vec3>,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct EngineSceneObject {
    pub id: String,
    #[cfg_attr(test, serde(rename = "objectKind", default))]
    #[value(rename = "objectKind", default)]
    pub object_kind: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub anchor: crate::standards::v1::subsets::any::schema::Puzzle3dObjectAnchor,
    #[cfg_attr(test, serde(rename = "meshUrl", default))]
    #[value(rename = "meshUrl", default)]
    pub mesh_url: Option<String>,
    pub origin: Vec3,
    pub orientation: Option<Quat>,
    pub scale: Option<semio_framework_value::DslValue>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub vortices: Vec<VortexProps>,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AttractionProps {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub id: String,
    pub attracting: String,
    pub attracted: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub gap: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub shift: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub rise: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub rotation: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub turn: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub tilt: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub x: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct WorldVolumeProps {
    pub id: String,
    pub origin: Vec3,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub orientation: Option<Quat>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub scale: Option<semio_framework_value::DslValue>,
}

/// 🏗️ A puzzle-3d scene's object/attraction/target-volume state, reachable through
/// `apply_brush_placement_to_snapshot`'s public signature.
#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct EngineSceneSnapshot {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub attractions: Vec<AttractionProps>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub objects: Vec<EngineSceneObject>,
    #[cfg_attr(test, serde(default, rename = "targetVolumes"))]
    #[value(default, rename = "targetVolumes")]
    pub target_volumes: Vec<WorldVolumeProps>,
}

/// 📨️ The full typed payload `Puzzle3dEngineCommand::SetScene` carries — the exact same shape
/// `Puzzle3dCollision::set_scene`'s JSON payload has always deserialized into, just reused directly
/// instead of re-declared, so the command enum's field IS this type, not a mirror of it.
#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct SceneConfig {
    #[cfg_attr(test, serde(rename = "sceneSnapshot"))]
    #[value(rename = "sceneSnapshot")]
    pub(crate) scene_snapshot: EngineSceneSnapshot,
    #[cfg_attr(test, serde(rename = "kindCatalogs", default))]
    #[value(rename = "kindCatalogs", default)]
    pub(crate) kind_catalogs: Option<KindCatalogBundle>,
    #[cfg_attr(test, serde(rename = "kindCompatibility", default))]
    #[value(rename = "kindCompatibility", default)]
    pub(crate) kind_compatibility: Vec<KindCompatEntry>,
    #[cfg_attr(test, serde(rename = "contactTolerance", default))]
    #[value(rename = "contactTolerance", default)]
    pub(crate) contact_tolerance: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) seed: u32,
    #[cfg_attr(test, serde(rename = "hostRules", default))]
    #[value(rename = "hostRules", default)]
    pub(crate) host_rules: BrushHostRules,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub(crate) weights: BrushKindWeights,
}

#[derive(Debug, Clone, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct BrushCompatibleCandidate {
    pub object_kind_id: String,
    pub source_vortex_index: usize,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct BrushPreviewState {
    pub target_vortex_full_id: String,
    pub object_kind_id: String,
    pub source_vortex_index: usize,
    pub mesh_url: String,
    pub origin: Vec3,
    pub orientation: Quat,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub scale: Option<semio_framework_value::DslValue>,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BrushPlacePayload {
    pub target_vortex_full_id: String,
    pub object_kind_id: String,
    pub source_vortex_index: usize,
    pub origin: Vec3,
    pub orientation: Quat,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub scale: Option<semio_framework_value::DslValue>,
}

/// 🎯️ A suggestion-popup preview accepted as-is becomes a placement at the exact same pose — the one
/// field `BrushPreviewState` carries that `BrushPlacePayload` doesn't (`mesh_url`, resolvable again
/// from `object_kind_id` via the kind catalog) is simply dropped.
impl From<BrushPreviewState> for BrushPlacePayload {
    fn from(preview: BrushPreviewState) -> Self {
        Self { target_vortex_full_id: preview.target_vortex_full_id, object_kind_id: preview.object_kind_id, source_vortex_index: preview.source_vortex_index, origin: preview.origin, orientation: preview.orientation, scale: preview.scale }
    }
}

/// 🎯️ Public so `Puzzle3dEngineOutcome::BrushCandidates` can hand this back to callers (the app's
/// brush slot) as a typed value instead of the JSON string the old `brush_candidates` wasm-bindgen
/// method returned.
#[derive(Debug, Clone, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct BrushCollisionFreeResult {
    pub free: Vec<BrushCompatibleCandidate>,
    pub unknown_pending: bool,
    #[value(default)]
    pub resume_candidate_index: usize,
}

/// 🧭️ Stage of a brush suggestions tool run, the index into `ToolRunDefinition.stages`
/// (`$defs.Puzzle3dBrushSuggestionsRun`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrushSuggestionsRunStage {
    Prepare,
    Target,
    Test,
    Idle,
}

impl BrushSuggestionsRunStage {
    pub const ALL: [Self; 4] = [Self::Prepare, Self::Target, Self::Test, Self::Idle];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Prepare => "prepare",
            Self::Target => "target",
            Self::Test => "test",
            Self::Idle => "idle",
        }
    }
}

/// 🔢️ Counter of a brush suggestions tool run, the index into `ToolRunDefinition.counters`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrushSuggestionsRunCounter {
    Tested,
    Free,
    Collisions,
}

impl BrushSuggestionsRunCounter {
    pub const ALL: [Self; 3] = [Self::Tested, Self::Free, Self::Collisions];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Tested => "tested",
            Self::Free => "free",
            Self::Collisions => "collisions",
        }
    }
}

/// 🏷️ Reason code of a brush suggestions trace record or step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrushSuggestionsRunReason {
    Free,
    Collision,
    PoseUnavailable,
    TargetMissing,
    SuggestionsBlocked,
    SearchComplete,
}

impl BrushSuggestionsRunReason {
    pub const ALL: [Self; 6] = [Self::Free, Self::Collision, Self::PoseUnavailable, Self::TargetMissing, Self::SuggestionsBlocked, Self::SearchComplete];

    pub fn code(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Free => "free",
            Self::Collision => "collision",
            Self::PoseUnavailable => "pose-unavailable",
            Self::TargetMissing => "target-missing",
            Self::SuggestionsBlocked => "suggestions-blocked",
            Self::SearchComplete => "search-complete",
        }
    }

    /// 🚥️ A free candidate and a finished search succeed, an overlap is a collision, the rest warn.
    pub fn verdict(self) -> semio_framework_tool_run::ToolRunVerdict {
        match self {
            Self::Free | Self::SearchComplete => semio_framework_tool_run::ToolRunVerdict::Success,
            Self::Collision => semio_framework_tool_run::ToolRunVerdict::Danger,
            Self::PoseUnavailable | Self::TargetMissing | Self::SuggestionsBlocked => semio_framework_tool_run::ToolRunVerdict::Warning,
        }
    }
}

/// 🧭️ Stage of a fill tool run, the index into `ToolRunDefinition.stages` (`$defs.Puzzle3dFillRun`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FillRunStage {
    Prepare,
    Search,
    Test,
    Lock,
    Retract,
}

impl FillRunStage {
    pub const ALL: [Self; 5] = [Self::Prepare, Self::Search, Self::Test, Self::Lock, Self::Retract];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Prepare => "prepare",
            Self::Search => "search",
            Self::Test => "test",
            Self::Lock => "lock",
            Self::Retract => "retract",
        }
    }
}

/// 🔢️ Counter of a fill tool run, the index into `ToolRunDefinition.counters`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FillRunCounter {
    Tested,
    Locked,
    Collisions,
    Rejected,
    Marked,
}

impl FillRunCounter {
    pub const ALL: [Self; 5] = [Self::Tested, Self::Locked, Self::Collisions, Self::Rejected, Self::Marked];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Tested => "tested",
            Self::Locked => "locked",
            Self::Collisions => "collisions",
            Self::Rejected => "rejected",
            Self::Marked => "marked",
        }
    }
}

/// 🏷️ Reason code of a fill run trace record or step; `id` spells the planner's own refusal string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FillRunReason {
    Fits,
    SolidOverlap,
    OutsideTargetVolume,
    MeshUnavailable,
    MissingPreview,
    MissingTarget,
    BroadPhaseEntryMissing,
    PlacedMeshUnavailable,
    StaleSpatialQuery,
    PlacementKindMissing,
    PlacementVortexMissing,
    PlacementMeshMissing,
    PlacementRejected,
    PlacementStateMissing,
    PlacementSpatialStateMissing,
    StaleSpatialMutation,
    Rejected,
    NoOpenVortex,
    NoCompatibleKind,
    NoFreePlacement,
    ArtifactCapacity,
    RequestedReached,
    Retracted,
    VortexExhausted,
}

impl FillRunReason {
    pub const ALL: [Self; 24] = [
        Self::Fits,
        Self::SolidOverlap,
        Self::OutsideTargetVolume,
        Self::MeshUnavailable,
        Self::MissingPreview,
        Self::MissingTarget,
        Self::BroadPhaseEntryMissing,
        Self::PlacedMeshUnavailable,
        Self::StaleSpatialQuery,
        Self::PlacementKindMissing,
        Self::PlacementVortexMissing,
        Self::PlacementMeshMissing,
        Self::PlacementRejected,
        Self::PlacementStateMissing,
        Self::PlacementSpatialStateMissing,
        Self::StaleSpatialMutation,
        Self::Rejected,
        Self::NoOpenVortex,
        Self::NoCompatibleKind,
        Self::NoFreePlacement,
        Self::ArtifactCapacity,
        Self::RequestedReached,
        Self::Retracted,
        Self::VortexExhausted,
    ];

    pub fn code(self) -> u16 {
        self as u16
    }

    pub fn from_code(code: u16) -> Option<Self> {
        Self::ALL.get(usize::from(code)).copied()
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Fits => "fits",
            Self::SolidOverlap => "solid-overlap",
            Self::OutsideTargetVolume => "outside-target-volume",
            Self::MeshUnavailable => "mesh-unavailable",
            Self::MissingPreview => "missing-preview",
            Self::MissingTarget => "missing-target",
            Self::BroadPhaseEntryMissing => "broad-phase-entry-missing",
            Self::PlacedMeshUnavailable => "placed-mesh-unavailable",
            Self::StaleSpatialQuery => "stale-spatial-query",
            Self::PlacementKindMissing => "placement-kind-missing",
            Self::PlacementVortexMissing => "placement-vortex-missing",
            Self::PlacementMeshMissing => "placement-mesh-missing",
            Self::PlacementRejected => "placement-rejected",
            Self::PlacementStateMissing => "placement-state-missing",
            Self::PlacementSpatialStateMissing => "placement-spatial-state-missing",
            Self::StaleSpatialMutation => "stale-spatial-mutation",
            Self::Rejected => "rejected",
            Self::NoOpenVortex => "no-open-vortex",
            Self::NoCompatibleKind => "no-compatible-kind",
            Self::NoFreePlacement => "no-free-placement",
            Self::ArtifactCapacity => "artifact-capacity",
            Self::RequestedReached => "requested-reached",
            Self::Retracted => "retracted",
            Self::VortexExhausted => "vortex-exhausted",
        }
    }

    pub fn of_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|candidate| candidate.id() == id)
    }

    /// 🔎️ The reason a planner refusal string names; an undeclared refusal is a plain `rejected`.
    pub fn of_refusal(reason: &str) -> Self {
        Self::ALL[..Self::Rejected as usize].iter().copied().find(|candidate| candidate.id() == reason).unwrap_or(Self::Rejected)
    }

    /// 🚥️ `fits` succeeds, `solid-overlap` is a collision, every other candidate refusal is a rule warning,
    /// the stalls warn, reaching the request succeeds, a retraction informs and a vortex without any collision-free
    /// candidate is marked as danger.
    pub fn verdict(self) -> semio_framework_tool_run::ToolRunVerdict {
        match self {
            Self::Fits | Self::RequestedReached => semio_framework_tool_run::ToolRunVerdict::Success,
            Self::SolidOverlap | Self::VortexExhausted => semio_framework_tool_run::ToolRunVerdict::Danger,
            Self::Retracted => semio_framework_tool_run::ToolRunVerdict::Testing,
            _ => semio_framework_tool_run::ToolRunVerdict::Warning,
        }
    }
}

/// 📸️ Resume point a fill run job reports through `JobTurn::Checkpoint`: fixed 68-byte
/// little-endian layout `requested u64 | placements u64 | provisionalOps u32 | tested u64 | nextKey u64 |
/// inputs [u8; 32]`. `inputs` digests everything the planner's deterministic sequence depends on except
/// the requested count (base revision, contact tolerance, weights, collision meshes), so a rebuilt run job
/// replays to the checkpoint only when its prefix is provably the same sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FillRunCheckpoint {
    pub requested: u64,
    pub placements: u64,
    pub provisional_ops: u32,
    pub tested: u64,
    pub next_key: u64,
    pub inputs: [u8; 32],
}



pub fn empty_puzzle3d_snapshot() -> Puzzle3dSnapshot {
    Puzzle3dSnapshot::default()
}
//#endregion 🔖️PrecomputeModel

//#region 🔖️PrecomputeCommand
/// 🎯️ Typed command envelope for `crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession::dispatch`
/// — the headless replacement for the old per-action JSON-string wasm-bindgen methods. Declared here (not
/// app-side) because `#[derive(dsl::DslEnum)]`'s generated code needs `SceneConfig`/`BrushPlacePayload` in
/// scope by value, and because `🧬️mutations/💾️binary`'s `encode_engine_command`/`decode_engine_command`
/// wrap it exactly like it already does for `Puzzle3dMutation`. Field shapes mirror the exact payload each
/// old JSON-string method parsed: `SetScene` mirrors `set_scene`'s `SceneConfig` JSON body,
/// `UpdateKindWeights` mirrors `update_kind_weights`'s two JSON map bodies.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
pub enum Puzzle3dEngineCommand {
    #[dsl(key = "set-scene")]
    SetScene { scene: SceneConfig },
    #[dsl(key = "update-kind-weights")]
    UpdateKindWeights { object_weights: std::collections::BTreeMap<String, f64>, vortex_weights: std::collections::BTreeMap<String, f64> },
    #[dsl(key = "brush-preview")]
    BrushPreview { vortex_full_id: String, candidate_index: u32 },
}
//#region 🔖️HandcraftedOpCodecs



//#endregion 🔖️HandcraftedOpCodecs

/// 📬️ What `dispatch` hands back — the typed counterpart of what each old JSON-string method
/// returned (a `BrushPreviewState` JSON string, or nothing). Plain Rust, no
/// DSL/wasm-bindgen requirement — this only ever crosses the artifact <-> app boundary in-process.
#[derive(Debug, Clone, PartialEq)]
pub enum Puzzle3dEngineOutcome {
    Unit,
    BrushPreview(Option<BrushPreviewState>),
}
//#endregion 🔖️PrecomputeCommand

//#region 🧪️PrecomputeModelTests
/// 🧪️ The one puzzle3d-precompute test harness — every sibling app-side precompute test file builds on
/// it instead of re-deriving a mesh-buffer/scene/fill-plan scaffold of its own. `pub(crate)` so the app's
/// own `#[cfg(test)]` modules (session/geometry/brush) can reach it across the artifact/app boundary.
#[cfg(test)]
#[path = "🧪️tests/🔬️precompute-model/🦀️.rs"]
pub(crate) mod precompute_model_tests;
//#endregion 🧪️PrecomputeModelTests


/// 🧯️ Puzzle 3d precompute session errors — JSON (de)serialization and brush/fill session state
/// failures. `Json` wraps `semio_framework_value::ValueError` (not `serde_json::Error`) since every production
/// (de)serialization call site now routes text through `dsl::os_pack::json::{from_json_str,
/// to_json_string}`, which are `ToValue`/`FromValue`-based, not `serde`-based.
#[derive(Debug)]
pub enum Puzzle3dError {
    Json(semio_framework_value::ValueError),
    FillSessionUnavailable,
}

impl std::fmt::Display for Puzzle3dError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "{error}"),
            Self::FillSessionUnavailable => formatter.write_str("fill session unavailable"),
        }
    }
}

impl std::error::Error for Puzzle3dError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => std::error::Error::source(error),
            Self::FillSessionUnavailable => None,
        }
    }
}

impl From<semio_framework_value::ValueError> for Puzzle3dError {
    fn from(error: semio_framework_value::ValueError) -> Self {
        Self::Json(error)
    }
}

pub const PUZZLE_3D_SCHEMA: &str = "puzzle.3d";

/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// surface (`✏️editor`, `👁️viewer`) of the `✳️any` subset binds `ArtifactEditor::DIALECT`/
/// `ArtifactViewer::DIALECT` to — `"s.puzzle.puzzle3d"` matches the artifact-kind id this subset's
/// own mutation/inference descriptors already key off (see `definition()`'s own
/// `"s.puzzle3d.schema.artifact"` row), standard `"1"` and subset `"*"` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location. Lives at the artifact level (not under the two
/// surfaces) so a viewer file can read it without ever importing through the sibling editor module.
pub const PUZZLE3D_DIALECT: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: "s.puzzle.puzzle3d", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId::ANY };

/// 📐️ A placed object's / target volume's freeform pose scale: either a single scalar broadcast
/// to all three axes, or an explicit per-axis `[x, y, z]` triple — the ONLY two shapes the engine
/// crate's `vec3_scale`/`volume_scale_vec` ever interpret (see that crate's implementation and its
/// `vec3_scale_variants` test), so this is a small closed union rather than genuinely heterogeneous
/// JSON. Replaces the former `serde_json::Value` passthrough with the actual shape.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Puzzle3dScale {
    Uniform(f64),
    Vec3([f64; 3]),
}

semio_framework_value::artifact_retire_leaf!(Puzzle3dScale);

impl semio_framework_value::retained_clone::RetainedClone for Puzzle3dScale {
    type Cursor = semio_framework_value::retained_clone::ScalarCursor<Self>;
    fn retained_clone_cursor() -> Self::Cursor {
        Default::default()
    }
}

/// 🌲️ The canonical tree keeps the wire shape: a bare number for `Uniform`, an `[x, y, z]` array for `Vec3`.
impl semio_framework_pack_json::ArtifactCanonicalJsonTree for Puzzle3dScale {
    fn canonical_tree_node(&self) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>, semio_framework_value::ValueError> {
        Ok(match self {
            Self::Uniform(value) => semio_framework_pack_json::ArtifactCanonicalJsonNode::F64(*value),
            Self::Vec3(_) => semio_framework_pack_json::ArtifactCanonicalJsonNode::Array(3),
        })
    }

    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree, semio_framework_value::ValueError> {
        match self {
            Self::Vec3(axes) => axes.get(ordinal).map(|axis| axis as &dyn semio_framework_pack_json::ArtifactCanonicalJsonTree).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical scale axis ordinal is absent")),
            Self::Uniform(_) => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical uniform scale has no child")),
        }
    }
}

/// 🔗️ Wire shape stays identical to the former `serde_json::Value` passthrough (a bare number or
/// an `[x, y, z]` array) so every JSON-boundary consumer (the engine/ui wasm crates' own mirror
/// structs, which bind `scale` as `Option<serde_json::Value>` and are out of this derive's scope)
/// keeps parsing it exactly as before.
#[cfg(test)]
impl serde::Serialize for Puzzle3dScale {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Puzzle3dScale::Uniform(scale) => serializer.serialize_f64(*scale),
            Puzzle3dScale::Vec3(vec3) => serde::Serialize::serialize(vec3, serializer),
        }
    }
}

#[cfg(test)]
impl<'de> serde::Deserialize<'de> for Puzzle3dScale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match <serde_json::Value as serde::Deserialize>::deserialize(deserializer)? {
            serde_json::Value::Number(n) => Ok(Puzzle3dScale::Uniform(n.as_f64().unwrap_or(1.0))),
            serde_json::Value::Array(items) if items.len() >= 3 => {
                let axis = |i: usize| items[i].as_f64().unwrap_or(1.0);
                Ok(Puzzle3dScale::Vec3([axis(0), axis(1), axis(2)]))
            }
            other => Err(serde::de::Error::custom(format!("expected scale to be a number or an [x, y, z] array, found {other}"))),
        }
    }
}

/// 🔁️ Hand-written (mirrors the `Serialize`/`Deserialize` pair directly above): same bare-number-or-
/// `[x, y, z]`-array wire shape.
impl semio_framework_value::ToValue for Puzzle3dScale {
    fn to_value(&self) -> semio_framework_value::DslValue {
        match self {
            Puzzle3dScale::Uniform(scale) => semio_framework_value::ToValue::to_value(scale),
            Puzzle3dScale::Vec3(vec3) => semio_framework_value::ToValue::to_value(vec3),
        }
    }
    fn to_value_controlled(&self,c:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_value::DslValue,semio_framework_value::ValueError>{match self{Self::Uniform(v)=><f64 as semio_framework_value::ToValue>::to_value_controlled(v,c),Self::Vec3(v)=><[f64;3] as semio_framework_value::ToValue>::to_value_controlled(v,c)}}

}

impl semio_framework_value::FromValue for Puzzle3dScale {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        match value {
            semio_framework_value::DslValue::Array(items) if items.len() >= 3 => {
                let axis = |i: usize| items[i].as_f64().unwrap_or(1.0);
                Ok(Puzzle3dScale::Vec3([axis(0), axis(1), axis(2)]))
            }
            other => f64::from_value(other).map(Puzzle3dScale::Uniform),
        }
    }
    fn from_value_controlled(value:&semio_framework_value::DslValue,c:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{c.charge(size_of::<Self>())?;match value{semio_framework_value::DslValue::Array(_)=><[f64;3] as semio_framework_value::FromValue>::from_value_controlled(value,c).map(Self::Vec3),_=><f64 as semio_framework_value::FromValue>::from_value_controlled(value,c).map(Self::Uniform)}}

}

/// 🔗️ Hand `DslField` bridge for `Puzzle3dScale`: `objects`/`targetVolumes` are `#[dsl(table)]`
/// collections, so this field prints as a BARE positional table column — the unbounded
/// `Shape::Tuple(Float, None)` the sibling `puzzle_5d::Puzzle5dScale` uses (its `scale` is reached
/// through a nested keyed record field, not a bare column) is rejected there at parse time
/// (`table column 'scale' has a non-self-delimiting shape (TUPLE) and cannot be a table column`,
/// per the engine's own `validate_table_columns`), so this binds through the bracketed
/// `Shape::List(Float)` instead: `scale=[2]` (uniform) / `scale=[2 3 4]` (per-axis) — the brackets
/// make it self-delimiting regardless of item count.
impl semio_framework_dsl_record::BorrowedDslField for Puzzle3dScale {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = <Vec<f64> as semio_framework_dsl_record::BorrowedDslField>::SHAPE;
}

impl semio_framework_dsl_record::DslField for Puzzle3dScale {
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, semio_framework_value::ValueError> {
        <Vec<f64> as semio_framework_dsl_record::DslField>::shape_controlled(control)
    }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::FieldValue, semio_framework_value::ValueError> {
        let axes = match self { Self::Uniform(v) => std::slice::from_ref(v), Self::Vec3(v) => v.as_slice() };
        control.scoped_stage(|control| {
            control.begin_stage(axes.len())?;
            let mut values = control.allocate_vec::<semio_framework_dsl_record::FieldValue>(axes.len())?;
            for value in axes { values.push(<f64 as semio_framework_dsl_record::DslField>::to_value_controlled(value, control)?); }
            Ok(semio_framework_dsl_record::FieldValue::List(values))
        })
    }
    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        control.checkpoint()?;
        let semio_framework_dsl_record::FieldValue::List(items) = value else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Puzzle3d Scale requires one or three scalars")) };
        if items.len() != 1 && items.len() != 3 { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Puzzle3d Scale requires one or three scalars")) }
        control.scoped_stage(|control| {
            control.begin_stage(items.len())?;
            control.charge(size_of::<Self>())?;
            let x = <f64 as semio_framework_dsl_record::DslField>::from_value_controlled(&items[0], control)?;
            if items.len() == 1 { return Ok(Self::Uniform(x)) }
            let y = <f64 as semio_framework_dsl_record::DslField>::from_value_controlled(&items[1], control)?;
            let z = <f64 as semio_framework_dsl_record::DslField>::from_value_controlled(&items[2], control)?;
            Ok(Self::Vec3([x,y,z]))
        })
    }

    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::List(Box::new(semio_framework_dsl_record::Shape::Float))
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        match self {
            Puzzle3dScale::Uniform(scale) => semio_framework_dsl_record::FieldValue::List(vec![semio_framework_dsl_record::FieldValue::Float(*scale)]),
            Puzzle3dScale::Vec3(vec3) => semio_framework_dsl_record::FieldValue::List(vec3.iter().map(|axis| semio_framework_dsl_record::FieldValue::Float(*axis)).collect()),
        }
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::List(items) if items.len() == 1 => match &items[0] {
                semio_framework_dsl_record::FieldValue::Float(scale) => Ok(Puzzle3dScale::Uniform(*scale)),
                other => Err(format!("expected Float, found {other:?}")),
            },
            semio_framework_dsl_record::FieldValue::List(items) if items.len() >= 3 => {
                let axis = |i: usize| match &items[i] {
                    semio_framework_dsl_record::FieldValue::Float(v) => Ok(*v),
                    other => Err(format!("expected Float, found {other:?}")),
                };
                Ok(Puzzle3dScale::Vec3([axis(0)?, axis(1)?, axis(2)?]))
            }
            other => Err(format!("expected a 1- or 3-item List, found {other:?}")),
        }
    }
}

// #region 🔖️Document
/// ⚓️ Whether a root object keeps its stored plane (`Fixed`) or resets to default XY (`Derived`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum Puzzle3dObjectAnchor {
    #[default]
    Fixed,
    Derived,
}

/// 🔘️ One vortex on an object's rim — `vortex_kind` gates attraction compatibility, `position`/
/// `direction` place and orient it, `radius` sizes its brush-fill collision.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dVortex {
    pub id: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "vortex_kind")]
    pub vortex_kind: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(dir)]
    pub direction: Option<[f64; 3]>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub hidden: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub locked: bool,
}

/// 🧱️ One placed object — `origin`/`orientation`/`scale` (a scalar-or-`[x,y,z]` `Puzzle3dScale`,
/// see that type and `vec3_scale`) pose it, `anchor` gates flatten-root plane retention, `vortices`
/// are its rim attraction ports.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dObject {
    pub id: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "object_kind")]
    pub object_kind: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub anchor: Puzzle3dObjectAnchor,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Puzzle3dScale>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mesh_url: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub vortices: Vec<Puzzle3dVortex>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub hidden: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub locked: bool,
}

/// 🔗️ One attraction between two full vortex ids (`object_id:vortex_id`), with the eight compose
/// connection parameters (`gap`/`shift`/`rise`/`rotation`/`turn`/`tilt` plus diagram `x`/`y`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dAttraction {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub id: String,
    pub attracting: String,
    pub attracted: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub gap: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub shift: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub rise: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub rotation: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub turn: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub tilt: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub x: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub y: f64,
}

/// 🧊️ A persisted oriented box constraining fill placement (Volume Brush voxels or Transform-gumball
/// edited volumes). `scale` is a scalar-or-`[x,y,z]` `Puzzle3dScale` — see that type and
/// `volume_scale_vec`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dTargetVolume {
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<[f64; 4]>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Puzzle3dScale>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub hidden: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub locked: bool,
}

/// 🌐️ Where a reference image/media's bytes live and what kind of media it is.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dReferenceSource {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub url: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub media_kind: Option<String>,
}

/// 🖼️ A reference plane pinned in world space at `origin`, `width_world` meters wide.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dReference {
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub source: Puzzle3dReferenceSource,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(unit = "m")]
    pub width_world: f64,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub locked: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub hidden: bool,
}

/// 🔗️ How specifically two vortex/cable kinds are allowed to attract.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "lowercase"))]
#[value(rename_all = "lowercase")]
pub enum Puzzle3dCompatSpecificity {
    General,
    Object,
    Attraction,
    Cable,
    #[default]
    Vortex,
}

/// 🧩️ One allowed (or, unidirectional, one-way-allowed) link pair between two vortex/cable kind ids.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dKindCompatibility {
    #[dsl(refs = "vortex_kind")]
    pub source: String,
    #[dsl(refs = "vortex_kind")]
    pub target: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub bidirectional: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub important: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub specificity: Puzzle3dCompatSpecificity,
}

/// 🏷️ One freeform attribute on a catalog object-kind (compose `Attribute` analogue).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dAttribute {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub key: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub value: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

/// ✍️ One author credit on a catalog object-kind (compose `Author` analogue).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dAuthor {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub email: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}

/// 🖼️ One tagged representation/LOD URL on a catalog object-kind (compose `Representation` analogue).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dRepresentation {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub url: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub mime: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub tags: Vec<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lod: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub description: String,
}

/// 🌱️ One rim-vortex template on a `Puzzle3dCatalogObjectKind` — compose connector analogue with
/// `point`/`direction`/`t`/`mandatory`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dCatalogVortexTemplate {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub description: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub icon: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(refs = "vortex_kind")]
    pub vortex_kind: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(coord)]
    pub point: [f64; 3],
    #[cfg_attr(test, serde(default = "puzzle3d_default_direction"))]
    #[value(default = "puzzle3d_default_direction")]
    #[dsl(dir)]
    pub direction: [f64; 3],
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<f64>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
}

fn puzzle3d_default_direction() -> [f64; 3] {
    [0.0, 0.0, 1.0]
}

impl Default for Puzzle3dCatalogVortexTemplate {
    fn default() -> Self {
        Self { id: String::new(), name: String::new(), label: String::new(), description: String::new(), icon: String::new(), vortex_kind: None, point: [0.0, 0.0, 0.0], direction: puzzle3d_default_direction(), t: None, mandatory: None, radius: None }
    }
}

/// 🧱️ One object-kind catalog row — type-like (compose `Type` analogue).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dCatalogObjectKind {
    #[dsl(defines = "object_kind")]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub description: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub icon: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub image: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub unit: String,
    #[cfg_attr(test, serde(default, rename = "abstract"))]
    #[value(default, rename = "abstract")]
    pub is_abstract: bool,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub base_kinds: Vec<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub representations: Vec<Puzzle3dRepresentation>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub vortices: Vec<Puzzle3dCatalogVortexTemplate>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub attributes: Vec<Puzzle3dAttribute>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub authors: Vec<Puzzle3dAuthor>,
}

/// 🔘️ One vortex-kind catalog row — port-like (compose `Port` analogue).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dCatalogVortexKind {
    #[dsl(defines = "vortex_kind")]
    pub id: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub compatible_with: Vec<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub description: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub icon: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub color: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(refs = "cable_kind")]
    pub default_cable_kind: String,
}

/// 🧵️ One cable-kind catalog row (mirrors `CableKindCatalog`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dCatalogCableKind {
    #[dsl(defines = "cable_kind")]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(refs = "attraction_kind")]
    pub default_attraction_kind: String,
}

/// 🔗️ One attraction-kind catalog row.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dCatalogAttractionKind {
    #[dsl(defines = "attraction_kind")]
    pub id: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub name: String,
}

/// 🗂️ The compile-time-catalog side of a self-contained scene_snapshot export: object/vortex/cable/
/// attraction kind rows — see `puzzle/3d/manifest/*.manifest.json` for the same schema at the
/// manifest layer.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dKindCatalogs {
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub objects: Vec<Puzzle3dCatalogObjectKind>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub vortices: Vec<Puzzle3dCatalogVortexKind>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub cables: Vec<Puzzle3dCatalogCableKind>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub attractions: Vec<Puzzle3dCatalogAttractionKind>,
}

/// 🗂️ EngineSceneSnapshot-carried metadata: the explicit link-compatibility table plus the object/vortex/cable/
/// attraction kind catalog bundle (typed — see `Puzzle3dKindCatalogs`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dMeta {
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind_catalogs: Option<Puzzle3dKindCatalogs>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    #[dsl(table)]
    pub kind_compatibility: Vec<Puzzle3dKindCompatibility>,
}

/// ✖️ The Hamilton product `a · b` of two `[x, y, z, w]` quaternions — `a` applied after `b`.
pub fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]]
}

/// 🧭️ The `[x, y, z, w]` quaternion turning `angle` radians about the axis `(ax, ay, az)`; the identity
/// for an axis shorter than `1e-8`.
pub fn quat_from_axis_angle(ax: f64, ay: f64, az: f64, angle: f64) -> [f64; 4] {
    let len = (ax * ax + ay * ay + az * az).sqrt();
    if len < 1e-8 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let half = angle * 0.5;
    let s = half.sin();
    [ax / len * s, ay / len * s, az / len * s, half.cos()]
}

/// 📏️ A pose scale multiplied per axis by `factors`, always as a per-axis triple: a uniform scalar
/// broadcasts first and an absent scale reads as `[1, 1, 1]`.
pub fn puzzle3d_scaled(scale: Option<crate::standards::v1::subsets::any::schema::Puzzle3dScale>, factors: [f64; 3]) -> crate::standards::v1::subsets::any::schema::Puzzle3dScale {
    let current = match scale {
        Some(crate::standards::v1::subsets::any::schema::Puzzle3dScale::Uniform(value)) => [value; 3],
        Some(crate::standards::v1::subsets::any::schema::Puzzle3dScale::Vec3(value)) => value,
        None => [1.0; 3],
    };
    crate::standards::v1::subsets::any::schema::Puzzle3dScale::Vec3([current[0] * factors[0], current[1] * factors[1], current[2] * factors[2]])
}

/// ⭕️ The quaternion of no rotation, `[x, y, z, w]` — what a record without an orientation stands at.
pub const PUZZLE3D_IDENTITY_QUATERNION: [f64; 4] = [0.0, 0.0, 0.0, 1.0];

/// 🪡️ Below this cross length an attracted vortex counts as (anti)parallel to its attracting one — the compose
/// kernel's own alignment tolerance.
const PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE: f64 = 0.01;

/// 🧭️ The pose fields a selection transform may change on one object — its world origin, orientation and scale.
#[derive(Clone, Debug, PartialEq)]
pub struct Puzzle3dPose {
    pub origin: [f64; 3],
    pub orientation: Option<[f64; 4]>,
    pub scale: Option<crate::standards::v1::subsets::any::schema::Puzzle3dScale>,
}

/// 🔗️ The full id an attraction endpoint names one vortex by: `<objectId>:<vortexId>`, or the vortex id itself
/// when it already is a full id.
pub fn puzzle3d_vortex_full_id(object_id: &str, vortex_id: &str) -> String {
    if vortex_id.contains(':') {
        vortex_id.to_string()
    } else {
        format!("{object_id}:{vortex_id}")
    }
}

/// ➖️ `a − b`.
pub fn vec3_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// ➕️ `a + b`.
pub fn vec3_add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// ✳️ `a · s`.
pub fn vec3_scale(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

/// ❎️ The cross product `a × b`.
pub fn vec3_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// ⚫️ The dot product `a · b`.
pub fn vec3_dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// 🦯️ The Euclidean length of `a`.
pub fn vec3_len(a: [f64; 3]) -> f64 {
    vec3_dot(a, a).sqrt()
}

/// 🎱️ `a` scaled to unit length; a vector shorter than `1e-12` stays as it is.
pub fn vec3_normalize(a: [f64; 3]) -> [f64; 3] {
    let len = vec3_len(a);
    if len < 1e-12 {
        a
    } else {
        vec3_scale(a, 1.0 / len)
    }
}

/// 🔁️ Degrees to radians.
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

/// 🔂️ Radians to degrees.
pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}

/// 🪞️ The conjugate of `q` — its inverse for a unit quaternion.
pub fn quat_conjugate(q: [f64; 4]) -> [f64; 4] {
    [-q[0], -q[1], -q[2], q[3]]
}

/// 🧼️ `q` scaled to unit length; a degenerate quaternion reads as the identity.
pub fn quat_normalize(q: [f64; 4]) -> [f64; 4] {
    let len = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    if len < 1e-12 {
        PUZZLE3D_IDENTITY_QUATERNION
    } else {
        [q[0] / len, q[1] / len, q[2] / len, q[3] / len]
    }
}

/// 🌐️ `vector` turned by the unit quaternion `quat` (`q · v · q⁻¹`).
pub fn quat_rotate_vector(quat: [f64; 4], vector: [f64; 3]) -> [f64; 3] {
    let [x, y, z, w] = quat;
    let vx = vector[0];
    let vy = vector[1];
    let vz = vector[2];
    let ix = w * vx + y * vz - z * vy;
    let iy = w * vy + z * vx - x * vz;
    let iz = w * vz + x * vy - y * vx;
    let iw = -x * vx - y * vy - z * vz;
    [ix * w + iw * -x + iy * -z - iz * -y, iy * w + iw * -y + iz * -x - ix * -z, iz * w + iw * -z + ix * -y - iy * -x]
}

/// 🎏️ The quaternion rotating unit vector `from` onto unit vector `to`.
pub fn puzzle3d_quaternion_from_unit_vectors(from: [f64; 3], to: [f64; 3]) -> [f64; 4] {
    let r = vec3_dot(from, to) + 1.0;
    let quat = if r < 0.000_001 {
        if from[0].abs() > from[2].abs() {
            [-from[1], from[0], 0.0, 0.0]
        } else {
            [0.0, -from[2], from[1], 0.0]
        }
    } else {
        let c = vec3_cross(from, to);
        [c[0], c[1], c[2], r]
    };
    quat_normalize(quat)
}

/// 🧲️ The align-quaternion special case for when the attracted vortex is already (anti)parallel to the
/// attracting vortex. Falls back to an alternate cross axis when the attracting direction is exactly
/// ±Z — a double-degenerate corner the compose kernel's own branch doesn't otherwise guard.
pub fn puzzle3d_attraction_align_quat(parent_dir: [f64; 3], child_dir: [f64; 3]) -> [f64; 4] {
    let reverse_child = vec3_scale(child_dir, -1.0);
    let cross_vec = vec3_cross(parent_dir, reverse_child);
    if vec3_len(cross_vec) < PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE {
        if parent_dir[2].abs() < PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE {
            puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], [0.0, 0.0, -1.0])
        } else {
            let mut axis = vec3_cross([0.0, 0.0, 1.0], parent_dir);
            if vec3_len(axis) < 1e-9 {
                axis = vec3_cross([1.0, 0.0, 0.0], parent_dir);
            }
            let axis = vec3_normalize(axis);
            let half = std::f64::consts::FRAC_PI_2;
            quat_normalize([axis[0] * half.sin(), axis[1] * half.sin(), axis[2] * half.sin(), half.cos()])
        }
    } else {
        puzzle3d_quaternion_from_unit_vectors(reverse_child, parent_dir)
    }
}

/// 📐️ Forward attraction placement — given the attracting object's world pose (`t_a`/`q_a`), both
/// vortices' LOCAL position/direction, and the 6 connection-style parameters (angles in degrees),
/// returns the attracted object's world pose.
#[allow(clippy::too_many_arguments)]
pub fn puzzle3d_attraction_child_pose(t_a: [f64; 3], q_a: [f64; 4], p_a: [f64; 3], d_a: [f64; 3], p_b: [f64; 3], d_b: [f64; 3], gap: f64, shift: f64, rise: f64, rotation_deg: f64, turn_deg: f64, tilt_deg: f64) -> ([f64; 3], [f64; 4]) {
    let parent_dir = vec3_normalize(d_a);
    let child_dir = vec3_normalize(d_b);
    let align_q = puzzle3d_attraction_align_quat(parent_dir, child_dir);

    let pq = puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], parent_dir);
    let gap_dir = quat_rotate_vector(pq, [0.0, 1.0, 0.0]);
    let shift_dir = quat_rotate_vector(pq, [1.0, 0.0, 0.0]);
    let raise_dir = quat_rotate_vector(pq, [0.0, 0.0, 1.0]);

    let rotate_q = quat_from_axis_angle(parent_dir[0], parent_dir[1], parent_dir[2], -deg_to_rad(rotation_deg));
    let turn_axis = quat_rotate_vector(rotate_q, raise_dir);
    let tilt_axis = quat_rotate_vector(rotate_q, shift_dir);
    let turn_q = quat_from_axis_angle(turn_axis[0], turn_axis[1], turn_axis[2], deg_to_rad(turn_deg));
    let tilt_q = quat_from_axis_angle(tilt_axis[0], tilt_axis[1], tilt_axis[2], deg_to_rad(tilt_deg));

    let mut orientation_local = quat_conjugate(align_q);
    orientation_local = quat_mul(orientation_local, quat_conjugate(rotate_q));
    orientation_local = quat_mul(orientation_local, quat_conjugate(turn_q));
    orientation_local = quat_mul(orientation_local, quat_conjugate(tilt_q));
    let orientation_local = quat_normalize(orientation_local);

    let offset = vec3_add(vec3_add(t_a, p_a), vec3_add(vec3_add(vec3_scale(gap_dir, gap), vec3_scale(shift_dir, shift)), vec3_scale(raise_dir, rise)));
    let t_b = vec3_sub(quat_rotate_vector(orientation_local, offset), p_b);
    let q_b = quat_normalize(quat_mul(orientation_local, q_a));
    (t_b, q_b)
}

/// 🔙️ Inverse of `puzzle3d_attraction_child_pose` — given the attracted object's CURRENT world pose,
/// derives the 6 parameters that reproduce it exactly, so moving/rotating an attracted object never
/// causes a resolve-triggered snap-back and creating an attraction never moves either endpoint.
#[allow(clippy::too_many_arguments)]
pub fn derive_attraction_params(t_a: [f64; 3], q_a: [f64; 4], p_a: [f64; 3], d_a: [f64; 3], p_b: [f64; 3], d_b: [f64; 3], t_b: [f64; 3], q_b: [f64; 4]) -> (f64, f64, f64, f64, f64, f64) {
    let parent_dir = vec3_normalize(d_a);
    let child_dir = vec3_normalize(d_b);
    let align_q = puzzle3d_attraction_align_quat(parent_dir, child_dir);
    let pq = puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], parent_dir);
    let gap_dir = quat_rotate_vector(pq, [0.0, 1.0, 0.0]);
    let shift_dir = quat_rotate_vector(pq, [1.0, 0.0, 0.0]);
    let raise_dir = quat_rotate_vector(pq, [0.0, 0.0, 1.0]);

    let orientation_local = quat_normalize(quat_mul(q_b, quat_conjugate(q_a)));

    let offset = quat_rotate_vector(quat_conjugate(orientation_local), vec3_add(t_b, p_b));
    let diff = vec3_sub(vec3_sub(offset, t_a), p_a);
    let gap = vec3_dot(diff, gap_dir);
    let shift = vec3_dot(diff, shift_dir);
    let rise = vec3_dot(diff, raise_dir);

    let residual = quat_mul(align_q, orientation_local);
    let m = quat_mul(quat_mul(quat_conjugate(pq), residual), pq);
    let col_x = quat_rotate_vector(m, [1.0, 0.0, 0.0]);
    let col_y = quat_rotate_vector(m, [0.0, 1.0, 0.0]);

    let clamp = |v: f64| v.clamp(-1.0, 1.0);
    let tilt_rad = -(clamp(col_y[2])).asin();
    let (rotation_rad, turn_rad) = if (col_y[2].abs() - 1.0).abs() < 1e-6 {
        (col_x[1].atan2(col_x[0]), 0.0)
    } else {
        let col_z = quat_rotate_vector(m, [0.0, 0.0, 1.0]);
        ((-col_x[2]).atan2(col_z[2]), col_y[0].atan2(col_y[1]))
    };

    (gap, shift, rise, rad_to_deg(rotation_rad), rad_to_deg(turn_rad), rad_to_deg(tilt_rad))
}
