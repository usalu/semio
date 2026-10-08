use kurbo::{Arc as KArc, BezPath, ParamCurveArclen, ParamCurveExtrema, ParamCurveMoments, ParamCurveNearest, PathSeg, Shape};
use parry3d::math::Point as PPoint;
use parry3d::shape::{Triangle, TriMesh};
use semio_framework_geometry::bulge::{band_loop, intersect, BulgeSeg, Extent};
use semio_framework_geometry::loops::{self, Vertex};
use semio_framework_geometry::mesh::{extrude, extrude_between_faces, extrude_loops, prism_between, sweep_profile, ElevationFace, TriMesh as OurMesh};
use semio_framework_geometry::placement::{Affine3, ZPlane};
use semio_framework_geometry::section::{chain, section_z};
use semio_framework_geometry::triangulation::triangulate;
use semio_framework_geometry::Point;
use serde_json::Value;

struct Lcg(u64);

impl Lcg {
    fn next(&mut self, low: f64, high: f64) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        low + (self.0 >> 11) as f64 / (1_u64 << 53) as f64 * (high - low)
    }
}

fn fixture(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

fn pt(v: &Value) -> Point {
    Point::new(v[0].as_f64().unwrap(), v[1].as_f64().unwrap())
}

fn seg(v: &Value) -> BulgeSeg {
    BulgeSeg::new(pt(&v["start"]), pt(&v["end"]), v["bulge"].as_f64().unwrap())
}

fn kp(p: Point) -> kurbo::Point {
    kurbo::Point::new(p.x, p.y)
}

fn arc_path(s: &BulgeSeg) -> Vec<PathSeg> {
    let mut path = BezPath::new();
    path.move_to(kp(s.start));
    match (s.center(), s.start_angle()) {
        (Some(c), Some(a0)) => {
            let arc = KArc::new(kp(c), (s.radius(), s.radius()), a0, s.sweep(), 0.0);
            for el in arc.append_iter(1e-10) {
                path.push(el);
            }
        }
        _ => path.line_to(kp(s.end)),
    }
    path.segments().collect()
}

fn loop_path(vertices: &[Vertex]) -> BezPath {
    let mut path = BezPath::new();
    path.move_to(kp(vertices[0].point));
    for s in loops::segments(vertices) {
        match (s.center(), s.start_angle()) {
            (Some(c), Some(a0)) => {
                for el in KArc::new(kp(c), (s.radius(), s.radius()), a0, s.sweep(), 0.0).append_iter(1e-10) {
                    path.push(el);
                }
            }
            _ => path.line_to(kp(s.end)),
        }
    }
    path.close_path();
    path
}

fn nearest(segments: &[PathSeg], p: Point) -> f64 {
    segments.iter().map(|s| s.nearest(kp(p), 1e-12).distance_sq).fold(f64::INFINITY, f64::min).sqrt()
}

fn random_seg(rng: &mut Lcg) -> BulgeSeg {
    let start = Point::new(rng.next(-10.0, 10.0), rng.next(-10.0, 10.0));
    let end = Point::new(start.x + rng.next(-6.0, 6.0), start.y + rng.next(-6.0, 6.0));
    let bulge = if rng.next(0.0, 1.0) < 0.2 { 0.0 } else { rng.next(-2.0, 2.0) };
    BulgeSeg::new(start, end, bulge)
}

#[test]
fn arc_measures_match_kurbo() {
    let mut rng = Lcg(0x1234_5678_9ABC_DEF0);
    let mut cases: Vec<BulgeSeg> = fixture(include_str!("../../🧫️fixtures/🌙️bulge/🔣️.json"))["arcs"].as_array().unwrap().iter().map(seg).collect();
    cases.extend((0..300).map(|_| random_seg(&mut rng)));
    for (index, s) in cases.iter().enumerate() {
        if s.chord() < 0.1 {
            continue;
        }
        let path = arc_path(s);
        let length: f64 = path.iter().map(|p| p.arclen(1e-10)).sum();
        assert!((s.length() - length).abs() <= 1e-7 * length.max(1.0), "case {index} length {} vs {length}", s.length());
        let bounds = path.iter().map(ParamCurveExtrema::bounding_box).reduce(|a, b| a.union(b)).unwrap();
        let ours = s.bounds();
        for (a, b) in [(ours.x0(), bounds.x0), (ours.y0(), bounds.y0), (ours.x1(), bounds.x1), (ours.y1(), bounds.y1)] {
            assert!((a - b).abs() <= 1e-7, "case {index} bounds {a} vs {b}");
        }
        for k in 0..=8 {
            let p = s.point_at(f64::from(k) / 8.0);
            assert!(nearest(&path, p) <= 1e-7, "case {index}: point_at({k}/8) is off the kurbo arc");
        }
        for _ in 0..5 {
            let q = Point::new(rng.next(-14.0, 14.0), rng.next(-14.0, 14.0));
            let (ours, theirs) = (s.closest(q).distance, nearest(&path, q));
            assert!((ours - theirs).abs() <= 1e-6, "case {index} closest {ours} vs {theirs}");
        }
        let mut closed = BezPath::from_path_segments(path.iter().copied());
        closed.close_path();
        let kurbo_area = closed.area();
        assert!((kurbo_area.abs() - s.segment_area().abs()).abs() <= 1e-7 * kurbo_area.abs().max(1.0), "case {index} segment area {} vs {kurbo_area}", s.segment_area());
    }
}

#[test]
fn offsets_keep_a_constant_distance_to_the_kurbo_curve() {
    let mut rng = Lcg(0xDEAD_BEEF_0000_0001);
    for index in 0..300 {
        let s = random_seg(&mut rng);
        if s.chord() < 0.5 {
            continue;
        }
        let distance = rng.next(-0.4, 0.4);
        let Some(o) = s.offset(distance) else { continue };
        let path = arc_path(&s);
        for k in 0..=6 {
            let p = o.point_at(f64::from(k) / 6.0);
            let d = nearest(&path, p);
            assert!((d - distance.abs()).abs() <= 1e-6, "case {index} offset {distance}: distance to curve {d}");
        }
    }
}

#[test]
fn intersections_lie_on_both_kurbo_curves_and_agree_with_kurbo_line_hits() {
    let mut rng = Lcg(0x0BAD_CAFE_1234_0001);
    let mut checked = 0;
    for _ in 0..3000 {
        let (a, b) = (random_seg(&mut rng), random_seg(&mut rng));
        if a.chord() < 0.5 || b.chord() < 0.5 {
            continue;
        }
        let hits = intersect(&a, &b, Extent::Bounded);
        let (pa, pb) = (arc_path(&a), arc_path(&b));
        for hit in &hits {
            assert!(nearest(&pa, hit.point) <= 1e-6 && nearest(&pb, hit.point) <= 1e-6, "{hit:?} is not on both curves");
            checked += 1;
        }
        if a.is_line() {
            let line = kurbo::Line::new(kp(a.start), kp(a.end));
            let theirs: usize = pb.iter().map(|seg| seg.intersect_line(line).len()).sum();
            let near_end = hits.iter().any(|h| h.tb < 1e-6 || h.tb > 1.0 - 1e-6 || h.ta < 1e-6 || h.ta > 1.0 - 1e-6);
            if !near_end {
                assert_eq!(hits.len(), theirs, "line {a:?} vs {b:?}");
            }
        }
    }
    assert!(checked > 40, "enough intersections exercised: {checked}");
}

fn random_loop(rng: &mut Lcg) -> Vec<Vertex> {
    let n = 3 + rng.next(0.0, 6.0) as usize;
    let radius = rng.next(2.0, 8.0);
    (0..n)
        .map(|k| {
            let angle = (k as f64 + rng.next(-0.2, 0.2)) / n as f64 * std::f64::consts::TAU;
            Vertex::new(Point::new(radius * angle.cos(), radius * angle.sin()), if rng.next(0.0, 1.0) < 0.5 { 0.0 } else { rng.next(-0.25, 0.25) })
        })
        .collect()
}

#[test]
fn loop_area_centroid_perimeter_bounds_and_winding_match_kurbo() {
    let mut rng = Lcg(0xFEED_FACE_0000_0007);
    let mut cases: Vec<Vec<Vertex>> = fixture(include_str!("../../🧫️fixtures/➰️loops/🔣️.json"))["loops"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["vertices"].as_array().unwrap().iter().map(|v| Vertex::new(Point::new(v[0].as_f64().unwrap(), v[1].as_f64().unwrap()), v[2].as_f64().unwrap())).collect())
        .collect();
    cases.extend((0..200).map(|_| random_loop(&mut rng)));
    let mut probes = 0;
    for (index, v) in cases.iter().enumerate() {
        let path = loop_path(v);
        let area = loops::signed_area(v);
        assert!((area.abs() - path.area().abs()).abs() <= 1e-8 * area.abs().max(1.0), "case {index} area {area} vs {}", path.area());
        let perimeter = path.perimeter(1e-10);
        assert!((loops::perimeter(v) - perimeter).abs() <= 1e-7 * perimeter, "case {index} perimeter");
        let (ours, theirs) = (loops::bounds(v).unwrap(), path.bounding_box());
        for (a, b) in [(ours.x0(), theirs.x0), (ours.y0(), theirs.y0), (ours.x1(), theirs.x1), (ours.y1(), theirs.y1)] {
            assert!((a - b).abs() <= 1e-7, "case {index} bounds {a} vs {b}");
        }
        let moments = path.moments();
        let c = loops::centroid(v);
        let sign = path.area().signum();
        let (cx, cy) = (moments.moment_x * sign / path.area().abs(), moments.moment_y * sign / path.area().abs());
        let swapped = (c.x - cy).abs() < 1e-6 && (c.y - cx).abs() < 1e-6;
        let direct = (c.x - cx).abs() < 1e-6 && (c.y - cy).abs() < 1e-6;
        assert!(direct || swapped, "case {index} centroid {c:?} vs ({cx}, {cy})");
        let boundary: Vec<PathSeg> = path.segments().collect();
        for _ in 0..20 {
            let q = Point::new(rng.next(-9.0, 9.0), rng.next(-9.0, 9.0));
            if nearest(&boundary, q) < 1e-3 {
                continue;
            }
            let theirs = path.winding(kp(q));
            assert_eq!(loops::winding(v, q).abs(), theirs.abs(), "case {index} winding at {q:?}");
            assert_eq!(loops::contains(v, q), theirs != 0, "case {index} contains at {q:?}");
            probes += 1;
        }
    }
    assert!(probes > 1000, "enough probes: {probes}");
}

fn parry_mesh(m: &OurMesh) -> (Vec<PPoint<f32>>, Vec<[u32; 3]>) {
    (m.positions.iter().map(|p| PPoint::new(p[0] as f32, p[1] as f32, p[2] as f32)).collect(), m.indices.clone())
}

fn agree_with_parry(name: &str, m: &OurMesh) {
    let (vertices, indices) = parry_mesh(m);
    let (volume, com) = parry3d::mass_properties::details::trimesh_signed_volume_and_center_of_mass(&vertices, &indices);
    let area: f32 = indices.iter().map(|t| Triangle::new(vertices[t[0] as usize], vertices[t[1] as usize], vertices[t[2] as usize]).area()).sum();
    let ours_volume = m.signed_volume();
    let tolerance = 2e-5f64.max(2e-8 * indices.len() as f64);
    assert!((f64::from(volume) - ours_volume).abs() <= tolerance * ours_volume.abs().max(1.0), "{name} volume {ours_volume} vs {volume}");
    assert!((f64::from(area) - m.surface_area()).abs() <= tolerance * m.surface_area().max(1.0), "{name} area {} vs {area}", m.surface_area());
    let c = m.volume_centroid().unwrap();
    for (axis, theirs) in [com.x, com.y, com.z].into_iter().enumerate() {
        assert!((f64::from(theirs) - c[axis]).abs() <= tolerance * c[axis].abs().max(1.0), "{name} centroid axis {axis}: {} vs {theirs}", c[axis]);
    }
    let aabb = parry3d::bounding_volume::details::point_cloud_aabb(&parry3d::math::Isometry::identity(), &vertices);
    let (lo, hi) = m.bounds().unwrap();
    for axis in 0..3 {
        assert!((f64::from(aabb.mins[axis]) - lo[axis]).abs() <= 1e-5 && (f64::from(aabb.maxs[axis]) - hi[axis]).abs() <= 1e-5, "{name} aabb axis {axis}");
    }
}

fn ring(v: &Value) -> Vec<Point> {
    v.as_array().unwrap().iter().map(pt).collect()
}

fn rings(v: &Value) -> Vec<Vec<Point>> {
    v.as_array().map(|a| a.iter().map(ring).collect()).unwrap_or_default()
}

fn plane(v: &Value) -> ZPlane {
    ZPlane { a: v[0].as_f64().unwrap(), b: v[1].as_f64().unwrap(), c: v[2].as_f64().unwrap() }
}

#[test]
fn solids_agree_with_parry3d_mass_properties_area_and_bounds() {
    let doc = fixture(include_str!("../../🧫️fixtures/🕸️mesh/🔣️.json"));
    for case in doc["extrusions"].as_array().unwrap() {
        let m = extrude(&ring(&case["outer"]), &rings(&case["holes"]), plane(&case["bottom"]), plane(&case["top"]));
        agree_with_parry(case["name"].as_str().unwrap(), &m);
    }
    for case in doc["loop_extrusions"].as_array().unwrap() {
        let loop_of = |v: &Value| -> Vec<Vertex> { v.as_array().unwrap().iter().map(|x| Vertex::new(pt(x), x[2].as_f64().unwrap())).collect() };
        let holes: Vec<Vec<Vertex>> = case["holes"].as_array().unwrap().iter().map(loop_of).collect();
        agree_with_parry(case["name"].as_str().unwrap(), &extrude_loops(&loop_of(&case["outer"]), &holes, case["tolerance"].as_f64().unwrap(), plane(&case["bottom"]), plane(&case["top"])));
    }
    for case in doc["prisms"].as_array().unwrap() {
        let ring3 = |v: &Value| -> Vec<[f64; 3]> { v.as_array().unwrap().iter().map(|p| [p[0].as_f64().unwrap(), p[1].as_f64().unwrap(), p[2].as_f64().unwrap()]).collect() };
        agree_with_parry(case["name"].as_str().unwrap(), &prism_between(&ring3(&case["lower"]), &ring3(&case["upper"])));
    }
    for case in doc["walls"].as_array().unwrap() {
        let axis = seg(&case["axis"]);
        let face = |v: &Value| ElevationFace { outer: ring(&v["outer"]), holes: rings(&v["holes"]) };
        let (left, right) = (axis.offset(case["left"].as_f64().unwrap()).unwrap(), axis.offset(-case["right"].as_f64().unwrap()).unwrap());
        agree_with_parry(case["name"].as_str().unwrap(), &extrude_between_faces(&face(&case["left_face"]), &face(&case["right_face"]), &left, &right, case["axis_length"].as_f64().unwrap(), case["base_z"].as_f64().unwrap(), case["tolerance"].as_f64().unwrap()));
    }
    for case in doc["sweeps"].as_array().unwrap() {
        let path: Vec<BulgeSeg> = case["path"].as_array().unwrap().iter().map(seg).collect();
        agree_with_parry(case["name"].as_str().unwrap(), &sweep_profile(&ring(&case["profile"]), &rings(&case["holes"]), &path, case["base_z"].as_f64().unwrap(), case["tolerance"].as_f64().unwrap()));
    }
}

#[test]
fn random_transformed_solids_agree_with_parry3d() {
    let mut rng = Lcg(0xABCD_EF01_2345_6789);
    for case in 0..60 {
        let n = 4 + rng.next(0.0, 8.0) as usize;
        let outer: Vec<Point> = (0..n)
            .map(|k| {
                let a = k as f64 / n as f64 * std::f64::consts::TAU;
                let r = rng.next(1.0, 3.0);
                Point::new(r * a.cos(), r * a.sin())
            })
            .collect();
        let hole: Vec<Point> = (0..5).map(|k| Point::new(0.3 * (k as f64 * 1.2566).cos(), 0.3 * (k as f64 * 1.2566).sin())).collect();
        let base = extrude(&outer, &[hole], ZPlane::flat(rng.next(-1.0, 1.0)), ZPlane::sloped(Point::ZERO, 3.0, rng.next(0.0, 6.0), rng.next(0.0, 0.3)));
        let m = base.transformed(&Affine3::rotation_axis([rng.next(-1.0, 1.0), rng.next(-1.0, 1.0), rng.next(0.2, 1.0)], rng.next(0.0, 6.0)).then(&Affine3::translation([rng.next(-5.0, 5.0), rng.next(-5.0, 5.0), rng.next(-5.0, 5.0)])));
        agree_with_parry(&format!("random solid {case}"), &m);
    }
}

#[test]
fn plan_sections_agree_with_parry3d_plane_intersections() {
    let doc = fixture(include_str!("../../🧫️fixtures/🔪️section/🔣️.json"));
    for case in doc.as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let m = extrude(&ring(&case["outer"]), &rings(&case["holes"]), ZPlane::flat(case["bottom"].as_f64().unwrap()), ZPlane::flat(case["top"].as_f64().unwrap()));
        let z = case["z"].as_f64().unwrap();
        let ours: f64 = section_z(&m, z, 1e-9).iter().map(|s| (s[1] - s[0]).hypot()).sum();
        let on_face = z == case["top"].as_f64().unwrap() || z == case["bottom"].as_f64().unwrap();
        if !on_face {
            let (vertices, indices) = parry_mesh(&m.welded());
            let mesh = TriMesh::new(vertices, indices);
            let theirs = match mesh.canonical_intersection_with_plane(2, z as f32, 1e-6) {
                parry3d::query::IntersectResult::Intersect(poly) => poly.indices().iter().map(|i| (poly.vertices()[i[1] as usize] - poly.vertices()[i[0] as usize]).norm()).sum::<f32>() as f64,
                _ => 0.0,
            };
            assert!((ours - theirs).abs() <= 1e-4 * ours.max(1.0), "{name}: cut length {ours} vs {theirs}");
        }
        let chains = chain(&section_z(&m, z, 1e-9), 1e-9);
        assert!(chains.iter().all(|c| c.closed), "{name}: closed meshes yield closed cuts");
    }
}

