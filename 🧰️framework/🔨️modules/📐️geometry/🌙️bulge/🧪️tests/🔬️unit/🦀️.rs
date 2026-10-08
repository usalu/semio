use super::*;
use serde_json::Value;

fn fixtures() -> Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/🌙️bulge/🔣️.json")).unwrap()
}

fn pt(v: &Value) -> Point {
    Point::new(v[0].as_f64().unwrap(), v[1].as_f64().unwrap())
}

fn seg(v: &Value) -> BulgeSeg {
    BulgeSeg::new(pt(&v["start"]), pt(&v["end"]), v["bulge"].as_f64().unwrap())
}

fn near(a: Point, b: Point, eps: f64, context: &str) {
    assert!((a - b).hypot() <= eps, "{context}: {a:?} vs {b:?}");
}

fn close(a: f64, b: f64, eps: f64, context: &str) {
    assert!((a - b).abs() <= eps, "{context}: {a} vs {b}");
}

#[test]
fn arcs_match_closed_form_fixtures() {
    for case in fixtures()["arcs"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let s = seg(case);
        let e = &case["expected"];
        close(s.sweep(), e["sweep"].as_f64().unwrap(), 1e-12, name);
        close(s.length(), e["length"].as_f64().unwrap(), 1e-12, name);
        close(s.segment_area(), e["segment_area"].as_f64().unwrap(), 1e-12, name);
        near(s.point_at(0.5), pt(&e["mid"]), 1e-12, name);
        near(s.point_at_length(s.length() / 2.0), pt(&e["mid"]), 1e-12, name);
        match (s.center(), e["center"].is_null()) {
            (Some(c), false) => {
                near(c, pt(&e["center"]), 1e-12, name);
                close(s.radius(), e["radius"].as_f64().unwrap(), 1e-12, name);
            }
            (None, true) => assert!(s.radius().is_infinite()),
            other => panic!("{name}: {other:?}"),
        }
        if let Some(b) = e["bounds"].as_array() {
            let r = s.bounds();
            let want: Vec<f64> = b.iter().map(|x| x.as_f64().unwrap()).collect();
            for (got, want) in [r.x0(), r.y0(), r.x1(), r.y1()].into_iter().zip(want) {
                close(got, want, 1e-12, name);
            }
        }
        near(s.point_at(0.0), s.start, 1e-12, name);
        near(s.point_at(1.0), s.end, 1e-12, name);
    }
}

#[test]
fn tangent_is_unit_and_follows_the_direction_of_travel() {
    let quarter = BulgeSeg::from_arc(Point::ZERO, 1.0, 0.0, std::f64::consts::FRAC_PI_2);
    let t0 = quarter.tangent_at(0.0);
    let t1 = quarter.tangent_at(1.0);
    near(Point::new(t0.x, t0.y), Point::new(0.0, 1.0), 1e-12, "start tangent");
    near(Point::new(t1.x, t1.y), Point::new(-1.0, 0.0), 1e-12, "end tangent");
    let cw = quarter.reversed();
    let c0 = cw.tangent_at(0.0);
    near(Point::new(c0.x, c0.y), Point::new(1.0, 0.0), 1e-12, "reversed start tangent");
    let line = BulgeSeg::line(Point::ZERO, Point::new(0.0, 3.0));
    close(line.tangent_at_length(1.0).y, 1.0, 1e-12, "line tangent");
}

#[test]
fn offsets_match_fixtures() {
    for case in fixtures()["offsets"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let got = seg(&case["seg"]).offset(case["distance"].as_f64().unwrap());
        match (got, case["expected"].is_null()) {
            (None, true) => {}
            (Some(g), false) => {
                let want = seg(&case["expected"]);
                near(g.start, want.start, 1e-12, name);
                near(g.end, want.end, 1e-12, name);
                close(g.bulge, want.bulge, 1e-12, name);
            }
            other => panic!("{name}: {other:?}"),
        }
    }
}

