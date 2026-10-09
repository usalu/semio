use super::*;
use serde_json::Value;

fn fixtures() -> Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/🕸️mesh/🔣️.json")).unwrap()
}

fn pt(v: &Value) -> Point {
    Point::new(v[0].as_f64().unwrap(), v[1].as_f64().unwrap())
}

fn ring(v: &Value) -> Vec<Point> {
    v.as_array().unwrap().iter().map(pt).collect()
}

fn rings(v: &Value) -> Vec<Vec<Point>> {
    v.as_array().map(|a| a.iter().map(ring).collect()).unwrap_or_default()
}

fn xyz(v: &Value) -> Xyz {
    [v[0].as_f64().unwrap(), v[1].as_f64().unwrap(), v[2].as_f64().unwrap()]
}

fn plane(v: &Value) -> ZPlane {
    ZPlane { a: v[0].as_f64().unwrap(), b: v[1].as_f64().unwrap(), c: v[2].as_f64().unwrap() }
}

fn loop_of(v: &Value) -> Vec<Vertex> {
    v.as_array().unwrap().iter().map(|x| Vertex::new(pt(x), x[2].as_f64().unwrap())).collect()
}

fn close(a: f64, b: f64, eps: f64, context: &str) {
    assert!((a - b).abs() <= eps * b.abs().max(1.0), "{context}: {a} vs {b}");
}

fn consistent(mesh: &TriMesh, name: &str) {
    assert!(mesh.is_watertight(), "{name}: not watertight");
    assert!(mesh.signed_volume() > 0.0, "{name}: inward winding");
    assert_eq!(mesh.normals.len(), mesh.positions.len(), "{name}: normals");
    for (i, t) in mesh.indices.iter().enumerate() {
        let [a, b, c] = mesh.triangle(i);
        let face = normalize3(cross3(sub3(b, a), sub3(c, a)));
        let n = mesh.normals[t[0] as usize];
        assert!(dot3(face, n) > 0.0, "{name}: normal opposes the winding of triangle {i}");
    }
}

#[test]
fn extrusions_match_closed_form_fixtures() {
    for case in fixtures()["extrusions"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mesh = extrude(&ring(&case["outer"]), &rings(&case["holes"]), plane(&case["bottom"]), plane(&case["top"]));
        let e = &case["expected"];
        close(mesh.volume(), e["volume"].as_f64().unwrap(), 1e-12, name);
        close(mesh.surface_area(), e["area"].as_f64().unwrap(), 1e-12, name);
        let (lo, hi) = mesh.bounds().unwrap();
        for (got, want) in lo.into_iter().chain(hi).zip(e["bounds"][0].as_array().unwrap().iter().chain(e["bounds"][1].as_array().unwrap())) {
            close(got, want.as_f64().unwrap(), 1e-12, name);
        }
        let c = mesh.volume_centroid().unwrap();
        for (got, want) in c.into_iter().zip(e["centroid"].as_array().unwrap()) {
            close(got, want.as_f64().unwrap(), 1e-12, name);
        }
        consistent(&mesh, name);
    }
}

#[test]
fn loop_extrusions_stay_within_the_chord_tolerance() {
    for case in fixtures()["loop_extrusions"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let holes: Vec<Vec<Vertex>> = case["holes"].as_array().unwrap().iter().map(loop_of).collect();
        let mesh = extrude_loops(&loop_of(&case["outer"]), &holes, case["tolerance"].as_f64().unwrap(), plane(&case["bottom"]), plane(&case["top"]));
        let v = case["volume"].as_array().unwrap();
        assert!(mesh.volume() >= v[0].as_f64().unwrap() && mesh.volume() <= v[1].as_f64().unwrap(), "{name}: volume {}", mesh.volume());
        let a = case["area"].as_array().unwrap();
        assert!(mesh.surface_area() >= a[0].as_f64().unwrap() && mesh.surface_area() <= a[1].as_f64().unwrap(), "{name}: area {}", mesh.surface_area());
        consistent(&mesh, name);
        let side = (0..mesh.indices.len()).map(|i| mesh.normals[mesh.indices[i][0] as usize]).filter(|n| n[2].abs() < 1e-9).count();
        assert!(side > 0, "{name}: curved wall normals present");
        let distinct: std::collections::BTreeSet<[i64; 2]> = mesh.normals.iter().filter(|n| n[2].abs() < 1e-9).map(|n| [(n[0] * 1e6).round() as i64, (n[1] * 1e6).round() as i64]).collect();
        assert!(distinct.len() > 8, "{name}: smooth wall normals vary around the circle");
    }
}

