//! 🧪️ Language-neutral modeling fixtures assert the surface, not just growing arrays.
use super::*;

#[test]
fn knife_cut_fixtures_preserve_surfaces_and_shared_boundaries() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/✂️knife-cut/🔣️.json")).unwrap();
    for case in fixtures["cases"].as_array().unwrap() {
        let positions: Vec<[f32; 3]> = serde_json::from_value(case["mesh"]["vertices"].clone()).unwrap();
        let faces: Vec<Vec<u32>> = serde_json::from_value(case["mesh"]["faces"].clone()).unwrap();
        let start: [f32; 3] = serde_json::from_value(case["cut"]["start"].clone()).unwrap();
        let end: [f32; 3] = serde_json::from_value(case["cut"]["end"].clone()).unwrap();
        let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
        let label = case["name"].as_str().unwrap();
        mesh.knife_cut(FaceId(case["cut"]["face"].as_u64().unwrap() as u32), Vec3(start), Vec3(end)).unwrap();
        let expected = &case["expected"];
        if let Some(count) = expected["vertices"].as_u64() { assert_eq!(mesh.vertex_count(), count as usize, "{label}"); }
        if let Some(count) = expected["faces"].as_u64() { assert_eq!(mesh.face_count(), count as usize, "{label}"); }
        if let Some(count) = expected["minimumFaces"].as_u64() { assert!(mesh.face_count() >= count as usize, "{label}"); }
        let (actual_positions, actual_faces) = mesh.polygon_soup();
        assert_eq!(&actual_positions[..positions.len()], &positions, "{label}: original vertices changed");
        let selected = &faces[case["cut"]["face"].as_u64().unwrap() as usize];
        let point = |p: [f32; 3]| (p[0] as f64, p[1] as f64, p[2] as f64);
        let normal = newell_normal(&selected.iter().map(|&id| Vec3(positions[id as usize])).collect::<Vec<_>>());
        let plane = cross3(sub3(point(end), point(start)), point(normal.0));
        let extent = selected.iter().map(|&id| length3(sub3(point(positions[id as usize]), point(positions[selected[0] as usize])))).fold(0.0f64, f64::max);
        for face in &actual_faces {
            if !face.iter().all(|id| *id as usize >= positions.len() || selected.contains(id)) { continue; }
            let distances = face.iter().map(|&id| dot3(sub3(point(actual_positions[id as usize]), point(start)), plane) / length3(plane)).collect::<Vec<_>>();
            assert!(!(distances.iter().any(|&d| d > extent * 1e-6) && distances.iter().any(|&d| d < -extent * 1e-6)), "{label}: a cut piece still crosses the knife plane");
        }
        for point in case["boundaryPoints"].as_array().unwrap() {
            let point: [f32; 3] = serde_json::from_value(point.clone()).unwrap();
            assert!(actual_positions.contains(&point), "{label}: missing boundary point {point:?}");
        }
        if let Some(unchanged) = case["unchangedFaces"].as_array() {
            for face in unchanged {
                let face: Vec<u32> = serde_json::from_value(face.clone()).unwrap();
                assert!(actual_faces.contains(&face), "{label}: unrelated face changed");
            }
        }
        let mut incidence = HashMap::<(u32, u32), (usize, i32)>::new();
        let mut used = HashSet::new();
        for face in &actual_faces {
            for i in 0..face.len() {
                let (a, b) = (face[i], face[(i + 1) % face.len()]);
                used.insert(a);
                let entry = incidence.entry((a.min(b), a.max(b))).or_default();
                entry.0 += 1; entry.1 += if a < b { 1 } else { -1 };
            }
        }
        assert_eq!(used.len(), mesh.vertex_count(), "{label}: unused vertices");
        assert!(incidence.values().all(|edge| edge.0 == 1 || *edge == (2, 0)), "{label}: broken adjacency");
        assert_eq!(incidence.values().filter(|edge| edge.0 == 1).count(), expected["boundaryEdges"].as_u64().unwrap() as usize, "{label}");
        let transfer = mesh.tessellate().unwrap();
        let mut area = 0.0;
        let mut volume = 0.0;
        for triangle in transfer.indices.chunks_exact(3) {
            let point = |id: u32| { let i = id as usize * 3; (transfer.positions[i] as f64, transfer.positions[i + 1] as f64, transfer.positions[i + 2] as f64) };
            let [a, b, c] = [point(triangle[0]), point(triangle[1]), point(triangle[2])];
            let normal = cross3(sub3(b, a), sub3(c, a));
            let triangle_area = length3(normal) / 2.0;
            assert!(triangle_area > 0.0, "{label}: degenerate triangle");
            if let Some(n) = case["normal"].as_array() { assert!(dot3(normal, (n[0].as_f64().unwrap(), n[1].as_f64().unwrap(), n[2].as_f64().unwrap())) > 0.0, "{label}: reversed winding"); }
            area += triangle_area; volume += dot3(a, cross3(b, c)) / 6.0;
        }
        assert!((area / expected["area"].as_f64().unwrap() - 1.0).abs() < 1e-6, "{label}: area={area}");
        if let Some(expected) = expected["volume"].as_f64() { assert!((volume / expected - 1.0).abs() < 1e-6, "{label}: volume={volume}"); }
        eprintln!("[DEBUG] knife cut {label}: vertices={} faces={} area={area} volume={volume}", mesh.vertex_count(), mesh.face_count());
    }
}