#[test]
fn offset_curves_stay_at_constant_distance() {
    for bulge in [0.0, 0.3, -0.7, 1.0, 2.0] {
        let s = BulgeSeg::new(Point::new(1.0, 2.0), Point::new(4.0, 3.0), bulge);
        for distance in [0.05, -0.05] {
            let Some(o) = s.offset(distance) else { continue };
            for k in 0..=10 {
                let p = o.point_at(f64::from(k) / 10.0);
                close(s.closest(p).distance, distance.abs(), 1e-9, "offset distance");
            }
        }
    }
}

#[test]
fn intersections_match_fixtures() {
    for case in fixtures()["intersections"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let extent = if case["extent"] == "bounded" { Extent::Bounded } else { Extent::Unbounded };
        let mut got: Vec<Point> = intersect(&seg(&case["a"]), &seg(&case["b"]), extent).into_iter().map(|h| h.point).collect();
        got.sort_by(|a, b| a.x.total_cmp(&b.x).then(a.y.total_cmp(&b.y)));
        let want: Vec<Point> = case["expected"].as_array().unwrap().iter().map(pt).collect();
        assert_eq!(got.len(), want.len(), "{name}: {got:?}");
        for (g, w) in got.iter().zip(&want) {
            near(*g, *w, 1e-9, name);
        }
    }
}

#[test]
fn intersection_parameters_locate_the_point_on_each_segment() {
    let a = BulgeSeg::line(Point::new(0.0, 0.0), Point::new(4.0, 4.0));
    let b = BulgeSeg::line(Point::new(0.0, 4.0), Point::new(4.0, 0.0));
    let hit = intersect(&a, &b, Extent::Bounded)[0];
    close(hit.ta, 0.5, 1e-12, "ta");
    close(hit.tb, 0.5, 1e-12, "tb");
    let arc = BulgeSeg::from_arc(Point::ZERO, 1.0, 0.0, std::f64::consts::FRAC_PI_2);
    let ray = BulgeSeg::line(Point::ZERO, Point::new(2.0, 2.0));
    let hit = intersect(&ray, &arc, Extent::Bounded)[0];
    close(hit.tb, 0.5, 1e-12, "arc parameter");
    close(hit.ta, 2f64.sqrt() / 4.0, 1e-12, "ray parameter");
}

#[test]
fn nearest_intersection_picks_the_miter_point() {
    let a = BulgeSeg::line(Point::new(0.0, 0.1), Point::new(3.9, 0.1));
    let b = BulgeSeg::line(Point::new(3.9, 0.0), Point::new(3.9, 4.0));
    near(nearest_intersection(&a, &b, Extent::Unbounded, Point::new(4.0, 0.0)).unwrap(), Point::new(3.9, 0.1), 1e-12, "miter");
    assert!(nearest_intersection(&a, &a, Extent::Unbounded, Point::ZERO).is_none());
}

#[test]
fn closest_matches_fixtures() {
    for case in fixtures()["closest"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let got = seg(&case["seg"]).closest(pt(&case["point"]));
        let e = &case["expected"];
        close(got.t, e["t"].as_f64().unwrap(), 1e-12, name);
        close(got.distance, e["distance"].as_f64().unwrap(), 1e-12, name);
        near(got.point, pt(&e["point"]), 1e-12, name);
    }
}

#[test]
fn three_point_arcs_match_fixtures() {
    for case in fixtures()["three_points"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let got = BulgeSeg::from_three_points(pt(&case["a"]), pt(&case["through"]), pt(&case["b"]));
        match (got, case["expected"].is_null()) {
            (None, true) => {}
            (Some(g), false) => {
                let e = &case["expected"];
                near(g.center().unwrap(), pt(&e["center"]), 1e-12, name);
                close(g.radius(), e["radius"].as_f64().unwrap(), 1e-12, name);
                close(g.sweep(), e["sweep"].as_f64().unwrap(), 1e-12, name);
            }
            other => panic!("{name}: {other:?}"),
        }
    }
}

#[test]
fn split_and_subsegment_stay_on_the_carrier() {
    let s = BulgeSeg::new(Point::new(1.0, 0.0), Point::new(0.0, -1.0), bulge_from_sweep(3.0 * std::f64::consts::FRAC_PI_2));
    let (a, b) = s.split_at(0.4);
    close(a.length() + b.length(), s.length(), 1e-12, "split length");
    near(a.end, b.start, 1e-12, "split point");
    near(a.end, s.point_at(0.4), 1e-12, "split point on arc");
    let sub = s.subsegment(0.25, 0.75);
    near(sub.start, s.point_at(0.25), 1e-12, "sub start");
    near(sub.end, s.point_at(0.75), 1e-12, "sub end");
    close(sub.length(), s.length() / 2.0, 1e-12, "sub length");
}

