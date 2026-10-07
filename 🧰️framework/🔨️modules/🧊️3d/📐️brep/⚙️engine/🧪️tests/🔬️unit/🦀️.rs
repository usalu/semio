use super::*;

/// 🛡️ Selected solid operations refuse foreign components before altering either body.
#[test]
fn brep_selected_solid_operations_require_scoped_components_and_positive_parameters() {
    use parry3d::shape::Shape;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/✏️selected-solid-scope/🔣️.json")).unwrap();
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
    println!("[DEBUG] selectedSolidScope refusals={} mutationFree=true independentParryVolume={volume}", fixture["refusals"].as_array().unwrap().len());
}

/// 🧮️ Selected features preserve their source and match independent primitive mass properties.
#[test]
fn brep_selected_solid_operations_match_independent_physical_volumes() {
    use parry3d::shape::Shape;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/✏️selected-solid-scope/🔣️.json")).unwrap();
    let dimensions: [f64; 3] = std::array::from_fn(|axis| fixture["box"][axis].as_f64().unwrap());
    let cuboid_volume = |dimensions: [f64; 3]| f64::from(parry3d::shape::Cuboid::new(parry3d::math::Vector::new(dimensions[0] as f32 / 2.0, dimensions[1] as f32 / 2.0, dimensions[2] as f32 / 2.0)).mass_properties(1.0).mass());
    let source_volume = cuboid_volume(dimensions);
    let edge_length = f64::from(parry3d::shape::Segment::new(parry3d::math::Point::origin(), parry3d::math::Point::new(dimensions[0] as f32, 0.0, 0.0)).length());
    for row in fixture["successes"].as_array().unwrap() {
        println!("[DEBUG] selectedSolidPhysical begin={row}");
        let mut kernel = Brep::new();
        let source = kernel.box_prim_sync(dimensions[0], dimensions[1], dimensions[2]).unwrap();
        let topology = kernel.deconstruct_sync(&source).unwrap();
        let size = row["size"].as_f64().unwrap();
        let selection = row["selection"].as_str().unwrap();
        let index = row["index"].as_u64().unwrap() as usize;
        let expected = match row["operation"].as_str().unwrap() {
            "filletEdges" => source_volume - cuboid_volume([size, size, edge_length]) + f64::from(parry3d::shape::Cylinder::new(edge_length as f32 / 2.0, size as f32).mass_properties(1.0).mass()) / 4.0,
            "chamferEdges" => source_volume - cuboid_volume([size, size, edge_length]) / 2.0,
            "shell" => source_volume - cuboid_volume(std::array::from_fn(|axis| dimensions[axis] - if selection == "singleFace" && axis == 2 - index / 2 { size } else { 2.0 * size })),
            _ => panic!("unknown operation"),
        };
        assert!((expected - row["volume"].as_f64().unwrap()).abs() < 1e-6, "{row}: independent Parry volume {expected}");
        let output = match row["operation"].as_str().unwrap() {
            "filletEdges" => kernel.fillet_edges_sync(&source, std::slice::from_ref(&topology.edges[index]), size),
            "chamferEdges" => kernel.chamfer_edges_sync(&source, std::slice::from_ref(&topology.edges[index]), size),
            "shell" => kernel.shell_sync(&source, size, if selection == "closed" { &[] } else { std::slice::from_ref(&topology.faces[index]) }),
            _ => panic!("unknown operation"),
        }.unwrap_or_else(|error| panic!("{row}: {error}"));
        assert_ne!(source, output);
        assert_eq!(kernel.kind_sync(&output).unwrap(), GeometryKind::Solid);
        kernel.validate_gate_sync(&output).unwrap_or_else(|issues| panic!("{row}: {issues:?}"));
        kernel.validate_gate_sync(&source).unwrap_or_else(|issues| panic!("{row}: source {issues:?}"));
        assert_eq!(kernel.deconstruct_sync(&source).unwrap(), topology);
        assert_eq!(kernel.volume_sync(&source).unwrap(), source_volume);
        let actual = kernel.volume_sync(&output).unwrap();
        assert!((actual - expected).abs() < 1e-6, "{row}: actual {actual}, oracle {expected}");
        println!("[DEBUG] selectedSolidPhysical complete={row} volume={actual}");
    }
    println!("[DEBUG] selectedSolidPhysical operations=filletEdges,chamferEdges,shell cases=9 unchangedSource=true validClosedTopology=true independentParry=true");
}

/// 🎯️ Analytic preview picks retain full labels independently of numeric renderer indices.
#[test]
fn brep_preview_component_picks_preserve_lossless_face_and_edge_labels() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️component-picking/🔣️.json")).unwrap();
    let source = semio_framework_pack_json::parse(&fixture["transfer"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let transfer = <MeshTransfer as protocol::value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&source)).unwrap();
    let mesh = mesh_data_from_mesh_transfer(&transfer).unwrap();
    assert_eq!(serde_json::json!(mesh.face_ids), fixture["expected"]["faceIds"]);
    assert_eq!(serde_json::json!(mesh.edge_ids), fixture["expected"]["edgeIds"]);
    let encoded: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::Value::from(mesh.clone()).to_string()).unwrap();
    assert_eq!(encoded["componentReferences"], fixture["expected"]["componentReferences"]);
    let area: f32 = mesh.indices.chunks_exact(3).map(|indices| {
        let points = [indices[0], indices[1], indices[2]].map(|index| { let offset=index as usize*3; parry3d::math::Point::new(mesh.positions[offset],mesh.positions[offset+1],mesh.positions[offset+2]) });
        parry3d::shape::Triangle::new(points[0],points[1],points[2]).area()
    }).sum();
    assert_eq!(area as f64, fixture["expected"]["area"].as_f64().unwrap());
    println!("[DEBUG] BRep analytic pick domains=2 losslessLabels=4 independentParryArea={area}");
}

/// 🪢️ Analytic wire edges keep resolvable labels and independent segment lengths.
#[test]
fn brep_preview_component_wire_edges_keep_resolvable_labels() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎯️component-picking/🔣️.json")).unwrap();
    let row=&fixture["wire"];
    let mut kernel=Brep::new();
    let wire=kernel.rectangle_wire_sync(row["width"].as_f64().unwrap(),row["height"].as_f64().unwrap()).unwrap();
    let transfer=kernel.tessellate_sync(&wire,0.1).unwrap();
    assert_eq!(transfer.edge_groups.len(),row["edgeCount"].as_u64().unwrap() as usize);
    let mesh=mesh_data_from_mesh_transfer(&transfer).unwrap();
    let labels=mesh.component_references.get("edge").unwrap();
    assert_eq!(labels.len(),transfer.edge_groups.len());
    assert_eq!(mesh.edge_ids.len(),mesh.edge_positions.len()/6);
    for label in labels {let label=PersistentLabel(label.parse().unwrap());let handle=kernel.handle_for_label(label).expect("wire edge label resolves in the same kernel family");assert_eq!(kernel.kind_sync(&handle).unwrap(),GeometryKind::Edge);assert_eq!(kernel.label_of(&handle),Some(label));}
    let perimeter:f32=mesh.edge_positions.chunks_exact(6).map(|points|parry3d::shape::Segment::new(parry3d::math::Point::new(points[0],points[1],points[2]),parry3d::math::Point::new(points[3],points[4],points[5])).length()).sum();
    assert_eq!(perimeter as f64,row["perimeter"].as_f64().unwrap());
    println!("[DEBUG] Analytic wire pick references={} independentParryPerimeter={perimeter}",labels.len());
}

