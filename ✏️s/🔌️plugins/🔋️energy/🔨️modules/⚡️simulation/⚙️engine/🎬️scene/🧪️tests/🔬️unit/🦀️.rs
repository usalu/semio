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

/// 🪟️ A window carrying nothing but its own authored polygon — every scalar zeroed, so a rendered
/// polygon can only have come from `vertices_m`.
fn authored_window(id: u32, host: u32, vertices_m: Vec<[f64; 3]>) -> Fenestration {
    Fenestration {
        id: EntityId(id),
        name: format!("Window {id}"),
        surface_id: EntityId(host),
        u_value_w_m2k: 3.0,
        shgc: 0.7,
        vlt: 0.8,
        area_m2: 0.0,
        height_m: 0.0,
        sill_height_m: 0.0,
        frame_conductance_w_k: 0.0,
        divider_conductance_w_k: 0.0,
        overhang_depth_m: 0.0,
        overhang_offset_m: 0.0,
        fin_depth_m: 0.0,
        fin_offset_m: 0.0,
        glazing_construction_id: None,
        vertices_m,
    }
}

/// 📦️ The six CCW-from-outside faces of an axis-aligned box, all in one zone — the shape every
/// authored example's zone actually has.
fn box_zone_surfaces(zone: u32, first_id: u32, size: [f64; 3]) -> Vec<Surface> {
    let [x, y, z] = size;
    let faces = [
        (SurfaceClass::Floor, vec![[0.0, 0.0, 0.0], [0.0, y, 0.0], [x, y, 0.0], [x, 0.0, 0.0]]),
        (SurfaceClass::Roof, vec![[0.0, 0.0, z], [x, 0.0, z], [x, y, z], [0.0, y, z]]),
        (SurfaceClass::ExteriorWall, vec![[0.0, 0.0, 0.0], [x, 0.0, 0.0], [x, 0.0, z], [0.0, 0.0, z]]),
        (SurfaceClass::ExteriorWall, vec![[x, 0.0, 0.0], [x, y, 0.0], [x, y, z], [x, 0.0, z]]),
        (SurfaceClass::ExteriorWall, vec![[x, y, 0.0], [0.0, y, 0.0], [0.0, y, z], [x, y, z]]),
        (SurfaceClass::ExteriorWall, vec![[0.0, y, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, z], [0.0, y, z]]),
    ];
    faces
        .into_iter()
        .enumerate()
        .map(|(offset, (class, vertices_m))| Surface {
            id: EntityId(first_id + offset as u32),
            name: format!("Face {offset}"),
            zone_id: EntityId(zone),
            class,
            vertices_m,
            construction_id: EntityId(1),
            outside_boundary_condition: crate::model::OutsideBoundary::OutdoorAir,
            sun_exposed: true,
            wind_exposed: true,
            multiplier: 1,
        })
        .collect()
}

