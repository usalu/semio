use super::*;

fn pts(list: &[[f64; 2]]) -> Vec<Point> {
    list.iter().map(|p| Point::new(p[0], p[1])).collect()
}

fn uniform(list: &[[f64; 2]]) -> Ring {
    Ring::uniform(&pts(list), 1.0)
}

fn weighted(list: &[[f64; 2]], speeds: &[f64]) -> Ring {
    Ring { points: pts(list), speeds: speeds.to_vec(), tags: (0..list.len() as u32).collect() }
}

fn area_of(points: &[V2]) -> f64 {
    signed_area(points)
}

fn plan(skeleton: &Skeleton, face: &SkeletonFace) -> Vec<V2> {
    face.nodes.iter().map(|&n| xy(skeleton.nodes[n as usize].point)).collect()
}

fn plane_of(skeleton: &Skeleton, face: &SkeletonFace) -> Option<[f64; 3]> {
    let nodes: Vec<&SkeletonNode> = face.nodes.iter().map(|&n| &skeleton.nodes[n as usize]).collect();
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            for k in j + 1..nodes.len() {
                let (a, b, c) = (xy(nodes[i].point), xy(nodes[j].point), xy(nodes[k].point));
                let det = cross2(sub(b, a), sub(c, a));
                if det.abs() > 1e-9 {
                    let (db, dc) = (nodes[j].time - nodes[i].time, nodes[k].time - nodes[i].time);
                    let (u, v) = (sub(b, a), sub(c, a));
                    let gx = (db * v[1] - dc * u[1]) / det;
                    let gy = (dc * u[0] - db * v[0]) / det;
                    return Some([gx, gy, nodes[i].time - gx * a[0] - gy * a[1]]);
                }
            }
        }
    }
    None
}

fn clip_below(polygon: &[V2], plane: [f64; 3], t: f64) -> Vec<V2> {
    let value = |p: V2| plane[0] * p[0] + plane[1] * p[1] + plane[2] - t;
    let mut out = Vec::new();
    for i in 0..polygon.len() {
        let (a, b) = (polygon[i], polygon[(i + 1) % polygon.len()]);
        let (fa, fb) = (value(a), value(b));
        if fa <= 0.0 {
            out.push(a);
        }
        if (fa < 0.0 && fb > 0.0) || (fa > 0.0 && fb < 0.0) {
            let s = fa / (fa - fb);
            out.push(add(a, scale(sub(b, a), s)));
        }
    }
    out
}

fn swept_area(skeleton: &Skeleton, t: f64) -> f64 {
    skeleton
        .faces
        .iter()
        .filter(|face| face.speed > 0.0)
        .map(|face| {
            let polygon = plan(skeleton, face);
            let plane = plane_of(skeleton, face).expect("a face has a plane");
            area_of(&clip_below(&polygon, plane, t))
        })
        .sum()
}

fn inside(polygon: &[V2], p: V2) -> bool {
    let mut winding = false;
    for i in 0..polygon.len() {
        let (a, b) = (polygon[i], polygon[(i + 1) % polygon.len()]);
        if (a[1] > p[1]) != (b[1] > p[1]) && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0] {
            winding = !winding;
        }
    }
    winding
}

fn height_at(skeleton: &Skeleton, p: V2) -> Option<f64> {
    skeleton.faces.iter().filter(|face| face.speed > 0.0).find(|face| inside(&plan(skeleton, face), p)).map(|face| {
        let plane = plane_of(skeleton, face).expect("plane");
        plane[0] * p[0] + plane[1] * p[1] + plane[2]
    })
}

fn region_area(rings: &[Ring]) -> f64 {
    rings.iter().enumerate().map(|(r, ring)| {
        let area = area_of(&ring.points.iter().map(|p| xy(*p)).collect::<Vec<_>>()).abs();
        if r == 0 { area } else { -area }
    }).sum()
}

