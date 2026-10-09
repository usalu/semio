use crate::{DslValue,FromValue,Number,ToValue};
use crate::retained_clone::RetainedCloneGrant;

#[test]
fn original_retained_grant_wire_preserves_independent_axes_and_refuses_malformed_authority(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["rows"].as_array().unwrap(){
        let oracle:RetainedCloneGrant=serde_json::from_value(row.clone()).unwrap();
        let encoded=oracle.to_value();
        let mut accepted=|_|true;let mut control=crate::NativeDecodeControl::new(0,&mut accepted);
        let actual=RetainedCloneGrant::from_value_controlled(&encoded,&mut control).unwrap();
        assert_eq!(actual,oracle);assert_eq!(serde_json::to_value(actual).unwrap(),*row);
        let DslValue::Object(entries)=encoded else{panic!("grant object")};
        for index in 0..entries.len(){let mut missing=entries.clone();missing.remove(index);assert!(RetainedCloneGrant::from_value(DslValue::Object(missing)).is_err());}
        let mut unknown=entries.clone();unknown.push(("bytes".into(),DslValue::Number(Number::UInt(1))));assert!(RetainedCloneGrant::from_value(DslValue::Object(unknown)).is_err());
        let mut duplicate=entries.clone();duplicate.push(entries[0].clone());assert!(RetainedCloneGrant::from_value(DslValue::Object(duplicate)).is_err());
        let mut signed=entries;signed[0].1=DslValue::Number(Number::Int(-1));assert!(RetainedCloneGrant::from_value(DslValue::Object(signed)).is_err());
    }
    let maximum=fixture["maximum64"].as_str().unwrap().parse::<u64>().unwrap();
    let fields=fixture["fields"].as_array().unwrap().iter().map(|name|(name.as_str().unwrap().to_owned(),DslValue::Number(Number::UInt(maximum)))).collect();
    let actual=RetainedCloneGrant::from_value(DslValue::Object(fields));
    if usize::BITS==64{assert_eq!(actual.unwrap().maximum_release_bytes,usize::MAX);}else{assert!(actual.is_err());}
    println!("[DEBUG] original grant native/serde strict wire conserves all unequal/zero axes and checked full-u64 authority");
}


#[test]
fn original_retained_progress_wire_preserves_actual_physical_axes() {
    use crate::retained_clone::RetainedCloneProgress;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["receipts"].as_array().unwrap(){
        let oracle:RetainedCloneProgress=serde_json::from_value(row.clone()).unwrap();
        let encoded=oracle.to_value();let mut accepted=|_|true;let mut control=crate::NativeDecodeControl::new(0,&mut accepted);
        let actual=RetainedCloneProgress::from_value_controlled(&encoded,&mut control).unwrap();
        assert_eq!(actual,oracle);assert_eq!(serde_json::to_value(actual).unwrap(),*row);
        let DslValue::Object(entries)=encoded else{panic!("original progress object")};
        for index in 0..entries.len(){let mut missing=entries.clone();missing.remove(index);assert!(RetainedCloneProgress::from_value(DslValue::Object(missing)).is_err());}
        let mut unknown=entries.clone();unknown.push(("bytes".into(),DslValue::Number(Number::UInt(1))));assert!(RetainedCloneProgress::from_value(DslValue::Object(unknown)).is_err());
        let mut duplicate=entries;duplicate.push(duplicate[0].clone());assert!(RetainedCloneProgress::from_value(DslValue::Object(duplicate)).is_err());
    }
    println!("[DEBUG] canonical physical receipt matches independent serde without dropping terminal release");
}

#[test]
fn original_retained_wire_restores_parent_workload_after_nested_axes(){
    use crate::retained_clone::RetainedCloneProgress;
    let policy:serde_json::Value=serde_json::from_str(include_str!("../../🪆️stage/🧫️fixtures/🔣️.json")).unwrap();
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let grant:RetainedCloneGrant=serde_json::from_value(fixture["rows"][1].clone()).unwrap();
    let progress:RetainedCloneProgress=serde_json::from_value(fixture["receipts"][1].clone()).unwrap();
    let mut accept=|_|true;let mut encode=crate::NativeEncodeControl::new(4096,&mut accept);encode.begin_stage(policy["parentUnits"].as_u64().unwrap() as usize).unwrap();
    let grant_wire=grant.to_value_controlled(&mut encode).unwrap();let progress_wire=progress.to_value_controlled(&mut encode).unwrap();encode.step().unwrap();
    let mut accept=|_|true;let mut decode=crate::NativeDecodeControl::new(0,&mut accept);decode.begin_stage(policy["parentUnits"].as_u64().unwrap() as usize).unwrap();
    assert_eq!(RetainedCloneGrant::from_value_controlled(&grant_wire,&mut decode).unwrap(),grant);assert_eq!(RetainedCloneProgress::from_value_controlled(&progress_wire,&mut decode).unwrap(),progress);decode.step().unwrap();
    assert_eq!(serde_json::to_value(grant).unwrap(),fixture["rows"][1]);assert_eq!(serde_json::to_value(progress).unwrap(),fixture["receipts"][1]);
    println!("[DEBUG] canonical retained wire child stages5/4 restore parent1 in encode/decode, independent serde axes unchanged");
}
