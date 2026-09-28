//! 📋️ Shared duplication vectors preserve sibling placement, metadata and inverse history.
use super::*;
use protocol::Mutation;
use crate::standards::v1::subsets::any::schema::{empty_raster_snapshot,layer_node_id,snapshot::retire_raster_snapshot};

fn assert_copy(source:&crate::RasterLayerNode,copy:&crate::RasterLayerNode) {
    let source=dsl::json::to_json_string(source);
    let mut source:serde_json::Value=serde_json::from_str(&source).unwrap();
    let copy:serde_json::Value=serde_json::from_str(&dsl::json::to_json_string(copy)).unwrap();
    fn compare(source:&mut serde_json::Value,copy:&serde_json::Value) {
        assert_ne!(source["id"],copy["id"]);
        source["id"]=copy["id"].clone();source["name"]=serde_json::json!(format!("{} copy",source["name"].as_str().unwrap()));
        if let Some(children)=source.get_mut("children").and_then(serde_json::Value::as_array_mut) {for (index,child) in children.iter_mut().enumerate(){compare(child,&copy["children"][index]);}}
        assert_eq!(&*source,copy);
    }
    compare(&mut source,&copy);
}

#[test]
fn duplicate_keeps_parent_properties_and_undo() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let mut document=empty_raster_snapshot();document.layers=dsl::json::from_json_str(&fixture["layers"].to_string()).unwrap();
    let history=semio_framework_plugin::HistoryView::empty();let doc=ArtifactView::new(&document,&history);
    let config=RasterConfig::default();let cfg=ConfigView {snapshot:&config,window:None};
    for row in fixture["cases"].as_array().unwrap() {
        let id=row["id"].as_str().unwrap();
        let emit=handle(&DuplicateLayer {layer_id:id.into()},&doc,&cfg).unwrap();assert_eq!(emit.artifact_mutations.len(),1);
        let operation=emit.artifact_mutations.into_iter().next().unwrap();
        let RasterMutation::CreateLayer(create)=&operation else {panic!("create layer")};
        assert_eq!(create.parent_id.as_deref(),row["parentId"].as_str(),"{id}");assert_eq!(create.index,row["index"].as_u64().unwrap() as usize,"{id}");
        assert_copy(find_layer(&document.layers,id).unwrap(),&create.layer);
        let copy_id=layer_node_id(&create.layer).to_owned();
        let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,&operation).unwrap();assert!(messages.is_empty());
        assert!(find_layer(&next.layers,&copy_id).is_some());assert_eq!(find_layer(&next.layers,id),find_layer(&document.layers,id));
        let inverse=operation.inverse(&document).pop().unwrap();let (restored,messages)=semio_framework_os_kernel::apply_mutation(&next,&inverse).unwrap();assert!(messages.is_empty());assert_eq!(restored,document);
        inverse.retire_cold();operation.retire_cold();retire_raster_snapshot(next);retire_raster_snapshot(restored);
    }
    for row in fixture["invalid"].as_array().unwrap() {let result=handle(&DuplicateLayer {layer_id:row["id"].as_str().unwrap().into()},&doc,&cfg);assert!(result.is_err(),"{}",row["id"]);assert!(result.err().unwrap().message.contains(row["error"].as_str().unwrap()));}
    drop(doc);retire_raster_snapshot(document);
}
