use super::*;
use crate::editor::model::interaction::energy_target_id;
use semio_framework_plugin::World3dScene;

fn scene_of(node: &BuiltNode) -> World3dScene {
    semio_framework_plugin::artifact_app_laws::built_surface_scene(node).expect("the built node decodes back into a world scene")
}

fn bestest_600() -> crate::model::Model {
    crate::examples::bestest_600::model()
}

/// 🔑️ Every node in the tree must have siblings with DISTINCT keys — two `#0` siblings publish as
/// `DuplicateSiblingKey parent=#0 key=#0` and scope the whole surface to a render fault.
fn assert_sibling_keys_unique(node: &BuiltNode, path: &str) {
    let mut seen = std::collections::BTreeSet::new();
    for child in node.children.iter() {
        let key = child.key.as_str().to_string();
        assert!(seen.insert(key.clone()), "duplicate sibling key '{key}' under {path}");
    }
    for child in node.children.iter() {
        assert_sibling_keys_unique(child, &format!("{path}/{}", child.key.as_str()));
    }
}

/// 🎬️ The one world surface inside a (possibly captioned) render.
fn surface_node(node: &BuiltNode) -> &BuiltNode {
    if matches!(node.component, semio_framework_plugin::Component::Surface(_)) {
        return node;
    }
    node.children.iter().find_map(|child| child.children.iter().chain(std::iter::once(child)).find(|entry| matches!(entry.component, semio_framework_plugin::Component::Surface(_)))).expect("a captioned render still carries exactly one world surface")
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_world3d_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.id, "energy.model.3d", "the window kind id is fixed — the tree panel and the results overlay address it");
    assert_eq!(def.body_key, BODY_KEY);
    assert_eq!(def.body_key, "energy.model.3d");
    assert_eq!(def.surface_kind, SurfaceKind::World3d);
    assert!(def.actions.is_empty(), "picking is framework-owned; this window declares no verbs");
    assert!(
        semio_framework::Terminology::ALL.iter().all(|&terminology| def.label.resolve(terminology, semio_framework::Locale::En) != def.label.resolve(terminology, semio_framework::Locale::De)),
        "the window label is really translated"
    );
}

#[semio_framework_async_macros::async_test]
async fn render_binds_the_shared_interaction_domain_at_surface_granularity() {
    let node = render(&bestest_600(), &EnergyModelInteractionSnapshot::default(), None, None).expect("the world surface assembles");
    let scene = scene_of(&node);
    assert_eq!(scene.domain_id.as_deref(), Some(ENERGY_MODEL_INTERACTION_DOMAIN));
    assert_eq!(scene.domain_granularity_id.as_deref(), Some(ENERGY_GRANULARITY_SURFACE));
    assert!(scene.fit_json.is_some(), "the camera auto-frames once per geometry revision");
}

#[semio_framework_async_macros::async_test]
async fn render_emits_one_instance_per_surface_and_per_window_keyed_by_the_shared_target_id() {
    let model = bestest_600();
    assert_eq!(model.surfaces.len(), 6, "ANSI/ASHRAE 140 case 600 is a six-sided box");
    assert_eq!(model.fenestrations.len(), 2, "case 600 carries two south windows");
    let node = render(&model, &EnergyModelInteractionSnapshot::default(), None, None).expect("the world surface assembles");
    let scene = scene_of(&node);
    for surface in &model.surfaces {
        let id = energy_target_id(surface.id);
        assert!(scene.instances_json.contains(&format!("\"id\":\"{id}\"")), "surface {} ({}) has no instance: {}", surface.id.0, surface.name, scene.instances_json);
    }
    for window in &model.fenestrations {
        let id = energy_target_id(window.id);
        assert!(scene.instances_json.contains(&format!("\"id\":\"{id}\"")), "window {} ({}) has no instance: {}", window.id.0, window.name, scene.instances_json);
    }
    assert!(scene.meshes_json.contains("energy-surface-"), "every surface carries a data mesh");
    assert!(scene.meshes_json.contains("energy-window-"), "every window carries a data mesh");
}

#[semio_framework_async_macros::async_test]
async fn the_scene_target_id_is_the_one_the_tree_and_the_inspector_use() {
    for raw in [0_u32, 1, 42, u32::MAX] {
        assert_eq!(crate::scene::energy_scene_target_id(crate::model::EntityId(raw)), energy_target_id(crate::model::EntityId(raw)), "the scene and the interaction module must spell one vocabulary");
    }
}

#[semio_framework_async_macros::async_test]
async fn selection_retints_the_baked_mesh_colours() {
    let model = bestest_600();
    let plain = render(&model, &EnergyModelInteractionSnapshot::default(), None, None).expect("plain");
    let selected_id = energy_target_id(model.surfaces[0].id);
    let picked = render(&model, &EnergyModelInteractionSnapshot { selected_ids: vec![selected_id.clone()], hovered_ids: Vec::new() }, None, None).expect("picked");
    assert_ne!(scene_of(&plain).meshes_json, scene_of(&picked).meshes_json, "a selected surface is tinted in the baked vertex colours");
    assert!(scene_of(&picked).selection_json.contains(&selected_id), "the selection lane names the picked id");
}

