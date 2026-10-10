#[cfg(test)]
use super::*;
use crate::mesh::extrude;
use crate::placement::{Affine3, ZPlane};
use crate::random::Rng;
use crate::Point;
use serde_json::Value;
use std::cell::Cell;

fn fixtures() -> Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/💥️collision/🔣️.json")).unwrap()
}

fn xyz(v: &Value) -> Xyz {
    [v[0].as_f64().unwrap(), v[1].as_f64().unwrap(), v[2].as_f64().unwrap()]
}

fn cuboid(v: &Value) -> TriMesh {
    boxed(xyz(&v["min"]), xyz(&v["max"]))
}

fn boxed(lo: Xyz, hi: Xyz) -> TriMesh {
    let ring = [Point::new(lo[0], lo[1]), Point::new(hi[0], lo[1]), Point::new(hi[0], hi[1]), Point::new(lo[0], hi[1])];
    extrude(&ring, &[], ZPlane::flat(lo[2]), ZPlane::flat(hi[2]))
}

fn never() -> bool {
    false
}

fn run(a: &TriMesh, b: &TriMesh, tolerance: f64, clearance: f64) -> Option<MeshClash> {
    clash(&Bvh::build(a), a, &Bvh::build(b), b, tolerance, clearance, &never)
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn fixtures_match_in_both_argument_orders() {
    for case in fixtures().as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let (a, b) = (cuboid(&case["a"]), cuboid(&case["b"]));
        let (tolerance, clearance) = (case["tolerance"].as_f64().unwrap(), case["clearance"].as_f64().unwrap());
        let expected = &case["expected"];
        for (first, second) in [(&a, &b), (&b, &a)] {
            let got = run(first, second, tolerance, clearance);
            match expected["kind"].as_str().unwrap() {
                "none" => assert_eq!(got, None, "{name}"),
                kind => {
                    let got = got.unwrap_or_else(|| panic!("{name}: expected a clash"));
                    assert_eq!(got.kind, if kind == "hard" { ClashKind::Hard } else { ClashKind::Clearance }, "{name}");
                    assert!(near(got.distance, expected["distance"].as_f64().unwrap()), "{name}: distance {}", got.distance);
                    for (axis, want) in expected["point"].as_array().unwrap().iter().enumerate() {
                        if let Some(want) = want.as_f64() {
                            assert!(near(got.point[axis], want), "{name}: point axis {axis} {}", got.point[axis]);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn hard_clash_reports_pairs_and_bounds() {
    let (a, b) = (boxed([0.0; 3], [2.0; 3]), boxed([1.0; 3], [3.0; 3]));
    let got = run(&a, &b, 0.0, 0.0).unwrap();
    assert!(got.pairs > 0);
    assert!(got.bounds.min.iter().all(|&v| near(v, 1.0)) && got.bounds.max.iter().all(|&v| near(v, 2.0)));
}

#[test]
fn triangle_pairs_cross_in_the_expected_segment() {
    let a = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
    let b = [[0.5, 0.5, -1.0], [0.5, 0.5, 1.0], [1.5, 0.5, 0.0]];
    let s = triangles_intersect(a, b).unwrap();
    let (lo, hi) = if s[0][0] < s[1][0] { (s[0], s[1]) } else { (s[1], s[0]) };
    assert!(lo.iter().zip([0.5, 0.5, 0.0]).all(|(x, y)| near(*x, y)));
    assert!(hi.iter().zip([1.5, 0.5, 0.0]).all(|(x, y)| near(*x, y)));
    assert_eq!(triangles_intersect(b, a).map(|s| s.len()), Some(2));
}

#[test]
fn separated_triangles_do_not_intersect() {
    let a = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
    let b = [[0.0, 0.0, 0.5], [1.0, 0.0, 0.5], [0.0, 1.0, 0.5]];
    assert_eq!(triangles_intersect(a, b), None);
    let (d, p, q) = triangle_distance(a, b);
    assert!(near(d, 0.5) && near(p[2], 0.0) && near(q[2], 0.5));
}

#[test]
fn point_touch_is_a_degenerate_segment() {
    let a = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
    let b = [[0.2, 0.2, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]];
    let s = triangles_intersect(a, b).unwrap();
    assert!(s[0].iter().zip(s[1]).all(|(x, y)| near(*x, y)));
    assert!(s[0].iter().zip([0.2, 0.2, 0.0]).all(|(x, y)| near(*x, y)));
}

#[test]
fn coplanar_overlap_returns_extreme_segment_and_polygon() {
    let a = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
    let b = [[1.0, 0.0, 0.0], [3.0, 0.0, 0.0], [1.0, 2.0, 0.0]];
    let polygon = triangle_contact(a, b).unwrap();
    assert!(polygon.len() >= 3);
    let s = triangles_intersect(a, b).unwrap();
    let length = length3(sub3(s[0], s[1]));
    assert!(length > 1.0 && length <= 2.0_f64.sqrt() + 1e-9);
    let apart = [[5.0, 5.0, 0.0], [6.0, 5.0, 0.0], [5.0, 6.0, 0.0]];
    assert_eq!(triangles_intersect(a, apart), None);
}

#[test]
fn degenerate_triangles_never_intersect_and_still_have_a_distance() {
    let line = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
    let point = [[0.5, 0.0, 0.0]; 3];
    let tri = [[0.0, -1.0, -1.0], [0.0, 1.0, -1.0], [0.0, 0.0, 1.0]];
    assert_eq!(triangles_intersect(line, tri), None);
    assert_eq!(triangles_intersect(point, tri), None);
    assert_eq!(triangles_intersect(tri, point), None);
    let (d, _, _) = triangle_distance(point, tri);
    assert!(near(d, 0.5));
    let (d, p, q) = triangle_distance(line, [[0.0, 3.0, 0.0]; 3]);
    assert!(near(d, 3.0) && near(p[1], 0.0) && near(q[1], 3.0));
}

#[test]
fn intersecting_triangles_have_zero_distance() {
    let a = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
    let b = [[0.5, 0.5, -1.0], [0.5, 0.5, 1.0], [1.5, 0.5, 0.0]];
    let (d, p, q) = triangle_distance(a, b);
    assert_eq!(d, 0.0);
    assert_eq!(p, q);
}

#[test]
fn bvh_covers_the_mesh() {
    let m = boxed([0.0; 3], [1.0, 2.0, 3.0]);
    let bvh = Bvh::build(&m);
    assert_eq!(bvh.triangle_count(), m.triangle_count());
    assert_eq!(bvh.bounds(), mesh_aabb(&m));
    let empty = Bvh::build(&TriMesh::new());
    assert_eq!((empty.triangle_count(), empty.bounds()), (0, None));
    assert_eq!(empty, Bvh::default());
    assert_eq!(run(&TriMesh::new(), &m, 0.0, 1.0), None);
}

#[test]
fn aabb_algebra() {
    let a = Aabb { min: [0.0; 3], max: [1.0; 3] };
    let b = Aabb { min: [2.0, 0.0, 0.0], max: [3.0, 1.0, 1.0] };
    assert!(!a.overlaps(&b, 0.5) && a.overlaps(&b, 1.0));
    assert!(near(a.distance(&b), 1.0) && a.distance(&a) == 0.0);
    assert_eq!(a.union(&b), Aabb { min: [0.0; 3], max: [3.0, 1.0, 1.0] });
    assert_eq!(a.centre(), [0.5; 3]);
    assert!(near(a.union(&b).min_extent(), 1.0));
    assert_eq!(Aabb::from_points(&[]), None);
}

fn plate(lo: Xyz, hi: Xyz, cells: usize) -> TriMesh {
    let mut mesh = TriMesh::new();
    let at = |i: usize, j: usize| [lo[0] + (hi[0] - lo[0]) * i as f64 / cells as f64, lo[1] + (hi[1] - lo[1]) * j as f64 / cells as f64, lo[2] + (hi[2] - lo[2]) * (i + j) as f64 / (2 * cells) as f64];
    for i in 0..cells {
        for j in 0..cells {
            mesh.push_quad(at(i, j), at(i + 1, j), at(i + 1, j + 1), at(i, j + 1));
        }
    }
    mesh
}

#[test]
fn bvh_descent_finds_exactly_the_brute_force_pairs() {
    let mut rng = Rng::from_seed(7);
    let mut total = 0;
    for case in 0..6 {
        let r = |rng: &mut Rng| rng.next_f64() * 0.5;
        let a = plate([0.0, 0.0, 0.0], [4.0, 4.0, 1.0 + r(&mut rng)], 14);
        let b = plate([r(&mut rng), 1.0 + r(&mut rng), -1.0 + r(&mut rng)], [5.0, 3.0, 0.5 + r(&mut rng)], 11).transformed(&Affine3::rotation_axis([1.0, 0.3, 0.2], 0.4 + case as f64 * 0.3));
        let (ba, bb) = (Bvh::build(&a), Bvh::build(&b));
        let (pairs, bounds) = crossings(&ba, &a, &bb, &b, &never).ok().unwrap();
        let mut expected = (0usize, None::<Aabb>);
        for x in 0..a.triangle_count() {
            for y in 0..b.triangle_count() {
                if let Some(points) = triangle_contact(a.triangle(x), b.triangle(y)) {
                    expected.0 += 1;
                    for p in points {
                        expected.1 = Some(expected.1.map_or(Aabb::point(p), |c| c.including(p)));
                    }
                }
            }
        }
        total += expected.0;
        assert_eq!((pairs, bounds), expected, "case {case}");
    }
    assert!(total > 50, "pairs found: {total}");
}

#[test]
fn nearest_matches_brute_force_distance() {
    let mut rng = Rng::from_seed(11);
    for case in 0..8 {
        let a = plate([0.0, 0.0, 0.0], [2.0, 2.0, 0.5], 6);
        let shift = [rng.next_f64() * 4.0 - 1.0, rng.next_f64() * 4.0 - 1.0, 1.5 + rng.next_f64()];
        let b = plate([0.0, 0.0, 0.0], [2.0, 2.0, 0.5], 5).transformed(&Affine3::rotation_axis([0.2, 1.0, 0.1], case as f64 * 0.4).then(&Affine3::translation(shift)));
        let brute = (0..a.triangle_count()).flat_map(|x| (0..b.triangle_count()).map(move |y| (x, y))).map(|(x, y)| triangle_distance(a.triangle(x), b.triangle(y)).0).fold(f64::INFINITY, f64::min);
        let got = nearest(&Bvh::build(&a), &a, &Bvh::build(&b), &b, f64::INFINITY, &never).ok().unwrap().unwrap().0;
        assert!(near(got, brute), "case {case}: {got} vs {brute}");
    }
}

#[test]
fn cancelled_runs_return_none() {
    let (a, b) = (boxed([0.0; 3], [2.0; 3]), boxed([1.0; 3], [3.0; 3]));
    let (ba, bb) = (Bvh::build(&a), Bvh::build(&b));
    assert!(clash(&ba, &a, &bb, &b, 0.0, 0.0, &never).is_some());
    assert_eq!(clash(&ba, &a, &bb, &b, 0.0, 0.0, &|| true), None);
    let polls = Cell::new(0);
    let after_three = || {
        polls.set(polls.get() + 1);
        polls.get() > 3
    };
    assert_eq!(clash(&ba, &a, &bb, &b, 0.0, 0.0, &after_three), None);
    assert!(polls.get() > 3);
    let far = boxed([9.0; 3], [10.0; 3]);
    assert_eq!(clash(&ba, &a, &Bvh::build(&far), &far, 0.0, 20.0, &|| true), None);
}

#[test]
fn clashes_are_deterministic() {
    let (a, b) = (boxed([0.0; 3], [2.0; 3]), boxed([1.0, 0.5, 1.5], [3.0; 3]));
    let first = run(&a, &b, 0.0, 0.0);
    for _ in 0..5 {
        assert_eq!(run(&a, &b, 0.0, 0.0), first);
    }
    assert_eq!(Bvh::build(&a), Bvh::build(&a));
}

#[test]
fn rotated_box_pairs_clash_symmetrically() {
    let a = boxed([0.0; 3], [2.0; 3]);
    let b = boxed([1.5, 0.2, 0.2], [4.0, 0.8, 0.8]).transformed(&Affine3::rotation_z(0.3));
    let (ab, ba) = (run(&a, &b, 0.0, 0.0).unwrap(), run(&b, &a, 0.0, 0.0).unwrap());
    assert_eq!(ab.kind, ClashKind::Hard);
    assert!(ab.distance < 0.0 && near(ab.distance, ba.distance) && ab.pairs == ba.pairs);
}
