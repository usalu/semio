//! 🎛️ Neutral tone vectors exercise retained publication, exact inverses and coalesced diffs.
use super::*;
use protocol::{Mutation,MutationDiff,OpText,OpBinary};
fn retire(document:RasterSnapshot) {crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);}
#[test]
fn adjustment_parameter_vectors_preserve_other_fields_and_exact_undo() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let parameter=row["parameter"].as_str().unwrap();
        let mut reference=serde_json::json!({"schema":"raster.document","id":"tone-test","layers":[{"kind":"adjustment","id":"tone","name":"Tone","adjustmentKind":"brightnessContrast","params":{"metadata":"preserved"}}]});
        if !row["before"].is_null() {reference["layers"][0]["params"][parameter]=row["before"].clone();}
        let before:RasterSnapshot=dsl::json::from_json_str(&reference.to_string()).unwrap();
        let payload=ChangeLayerAdjustmentParameter {layer_id:"tone".into(),parameter:parameter.into(),expected:if row["before"].is_null(){None}else{Some(dsl::json::from_json_str(&row["before"].to_string()).unwrap())},value:if row["after"].is_null(){None}else{Some(dsl::json::from_json_str(&row["after"].to_string()).unwrap())}};
        let mutation=RasterMutation::ChangeLayerAdjustmentParameter(payload.clone());
        let text=mutation.print_op();assert_eq!(RasterMutation::parse_op(&text).unwrap(),mutation);
        assert_eq!(RasterMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(),mutation);
        let (after,messages)=semio_framework_os_kernel::apply_mutation(&before,&mutation).unwrap();assert!(messages.is_empty());
        if row["after"].is_null() {reference["layers"][0]["params"].as_object_mut().unwrap().remove(parameter);} else {reference["layers"][0]["params"][parameter]=row["after"].clone();}
        let expected:RasterSnapshot=dsl::json::from_json_str(&reference.to_string()).unwrap();assert_eq!(after,expected);
        let inverse=mutation.inverse(&before).remove(0);
        let (restored,messages)=semio_framework_os_kernel::apply_mutation(&after,&inverse).unwrap();assert!(messages.is_empty());assert_eq!(restored,before);
        assert!(validate(&ChangeLayerAdjustmentParameter {expected:Some(crate::RasterAdjustmentNumber::decimal(0.777)),..payload.clone()},&before).is_err());
        for invalid in fixture["invalid"].as_array().unwrap() {if let Some(value)=invalid["value"].as_f64() {assert!(validate(&ChangeLayerAdjustmentParameter {parameter:invalid["parameter"].as_str().unwrap().into(),value:Some(crate::RasterAdjustmentNumber::decimal(value)),..payload.clone()},&before).is_err());}}
        for document in [before,after,expected,restored] {retire(document);}
    }
}
#[test]
fn adjustment_parameter_diff_composition_keeps_brightness_and_contrast() {
    let mut before=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();
    let layer=crate::standards::v1::subsets::any::schema::create_layer_of_kind("adjustment");
    let id=crate::standards::v1::subsets::any::schema::layer_node_id(&layer).to_owned();before.layers.push(layer);
    let first=RasterMutation::ChangeLayerAdjustmentParameter(ChangeLayerAdjustmentParameter {layer_id:id.clone(),parameter:"brightness".into(),expected:None,value:Some(crate::RasterAdjustmentNumber::decimal(0.2))});
    let second=RasterMutation::ChangeLayerAdjustmentParameter(ChangeLayerAdjustmentParameter {layer_id:id,parameter:"contrast".into(),expected:None,value:Some(crate::RasterAdjustmentNumber::decimal(-0.3))});
    let (mut combined,_)=first.diff(&before).into_parts();let middle=combined.apply(&before).unwrap();
    let (tail,_)=second.diff(&middle).into_parts();let expected=tail.apply(&middle).unwrap();combined.absorb(tail);
    let actual=combined.apply(&before).unwrap();assert_eq!(actual,expected);combined.retire_cold();
    for document in [before,middle,actual,expected] {retire(document);}
}

#[test]
fn adjustment_parameter_diff_can_replace_a_key_at_map_capacity() {
    let mut before=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();
    let mut layer=crate::standards::v1::subsets::any::schema::create_layer_of_kind("adjustment");
    let id=crate::standards::v1::subsets::any::schema::layer_node_id(&layer).to_owned();
    let RasterLayerNode::Adjustment {params,..}=&mut layer else {panic!("adjustment")};
    params.insert("brightness".into(),dsl::DslValue::float(0.2)).unwrap();
    for index in 1..crate::RASTER_OWNED_MAP_CAPACITY {params.insert(format!("metadata-{index}"),dsl::DslValue::Bool(true)).unwrap();}
    before.layers.push(layer);
    let patch=RasterLayerPatch {adjustment_parameters:Some(vec![RasterAdjustmentParameter {parameter:"contrast".into(),value:Some(crate::RasterAdjustmentNumber::decimal(0.3))},RasterAdjustmentParameter {parameter:"brightness".into(),value:None}]),..Default::default()};
    let diff=diff_patch_layer(&id,patch);let after=diff.apply(&before).unwrap();
    let RasterLayerNode::Adjustment {params,..}=&after.layers[0] else {panic!("adjustment")};
    assert_eq!(params.len(),crate::RASTER_OWNED_MAP_CAPACITY);assert!(!params.contains_key("brightness"));assert_eq!(params.get("contrast").and_then(dsl::DslValue::as_f64),Some(0.3));
    diff.retire_cold();retire(before);retire(after);
}

#[test]
fn adjustment_parameter_patch_vectors_match_schema_acceptance() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for row in fixture["parameterPatches"].as_array().unwrap() {
        let mut before=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();
        let layer=crate::standards::v1::subsets::any::schema::create_layer_of_kind("adjustment");
        let id=crate::standards::v1::subsets::any::schema::layer_node_id(&layer).to_owned();
        before.layers.push(layer);
        let patch:RasterLayerPatch=dsl::json::from_json_str(&serde_json::json!({"adjustmentParameters":row["parameters"]}).to_string()).unwrap();
        let diff=diff_patch_layer(&id,patch);
        let result=diff.apply(&before);
        assert_eq!(result.is_ok(),row["valid"].as_bool().unwrap(),"{row}");
        if let Ok(after)=result {retire(after);}
        diff.retire_cold();retire(before);
    }
}