#[semio_framework_async_macros::async_test]
async fn an_overlay_replaces_a_surfaces_colour_and_a_caption_wraps_the_scene() {
    let model = bestest_600();
    let plain = render(&model, &EnergyModelInteractionSnapshot::default(), None, None).expect("plain");
    let overlay = HashMap::from([(model.surfaces[0].id.0, [1.0_f64, 0.0, 0.0])]);
    let coloured = render(&model, &EnergyModelInteractionSnapshot::default(), Some(&overlay), None).expect("coloured");
    assert_ne!(scene_of(&plain).meshes_json, scene_of(&coloured).meshes_json, "a results overlay replaces the class swatch");

    let captioned = render(&model, &EnergyModelInteractionSnapshot::default(), None, Some("Transmission loss (kWh)")).expect("captioned");
    assert!(!matches!(captioned.component, semio_framework_plugin::Component::Surface(_)), "a captioned render wraps the surface in a column");
    assert_eq!(captioned.children.len(), 2, "the caption sits above the scene stack");
}

/// 🔑️ The exact shape the results run drives: an overlay AND a caption at once. Before the caption
/// nodes carried explicit ids, both siblings defaulted to `#0` and every publication of
/// `energy.model.3d` faulted with `DuplicateSiblingKey parent=#0 key=#0`, blanking the window for the
/// whole run (`🗑️generated/energy-results-w1/console.txt`). This is the regression guard.
#[semio_framework_async_macros::async_test]
async fn a_captioned_overlay_render_has_unique_sibling_keys_and_still_decodes() {
    let model = bestest_600();
    let overlay: HashMap<u32, [f64; 3]> = model.surfaces.iter().enumerate().map(|(index, surface)| (surface.id.0, [index as f64 / 10.0, 0.5, 1.0 - index as f64 / 10.0])).collect();
    let node = render(&model, &EnergyModelInteractionSnapshot { selected_ids: vec![energy_target_id(model.surfaces[0].id)], hovered_ids: vec![energy_target_id(model.surfaces[1].id)] }, Some(&overlay), Some("Conduction loss 0 – 412 kWh")).expect("the captioned overlay render assembles");

    assert_sibling_keys_unique(&node, "root");
    let keys: Vec<&str> = node.children.iter().map(|child| child.key.as_str()).collect();
    assert_eq!(keys, vec![CAPTION_NODE_ID, CAPTION_SCENE_NODE_ID], "the caption and the scene stack carry their own distinct ids: {keys:?}");

    let scene = scene_of(surface_node(&node));
    assert_eq!(scene.domain_id.as_deref(), Some(ENERGY_MODEL_INTERACTION_DOMAIN), "the captioned scene is still domain-bound");
    assert!(scene.instances_json.contains(&format!("\"id\":\"{}\"", model.surfaces[0].id.0)), "the captioned scene still carries every instance");
    assert_ne!(scene.meshes_json, scene_of(&render(&model, &EnergyModelInteractionSnapshot::default(), None, None).expect("plain")).meshes_json, "the overlay still repaints under a caption");
}

#[semio_framework_async_macros::async_test]
async fn an_empty_model_renders_without_faulting() {
    let node = render(&crate::model::Model::default(), &EnergyModelInteractionSnapshot::default(), None, None).expect("an empty model still assembles a surface");
    let scene = scene_of(&node);
    assert_eq!(scene.meshes_json, "[]");
    assert_eq!(scene.instances_json, "[]");
}

#[semio_framework_async_macros::async_test]
async fn a_quad_surface_fan_triangulates_into_two_triangles() {
    let model = bestest_600();
    let node = render(&model, &EnergyModelInteractionSnapshot::default(), None, None).expect("assembles");
    let meshes: serde_json::Value = serde_json::from_str(&scene_of(&node).meshes_json).expect("the mesh lane is json");
    let first = &meshes[0];
    assert_eq!(first["id"].as_str(), Some(format!("energy-surface-{}", model.surfaces[0].id.0).as_str()));
    assert_eq!(first["data"]["indices"].as_array().expect("indices").len(), 6, "case 600's walls are quads — exactly two triangles each");
}

#[semio_framework_async_macros::async_test]
async fn a_window_polygon_lies_on_its_host_surfaces_plane() {
    let model = bestest_600();
    let window = &model.fenestrations[0];
    let host = model.surfaces.iter().find(|surface| surface.id == window.surface_id).expect("the host wall");
    let polygon = crate::scene::fenestration_polygon(host, window, 0, model.fenestrations.len()).expect("a rectangular aperture");
    let normal = crate::geometry::polygon_normal(&host.vertices_m);
    let origin = host.vertices_m[0];
    for corner in polygon {
        let offset = [corner[0] - origin[0], corner[1] - origin[1], corner[2] - origin[2]];
        let distance = offset[0] * normal[0] + offset[1] * normal[1] + offset[2] * normal[2];
        assert!(
            (distance - crate::scene::ENERGY_SCENE_WINDOW_OFFSET_M).abs() < 1e-9,
            "corner {corner:?} sits {distance} m off the host plane, expected exactly the {} m anti-z-fighting lift",
            crate::scene::ENERGY_SCENE_WINDOW_OFFSET_M
        );
    }
}
