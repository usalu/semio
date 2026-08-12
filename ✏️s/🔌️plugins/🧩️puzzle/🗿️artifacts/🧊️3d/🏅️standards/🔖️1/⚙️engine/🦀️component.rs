//! ⚙️ Puzzle 3d artifact — headless compute over a puzzle-3d scene: the domain model the whole engine
//! speaks (`Fixture`/`SceneConfig` and their object/vortex/attraction/target-volume records, the
//! kind catalogs, the brush host rules and distribution weights, the brush/fill readouts), plus the
//! plugin `setup:` hook.
//!
//! 📚️ Sibling topic files: `🦀️geometry.rs` (the `nalgebra`/`parry3d` adapter, the vector/quaternion
//! math, the placement pose solver and the collision/AABB primitives), `🦀️brush.rs` (the
//! compatibility rulebook, candidate ranking/weighting and one accepted placement's splice into a
//! fixture), `🦀️fill.rs` (the running fill plan's state), `🦀️session.rs` (the two precompute lanes,
//! the `Puzzle3dCollision` they drive and the typed `Puzzle3dEngineCommand` dispatch envelope).
//!
//! 🧭️ Placement rule for helpers reaching across nodes: a helper with exactly ONE consumer lives in
//! that consumer's file; two or more consumers put it here. Helpers taking an app-only view-state
//! type (`Puzzle3dConfig`, `Puzzle3dScene`) never come here — artifacts must not depend on apps.

use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Reexports
// 🧩️ The sibling topic modules are declared (with their `#[path]`s) in the plugin-root `📦️glue.rs`,
// beside every other taxonomy component; these re-exports keep the whole engine surface reachable
// under one `crate::artifacts::puzzle3d::engine::…` name regardless of which topic file owns it.
pub use crate::artifacts::puzzle3d::engine::brush::apply_brush_placement_to_fixture;
pub use crate::artifacts::puzzle3d::engine::session::{Puzzle3dEngineCommand, Puzzle3dEngineOutcome, Puzzle3dPrecomputeSession};
pub(crate) use crate::artifacts::puzzle3d::engine::geometry::{collision_body_from_buffers, CollisionBody};
//#endregion 🔖️Reexports

//#region 🔖️Constants
pub(crate) const FILL_COUNT_MAX: usize = 1000;

pub(crate) type Quat = [f64; 4];
pub(crate) type Vec3 = [f64; 3];
//#endregion 🔖️Constants

