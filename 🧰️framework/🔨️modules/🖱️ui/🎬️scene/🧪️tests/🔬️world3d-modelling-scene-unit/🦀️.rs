//! 🏷️ Unit proofs of the modelling lanes carried by the world-3d scene: split, merge, refusal and builder validation.

use super::*;
use semio_framework_ui_scene_modelling::{World3dAnnotation, World3dColorRamp, World3dScalarDomain, World3dScalarRange, World3dSubElementStyle, World3dText, World3dTone};
use serde_json::Value;

const FIXTURE: &str = include_str!("../🧫️fixtures/📏️world3d-modelling/🔣️.json");

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("modelling fixture parses")
}

fn decode<T: FromValue>(value: &Value) -> Result<T, ValueError> {
    world3d_modelling_from_lane_text(&serde_json::to_string(value).unwrap())
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
    assert!(matches!(assembled.encode_pack(), Err(semio_framework_ui_scene_pack::PackError::Unsupported(_))), "an unsplit scene must not silently drop its typed lanes");
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
