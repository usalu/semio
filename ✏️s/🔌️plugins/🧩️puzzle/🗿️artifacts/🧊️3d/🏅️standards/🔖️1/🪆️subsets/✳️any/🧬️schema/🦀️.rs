//! 🧬️ Puzzle3d artifact schema — every field of the artifact with its state class.

use crate::{Puzzle3dSnapshot};
use ::semio_framework_schema::ArtifactSchema;
//#region 🔖️Artifact
/// 🧬️ puzzle3d document artifact state.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
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

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct CableKindCatalog {
    pub(crate) id: String,
    #[cfg_attr(test, serde(rename = "defaultAttractionKind", default))]
    #[value(rename = "defaultAttractionKind", default)]
    pub(crate) default_attraction_kind: Option<String>,
}

/// 🗂️ The compile-time-catalog side of a scene: object/vortex/cable kind rows, reachable through
/// `apply_brush_placement_to_snapshot`'s public signature.
#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct VortexProps {
    pub id: String,
    #[cfg_attr(test, serde(rename = "vortexKind", default))]
    #[value(rename = "vortexKind", default)]
    pub vortex_kind: Option<String>,
    pub position: Vec3,
    pub direction: Option<Vec3>,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct EngineSceneObject {
    pub id: String,
    #[cfg_attr(test, serde(rename = "objectKind", default))]
    #[value(rename = "objectKind", default)]
    pub object_kind: Option<String>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub anchor: crate::Puzzle3dObjectAnchor,
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

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

#[derive(Debug, Clone, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrushCompatibleCandidate {
    pub object_kind_id: String,
    pub source_vortex_index: usize,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
#[derive(Debug, Clone, value_derive::ToValue, value_derive::FromValue)]
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

/// 📸️ Resume point a fill run job reports through `StepOutcome::CheckpointReady`: fixed 68-byte
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
/// `ApplyBrushPlacement` mirrors `apply_brush_placement_json`'s `BrushPlacePayload` body,
/// `UpdateKindWeights` mirrors `update_kind_weights`'s two JSON map bodies.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum Puzzle3dEngineCommand {
    #[dsl(key = "set-scene")]
    SetScene { scene: SceneConfig },
    #[dsl(key = "apply-brush-placement")]
    ApplyBrushPlacement { payload: BrushPlacePayload },
    #[dsl(key = "update-kind-weights")]
    UpdateKindWeights { object_weights: std::collections::BTreeMap<String, f64>, vortex_weights: std::collections::BTreeMap<String, f64> },
    #[dsl(key = "brush-preview")]
    BrushPreview { vortex_full_id: String, candidate_index: u32 },
}
//#region 🔖️HandcraftedOpCodecs



//#endregion 🔖️HandcraftedOpCodecs

/// 📬️ What `dispatch` hands back — the typed counterpart of what each old JSON-string method
/// returned (a `EngineSceneSnapshot` JSON string, a `BrushPreviewState` JSON string, or nothing). Plain Rust, no
/// DSL/wasm-bindgen requirement — this only ever crosses the artifact <-> app boundary in-process.
#[derive(Debug, Clone, PartialEq)]
pub enum Puzzle3dEngineOutcome {
    Unit,
    EngineSceneSnapshot(EngineSceneSnapshot),
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

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::Puzzle3dMeta;
pub use crate::Puzzle3dObject;
pub use crate::Puzzle3dAttraction;
pub use crate::Puzzle3dTargetVolume;
pub use crate::Puzzle3dReference;
//#endregion 🔁️Re-exports