/// 🧩️ Every shape kind exposes only its own stable topology through the same kernel family.
#[test]
fn brep_deconstruction_covers_every_topological_shape_kind() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎯️component-picking/🔣️.json")).unwrap();
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
    println!("[DEBUG] BRep deconstruction shapeKinds=10 stableLabels=true sharedTopologyDeduplicated=true independentParryPerimeter=true");
}

/// 📏️ Neutral profile and compound fixtures exercise the existing measurement owner.
fn verify_topology_measurement(metric: &str) {
    use parry3d::shape::Shape;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️component-picking/🔣️.json")).unwrap();
    let mut kernel = Brep::new();
    let width = fixture["wire"]["width"].as_f64().unwrap();
    let height = fixture["wire"]["height"].as_f64().unwrap();
    let wire = kernel.rectangle_wire_sync(width, height).unwrap();
    let face = kernel.face_from_wire_sync(&wire).unwrap();
    let solid = kernel.box_prim_sync(width, height, 1.0).unwrap();
    let components = kernel.deconstruct_sync(&solid).unwrap();
    let second = kernel.box_prim_sync(width, height, 1.0).unwrap();
    let compound = kernel.compound_sync(&[solid.clone(), second]).unwrap();
    let shared = kernel.compound_sync(&[solid.clone(), solid.clone()]).unwrap();
    let vertex = kernel.vertex_sync([0.0, 0.0, 0.0]).unwrap();
    let curve = kernel.line_curve_sync([0.0, 0.0, 0.0], [width, 0.0, 0.0]).unwrap();
    let surface = kernel.plane_surface_sync([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]).unwrap();
    let circle = kernel.circle_curve_sync([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], fixture["measurements"]["circleRadius"].as_f64().unwrap()).unwrap();
    let shapes = std::collections::BTreeMap::from([("wire", wire), ("face", face), ("shell", components.shells[0].clone()), ("solid", solid), ("compound", compound), ("shared-compound", shared), ("vertex", vertex), ("curve", curve), ("surface", surface), ("circle", circle)]);
    let oracle = parry3d::shape::Cuboid::new(parry3d::math::Vector::new(width as f32 / 2.0, height as f32 / 2.0, 0.5));
    assert_eq!(f64::from(oracle.mass_properties(1.0).mass()), fixture["measurements"]["volume"][0]["value"].as_f64().unwrap());
    let a = parry3d::math::Point::new(0.0, 0.0, 0.0);
    let b = parry3d::math::Point::new(width as f32, 0.0, 0.0);
    let c = parry3d::math::Point::new(width as f32, height as f32, 0.0);
    let d = parry3d::math::Point::new(0.0, height as f32, 0.0);
    let area = f64::from(parry3d::shape::Triangle::new(a, b, c).area() + parry3d::shape::Triangle::new(a, c, d).area());
    assert_eq!(area, fixture["measurements"]["area"][0]["value"].as_f64().unwrap());
    let perimeter = [(a, b), (b, c), (c, d), (d, a)].into_iter().map(|(a, b)| f64::from(parry3d::shape::Segment::new(a, b).length())).sum::<f64>();
    assert_eq!(perimeter, fixture["measurements"]["length"][0]["value"].as_f64().unwrap());
    for row in fixture["measurements"][metric].as_array().unwrap() {
        let kind = row["kind"].as_str().unwrap();
        let shape = &shapes[kind];
        let value = match metric { "length" => kernel.length_sync(shape), "area" => kernel.area_sync(shape), "volume" => kernel.volume_sync(shape), _ => unreachable!() }.unwrap_or_else(|error| panic!("{metric} {kind}: {error}"));
        assert!((value - row["value"].as_f64().unwrap()).abs() < 1e-6, "{metric} {kind}: {value}");
    }
    if metric == "length" { for kind in fixture["measurements"]["lengthRefusals"].as_array().unwrap() { assert!(kernel.length_sync(&shapes[kind.as_str().unwrap()]).is_err()); } }
    println!("[DEBUG] BRep measurement={metric} cases={} independentParry=true sharedTopologyDeduplicated=true", fixture["measurements"][metric].as_array().unwrap().len());
}

#[test]
fn brep_length_measures_profiles_and_topological_boundaries() { verify_topology_measurement("length"); }

#[test]
fn brep_area_measures_compound_surface_topology() { verify_topology_measurement("area"); }

#[test]
fn brep_volume_measures_compound_solid_topology() { verify_topology_measurement("volume"); }

/// 🧭️ Neutral topology closest points agree with independent Parry segment, triangle, and box projections.
#[test]
fn brep_closest_point_supports_profiles_and_compounds() {
    use parry3d::query::PointQuery;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️component-picking/🔣️.json")).unwrap();
    let width = fixture["wire"]["width"].as_f64().unwrap();
    let height = fixture["wire"]["height"].as_f64().unwrap();
    let query = std::array::from_fn(|axis| fixture["closestPoint"]["query"][axis].as_f64().unwrap());
    let point = parry3d::math::Point::new(query[0] as f32, query[1] as f32, query[2] as f32);
    let a = parry3d::math::Point::new(0.0, 0.0, 0.0);
    let b = parry3d::math::Point::new(width as f32, 0.0, 0.0);
    let c = parry3d::math::Point::new(width as f32, height as f32, 0.0);
    let edge = parry3d::shape::Segment::new(a, b).project_local_point(&point, false).point;
    let face = parry3d::shape::Triangle::new(a, b, c).project_local_point(&point, false).point;
    let solid = parry3d::shape::Cuboid::new(parry3d::math::Vector::new(width as f32 / 2.0, height as f32 / 2.0, 0.5)).project_point(&parry3d::math::Isometry::translation(width as f32 / 2.0, height as f32 / 2.0, 0.5), &point, false).point;
    for row in fixture["closestPoint"]["cases"].as_array().unwrap() {
        let expected = match row["kind"].as_str().unwrap() { "vertex" => a, "edge" | "wire" => edge, "face" => face, _ => solid };
        for axis in 0..3 { assert!((f64::from(expected[axis]) - row["point"][axis].as_f64().unwrap()).abs() < 1e-6); }
        assert!((f64::from((point - expected).norm()) - row["distance"].as_f64().unwrap()).abs() < 1e-6);
    }
    let mut kernel = Brep::new();
    let wire = kernel.rectangle_wire_sync(width, height).unwrap();
    let face = kernel.face_from_wire_sync(&wire).unwrap();
    let edge = kernel.deconstruct_sync(&wire).unwrap().edges[0].clone();
    let solid = kernel.box_prim_sync(width, height, 1.0).unwrap();
    let shell = kernel.deconstruct_sync(&solid).unwrap().shells[0].clone();
    let second = kernel.box_prim_sync(width, height, 1.0).unwrap();
    let compound = kernel.compound_sync(&[solid.clone(), second]).unwrap();
    let shared = kernel.compound_sync(&[solid.clone(), solid.clone()]).unwrap();
    let vertex = kernel.vertex_sync([0.0, 0.0, 0.0]).unwrap();
    let shapes = std::collections::BTreeMap::from([("vertex", vertex), ("edge", edge), ("wire", wire), ("face", face), ("shell", shell), ("solid", solid), ("compound", compound), ("shared-compound", shared)]);
    for row in fixture["closestPoint"]["cases"].as_array().unwrap() {
        let kind = row["kind"].as_str().unwrap();
        let closest = kernel.closest_point_sync(&shapes[kind], query).unwrap_or_else(|error| panic!("closest {kind}: {error}"));
        for axis in 0..3 { assert!((closest.point[axis] - row["point"][axis].as_f64().unwrap()).abs() < 1e-6, "{kind} axis{axis}"); }
        assert!((closest.distance - row["distance"].as_f64().unwrap()).abs() < 1e-6, "{kind}");
    }
    eprintln!("[DEBUG] BRep closest-point shapeKinds=8 independentParry=true");
}