#[test]
fn knife_cut_rejects_invalid_requests_atomically() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/✂️knife-cut/🔣️.json")).unwrap();
    let positions: Vec<[f32; 3]> = serde_json::from_value(fixtures["cases"][0]["mesh"]["vertices"].clone()).unwrap();
    let faces: Vec<Vec<u32>> = serde_json::from_value(fixtures["cases"][0]["mesh"]["faces"].clone()).unwrap();
    let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
    let before = mesh.to_obj().unwrap();
    for case in fixtures["invalid"].as_array().unwrap() {
        let start: [f32; 3] = serde_json::from_value(case["cut"]["start"].clone()).unwrap();
        let end: [f32; 3] = serde_json::from_value(case["cut"]["end"].clone()).unwrap();
        assert!(mesh.knife_cut(FaceId(case["cut"]["face"].as_u64().unwrap() as u32), Vec3(start), Vec3(end)).is_err(), "{}", case["name"]);
        assert_eq!(mesh.to_obj().unwrap(), before);
    }
    for start in [[f32::INFINITY, 0.0, 0.0], [f32::NAN, 0.0, 0.0]] {
        assert!(mesh.knife_cut(FaceId(0), Vec3(start), Vec3::new(1.0, 3.0, 0.0)).is_err());
        assert_eq!(mesh.to_obj().unwrap(), before);
    }
}

#[test]
fn component_transforms_apply_duplicate_vertices_once_and_reject_partial_mutation() {
    for operation in 0..3 {
        let apply = |mesh: &mut HalfedgeMesh, ids: &[VertexId]| match operation {
            0 => mesh.move_vertices(ids, Vec3::new(1.0, 2.0, 3.0)),
            1 => mesh.rotate_vertices(ids, Vec3::new(0.0, 0.0, 1.0), 0.5, Vec3::ZERO),
            _ => mesh.scale_vertices(ids, Vec3::new(2.0, 3.0, 4.0), Vec3::ZERO),
        };
        let mut once = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        let mut repeated = once.clone();
        let before = once.to_obj().unwrap();
        assert!(apply(&mut once, &[VertexId(0), VertexId(999)]).is_err());
        assert_eq!(once.to_obj().unwrap(), before);
        apply(&mut once, &[VertexId(0)]).unwrap();
        apply(&mut repeated, &[VertexId(0), VertexId(0)]).unwrap();
        assert_eq!(once.to_obj().unwrap(), repeated.to_obj().unwrap());
    }
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    let before = mesh.to_obj().unwrap();
    assert!(mesh.move_vertices(&[VertexId(0)], Vec3::new(f32::INFINITY, 0.0, 0.0)).is_err());
    assert!(mesh.rotate_vertices(&[VertexId(0)], Vec3::ZERO, 1.0, Vec3::ZERO).is_err());
    assert!(mesh.scale_vertices(&[VertexId(0)], Vec3::new(1.0, 0.0, 1.0), Vec3::ZERO).is_err());
    assert_eq!(mesh.to_obj().unwrap(), before);
}

