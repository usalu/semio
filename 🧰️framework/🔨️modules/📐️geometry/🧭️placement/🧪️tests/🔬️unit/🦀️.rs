use super::*;
use serde_json::Value;

fn fixtures() -> Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/🧭️placement/🔣️.json")).unwrap()
}

fn xyz(v: &Value) -> Xyz {
    [v[0].as_f64().unwrap(), v[1].as_f64().unwrap(), v[2].as_f64().unwrap()]
}

fn near(a: Xyz, b: Xyz, context: &str) {
    assert!(length3(sub3(a, b)) <= 1e-12, "{context}: {a:?} vs {b:?}");
}

use crate::vector::{length3, sub3};

#[test]
fn placements_match_closed_form_fixtures() {
    for case in fixtures()["placements"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mut m = Affine3::IDENTITY;
        for op in case["ops"].as_array().unwrap() {
            let step = if let Some(angle) = op.get("rotation_z") {
                Affine3::rotation_z(angle.as_f64().unwrap())
            } else if let Some(t) = op.get("translation") {
                Affine3::translation(xyz(t))
            } else if let Some(s) = op.get("scaling") {
                Affine3::scaling(xyz(s))
            } else {
                Affine3::rotation_axis(xyz(&op["rotation_axis"]), op["angle"].as_f64().unwrap())
            };
            m = m.then(&step);
        }
        near(m.apply_point(xyz(&case["point"])), xyz(&case["expected"]), name);
    }
}

#[test]
fn vectors_ignore_translation_and_normals_survive_mirrors() {
    let m = Affine3::rotation_z(std::f64::consts::FRAC_PI_2).then(&Affine3::translation([5.0, 5.0, 5.0]));
    near(m.apply_vector([1.0, 0.0, 0.0]), [0.0, 1.0, 0.0], "vector");
    near(m.apply_normal([1.0, 0.0, 0.0]), [0.0, 1.0, 0.0], "normal under rotation");
    let mirror = Affine3::scaling([-1.0, 1.0, 1.0]);
    near(mirror.apply_normal([1.0, 0.0, 0.0]), [-1.0, 0.0, 0.0], "normal under mirror");
    assert!(mirror.determinant() < 0.0);
    assert!((m.determinant() - 1.0).abs() < 1e-12);
    let stretched = Affine3::scaling([2.0, 1.0, 1.0]);
    near(stretched.apply_normal([1.0, 1.0, 0.0]), normalize3([0.5, 1.0, 0.0]), "inverse transpose");
}

#[test]
fn frame_maps_local_axes() {
    let f = Affine3::from_frame([1.0, 2.0, 3.0], [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    near(f.apply_point([1.0, 0.0, 0.0]), [1.0, 3.0, 3.0], "x axis");
    near(f.apply_point([0.0, 1.0, 0.0]), [0.0, 2.0, 3.0], "y axis");
}

#[test]
fn z_planes_match_fixtures() {
    for case in fixtures()["z_planes"].as_array().unwrap() {
        let anchor = Point::new(case["anchor"][0].as_f64().unwrap(), case["anchor"][1].as_f64().unwrap());
        let plane = ZPlane::sloped(anchor, case["z"].as_f64().unwrap(), case["direction"].as_f64().unwrap(), case["slope"].as_f64().unwrap());
        for q in case["queries"].as_array().unwrap() {
            let p = Point::new(q["point"][0].as_f64().unwrap(), q["point"][1].as_f64().unwrap());
            assert!((plane.at(p) - q["z"].as_f64().unwrap()).abs() < 1e-12);
        }
        assert!((plane.raised(0.5).at(anchor) - case["z"].as_f64().unwrap() - 0.5).abs() < 1e-12);
    }
    assert_eq!(ZPlane::flat(2.0).at(Point::new(9.0, 9.0)), 2.0);
}