#[test]
fn triangulated_area_matches_kurbo_polygon_area() {
    let doc = fixture(include_str!("../../🧫️fixtures/🔺️triangulation/🔣️.json"));
    for case in doc.as_array().unwrap() {
        let (outer, holes) = (ring(&case["outer"]), rings(&case["holes"]));
        let poly = |points: &[Point]| {
            let mut path = BezPath::new();
            path.move_to(kp(points[0]));
            for p in &points[1..] {
                path.line_to(kp(*p));
            }
            path.close_path();
            path.area().abs()
        };
        let expected = poly(&outer) - holes.iter().map(|h| poly(h)).sum::<f64>();
        let got = triangulate(&outer, &holes).area();
        assert!((got - expected).abs() <= 1e-9 * expected.max(1.0), "{}: {got} vs {expected}", case["name"]);
    }
}

#[test]
fn band_footprints_match_kurbo_area_and_distance() {
    let mut rng = Lcg(0x5555_AAAA_1111_2222);
    for case in 0..100 {
        let axis = random_seg(&mut rng);
        if axis.chord() < 1.0 {
            continue;
        }
        let (left, right) = (rng.next(0.05, 0.3), rng.next(0.05, 0.3));
        let Some(band) = band_loop(&axis, left, right, None, None) else { continue };
        let vertices: Vec<Vertex> = band.iter().map(|(p, b)| Vertex::new(*p, *b)).collect();
        let path = loop_path(&vertices);
        assert!(path.area() != 0.0, "case {case}");
        let kurbo_area = path.area().abs();
        let ours = loops::area(&vertices);
        assert!((ours - kurbo_area).abs() <= 1e-8 * kurbo_area.max(1.0), "case {case}: {ours} vs {kurbo_area}");
        assert!(loops::is_ccw(&vertices), "case {case}: band footprints are counter-clockwise");
        let centre = arc_path(&axis);
        for (p, _) in &band[2..] {
            let d = nearest(&centre, *p);
            assert!((d - left).abs() <= 1e-6, "case {case}: left corner {d}");
        }
    }
}
