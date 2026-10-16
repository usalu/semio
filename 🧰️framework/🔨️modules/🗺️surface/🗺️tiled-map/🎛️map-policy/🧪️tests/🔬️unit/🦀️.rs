//! 🔬️ Unit tests of the neutral map policy port: the descriptor fixture and schema, the interpreter's rules.

use super::*;

const FIXTURE: &str = include_str!("../🧫️fixtures/🔣️.json");
const SCHEMA: &str = include_str!("../../🧬️schema/🔣️.json");

fn fixture() -> serde_json::Value {
    serde_json::from_str(FIXTURE).expect("policy fixture")
}

fn sample_policy() -> DescribedMapPolicy {
    DescribedMapPolicy::new(serde_json::from_value(fixture()["samplePolicy"].clone()).expect("sample policy descriptor"))
}

#[test]
fn the_neutral_descriptor_matches_its_language_agnostic_fixture() {
    let parsed: MapPolicyDescriptor = serde_json::from_value(fixture()["neutral"].clone()).expect("neutral fixture");
    assert_eq!(parsed, MapPolicyDescriptor::neutral());
    let round_trip: MapPolicyDescriptor = serde_json::from_value(serde_json::to_value(MapPolicyDescriptor::neutral()).expect("serialise")).expect("deserialise");
    assert_eq!(round_trip, MapPolicyDescriptor::neutral());
}

#[test]
fn the_serialised_descriptor_carries_exactly_the_schema_properties() {
    let schema: serde_json::Value = serde_json::from_str(SCHEMA).expect("policy schema");
    let mut required: Vec<&str> = schema["required"].as_array().expect("required").iter().map(|key| key.as_str().expect("key")).collect();
    let value = serde_json::to_value(MapPolicyDescriptor::neutral()).expect("serialise");
    let mut keys: Vec<&str> = value.as_object().expect("object").keys().map(String::as_str).collect();
    required.sort_unstable();
    keys.sort_unstable();
    assert_eq!(keys, required);
    let band_required = schema["definitions"]["MapLodBand"]["required"].as_array().expect("band required").len();
    assert_eq!(value["bands"][0].as_object().expect("band").len(), band_required);
}

#[test]
fn the_neutral_policy_is_one_open_band_without_roles() {
    let policy = DescribedMapPolicy::default();
    assert_eq!(policy.id(), MAP_POLICY_NEUTRAL_ID);
    assert_eq!(policy.bands().len(), 1);
    assert_eq!(policy.band_for_span(360.0, None), 0);
    let gates = policy.layer_gates(0, 0.01, 0);
    assert!(gates.water && gates.land_cover && gates.roads && gates.buildings && gates.boundaries && gates.coastline);
    assert!(!gates.land_backdrop, "the neutral policy paints no land backdrop");
    assert_eq!(policy.layer_role("water"), None, "the framework names no vendor source layer");
    assert_eq!(policy.line_scale(0.001), 1.0);
    assert_eq!(policy.line_scale(300.0), 1.0);
    assert!(policy.road_visible("anything", 1_000.0, 0));
    assert!(policy.boundary_visible(42, &gates));
}

#[test]
fn an_empty_band_table_is_admitted_with_the_neutral_band() {
    let policy = DescribedMapPolicy::new(MapPolicyDescriptor { bands: Vec::new(), ..MapPolicyDescriptor::neutral() });
    assert_eq!(policy.bands(), MapPolicyDescriptor::neutral().bands.as_slice());
}

#[test]
fn the_sample_policy_resolves_its_specification_vectors() {
    let fixture = fixture();
    let policy = sample_policy();
    for case in fixture["bandForSpan"].as_array().expect("bandForSpan") {
        let span = case["spanDeg"].as_f64().expect("span");
        assert_eq!(policy.band_for_span(span, None) as u64, case["band"].as_u64().expect("band"), "band for span {span}");
    }
    for case in fixture["tileZTarget"].as_array().expect("tileZTarget") {
        let view = MapViewFacts { span_deg: case["spanDeg"].as_f64().expect("span"), ideal_tile_z: case["idealTileZ"].as_u64().expect("ideal") as u32 };
        assert_eq!(u64::from(policy.tile_z_target(view, case["pinned"].as_str())), case["tileZ"].as_u64().expect("tile z"), "tile z target for {case}");
    }
    for case in fixture["vectorTileZTarget"].as_array().expect("vectorTileZTarget") {
        let view = MapViewFacts { span_deg: case["spanDeg"].as_f64().expect("span"), ideal_tile_z: 0 };
        assert_eq!(u64::from(policy.vector_tile_z_target(view, case["rasterTileZ"].as_u64().expect("raster") as u32)), case["tileZ"].as_u64().expect("tile z"), "vector tile z for {case}");
    }
}

