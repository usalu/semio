use super::*;


/// 🛡️ Selected solid operations refuse foreign components before altering either body.
#[test]
fn brep_selected_solid_operations_require_scoped_components_and_positive_parameters() {
    use parry3d::shape::Shape;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/✏️selected-solid-scope/🔣️.json")).unwrap();
    let dimensions: [f64; 3] = std::array::from_fn(|axis| fixture["box"][axis].as_f64().unwrap());
    let oracle = parry3d::shape::Cuboid::new(parry3d::math::Vector::new(dimensions[0] as f32 / 2.0, dimensions[1] as f32 / 2.0, dimensions[2] as f32 / 2.0));
    let volume = f64::from(oracle.mass_properties(1.0).mass());
    assert_eq!(volume, fixture["volume"].as_f64().unwrap());
    for row in fixture["refusals"].as_array().unwrap() {
        let mut kernel = Brep::new();
        let source = kernel.box_prim_sync(dimensions[0], dimensions[1], dimensions[2]).unwrap();
        let foreign = kernel.box_prim_sync(dimensions[0], dimensions[1], dimensions[2]).unwrap();
        let topology = kernel.deconstruct_sync(&source).unwrap();
        let foreign_topology = kernel.deconstruct_sync(&foreign).unwrap();
        let operation = row["operation"].as_str().unwrap();
        let is_face = operation == "shell";
        let selected: Vec<_> = row["selection"].as_array().unwrap().iter().flat_map(|selection| if selection.as_str() == Some("allCurrent") { if is_face { topology.faces.clone() } else { topology.edges.clone() } } else { vec![match selection.as_str().unwrap() {
            "current" => if is_face { topology.faces[0].clone() } else { topology.edges[0].clone() },
            "foreign" => if is_face { foreign_topology.faces[0].clone() } else { foreign_topology.edges[0].clone() },
            "wrongDomain" => if is_face { topology.edges[0].clone() } else { topology.faces[0].clone() },
            "unknown" => GeometryHandle("0".repeat(64)),
            _ => panic!("unknown selection case"),
        }] }).collect();
        let parameter = match row["parameter"].as_str().unwrap() { "positive" => 0.1, "zero" => 0.0, "negative" => -0.1, "nan" => f64::NAN, "infinity" => f64::INFINITY, _ => panic!("unknown parameter case") };
        let counts = [kernel.body.vertices.len(), kernel.body.edges.len(), kernel.body.faces.len(), kernel.body.solids.len(), kernel.live.len()];
        let result = match operation { "filletEdges" => kernel.fillet_edges_sync(&source, &selected, parameter), "chamferEdges" => kernel.chamfer_edges_sync(&source, &selected, parameter), "shell" => kernel.shell_sync(&source, parameter, &selected), _ => panic!("unknown operation") };
        let error = result.expect_err(&format!("must refuse {row}"));
        match row["error"].as_str().unwrap() { "invalidInput" => assert!(matches!(error, BrepError::InvalidInput(_)), "{row}: {error}"), "missingHandle" => assert!(matches!(error, BrepError::MissingHandle(_)), "{row}: {error}"), _ => panic!("unknown error case") }
        assert_eq!(counts, [kernel.body.vertices.len(), kernel.body.edges.len(), kernel.body.faces.len(), kernel.body.solids.len(), kernel.live.len()], "{row}: refusal must precede mutation");
        assert_eq!(kernel.deconstruct_sync(&source).unwrap(), topology);
        assert_eq!(kernel.deconstruct_sync(&foreign).unwrap(), foreign_topology);
        assert_eq!(kernel.volume_sync(&source).unwrap(), volume);
        assert_eq!(kernel.volume_sync(&foreign).unwrap(), volume);
    }
}


