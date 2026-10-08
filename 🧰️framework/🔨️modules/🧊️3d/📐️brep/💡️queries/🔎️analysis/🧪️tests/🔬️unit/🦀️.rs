use super::*;
use crate::brep::operations::primitives::{make_box, make_cone, make_cylinder, make_sphere, make_torus};
use crate::brep::representation::curve::bspline::KnotVector;
use crate::brep::representation::curve::Curve3;
use crate::brep::representation::surface::Surface;
use crate::brep::representation::topology::history::OpRecorder;
use crate::brep::representation::topology::Shell;
use crate::brep::representation::vector::matrix::Frame3;
use crate::brep::representation::vector::{Pnt3, Vec3};
use serde_json::Value;

const FIXTURE: &str = include_str!("../../../../🧫️fixtures/🔎️analysis/🔣️.json");
const MASS_TOLERANCE: f64 = 1e-6;

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("analysis fixture parses")
}

fn number(value: &Value) -> f64 {
    value.as_f64().expect("number")
}

fn triple(value: &Value) -> [f64; 3] {
    [number(&value[0]), number(&value[1]), number(&value[2])]
}

fn param(case: &Value, key: &str) -> f64 {
    number(&case["params"][key])
}

fn build(body: &mut Body, case: &Value) -> SolidId {
    let mut rec = OpRecorder::new();
    let made = match case["primitive"].as_str().unwrap() {
        "box" => make_box(body, param(case, "width"), param(case, "depth"), param(case, "height"), &mut rec),
        "sphere" => make_sphere(body, param(case, "radius"), &mut rec),
        "cylinder" => make_cylinder(body, param(case, "radius"), param(case, "height"), &mut rec),
        "cone" => make_cone(body, param(case, "radius"), param(case, "height"), &mut rec),
        "torus" => make_torus(body, param(case, "major"), param(case, "minor"), &mut rec),
        other => panic!("unknown primitive {other}"),
    };
    made.expect("primitive builds")
}

fn close(actual: f64, expected: f64, relative: f64, what: &str) {
    assert!((actual - expected).abs() <= relative * expected.abs().max(1.0), "{what}: {actual} vs {expected}");
}

fn kind_name(kind: SurfaceKind) -> &'static str {
    match kind {
        SurfaceKind::Plane => "plane",
        SurfaceKind::Cylinder => "cylinder",
        SurfaceKind::Cone => "cone",
        SurfaceKind::Sphere => "sphere",
        SurfaceKind::Torus => "torus",
        SurfaceKind::Nurbs => "nurbs",
    }
}

#[test]
fn primitives_match_their_closed_form_mass_properties_and_bounds() {
    for case in fixture()["primitives"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let expected = &case["expected"];
        let mut body = Body::new();
        let solid = build(&mut body, case);
        let scope = ShapeScope::solid(solid);
        let mass = mass_properties(&body, &scope, MASS_TOLERANCE).unwrap();
        let volumetric = mass.volumetric.expect("a solid has volume");
        close(volumetric.volume, number(&expected["volume"]), 1e-6, &format!("{name} volume"));
        close(mass.area, number(&expected["area"]), 1e-6, &format!("{name} area"));
        let centroid = triple(&expected["centroid"]);
        let scale = number(&expected["volume"]).cbrt();
        for axis in 0..3 {
            assert!((volumetric.centroid[axis] - centroid[axis]).abs() <= 1e-6 * scale, "{name} centroid[{axis}] {} vs {}", volumetric.centroid[axis], centroid[axis]);
        }
        let inertia_scale = (0..3).map(|i| number(&expected["inertia"][i][i])).fold(0.0, f64::max);
        for i in 0..3 {
            for j in 0..3 {
                let wanted = number(&expected["inertia"][i][j]);
                assert!((volumetric.inertia[i][j] - wanted).abs() <= 1e-5 * inertia_scale, "{name} inertia[{i}][{j}] {} vs {wanted}", volumetric.inertia[i][j]);
            }
        }
        let mut moments: Vec<f64> = (0..3).map(|i| number(&expected["inertia"][i][i])).collect();
        moments.sort_by(f64::total_cmp);
        for (actual, wanted) in volumetric.principal.values.iter().zip(&moments) {
            assert!((actual - wanted).abs() <= 1e-5 * inertia_scale, "{name} principal {actual} vs {wanted}");
        }
        for (k, axis) in volumetric.principal.axes.iter().enumerate() {
            for i in 0..3 {
                let image: f64 = (0..3).map(|j| volumetric.inertia[i][j] * axis[j]).sum();
                assert!((image - volumetric.principal.values[k] * axis[i]).abs() <= 1e-9 * inertia_scale, "{name} axis {k} is not an eigenvector");
            }
        }
        let bounds = bounding_box(&body, &scope).unwrap();
        assert!(bounds.exact, "{name} bounds are analytic");
        for axis in 0..3 {
            assert!((bounds.min[axis] - number(&expected["boundingBox"][0][axis])).abs() < 1e-9, "{name} bbox min[{axis}] {}", bounds.min[axis]);
            assert!((bounds.max[axis] - number(&expected["boundingBox"][1][axis])).abs() < 1e-9, "{name} bbox max[{axis}] {}", bounds.max[axis]);
        }
    }
}

