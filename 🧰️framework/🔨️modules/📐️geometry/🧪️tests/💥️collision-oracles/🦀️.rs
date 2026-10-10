use parry3d::math::{Isometry, Point as PPoint, Vector as PVector};
use parry3d::query::{closest_points, contact, intersection_test, ClosestPoints, PointQuery};
use parry3d::shape::{Cuboid, Shape, Triangle};
use semio_framework_geometry::collision::{clash, triangle_distance, triangles_intersect, Bvh, ClashKind};
use semio_framework_geometry::mesh::{extrude, TriMesh};
use semio_framework_geometry::placement::{Affine3, ZPlane};
use semio_framework_geometry::random::Rng;
use semio_framework_geometry::vector::{cross3, length3, sub3, Xyz};
use semio_framework_geometry::Point;
use serde_json::Value;

fn pp(p: Xyz) -> PPoint<f32> {
    PPoint::new(p[0] as f32, p[1] as f32, p[2] as f32)
}

fn parry_triangle(t: [Xyz; 3]) -> Triangle {
    Triangle::new(pp(t[0]), pp(t[1]), pp(t[2]))
}

fn separation(p1: &Isometry<f32>, g1: &dyn Shape, p2: &Isometry<f32>, g2: &dyn Shape) -> f64 {
    match closest_points(p1, g1, p2, g2, 1.0e6).unwrap() {
        ClosestPoints::Intersecting => 0.0,
        ClosestPoints::WithinMargin(a, b) => f64::from((a - b).norm()),
        ClosestPoints::Disjoint => f64::INFINITY,
    }
}

fn boxed(lo: Xyz, hi: Xyz) -> TriMesh {
    let ring = [Point::new(lo[0], lo[1]), Point::new(hi[0], lo[1]), Point::new(hi[0], hi[1]), Point::new(lo[0], hi[1])];
    extrude(&ring, &[], ZPlane::flat(lo[2]), ZPlane::flat(hi[2]))
}

fn run(a: &TriMesh, b: &TriMesh, tolerance: f64, clearance: f64) -> Option<semio_framework_geometry::collision::MeshClash> {
    clash(&Bvh::build(a), a, &Bvh::build(b), b, tolerance, clearance, &|| false)
}

fn random_triangle(rng: &mut Rng) -> [Xyz; 3] {
    loop {
        let t: [Xyz; 3] = std::array::from_fn(|_| [rng.next_f64(), rng.next_f64(), rng.next_f64()]);
        if length3(cross3(sub3(t[1], t[0]), sub3(t[2], t[0]))) > 0.05 {
            return t;
        }
    }
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-4 * b.abs().max(1.0)
}

#[test]
fn triangle_intersection_agrees_with_parry3d() {
    let mut rng = Rng::from_seed(0xC011_1DE);
    let (mut hits, mut misses) = (0, 0);
    for case in 0..4000 {
        let (a, b) = (random_triangle(&mut rng), random_triangle(&mut rng));
        let theirs = intersection_test(&Isometry::identity(), &parry_triangle(a), &Isometry::identity(), &parry_triangle(b)).unwrap();
        let ours = triangles_intersect(a, b);
        let gap = triangle_distance(a, b).0;
        let agrees = ours.is_some() == theirs;
        assert!(agrees || gap < 1e-3 || separation(&Isometry::identity(), &parry_triangle(a), &Isometry::identity(), &parry_triangle(b)) < 1e-3, "case {case}: ours {ours:?} parry {theirs} gap {gap}");
        if let Some(s) = ours {
            hits += 1;
            for p in s {
                let d = parry_triangle(a).distance_to_point(&Isometry::identity(), &pp(p), true).max(parry_triangle(b).distance_to_point(&Isometry::identity(), &pp(p), true));
                assert!(d < 1e-4, "case {case}: segment end off a triangle by {d}");
            }
        } else {
            misses += 1;
        }
    }
    assert!(hits > 200 && misses > 200, "hits {hits} misses {misses}");
}

