//! 🖱️ Layer creation requests one framework selection after publication, without a second history edit.
use super::*;

#[test]
fn layer_selection_matches_neutral_wire_arguments() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let args=layer_selection_args(row["id"].as_str().unwrap()).unwrap();
        let json:serde_json::Value=serde_json::from_str(&dsl::json::to_json_string(&args)).unwrap();
        assert_eq!(json["domainId"],"layers");assert_eq!(json["merge"],"replace");assert_eq!(json["method"],"pick");
        assert_eq!(serde_json::from_str::<serde_json::Value>(json["targets"].as_str().unwrap()).unwrap(),serde_json::json!([row["target"]]));
    }
    assert!(layer_selection_args("").is_err());
}

#[semio_framework_async_macros::async_test]
async fn created_and_duplicated_layers_are_selected_and_deleted_targets_are_pruned() {
    use crate::editor::raster::{unit_tests::context,RasterCommand,commands::{add_layer,drop_layer_kind,duplicate_layer,delete_layer}};
    use crate::standards::v1::subsets::any::schema::{layer_node_id,snapshot::retire_raster_snapshot};
    let mut app=context::app().await;
    for command in [RasterCommand::AddLayer(add_layer::AddLayer {kind:"pixel".into()}),RasterCommand::DropLayerKind(drop_layer_kind::DropLayerKind {kind:"adjustment".into()})] {
        let result=context::dispatch(&mut app,command).await;
        assert!(!result.requested_effects.iter().any(|effect|matches!(effect,semio_framework::kernel::Effect::DispatchAction {..})),"selection must be applied inside the reactor");
        let snapshot=app.snapshot().unwrap();let id=layer_node_id(snapshot.layers.last().unwrap()).to_owned();retire_raster_snapshot(snapshot);
        assert_eq!(app.interaction_state().await.selection.get(RASTER_INTERACTION_DOMAIN).unwrap().ids,vec![id]);
    }
    let selected=app.interaction_state().await.selection.get(RASTER_INTERACTION_DOMAIN).unwrap().ids[0].clone();
    context::dispatch(&mut app,RasterCommand::DuplicateLayer(duplicate_layer::DuplicateLayer {layer_id:selected.clone()})).await;
    let snapshot=app.snapshot().unwrap();let copy=layer_node_id(snapshot.layers.last().unwrap()).to_owned();assert_ne!(copy,selected);assert_eq!(snapshot.layers.len(),3);retire_raster_snapshot(snapshot);
    assert_eq!(app.interaction_state().await.selection.get(RASTER_INTERACTION_DOMAIN).unwrap().ids,vec![copy.clone()]);
    context::dispatch(&mut app,RasterCommand::DeleteLayer(delete_layer::DeleteLayer {layer_id:copy})).await;
    assert!(app.interaction_state().await.selection.get(RASTER_INTERACTION_DOMAIN).is_none_or(|selection|selection.ids.is_empty()));
    for count in [3,2,1,0] {context::history(&mut app,"undo").await;let snapshot=app.snapshot().unwrap();assert_eq!(snapshot.layers.len(),count);retire_raster_snapshot(snapshot);}
}
