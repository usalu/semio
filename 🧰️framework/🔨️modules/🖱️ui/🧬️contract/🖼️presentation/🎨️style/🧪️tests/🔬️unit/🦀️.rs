use super::*;

#[test]
fn default_style_spec_serializes_to_empty_object() {
    let json = serde_json::to_value(StyleSpec::default()).expect("serialize");
    assert_eq!(json, serde_json::json!({}));
}

#[test]
fn variant_field_omitted_at_default() {
    let json = serde_json::to_value(StyleSpec { variant: Variant::Outline, ..StyleSpec::default() }).expect("serialize");
    assert_eq!(json, serde_json::json!({ "variant": "outline" }));
}

#[test]
fn size_field_omitted_at_default() {
    let json = serde_json::to_value(StyleSpec { size: SizeToken::Lg, ..StyleSpec::default() }).expect("serialize");
    assert_eq!(json, serde_json::json!({ "size": "lg" }));
}

#[test]
fn density_field_omitted_at_default() {
    let json = serde_json::to_value(StyleSpec { density: Density::Touch, ..StyleSpec::default() }).expect("serialize");
    assert_eq!(json, serde_json::json!({ "density": "touch" }));
}

#[test]
fn tone_field_omitted_at_default() {
    let json = serde_json::to_value(StyleSpec { tone: Tone::Danger, ..StyleSpec::default() }).expect("serialize");
    assert_eq!(json, serde_json::json!({ "tone": "danger" }));
}

#[test]
fn emphasis_field_omitted_at_default() {
    let json = serde_json::to_value(StyleSpec { emphasis: Emphasis::Strong, ..StyleSpec::default() }).expect("serialize");
    assert_eq!(json, serde_json::json!({ "emphasis": "strong" }));
}

#[test]
fn fully_styled_spec_roundtrips() {
    let spec = StyleSpec { variant: Variant::Ghost, size: SizeToken::Xs, density: Density::Compact, tone: Tone::Success, emphasis: Emphasis::Subtle };
    let json = serde_json::to_string(&spec).expect("serialize");
    let back: StyleSpec = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(spec, back);
}
