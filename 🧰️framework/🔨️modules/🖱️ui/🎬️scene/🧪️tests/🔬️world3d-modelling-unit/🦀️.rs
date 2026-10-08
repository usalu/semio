use super::*;
use crate::{scene_lane_hash, SceneDoc, World3dSceneLane};
use serde_json::Value;

const FIXTURE: &str = include_str!("../../🧫️fixtures/📏️world3d-modelling/🔣️.json");

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("modelling fixture parses")
}

fn json_eq(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => (a.as_f64().unwrap() - b.as_f64().unwrap()).abs() <= 1e-12,
        (Value::Array(a), Value::Array(b)) => a.len() == b.len() && a.iter().zip(b).all(|(x, y)| json_eq(x, y)),
        (Value::Object(a), Value::Object(b)) => a.len() == b.len() && a.iter().all(|(key, x)| b.get(key).is_some_and(|y| json_eq(x, y))),
        _ => left == right,
    }
}

fn decode<T: FromValue>(value: &Value) -> Result<T, ValueError> {
    world3d_modelling_from_lane_text(&serde_json::to_string(value).unwrap())
}

fn check_valid<T: FromValue + ToValue + Serialize + PartialEq + std::fmt::Debug>(name: &str, value: &Value, normalized: &Value) {
    let typed: T = decode(value).unwrap_or_else(|error| panic!("{name} must decode: {error}"));
    let shape = serde_json::to_value(&typed).unwrap();
    assert!(json_eq(&shape, normalized), "{name}: serde shape {shape} differs from normalized {normalized}");
    let again: T = world3d_modelling_from_lane_text(&world3d_modelling_lane_text(&typed)).unwrap_or_else(|error| panic!("{name} lane text must decode back: {error}"));
    assert_eq!(again, typed, "{name} lane text round trip");
}

#[test]
fn valid_fixtures_decode_to_their_normalized_shape_and_round_trip_through_lane_text() {
    for entry in fixture()["valid"].as_array().unwrap() {
        let (name, value, normalized) = (entry["name"].as_str().unwrap(), &entry["value"], &entry["normalized"]);
        match entry["def"].as_str().unwrap() {
            "annotationLayer" => check_valid::<World3dAnnotationLayer>(name, value, normalized),
            "scalarField" => check_valid::<World3dScalarField>(name, value, normalized),
            "modellingOptions" => check_valid::<World3dModellingOptions>(name, value, normalized),
            other => panic!("unknown def {other}"),
        }
    }
}

#[test]
fn invalid_fixtures_are_refused_by_the_lane_codec_schema_level_and_semantic() {
    for entry in fixture()["invalid"].as_array().unwrap() {
        let (name, value) = (entry["name"].as_str().unwrap(), &entry["value"]);
        let refused = match entry["def"].as_str().unwrap() {
            "annotationLayer" => decode::<World3dAnnotationLayer>(value).is_err(),
            "scalarField" => decode::<World3dScalarField>(value).is_err(),
            "modellingOptions" => decode::<World3dModellingOptions>(value).is_err(),
            other => panic!("unknown def {other}"),
        };
        assert!(refused, "{name} must be refused: {}", entry["reason"]);
    }
    let marker = World3dAnnotation::marker("m", [0.0; 3], World3dText::new("Point", "Punkt"));
    let layer = |count: usize| World3dAnnotationLayer::new((0..count).map(|index| World3dAnnotation::marker(format!("m{index}"), [0.0; 3], World3dText::new("Point", "Punkt"))).collect());
    assert!(layer(WORLD3D_ANNOTATIONS_MAX).validate().is_ok());
    assert!(layer(WORLD3D_ANNOTATIONS_MAX + 1).validate().is_err());
    assert!(World3dAnnotationLayer::new(vec![marker.clone(), marker]).validate().is_err());
    for hostile in ["", "null", "[]", "4", "{\"items\":\"no\"}", "{\"items\":[null]}", "{\"items\":[],\"items\":[]}"] {
        assert!(world3d_modelling_from_lane_text::<World3dAnnotationLayer>(hostile).is_err(), "{hostile}");
    }
}

