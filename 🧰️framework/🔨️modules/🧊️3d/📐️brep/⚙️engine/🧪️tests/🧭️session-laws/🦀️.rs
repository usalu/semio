//! 🧭️ Public session laws for bounded publication, topology, measures, and operation admission.
use semio_framework_3d_brep_analysis::SurfaceKind;
use semio_framework_3d_brep_engine::{AnalyticCurveParameters, Brep, BrepError, BrepKernel, GeometryHandle, GeometryKind};
use semio_framework_3d_brep_representation::topology::EntityRef;

/// 🌘️ Publishes a rational arc from bounded construction steps and matches a Parry distance oracle.
#[test]
fn funded_arc_keeps_its_recipe_and_grants_across_the_registry_boundary() {
    use semio_framework_job::OriginalRetirementPump;
    use semio_framework_value::RetainedCloneGrant;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🌙️retained-arc/🔣️.json")).unwrap();
    let center = std::array::from_fn(|axis| fixture["center"][axis].as_f64().unwrap());
    let normal = std::array::from_fn(|axis| fixture["normal"][axis].as_f64().unwrap());
    let radius = fixture["radius"].as_f64().unwrap();
    let [start, end] = std::array::from_fn(|index| fixture["angles"][index].as_f64().unwrap());
    let mut kernel = Brep::new();
    let mut plan = kernel.plan_analytic_arc(center, normal, radius, start, end).unwrap();
    let grant = RetainedCloneGrant {
        maximum_items: 1, maximum_copy_bytes: 1024, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 0, maximum_depth: 64,
    };
    for _ in 0..64 {
        if plan.construction_demand().is_none() { break; }
        assert!(!kernel.contains_live_handle(plan.handle()));
        let (progress, _) = OriginalRetirementPump::new()
            .turn(64, 100, || Some(1), grant).unwrap()
            .with_effect(|effect| plan.step_construction(None, effect)).unwrap().unwrap();
        assert!(progress.unwrap().fits(grant));
    }
    assert!(plan.construction_demand().is_none());
    let handle = GeometryHandle(plan.handle().to_owned());
    for _ in 0..64 {
        if kernel.analytic_publication_demand(&plan, grant.maximum_copy_bytes).unwrap().capacity_bytes == 0 { break; }
        assert!(kernel.reserve_analytic_publication(&plan, grant).unwrap().fits(grant));
    }
    assert!(kernel.publish_analytic_geometry(&mut plan, handle.0.clone(), grant).unwrap().fits(grant));
    for fraction in fixture["sampleFractions"].as_array().unwrap() {
        let point = kernel.curve_point_sync(&handle, start + fraction.as_f64().unwrap() * (end - start)).unwrap();
        let to_point = |p: [f64; 3]| parry3d::math::Point::new(p[0] as f32, p[1] as f32, p[2] as f32);
        let observed = parry3d::na::distance(&to_point(center), &to_point(point));
        assert!((f64::from(observed) - radius).abs() < 1e-6);
        assert!((point[2] - center[2]).abs() < 1e-12);
    }
}