#[test]
fn primitives_report_their_topology_validity_and_watertightness() {
    for case in fixture()["primitives"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let counts = &case["expected"]["counts"];
        let mut body = Body::new();
        let solid = build(&mut body, case);
        let scope = ShapeScope::solid(solid);
        let actual = topology_counts(&body, &scope).unwrap();
        let wanted = |key: &str| counts[key].as_u64().unwrap() as usize;
        assert_eq!((actual.solids, actual.shells, actual.faces, actual.loops, actual.inner_loops, actual.edges, actual.degenerate_edges, actual.vertices), (wanted("solids"), wanted("shells"), wanted("faces"), wanted("loops"), wanted("innerLoops"), wanted("edges"), wanted("degenerateEdges"), wanted("vertices")), "{name}");
        assert_eq!(actual.euler_characteristic, counts["eulerCharacteristic"].as_i64().unwrap(), "{name} euler");
        assert_eq!(actual.genus, counts["genus"].as_i64(), "{name} genus");
        assert_eq!(actual.closed_shells, 1, "{name}");
        let manifold = manifold_report(&body, &scope).unwrap();
        assert_eq!(manifold.verdict, Watertightness::Watertight, "{name}");
        assert!(manifold.closed && manifold.manifold && manifold.consistently_oriented, "{name}");
        let report = validity(&body, &scope).unwrap();
        assert!(report.ok, "{name}: {:?}", report.issues);
    }
}

