
use super::*;
use crate::editor::sourcing::testkit::{new_app, render as render_body};

/// 🧬️ Direct unit coverage for `render`'s own id-lookup logic — the app-level call site always
/// passes an empty slice (see the `selected_ids` doc comment above) until a future wave threads
/// interaction into `render`.
/// 🎬️ Asserts through the packed scene, not the node JSON: `MeshWindowKit::render` hands the
/// `World3dScene` to `semio_framework_ui_scene::encode`, which packs it into the surface's opaque
/// `UiFixedBytes` — so a serialized `BuiltNode` no longer carries any scene string. Decoding the
/// surface back is the current idiom (see `🎪️demonstrator`'s own `🪟️main` window test).
#[semio_framework_async_macros::async_test]
async fn preview_renders_selected_mesh_id() {
    let document = crate::schema::default_document();
    let object_id = crate::stock_of(&document)[0].id.clone();
    let node = render(&document, &[object_id.clone()], crate::editor::sourcing::terminology::sourcing_curation_labels(&semio_framework_plugin::ViewModel::default())).expect("bounded preview");
    let scene: semio_framework_ui_scene::World3dScene = semio_framework_plugin::testkit::built_surface_scene(&node).expect("assemble world3d scene");
    assert!(scene.meshes_json.contains(&object_id), "the selected kind's mesh must be in the scene");
    assert!(scene.instances_json.contains(&object_id), "the selected kind must be instanced once");
}

#[semio_framework_async_macros::async_test]
async fn preview_shows_placeholder_without_selection() {
    let document = crate::schema::default_document();
    let node = render(&document, &[], crate::editor::sourcing::terminology::sourcing_curation_labels(&semio_framework_plugin::ViewModel::default())).expect("bounded placeholder");
    let json = serde_json::to_string(&node).unwrap();
    assert!(json.contains("No selection"));
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_world3d_surface_and_body_key() {
    let def = definition();
    assert_eq!(def.body_key, SOURCING_CURATION_BODY_PREVIEW);
    assert!(matches!(def.surface_kind, SurfaceKind::World3d));
}

#[semio_framework_async_macros::async_test]
async fn renders_via_the_app() {
    let mut app = new_app().await;
    // `render` carries no `InteractionView` yet, so the app-level render always shows the placeholder.
    assert!(render_body(&mut app, SOURCING_CURATION_BODY_PREVIEW).await.contains("No selection"));
}
