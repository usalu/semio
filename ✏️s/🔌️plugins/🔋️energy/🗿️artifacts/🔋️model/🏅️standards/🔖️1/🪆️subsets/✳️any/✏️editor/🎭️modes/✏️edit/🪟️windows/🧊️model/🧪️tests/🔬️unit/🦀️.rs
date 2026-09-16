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
    // 🕹️ Picking is framework-owned, so the ONLY verb this window owns is its camera.
    assert_eq!(def.actions.len(), 1);
    assert_eq!(def.actions[0].id, SET_CAMERA_ACTION_ID);
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
async fn selection_publishes_the_lane_without_rebaking_mesh_colours() {
    let model = bestest_600();
    let plain = render(&model, &EnergyModelInteractionSnapshot::default(), None, None).expect("plain");
    let selected_id = energy_target_id(model.surfaces[0].id);
    let picked = render(&model, &EnergyModelInteractionSnapshot { selected_ids: vec![selected_id.clone()], hovered_ids: Vec::new() }, None, None).expect("picked");
    assert_eq!(scene_of(&plain).meshes_json, scene_of(&picked).meshes_json, "interaction paint is host-owned; geometry stays on the class swatch");
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
    assert_eq!(keys, vec![LEGEND_NODE_ID, CAPTION_SCENE_NODE_ID], "the legend column and the scene stack carry their own distinct ids: {keys:?}");
    let caption_keys: Vec<&str> = node.children[0].children.iter().map(|child| child.key.as_str()).collect();
    assert_eq!(caption_keys, vec![CAPTION_NODE_ID], "without result bounds the legend is the caption line and nothing else: {caption_keys:?}");

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

/// 🎥️ The react `World3dHost` dispatches `setCamera` itself after every orbit/pan/zoom. A window kind
/// that does not DECLARE it makes the shell drop the gesture
/// (`dropped action "setCamera" ... no window kind declares it`), which is what the first restage saw.
#[semio_framework_async_macros::async_test]
async fn the_window_declares_the_camera_verb_the_world3d_host_dispatches() {
    let action = definition().actions.into_iter().find(|action| action.id == SET_CAMERA_ACTION_ID).expect("the window declares setCamera");
    assert_eq!(action.id, "setCamera", "the id is the host's own wire spelling, not an authored name");
    assert!(matches!(action.kind, semio_framework_plugin::ActionKind::View), "a camera pose is never document data");
    assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "Migrated is the only UI-dispatchable classification");
    let camera = action.args.iter().find(|arg| arg.id == "camera").expect("the camera argument is declared");
    assert!(camera.required, "the pose is the whole payload");
    assert!(
        semio_framework::Terminology::ALL.iter().all(|&terminology| action.label.resolve(terminology, semio_framework::Locale::En) != action.label.resolve(terminology, semio_framework::Locale::De)),
        "the camera verb is really translated"
    );
}

/// 🎥️ A window that has been orbited republishes ITS OWN pose, not the model-derived default — that
/// is what stops a re-render (a selection, a results tick) from yanking the camera back.
#[semio_framework_async_macros::async_test]
async fn a_stored_camera_replaces_the_model_derived_one() {
    let model = bestest_600();
    let stored = config::EnergyModelWindowConfig { camera: config::EnergyModelCameraPose { position: [30.0, -25.0, 18.0], target: [4.0, 3.0, 1.35], zoom: 1.0 } };
    let default_scene = scene_of(&render(&model, &EnergyModelInteractionSnapshot::default(), None, None).expect("plain"));
    let stored_scene = scene_of(&render_with_camera(&model, &EnergyModelInteractionSnapshot::default(), None, None, Some(&stored)).expect("stored"));

    assert_ne!(default_scene.camera_json, stored_scene.camera_json);
    assert_eq!(stored_scene.camera_json, stored.camera.scene_camera_json());
    assert!(stored_scene.camera_json.contains("\"position\":[30.0,-25.0,18.0]"), "{}", stored_scene.camera_json);

    // 🎥️ The geometry, the picking domain and the fit revision are untouched by the camera: a camera
    // change must never re-arm the one-shot auto-fit (`world3dAutoFitKey` keys on fit revision +
    // the ATTACHED scene camera, never on the live pose).
    assert_eq!(default_scene.meshes_json, stored_scene.meshes_json);
    assert_eq!(default_scene.instances_json, stored_scene.instances_json);
    assert_eq!(default_scene.fit_json, stored_scene.fit_json);
    assert_eq!(default_scene.domain_id, stored_scene.domain_id);
}

/// 🎥️ Rendering the SAME model twice must publish byte-identical `fit_json` and `camera_json` —
/// `world3dAutoFitKey` keys the one-shot framing on exactly those, so a drifting value would re-frame
/// the viewport on every publication and fight the user's orbit.
#[semio_framework_async_macros::async_test]
async fn a_re_render_of_the_same_model_never_re_arms_the_auto_fit() {
    let model = bestest_600();
    let first = scene_of(&render(&model, &EnergyModelInteractionSnapshot::default(), None, None).expect("first"));
    let picked = scene_of(&render(&model, &EnergyModelInteractionSnapshot { selected_ids: vec![energy_target_id(model.surfaces[0].id)], hovered_ids: Vec::new() }, None, None).expect("picked"));
    assert_eq!(first.fit_json, picked.fit_json, "a selection must not re-frame the camera");
    assert_eq!(first.camera_json, picked.camera_json, "a selection must not reattach the camera");

    let other = crate::examples::bestest_610::model();
    let changed = scene_of(&render(&other, &EnergyModelInteractionSnapshot::default(), None, None).expect("other"));
    assert_ne!(first.fit_json, changed.fit_json, "new geometry DOES re-frame once");
}