#[test]
fn face_and_edge_tables_describe_every_primitive() {
    for case in fixture()["primitives"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let expected = &case["expected"];
        let mut body = Body::new();
        let solid = build(&mut body, case);
        let scope = ShapeScope::solid(solid);
        let faces = face_table(&body, &scope, MASS_TOLERANCE).unwrap();
        let wanted_faces = expected["faces"].as_array().unwrap();
        assert_eq!(faces.len(), wanted_faces.len(), "{name}");
        let mut kinds: Vec<&str> = faces.iter().map(|row| kind_name(row.surface_kind)).collect();
        let mut wanted_kinds: Vec<&str> = expected["faceKinds"].as_array().unwrap().iter().map(|k| k.as_str().unwrap()).collect();
        kinds.sort_unstable();
        wanted_kinds.sort_unstable();
        assert_eq!(kinds, wanted_kinds, "{name}");
        let mut unmatched: Vec<&FaceRow> = faces.iter().collect();
        for wanted in wanted_faces {
            let position = unmatched
                .iter()
                .position(|row| {
                    let same_normal = match (&row.normal, wanted["normal"].as_array()) {
                        (None, None) => true,
                        (Some(n), Some(w)) => (0..3).all(|axis| (n[axis] - number(&w[axis])).abs() < 1e-9),
                        _ => false,
                    };
                    same_normal && (row.area - number(&wanted["area"])).abs() < 1e-5 * number(&wanted["area"]).max(1.0) && (0..3).all(|axis| (row.centroid[axis] - number(&wanted["centroid"][axis])).abs() < 1e-6)
                })
                .unwrap_or_else(|| panic!("{name}: no face row matches {wanted}; rows {faces:?}"));
            unmatched.remove(position);
        }
        let labels: std::collections::BTreeSet<u64> = faces.iter().map(|row| row.label).collect();
        assert_eq!(labels.len(), faces.len(), "{name}: face labels are unique");
        for row in &faces {
            assert!(row.adjacent.iter().all(|label| labels.contains(label) && *label != row.label), "{name}: adjacency stays inside the solid");
        }

        let edges = edge_table(&body, &scope).unwrap();
        let wanted_edges = &expected["edges"];
        assert_eq!(edges.len(), wanted_edges["count"].as_u64().unwrap() as usize, "{name}");
        let convex = edges.iter().filter(|row| row.dihedral.is_some_and(|d| d.convexity == Convexity::Convex)).count();
        let concave = edges.iter().filter(|row| row.dihedral.is_some_and(|d| d.convexity == Convexity::Concave)).count();
        let without = edges.iter().filter(|row| row.dihedral.is_none()).count();
        assert_eq!((convex, concave, without), (wanted_edges["convex"].as_u64().unwrap() as usize, wanted_edges["concave"].as_u64().unwrap() as usize, wanted_edges["noDihedral"].as_u64().unwrap() as usize), "{name}");
        if let Some(angle) = wanted_edges.get("interiorAngle") {
            for row in edges.iter().filter_map(|row| row.dihedral) {
                assert!((row.interior_angle - number(angle)).abs() < 1e-9, "{name}: interior angle {} vs {}", row.interior_angle, number(angle));
            }
        }
        let lengths: Vec<f64> = edges.iter().map(|row| row.length).collect();
        close(lengths.iter().copied().fold(f64::INFINITY, f64::min), number(&wanted_edges["lengths"]["min"]), 1e-9, &format!("{name} min edge"));
        close(lengths.iter().copied().fold(0.0, f64::max), number(&wanted_edges["lengths"]["max"]), 1e-9, &format!("{name} max edge"));
        close(lengths.iter().sum::<f64>(), number(&wanted_edges["lengths"]["total"]), 1e-9, &format!("{name} total edge length"));
    }
}

fn frame_for(case: &Value) -> Surface {
    let surface = &case["surface"];
    match surface["kind"].as_str().unwrap() {
        "sphere" => Surface::Sphere { frame: Frame3::WORLD, radius: number(&surface["radius"]) },
        "cylinder" => Surface::Cylinder { frame: Frame3::WORLD, radius: number(&surface["radius"]) },
        "cone" => Surface::Cone { frame: Frame3::WORLD, half_angle: number(&surface["halfAngle"]) },
        "torus" => Surface::Torus { frame: Frame3::WORLD, major_radius: number(&surface["major"]), minor_radius: number(&surface["minor"]) },
        "paraboloid" => {
            let heights = [1.0, -1.0, 1.0];
            let controls = (0..3).map(|i| (0..3).map(|j| Pnt3::new(i as f64 - 1.0, j as f64 - 1.0, heights[i] + heights[j])).collect()).collect();
            Surface::Nurbs { u_knots: KnotVector::clamped_uniform(3, 2), v_knots: KnotVector::clamped_uniform(3, 2), controls, weights: vec![vec![1.0; 3]; 3] }
        }
        other => panic!("unknown surface {other}"),
    }
}

