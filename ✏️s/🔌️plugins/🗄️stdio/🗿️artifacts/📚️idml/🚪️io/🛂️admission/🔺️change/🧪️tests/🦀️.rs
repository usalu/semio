use semio_s_artifact_stdio_idml::io::admission::{IdmlAdmissionCursor,IdmlAdmissionSource,IdmlAdmissionGrant,IdmlAdmissionParts,change::*};
use crate::{IdmlSnapshot,IdmlDiff,schema::{IdmlPart,diff::{IdmlPatch,IdmlPartPatch}}};
fn admitted(snapshot:&IdmlSnapshot)->IdmlAdmissionCursor{let mut cursor=IdmlAdmissionCursor::new();loop{let step=cursor.advance(IdmlAdmissionSource::snapshot(snapshot),IdmlAdmissionGrant{maximum_work:7,maximum_copy_bytes:2048,maximum_capacity_bytes:1024,maximum_release_bytes:0});assert!(step.error.is_none());if step.done{return cursor;}}}
fn close(index:&mut IdmlAdmissionCursor){index.begin_close();while index.next_release_bytes()!=0{index.close_one(index.next_release_bytes());}assert!(index.terminal_is_empty());}
fn event_value(event:IdmlChangeEvent)->serde_json::Value{match event{IdmlChangeEvent::Version=>serde_json::json!({"kind":"version"}),IdmlChangeEvent::Order=>serde_json::json!({"kind":"order"}),IdmlChangeEvent::Remove{before}=>serde_json::json!({"kind":"remove","before":before}),IdmlChangeEvent::Part{next,before:Some(before)}=>serde_json::json!({"kind":"part","next":next,"before":before}),IdmlChangeEvent::Part{next,before:None}=>serde_json::json!({"kind":"part","next":next})}}
#[test]
fn portable_numeric_changes_compile_to_existing_typed_sparse_diffs(){
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧪️fixtures/🔣️.json")).unwrap();
    for row in corpus["cases"].as_array().unwrap(){
        let before=IdmlSnapshot{dom_version:corpus["base"]["version"].as_str().unwrap().into(),parts:corpus["base"]["parts"].as_array().unwrap().iter().map(|row|{let text=row[1].as_str().unwrap();IdmlPart{path:row[0].as_str().unwrap().into(),bytes:if text=="$mime"{crate::PACKAGE_MIME.as_bytes().into()}else{text.as_bytes().into()}}}).collect()};let mut after=before.clone();
        for operation in row["operations"].as_array().unwrap(){match operation["kind"].as_str().unwrap(){
            "replace"=>after.parts[operation["index"].as_u64().unwrap() as usize].bytes=operation["text"].as_str().unwrap().as_bytes().into(),
            "append"=>after.parts.push(IdmlPart{path:operation["path"].as_str().unwrap().into(),bytes:if let Some(bytes)=operation["bytes"].as_array(){bytes.iter().map(|byte|byte.as_u64().unwrap() as u8).collect()}else{operation["text"].as_str().unwrap().as_bytes().into()}}),
            "remove"=>{after.parts.remove(operation["index"].as_u64().unwrap() as usize);},
            "order"=>{let original=after.parts;after.parts=operation["indices"].as_array().unwrap().iter().map(|index|original[index.as_u64().unwrap() as usize].clone()).collect();},
            "version"=>after.dom_version=operation["value"].as_str().unwrap().into(),_=>panic!("unknown operation")}}
        assert!(before.package().is_ok());assert!(after.package().is_ok());let saved_before=before.clone();let saved_after=after.clone();let mut base_index=admitted(&before);let mut next_index=admitted(&after);let base_source=IdmlAdmissionSource::snapshot(&before);let next_source=IdmlAdmissionSource::snapshot(&after);
        for work in [1,7,64,4096]{let mut cursor=IdmlChangeCursor::new();let mut values=Vec::new();let mut patch=IdmlPatch::default();
            loop{let step=cursor.advance(base_source,next_source,&base_index,&next_index,work);assert!(step.work<=work);assert!(step.error.is_none(),"{} {:?}",row["id"],step.error);if let Some(event)=step.event{values.push(event_value(event));match event{IdmlChangeEvent::Version=>patch.dom_version=Some(after.dom_version.clone()),IdmlChangeEvent::Order=>patch.part_order=Some(after.parts.iter().map(|part|part.path.clone()).collect()),IdmlChangeEvent::Remove{before:at}=>patch.parts.push(IdmlPartPatch{path:before.parts[at].path.clone(),bytes:None}),IdmlChangeEvent::Part{next:at,..}=>patch.parts.push(IdmlPartPatch{path:after.parts[at].path.clone(),bytes:Some(after.parts[at].bytes.clone())})}}if step.done{break;}}
            assert_eq!(serde_json::Value::Array(values.clone()),row["expected"],"{}",row["id"]);let diff=IdmlDiff{steps:if values.is_empty(){vec![]}else{vec![patch]}};assert_eq!(diff,IdmlDiff::changing(&before,&after));assert_eq!(semio_framework_os_kernel::apply_diff_cold(&diff,&before).unwrap(),after);assert_eq!(before,saved_before);assert_eq!(after,saved_after);cursor.close();assert!(cursor.terminal_is_empty());
        }
        for cut in [0,1,10,100]{let mut cursor=IdmlChangeCursor::new();for _ in 0..cut{let step=cursor.advance(base_source,next_source,&base_index,&next_index,1);assert!(step.error.is_none());if step.done{break;}}cursor.close();assert!(cursor.terminal_is_empty());assert_eq!(before,saved_before);assert_eq!(after,saved_after);}close(&mut base_index);close(&mut next_index);
    }
}
