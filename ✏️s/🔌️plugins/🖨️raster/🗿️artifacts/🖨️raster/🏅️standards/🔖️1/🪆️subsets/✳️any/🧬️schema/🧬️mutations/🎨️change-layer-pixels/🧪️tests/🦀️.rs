//! 🧪️ Pixel content replacement preserves metadata and has an exact nullable inverse.
use crate::{RasterSnapshot, RasterMutation};
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

#[test]
fn change_layer_pixels_matches_independent_json_and_round_trips() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let before_json = fixture["before"].to_string();
    let base: RasterSnapshot = dsl::os_pack::json::from_json_str(&before_json).unwrap();
    let mutation: RasterMutation = dsl::os_pack::json::from_json_str(&fixture["mutation"].to_string()).unwrap();
    let inverse = mutation.inverse(&base);
    let mut expected = fixture["before"].clone();
    for key in ["imageKey", "width", "height"] { expected["layers"][0][key] = fixture["mutation"]["content"][key].clone(); }
    let (diff, _) = mutation.diff(&base).into_parts();
    let actual = diff.apply(&base).unwrap();
    let rendered: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_string(&dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(&actual)))).unwrap();
    assert_eq!(json_numbers(rendered), json_numbers(expected));
    assert_eq!(RasterMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
    assert_eq!(RasterMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    let mut restored = actual;
    for undo in inverse {
        let (change, _) = undo.diff(&restored).into_parts();
        let previous = change.apply(&restored).unwrap();
        MutationDiff::retire_cold(change);
        crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(restored);
        restored = previous;
        Mutation::retire_cold(undo);
    }
    assert_eq!(restored, base);
    MutationDiff::retire_cold(diff);
    Mutation::retire_cold(mutation);
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(restored);
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(base);
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