fn hex(rgb: [u8; 3]) -> String {
    format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2])
}

fn ramp_named(name: &str) -> World3dColorRamp {
    World3dColorRamp::parse(name).expect("fixture ramp")
}

#[test]
fn colour_ramps_interpolate_exactly_as_the_three_js_oracle_in_the_fixture() {
    let fixture = fixture();
    let ramps = fixture["ramps"].as_object().unwrap();
    assert_eq!(ramps.len(), World3dColorRamp::ALL.len());
    for (name, ramp) in ramps {
        let kind = ramp_named(name);
        let stops: Vec<String> = kind.stops().iter().map(|stop| hex(*stop)).collect();
        let expected: Vec<&str> = ramp["stops"].as_array().unwrap().iter().map(|stop| stop.as_str().unwrap()).collect();
        assert_eq!(stops, expected, "{name} stops");
        for sample in ramp["samples"].as_array().unwrap() {
            let t = sample["t"].as_f64().unwrap();
            assert_eq!(hex(kind.sample(t)), sample["hex"].as_str().unwrap(), "{name} @ {t}");
        }
        assert_eq!(kind.sample(-3.0), kind.sample(0.0));
        assert_eq!(kind.sample(7.0), kind.sample(1.0));
        assert_eq!(kind.sample(f64::NAN), kind.sample(0.0));
    }
}

#[test]
fn scalar_fields_colour_values_clamp_to_the_range_and_mark_missing_data() {
    let fixture = fixture();
    assert_eq!(hex(WORLD3D_SCALAR_NO_DATA_RGB), fixture["noDataHex"].as_str().unwrap());
    for entry in fixture["scalarColors"].as_array().unwrap() {
        let field: World3dScalarField = decode(&entry["field"]).unwrap();
        let expected: Vec<&str> = entry["colors"].as_array().unwrap().iter().map(|color| color.as_str().unwrap()).collect();
        let colors: Vec<String> = field.values.iter().map(|value| hex(field.color_of(*value))).collect();
        assert_eq!(colors, expected);
        let bytes = field.color_bytes();
        assert_eq!(bytes.len(), field.values.len() * 3);
        for (index, color) in expected.iter().enumerate() {
            assert_eq!(hex([bytes[index * 3], bytes[index * 3 + 1], bytes[index * 3 + 2]]), *color);
        }
    }
}

#[test]
fn scalar_field_legends_sample_the_ramp_at_evenly_spaced_ticks() {
    for entry in fixture()["legends"].as_array().unwrap() {
        let field: World3dScalarField = decode(&entry["field"]).unwrap();
        let ticks = field.legend_ticks();
        let expected = entry["ticks"].as_array().unwrap();
        assert_eq!(ticks.len(), expected.len());
        assert_eq!(ticks.len(), usize::from(field.legend.ticks));
        for (tick, want) in ticks.iter().zip(expected) {
            assert!((tick.value - want["value"].as_f64().unwrap()).abs() <= 1e-12);
            assert_eq!(hex(tick.rgb), want["hex"].as_str().unwrap());
        }
    }
}

#[test]
fn the_pick_filter_admits_exactly_one_granularity() {
    for row in fixture()["pickTargets"].as_array().unwrap() {
        let filter = World3dPickGranularity::parse(row["filter"].as_str().unwrap()).unwrap();
        let targets = World3dPickGranularity::targets(Some(filter));
        let want = &row["targets"];
        assert_eq!((targets.mesh, targets.face, targets.edge, targets.vertex), (want["mesh"].as_bool().unwrap(), want["face"].as_bool().unwrap(), want["edge"].as_bool().unwrap(), want["vertex"].as_bool().unwrap()), "{filter:?}");
    }
    assert_eq!(World3dPickGranularity::targets(None), World3dPickTargets { mesh: true, face: true, edge: true, vertex: true });
}