#[test]
fn prisms_match_closed_form_fixtures() {
    for case in fixtures()["prisms"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let lower: Vec<Xyz> = case["lower"].as_array().unwrap().iter().map(xyz).collect();
        let upper: Vec<Xyz> = case["upper"].as_array().unwrap().iter().map(xyz).collect();
        let mesh = prism_between(&lower, &upper);
        close(mesh.volume(), case["volume"].as_f64().unwrap(), 1e-12, name);
        close(mesh.surface_area(), case["area"].as_f64().unwrap(), 1e-12, name);
        consistent(&mesh, name);
    }
}

#[test]
fn sweeps_match_closed_form_fixtures() {
    for case in fixtures()["sweeps"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let path: Vec<BulgeSeg> = case["path"].as_array().unwrap().iter().map(|s| BulgeSeg::new(pt(&s["start"]), pt(&s["end"]), s["bulge"].as_f64().unwrap())).collect();
        let mesh = sweep_profile(&ring(&case["profile"]), &rings(&case["holes"]), &path, case["base_z"].as_f64().unwrap(), case["tolerance"].as_f64().unwrap());
        let v = case["volume"].as_array().unwrap();
        assert!(mesh.volume() >= v[0].as_f64().unwrap() - 1e-12 && mesh.volume() <= v[1].as_f64().unwrap() + 1e-12, "{name}: volume {} not in {v:?}", mesh.volume());
        if let Some(a) = case["area"].as_array() {
            close(mesh.surface_area(), a[0].as_f64().unwrap(), 1e-12, name);
        }
        let (lo, hi) = mesh.bounds().unwrap();
        for (got, want) in lo.into_iter().chain(hi).zip(case["bounds"][0].as_array().unwrap().iter().chain(case["bounds"][1].as_array().unwrap())) {
            assert!((got - want.as_f64().unwrap()).abs() < 1e-3, "{name}: bounds {got} vs {want}");
        }
        consistent(&mesh, name);
    }
}

fn wall_of(case: &Value) -> TriMesh {
    let axis = BulgeSeg::new(pt(&case["axis"]["start"]), pt(&case["axis"]["end"]), case["axis"]["bulge"].as_f64().unwrap());
    let face = |v: &Value| ElevationFace { outer: ring(&v["outer"]), holes: rings(&v["holes"]) };
    let (left, right) = (axis.offset(case["left"].as_f64().unwrap()).unwrap(), axis.offset(-case["right"].as_f64().unwrap()).unwrap());
    extrude_between_faces(&face(&case["left_face"]), &face(&case["right_face"]), &left, &right, case["axis_length"].as_f64().unwrap(), case["base_z"].as_f64().unwrap(), case["tolerance"].as_f64().unwrap())
}

#[test]
fn walls_with_openings_match_closed_form_fixtures() {
    for case in fixtures()["walls"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mesh = wall_of(case);
        let v = case["volume"].as_array().unwrap();
        assert!(mesh.volume() >= v[0].as_f64().unwrap() - 1e-9 && mesh.volume() <= v[1].as_f64().unwrap() + 1e-9, "{name}: volume {} not in {v:?}", mesh.volume());
        if let Some(a) = case["area"].as_array() {
            close(mesh.surface_area(), a[0].as_f64().unwrap(), 1e-9, name);
        }
        consistent(&mesh, name);
    }
}

#[test]
fn wall_faces_and_reveals_face_outwards() {
    let case = &fixtures()["walls"][0];
    let mesh = wall_of(case);
    let mut left = 0;
    let mut right = 0;
    for (i, t) in mesh.indices.iter().enumerate() {
        let [a, ..] = mesh.triangle(i);
        let n = mesh.normals[t[0] as usize];
        if n[1] > 0.99 && (a[1] - 0.1).abs() < 1e-9 {
            left += 1;
        }
        if n[1] < -0.99 && (a[1] + 0.1).abs() < 1e-9 {
            right += 1;
        }
    }
    assert!(left > 0 && left == right, "left {left} right {right}");
    let reveal = (0..mesh.indices.len()).any(|i| {
        let n = mesh.normals[mesh.indices[i][0] as usize];
        let [a, ..] = mesh.triangle(i);
        n[2] > 0.99 && (a[2] - 2.0).abs() < 1e-9 && a[0] > 3.0 && a[0] < 4.2
    });
    assert!(!reveal, "the window head faces down into the opening");
}