//#region 🧱️LegendStrip
/// 🎨️ Depth-first walk collecting every `Component::Image` src under a node.
fn swatch_sources(node: &BuiltNode) -> Vec<String> {
    let mut found = Vec::new();
    if let semio_framework_plugin::Component::Image(props) = &node.component {
        found.push(props.src.as_str().to_string());
    }
    for child in node.children.iter() {
        found.extend(swatch_sources(child));
    }
    found
}

fn text_values(node: &BuiltNode) -> Vec<String> {
    let mut found = Vec::new();
    if let semio_framework_plugin::Component::Text(props) = &node.component {
        found.push(props.value.to_string());
    }
    for child in node.children.iter() {
        found.extend(text_values(child));
    }
    found
}

#[semio_framework_async_macros::async_test]
async fn the_results_legend_draws_eight_swatches_from_the_very_ramp_the_meshes_are_painted_with() {
    let model = bestest_600();
    let overlay: HashMap<u32, [f64; 3]> = model.surfaces.iter().map(|surface| (surface.id.0, [0.5, 0.5, 0.5])).collect();
    let node = render_with_legend(&model, &EnergyModelInteractionSnapshot::default(), Some(&overlay), Some("Conduction loss · 0.0 – 412.3 kWh"), Some((0.0, 412.3)), None).expect("the legend render assembles");

    // 🔑️ The whole point of the explicit ids: eight swatches plus two labels are ten siblings.
    assert_sibling_keys_unique(&node, "root");
    let legend = &node.children[0];
    assert_eq!(legend.key.as_str(), LEGEND_NODE_ID);
    let strip = legend.children.iter().find(|child| child.key.as_str() == LEGEND_STRIP_NODE_ID).expect("the ramp strip rides under the caption line");
    assert_eq!(strip.children.iter().count(), 10, "min label, eight bands, max label");

    let sources = swatch_sources(strip);
    let bands = crate::editor::model::results::legend_bands();
    assert_eq!(sources.len(), 8, "one swatch per ramp band: {sources:?}");
    for (source, band) in sources.iter().zip(bands.iter()) {
        // 🎨️ `#` is percent-encoded in a data URI, so the band hex appears as `%23rrggbb`.
        assert!(source.contains(&format!("%23{}", band.trim_start_matches('#'))), "swatch {source} does not carry band {band}");
        assert!(source.len() <= 512, "a swatch source must fit UI_TEXT_MAX_BYTES: {} bytes", source.len());
    }
    // 🌡️ The strip and the meshes must agree band for band: `band_color` indexes this same array.
    for (index, band) in bands.iter().enumerate() {
        let value = index as f64 / (bands.len() - 1) as f64 * 412.3;
        assert_eq!(crate::editor::model::results::band_color(value, 0.0, 412.3), *band, "band {index} of the strip is not the colour the mesh at that value takes");
    }

    let labels = text_values(legend);
    assert!(labels.contains(&"0.0".to_string()), "the strip prints its minimum: {labels:?}");
    assert!(labels.contains(&"412.3 kWh".to_string()), "the strip prints its maximum: {labels:?}");
    assert!(labels.iter().any(|label| label.contains("Conduction loss")), "the caption line still names the field: {labels:?}");

    // 🎬️ …and the scene underneath is untouched by any of it.
    let scene = scene_of(surface_node(&node));
    assert_eq!(scene.domain_id.as_deref(), Some(ENERGY_MODEL_INTERACTION_DOMAIN));
    assert!(scene.instances_json.contains(&format!("\"id\":\"{}\"", model.surfaces[0].id.0)));
}

#[semio_framework_async_macros::async_test]
async fn without_an_overlay_there_is_no_legend_at_all() {
    let model = bestest_600();
    // 🚫️ No caption ⇒ the render is the bare surface, bounds or not.
    let bare = render_with_legend(&model, &EnergyModelInteractionSnapshot::default(), None, None, Some((0.0, 9.0)), None).expect("the bare render assembles");
    assert!(matches!(bare.component, semio_framework_plugin::Component::Surface(_)), "an un-overlaid viewport is the surface itself, with no legend column");
    assert!(swatch_sources(&bare).is_empty(), "no overlay, no swatches");

    // 🏷️ A caption with no bounds is the caption line alone — the old shape, unchanged.
    let captioned = render(&model, &EnergyModelInteractionSnapshot::default(), None, Some("Conduction loss")).expect("the captioned render assembles");
    assert!(swatch_sources(&captioned).is_empty(), "a caption without bounds draws no ramp strip");
    assert_eq!(captioned.children[0].children.iter().count(), 1, "the legend column is just the caption line");
}

#[semio_framework_async_macros::async_test]
async fn a_swatch_source_is_a_self_contained_inline_svg() {
    let source = legend_swatch_src("#1d4ed8");
    assert!(source.starts_with("data:image/svg+xml,"), "the swatch must need no network fetch: {source}");
    assert!(source.contains("%3Csvg") && source.contains("%3C/svg%3E"), "the angle brackets are percent-encoded so the URI is valid: {source}");
    assert!(source.contains("xmlns='http://www.w3.org/2000/svg'"), "an SVG data URI without the namespace does not render: {source}");
    assert!(source.contains("fill='%231d4ed8'"), "the hex is carried with its `#` encoded: {source}");
    assert_eq!(legend_swatch_src("1d4ed8"), source, "a bare hex and a `#`-prefixed one are the same swatch");
}
//#endregion 🧱️LegendStrip
