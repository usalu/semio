use super::*;
use serde_json::Value;

fn fixtures() -> Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/➰️loops/🔣️.json")).unwrap()
}

fn vertices(v: &Value) -> Vec<Vertex> {
    v.as_array().unwrap().iter().map(|x| Vertex::new(Point::new(x[0].as_f64().unwrap(), x[1].as_f64().unwrap()), x[2].as_f64().unwrap())).collect()
}

fn close(a: f64, b: f64, eps: f64, context: &str) {
    assert!((a - b).abs() <= eps, "{context}: {a} vs {b}");
}

#[test]
fn measures_match_closed_form_fixtures() {
    for case in fixtures()["loops"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let v = vertices(&case["vertices"]);
        let e = &case["expected"];
        close(signed_area(&v), e["signed_area"].as_f64().unwrap(), 1e-12, name);
        close(area(&v), e["area"].as_f64().unwrap(), 1e-12, name);
        close(perimeter(&v), e["perimeter"].as_f64().unwrap(), 1e-12, name);
        assert_eq!(is_ccw(&v), e["ccw"].as_bool().unwrap(), "{name}");
        let c = centroid(&v);
        close(c.x, e["centroid"][0].as_f64().unwrap(), 1e-12, name);
        close(c.y, e["centroid"][1].as_f64().unwrap(), 1e-12, name);
        let b = bounds(&v).unwrap();
        for (got, want) in [b.x0(), b.y0(), b.x1(), b.y1()].into_iter().zip(e["bounds"].as_array().unwrap()) {
            close(got, want.as_f64().unwrap(), 1e-12, name);
        }
    }
}

#[test]
fn containment_matches_fixtures() {
    for case in fixtures()["loops"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let v = vertices(&case["vertices"]);
        for probe in case["contains"].as_array().unwrap() {
            let p = Point::new(probe["point"][0].as_f64().unwrap(), probe["point"][1].as_f64().unwrap());
            let want = match probe["at"].as_str().unwrap() {
                "inside" => Containment::Inside,
                "outside" => Containment::Outside,
                _ => Containment::Boundary,
            };
            assert_eq!(locate(&v, p, 1e-9), want, "{name} {p:?}");
            assert_eq!(contains(&v, p), want == Containment::Inside, "{name} {p:?}");
        }
    }
}

#[test]
fn reversed_loop_negates_area_and_keeps_geometry() {
    for case in fixtures()["loops"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let v = vertices(&case["vertices"]);
        let r = reversed(&v);
        close(signed_area(&r), -signed_area(&v), 1e-12, name);
        close(perimeter(&r), perimeter(&v), 1e-12, name);
        let (a, b) = (centroid(&r), centroid(&v));
        close(a.x, b.x, 1e-12, name);
        close(a.y, b.y, 1e-12, name);
        assert!(is_ccw(&ccw(&r)), "{name}");
        assert_eq!(reversed(&r), v, "{name} double reverse");
    }
}

#[test]
fn flattening_converges_to_the_exact_area() {
    for case in fixtures()["loops"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let v = vertices(&case["vertices"]);
        let exact = signed_area(&v);
        for tolerance in [1e-2, 1e-4] {
            let flat = flatten(&v, tolerance);
            let polygon = crate::polygon_area(&flat);
            let curved: f64 = v.iter().map(|x| x.bulge.abs()).sum();
            let allowed = if curved > 0.0 { perimeter(&v) * tolerance } else { 1e-12 };
            assert!((polygon - exact).abs() <= allowed + 1e-12, "{name} tol {tolerance}: {polygon} vs {exact}");
        }
    }
}

#[test]
fn offsets_match_fixtures() {
    for case in fixtures()["offsets"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let v = vertices(&case["vertices"]);
        let got = offset(&v, case["distance"].as_f64().unwrap(), case["miter_limit"].as_f64().unwrap());
        match (got, case["expected_area"].is_null()) {
            (None, true) => {}
            (Some(o), false) => {
                close(area(&o), case["expected_area"].as_f64().unwrap(), 1e-9, name);
                close(perimeter(&o), case["expected_perimeter"].as_f64().unwrap(), 1e-9, name);
                assert!(self_intersections(&o).is_empty(), "{name}");
            }
            other => panic!("{name}: {other:?}"),
        }
    }
}

#[test]
fn sharp_corners_are_bevelled_beyond_the_miter_limit() {
    let spike = from_polygon(&[Point::new(0.0, 0.0), Point::new(10.0, 0.0), Point::new(0.0, 0.5)]);
    let mitered = offset(&spike, 0.1, 100.0).unwrap();
    let bevelled = offset(&spike, 0.1, 2.0).unwrap();
    assert!(bevelled.len() > mitered.len());
    assert!(area(&bevelled) < area(&mitered));
}

#[test]
fn transform_scales_area_and_mirror_keeps_arc_geometry() {
    let v = vertices(&fixtures()["loops"][2]["vertices"]);
    let scaled = transformed(&v, Affine::IDENTITY.scale(3.0));
    close(area(&scaled), area(&v) * 9.0, 1e-9, "area scales quadratically");
    let mirrored = transformed(&v, Affine::new([-1.0, 0.0, 0.0, 1.0, 0.0, 0.0]));
    close(signed_area(&mirrored), -signed_area(&v), 1e-12, "mirror flips orientation");
    close(perimeter(&mirrored), perimeter(&v), 1e-12, "mirror keeps perimeter");
}

#[test]
fn self_intersections_flag_bow_ties() {
    let bow = from_polygon(&[Point::new(0.0, 0.0), Point::new(2.0, 2.0), Point::new(2.0, 0.0), Point::new(0.0, 2.0)]);
    assert_eq!(self_intersections(&bow).len(), 1);
    let square = from_polygon(&[Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(0.0, 1.0)]);
    assert!(self_intersections(&square).is_empty());
}
