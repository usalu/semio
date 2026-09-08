
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
            objects: vec![ObjectKind {
                id: "Host".to_string(),
                representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/test/host.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                scale: None,
                vortices: vec![],
            }],
            vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None, ..Default::default() }],
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
/// prefix-stability laws in the app's own precompute session tests.
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
