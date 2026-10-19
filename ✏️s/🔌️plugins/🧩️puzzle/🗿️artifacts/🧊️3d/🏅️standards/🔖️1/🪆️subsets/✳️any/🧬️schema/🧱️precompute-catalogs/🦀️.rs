//! 🧱️ Precompute catalogs: the host rules and weights, the kind catalogs, the scene wire projection and the object, vortex and attraction props the precompute session exchanges with its host.

use super::*;

//#region 🔖️PrecomputeModel
// ⚙️➡️🧬️ Rehomed from the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
// the pure data shapes the interactive brush/fill precompute session (now `crate::editor::puzzle3d::precompute`)
// exchanges with its host — the kind catalogs, the host rules/weights, the `EngineSceneSnapshot`/`SceneConfig` wire
// projection `Puzzle3dEngineCommand::SetScene` carries, and the brush/fill readouts. An artifact is a schema
// plus an io system, never an engine — the actual stateful session lives app-side; this is its data.
pub type Quat = [f64; 4];
pub type Vec3 = [f64; 3];

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BrushHostRules {
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub reject_capital_on_tambour: bool,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub reject_last_single_storey_on_mid_tambour: bool,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub door_tambour_requires_door_capsule: bool,
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_door_capsule_min_abs_x"))]
    #[value(default = "default_door_capsule_min_abs_x")]
    pub door_capsule_min_abs_x: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default = "default_door_capsule_max_abs_y"))]
    #[value(default = "default_door_capsule_max_abs_y")]
    pub door_capsule_max_abs_y: f64,
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
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BrushKindWeights {
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub object_weights: semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap<String, f64>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub vortex_weights: semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
pub struct KindCompatEntry {
    pub source: String,
    pub target: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub bidirectional: bool,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub important: bool,
    pub specificity: Option<String>,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectKindVortexTemplate {
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub label: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub description: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub icon: String,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "vortexKind", default))]
    #[value(rename = "vortexKind", default)]
    pub vortex_kind: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub point: Vec3,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub direction: Option<Vec3>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub t: Option<f64>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub mandatory: Option<bool>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub radius: Option<f64>,
}

impl Default for ObjectKindVortexTemplate {
    fn default() -> Self {
        Self { id: String::new(), name: String::new(), label: String::new(), description: String::new(), icon: String::new(), vortex_kind: None, point: [0.0, 0.0, 0.0], direction: None, t: None, mandatory: None, radius: None }
    }
}

#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectKindRepresentation {
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub name: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub url: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub mime: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub tags: Vec<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub lod: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectKind {
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub representations: Vec<ObjectKindRepresentation>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub scale: Option<semio_framework_value::DslValue>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub vortices: Vec<ObjectKindVortexTemplate>,
}

#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
pub struct VortexKindCatalog {
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub code: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub label: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub order: Option<i32>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, rename = "compatibleWith"))]
    #[value(default, rename = "compatibleWith")]
    pub compatible_with: Vec<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub description: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub icon: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub color: String,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "defaultCableKind", default))]
    #[value(rename = "defaultCableKind", default)]
    pub default_cable_kind: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
pub struct CableKindCatalog {
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "defaultAttractionKind", default))]
    #[value(rename = "defaultAttractionKind", default)]
    pub default_attraction_kind: Option<String>,
}

/// 🗂️ The compile-time-catalog side of a scene: object/vortex/cable kind rows, reachable through
/// `apply_brush_placement_to_snapshot`'s public signature.
#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
pub struct KindCatalogBundle {
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub objects: Vec<ObjectKind>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub vortices: Vec<VortexKindCatalog>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub cables: Vec<CableKindCatalog>,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
pub struct VortexProps {
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "vortexKind", default))]
    #[value(rename = "vortexKind", default)]
    pub vortex_kind: Option<String>,
    pub position: Vec3,
    pub direction: Option<Vec3>,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
pub struct EngineSceneObject {
    pub id: String,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "objectKind", default))]
    #[value(rename = "objectKind", default)]
    pub object_kind: Option<String>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub anchor: crate::Puzzle3dObjectAnchor,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "meshUrl", default))]
    #[value(rename = "meshUrl", default)]
    pub mesh_url: Option<String>,
    pub origin: Vec3,
    pub orientation: Option<Quat>,
    pub scale: Option<semio_framework_value::DslValue>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub vortices: Vec<VortexProps>,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AttractionProps {
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub id: String,
    pub attracting: String,
    pub attracted: String,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub gap: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub shift: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub rise: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub rotation: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub turn: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub tilt: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub x: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct WorldVolumeProps {
    pub id: String,
    pub origin: Vec3,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub orientation: Option<Quat>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub scale: Option<semio_framework_value::DslValue>,
}

/// 🏗️ A puzzle-3d scene's object/attraction/target-volume state, reachable through
/// `apply_brush_placement_to_snapshot`'s public signature.
#[derive(Debug, Clone, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
pub struct EngineSceneSnapshot {
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub attractions: Vec<AttractionProps>,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub objects: Vec<EngineSceneObject>,
    #[cfg_attr(any(test, feature = "testing"), serde(default, rename = "targetVolumes"))]
    #[value(default, rename = "targetVolumes")]
    pub target_volumes: Vec<WorldVolumeProps>,
}

/// 📨️ The full typed payload `Puzzle3dEngineCommand::SetScene` carries — the exact same shape
/// `Puzzle3dCollision::set_scene`'s JSON payload has always deserialized into, just reused directly
/// instead of re-declared, so the command enum's field IS this type, not a mirror of it.
#[derive(Debug, Clone, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testing"), derive(serde::Serialize, serde::Deserialize))]
pub struct SceneConfig {
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "sceneSnapshot"))]
    #[value(rename = "sceneSnapshot")]
    pub scene_snapshot: EngineSceneSnapshot,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "kindCatalogs", default))]
    #[value(rename = "kindCatalogs", default)]
    pub kind_catalogs: Option<KindCatalogBundle>,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "kindCompatibility", default))]
    #[value(rename = "kindCompatibility", default)]
    pub kind_compatibility: Vec<KindCompatEntry>,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "contactTolerance", default))]
    #[value(rename = "contactTolerance", default)]
    pub contact_tolerance: f64,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub seed: u32,
    #[cfg_attr(any(test, feature = "testing"), serde(rename = "hostRules", default))]
    #[value(rename = "hostRules", default)]
    pub host_rules: BrushHostRules,
    #[cfg_attr(any(test, feature = "testing"), serde(default))]
    #[value(default)]
    pub weights: BrushKindWeights,
}
//#endregion 🔖️PrecomputeModel
