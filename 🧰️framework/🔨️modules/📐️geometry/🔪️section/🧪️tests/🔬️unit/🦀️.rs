use super::*;
use crate::mesh::extrude;
use crate::placement::ZPlane;
use crate::polygon_area;
use serde_json::Value;

fn fixtures() -> Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/🔪️section/🔣️.json")).unwrap()
}

fn ring(v: &Value) -> Vec<Point> {
    v.as_array().unwrap().iter().map(|p| Point::new(p[0].as_f64().unwrap(), p[1].as_f64().unwrap())).collect()
}

#[test]
fn plan_cuts_match_closed_form_fixtures() {
    for case in fixtures().as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let holes: Vec<Vec<Point>> = case["holes"].as_array().unwrap().iter().map(ring).collect();
        let mesh = extrude(&ring(&case["outer"]), &holes, ZPlane::flat(case["bottom"].as_f64().unwrap()), ZPlane::flat(case["top"].as_f64().unwrap()));
        let segments = section_z(&mesh, case["z"].as_f64().unwrap(), 1e-9);
        let mut chains = chain(&segments, 1e-9);
        chains.sort_by(|a, b| polygon_area(&b.points).total_cmp(&polygon_area(&a.points)));
        let expected = case["expected"].as_array().unwrap();
        assert_eq!(chains.len(), expected.len(), "{name}: chain count");
        for (got, want) in chains.iter().zip(expected) {
            assert_eq!(got.closed, want["closed"].as_bool().unwrap(), "{name}");
            assert_eq!(got.points.len(), want["points"].as_u64().unwrap() as usize, "{name}");
            assert!((polygon_area(&got.points) - want["area"].as_f64().unwrap()).abs() < 1e-9, "{name}: area {}", polygon_area(&got.points));
        }
    }
}

#[test]
fn vertical_sections_use_plane_coordinates_with_the_normal_towards_the_viewer() {
    let mesh = extrude(&[Point::new(0.0, 0.0), Point::new(4.0, 0.0), Point::new(4.0, 2.0), Point::new(0.0, 2.0)], &[], ZPlane::flat(0.0), ZPlane::flat(3.0));
    let segments = section_plane(&mesh, [0.0, 1.0, 0.0], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], 1e-9);
    let chains = chain(&segments, 1e-9);
    assert_eq!(chains.len(), 1);
    let area = polygon_area(&chains[0].points);
    assert!((area - 12.0).abs() < 1e-9, "elevation cut is the 4 x 3 rectangle with the material on the left, area {area}");
    let (min_x, max_x) = chains[0].points.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| (lo.min(p.x), hi.max(p.x)));
    assert!((min_x, max_x) == (0.0, 4.0));
    let (min_v, max_v) = chains[0].points.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| (lo.min(p.y), hi.max(p.y)));
    assert!((min_v, max_v) == (0.0, 3.0), "v runs up the wall");
}

#[test]
fn chain_reports_open_chains_and_ignores_zero_length_segments() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(1.0, 0.0);
    let c = Point::new(1.0, 1.0);
    let open = chain(&[[b, c], [a, b], [c, c]], 1e-9);
    assert_eq!(open.len(), 1);
    assert!(!open[0].closed);
    assert_eq!(open[0].points, vec![a, b, c]);
    let snapped = chain(&[[a, b], [Point::new(1.0 + 1e-12, 1e-12), c], [c, a]], 1e-9);
    assert_eq!(snapped.len(), 1);
    assert!(snapped[0].closed);
    assert!(chain(&[], 1e-9).is_empty());
}