#[test]
fn surface_differentials_match_the_closed_form_curvatures() {
    for case in fixture()["curvature"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let surface = frame_for(case);
        let flipped = case["flipped"].as_bool().unwrap_or(false);
        for sample in case["samples"].as_array().unwrap() {
            let d = surface_differential(&surface, flipped, number(&sample["u"]), number(&sample["v"])).unwrap();
            let principal = sample.get("principal").or_else(|| case["expected"].get("principal")).unwrap();
            for index in 0..2 {
                assert!((d.principal_curvatures[index] - number(&principal[index])).abs() < 1e-9, "{name}: k{index} {} vs {}", d.principal_curvatures[index], number(&principal[index]));
            }
            let gaussian = sample.get("gaussian").or_else(|| case["expected"].get("gaussian")).map(number).unwrap_or(d.gaussian);
            assert!((d.gaussian - gaussian).abs() < 1e-9, "{name}: K {} vs {gaussian}", d.gaussian);
            assert!((d.mean - (d.principal_curvatures[0] + d.principal_curvatures[1]) / 2.0).abs() < 1e-12);
            if let Some(mean) = sample.get("mean").or_else(|| case["expected"].get("mean")) {
                assert!((d.mean - number(mean)).abs() < 1e-9, "{name}: H {} vs {}", d.mean, number(mean));
            }
            if let Some(umbilic) = case["expected"].get("umbilic") {
                assert_eq!(d.umbilic, umbilic.as_bool().unwrap(), "{name}");
            }
            let [a, b] = d.principal_directions;
            let dot = |x: [f64; 3], y: [f64; 3]| x[0] * y[0] + x[1] * y[1] + x[2] * y[2];
            assert!(dot(a, b).abs() < 1e-9 && (dot(a, a) - 1.0).abs() < 1e-9 && (dot(b, b) - 1.0).abs() < 1e-9, "{name}: orthonormal directions");
            assert!(dot(a, d.normal).abs() < 1e-9 && dot(b, d.normal).abs() < 1e-9, "{name}: directions are tangent");
            if case["expected"]["firstDirection"] == "circumferential" {
                let u = number(&sample["u"]);
                assert!((dot(a, [-u.sin(), u.cos(), 0.0]).abs() - 1.0).abs() < 1e-9, "{name}: first direction runs around the axis");
                assert!((dot(b, [0.0, 0.0, 1.0]).abs() - 1.0).abs() < 1e-9, "{name}: second direction runs along the axis");
            }
        }
    }
}

#[test]
fn a_singular_pole_has_no_differential() {
    let sphere = Surface::Sphere { frame: Frame3::WORLD, radius: 1.0 };
    assert!(surface_differential(&sphere, false, 0.3, std::f64::consts::FRAC_PI_2).is_err());
}

#[test]
fn flipping_a_face_negates_its_curvature_but_not_its_gaussian() {
    let sphere = Surface::Sphere { frame: Frame3::WORLD, radius: 2.0 };
    let outward = surface_differential(&sphere, false, 0.7, 0.3).unwrap();
    let inward = surface_differential(&sphere, true, 0.7, 0.3).unwrap();
    assert!((outward.mean + inward.mean).abs() < 1e-12);
    assert!((outward.gaussian - inward.gaussian).abs() < 1e-12);
}

fn placed_box(body: &mut Body, size: &Value, offset: &Value) -> SolidId {
    let mut scratch = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut scratch, number(&size[0]), number(&size[1]), number(&size[2]), &mut rec).unwrap();
    let delta = Vec3::new(number(&offset[0]), number(&offset[1]), number(&offset[2]));
    let ids: Vec<_> = scratch.vertices.ids().collect();
    for id in ids {
        let moved = scratch.vertices.get(id).unwrap().position + delta;
        scratch.vertices.get_mut(id).unwrap().position = moved;
    }
    let curve_ids: Vec<_> = scratch.curves3.ids().collect();
    for id in curve_ids {
        if let Curve3::Line { origin, .. } = scratch.curves3.get_mut(id).unwrap() {
            *origin = *origin + delta;
        }
    }
    let surface_ids: Vec<_> = scratch.surfaces.ids().collect();
    for id in surface_ids {
        if let Surface::Plane { frame } = scratch.surfaces.get_mut(id).unwrap() {
            frame.origin = frame.origin + delta;
        }
    }
    body.merge(&scratch).solids[&solid]
}