#[test]
fn loop_cut_fixtures_preserve_connected_surfaces() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for case in fixtures["loopCuts"].as_array().unwrap() {
        let positions: Vec<[f32; 3]> = serde_json::from_value(case["mesh"]["vertices"].clone()).unwrap();
        let faces: Vec<Vec<u32>> = serde_json::from_value(case["mesh"]["faces"].clone()).unwrap();
        let seeds: Vec<[u32; 2]> = serde_json::from_value(case["edges"].clone()).unwrap();
        let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
        let selected: Vec<_> = seeds.iter().map(|&[a, b]| {
            let find = |a, b| (0..mesh.halfedges.len()).find(|&id| mesh.edge_endpoints(EdgeId(id as u32)).unwrap() == (VertexId(a), VertexId(b)));
            EdgeId(find(a, b).or_else(|| find(b, a)).unwrap() as u32)
        }).collect();
        mesh.loop_cut(&selected, case["cuts"].as_u64().unwrap() as u32).unwrap();
        let expected = &case["expected"];
        let label = case["name"].as_str().unwrap();
        assert_eq!(mesh.vertex_count(), expected["vertices"].as_u64().unwrap() as usize, "{label}");
        assert_eq!(mesh.face_count(), expected["faces"].as_u64().unwrap() as usize, "{label}");
        if let Some(expected) = case.get("addedPositions") {
            let expected: Vec<[f32; 3]> = serde_json::from_value(expected.clone()).unwrap();
            let actual = (positions.len()..mesh.vertex_count()).map(|id| mesh.vertex_position(VertexId(id as u32)).unwrap().0).collect::<Vec<_>>();
            assert_eq!(actual.len(), expected.len(), "{label}");
            for point in expected { assert!(actual.contains(&point), "{label}: missing cut point {point:?}"); }
        }
        if let Some(expected) = case.get("unchangedFaces") {
            let expected: Vec<Vec<u32>> = serde_json::from_value(expected.clone()).unwrap();
            let (_, actual) = mesh.polygon_soup();
            for face in expected { assert!(actual.contains(&face), "{label}: untouched face changed"); }
        }
        let mut incidence = HashMap::<(u32, u32), (usize, i32)>::new();
        let mut used = HashSet::new();
        for face in 0..mesh.face_count() {
            let ids = mesh.face_vertex_ids(FaceId(face as u32)).unwrap();
            for i in 0..ids.len() {
                let (a, b) = (ids[i].0, ids[(i + 1) % ids.len()].0);
                used.insert(a);
                let entry = incidence.entry((a.min(b), a.max(b))).or_default();
                entry.0 += 1; entry.1 += if a < b { 1 } else { -1 };
            }
        }
        assert_eq!(used.len(), mesh.vertex_count(), "{label}: disconnected vertices");
        assert_eq!(incidence.len(), expected["edges"].as_u64().unwrap() as usize, "{label}");
        assert!(incidence.values().all(|edge| edge.0 == 1 || *edge == (2, 0)), "{label}: nonmanifold or inverted edge");
        assert_eq!(incidence.values().filter(|edge| edge.0 == 1).count(), expected["boundaryEdges"].as_u64().unwrap() as usize, "{label}");
        let transfer = mesh.tessellate().unwrap();
        let mut area = 0.0;
        let mut volume = 0.0;
        for triangle in transfer.indices.chunks_exact(3) {
            let point = |id: u32| { let offset = id as usize * 3; (transfer.positions[offset] as f64, transfer.positions[offset + 1] as f64, transfer.positions[offset + 2] as f64) };
            let [a, b, c] = [point(triangle[0]), point(triangle[1]), point(triangle[2])];
            let normal = cross3(sub3(b, a), sub3(c, a));
            let triangle_area = normal.0.hypot(normal.1).hypot(normal.2) / 2.0;
            assert!(triangle_area > 0.0, "{label}: degenerate triangle");
            area += triangle_area; volume += dot3(a, cross3(b, c)) / 6.0;
        }
        assert!((area - expected["area"].as_f64().unwrap()).abs() < 1e-6, "{label}: area={area}");
        if let Some(expected) = expected["volume"].as_f64() { assert!((volume - expected).abs() < 1e-6, "{label}: volume={volume}"); }
        eprintln!("[DEBUG] loop cut {label}: vertices={} faces={} area={area} volume={volume}", mesh.vertex_count(), mesh.face_count());
    }
}

