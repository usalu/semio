//! 🎛️ Property actions compared with independent JSON field updates.
use super::*;
use protocol::{Mutation, MutationDiff};

#[test]
fn mask_link_changes_preserve_nested_composite_pixels_and_exact_undo() {
    use crate::standards::v1::subsets::any::schema::{semio_fixture_snapshot,snapshot::retire_raster_snapshot};
    use store::ArtifactPack;
    for owner in ["paint","group"] {
        let mut document=semio_fixture_snapshot();
        let mask=serde_json::json!({"enabled":true,"linked":true,"invert":false,"width":4,"height":4,"imageKey":"semio-emblem","transform":{"x":0.0,"y":0.0,"a":1.0,"b":1.0,"c":-1.0,"d":1.0}});
        let layers=serde_json::json!([{"kind":"group","id":"group","name":"Group","visible":true,"locked":false,"opacity":1.0,"blendMode":"normal","transform":{"x":3.0,"y":2.0,"a":1.0,"b":0.5,"c":0.0,"d":1.0},"mask":if owner=="group" {mask.clone()}else{serde_json::Value::Null},"children":[{"kind":"pixel","id":"paint","name":"Paint","visible":true,"locked":false,"opacity":1.0,"blendMode":"normal","transform":{"x":0.0,"y":0.0,"a":2.0,"b":0.0,"c":0.0,"d":1.0},"width":2,"height":2,"imageKey":"semio-emblem","mask":if owner=="paint" {mask.clone()}else{serde_json::Value::Null}}]}]);
        crate::retire_raster_layers(std::mem::replace(&mut document.layers,dsl::json::from_json_str(&layers.to_string()).unwrap()));
        let render=|snapshot:&RasterSnapshot| {
            let mut job=crate::io::raster_composite_job(snapshot).unwrap();while !job.advance(17).unwrap().done {}
            let output=job.into_result().unwrap();(output.origin,output.image)
        };
        let before=render(&document);assert!(before.1.pixels.chunks_exact(4).any(|pixel|pixel[3]>0));
        let operation=raster_patch_layer_operations(&document,&[owner.into()],"maskLinked",&Value::Bool(false)).unwrap().remove(0);
        let inverse=operation.inverse(&document).remove(0);let (diff,_)=operation.diff(&document).into_parts();let unlinked=diff.apply(&document).unwrap();
        assert_eq!(render(&unlinked),before);
        let packed=unlinked.encode_pack();let restored=RasterSnapshot::decode_pack(&packed).unwrap();assert_eq!(restored,unlinked);
        let (undo,_)=inverse.diff(&unlinked).into_parts();let undone=undo.apply(&unlinked).unwrap();assert_eq!(undone,document);
        let relink=raster_patch_layer_operations(&unlinked,&[owner.into()],"maskLinked",&Value::Bool(true)).unwrap().remove(0);
        let (redo,_)=relink.diff(&unlinked).into_parts();let linked=redo.apply(&unlinked).unwrap();assert_eq!(render(&linked),before);
        for value in [operation,inverse,relink] {value.retire_cold();}
        for value in [diff,undo,redo] {value.retire_cold();}
        for value in [document,unlinked,restored,undone,linked] {retire_raster_snapshot(value);}
    }
}

#[test]
fn inspector_mask_controls_emit_semantic_changes_and_restore_history() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for kind in ["pixel", "group"] {
        let mut expected = serde_json::json!({"schema":"raster.document","id":"mask-controls","title":null,"assets":{},"layers":[{"kind":"pixel","id":"paint","name":"Paint","visible":true,"opacity":1.0,"blendMode":"normal","transform":{"x":0.0,"y":0.0,"a":1.0,"b":0.0,"c":-0.0,"d":1.0},"width":32,"height":16,"imageKey":null,"mask":null}]});
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
            let mut reference: RasterSnapshot = dsl::json::from_json_str(&expected.to_string()).unwrap();
            let actual_mask=match &next.layers[0] {RasterLayerNode::Pixel {mask,..}|RasterLayerNode::Group {mask,..}=>mask.as_ref(),_=>None};
            let reference_mask=match &mut reference.layers[0] {RasterLayerNode::Pixel {mask,..}|RasterLayerNode::Group {mask,..}=>mask.as_mut(),_=>None};
            if let (Some(actual),Some(reference))=(actual_mask,reference_mask) {
                for (a,b) in actual.transform.as_affine().into_iter().zip(reference.transform.as_affine()) {assert!((a-b).abs()<1e-12,"control {field}: {a} != {b}");}
                reference.transform=actual.transform.clone();
            }
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

#[test]
fn property_commands_enforce_neutral_protection_capabilities() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧬️schema/🧫️fixtures/🔒️protection/🔣️.json")).unwrap();
    let mut document=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();document.layers=dsl::json::from_json_str(&fixture["layers"].to_string()).unwrap();
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
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
}

