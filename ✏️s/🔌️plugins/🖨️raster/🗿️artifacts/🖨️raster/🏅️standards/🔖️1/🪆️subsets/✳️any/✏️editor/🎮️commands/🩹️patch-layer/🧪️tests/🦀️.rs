//! 🎛️ Property actions compared with independent JSON field updates.
use super::*;
use protocol::{Mutation, MutationDiff};

#[test]
fn inspector_mask_controls_emit_semantic_changes_and_restore_history() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for kind in ["pixel", "group"] {
        let mut expected = serde_json::json!({"schema":"raster.document","id":"mask-controls","title":null,"assets":{},"layers":[{"kind":"pixel","id":"paint","name":"Paint","visible":true,"opacity":1.0,"blendMode":"normal","transform":{"x":0.0,"y":0.0,"scaleX":1.0,"scaleY":1.0,"rotation":0.0},"width":32,"height":16,"imageKey":null,"mask":null}]});
        if kind == "group" {
            let layer = expected["layers"][0].as_object_mut().unwrap();
            for key in ["width", "height", "imageKey"] { layer.remove(key); }
            layer.insert("kind".into(), serde_json::json!("group"));layer.insert("children".into(), serde_json::json!([]));
        }
        let mut document: RasterSnapshot = dsl::json::from_json_str(&expected.to_string()).unwrap();
        for step in fixture["steps"].as_array().unwrap() {
            if expected["layers"][0]["mask"].is_object() { for invalid in fixture["invalid"].as_array().unwrap() {
                assert!(raster_patch_layer_operations(&document, &["paint".into()], invalid["field"].as_str().unwrap(), &patch_value_json(invalid["field"].as_str().unwrap(), &invalid["value"].to_string())).is_err(), "invalid mask property {invalid}");
            } }
            let field = step["field"].as_str().unwrap();
            let value = patch_value_json(field, &step["value"].to_string());
            let operation = raster_patch_layer_operations(&document, &["paint".into()], field, &value).unwrap().remove(0);
            assert!(matches!(operation, RasterMutation::ChangeLayerMask(_)));
            let inverse = operation.inverse(&document).remove(0);
            let (diff, _) = operation.diff(&document).into_parts();
            let next = diff.apply(&document).unwrap();
            if let Some(mask) = step.get("mask") { expected["layers"][0]["mask"] = mask.clone(); }
            if let Some(changes) = step["change"].as_object() { for (key,value) in changes { expected["layers"][0]["mask"][key] = value.clone(); } }
            let reference: RasterSnapshot = dsl::json::from_json_str(&expected.to_string()).unwrap();
            assert_eq!(next, reference);
            let (undo, _) = inverse.diff(&next).into_parts();
            let restored = undo.apply(&next).unwrap();
            assert_eq!(restored, document);
            for diff in [diff, undo] { protocol::MutationDiff::retire_cold(diff); }
            for snapshot in [reference, restored, std::mem::replace(&mut document, next)] { crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(snapshot); }
        }
        assert!(raster_patch_layer_operations(&document, &["paint".into()], "maskInvert", &Value::Bool(true)).is_err());
        crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
    }
}
