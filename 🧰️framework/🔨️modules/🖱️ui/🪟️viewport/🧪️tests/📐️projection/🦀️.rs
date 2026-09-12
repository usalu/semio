use super::*;
use protocol::value::{DslValue, FromValue, Number, ToValue};

fn patched(base: &serde_json::Value, patch: &serde_json::Value) -> serde_json::Value {
    let mut value = base.clone();
    let object = value.as_object_mut().unwrap();
    for operation in patch.as_array().unwrap() {
        let path = operation["path"].as_str().unwrap().strip_prefix('/').unwrap();
        match operation["op"].as_str().unwrap() {
            "remove" => { object.remove(path); }
            "add" | "replace" => { object.insert(path.into(), operation["value"].clone()); }
            operation => panic!("unsupported fixture patch {operation}"),
        }
    }
    value
}

fn json_equal(left: &serde_json::Value, right: &serde_json::Value) -> bool {
    match (left, right) {
        (serde_json::Value::Number(left), serde_json::Value::Number(right)) => left.as_f64() == right.as_f64(),
        (serde_json::Value::Array(left), serde_json::Value::Array(right)) => left.len() == right.len() && left.iter().zip(right).all(|(left, right)| json_equal(left, right)),
        (serde_json::Value::Object(left), serde_json::Value::Object(right)) => left.len() == right.len() && left.iter().all(|(key, left)| right.get(key).is_some_and(|right| json_equal(left, right))),
        _ => left == right,
    }
}

#[test]
fn viewport_projection_corpus_matches_serde_value_and_derivation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📐️projection/🔣️.json")).unwrap();
    let default_json = fixture["defaultPreferences"].clone();
    let defaults: Viewport3dProjectionPreferences = serde_json::from_value(default_json.clone()).unwrap();
    assert_eq!(defaults, Viewport3dProjectionPreferences::default());
    assert!(json_equal(&serde_json::to_value(defaults).unwrap(), &default_json));
    assert_eq!(Viewport3dProjectionPreferences::from_value(defaults.to_value()).unwrap(), defaults);

    let mut combinations = 0;
    for mode in fixture["modes"].as_array().unwrap() {
        for orientation in fixture["orientations"].as_array().unwrap() {
            let json = serde_json::json!({ "mode": mode, "orientation": orientation });
            let serde_spec: Viewport3dProjectionSpec = serde_json::from_value(json.clone()).unwrap();
            let value_spec = Viewport3dProjectionSpec::from_value(DslValue::from(json.clone())).unwrap();
            assert_eq!(serde_spec, value_spec);
            assert!(json_equal(&serde_json::to_value(serde_spec).unwrap(), &json));
            assert_eq!(Viewport3dProjectionSpec::from_value(serde_spec.to_value()).unwrap(), serde_spec);
            combinations += 1;
        }
    }

    for row in fixture["derivations"].as_array().unwrap() {
        let preferences_json = patched(&default_json, &row["patch"]);
        let preferences: Viewport3dProjectionPreferences = serde_json::from_value(preferences_json.clone()).unwrap();
        let expected: Viewport3dProjectionSpec = serde_json::from_value(row["expected"].clone()).unwrap();
        assert_eq!(derive_active_projection(&preferences), expected, "{}", row["name"]);
        assert_eq!(Viewport3dProjectionPreferences::from_value(DslValue::from(preferences_json)).unwrap(), preferences);
    }
    eprintln!("[DEBUG] Shared viewport projection native codecs matched {combinations} mode-orientation pairs and {} pure derivations", fixture["derivations"].as_array().unwrap().len());
}