#[test]
fn layer_transform_controls_apply_neutral_vectors_and_restore_exact_history(){
    use crate::standards::v1::subsets::any::schema::{create_layer_of_kind,empty_raster_snapshot,layer_node_id,snapshot::retire_raster_snapshot};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧬️schema/🧬️mutations/📐️change-layer-transform/🧪️tests/🔣️.json")).unwrap();
    for kind in ["pixel","group"] {
        let mut document=empty_raster_snapshot();document.layers.push(create_layer_of_kind(kind));let id=layer_node_id(&document.layers[0]).to_owned();
        for row in fixture["controls"].as_array().unwrap(){
            let field=row["field"].as_str().unwrap();let operation=raster_patch_layer_operations(&document,&[id.clone()],field,&patch_value_json(field,&row["value"].to_string())).unwrap().remove(0);
            assert!(matches!(operation,RasterMutation::ChangeLayerTransform(_)));
            let inverse=operation.inverse(&document).remove(0);let (diff,_)=operation.diff(&document).into_parts();let next=diff.apply(&document).unwrap();
            for (actual,expected) in layer_transform(&next.layers[0]).as_affine().into_iter().zip(row["matrix"].as_array().unwrap()){assert!((actual-expected.as_f64().unwrap()).abs()<1e-12);}
            let (undo,_)=inverse.diff(&next).into_parts();let restored=undo.apply(&next).unwrap();assert_eq!(restored,document);retire_raster_snapshot(restored);retire_raster_snapshot(std::mem::replace(&mut document,next));
            for value in [operation,inverse]{value.retire_cold();}for value in [diff,undo]{value.retire_cold();}
        }
        for (field,value) in [("transformScaleX","0"),("transformScaleY","1e-20"),("transformShearX","true"),("transformRotation","\"90\"")]{assert!(raster_patch_layer_operations(&document,&[id.clone()],field,&patch_value_json(field,value)).is_err());}
        retire_raster_snapshot(document);
    }
}

#[semio_framework_async_macros::async_test]
async fn layer_transform_controls_publish_independent_reversible_edits(){
    use crate::editor::raster::{RasterCommand,unit_tests::context};
    use crate::standards::v1::subsets::any::schema::{create_layer_of_kind,empty_raster_snapshot,layer_node_id,snapshot::retire_raster_snapshot};
    use semio_framework_plugin::PluginApp;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧬️schema/🧬️mutations/📐️change-layer-transform/🧪️tests/🔣️.json")).unwrap();
    for kind in ["pixel","group"] {
        let mut source=empty_raster_snapshot();source.layers.push(create_layer_of_kind(kind));let id=layer_node_id(&source.layers[0]).to_owned();
        let envelope=store::create_document_envelope::<RasterSnapshot,RasterMutation>(crate::RASTER_DOCUMENT_SCHEMA,"transform-controls",source,None);
        let files=store::print_document_pack(&envelope).await.unwrap();context::retire_raster_envelope(envelope);
        let mut app=context::app().await;app.load_document_pack(&files).await.unwrap();let mut history=vec![app.snapshot().unwrap()];
        for row in fixture["controls"].as_array().unwrap(){
            context::dispatch(&mut app,RasterCommand::PatchLayer(PatchLayer {layer_id:id.clone(),field:row["field"].as_str().unwrap().into(),value:row["value"].to_string()})).await;
            let next=app.snapshot().unwrap();
            for (actual,expected) in layer_transform(&next.layers[0]).as_affine().into_iter().zip(row["matrix"].as_array().unwrap()){assert!((actual-expected.as_f64().unwrap()).abs()<1e-12);}
            history.push(next);
        }
        for expected in history[..history.len()-1].iter().rev(){context::history(&mut app,"undo").await;let actual=app.snapshot().unwrap();assert_eq!(&actual,expected);retire_raster_snapshot(actual);}
        for expected in &history[1..]{context::history(&mut app,"redo").await;let actual=app.snapshot().unwrap();assert_eq!(&actual,expected);retire_raster_snapshot(actual);}
        for snapshot in history {retire_raster_snapshot(snapshot);}
    }
}