#[test]
fn class_tables_widths_and_line_damping_follow_their_rows() {
    let spans = MapClassSpans { rows: vec![MapClassSpan { classes: vec!["major".into()], below_span_deg: Some(10.0) }, MapClassSpan { classes: vec!["always".into()], below_span_deg: None }], fallback_below_span_deg: Some(1.0) };
    assert!(spans.admits("major", 9.9) && !spans.admits("major", 10.0));
    assert!(spans.admits("always", 1e9));
    assert!(spans.admits("other", 0.5) && !spans.admits("other", 1.0));
    let widths = MapClassWidths { rows: vec![MapClassWidth { classes: vec!["wide".into()], width: 4.0 }], fallback: 1.0, min: 0.5, max: 3.0 };
    assert_eq!(widths.at("wide", 1.0), 3.0);
    assert_eq!(widths.at("other", 0.1), 0.5);
    let levels = MapLevelWidths { rows: vec![MapLevelWidth { min_level: 3, max_level: 4, width: 2.0 }], fallback: 1.0, min: 0.0, max: 10.0 };
    assert_eq!(levels.at(4, 1.5), 3.0);
    assert_eq!(levels.at(5, 1.5), 1.5);
    let scale = MapLineScale {
        coefficient: 10.0,
        min_span_deg: 0.0,
        coefficient_min_span_deg: 1.0,
        min: 0.0,
        max: 100.0,
        segments: vec![MapLineDamping { up_to_span_deg: 4.0, from_span_deg: 0.0, base: 0.5, slope: 0.5, curve: MapDampingCurve::SquareRoot }, MapLineDamping { up_to_span_deg: 8.0, from_span_deg: 4.0, base: 1.0, slope: 1.0, curve: MapDampingCurve::Linear }],
        beyond: 3.0,
    };
    assert_eq!(scale.at(1.0), 10.0 * (0.5 + 0.5 * 0.5));
    assert_eq!(scale.at(6.0), 10.0 / 6.0 * 1.5);
    assert_eq!(scale.at(10.0), 3.0);
    let rules = MapLabelRules { place_label_spans: vec![MapClassSpanRange { classes: vec!["town".into()], max_span_deg: Some(4.0), min_span_deg: Some(1.0) }], ..MapPolicyDescriptor::neutral().labels };
    assert!(rules.place_admits("town", 4.0) && !rules.place_admits("town", 1.0) && !rules.place_admits("village", 2.0));
}

#[test]
fn roles_ranks_labels_and_buildings_come_from_the_descriptor() {
    let mut descriptor = MapPolicyDescriptor::neutral();
    descriptor.layers = vec![MapSourceLayer { name: "areas".into(), role: Some(MapLayerRole::LandCover), draw_rank: 2, label_rank: None }, MapSourceLayer { name: "names".into(), role: Some(MapLayerRole::PlaceLabel), draw_rank: 9, label_rank: Some(7) }];
    descriptor.default_draw_rank = 5;
    descriptor.labels.place_label_spans = vec![MapClassSpanRange { classes: vec!["".into()], max_span_deg: None, min_span_deg: None }];
    descriptor.bands[0].buildings_max_span_deg = Some(1.0);
    descriptor.bands[0].buildings_min_tile_z = 13;
    descriptor.feature_keys.label = vec!["label".into()];
    let policy = DescribedMapPolicy::new(descriptor);
    assert_eq!(policy.layer_role("areas"), Some(MapLayerRole::LandCover));
    assert_eq!(policy.draw_rank("names"), 9);
    assert_eq!(policy.draw_rank("unlisted"), 5);
    assert_eq!(policy.label_gate_and_rank("names", "", 2.0), Some(7));
    assert_eq!(policy.label_gate_and_rank("areas", "", 2.0), None);
    assert!(policy.layer_gates(0, 0.5, 13).buildings);
    assert!(!policy.layer_gates(0, 0.5, 12).buildings && !policy.layer_gates(0, 1.0, 14).buildings);
    let properties = std::collections::BTreeMap::from([("label".to_string(), " Bern ".to_string())]);
    assert_eq!(policy.feature_label(&properties).as_deref(), Some("Bern"));
}

#[test]
fn json_surfaces_are_parameterised_by_the_policy() {
    let rows: Vec<serde_json::Value> = serde_json::from_str(&map_lod_scale_json(&sample_policy())).expect("lod scale");
    assert_eq!(rows.iter().map(|row| row["id"].as_str().expect("id")).collect::<Vec<_>>(), ["far", "near"]);
    assert_eq!(rows[0]["maxZoom"], 500.0);
    assert!(rows[1]["maxZoom"].is_null());
    let ids: Vec<String> = serde_json::from_str(&map_layer_weight_slider_ids_json(&sample_policy(), "far", "image")).expect("slider ids");
    assert!(ids.is_empty(), "the sample policy declares no layer toggle");
}