//#region 🔖️Model
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BrushHostRules {
    #[serde(default)]
    pub(crate) reject_capital_on_tambour: bool,
    #[serde(default)]
    pub(crate) reject_last_single_storey_on_mid_tambour: bool,
    #[serde(default)]
    pub(crate) door_tambour_requires_door_capsule: bool,
    #[serde(default = "default_door_capsule_min_abs_x")]
    pub(crate) door_capsule_min_abs_x: f64,
    #[serde(default = "default_door_capsule_max_abs_y")]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BrushKindWeights {
    #[serde(default)]
    pub(crate) object_weights: std::collections::BTreeMap<String, f64>,
    #[serde(default)]
    pub(crate) vortex_weights: std::collections::BTreeMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct KindCompatEntry {
    pub(crate) source: String,
    pub(crate) target: String,
    #[serde(default)]
    pub(crate) bidirectional: bool,
    #[serde(default)]
    pub(crate) important: bool,
    pub(crate) specificity: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct ObjectKindVortexTemplate {
    #[serde(default)]
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) label: String,
    #[serde(default)]
    pub(crate) description: String,
    #[serde(default)]
    pub(crate) icon: String,
    #[serde(rename = "vortexKind", default)]
    pub(crate) vortex_kind: Option<String>,
    #[serde(default)]
    pub(crate) point: Vec3,
    #[serde(default)]
    pub(crate) direction: Option<Vec3>,
    #[serde(default)]
    pub(crate) t: Option<f64>,
    #[serde(default)]
    pub(crate) mandatory: Option<bool>,
    #[serde(default)]
    pub(crate) radius: Option<f64>,
}

impl Default for ObjectKindVortexTemplate {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            label: String::new(),
            description: String::new(),
            icon: String::new(),
            vortex_kind: None,
            point: [0.0, 0.0, 0.0],
            direction: None,
            t: None,
            mandatory: None,
            radius: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
pub struct ObjectKindRepresentation {
    #[serde(default)]
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) url: String,
    #[serde(default)]
    pub(crate) mime: String,
    #[serde(default)]
    pub(crate) tags: Vec<String>,
    #[serde(default)]
    pub(crate) lod: Option<String>,
    #[serde(default)]
    pub(crate) description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
pub struct ObjectKind {
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) representations: Vec<ObjectKindRepresentation>,
    #[serde(default)]
    pub(crate) scale: Option<dsl::DslValue>,
    #[serde(default)]
    pub(crate) vortices: Vec<ObjectKindVortexTemplate>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
pub struct VortexKindCatalog {
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) code: Option<String>,
    #[serde(default)]
    pub(crate) label: Option<String>,
    #[serde(default)]
    pub(crate) order: Option<i32>,
    #[serde(default, rename = "compatibleWith")]
    pub(crate) compatible_with: Vec<String>,
    #[serde(default)]
    pub(crate) description: String,
    #[serde(default)]
    pub(crate) icon: String,
    #[serde(default)]
    pub(crate) color: String,
    #[serde(rename = "defaultCableKind", default)]
    pub(crate) default_cable_kind: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
pub struct CableKindCatalog {
    pub(crate) id: String,
    #[serde(rename = "defaultAttractionKind", default)]
    pub(crate) default_attraction_kind: Option<String>,
}

/// 🗂️ The compile-time-catalog side of a scene: object/vortex/cable kind rows, reachable through
/// `apply_brush_placement_to_fixture`'s public signature.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
pub struct KindCatalogBundle {
    #[serde(default)]
    pub(crate) objects: Vec<ObjectKind>,
    #[serde(default)]
    pub(crate) vortices: Vec<VortexKindCatalog>,
    #[serde(default)]
    pub(crate) cables: Vec<CableKindCatalog>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct VortexProps {
    pub id: String,
    #[serde(rename = "vortexKind", default)]
    pub vortex_kind: Option<String>,
    pub position: Vec3,
    pub direction: Option<Vec3>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct FixtureObject {
    pub id: String,
    #[serde(rename = "objectKind", default)]
    pub object_kind: Option<String>,
    #[serde(default)]
    pub anchor: crate::artifacts::puzzle3d::Puzzle3dObjectAnchor,
    #[serde(rename = "meshUrl", default)]
    pub mesh_url: Option<String>,
    pub origin: Vec3,
    pub orientation: Option<Quat>,
    pub scale: Option<dsl::DslValue>,
    #[serde(default)]
    pub vortices: Vec<VortexProps>,
    /// 🪣️ Live-viewport-only tag (never persisted to the document): this object's 0-based position in
    /// the fill plan's sequence, so the viewport can reveal/hide planned pieces by drag position without
    /// a WASM round trip. Set only on `compose_fill_display`'s output, stripped from committed fixtures.
    #[serde(rename = "revealIndex", default, skip_serializing_if = "Option::is_none")]
    pub reveal_index: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct AttractionProps {
    #[serde(default)]
    pub id: String,
    pub attracting: String,
    pub attracted: String,
    #[serde(default)]
    pub gap: f64,
    #[serde(default)]
    pub shift: f64,
    #[serde(default)]
    pub rise: f64,
    #[serde(default)]
    pub rotation: f64,
    #[serde(default)]
    pub turn: f64,
    #[serde(default)]
    pub tilt: f64,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WorldVolumeProps {
    pub id: String,
    pub origin: Vec3,
    #[serde(default)]
    pub orientation: Option<Quat>,
    #[serde(default)]
    pub scale: Option<dsl::DslValue>,
}

/// 🏗️ A puzzle-3d scene's object/attraction/target-volume state, reachable through
/// `apply_brush_placement_to_fixture`'s public signature.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct Fixture {
    #[serde(default)]
    pub attractions: Vec<AttractionProps>,
    #[serde(default)]
    pub objects: Vec<FixtureObject>,
    #[serde(default, rename = "targetVolumes")]
    pub target_volumes: Vec<WorldVolumeProps>,
}

/// 📨️ The full typed payload `Puzzle3dEngineCommand::SetScene` carries — the exact same shape
/// `Puzzle3dCollision::set_scene`'s JSON payload has always deserialized into, just reused directly
/// instead of re-declared, so the command enum's field IS this type, not a mirror of it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct SceneConfig {
    pub(crate) fixture: Fixture,
    #[serde(rename = "kindCatalogs", default)]
    pub(crate) kind_catalogs: Option<KindCatalogBundle>,
    #[serde(rename = "kindCompatibility", default)]
    pub(crate) kind_compatibility: Vec<KindCompatEntry>,
    #[serde(rename = "overlapBudget", default)]
    pub(crate) overlap_budget: f64,
    #[serde(default)]
    pub(crate) seed: u32,
    #[serde(rename = "hostRules", default)]
    pub(crate) host_rules: BrushHostRules,
    #[serde(default)]
    pub(crate) weights: BrushKindWeights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrushCompatibleCandidate {
    pub object_kind_id: String,
    pub source_vortex_index: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrushPreviewState {
    pub target_vortex_full_id: String,
    pub object_kind_id: String,
    pub source_vortex_index: usize,
    pub mesh_url: String,
    pub origin: Vec3,
    pub orientation: Quat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<dsl::DslValue>,
}

/// 🎯️ Public so `Puzzle3dEngineOutcome::BrushCandidates` can hand this back to callers (the app's
/// brush slot) as a typed value instead of the JSON string the old `brush_candidates` wasm-bindgen
/// method returned.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrushCollisionFreeResult {
    pub free: Vec<BrushCompatibleCandidate>,
    pub unknown_pending: bool,
    #[serde(default)]
    pub resume_candidate_index: usize,
}

/// 🚦️ Which background precompute lane a tick should advance — fill and brush never share one FIFO queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrecomputeLane {
    Brush = 0,
    Fill = 1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BrushPlacePayload {
    pub target_vortex_full_id: String,
    pub object_kind_id: String,
    pub source_vortex_index: usize,
    pub origin: Vec3,
    pub orientation: Quat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<dsl::DslValue>,
}

/// 🎯️ A suggestion-popup preview accepted as-is becomes a placement at the exact same pose — the one
/// field `BrushPreviewState` carries that `BrushPlacePayload` doesn't (`mesh_url`, resolvable again
/// from `object_kind_id` via the kind catalog) is simply dropped.
impl From<BrushPreviewState> for BrushPlacePayload {
    fn from(preview: BrushPreviewState) -> Self {
        Self { target_vortex_full_id: preview.target_vortex_full_id, object_kind_id: preview.object_kind_id, source_vortex_index: preview.source_vortex_index, origin: preview.origin, orientation: preview.orientation, scale: preview.scale }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillBuildProgress {
    pub(crate) count: usize,
    pub(crate) applied_count: usize,
    pub(crate) max_count: usize,
    pub(crate) done: bool,
    #[serde(default)]
    pub(crate) appended_objects: Vec<FixtureObject>,
    #[serde(default)]
    pub(crate) appended_attractions: Vec<AttractionProps>,
    #[serde(default)]
    pub(crate) sequence: Vec<BrushPlacePayload>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillProgressSummary {
    pub count: usize,
    pub applied_count: usize,
    pub max_count: usize,
    pub done: bool,
}

/// 🪪️ `objectId:vortexId`, unless the vortex id already carries its owner's prefix.
pub(crate) fn puzzle3d_vortex_full_id(object_id: &str, vortex_id: &str) -> String {
    if vortex_id.contains(':') {
        vortex_id.to_string()
    } else {
        format!("{object_id}:{vortex_id}")
    }
}
//#endregion 🔖️Model

//#region 🔖️DocumentHelpers
pub fn empty_puzzle3d_snapshot() -> Puzzle3dSnapshot {
    Puzzle3dSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Testkit
/// 🧪️ The one puzzle3d-engine test harness — every sibling topic file's `🧪️Tests` region builds on it
/// instead of re-deriving a mesh-buffer/scene/fill-plan scaffold of its own.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;

    pub(crate) const DEFAULT_OVERLAP_BUDGET: f64 = 0.02;

    pub(crate) fn unit_cube_mesh_buffers() -> (Vec<f32>, Vec<u32>) {
        (
            vec![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0],
            vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2],
        )
    }

    /// 🧊️ Same box as `unit_cube_mesh_buffers` but with outward-facing (CCW-from-outside) winding, needed
    /// for tests that rely on `CollisionShape::contains_point` actually reporting interior points as inside.
    pub(crate) fn outward_wound_unit_cube_mesh_buffers() -> (Vec<f32>, Vec<u32>) {
        (
            vec![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0],
            vec![0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 0, 5, 4, 0, 1, 5, 2, 7, 6, 2, 3, 7, 0, 7, 3, 0, 4, 7, 1, 6, 5, 1, 2, 6],
        )
    }

    /// 🏗️ One `Host` object with a single free `port-a` vortex — the smallest scene that still schedules
    /// both precompute lanes.
    pub(crate) fn single_object_scene_json() -> String {
        let scene = SceneConfig {
            fixture: Fixture {
                attractions: vec![],
                target_volumes: vec![],
                objects: vec![FixtureObject {
                    id: "host".to_string(),
                    object_kind: Some("Host".to_string()),
                    anchor: Default::default(),
                    mesh_url: Some("/test/host.glb".to_string()),
                    origin: [0.0, 0.0, 0.0],
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                    vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
                    reveal_index: None,
                }],
            },
            kind_catalogs: Some(KindCatalogBundle {
                objects: vec![ObjectKind { id: "Host".to_string(), representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/test/host.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }], scale: None, vortices: vec![] }],
                vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None , ..Default::default() }],
                cables: vec![],
            }),
            kind_compatibility: vec![],
            overlap_budget: DEFAULT_OVERLAP_BUDGET,
            seed: 1,
            host_rules: BrushHostRules::default(),
            weights: BrushKindWeights::default(),
        };
        serde_json::to_string(&scene).unwrap()
    }

    /// 🪣️ One synthetic already-planned fill object / attraction / placement payload, for the fill-plan
    /// prefix-stability laws in `🦀️session.rs`.
    pub(crate) fn fill_plan_object(id: &str) -> FixtureObject {
        FixtureObject {
            id: id.to_string(),
            object_kind: Some("Placed".to_string()),
            anchor: Default::default(),
            mesh_url: Some("/test/placed.glb".to_string()),
            origin: [0.0, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            vortices: vec![],
            reveal_index: None,
        }
    }

    pub(crate) fn fill_plan_attraction(index: usize) -> AttractionProps {
        AttractionProps { id: format!("a{index}"), attracting: format!("p{index}:v0"), attracted: format!("p{}:v0", index + 1), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 }
    }

    pub(crate) fn fill_plan_payload(index: usize) -> BrushPlacePayload {
        BrushPlacePayload { target_vortex_full_id: format!("p{index}:v0"), object_kind_id: "Placed".to_string(), source_vortex_index: 0, origin: [index as f64, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None }
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🔗️ Keeps the example fixture's scene-authored kind catalog in sync with the compile-time
    /// `puzzle3d-default` manifest.
    #[test]
    fn concrete_forest_kind_catalog_matches_puzzle3d_default_manifest() {
        let fixture = crate::artifacts::puzzle3d::dsl::parse_dsl(crate::artifacts::puzzle3d::dsl::PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT).expect("concrete-forest example parses as dsl");
        let catalogs: KindCatalogBundle = serde_json::from_value(serde_json::to_value(&fixture.meta.kind_catalogs).unwrap()).unwrap();
        let manifest = math::graph::manifest::manifest_by_id("puzzle3d-default").expect("puzzle3d-default manifest must be registered");
        let wire_kind_ids: std::collections::BTreeSet<_> = manifest.wire_kinds.iter().map(|row| row.id.as_str()).collect();
        let edge_kind_ids: std::collections::BTreeSet<_> = manifest.edge_kinds.iter().map(|row| row.id.as_str()).collect();
        for vortex in &catalogs.vortices {
            if let Some(default_cable_kind) = &vortex.default_cable_kind {
                assert!(wire_kind_ids.contains(default_cable_kind.as_str()), "vortex kind {:?} references unknown wire kind {default_cable_kind:?}", vortex.id);
            }
        }
        for cable in &catalogs.cables {
            if let Some(default_attraction_kind) = &cable.default_attraction_kind {
                assert!(edge_kind_ids.contains(default_attraction_kind.as_str()), "cable kind {:?} references unknown edge kind {default_attraction_kind:?}", cable.id);
            }
        }
    }

    /// 🪪️ A vortex id that already carries its owner's prefix is passed through untouched.
    #[test]
    fn vortex_full_id_prefixes_only_bare_ids() {
        assert_eq!(puzzle3d_vortex_full_id("host", "v0"), "host:v0");
        assert_eq!(puzzle3d_vortex_full_id("host", "other:v0"), "other:v0");
    }

    /// 🖐️ Compile-guard for the 🖐️5d app, which builds its own `Puzzle5dPrecomputeSession` on top of
    /// this one: every item it names must stay publicly reachable under
    /// `crate::artifacts::puzzle3d::…`. A rename or a visibility narrowing breaks this test long
    /// before it breaks 5d.
    #[test]
    fn the_5d_facing_engine_surface_stays_public() {
        use crate::artifacts::puzzle3d::engine::{BrushCollisionFreeResult, BrushPlacePayload, BrushPreviewState, FillBuildProgress, Fixture, PrecomputeLane, Puzzle3dEngineCommand, Puzzle3dEngineOutcome, Puzzle3dPrecomputeSession};
        use crate::artifacts::puzzle3d::Puzzle3dError;

        let mut session = Puzzle3dPrecomputeSession::new();
        assert!(session.set_scene("{ not json").is_err(), "set_scene surfaces a Puzzle3dError");
        session.register_mesh("/probe.glb", &[], &[]);
        assert!(!session.has_mesh("/probe.glb"));
        assert!(!session.precompute_step(1));
        let _: BrushCollisionFreeResult = session.brush_candidates("probe:v0");
        let _: Option<BrushPreviewState> = session.brush_preview("probe:v0", 0);
        let _: FillBuildProgress = session.fill_progress();
        assert!(session.precompute_step_lane(PrecomputeLane::Brush, 1) || true);
        let payload = BrushPlacePayload { target_vortex_full_id: "probe:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        let rejected: Result<Puzzle3dEngineOutcome, Puzzle3dError> = session.dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload });
        assert!(matches!(rejected, Err(Puzzle3dError::BrushPlacementRejected)));
        assert!(session.dispatch(Puzzle3dEngineCommand::ApplyFillCount { count: 0 }).is_err());
        let _: fn(&Fixture, &BrushPlacePayload, &KindCatalogBundle) -> Fixture = apply_brush_placement_to_fixture;
    }

    #[test]
    fn brush_preview_state_converts_into_a_placement_payload() {
        let preview = BrushPreviewState {
            target_vortex_full_id: "host:v0".into(),
            object_kind_id: "Kind".into(),
            source_vortex_index: 2,
            mesh_url: "/mesh.glb".into(),
            origin: [1.0, 2.0, 3.0],
            orientation: [0.0, 0.0, 0.0, 1.0],
            scale: Some(dsl::DslValue::Number(2.0)),
        };
        let payload = BrushPlacePayload::from(preview);
        assert_eq!(payload.target_vortex_full_id, "host:v0");
        assert_eq!(payload.object_kind_id, "Kind");
        assert_eq!(payload.source_vortex_index, 2);
        assert_eq!(payload.origin, [1.0, 2.0, 3.0]);
        assert_eq!(payload.scale, Some(dsl::DslValue::Number(2.0)));
    }
}
//#endregion 🧪️Tests


/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "puzzle.puzzle3d",
        extension: Some("puzzle3d"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::puzzle3d::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::puzzle3d::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::puzzle3d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::puzzle3d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("puzzle.puzzle3d"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "puzzle.puzzle3d.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::puzzle3d::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::puzzle3d::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::puzzle3d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::puzzle3d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("puzzle.puzzle3d.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "puzzle.puzzle3d.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::puzzle3d::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::puzzle3d::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("puzzle.puzzle3d.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "3d.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::puzzle3d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::puzzle3d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("3d.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "3d.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::puzzle3d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::puzzle3d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("3d.spr"),
    });
}


//#region 🔖️ArtifactEngine
/// ⚙️ UI-independent puzzle3d artifact engine — owns the full artifact; `snapshot()` is its persisted subset.
pub struct Puzzle3dEngine {
    artifact: crate::artifacts::puzzle3d::schema::Puzzle3dArtifact,
    snapshot: crate::artifacts::puzzle3d::Puzzle3dSnapshot,
}

impl Puzzle3dEngine {
    pub fn new(snapshot: crate::artifacts::puzzle3d::Puzzle3dSnapshot) -> Self {
        let artifact = crate::artifacts::puzzle3d::schema::Puzzle3dArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    pub fn into_snapshot(self) -> crate::artifacts::puzzle3d::Puzzle3dSnapshot {
        self.snapshot
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🔖️IoFacet
/// 🔌️ Registers this artifact's `ComposerEntry` io tree plus the `"3d.puzzle"` OS-host mesh
/// export/import bridge — the latter relocated from `apps::puzzle3d::register_puzzle3d_exports`
/// (APA, ticket `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`: OS-host registration belongs to the
/// owning artifact's own engine, never to an app file). The callback bodies stay in `apps::puzzle3d`
/// next to the app-local helpers (mesh registry, fixture types) they depend on, exposed `pub(crate)`
/// for this call site only.
pub fn register_io() {
    crate::artifacts::puzzle3d::io_registry::register();
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    {
        semio_framework_os::register_mesh_exporter("3d.puzzle", "puzzle", crate::apps::puzzle3d::puzzle3d_mesh_from_document, Box::new(semio_framework_plugin::ObjExporter));
        semio_framework_os::register_mesh_exporter("3d.puzzle", "puzzle", crate::apps::puzzle3d::puzzle3d_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
        semio_framework_os::register_mesh_exporter("3d.puzzle", "puzzle", crate::apps::puzzle3d::puzzle3d_mesh_from_document, Box::new(semio_framework_plugin::StlExporter));
        semio_framework_os::register_mesh_importer("3d.puzzle", crate::apps::puzzle3d::puzzle3d_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
        semio_framework_os::register_mesh_importer("3d.puzzle", crate::apps::puzzle3d::puzzle3d_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
        semio_framework_os::register_mesh_importer("3d.puzzle", crate::apps::puzzle3d::puzzle3d_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
        semio_framework_os::register_mesh_dwg_export_handler("3d.puzzle", "puzzle", crate::apps::puzzle3d::puzzle3d_mesh_from_document);
        semio_framework_os::register_mesh_dwg_import_handler("3d.puzzle", crate::apps::puzzle3d::puzzle3d_document_from_mesh);
    }
}
//#endregion 🔖️IoFacet
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::Puzzle3dComposer as Puzzle3dAnyComposer;
    use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::Puzzle3dBuilder as Puzzle3dAnyBuilder;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const PUZZLE3D_DIALECT: Dialect = Dialect { artifact_kind: "s.puzzle3d", standard: StandardId("1"), subset: SubsetId("*") };
    const PUZZLE3D_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::puzzle3d::Puzzle3dSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == PUZZLE3D_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => Puzzle3dAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => Puzzle3dAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "Puzzle3dComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == PUZZLE3D_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::puzzle3d::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "Puzzle3dComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_LAS_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId("*") };
    fn compose_export_las(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle3d::io::export::serializers::artifacts::las::v1_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_LAS_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PLY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };
    fn compose_export_ply(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle3d::io::export::serializers::artifacts::ply::v1_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PLY_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle3d::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle3d::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    fn compose_export_dwg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle3d::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    fn compose_export_stl(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle3d::io::export::serializers::artifacts::stl::v_ascii::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_STL_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_GLTF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    fn compose_export_gltf(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle3d::io::export::serializers::artifacts::gltf::v2_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_GLTF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    fn compose_export_obj(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::puzzle3d::io::export::serializers::artifacts::obj::v3_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_OBJ_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<Puzzle3dAnyComposer>(),
            ComposerEntry { writes: EXPORT_LAS_DIALECT, reads: &[PUZZLE3D_DIALECT], compose: compose_export_las },
            ComposerEntry { writes: EXPORT_PLY_DIALECT, reads: &[PUZZLE3D_DIALECT], compose: compose_export_ply },
            ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[PUZZLE3D_DIALECT], compose: compose_export_png },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[PUZZLE3D_DIALECT], compose: compose_export_json },
            ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[PUZZLE3D_DIALECT], compose: compose_export_dwg },
            ComposerEntry { writes: EXPORT_STL_DIALECT, reads: &[PUZZLE3D_DIALECT], compose: compose_export_stl },
            ComposerEntry { writes: EXPORT_GLTF_DIALECT, reads: &[PUZZLE3D_DIALECT], compose: compose_export_gltf },
            ComposerEntry { writes: EXPORT_OBJ_DIALECT, reads: &[PUZZLE3D_DIALECT], compose: compose_export_obj },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
