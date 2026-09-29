//! 📐️ The shared framing corpus across renderer implementations.
use super::*;

#[test]
fn canvas_framing_shared_fixtures() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let request: Canvas2dFraming = serde_json::from_value(case["request"].clone()).unwrap();
        let result = request.fit(case["viewport"][0].as_f64().unwrap(),case["viewport"][1].as_f64().unwrap());
        if case["expected"].is_null() { assert!(result.is_none(),"{}",case["name"]); continue; }
        let camera = result.unwrap_or_else(|| panic!("missing fit {}",case["name"]));
        for (actual,expected) in [camera.x,camera.y,camera.zoom].iter().zip(case["expected"].as_array().unwrap()) {
            assert!((actual-expected.as_f64().unwrap()).abs()<1e-10,"{}: {camera:?}",case["name"]);
        }
    }
}

#[test]
fn canvas_framing_rejects_nonfinite_geometry() {
    for invalid in [f64::NAN,f64::INFINITY,f64::NEG_INFINITY] {
        let request = Canvas2dFraming { revision: 1,bounds: [0.0,0.0,100.0,invalid],padding: 40.0 };
        assert!(request.fit(800.0,600.0).is_none());
        let request = Canvas2dFraming { bounds: [0.0,0.0,100.0,100.0],..request };
        assert!(request.fit(invalid,600.0).is_none());
    }
}
