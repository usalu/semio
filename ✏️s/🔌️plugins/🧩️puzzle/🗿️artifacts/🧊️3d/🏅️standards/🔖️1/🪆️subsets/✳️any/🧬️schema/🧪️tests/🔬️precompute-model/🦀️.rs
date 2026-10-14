pub mod context {
    
    use super::super::*;
    
    pub const DEFAULT_OVERLAP_BUDGET: f64 = 0.02;
    
    pub fn unit_cube_mesh_buffers() -> (Vec<f32>, Vec<u32>) {
        (
            vec![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0],
            vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2],
        )
    }
    
    /// 🎲️ A `unit_cube_mesh_buffers` box nudged by `seed`, so a law needing geometry NO other law in the
    /// binary ever derived has it. The brush-mesh store is process-wide and content-addressed since wave
    /// B22 — a second id whose DIGEST is already resident adopts the resident page — so a law asserting
    /// "this process holds nothing for that geometry" must not reuse the shared cube, or it passes or fails
    /// by test order.
    pub fn seeded_cube_mesh_buffers(seed: f32) -> (Vec<f32>, Vec<u32>) {
        let (positions, indices) = unit_cube_mesh_buffers();
        (positions.iter().map(|value| value + seed).collect(), indices)
    }
    
    /// 🧊️ Same box as `unit_cube_mesh_buffers` but with outward-facing (CCW-from-outside) winding, needed
    /// for tests that rely on `CollisionShape::contains_point` actually reporting interior points as inside.
    pub fn outward_wound_unit_cube_mesh_buffers() -> (Vec<f32>, Vec<u32>) {
        (
            vec![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0],
            vec![0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 0, 5, 4, 0, 1, 5, 2, 7, 6, 2, 3, 7, 0, 7, 3, 0, 4, 7, 1, 6, 5, 1, 2, 6],
        )
    }
    
    /// 🏗️ One `Host` object with a single free `port-a` vortex — the smallest scene that still schedules
    /// both precompute lanes.
    pub fn single_object_scene_json() -> String {
        let scene = SceneConfig {
            scene_snapshot: EngineSceneSnapshot {
                attractions: vec![],
                target_volumes: vec![],
                objects: vec![EngineSceneObject {
                    id: "host".to_string(),
                    object_kind: Some("Host".to_string()),
                    anchor: Default::default(),
                    mesh_url: Some("/test/host.glb".to_string()),
                    origin: [0.0, 0.0, 0.0],
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                    vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
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
            contact_tolerance: DEFAULT_OVERLAP_BUDGET,
            seed: 1,
            host_rules: BrushHostRules::default(),
            weights: BrushKindWeights::default(),
        };
        serde_json::to_string(&scene).unwrap()
    }

}

use super::*;

/// 🪪️ A vortex id that already carries its owner's prefix is passed through untouched.
#[test]
fn vortex_full_id_prefixes_only_bare_ids() {
    use crate::puzzle3d_vortex_full_id;
    assert_eq!(puzzle3d_vortex_full_id("host", "v0"), "host:v0");
    assert_eq!(puzzle3d_vortex_full_id("host", "other:v0"), "other:v0");
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
        scale: Some(semio_framework_value::DslValue::float(2.0)),
    };
    let payload = BrushPlacePayload::from(preview);
    assert_eq!(payload.target_vortex_full_id, "host:v0");
    assert_eq!(payload.object_kind_id, "Kind");
    assert_eq!(payload.source_vortex_index, 2);
    assert_eq!(payload.origin, [1.0, 2.0, 3.0]);
    assert_eq!(payload.scale, Some(semio_framework_value::DslValue::float(2.0)));
}