fn zone(id: u32, name: &str) -> crate::model::Zone {
    crate::model::Zone { id: EntityId(id), name: name.into(), volume_m3: 0.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true }
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
fn selection_and_hover_publish_instance_flags_and_an_overlay_replaces_the_class_swatch() {
    let model = Model { surfaces: vec![quad(5, SurfaceClass::ExteriorWall)], ..Model::default() };
    let plain = EnergySceneStyle::default();
    let (base_json, _) = energy_model_scene_parts(&model, &plain);

    let selected_ids = vec!["5".to_string()];
    let (selected_json, selected_instances) = energy_model_scene_parts(&model, &EnergySceneStyle { selected_ids: &selected_ids, ..EnergySceneStyle::default() });
    assert_eq!(base_json, selected_json, "selection paint is host-owned");
    assert_eq!(parsed(&selected_instances)[0]["selected"].as_bool(), Some(true));

    let hovered_ids = vec!["5".to_string()];
    let (hovered_json, hovered_instances) = energy_model_scene_parts(&model, &EnergySceneStyle { hovered_ids: &hovered_ids, ..EnergySceneStyle::default() });
    assert_eq!(base_json, hovered_json, "hover paint is host-owned");
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
    assert_eq!(ids.len(), model.surfaces.len() + model.fenestrations.len() + model.zones.len(), "one instance per surface, per window and per zone volume: {instances_json}");
    for zone in &model.zones {
        assert!(ids.contains(&energy_scene_target_id(zone.id).as_str()), "zone {} draws no volume", zone.id.0);
    }
    for surface in &model.surfaces {
        assert!(ids.contains(&energy_scene_target_id(surface.id).as_str()), "surface {} is missing", surface.id.0);
    }
    for window in &model.fenestrations {
        assert!(ids.contains(&energy_scene_target_id(window.id).as_str()), "window {} is missing", window.id.0);
    }
    assert!(meshes_json.contains(ENERGY_SCENE_FENESTRATION_MESH_PREFIX), "the two south windows carry their own meshes");
}

//#region 🧱️ZoneVolumes
#[test]
fn a_zone_of_six_box_faces_draws_one_closed_translucent_unpickable_hull() {
    let model = Model { zones: vec![zone(1, "Living")], surfaces: box_zone_surfaces(1, 40, [4.0, 3.0, 2.5]), ..Model::default() };
    let (meshes_json, instances_json) = energy_model_scene_parts(&model, &EnergySceneStyle::default());
    let instances = parsed(&instances_json);
    let rows = instances.as_array().expect("instances");
    assert_eq!(rows.len(), 7, "six faces plus exactly one zone volume: {instances_json}");

    let volume = rows.last().expect("the zone volume is published last");
    assert_eq!(volume["id"].as_str(), Some("1"), "the zone instance id is the raw entity id the tree addresses");
    assert_eq!(volume["label"].as_str(), Some("Living"), "the zone volume is labelled by its zone");
    assert_eq!(volume["objectKind"].as_str(), Some(ENERGY_SCENE_OBJECT_KIND_ZONE));
    assert_eq!(volume["meshId"].as_str(), Some("energy-zone-1"));
    // 🚫️ The ONE lane that is both "translucent" and "invisible to a raycast" — without it the hull
    // would swallow every pick of the walls inside it.
    assert_eq!(volume["disabled"].as_bool(), Some(true), "a zone volume must never occlude the pick of a wall: {instances_json}");
    for row in &rows[..6] {
        assert!(row["disabled"].as_bool().is_none(), "an envelope surface stays pickable: {row}");
    }

    let meshes = parsed(&meshes_json);
    let hull = meshes.as_array().expect("meshes").last().expect("the zone mesh").clone();
    assert_eq!(hull["id"].as_str(), Some("energy-zone-1"));
    let indices = hull["data"]["indices"].as_array().expect("indices").len();
    assert_eq!(indices, 36, "a box hull is 12 triangles: {hull}");
    let colors = hull["data"]["colors"].as_array().expect("colors");
    assert_eq!(colors.len(), indices * 3, "one colour triple per emitted corner");
    assert_eq!(colors[0].as_f64(), Some(ENERGY_SCENE_ZONE_COLOR[0]), "the zone swatch is its own");
}

#[test]
fn the_box_hull_encloses_exactly_the_zone_volume_the_engine_integrates() {
    let model = Model { zones: vec![zone(1, "Living")], surfaces: box_zone_surfaces(1, 40, [4.0, 3.0, 2.5]), ..Model::default() };
    let triangles = convex_hull_triangles(&zone_hull_points(&model, EntityId(1))).expect("a box encloses a volume");
    // 🧮️ Divergence-theorem volume of the drawn shell, against the engine's own pyramid sum over the
    // authored faces: the hull IS the zone for a rectangular room, so the two must agree exactly.
    let drawn = triangles
        .iter()
        .map(|triangle| {
            let normal = cross3(sub3(triangle[1], triangle[0]), sub3(triangle[2], triangle[0]));
            dot(triangle[0], normal) / 6.0
        })
        .sum::<f64>()
        .abs();
    let faces: Vec<&[[f64; 3]]> = model.surfaces.iter().map(|surface| surface.vertices_m.as_slice()).collect();
    let integrated = crate::geometry::zone_volume_from_surfaces(&faces);
    assert!((drawn - 30.0).abs() < 1e-9, "4 × 3 × 2.5 m is 30 m³, drawn {drawn}");
    assert!((drawn - integrated).abs() < 1e-9, "the drawn hull and the engine's volume must agree: {drawn} vs {integrated}");
}

#[test]
fn an_empty_or_flat_zone_draws_no_volume_at_all() {
    let empty = Model { zones: vec![zone(9, "Plenum")], ..Model::default() };
    let (meshes_json, instances_json) = energy_model_scene_parts(&empty, &EnergySceneStyle::default());
    assert_eq!(meshes_json, "[]", "a zone with no member surface has no shape to draw");
    assert_eq!(instances_json, "[]");
    assert!(zone_hull_points(&empty, EntityId(9)).is_empty());

    // 📐️ One floor only: four coplanar corners enclose nothing, so no zero-thickness shell is emitted.
    let mut flat = Model { zones: vec![zone(9, "Slab")], surfaces: vec![quad(40, SurfaceClass::Floor)], ..Model::default() };
    flat.surfaces[0].zone_id = EntityId(9);
    let (_, flat_instances) = energy_model_scene_parts(&flat, &EnergySceneStyle::default());
    let flat_rows = parsed(&flat_instances);
    let ids: Vec<&str> = flat_rows.as_array().expect("instances").iter().filter_map(|row| row["id"].as_str()).collect();
    assert_eq!(ids, vec!["40"], "the floor renders, the flat zone does not: {flat_instances}");
    assert!(convex_hull_triangles(&zone_hull_points(&flat, EntityId(9))).is_none());
}

#[test]
fn the_hull_refuses_degenerate_clouds_and_survives_interior_and_duplicate_points() {
    assert!(convex_hull_triangles(&[]).is_none(), "nothing has no hull");
    assert!(convex_hull_triangles(&[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0], [3.0, 0.0, 0.0]]).is_none(), "collinear points enclose nothing");
    assert!(convex_hull_triangles(&[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]]).is_none(), "coplanar points enclose nothing");
    assert!(convex_hull_triangles(&[[0.0, 0.0, f64::NAN], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]).is_none(), "a non-finite corner is refused, never rendered");

    // 🧊️ A unit cube's eight corners, each authored twice (shared between faces), plus one point in
    // the middle that the hull must swallow: still exactly 12 triangles.
    let mut cloud: Vec<[f64; 3]> = Vec::new();
    for corner in 0..8 {
        let point = [f64::from(corner & 1), f64::from((corner >> 1) & 1), f64::from((corner >> 2) & 1)];
        cloud.push(point);
        cloud.push(point);
    }
    cloud.push([0.5, 0.5, 0.5]);
    let triangles = convex_hull_triangles(&cloud).expect("a cube has a hull");
    assert_eq!(triangles.len(), 12, "a cube's hull is 12 triangles whatever the input order");
    for triangle in &triangles {
        let normal = cross3(sub3(triangle[1], triangle[0]), sub3(triangle[2], triangle[0]));
        assert!(dot(normal, sub3([0.5, 0.5, 0.5], triangle[0])) < 0.0, "every hull face winds outward: {triangle:?}");
    }
}

