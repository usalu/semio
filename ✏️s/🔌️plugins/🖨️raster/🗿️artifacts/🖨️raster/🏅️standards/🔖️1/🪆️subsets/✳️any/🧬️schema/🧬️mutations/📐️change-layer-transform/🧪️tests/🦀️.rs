//! 📐️ Pixel/group placement, exact inverse and sequential transform coalescing.
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::layer_transform;
use semio_s_artifact_raster_raster_mutations_layer_properties::change_layer_transform::*;
use protocol::Mutation;
#[semio_framework_async_macros::async_test]
async fn layer_transforms_preserve_exact_inverse_and_sequential_moves(){
    use semio_s_artifact_raster_raster_model::{create_layer_of_kind, layer_node_id};
    use semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot;
    use semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot;
    let fixture:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../../📐️change-layer-transform/🧪️tests/🔣️.json")).unwrap();
    for kind in ["pixel","group"] {for row in fixture["cases"].as_array().unwrap(){
        let mut base=empty_raster_snapshot();base.layers.push(create_layer_of_kind(kind));let id=layer_node_id(&base.layers[0]).to_owned();
        let transform:RasterTransform=semio_framework_pack_json::from_json_str(&row["transform"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let payload=ChangeLayerTransform {layer_id:id.clone(),expected:RasterTransform::default(),transform:transform.clone()};
        let mutation=RasterMutation::ChangeLayerTransform(payload.clone());
        let (diff,messages)=mutation.diff(&base).into_parts();assert!(messages.is_empty());let after=protocol::apply_diff(&diff, &base).unwrap();assert_eq!(layer_transform(&after.layers[0]),&transform);
        for horizontal in [false,true] {
            let mut ambiguous=diff.clone();let patch=&mut ambiguous.layers.as_mut().unwrap().modified[0].patch;
            if horizontal {patch.transform_x=Some(0.0);}else{patch.transform_y=Some(0.0);}
            assert!(protocol::apply_diff(&ambiguous, &base).is_err());ambiguous.retire_cold();
        }
        assert_eq!(validate(&payload,&after),Err(protocol::OutcomeCode::TargetMismatch));
        let inverse=mutation.inverse(&base).expect("valid retained mutation inverse fixture").remove(0);let (undo,_)=inverse.diff(&after).into_parts();let restored=protocol::apply_diff(&undo, &after).unwrap();assert_eq!(restored,base);
        let movement=RasterMutation::MoveLayer(semio_s_artifact_raster_raster_mutations_layers::move_layer::mutation::MoveLayer {layer_id:id,new_x:9.0,new_y:10.0});
        for full_first in [false,true] {
            let full=mutation.diff(&base).diff().clone();let partial=movement.diff(&base).diff().clone();
            let (first,second)=if full_first {(full,partial)}else{(partial,full)};
            protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base,first,second).await;
        }
        let mut singular=payload.clone();singular.transform.a=0.0;singular.transform.b=0.0;assert!(validate(&singular,&base).is_err());
        let mut unsupported=empty_raster_snapshot();unsupported.layers.push(create_layer_of_kind("adjustment"));singular.layer_id=layer_node_id(&unsupported.layers[0]).into();assert!(validate(&singular,&unsupported).is_err());retire_raster_snapshot(unsupported);
        for value in [mutation,inverse,movement]{value.retire_cold();}for value in [diff,undo]{value.retire_cold();}for value in [base,after,restored]{retire_raster_snapshot(value);}
    }}
}
/// 🧾️ The committed wire witness is the canonical Rust wire of the fixture's shear case over the identity placement.
#[test]
fn committed_wire_witness_is_the_canonical_rust_wire(){
    let fixture:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../../📐️change-layer-transform/🧪️tests/🔣️.json")).unwrap();
    let witnessed:RasterMutation=store::os_store::test_support::assert_wire_witness(runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/📐️change-layer-transform/🧾️wire-witness/🦠️mutation/🔣️.json"));
    let expected=RasterMutation::ChangeLayerTransform(ChangeLayerTransform{layer_id:"ink".into(),expected:semio_framework_pack_json::from_json_str(&fixture["identity"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(),transform:semio_framework_pack_json::from_json_str(&fixture["cases"][0]["transform"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()});
    assert_eq!(witnessed,expected);
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
