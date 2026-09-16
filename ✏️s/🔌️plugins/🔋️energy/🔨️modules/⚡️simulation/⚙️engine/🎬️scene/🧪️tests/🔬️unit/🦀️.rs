use super::*;

fn quad(id: u32, class: SurfaceClass) -> Surface {
    Surface {
        id: EntityId(id),
        name: format!("Surface {id}"),
        zone_id: EntityId(1),
        class,
        vertices_m: vec![[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [4.0, 0.0, 2.5], [0.0, 0.0, 2.5]],
        construction_id: EntityId(1),
        outside_boundary_condition: crate::model::OutsideBoundary::OutdoorAir,
        sun_exposed: true,
        wind_exposed: true,
        multiplier: 1,
    }
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn parsed(json: &str) -> Value {
    serde_json::from_str(json).expect("the scene payload is json")
}

#[test]
fn a_quad_fan_triangulates_into_two_triangles() {
    let mut model = Model { surfaces: vec![quad(7, SurfaceClass::ExteriorWall)], ..Model::default() };
    model.name = "quad".into();
    let (meshes_json, _) = energy_model_scene_parts(&model, &EnergySceneStyle::default());
    let meshes = parsed(&meshes_json);
    let data = &meshes[0]["data"];
    assert_eq!(data["indices"].as_array().expect("indices").len(), 6, "a quad is exactly two triangles: {meshes_json}");
    assert_eq!(data["positions"].as_array().expect("positions").len(), 18, "three corners per triangle, three floats per corner");
    assert_eq!(data["colors"].as_array().expect("colors").len(), 18, "one colour triple per emitted corner");
    assert_eq!(data["normals"].as_array().expect("normals").len(), 18, "one flat normal per emitted corner");
}

#[test]
fn every_instance_id_is_the_raw_entity_id() {
    let model = Model { surfaces: vec![quad(12, SurfaceClass::Roof)], ..Model::default() };
    let (_, instances_json) = energy_model_scene_parts(&model, &EnergySceneStyle::default());
    let instances = parsed(&instances_json);
    assert_eq!(instances[0]["id"].as_str(), Some("12"), "the instance id is the pick vocabulary: {instances_json}");
    assert_eq!(instances[0]["meshId"].as_str(), Some("energy-surface-12"));
    assert_eq!(instances[0]["objectKind"].as_str(), Some(ENERGY_SCENE_OBJECT_KIND_SURFACE));
    assert_eq!(energy_scene_target_id(EntityId(12)), "12");
}

#[test]
fn a_degenerate_surface_is_skipped_rather_than_faulting() {
    let mut surface = quad(3, SurfaceClass::Floor);
    surface.vertices_m = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
    let model = Model { surfaces: vec![surface], ..Model::default() };
    let (meshes_json, instances_json) = energy_model_scene_parts(&model, &EnergySceneStyle::default());
    assert_eq!(meshes_json, "[]");
    assert_eq!(instances_json, "[]");
}

#[test]
fn a_window_polygon_lies_on_its_host_plane_offset_by_the_lift() {
    let host = quad(1, SurfaceClass::ExteriorWall);
    let window = Fenestration {
        id: EntityId(2),
        name: "Window".into(),
        surface_id: EntityId(1),
        u_value_w_m2k: 3.0,
        shgc: 0.7,
        vlt: 0.8,
        area_m2: 2.0,
        height_m: 1.0,
        sill_height_m: 0.5,
        frame_conductance_w_k: 0.0,
        divider_conductance_w_k: 0.0,
        overhang_depth_m: 0.0,
        overhang_offset_m: 0.0,
        fin_depth_m: 0.0,
        fin_offset_m: 0.0,
        glazing_construction_id: None,
        vertices_m: Vec::new(),
    };
    let polygon = fenestration_polygon(&host, &window, 0, 1).expect("a rectangular aperture");
    let normal = normalized(crate::geometry::polygon_normal(&host.vertices_m)).expect("a host normal");
    let origin = host.vertices_m[0];
    for corner in &polygon {
        let distance = dot(sub(*corner, origin), normal);
        assert!((distance - ENERGY_SCENE_WINDOW_OFFSET_M).abs() < 1e-9, "corner {corner:?} is {distance} off the host plane, expected the 5 mm lift");
    }
    let width = dot(sub(polygon[1], polygon[0]), sub(polygon[1], polygon[0])).sqrt();
    let height = dot(sub(polygon[3], polygon[0]), sub(polygon[3], polygon[0])).sqrt();
    assert!((width - 2.0).abs() < 1e-9, "area / height is the width: {width}");
    assert!((height - 1.0).abs() < 1e-9, "the stated height is the height: {height}");
}

#[test]
fn selection_and_hover_retint_and_an_overlay_replaces_the_class_swatch() {
    let model = Model { surfaces: vec![quad(5, SurfaceClass::ExteriorWall)], ..Model::default() };
    let plain = EnergySceneStyle::default();
    let (base_json, _) = energy_model_scene_parts(&model, &plain);

    let selected_ids = vec!["5".to_string()];
    let (selected_json, selected_instances) = energy_model_scene_parts(&model, &EnergySceneStyle { selected_ids: &selected_ids, ..EnergySceneStyle::default() });
    assert_ne!(base_json, selected_json, "a selected surface must be tinted");
    assert_eq!(parsed(&selected_instances)[0]["selected"].as_bool(), Some(true));

    let hovered_ids = vec!["5".to_string()];
    let (hovered_json, hovered_instances) = energy_model_scene_parts(&model, &EnergySceneStyle { hovered_ids: &hovered_ids, ..EnergySceneStyle::default() });
    assert_ne!(base_json, hovered_json, "a hovered surface must be lightened");
    assert_ne!(selected_json, hovered_json, "hover and selection are different paints");
    assert_eq!(parsed(&hovered_instances)[0]["hovered"].as_bool(), Some(true));

    let overlay = HashMap::from([(5_u32, [1.0, 0.0, 0.0])]);
    let (overlay_json, _) = energy_model_scene_parts(&model, &EnergySceneStyle { overlay: Some(&overlay), ..EnergySceneStyle::default() });
    let colors = parsed(&overlay_json)[0]["data"]["colors"].as_array().expect("colors").clone();
    assert_eq!(colors[0].as_f64(), Some(1.0), "the overlay replaces the base colour: {overlay_json}");
    assert_eq!(colors[1].as_f64(), Some(0.0));
    assert_eq!(colors[2].as_f64(), Some(0.0));
}

#[test]
fn the_fit_revision_follows_the_geometry_and_the_empty_model_renders() {
    let empty = Model::default();
    let (meshes_json, instances_json) = energy_model_scene_parts(&empty, &EnergySceneStyle::default());
    assert_eq!(meshes_json, "[]");
    assert_eq!(instances_json, "[]");
    assert_eq!(energy_model_extent(&empty), 1.0);
    assert!(energy_model_bounds(&empty).is_none());

    let one = Model { surfaces: vec![quad(1, SurfaceClass::ExteriorWall)], ..Model::default() };
    let two = Model { surfaces: vec![quad(1, SurfaceClass::ExteriorWall), quad(2, SurfaceClass::Roof)], ..Model::default() };
    assert_eq!(energy_model_fit_revision(&one), energy_model_fit_revision(&one), "the revision is deterministic");
    assert_ne!(energy_model_fit_revision(&one), energy_model_fit_revision(&two), "new geometry re-frames the camera");
}

#[test]
fn every_surface_class_and_family_carries_its_own_swatch() {
    let classes = [
        SurfaceClass::ExteriorWall,
        SurfaceClass::InteriorWall,
        SurfaceClass::Roof,
        SurfaceClass::Ceiling,
        SurfaceClass::Floor,
        SurfaceClass::Interzone,
        SurfaceClass::Adiabatic,
        SurfaceClass::Ground,
    ];
    let mut seen: Vec<[f64; 3]> = Vec::new();
    for class in classes {
        let color = surface_class_color(class);
        assert!(color.iter().all(|channel| (0.0..=1.0).contains(channel)), "{class:?} is outside the unit cube");
        assert!(!seen.contains(&color), "{class:?} repeats another class's swatch");
        seen.push(color);
    }
    assert!(!seen.contains(&ENERGY_SCENE_FENESTRATION_COLOR), "glazing must not reuse an opaque class swatch");
    assert!(!seen.contains(&ENERGY_SCENE_SHADING_COLOR), "shading must not reuse an opaque class swatch");
}

#[test]
fn the_bestest_600_example_renders_every_surface_and_every_window() {
    let model = crate::examples::bestest_600::model();
    let (meshes_json, instances_json) = energy_model_scene_parts(&model, &EnergySceneStyle::default());
    let instances = parsed(&instances_json);
    let ids: Vec<&str> = instances.as_array().expect("instances").iter().filter_map(|entry| entry["id"].as_str()).collect();
    assert_eq!(ids.len(), model.surfaces.len() + model.fenestrations.len(), "one instance per surface and per window: {instances_json}");
    for surface in &model.surfaces {
        assert!(ids.contains(&energy_scene_target_id(surface.id).as_str()), "surface {} is missing", surface.id.0);
    }
    for window in &model.fenestrations {
        assert!(ids.contains(&energy_scene_target_id(window.id).as_str()), "window {} is missing", window.id.0);
    }
    assert!(meshes_json.contains(ENERGY_SCENE_FENESTRATION_MESH_PREFIX), "the two south windows carry their own meshes");
}