#[test]
fn loop_cut_rejects_invalid_requests_atomically() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for case in fixtures["invalidLoopCuts"].as_array().unwrap() {
        let positions: Vec<[f32; 3]> = serde_json::from_value(case["mesh"]["vertices"].clone()).unwrap();
        let faces: Vec<Vec<u32>> = serde_json::from_value(case["mesh"]["faces"].clone()).unwrap();
        let edges = case["edges"].as_array().unwrap().iter().map(|id| EdgeId(id.as_u64().unwrap() as u32)).collect::<Vec<_>>();
        let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
        let before = mesh.to_obj().unwrap();
        assert!(mesh.loop_cut(&edges, case["cuts"].as_u64().unwrap() as u32).is_err(), "{}", case["name"]);
        assert_eq!(mesh.to_obj().unwrap(), before);
    }
    for (edges, cuts) in [(vec![], 1), (vec![EdgeId(0)], 0), (vec![EdgeId(0)], 257), (vec![EdgeId(0), EdgeId(999)], 1), (vec![EdgeId(0), EdgeId(1), EdgeId(8)], 256)] {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        let before = mesh.to_obj().unwrap();
        assert!(mesh.loop_cut(&edges, cuts).is_err());
        assert_eq!(mesh.to_obj().unwrap(), before);
    }
    let mut triangle = HalfedgeMesh::from_faces(&[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]], &[vec![0, 1, 2]]).unwrap();
    let before = triangle.to_obj().unwrap();
    assert!(triangle.loop_cut(&[EdgeId(0)], 1).is_err());
    assert_eq!(triangle.to_obj().unwrap(), before);
}

#[test]
fn vector_normalization_preserves_direction_across_scales() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for direction in fixtures["directions"].as_array().unwrap() {
        for scale in fixtures["scales"].as_array().unwrap() {
            let scale = scale.as_f64().unwrap();
            let vector: [f64; 3] = serde_json::from_value(direction["vector"].clone()).unwrap();
            let expected: [f32; 3] = serde_json::from_value(direction["unit"].clone()).unwrap();
            let actual = Vec3(vector.map(|coordinate| (coordinate * scale) as f32)).normalize();
            for axis in 0..3 { assert!((actual.0[axis] - expected[axis]).abs() < 1e-6, "scale={scale}: {actual:?}"); }
        }
    }
}