fn check(name: &str, rings: &[Ring]) -> Skeleton {
    let skeleton = straight_skeleton(rings).unwrap_or_else(|error| panic!("{name}: {error}"));
    let total = region_area(rings);
    let swept: f64 = skeleton.faces.iter().map(|face| skeleton.face_area(face)).sum();
    assert!((swept - total).abs() <= 1e-7 * total.max(1.0), "{name}: faces cover {swept} of {total}");
    for face in skeleton.faces.iter().filter(|face| face.speed > 0.0) {
        assert!(skeleton.face_area(face) > 0.0, "{name}: face {} has no counter-clockwise area", face.edge);
        let ring = &rings[face.ring];
        let (a, b) = (xy(ring.points[face.edge]), xy(ring.points[(face.edge + 1) % ring.points.len()]));
        let d = unit(Vec2::new(b[0] - a[0], b[1] - a[1]), 1e-12).unwrap();
        let ring_area = area_of(&ring.points.iter().map(|p| xy(*p)).collect::<Vec<_>>());
        let inward = if (ring_area > 0.0) == (face.ring == 0) { 1.0 } else { -1.0 };
        let normal = [-d.y * inward, d.x * inward];
        for &n in &face.nodes {
            let node = &skeleton.nodes[n as usize];
            let distance = dot(normal, sub(xy(node.point), a));
            assert!((distance - face.speed * node.time).abs() <= 1e-7 * (1.0 + distance.abs()), "{name}: node of edge {} is off its plane ({distance} vs {})", face.edge, face.speed * node.time);
        }
    }
    skeleton
}

fn convex_height(rings: &[Ring], p: V2) -> f64 {
    let ring = &rings[0];
    let n = ring.points.len();
    (0..n).filter(|&i| ring.speeds[i] > 0.0).map(|i| {
        let (a, b) = (xy(ring.points[i]), xy(ring.points[(i + 1) % n]));
        let d = unit(Vec2::new(b[0] - a[0], b[1] - a[1]), 1e-12).unwrap();
        dot([-d.y, d.x], sub(p, a)) / ring.speeds[i]
    }).fold(f64::INFINITY, f64::min)
}

#[test]
fn a_rectangle_is_a_hip_with_a_horizontal_ridge() {
    let rings = [uniform(&[[0.0, 0.0], [10.0, 0.0], [10.0, 4.0], [0.0, 4.0]])];
    let skeleton = check("rectangle", &rings);
    assert_eq!((skeleton.nodes.len(), skeleton.arcs.len(), skeleton.faces.len()), (6, 5, 4));
    assert!((skeleton.peak() - 2.0).abs() < 1e-9);
    let areas: Vec<f64> = skeleton.faces.iter().map(|face| skeleton.face_area(face)).collect();
    for (got, want) in areas.iter().zip([16.0, 4.0, 16.0, 4.0]) {
        assert!((got - want).abs() < 1e-9, "{areas:?}");
    }
    let ridge = skeleton.arcs.iter().filter(|arc| (skeleton.nodes[arc.from as usize].time - 2.0).abs() < 1e-9 && (skeleton.nodes[arc.to as usize].time - 2.0).abs() < 1e-9).count();
    assert_eq!(ridge, 1);
}

#[test]
fn a_square_collapses_to_one_apex() {
    let rings = [uniform(&[[0.0, 0.0], [4.0, 0.0], [4.0, 4.0], [0.0, 4.0]])];
    let skeleton = check("square", &rings);
    assert_eq!((skeleton.nodes.len(), skeleton.arcs.len()), (5, 4));
    assert!((skeleton.peak() - 2.0).abs() < 1e-9);
}

#[test]
fn a_triangle_meets_at_the_incentre() {
    let rings = [uniform(&[[0.0, 0.0], [6.0, 0.0], [0.0, 8.0]])];
    let skeleton = check("triangle", &rings);
    assert_eq!(skeleton.arcs.len(), 3);
    assert!((skeleton.peak() - 2.0).abs() < 1e-9);
}

#[test]
fn clockwise_input_gives_the_same_faces_as_counter_clockwise() {
    let ccw = [uniform(&[[0.0, 0.0], [10.0, 0.0], [10.0, 4.0], [0.0, 4.0]])];
    let cw = [uniform(&[[0.0, 4.0], [10.0, 4.0], [10.0, 0.0], [0.0, 0.0]])];
    let (a, b) = (check("ccw", &ccw), check("cw", &cw));
    assert_eq!((a.nodes.len(), a.arcs.len()), (b.nodes.len(), b.arcs.len()));
    let area = |s: &Skeleton, edge: usize| s.face_area(s.faces.iter().find(|f| f.edge == edge).unwrap());
    assert!((area(&a, 0) - area(&b, 2)).abs() < 1e-9 && (area(&a, 1) - area(&b, 1)).abs() < 1e-9);
}