#[test]
fn triangle_distance_agrees_with_parry3d() {
    let mut rng = Rng::from_seed(0xD157_4CE);
    let mut positive = 0;
    for case in 0..4000 {
        let a = random_triangle(&mut rng);
        let offset = [rng.next_f64() * 3.0 - 1.0, rng.next_f64() * 3.0 - 1.0, rng.next_f64() * 3.0 - 1.0];
        let b = random_triangle(&mut rng).map(|p| [p[0] + offset[0], p[1] + offset[1], p[2] + offset[2]]);
        let (d, p, q) = triangle_distance(a, b);
        let theirs = separation(&Isometry::identity(), &parry_triangle(a), &Isometry::identity(), &parry_triangle(b));
        assert!(d <= theirs + 1e-4 && theirs - d <= 1e-3 * theirs.max(1.0), "case {case}: ours {d} parry {theirs}");
        assert!((length3(sub3(p, q)) - d).abs() < 1e-9, "case {case}: points do not realise the distance");
        assert!(parry_triangle(a).distance_to_point(&Isometry::identity(), &pp(p), true) < 1e-4);
        assert!(parry_triangle(b).distance_to_point(&Isometry::identity(), &pp(q), true) < 1e-4);
        if d > 0.0 {
            positive += 1;
        }
    }
    assert!(positive > 1000, "positive {positive}");
}

#[test]
fn axis_aligned_box_pairs_match_the_analytic_overlap() {
    let mut rng = Rng::from_seed(0xB0C5);
    let (mut hard, mut gaps, mut clear) = (0, 0, 0);
    for case in 0..400 {
        let mut corner = || -> (Xyz, Xyz) {
            let lo: Xyz = std::array::from_fn(|_| rng.next_f64() * 4.0);
            let size: Xyz = std::array::from_fn(|_| 0.2 + rng.next_f64() * 2.5);
            (lo, [lo[0] + size[0], lo[1] + size[1], lo[2] + size[2]])
        };
        let ((alo, ahi), (blo, bhi)) = (corner(), corner());
        let overlap: Xyz = std::array::from_fn(|k| ahi[k].min(bhi[k]) - alo[k].max(blo[k]));
        let gap = (0..3).map(|k| (-overlap[k]).max(0.0).powi(2)).sum::<f64>().sqrt();
        if overlap.iter().any(|o| o.abs() < 1e-6) {
            continue;
        }
        let clearance = 0.6;
        if (gap - clearance).abs() < 1e-6 {
            continue;
        }
        let got = run(&boxed(alo, ahi), &boxed(blo, bhi), 0.0, clearance);
        if overlap.iter().all(|&o| o > 0.0) {
            let extent = overlap.iter().cloned().fold(f64::INFINITY, f64::min);
            let got = got.unwrap_or_else(|| panic!("case {case}: expected a hard clash"));
            assert_eq!(got.kind, ClashKind::Hard, "case {case}");
            assert!((got.distance + extent).abs() < 1e-9, "case {case}: {} vs {extent}", got.distance);
            for k in 0..3 {
                assert!((got.point[k] - (ahi[k].min(bhi[k]) + alo[k].max(blo[k])) / 2.0).abs() < 1e-9, "case {case}: point axis {k}");
            }
            hard += 1;
        } else if gap < clearance {
            let got = got.unwrap_or_else(|| panic!("case {case}: expected a clearance clash"));
            assert_eq!(got.kind, ClashKind::Clearance, "case {case}");
            assert!((got.distance - gap).abs() < 1e-9, "case {case}: {} vs {gap}", got.distance);
            clear += 1;
        } else {
            assert_eq!(got, None, "case {case}");
            gaps += 1;
        }
    }
    assert!(hard > 40 && gaps > 40 && clear > 10, "hard {hard} gaps {gaps} clear {clear}");
}

fn placed(half: Xyz, axis: Xyz, angle: f64, at: Xyz) -> (TriMesh, Cuboid, Isometry<f32>) {
    let mesh = boxed([-half[0], -half[1], -half[2]], half).transformed(&Affine3::rotation_axis(axis, angle).then(&Affine3::translation(at)));
    let a = parry3d::na::Unit::new_normalize(PVector::new(axis[0] as f32, axis[1] as f32, axis[2] as f32)).into_inner() * angle as f32;
    (mesh, Cuboid::new(PVector::new(half[0] as f32, half[1] as f32, half[2] as f32)), Isometry::new(PVector::new(at[0] as f32, at[1] as f32, at[2] as f32), a))
}

