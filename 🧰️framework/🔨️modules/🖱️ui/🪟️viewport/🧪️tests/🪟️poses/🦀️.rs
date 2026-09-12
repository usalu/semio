use super::*;
use protocol::value::{Number, ToValue};

#[derive(Debug, PartialEq, serde::Deserialize)]
struct PlanarOracle { x: f64, y: f64, zoom: f64 }
#[derive(Debug, PartialEq, serde::Deserialize)]
struct OrbitOracle { position: [f64; 3], target: [f64; 3], zoom: f64, up: Option<[f64; 3]> }

#[test]
fn viewport_ownership_neutral_admission_and_serde_round_trip() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🪟️poses/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let value = DslValue::from(row["value"].clone());
        let valid = row["valid"].as_bool().unwrap();
        if row["dimension"] == "2d" {
            let decoded = Viewport2d::from_value(value);
            let serde_decoded = serde_json::from_value::<Viewport2d>(row["value"].clone());
            assert_eq!(decoded.is_ok(), valid, "{}", row["name"]);
            assert_eq!(serde_decoded.is_ok(), valid, "{} serde", row["name"]);
            if let Ok(pose) = decoded {
                assert_eq!(serde_decoded.unwrap(), pose);
                let expected: PlanarOracle = serde_json::from_value(row["value"].clone()).unwrap();
                assert_eq!(PlanarOracle { x: pose.x, y: pose.y, zoom: pose.zoom }, expected);
                let encoded: PlanarOracle = serde_json::from_value(pose.to_value().into()).unwrap();
                assert_eq!(encoded, expected);
                let serde_value = serde_json::to_value(pose).unwrap();
                assert_eq!(serde_json::from_value::<PlanarOracle>(serde_value.clone()).unwrap(), expected);
                assert_eq!(serde_json::from_value::<Viewport2d>(serde_value).unwrap(), pose);
                assert_eq!(Viewport2d::from_value(pose.to_value()).unwrap(), pose);
            }
        } else {
            let decoded = Viewport3dOrbit::from_value(value);
            let serde_decoded = serde_json::from_value::<Viewport3dOrbit>(row["value"].clone());
            assert_eq!(decoded.is_ok(), valid, "{}", row["name"]);
            assert_eq!(serde_decoded.is_ok(), valid, "{} serde", row["name"]);
            if let Ok(pose) = decoded {
                assert_eq!(serde_decoded.unwrap(), pose);
                let expected: OrbitOracle = serde_json::from_value(row["value"].clone()).unwrap();
                assert_eq!(OrbitOracle { position: pose.position, target: pose.target, zoom: pose.zoom, up: pose.up }, expected);
                let encoded: OrbitOracle = serde_json::from_value(pose.to_value().into()).unwrap();
                assert_eq!(encoded, expected);
                let serde_value = serde_json::to_value(pose).unwrap();
                assert_eq!(serde_json::from_value::<OrbitOracle>(serde_value.clone()).unwrap(), expected);
                assert_eq!(serde_json::from_value::<Viewport3dOrbit>(serde_value).unwrap(), pose);
                assert_eq!(Viewport3dOrbit::from_value(pose.to_value()).unwrap(), pose);
            }
        }
    }
    eprintln!("[DEBUG] Shared viewport native admission matched all 20 neutral cases through Serde, FromValue, and independent Serde values");
}

#[test]
fn viewport_ownership_rejects_nonfinite_and_duplicate_fields() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        for field in ["x", "y", "zoom"] {
            let DslValue::Object(mut entries) = Viewport2d::default().to_value() else { unreachable!() };
            entries.iter_mut().find(|(name, _)| name == field).unwrap().1 = DslValue::Number(Number::Float(value));
            assert!(Viewport2d::from_value(DslValue::Object(entries)).is_err());
            let mut pose = Viewport2d::default();
            match field { "x" => pose.x = value, "y" => pose.y = value, _ => pose.zoom = value }
            assert!(serde_json::from_value::<Viewport2d>(serde_json::to_value(pose).unwrap()).is_err());
        }
        for field in ["position", "target", "up"] {
            let mut pose = Viewport3dOrbit { position: [8.0, -3.0, 5.0], target: [0.0; 3], zoom: 1.25, up: Some([0.0, 0.0, 1.0]) };
            match field { "position" => pose.position[0] = value, "target" => pose.target[1] = value, _ => pose.up.as_mut().unwrap()[2] = value }
            assert!(pose.validate().is_err());
            assert!(Viewport3dOrbit::from_value(pose.to_value()).is_err());
            assert!(serde_json::from_value::<Viewport3dOrbit>(serde_json::to_value(pose).unwrap()).is_err());
        }
        let pose = Viewport3dOrbit { position: [8.0, -3.0, 5.0], target: [0.0; 3], zoom: value, up: None };
        assert!(Viewport3dOrbit::from_value(pose.to_value()).is_err());
        assert!(serde_json::from_value::<Viewport3dOrbit>(serde_json::to_value(pose).unwrap()).is_err());
    }
    let duplicate = DslValue::Object(vec![("x".into(), 0.0.to_value()), ("x".into(), 1.0.to_value()), ("zoom".into(), 1.0.to_value())]);
    assert!(Viewport2d::from_value(duplicate).is_err());
    assert!(serde_json::from_str::<Viewport2d>(r#"{"x":0,"x":1,"y":0,"zoom":1}"#).is_err());
    assert!(serde_json::from_str::<Viewport3dOrbit>(r#"{"position":[0,0,1],"position":[1,0,0],"target":[0,0,0],"zoom":1}"#).is_err());
    eprintln!("[DEBUG] Shared viewport native decode rejected nonfinite fields and duplicate field authority");
}