#[test]
fn flatten_respects_the_chord_tolerance() {
    let s = BulgeSeg::from_arc(Point::new(3.0, -2.0), 5.0, 0.4, 2.2);
    for tolerance in [1e-1, 1e-2, 1e-4] {
        let points = s.flatten(tolerance);
        assert_eq!(points[0], s.start);
        near(*points.last().unwrap(), s.end, 1e-12, "end");
        for w in points.windows(2) {
            let mid = Point::new((w[0].x + w[1].x) / 2.0, (w[0].y + w[1].y) / 2.0);
            let deviation = 5.0 - (mid - Point::new(3.0, -2.0)).hypot();
            assert!(deviation <= tolerance * (1.0 + 1e-9), "tolerance {tolerance}: {deviation}");
        }
    }
    assert_eq!(BulgeSeg::line(Point::ZERO, Point::new(1.0, 0.0)).flatten(0.01).len(), 2);
}

#[test]
fn retarget_extends_and_trims_along_the_same_circle() {
    let s = BulgeSeg::from_arc(Point::ZERO, 2.0, 0.0, 1.0);
    let trimmed = s.retarget(s.point_at(0.25), s.point_at(0.75));
    close(trimmed.sweep(), 0.5, 1e-12, "trimmed sweep");
    near(trimmed.center().unwrap(), Point::ZERO, 1e-12, "centre kept");
    let cw = s.reversed().retarget(s.point_at(0.9), s.point_at(0.1));
    close(cw.sweep(), -0.8, 1e-12, "clockwise sweep");
    let line = BulgeSeg::line(Point::ZERO, Point::new(1.0, 0.0)).retarget(Point::new(-1.0, 0.0), Point::new(3.0, 0.0));
    assert!(line.is_line());
}

#[test]
fn transform_keeps_geometry_and_mirror_flips_the_bulge() {
    let s = BulgeSeg::new(Point::new(1.0, 0.0), Point::new(0.0, 1.0), 0.4);
    let moved = s.transformed(Affine::IDENTITY.rotate(0.7).scale(2.0).translate((1.0, -3.0)));
    close(moved.length(), s.length() * 2.0, 1e-12, "scaled length");
    close(moved.bulge, s.bulge, 1e-12, "bulge kept");
    let mirrored = s.transformed(Affine::new([-1.0, 0.0, 0.0, 1.0, 0.0, 0.0]));
    close(mirrored.bulge, -s.bulge, 1e-12, "bulge flipped");
    close(mirrored.length(), s.length(), 1e-12, "mirror keeps length");
}

#[test]
fn bands_match_fixtures() {
    for case in fixtures()["bands"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let band = band_loop(&seg(&case["axis"]), case["left"].as_f64().unwrap(), case["right"].as_f64().unwrap(), None, None).unwrap();
        for (got, want) in band.iter().zip(case["expected"].as_array().unwrap()) {
            near(got.0, pt(want), 1e-12, name);
            close(got.1, want[2].as_f64().unwrap(), 1e-12, name);
        }
        let area: f64 = {
            let n = band.len();
            let segs: Vec<BulgeSeg> = (0..n).map(|i| BulgeSeg::new(band[i].0, band[(i + 1) % n].0, band[i].1)).collect();
            segs.iter().map(|s| 0.5 * (s.start.x * s.end.y - s.end.x * s.start.y) + s.segment_area()).sum()
        };
        close(area, case["area"].as_f64().unwrap(), 1e-12, name);
    }
}

#[test]
fn corner_joins_match_fixtures() {
    for case in fixtures()["corners"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let widths = |v: &Value| (v[0].as_f64().unwrap(), v[1].as_f64().unwrap());
        let corner = corner_join(&seg(&case["prev"]), widths(&case["prev_widths"]), &seg(&case["next"]), widths(&case["next_widths"]));
        for (got, want) in [(corner.left, &case["expected"]["left"]), (corner.right, &case["expected"]["right"])] {
            match (got, want.is_null()) {
                (None, true) => {}
                (Some(g), false) => near(g, pt(want), 1e-12, name),
                other => panic!("{name}: {other:?}"),
            }
        }
    }
}

