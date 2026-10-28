use super::*;
use crate::standards::v4::subsets::any::schema::snapshot::from_part21_document;

fn fixture(case: &semio_framework_pack_json::Value) -> IfcSnapshot {
    let text = format!("ISO-10303-21;HEADER;FILE_DESCRIPTION((''),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4'));ENDSEC;DATA;{}ENDSEC;END-ISO-10303-21;", case["data"].as_str().unwrap());
    from_part21_document("stdio.ifc", &semio_s_artifact_stdio_contract::part21::parse_part21(&text).unwrap())
}

#[semio_framework_async_macros::async_test]
async fn language_neutral_geometry_cases() {
    let cases: semio_framework_pack_json::Value = semio_framework_pack_json::parse(include_str!("../🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for case in cases["cases"].as_array().unwrap() {
        let scene = project(&fixture(case));
        assert_eq!(scene.status, GeometryStatus::Complete);
        let expected = &case["expected"];
        assert_eq!(scene.meshes.iter().map(|mesh| mesh.vertices.len()).sum::<usize>() as u64, expected["vertices"].as_u64().unwrap(), "{:?}", case["name"]);
        assert_eq!(scene.meshes.iter().map(|mesh| mesh.triangles.len()).sum::<usize>() as u64, expected["triangles"].as_u64().unwrap(), "{}", case["name"]);
        if let Some(bounds) = expected.get("bounds") {
            let actual = scene.bounds().unwrap();
            for side in 0..2 { for axis in 0..3 { assert!((actual[side][axis] - bounds[side][axis].as_f64().unwrap()).abs() < 1e-9); } }
        }
        if expected.get("issue").is_some() { assert!(!scene.issues.is_empty()); }
        let products: Vec<u64> = scene.meshes.iter().map(|mesh| mesh.product).collect();
        let expected_products: Vec<u64> = expected["products"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap()).collect();
        assert_eq!(products, expected_products);
        if let Some(area) = expected.get("area") {
            let mut actual = 0.;
            for mesh in &scene.meshes { for triangle in &mesh.triangles {
                let a = mesh.vertices[triangle[0] as usize]; let b = mesh.vertices[triangle[1] as usize]; let c = mesh.vertices[triangle[2] as usize];
                let normal = cross(subtract(b, a), subtract(c, a)); actual += dot(normal, normal).sqrt() / 2.;
            } }
            assert!((actual - area.as_f64().unwrap()).abs() < 1e-9);
        }
        if let Some(volume) = expected.get("volume") {
            let mut actual = 0.0;
            for mesh in &scene.meshes { for triangle in &mesh.triangles {
                actual += dot(mesh.vertices[triangle[0] as usize], cross(mesh.vertices[triangle[1] as usize], mesh.vertices[triangle[2] as usize])) / 6.0;
            } }
            assert!((actual.abs() - volume.as_f64().unwrap()).abs() < 1e-9);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn cancellation_and_limits_are_explicit() {
    let snapshot = IfcSnapshot::default();
    assert_eq!(project_with(&snapshot, GeometryLimits::default(), &mut |_| false).status, GeometryStatus::Cancelled);
    let cases: semio_framework_pack_json::Value = semio_framework_pack_json::parse(include_str!("../🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let snapshot = fixture(&cases["cases"][0]);
    let scene = project_with(&snapshot, GeometryLimits { work: 2, ..GeometryLimits::default() }, &mut |_| true);
    assert_eq!(scene.status, GeometryStatus::Limited);
    assert!(scene.meshes.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn cancellation_during_product_never_publishes_partial_meshes() {
    let cases = semio_framework_pack_json::parse(include_str!("../🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let snapshot = fixture(&cases["cases"][0]);
    let full = project(&snapshot);
    for stop in 1..full.work {
        let scene = project_with(&snapshot, GeometryLimits::default(), &mut |progress| progress.work < stop);
        assert_eq!(scene.status, GeometryStatus::Cancelled);
        assert!(scene.meshes.is_empty());
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_payloads_use_real_topology_and_stable_product_identity() {
    let cases = semio_framework_pack_json::parse(include_str!("../🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let scene = project(&fixture(&cases["cases"][0]));
    let meshes = semio_framework_pack_json::parse(&scene.meshes_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let instances = semio_framework_pack_json::parse(&scene.instances_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(meshes[0]["id"].as_str(), Some("ifc-16-14"));
    assert_eq!(meshes[0]["data"]["positions"].as_array().unwrap().len(), 24);
    assert_eq!(meshes[0]["data"]["indices"].as_array().unwrap().len(), 36);
    assert_eq!(instances[0]["interactionId"].as_str(), Some("ifc-16"));
    let empty = project(&IfcSnapshot::default());
    assert_eq!(empty.meshes_json(), "[]");
    assert_eq!(empty.instances_json(), "[]");
}

#[semio_framework_async_macros::async_test]
async fn caller_deadline_and_work_yield_are_limited_not_cancelled() {
    let cases = semio_framework_pack_json::parse(include_str!("../🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let snapshot = fixture(&cases["cases"][0]);
    let full = project(&snapshot);
    for stop in 1..full.work {
        let scene = project_controlled(&snapshot, GeometryLimits::default(), &mut |progress| if progress.work < stop { GeometryControl::Continue } else { GeometryControl::Limit });
        assert_eq!(scene.status, GeometryStatus::Limited);
        assert!(scene.meshes.is_empty());
    }
}

#[semio_framework_async_macros::async_test]
async fn product_representation_failures_are_diagnosed_without_metadata_noise() {
    let cases = semio_framework_pack_json::parse(include_str!("../🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let contract = semio_framework_pack_json::parse(include_str!("../📜️references.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for row in contract["cases"].as_array().unwrap() {
        let placed = row.get("base").and_then(semio_framework_pack_json::Value::as_str) == Some("placed");
        let mut snapshot = fixture(&cases["cases"][if placed { 0 } else { 1 }]);
        let id = if placed { 16 } else { 5 };
        let product = snapshot.entities.iter_mut().find(|entity| entity.id == id).unwrap();
        product.name = row["type"].as_str().unwrap().into();
        product.args[6] = IfcValue::Reference(row["target"].as_u64().unwrap());
        let scene = project(&snapshot);
        assert_eq!(scene.status, GeometryStatus::Complete);
        assert!(scene.meshes.is_empty());
        let expected: Vec<GeometryIssue> = row.get("expected").and_then(semio_framework_pack_json::Value::as_str).map(|reason| vec![GeometryIssue { entity: id, reason: reason.into() }]).unwrap_or_default();
        assert_eq!(scene.issues, expected);
    }
}

#[semio_framework_async_macros::async_test]
async fn parameterized_curves_match_neutral_contract() {
    let contract = semio_framework_pack_json::parse(include_str!("../📜️profiles.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for case in contract["cases"].as_array().unwrap() {
        let scene = project(&fixture(case));
        assert_eq!(scene.status, GeometryStatus::Complete);
        assert!(scene.issues.is_empty(), "{:?}", scene.issues);
        let mesh = &scene.meshes[0];
        assert_eq!(mesh.vertices.len() as u64, case["expected"]["vertices"].as_u64().unwrap());
        assert_eq!(mesh.triangles.len() as u64, case["expected"]["triangles"].as_u64().unwrap());
        let bounds = scene.bounds().unwrap();
        for side in 0..2 { for axis in 0..3 { assert!((bounds[side][axis] - case["expected"]["bounds"][side][axis].as_f64().unwrap()).abs() < 1e-9); } }
        let volume: f64 = mesh.triangles.iter().map(|face| dot(mesh.vertices[face[0] as usize], cross(mesh.vertices[face[1] as usize], mesh.vertices[face[2] as usize])) / 6.).sum();
        assert!(volume > 0.);
        assert!((volume - case["expected"]["volume"].as_f64().unwrap()).abs() < 1e-9);
        let a = case["oracle"]["a"].as_f64().unwrap();
        let b = case["oracle"]["b"].as_f64().unwrap();
        let rotation = case["oracle"]["rotation"].as_f64().unwrap();
        for i in 0..CURVE_SEGMENTS {
            let theta = std::f64::consts::TAU * (i as f64 + 0.5) / CURVE_SEGMENTS as f64;
            let (x, y) = (a * theta.cos(), b * theta.sin());
            let exact = [case["oracle"]["origin"][0].as_f64().unwrap() + x * rotation.cos() - y * rotation.sin(), case["oracle"]["origin"][1].as_f64().unwrap() + x * rotation.sin() + y * rotation.cos()];
            let midpoint = [0, 1].map(|axis| (mesh.vertices[i][axis] + mesh.vertices[(i + 1) % CURVE_SEGMENTS][axis]) / 2.);
            assert!((exact[0] - midpoint[0]).hypot(exact[1] - midpoint[1]) <= a.max(b) * CURVE_RELATIVE_TOLERANCE * (1. + 1e-12));
        }
    }
    let baseline = contract["cases"][0]["data"].as_str().unwrap();
    for case in contract["invalid"].as_array().unwrap() {
        let data = baseline.replace("IFCCIRCLEPROFILEDEF(.AREA.,$,$,2.)", case["definition"].as_str().unwrap());
        let source = semio_framework_pack_json::json!({"data": data});
        let scene = project(&fixture(&source));
        assert!(scene.meshes.is_empty());
        assert_eq!(scene.issues[0].reason, case["reason"].as_str().unwrap());
    }
}

#[semio_framework_async_macros::async_test]
async fn parameterized_curve_work_is_cancellable_and_bounded() {
    let contract = semio_framework_pack_json::parse(include_str!("../📜️profiles.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let snapshot = fixture(&contract["cases"][0]);
    let full = project(&snapshot);
    for stop in [50, 90, 130, full.work / 2, full.work - 1] {
        let cancelled = project_controlled(&snapshot, GeometryLimits::default(), &mut |value| if value.work < stop { GeometryControl::Continue } else { GeometryControl::Cancel });
        assert_eq!(cancelled.status, GeometryStatus::Cancelled);
        assert!(cancelled.meshes.is_empty());
        let limited = project_controlled(&snapshot, GeometryLimits { work: stop, ..GeometryLimits::default() }, &mut |_| GeometryControl::Continue);
        assert_eq!(limited.status, GeometryStatus::Limited);
        assert!(limited.meshes.is_empty());
        assert!(limited.work <= stop);
    }
}

#[semio_framework_async_macros::async_test]
async fn unchanged_official_polygonal_hole_sample_preserves_closed_solid() {
    let text = include_str!("../../../../🧪️tests/🧫️fixtures/🏛️official/polygonal-face-tessellation.ifc");
    let source = semio_s_artifact_stdio_contract::part21::parse_part21(text).unwrap();
    let snapshot = from_part21_document("stdio.ifc", &source);
    let scene = project(&snapshot);
    assert!(scene.issues.is_empty(), "{:?}", scene.issues);
    assert_eq!(scene.meshes.len(), 1);
    let mesh = &scene.meshes[0];
    assert_eq!(mesh.product, 30);
    assert_eq!(mesh.vertices.len(), 16);
    assert_eq!(mesh.triangles.len(), 28);
    let mut volume = 0.;
    let mut area = 0.;
    let mut edges = std::collections::BTreeMap::new();
    for triangle in &mesh.triangles {
        let [a, b, c] = triangle.map(|id| mesh.vertices[id as usize]);
        volume += dot(a, cross(b, c)) / 6.;
        let normal = cross(subtract(b, a), subtract(c, a));
        area += dot(normal, normal).sqrt() / 2.;
        for i in 0..3 {
            let (a, b) = (triangle[i], triangle[(i + 1) % 3]);
            let key = if a < b { (a, b) } else { (b, a) };
            *edges.entry(key).or_insert(0usize) += 1;
        }
    }
    assert!((volume - 6.5e12).abs() < 1.);
    assert!((area - 3e9).abs() < 0.001);
    assert!(edges.values().all(|&count| count == 2));
}

#[semio_framework_async_macros::async_test]
async fn complex_constituents_never_publish_guessed_geometry(){for(source,id,reason)in [(include_str!("../../../../../../../🔖️2x3/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/📐️geometry/complex-product.ifc"),17,"unsupported-complex-product"),(include_str!("../../../../../../../🔖️2x3/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/📐️geometry/complex-item.ifc"),10,"unsupported-complex-representation-#10")]{let source=source.replace("'IFC2X3'","'IFC4'");let snapshot=from_part21_document("stdio.ifc",&semio_s_artifact_stdio_contract::part21::parse_part21(&source).unwrap());let graph=reader::Ifc4GeometryGraph(&snapshot);use shared::graph::GeometryGraph;let index=snapshot.entities.iter().position(|e|e.id==id).unwrap();assert_eq!(graph.entity(index).unwrap().constituent_count,2);assert!(graph.constituent(index,0).is_some());assert!(graph.constituent(index,1).is_some());assert!(graph.constituent(index,2).is_none());let scene=project(&snapshot);assert!(scene.meshes.is_empty());assert!(scene.issues.iter().any(|i|i.reason==reason));}}

#[semio_framework_async_macros::async_test]
async fn finite_tiny_and_huge_geometry_normals_are_scale_invariant() {
    for number in ["1.E-120", "1.E120"] {
        let data = format!("#1=IFCCARTESIANPOINTLIST3D(((0.,0.,0.),(0.,{number},0.),(0.,0.,{number})));#2=IFCTRIANGULATEDFACESET(#1,$,.F.,((1,2,3)),$);#3=IFCSHAPEREPRESENTATION($,'Body','Tessellation',(#2));#4=IFCPRODUCTDEFINITIONSHAPE($,$,(#3));#5=IFCBUILDINGELEMENTPROXY('0123456789012345678901',$,'Scale',$,$,$,#4,$,$);");
        let case = semio_framework_pack_json::json!({"data":data});
        let scene = project(&fixture(&case));
        assert_eq!(scene.meshes.len(), 1);
        let payload = semio_framework_pack_json::parse(&scene.meshes_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        for normal in payload[0]["data"]["normals"].as_array().unwrap().chunks(3) {
            assert!((normal[0].as_f64().unwrap() - 1.).abs() < 1e-9);
            assert_eq!(normal[1].as_f64(), Some(0.));
            assert_eq!(normal[2].as_f64(), Some(0.));
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn source_openings_publish_only_complete_cut_hosts(){for(text,expected)in [(include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/placed-opening.ifc"),48.),(include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/multiple-openings.ifc"),47.75),(include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/translated-opening.ifc"),48.)]{let snapshot=from_part21_document("stdio.ifc",&semio_s_artifact_stdio_contract::part21::parse_part21(text).unwrap());let scene=project(&snapshot);assert_eq!(scene.status,GeometryStatus::Complete);assert_eq!(scene.meshes.len(),1,"{:?}",scene.issues);assert_eq!(scene.meshes[0].product,17);assert_eq!(scene.meshes[0].representation,11);let volume:f64=scene.meshes[0].triangles.iter().map(|ids|{let[a,b,c]=ids.map(|i|scene.meshes[0].vertices[i as usize]);dot(a,cross(b,c))/6.}).sum();assert!((volume-expected).abs()<1e-8);assert!(scene.issues.iter().all(|i|i.reason=="opening-is-a-void-volume"));let cancel=project_controlled(&snapshot,GeometryLimits::default(),&mut|p|if p.work>=scene.work-10{GeometryControl::Cancel}else{GeometryControl::Continue});assert_eq!(cancel.status,GeometryStatus::Cancelled);assert!(cancel.meshes.is_empty());}}
#[semio_framework_async_macros::async_test]
async fn unchanged_official_slab_openings_match_independent_sampled_solid_volume(){let snapshot=from_part21_document("stdio.ifc",&semio_s_artifact_stdio_contract::part21::parse_part21(include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/official-slab-openings.ifc")).unwrap());let scene=project(&snapshot);assert_eq!(scene.status,GeometryStatus::Complete);assert_eq!(scene.meshes.len(),1,"{:?}",scene.issues);let mesh=&scene.meshes[0];assert_eq!(mesh.product,303);assert_eq!(mesh.representation,314);let volume:f64=mesh.triangles.iter().map(|ids|{let[a,b,c]=ids.map(|i|mesh.vertices[i as usize]);dot(a,cross(b,c))/6.}).sum();assert!((volume-1199056451.5686657).abs()<0.001);assert_eq!(scene.issues.iter().map(|i|i.entity).collect::<Vec<_>>(),vec![325,336]);}

#[semio_framework_async_macros::async_test]
async fn public_payload_camera_and_invalid_meshes_follow_neutral_contract() {
    fn scene(min: [f64; 3], max: [f64; 3]) -> GeometryScene {
        GeometryScene { meshes: vec![GeometryMesh { product: 1, representation: 2, label: "Camera".into(), vertices: vec![min, [max[0], min[1], min[2]], [min[0], max[1], min[2]], [min[0], min[1], max[2]], max], triangles: vec![[0, 1, 2], [0, 3, 1]] }], ..Default::default() }
    }
    let contract = semio_framework_pack_json::parse(include_str!("../📷️camera.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let point = |value: &semio_framework_pack_json::Value| std::array::from_fn(|axis| value[axis].as_f64().unwrap());
    for case in contract["cases"].as_array().unwrap() {
        let source = scene(point(&case["min"]), point(&case["max"]));
        let (position, target) = source.camera();
        assert!(position.iter().chain(target.iter()).all(|value| value.is_finite()));
        if case["status"].as_str() == Some("limited") {
            assert_eq!(source.payload_issue().unwrap().reason, case["reason"].as_str().unwrap());
            assert_eq!(source.meshes_json(), "[]");
            assert_eq!(source.instances_json(), "[]");
        } else {
            assert!(source.payload_issue().is_none());
            for (actual, expected) in position.into_iter().chain(target).zip(point(&case["position"]).into_iter().chain(point(&case["target"]))) {
                assert!((actual - expected).abs() <= f64::MIN_POSITIVE.max(expected.abs() * 2e-15));
            }
        }
    }
    for triangle in [[0, 1, 99], [0, 1, 1]] {
        let mut source = scene([0.; 3], [1.; 3]);
        source.meshes[0].triangles = vec![triangle];
        assert_eq!(source.payload_issue().unwrap().reason, "invalid-mesh-index");
        assert_eq!(source.meshes_json(), "[]");
        assert_eq!(source.instances_json(), "[]");
    }
    for number in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut source = scene([0.; 3], [1.; 3]);
        source.meshes[0].vertices[0][0] = number;
        assert_eq!(source.payload_issue().unwrap().reason, "nonfinite-mesh-coordinate");
        assert_eq!(source.meshes_json(), "[]");
    }
    let mut source = scene([0.; 3], [1.; 3]);
    source.meshes[0].vertices[2] = [0.5, 0., 0.];
    assert_eq!(source.payload_issue().unwrap().reason, "unrepresentable-mesh-normal");
    assert_eq!(source.meshes_json(), "[]");
}

#[semio_framework_async_macros::async_test]
async fn source_regularized_boolean_results_preserve_identity_and_control(){let source=include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/boolean-result.ifc");for(operator,expected)in [("DIFFERENCE",48.),("UNION",72.),("INTERSECTION",16.)]{let text=source.replace(".DIFFERENCE.",&format!(".{operator}."));let snapshot=from_part21_document("stdio.ifc",&semio_s_artifact_stdio_contract::part21::parse_part21(&text).unwrap());let scene=project(&snapshot);assert_eq!(scene.status,GeometryStatus::Complete);assert_eq!(scene.meshes.len(),1,"{:?}",scene.issues);assert!(scene.issues.is_empty());let mesh=&scene.meshes[0];assert_eq!(mesh.product,17);assert_eq!(mesh.representation,11);let volume:f64=mesh.triangles.iter().map(|ids|{let[a,b,c]=ids.map(|i|mesh.vertices[i as usize]);dot(a,cross(b,c))/6.}).sum();assert!((volume-expected).abs()<1e-8);let cancelled=project_controlled(&snapshot,GeometryLimits::default(),&mut|p|if p.work>=scene.work-10{GeometryControl::Cancel}else{GeometryControl::Continue});assert_eq!(cancelled.status,GeometryStatus::Cancelled);assert!(cancelled.meshes.is_empty());}for text in [source.replace(".DIFFERENCE.",".XOR."),source.replace(".DIFFERENCE.,#10,#22",".DIFFERENCE.,#60,#22"),source.replace(".DIFFERENCE.,#10,#22",".DIFFERENCE.,#11,#22")]{let snapshot=from_part21_document("stdio.ifc",&semio_s_artifact_stdio_contract::part21::parse_part21(&text).unwrap());let scene=project(&snapshot);assert!(scene.meshes.is_empty());assert!(scene.issues.iter().any(|i|i.entity==17));}}

#[semio_framework_async_macros::async_test]
async fn csg_prerequisites_and_unchanged_official_bath_project_real_closed_solids(){for(text,expected,product,representation)in [(include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/csg-wrapper.ifc"),48.,17,11),(include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/block.ifc"),24.,17,11),(include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/rounded-rectangle.ifc"),(12.+36.*(std::f64::consts::PI/36.).sin())*4.,17,11),(include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/official-bath.ifc"),548147011.3103606,225,223)]{let snapshot=from_part21_document("stdio.ifc",&semio_s_artifact_stdio_contract::part21::parse_part21(text).unwrap());let scene=project(&snapshot);assert_eq!(scene.status,GeometryStatus::Complete);assert!(scene.issues.is_empty(),"{:?}",scene.issues);assert_eq!(scene.meshes.len(),1);let mesh=&scene.meshes[0];assert_eq!(mesh.product,product);assert_eq!(mesh.representation,representation);let volume:f64=mesh.triangles.iter().map(|ids|{let[a,b,c]=ids.map(|i|mesh.vertices[i as usize]);dot(a,cross(b,c))/6.}).sum();assert!((volume-expected).abs()<0.001);let mut edges=std::collections::BTreeMap::<(u32,u32),(usize,i32)>::new();for ids in &mesh.triangles{for i in 0..3{let(a,b)=(ids[i],ids[(i+1)%3]);let key=if a<b{(a,b)}else{(b,a)};let edge=edges.entry(key).or_default();edge.0+=1;edge.1+=if a<b{1}else{-1};}}assert!(edges.values().all(|v|*v==(2,0)));let cancelled=project_controlled(&snapshot,GeometryLimits::default(),&mut|p|if p.work>=scene.work-10{GeometryControl::Cancel}else{GeometryControl::Continue});assert_eq!(cancelled.status,GeometryStatus::Cancelled);assert!(cancelled.meshes.is_empty());}}
#[semio_framework_async_macros::async_test]
async fn rounded_rectangle_radius_bounds_and_profile_placement_are_explicit(){let source=include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/rounded-rectangle.ifc");for(dimensions,valid)in [("4.,4.,2.",true),("4.,2.,1.",true),("4.,4.,0.",false),("4.,4.,2.1",false),("4.,2.,1.1",false)]{let text=source.replace("4.,4.,1.",dimensions);let snapshot=from_part21_document("stdio.ifc",&semio_s_artifact_stdio_contract::part21::parse_part21(&text).unwrap());let scene=project(&snapshot);assert_eq!(scene.meshes.len(),usize::from(valid));}let text=source.replace("#9=IFCAXIS2PLACEMENT2D(#8,$);","#9=IFCAXIS2PLACEMENT3D(#1,#2,#3);");let snapshot=from_part21_document("stdio.ifc",&semio_s_artifact_stdio_contract::part21::parse_part21(&text).unwrap());let scene=project(&snapshot);assert!(scene.meshes.is_empty());assert!(scene.issues.iter().any(|i|i.reason=="profile-placement-must-be-2d"));}

#[semio_framework_async_macros::async_test]
async fn native_plane_halfspace_clipping_preserves_material_side_holes_and_affine_normals(){for(text,expected,representation)in [(include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/plane-clipping.ifc"),32.,11),(include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/holed-plane-clipping.ifc"),24.,11),(include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/nested-plane-clipping.ifc"),16.,11),(include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/mapped-plane-clipping.ifc"),192.,90),(include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/reflected-plane-clipping.ifc"),32.,90)]{let snapshot=from_part21_document("stdio.ifc",&semio_s_artifact_stdio_contract::part21::parse_part21(text).unwrap());let scene=project(&snapshot);assert_eq!(scene.status,GeometryStatus::Complete);assert!(scene.issues.is_empty(),"{:?}",scene.issues);assert_eq!(scene.meshes.len(),1);let mesh=&scene.meshes[0];assert_eq!(mesh.product,17);assert_eq!(mesh.representation,representation);let volume:f64=mesh.triangles.iter().map(|ids|{let[a,b,c]=ids.map(|i|mesh.vertices[i as usize]);dot(a,cross(b,c))/6.}).sum();assert!((volume-expected).abs()<1e-8);let cancelled=project_controlled(&snapshot,GeometryLimits::default(),&mut|p|if p.work>=scene.work/2{GeometryControl::Cancel}else{GeometryControl::Continue});assert_eq!(cancelled.status,GeometryStatus::Cancelled);assert!(cancelled.meshes.is_empty());}let source=include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/plane-clipping.ifc");let text=source.replace("#61=IFCHALFSPACESOLID(#62,.F.)","#61=IFCHALFSPACESOLID(#62,.T.)");let snapshot=from_part21_document("stdio.ifc",&semio_s_artifact_stdio_contract::part21::parse_part21(&text).unwrap());let scene=project(&snapshot);assert_eq!(scene.meshes.len(),1);assert!(scene.meshes[0].vertices.iter().all(|point|point[2]>=2.-1e-9));for text in [source.replace(".DIFFERENCE.",".UNION."),source.replace("IFCHALFSPACESOLID(#62,.F.)","IFCBOXEDHALFSPACE(#62,.F.,#4)"),source.replace(".DIFFERENCE.,#10,#61",".DIFFERENCE.,#60,#61")]{let snapshot=from_part21_document("stdio.ifc",&semio_s_artifact_stdio_contract::part21::parse_part21(&text).unwrap());assert!(project(&snapshot).meshes.is_empty());}}

#[test]
fn retained_placed_primitives_resume_from_original_caller_grants(){for(source,expected)in [(include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/block.ifc").to_owned(),24.),(include_str!("../../../../../../../../📐️geometry/🧊️boolean/🧪️tests/rounded-rectangle.ifc").replace("IFCROUNDEDRECTANGLEPROFILEDEF(.AREA.,$,#9,4.,4.,1.)","IFCRECTANGLEPROFILEDEF(.AREA.,$,#9,4.,4.)"),64.)]{let text=source.replace("'IFC4'","'IFC4'");let doc=semio_s_artifact_stdio_contract::part21::parse_part21(&text).unwrap();let snapshot=from_part21_document("stdio.ifc",&doc);let graph=Ifc4GeometryGraph(&snapshot);super::shared::preparation::tests::verify_graph(&graph,4,text.len(),expected);}}
