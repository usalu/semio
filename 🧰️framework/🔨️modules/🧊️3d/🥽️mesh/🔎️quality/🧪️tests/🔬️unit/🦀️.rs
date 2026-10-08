use super::*;
use crate::mesh::FaceId;
use parry3d::mass_properties::MassProperties;
use parry3d::math::Point;
use serde_json::Value;

fn fixtures() -> Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/🔎️quality/🔣️.json")).expect("mesh quality fixture parses")
}

fn soup(case: &Value) -> (Vec<[f32; 3]>, Vec<Vec<u32>>) {
    let positions = case["positions"].as_array().unwrap().iter().map(|p| {
        let p = p.as_array().unwrap();
        [p[0].as_f64().unwrap() as f32, p[1].as_f64().unwrap() as f32, p[2].as_f64().unwrap() as f32]
    });
    let faces = case["faces"].as_array().unwrap().iter().map(|f| f.as_array().unwrap().iter().map(|i| i.as_u64().unwrap() as u32).collect());
    (positions.collect(), faces.collect())
}

fn close(actual: f64, expected: f64, what: &str) {
    assert!((actual - expected).abs() <= 1e-9 * expected.abs().max(1.0), "{what}: {actual} vs {expected}");
}

fn vec3(value: &Value) -> [f64; 3] {
    let array = value.as_array().unwrap();
    [array[0].as_f64().unwrap(), array[1].as_f64().unwrap(), array[2].as_f64().unwrap()]
}

fn assert_case(name: &str, report: &MeshQualityReport, expected: &Value) {
    let count = |key: &str| expected[key].as_u64().map(|v| v as usize);
    for (key, actual) in [
        ("vertexCount", report.vertex_count),
        ("edgeCount", report.edge_count),
        ("faceCount", report.face_count),
        ("triangleCount", report.triangle_count),
        ("boundaryEdges", report.boundary_edges),
        ("nonManifoldEdges", report.non_manifold_edges),
        ("nonManifoldVertices", report.non_manifold_vertices),
        ("degenerateFaces", report.degenerate_faces),
        ("duplicateVertices", report.duplicate_vertices),
        ("isolatedVertices", report.isolated_vertices),
        ("connectedComponents", report.connected_components),
    ] {
        assert_eq!(Some(actual), count(key), "{name}.{key}");
    }
    let loops: Vec<usize> = expected["boundaryLoops"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as usize).collect();
    assert_eq!(report.boundary_loops, loops, "{name}.boundaryLoops");
    assert_eq!(report.closed, expected["closed"].as_bool().unwrap(), "{name}.closed");
    assert_eq!(report.orientable, expected["orientable"].as_bool().unwrap(), "{name}.orientable");
    assert_eq!(report.consistent_winding, expected["consistentWinding"].as_bool().unwrap(), "{name}.consistentWinding");
    assert_eq!(report.euler_characteristic, expected["eulerCharacteristic"].as_i64().unwrap(), "{name}.eulerCharacteristic");
    assert_eq!(report.genus, expected["genus"].as_i64(), "{name}.genus");
    close(report.area, expected["area"].as_f64().unwrap(), &format!("{name}.area"));
    close(report.signed_volume, expected["signedVolume"].as_f64().unwrap(), &format!("{name}.signedVolume"));
    let bounds = report.bounding_box.expect("bounds");
    assert_eq!(bounds.min.to_vec(), vec3(&expected["boundingBox"][0]).to_vec(), "{name}.bbox.min");
    assert_eq!(bounds.max.to_vec(), vec3(&expected["boundingBox"][1]).to_vec(), "{name}.bbox.max");
    let lengths = report.edge_length.expect("edge lengths");
    close(lengths.min, expected["edgeLength"]["min"].as_f64().unwrap(), &format!("{name}.edgeLength.min"));
    close(lengths.max, expected["edgeLength"]["max"].as_f64().unwrap(), &format!("{name}.edgeLength.max"));
    close(lengths.mean, expected["edgeLength"]["mean"].as_f64().unwrap(), &format!("{name}.edgeLength.mean"));
    if let Some(aspect) = expected.get("aspectRatio") {
        let actual = report.aspect_ratio.expect("aspect ratios");
        close(actual.min, aspect["min"].as_f64().unwrap(), &format!("{name}.aspect.min"));
        close(actual.max, aspect["max"].as_f64().unwrap(), &format!("{name}.aspect.max"));
        close(actual.mean, aspect["mean"].as_f64().unwrap(), &format!("{name}.aspect.mean"));
    }
    if expected.get("massAbsent").is_some() {
        assert!(report.mass.is_none(), "{name}: open or inconsistent meshes carry no mass");
        return;
    }
    if let Some(centroid) = expected.get("centroid") {
        let mass = report.mass.expect("closed consistent meshes carry mass");
        for (actual, wanted) in mass.centroid.iter().zip(vec3(centroid)) {
            assert!((actual - wanted).abs() < 1e-9, "{name}.centroid {actual} vs {wanted}");
        }
        close(mass.volume, expected["signedVolume"].as_f64().unwrap().abs(), &format!("{name}.mass.volume"));
        if let Some(inertia) = expected.get("inertia") {
            for i in 0..3 {
                for j in 0..3 {
                    let wanted = inertia[i][j].as_f64().unwrap();
                    assert!((mass.inertia[i][j] - wanted).abs() < 1e-9, "{name}.inertia[{i}][{j}] {} vs {wanted}", mass.inertia[i][j]);
                }
            }
        }
        if let Some(moments) = expected.get("principalMoments") {
            for (actual, wanted) in mass.principal.values.iter().zip(moments.as_array().unwrap()) {
                close(*actual, wanted.as_f64().unwrap(), &format!("{name}.principal"));
            }
        }
    }
}

