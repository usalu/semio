use super::*;
use crate::standards::v4::subsets::any::schema::snapshot::from_part21_document;
fn fixtures() -> semio_framework_pack_json::Value { semio_framework_pack_json::parse(include_str!("🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap() }
fn snapshot() -> IfcSnapshot {
    let source = format!("ISO-10303-21;HEADER;FILE_DESCRIPTION((''),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4'));ENDSEC;DATA;{}ENDSEC;END-ISO-10303-21;", fixtures()["data"].as_str().unwrap());
    from_part21_document("stdio.ifc", &semio_s_artifact_stdio_contract::part21::parse_part21(&source).unwrap())
}
#[semio_framework_async_macros::async_test]
async fn semantic_fields_and_navigation_follow_language_neutral_vectors() {
    for row in fixtures()["cases"].as_array().unwrap() {
        let projected = project_semantics(&snapshot(), row["id"].as_str().unwrap().parse().unwrap(), &mut || Ok(())).unwrap();
        assert_eq!(projected.fields.len(), row["fields"].as_object().unwrap().len());
        for field in projected.fields {
            let expected = &row["fields"][field.descriptor.id.as_str()];
            assert_eq!(field.value.as_deref(), expected.as_str());
            assert!(!field.descriptor.label_en.is_empty() && !field.descriptor.label_de.is_empty());
        }
        for (actual, key) in [(projected.forward, "forward"), (projected.back, "back")] { assert_eq!(actual, row[key].as_array().unwrap().iter().map(|value| value.as_str().unwrap().parse::<u64>().unwrap()).collect::<Vec<_>>()); }
    }
}
#[semio_framework_async_macros::async_test]
async fn semantic_edits_enforce_declared_types_and_optionality() {
    let original = snapshot();
    for row in fixtures()["edits"].as_array().unwrap() {
        let input = semio_framework_pack_json::to_dsl_value(&row["value"]);
        let result = edit_semantic(&original, row["id"].as_str().unwrap().parse().unwrap(), row["field"].as_str().unwrap(), &input);
        if let Some(reason) = row.get("error").and_then(semio_framework_pack_json::Value::as_str) { assert_eq!(result.unwrap_err(), reason); } else {
            let edit = result.unwrap(); assert_eq!(edit.index as u64, row["index"].as_u64().unwrap());
            match edit.value { IfcValue::Unset => assert_eq!(row["expected"]["kind"].as_str(), Some("unset")), IfcValue::String(value) => assert_eq!(Some(value.as_str()), row["expected"]["value"].as_str()), _ => panic!("unexpected semantic edit") }
        }
    }
}
#[semio_framework_async_macros::async_test]
async fn semantic_projection_propagates_every_interruption() {
    let original = snapshot(); let mut work=0; project_semantics(&original,10,&mut || {work+=1;Ok(())}).unwrap();
    for stop in 1..=work {let mut count=0;let result=project_semantics(&original,10,&mut ||{count+=1;if count==stop{Err("cancelled".into())}else{Ok(())}});assert_eq!(result.unwrap_err(),"cancelled");assert_eq!(count,stop);}
}
#[semio_framework_async_macros::async_test]
async fn semantic_identity_is_canonical_and_complete() {
    for id in [1,9007199254740993,u64::MAX] {assert_eq!(parse_semantic_identity(&semantic_identity(id)),Some(id));}
    for value in ["ifc-0","ifc-01","ifc--1","ifc-1-2","ifc-18446744073709551616","1"] {assert_eq!(parse_semantic_identity(value),None);}
}

#[semio_framework_async_macros::async_test]
async fn controlled_semantic_edit_scans_obey_every_interruption(){
    let snapshot=snapshot();for (field,value) in [("name","New"),("globalId","0123456789012345678907")]{let value=DslValue::String(value.into());let mut work=0;edit_semantic_controlled(&snapshot,10,field,&value,&mut ||{work+=1;Ok(())}).unwrap();for stop in 1..=work{let mut count=0;let result=edit_semantic_controlled(&snapshot,10,field,&value,&mut ||{count+=1;if count==stop{Err("cancelled".into())}else{Ok(())}});assert_eq!(result.unwrap_err(),"cancelled");assert_eq!(count,stop);}}
    assert!(edit_semantic(&snapshot,10,"name",&DslValue::String("😀".repeat(255))).is_ok());assert_eq!(edit_semantic(&snapshot,10,"name",&DslValue::String("😀".repeat(256))).unwrap_err(),"semantic-text-limit");assert_eq!(edit_semantic(&snapshot,10,"description",&DslValue::String("x".repeat(8193))).unwrap_err(),"semantic-text-limit");
}
