//! 🔬️ Borrowed durable wire projections preserve the portable schema and original source fields.
use crate::lifted_durable_group::DurableOwnedGroupAnchorV1;
use semio_framework_dsl_record::{DslField,native_encoding::FieldProjectionView as View};
fn project(source:&impl DslField,path:&mut Vec<usize>)->serde_json::Value{
    match source.projection_view(path).unwrap(){
        View::IntrinsicText(value)=>serde_json::Value::String(value.into()),
        View::IntrinsicObject(count)=>{let mut object=serde_json::Map::new();for index in 0..count{let key=source.projection_key(path,index).unwrap().to_owned();path.push(index);object.insert(key,project(source,path));path.pop();}serde_json::Value::Object(object)},
        _=>panic!("portable anchor has only object and text fields"),
    }
}
#[test]
fn original_durable_wire_projection_matches_portable_anchor(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let input=&fixture["anchor"];let dialect=&input["parent"]["dialect"];let source=DurableOwnedGroupAnchorV1{schema:input["schema"].as_str().unwrap().into(),parent:semio_framework_artifact_reference::ArtifactRef{artifact_id:input["parent"]["artifactId"].as_str().unwrap().into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:dialect["artifactKind"].as_str().unwrap().into(),standard:dialect["standard"].as_str().unwrap().into(),subset:dialect["subset"].as_str().unwrap().into()}},shape:input["shape"].as_str().unwrap().into()};let original=(source.schema.as_ptr(),source.parent.artifact_id.as_ptr());assert_eq!(project(&source,&mut Vec::new()),fixture["anchor"]);assert_eq!(serde_json::to_vec(&project(&source,&mut Vec::new())).unwrap(),serde_json::to_vec(&fixture["anchor"]).unwrap());assert_eq!((source.schema.as_ptr(),source.parent.artifact_id.as_ptr()),original);assert!(source.projection_view(&[3]).is_err());
}