#[test]
fn solid_distances_match_the_fixture() {
    for case in fixture()["distances"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mut body = Body::new();
        let a = placed_box(&mut body, &case["a"]["box"], &case["a"]["offset"]);
        let b = placed_box(&mut body, &case["b"]["box"], &case["b"]["offset"]);
        let pair = shape_distance(&body, &ShapeScope::solid(a), &ShapeScope::solid(b)).unwrap();
        assert!((pair.distance - number(&case["distance"])).abs() < 1e-9, "{name}: {} vs {}", pair.distance, case["distance"]);
        let gap = ((pair.point_a[0] - pair.point_b[0]).powi(2) + (pair.point_a[1] - pair.point_b[1]).powi(2) + (pair.point_a[2] - pair.point_b[2]).powi(2)).sqrt();
        assert!((gap - pair.distance).abs() < 1e-9, "{name}: witness points realise the distance");
        let reverse = shape_distance(&body, &ShapeScope::solid(b), &ShapeScope::solid(a)).unwrap();
        assert!((reverse.distance - pair.distance).abs() < 1e-9, "{name}: symmetric");
    }
}

#[test]
fn point_distances_match_the_fixture_and_carry_the_inside_sign() {
    for case in fixture()["pointDistances"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mut body = Body::new();
        let solid = build(&mut body, &case["shape"]);
        let result = point_distance(&body, &ShapeScope::solid(solid), Pnt3::new(number(&case["point"][0]), number(&case["point"][1]), number(&case["point"][2]))).unwrap();
        assert!((result.distance - number(&case["distance"])).abs() < 1e-7, "{name}: {}", result.distance);
        assert!((result.signed.unwrap() - number(&case["signed"])).abs() < 1e-7, "{name}: signed {:?}", result.signed);
        for axis in 0..3 {
            assert!((result.closest[axis] - number(&case["closest"][axis])).abs() < 1e-6, "{name}: closest[{axis}] {}", result.closest[axis]);
        }
    }
}

#[test]
fn a_shell_missing_a_face_is_open_and_encloses_no_volume() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let faces = body.solid_faces(solid);
    let label = body.new_label();
    let shell = body.shells.insert(Shell { faces: faces[..5].to_vec(), label });
    let scope = ShapeScope::shell(shell);
    let report = manifold_report(&body, &scope).unwrap();
    assert_eq!((report.verdict, report.boundary_edges, report.closed), (Watertightness::Open, 4, false));
    let mass = mass_properties(&body, &scope, MASS_TOLERANCE).unwrap();
    assert!(mass.volumetric.is_none());
    close(mass.area, 5.0, 1e-9, "open shell area");
    assert_eq!(topology_counts(&body, &scope).unwrap().genus, None);
    let whole = mass_properties(&body, &ShapeScope::shell(body.solids.get(solid).unwrap().outer), MASS_TOLERANCE).unwrap();
    close(whole.volumetric.unwrap().volume, 1.0, 1e-9, "closed shell volume");
}

#[test]
fn one_misoriented_face_is_named_and_does_not_spoil_other_shapes() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let broken = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let sound = make_box(&mut body, 2.0, 2.0, 2.0, &mut rec).unwrap();
    let face = body.solid_faces(broken)[0];
    body.faces.get_mut(face).unwrap().flipped ^= true;
    let report = manifold_report(&body, &ShapeScope::solid(broken)).unwrap();
    assert_eq!((report.verdict, report.misoriented_edges), (Watertightness::Misoriented, 4));
    assert!(!validity(&body, &ShapeScope::solid(broken)).unwrap().ok);
    assert!(validity(&body, &ShapeScope::solid(sound)).unwrap().ok, "the sound box is judged alone");
    assert!(!validity(&body, &ShapeScope::compound(&[broken, sound])).unwrap().ok);
    assert_eq!(manifold_report(&body, &ShapeScope::solid(sound)).unwrap().verdict, Watertightness::Watertight);
}