#[test]
fn a_gable_is_a_hip_whose_short_edges_do_not_move() {
    let speed = 1.0 / 30.0_f64.to_radians().tan();
    let rings = [weighted(&[[0.0, 0.0], [10.0, 0.0], [10.0, 4.0], [0.0, 4.0]], &[speed, 0.0, speed, 0.0])];
    let skeleton = check("gable", &rings);
    assert!((skeleton.peak() - 2.0 * 30.0_f64.to_radians().tan()).abs() < 1e-9);
    assert_eq!(skeleton.arcs.len(), 5);
    let wall = skeleton.faces.iter().find(|face| face.speed == 0.0 && face.edge == 1).expect("gable end");
    let heights: Vec<f64> = wall.nodes.iter().map(|&n| skeleton.nodes[n as usize].time).collect();
    assert_eq!(heights.len(), 3);
    assert!((heights.iter().cloned().fold(0.0, f64::max) - skeleton.peak()).abs() < 1e-9);
}

#[test]
fn unequal_speeds_tilt_the_planes_and_shift_the_ridge() {
    let rings = [weighted(&[[0.0, 0.0], [10.0, 0.0], [10.0, 6.0], [0.0, 6.0]], &[1.0, 1.0, 2.0, 1.0])];
    let skeleton = check("asymmetric", &rings);
    let ridge: Vec<&SkeletonNode> = skeleton.nodes.iter().filter(|node| node.time > 0.0).collect();
    assert!((ridge[0].time - 2.0).abs() < 1e-9, "the lines meet when 1 t + 2 t = 6");
    assert!((ridge[0].point.y - 2.0).abs() < 1e-9);
}

#[test]
fn convex_polygons_equal_the_lower_envelope_of_their_planes() {
    let rings = [weighted(&[[0.0, 0.0], [9.0, 0.0], [11.0, 5.0], [4.0, 8.0], [-2.0, 4.0]], &[1.0, 0.7, 1.3, 0.5, 1.0])];
    let skeleton = check("pentagon", &rings);
    for i in 0..40 {
        for j in 0..40 {
            let p = [-1.5 + 12.0 * f64::from(i) / 39.0, 0.3 + 7.0 * f64::from(j) / 39.0];
            if let Some(h) = height_at(&skeleton, p) {
                assert!((h - convex_height(&rings, p)).abs() < 1e-7, "at {p:?}: {h} vs {}", convex_height(&rings, p));
            }
        }
    }
}

#[test]
fn an_l_shape_has_a_valley_from_its_reflex_corner() {
    let rings = [uniform(&[[0.0, 0.0], [6.0, 0.0], [6.0, 2.0], [2.0, 2.0], [2.0, 6.0], [0.0, 6.0]])];
    let skeleton = check("L", &rings);
    assert_eq!(skeleton.arcs.iter().filter(|arc| arc.reflex).count(), 1);
    assert!((skeleton.peak() - 1.0).abs() < 1e-9);
}

#[test]
fn u_t_plus_and_h_shapes_partition_their_area() {
    let shapes: [(&str, Vec<[f64; 2]>); 4] = [
        ("U", vec![[0.0, 0.0], [9.0, 0.0], [9.0, 6.0], [6.0, 6.0], [6.0, 2.0], [3.0, 2.0], [3.0, 6.0], [0.0, 6.0]]),
        ("T", vec![[0.0, 4.0], [3.0, 4.0], [3.0, 0.0], [5.0, 0.0], [5.0, 4.0], [8.0, 4.0], [8.0, 6.0], [0.0, 6.0]]),
        ("plus", vec![[2.0, 0.0], [4.0, 0.0], [4.0, 2.0], [6.0, 2.0], [6.0, 4.0], [4.0, 4.0], [4.0, 6.0], [2.0, 6.0], [2.0, 4.0], [0.0, 4.0], [0.0, 2.0], [2.0, 2.0]]),
        ("H", vec![[0.0, 0.0], [2.0, 0.0], [2.0, 3.0], [5.0, 3.0], [5.0, 0.0], [7.0, 0.0], [7.0, 8.0], [5.0, 8.0], [5.0, 5.0], [2.0, 5.0], [2.0, 8.0], [0.0, 8.0]]),
    ];
    for (name, shape) in shapes {
        check(name, &[uniform(&shape)]);
    }
}

#[test]
fn a_frame_with_a_hole_merges_the_two_rings() {
    let outer = uniform(&[[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]]);
    let hole = uniform(&[[3.0, 3.0], [3.0, 7.0], [7.0, 7.0], [7.0, 3.0]]);
    let skeleton = check("frame", &[outer, hole]);
    assert_eq!(skeleton.faces.len(), 8);
    assert!((skeleton.peak() - 1.5).abs() < 1e-9);
}