#[test]
fn rotated_box_pairs_agree_with_parry3d() {
    let mut rng = Rng::from_seed(0xA0C0);
    let (mut hard, mut clear, mut apart) = (0, 0, 0);
    for case in 0..600 {
        let mut one = || {
            let half: Xyz = std::array::from_fn(|_| 0.3 + rng.next_f64() * 1.0);
            let axis: Xyz = [rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() + 0.2];
            let at: Xyz = std::array::from_fn(|_| rng.next_f64() * 4.0 - 2.0);
            placed(half, axis, rng.next_f64() * 6.0, at)
        };
        let ((ma, ca, ia), (mb, cb, ib)) = (one(), one());
        let clearance = 0.5;
        let theirs = intersection_test(&ia, &ca, &ib, &cb).unwrap();
        let gap = separation(&ia, &ca, &ib, &cb);
        let depth = contact(&ia, &ca, &ib, &cb, 0.01).unwrap().map(|c| f64::from(c.dist));
        if depth.is_some_and(|d| d.abs() < 1e-3) || (gap - clearance).abs() < 1e-3 {
            continue;
        }
        let got = run(&ma, &mb, 0.0, clearance);
        if theirs {
            let got = got.unwrap_or_else(|| panic!("case {case}: parry intersects, ours none"));
            assert_eq!(got.kind, ClashKind::Hard, "case {case}");
            assert!(got.distance < 0.0 && -got.distance <= 2.0 * 1.3 + 1e-9, "case {case}: extent {}", got.distance);
            hard += 1;
        } else if gap < clearance {
            let got = got.unwrap_or_else(|| panic!("case {case}: gap {gap} within clearance, ours none"));
            assert_eq!(got.kind, ClashKind::Clearance, "case {case}");
            assert!(got.distance <= gap + 1e-4 && gap - got.distance <= 1e-3 * gap.max(1.0), "case {case}: ours {} parry {gap}", got.distance);
            clear += 1;
        } else {
            assert_eq!(got, None, "case {case}: gap {gap}");
            apart += 1;
        }
    }
    assert!(hard > 30 && clear > 10 && apart > 100, "hard {hard} clear {clear} apart {apart}");
}

fn fixture_box(v: &Value) -> TriMesh {
    let xyz = |w: &Value| -> Xyz { [w[0].as_f64().unwrap(), w[1].as_f64().unwrap(), w[2].as_f64().unwrap()] };
    boxed(xyz(&v["min"]), xyz(&v["max"]))
}

#[test]
fn fixture_cases_agree_with_parry3d_where_parry_has_an_answer() {
    let cases: Value = serde_json::from_str(include_str!("../../🧫️fixtures/💥️collision/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let (a, b) = (fixture_box(&case["a"]), fixture_box(&case["b"]));
        let cuboid = |m: &TriMesh| {
            let (lo, hi) = m.bounds().unwrap();
            (Cuboid::new(PVector::new(((hi[0] - lo[0]) / 2.0) as f32, ((hi[1] - lo[1]) / 2.0) as f32, ((hi[2] - lo[2]) / 2.0) as f32)), Isometry::translation(((hi[0] + lo[0]) / 2.0) as f32, ((hi[1] + lo[1]) / 2.0) as f32, ((hi[2] + lo[2]) / 2.0) as f32))
        };
        let ((ca, ia), (cb, ib)) = (cuboid(&a), cuboid(&b));
        let gap = separation(&ia, &ca, &ib, &cb);
        let expected = &case["expected"];
        if expected["kind"] == "clearance" {
            assert!(close(gap, expected["distance"].as_f64().unwrap()), "{name}: parry gap {gap}");
        }
        if expected["kind"] == "hard" {
            assert!(intersection_test(&ia, &ca, &ib, &cb).unwrap(), "{name}: parry sees no overlap");
        }
    }
}
