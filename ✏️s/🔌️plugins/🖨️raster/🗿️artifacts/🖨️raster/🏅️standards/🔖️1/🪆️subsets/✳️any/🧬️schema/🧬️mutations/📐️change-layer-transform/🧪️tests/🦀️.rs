//! 📐️ Pixel/group placement, exact inverse and sequential transform coalescing.
use super::*;
use protocol::{Mutation,MutationDiff};
#[semio_framework_async_macros::async_test]
async fn layer_transforms_preserve_exact_inverse_and_sequential_moves(){
    use crate::standards::v1::subsets::any::schema::{create_layer_of_kind,empty_raster_snapshot,layer_node_id,snapshot::retire_raster_snapshot};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for kind in ["pixel","group"] {for row in fixture["cases"].as_array().unwrap(){
        let mut base=empty_raster_snapshot();base.layers.push(create_layer_of_kind(kind));let id=layer_node_id(&base.layers[0]).to_owned();
        let transform:RasterTransform=dsl::json::from_json_str(&row["transform"].to_string()).unwrap();
        let payload=ChangeLayerTransform {layer_id:id.clone(),expected:RasterTransform::default(),transform:transform.clone()};
        let mutation=RasterMutation::ChangeLayerTransform(payload.clone());
        let (diff,messages)=mutation.diff(&base).into_parts();assert!(messages.is_empty());let after=diff.apply(&base).unwrap();assert_eq!(layer_transform(&after.layers[0]),&transform);
        for horizontal in [false,true] {
            let mut ambiguous=diff.clone();let patch=&mut ambiguous.layers.as_mut().unwrap().patched[0].patch;
            if horizontal {patch.transform_x=Some(0.0);}else{patch.transform_y=Some(0.0);}
            assert!(ambiguous.apply(&base).is_err());ambiguous.retire_cold();
        }
        assert_eq!(validate(&payload,&after),Err("mutation.transform-conflict"));
        let inverse=mutation.inverse(&base).remove(0);let (undo,_)=inverse.diff(&after).into_parts();let restored=undo.apply(&after).unwrap();assert_eq!(restored,base);
        let movement=RasterMutation::MoveLayer(crate::mutations::move_layer::MoveLayer {layer_id:id,new_x:9.0,new_y:10.0});
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