#[test]
fn a_selected_zone_volume_still_publishes_the_selected_flag() {
    let model = Model { zones: vec![zone(1, "Living")], surfaces: box_zone_surfaces(1, 40, [4.0, 3.0, 2.5]), ..Model::default() };
    let (plain, _) = energy_model_scene_parts(&model, &EnergySceneStyle::default());
    let picked = vec!["1".to_string()];
    let (selected, selected_instances) = energy_model_scene_parts(&model, &EnergySceneStyle { selected_ids: &picked, ..EnergySceneStyle::default() });
    assert_eq!(plain, selected, "zone hull vertex colours stay on the violet swatch; disabled style keeps them visible");
    let rows = parsed(&selected_instances);
    assert_eq!(rows.as_array().expect("instances").last().expect("the zone")["selected"].as_bool(), Some(true));
}
//#endregion 🧱️ZoneVolumes

//#region 🧱️AuthoredPolygons
#[test]
fn an_authored_five_vertex_window_renders_as_authored() {
    let host = quad(1, SurfaceClass::ExteriorWall);
    // 🏠️ A gabled five-corner window on the south wall (the host quad spans x 0..4, z 0..2.5 at y=0),
    // authored in the host's own plane so only the 5 mm lift moves it.
    let authored = vec![[1.0, 0.0, 0.5], [3.0, 0.0, 0.5], [3.0, 0.0, 1.6], [2.0, 0.0, 2.1], [1.0, 0.0, 1.6]];
    let model = Model { surfaces: vec![host], fenestrations: vec![authored_window(50, 1, authored.clone())], ..Model::default() };

    let polygon = model_fenestration_polygon(&model, &model.fenestrations[0]).expect("an authored aperture");
    assert_eq!(polygon.len(), 5, "an authored polygon is rendered as authored, never re-derived: {polygon:?}");
    let normal = normalized(crate::geometry::polygon_normal(&model.surfaces[0].vertices_m)).expect("a host normal");
    for (rendered, corner) in polygon.iter().zip(authored.iter()) {
        let offset = sub(*rendered, *corner);
        assert!((dot(offset, normal) - ENERGY_SCENE_WINDOW_OFFSET_M).abs() < 1e-9, "corner {rendered:?} is not the authored corner {corner:?} lifted 5 mm");
        assert!((dot(offset, offset).sqrt() - ENERGY_SCENE_WINDOW_OFFSET_M).abs() < 1e-9, "the lift is the ONLY motion applied to an authored corner");
    }

    let (meshes_json, instances_json) = energy_model_scene_parts(&model, &EnergySceneStyle::default());
    let window_mesh = parsed(&meshes_json).as_array().expect("meshes").iter().find(|mesh| mesh["id"].as_str() == Some("energy-window-50")).expect("the window mesh").clone();
    assert_eq!(window_mesh["data"]["indices"].as_array().expect("indices").len(), 9, "a five-gon fans into three triangles: {window_mesh}");
    assert_eq!(window_mesh["data"]["positions"].as_array().expect("positions").len(), 27, "three corners per triangle, three floats per corner");
    let rows = parsed(&instances_json);
    let ids: Vec<&str> = rows.as_array().expect("instances").iter().filter_map(|row| row["id"].as_str()).collect();
    assert!(ids.contains(&"50"), "the authored window is one pickable instance: {instances_json}");
}