#[test]
fn transform_append_and_mirror_keep_the_measures() {
    let cube = extrude(&[Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(0.0, 1.0)], &[], ZPlane::flat(0.0), ZPlane::flat(1.0));
    let moved = cube.transformed(&Affine3::rotation_axis([1.0, 2.0, 3.0], 0.7).then(&Affine3::translation([5.0, -1.0, 2.0])));
    close(moved.volume(), 1.0, 1e-12, "rigid volume");
    close(moved.surface_area(), 6.0, 1e-12, "rigid area");
    consistent(&moved, "rigid");
    let mirrored = cube.transformed(&Affine3::scaling([-1.0, 1.0, 1.0]));
    consistent(&mirrored, "mirror");
    let scaled = cube.transformed(&Affine3::scaling([2.0, 3.0, 4.0]));
    close(scaled.volume(), 24.0, 1e-12, "anisotropic volume");
    let mut both = cube.clone();
    both.append(&cube.translated([3.0, 0.0, 0.0]));
    close(both.volume(), 2.0, 1e-12, "merged volume");
    assert_eq!(both.triangle_count(), 2 * cube.triangle_count());
    assert!(both.is_watertight());
}

#[test]
fn empty_and_degenerate_inputs_are_harmless() {
    let empty = TriMesh::new();
    assert!(empty.bounds().is_none() && empty.volume_centroid().is_none() && !empty.is_watertight());
    assert_eq!(empty.volume(), 0.0);
    assert_eq!(extrude(&[Point::ZERO, Point::new(1.0, 0.0)], &[], ZPlane::flat(0.0), ZPlane::flat(1.0)).triangle_count(), 0);
    assert_eq!(prism_between(&[[0.0; 3]; 2], &[[0.0; 3]; 2]).triangle_count(), 0);
    assert_eq!(sweep_profile(&[Point::ZERO; 4], &[], &[], 0.0, 0.01).triangle_count(), 0);
    let mut open = TriMesh::new();
    open.push_triangle([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    open.push_triangle([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
    assert_eq!(open.triangle_count(), 1);
    assert!(!open.is_watertight());
}

#[test]
fn welding_merges_shared_corners_and_keeps_the_measures() {
    let cube = extrude(&[Point::new(0.0, 0.0), Point::new(2.0, 0.0), Point::new(2.0, 2.0), Point::new(0.0, 2.0)], &[], ZPlane::flat(0.0), ZPlane::flat(1.0));
    let welded = cube.welded();
    assert_eq!(welded.vertex_count(), 8);
    assert_eq!(welded.triangle_count(), cube.triangle_count());
    close(welded.volume(), 4.0, 1e-12, "welded volume");
    close(welded.surface_area(), cube.surface_area(), 1e-12, "welded area");
    assert!(welded.is_watertight());
}

#[test]
fn render_buffers_are_flat_f32_views() {
    let mesh = extrude(&[Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(0.0, 1.0)], &[], ZPlane::flat(0.0), ZPlane::flat(1.0));
    assert_eq!(mesh.positions_f32().len(), mesh.vertex_count() * 3);
    assert_eq!(mesh.normals_f32().len(), mesh.vertex_count() * 3);
    assert_eq!(mesh.indices_flat().len(), mesh.triangle_count() * 3);
    assert!(mesh.indices_flat().iter().all(|&i| (i as usize) < mesh.vertex_count()));
}

#[test]
fn crease_normals_keep_hard_edges_and_smooth_curves() {
    let cube = extrude(&[Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(0.0, 1.0)], &[], ZPlane::flat(0.0), ZPlane::flat(1.0));
    let smoothed = cube.crease_normals(30f64.to_radians());
    for (a, b) in cube.normals.iter().zip(&smoothed.normals) {
        assert!(length3(sub3(*a, *b)) < 1e-12, "a cube has only 90 degree edges");
    }
    let loose = cube.crease_normals(100f64.to_radians());
    assert!(loose.normals.iter().any(|n| (n[0].abs() - n[2].abs()).abs() < 1e-9 && n[0] != 0.0 && n[2] != 0.0), "wide crease averages across edges");
}

fn rectangle_in_half_plane(inner: f64, outer: f64, height: f64) -> Vec<Point> {
    vec![Point::new(inner, 0.0), Point::new(outer, 0.0), Point::new(outer, height), Point::new(inner, height)]
}

#[test]
fn revolving_a_rectangle_about_the_axis_makes_a_cylinder() {
    let mesh = revolve_profile(&rectangle_in_half_plane(0.0, 1.0, 2.0), std::f64::consts::TAU, 1e-4);
    consistent(&mesh, "cylinder");
    close(mesh.volume(), std::f64::consts::PI * 2.0, 5e-4, "cylinder volume");
    close(mesh.surface_area(), 6.0 * std::f64::consts::PI, 1e-3, "cylinder surface");
}

#[test]
fn revolving_away_from_the_axis_makes_a_ring_and_either_orientation_works() {
    let ring = rectangle_in_half_plane(0.5, 1.0, 1.0);
    let mut reversed = ring.clone();
    reversed.reverse();
    for profile in [ring, reversed] {
        let mesh = revolve_profile(&profile, std::f64::consts::TAU, 1e-4);
        consistent(&mesh, "ring");
        close(mesh.volume(), std::f64::consts::PI * (1.0 - 0.25), 5e-4, "ring volume");
    }
}

#[test]
fn a_partial_sweep_is_capped_and_scales_with_the_angle() {
    for fraction in [0.25, 0.5, 0.75] {
        let mesh = revolve_profile(&rectangle_in_half_plane(0.25, 1.0, 1.5), std::f64::consts::TAU * fraction, 1e-4);
        consistent(&mesh, "wedge");
        close(mesh.volume(), std::f64::consts::PI * (1.0 - 0.0625) * 1.5 * fraction, 5e-4, "wedge volume");
    }
}

#[test]
fn an_invalid_revolution_is_empty() {
    assert!(revolve_profile(&rectangle_in_half_plane(-0.5, 1.0, 1.0), 1.0, 1e-3).indices.is_empty(), "crosses the axis");
    assert!(revolve_profile(&rectangle_in_half_plane(0.0, 1.0, 1.0), 0.0, 1e-3).indices.is_empty(), "no sweep");
    assert!(revolve_profile(&rectangle_in_half_plane(0.0, 1.0, 1.0), f64::NAN, 1e-3).indices.is_empty(), "not finite");
    assert!(revolve_profile(&[Point::new(1.0, 0.0), Point::new(2.0, 0.0)], 1.0, 1e-3).indices.is_empty(), "two points");
}

fn section(width: f64, depth: f64) -> Vec<Point> {
    vec![Point::new(-width / 2.0, -depth), Point::new(width / 2.0, -depth), Point::new(width / 2.0, 0.0), Point::new(-width / 2.0, 0.0)]
}

#[test]
fn a_ramped_sweep_with_no_rise_is_the_plain_sweep() {
    let path = [BulgeSeg::line(Point::new(0.0, 0.0), Point::new(4.0, 0.0))];
    let (plain, ramped) = (sweep_profile(&section(0.3, 0.5), &[], &path, 2.0, 1e-4), sweep_profile_ramped(&section(0.3, 0.5), &[], &path, 2.0, 0.0, 1e-4));
    assert_eq!(plain, ramped);
}

#[test]
fn a_ramped_straight_sweep_keeps_the_section_perpendicular_to_the_climbing_axis() {
    let (rise, run, width, depth) = (3.0_f64, 4.0_f64, 0.3, 0.5);
    let mesh = sweep_profile_ramped(&section(width, depth), &[], &[BulgeSeg::line(Point::new(0.0, 0.0), Point::new(run, 0.0))], 1.0, rise, 1e-4);
    close(mesh.volume(), width * depth * run.hypot(rise), 1e-9, "section area times the sloping length");
    let (lo, hi) = mesh.bounds().expect("a solid");
    close(hi[2], 1.0 + rise, 1e-9, "the top line ends at base + rise");
    close(lo[2], 1.0 - depth * run / run.hypot(rise), 1e-9, "the lowest start corner hangs perpendicular to the axis");
    assert!(mesh.is_watertight() || mesh.welded().is_watertight());
}

#[test]
fn a_ramped_arc_sweep_climbs_with_the_arc_length() {
    let arc = BulgeSeg::new(Point::new(0.0, 0.0), Point::new(4.0, 0.0), 0.4);
    let mesh = sweep_profile_ramped(&section(0.3, 0.5), &[], &[arc], 0.0, -1.2, 1e-4);
    let (lo, hi) = mesh.bounds().expect("a solid");
    close(hi[2], 0.0, 1e-6, "the top starts on the reference line");
    assert!(lo[2] < -1.2, "the descending end hangs below the reference line");
    let level = sweep_profile(&section(0.3, 0.5), &[], &[arc], 0.0, 1e-4);
    assert!(mesh.volume() > level.volume(), "the sloping length is longer than the plan length");
    close(mesh.volume() / level.volume(), (arc.length().powi(2) + 1.2_f64.powi(2)).sqrt() / arc.length(), 5e-2, "volume scales about with the 3D length (the hanging section twists with the torsion of the helix)");
}