#[test]
fn trimmed_band_follows_the_corner_join() {
    let prev = BulgeSeg::line(Point::new(0.0, 0.0), Point::new(4.0, 0.0));
    let next = BulgeSeg::line(Point::new(4.0, 0.0), Point::new(4.0, 4.0));
    let corner = corner_join(&prev, (0.1, 0.1), &next, (0.1, 0.1));
    let band = band_loop(&prev, 0.1, 0.1, None, Some((corner.left.unwrap(), corner.right.unwrap()))).unwrap();
    near(band[1].0, Point::new(4.1, -0.1), 1e-12, "right end");
    near(band[2].0, Point::new(3.9, 0.1), 1e-12, "left end");
}

fn band_area(band: &[(Point, f64); 4]) -> f64 {
    (0..4)
        .map(|i| {
            let s = BulgeSeg::new(band[i].0, band[(i + 1) % 4].0, band[i].1);
            0.5 * (s.start.x * s.end.y - s.end.x * s.start.y) + s.segment_area()
        })
        .sum()
}

#[test]
fn mitered_wall_network_tiles_the_union_without_overlap() {
    let a = BulgeSeg::line(Point::new(0.0, 0.0), Point::new(4.0, 0.0));
    let b = BulgeSeg::line(Point::new(4.0, 0.0), Point::new(4.0, 4.0));
    let corner = corner_join(&a, (0.1, 0.1), &b, (0.1, 0.1));
    let (left, right) = (corner.left.unwrap(), corner.right.unwrap());
    let first = band_loop(&a, 0.1, 0.1, None, Some((left, right))).unwrap();
    let second = band_loop(&b, 0.1, 0.1, Some((left, right)), None).unwrap();
    close(band_area(&first), 0.8, 1e-12, "first wall");
    close(band_area(&second), 0.8, 1e-12, "second wall");
    close(band_area(&first) + band_area(&second), 0.82 + 0.82 - 0.04, 1e-12, "tiling equals the union of the square-cut strips");
}

#[test]
fn t_join_trims_the_branch_to_the_through_wall_face() {
    let through = BulgeSeg::line(Point::new(0.0, 0.0), Point::new(6.0, 0.0));
    let branch = BulgeSeg::line(Point::new(3.0, 3.0), Point::new(3.0, 0.0));
    let face = through.offset(0.1).unwrap();
    let left = nearest_intersection(&branch.offset(0.1).unwrap(), &face, Extent::Unbounded, branch.end).unwrap();
    let right = nearest_intersection(&branch.offset(-0.1).unwrap(), &face, Extent::Unbounded, branch.end).unwrap();
    near(left, Point::new(3.1, 0.1), 1e-12, "left face end");
    near(right, Point::new(2.9, 0.1), 1e-12, "right face end");
    let band = band_loop(&branch, 0.1, 0.1, None, Some((left, right))).unwrap();
    close(band_area(&band), 0.2 * 2.9, 1e-12, "trimmed branch");
}

#[test]
fn curved_walls_join_with_exact_arcs() {
    let arc = BulgeSeg::from_arc(Point::ZERO, 5.0, 0.0, std::f64::consts::FRAC_PI_2);
    let tail = BulgeSeg::line(arc.end, Point::new(arc.end.x - 3.0, arc.end.y));
    let corner = corner_join(&arc, (0.1, 0.1), &tail, (0.1, 0.1));
    let band = band_loop(&arc, 0.1, 0.1, None, Some((corner.left.unwrap(), corner.right.unwrap()))).unwrap();
    near(band[1].0, corner.right.unwrap(), 1e-12, "outer end");
    near(band[2].0, corner.left.unwrap(), 1e-12, "inner end");
    let expected = 0.2 * 5.0 * std::f64::consts::FRAC_PI_2;
    close(band_area(&band), expected, 1e-9, "tangent continuation keeps the quarter annulus area");
}