/// 🧩️ Every shape kind exposes only its own stable topology through the same kernel family.
#[test]
fn brep_deconstruction_covers_every_topological_shape_kind() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎯️component-picking/🔣️.json")).unwrap();
    let mut kernel=Brep::new();
    let width=fixture["wire"]["width"].as_f64().unwrap();let height=fixture["wire"]["height"].as_f64().unwrap();
    let wire=kernel.rectangle_wire_sync(width,height).unwrap();let face=kernel.face_from_wire_sync(&wire).unwrap();
    let solid=kernel.box_prim_sync(width,height,1.0).unwrap();let components=kernel.deconstruct_sync(&solid).unwrap();
    let second=kernel.box_prim_sync(width,height,1.0).unwrap();let compound=kernel.compound_sync(&[solid.clone(),second]).unwrap();let shared=kernel.compound_sync(&[solid.clone(),solid.clone()]).unwrap();
    let vertex=kernel.vertex_sync([0.0,0.0,0.0]).unwrap();let curve=kernel.line_curve_sync([0.0,0.0,0.0],[width,0.0,0.0]).unwrap();let surface=kernel.plane_surface_sync([0.0,0.0,0.0],[0.0,0.0,1.0]).unwrap();
    for row in fixture["deconstruction"].as_array().unwrap() {
        let kind=row["kind"].as_str().unwrap();let shape=match kind {"wire"=>&wire,"face"=>&face,"edge"=>&components.edges[0],"vertex"=>&vertex,"shell"=>&components.shells[0],"solid"=>&solid,"compound"=>&compound,"shared-compound"=>&shared,"curve"=>&curve,"surface"=>&surface,_=>panic!("unknown fixture shape")};
        let topology=kernel.deconstruct_sync(shape).unwrap_or_else(|error|panic!("{kind}: {error}"));let repeated=kernel.deconstruct_sync(shape).unwrap();
        let counts=[topology.vertices.len(),topology.edges.len(),topology.faces.len(),topology.shells.len()];assert_eq!(serde_json::json!(counts),row["counts"],"{kind}");
        for(handles,again,expected)in [(&topology.vertices,&repeated.vertices,GeometryKind::Vertex),(&topology.edges,&repeated.edges,GeometryKind::Edge),(&topology.faces,&repeated.faces,GeometryKind::Face),(&topology.shells,&repeated.shells,GeometryKind::Shell)] {
            assert_eq!(handles,again,"{kind} stable handles");let mut labels=std::collections::BTreeSet::new();
            for handle in handles {assert_eq!(kernel.kind_sync(handle).unwrap(),expected);let label=kernel.label_of(handle).unwrap();assert!(labels.insert(label));assert_eq!(kernel.handle_for_label(label).as_ref(),Some(handle));}
        }
        if kind=="wire" {let mut length=0.0;for handle in &topology.edges {let id=kernel.edge_id(handle).unwrap();let edge=kernel.body.edges.get(id).unwrap();let a=kernel.body.vertices.get(edge.v0).unwrap().position;let b=kernel.body.vertices.get(edge.v1).unwrap().position;let oracle=parry3d::shape::Segment::new(parry3d::math::Point::new(a.x as f32,a.y as f32,a.z as f32),parry3d::math::Point::new(b.x as f32,b.y as f32,b.z as f32)).length();assert!((kernel.length_sync(handle).unwrap()-f64::from(oracle)).abs()<1e-6);length+=f64::from(oracle);}assert_eq!(length,fixture["wire"]["perimeter"].as_f64().unwrap());}
    }
}