#[test]
fn the_section_plane_matches_the_three_js_clipping_convention() {
    for row in fixture()["sectionPlanes"].as_array().unwrap() {
        let options: World3dModellingOptions = decode(&serde_json::json!({ "section": row["section"] })).unwrap();
        let plane = options.section.unwrap().clip_plane();
        for (component, want) in plane.iter().zip(row["plane"].as_array().unwrap()) {
            assert!((component - want.as_f64().unwrap()).abs() <= 1e-12, "{plane:?} vs {}", row["plane"]);
        }
    }
}

#[test]
fn text_resolves_for_the_active_locale_and_never_invents_a_language() {
    let text = World3dText::new("Width", "Breite");
    assert_eq!(text.resolve("de"), "Breite");
    assert_eq!(text.resolve("de-CH"), "Breite");
    assert_eq!(text.resolve("en-GB"), "Width");
    assert_eq!(text.resolve("fr"), "Width");
}

fn modelling_scene() -> World3dScene {
    let scene = &fixture()["laneRoundTrip"]["scene"];
    let mut assembled = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
    assembled.annotations = Some(decode(&scene["annotations"]).unwrap());
    assembled.scalar_field = Some(decode(&scene["scalarField"]).unwrap());
    assembled.modelling_options = Some(decode(&scene["modellingOptions"]).unwrap());
    assembled
}

#[test]
fn the_three_modelling_lanes_split_out_of_the_spine_and_merge_back_as_typed_values() {
    let fixture = fixture();
    let assembled = modelling_scene();
    let (spine, lanes) = assembled.split_lanes();
    assert!(spine.annotations.is_none() && spine.scalar_field.is_none() && spine.modelling_options.is_none());
    let keys: Vec<&str> = lanes.iter().map(|lane| lane.key).collect();
    for key in fixture["laneRoundTrip"]["laneKeys"].as_array().unwrap() {
        assert!(keys.contains(&key.as_str().unwrap()), "{key} must ride its own lane");
    }
    for reference in spine.lanes.iter().filter(|reference| ["annotations", "scalarField", "modellingOptions"].contains(&reference.lane.as_str())) {
        let lane = World3dSceneLane::from_name(&reference.lane).unwrap();
        let payload = &lanes.iter().find(|candidate| candidate.key == lane.body_key()).unwrap().payload;
        assert_eq!(reference.hash, scene_lane_hash(payload));
        assert_eq!(reference.bytes as usize, payload.len());
        assert!(lane.optional());
    }
    let mut merged = spine.clone();
    for lane in &lanes {
        assert!(merged.merge_lane(lane.key, lane.payload.clone()), "{} merges", lane.key);
    }
    merged.lanes = Vec::new();
    assert_eq!(merged, assembled);
}