#[test]
fn an_off_centre_hole_and_a_hole_in_an_l_shape_still_partition_the_region() {
    let outer = uniform(&[[0.0, 0.0], [12.0, 0.0], [12.0, 8.0], [0.0, 8.0]]);
    let hole = uniform(&[[2.0, 2.0], [2.0, 4.0], [5.0, 4.0], [5.0, 2.0]]);
    check("off-centre hole", &[outer, hole]);
    let outer = uniform(&[[0.0, 0.0], [12.0, 0.0], [12.0, 4.0], [6.0, 4.0], [6.0, 12.0], [0.0, 12.0]]);
    let hole = uniform(&[[1.5, 1.5], [1.5, 3.0], [3.5, 3.0], [3.5, 1.5]]);
    check("L with hole", &[outer, hole]);
}

#[test]
fn a_horizon_stops_the_wavefront_and_closes_the_faces() {
    let rings = [uniform(&[[0.0, 0.0], [10.0, 0.0], [10.0, 4.0], [0.0, 4.0]])];
    let skeleton = straight_skeleton_until(&rings, Some(1.0), &mut || true).unwrap();
    assert_eq!(skeleton.wavefront.len(), 1);
    let ring = &skeleton.wavefront[0];
    let corners: Vec<(f64, f64)> = ring.points.iter().map(|p| (p.x, p.y)).collect();
    for want in [(1.0, 1.0), (9.0, 1.0), (9.0, 3.0), (1.0, 3.0)] {
        assert!(corners.iter().any(|c| (c.0 - want.0).abs() < 1e-9 && (c.1 - want.1).abs() < 1e-9), "{corners:?}");
    }
    let swept: f64 = skeleton.faces.iter().map(|face| skeleton.face_area(face)).sum();
    assert!((swept - (40.0 - 16.0)).abs() < 1e-9, "{swept}");
}

#[test]
fn the_wavefront_feeds_a_second_run_with_other_speeds() {
    let rings = [uniform(&[[0.0, 0.0], [10.0, 0.0], [10.0, 4.0], [0.0, 4.0]])];
    let first = straight_skeleton_until(&rings, Some(1.0), &mut || true).unwrap();
    let next: Vec<Ring> = first.wavefront.iter().map(|w| Ring { points: w.points.clone(), speeds: vec![2.0; w.points.len()], tags: w.tags.clone() }).collect();
    let second = check("second stage", &next);
    assert!((second.peak() - 0.5).abs() < 1e-9, "the 2 m wide remainder closes when 2 t + 2 t = 2: {}", second.peak());
}

#[test]
fn cancellation_stops_the_run() {
    let rings = [uniform(&[[0.0, 0.0], [10.0, 0.0], [10.0, 4.0], [0.0, 4.0]])];
    assert_eq!(straight_skeleton_until(&rings, None, &mut || false), Err(SkeletonError::Cancelled));
}

#[test]
fn invalid_inputs_are_refused() {
    assert_eq!(straight_skeleton(&[]), Err(SkeletonError::NoRings));
    assert_eq!(straight_skeleton(&[uniform(&[[0.0, 0.0], [1.0, 0.0]])]), Err(SkeletonError::TooFewVertices { ring: 0 }));
    assert_eq!(straight_skeleton(&[uniform(&[[0.0, 0.0], [0.0, 0.0], [1.0, 1.0]])]), Err(SkeletonError::DegenerateEdge { ring: 0, edge: 0 }));
    assert_eq!(straight_skeleton(&[uniform(&[[0.0, 0.0], [4.0, 4.0], [4.0, 0.0], [0.0, 4.0]])]), Err(SkeletonError::SelfIntersecting));
    assert_eq!(straight_skeleton(&[weighted(&[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]], &[1.0, -1.0, 1.0])]), Err(SkeletonError::InvalidSpeed { ring: 0, edge: 1 }));
    assert_eq!(straight_skeleton(&[uniform(&[[0.0, 0.0], [1.0, f64::NAN], [1.0, 1.0]])]), Err(SkeletonError::NonFinite));
}

struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1_u64 << 53) as f64
    }
}