#[test]
fn sphere_refinement_has_no_missing_center_faces() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for case in fixtures["spheres"].as_array().unwrap() {
        let radius = case["radius"].as_f64().unwrap();
        let level = case["subdivisions"].as_u64().unwrap() as u32;
        let mesh = HalfedgeMesh::ico_sphere_prim(radius as f32, level).unwrap();
        assert_eq!(mesh.vertex_count(), case["vertices"].as_u64().unwrap() as usize);
        assert_eq!(mesh.face_count(), case["faces"].as_u64().unwrap() as usize);
        let mut edges = HashMap::<(u32, u32), (usize, i32)>::new();
        for face in 0..mesh.face_count() {
            let ids = mesh.face_vertex_ids(FaceId(face as u32)).unwrap();
            let points: Vec<_> = ids.iter().map(|id| mesh.vertex_position(*id).unwrap().0.map(|coordinate| coordinate as f64 / radius)).collect();
            let [a, b, c] = [points[0], points[1], points[2]].map(|p| (p[0], p[1], p[2]));
            assert!(dot3(a, cross3(sub3(b, a), sub3(c, a))) > 0.0);
            for i in 0..3 {
                let (a, b) = (ids[i].0, ids[(i + 1) % 3].0);
                let entry = edges.entry((a.min(b), a.max(b))).or_default();
                entry.0 += 1; entry.1 += if a < b { 1 } else { -1 };
            }
        }
        assert!(edges.values().all(|edge| *edge == (2, 0)));
        for vertex in 0..mesh.vertex_count() {
            let point = mesh.vertex_position(VertexId(vertex as u32)).unwrap().0.map(|coordinate| coordinate as f64 / radius);
            assert!((point[0].hypot(point[1]).hypot(point[2]) - 1.0).abs() < 1e-6);
        }
        eprintln!("[DEBUG] sphere level={level} radius={radius}: {} closed outward faces", mesh.face_count());
    }
    for (radius, level) in [(0.0, 0), (-1.0, 1), (f32::NAN, 1), (1.0, 6)] { assert!(HalfedgeMesh::ico_sphere_prim(radius, level).is_err()); }
}

#[test]
fn polygon_tessellation_preserves_shape_at_all_fixture_scales() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for case in fixtures["polygons"].as_array().unwrap() {
        for scale in fixtures["scales"].as_array().unwrap() {
            let scale = scale.as_f64().unwrap();
            let source: Vec<[f64; 3]> = serde_json::from_value(case["vertices"].clone()).unwrap();
            let positions: Vec<_> = source.iter().map(|point| point.map(|coordinate| (coordinate * scale) as f32)).collect();
            let faces: Vec<Vec<u32>> = serde_json::from_value(case["faces"].clone()).unwrap();
            let mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
            let transfer = mesh.tessellate().unwrap();
            let mut area = 0.0;
            for triangle in transfer.indices.chunks_exact(3) {
                let point = |index: u32| (transfer.positions[index as usize * 3] as f64 / scale, transfer.positions[index as usize * 3 + 1] as f64 / scale, transfer.positions[index as usize * 3 + 2] as f64 / scale);
                let [a, b, c] = [point(triangle[0]), point(triangle[1]), point(triangle[2])];
                let signed_area = cross3(sub3(b, a), sub3(c, a)).2 / 2.0;
                assert!(signed_area > 0.0, "scale={scale}: inverted or collapsed triangle");
                area += signed_area;
            }
            assert!((area - case["area"].as_f64().unwrap()).abs() < 1e-6, "scale={scale}: area={area}");
            for normal in transfer.normals.chunks_exact(3) { assert!((normal[2] - 1.0).abs() < 1e-6, "scale={scale}: normal={normal:?}"); }
            eprintln!("[DEBUG] polygon scale={scale}: normalized area={area}, triangles={}", transfer.indices.len() / 3);
        }
    }
}

