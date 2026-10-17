/// 🎚️ Theme, level of detail and label guards: the invalid theme, layer visibility and stroke scale JSON, the neutral band list, the pinned band fallback, the opaque fill and the label declutter.

#[test]
fn set_map_theme_from_json_invalid_json_returns_err() {
    let mut host = super::MapHost::new();
    assert!(host.set_map_theme_from_json("not json").is_err());
}

#[test]
fn set_layer_visibility_from_json_invalid_json_returns_err_and_partial_json_overrides_only_given_fields() {
    let mut host = super::MapHost::new();
    assert!(host.set_layer_visibility_from_json("not json").is_err());
    host.set_layer_visibility_from_json(r#"{"roads":false}"#).expect("partial");
    let v: serde_json::Value = serde_json::from_str(&host.layer_visibility_json()).expect("json");
    assert_eq!(v["roads"], false);
    assert_eq!(v["water"], true);
}

#[test]
fn set_layer_stroke_scale_from_json_sanitizes_out_of_range_weights() {
    let mut host = super::MapHost::new();
    assert!(host.set_layer_stroke_scale_from_json("not json").is_err());
    host.set_layer_stroke_scale_from_json(r#"{"roads":999.0,"water":-5.0}"#).expect("scale");
    let v: serde_json::Value = serde_json::from_str(&host.layer_stroke_scale_json()).expect("json");
    assert_eq!(v["roads"].as_f64().unwrap(), super::MAP_LAYER_WEIGHT_MAX);
    assert_eq!(v["water"].as_f64().unwrap(), super::MAP_LAYER_WEIGHT_MIN);
}

#[test]
fn map_lod_scale_json_lists_the_neutral_band() {
    let host = super::MapHost::new();
    let v: Vec<serde_json::Value> = serde_json::from_str(&super::map_lod_scale_json(host.policy())).expect("json");
    assert_eq!(v.len(), 1);
    assert_eq!(v[0]["id"], "all");
    assert!(v[0]["maxZoom"].is_null(), "the neutral band covers every zoom");
}

#[test]
fn band_for_span_falls_back_to_the_span_when_the_pinned_band_is_unknown() {
    let policy = fixture_band_policy();
    let span = 50.0;
    assert_eq!(policy.band_for_span(span, Some("bogus-lod")), policy.band_for_span(span, None));
    assert_eq!(policy.band_for_span(span, Some("band-4")), 4);
}

#[test]
fn weighted_opaque_fill_keeps_alpha_solid() {
    let c = super::weighted_opaque_fill(super::Color::from_rgba8(40, 50, 60, 128), 1.0);
    assert_eq!(c.to_rgba8().a, 255);
}

#[test]
fn label_declutter_rejects_when_over_capacity_or_offscreen() {
    let viewport = Viewport { width: 200, height: 200, dpr: 1.0 };
    let mut declutter = super::LabelDeclutter::for_viewport(&viewport, 20.0, 1);
    assert!(declutter.try_place("first", super::Point::new(50.0, 50.0), 12.0));
    assert!(!declutter.try_place("second", super::Point::new(150.0, 150.0), 12.0), "max_count of 1 must reject a second label");
    let mut roomy = super::LabelDeclutter::for_viewport(&viewport, 20.0, 10);
    assert!(!roomy.try_place("offscreen", super::Point::new(10_000.0, 10_000.0), 12.0));
}