#[test]
fn a_typed_lane_refuses_an_invalid_payload_and_leaves_the_scene_unchanged() {
    let mut scene = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
    assert!(!scene.merge_lane(World3dSceneLane::Annotations.body_key(), r#"{"items":[{"kind":"radius","id":"r"}]}"#.into()));
    assert!(!scene.merge_lane(World3dSceneLane::ScalarField.body_key(), "{".into()));
    assert!(!scene.merge_lane(World3dSceneLane::ModellingOptions.body_key(), r#"{"pickFilter":"solid"}"#.into()));
    assert!(scene.annotations.is_none() && scene.scalar_field.is_none() && scene.modelling_options.is_none());
    assert!(scene.merge_lane(World3dSceneLane::ModellingOptions.body_key(), r#"{"pickFilter":"vertex"}"#.into()));
    assert_eq!(scene.modelling_options.unwrap().pick_filter, Some(World3dPickGranularity::Vertex));
}

#[test]
fn only_a_changed_modelling_lane_changes_the_spine_manifest() {
    let assembled = modelling_scene();
    let (before, _) = assembled.split_lanes();
    let mut moved = assembled.clone();
    moved.modelling_options = Some(World3dModellingOptions { pick_filter: Some(World3dPickGranularity::Shape), ..Default::default() });
    let (after, _) = moved.split_lanes();
    let changed: Vec<&str> = before.lanes.iter().zip(&after.lanes).filter(|(a, b)| a != b).map(|(a, _)| a.lane.as_str()).collect();
    assert_eq!(changed, vec!["modellingOptions"]);
}

#[test]
fn the_scene_value_codec_and_serde_shape_carry_the_typed_fields() {
    let assembled = modelling_scene();
    let value = assembled.to_value();
    assert_eq!(World3dScene::from_value(value).expect("value round trip"), assembled);
    let shape = serde_json::to_value(&assembled).unwrap();
    assert!(shape["annotations"]["items"].is_array() && shape["scalarField"]["values"].is_array() && shape["modellingOptions"]["section"].is_object());
    let back: World3dScene = serde_json::from_value(shape).unwrap();
    assert_eq!(back, assembled);
    let bare = serde_json::to_value(World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into())).unwrap();
    for key in ["annotations", "scalarField", "modellingOptions"] {
        assert!(bare.get(key).is_none(), "{key} is omitted when unset");
    }
}

#[test]
fn the_typed_lanes_never_ride_the_pack_and_the_split_spine_does() {
    let assembled = modelling_scene();
    assert!(matches!(assembled.encode_pack(), Err(crate::pack::PackError::Unsupported(_))), "an unsplit scene must not silently drop its typed lanes");
    let (spine, lanes) = assembled.split_lanes();
    let mut decoded = World3dScene::decode_pack(&spine.encode_pack().expect("spine encodes")).expect("spine decodes");
    assert_eq!(decoded, spine);
    for lane in &lanes {
        assert!(decoded.merge_lane(lane.key, lane.payload.clone()));
    }
    decoded.lanes = Vec::new();
    assert_eq!(decoded, assembled);
}

#[test]
fn the_builder_validates_what_it_sets() {
    let base = || World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
    let text = || World3dText::new("Width 40 mm", "Breite 40 mm");
    let layer = World3dAnnotationLayer::new(vec![World3dAnnotation::dimension("w", [0.0; 3], [40.0, 0.0, 0.0], [0.0, -8.0, 0.0], text()).with_tone(World3dTone::Primary)]);
    let scene = base()
        .with_annotations(layer.clone())
        .and_then(|scene| scene.with_section(World3dSection::new([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]).capped(World3dTone::Secondary)))
        .and_then(|scene| scene.with_highlight(World3dHighlight { face: Some(World3dSubElementStyle { hover: Some(World3dTone::Info), selected: None, width_px: Some(3.0) }), ..Default::default() }))
        .expect("valid modelling")
        .with_pick_filter(World3dPickGranularity::Face);
    let options = scene.modelling_options.unwrap();
    assert_eq!(options.pick_filter, Some(World3dPickGranularity::Face));
    assert!(options.section.unwrap().cap.is_some() && options.highlight.is_some());
    assert_eq!(scene.annotations, Some(layer));
    assert!(base().with_annotations(World3dAnnotationLayer::new(vec![World3dAnnotation::dimension("w", [1.0; 3], [1.0; 3], [0.0; 3], text())])).is_err());
    assert!(base().with_section(World3dSection::new([0.0; 3], [0.0; 3])).is_err());
    assert!(base().with_highlight(World3dHighlight { edge: Some(World3dSubElementStyle { width_px: Some(20.0), ..Default::default() }), ..Default::default() }).is_err());
    let field = World3dScalarField::new("mesh:part", World3dScalarDomain::Vertex, vec![Some(0.0), None, Some(1.0)], World3dColorRamp::Viridis, World3dScalarRange { min: 0.0, max: 1.0 }, World3dText::new("Thickness", "Staerke")).with_unit("mm").with_ticks(3);
    assert!(base().with_scalar_field(field.clone()).is_ok());
    assert!(base().with_scalar_field(World3dScalarField { range: World3dScalarRange { min: 1.0, max: 1.0 }, ..field }).is_err());
}