#[test]
fn brep_closest_point_honors_curved_face_trims() {
    use parry3d::query::PointQuery;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️component-picking/🔣️.json")).unwrap();
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
    eprintln!("[DEBUG] BRep curved closest-point trim cases=3 faceAndSolid=true independentParry=true");
}

#[test]
fn brep_error_contract_is_owned_and_stable() {
    let errors = [(BrepError::InvalidInput("mesh".into()), "invalid input: mesh"), (BrepError::MissingHandle("a1".into()), "missing handle: a1"), (BrepError::Operation("split".into()), "operation failed: split")];
    for (error, message) in errors {
        assert_eq!(error.to_string(), message);
        assert!(std::error::Error::source(&error).is_none());
    }
}

#[semio_framework_async_macros::async_test]
async fn native_box_volume() {
    let mut k = Brep::new();
    let solid = k.box_prim(1.0, 1.0, 1.0).unwrap();
    let v = k.volume(&solid).unwrap();
    assert!((v - 1.0).abs() < 1e-3, "volume {v}");
}

#[semio_framework_async_macros::async_test]
async fn native_fuse_disjoint() {
    let mut k = Brep::new();
    let a = k.box_prim(1.0, 1.0, 1.0).unwrap();
    let b = k.convex_hull(&[[2.0, 0.0, 0.0], [3.0, 0.0, 0.0], [3.0, 1.0, 0.0], [2.0, 1.0, 0.0], [2.0, 0.0, 1.0], [3.0, 0.0, 1.0], [3.0, 1.0, 1.0], [2.0, 1.0, 1.0]]).unwrap();
    let u = k.fuse(&a, &b).unwrap();
    let v = k.volume(&u).unwrap();
    assert!((v - 2.0).abs() < 1e-2, "volume {v}");
}

