//! 🧪️ Shared coordinate frame vectors and world placement invariance.
use super::*;
use serde_json::Value;

fn affine(value:&Value)->CompositeAffine {std::array::from_fn(|i|value[i].as_f64().unwrap())}
fn close(actual:CompositeAffine,expected:CompositeAffine) {
    for (a,b) in actual.into_iter().zip(expected) {assert!((a-b).abs()<1e-10,"{a} != {b}");}
}

#[test]
fn neutral_frames_preserve_world_placement_and_round_trip() {
    let fixture:Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let input=&row["input"];
        let source=affine(&input["source"]);let target=affine(&input["target"]);let transform=affine(&input["transform"]);
        let result=reframe(transform,source,target).unwrap();
        close(result,affine(&row["expected"]));
        close(reframe(result,target,source).unwrap(),transform);
        close(multiply(target,result),multiply(source,transform));
        for point in fixture["points"].as_array().unwrap() {
            let translation=[1.0,0.0,0.0,1.0,point[0].as_f64().unwrap(),point[1].as_f64().unwrap()];
            close(multiply(multiply(target,result),translation),multiply(multiply(source,transform),translation));
        }
    }
}

#[test]
fn frame_changes_reject_unsafe_geometry() {
    let identity=[1.0,0.0,0.0,1.0,0.0,0.0];
    let fixture:Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let invalid=fixture["invalid"].as_array().unwrap().iter().map(affine).chain([
        [f64::INFINITY,0.0,0.0,1.0,0.0,0.0], [1.0,0.0,0.0,1.0,f64::NAN,0.0]
    ]);
    for value in invalid {
        assert!(reframe(value,identity,identity).is_err());
        assert!(reframe(identity,value,identity).is_err());
        assert!(reframe(identity,identity,value).is_err());
    }
    assert!(reframe([1e154,0.0,0.0,1.0,0.0,0.0],[1e155,0.0,0.0,1.0,0.0,0.0],identity).is_err());
}

#[test]
fn affine_controls_match_neutral_matrices_and_preserve_reflections() {
    let fixture:Value=serde_json::from_str(include_str!("../🧫️fixtures/🎛️components.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let t=&row["input"];
        let controls=AffineControls {x:t["x"].as_f64().unwrap(),y:t["y"].as_f64().unwrap(),scale_x:t["scaleX"].as_f64().unwrap(),scale_y:t["scaleY"].as_f64().unwrap(),rotation:t["rotation"].as_f64().unwrap(),shear_x:t["shearX"].as_f64().unwrap()};
        let result=compose(controls).unwrap();close(result,affine(&row["expected"]));
        let components=decompose(result).unwrap();assert!(components.scale_x>0.0);
        close(compose(components).unwrap(),result);
    }
    let frames:Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in frames["cases"].as_array().unwrap() {
        let matrix=affine(&row["expected"]);close(compose(decompose(matrix).unwrap()).unwrap(),matrix);
    }
}

#[test]
fn affine_controls_reject_nonfinite_and_singular_inputs() {
    let good=AffineControls {x:0.0,y:0.0,scale_x:1.0,scale_y:1.0,rotation:0.0,shear_x:0.0};
    for bad in [AffineControls {x:f64::NAN,..good},AffineControls {y:f64::INFINITY,..good},AffineControls {scale_x:0.0,..good},AffineControls {scale_y:1e-13,..good},AffineControls {rotation:f64::NAN,..good},AffineControls {shear_x:f64::INFINITY,..good}] {
        assert!(compose(bad).is_err());
    }
    let frames:Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in frames["invalid"].as_array().unwrap() {assert!(decompose(affine(row)).is_err());}
}
