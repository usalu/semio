//! 📦️ Packed coverage preserves flat JSON and metadata at every immutable page boundary.
use super::*;
use semio_framework_plugin::artifact_app_laws::OriginalWorkFixture;
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
fn serde_rows(count:usize,cycle:&[serde_json::Value])->serde_json::Value{serde_json::Value::Array((0..count).map(|index|serde_json::json!({"start":index,"length":1,"coverage":cycle[index%cycle.len()]})).collect())}
#[test]
fn packed_coverage_read_codecs_match_the_independent_flat_vector(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for count in fixture["counts"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as usize){
        let mut caller=OriginalWorkFixture::new();let cycle=fixture["span"]["coverageCycle"].as_array().unwrap();
        let rows:Vec<_>=(0..count).map(|index|RasterSelectionSpan{start:index as u32,length:1,coverage:cycle[index%cycle.len()].as_u64().unwrap() as u32}).collect();
        let expected=ToValue::to_value(&rows);let root:RasterSelectionSpans=caller.receive_native(expected.clone(),OriginalWorkFixture::stock());
        assert_eq!(ToValue::to_value(&root),expected);assert_eq!(serde_json::Value::from(&ToValue::to_value(&root)),serde_rows(count,cycle));
        assert_eq!(root.len(),count);assert_eq!(root.pages().len(),RasterSelectionSpans::page_count(count));assert_eq!(root.value_shape_at_path(&[]).unwrap(),ValueShape::Array{len:count});assert_eq!(root.iter().count(),count);
        for index in 0..count{assert_eq!(root.get(index),rows.get(index));let(value,work)=root.get_with_work(index);assert_eq!(value,root.get(index));assert!(work>0);let(slice,work)=root.page_slice_with_work(index);assert_eq!(slice,root.page_slice(index));assert!(work>0);assert_eq!(root.value_at_path(&[&index.to_string()]).unwrap(),ToValue::to_value(&rows[index]));assert!(matches!(root.canonical_tree_child(index).unwrap().canonical_tree_node().unwrap(),semio_framework_pack_json::ArtifactCanonicalJsonNode::Object(_)));}
        assert!(root.canonical_tree_child(count).is_err());assert!(root.value_at_path(&[&count.to_string()]).is_err());assert!(<RasterSelectionSpans as FromValue>::from_value(expected).is_err());
        assert_eq!(format!("{:?}",<RasterSelectionSpans as DslField>::shape()),format!("{:?}",<Vec<RasterSelectionSpan> as DslField>::shape()));assert_eq!(DslField::to_value(&root),DslField::to_value(&rows));
        caller.retire_owned(root,OriginalWorkFixture::stock());caller.finish();
    }
}
