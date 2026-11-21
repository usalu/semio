//! 🎛️ Property actions compared with independent JSON field updates.
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::layer_transform;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_framework_pack_json::Value;
use semio_s_artifact_raster_raster_editor::commands::patch_layer::*;
use protocol::{ColdMutation, Mutation};
use semio_framework_test_fixtures::fixture_text;

#[test]
fn mask_link_changes_preserve_nested_composite_pixels_and_exact_undo() {
    use semio_s_artifact_raster_raster_io::testing::raster_image_test_snapshot;
    use semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot;
    use pack::snapshot::ArtifactPack;
    for owner in ["paint","group"] {
        let mut document=raster_image_test_snapshot();
        let mask=serde_json::json!({"enabled":true,"linked":true,"invert":false,"width":4,"height":4,"imageKey":"semio-emblem","transform":{"x":0.0,"y":0.0,"a":1.0,"b":1.0,"c":-1.0,"d":1.0}});
        let layers=serde_json::json!([{"kind":"group","id":"group","name":"Group","visible":true,"locked":false,"opacity":1.0,"blendMode":"normal","transform":{"x":3.0,"y":2.0,"a":1.0,"b":0.5,"c":0.0,"d":1.0},"mask":if owner=="group" {mask.clone()}else{serde_json::Value::Null},"children":[{"kind":"pixel","id":"paint","name":"Paint","visible":true,"locked":false,"opacity":1.0,"blendMode":"normal","transform":{"x":0.0,"y":0.0,"a":2.0,"b":0.0,"c":0.0,"d":1.0},"width":2,"height":2,"imageKey":"semio-emblem","mask":if owner=="paint" {mask.clone()}else{serde_json::Value::Null}}]}]);
        semio_s_artifact_raster_raster_model::retire_raster_layers(std::mem::replace(&mut document.layers,semio_framework_pack_json::from_json_str(&layers.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()));
        let render=|snapshot:&RasterSnapshot| {
            let mut job=semio_s_artifact_raster_raster_io::raster_composite_job(snapshot).unwrap();while !semio_s_artifact_raster_raster_io::advance_stack(&mut job,17).unwrap().done {}
            let output=job.into_result().unwrap();(output.origin,output.image)
        };
        let before=render(&document);assert!(before.1.pixels.chunks_exact(4).any(|pixel|pixel[3]>0));
        let operation=raster_patch_layer_operations(&document,&[owner.into()],"maskLinked",&Value::Bool(false)).unwrap().remove(0);
        let inverse=operation.inverse(&document).expect("valid retained mutation inverse fixture").remove(0);let (diff,_)=operation.diff(&document).into_parts();let unlinked=protocol::apply_diff_cold(&diff, &document).unwrap();
        assert_eq!(render(&unlinked),before);
        let packed=unlinked.encode_pack();let restored=RasterSnapshot::decode_pack(&packed).unwrap();assert_eq!(restored,unlinked);
        let (undo,_)=inverse.diff(&unlinked).into_parts();let undone=protocol::apply_diff_cold(&undo, &unlinked).unwrap();assert_eq!(undone,document);
        let relink=raster_patch_layer_operations(&unlinked,&[owner.into()],"maskLinked",&Value::Bool(true)).unwrap().remove(0);
        let (redo,_)=relink.diff(&unlinked).into_parts();let linked=protocol::apply_diff_cold(&redo, &unlinked).unwrap();assert_eq!(render(&linked),before);
        for value in [operation,inverse,relink] {value.retire_cold();}
        for value in [diff,undo,redo] {value.retire_cold();}
        for value in [document,unlinked,restored,undone,linked] {retire_raster_snapshot(value);}
    }
}

#[semio_framework_async_macros::async_test]
async fn layer_transform_controls_publish_independent_reversible_edits(){
    use semio_s_artifact_raster_raster_editor::RasterCommand;
    use crate::unit_tests::context;
    use semio_s_artifact_raster_raster_model::{create_layer_of_kind, layer_node_id};
    use semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot;
    use semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot;
    use semio_plugin::PluginApp;
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../🧬️schema/🧬️mutations/📐️change-layer-transform/🧪️tests/🔣️.json")).unwrap();
    for kind in ["pixel","group"] {
        let mut source=empty_raster_snapshot();source.layers.push(create_layer_of_kind(kind));let id=layer_node_id(&source.layers[0]).to_owned();
        let envelope=store::os_store::document_materialize::create_document_envelope::<RasterSnapshot,RasterMutation>(semio_s_artifact_raster_raster_model::RASTER_DOCUMENT_SCHEMA,"transform-controls",source,None, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let files=store::os_store::text_format::print_document_pack(&envelope).await.unwrap();context::retire_raster_envelope(envelope);
        let mut app=context::app().await;semio_plugin::artifact_app_laws::load_document(&mut app.0, &files, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap();let mut history=vec![app.snapshot().unwrap()];
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