#[test]
fn fixture_cases_match_the_closed_form_reports() {
    let fixtures = fixtures();
    for case in fixtures["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let (positions, faces) = soup(case);
        assert_case(name, &analyze_polygon_soup(&positions, &faces), &case["expected"]);
    }
}

#[test]
fn halfedge_meshes_report_what_their_polygon_soup_reports() {
    let fixtures = fixtures();
    let mut checked = 0;
    for case in fixtures["cases"].as_array().unwrap() {
        let (positions, faces) = soup(case);
        let Ok(mesh) = HalfedgeMesh::from_faces(&positions, &faces) else { continue };
        assert_eq!(mesh.quality_report(), analyze_polygon_soup(&positions, &faces), "{}", case["name"]);
        checked += 1;
    }
    assert!(checked >= 8, "only {checked} fixture meshes build as half-edge meshes");
}

#[test]
fn a_flipped_face_in_a_halfedge_mesh_is_reported_through_the_flip_flag() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    let before = mesh.quality_report();
    assert!(before.closed && before.consistent_winding);
    mesh.flip_faces(&[FaceId(0)]).unwrap();
    let after = mesh.quality_report();
    assert!(after.closed);
    assert_eq!(after.euler_characteristic, 2);
    close(after.area, before.area, "area survives a flip");
}

#[test]
fn parry3d_trimesh_mass_properties_agree_with_the_divergence_report() {
    let fixtures = fixtures();
    let mut compared = 0;
    for case in fixtures["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let (positions, faces) = soup(case);
        let report = analyze_polygon_soup(&positions, &faces);
        let Some(mass) = report.mass else { continue };
        if report.signed_volume < 0.0 {
            continue;
        }
        let points: Vec<Point<f32>> = positions.iter().map(|p| Point::new(p[0], p[1], p[2])).collect();
        let mut triangles: Vec<[u32; 3]> = Vec::new();
        for face in &faces {
            for k in 1..face.len() - 1 {
                triangles.push([face[0], face[k], face[k + 1]]);
            }
        }
        let reference = MassProperties::from_trimesh(1.0, &points, &triangles);
        let tolerance = 2e-4 * mass.volume.max(1.0);
        assert!((reference.mass() as f64 - mass.volume).abs() < tolerance, "{name}: volume {} vs {}", reference.mass(), mass.volume);
        for axis in 0..3 {
            assert!((reference.local_com[axis] as f64 - mass.centroid[axis]).abs() < 2e-4, "{name}: centroid[{axis}]");
        }
        let theirs = reference.reconstruct_inertia_matrix();
        for i in 0..3 {
            for j in 0..3 {
                assert!((theirs[(i, j)] as f64 - mass.inertia[i][j]).abs() < 2e-3 * mass.volume.max(1.0).max(mass.inertia[0][0].abs()), "{name}: inertia[{i}][{j}] {} vs {}", theirs[(i, j)], mass.inertia[i][j]);
            }
        }
        compared += 1;
    }
    assert!(compared >= 5, "compared {compared} meshes");
}