#[test]
fn viewport_projection_rejects_closed_schema_and_range_violations() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📐️projection/🔣️.json")).unwrap();
    let defaults = &fixture["defaultPreferences"];
    for row in fixture["preferenceRejections"].as_array().unwrap() {
        let invalid = patched(defaults, &row["patch"]);
        assert!(serde_json::from_value::<Viewport3dProjectionPreferences>(invalid.clone()).is_err(), "{} serde", row["name"]);
        assert!(Viewport3dProjectionPreferences::from_value(DslValue::from(invalid)).is_err(), "{} value", row["name"]);
    }
    for invalid in fixture["specRejections"].as_array().unwrap() {
        assert!(serde_json::from_value::<Viewport3dProjectionSpec>(invalid.clone()).is_err(), "{invalid}");
        assert!(Viewport3dProjectionSpec::from_value(DslValue::from(invalid.clone())).is_err(), "{invalid}");
    }

    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut preferences = Viewport3dProjectionPreferences::default();
        for field in 0..8 {
            let slot = match field {
                0 => &mut preferences.axonometric_angle_a,
                1 => &mut preferences.axonometric_angle_b,
                2 => &mut preferences.oblique_angle,
                3 => &mut preferences.oblique_depth,
                4 => &mut preferences.fov,
                5 => &mut preferences.two_point_shift,
                6 => &mut preferences.curvilinear_fov,
                _ => &mut preferences.curvilinear_strength,
            };
            *slot = value;
            assert!(preferences.validate().is_err());
            assert!(Viewport3dProjectionPreferences::from_value(preferences.to_value()).is_err());
            preferences = Viewport3dProjectionPreferences::default();
        }
        for mode in [
            Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Trimetric, angle_a: value, angle_b: 13.0 },
            Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Trimetric, angle_a: 22.0, angle_b: value },
            Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Cabinet, angle: value, depth_scale: 0.5 },
            Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Cabinet, angle: 45.0, depth_scale: value },
            Viewport3dProjectionMode::OnePoint { fov: value },
            Viewport3dProjectionMode::TwoPoint { fov: value, vertical_shift: 0.0 },
            Viewport3dProjectionMode::TwoPoint { fov: 50.0, vertical_shift: value },
            Viewport3dProjectionMode::ThreePoint { fov: value },
            Viewport3dProjectionMode::Curvilinear { fov: value, strength: 1.0, mapping: Viewport3dCurvilinearMapping::Fisheye },
            Viewport3dProjectionMode::Curvilinear { fov: 120.0, strength: value, mapping: Viewport3dCurvilinearMapping::Fisheye },
        ] {
            assert!(mode.validate().is_err());
            assert!(Viewport3dProjectionMode::from_value(mode.to_value()).is_err());
        }
    }

    assert!(serde_json::from_str::<Viewport3dProjectionPreferences>(r#"{"kind":"threePoint","kind":"onePoint","orthographicView":"top","axonometricVariant":"isometric","axonometricAngleA":15,"axonometricAngleB":12,"axonometricQuadrant":"ne","obliqueVariant":"cavalier","obliqueAngle":45,"obliqueDepth":1,"onePointAxis":"y","fov":50,"twoPointShift":0,"curvilinearFov":120,"curvilinearStrength":1,"curvilinearMapping":"fisheye"}"#).is_err());
    let duplicate = DslValue::Object(vec![("kind".into(), "orthographic".to_value()), ("kind".into(), "threePoint".to_value())]);
    assert!(Viewport3dProjectionMode::from_value(duplicate).is_err());
    let explicit_null = DslValue::Object(vec![
        ("mode".into(), Viewport3dProjectionMode::ThreePoint { fov: 50.0 }.to_value()),
        ("orientation".into(), DslValue::Object(vec![("type".into(), "corner".to_value()), ("quadrant".into(), "ne".to_value()), ("hemisphere".into(), DslValue::Null)])),
    ]);
    assert!(Viewport3dProjectionSpec::from_value(explicit_null).is_err());
    assert!(Viewport3dProjectionMode::from_value(DslValue::Object(vec![("kind".into(), "threePoint".to_value()), ("fov".into(), DslValue::Number(Number::Float(f64::NAN)))])).is_err());
    eprintln!("[DEBUG] Shared viewport projection native codecs rejected unknown, missing, duplicate, null, nonfinite, and out-of-range fields");
}

#[test]
fn viewport_projection_retains_inactive_preferences_across_selection() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📐️projection/🔣️.json")).unwrap();
    let edited = patched(&fixture["defaultPreferences"], &fixture["retention"]["editWhileOrthographic"]);
    let retained: Viewport3dProjectionPreferences = serde_json::from_value(edited.clone()).unwrap();
    assert_eq!(retained.kind, Viewport3dProjectionKind::Orthographic);
    assert_eq!((retained.curvilinear_fov, retained.curvilinear_strength, retained.curvilinear_mapping), (150.0, 0.4, Viewport3dCurvilinearMapping::Panini));
    let recalled: Viewport3dProjectionPreferences = serde_json::from_value(patched(&edited, &fixture["retention"]["recallCurvilinear"])).unwrap();
    let expected: Viewport3dProjectionSpec = serde_json::from_value(fixture["retention"]["expected"].clone()).unwrap();
    assert_eq!(derive_active_projection(&recalled), expected);
    eprintln!("[DEBUG] Shared viewport projection retained all inactive curvilinear preferences through orthographic selection and recall");
}