#[semio_framework_async_macros::async_test]
async fn wire_tessellate_preserves_edge_positions() {
    let mut k = Brep::new();
    let wire = k.rectangle_wire(2.0, 1.5).expect("wire");
    let transfer = k.tessellate_sync(&wire, 0.1).expect("tessellate");
    let data = mesh_data_from_mesh_transfer(&transfer).unwrap();
    assert!(data.edge_positions.len() >= 24, "edge_positions {}", data.edge_positions.len());
    assert!(data.indices.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn box_shell_produces_positive_volume() {
    let mut k = Brep::new();
    let box_h = k.box_prim(2.0, 2.0, 2.0).expect("box");
    let shelled = k.shell(&box_h, 0.2, &[]).expect("shell");
    let vol = k.volume(&shelled).expect("shell volume");
    assert!(vol > 0.0, "shelled volume {vol}");
    let mesh = k.tessellate_sync(&shelled, 0.1).expect("tessellate shell");
    assert!(!mesh.position.is_empty() || !mesh.edges.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn sphere_torus_cut_produces_preview_mesh() {
    let mut k2 = Brep::new();
    let sphere = k2.sphere_prim(2.2).expect("sphere");
    let torus = k2.torus_prim(2.0, 0.5).expect("torus");
    let tv = k2.volume(&torus).expect("torus volume");
    assert!(tv > 0.5, "torus volume too small: {tv}");
    let tmesh = k2.tessellate_sync(&torus, 0.15).expect("tessellate torus");
    assert!(tmesh.position.len() >= 9 && tmesh.index.len() >= 3, "torus mesh empty");
    let cut = k2.cut(&sphere, &torus).expect("cut");
    let mesh = k2.tessellate_sync(&cut, 0.15).expect("tessellate cut");
    assert!(mesh.position.len() >= 9 && mesh.index.len() >= 3, "cut mesh empty: pos={} idx={}", mesh.position.len(), mesh.index.len());
}

#[semio_framework_async_macros::async_test]
async fn arc_curve_respects_start_end_angles() {
    let mut k = Brep::new();
    let start = 0.0;
    let end = std::f64::consts::FRAC_PI_2;
    let radius = 2.0;
    let arc = k.arc_curve_sync([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], radius, start, end).expect("arc");
    let domain = k.curve_domain_sync(&arc).expect("domain");
    assert!((domain.min - start).abs() < 1e-9 && (domain.max - end).abs() < 1e-9);
    let p0 = k.curve_point_sync(&arc, start).expect("p0");
    let p1 = k.curve_point_sync(&arc, end).expect("p1");
    let r0 = (p0[0] * p0[0] + p0[1] * p0[1] + p0[2] * p0[2]).sqrt();
    let r1 = (p1[0] * p1[0] + p1[1] * p1[1] + p1[2] * p1[2]).sqrt();
    assert!((r0 - radius).abs() < 1e-4, "start radius {r0} from {p0:?}");
    assert!((r1 - radius).abs() < 1e-4, "end radius {r1} from {p1:?}");
    let chord = ((p1[0] - p0[0]).powi(2) + (p1[1] - p0[1]).powi(2) + (p1[2] - p0[2]).powi(2)).sqrt();
    let expected_chord = (2.0 * radius * radius * (1.0 - (end - start).cos())).sqrt();
    assert!((chord - expected_chord).abs() < 1e-3, "chord {chord} expected {expected_chord}");
    // Full circle would land start≈end; a quarter arc must keep endpoints distinct.
    assert!(chord > radius * 0.5);
    let kappa = k.curve_curvature_sync(&arc, (start + end) * 0.5).expect("kappa");
    assert!((kappa - 1.0 / radius).abs() < 5e-2, "kappa {kappa}");
}

#[semio_framework_async_macros::async_test]
async fn solid_face_loops_returns_a_quad_per_box_face() {
    let mut k = Brep::new();
    let solid = k.box_prim_sync(2.0, 3.0, 4.0).expect("box");
    let (positions, face_loops) = k.solid_face_loops_sync(&solid).expect("loops");
    assert_eq!(positions.len(), 8, "a box has 8 distinct vertices");
    assert_eq!(face_loops.len(), 6, "a box has 6 faces");
    for (outer, holes) in &face_loops {
        assert_eq!(outer.len(), 4, "each box face is a quad");
        assert!(holes.is_empty());
    }
}

#[semio_framework_async_macros::async_test]
async fn validate_returns_structured_json_report() {
    let mut k = Brep::new();
    let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
    let report = k.validate_sync(&solid).expect("validate");
    let value: serde_json::Value = serde_json::from_str(&report).expect("json");
    assert_eq!(value["ok"], true);
    assert_eq!(value["issueCount"], 0);
    assert!(value["issues"].as_array().unwrap().is_empty());
}

#[semio_framework_async_macros::async_test]
async fn deconstruct_includes_vertices_edges_and_faces() {
    let mut k = Brep::new();
    let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
    let topo = k.deconstruct_sync(&solid).expect("deconstruct");
    assert_eq!(topo.faces.len(), 6);
    assert_eq!(topo.edges.len(), 12);
    assert_eq!(topo.vertices.len(), 8);
}

/// 🏷️ Deconstruct must be idempotent: minting from each entity's [`PersistentLabel`] (not a
/// session counter) means calling it twice on the same untouched shape yields byte-identical
/// handles, and shells are now included (audit §5.4 — shell was not a first-class handle kind).
#[semio_framework_async_macros::async_test]
async fn deconstruct_twice_yields_identical_handles_and_includes_shells() {
    let mut k = Brep::new();
    let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
    let first = k.deconstruct_sync(&solid).expect("first deconstruct");
    let second = k.deconstruct_sync(&solid).expect("second deconstruct");
    assert_eq!(first.shells.len(), 1, "a box has exactly one outer shell");
    assert_eq!(first.vertices, second.vertices, "deconstruct must be idempotent per PersistentLabel");
    assert_eq!(first.edges, second.edges);
    assert_eq!(first.faces, second.faces);
    assert_eq!(first.shells, second.shells);
}

/// 🏷️ Registering unrelated geometry between the two calls must not perturb a single handle —
/// under the old counter-based `mint`, this alone would have changed every handle the second
/// `deconstruct` produces (audit §5.1: "deconstructing the same body repeatedly can mint new
/// handles repeatedly").
#[semio_framework_async_macros::async_test]
async fn deconstruct_handles_are_unaffected_by_unrelated_registrations_between_calls() {
    let mut k = Brep::new();
    let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
    let before = k.deconstruct_sync(&solid).expect("deconstruct before");
    let _ = k.line_curve_sync([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]).expect("unrelated curve");
    let _ = k.sphere_prim_sync(0.5).expect("unrelated sphere");
    let after = k.deconstruct_sync(&solid).expect("deconstruct after");
    assert_eq!(before.vertices, after.vertices);
    assert_eq!(before.edges, after.edges);
    assert_eq!(before.faces, after.faces);
    assert_eq!(before.shells, after.shells);
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

/// ♻️ `retain` is dispose-of-everything-else-then-compact: the dropped solid's own geometry
/// must be freed while the kept solid stays fully intact and resolvable.
#[semio_framework_async_macros::async_test]
async fn retain_compacts_everything_not_kept() {
    let mut k = Brep::new();
    let keep = k.box_prim_sync(1.0, 1.0, 1.0).expect("keep");
    let drop = k.box_prim_sync(2.0, 2.0, 2.0).expect("drop");
    let mut live = std::collections::HashSet::new();
    live.insert(keep.as_str().to_string());
    k.retain(&live);
    assert_eq!(k.registry_len(), 1);
    assert!(k.kind(&keep).is_ok());
    assert!(k.kind(&drop).is_err(), "the dropped handle must no longer resolve");
    let vol = k.volume(&keep).expect("kept solid stays intact");
    assert!((vol - 1.0).abs() < 1e-6, "volume {vol}");
}

/// 🧩️ Shell/compound are first-class handle kinds now (audit §5.4); `explode` is `compound`'s
/// inverse.
#[semio_framework_async_macros::async_test]
async fn solid_shells_compound_and_explode_round_trip() {
    let mut k = Brep::new();
    let a = k.box_prim_sync(1.0, 1.0, 1.0).expect("a");
    let b = k.box_prim_sync(1.0, 1.0, 1.0).expect("b");

    let shells = k.solid_shells(&a).expect("shells");
    assert_eq!(shells.len(), 1);
    assert_eq!(k.kind(&shells[0]).unwrap(), GeometryKind::Shell);

    let compound = k.compound(&[a.clone(), b.clone()]).expect("compound");
    assert_eq!(k.kind(&compound).unwrap(), GeometryKind::Compound);

    let members = k.explode(&compound).expect("explode");
    assert_eq!(members.len(), 2);
    for m in &members {
        assert!((k.volume(m).unwrap() - 1.0).abs() < 1e-6);
    }
}

/// 🏷️ `label`/`handle_for_label` are the ephemeral-handle↔persistent-label bridge the audit's
/// §5.5 required fix asks for.
#[semio_framework_async_macros::async_test]
async fn label_and_handle_for_label_round_trip() {
    let mut k = Brep::new();
    let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
    let label = k.label(&solid).expect("solid must carry a label");
    let via_label = k.handle_for_label(PersistentLabel(label));
    assert_eq!(via_label, Some(solid));
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
    // 🧨️ Break the STRANGER's outer shell orientation, which `validate_body` reports as a blocking
    // `shell-orientation-inward` issue against that solid alone.
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

/// ⚖️ LAW: a boolean leaves BOTH its inputs usable — it owns copies of everything it consumes.
///
/// 🐛️ The exact imprint engine used to imprint ON the operands' own faces and then
/// `remove_solid_and_orphans` both of them, and the trivial fast paths used to alias the operands'
/// faces into a second shell. Either way the sibling input's handle named a freed arena slot the
/// moment the boolean answered, so the next evaluation of the same graph — where the unchanged
/// input node is served from the evaluator's cache — failed with `missing handle: <digest>` on a
/// node whose own parameters never moved. Witnessed as `sphere-box-fuse` and `sphere-cut-with-torus`
/// refusing to follow their slider after the first edit
/// (`📓️brep-boolean-input-lifetime-2026-09-15.md`).
#[semio_framework_async_macros::async_test]
async fn a_boolean_leaves_both_of_its_input_solids_alive_for_the_next_evaluation() {
    for op in [BooleanOp::Unite, BooleanOp::Cut, BooleanOp::Intersect] {
        let mut kernel = Brep::new();
        let sphere = kernel.sphere_prim_sync(1.2).expect("sphere");
        let cube = kernel.box_prim_sync(1.5, 1.5, 1.5).expect("box");
        let sphere_volume = kernel.volume_sync(&sphere).expect("sphere volume");
        let cube_volume = kernel.volume_sync(&cube).expect("box volume");
        let result = kernel.boolean_sync(&sphere, &cube, op).expect("the boolean itself must answer");
        let after_sphere = kernel.volume_sync(&sphere).unwrap_or_else(|error| panic!("{op:?} freed its first input: {error}"));
        let after_cube = kernel.volume_sync(&cube).unwrap_or_else(|error| panic!("{op:?} freed its second input: {error}"));
        assert!((after_sphere - sphere_volume).abs() < 1e-9, "{op:?} changed its first input: {sphere_volume} → {after_sphere}");
        assert!((after_cube - cube_volume).abs() < 1e-9, "{op:?} changed its second input: {cube_volume} → {after_cube}");
        let live: std::collections::HashSet<String> = [&sphere, &cube, &result].iter().map(|handle| handle.as_str().to_string()).collect();
        kernel.retain(&live);
        assert_eq!(kernel.registry_len(), 3, "{op:?}: the claim named three live handles and the compaction must keep all three");
        kernel.tessellate_sync(&sphere, 0.5).unwrap_or_else(|error| panic!("{op:?}: the first input must still tessellate after a compaction: {error}"));
        kernel.tessellate_sync(&cube, 0.5).unwrap_or_else(|error| panic!("{op:?}: the second input must still tessellate after a compaction: {error}"));
        kernel.tessellate_sync(&result, 0.5).unwrap_or_else(|error| panic!("{op:?}: the result must still tessellate after a compaction: {error}"));
        let again = kernel.boolean_sync(&sphere, &cube, op).unwrap_or_else(|error| panic!("{op:?}: the same graph must evaluate a second time from the same cached inputs: {error}"));
        let first = kernel.volume_sync(&result).expect("first result volume");
        let second = kernel.volume_sync(&again).expect("second result volume");
        assert!((first - second).abs() < 1e-6, "{op:?}: re-evaluating the same inputs must answer the same solid: {first} vs {second}");
    }
}

// #region 🔁️AffineTransforms
/// 🔁️ The kernel-neutral affine-transform vectors — the same rows the `brep_invoke` bridge law
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/📐️brep-invoke`), the CAD `SemioBrepKernel` and OpenCascade (the
/// third-party oracle, `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio` suite) answer.
const AFFINE_TRANSFORMS_FIXTURE: &str = include_str!("../../../🧫️fixtures/🔁️affine-transforms/🔣️.json");

/// 📐️ What the laws read off a solid: exact mass properties, the tessellation's extent, planar faces' outward normals,
/// topology counts, and the tessellated soup's own signed volume and centroid as `parry3d` (third party) integrates them.
struct AffineMeasure {
    volume: f64,
    center_of_mass: EVec3,
    min: EVec3,
    max: EVec3,
    normals: Vec<EVec3>,
    soup_volume: f64,
    soup_centroid: EVec3,
    topology: [usize; 3],
}

/// 🧮️ A fixture step as `(linear, translation)`, derived here (Rodrigues rotation, Householder reflection) independently of
/// the kernel's own `Affine3`, so the exactness law never checks the kernel against itself.
type AffineMap = ([[f64; 3]; 3], EVec3);

fn affine_fixture() -> serde_json::Value {
    let root: serde_json::Value = serde_json::from_str(AFFINE_TRANSFORMS_FIXTURE).expect("affine-transforms fixture parses");
    assert_eq!(root["schema"].as_str(), Some("semio.geometry.brep.affine-transforms/v1"), "fixture schema");
    assert!(!root["cases"].as_array().expect("cases").is_empty(), "the fixture declares at least one case");
    assert!(!root["refusals"].as_array().expect("refusals").is_empty(), "the fixture declares at least one refusal");
    root
}

fn affine_number(value: &serde_json::Value, key: &str) -> f64 {
    value[key].as_f64().unwrap_or_else(|| panic!("fixture field {key} is a number"))
}

fn affine_vec3(value: &serde_json::Value) -> EVec3 {
    let items = value.as_array().expect("fixture vector is an array");
    assert_eq!(items.len(), 3, "fixture vector has three components");
    [items[0].as_f64().expect("x"), items[1].as_f64().expect("y"), items[2].as_f64().expect("z")]
}

fn affine_make(kernel: &mut Brep, solid: &serde_json::Value) -> GeometryHandle {
    match solid["kind"].as_str() {
        Some("box") => kernel.box_prim(affine_number(solid, "width"), affine_number(solid, "depth"), affine_number(solid, "height")),
        Some("sphere") => kernel.sphere_prim(affine_number(solid, "radius")),
        Some("cylinder") => kernel.cylinder_prim(affine_number(solid, "radius"), affine_number(solid, "height")),
        Some("cone") => kernel.cone_prim(affine_number(solid, "radius"), affine_number(solid, "height")),
        other => panic!("unknown fixture solid {other:?}"),
    }
    .expect("fixture primitive builds")
}

fn affine_apply(kernel: &mut Brep, shape: &GeometryHandle, step: &serde_json::Value) -> Result<GeometryHandle, BrepError> {
    match step["kind"].as_str() {
        Some("translate") => kernel.translate(shape, affine_vec3(&step["offset"])),
        Some("rotate") => kernel.rotate(shape, affine_vec3(&step["axis"]), affine_number(step, "angle")),
        Some("rotateAbout") => kernel.rotate_about(shape, affine_vec3(&step["origin"]), affine_vec3(&step["axis"]), affine_number(step, "angle")),
        Some("scale") => kernel.scale(shape, affine_number(step, "factor"), affine_vec3(&step["center"])),
        Some("mirror") => kernel.mirror(shape, affine_vec3(&step["origin"]), affine_vec3(&step["normal"])),
        other => panic!("unknown fixture step {other:?}"),
    }
}

fn affine_unit(v: EVec3) -> EVec3 {
    let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / length, v[1] / length, v[2] / length]
}

fn affine_linear_about(linear: [[f64; 3]; 3], fixed: EVec3) -> AffineMap {
    let moved = affine_times(&linear, fixed);
    (linear, [fixed[0] - moved[0], fixed[1] - moved[1], fixed[2] - moved[2]])
}

fn affine_rotation(axis: EVec3, angle: f64) -> [[f64; 3]; 3] {
    let [x, y, z] = affine_unit(axis);
    let (s, c) = angle.sin_cos();
    let t = 1.0 - c;
    [[t * x * x + c, t * x * y - s * z, t * x * z + s * y], [t * x * y + s * z, t * y * y + c, t * y * z - s * x], [t * x * z - s * y, t * y * z + s * x, t * z * z + c]]
}

fn affine_step_map(step: &serde_json::Value) -> AffineMap {
    let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    match step["kind"].as_str() {
        Some("translate") => (identity, affine_vec3(&step["offset"])),
        Some("rotate") => (affine_rotation(affine_vec3(&step["axis"]), affine_number(step, "angle")), [0.0; 3]),
        Some("rotateAbout") => affine_linear_about(affine_rotation(affine_vec3(&step["axis"]), affine_number(step, "angle")), affine_vec3(&step["origin"])),
        Some("scale") => {
            let f = affine_number(step, "factor");
            affine_linear_about([[f, 0.0, 0.0], [0.0, f, 0.0], [0.0, 0.0, f]], affine_vec3(&step["center"]))
        }
        Some("mirror") => {
            let n = affine_unit(affine_vec3(&step["normal"]));
            let row = |i: usize| [0, 1, 2].map(|j| if i == j { 1.0 } else { 0.0 } - 2.0 * n[i] * n[j]);
            affine_linear_about([row(0), row(1), row(2)], affine_vec3(&step["origin"]))
        }
        other => panic!("unknown fixture step {other:?}"),
    }
}

fn affine_times(m: &[[f64; 3]; 3], v: EVec3) -> EVec3 {
    [0, 1, 2].map(|i| m[i][0] * v[0] + m[i][1] * v[1] + m[i][2] * v[2])
}

fn affine_compose(outer: &AffineMap, inner: &AffineMap) -> AffineMap {
    let linear = [0, 1, 2].map(|i| [0, 1, 2].map(|j| (0..3).map(|k| outer.0[i][k] * inner.0[k][j]).sum::<f64>()));
    let moved = affine_times(&outer.0, inner.1);
    (linear, [moved[0] + outer.1[0], moved[1] + outer.1[1], moved[2] + outer.1[2]])
}

fn affine_point(map: &AffineMap, p: EVec3) -> EVec3 {
    let moved = affine_times(&map.0, p);
    [moved[0] + map.1[0], moved[1] + map.1[1], moved[2] + map.1[2]]
}

fn affine_determinant(m: &[[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]) + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

/// 🧭️ An outward normal maps by the inverse transpose — `cofactor / det`, so a reflection keeps it outward.
fn affine_normal(map: &AffineMap, n: EVec3) -> EVec3 {
    let m = &map.0;
    let cofactor = [
        [m[1][1] * m[2][2] - m[1][2] * m[2][1], m[1][2] * m[2][0] - m[1][0] * m[2][2], m[1][0] * m[2][1] - m[1][1] * m[2][0]],
        [m[0][2] * m[2][1] - m[0][1] * m[2][2], m[0][0] * m[2][2] - m[0][2] * m[2][0], m[0][1] * m[2][0] - m[0][0] * m[2][1]],
        [m[0][1] * m[1][2] - m[0][2] * m[1][1], m[0][2] * m[1][0] - m[0][0] * m[1][2], m[0][0] * m[1][1] - m[0][1] * m[1][0]],
    ];
    let det = affine_determinant(m);
    affine_unit(affine_times(&cofactor, n).map(|component| component / det))
}

fn affine_measure(kernel: &mut Brep, shape: &GeometryHandle, tessellation: f64) -> AffineMeasure {
    let mesh = kernel.tessellate(shape, tessellation).expect("the solid tessellates");
    let points: Vec<parry3d::na::Point3<f32>> = mesh.position.chunks_exact(3).map(|p| parry3d::na::Point3::new(p[0], p[1], p[2])).collect();
    let triangles: Vec<[u32; 3]> = mesh.index.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect();
    assert!(!points.is_empty() && !triangles.is_empty(), "tessellation produced an empty mesh");
    let (soup_volume, soup_centroid) = parry3d::mass_properties::details::trimesh_signed_volume_and_center_of_mass(&points, &triangles);
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for point in &points {
        for (axis, value) in [point.x, point.y, point.z].into_iter().enumerate() {
            min[axis] = min[axis].min(f64::from(value));
            max[axis] = max[axis].max(f64::from(value));
        }
    }
    let topology = kernel.deconstruct(shape).expect("the solid deconstructs");
    AffineMeasure {
        volume: kernel.volume(shape).expect("volume"),
        center_of_mass: kernel.center_of_mass(shape).expect("centre of mass"),
        min,
        max,
        normals: mesh.face_infos.iter().filter(|info| matches!(info.surface_kind, SurfaceKind::Plane)).map(|info| info.normal).collect(),
        soup_volume: f64::from(soup_volume),
        soup_centroid: [f64::from(soup_centroid.x), f64::from(soup_centroid.y), f64::from(soup_centroid.z)],
        topology: [topology.faces.len(), topology.edges.len(), topology.vertices.len()],
    }
}

/// 🧷️ Moves every normal of `expected` onto its match in `got` (within `tolerance`), reporting what is left on either side.
fn affine_normal_sets(id: &str, got: &[EVec3], expected: &[EVec3], tolerance: f64, failures: &mut Vec<String>) {
    let mut unmatched = got.to_vec();
    for want in expected {
        match unmatched.iter().position(|n| (0..3).all(|axis| (n[axis] - want[axis]).abs() <= tolerance)) {
            Some(index) => {
                unmatched.remove(index);
            }
            None => failures.push(format!("{id}: outward face normal {want:?} missing from {got:?}")),
        }
    }
    if !unmatched.is_empty() {
        failures.push(format!("{id}: unexpected face normals {unmatched:?}"));
    }
}

/// 🔁️ LAW (exactness): every step maps the solid EXACTLY by its affine map — the tessellated soup's signed volume scales by
/// `|det|` and stays POSITIVE (to f32 precision under an isometry; a non-unit scale re-samples curved faces at the same
/// absolute chord, so there the chordal tolerances apply) (`volume` reports a magnitude, so a reflection that left the solid inside out fails here on the
/// sign alone), its centroid lands where the map sends it, every planar face's outward normal lands where the inverse
/// transpose sends it, and face/edge/vertex counts are preserved. The map is derived in this law, not read from the kernel.
#[test]
fn every_affine_transform_maps_the_solid_exactly() {
    let root = affine_fixture();
    let tessellation = affine_number(&root, "tessellationTolerance");
    let normal_tolerance = affine_number(&root, "normalTolerance");
    let soup_volume_tolerance = affine_number(&root, "soupVolumeRelativeTolerance");
    let soup_centroid_tolerance = affine_number(&root, "soupCentroidTolerance");
    let mut failures = Vec::new();
    for case in root["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let mut kernel = Brep::new();
        let mut shape = affine_make(&mut kernel, &case["solid"]);
        let before = affine_measure(&mut kernel, &shape, tessellation);
        let mut map: AffineMap = ([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]], [0.0; 3]);
        for step in case["steps"].as_array().expect("steps") {
            shape = affine_apply(&mut kernel, &shape, step).unwrap_or_else(|error| panic!("{id}: {error}"));
            map = affine_compose(&affine_step_map(step), &map);
        }
        let after = affine_measure(&mut kernel, &shape, tessellation);
        let scale = affine_determinant(&map.0).abs();
        let isometry = (scale - 1.0).abs() <= 1e-12;
        let (volume_tolerance, centroid_tolerance) = if isometry { (1e-5, 1e-5) } else { (soup_volume_tolerance, soup_centroid_tolerance) };
        if !(after.soup_volume > 0.0 && (after.soup_volume - scale * before.soup_volume).abs() <= volume_tolerance * scale * before.soup_volume) {
            failures.push(format!("{id}: soup signed volume {} != |det| · {} = {}", after.soup_volume, before.soup_volume, scale * before.soup_volume));
        }
        let centroid = affine_point(&map, before.soup_centroid);
        let extent = (0..3).map(|axis| after.max[axis] - after.min[axis]).fold(1.0, f64::max);
        if (0..3).any(|axis| (after.soup_centroid[axis] - centroid[axis]).abs() > centroid_tolerance * extent) {
            failures.push(format!("{id}: soup centroid {:?} != mapped {centroid:?}", after.soup_centroid));
        }
        if before.topology != after.topology {
            failures.push(format!("{id}: faces/edges/vertices {:?} -> {:?}", before.topology, after.topology));
        }
        let mapped: Vec<EVec3> = before.normals.iter().map(|n| affine_normal(&map, *n)).collect();
        affine_normal_sets(id, &after.normals, &mapped, normal_tolerance, &mut failures);
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// 📐️ LAW (closed form): every vector answers its closed form — volume and centre of mass within the fixture's
/// mass-property tolerances, the tessellation's extent within `boundsTolerance`, the soup (parry3d) within its chordal
/// tolerances, every planar face's outward normal exactly — the same rows OpenCascade answers in the `🧠️semio` suite.
#[test]
fn every_affine_transform_vector_meets_its_closed_form() {
    let root = affine_fixture();
    let tessellation = affine_number(&root, "tessellationTolerance");
    let volume_tolerance = affine_number(&root, "volumeRelativeTolerance");
    let center_tolerance = affine_number(&root, "centerOfMassTolerance");
    let bounds_tolerance = affine_number(&root, "boundsTolerance");
    let normal_tolerance = affine_number(&root, "normalTolerance");
    let soup_volume_tolerance = affine_number(&root, "soupVolumeRelativeTolerance");
    let soup_centroid_tolerance = affine_number(&root, "soupCentroidTolerance");
    let mut failures = Vec::new();
    for case in root["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let expect = &case["expect"];
        let mut kernel = Brep::new();
        let mut shape = affine_make(&mut kernel, &case["solid"]);
        for step in case["steps"].as_array().expect("steps") {
            shape = affine_apply(&mut kernel, &shape, step).unwrap_or_else(|error| panic!("{id}: {error}"));
        }
        let after = affine_measure(&mut kernel, &shape, tessellation);
        let volume = affine_number(expect, "volume");
        let mut near = |label: String, got: f64, want: f64, tolerance: f64| {
            if !((got - want).abs() <= tolerance) {
                failures.push(format!("{id}: {label} {got} != {want} (±{tolerance})"));
            }
        };
        near("volume".into(), after.volume, volume, volume_tolerance * volume.abs());
        near("soup signed volume".into(), after.soup_volume, volume, soup_volume_tolerance * volume.abs());
        let center = affine_vec3(&expect["centerOfMass"]);
        let min = affine_vec3(&expect["bounds"]["min"]);
        let max = affine_vec3(&expect["bounds"]["max"]);
        for axis in 0..3 {
            near(format!("centerOfMass[{axis}]"), after.center_of_mass[axis], center[axis], center_tolerance);
            near(format!("soup centroid[{axis}]"), after.soup_centroid[axis], center[axis], soup_centroid_tolerance);
            near(format!("bounds.min[{axis}]"), after.min[axis], min[axis], bounds_tolerance);
            near(format!("bounds.max[{axis}]"), after.max[axis], max[axis], bounds_tolerance);
        }
        if let Some(normals) = expect["faceNormals"].as_array() {
            let expected: Vec<EVec3> = normals.iter().map(affine_vec3).collect();
            affine_normal_sets(id, &after.normals, &expected, normal_tolerance, &mut failures);
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// 🧭️ LAW (refusal): a degenerate step — a zero axis or normal, a zero factor — is refused as invalid input and never
/// answered with a substituted direction; OpenCascade refuses every one of them too (`🧠️semio` suite).
#[test]
fn every_degenerate_affine_transform_is_refused() {
    let root = affine_fixture();
    let mut failures = Vec::new();
    for refusal in root["refusals"].as_array().expect("refusals") {
        let id = refusal["id"].as_str().expect("refusal id");
        let mut kernel = Brep::new();
        let shape = affine_make(&mut kernel, &refusal["solid"]);
        match affine_apply(&mut kernel, &shape, &refusal["step"]) {
            Err(BrepError::InvalidInput(_)) => {}
            other => failures.push(format!("{id}: expected an invalid-input refusal, got {other:?}")),
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}
// #endregion 🔁️AffineTransforms

/// 🛂️ Analytic reference tables cannot claim geometry without matching picking buffers.
#[test]
fn brep_preview_component_reference_tables_refuse_missing_picking_buffers() {
    use protocol::value::FromValue;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎯️component-picking/🔣️.json")).unwrap();
    for row in fixture["referenceRefusals"].as_array().unwrap() {
        let input=serde_json::json!({"componentReferences":row["references"]});
        let input=semio_framework_pack_json::parse(&input.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert!(semio_framework_mesh_engine::MeshData::from_value(semio_framework_pack_json::to_dsl_value(&input)).is_err(),"{}",row["id"]);
    }
    println!("[DEBUG] Analytic reference-table refusals=6 owned first-party value boundary");
}

/// 🚧️ Malformed analytic ranges are refused before allocating picking buffers.
#[test]
fn brep_preview_component_ranges_refuse_incomplete_or_overlapping_groups() {
    use protocol::value::FromValue;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎯️component-picking/🔣️.json")).unwrap();
    for row in fixture["refusals"].as_array().unwrap() {
        let mut source=fixture["transfer"].clone();
        source[format!("{}_groups",row["domain"].as_str().unwrap())][row["row"].as_u64().unwrap() as usize][row["field"].as_str().unwrap()]=row["value"].clone();
        let source=semio_framework_pack_json::parse(&source.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        if let Ok(transfer)=MeshTransfer::from_value(semio_framework_pack_json::to_dsl_value(&source)) {assert!(mesh_data_from_mesh_transfer(&transfer).is_err());}
    }
    println!("[DEBUG] Analytic group range refusals=6 before pick buffer publication");
}

/// 🎯️ Only original topology vertices are selectable in analytic preview meshes.
#[test]
fn brep_preview_vertices_resolve_original_topology_without_sampling_guesses() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️vertex-provenance/🔣️.json")).unwrap();
    let absent = fixture["unselectable"].as_u64().unwrap() as u32;
    for row in fixture["cases"].as_array().unwrap() {
        let mut kernel = Brep::new();
        let dimensions = row["dimensions"].as_array().unwrap();
        let shape = match row["kind"].as_str().unwrap() {
            "box" => kernel.box_prim_sync(dimensions[0].as_f64().unwrap(), dimensions[1].as_f64().unwrap(), dimensions[2].as_f64().unwrap()).unwrap(),
            "wire" => kernel.rectangle_wire_sync(dimensions[0].as_f64().unwrap(), dimensions[1].as_f64().unwrap()).unwrap(),
            "point" => kernel.vertex_sync(std::array::from_fn(|index| row["points"][0][index].as_f64().unwrap())).unwrap(),
            _ => unreachable!(),
        };
        let topology = kernel.deconstruct_sync(&shape).unwrap();
        let transfer = kernel.tessellate_sync(&shape, 0.1).unwrap();
        let mesh = mesh_data_from_mesh_transfer(&transfer).unwrap();
        if row["kind"] == "point" {
            assert!(topology.edges.is_empty() && topology.faces.is_empty() && topology.shells.is_empty());
            assert!(transfer.position.is_empty() && transfer.index.is_empty() && transfer.edges.is_empty());
            assert_eq!(transfer.points.len(), 3);
            let mut job = kernel.tessellate_job_sync(&shape, 0.1).unwrap();
            assert_eq!(job.progress().units_total, 1);
            assert!(matches!(job.step(kernel.tessellation_body(), 0).unwrap(), crate::brep::queries::tessellation::TessellationStep::Working(_)));
            assert_eq!(job.progress().units_done, 0);
            assert!(matches!(job.step(kernel.tessellation_body(), 1).unwrap(), crate::brep::queries::tessellation::TessellationStep::Done(_)));
            assert_eq!(job.progress().units_done, 1);
            let (budgeted, _) = job.into_mesh().unwrap();
            assert_eq!(budgeted.points, transfer.points);
            assert_eq!(budgeted.vertex_groups[0].entity_id, transfer.vertex_groups[0].entity_id);
            let mut cancelled = kernel.tessellate_job_sync(&shape, 0.1).unwrap();
            cancelled.cancel();
            assert!(matches!(cancelled.step(kernel.tessellation_body(), 1).unwrap(), crate::brep::queries::tessellation::TessellationStep::Cancelled(_)));
            assert!(cancelled.into_mesh().is_none());
        }
        let labels = mesh.component_references.get("vertex").expect("original topology vertex references");
        assert_eq!(labels.len(), topology.vertices.len());
        assert_eq!(labels.len(), row["points"].as_array().unwrap().len());
        assert_eq!(mesh.vertex_ids.len(), mesh.positions.len() / 3);
        for index in &mesh.indices { assert_eq!(mesh.vertex_ids[*index as usize], absent, "surface samples are not topology vertex targets"); }
        let expected: Vec<parry3d::math::Point<f32>> = row["points"].as_array().unwrap().iter().map(|point| parry3d::math::Point::new(point[0].as_f64().unwrap() as f32, point[1].as_f64().unwrap() as f32, point[2].as_f64().unwrap() as f32)).collect();
        let mut seen = std::collections::BTreeSet::new();
        for (index, group) in mesh.vertex_ids.iter().enumerate().filter(|(_, group)| **group != absent) {
            let label = PersistentLabel(labels[*group as usize].parse::<u64>().unwrap());
            assert!(seen.insert(label));
            let handle = kernel.handle_for_label(label).unwrap();
            assert_eq!(kernel.kind_sync(&handle).unwrap(), GeometryKind::Vertex);
            assert!(topology.vertices.contains(&handle));
            let point = parry3d::math::Point::new(mesh.positions[index * 3], mesh.positions[index * 3 + 1], mesh.positions[index * 3 + 2]);
            assert!(expected.iter().any(|candidate| (point - candidate).norm() < 1e-6));
            let current = kernel.closest_point_sync(&handle, [0.0; 3]).unwrap().point;
            assert!((point - parry3d::math::Point::new(current[0] as f32, current[1] as f32, current[2] as f32)).norm() < 1e-6);
        }
        assert_eq!(seen.len(), labels.len());
        println!("[DEBUG] originalVertexPreview kind={} topologyVertices={} independentParry=true surfaceSamplesExcluded=true", row["kind"], labels.len());
    }
}

/// 🔗️ Merged preview topology resolves the same neutral exact component references.
#[test]
fn brep_merged_preview_preserves_original_component_ranges() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔗️mesh-transfer-merge/🔣️.json")).unwrap();
    let wire = semio_framework_pack_json::parse(&fixture["merged"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let transfer = <MeshTransfer as protocol::value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&wire)).unwrap();
    let mesh = mesh_data_from_mesh_transfer(&transfer).unwrap();
    assert_eq!(serde_json::json!(mesh.face_ids),fixture["expected"]["faceIds"]);
    assert_eq!(serde_json::json!(mesh.edge_ids),fixture["expected"]["edgeIds"]);
    assert_eq!(serde_json::json!(mesh.vertex_ids),fixture["expected"]["vertexIds"]);
    let encoded: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::Value::from(mesh.clone()).to_string()).unwrap();
    assert_eq!(encoded["componentReferences"],fixture["expected"]["componentReferences"]);
    let area: f32 = mesh.indices.chunks_exact(3).map(|indices| {
        let points = [indices[0],indices[1],indices[2]].map(|index| {let at=index as usize*3;parry3d::math::Point::new(mesh.positions[at],mesh.positions[at+1],mesh.positions[at+2])});
        parry3d::shape::Triangle::new(points[0],points[1],points[2]).area()
    }).sum();
    assert_eq!(area as f64,fixture["expected"]["area"].as_f64().unwrap());
    println!("[DEBUG] originalMergedPreview domains=3 independentParryArea={area}");
}

/// 🚫️ Original vertex ranges obey the same neutral refusal contract before mesh publication.
#[test]
fn brep_original_vertex_ranges_refuse_malformed_coverage_and_labels() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎯️vertex-provenance/🔣️.json")).unwrap();
    let points = fixture["cases"][0]["points"].as_array().unwrap();
    let raw = serde_json::json!({"position":[0,0,0,1,0,0,0,1,0],"normal":[0,0,1,0,0,1,0,0,1],"index":[0,1,2],"edges":[],"points":points.iter().flat_map(|point|point.as_array().unwrap().iter().cloned()).collect::<Vec<_>>(),"face_groups":[],"edge_groups":[],"vertex_groups":points.iter().enumerate().map(|(index,_)|serde_json::json!({"start":index,"count":1,"entity_id":(index+1).to_string()})).collect::<Vec<_>>()});
    for refusal in fixture["refusals"].as_array().unwrap() {
        let mut source = raw.clone();
        source["vertex_groups"][refusal["row"].as_u64().unwrap() as usize][refusal["field"].as_str().unwrap()] = refusal["value"].clone();
        let source = semio_framework_pack_json::parse(&source.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        if let Ok(transfer) = <MeshTransfer as protocol::value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&source)) { assert!(mesh_data_from_mesh_transfer(&transfer).is_err(),"refusal {refusal}"); }
    }
    println!("[DEBUG] originalVertexRangeRefusals cases=6 independentSerdeFixture=true");
}