fn bounded(skeleton: &Skeleton, rings: &[Ring], rng: &mut Lcg, name: &str) {
    let outer = &rings[0];
    let polygon: Vec<V2> = outer.points.iter().map(|p| xy(*p)).collect();
    let (lo, hi) = polygon.iter().fold(([f64::MAX; 2], [f64::MIN; 2]), |(lo, hi), p| ([lo[0].min(p[0]), lo[1].min(p[1])], [hi[0].max(p[0]), hi[1].max(p[1])]));
    for _ in 0..60 {
        let p = [lo[0] + rng.next() * (hi[0] - lo[0]), lo[1] + rng.next() * (hi[1] - lo[1])];
        if !inside(&polygon, p) || rings[1..].iter().any(|hole| inside(&hole.points.iter().map(|q| xy(*q)).collect::<Vec<_>>(), p)) {
            continue;
        }
        let Some(h) = height_at(skeleton, p) else { continue };
        let boundary = rings.iter().map(|ring| (0..ring.points.len()).map(|i| {
            let (a, b) = (xy(ring.points[i]), xy(ring.points[(i + 1) % ring.points.len()]));
            let ab = sub(b, a);
            let s = (dot(sub(p, a), ab) / dot(ab, ab)).clamp(0.0, 1.0);
            dist(p, add(a, scale(ab, s)))
        }).fold(f64::INFINITY, f64::min)).fold(f64::INFINITY, f64::min);
        assert!(h <= boundary + 1e-7 && h >= -1e-9, "{name}: height {h} at {p:?} exceeds the distance {boundary} to the boundary");
    }
}

#[test]
fn random_star_polygons_partition_and_stay_below_the_boundary_distance() {
    let mut rng = Lcg(0x5851F42D4C957F2D);
    for case in 0..400 {
        let n = 4 + (rng.next() * 14.0) as usize;
        let points: Vec<Point> = (0..n).map(|k| {
            let angle = (f64::from(k as u32) + 0.15 + 0.7 * rng.next()) / n as f64 * std::f64::consts::TAU;
            let radius = 2.0 + 8.0 * rng.next();
            Point::new(radius * angle.cos(), radius * angle.sin())
        }).collect();
        let rings = [Ring::uniform(&points, 1.0)];
        let skeleton = check(&format!("star {case}"), &rings);
        bounded(&skeleton, &rings, &mut rng, &format!("star {case}"));
    }
}

#[test]
fn a_footprint_of_two_hundred_edges_runs_to_the_end() {
    let mut rng = Lcg(0xC0FFEE);
    let n = 200;
    let points: Vec<Point> = (0..n).map(|k| {
        let angle = (f64::from(k as u32) + 0.1 + 0.8 * rng.next()) / n as f64 * std::f64::consts::TAU;
        let radius = 20.0 + 10.0 * rng.next();
        Point::new(radius * angle.cos(), radius * angle.sin())
    }).collect();
    check("two hundred edges", &[Ring::uniform(&points, 1.0)]);
}

#[test]
fn random_weighted_star_polygons_partition_and_keep_their_planes() {
    let mut rng = Lcg(0x2545F4914F6CDD1D);
    for case in 0..400 {
        let n = 4 + (rng.next() * 12.0) as usize;
        let points: Vec<Point> = (0..n).map(|k| {
            let angle = (f64::from(k as u32) + 0.2 + 0.6 * rng.next()) / n as f64 * std::f64::consts::TAU;
            let radius = 2.0 + 8.0 * rng.next();
            Point::new(radius * angle.cos(), radius * angle.sin())
        }).collect();
        let speeds: Vec<f64> = (0..n).map(|_| 0.4 + 1.6 * rng.next()).collect();
        let ring = Ring { points: points.clone(), speeds, tags: (0..n as u32).collect() };
        check(&format!("weighted star {case}"), &[ring]);
    }
}

#[test]
fn random_orthogonal_histograms_with_many_simultaneous_events_partition_their_area() {
    let mut rng = Lcg(0x9E3779B97F4A7C15);
    for case in 0..300 {
        let columns = 2 + (rng.next() * 7.0) as usize;
        let heights: Vec<f64> = (0..columns).map(|_| 1.0 + (rng.next() * 5.0).floor()).collect();
        let mut points = vec![Point::new(0.0, 0.0), Point::new(columns as f64, 0.0)];
        let mut x = columns as f64;
        let mut previous = None;
        for (k, &h) in heights.iter().enumerate().rev() {
            if previous != Some(h) {
                points.push(Point::new(x, h));
            }
            x = k as f64;
            points.push(Point::new(x, h));
            previous = Some(h);
        }
        points.dedup_by(|a, b| a.x == b.x && a.y == b.y);
        if points.first() == points.last() {
            points.pop();
        }
        let mut cleaned: Vec<Point> = Vec::new();
        for (i, p) in points.iter().enumerate() {
            let (a, b) = (points[(i + points.len() - 1) % points.len()], points[(i + 1) % points.len()]);
            let collinear = (p.x - a.x) * (b.y - p.y) - (p.y - a.y) * (b.x - p.x) == 0.0;
            if !collinear {
                cleaned.push(*p);
            }
        }
        check(&format!("histogram {case} {heights:?}"), &[Ring::uniform(&cleaned, 1.0)]);
    }
}