#[test]
fn an_authored_polygon_wins_over_the_area_height_sill_rectangle() {
    let host = quad(1, SurfaceClass::ExteriorWall);
    let mut derived = authored_window(50, 1, Vec::new());
    derived.area_m2 = 2.0;
    derived.height_m = 1.0;
    derived.sill_height_m = 0.5;
    let mut authored = derived.clone();
    authored.vertices_m = vec![[1.0, 0.0, 0.5], [3.0, 0.0, 0.5], [3.0, 0.0, 1.6], [2.0, 0.0, 2.1], [1.0, 0.0, 1.6]];

    let derived_model = Model { surfaces: vec![host.clone()], fenestrations: vec![derived.clone()], ..Model::default() };
    let authored_model = Model { surfaces: vec![host], fenestrations: vec![authored.clone()], ..Model::default() };
    let rectangle = model_fenestration_polygon(&derived_model, &derived).expect("a derived aperture");
    let polygon = model_fenestration_polygon(&authored_model, &authored).expect("an authored aperture");
    assert_eq!(rectangle.len(), 4, "the same scalars alone still derive the bay rectangle");
    assert_eq!(polygon.len(), 5, "adding `vertices_m` replaces that rectangle rather than being ignored");
}
//#endregion 🧱️AuthoredPolygons

//#region 🧱️InteractionFlags
#[test]
fn hover_and_selection_publish_per_instance_flags_for_every_family() {
    let model = Model { zones: vec![zone(1, "Living")], surfaces: box_zone_surfaces(1, 40, [4.0, 3.0, 2.5]), fenestrations: vec![authored_window(50, 42, vec![[1.0, 0.0, 0.5], [3.0, 0.0, 0.5], [3.0, 0.0, 1.6], [1.0, 0.0, 1.6]])], ..Model::default() };
    for target in ["40", "50", "1"] {
        let ids = vec![target.to_string()];
        let (plain, _) = energy_model_scene_parts(&model, &EnergySceneStyle::default());
        let (hovered_json, instances) = energy_model_scene_parts(&model, &EnergySceneStyle { hovered_ids: &ids, ..EnergySceneStyle::default() });
        let (selected_json, _) = energy_model_scene_parts(&model, &EnergySceneStyle { selected_ids: &ids, ..EnergySceneStyle::default() });
        assert_eq!(plain, hovered_json, "hover paint is host-owned for {target}");
        assert_eq!(plain, selected_json, "selection paint is host-owned for {target}");
        let rows = parsed(&instances);
        let row = rows.as_array().expect("instances").iter().find(|row| row["id"].as_str() == Some(target)).expect("the hovered instance");
        assert_eq!(row["hovered"].as_bool(), Some(true), "the per-instance hover flag keeps the host chrome right: {row}");
    }
}

#[test]
fn a_selected_and_hovered_entity_still_carries_both_flags() {
    let model = Model { surfaces: vec![quad(5, SurfaceClass::ExteriorWall)], ..Model::default() };
    let ids = vec!["5".to_string()];
    let (plain, _) = energy_model_scene_parts(&model, &EnergySceneStyle::default());
    let (both, instances) = energy_model_scene_parts(&model, &EnergySceneStyle { selected_ids: &ids, hovered_ids: &ids, ..EnergySceneStyle::default() });
    assert_eq!(parsed(&both)[0]["data"]["colors"], parsed(&plain)[0]["data"]["colors"], "vertex colours stay on the class swatch");
    let rows = parsed(&instances);
    let row = &rows.as_array().expect("instances")[0];
    assert_eq!((row["selected"].as_bool(), row["hovered"].as_bool()), (Some(true), Some(true)), "both flags are still published for the host's chrome: {row}");
}
//#endregion 🧱️InteractionFlags