#[test]
fn a_compound_combines_its_solids_with_the_parallel_axis_theorem() {
    let mut body = Body::new();
    let first = placed_box(&mut body, &serde_json::json!([1.0, 1.0, 1.0]), &serde_json::json!([0.0, 0.0, 0.0]));
    let second = placed_box(&mut body, &serde_json::json!([1.0, 1.0, 1.0]), &serde_json::json!([1.0, 0.0, 0.0]));
    let mass = mass_properties(&body, &ShapeScope::compound(&[first, second]), MASS_TOLERANCE).unwrap();
    let volumetric = mass.volumetric.unwrap();
    close(volumetric.volume, 2.0, 1e-12, "volume");
    close(mass.area, 12.0, 1e-12, "area counts every face of both boxes");
    assert!((volumetric.centroid[0] - 1.0).abs() < 1e-12 && (volumetric.centroid[1] - 0.5).abs() < 1e-12);
    let (w, d, h, m) = (2.0, 1.0, 1.0, 2.0);
    for (axis, wanted) in [m * (d * d + h * h) / 12.0, m * (w * w + h * h) / 12.0, m * (w * w + d * d) / 12.0].into_iter().enumerate() {
        assert!((volumetric.inertia[axis][axis] - wanted).abs() < 1e-9, "I[{axis}][{axis}] {}", volumetric.inertia[axis][axis]);
    }
    let topology = topology_counts(&body, &ShapeScope::compound(&[first, second])).unwrap();
    assert_eq!((topology.solids, topology.faces, topology.euler_characteristic, topology.genus), (2, 12, 4, Some(0)));
}

#[test]
fn a_single_edge_and_vertex_scope_report_what_they_can() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 2.0, 3.0, 4.0, &mut rec).unwrap();
    let edge = body.edges.ids().next().unwrap();
    let scope = ShapeScope::edge(edge);
    let bounds = bounding_box(&body, &scope).unwrap();
    assert!(bounds.min.iter().zip(&bounds.max).any(|(lo, hi)| hi > lo));
    assert!(mass_properties(&body, &scope, MASS_TOLERANCE).is_err());
    let counts = topology_counts(&body, &scope).unwrap();
    assert_eq!((counts.edges, counts.vertices, counts.faces, counts.genus), (1, 2, 0, None));
    assert_eq!(manifold_report(&body, &scope).unwrap().verdict, Watertightness::Open);
    let vertex = body.vertices.ids().next().unwrap();
    let nearest = point_distance(&body, &ShapeScope::vertex(vertex), Pnt3::new(10.0, 10.0, 10.0)).unwrap();
    assert!(nearest.signed.is_none() && nearest.distance > 0.0);
    let analysis = analyze_shape(&body, &ShapeScope::solid(solid), MASS_TOLERANCE).unwrap();
    assert!(analysis.unavailable.is_empty(), "{:?}", analysis.unavailable);
    assert_eq!((analysis.faces.len(), analysis.edges.len(), analysis.kind), (6, 12, ShapeKind::Solid));
    assert!(analysis.mass.is_some() && analysis.bounds.is_some() && analysis.validity.ok);
}

#[test]
fn analysis_values_round_trip_through_the_value_codec() {
    use protocol::value::{FromValue, ToValue};
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_cylinder(&mut body, 1.0, 3.0, &mut rec).unwrap();
    let analysis = analyze_shape(&body, &ShapeScope::solid(solid), MASS_TOLERANCE).unwrap();
    let back = ShapeAnalysis::from_value(analysis.to_value()).unwrap();
    assert_eq!(back, analysis);
}
