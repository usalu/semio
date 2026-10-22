//! 🧪️ Pixel content replacement preserves metadata and has an exact nullable inverse.
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use protocol::{ColdMutation, Mutation, OpBinary, OpText};
use semio_framework_mutation_diff::MutationDiff;

#[test]
fn change_layer_pixels_matches_independent_json_and_round_trips() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../../🎨️change-layer-pixels/🧪️tests/🔣️.json")).unwrap();
    let before_json = fixture["before"].to_string();
    let base: RasterSnapshot = semio_framework_pack_json::from_json_str(&before_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mutation: RasterMutation = semio_framework_pack_json::from_json_str(&fixture["mutation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let inverse = mutation.inverse(&base).expect("valid retained mutation inverse fixture");
    let mut expected = fixture["before"].clone();
    for key in ["imageKey", "width", "height"] { expected["layers"][0][key] = fixture["mutation"]["content"][key].clone(); }
    let (diff, _) = mutation.diff(&base).into_parts();
    let actual = protocol::apply_diff_cold(&diff, &base).unwrap();
    let rendered: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&actual)))).unwrap();
    assert_eq!(json_numbers(rendered), json_numbers(expected));
    assert_eq!(RasterMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
    assert_eq!(RasterMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    let mut restored = actual;
    for undo in inverse {
        let (change, _) = undo.diff(&restored).into_parts();
        let previous = protocol::apply_diff_cold(&change, &restored).unwrap();
        MutationDiff::retire_cold(change);
        semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(restored);
        restored = previous;
        Mutation::retire_cold(undo);
    }
    assert_eq!(restored, base);
    MutationDiff::retire_cold(diff);
    Mutation::retire_cold(mutation);
    semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(restored);
    semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(base);
}

/// 🔢️ JSON numbers compare by value regardless of integer or floating-point token spelling.
fn json_numbers(mut value: serde_json::Value) -> serde_json::Value {
    match &mut value {
        serde_json::Value::Number(number) => serde_json::Value::from(number.as_f64().unwrap()),
        serde_json::Value::Array(values) => serde_json::Value::Array(values.drain(..).map(json_numbers).collect()),
        serde_json::Value::Object(values) => serde_json::Value::Object(std::mem::take(values).into_iter().map(|(key, value)| (key, json_numbers(value))).collect()),
        _ => value,
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
