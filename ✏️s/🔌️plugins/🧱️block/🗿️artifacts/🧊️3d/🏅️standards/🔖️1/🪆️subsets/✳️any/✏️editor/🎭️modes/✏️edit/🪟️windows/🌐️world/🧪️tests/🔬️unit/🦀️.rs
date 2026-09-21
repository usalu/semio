
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_world_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, BLOCK3D_BODY_WORLD);
    assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
    assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
}

/// 🎯️ ticket 26/09/19 (play grid, visual audit class 1): the `block3d` pane booted ready with the
/// curated `hexagonal-cut-concrete-forest-left` example selected and showed only the camera gizmo and
/// the rim-vortex markers — the representation GLB (an ~11 m × 4.7 m slab reaching away from the
/// origin) was loaded and served (200, `model/gltf-binary`) but sat outside the frustum of the
/// camera the example authored around the 0.36 m rim ring. A world window that publishes no `fit`
/// lane can be framed by nothing but that authored camera, so the law is: this window always stages
/// the one-shot fit, and it stages it for a document that really does carry the mesh.
#[semio_framework_async_macros::async_test]
async fn world_scene_stages_a_one_shot_fit_for_the_booted_example() {
    let definition = crate::standards::v1::subsets::any::schema::snapshot::text::block3d_boot_snapshot();
    assert!(!definition.representations.is_empty(), "the booted example must carry the representation whose mesh this window frames");
    let node = render(&definition, &Block3dConfig::default(), "block3d-window-1", "select", None).expect("the booted example assembles its world surface");
    let scene: World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("the world surface decodes as a 3d scene");
    for representation in &definition.representations {
        if let Some(url) = representation.mesh_url.as_deref() {
            assert!(scene.meshes_json.contains(url), "every representation's mesh must reach the scene — {url}");
        }
    }
    let fit = scene.fit_json.as_deref().expect("the world window must publish a fit lane");
    assert!(fit.contains("\"enabled\":true"), "the fit lane must be enabled, else the authored camera is the only framing: {fit}");
    assert!(fit.contains(&format!("\"revision\":{}", crate::block3d_world_fit_revision(&definition))), "the lane must carry this document's own identity revision: {fit}");
    assert!(!fit.contains("boundsMin"), "block3d knows mesh URLs, never mesh extents — the host measures what it loaded");
}

/// 🎯️ The fit revision is a document IDENTITY, not a geometry digest: switching example refits,
/// dragging a vortex must not yank the camera out from under the hand that is dragging it.
#[semio_framework_async_macros::async_test]
async fn fit_revision_tracks_document_identity_not_vortex_edits() {
    use crate::standards::v1::subsets::any::schema::snapshot::text::{parse_dsl, BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT, BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT};
    let forest = parse_dsl(BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).expect("the concrete forest example parses");
    let capsule = parse_dsl(BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).expect("the nakagin capsule example parses");
    assert_ne!(crate::block3d_world_fit_revision(&forest), crate::block3d_world_fit_revision(&capsule), "two examples with different kinds and meshes must each get their own framing");

    let mut moved = forest.clone();
    let vortex = moved.vortices.first_mut().expect("the example carries rim vortices");
    vortex.position = [vortex.position[0] + 5.0, vortex.position[1], vortex.position[2]];
    assert_eq!(crate::block3d_world_fit_revision(&moved), crate::block3d_world_fit_revision(&forest), "moving a vortex is an edit of this document, never a new document");

    let mut retargeted = forest.clone();
    retargeted.representations[0].mesh_url = Some("/mesh/🧊️capsule_J.glb".into());
    assert_ne!(crate::block3d_world_fit_revision(&retargeted), crate::block3d_world_fit_revision(&forest), "pointing a representation at another mesh delivers other geometry, so it must refit");
}