#[test]
fn analytic_boundaries_fill_and_extrude_with_their_full_trim_ranges() {
    use parry3d::shape::Shape;
    let mut kernel = Brep::new();
    let center = [1.0, 2.0, 3.0];
    let normal = [0.0, 0.0, 1.0];
    let cold = Brep::new().circle_curve_sync(center, normal, 2.0).unwrap();
    let mut plan = kernel.plan_analytic_curve(AnalyticCurveParameters::Circle { center, normal, radius: 2.0 }).unwrap();
    assert_eq!(plan.handle(), cold.as_str());
    assert!(!kernel.contains_live_handle(plan.handle()));
    let grant = semio_framework_value::RetainedCloneGrant {
        maximum_items: 1, maximum_copy_bytes: 1024, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 0, maximum_depth: 64,
    };
    loop {
        let demand = kernel.analytic_publication_demand(&plan, grant.maximum_copy_bytes).unwrap();
        if demand.capacity_bytes == 0 { break; }
        let progress = kernel.reserve_analytic_publication(&plan, grant).unwrap();
        assert!(progress.fits(grant));
    }
    let circle = GeometryHandle(plan.handle().to_owned());
    assert!(kernel.publish_analytic_geometry(&mut plan, circle.0.clone(), grant).unwrap().fits(grant));
    let solid = kernel.extrude_wire_sync(&circle, [0.0, 0.0, 4.0]).unwrap();
    let expected = f64::from(parry3d::shape::Cylinder::new(2.0, 2.0).mass_properties(1.0).mass());
    let volume = kernel.volume_sync(&solid).unwrap();
    assert!((volume - expected).abs() < 0.005, "{volume} != {expected}");
    assert_eq!(kernel.kind_sync(&solid).unwrap(), GeometryKind::Solid);
    assert!(!kernel.tessellate_sync(&solid, 0.01).unwrap().position.is_empty());

    let ellipse = kernel.ellipse_curve_sync([3.0, 0.0, 0.0], [1.0, 0.0, 0.0], 2.0, 1.0).unwrap();
    let face = kernel.face_from_wire_sync(&ellipse).unwrap();
    let area = kernel.area_sync(&face).unwrap();
    assert!((area - std::f64::consts::TAU).abs() < 0.005, "{area}");
    assert!(!kernel.tessellate_sync(&face, 0.01).unwrap().position.is_empty());
    assert!(kernel.planar_face_from_wire_sync(&ellipse).is_err());

    let open = kernel.arc_curve_sync([0.0; 3], [0.0, 0.0, 1.0], 2.0, 0.0, std::f64::consts::FRAC_PI_2).unwrap();
    let live = kernel.registry_len();
    assert!(matches!(
        kernel.extrude_wire_sync(&open, [0.0, 0.0, 1.0]),
        Err(BrepError::Operation(reason)) if reason.contains("profile boundary must be closed")
    ));
    assert_eq!(kernel.registry_len(), live);
}


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
        let session_counts = |kernel: &Brep| {
            let body = kernel.tessellation_body();
            [body.vertices.len(), body.edges.len(), body.faces.len(), body.solids.len(), kernel.registry_len()]
        };
        let counts = session_counts(&kernel);
        let result = match operation { "filletEdges" => kernel.fillet_edges_sync(&source, &selected, parameter), "chamferEdges" => kernel.chamfer_edges_sync(&source, &selected, parameter), "shell" => kernel.shell_sync(&source, parameter, &selected), _ => panic!("unknown operation") };
        let error = result.expect_err(&format!("must refuse {row}"));
        match row["error"].as_str().unwrap() { "invalidInput" => assert!(matches!(error, BrepError::InvalidInput(_)), "{row}: {error}"), "missingHandle" => assert!(matches!(error, BrepError::MissingHandle(_)), "{row}: {error}"), _ => panic!("unknown error case") }
        assert_eq!(counts, session_counts(&kernel), "{row}: refusal must precede mutation");
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
        if kind == "wire" {
            let mut length = 0.0;
            let body = kernel.tessellation_body();
            for handle in &topology.edges {
                let Some(EntityRef::Edge(id)) = kernel.preview_validation_root(handle.as_str(), 0).unwrap() else {
                    panic!("wire member must resolve to an edge");
                };
                let edge = body.edges.get(id).unwrap();
                let point = |vertex| {
                    let point = body.vertices.get(vertex).unwrap().position;
                    parry3d::math::Point::new(point.x as f32, point.y as f32, point.z as f32)
                };
                let oracle = f64::from(parry3d::shape::Segment::new(point(edge.v0), point(edge.v1)).length());
                assert!((kernel.length_sync(handle).unwrap() - oracle).abs() < 1e-6);
                length += oracle;
            }
            assert_eq!(length, fixture["wire"]["perimeter"].as_f64().unwrap());
        }
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
    let side = kernel.face_table_sync(&solid, 1e-6).unwrap().into_iter()
        .find(|row| row.surface_kind == SurfaceKind::Cylinder)
        .unwrap();
    let face = kernel.deconstruct_sync(&solid).unwrap().faces.into_iter()
        .find(|handle| kernel.original_geometry_label(handle.as_str()).ok() == Some(side.label))
        .unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let query = std::array::from_fn(|axis| row["query"][axis].as_f64().unwrap());
        for shape in [&face, &solid] {
            let closest = kernel.closest_point_sync(shape, query).unwrap();
            for axis in 0..3 { assert!((closest.point[axis] - row["point"][axis].as_f64().unwrap()).abs() < 1e-6, "query{query:?} axis{axis}: {:?}", closest.point); }
            assert!((closest.distance - row["distance"].as_f64().unwrap()).abs() < 1e-6);
        }
    }
}

/// 🧮️ Pattern admission refuses oversized instance counts before building any transform.
#[test]
fn oversized_patterns_are_refused_at_admission() {
    let mut kernel = Brep::new();
    let shape = kernel.box_prim_sync(1., 1., 1.).unwrap();
    assert!(kernel.linear_pattern_sync(&shape, [1., 0., 0.], 2., 1 << 40).is_err());
    assert!(kernel.grid_pattern_sync(&shape, [1., 0., 0.], [0., 1., 0.], (2., 2.), usize::MAX, usize::MAX).is_err());
}