#[test]
fn brep_closest_point_honors_curved_face_trims() {
    use parry3d::query::PointQuery;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎯️component-picking/🔣️.json")).unwrap();
    let fixture = &fixture["curvedClosestPoint"];
    let radius = fixture["radius"].as_f64().unwrap();
    let height = fixture["height"].as_f64().unwrap();
    let oracle = parry3d::shape::Cylinder::new(height as f32 / 2.0, radius as f32);
    for row in fixture["cases"].as_array().unwrap() {
        let query = parry3d::math::Point::new(row["query"][0].as_f64().unwrap() as f32, (row["query"][2].as_f64().unwrap() - height / 2.0) as f32, row["query"][1].as_f64().unwrap() as f32);
        let projected = oracle.project_local_point(&query, false).point;
        let projected = [f64::from(projected.x), f64::from(projected.z), f64::from(projected.y) + height / 2.0];
        for axis in 0..3 { assert!((projected[axis] - row["point"][axis].as_f64().unwrap()).abs() < 1e-6); }
    }
    let mut kernel = Brep::new();
    let solid = kernel.cylinder_prim_sync(radius, height).unwrap();
    let face = kernel.deconstruct_sync(&solid).unwrap().faces.into_iter().find(|handle| {
        let Entity::Face(id) = kernel.entity(handle).unwrap() else { return false };
        matches!(kernel.body.surfaces.get(kernel.body.faces.get(*id).unwrap().surface), Some(Surface::Cylinder { .. }))
    }).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let query = std::array::from_fn(|axis| row["query"][axis].as_f64().unwrap());
        for shape in [&face, &solid] {
            let closest = kernel.closest_point_sync(shape, query).unwrap();
            for axis in 0..3 { assert!((closest.point[axis] - row["point"][axis].as_f64().unwrap()).abs() < 1e-6, "query{query:?} axis{axis}: {:?}", closest.point); }
            assert!((closest.distance - row["distance"].as_f64().unwrap()).abs() < 1e-6);
        }
    }
}


/// ♻️ Disposing the only handle reaching a body's geometry must actually free it (audit §5.3:
/// "dispose is not equivalent to deleting geometry" under the old registry-only dispose).
#[semio_framework_async_macros::async_test]
async fn dispose_reclaims_unreferenced_topology() {
    let mut k = Brep::new();
    let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
    k.dispose(&solid);
    assert_eq!(k.registry_len(), 0);
    let counts = k.body.entity_counts();
    assert_eq!(counts.vertices, 0);
    assert_eq!(counts.edges, 0);
    assert_eq!(counts.faces, 0);
    assert_eq!(counts.shells, 0);
    assert_eq!(counts.solids, 0);
}


/// ⚖️ LAW: the validation gate judges the SHAPE it was asked about, never the arena around it.
///
/// 🐛️ It used to run `validate_body` over the whole body and block on any error-class issue found
/// anywhere, and this kernel is process-wide: one invalid solid — a boolean result whose void shell
/// is not inverted, say — refused the tessellation of every other live handle for as long as it
/// stayed live. In the procedural playground that read as six of eight examples settling on an empty
/// preview payload reported as `idle` (`📓️slider-reevaluation-correctness-2026-09-15.md`).
#[semio_framework_async_macros::async_test]
async fn the_validate_gate_judges_the_shape_it_was_asked_about_and_not_the_arena_around_it() {
    let mut kernel = Brep::new();
    let healthy = kernel.box_prim_sync(1.0, 1.0, 1.0).expect("a plain box is valid geometry");
    assert!(kernel.validate_gate_sync(&healthy).is_ok(), "a plain box must pass its own gate");
    let solid = kernel.solid_id(&healthy).expect("the box is a solid");
    let stranger = kernel.box_prim_sync(2.0, 2.0, 2.0).expect("a second box");
    let stranger_solid = kernel.solid_id(&stranger).expect("the second box is a solid");
    assert_ne!(solid, stranger_solid, "the two boxes are different solids");
    let outer = kernel.body.solids.get(stranger_solid).expect("stranger solid").outer;
    let faces = kernel.body.shells.get(outer).expect("stranger shell").faces.clone();
    for face in faces {
        if let Some(entry) = kernel.body.faces.get_mut(face) {
            entry.flipped = !entry.flipped;
        }
    }
    let stranger_issues = kernel.validate_gate_sync(&stranger).expect_err("the broken box must fail its own gate");
    assert!(!stranger_issues.is_empty(), "the broken box's gate must name at least one issue");
    assert!(kernel.validate_gate_sync(&healthy).is_ok(), "the healthy box must still pass while a STRANGER is broken: {stranger_issues:?}");
    eprintln!("scoped gate: healthy=ok stranger={:?}", stranger_issues.iter().map(|issue| format!("{}:{}", issue.entity, issue.code)).collect::<Vec<_>>());
}
