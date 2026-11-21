//! 🛠️ Layer field contracts use neutral vectors and their exact mutation inverses.
use super::{patch_value_json, raster_patch_layer_operations};
use protocol::{ColdMutation, Mutation};
use semio_framework_pack_json::Value;
use semio_s_artifact_raster_raster_model::{find_layer, layer_transform, RasterLayerNode};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_mutations::RasterMutation;

#[test]
fn inspector_mask_controls_emit_semantic_changes_and_restore_history() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🔣️.json")).unwrap();
    for kind in ["pixel", "group"] {
        let mut expected = serde_json::json!({"schema":"raster.document","id":"mask-controls","title":null,"assets":{},"layers":[{"kind":"pixel","id":"paint","name":"Paint","visible":true,"opacity":1.0,"blendMode":"normal","transform":{"x":0.0,"y":0.0,"a":1.0,"b":0.0,"c":-0.0,"d":1.0},"width":32,"height":16,"imageKey":null,"mask":null}]});
        if kind == "group" {
            let layer = expected["layers"][0].as_object_mut().unwrap();
            for key in ["width", "height", "imageKey"] { layer.remove(key); }
            layer.insert("kind".into(), serde_json::json!("group"));layer.insert("children".into(), serde_json::json!([]));
        }
        let mut document: RasterSnapshot = semio_framework_pack_json::from_json_str(&expected.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        for step in fixture["steps"].as_array().unwrap() {
            if expected["layers"][0]["mask"].is_object() { for invalid in fixture["invalid"].as_array().unwrap() {
                assert!(raster_patch_layer_operations(&document, &["paint".into()], invalid["field"].as_str().unwrap(), &patch_value_json(invalid["field"].as_str().unwrap(), &invalid["value"].to_string())).is_err(), "invalid mask property {invalid}");
            } }
            let field = step["field"].as_str().unwrap();
            let value = patch_value_json(field, &step["value"].to_string());
            let operation = raster_patch_layer_operations(&document, &["paint".into()], field, &value).unwrap().remove(0);
            assert!(matches!(operation, RasterMutation::ChangeLayerMask(_)));
            let inverse = operation.inverse(&document).expect("valid retained mutation inverse fixture").remove(0);
            let (diff, _) = operation.diff(&document).into_parts();
            let next = protocol::apply_diff_cold(&diff, &document).unwrap();
            if let Some(mask) = step.get("mask") { expected["layers"][0]["mask"] = mask.clone(); }
            if let Some(changes) = step["change"].as_object() { for (key,value) in changes { expected["layers"][0]["mask"][key] = value.clone(); } }
            let mut reference: RasterSnapshot = semio_framework_pack_json::from_json_str(&expected.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            let actual_mask=match &next.layers[0] {RasterLayerNode::Pixel {mask,..}|RasterLayerNode::Group {mask,..}=>mask.as_ref(),_=>None};
            let reference_mask=match &mut reference.layers[0] {RasterLayerNode::Pixel {mask,..}|RasterLayerNode::Group {mask,..}=>mask.as_mut(),_=>None};
            if let (Some(actual),Some(reference))=(actual_mask,reference_mask) {
                for (a,b) in actual.transform.as_affine().into_iter().zip(reference.transform.as_affine()) {assert!((a-b).abs()<1e-12,"control {field}: {a} != {b}");}
                reference.transform=actual.transform.clone();
            }
            assert_eq!(next, reference);
            let (undo, _) = inverse.diff(&next).into_parts();
            let restored = protocol::apply_diff_cold(&undo, &next).unwrap();
            assert_eq!(restored, document);
            for diff in [diff, undo] { protocol::MutationDiff::retire_cold(diff); }
            for snapshot in [reference, restored, std::mem::replace(&mut document, next)] { semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(snapshot); }
        }
        assert!(raster_patch_layer_operations(&document, &["paint".into()], "maskInvert", &Value::Bool(true)).is_err());
        semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(document);
    }
}

#[test]
fn property_commands_enforce_neutral_protection_capabilities() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🧫️fixtures/🔒️protection/🔣️.json")).unwrap();
    let mut document=semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot();document.layers=semio_framework_pack_json::from_json_str(&fixture["layers"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let id=case["id"].as_str().unwrap();
        for (field,value,capability) in [("name",Value::String("Updated".into()),"editable"),("locked",Value::Bool(false),"canChangeLock"),("visible",Value::Bool(false),"visible")] {
            let outcome=raster_patch_layer_operations(&document,&[id.into()],field,&value);
            let accepted=capability=="visible"||case["expected"][capability].as_bool().unwrap();
            assert_eq!(outcome.is_ok(),accepted,"{id}: {field}");
            if let Ok(operations)=outcome {for operation in operations {operation.retire_cold();}}
        }
        if !matches!(find_layer(&document.layers,id).unwrap(),RasterLayerNode::Adjustment {..}) {
            let outcome=raster_patch_layer_operations(&document,&[id.into()],"transformShearX",&patch_value_json("transformShearX","1"));
            assert_eq!(outcome.is_ok(),case["expected"]["structural"].as_bool().unwrap());
            if let Ok(operations)=outcome {for operation in operations {operation.retire_cold();}}
        }
    }
    assert!(raster_patch_layer_operations(&document,&["editable-pixel".into(),"locked-pixel".into()],"name",&Value::String("Updated".into())).is_err());
    semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(document);
}

#[test]
fn layer_transform_controls_apply_neutral_vectors_and_restore_exact_history(){
    use semio_s_artifact_raster_raster_model::{create_layer_of_kind, layer_node_id};
    use semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot;
    use semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧬️schema/🧬️mutations/📐️change-layer-transform/🧪️tests/🔣️.json")).unwrap();
    for kind in ["pixel","group"] {
        let mut document=empty_raster_snapshot();document.layers.push(create_layer_of_kind(kind));let id=layer_node_id(&document.layers[0]).to_owned();
        for row in fixture["controls"].as_array().unwrap(){
            let field=row["field"].as_str().unwrap();let operation=raster_patch_layer_operations(&document,&[id.clone()],field,&patch_value_json(field,&row["value"].to_string())).unwrap().remove(0);
            assert!(matches!(operation,RasterMutation::ChangeLayerTransform(_)));
            let inverse=operation.inverse(&document).expect("valid retained mutation inverse fixture").remove(0);let (diff,_)=operation.diff(&document).into_parts();let next=protocol::apply_diff_cold(&diff, &document).unwrap();
            for (actual,expected) in layer_transform(&next.layers[0]).as_affine().into_iter().zip(row["matrix"].as_array().unwrap()){assert!((actual-expected.as_f64().unwrap()).abs()<1e-12);}
            let (undo,_)=inverse.diff(&next).into_parts();let restored=protocol::apply_diff_cold(&undo, &next).unwrap();assert_eq!(restored,document);retire_raster_snapshot(restored);retire_raster_snapshot(std::mem::replace(&mut document,next));
            for value in [operation,inverse]{value.retire_cold();}for value in [diff,undo]{value.retire_cold();}
        }
        for (field,value) in [("transformScaleX","0"),("transformScaleY","1e-20"),("transformShearX","true"),("transformRotation","\"90\"")]{assert!(raster_patch_layer_operations(&document,&[id.clone()],field,&patch_value_json(field,value)).is_err());}
        retire_raster_snapshot(document);
    }
}
