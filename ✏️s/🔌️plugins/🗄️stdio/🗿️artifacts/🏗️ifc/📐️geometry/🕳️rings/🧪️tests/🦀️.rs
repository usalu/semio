use super::*;

fn fixture_input(row: &semio_framework_pack_json::Value) -> (Vec<Point>, Vec<Vec<u32>>) {
    let mut vertices = Vec::new();
    let rings = row["rings"].as_array().unwrap().iter().map(|ring| ring.as_array().unwrap().iter().map(|point| {
        let index = vertices.len() as u32;
        vertices.push([0, 1, 2].map(|axis| point[axis].as_f64().unwrap()));
        index
    }).collect()).collect();
    (vertices, rings)
}

#[semio_framework_async_macros::async_test]
async fn language_neutral_rings_preserve_area_across_scale_and_orientation() {
    let contract = semio_framework_pack_json::parse(include_str!("🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for row in contract["cases"].as_array().unwrap() {
        for reversed in [false, true] { for scale in [1., 1e-120, 1e120] {
            let (mut vertices, mut rings) = fixture_input(row);
            for point in &mut vertices { for coordinate in point { *coordinate *= scale; } }
            if reversed { for ring in &mut rings { ring.reverse(); } }
            let triangles = triangulate_rings(&vertices, &rings, &mut || Ok(())).unwrap();
            let mut area = 0.;
            for triangle in triangles {
                let [a, b, c] = triangle.map(|id| [vertices[id as usize][0] / scale, vertices[id as usize][1] / scale]);
                let signed = cross(a, b, c);
                assert_eq!(signed > 0., !reversed);
                area += signed.abs() / 2.;
            }
            assert!((area - row["area"].as_f64().unwrap()).abs() < 1e-8);
        } }
    }
}

#[semio_framework_async_macros::async_test]
async fn invalid_rings_are_rejected() {
    let outer = vec![[0., 0., 0.], [10., 0., 0.], [10., 10., 0.], [0., 10., 0.]];
    let hole = vec![[2., 2., 0.], [4., 2., 0.], [4., 4., 0.], [2., 4., 0.]];
    let inputs = [
        vec![outer.clone(), vec![[11., 1., 0.], [12., 1., 0.], [12., 2., 0.]]],
        vec![outer.clone(), hole, vec![[2.5, 2.5, 0.], [3., 2.5, 0.], [3., 3., 0.]]],
        vec![outer.clone(), vec![[0., 2., 0.], [2., 2., 0.], [2., 4., 0.]]],
        vec![outer.clone(), vec![[2., 2., 0.], [8., 8., 0.], [2., 8., 0.], [8., 2., 0.]]],
        vec![outer, vec![[2., 2., 0.], [4., 2., 0.1], [4., 4., 0.]]],
    ];
    for input in inputs {
        let mut vertices = Vec::new();
        let rings: Vec<Vec<u32>> = input.into_iter().map(|ring| ring.into_iter().map(|point| { let id = vertices.len() as u32; vertices.push(point); id }).collect()).collect();
        assert!(triangulate_rings(&vertices, &rings, &mut || Ok(())).is_err());
    }
}

#[semio_framework_async_macros::async_test]
async fn every_ring_control_interruption_propagates() {
    let contract = semio_framework_pack_json::parse(include_str!("🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let (vertices, rings) = fixture_input(&contract["cases"][0]);
    let mut work = 0;
    triangulate_rings(&vertices, &rings, &mut || { work += 1; Ok(()) }).unwrap();
    for stop in 1..=work {
        let mut count = 0;
        let result = triangulate_rings(&vertices, &rings, &mut || { count += 1; if count == stop { Err("cancelled".into()) } else { Ok(()) } });
        assert_eq!(result.unwrap_err(), "cancelled");
        assert_eq!(count, stop);
    }
}

#[semio_framework_async_macros::async_test]
async fn collinear_boundary_segments_remain_individually_incident() {
    let contract = semio_framework_pack_json::parse(include_str!("🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for row in &contract["cases"].as_array().unwrap()[3..] {
    let (vertices, mut rings) = fixture_input(row);
    let triangles = triangulate_rings(&vertices, &rings, &mut || Ok(())).unwrap();
    let mut edges = BTreeMap::new();
    for triangle in triangles { for i in 0..3 { *edges.entry(edge_key(triangle[i], triangle[(i + 1) % 3])).or_insert(0) += 1; } }
    for ring in &mut rings {
        if vertices[ring[0] as usize] == vertices[*ring.last().unwrap() as usize] { ring.pop(); }
        for i in 0..ring.len() { assert_eq!(edges.get(&edge_key(ring[i], ring[(i + 1) % ring.len()])), Some(&1)); }
    }
    assert_eq!(edges.get(&edge_key(0, 1)), Some(&1));
    assert_eq!(edges.get(&edge_key(1, 2)), Some(&1));
    assert_ne!(edges.get(&edge_key(0, 2)), Some(&1));
    }
}

#[semio_framework_async_macros::async_test]
async fn oblique_holes_choose_visible_boundary_occurrences() {
    let row = semio_framework_pack_json::parse(include_str!("📜️oblique.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for reversed in [false, true] { for scale in [1., 1e-90, 1e90] {
        let mut vertices = Vec::new();
        let mut source = Vec::new();
        let mut rings = Vec::new();
        for (i, ring) in row["rings"].as_array().unwrap().iter().enumerate() {
            let mut boundary: Vec<[f64; 2]> = ring.as_array().unwrap().iter().map(|point| [point[0].as_f64().unwrap(), point[1].as_f64().unwrap()]).collect();
            if reversed && i % 2 == 0 { boundary.reverse(); }
            let mut indices = Vec::new();
            for point in boundary {
                let [x, y] = point;
                indices.push(vertices.len() as u32);
                source.push(point);
                vertices.push([(x / 2f64.sqrt() - y / 6f64.sqrt()) * scale, (x / 2f64.sqrt() + y / 6f64.sqrt()) * scale, 2. * y / 6f64.sqrt() * scale]);
            }
            rings.push(indices);
        }
        let triangles = triangulate_rings(&vertices, &rings, &mut || Ok(())).unwrap();
        assert_eq!(triangles.len() as u64, row["triangles"].as_u64().unwrap());
        let area: f64 = triangles.into_iter().map(|triangle| { let [a, b, c] = triangle.map(|id| source[id as usize]); cross(a, b, c).abs() / 2. }).sum();
        assert!((area - row["area"].as_f64().unwrap()).abs() < 1e-8);
    } }
}