#[test]
fn modeling_fixtures_preserve_closed_oriented_surfaces() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for case in fixtures["cases"].as_array().unwrap() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        let selected: Vec<FaceId> = case["selection"].as_array().unwrap().iter().map(|id| FaceId(id.as_u64().unwrap() as u32)).collect();
        let amount = case["amount"].as_f64().unwrap() as f32;
        match case["operation"].as_str().unwrap() {
            "extrude" => mesh.extrude_faces(&selected, amount).unwrap(),
            "inset" => mesh.inset_faces(&selected, amount).unwrap(),
            "subdivide" => mesh.subdivide_faces(&selected).unwrap(),
            _ => panic!("unknown fixture operation"),
        }
        let label = case["name"].as_str().unwrap();
        assert_eq!(mesh.vertex_count(), case["vertices"].as_u64().unwrap() as usize, "{label}");
        assert_eq!(mesh.face_count(), case["faces"].as_u64().unwrap() as usize, "{label}");
        let mut incidence = HashMap::<(u32, u32), Vec<(u32, u32)>>::new();
        for face in 0..mesh.face_count() {
            let vertices = mesh.face_vertex_ids(FaceId(face as u32)).unwrap();
            for i in 0..vertices.len() {
                let a = vertices[i].0;
                let b = vertices[(i + 1) % vertices.len()].0;
                incidence.entry((a.min(b), a.max(b))).or_default().push((a, b));
            }
        }
        assert_eq!(incidence.values().filter(|uses| uses.len() == 1).count(), case["boundaryEdges"].as_u64().unwrap() as usize, "{label}");
        assert!(incidence.values().all(|uses| uses.len() == 2 && uses[0] == (uses[1].1, uses[1].0)), "{label}: winding or nonmanifold edge");
        let transfer = mesh.tessellate().unwrap();
        let mut area = 0.0f64;
        let mut volume = 0.0f64;
        for triangle in transfer.indices.chunks_exact(3) {
            let point = |index: u32| Vec3::new(transfer.positions[index as usize * 3], transfer.positions[index as usize * 3 + 1], transfer.positions[index as usize * 3 + 2]);
            let [a, b, c] = [point(triangle[0]), point(triangle[1]), point(triangle[2])];
            area += b.sub(a).cross(c.sub(a)).length() as f64 / 2.0;
            volume += a.dot(b.cross(c)) as f64 / 6.0;
        }
        assert!((area - case["area"].as_f64().unwrap()).abs() < 1e-5, "{label}: area {area}");
        assert!((volume - case["volume"].as_f64().unwrap()).abs() < 1e-5, "{label}: signed volume {volume}");
        eprintln!("[DEBUG] {label}: vertices={} faces={} area={area} volume={volume}", mesh.vertex_count(), mesh.face_count());
    }
}

#[test]
fn modeling_rejects_invalid_selection_without_mutation() {
    for operation in [HalfedgeMesh::extrude_faces, HalfedgeMesh::inset_faces] {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        let before = mesh.to_obj().unwrap();
        assert!(operation(&mut mesh, &[FaceId(0), FaceId(999)], 0.1).is_err());
        assert_eq!(mesh.to_obj().unwrap(), before);
        assert!(operation(&mut mesh, &[FaceId(0)], f32::NAN).is_err());
        assert_eq!(mesh.to_obj().unwrap(), before);
    }
}

#[test]
fn subdivision_preserves_concave_polygon_area() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for case in fixtures["polygons"].as_array().unwrap() {
        let positions: Vec<[f32; 3]> = serde_json::from_value(case["vertices"].clone()).unwrap();
        let faces: Vec<Vec<u32>> = serde_json::from_value(case["faces"].clone()).unwrap();
        let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
        mesh.subdivide_faces(&[FaceId(0)]).unwrap();
        assert_eq!(mesh.vertex_count(), case["expectedVertices"].as_u64().unwrap() as usize);
        assert_eq!(mesh.face_count(), case["expectedFaces"].as_u64().unwrap() as usize);
        let transfer = mesh.tessellate().unwrap();
        let mut area = 0.0;
        for triangle in transfer.indices.chunks_exact(3) {
            let point = |index: u32| Vec3::new(transfer.positions[index as usize * 3], transfer.positions[index as usize * 3 + 1], transfer.positions[index as usize * 3 + 2]);
            let [a, b, c] = [point(triangle[0]), point(triangle[1]), point(triangle[2])];
            let signed_area = b.sub(a).cross(c.sub(a)).z() / 2.0;
            assert!(signed_area > 0.0);
            area += signed_area;
        }
        assert!((area as f64 - case["area"].as_f64().unwrap()).abs() < 1e-5);
        eprintln!("[DEBUG] concave subdivision: area={area}, faces={}", mesh.face_count());
    }
}