fn fixture_rings(case: &serde_json::Value) -> Vec<Ring> {
    let ring = |value: &serde_json::Value| -> Vec<Point> { value.as_array().unwrap().iter().map(|p| Point::new(p[0].as_f64().unwrap(), p[1].as_f64().unwrap())).collect() };
    let mut rings = vec![ring(&case["outer"])];
    rings.extend(case["holes"].as_array().unwrap().iter().map(ring));
    let speeds = &case["speeds"];
    rings.iter().enumerate().map(|(r, points)| {
        let list: Vec<f64> = if speeds.is_null() { vec![1.0; points.len()] } else { speeds[r].as_array().unwrap().iter().map(|s| s.as_f64().unwrap()).collect() };
        Ring { points: points.clone(), speeds: list, tags: (0..points.len() as u32).collect() }
    }).collect()
}

#[test]
fn fixtures_reproduce_the_third_party_face_areas_and_unswept_areas() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🦴️skeleton/🔣️.json")).unwrap();
    assert!(cases.as_array().unwrap().len() >= 50);
    for case in cases.as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let rings = fixture_rings(case);
        let skeleton = check(name, &rings);
        if !case["peak"].is_null() {
            let peak = case["peak"].as_f64().unwrap();
            assert!((skeleton.peak() - peak).abs() <= 1e-6 * peak.max(1.0), "{name}: peak {} vs {peak}", skeleton.peak());
        }
        if !case["faces"].is_null() {
            let wanted: Vec<f64> = case["faces"].as_array().unwrap().iter().map(|a| a.as_f64().unwrap()).collect();
            let got: Vec<f64> = skeleton.faces.iter().map(|face| skeleton.face_area(face)).collect();
            assert_eq!(got.len(), wanted.len(), "{name}");
            for (edge, (a, b)) in got.iter().zip(&wanted).enumerate() {
                assert!((a - b).abs() <= 1e-6 * b.max(1.0), "{name}: face {edge} area {a} vs {b}");
            }
        }
        if !case["levels"].is_null() {
            let total = region_area(&rings);
            for level in case["levels"].as_array().unwrap() {
                let (t, remaining) = (level[0].as_f64().unwrap(), level[1].as_f64().unwrap());
                let got = total - swept_area(&skeleton, t);
                assert!((got - remaining).abs() <= 1e-6 * total.max(1.0), "{name}: unswept area at {t}: {got} vs {remaining}");
            }
        }
    }
}

#[test]
fn vertical_edges_next_to_reflex_corners_keep_the_surface_continuous() {
    let speed = 1.0 / 30.0_f64.to_radians().tan();
    let rings = [weighted(&[[0.0, 4.0], [3.0, 4.0], [3.0, 0.0], [5.0, 0.0], [5.0, 4.0], [8.0, 4.0], [8.0, 6.0], [0.0, 6.0]], &[speed, 0.0, speed, 0.0, speed, 0.0, speed, 0.0])];
    let skeleton = check("gable T", &rings);
    assert!(skeleton.faces.iter().filter(|face| face.speed == 0.0).all(|face| skeleton.face_area(face).abs() < 1e-6));
}

#[test]
fn a_cross_gable_closes_with_vertical_ends_only_at_convex_corners() {
    let speed = 1.0 / 30.0_f64.to_radians().tan();
    let rings = [weighted(&[[0.0, 4.0], [3.0, 4.0], [3.0, 0.0], [5.0, 0.0], [5.0, 4.0], [8.0, 4.0], [8.0, 6.0], [0.0, 6.0]], &[speed, speed, 0.0, speed, speed, 0.0, speed, 0.0])];
    let skeleton = check("cross gable", &rings);
    assert!((skeleton.peak() - 30.0_f64.to_radians().tan()).abs() < 1e-7);
}
