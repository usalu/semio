//! 🎭️ Mask events match independent JSON edits and undo exactly through both wire formats.
use crate::{RasterMutation,RasterSnapshot};
use protocol::{Mutation,MutationDiff,OpText,OpBinary};

fn fixture()->serde_json::Value {serde_json::from_str(include_str!("🔣️.json")).unwrap()}
fn normalized(value:serde_json::Value)->serde_json::Value {
    match value {
        serde_json::Value::Number(value)=>serde_json::Value::from(value.as_f64().unwrap()),
        serde_json::Value::Array(value)=>serde_json::Value::Array(value.into_iter().map(normalized).collect()),
        serde_json::Value::Object(value)=>serde_json::Value::Object(value.into_iter().map(|(key,value)|(key,normalized(value))).collect()),
        value=>value,
    }
}
#[test]
fn mask_mutations_match_json_oracle_and_exact_history() {
    let fixture=fixture();
    for case in fixture["cases"].as_array().unwrap() {
        let mask=|name:&str|case[name].as_str().map_or(serde_json::Value::Null,|key|fixture[key].clone());
        let mut before=fixture["before"].clone();before["layers"][0]["mask"]=mask("before");
        let base:RasterSnapshot=dsl::json::from_json_str(&before.to_string()).unwrap();
        let mut expected:serde_json::Value=serde_json::from_str(&dsl::json::to_json_string(&base)).unwrap();
        expected["layers"][0]["mask"]=mask("after");
        let value=serde_json::json!({"mutation":"changeLayerMask","layerId":"paint","expected":mask("before"),"mask":mask("after")});
        let mutation:RasterMutation=dsl::json::from_json_str(&value.to_string()).unwrap();
        let inverse=mutation.inverse(&base);
        let (diff,messages)=mutation.diff(&base).into_parts();assert!(messages.is_empty());
        let actual=diff.apply(&base).unwrap();
        let rendered:serde_json::Value=serde_json::from_str(&dsl::json::to_json_string(&actual)).unwrap();
        assert_eq!(normalized(rendered),normalized(expected),"{}",case["name"]);
        assert_eq!(RasterMutation::parse_op(&mutation.print_op()).unwrap(),mutation);
        assert_eq!(RasterMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(),mutation);
        let (undo,messages)=inverse[0].diff(&actual).into_parts();assert!(messages.is_empty());
        let restored=undo.apply(&actual).unwrap();assert_eq!(restored,base);
        MutationDiff::retire_cold(diff);MutationDiff::retire_cold(undo);
        for document in [base,actual,restored] {crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);}
    }
}
#[test]
fn mask_mutations_reject_conflicts_and_invalid_assets() {
    let fixture=fixture();let base:RasterSnapshot=dsl::json::from_json_str(&fixture["before"].to_string()).unwrap();
    let mut missing=fixture["reveal"].clone();missing["imageKey"]=serde_json::Value::from("missing");
    let mut singular=fixture["reveal"].clone();singular["transform"]["a"]=serde_json::Value::from(0);
    for (expected,mask) in [(fixture["reveal"].clone(),fixture["hidden"].clone()),(serde_json::Value::Null,missing),(serde_json::Value::Null,singular)] {
        let mutation:RasterMutation=dsl::json::from_json_str(&serde_json::json!({"mutation":"changeLayerMask","layerId":"paint","expected":expected,"mask":mask}).to_string()).unwrap();
        let (diff,messages)=mutation.diff(&base).into_parts();assert!(!messages.is_empty());MutationDiff::retire_cold(diff);
    }
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(base);
}
/// 🧾️ The committed wire witness is the canonical Rust wire of the fixture's "Add Reveal Mask" case.
#[test]
fn committed_wire_witness_is_the_canonical_rust_wire() {
    let fixture=fixture();
    let witnessed:RasterMutation=store::os_store::test_support::assert_wire_witness(include_str!("../../../../🧫️fixtures/🧬️mutations/🎭️change-layer-mask/🧾️wire-witness/🦠️mutation/🔣️.json"));
    let expected:RasterMutation=dsl::json::from_json_str(&serde_json::json!({"mutation":"changeLayerMask","layerId":"paint","expected":null,"mask":fixture["reveal"]}).to_string()).unwrap();
    assert_eq!(witnessed,expected);
}
